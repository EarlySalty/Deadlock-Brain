use serde_json::Value;
use sqlx::PgPool;

use crate::{
    BuildItem, BuildObject, EvidenceKind, ReasonerError, Result, SituationBlock, SituationKind,
};
use dbrain_builds::spec::{AbilityOrderEntry, BuildSpecCategory, BuildSpecMod, BuildSpecPayload};

fn category_name(block: &SituationBlock) -> String {
    match block.kind {
        SituationKind::CanBuyN(count) => format!("Can buy {count}"),
        _ => block.label.clone(),
    }
}

fn category_description(items: &[BuildItem]) -> Option<String> {
    let details = items
        .iter()
        .flat_map(|item| item.sources.iter())
        .filter(|source| matches!(source.kind, EvidenceKind::Patch | EvidenceKind::Mechanic))
        .map(|source| source.detail.trim())
        .filter(|detail| !detail.is_empty())
        .take(3)
        .collect::<Vec<_>>();
    (!details.is_empty()).then(|| details.join("; "))
}

fn mod_spec(item: &BuildItem) -> BuildSpecMod {
    BuildSpecMod {
        ability_id: item.item_id,
        annotation: item.why.clone(),
        imbue: item.imbue_target,
        sell_priority: item.sell_priority,
    }
}

pub fn to_publish_payload(build: &BuildObject) -> BuildSpecPayload {
    let core = BuildSpecCategory {
        name: "Core".to_string(),
        optional: false,
        description: (!build.rationale.trim().is_empty())
            .then(|| build.rationale.clone())
            .or_else(|| category_description(&build.core)),
        width: Some(780.0),
        height: Some(260.0),
        mods: build.core.iter().map(mod_spec).collect(),
    };
    let situations = build
        .situations
        .iter()
        .map(|block| BuildSpecCategory {
            name: category_name(block),
            optional: block.optional,
            description: category_description(&block.items),
            width: Some(780.0),
            height: Some(260.0),
            mods: block.items.iter().map(mod_spec).collect(),
        })
        .collect::<Vec<_>>();
    let ability_order = (!build.ability_order.is_empty()).then(|| {
        build
            .ability_order
            .iter()
            .map(|step| AbilityOrderEntry {
                ability_id: step.ability_id,
                currency_type: step.currency_type,
                delta: step.delta,
            })
            .collect()
    });
    BuildSpecPayload {
        hero_id: build.hero_id,
        name: build.name.clone(),
        description: build.rationale.clone(),
        language: 1,
        mod_categories: std::iter::once(core).chain(situations).collect(),
        ability_order,
    }
}

pub fn publish_task_payload(build: &BuildObject) -> Value {
    serde_json::to_value(to_publish_payload(build)).expect("BuildSpecPayload is serializable")
}

pub async fn enqueue_publish_task(pool: &PgPool, build: &BuildObject) -> Result<i64> {
    let payload = publish_task_payload(build);
    sqlx::query_scalar::<_, i64>("INSERT INTO steam.steam_tasks(type, payload, status) VALUES('BUILD_PUBLISH_ORIGINAL', $1, 'PENDING') RETURNING id")
        .bind(payload)
        .fetch_one(pool)
        .await
        .map_err(ReasonerError::Db)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BuildItem, BuyPhase, Confidence, Evidence, EvidenceKind};

    fn item(id: i64, imbue: Option<i64>, sell: Option<u32>) -> BuildItem {
        BuildItem {
            item_id: id,
            name: format!("Item {id}"),
            tier: 2,
            buy_phase: BuyPhase::Core,
            why: "Warum".to_string(),
            confidence: Confidence::High,
            imbue_target: imbue,
            sell_priority: sell,
            sources: vec![Evidence {
                kind: EvidenceKind::Mechanic,
                detail: "Mechanik".to_string(),
            }],
        }
    }

    #[test]
    fn publish_roundtrip_keeps_annotations_imbue_sell_and_layout() {
        let build = BuildObject {
            hero_id: 25,
            hero_name: "Warden".to_string(),
            patch_tag: "current".to_string(),
            name: "Warden test".to_string(),
            core: vec![item(10, Some(100), Some(2))],
            situations: vec![SituationBlock {
                label: "Can buy 1".to_string(),
                optional: true,
                kind: SituationKind::CanBuyN(1),
                items: vec![item(11, None, None)],
            }],
            ability_order: vec![crate::AbilityStep {
                ability_id: 100,
                currency_type: 1,
                delta: 1,
            }],
            confidence: Confidence::High,
            rationale: "Rationale".to_string(),
        };
        let payload = to_publish_payload(&build);
        assert_eq!(payload.mod_categories[0].mods[0].imbue, Some(100));
        assert_eq!(payload.mod_categories[0].mods[0].sell_priority, Some(2));
        assert_eq!(payload.mod_categories[1].name, "Can buy 1");
        assert_eq!(payload.mod_categories[1].width, Some(780.0));
        assert_eq!(payload.mod_categories[1].height, Some(260.0));
        assert_eq!(payload.ability_order.as_ref().unwrap()[0].ability_id, 100);
        let json = publish_task_payload(&build);
        assert_eq!(json["mod_categories"][0]["mods"][0]["annotation"], "Warum");
        assert_eq!(json["mod_categories"][0]["mods"][0]["imbue"], 100);
    }
}
