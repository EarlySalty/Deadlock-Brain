#!/usr/bin/env bash
# Bindet vorhandene interne Entscheidungen; liest weder Credentials noch Rohtexte.
set -euo pipefail
[[ $# == 4 ]] || { echo 'Aufruf: bind-v1-import.sh BEOBACHTUNG AUTORISIERUNG VORLAGE ZIELVERZEICHNIS' >&2; exit 64; }
observation=$1
authorization=$2
template=$3
destination=$4
[[ -f $observation && ! -L $observation && -f $authorization && ! -L $authorization && -f $template && ! -L $template ]] || exit 65
[[ $destination == /* && -d $destination && ! -L $destination ]] || exit 65
umask 077
observation_hash=$(sha256sum -- "$observation")
observation_hash=${observation_hash%% *}
approval_hash=$(sha256sum -- "$authorization")
approval_hash=${approval_hash%% *}
approval="sha256:$approval_hash"
# Die Akte bestätigt genau diese neue Aufnahme und die bestehenden Grenzen.
jq -e --arg digest "$observation_hash" '
  .observation_file_sha256 == $digest and
  .authorization_kind == "existing_internal_v1_import" and
  .inventory_decision == "observed_archive_ids_active_no_known_local_revocations" and
  .provider_egress_allowed == false and .publication_allowed == false and
  (.decision_commit | test("^[0-9a-f]{40}$")) and
  (.decision_document_sha256 | test("^[0-9a-f]{64}$")) and
  (.user_instruction_ref | type == "string" and length > 0)
' "$authorization" >/dev/null
jq -e '
  .observation_kind == "new_read_only_snapshot" and .database == "brain" and
  .role == "brain_readonly" and .transport == "unix_socket" and
  (.database_oid > 0) and (.archive_schema_oid > 0) and (.core_schema_oid > 0) and
  (.archive_schema_oid != .core_schema_oid) and (.snapshot_epoch > 0) and
  (.schema_sha256 | test("^[0-9a-f]{64}$")) and
  (.snapshot_sha256 | test("^[0-9a-f]{64}$")) and
  (.observed_logical_ids | keys == ["legacy-entities", "legacy-patchnotes"]) and
  all(.observed_logical_ids[]; length > 0 and (unique | length) == length) and
  .document_counts["legacy-entities"] == (.observed_logical_ids["legacy-entities"] | length) and
  .document_counts["legacy-patchnotes"] == (.observed_logical_ids["legacy-patchnotes"] | length)
' "$observation" >/dev/null
scratch=$(mktemp -d "$destination/.v1-bind.XXXXXX")
trap 'rm -rf -- "$scratch"' EXIT
jq --arg approval "$approval" --arg report "$destination/import.report.json" --slurpfile observed "$observation" '
  . as $template | $observed[0] as $o |
  if .sources["legacy-entities"].visibility != "public" or
     .sources["legacy-entities"].allowed_scopes != ["game.public"] or
     .sources["legacy-patchnotes"].visibility != "private" or
     .sources["legacy-patchnotes"].allowed_scopes != ["brain.legacy.review"] or
     any(.sources[]; .provider_egress_allowed != false or .publication_allowed != false or .raw_retention_allowed != true)
  then error("Quellgrenzen stimmen nicht überein") else . end |
  .sources |= with_entries(.value = {
    visibility: .value.visibility, allowed_scopes: .value.allowed_scopes,
    provider_egress_allowed: false, publication_allowed: false,
    raw_retention_allowed: true, authorization_ref: $approval
  }) |
  .snapshot_label = $o.snapshot_label | .snapshot_epoch = $o.snapshot_epoch |
  .release = {id_prefix: "legacy-core-v1", knowledge_version: "brain-legacy-core-v1", patch: $o.snapshot_label} |
  .report = $report |
  .production_binding = {
    approval_ref: $approval, database_oid: $o.database_oid,
    archive_schema_oid: $o.archive_schema_oid, core_schema_oid: $o.core_schema_oid,
    schema_sha256: $o.schema_sha256, snapshot_sha256: $o.snapshot_sha256,
    policy_sha256: "", table_counts: $o.table_counts,
    active_ids: ($o.observed_logical_ids | with_entries(.value |= sort)),
    revoked_ids: {"legacy-entities": [], "legacy-patchnotes": []},
    tombstone_ids: {"legacy-entities": [], "legacy-patchnotes": []}
  }
' "$template" > "$scratch/import.json"
# Exact serde tuple/SourcePolicyConfig field order from the pinned Rust contract.
# -j emits no trailing newline; BTreeMap keys and BTreeSet IDs are ordered above.
jq -cj '[.production_binding.approval_ref, .sources,
  .production_binding.active_ids, .production_binding.revoked_ids,
  .production_binding.tombstone_ids]' "$scratch/import.json" > "$scratch/policy.bin"
policy_hash=$(sha256sum -- "$scratch/policy.bin")
policy_hash=${policy_hash%% *}
jq --arg digest "$policy_hash" '.production_binding.policy_sha256 = $digest' "$scratch/import.json" > "$scratch/final.json"
[[ ! -e $destination/import.json ]] || { echo 'Zielconfig existiert bereits.' >&2; exit 73; }
ln -- "$scratch/final.json" "$destination/import.json"
printf 'V1_IMPORT_BOUND approval=%s policy=%s\n' "$approval" "$policy_hash"
