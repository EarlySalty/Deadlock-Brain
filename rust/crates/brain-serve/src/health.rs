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
    if scope == "bot.public" {
        let scopes = BTreeSet::from(["bot.public".into()]);
        if snapshot
            .revisions
            .iter()
            .chain(&snapshot.heads)
            .any(|record| {
                record.visibility != SourceVisibility::Public
                    || record.allowed_scopes != scopes
                    || record.tombstone
            })
        {
            return Err(Error::ReleaseUnavailable);
        }
        let published = snapshot
            .authorized_for_publication(&Principal {
                actor_id: "brain-serve-readiness".into(),
                channel: "health".into(),
                scopes,
                provider_egress: BTreeSet::new(),
            })
            .map_err(|_| Error::ReleaseUnavailable)?;
        return if published.len() == snapshot.revisions.len() {
            Ok(())
        } else {
            Err(Error::ReleaseUnavailable)
        };
    }
    let public = scope == "docs.public";
    let source = if public {
        "docs-c9-public:Deadlock-Docs"
    } else {
        "second-brain-c9:Deadlock-2nd-Brain"
    };
    for record in snapshot.revisions.iter().chain(&snapshot.heads) {
        let origin = origin_from_record(record).map_err(|_| Error::ReleaseUnavailable)?;
        let internal_feed = !public
            && record
                .source_id
                .strip_prefix("google-sheet/")
                .or_else(|| record.source_id.strip_prefix("youtube-core/"))
                .is_some_and(|id| {
                    !id.is_empty()
                        && id
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
                });
        if (record.source_id != source && !internal_feed)
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
            || origin.policy.raw_retention_allowed != internal_feed
            || (internal_feed
                && !matches!(&origin.policy.authorization_ref,
                brain_contracts::value::Observed::Known { value } if !value.trim().is_empty()))
            || (internal_feed
                && !matches!(
                    &origin.policy.license,
                    brain_contracts::value::Observed::Unknown {
                        reason: brain_contracts::value::UnknownReason::NotPresent
                    }
                ))
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

    fn bindings_sha256(&self) -> String {
        let Some(standard) = self.releases.first() else {
            return String::new();
        };
        let releases = self
            .releases
            .iter()
            .map(|release| {
                (
                    release.release_id.clone(),
                    release.knowledge_version.clone(),
                )
            })
            .collect();
        let bound_scopes = self.bound_scopes.iter().cloned().collect();
        crate::config::loaded_release_bindings_sha256(
            (&standard.release_id, &standard.knowledge_version),
            &releases,
            &bound_scopes,
            self.operator_socket.is_some(),
        )
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
                            "knowledge_version":health.releases[0].knowledge_version,
                            "release_bindings_sha256":health.bindings_sha256()
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

#[cfg(test)]
mod binding_tests {
    use super::*;
    use brain_contracts::SourceRecordV2;
    use std::collections::BTreeMap;

    fn maintenance_snapshot() -> CorpusSnapshot {
        let record = SourceRecordV2 {
            source_id: "maintenance-public".into(),
            logical_id: "public-document".into(),
            revision: 1,
            content_hash: "a".repeat(64),
            content: "Öffentlicher Betriebsbestand".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["bot.public".into()]),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        };
        CorpusSnapshot {
            release: CorpusRelease {
                release_id: "maintenance-fixture".into(),
                knowledge_version: "maintenance-version".into(),
                patch: "maintenance-patch".into(),
                created_at_epoch: 1,
                source_revisions: BTreeMap::from([(
                    record.source_id.clone(),
                    BTreeMap::from([(record.logical_id.clone(), 1)]),
                )]),
            },
            revisions: vec![record.clone()],
            heads: vec![record],
        }
    }

    #[test]
    fn maintenance_binding_checks_public_scope_and_current_head_publication() {
        let snapshot = maintenance_snapshot();
        assert!(validate_bound_scope(&snapshot, "bot.public").is_ok());
        for visibility in [SourceVisibility::Internal, SourceVisibility::Private] {
            let mut changed = snapshot.clone();
            changed.heads[0].visibility = visibility;
            assert!(validate_bound_scope(&changed, "bot.public").is_err());
        }
        let mut changed = snapshot.clone();
        changed.heads[0].allowed_scopes = BTreeSet::from(["docs.public".into()]);
        assert!(validate_bound_scope(&changed, "bot.public").is_err());
        let mut changed = snapshot.clone();
        changed.heads[0].tombstone = true;
        assert!(validate_bound_scope(&changed, "bot.public").is_err());
        let mut changed = snapshot;
        changed.heads[0]
            .metadata
            .insert("brain.origin".into(), "{}".into());
        assert!(validate_bound_scope(&changed, "bot.public").is_err());
    }
}
