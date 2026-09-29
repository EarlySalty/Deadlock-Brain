use crate::{config, Error};
use axum::{
    extract::{DefaultBodyLimit, State},
    http::{header, HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use brain_policy::CredentialRegistry;
use dbrain_sources::{AnalyticsLookupRequest, DeadlockAnalyticsClient};
use serde_json::{json, Value};
use std::{sync::Arc, time::Duration};
use tokio::sync::Semaphore;

pub(crate) struct AnalyticsRuntime {
    client: DeadlockAnalyticsClient,
    config: config::Analytics,
    credentials: CredentialRegistry,
    slots: Arc<Semaphore>,
    _scratch: tempfile::TempDir,
}

impl AnalyticsRuntime {
    pub(crate) fn new(
        config: config::Analytics,
        credentials: CredentialRegistry,
    ) -> Result<Self, Error> {
        let scratch = tempfile::tempdir().map_err(|_| Error::ConfigInvalid("analytics_scratch"))?;
        let http =
            dbrain_sources::core::http::HttpClient::new("brain-serve-analytics/1", scratch.path())
                .map_err(|_| Error::ConfigInvalid("analytics_http"))?;
        let client =
            DeadlockAnalyticsClient::new(http, Duration::from_millis(config.request_timeout_ms))
                .map_err(|_| Error::ConfigInvalid("analytics_http"))?;
        Ok(Self {
            client,
            config,
            credentials,
            slots: Arc::new(Semaphore::new(4)),
            _scratch: scratch,
        })
    }

    fn authorize(&self, headers: &HeaderMap, request: &AnalyticsLookupRequest) -> StatusCode {
        let Some(token) = headers
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
        else {
            return StatusCode::UNAUTHORIZED;
        };
        let Ok(principal) = self.credentials.authenticate(token) else {
            return StatusCode::UNAUTHORIZED;
        };
        if !principal.scopes.contains("analytics.internal") {
            return StatusCode::FORBIDDEN;
        }
        if request.patch != self.config.patch
            || request.min_unix_timestamp < self.config.min_unix_timestamp
            || request.max_unix_timestamp > self.config.max_unix_timestamp
            || request.max_rows > self.config.max_rows
            || request.validate().is_err()
        {
            return StatusCode::BAD_REQUEST;
        }
        StatusCode::OK
    }

    pub(crate) fn router(self: Arc<Self>) -> Router {
        Router::new()
            .route("/v1/analytics/observation", post(lookup))
            .layer(DefaultBodyLimit::max(1024))
            .with_state(self)
    }
}

async fn lookup(
    State(runtime): State<Arc<AnalyticsRuntime>>,
    headers: HeaderMap,
    Json(request): Json<AnalyticsLookupRequest>,
) -> (StatusCode, Json<Value>) {
    let authorization = runtime.authorize(&headers, &request);
    if authorization != StatusCode::OK {
        return (
            authorization,
            Json(json!({"error": "analytics_request_rejected"})),
        );
    }
    let wait = Duration::from_millis(runtime.config.request_timeout_ms.saturating_mul(2));
    let Ok(Ok(permit)) = tokio::time::timeout(wait, runtime.slots.clone().acquire_owned()).await
    else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "analytics_busy"})),
        );
    };
    let client = runtime.client.clone();
    let task = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        client.lookup(&request)
    });
    match tokio::time::timeout(wait + Duration::from_millis(250), task).await {
        Ok(Ok(Ok(observation))) => (StatusCode::OK, Json(json!(observation))),
        _ => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "analytics_unavailable"})),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_policy::AuthGrant;
    use dbrain_sources::AnalyticsKind;
    use std::collections::BTreeSet;

    fn runtime() -> AnalyticsRuntime {
        AnalyticsRuntime::new(
            config::Analytics {
                patch: "2026-09-24".into(),
                min_unix_timestamp: 1_790_000_000,
                max_unix_timestamp: 1_790_086_400,
                max_rows: 8,
                request_timeout_ms: 100,
                schema_sha256: dbrain_sources::schema_watch::OpenApiSnapshot::pinned()
                    .unwrap()
                    .schema_sha256,
            },
            CredentialRegistry::new(vec![AuthGrant::from_secret(
                "fixture-secret",
                "fixture-actor",
                "fixture-channel",
                BTreeSet::from(["analytics.internal".into()]),
                BTreeSet::new(),
            )]),
        )
        .unwrap()
    }

    #[test]
    fn runtime_rejects_missing_grant_wrong_patch_and_unapproved_window() {
        let runtime = runtime();
        let mut request = AnalyticsLookupRequest {
            kind: AnalyticsKind::Population,
            hero_id: 18,
            patch: "2026-09-24".into(),
            min_unix_timestamp: 1_790_000_000,
            max_unix_timestamp: 1_790_086_400,
            max_rows: 8,
        };
        assert_eq!(
            runtime.authorize(&HeaderMap::new(), &request),
            StatusCode::UNAUTHORIZED
        );
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            "Bearer fixture-secret".parse().unwrap(),
        );
        assert_eq!(runtime.authorize(&headers, &request), StatusCode::OK);
        request.patch = "wrong-patch".into();
        assert_eq!(
            runtime.authorize(&headers, &request),
            StatusCode::BAD_REQUEST
        );
        request.patch = "2026-09-24".into();
        request.min_unix_timestamp -= 1;
        assert_eq!(
            runtime.authorize(&headers, &request),
            StatusCode::BAD_REQUEST
        );
        request.min_unix_timestamp += 1;
        request.max_rows = 9;
        assert_eq!(
            runtime.authorize(&headers, &request),
            StatusCode::BAD_REQUEST
        );
    }
}
