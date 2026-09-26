//! Bounded, uncached source reads through the existing HTTP client.
//!
//! No redirects, no transparent decompression, no credential/header persistence.
//! `content` contains the received HTTP entity bytes (not reserialized JSON and
//! not TLS/framing bytes). Retry-After is a lower bound, never clamped down.
use super::{run_on_http_thread, HttpClient};
use crate::{CoreError, Result};
use reqwest::header::{ACCEPT_ENCODING, CONTENT_LENGTH};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::Read,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct SourceHttpOptions {
    pub max_bytes: usize,
    pub attempts: usize,
    pub request_timeout: Duration,
    pub total_timeout: Duration,
    pub backoff: Duration,
    pub max_retry_wait: Duration,
    pub headers: Vec<(String, String)>,
}
impl Default for SourceHttpOptions {
    fn default() -> Self {
        Self {
            max_bytes: 8 * 1024 * 1024,
            attempts: 3,
            request_timeout: Duration::from_secs(15),
            total_timeout: Duration::from_secs(45),
            backoff: Duration::from_millis(200),
            max_retry_wait: Duration::from_secs(10),
            headers: vec![("Accept".into(), "application/json".into())],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceHttpResponse {
    pub url: String,
    pub status: u16,
    pub content: Vec<u8>,
    /// Only the explicit safe allowlist below; never cookies or authorization.
    pub headers: BTreeMap<String, String>,
    pub observed_at: i64,
    pub attempts: usize,
}
impl SourceHttpResponse {
    pub fn ensure_success(&self) -> Result<()> {
        if (200..300).contains(&self.status) {
            return Ok(());
        }
        Err(CoreError::HttpStatus {
            status: reqwest::StatusCode::from_u16(self.status)
                .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
            url: self.url.clone(),
            body: format!(
                "source request rejected; retry-after={:?}",
                self.headers.get("retry-after")
            ),
        })
    }
    pub fn content_type(&self) -> &str {
        self.headers
            .get("content-type")
            .map(String::as_str)
            .unwrap_or("")
    }
}

fn bounded_error(message: &str) -> CoreError {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message).into()
}

impl HttpClient {
    /// Uses the common client, without the URL-only legacy cache (which cannot
    /// represent request headers, source validators and original fetch time).
    pub fn get_bounded(&self, url: &str, options: SourceHttpOptions) -> Result<SourceHttpResponse> {
        if options.max_bytes == 0
            || options.max_bytes > 64 * 1024 * 1024
            || !(1..=8).contains(&options.attempts)
            || options.request_timeout.is_zero()
            || options.total_timeout.is_zero()
            || options.total_timeout > Duration::from_secs(300)
        {
            return Err(bounded_error("invalid bounded HTTP budget"));
        }
        let parsed = reqwest::Url::parse(url).map_err(|_| bounded_error("invalid source URL"))?;
        if !matches!(parsed.scheme(), "http" | "https")
            || !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.fragment().is_some()
        {
            return Err(bounded_error(
                "source URL must be HTTP(S), without credentials or fragment",
            ));
        }
        run_on_http_thread(|| self.fetch_bounded(url, &options))
    }

    fn fetch_bounded(&self, url: &str, options: &SourceHttpOptions) -> Result<SourceHttpResponse> {
        let started = Instant::now();
        for attempt in 1..=options.attempts {
            let remaining = options.total_timeout.saturating_sub(started.elapsed());
            if remaining.is_zero() {
                return Err(bounded_error("source HTTP total timeout"));
            }
            let mut request = self
                .bounded_source_client
                .get(url)
                .timeout(options.request_timeout.min(remaining));
            for (name, value) in &options.headers {
                request = request.header(name, value);
            }
            // The bounded source policy has all transparent decoding disabled.
            request = request.header(ACCEPT_ENCODING, "identity");
            let response = match request.send() {
                Ok(response) => response,
                Err(error) => {
                    let delay = options.backoff.saturating_mul(attempt as u32);
                    if attempt == options.attempts
                        || !(error.is_timeout() || error.is_connect())
                        || !can_wait(delay, started, options)
                    {
                        return Err(error.into());
                    }
                    thread::sleep(delay);
                    continue;
                }
            };
            let status = response.status();
            if response
                .headers()
                .get(CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .is_some_and(|n| n > options.max_bytes as u64)
            {
                return Err(bounded_error(
                    "source HTTP body exceeds byte limit (Content-Length)",
                ));
            }
            let mut headers = BTreeMap::new();
            for key in [
                "content-type",
                "content-encoding",
                "etag",
                "last-modified",
                "date",
                "retry-after",
            ] {
                if let Some(value) = response.headers().get(key).and_then(|v| v.to_str().ok()) {
                    // reqwest bounds HTTP headers; additionally bound stored provenance.
                    if value.len() > 4096 {
                        return Err(bounded_error("source HTTP provenance header too large"));
                    }
                    headers.insert(key.to_owned(), value.to_owned());
                }
            }
            let mut content = Vec::new();
            response
                .take(options.max_bytes as u64 + 1)
                .read_to_end(&mut content)?;
            if content.len() > options.max_bytes {
                return Err(bounded_error(
                    "source HTTP body exceeds byte limit (stream)",
                ));
            }
            let result = SourceHttpResponse {
                url: url.to_owned(),
                status: status.as_u16(),
                content,
                headers,
                observed_at: crate::now_epoch_seconds()?,
                attempts: attempt,
            };
            if !(status.is_server_error() || status == reqwest::StatusCode::TOO_MANY_REQUESTS)
                || attempt == options.attempts
            {
                return Ok(result);
            }
            let delay = match result.headers.get("retry-after") {
                Some(value) => match retry_after(value, result.observed_at) {
                    Some(delay) => delay.max(options.backoff.saturating_mul(attempt as u32)),
                    // Unknown Retry-After must not cause an early retry.
                    None => return Ok(result),
                },
                None => options.backoff.saturating_mul(attempt as u32),
            };
            if !can_wait(delay, started, options) {
                return Ok(result);
            }
            thread::sleep(delay);
        }
        Err(bounded_error("exhausted source HTTP budget"))
    }
}

fn can_wait(delay: Duration, started: Instant, options: &SourceHttpOptions) -> bool {
    delay <= options.max_retry_wait
        && delay < options.total_timeout.saturating_sub(started.elapsed())
}

fn retry_after(value: &str, now: i64) -> Option<Duration> {
    if let Ok(seconds) = value.trim().parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    let at = chrono::DateTime::parse_from_rfc2822(value.trim())
        .ok()?
        .timestamp();
    Some(Duration::from_secs(at.saturating_sub(now).max(0) as u64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, net::TcpListener};
    fn serve(responses: Vec<Vec<u8>>) -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}/source", listener.local_addr().unwrap());
        let handle = thread::spawn(move || {
            for response in responses {
                let deadline = Instant::now() + Duration::from_secs(3);
                let mut stream = loop {
                    match listener.accept() {
                        Ok((stream, _)) => break stream,
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            assert!(Instant::now() < deadline, "missing expected request");
                            thread::sleep(Duration::from_millis(2));
                        }
                        Err(e) => panic!("{e}"),
                    }
                };
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut request = [0; 4096];
                let n = stream.read(&mut request).unwrap();
                let request = String::from_utf8_lossy(&request[..n]).to_lowercase();
                assert!(request.contains("accept-encoding: identity"));
                let _ = stream.write_all(&response);
            }
        });
        (url, handle)
    }
    fn response(status: &str, headers: &str, body: &[u8]) -> Vec<u8> {
        let mut result = format!(
            "HTTP/1.1 {status}\r\nContent-Length: {}\r\n{headers}Connection: close\r\n\r\n",
            body.len()
        )
        .into_bytes();
        result.extend_from_slice(body);
        result
    }
    fn client() -> (tempfile::TempDir, HttpClient) {
        let dir = tempfile::tempdir().unwrap();
        let client = HttpClient::new("bounded-source-test", dir.path()).unwrap();
        (dir, client)
    }
    #[test]
    fn preserves_bytes_and_safe_headers_without_cache() {
        let raw = b" {\"id\": 0, \"name\":\"x\"}\n";
        let (url, task) = serve(vec![response(
            "200 OK",
            "Content-Type: application/json\r\nETag: \"a\"\r\nSet-Cookie: secret=ignored\r\n",
            raw,
        )]);
        let (dir, http) = client();
        let result = http
            .get_bounded(&url, SourceHttpOptions::default())
            .unwrap();
        task.join().unwrap();
        assert_eq!(result.content, raw);
        assert_eq!(result.headers["etag"], "\"a\"");
        assert!(!result.headers.contains_key("set-cookie"));
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    }
    #[test]
    fn bounds_declared_and_chunked_bodies() {
        for bytes in [response("200 OK", "", b"123456"), b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n6\r\n123456\r\n0\r\n\r\n".to_vec()] {
            let (url, task) = serve(vec![bytes]); let (_dir, http) = client();
            let error = http.get_bounded(&url, SourceHttpOptions { max_bytes: 5, ..Default::default() }).unwrap_err();
            task.join().unwrap(); assert!(error.to_string().contains("byte limit"));
        }
    }
    #[test]
    fn bounded_retry_and_rate_limit_defer() {
        let (url, task) = serve(vec![
            response("429 Too Many Requests", "Retry-After: 0\r\n", b"busy"),
            response("200 OK", "", b"[]"),
        ]);
        let (_dir, http) = client();
        let result = http
            .get_bounded(
                &url,
                SourceHttpOptions {
                    backoff: Duration::ZERO,
                    ..Default::default()
                },
            )
            .unwrap();
        task.join().unwrap();
        assert_eq!(result.attempts, 2);
        assert_eq!(result.content, b"[]");
        for value in ["3600", "not-a-date"] {
            let (url, task) = serve(vec![response(
                "429 Too Many Requests",
                &format!("Retry-After: {value}\r\n"),
                b"busy",
            )]);
            let result = http
                .get_bounded(&url, SourceHttpOptions::default())
                .unwrap();
            task.join().unwrap();
            assert_eq!(result.attempts, 1);
            assert!(result.ensure_success().is_err());
        }
    }
    #[test]
    fn redirects_and_client_errors_are_not_retried() {
        for status in ["302 Found", "404 Not Found"] {
            let (url, task) = serve(vec![response(
                status,
                "Location: http://127.0.0.1:9/secret\r\n",
                b"",
            )]);
            let (_dir, http) = client();
            let result = http
                .get_bounded(&url, SourceHttpOptions::default())
                .unwrap();
            task.join().unwrap();
            assert_eq!(result.attempts, 1);
            assert!(result.ensure_success().is_err());
        }
    }
    #[test]
    fn rate_limit_dates_and_invalid_budgets() {
        assert_eq!(
            retry_after("Thu, 01 Jan 1970 00:01:00 GMT", 0),
            Some(Duration::from_secs(60))
        );
        assert_eq!(
            retry_after("Thu, 01 Jan 1970 00:01:00 GMT", 120),
            Some(Duration::ZERO)
        );
        let (_dir, http) = client();
        assert!(http
            .get_bounded(
                "http://127.0.0.1:9",
                SourceHttpOptions {
                    attempts: 0,
                    ..Default::default()
                }
            )
            .is_err());
        assert!(http
            .get_bounded(
                "http://user:password@localhost",
                SourceHttpOptions::default()
            )
            .is_err());
    }
    #[test]
    fn source_reads_preserve_compressed_bytes_without_changing_legacy_decoding() {
        let compressed: &[u8] = &[
            31, 139, 8, 0, 0, 0, 0, 0, 2, 3, 139, 142, 5, 0, 41, 187, 76, 13, 2, 0, 0, 0,
        ];
        let (url, task) = serve(vec![response(
            "200 OK",
            "Content-Encoding: gzip\r\nContent-Type: application/json\r\n",
            compressed,
        )]);
        let (_dir, http) = client();
        let raw = http
            .get_bounded(&url, SourceHttpOptions::default())
            .unwrap();
        task.join().unwrap();
        assert_eq!(raw.content, compressed);
        assert_eq!(raw.headers["content-encoding"], "gzip");
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/legacy", listener.local_addr().unwrap());
        let task = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut request = [0; 4096];
            assert!(stream.read(&mut request).unwrap() > 0);
            stream
                .write_all(&response(
                    "200 OK",
                    "Content-Encoding: gzip\r\nContent-Type: application/json\r\n",
                    compressed,
                ))
                .unwrap();
        });
        let legacy = http
            .get_no_redirect(&url, super::super::HttpGetOptions::default())
            .unwrap();
        task.join().unwrap();
        assert_eq!(legacy.content, b"[]");
    }

    #[test]
    fn timeout_is_bounded() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let task = thread::spawn(move || {
            let (_s, _) = listener.accept().unwrap();
            thread::sleep(Duration::from_millis(200));
        });
        let (_dir, http) = client();
        let started = Instant::now();
        assert!(http
            .get_bounded(
                &url,
                SourceHttpOptions {
                    attempts: 1,
                    request_timeout: Duration::from_millis(30),
                    total_timeout: Duration::from_millis(50),
                    ..Default::default()
                }
            )
            .is_err());
        assert!(started.elapsed() < Duration::from_secs(1));
        task.join().unwrap();
    }
}
