use brain_contracts::feeds::{
    BuildPublishErrorClass, BuildPublishRequest, BuildPublishState, BuildPublishStatus,
    BUILD_PUBLISH_VERSION,
};
use std::{collections::BTreeMap, io::Read, sync::Mutex, time::Duration};
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
        self.entries
            .lock()
            .map_err(|_| PublishError::Unavailable("fixture poisoned".into()))?
            .get(request_id)
            .cloned()
            .ok_or(PublishError::NotFound)
    }
}

pub struct HttpBuildPublishClient {
    base_url: String,
    token: String,
    client: reqwest::blocking::Client,
}

impl HttpBuildPublishClient {
    pub fn new(base_url: &str, token_env: &str, timeout: Duration) -> Result<Self, PublishError> {
        let loopback = base_url.starts_with("http://127.0.0.1:")
            || base_url.starts_with("http://[::1]:")
            || base_url.starts_with("http://localhost:");
        if !(loopback || base_url.starts_with("https://")) || base_url.ends_with('/') {
            return Err(PublishError::InvalidRequest(
                "publish endpoint must be https or loopback".into(),
            ));
        }
        let token = std::env::var(token_env)
            .ok()
            .filter(|t| !t.trim().is_empty())
            .ok_or(PublishError::Unauthorized)?;
        let client = reqwest::blocking::Client::builder()
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| PublishError::Unavailable(e.to_string()))?;
        Ok(Self {
            base_url: base_url.into(),
            token,
            client,
        })
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
            code => return Err(PublishError::Unavailable(format!("HTTP {code}"))),
        }
        let mut body = Vec::new();
        response
            .take(MAX_STATUS_BYTES + 1)
            .read_to_end(&mut body)
            .map_err(|e| PublishError::Unavailable(e.to_string()))?;
        if body.len() as u64 > MAX_STATUS_BYTES {
            return Err(PublishError::InvalidResponse(
                "status exceeds byte limit".into(),
            ));
        }
        let status: BuildPublishStatus = serde_json::from_slice(&body)
            .map_err(|e| PublishError::InvalidResponse(e.to_string()))?;
        status
            .validate_for(request_id, hash)
            .map_err(PublishError::InvalidResponse)?;
        Ok(status)
    }
}

impl BuildPublishPort for HttpBuildPublishClient {
    fn submit(&self, request: &BuildPublishRequest) -> Result<BuildPublishStatus, PublishError> {
        request.validate().map_err(PublishError::InvalidRequest)?;
        let hash = request
            .request_sha256()
            .map_err(PublishError::InvalidRequest)?;
        let response = self
            .client
            .post(format!("{}/builds/v1/publish", self.base_url))
            .bearer_auth(&self.token)
            .header("Idempotency-Key", &request.request_id)
            .json(request)
            .send()
            .map_err(|e| PublishError::Unavailable(e.to_string()))?;
        self.read(response, &request.request_id, Some(&hash))
    }
    fn status(&self, request_id: &str) -> Result<BuildPublishStatus, PublishError> {
        if request_id.is_empty()
            || !request_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        {
            return Err(PublishError::InvalidRequest("invalid request id".into()));
        }
        let response = self
            .client
            .get(format!("{}/builds/v1/publish/{request_id}", self.base_url))
            .bearer_auth(&self.token)
            .send()
            .map_err(|e| PublishError::Unavailable(e.to_string()))?;
        self.read(response, request_id, None)
    }
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
        std::env::set_var("BRAIN_FEEDS_TEST_PUBLISH_TOKEN", "fixture-token");
        let client = HttpBuildPublishClient::new(
            &address,
            "BRAIN_FEEDS_TEST_PUBLISH_TOKEN",
            Duration::from_secs(5),
        )
        .unwrap();
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
            "BRAIN_FEEDS_TEST_PUBLISH_TOKEN",
            Duration::from_secs(1)
        )
        .is_err());
        assert!(client.status("../steam").is_err());
    }
}
