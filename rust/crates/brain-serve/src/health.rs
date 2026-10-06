use crate::Error;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use brain_contracts::{CorpusRelease, Principal, ReleaseReadManifest, SourceVisibility};
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

pub(crate) fn validate_bound_manifest(
    snapshot: &ReleaseReadManifest,
    scope: &str,
) -> Result<(), Error> {
    if snapshot.revisions.is_empty() {
        return Err(Error::ReleaseUnavailable);
    }
    if scope == "bot.public" {
        let scopes = BTreeSet::from(["bot.public".into()]);
        let public: Vec<_> = snapshot
            .revisions
            .iter()
            .map(|descriptor| &descriptor.head)
            .filter(|record| {
                record.visibility == SourceVisibility::Public && record.allowed_scopes == scopes
            })
            .collect();
        if public.is_empty()
            || public.iter().any(|record| {
                record.tombstone
                    || !snapshot.heads.iter().any(|head| {
                        head.source_id == record.source_id
                            && head.logical_id == record.logical_id
                            && head.visibility == SourceVisibility::Public
                            && head.allowed_scopes == scopes
                            && !head.tombstone
                    })
            })
        {
            return Err(Error::ReleaseUnavailable);
        }
        let published = snapshot
            .authorized(
                &Principal {
                    actor_id: "brain-serve-readiness".into(),
                    channel: "health".into(),
                    scopes: scopes.clone(),
                    provider_egress: BTreeSet::new(),
                },
                false,
                true,
            )
            .map_err(|_| Error::ReleaseUnavailable)?;
        return if published.len() == public.len() {
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
    for record in snapshot
        .revisions
        .iter()
        .map(|descriptor| &descriptor.head)
        .chain(&snapshot.heads)
    {
        let origin = record
            .canonical_origin()
            .map_err(|_| Error::ReleaseUnavailable)?
            .ok_or(Error::ReleaseUnavailable)?;
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

pub(crate) fn validate_manifest(snapshot: &ReleaseReadManifest) -> Result<(), Error> {
    snapshot
        .authorized(
            &Principal {
                actor_id: "brain-serve-readiness".into(),
                channel: "health".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::new(),
            },
            false,
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
                        .read_manifest(&release.release_id)
                        .map_err(|_| Error::ReaderUnavailable)?;
                    if actual.release != release {
                        return Err(Error::ReleaseUnavailable);
                    }
                    validate_manifest(&actual)?;
                    for (_, scope) in bound_scopes
                        .iter()
                        .filter(|(id, _)| *id == release.release_id)
                    {
                        validate_bound_manifest(&actual, scope)?;
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
    use brain_contracts::{CorpusSnapshot, SourceRecordV2};
    use std::collections::BTreeMap;

    fn validate_bound_scope(snapshot: &CorpusSnapshot, scope: &str) -> Result<(), Error> {
        validate_bound_manifest(&ReleaseReadManifest::from_snapshot(snapshot), scope)
    }

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

    #[test]
    fn maintenance_binding_keeps_internal_documents_outside_the_public_view() {
        let mut snapshot = maintenance_snapshot();
        let mut internal = snapshot.revisions[0].clone();
        internal.source_id = "maintenance-internal".into();
        internal.logical_id = "internal-document".into();
        internal.visibility = SourceVisibility::Internal;
        internal.allowed_scopes = BTreeSet::from(["internal_docs".into()]);
        snapshot.release.source_revisions.insert(
            internal.source_id.clone(),
            BTreeMap::from([(internal.logical_id.clone(), 1)]),
        );
        snapshot.revisions.push(internal.clone());
        snapshot.heads.push(internal);
        assert!(validate_bound_scope(&snapshot, "bot.public").is_ok());
        let published = snapshot
            .authorized_for_publication(&Principal {
                actor_id: "brain-serve-readiness".into(),
                channel: "health".into(),
                scopes: BTreeSet::from(["bot.public".into()]),
                provider_egress: BTreeSet::new(),
            })
            .unwrap();
        assert_eq!(published.len(), 1);
        assert_eq!(published[0].visibility, SourceVisibility::Public);
        let mut changed = snapshot.clone();
        changed.heads[0].visibility = SourceVisibility::Private;
        assert!(validate_bound_scope(&changed, "bot.public").is_err());
        let mut changed = snapshot.clone();
        changed.revisions.remove(0);
        changed.heads.remove(0);
        assert!(validate_bound_scope(&changed, "bot.public").is_err());
        let mut changed = snapshot;
        changed.revisions[1].visibility = SourceVisibility::Private;
        changed.revisions[1].allowed_scopes = BTreeSet::from(["bot.public".into()]);
        changed.heads[1] = changed.revisions[1].clone();
        assert!(validate_bound_scope(&changed, "bot.public").is_err());
    }
}
