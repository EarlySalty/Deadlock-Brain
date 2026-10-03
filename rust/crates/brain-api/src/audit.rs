use crate::ApiResponse;
use brain_contracts::Principal;
use sha2::{Digest, Sha256};
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

pub const TARGET: &str = "brain_api::redacted_request";

#[derive(Clone, Copy)]
pub(crate) enum Route {
    Answer,
    Retrieve,
    OperatorQuery,
}

impl Route {
    fn path(self) -> &'static str {
        match self {
            Self::Answer => "/v1/answer",
            Self::Retrieve => "/v1/retrieve",
            Self::OperatorQuery => "/v1/operator/query",
        }
    }
}

pub(crate) struct RequestAudit {
    consumer_id: String,
    request_id: String,
    route: Route,
    started: Instant,
    status: u16,
    outcome: &'static str,
}

impl RequestAudit {
    pub(crate) fn new(principal: &Principal, route: Route, started: Instant) -> Self {
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let consumer_id = match (principal.actor_id.as_str(), principal.channel.as_str()) {
            ("twitch-bot", "twitch") => "twitch-bot".into(),
            ("docs-client", "docs") => "docs-client".into(),
            ("second-brain", "internal") => "second-brain".into(),
            _ => {
                let mut digest = Sha256::new();
                digest.update((principal.actor_id.len() as u64).to_be_bytes());
                digest.update(principal.actor_id.as_bytes());
                digest.update(principal.channel.as_bytes());
                format!("grant-sha256:{:x}", digest.finalize())
            }
        };
        let mut digest = Sha256::new();
        digest.update(std::process::id().to_be_bytes());
        digest.update(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
                .to_be_bytes(),
        );
        digest.update(SEQUENCE.fetch_add(1, Ordering::Relaxed).to_be_bytes());
        Self {
            consumer_id,
            request_id: format!("server-sha256:{:x}", digest.finalize()),
            route,
            started,
            status: 499,
            outcome: "cancelled",
        }
    }

    pub(crate) fn observe_body(&mut self, body: &[u8]) {
        if let Ok(value) = serde_json::from_slice::<serde_json::Value>(body) {
            if let Some(request_id) = value.get("request_id").and_then(|value| value.as_str()) {
                self.request_id =
                    format!("client-sha256:{:x}", Sha256::digest(request_id.as_bytes()));
            }
        }
    }

    pub(crate) fn finish(mut self, response: ApiResponse) -> ApiResponse {
        self.status = response.status;
        self.outcome = match response.status {
            200 => {
                let value = serde_json::from_str::<serde_json::Value>(&response.body).ok();
                match value
                    .as_ref()
                    .and_then(|value| value.get("status"))
                    .and_then(|value| value.as_str())
                {
                    Some("answered") => "answered",
                    Some("build_rejected") => "build_rejected",
                    Some("insufficient_evidence") => "insufficient_evidence",
                    Some("unauthorized_evidence") => "unauthorized_evidence",
                    Some("unavailable") => "unavailable",
                    Some("provider_error") => "provider_error",
                    Some("budget_exceeded") => "budget_exceeded",
                    _ => "completed",
                }
            }
            400 => "invalid_request",
            401 => "unauthorized",
            403 => "forbidden",
            413 => "payload_too_large",
            415 => "unsupported_media_type",
            429 => "overloaded",
            500 => "internal_error",
            502 => "invalid_response",
            503 => "unavailable",
            504 => "deadline_exceeded",
            _ => "failed",
        };
        response
    }
}

impl Drop for RequestAudit {
    fn drop(&mut self) {
        let duration_ms = self.started.elapsed().as_millis().min(u64::MAX as u128) as u64;
        tracing::event!(
            target: "brain_api::redacted_request",
            tracing::Level::INFO,
            event = "authenticated_request",
            consumer_id = self.consumer_id.as_str(),
            request_id = self.request_id.as_str(),
            route = self.route.path(),
            status = self.status,
            outcome = self.outcome,
            duration_ms = duration_ms,
        );
    }
}
