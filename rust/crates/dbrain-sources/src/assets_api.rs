use crate::{
    external::SourceIr,
    schema_watch::{
        asset_dependencies, validate_consumed, DriftReport, OpenApiSnapshot, OPENAPI_URL,
    },
    store::{complete_run, open_pool, EntitySnapshotInput, SourceStore},
    Result, SourcesError,
};
use brain_storage::asset_mirror::{
    mirrored_asset_key, mirrored_asset_languages, MIRRORED_ASSET_KINDS, REQUIRED_MIRROR_KINDS,
};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions, SourceHttpResponse};
use serde_json::{json, Map, Value};
use std::{collections::BTreeSet, path::Path};

pub const SOURCE: &str = "deadlock_assets_api";
pub const PARSER_REVISION: &str = "dbrain-assets/4";
pub const BASE_URL: &str = "https://api.deadlock-api.com";
pub const ENDPOINTS: &[(&str, &str)] = &[
    ("items", "/v1/assets/items"),
    ("heroes", "/v1/assets/heroes?only_active=true"),
    ("heroes_all", "/v1/assets/heroes"),
    ("ranks", "/v1/assets/ranks"),
    ("colors", "/v1/assets/colors"),
    ("build_tags", "/v1/assets/build-tags"),
    ("npc_units", "/v1/assets/npc-units"),
    ("generic_data", "/v1/assets/generic-data"),
    ("misc_entities", "/v1/assets/misc-entities"),
    ("modifiers", "/v1/assets/modifiers"),
];
const RETIRED_KINDS: &[&str] = &["raw_items", "raw_heroes"];
const DEFAULT_KINDS: &[&str] = REQUIRED_MIRROR_KINDS;
const LANGUAGES: &[&str] = &["english", "german"];

#[derive(Debug, Clone, Default)]
pub struct PullAssetsOptions {
    pub kinds: Vec<String>,
}

pub async fn pull_assets(
    raw_dir: &Path,
    http: &HttpClient,
    options: PullAssetsOptions,
) -> Result<Value> {
    let selected = resolve_kinds(&options.kinds)?;
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run("assets").await?;
    let outcome = pull_assets_inner(&store, http, &selected).await;
    complete_run(&store, run_id, outcome).await
}
fn resolve_kinds(kinds: &[String]) -> Result<Vec<(String, &'static str)>> {
    let requested: Vec<String> = if kinds.is_empty() {
        MIRRORED_ASSET_KINDS.iter().map(|k| k.to_string()).collect()
    } else {
        kinds.to_vec()
    };
    let mut selected = Vec::new();
    let mut seen = BTreeSet::new();
    for kind in requested {
        if RETIRED_KINDS.contains(&kind.as_str()) {
            return Err(SourcesError::invalid_input(format!(
                "Assets-Art {kind} wird von api.deadlock-api.com nicht mehr angeboten"
            )));
        }
        let endpoint = endpoint_path(&kind).ok_or_else(|| {
            SourcesError::invalid_input(format!("Unbekannter Assets-Endpoint: {kind}"))
        })?;
        if seen.insert(kind.clone()) {
            selected.push((kind, endpoint));
        }
    }
    Ok(selected)
}

pub(crate) async fn pull_assets_inner(
    store: &SourceStore<'_>,
    http: &HttpClient,
    selected: &[(String, &'static str)],
) -> Result<Value> {
    let manifest = SourceIr::from_http(
        SOURCE,
        PARSER_REVISION,
        http.get_bounded(
            &format!("{BASE_URL}/v1/assets/steam-info"),
            SourceHttpOptions::default(),
        )?,
    )?;
    let client_version = manifest
        .payload()?
        .get("client_version")
        .and_then(Value::as_i64)
        .filter(|version| *version > 0)
        .ok_or_else(|| SourcesError::invalid_input("Assets-Manifest ohne gültige Clientversion"))?;
    let manifest_id = store.persist_ir(
        &format!("steam_info/{client_version}"), "Deadlock Assets Versionsmanifest", &manifest,
        &json!({"role":"client_manifest","client_version":client_version,"not_patch_mapping":true}),
    ).await?;
    let now = deadlock_brain_core::now_epoch_seconds()
        .map_err(|error| SourcesError::invalid_input(error.to_string()))?;
    let previous = sqlx::query_scalar::<_, String>(
        "SELECT summary::text FROM brain.source_runs WHERE source='assets' AND status='ok' \
         AND summary->>'client_version'=$1 AND summary->>'parser_revision'=$2 ORDER BY id DESC LIMIT 1",
    ).bind(client_version.to_string()).bind(PARSER_REVISION).fetch_optional(store.pool()).await?;
    if let Some(previous) = previous {
        let mut summary: Value = serde_json::from_str(&previous)?;
        if mirror_is_fresh(&summary, selected, now) {
            summary["checked_at"] = json!(now);
            summary["reused_local_mirror"] = json!(true);
            return Ok(summary);
        }
    }
    let baseline = OpenApiSnapshot::pinned()?;
    let schema_response = http.get_bounded(
        OPENAPI_URL,
        SourceHttpOptions {
            max_bytes: 1024 * 1024,
            ..Default::default()
        },
    )?;
    let mut schema_ir = SourceIr::from_http(SOURCE, "dbrain-openapi/1", schema_response)?;
    let comparison = OpenApiSnapshot::from_raw(schema_ir.raw().to_vec())
        .and_then(|new| baseline.compare(&new, &asset_dependencies()));
    if comparison.is_err() {
        schema_ir.quarantine("invalid_or_unsupported_openapi");
    }
    store
        .persist_ir(
            "openapi",
            "Deadlock API OpenAPI observation",
            &schema_ir,
            &json!({"role":"schema_contract","not_gameplay_evidence":true}),
        )
        .await?;
    let report = comparison?;
    let mut staged = Vec::new();
    let mut first_error = None;
    for (kind, endpoint) in selected {
        let languages = mirrored_asset_languages(kind).unwrap_or(&["english"]);
        for language in languages {
            let url = asset_url(kind, endpoint, client_version, language);
            let response = http.get_bounded(&url, SourceHttpOptions::default())?;
            let ir = prepare_assets(kind, response, Some(&report))?;
            let pinned = MIRRORED_ASSET_KINDS.contains(&kind.as_str());
            let language = (!language.is_empty()).then_some(*language);
            let key = if pinned {
                mirrored_asset_key(kind, language)
                    .map_err(|error| SourcesError::invalid_input(error.to_string()))?
            } else {
                format!("{kind}/english")
            };
            let document_key = if pinned {
                format!("{client_version}/{key}")
            } else {
                format!("unversioned/{kind}")
            };
            let title = format!("Deadlock Assets API {document_key}");
            let details = json!({"kind":kind,"endpoint":endpoint,
                "client_version":pinned.then_some(client_version),"language":if pinned { language } else { None },
                "schema_report":report,"contract_coverage":"container_and_consumed_fields"});
            match store.persist_ir(&document_key, &title, &ir, &details).await {
                Ok(document_id) => staged.push((kind.clone(), key, ir, document_id)),
                Err(error) => {
                    if first_error.is_none() {
                        first_error = Some(error);
                    }
                }
            }
        }
    }
    if let Some(error) = first_error {
        return Err(error);
    }
    let mut summary = Map::new();
    let mut total = 0usize;
    for (kind, key, ir, document_id) in staged {
        let snapshots = snapshots_for(&kind, ir.payload()?)?;
        let count = store
            .insert_many_snapshots(&snapshots, Some(document_id))
            .await?;
        summary.insert(key, json!({"url":ir.provenance().locator,"snapshots":count,"raw_sha256":ir.provenance().raw_sha256,"source_document_id":document_id,"validation":ir.validation()}));
        total += count;
    }
    let mirror_complete = DEFAULT_KINDS.iter().all(|kind| {
        LANGUAGES.iter().all(|language| {
            summary
                .get(&format!("{kind}/{language}"))
                .and_then(|endpoint| endpoint["snapshots"].as_u64())
                .is_some_and(|count| count > 0)
        })
    });
    let mirrored_at = deadlock_brain_core::now_epoch_seconds()
        .map_err(|error| SourcesError::invalid_input(error.to_string()))?;
    Ok(
        json!({"endpoints":summary,"snapshots":total,"schema_drift":report,
        "client_version":client_version,"manifest_document_id":manifest_id,
        "manifest_raw_sha256":manifest.provenance().raw_sha256,
        "parser_revision":PARSER_REVISION,"mirrored_at":mirrored_at,"checked_at":mirrored_at,
        "mirror_complete":mirror_complete,"reused_local_mirror":false}),
    )
}

fn asset_url(kind: &str, endpoint: &str, client_version: i64, language: &str) -> String {
    let separator = if endpoint.contains('?') { '&' } else { '?' };
    if MIRRORED_ASSET_KINDS.contains(&kind) {
        let url = format!("{BASE_URL}{endpoint}{separator}client_version={client_version}");
        if kind == "modifiers" {
            url
        } else {
            format!("{url}&language={language}")
        }
    } else {
        format!("{BASE_URL}{endpoint}")
    }
}

fn mirror_is_fresh(summary: &Value, selected: &[(String, &'static str)], now: i64) -> bool {
    summary["mirrored_at"]
        .as_i64()
        .is_some_and(|at| now >= at && now - at < 86_400)
        && selected.iter().all(|(kind, _)| {
            let languages = mirrored_asset_languages(kind).unwrap_or(&["english"]);
            languages.iter().all(|language| {
                let key = if MIRRORED_ASSET_KINDS.contains(&kind.as_str()) {
                    let Ok(key) =
                        mirrored_asset_key(kind, (!language.is_empty()).then_some(*language))
                    else {
                        return false;
                    };
                    key
                } else {
                    format!("{kind}/english")
                };
                let endpoint = &summary["endpoints"][key];
                endpoint["source_document_id"]
                    .as_i64()
                    .is_some_and(|id| id > 0)
                    && (!MIRRORED_ASSET_KINDS.contains(&kind.as_str())
                        || endpoint["snapshots"]
                            .as_u64()
                            .is_some_and(|count| count > 0))
            })
        })
}

pub fn prepare_assets(
    kind: &str,
    response: SourceHttpResponse,
    drift: Option<&DriftReport>,
) -> Result<SourceIr> {
    let contract = consumed_contract(kind)?;
    let mut ir = SourceIr::from_http(SOURCE, PARSER_REVISION, response)?;
    ir.set_derivation_family("deadlock-game-assets");
    let schema = OpenApiSnapshot::pinned()?;
    ir.pin_schema(&schema.schema_sha256);
    ir.pin_schema_version(&schema.api_version)?;
    if let Ok(payload) = ir.payload() {
        let validation = validate_consumed(&contract, payload);
        let identities = snapshots_for(kind, payload);
        match validation {
            Ok(extra) => ir.note_extra_fields(extra),
            Err(errors) => {
                for error in errors {
                    ir.quarantine(error);
                }
            }
        }
        if identities.is_err() {
            ir.quarantine("missing_duplicate_or_invalid_entity_identity");
        }
    }
    if let Some(report) = drift {
        report.gate(kind, &mut ir);
    }
    Ok(ir)
}

pub fn consumed_contract(kind: &str) -> Result<Value> {
    let id = json!({"type":"integer","minimum":0});
    let text = json!({"type":"string","minLength":1});
    let entry = match kind {
        "items" | "heroes" | "heroes_all" => {
            json!({"type":"object","required":["id"],"properties":{"id":id,"name":{"type":"string"},"class_name":text}})
        }
        "ranks" => {
            json!({"type":"object","required":["tier","name"],"properties":{"tier":id,"name":text}})
        }
        "build_tags" => {
            json!({"type":"object","required":["id","class_name","label"],"properties":{"id":id,"class_name":text,"label":text}})
        }
        "npc_units" | "misc_entities" | "modifiers" => {
            json!({"type":"object","required":["id","class_name"],"properties":{"id":id,"class_name":text}})
        }
        "generic_data" => return Ok(json!({"type":"object"})),
        "colors" => {
            return Ok(
                json!({"type":"object","additionalProperties":{"type":"object","required":["red","green","blue","alpha"],"properties":{"red":id,"green":id,"blue":id,"alpha":id}}}),
            )
        }
        _ => {
            return Err(SourcesError::invalid_input(
                "unknown/retired asset contract",
            ))
        }
    };
    Ok(json!({"type":"array","items":entry}))
}
fn endpoint_path(kind: &str) -> Option<&'static str> {
    ENDPOINTS
        .iter()
        .find_map(|(candidate, path)| (*candidate == kind).then_some(*path))
}
#[derive(Debug, Clone, PartialEq)]
pub struct AssetEntity {
    pub entity_type: String,
    pub external_id: String,
    pub canonical_name: Option<String>,
    pub payload: Value,
}

pub fn asset_entities(kind: &str, payload: &Value) -> Result<Vec<AssetEntity>> {
    Ok(snapshots_for(kind, payload)?
        .into_iter()
        .map(|s| AssetEntity {
            entity_type: s.entity_type,
            external_id: s.external_id,
            canonical_name: s.canonical_name,
            payload: s.payload,
        })
        .collect())
}

fn snapshots_for(kind: &str, payload: &Value) -> Result<Vec<EntitySnapshotInput>> {
    if kind == "generic_data" {
        if !payload.as_object().is_some_and(|object| !object.is_empty()) {
            return Err(SourcesError::invalid_input(
                "Globale Spieldaten fehlen oder haben den falschen Datentyp",
            ));
        }
        return Ok(vec![EntitySnapshotInput {
            source: SOURCE.into(),
            entity_type: "generic_data".into(),
            external_id: "generic_data".into(),
            canonical_name: None,
            payload: payload.clone(),
        }]);
    }
    let entity_type = match kind {
        "items" => "item_or_ability",
        "heroes" | "heroes_all" => "hero",
        "ranks" => "rank",
        "build_tags" => "build_tag",
        "npc_units" => "npc_unit",
        "misc_entities" => "misc_entity",
        "modifiers" => "modifier",
        "colors" => "color",
        _ => return Err(SourcesError::invalid_input("unknown asset entity type")),
    };
    let entries: Vec<(String, &Value)> = if kind == "colors" {
        let object = payload
            .as_object()
            .ok_or_else(|| SourcesError::invalid_input("colors must be an object"))?;
        object
            .iter()
            .map(|(key, value)| (key.clone(), value))
            .collect()
    } else {
        let array = payload
            .as_array()
            .ok_or_else(|| SourcesError::invalid_input("assets must be an array"))?;
        if MIRRORED_ASSET_KINDS.contains(&kind) && array.is_empty() {
            return Err(SourcesError::invalid_input(
                "Pflicht-Assets dürfen nicht leer sein",
            ));
        }
        array
            .iter()
            .map(|value| {
                let field = if kind == "ranks" { "tier" } else { "id" };
                value
                    .get(field)
                    .and_then(Value::as_u64)
                    .map(|id| (id.to_string(), value))
                    .ok_or_else(|| {
                        SourcesError::invalid_input(
                            "asset ID missing/wrong type; positional fallback is forbidden",
                        )
                    })
            })
            .collect::<Result<Vec<_>>>()?
    };
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for (id, entry) in entries {
        if id.trim().is_empty() || !entry.is_object() || !seen.insert(id.clone()) {
            return Err(SourcesError::invalid_input(
                "duplicate/invalid asset identity",
            ));
        }
        let name = ["name", "label", "class_name"]
            .iter()
            .find_map(|key| entry.get(key).and_then(Value::as_str))
            .map(str::to_owned);
        out.push(EntitySnapshotInput {
            source: SOURCE.into(),
            entity_type: entity_type.into(),
            external_id: id,
            canonical_name: name,
            payload: entry.clone(),
        });
    }
    Ok(out)
}

#[cfg(test)]
#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

#[cfg(test)]
#[path = "asset_receipt_tests.rs"]
mod receipt_tests;

#[cfg(test)]
mod tests {
    use super::*;
    fn response(raw: &[u8]) -> SourceHttpResponse {
        SourceHttpResponse {
            url: format!("{BASE_URL}/v1/assets/items"),
            status: 200,
            content: raw.into(),
            headers: std::collections::BTreeMap::from([(
                "content-type".into(),
                "application/json".into(),
            )]),
            observed_at: 0,
            attempts: 1,
        }
    }
    #[test]
    fn current_paths_and_retired_kinds() {
        assert_eq!(
            resolve_kinds(&[]).unwrap(),
            vec![
                ("items".into(), "/v1/assets/items"),
                ("heroes".into(), "/v1/assets/heroes?only_active=true"),
                ("heroes_all".into(), "/v1/assets/heroes"),
                ("generic_data".into(), "/v1/assets/generic-data"),
                ("npc_units".into(), "/v1/assets/npc-units"),
                ("misc_entities".into(), "/v1/assets/misc-entities"),
                ("modifiers".into(), "/v1/assets/modifiers")
            ]
        );
        for kind in ["raw_items", "raw_heroes", "bogus"] {
            assert!(resolve_kinds(&[kind.into()]).is_err());
        }
        assert_eq!(
            resolve_kinds(&["items".into(), "items".into()])
                .unwrap()
                .len(),
            1
        );
    }
    #[test]
    fn version_language_and_daily_refresh_are_explicit() {
        assert_eq!(asset_url("heroes", ENDPOINTS[1].1, 6759, "german"),
            "https://api.deadlock-api.com/v1/assets/heroes?only_active=true&client_version=6759&language=german");
        assert_eq!(
            asset_url("items", ENDPOINTS[0].1, 6757, "english"),
            "https://api.deadlock-api.com/v1/assets/items?client_version=6757&language=english"
        );
        let selected = resolve_kinds(&["items".into()]).unwrap();
        let mut summary = json!({"mirrored_at":1000,"endpoints":{
            "items/english":{"source_document_id":1,"snapshots":1},
            "items/german":{"source_document_id":2,"snapshots":1}}});
        assert!(mirror_is_fresh(&summary, &selected, 1001));
        assert!(!mirror_is_fresh(&summary, &selected, 87400));
        assert!(!mirror_is_fresh(&summary, &selected, 999));
        assert!(!mirror_is_fresh(
            &summary,
            &resolve_kinds(&[]).unwrap(),
            1001
        ));
        summary["endpoints"]["items/german"]["snapshots"] = json!(0);
        assert!(!mirror_is_fresh(&summary, &selected, 1001));
        summary["endpoints"]["items/german"]
            .as_object_mut()
            .unwrap()
            .remove("snapshots");
        assert!(!mirror_is_fresh(&summary, &selected, 1001));
    }
    #[test]
    fn zero_is_an_id_not_a_fallback_and_parser_preserves_bytes() {
        let raw = b" [{\"id\":0, \"name\":\"Synthetic\",\"extra\":true}]\n";
        let ir = prepare_assets("items", response(raw), None).unwrap();
        assert!(!ir.is_quarantined());
        assert_eq!(ir.raw(), raw);
        assert_eq!(
            snapshots_for("items", ir.payload().unwrap()).unwrap()[0].external_id,
            "0"
        );
        assert!(ir.metadata()["validation"]["extra_fields"]
            .as_array()
            .unwrap()
            .contains(&json!("/0/extra")));
    }
    #[test]
    fn malformed_missing_wrong_container_and_duplicate_quarantine() {
        for raw in [
            b"{broken".as_slice(),
            b"{}",
            b"[null]",
            b"[{\"name\":\"No id\"}]",
            b"[{\"id\":false}]",
            b"[{\"id\":-1}]",
            b"[{\"id\":1},{\"id\":1}]",
        ] {
            let ir = prepare_assets("items", response(raw), None).unwrap();
            assert!(ir.is_quarantined(), "{raw:?}");
            assert_eq!(ir.raw(), raw);
            assert!(ir.payload().is_err());
        }
        for kind in DEFAULT_KINDS {
            let ir = prepare_assets(kind, response(b"[]"), None).unwrap();
            assert!(ir.is_quarantined(), "{kind}");
            assert_eq!(ir.raw(), b"[]");
            assert!(ir.payload().is_err());
            assert!(asset_entities(kind, &json!([])).is_err());
        }
        assert!(!prepare_assets("build_tags", response(b"[]"), None)
            .unwrap()
            .is_quarantined());
    }
    #[test]
    fn empty_asset_names_preserve_stable_identity_and_raw_values() {
        let raw = br#"[{"id":0,"name":"","class_name":"internal_ability"}]"#;
        for kind in DEFAULT_KINDS {
            let ir = prepare_assets(kind, response(raw), None).unwrap();
            assert!(!ir.is_quarantined());
            assert_eq!(ir.raw(), raw);
            assert_eq!(ir.payload().unwrap()[0]["name"], "");
            assert_eq!(
                asset_entities(kind, ir.payload().unwrap()).unwrap()[0].external_id,
                "0"
            );
            assert!(
                prepare_assets(kind, response(br#"[{"id":0,"name":false}]"#), None)
                    .unwrap()
                    .is_quarantined()
            );
        }
    }

    #[tokio::test]
    async fn persisted_asset_ir_roundtrips_through_shared_mirror_reader() {
        use brain_storage::asset_mirror::{latest_mirrored_client_version, load_mirrored_assets};
        use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

        let pg = scratch_pg::ScratchPg::start();
        let socket = pg.directory.join("socket");
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                PgConnectOptions::new_without_pgpass()
                    .host(socket.to_str().unwrap())
                    .port(55439)
                    .username("brain_core_test")
                    .database("postgres"),
            )
            .await
            .unwrap();
        let identity: (Option<String>, String, String) = sqlx::query_as(
            "SELECT inet_server_addr()::text, current_user::text, current_setting('data_directory')",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(identity.0, None);
        assert_eq!(identity.1, "brain_core_test");
        assert_eq!(std::path::Path::new(&identity.2), pg.directory.join("data"));
        sqlx::raw_sql("CREATE SCHEMA brain")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("wiki_scratch.sql"))
            .execute(&pool)
            .await
            .unwrap();
        let raw_dir = tempfile::tempdir().unwrap();
        let store = SourceStore::new(&pool, raw_dir.path()).unwrap();
        assert!(latest_mirrored_client_version(&pool).await.is_err());
        let mut versions = vec![(6757, None), (6759, None)];
        for kind in DEFAULT_KINDS {
            for language in LANGUAGES {
                versions.push((6761 + versions.len() as i64, Some((*kind, *language))));
            }
        }
        for (version, empty_endpoint) in versions {
            let run = store.begin_run("assets").await.unwrap();
            let mut endpoints = Map::new();
            let mut expected = Vec::new();
            for kind in DEFAULT_KINDS {
                for language in LANGUAGES {
                    let payload = json!([{"id":1998374645,"name":if *language == "german" {
                        "Mystischer Ausbruch" } else { "Mystic Burst" },
                        "properties":{"Damage":"40","Radius":"16m"},"extra":{"version":version}}]);
                    let raw = serde_json::to_vec(&payload).unwrap();
                    let mut transport = response(&raw);
                    transport.url =
                        asset_url(kind, endpoint_path(kind).unwrap(), version, language);
                    let ir = prepare_assets(kind, transport, None).unwrap();
                    assert!(!ir.is_quarantined());
                    let key = format!("{version}/{kind}/{language}");
                    let id = store
                        .persist_ir(
                            &key,
                            &key,
                            &ir,
                            &json!({"client_version":version,"kind":kind,"language":language}),
                        )
                        .await
                        .unwrap();
                    let document: (String, String) = sqlx::query_as(
                        "SELECT raw_path, content_hash FROM brain.source_documents WHERE id=$1",
                    )
                    .bind(id)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
                    assert_eq!(std::fs::read(document.0).unwrap(), raw);
                    assert_eq!(document.1, ir.provenance().raw_sha256);
                    if empty_endpoint == Some((*kind, *language)) {
                        sqlx::query(
                            "UPDATE brain.source_documents SET metadata=jsonb_set(metadata, \
                             '{contract,data,payload,value}', '[]'::jsonb) WHERE id=$1",
                        )
                        .bind(id)
                        .execute(&pool)
                        .await
                        .unwrap();
                    }
                    endpoints.insert(
                        format!("{kind}/{language}"),
                        json!({"source_document_id":id}),
                    );
                    expected.push((*kind, *language, payload));
                }
            }
            assert!(load_mirrored_assets(&pool, version, "items", "english")
                .await
                .is_err());
            store
                .finish_run(
                    run,
                    "ok",
                    &json!({"client_version":version,
                "mirror_complete":true,"endpoints":endpoints}),
                )
                .await
                .unwrap();
            assert_eq!(
                latest_mirrored_client_version(&pool).await.unwrap(),
                if empty_endpoint.is_some() {
                    6759
                } else {
                    version
                }
            );
            for (kind, language, payload) in expected {
                let loaded = load_mirrored_assets(&pool, version, kind, language).await;
                if empty_endpoint.is_some() {
                    assert!(loaded.is_err(), "{version}/{kind}/{language}");
                } else {
                    assert_eq!(loaded.unwrap(), payload);
                }
            }
        }
        let run = store.begin_run("assets").await.unwrap();
        let quarantined = prepare_assets("items", response(br#"[{"id":false}]"#), None).unwrap();
        assert!(store
            .persist_ir(
                "6760/items/english",
                "quarantined",
                &quarantined,
                &json!({"client_version":6760,"kind":"items","language":"english"})
            )
            .await
            .is_err());
        store
            .finish_run(
                run,
                "error",
                &json!({"client_version":6760,"mirror_complete":false}),
            )
            .await
            .unwrap();
        assert_eq!(latest_mirrored_client_version(&pool).await.unwrap(), 6759);
        assert!(load_mirrored_assets(&pool, 6760, "items", "english")
            .await
            .is_err());
        for version in [6757, 6759] {
            assert_eq!(
                load_mirrored_assets(&pool, version, "items", "german")
                    .await
                    .unwrap()[0]["extra"]["version"],
                version
            );
        }
        pool.close().await;
    }

    #[test]
    #[ignore = "explizite öffentliche Assets-Probe ohne DB oder Nutzerdaten"]
    fn live_versioned_game_assets_contract() {
        assert_eq!(
            std::env::var("DBRAIN_EXTERNAL_LIVE_CONTRACT").as_deref(),
            Ok("1")
        );
        let temp = tempfile::tempdir().unwrap();
        let http = HttpClient::new("Deadlock-Brain-Contract/1", temp.path()).unwrap();
        let options = SourceHttpOptions {
            attempts: 1,
            ..Default::default()
        };
        let manifest = SourceIr::from_http(
            SOURCE,
            PARSER_REVISION,
            http.get_bounded(&format!("{BASE_URL}/v1/assets/steam-info"), options.clone())
                .unwrap(),
        )
        .unwrap();
        let version = manifest.payload().unwrap()["client_version"]
            .as_i64()
            .unwrap();
        assert!(version > 0);
        let schema = http
            .get_bounded(
                OPENAPI_URL,
                SourceHttpOptions {
                    max_bytes: 1024 * 1024,
                    ..options.clone()
                },
            )
            .unwrap();
        schema.ensure_success().unwrap();
        let report = OpenApiSnapshot::pinned()
            .unwrap()
            .compare(
                &OpenApiSnapshot::from_raw(schema.content).unwrap(),
                &asset_dependencies(),
            )
            .unwrap();
        for (kind, endpoint) in resolve_kinds(&[]).unwrap() {
            for language in mirrored_asset_languages(&kind).unwrap() {
                let response = http
                    .get_bounded(
                        &asset_url(&kind, endpoint, version, language),
                        options.clone(),
                    )
                    .unwrap();
                let ir = prepare_assets(&kind, response, Some(&report)).unwrap();
                assert!(
                    !ir.is_quarantined(),
                    "{kind}/{language}: {:?}",
                    ir.validation()
                );
                let entities = asset_entities(&kind, ir.payload().unwrap()).unwrap();
                assert!(!entities.is_empty());
                println!("client_version={version} kind={kind} language={language} bytes={} entities={} raw_sha256={}",
                    ir.raw().len(), entities.len(), ir.provenance().raw_sha256);
            }
        }
    }

    #[test]
    fn current_color_map_and_rank_tier_have_stable_identities() {
        let colors = json!({"neutral":{"red":1,"green":2,"blue":3,"alpha":255}});
        let ir = prepare_assets("colors", response(colors.to_string().as_bytes()), None).unwrap();
        assert!(!ir.is_quarantined());
        assert_eq!(
            snapshots_for("colors", ir.payload().unwrap()).unwrap()[0].external_id,
            "neutral"
        );
        assert_eq!(
            snapshots_for("ranks", &json!([{"tier":0,"name":"Unranked"}])).unwrap()[0].external_id,
            "0"
        );
        assert_eq!(
            snapshots_for("heroes_all", &json!([{"id":1}])).unwrap()[0].entity_type,
            "hero"
        );
    }
}
