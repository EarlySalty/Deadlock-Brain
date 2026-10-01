use crate::Error;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use brain_contracts::{CorpusRelease, CorpusSnapshot, Principal, SnapshotReadPort};
use brain_storage::LocalPgReader;
use std::{
    collections::BTreeSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::Semaphore;

pub(crate) struct Health {
    pub draining: AtomicBool,
    release: CorpusRelease,
    timeout: Duration,
    slots: Arc<Semaphore>,
    reader: LocalPgReader,
}

pub(crate) fn validate_snapshot(snapshot: &CorpusSnapshot) -> Result<(), Error> {
    snapshot
        .authorized(
            &Principal {
                actor_id: "brain-serve-readiness".into(),
                channel: "health".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::new(),
            },
            false,
        )
        .map_err(|_| Error::ReleaseUnavailable)?;
    Ok(())
}

impl Health {
    pub(crate) fn new(release: CorpusRelease, timeout: Duration, reader: LocalPgReader) -> Self {
        Self {
            draining: AtomicBool::new(false),
            release,
            timeout,
            slots: Arc::new(Semaphore::new(1)),
            reader,
        }
    }

    async fn ready(&self) -> bool {
        if self.draining.load(Ordering::SeqCst) {
            return false;
        }
        let Ok(permit) = self.slots.clone().try_acquire_owned() else {
            return false;
        };
        let reader = self.reader.clone();
        let expected = self.release.clone();
        let result = tokio::time::timeout(
            self.timeout,
            tokio::task::spawn_blocking(move || {
                // Hold the probe slot until the blocking operation truly exits, even if the HTTP
                // probe reaches its deadline. All checks share the request pool.
                let _permit = permit;
                let actual = reader
                    .read_snapshot(&expected.release_id)
                    .map_err(|_| Error::ReaderUnavailable)?;
                if actual.release != expected {
                    return Err(Error::ReleaseUnavailable);
                }
                validate_snapshot(&actual)?;
                reader
                    .check_permissions()
                    .map_err(|_| Error::DatabasePermissions)
            }),
        )
        .await;
        matches!(result, Ok(Ok(Ok(())))) && !self.draining.load(Ordering::SeqCst)
    }
}

pub(crate) fn status(code: StatusCode, body: &'static str) -> Response {
    (
        code,
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::CACHE_CONTROL, "no-store"),
        ],
        body,
    )
        .into_response()
}

pub(crate) fn unavailable() -> Response {
    status(StatusCode::SERVICE_UNAVAILABLE, r#"{"status":"not_ready"}"#)
}

pub(crate) fn router(health: Arc<Health>) -> Router {
    Router::new()
        .route(
            "/healthz",
            get(|| async { status(StatusCode::OK, r#"{"status":"ok"}"#) }),
        )
        .route(
            "/readyz",
            get(|State(health): State<Arc<Health>>| async move {
                if health.ready().await {
                    status(StatusCode::OK, r#"{"status":"ready"}"#)
                } else {
                    unavailable()
                }
            }),
        )
        .with_state(health)
}
