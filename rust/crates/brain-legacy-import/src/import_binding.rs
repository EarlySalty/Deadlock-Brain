//! File-only binding of an existing internal import decision; no database or secret access.
use super::{observation_route, Config, PRODUCTION};
use brain_legacy_import::{
    cutover::CutoverBinding, sha256_hex, ENTITIES_SOURCE, PATCHNOTES_SOURCE,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::OpenOptions,
    io::{Read, Write},
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

type Result<T> = std::result::Result<T, &'static str>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Authorization {
    observation_file_sha256: String,
    authorization_kind: String,
    inventory_decision: String,
    provider_egress_allowed: bool,
    publication_allowed: bool,
    decision_commit: String,
    decision_document_sha256: String,
    user_instruction_ref: String,
}

fn hash(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub(super) fn read(path: &str) -> Result<Vec<u8>> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
        .map_err(|_| "input unavailable")?;
    let metadata = file.metadata().map_err(|_| "input metadata unavailable")?;
    if !metadata.is_file() || metadata.len() > 1024 * 1024 {
        return Err("input must be a bounded regular file");
    }
    let mut bytes = Vec::new();
    file.take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "input unreadable")?;
    if bytes.len() > 1024 * 1024 {
        return Err("input too large");
    }
    Ok(bytes)
}

pub(super) fn bind_command(args: &[String]) -> Result<()> {
    let [_, mode, observation_flag, observation_path, authorization_flag, authorization_path, config_flag, config_path, output_flag, output_path] =
        args
    else {
        return Err("usage: --bind-v1-config --observation FILE --authorization FILE --config FILE --output FILE");
    };
    if mode != "--bind-v1-config"
        || observation_flag != "--observation"
        || authorization_flag != "--authorization"
        || config_flag != "--config"
        || output_flag != "--output"
        || !Path::new(output_path).is_absolute()
    {
        return Err("binding arguments invalid");
    }
    let observation_bytes = read(observation_path)?;
    let observation: Value =
        serde_json::from_slice(&observation_bytes).map_err(|_| "observation invalid")?;
    let authorization_bytes = read(authorization_path)?;
    let authorization: Authorization =
        serde_json::from_slice(&authorization_bytes).map_err(|_| "authorization invalid")?;
    if authorization.observation_file_sha256 != sha256_hex(&observation_bytes)
        || authorization.authorization_kind != "existing_internal_v1_import"
        || authorization.inventory_decision
            != "observed_archive_ids_active_no_known_local_revocations"
        || authorization.provider_egress_allowed
        || authorization.publication_allowed
        || !hash(&authorization.decision_commit, 40)
        || !hash(&authorization.decision_document_sha256, 64)
        || authorization.user_instruction_ref.trim().is_empty()
    {
        return Err("authorization does not bind the existing internal decision");
    }
    let template_bytes = read(config_path)?;
    let mut config: Config =
        serde_json::from_slice(&template_bytes).map_err(|_| "template invalid")?;
    observation_route(&config, PRODUCTION).map_err(|_| "template endpoints invalid")?;
    if config.production_binding.is_some() {
        return Err("template is already bound");
    }
    let mut template: Value =
        serde_json::from_slice(&template_bytes).map_err(|_| "template invalid")?;
    if observation["observation_kind"] != "new_read_only_snapshot"
        || observation["database"] != "brain"
        || observation["role"] != "brain_readonly"
        || observation["transport"] != "unix_socket"
    {
        return Err("observation identity invalid");
    }
    let active: BTreeMap<String, Vec<String>> =
        serde_json::from_value(observation["observed_logical_ids"].clone())
            .map_err(|_| "observation inventory invalid")?;
    if active
        .keys()
        .map(String::as_str)
        .ne([ENTITIES_SOURCE, PATCHNOTES_SOURCE])
    {
        return Err("observation sources invalid");
    }
    let mut active_ids = BTreeMap::new();
    for (source, ids) in active {
        let ordered: BTreeSet<_> = ids.iter().cloned().collect();
        if ids.is_empty()
            || ordered.len() != ids.len()
            || ids
                .iter()
                .any(|id| id.trim().is_empty() || id.chars().any(char::is_control))
            || observation["document_counts"][&source].as_u64() != Some(ids.len() as u64)
        {
            return Err("observation document inventory invalid");
        }
        active_ids.insert(source, ordered);
    }
    let approval_ref = format!("sha256:{}", sha256_hex(&authorization_bytes));
    for (source, policy) in &mut config.sources {
        let expected_visibility = if source == ENTITIES_SOURCE {
            "public"
        } else {
            "private"
        };
        let expected_scope = if source == ENTITIES_SOURCE {
            "game.public"
        } else {
            "brain.legacy.review"
        };
        if serde_json::to_value(policy.visibility).map_err(|_| "policy invalid")?
            != expected_visibility
            || policy.allowed_scopes != BTreeSet::from([expected_scope.to_owned()])
            || policy.provider_egress_allowed
            || policy.publication_allowed
            || !policy.raw_retention_allowed
        {
            return Err("source rights differ from the existing decision");
        }
        policy.authorization_ref = Some(approval_ref.clone());
    }
    if config
        .sources
        .keys()
        .map(String::as_str)
        .ne([ENTITIES_SOURCE, PATCHNOTES_SOURCE])
    {
        return Err("template sources invalid");
    }
    let empty = BTreeMap::from([
        (ENTITIES_SOURCE.to_owned(), BTreeSet::<String>::new()),
        (PATCHNOTES_SOURCE.to_owned(), BTreeSet::new()),
    ]);
    let mut binding_json = json!({
        "approval_ref": approval_ref,
        "database_oid": observation["database_oid"], "archive_schema_oid": observation["archive_schema_oid"],
        "core_schema_oid": observation["core_schema_oid"], "schema_sha256": observation["schema_sha256"],
        "snapshot_sha256": observation["snapshot_sha256"], "policy_sha256": "",
        "table_counts": observation["table_counts"], "active_ids": active_ids,
        "revoked_ids": empty, "tombstone_ids": empty,
    });
    let binding: CutoverBinding =
        serde_json::from_value(binding_json.clone()).map_err(|_| "binding invalid")?;
    if binding.database_oid <= 0
        || binding.archive_schema_oid <= 0
        || binding.core_schema_oid <= 0
        || binding.archive_schema_oid == binding.core_schema_oid
        || !hash(&binding.schema_sha256, 64)
        || !hash(&binding.snapshot_sha256, 64)
        || binding.table_counts.keys().map(String::as_str).ne([
            "entities",
            "entity_aliases",
            "patch_event_enrichments",
            "patch_events",
        ])
        || binding.table_counts.values().any(|count| *count < 0)
        || observation["snapshot_epoch"]
            .as_i64()
            .is_none_or(|epoch| epoch <= 0)
        || observation["snapshot_label"]
            .as_str()
            .is_none_or(|label| label.trim().is_empty())
    {
        return Err("snapshot binding invalid");
    }
    binding_json["policy_sha256"] = json!(binding
        .policy_sha256(&config.sources)
        .map_err(|_| "policy fingerprint invalid")?);
    template["sources"] = serde_json::to_value(&config.sources).map_err(|_| "policy invalid")?;
    template["production_binding"] = binding_json;
    template["snapshot_label"] = observation["snapshot_label"].clone();
    template["snapshot_epoch"] = observation["snapshot_epoch"].clone();
    template["release"] = json!({"id_prefix":"legacy-core-v1", "knowledge_version":"brain-legacy-core-v1", "patch": observation["snapshot_label"]});
    template["report"] = json!(Path::new(output_path)
        .parent()
        .ok_or("output parent missing")?
        .join("import.report.json"));
    let bytes = serde_json::to_vec_pretty(&template).map_err(|_| "output encoding failed")?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(output_path)
        .map_err(|_| "output must be a new private file")?;
    output
        .write_all(&bytes)
        .map_err(|_| "output write failed")?;
    output.sync_all().map_err(|_| "output sync failed")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opened_descriptor_rejects_symlinks_and_fifo_without_waiting_for_a_writer() {
        let dir = tempfile::tempdir().unwrap();
        let regular = dir.path().join("regular.json");
        let link = dir.path().join("link.json");
        let fifo = dir.path().join("fifo");
        std::fs::write(&regular, b"{}").unwrap();
        std::os::unix::fs::symlink(&regular, &link).unwrap();
        assert!(read(link.to_str().unwrap()).is_err());
        assert!(std::process::Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success());
        let started = std::time::Instant::now();
        assert_eq!(
            read(fifo.to_str().unwrap()),
            Err("input must be a bounded regular file")
        );
        assert!(started.elapsed() < std::time::Duration::from_secs(1));
        assert_eq!(read(regular.to_str().unwrap()).unwrap(), b"{}");
    }

    #[test]
    fn complete_binding_rejects_appended_observation_and_authorization_objects() {
        let dir = tempfile::tempdir().unwrap();
        let observed = json!({
            "observation_kind":"new_read_only_snapshot", "database":"brain",
            "role":"brain_readonly", "transport":"unix_socket", "database_oid":1,
            "archive_schema_oid":2, "core_schema_oid":3, "schema_sha256":"a".repeat(64),
            "snapshot_sha256":"b".repeat(64), "snapshot_epoch":1,"snapshot_label":"synthetic-regression",
            "observed_logical_ids":{"legacy-entities":["entity/test"],"legacy-patchnotes":["patch/test"]},
            "document_counts":{"legacy-entities":1,"legacy-patchnotes":1},
            "table_counts":{"entities":1,"entity_aliases":1,"patch_event_enrichments":1,"patch_events":1}
        });
        let observation = serde_json::to_vec(&observed).unwrap();
        let authorization = serde_json::to_vec(&json!({
            "observation_file_sha256":sha256_hex(&observation),
            "authorization_kind":"existing_internal_v1_import",
            "inventory_decision":"observed_archive_ids_active_no_known_local_revocations",
            "provider_egress_allowed":false,"publication_allowed":false,
            "decision_commit":"a".repeat(40),"decision_document_sha256":"b".repeat(64),
            "user_instruction_ref":"synthetic unit regression only"
        }))
        .unwrap();
        let observation_path = dir.path().join("observation.json");
        let authorization_path = dir.path().join("authorization.json");
        let config_path = dir.path().join("template.json");
        let output_path = dir.path().join("output.json");
        std::fs::write(
            &config_path,
            include_bytes!("../../../../ops/brain-postgres/legacy-core-cutover.json"),
        )
        .unwrap();
        std::fs::write(&observation_path, &observation).unwrap();
        std::fs::write(&authorization_path, &authorization).unwrap();
        let args = vec![
            "brain-legacy-import".into(),
            "--bind-v1-config".into(),
            "--observation".into(),
            observation_path.to_str().unwrap().into(),
            "--authorization".into(),
            authorization_path.to_str().unwrap().into(),
            "--config".into(),
            config_path.to_str().unwrap().into(),
            "--output".into(),
            output_path.to_str().unwrap().into(),
        ];
        let mut streamed = observation.clone();
        streamed.extend_from_slice(&observation);
        std::fs::write(&observation_path, &streamed).unwrap();
        assert_eq!(bind_command(&args), Err("observation invalid"));
        assert!(!output_path.exists());
        std::fs::write(&observation_path, &observation).unwrap();
        let mut streamed = authorization.clone();
        streamed.extend_from_slice(&authorization);
        std::fs::write(&authorization_path, &streamed).unwrap();
        assert_eq!(bind_command(&args), Err("authorization invalid"));
        assert!(!output_path.exists());
        std::fs::write(&authorization_path, &authorization).unwrap();
        bind_command(&args).unwrap();
        let output: Config = serde_json::from_slice(&std::fs::read(&output_path).unwrap()).unwrap();
        let binding = output.production_binding.unwrap();
        assert_eq!(
            binding.policy_sha256,
            binding.policy_sha256(&output.sources).unwrap()
        );
        assert_eq!(
            bind_command(&args),
            Err("output must be a new private file")
        );
    }
}
