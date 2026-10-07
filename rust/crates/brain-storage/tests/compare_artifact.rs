use brain_contracts::{
    CorpusRelease, CorpusSnapshot, DocumentHead, DocumentRevision, PortError, Principal,
    SnapshotReadPort, SourceRecordV2, SourceVisibility,
};
use brain_storage::compare_artifact::{
    compare_dependency, compare_sha256, CompareArtifact, CompareArtifactBody,
    CompareCalculationVerifier, CompareReleaseBinding, UnconfirmedCompareCalculation,
};
use brain_storage::PgStore;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{
    collections::{BTreeMap, BTreeSet},
    os::unix::fs::DirBuilderExt,
    path::PathBuf,
    process::{Command, Stdio},
};

fn principal() -> Principal {
    Principal {
        actor_id: "fixture".into(),
        channel: "fixture".into(),
        scopes: BTreeSet::from(["private".into()]),
        provider_egress: BTreeSet::new(),
    }
}

fn record() -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: "fixture".into(),
        logical_id: "mechanism".into(),
        revision: 1,
        content_hash: compare_sha256(b"fixture mechanism"),
        content: "fixture mechanism".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    }
}

fn release() -> CorpusRelease {
    CorpusRelease {
        release_id: "fixture-release".into(),
        knowledge_version: "fixture-version".into(),
        patch: "fixture-patch".into(),
        created_at_epoch: 1,
        source_revisions: BTreeMap::from([(
            "fixture".into(),
            BTreeMap::from([("mechanism".into(), 1)]),
        )]),
    }
}

fn candidate() -> CompareArtifact {
    CompareArtifact::pending(CompareArtifactBody {
        release: CompareReleaseBinding::from_release(&release()).unwrap(),
        calculation: json!({"fixture":true,"scenario":{"boons":[0,1]}}),
        render_model: json!({"fixture":true}),
        mechanism_version: "fixture-mechanism-v1".into(),
        dependencies: vec![compare_dependency(&record()).unwrap()],
        html: "<html>fixture</html>".into(),
        svg: "<svg>fixture</svg>".into(),
    })
    .unwrap()
}

struct Reader {
    record: SourceRecordV2,
    head: SourceRecordV2,
}
impl SnapshotReadPort for Reader {
    fn read_snapshot(&self, _: &str) -> Result<CorpusSnapshot, PortError> {
        Ok(CorpusSnapshot {
            release: release(),
            revisions: vec![self.record.clone()],
            heads: vec![self.head.clone()],
        })
    }
    fn read_heads(&self, _: &[DocumentRevision]) -> Result<Vec<DocumentHead>, PortError> {
        Ok(vec![DocumentHead::from(&self.head)])
    }
}

struct FixtureVerifier;
impl CompareCalculationVerifier for FixtureVerifier {
    fn verify(&self, body: &CompareArtifactBody) -> Result<(), PortError> {
        if body != candidate().body() {
            return Err(PortError::InvalidResponse("fixture mismatch".into()));
        }
        Ok(())
    }
}

#[test]
fn complete_body_and_every_rendered_byte_change_the_id() {
    let original = candidate();
    assert_eq!(original, candidate());
    for mutation in 0..5 {
        let mut body = original.body().clone();
        match mutation {
            0 => body.html.push('x'),
            1 => body.svg.push('x'),
            2 => body.calculation["scenario"]["boons"] = json!([0, 2]),
            3 => body.mechanism_version.push('x'),
            _ => body.render_model["fixture"] = json!(false),
        }
        assert_ne!(original.id(), CompareArtifact::pending(body).unwrap().id());
    }
}

#[test]
fn artifact_keeps_release_fingerprint_without_unrelated_private_document_ids() {
    let mut release = release();
    release.source_revisions.insert(
        "private-chat-source".into(),
        BTreeMap::from([("private-conversation-id".into(), 1)]),
    );
    let mut body = candidate().body().clone();
    body.release = CompareReleaseBinding::from_release(&release).unwrap();
    let bytes = serde_json::to_string(CompareArtifact::pending(body).unwrap().body()).unwrap();
    assert!(!bytes.contains("private-chat-source"));
    assert!(!bytes.contains("private-conversation-id"));
}

#[test]
fn private_scope_never_turns_a_private_source_into_a_public_artifact() {
    let artifact = candidate();
    let mut reader = Reader {
        record: record(),
        head: record(),
    };
    artifact.check_public(&reader, &principal()).unwrap();
    for mutation in 0..4 {
        reader.head = record();
        match mutation {
            0 => reader.head.visibility = SourceVisibility::Private,
            1 => {
                reader.head.allowed_scopes.insert("private".into());
            }
            2 => {
                reader.head.revision = 2;
                reader.head.tombstone = true;
                reader.head.content.clear();
            }
            _ => reader.record.content.push('x'),
        }
        assert!(artifact.check_public(&reader, &principal()).is_err());
    }
}

struct ScratchPg {
    directory: PathBuf,
}
impl ScratchPg {
    fn start() -> Self {
        let directory =
            std::env::temp_dir().join(format!("brain-compare-pg-{}", std::process::id()));
        std::fs::DirBuilder::new()
            .mode(0o700)
            .create(&directory)
            .unwrap();
        let instance = Self { directory };
        std::fs::create_dir(instance.directory.join("socket")).unwrap();
        assert!(Command::new("/usr/lib/postgresql/16/bin/initdb")
            .arg("-D")
            .arg(instance.directory.join("data"))
            .args(["-A", "trust", "-U", "brain_core_test", "--no-locale"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success());
        let options = format!(
            "-k {} -p 55439 -c listen_addresses='' -c shared_buffers=16MB",
            instance.directory.join("socket").display()
        );
        assert!(Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(instance.directory.join("data"))
            .arg("-l")
            .arg(instance.directory.join("postgres.log"))
            .args(["-o", &options, "-w", "start"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success());
        instance
    }
}
impl Drop for ScratchPg {
    fn drop(&mut self) {
        let stopped = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(self.directory.join("data"))
            .args(["-m", "immediate", "-w", "stop"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
        assert!(stopped.success());
        std::fs::remove_dir_all(&self.directory).unwrap();
    }
}

#[tokio::test]
async fn isolated_postgres_pending_receipt_read_and_revocation() {
    let pg = ScratchPg::start();
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(pg.directory.join("socket").to_str().unwrap())
                .port(55439)
                .username("brain_core_test")
                .database("postgres")
                .password(""),
        )
        .await
        .unwrap();
    let address: Option<String> = sqlx::query_scalar("SELECT inet_server_addr()::text")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(address.is_none());
    let store = PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    sqlx::query("CREATE ROLE brain_site LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("GRANT USAGE ON SCHEMA brain TO brain_site")
        .execute(&pool)
        .await
        .unwrap();
    store.migrate_compare_artifacts().await.unwrap();
    store.migrate_compare_artifacts().await.unwrap();
    store.apply(&record()).await.unwrap();
    sqlx::query("INSERT INTO brain.corpus_releases_v1(release_id,knowledge_version,patch,release_json) VALUES($1,$2,$3,$4)").bind("fixture-release").bind("fixture-version").bind("fixture-patch").bind(serde_json::to_value(release()).unwrap()).execute(&pool).await.unwrap();
    let artifact = candidate();
    store.save_pending_compare(&artifact).await.unwrap();
    store.save_pending_compare(&artifact).await.unwrap();
    assert!(store
        .read_public_compare(artifact.id(), &principal())
        .await
        .unwrap()
        .is_none());
    let reader = Reader {
        record: record(),
        head: record(),
    };
    assert!(store
        .publish_compare_artifact(
            &artifact,
            &reader,
            &principal(),
            &UnconfirmedCompareCalculation
        )
        .await
        .is_err());
    let publication = store
        .publish_compare_artifact(&artifact, &reader, &principal(), &FixtureVerifier)
        .await
        .unwrap();
    assert_eq!(publication.artifact_id, artifact.id());
    let site_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(
            PgConnectOptions::new_without_pgpass()
                .host(pg.directory.join("socket").to_str().unwrap())
                .port(55439)
                .username("brain_site")
                .database("postgres")
                .password(""),
        )
        .await
        .unwrap();
    let site_store = PgStore::new(site_pool.clone());
    assert_eq!(
        site_store
            .read_public_compare(artifact.id(), &principal())
            .await
            .unwrap()
            .unwrap(),
        artifact
    );
    for statement in [
        "SELECT record_json FROM brain.source_record_heads",
        "SELECT body_json FROM brain.compare_artifacts_v1",
        "UPDATE brain.compare_artifacts_v1 SET publication_receipt=NULL",
    ] {
        let error = sqlx::query(statement)
            .execute(&site_pool)
            .await
            .unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("42501")
        );
    }
    assert_eq!(
        store
            .read_public_compare(artifact.id(), &principal())
            .await
            .unwrap()
            .unwrap(),
        artifact
    );
    let mut revoked = record();
    revoked.revision = 2;
    revoked.visibility = SourceVisibility::Private;
    revoked.allowed_scopes.insert("private".into());
    store.apply(&revoked).await.unwrap();
    assert!(store
        .read_public_compare(artifact.id(), &principal())
        .await
        .is_err());
    revoked.revision = 3;
    revoked.visibility = SourceVisibility::Public;
    revoked.allowed_scopes.clear();
    store.apply(&revoked).await.unwrap();
    assert!(store
        .read_public_compare(artifact.id(), &principal())
        .await
        .unwrap()
        .is_some());
    sqlx::query("UPDATE brain.source_record_heads SET content_hash=$1 WHERE source_id='fixture' AND logical_id='mechanism'").bind("0".repeat(64)).execute(&pool).await.unwrap();
    assert!(store
        .read_public_compare(artifact.id(), &principal())
        .await
        .is_err());
    revoked.revision = 4;
    store.apply(&revoked).await.unwrap();
    assert!(store
        .read_public_compare(artifact.id(), &principal())
        .await
        .unwrap()
        .is_some());
    sqlx::query("UPDATE brain.compare_artifacts_v1 SET body_json=jsonb_set(body_json,'{svg}','\"changed\"'::jsonb) WHERE artifact_id=$1").bind(artifact.id()).execute(&pool).await.unwrap();
    assert!(store
        .read_public_compare(artifact.id(), &principal())
        .await
        .is_err());
    assert!(store.save_pending_compare(&artifact).await.is_err());
    site_pool.close().await;
    pool.close().await;
}
