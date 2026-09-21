use serde_json::Value;
use sqlx::PgPool;

use crate::{
    BuildItem, BuildObject, EvidenceKind, ReasonerError, Result, SituationBlock, SituationKind,
};
use dbrain_builds::spec::{AbilityOrderEntry, BuildSpecCategory, BuildSpecMod, BuildSpecPayload};

fn category_name(block: &SituationBlock) -> String {
    match block.kind {
        SituationKind::CanBuyN(1) => "Ein Item nach Bedarf".to_string(),
        SituationKind::CanBuyN(count) => format!("Bis zu {count} Items nach Bedarf"),
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
    (!details.is_empty()).then(|| {
        let description = details
            .join("; ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if description.chars().count() > 400 {
            format!("{}…", description.chars().take(399).collect::<String>())
        } else {
            description
        }
    })
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
        name: "Kern".to_string(),
        optional: false,
        description: category_description(&build.core),
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

/// Validation precedes any queue write. A multi-family response cannot silently
/// publish only its dominant child through the legacy single-build endpoint.
pub fn validate_publish_input(build: &BuildObject) -> Result<()> {
    if !build.variants.is_empty() {
        return Err(ReasonerError::Data("Mehrere Buildfamilien: explizite Auswahl und Abnahme einer Variante erforderlich; der Einzelbuild-Publisher darf Varianten nicht still verwerfen.".into()));
    }
    let family = build.family.as_ref().ok_or_else(|| {
        ReasonerError::Data(
            "Keine belegte Buildfamilie: Legacy-Eingaben ohne aktuelle Familien-/Patch-Abnahme dürfen nicht veröffentlicht werden."
                .into(),
        )
    })?;
    // A diagnostic clustering policy may require fewer rows in tests or local
    // experiments. It cannot lower the existing production publication floor.
    let minimum = build
        .family_discovery
        .as_ref()
        .map_or(100, |discovery| discovery.policy.min_matches.max(100));
    if !family.eligible_for_planning
        || family
            .post_patch_player_matches
            .is_none_or(|count| count < minimum)
    {
        return Err(ReasonerError::Data(format!("Familie {} ist nicht zur Veröffentlichung freigegeben: Nach-Patch-Stichprobe {:?}, mindestens {minimum} erforderlich. Historischer Populations-Support ist keine aktuelle Patch-Abnahme.", family.id, family.post_patch_player_matches)));
    }
    if build.core.is_empty() || matches!(build.confidence, crate::Confidence::Low) {
        return Err(ReasonerError::Data(format!("Familie {}: fehlender Core oder unzureichend abgesicherte Mechanik-/Datengrundlage; keine Steam-Veröffentlichung.",family.id)));
    }
    if build.ability_order.is_empty() || build.patch_tag.trim().is_empty() {
        return Err(ReasonerError::Data(format!(
            "Familie {}: fehlende Skillorder oder Patch-Provenienz; keine Steam-Veröffentlichung.",
            family.id
        )));
    }
    Ok(())
}

pub async fn enqueue_publish_task(pool: &PgPool, build: &BuildObject) -> Result<i64> {
    validate_publish_input(build)?;
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
            family: None,
            variants: Vec::new(),
            family_discovery: None,
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
        assert_eq!(payload.mod_categories[0].name, "Kern");
        assert_eq!(
            payload.mod_categories[0].description.as_deref(),
            Some("Mechanik")
        );
        assert_eq!(payload.description, "Rationale");
        assert_eq!(payload.mod_categories[1].name, "Ein Item nach Bedarf");
        assert_eq!(payload.mod_categories[1].width, Some(780.0));
        assert_eq!(payload.mod_categories[1].height, Some(260.0));
        assert_eq!(payload.ability_order.as_ref().unwrap()[0].ability_id, 100);
        let json = publish_task_payload(&build);
        assert_eq!(json["mod_categories"][0]["mods"][0]["annotation"], "Warum");
        assert_eq!(json["mod_categories"][0]["mods"][0]["imbue"], 100);
    }
}
