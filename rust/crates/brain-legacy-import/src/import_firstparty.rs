//! Narrow firstparty command ingestion through the existing document-set/store ports.
use super::{configured_options, Endpoint};
use brain_contracts::{
    source::{
        GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision, SourceTimestamp,
    },
    value::{Observed, UnknownReason},
    DocumentStorePort, SourceRecordV2, SourceVisibility,
};
use brain_ingestion::document_set::{prepare_document_batch, CoreDocument, DocumentSetSource};
use brain_legacy_import::{release_from_checkpoints, sha256_hex, ReleaseConfig};
use brain_storage::PgStore;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, Row};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

type Result<T> = std::result::Result<T, &'static str>;
const ORIGINAL: &str = "ddc-bot-firstparty-v1";
const PROBE: &str = "ddc-bot-firstparty-probe-v1";
const LOGICAL: &str = "commands-overview";
const DOC_PATH: &str = "rust/knowledge/bot/chat-befehle.md";
const CONTENT: &str = "`!commands` schickt einen Link zur Befehlsübersicht.";

fn command_document(origin: OriginArtifact) -> CoreDocument {
    CoreDocument {
        logical_id: LOGICAL.into(),
        content: CONTENT.into(),
        metadata: BTreeMap::from([
            ("kind".into(), "fact".into()),
            ("title".into(), "Twitch-Chat-Befehl !commands".into()),
            ("name".into(), "!commands".into()),
            ("field".into(), "Funktion".into()),
            ("locator".into(), origin.locator.clone()),
        ]),
        origin,
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    target: Endpoint,
    source_id: String,
    action: Action,
    repository: PathBuf,
    document_commit: String,
    document_sha256: String,
    authorization: PathBuf,
    authorization_sha256: String,
    epoch: i64,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Action {
    Import,
    Revoke,
    RevokeProvider,
    Delete,
}

fn git(config: &Config) -> Result<Vec<u8>> {
    if config.document_commit.len() != 40
        || !config
            .document_commit
            .bytes()
            .all(|b| b.is_ascii_hexdigit())
        || !config.repository.is_absolute()
    {
        return Err("source revision invalid");
    }
    let mut child = Command::new("git")
        .arg("-C")
        .arg(&config.repository)
        .arg("show")
        .arg(format!("{}:{DOC_PATH}", config.document_commit))
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .map_err(|_| "source object unavailable")?;
    let stdout = child.stdout.take().ok_or("source pipe unavailable")?;
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let reader = std::thread::Builder::new()
        .name("firstparty-git-reader".into())
        .spawn(move || {
            let mut bytes = Vec::new();
            let result = stdout
                .take(64 * 1024 + 1)
                .read_to_end(&mut bytes)
                .map(|_| bytes)
                .map_err(|_| "source object unreadable");
            let _ = sender.send(result);
        });
    let reader = match reader {
        Ok(reader) => reader,
        Err(_) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err("source reader unavailable");
        }
    };
    let deadline = Instant::now() + Duration::from_secs(5);
    let result = receiver
        .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        .map_err(|_| "source object timeout")
        .and_then(|result| result);
    let mut bytes = match result {
        Ok(bytes) if bytes.len() <= 64 * 1024 => bytes,
        _ => {
            let _ = child.kill();
            let _ = child.wait();
            // Do not join a pipe reader that might be held by a descendant process.
            return Err("source object too large, unreadable or timed out");
        }
    };
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let _ = reader.join();
                if !status.success() {
                    return Err("source object invalid");
                }
                return Ok(std::mem::take(&mut bytes));
            }
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(10)),
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("source exit timeout");
            }
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("source exit unavailable");
            }
        }
    }
}

pub(super) async fn command(args: &[String]) -> Result<Value> {
    let [_, command, flag, path] = args else {
        return Err("usage: --firstparty-command --config FILE");
    };
    if command != "--firstparty-command" || flag != "--config" {
        return Err("arguments invalid");
    }
    let bytes = super::import_binding::read(path).map_err(|_| "config unavailable")?;
    if bytes.len() > 64 * 1024 {
        return Err("config too large");
    }
    let config: Config = serde_json::from_slice(&bytes).map_err(|_| "config invalid")?;
    if config.target.socket != "/run/deadlock-brain-postgresql"
        || config.target.port != 5446
        || config.target.database != "brain"
        || config.target.username != "brain_ingest"
        || config.target.auth_secret.as_deref() != Some("BRAIN_PG_INGEST_PASSWORD")
        || !matches!(config.source_id.as_str(), ORIGINAL | PROBE)
        || config.epoch <= 0
        || (!matches!(config.action, Action::Import) && config.source_id != PROBE)
    {
        return Err("target or purpose not authorized");
    }
    let authorization = super::import_binding::read(
        config
            .authorization
            .to_str()
            .ok_or("authorization path invalid")?,
    )
    .map_err(|_| "authorization unavailable")?;
    if authorization.len() > 64 * 1024 || sha256_hex(&authorization) != config.authorization_sha256
    {
        return Err("authorization fingerprint invalid");
    }
    let approval: Value =
        serde_json::from_slice(&authorization).map_err(|_| "authorization invalid")?;
    if approval["purpose"] != "existing_public_bot_command_answering"
        || approval["scope"] != "bot.public"
        || approval["publication_allowed"] != true
        || approval["provider_egress_allowed"] != true
        || approval["document_commit"] != config.document_commit
        || approval["document_sha256"] != config.document_sha256
        || approval["content_sha256"] != sha256_hex(CONTENT.as_bytes())
        || (!matches!(config.action, Action::Import)
            && approval["probe_revocation_and_deletion_allowed"] != true)
    {
        return Err("source-purpose decision does not match");
    }
    let git_config = config.clone();
    let document = tokio::task::spawn_blocking(move || git(&git_config))
        .await
        .map_err(|_| "source worker unavailable")??;
    let text = std::str::from_utf8(&document).map_err(|_| "source encoding invalid")?;
    if sha256_hex(&document) != config.document_sha256
        || !text.contains(CONTENT)
        || !text.contains("audience: streamer")
        || !text.contains("namespace: bot")
    {
        return Err("published command source differs from decision");
    }
    let options = configured_options(&[&config.target])
        .map_err(|_| "secret configuration invalid")?
        .remove(0);
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .map_err(|_| "target connection failed")?;
    let row = sqlx::query("SELECT current_database() AS database,current_user AS username,inet_server_addr()::text AS address")
        .fetch_one(&pool).await.map_err(|_| "target identity unavailable")?;
    if row
        .try_get::<String, _>("database")
        .map_err(|_| "target identity invalid")?
        != "brain"
        || row
            .try_get::<String, _>("username")
            .map_err(|_| "target identity invalid")?
            != "brain_ingest"
        || row
            .try_get::<Option<String>, _>("address")
            .map_err(|_| "target identity invalid")?
            .is_some()
    {
        return Err("target identity invalid");
    }
    let store = PgStore::new(pool.clone());
    store
        .check_core_schema()
        .await
        .map_err(|_| "core schema incompatible")?;
    if !matches!(config.action, Action::Import) {
        let value: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2")
            .bind(&config.source_id).bind(LOGICAL).fetch_one(&pool).await.map_err(|_| "probe head unavailable")?;
        let mut record: SourceRecordV2 =
            serde_json::from_value(value).map_err(|_| "probe head invalid")?;
        if record.tombstone && !matches!(config.action, Action::Delete) {
            return Err("deleted probe cannot be revived");
        }
        let mut origin = brain_contracts::source::origin_from_record(&record)
            .map_err(|_| "probe origin invalid")?;
        if record.source_id != PROBE
            || origin.policy.authorization_ref
                != Observed::known(format!("sha256:{}", config.authorization_sha256))
            || origin.source_revision
                != (SourceRevision::Git {
                    commit: config.document_commit.clone(),
                })
        {
            return Err("probe head differs from decision");
        }
        record.revision = record
            .revision
            .checked_add(1)
            .ok_or("probe revision exhausted")?;
        if !matches!(config.action, Action::RevokeProvider) {
            record.visibility = SourceVisibility::Private;
            record.allowed_scopes = BTreeSet::from(["brain.firstparty.probe.review".into()]);
            record.tombstone = matches!(config.action, Action::Delete);
            origin.policy.publication_allowed = false;
        }
        if record.tombstone {
            record.content.clear();
        }
        record.content_hash = sha256_hex(record.content.as_bytes());
        origin.raw_sha256 = record.content_hash.clone();
        origin.policy.visibility = record.visibility;
        origin.policy.allowed_scopes = record.allowed_scopes.clone();
        origin.policy.provider_egress_allowed = false;
        origin
            .bind_record(&mut record)
            .map_err(|_| "probe withdrawal invalid")?;
        store
            .apply(&record)
            .await
            .map_err(|_| "probe withdrawal failed")?;
        return Ok(
            json!({"source_id":config.source_id,"revision":record.revision,"tombstone":record.tombstone}),
        );
    }
    let scope = BTreeSet::from(["bot.public".to_owned()]);
    let locator = format!(
        "https://github.com/EarlySalty/Deadlock-Twitch-Bot/blob/{}/{DOC_PATH}",
        config.document_commit
    );
    let origin = OriginArtifact {
        identity: SourceIdentity {
            source_id: config.source_id.clone(),
            logical_id: LOGICAL.into(),
        },
        source_revision: SourceRevision::Git {
            commit: config.document_commit.clone(),
        },
        raw_sha256: sha256_hex(CONTENT.as_bytes()),
        locator: locator.clone(),
        parser_revision: "brain-firstparty-command.v1".into(),
        parser_family: "firstparty_git_excerpt".into(),
        schema_version: Observed::known("brain.ir.v1".into()),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(config.epoch)),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("de".into()),
        origin_artifacts: BTreeSet::from([format!("git:{}:{DOC_PATH}", config.document_commit)]),
        derivation_family: Observed::known("ddc-bot-command-documentation".into()),
        policy: SourcePolicy {
            visibility: SourceVisibility::Public,
            allowed_scopes: scope.clone(),
            authorization_ref: Observed::known(format!("sha256:{}", config.authorization_sha256)),
            license: Observed::unknown(UnknownReason::NotPresent),
            publication_allowed: true,
            provider_egress_allowed: true,
            raw_retention_allowed: true,
        },
        validity: GameValidity::unknown(),
    };
    let source = DocumentSetSource {
        source_id: config.source_id.clone(),
        configuration: format!("firstparty-v1:{}", config.authorization_sha256),
        visibility: SourceVisibility::Public,
        allowed_scopes: scope,
        tombstone_metadata: BTreeMap::from([("kind".into(), "fact".into())]),
    };
    let previous = store
        .checkpoint(&config.source_id)
        .await
        .map_err(|_| "checkpoint unavailable")?;
    let batch = prepare_document_batch(&source, &[command_document(origin)], previous.as_ref())
        .map_err(|_| "document preparation failed")?;
    let release = release_from_checkpoints(
        std::slice::from_ref(&batch.checkpoint),
        &ReleaseConfig {
            id_prefix: "firstparty-bot-v1".into(),
            knowledge_version: "brain-firstparty-bot-v1".into(),
            patch: "firstparty-documentation-v1".into(),
        },
        config.epoch,
    )
    .map_err(|_| "release invalid")?;
    // The complete single-source expected head is checked atomically with publication.
    let expected = if let Some(record) = batch.records.first() {
        record.clone()
    } else {
        let value: Value = sqlx::query_scalar("SELECT record_json FROM brain.source_record_heads WHERE source_id=$1 AND logical_id=$2")
            .bind(&config.source_id).bind(LOGICAL).fetch_one(&pool).await.map_err(|_| "unchanged head unavailable")?;
        serde_json::from_value(value).map_err(|_| "unchanged head invalid")?
    };
    let lease = store
        .claim(&config.source_id, "brain-firstparty-import", 60_000)
        .await
        .map_err(|_| "source lease unavailable")?;
    store
        .commit_batches_and_publish_checked(&[(&batch, &lease)], &release, &[expected])
        .await
        .map_err(|_| "atomic import failed")?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "clock invalid")?
        .as_secs();
    Ok(
        json!({"release_id":release.release_id,"knowledge_version":release.knowledge_version,"source_id":config.source_id,
        "documents":1,"observed_at_epoch":now,"authorization_sha256":config.authorization_sha256}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{
        AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget, Evidence,
        PortError, Principal, ProviderAnswer, Query,
    };
    use brain_kernel::{AnswerKernelPort, Kernel};
    use brain_storage::MemoryRepository;
    use dbrain_retrieval::ReleaseRetriever;

    struct NoProvider;
    impl AnswerProviderPort for NoProvider {
        fn answer(
            &self,
            _: &Query,
            _: &AuthorizedContext,
            _: &[Evidence],
        ) -> std::result::Result<ProviderAnswer, PortError> {
            panic!("deterministic command fact must not call a provider")
        }
    }

    #[tokio::test]
    async fn command_projection_answers_only_its_explicit_identity_and_field() {
        let scopes = BTreeSet::from(["bot.public".into()]);
        let origin = OriginArtifact {
            identity: SourceIdentity {
                source_id: ORIGINAL.into(),
                logical_id: LOGICAL.into(),
            },
            source_revision: SourceRevision::Git {
                commit: "a".repeat(40),
            },
            raw_sha256: sha256_hex(CONTENT.as_bytes()),
            locator: "https://example.invalid/firstparty-command".into(),
            parser_revision: "brain-firstparty-command.v1".into(),
            parser_family: "firstparty_git_excerpt".into(),
            schema_version: Observed::known("brain.ir.v1".into()),
            schema_sha256: Observed::unknown(UnknownReason::NotPresent),
            retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(1)),
            source_time: Observed::unknown(UnknownReason::NotPresent),
            language: Observed::known("de".into()),
            origin_artifacts: BTreeSet::from(["fixture-origin".into()]),
            derivation_family: Observed::known("ddc-bot-command-documentation".into()),
            policy: SourcePolicy {
                visibility: SourceVisibility::Public,
                allowed_scopes: scopes.clone(),
                authorization_ref: Observed::known(format!("sha256:{}", "a".repeat(64))),
                license: Observed::unknown(UnknownReason::NotPresent),
                publication_allowed: true,
                provider_egress_allowed: true,
                raw_retention_allowed: true,
            },
            validity: GameValidity::unknown(),
        };
        let source = DocumentSetSource {
            source_id: ORIGINAL.into(),
            configuration: "contract-projection".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: scopes.clone(),
            tombstone_metadata: BTreeMap::from([("kind".into(), "fact".into())]),
        };
        let batch = prepare_document_batch(&source, &[command_document(origin)], None).unwrap();
        assert_eq!(batch.records[0].content, CONTENT);
        let release = release_from_checkpoints(
            std::slice::from_ref(&batch.checkpoint),
            &ReleaseConfig {
                id_prefix: "firstparty-contract".into(),
                knowledge_version: "command-documentation-v1".into(),
                patch: "firstparty-documentation-v1".into(),
            },
            1,
        )
        .unwrap();
        let store = MemoryRepository::default();
        let lease = store
            .claim(ORIGINAL, "projection-test", 60_000)
            .await
            .unwrap();
        store.commit(&batch, &lease).await.unwrap();
        store.publish(&release).await.unwrap();
        let kernel = Kernel::new(ReleaseRetriever::new(store, 6), NoProvider);
        let context = AuthorizedContext {
            discord: None,
            request_deadline: None,
            principal: Principal {
                actor_id: "projection-test".into(),
                channel: "private-contract-test".into(),
                scopes: scopes.clone(),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            conversation_id: "projection-test".into(),
            knowledge_release: release.release_id,
            deadline_ms: 1_000,
            budget: Budget::default(),
        };
        for (text, status) in [
            ("Welche Funktion hat !commands?", AnswerStatus::Answered),
            (
                "Welche Funktion hat !clip?",
                AnswerStatus::InsufficientEvidence,
            ),
            (
                "Welche Reichweite hat !commands?",
                AnswerStatus::InsufficientEvidence,
            ),
        ] {
            let query = Query {
                domain: None,
                request_id: "projection-test".into(),
                conversation_id: context.conversation_id.clone(),
                text: text.into(),
                requested_scopes: scopes.clone(),
                profile: AnswerProfile::Fact,
                patch: None,
                mode: None,
            };
            let answer = kernel.answer_for_publication(&query, &context);
            assert_eq!(answer.status, status, "{text}");
            if status == AnswerStatus::Answered {
                assert!(answer.text.contains(CONTENT));
                assert_eq!(answer.citations.len(), 1);
            } else {
                assert!(answer.citations.is_empty());
            }
        }
    }
}
