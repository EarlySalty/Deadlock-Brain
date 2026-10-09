use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BuildDataOrigin {
    pub client_version: i64,
    pub source_run_id: i64,
    pub mirrored_at: i64,
    pub parser_revision: String,
    pub manifest_document_id: i64,
    pub manifest_sha256: String,
    pub heroes_document_id: i64,
    pub heroes_sha256: String,
    pub items_document_id: i64,
    pub items_sha256: String,
}

impl BuildDataOrigin {
    pub(crate) fn snapshot_source(&self) -> Result<String> {
        self.validate()?;
        let origin = serde_json::to_string(self)
            .map_err(|error| ReasonerError::Data(format!("Buildherkunft: {error}")))?;
        Ok(format!("deadlock_assets_api/hero#build-data={origin}"))
    }

    pub(crate) fn validate(&self) -> Result<()> {
        let hash_valid =
            |hash: &str| hash.len() == 64 && hash.bytes().all(|byte| byte.is_ascii_hexdigit());
        if self.client_version <= 0
            || self.source_run_id <= 0
            || self.mirrored_at <= 0
            || self.parser_revision.trim().is_empty()
            || self.manifest_document_id <= 0
            || self.heroes_document_id <= 0
            || self.items_document_id <= 0
            || !hash_valid(&self.manifest_sha256)
            || !hash_valid(&self.heroes_sha256)
            || !hash_valid(&self.items_sha256)
        {
            return Err(ReasonerError::Data(
                "Originalspieldaten sind nicht vollständig belegt.".into(),
            ));
        }
        Ok(())
    }

    pub(crate) fn from_receipts(
        heroes: &brain_storage::asset_mirror::AssetMirrorReceipt,
        items: &brain_storage::asset_mirror::AssetMirrorReceipt,
    ) -> Result<Self> {
        if heroes.client_version != items.client_version
            || heroes.source_run_id != items.source_run_id
            || heroes.mirrored_at != items.mirrored_at
            || heroes.parser_revision != items.parser_revision
            || heroes.manifest.source_document_id != items.manifest.source_document_id
            || heroes.manifest.raw_sha256 != items.manifest.raw_sha256
            || heroes.kind != "heroes"
            || items.kind != "items"
            || heroes.language.as_deref() != Some("english")
            || items.language != heroes.language
        {
            return Err(ReasonerError::Data(
                "Originalspieldaten gehören nicht zum selben API-Abgleich.".into(),
            ));
        }
        let origin = Self {
            client_version: heroes.client_version,
            source_run_id: heroes.source_run_id,
            mirrored_at: heroes.mirrored_at,
            parser_revision: heroes.parser_revision.clone(),
            manifest_document_id: heroes.manifest.source_document_id,
            manifest_sha256: heroes.manifest.raw_sha256.clone(),
            heroes_document_id: heroes.endpoint.source_document_id,
            heroes_sha256: heroes.endpoint.raw_sha256.clone(),
            items_document_id: items.endpoint.source_document_id,
            items_sha256: items.endpoint.raw_sha256.clone(),
        };
        origin.validate()?;
        Ok(origin)
    }
}

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

#[cfg(test)]
mod catalog_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn publication_checks_actual_current_shop_entries_before_queue_or_http() {
        let payload = json!({"mod_categories":[{"mods":[{"ability_id":1}]}]});
        let catalog = json!([{"id":1,"name":"Any name","type":"upgrade","item_slot_type":"weapon","item_tier":1,"cost":800,"shopable":true}]);
        assert!(validate_publish_catalog(&payload, &catalog).is_ok());
        for (field, value) in [
            ("id", json!(2)),
            ("shopable", json!(false)),
            ("disabled", json!(true)),
            ("item_tier", json!(5)),
            ("cost", json!(0)),
            ("type", json!("ability")),
        ] {
            let mut changed = catalog.clone();
            changed[0][field] = value;
            assert!(
                validate_publish_catalog(&payload, &changed).is_err(),
                "{field}"
            );
        }
        assert!(validate_publish_catalog(&payload, &json!([])).is_err());
        assert!(validate_publish_catalog(&json!({"mod_categories":[{}]}), &catalog).is_err());
    }
}

pub fn validate_publish_catalog(payload: &Value, items: &Value) -> Result<()> {
    let rows = items
        .as_array()
        .ok_or_else(|| ReasonerError::Data("Aktueller Item-Katalog fehlt.".into()))?;
    let allowed: std::collections::BTreeSet<_> = rows
        .iter()
        .filter(|item| brain_contracts::game_mode::GameMode::Normal.allows_item(item))
        .filter_map(|item| item["id"].as_i64())
        .collect();
    let categories = payload["mod_categories"]
        .as_array()
        .ok_or_else(|| ReasonerError::Data("Item-Kategorien fehlen.".into()))?;
    for category in categories {
        let mods = category["mods"]
            .as_array()
            .ok_or_else(|| ReasonerError::Data("Item-Liste fehlt.".into()))?;
        for item in mods {
            if item["ability_id"]
                .as_i64()
                .is_none_or(|id| !allowed.contains(&id))
            {
                return Err(ReasonerError::Data("Build enthält ein Item, das aktuell nicht im Standardmodus kaufbar ist. Es wurde nichts veröffentlicht.".into()));
            }
        }
    }
    Ok(())
}

pub async fn validate_publish_items(pool: &PgPool, payload: &Value) -> Result<()> {
    let version = brain_storage::asset_mirror::latest_mirrored_client_version(pool)
        .await
        .map_err(|error| ReasonerError::Data(format!("API-Spiegel: {error}")))?;
    let items = brain_storage::asset_mirror::load_mirrored_assets_with_receipt(
        pool,
        version,
        "items",
        Some("english"),
    )
    .await
    .map_err(|error| ReasonerError::Data(format!("API-Items: {error}")))?;
    validate_publish_catalog(payload, &items.payload)
}

pub async fn enqueue_publish_task(pool: &PgPool, build: &BuildObject) -> Result<i64> {
    validate_publish_input(build)?;
    let payload = publish_task_payload(build);
    validate_publish_items(pool, &payload).await?;
    sqlx::query_scalar::<_, i64>("INSERT INTO steam.steam_tasks(type, payload, status) VALUES('BUILD_PUBLISH_ORIGINAL', $1, 'PENDING') RETURNING id")
        .bind(payload)
        .fetch_one(pool)
        .await
        .map_err(ReasonerError::Db)
}

/// Expliziter Review-Pfad für interaktive Brain-Tests. Er verändert die reguläre
/// Veröffentlichungsfreigabe nicht und kennzeichnet den Build sichtbar als Review.
fn truncate_review_text(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut out = text
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    out.push('…');
    out
}

pub fn review_publish_task_payload(build: &BuildObject) -> Result<Value> {
    if build.hero_id <= 0 || build.core.is_empty() {
        return Err(ReasonerError::Data(
            "Review-Build braucht einen gültigen Helden und mindestens ein Kern-Item.".into(),
        ));
    }
    let mut payload = to_publish_payload(build);
    payload.name = truncate_review_text(&format!("[REVIEW] {}", build.name), 80);
    payload.description = format!(
        "Experimentelles Brain Review Build. Nicht als reguläre Empfehlung freigegeben.\n{}",
        truncate_review_text(&build.rationale, 360)
    );
    for category in &mut payload.mod_categories {
        category.description = category
            .description
            .as_deref()
            .map(|text| truncate_review_text(text, 220));
        for item in &mut category.mods {
            item.annotation = truncate_review_text(&item.annotation, 180);
        }
    }
    Ok(serde_json::to_value(payload).expect("BuildSpecPayload is serializable"))
}

pub async fn enqueue_review_publish_task(pool: &PgPool, build: &BuildObject) -> Result<i64> {
    let payload = review_publish_task_payload(build)?;
    validate_publish_items(pool, &payload).await?;
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
        let review = review_publish_task_payload(&build).expect("Review-Payload");
        assert_eq!(review["name"], "[REVIEW] Warden test");
        assert!(review["description"]
            .as_str()
            .is_some_and(|text| text.contains("Nicht als reguläre Empfehlung freigegeben.")));
    }
}
