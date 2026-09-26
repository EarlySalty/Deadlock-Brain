use crate::Error;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use brain_contracts::{CorpusRelease, CorpusSnapshot, Principal, SnapshotReadPort};
use brain_storage::{LocalPgReader, PgStore};
use sqlx::PgPool;
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
    store: PgStore,
    pool: PgPool,
    release: CorpusRelease,
    timeout: Duration,
    slots: Arc<Semaphore>,
    reader: LocalPgReader,
}

pub(crate) fn validate_snapshot(snapshot: &CorpusSnapshot) -> Result<(), Error> {
    // authorized() validates all pinned records/current heads before applying this empty grant.
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

pub(crate) async fn permissions(pool: &PgPool) -> Result<(), Error> {
    let allowed: bool = sqlx::query_scalar(
        "SELECT has_table_privilege(current_user, 'brain.conversation_owners_v1', 'SELECT')
            AND has_table_privilege(current_user, 'brain.conversation_owners_v1', 'INSERT')",
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::DatabasePermissions)?;
    if allowed {
        Ok(())
    } else {
        Err(Error::DatabasePermissions)
    }
}

impl Health {
    pub(crate) fn new(
        store: PgStore,
        pool: PgPool,
        release: CorpusRelease,
        timeout: Duration,
        reader: LocalPgReader,
    ) -> Self {
        Self {
            draining: AtomicBool::new(false),
            store,
            pool,
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
        let result = tokio::time::timeout(self.timeout, async {
            let snapshot = self
                .store
                .snapshot(&self.release.release_id)
                .await
                .map_err(|_| Error::ReleaseUnavailable)?;
            if snapshot.release != self.release {
                return Err(Error::ReleaseUnavailable);
            }
            validate_snapshot(&snapshot)?;
            permissions(&self.pool).await?;
            let reader = self.reader.clone();
            let expected = self.release.clone();
            // Hold the permit inside the blocking job even if the HTTP probe times out.
            tokio::task::spawn_blocking(move || {
                let _permit = permit;
                let actual = reader
                    .read_snapshot(&expected.release_id)
                    .map_err(|_| Error::ReaderUnavailable)?;
                if actual.release != expected {
                    return Err(Error::ReleaseUnavailable);
                }
                validate_snapshot(&actual)
            })
            .await
            .map_err(|_| Error::ReaderUnavailable)?
        })
        .await;
        matches!(result, Ok(Ok(()))) && !self.draining.load(Ordering::SeqCst)
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
