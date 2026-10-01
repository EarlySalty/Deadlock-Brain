use crate::{
    external::SourceIr,
    schema_watch::{
        asset_dependencies, validate_consumed, DriftReport, OpenApiSnapshot, OPENAPI_URL,
    },
    store::{complete_run, open_pool, EntitySnapshotInput, SourceStore},
    Result, SourcesError,
};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions, SourceHttpResponse};
use serde_json::{json, Map, Value};
use std::{collections::BTreeSet, path::Path};

pub const SOURCE: &str = "deadlock_assets_api";
pub const PARSER_REVISION: &str = "dbrain-assets/2";
// Keep the current main endpoint fix; do not revive the retired assets host.
pub const BASE_URL: &str = "https://api.deadlock-api.com";
pub const ENDPOINTS: &[(&str, &str)] = &[
    ("items", "/v1/assets/items"),
    ("heroes", "/v1/assets/heroes?only_active=true"),
    ("heroes_all", "/v1/assets/heroes"),
    ("ranks", "/v1/assets/ranks"),
    ("colors", "/v1/assets/colors"),
    ("build_tags", "/v1/assets/build-tags"),
    ("npc_units", "/v1/assets/npc-units"),
];
const RETIRED_KINDS: &[&str] = &["raw_items", "raw_heroes"];
const DEFAULT_KINDS: &[&str] = &["items", "heroes"];

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
        DEFAULT_KINDS.iter().map(|k| k.to_string()).collect()
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
    // One small schema request, not an automatic baseline upgrade or bulk job.
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
        let response = http.get_bounded(
            &format!("{BASE_URL}{endpoint}"),
            SourceHttpOptions::default(),
        )?;
        let ir = prepare_assets(kind, response, Some(&report))?;
        let title = format!("Deadlock Assets API {kind}");
        match store.persist_ir(kind, &title, &ir, &json!({"endpoint":endpoint,"schema_report":report,"contract_coverage":"container_and_consumed_fields"})).await {
            Ok(document_id) => staged.push((kind.clone(), ir, document_id)),
            Err(error) => { if first_error.is_none() { first_error=Some(error); } }
        }
    }
    // No EntitySnapshot is written before all selected payloads pass preflight.
    if let Some(error) = first_error {
        return Err(error);
    }
    let mut summary = Map::new();
    let mut total = 0usize;
    for (kind, ir, document_id) in staged {
        let snapshots = snapshots_for(&kind, ir.payload()?)?;
        let count = store
            .insert_many_snapshots(&snapshots, Some(document_id))
            .await?;
        summary.insert(kind, json!({"url":ir.provenance().locator,"snapshots":count,"raw_sha256":ir.provenance().raw_sha256,"source_document_id":document_id,"validation":ir.validation()}));
        total += count;
    }
    Ok(json!({"endpoints":summary,"snapshots":total,"schema_drift":report}))
}

/// DB-free adapter path used by fixtures and the opt-in small live contract test.
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
            json!({"type":"object","required":["id"],"properties":{"id":id,"name":text,"class_name":text}})
        }
        "ranks" => {
            json!({"type":"object","required":["tier","name"],"properties":{"tier":id,"name":text}})
        }
        "build_tags" => {
            json!({"type":"object","required":["id","class_name","label"],"properties":{"id":id,"class_name":text,"label":text}})
        }
        "npc_units" => {
            json!({"type":"object","required":["id","class_name"],"properties":{"id":id,"class_name":text}})
        }
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
    let entity_type = match kind {
        "items" => "item_or_ability",
        "heroes" | "heroes_all" => "hero",
        "ranks" => "rank",
        "build_tags" => "build_tag",
        "npc_units" => "npc_unit",
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
                ("heroes".into(), "/v1/assets/heroes?only_active=true")
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
        assert!(!prepare_assets("items", response(b"[]"), None)
            .unwrap()
            .is_quarantined());
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
