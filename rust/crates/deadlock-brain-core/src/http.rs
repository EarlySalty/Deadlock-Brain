use std::{
    fs,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{HeaderName, HeaderValue, CONTENT_TYPE},
    StatusCode,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{CoreError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryPolicy {
    pub attempts: usize,
    pub backoff: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            attempts: 3,
            backoff: Duration::from_millis(600),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpGetOptions {
    pub cache_ttl_seconds: Option<u64>,
    pub timeout: Option<Duration>,
    pub headers: Vec<(String, String)>,
    pub retry: Option<RetryPolicy>,
    pub allow_forbidden: bool,
}

impl Default for HttpGetOptions {
    fn default() -> Self {
        Self {
            cache_ttl_seconds: None,
            timeout: None,
            headers: Vec::new(),
            retry: None,
            allow_forbidden: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResult {
    pub url: String,
    pub content: Vec<u8>,
    pub from_cache: bool,
    pub content_type: String,
}

impl HttpResult {
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.content).into_owned()
    }

    pub fn json<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_str(&self.text())?)
    }
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    no_redirect_client: Client,
    user_agent: String,
    cache_dir: PathBuf,
    default_timeout: Duration,
    default_retry: RetryPolicy,
}

impl HttpClient {
    pub fn new(user_agent: impl Into<String>, cache_dir: impl Into<PathBuf>) -> Result<Self> {
        Self::new_with_defaults(
            user_agent,
            cache_dir,
            Duration::from_secs(30),
            RetryPolicy::default(),
        )
    }

    pub fn new_with_defaults(
        user_agent: impl Into<String>,
        cache_dir: impl Into<PathBuf>,
        default_timeout: Duration,
        default_retry: RetryPolicy,
    ) -> Result<Self> {
        let cache_dir = cache_dir.into();
        fs::create_dir_all(&cache_dir)?;
        let user_agent = user_agent.into();
        let client = Client::builder()
            .user_agent(user_agent.clone())
            .timeout(default_timeout)
            .build()?;
        let no_redirect_client = Client::builder()
            .user_agent(user_agent.clone())
            .timeout(default_timeout)
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        Ok(Self {
            client,
            no_redirect_client,
            user_agent,
            cache_dir,
            default_timeout,
            default_retry,
        })
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    fn apply_defaults(&self, options: &mut HttpGetOptions) {
        if options.timeout.is_none() {
            options.timeout = Some(self.default_timeout);
        }
        if options.retry.is_none() {
            options.retry = Some(self.default_retry.clone());
        }
    }

    pub fn get(&self, url: &str, mut options: HttpGetOptions) -> Result<HttpResult> {
        self.apply_defaults(&mut options);
        if let Some(ttl) = options.cache_ttl_seconds {
            if let Some(cached) = self.cached_result(url, ttl)? {
                return Ok(cached);
            }
        }

        let result =
            run_on_http_thread(|| self.fetch_with_retry(url, &options, || self.client.get(url)))?;
        if !options.allow_forbidden {
            self.write_cache(&result)?;
        }
        Ok(result)
    }

    pub fn get_json<T: DeserializeOwned>(&self, url: &str, options: HttpGetOptions) -> Result<T> {
        self.get(url, options)?.json()
    }

    pub fn get_no_redirect(&self, url: &str, mut options: HttpGetOptions) -> Result<HttpResult> {
        self.apply_defaults(&mut options);
        run_on_http_thread(|| {
            self.fetch_with_retry(url, &options, || self.no_redirect_client.get(url))
        })
    }

    pub fn post_json<T: Serialize>(
        &self,
        url: &str,
        body: &T,
        mut options: HttpGetOptions,
    ) -> Result<HttpResult> {
        self.apply_defaults(&mut options);
        let body = serde_json::to_vec(body)?;
        run_on_http_thread(|| {
            self.fetch_with_retry(url, &options, || {
                self.client
                    .post(url)
                    .header(CONTENT_TYPE, "application/json")
                    .body(body.clone())
            })
        })
    }

    fn fetch_with_retry(
        &self,
        url: &str,
        options: &HttpGetOptions,
        build_request: impl Fn() -> RequestBuilder,
    ) -> Result<HttpResult> {
        let retry = options.retry.as_ref().unwrap_or(&self.default_retry);
        let attempts = retry.attempts.max(1);
        let mut last_server_status = None;
        let mut last_server_body = String::new();

        for attempt in 1..=attempts {
            match self.fetch_once(url, options, build_request()) {
                Ok(result) => return Ok(result),
                Err(CoreError::HttpStatus { status, body, .. }) if should_retry_status(status) => {
                    last_server_status = Some(status);
                    last_server_body = body;
                }
                Err(error @ CoreError::Reqwest(_)) => {
                    if attempt == attempts {
                        return Err(error);
                    }
                }
                Err(error) => return Err(error),
            }

            if attempt < attempts {
                thread::sleep(retry.backoff.saturating_mul(attempt as u32));
            }
        }

        Err(CoreError::HttpStatus {
            status: last_server_status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            url: url.to_string(),
            body: last_server_body,
        })
    }

    fn fetch_once(
        &self,
        url: &str,
        options: &HttpGetOptions,
        request: RequestBuilder,
    ) -> Result<HttpResult> {
        let mut request = request.timeout(options.timeout.unwrap_or(self.default_timeout));
        for (name, value) in &options.headers {
            let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|error| {
                CoreError::InvalidHeader {
                    name: name.clone(),
                    message: error.to_string(),
                }
            })?;
            let header_value =
                HeaderValue::from_str(value).map_err(|error| CoreError::InvalidHeader {
                    name: name.clone(),
                    message: error.to_string(),
                })?;
            request = request.header(header_name, header_value);
        }

        let response = request.send()?;
        let status = response.status();
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
            .to_string();
        let content = response.bytes()?.to_vec();
        if !status.is_success() {
            if options.allow_forbidden && status == StatusCode::FORBIDDEN {
                return Ok(HttpResult {
                    url: url.to_string(),
                    content,
                    from_cache: false,
                    content_type,
                });
            }
            return Err(CoreError::HttpStatus {
                status,
                url: url.to_string(),
                body: truncate_for_error(&String::from_utf8_lossy(&content)),
            });
        }
        Ok(HttpResult {
            url: url.to_string(),
            content,
            from_cache: false,
            content_type,
        })
    }

    /// Read the shared cache without issuing a request. Source-specific rate
    /// limits can then apply only to actual network requests, not cache hits.
    pub fn cached_result(&self, url: &str, ttl_seconds: u64) -> Result<Option<HttpResult>> {
        let cache_path = self.cache_path(url);
        let meta_path = metadata_path(&cache_path);
        if !cache_path.exists() || !meta_path.exists() {
            return Ok(None);
        }
        let age = fs::metadata(&cache_path)?.modified()?.elapsed()?;
        if age > Duration::from_secs(ttl_seconds) {
            return Ok(None);
        }
        let metadata: CacheMetadata = serde_json::from_str(&fs::read_to_string(meta_path)?)?;
        Ok(Some(HttpResult {
            url: url.to_string(),
            content: fs::read(cache_path)?,
            from_cache: true,
            content_type: metadata.content_type,
        }))
    }

    fn write_cache(&self, result: &HttpResult) -> Result<()> {
        let cache_path = self.cache_path(&result.url);
        fs::write(&cache_path, &result.content)?;
        fs::write(
            metadata_path(&cache_path),
            serde_json::to_vec_pretty(&CacheMetadata {
                url: result.url.clone(),
                content_type: result.content_type.clone(),
                fetched_at: crate::now_epoch_seconds()?,
            })?,
        )?;
        Ok(())
    }

    fn cache_path(&self, url: &str) -> PathBuf {
        let digest = hex::encode(Sha256::digest(url.as_bytes()));
        self.cache_dir.join(format!("{digest}.bin"))
    }
}

fn run_on_http_thread<T>(operation: impl FnOnce() -> Result<T> + Send) -> Result<T>
where
    T: Send,
{
    thread::scope(|scope| match scope.spawn(operation).join() {
        Ok(result) => result,
        Err(panic) => std::panic::resume_unwind(panic),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheMetadata {
    url: String,
    content_type: String,
    fetched_at: i64,
}

fn metadata_path(cache_path: &Path) -> PathBuf {
    cache_path.with_extension("bin.json")
}

fn should_retry_status(status: StatusCode) -> bool {
    status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS
}

fn truncate_for_error(value: &str) -> String {
    value.chars().take(1000).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    #[test]
    fn configured_defaults_apply_without_overriding_request_specific_policy() {
        let cache_dir = tempfile::tempdir().unwrap();
        let retry = RetryPolicy {
            attempts: 5,
            backoff: Duration::from_millis(250),
        };
        let client = HttpClient::new_with_defaults(
            "deadlock-brain-core-test",
            cache_dir.path(),
            Duration::from_secs(45),
            retry.clone(),
        )
        .unwrap();

        let mut defaults = HttpGetOptions::default();
        client.apply_defaults(&mut defaults);
        assert_eq!(defaults.timeout, Some(Duration::from_secs(45)));
        assert_eq!(defaults.retry, Some(retry));

        let mut specific = HttpGetOptions {
            timeout: Some(Duration::from_secs(12)),
            retry: Some(RetryPolicy {
                attempts: 1,
                backoff: Duration::ZERO,
            }),
            ..HttpGetOptions::default()
        };
        client.apply_defaults(&mut specific);
        assert_eq!(specific.timeout, Some(Duration::from_secs(12)));
        assert_eq!(specific.retry.as_ref().unwrap().attempts, 1);

        let mut explicit_defaults = HttpGetOptions {
            timeout: Some(Duration::from_secs(30)),
            retry: Some(RetryPolicy::default()),
            ..HttpGetOptions::default()
        };
        client.apply_defaults(&mut explicit_defaults);
        assert_eq!(explicit_defaults.timeout, Some(Duration::from_secs(30)));
        assert_eq!(explicit_defaults.retry, Some(RetryPolicy::default()));
    }

    #[test]
    fn post_json_sends_body_and_reads_response() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/query", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let (headers, body) = read_request(&mut stream);

            assert!(headers.starts_with("POST /query HTTP/1.1\r\n"));
            assert!(headers
                .lines()
                .any(|line| line.eq_ignore_ascii_case("content-type: application/json")));
            assert_eq!(body, br#"{"match_id":92242282,"format":"ndjson"}"#.to_vec());

            let response_body = br#"{"status":"queued"}"#;
            write!(
                stream,
                "HTTP/1.1 202 Accepted\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                response_body.len()
            )
            .unwrap();
            stream.write_all(response_body).unwrap();
        });

        let cache_dir = tempfile::tempdir().unwrap();
        let client = HttpClient::new("deadlock-brain-core-test", cache_dir.path()).unwrap();
        let body = QueryBody {
            match_id: 92242282,
            format: "ndjson",
        };

        let result = client
            .post_json(&url, &body, HttpGetOptions::default())
            .unwrap();

        assert_eq!(
            result.json::<serde_json::Value>().unwrap()["status"],
            "queued"
        );
        server.join().unwrap();
    }

    #[test]
    fn rate_limits_are_retryable_server_responses() {
        assert!(should_retry_status(StatusCode::TOO_MANY_REQUESTS));
    }

    #[test]
    fn allow_forbidden_returns_the_body_on_403() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/feed", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 1024];
            assert!(stream.read(&mut request).unwrap() > 0);
            let body =
                "<feed xmlns=\"http://www.w3.org/2005/Atom\"><title>r/Deadlock</title></feed>";
            write!(
                stream,
                "HTTP/1.1 403 Forbidden\r\nContent-Type: application/atom+xml\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        });
        let cache_dir = tempfile::tempdir().unwrap();
        let client = HttpClient::new("deadlock-brain-core-test", cache_dir.path()).unwrap();
        let result = client
            .get(
                &url,
                HttpGetOptions {
                    allow_forbidden: true,
                    cache_ttl_seconds: None,
                    ..HttpGetOptions::default()
                },
            )
            .unwrap();
        server.join().unwrap();
        assert!(result.text().contains("r/Deadlock"));
        assert!(std::fs::read_dir(cache_dir.path())
            .unwrap()
            .next()
            .is_none());
    }

    #[test]
    fn no_redirect_get_returns_the_redirect_response() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/result", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 1024];
            assert!(stream.read(&mut request).unwrap() > 0);
            write!(
                stream,
                "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/private\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )
            .unwrap();
        });
        let cache_dir = tempfile::tempdir().unwrap();
        let client = HttpClient::new("deadlock-brain-core-test", cache_dir.path()).unwrap();

        let error = client
            .get_no_redirect(&url, HttpGetOptions::default())
            .unwrap_err();

        server.join().unwrap();
        assert!(matches!(
            error,
            CoreError::HttpStatus { status, .. } if status == StatusCode::FOUND
        ));
    }

    #[derive(serde::Serialize)]
    struct QueryBody<'a> {
        match_id: u64,
        format: &'a str,
    }

    fn read_request(stream: &mut std::net::TcpStream) -> (String, Vec<u8>) {
        let mut request = Vec::new();
        let mut buffer = [0_u8; 512];
        let header_end = loop {
            let read = stream.read(&mut buffer).unwrap();
            assert!(read > 0);
            request.extend_from_slice(&buffer[..read]);
            if let Some(index) = request.windows(4).position(|window| window == b"\r\n\r\n") {
                break index + 4;
            }
        };

        let headers = String::from_utf8(request[..header_end].to_vec()).unwrap();
        let content_length = headers
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().unwrap())
            })
            .unwrap();

        while request.len() < header_end + content_length {
            let read = stream.read(&mut buffer).unwrap();
            assert!(read > 0);
            request.extend_from_slice(&buffer[..read]);
        }

        (
            headers,
            request[header_end..header_end + content_length].to_vec(),
        )
    }
}
