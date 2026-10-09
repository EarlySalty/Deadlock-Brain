use brain_storage::asset_mirror::{AssetMirrorReceipt, MirroredAssets};
use serde_json::Value;

use crate::{
    publish::BuildDataOrigin, CalculationModels, ModelSource, ReasonerCtx, ReasonerError, Result,
};

pub struct MirroredCalculationModels {
    pub models: CalculationModels,
    pub origin: BuildDataOrigin,
    pub checked_at: i64,
}

pub async fn load_calculation_models_from_mirror(ctx: &ReasonerCtx) -> Result<CalculationModels> {
    Ok(load_calculation_models_with_origin_from_mirror(ctx)
        .await?
        .models)
}

pub async fn load_calculation_models_with_origin_from_mirror(
    ctx: &ReasonerCtx,
) -> Result<MirroredCalculationModels> {
    let client_version = brain_storage::asset_mirror::latest_mirrored_client_version(&ctx.pool)
        .await
        .map_err(|error| ReasonerError::Data(format!("API-Spiegel: {error}")))?;
    let heroes = brain_storage::asset_mirror::load_mirrored_assets_with_receipt(
        &ctx.pool,
        client_version,
        "heroes",
        Some("english"),
    )
    .await
    .map_err(|error| ReasonerError::Data(format!("API-Helden: {error}")))?;
    let items = brain_storage::asset_mirror::load_mirrored_assets_for_run(
        &ctx.pool,
        heroes.receipt.source_run_id,
        client_version,
        "items",
        Some("english"),
    )
    .await
    .map_err(|error| ReasonerError::Data(format!("API-Items: {error}")))?;
    let run: Value = sqlx::query_scalar(
        "SELECT jsonb_build_object('id',id,'status',status,'summary',summary) FROM brain.source_runs WHERE source='assets' ORDER BY id DESC LIMIT 1",
    )
    .fetch_one(&ctx.pool)
    .await
    .map_err(ReasonerError::Db)?;
    models_from_receipts(heroes, items, &run)
}

fn models_from_receipts(
    heroes: MirroredAssets,
    items: MirroredAssets,
    run: &Value,
) -> Result<MirroredCalculationModels> {
    let origin = BuildDataOrigin::from_receipts(&heroes.receipt, &items.receipt)?;
    let checked_at = current_mirror_check(run, &origin)?;
    let models = crate::calculation_models_from_payloads(
        &heroes.payload,
        &items.payload,
        &model_source(&heroes.receipt),
        &model_source(&items.receipt),
    )?;
    Ok(MirroredCalculationModels {
        models,
        origin,
        checked_at,
    })
}

fn model_source(receipt: &AssetMirrorReceipt) -> ModelSource {
    ModelSource {
        client_version: receipt.client_version,
        document_id: receipt.endpoint.source_document_id.to_string(),
        original_url: receipt.endpoint.url.clone(),
        kind: receipt.kind.clone(),
        language: receipt.language.clone().unwrap_or_default(),
        json_pointer: String::new(),
    }
}

fn current_mirror_check(run: &Value, origin: &BuildDataOrigin) -> Result<i64> {
    origin.validate()?;
    let summary = &run["summary"];
    let reused = summary["reused_local_mirror"].as_bool() == Some(true);
    let run_matches = run["id"]
        .as_i64()
        .is_some_and(|id| id == origin.source_run_id || reused && id > origin.source_run_id);
    let endpoint_matches = |key: &str, id: i64, hash: &str| {
        summary["endpoints"][key]["source_document_id"].as_i64() == Some(id)
            && summary["endpoints"][key]["raw_sha256"].as_str() == Some(hash)
    };
    let checked_at = summary["checked_at"].as_i64();
    if run["status"].as_str() != Some("ok")
        || summary["mirror_complete"].as_bool() != Some(true)
        || !run_matches
        || summary["client_version"].as_i64() != Some(origin.client_version)
        || summary["mirrored_at"].as_i64() != Some(origin.mirrored_at)
        || summary["parser_revision"].as_str() != Some(origin.parser_revision.as_str())
        || summary["manifest_document_id"].as_i64() != Some(origin.manifest_document_id)
        || summary["manifest_raw_sha256"].as_str() != Some(origin.manifest_sha256.as_str())
        || !endpoint_matches(
            "heroes/english",
            origin.heroes_document_id,
            &origin.heroes_sha256,
        )
        || !endpoint_matches(
            "items/english",
            origin.items_document_id,
            &origin.items_sha256,
        )
        || checked_at.is_none_or(|at| at < origin.mirrored_at)
    {
        return Err(ReasonerError::Data(
            "Neuester API-Abgleich bestätigt nicht die gebundenen Originalspieldaten.".into(),
        ));
    }
    checked_at.ok_or_else(|| ReasonerError::Data("API-Prüfzeitpunkt fehlt.".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn origin() -> BuildDataOrigin {
        BuildDataOrigin {
            client_version: 6759,
            source_run_id: 10,
            mirrored_at: 100,
            parser_revision: "assets-test/1".into(),
            manifest_document_id: 20,
            manifest_sha256: "a".repeat(64),
            heroes_document_id: 21,
            heroes_sha256: "b".repeat(64),
            items_document_id: 22,
            items_sha256: "c".repeat(64),
        }
    }

    fn run(origin: &BuildDataOrigin) -> Value {
        json!({"id":origin.source_run_id,"status":"ok","summary":{
            "client_version":origin.client_version,"mirror_complete":true,
            "mirrored_at":origin.mirrored_at,"checked_at":origin.mirrored_at,
            "parser_revision":origin.parser_revision,
            "manifest_document_id":origin.manifest_document_id,
            "manifest_raw_sha256":origin.manifest_sha256,
            "reused_local_mirror":false,"endpoints":{
                "heroes/english":{"source_document_id":origin.heroes_document_id,"raw_sha256":origin.heroes_sha256},
                "items/english":{"source_document_id":origin.items_document_id,"raw_sha256":origin.items_sha256}
            }
        }})
    }

    #[test]
    fn current_mirror_rejects_failed_incomplete_or_different_originals() {
        let origin = origin();
        let valid = run(&origin);
        assert_eq!(current_mirror_check(&valid, &origin).unwrap(), 100);
        for (pointer, value) in [
            ("/status", json!("failed")),
            ("/id", json!(11)),
            ("/summary/mirror_complete", json!(false)),
            ("/summary/client_version", json!(6760)),
            ("/summary/mirrored_at", json!(101)),
            ("/summary/checked_at", json!(99)),
            ("/summary/checked_at", Value::Null),
            ("/summary/parser_revision", json!("assets-test/2")),
            ("/summary/manifest_document_id", json!(30)),
            ("/summary/manifest_raw_sha256", json!("d".repeat(64))),
            (
                "/summary/endpoints/heroes~1english/source_document_id",
                json!(31),
            ),
            (
                "/summary/endpoints/heroes~1english/raw_sha256",
                json!("d".repeat(64)),
            ),
            (
                "/summary/endpoints/items~1english/source_document_id",
                json!(32),
            ),
            (
                "/summary/endpoints/items~1english/raw_sha256",
                json!("d".repeat(64)),
            ),
        ] {
            let mut changed = valid.clone();
            *changed.pointer_mut(pointer).unwrap() = value;
            assert!(
                current_mirror_check(&changed, &origin).is_err(),
                "{pointer}"
            );
        }
    }

    #[test]
    fn fresh_recheck_keeps_original_run_but_cannot_replace_bound_documents() {
        let origin = origin();
        let mut checked = run(&origin);
        checked["id"] = json!(11);
        checked["summary"]["reused_local_mirror"] = json!(true);
        checked["summary"]["checked_at"] = json!(120);
        assert_eq!(current_mirror_check(&checked, &origin).unwrap(), 120);
        for pointer in [
            "/summary/endpoints/heroes~1english/raw_sha256",
            "/summary/endpoints/items~1english/raw_sha256",
            "/summary/manifest_raw_sha256",
        ] {
            let mut changed = checked.clone();
            *changed.pointer_mut(pointer).unwrap() = json!("d".repeat(64));
            assert!(current_mirror_check(&changed, &origin).is_err());
        }
        checked["id"] = json!(9);
        assert!(current_mirror_check(&checked, &origin).is_err());
    }

    #[test]
    fn invalid_origin_cannot_be_confirmed_by_matching_run_json() {
        for pointer in [
            "/client_version",
            "/source_run_id",
            "/mirrored_at",
            "/manifest_document_id",
            "/heroes_document_id",
            "/items_document_id",
        ] {
            let mut raw = serde_json::to_value(origin()).unwrap();
            *raw.pointer_mut(pointer).unwrap() = json!(0);
            let invalid = serde_json::from_value::<BuildDataOrigin>(raw).unwrap();
            assert!(current_mirror_check(&run(&invalid), &invalid).is_err());
        }
        let mut invalid = origin();
        invalid.items_sha256 = "not-a-hash".into();
        assert!(current_mirror_check(&run(&invalid), &invalid).is_err());
    }
}
