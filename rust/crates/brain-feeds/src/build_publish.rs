use brain_contracts::feeds::{
    BuildPublishErrorClass, BuildPublishRequest, BuildPublishState, BuildPublishStatus,
    BUILD_PUBLISH_VERSION,
};
use std::{collections::BTreeMap, io::Read, net::IpAddr, sync::Mutex, time::Duration};
use thiserror::Error;

pub const MAX_STATUS_BYTES: u64 = 64 * 1024;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PublishError {
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("request id reused with a different payload")]
    Conflict,
    #[error("unknown request")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("publish service unavailable: {0}")]
    Unavailable(String),
    #[error("invalid response: {0}")]
    InvalidResponse(String),
    #[error("publish service rate limited")]
    RateLimited,
    #[error("publication remains unconfirmed after deadline")]
    Timeout,
}

pub trait BuildPublishPort: Send + Sync {
    fn submit(&self, request: &BuildPublishRequest) -> Result<BuildPublishStatus, PublishError>;
    fn status(&self, request_id: &str) -> Result<BuildPublishStatus, PublishError>;
}

#[derive(Default)]
pub struct FixtureBuildPublish {
    entries: Mutex<BTreeMap<String, BuildPublishStatus>>,
}

impl FixtureBuildPublish {
    pub fn advance(
        &self,
        request_id: &str,
        state: BuildPublishState,
        at: i64,
    ) -> Result<BuildPublishStatus, PublishError> {
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| PublishError::Unavailable("fixture poisoned".into()))?;
        let entry = entries.get_mut(request_id).ok_or(PublishError::NotFound)?;
        let allowed = matches!(
            (&entry.state, &state),
            (BuildPublishState::Queued, BuildPublishState::Running)
                | (BuildPublishState::Queued, BuildPublishState::Failed { .. })
                | (
                    BuildPublishState::Running,
                    BuildPublishState::Succeeded { .. }
                )
                | (BuildPublishState::Running, BuildPublishState::Failed { .. })
        );
        if !allowed || at < entry.updated_at {
            return Err(PublishError::InvalidRequest(
                "illegal state transition".into(),
            ));
        }
        entry.state = state;
        entry.updated_at = at;
        Ok(entry.clone())
    }
}

impl BuildPublishPort for FixtureBuildPublish {
    fn submit(&self, request: &BuildPublishRequest) -> Result<BuildPublishStatus, PublishError> {
        request.validate().map_err(PublishError::InvalidRequest)?;
        let hash = request
            .request_sha256()
            .map_err(PublishError::InvalidRequest)?;
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| PublishError::Unavailable("fixture poisoned".into()))?;
        if let Some(existing) = entries.get(&request.request_id) {
            return if existing.request_sha256 == hash {
                Ok(existing.clone())
            } else {
                Err(PublishError::Conflict)
            };
        }
        let status = BuildPublishStatus {
            contract_version: BUILD_PUBLISH_VERSION.into(),
            request_id: request.request_id.clone(),
            request_sha256: hash,
            state: BuildPublishState::Queued,
            submitted_at: 1,
            updated_at: 1,
        };
        entries.insert(request.request_id.clone(), status.clone());
        Ok(status)
    }
    fn status(&self, request_id: &str) -> Result<BuildPublishStatus, PublishError> {
        BuildPublishRequest::validate_request_id(request_id)
            .map_err(PublishError::InvalidRequest)?;
        self.entries
            .lock()
            .map_err(|_| PublishError::Unavailable("fixture poisoned".into()))?
            .get(request_id)
            .cloned()
            .ok_or(PublishError::NotFound)
    }
}

pub struct HttpBuildPublishClient {
    publish_url: reqwest::Url,
    token: zeroize::Zeroizing<String>,
    client: reqwest::blocking::Client,
    timeout: Duration,
}

impl HttpBuildPublishClient {
    pub fn new(base_url: &str, token: &str, timeout: Duration) -> Result<Self, PublishError> {
        let invalid_endpoint = || PublishError::InvalidRequest("invalid publish endpoint".into());
        let mut publish_url = reqwest::Url::parse(base_url).map_err(|_| invalid_endpoint())?;
        let loopback = publish_url.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .trim_matches(['[', ']'])
                    .parse::<IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        });
        if !(publish_url.scheme() == "https" || (publish_url.scheme() == "http" && loopback))
            || publish_url.host_str().is_none()
            || !publish_url.username().is_empty()
            || publish_url.password().is_some()
            || publish_url.query().is_some()
            || publish_url.fragment().is_some()
            || publish_url.port() == Some(0)
        {
            return Err(invalid_endpoint());
        }
        publish_url
            .path_segments_mut()
            .map_err(|_| invalid_endpoint())?
            .pop_if_empty()
            .extend(["builds", "v1", "publish"]);
        if token.is_empty()
            || token.len() > 4096
            || !token.bytes().all(|byte| byte.is_ascii_graphic())
        {
            return Err(PublishError::Unauthorized);
        }
        let token = zeroize::Zeroizing::new(token.to_owned());
        let mut client = reqwest::blocking::Client::builder()
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::none());
        if publish_url.scheme() == "http" {
            client = client.no_proxy();
        }
        let client = client
            .build()
            .map_err(|e| PublishError::Unavailable(e.to_string()))?;
        if timeout.is_zero() {
            return Err(PublishError::InvalidRequest(
                "zero transport timeout".into(),
            ));
        }
        Ok(Self {
            publish_url,
            token,
            client,
            timeout,
        })
    }

    pub fn status_for(
        &self,
        request: &BuildPublishRequest,
    ) -> Result<BuildPublishStatus, PublishError> {
        self.status_bound(request, self.timeout)
    }

    fn status_bound(
        &self,
        request: &BuildPublishRequest,
        timeout: Duration,
    ) -> Result<BuildPublishStatus, PublishError> {
        request.validate().map_err(PublishError::InvalidRequest)?;
        let hash = request
            .request_sha256()
            .map_err(PublishError::InvalidRequest)?;
        let mut url = self.publish_url.clone();
        url.path_segments_mut()
            .map_err(|_| PublishError::InvalidRequest("invalid publish endpoint path".into()))?
            .push(&request.request_id);
        url.query_pairs_mut().append_pair("request_sha256", &hash);
        let response = self
            .client
            .get(url)
            .bearer_auth(self.token.as_str())
            .timeout(timeout)
            .send()
            .map_err(transport_error)?;
        self.read(response, &request.request_id, Some(&hash))
    }

    fn submit_bound(
        &self,
        request: &BuildPublishRequest,
        timeout: Duration,
    ) -> Result<BuildPublishStatus, PublishError> {
        request.validate().map_err(PublishError::InvalidRequest)?;
        let hash = request
            .request_sha256()
            .map_err(PublishError::InvalidRequest)?;
        let response = self
            .client
            .post(self.publish_url.clone())
            .bearer_auth(self.token.as_str())
            .header("Idempotency-Key", &request.request_id)
            .json(request)
            .timeout(timeout)
            .send()
            .map_err(transport_error)?;
        self.read(response, &request.request_id, Some(&hash))
    }

    pub fn publish_and_wait(
        &self,
        request: &BuildPublishRequest,
        wait: Duration,
        poll_interval: Duration,
    ) -> Result<BuildPublishStatus, PublishError> {
        self.publish_and_wait_with_clock(request, wait, poll_interval, std::time::Instant::now)
    }

    fn publish_and_wait_with_clock(
        &self,
        request: &BuildPublishRequest,
        wait: Duration,
        poll_interval: Duration,
        now: impl Fn() -> std::time::Instant,
    ) -> Result<BuildPublishStatus, PublishError> {
        if wait.is_zero() || wait > Duration::from_secs(120) {
            return Err(PublishError::InvalidRequest(
                "wait must be between zero and 120 seconds".into(),
            ));
        }
        request.validate().map_err(PublishError::InvalidRequest)?;
        let deadline = now() + wait;
        let rate_limited = std::cell::Cell::new(false);
        let observe = |result: Result<BuildPublishStatus, PublishError>| {
            if matches!(&result, Err(PublishError::RateLimited)) {
                rate_limited.set(true);
            } else if result.is_ok() {
                rate_limited.set(false);
            }
            result
        };
        let remaining = || {
            let remaining = deadline.saturating_duration_since(now());
            if remaining.is_zero() {
                Err(if rate_limited.get() {
                    PublishError::RateLimited
                } else {
                    PublishError::Timeout
                })
            } else {
                Ok(remaining.min(self.timeout))
            }
        };
        let pause = || {
            std::thread::sleep(poll_interval.min(deadline.saturating_duration_since(now())));
        };
        let recover = || {
            let mut attempts = 0;
            loop {
                attempts += 1;
                match observe(self.status_bound(request, remaining()?)) {
                    Err(
                        PublishError::Unavailable(_)
                        | PublishError::Timeout
                        | PublishError::RateLimited,
                    ) if attempts < 3 => pause(),
                    result => return result,
                }
            }
        };
        let mut status = match recover() {
            Ok(status) => status,
            Err(PublishError::NotFound) => {
                let mut attempts = 0;
                loop {
                    attempts += 1;
                    match observe(self.submit_bound(request, remaining()?)) {
                        Ok(status) => break status,
                        Err(
                            error @ (PublishError::Unavailable(_)
                            | PublishError::Timeout
                            | PublishError::RateLimited),
                        ) => match recover() {
                            Ok(status) => break status,
                            Err(PublishError::NotFound) if attempts < 3 => pause(),
                            Err(PublishError::NotFound) => return Err(error),
                            Err(error) => return Err(error),
                        },
                        Err(error) => return Err(error),
                    }
                }
            }
            Err(error) => return Err(error),
        };
        loop {
            if status.state.is_terminal() {
                return Ok(status);
            }
            remaining()?;
            pause();
            match observe(self.status_bound(request, remaining()?)) {
                Ok(next) => status = next,
                Err(
                    PublishError::Unavailable(_)
                    | PublishError::Timeout
                    | PublishError::RateLimited,
                ) => {}
                Err(error) => return Err(error),
            }
        }
    }

    fn read(
        &self,
        response: reqwest::blocking::Response,
        request_id: &str,
        hash: Option<&str>,
    ) -> Result<BuildPublishStatus, PublishError> {
        match response.status().as_u16() {
            200 | 202 => {}
            401 | 403 => return Err(PublishError::Unauthorized),
            404 => return Err(PublishError::NotFound),
            409 => return Err(PublishError::Conflict),
            429 => return Err(PublishError::RateLimited),
            code => return Err(PublishError::Unavailable(format!("HTTP {code}"))),
        }
        let mut body = Vec::new();
        response
            .take(MAX_STATUS_BYTES + 1)
            .read_to_end(&mut body)
            .map_err(transport_error)?;
        if body.len() as u64 > MAX_STATUS_BYTES {
            return Err(PublishError::InvalidResponse(
                "status exceeds byte limit".into(),
            ));
        }
        let status: BuildPublishStatus = serde_json::from_slice(&body)
            .map_err(|_| PublishError::InvalidResponse("invalid status JSON".into()))?;
        status
            .validate_for(request_id, hash)
            .map_err(PublishError::InvalidResponse)?;
        Ok(status)
    }
}

impl BuildPublishPort for HttpBuildPublishClient {
    fn submit(&self, request: &BuildPublishRequest) -> Result<BuildPublishStatus, PublishError> {
        self.submit_bound(request, self.timeout)
    }
    fn status(&self, request_id: &str) -> Result<BuildPublishStatus, PublishError> {
        BuildPublishRequest::validate_request_id(request_id)
            .map_err(PublishError::InvalidRequest)?;
        let mut url = self.publish_url.clone();
        url.path_segments_mut()
            .map_err(|_| PublishError::InvalidRequest("invalid publish endpoint path".into()))?
            .push(request_id);
        let response = self
            .client
            .get(url)
            .bearer_auth(self.token.as_str())
            .send()
            .map_err(transport_error)?;
        self.read(response, request_id, None)
    }
}

fn transport_error(error: impl std::error::Error + 'static) -> PublishError {
    let error = &error as &dyn std::error::Error;
    let timeout = error
        .downcast_ref::<reqwest::Error>()
        .is_some_and(reqwest::Error::is_timeout)
        || error.downcast_ref::<std::io::Error>().is_some_and(|error| {
            error.kind() == std::io::ErrorKind::TimedOut
                || error
                    .get_ref()
                    .and_then(|error| error.downcast_ref::<reqwest::Error>())
                    .is_some_and(reqwest::Error::is_timeout)
        });
    if timeout {
        PublishError::Timeout
    } else {
        PublishError::Unavailable("transport failed".into())
    }
}

pub fn deterministic_request(
    mut request: BuildPublishRequest,
) -> Result<BuildPublishRequest, PublishError> {
    request.request_id = "brain-publish-v1".into();
    request.validate().map_err(PublishError::InvalidRequest)?;
    request.request_id = format!(
        "brain-publish-{}",
        request
            .request_sha256()
            .map_err(PublishError::InvalidRequest)?
    );
    Ok(request)
}

pub fn failed_is_retryable(class: BuildPublishErrorClass) -> bool {
    matches!(
        class,
        BuildPublishErrorClass::SteamUnavailable
            | BuildPublishErrorClass::RateLimited
            | BuildPublishErrorClass::Timeout
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{BufRead, BufReader, Write},
        net::TcpListener,
        thread,
    };

    fn request(id: &str) -> BuildPublishRequest {
        BuildPublishRequest {
            contract_version: BUILD_PUBLISH_VERSION.into(),
            request_id: id.into(),
            hero_id: 25,
            hero_name: "Warden".into(),
            build_name: "Fixture".into(),
            payload: serde_json::json!({"categories": []}),
            caller: "brain-test".into(),
        }
    }

    #[test]
    fn fixture_is_idempotent_rejects_conflicts_and_enforces_transitions() {
        let fixture = FixtureBuildPublish::default();
        let first = fixture.submit(&request("r1")).unwrap();
        assert_eq!(first.state, BuildPublishState::Queued);
        assert_eq!(fixture.submit(&request("r1")).unwrap(), first);
        let mut other = request("r1");
        other.build_name = "Other".into();
        assert_eq!(fixture.submit(&other), Err(PublishError::Conflict));
        assert!(fixture
            .advance("r1", BuildPublishState::Succeeded { hero_build_id: 9 }, 2)
            .is_err());
        fixture
            .advance("r1", BuildPublishState::Running, 2)
            .unwrap();
        let done = fixture
            .advance("r1", BuildPublishState::Succeeded { hero_build_id: 9 }, 3)
            .unwrap();
        assert!(done.state.is_terminal());
        assert!(fixture
            .advance(
                "r1",
                BuildPublishState::Failed {
                    error_class: BuildPublishErrorClass::Internal
                },
                4
            )
            .is_err());
        assert_eq!(fixture.status("missing"), Err(PublishError::NotFound));
        assert!(failed_is_retryable(BuildPublishErrorClass::RateLimited));
        assert!(!failed_is_retryable(BuildPublishErrorClass::Rejected));
    }

    fn serve(responses: Vec<(u16, String)>) -> (String, thread::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap());
        let handle = thread::spawn(move || {
            let mut seen = Vec::new();
            for (code, body) in responses {
                let (mut stream, _) = listener.accept().unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut head = String::new();
                let mut length = 0usize;
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line.to_ascii_lowercase().starts_with("content-length:") {
                        length = line[15..].trim().parse().unwrap();
                    }
                    if line == "\r\n" {
                        break;
                    }
                    head.push_str(&line);
                }
                let mut payload = vec![0; length];
                std::io::Read::read_exact(&mut reader, &mut payload).unwrap();
                seen.push(head);
                write!(
                    stream,
                    "HTTP/1.1 {code} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                )
                .unwrap();
            }
            seen
        });
        (address, handle)
    }

    #[test]
    fn status_parse_errors_do_not_expose_response_contents() {
        const PRIVATE_MARKER: &str = "synthetic-private-content";
        let req = request("redacted-response");
        let body = format!(r#"{{"{PRIVATE_MARKER}":true}}"#);
        let (address, server) = serve(vec![(202, body.clone()), (200, body.clone()), (200, body)]);
        let client =
            HttpBuildPublishClient::new(&address, "fixture-token", Duration::from_secs(5)).unwrap();
        for result in [
            client.submit(&req),
            client.status_for(&req),
            client.status(&req.request_id),
        ] {
            let error = result.unwrap_err();
            assert!(matches!(&error, PublishError::InvalidResponse(_)));
            assert!(!error.to_string().contains(PRIVATE_MARKER));
            assert!(!format!("{error:?}").contains(PRIVATE_MARKER));
        }
        assert_eq!(server.join().unwrap().len(), 3);
    }

    #[test]
    fn body_transport_errors_preserve_timeouts_without_raw_details() {
        assert_eq!(
            transport_error(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "synthetic body timeout"
            )),
            PublishError::Timeout
        );
        assert_eq!(
            transport_error(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "synthetic body failure"
            )),
            PublishError::Unavailable("transport failed".into())
        );
    }

    #[test]
    fn confirmed_terminal_status_survives_an_elapsed_deadline() {
        let req = request("terminal-deadline");
        for state in [
            BuildPublishState::Succeeded { hero_build_id: 456 },
            BuildPublishState::Failed {
                error_class: BuildPublishErrorClass::Rejected,
            },
        ] {
            let status = BuildPublishStatus {
                contract_version: BUILD_PUBLISH_VERSION.into(),
                request_id: req.request_id.clone(),
                request_sha256: req.request_sha256().unwrap(),
                state,
                submitted_at: 1,
                updated_at: 2,
            };
            let terminal = serde_json::to_string(&status).unwrap();
            let queued = serde_json::to_string(&BuildPublishStatus {
                state: BuildPublishState::Queued,
                ..status.clone()
            })
            .unwrap();
            for (replies, calls_before_deadline) in [
                (vec![(200, terminal.clone())], 2),
                (vec![(404, "{}".into()), (202, terminal.clone())], 3),
                (vec![(200, queued), (200, terminal.clone())], 5),
            ] {
                let (address, server) = serve(replies);
                let client =
                    HttpBuildPublishClient::new(&address, "fixture-token", Duration::from_secs(5))
                        .unwrap();
                let start = std::time::Instant::now();
                let wait = Duration::from_secs(1);
                let calls = std::cell::Cell::new(0);
                let now = || {
                    calls.set(calls.get() + 1);
                    if calls.get() > calls_before_deadline {
                        start + wait
                    } else {
                        start
                    }
                };
                assert_eq!(
                    client.publish_and_wait_with_clock(&req, wait, Duration::ZERO, now),
                    Ok(status.clone())
                );
                assert_eq!(now(), start + wait);
                server.join().unwrap();
            }
        }
    }

    #[test]
    fn repeated_rate_limits_remain_visible_at_the_deadline() {
        let req = request("rate-limit-deadline");
        let queued = serde_json::to_string(&BuildPublishStatus {
            contract_version: BUILD_PUBLISH_VERSION.into(),
            request_id: req.request_id.clone(),
            request_sha256: req.request_sha256().unwrap(),
            state: BuildPublishState::Queued,
            submitted_at: 1,
            updated_at: 1,
        })
        .unwrap();
        for (replies, calls_before_deadline, expected) in [
            (
                vec![
                    (200, queued.clone()),
                    (429, "{}".into()),
                    (429, "{}".into()),
                ],
                8,
                PublishError::RateLimited,
            ),
            (
                vec![(429, "{}".into()), (429, "{}".into())],
                5,
                PublishError::RateLimited,
            ),
            (
                vec![
                    (200, queued.clone()),
                    (429, "{}".into()),
                    (200, queued.clone()),
                ],
                8,
                PublishError::Timeout,
            ),
            (
                vec![(200, queued), (503, "{}".into()), (503, "{}".into())],
                8,
                PublishError::Timeout,
            ),
        ] {
            let expected_requests = replies.len();
            let (address, server) = serve(replies);
            let client =
                HttpBuildPublishClient::new(&address, "fixture-token", Duration::from_secs(5))
                    .unwrap();
            let start = std::time::Instant::now();
            let wait = Duration::from_secs(1);
            let calls = std::cell::Cell::new(0);
            let now = || {
                calls.set(calls.get() + 1);
                if calls.get() > calls_before_deadline {
                    start + wait
                } else {
                    start
                }
            };
            assert_eq!(
                client.publish_and_wait_with_clock(&req, wait, Duration::ZERO, now),
                Err(expected)
            );
            assert_eq!(server.join().unwrap().len(), expected_requests);
        }
    }

    #[test]
    fn http_client_sends_bearer_and_idempotency_key_and_validates_status() {
        let req = request("r2");
        let hash = req.request_sha256().unwrap();
        let ok = serde_json::to_string(&BuildPublishStatus {
            contract_version: BUILD_PUBLISH_VERSION.into(),
            request_id: "r2".into(),
            request_sha256: hash.clone(),
            state: BuildPublishState::Queued,
            submitted_at: 5,
            updated_at: 5,
        })
        .unwrap();
        let foreign = ok.replace("\"r2\"", "\"other\"");
        let (address, handle) = serve(vec![
            (202, ok),
            (200, foreign),
            (409, "{}".into()),
            (401, "{}".into()),
        ]);
        let client =
            HttpBuildPublishClient::new(&address, "fixture-token", Duration::from_secs(5)).unwrap();
        assert_eq!(
            client.submit(&req).unwrap().state,
            BuildPublishState::Queued
        );
        assert!(matches!(
            client.status("r2"),
            Err(PublishError::InvalidResponse(_))
        ));
        assert_eq!(client.submit(&req), Err(PublishError::Conflict));
        assert_eq!(client.status("r2"), Err(PublishError::Unauthorized));
        let seen = handle.join().unwrap();
        assert!(seen[0].starts_with("POST /builds/v1/publish "));
        assert!(seen[0].contains("authorization: Bearer fixture-token"));
        assert!(seen[0].contains("idempotency-key: r2"));
        assert!(seen[1].starts_with("GET /builds/v1/publish/r2 "));
        assert!(HttpBuildPublishClient::new(
            "http://example.invalid",
            "fixture-token",
            Duration::from_secs(1)
        )
        .is_err());
        assert!(client.status("../steam").is_err());
    }
}
