use crate::Error;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use brain_contracts::{
    source::origin_from_record, CorpusRelease, CorpusSnapshot, Principal, SnapshotReadPort,
    SourceVisibility,
};
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
    releases: Vec<CorpusRelease>,
    timeout: Duration,
    slots: Arc<Semaphore>,
    reader: LocalPgReader,
    bound_scopes: Vec<(String, String)>,
    operator_socket: Option<std::path::PathBuf>,
}

pub(crate) fn validate_bound_scope(snapshot: &CorpusSnapshot, scope: &str) -> Result<(), Error> {
    if snapshot.revisions.is_empty() {
        return Err(Error::ReleaseUnavailable);
    }
    let public = scope == "docs.public";
    let source = if public {
        "docs-c9-public:Deadlock-Docs"
    } else {
        "second-brain-c9:Deadlock-2nd-Brain"
    };
    for record in snapshot.revisions.iter().chain(&snapshot.heads) {
        let origin = origin_from_record(record).map_err(|_| Error::ReleaseUnavailable)?;
        if record.source_id != source
            || record.tombstone
            || record.allowed_scopes != BTreeSet::from([scope.to_owned()])
            || record.visibility
                != if public {
                    SourceVisibility::Public
                } else {
                    SourceVisibility::Internal
                }
            || origin.policy.allowed_scopes != record.allowed_scopes
            || origin.policy.publication_allowed != public
            || origin.policy.provider_egress_allowed != public
            || origin.policy.raw_retention_allowed
        {
            return Err(Error::ReleaseUnavailable);
        }
    }
    Ok(())
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
    pub(crate) fn new(
        releases: Vec<CorpusRelease>,
        timeout: Duration,
        reader: LocalPgReader,
        bound_scopes: Vec<(String, String)>,
        operator_socket: Option<std::path::PathBuf>,
    ) -> Self {
        Self {
            draining: AtomicBool::new(false),
            releases,
            timeout,
            slots: Arc::new(Semaphore::new(1)),
            reader,
            bound_scopes,
            operator_socket,
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
        let expected = self.releases.clone();
        let bound_scopes = self.bound_scopes.clone();
        let operator_socket = self.operator_socket.clone();
        let result = tokio::time::timeout(
            self.timeout,
            tokio::task::spawn_blocking(move || {
                // Hold the probe slot until the blocking operation truly exits, even if the HTTP
                // probe reaches its deadline. All checks share the request pool.
                let _permit = permit;
                if let Some(path) = operator_socket {
                    use std::os::unix::fs::MetadataExt;
                    let directory = std::fs::symlink_metadata(path.parent().ok_or(Error::Bind)?)
                        .map_err(|_| Error::Bind)?;
                    let socket = std::fs::symlink_metadata(&path).map_err(|_| Error::Bind)?;
                    if directory.mode() & 0o777 != 0o700 || socket.mode() & 0o777 != 0o600 {
                        return Err(Error::ConfigInvalid("operator_socket"));
                    }
                    uplink_infisical_transport::validate_socket(&path, directory.uid())
                        .map_err(|_| Error::Bind)?;
                }
                reader
                    .check_core_schema()
                    .map_err(|_| Error::SchemaIncompatible)?;
                if expected.is_empty() {
                    return Err(Error::ReleaseUnavailable);
                }
                for release in expected {
                    let actual = reader
                        .read_snapshot(&release.release_id)
                        .map_err(|_| Error::ReaderUnavailable)?;
                    if actual.release != release {
                        return Err(Error::ReleaseUnavailable);
                    }
                    validate_snapshot(&actual)?;
                    for (_, scope) in bound_scopes
                        .iter()
                        .filter(|(id, _)| *id == release.release_id)
                    {
                        validate_bound_scope(&actual, scope)?;
                    }
                }
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
                    (
                        StatusCode::OK,
                        [(header::CACHE_CONTROL, "no-store")],
                        axum::Json(serde_json::json!({
                            "status":"ready", "release_id":health.releases[0].release_id,
                            "knowledge_version":health.releases[0].knowledge_version
                        })),
                    )
                        .into_response()
                } else {
                    unavailable()
                }
            }),
        )
        .with_state(health)
}
