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
    if build.hero_id <= 0 || build.core.is_empty() {
        return Err(ReasonerError::Data(
            "Build braucht einen gültigen Helden und einen mechanisch abgeleiteten Kern.".into(),
        ));
    }
    if build.ability_order.is_empty()
        || build.patch_tag.trim().is_empty()
        || build.patch_tag == "current"
    {
        return Err(ReasonerError::Data(
            "Skillfolge oder konkrete Patch-Provenienz fehlt.".into(),
        ));
    }
    let mut ids = std::collections::BTreeSet::new();
    for item in build
        .core
        .iter()
        .chain(build.situations.iter().flat_map(|block| &block.items))
    {
        if item.item_id <= 0
            || !ids.insert(item.item_id)
            || !item.sources.iter().any(|source| {
                matches!(source.kind, EvidenceKind::Mechanic) && !source.detail.trim().is_empty()
            })
        {
            return Err(ReasonerError::Data(
                "Ungültiges, doppelt kategorisiertes oder nicht mechanisch belegtes Item.".into(),
            ));
        }
    }
    if build.situations.iter().any(|block| {
        block.items.is_empty()
            || block.label.trim().is_empty()
            || !block.optional
            || matches!(block.kind, SituationKind::CanBuyN(0))
    }) {
        return Err(ReasonerError::Data(
            "Situationskategorien müssen benannt, gefüllt und optional sein.".into(),
        ));
    }
    Ok(())
}

pub async fn validate_publish_current(pool: &PgPool, build: &BuildObject) -> Result<()> {
    validate_publish_input(build)?;
    let patch_tag = dbrain_builds::latest_patch_tag(pool)
        .await
        .map_err(ReasonerError::Db)?;
    if build.patch_tag != patch_tag {
        return Err(ReasonerError::Data(
            "Build stammt nicht aus dem aktiven Patch.".into(),
        ));
    }
    let ctx = crate::ReasonerCtx {
        pool: pool.clone(),
        ai: None,
        config: crate::ReasonerConfig {
            patch_tag,
            use_ai: false,
            ..Default::default()
        },
    };
    let start = crate::data::load_family_policy(&ctx)
        .await?
        .patch_started_at
        .ok_or_else(|| {
            ReasonerError::Data("Beginn des aktiven Patches ist nicht belegt.".into())
        })?;
    let (hero, mut snapshots) =
        crate::data::load_hero_model_with_snapshots(&ctx, &build.hero_name).await?;
    let (items, item_snapshots) = crate::data::load_item_models_with_snapshots(&ctx).await?;
    snapshots.extend(item_snapshots);
    validate_publish_models(build, &hero, &items, &snapshots, start as f64, &ctx.config)?;
    validate_mirror_membership(pool, build, &hero, &snapshots, start).await
}

fn current_mirror_version(summary: &Value, patch_started_at: i64) -> Result<i64> {
    summary["client_version"]
        .as_i64()
        .filter(|version| *version > 0)
        .filter(|_| summary["mirror_complete"].as_bool() == Some(true))
        .filter(|_| {
            summary["checked_at"]
                .as_i64()
                .is_some_and(|checked| checked >= patch_started_at)
        })
        .ok_or_else(|| {
            ReasonerError::Data(
                "Vollständiger API-Spiegel der aktuellen Clientversion ist nicht belegt.".into(),
            )
        })
}

async fn validate_mirror_membership(
    pool: &PgPool,
    build: &BuildObject,
    hero: &crate::HeroModel,
    snapshots: &[crate::PatchSnapshot],
    patch_started_at: i64,
) -> Result<()> {
    let summary: Option<Value> = sqlx::query_scalar(
        "SELECT summary FROM brain.source_runs WHERE source='assets' AND status='ok' ORDER BY finished_at DESC, id DESC LIMIT 1",
    ).fetch_optional(pool).await.map_err(ReasonerError::Db)?;
    let version = current_mirror_version(&summary.unwrap_or(Value::Null), patch_started_at)?;
    let hero_payload: Value = sqlx::query_scalar(
        "SELECT payload FROM brain.entity_snapshots WHERE source='deadlock_assets_api' AND entity_type='hero' AND payload->>'id'=$1 ORDER BY fetched_at DESC, id DESC LIMIT 1",
    ).bind(hero.hero_id.to_string()).fetch_one(pool).await.map_err(ReasonerError::Db)?;
    let mut targets = vec![(
        crate::DeltaTarget::Hero(hero.hero_id),
        "hero",
        "id",
        hero.hero_id.to_string(),
    )];
    targets.extend(hero.abilities.iter().map(|ability| {
        (
            crate::DeltaTarget::Ability(ability.ability_id),
            "item_or_ability",
            "class_name",
            ability.class_name.clone(),
        )
    }));
    targets.extend(
        build
            .core
            .iter()
            .chain(build.situations.iter().flat_map(|block| &block.items))
            .map(|item| {
                (
                    crate::DeltaTarget::Item(item.item_id),
                    "item_or_ability",
                    "id",
                    item.item_id.to_string(),
                )
            }),
    );
    if let Some(weapon) = hero_payload
        .pointer("/items/weapon_primary")
        .and_then(Value::as_str)
        .filter(|_| hero_payload.get("weapon_info").is_none_or(Value::is_null))
    {
        targets.push((
            crate::DeltaTarget::Hero(hero.hero_id),
            "item_or_ability",
            "class_name",
            weapon.to_owned(),
        ));
    }
    for (target, entity_type, field, key) in targets {
        let provenance: Option<Value> = sqlx::query_scalar(
            "SELECT jsonb_build_object('version', sd.metadata->'adapter'->'client_version', 'fetched_at', extract(epoch FROM es.fetched_at)) FROM brain.entity_snapshots es LEFT JOIN brain.source_documents sd ON sd.id=es.source_document_id AND sd.source=es.source WHERE es.source='deadlock_assets_api' AND es.entity_type=$1 AND es.payload->>$2=$3 ORDER BY es.fetched_at DESC, es.id DESC LIMIT 1",
        ).bind(entity_type).bind(field).bind(&key).fetch_optional(pool).await.map_err(ReasonerError::Db)?;
        let provenance = provenance.unwrap_or(Value::Null);
        let selected_time = provenance["fetched_at"].as_f64();
        let matching_snapshot = snapshots
            .iter()
            .find(|snapshot| snapshot.target == target)
            .is_some_and(|snapshot| {
                let fields = snapshot
                    .fields
                    .iter()
                    .filter(|(name, _)| {
                        !matches!(target, crate::DeltaTarget::Hero(_))
                            || name.starts_with("weapon.") == (entity_type == "item_or_ability")
                    })
                    .map(|(_, field)| field)
                    .collect::<Vec<_>>();
                !fields.is_empty() && fields.iter().all(|field| field.fetched_at == selected_time)
            });
        if provenance["version"].as_i64() != Some(version)
            || selected_time.is_none()
            || !matching_snapshot
        {
            return Err(ReasonerError::Data(format!("API-Spielwerte {entity_type}/{key} gehören nicht nachweislich zur aktuellen Clientversion {version}.")));
        }
    }
    Ok(())
}

fn validate_publish_models(
    build: &BuildObject,
    hero: &crate::HeroModel,
    items: &[crate::ItemModel],
    snapshots: &[crate::PatchSnapshot],
    patch_started_at: f64,
    cfg: &crate::ReasonerConfig,
) -> Result<()> {
    let error = |text: &str| ReasonerError::Data(text.into());
    if !patch_started_at.is_finite() || patch_started_at <= 0.0 {
        return Err(error("Aktiver Patch hat keinen gültigen Zeitpunkt."));
    }
    if hero.hero_id != build.hero_id || hero.abilities.is_empty() {
        return Err(error("Helden- oder Fähigkeitsdaten fehlen."));
    }
    let (order, notes) = crate::progression::coherent_order(hero, &build.ability_order);
    if order != build.ability_order || !notes.is_empty() {
        return Err(error(
            "Skillfolge passt nicht zu den aktuellen Fähigkeiten.",
        ));
    }
    let mut ranks = std::collections::BTreeMap::<i64, usize>::new();
    for step in &order {
        if step.currency_type == 1 {
            let rank = ranks.entry(step.ability_id).or_default();
            let cost = [1, 2, 5].get(*rank).copied();
            if cost != step.delta.checked_neg() {
                return Err(error("Ungültige Punktkosten in der Skillfolge."));
            }
            *rank += 1;
        }
    }
    let targets = std::iter::once(crate::DeltaTarget::Hero(hero.hero_id))
        .chain(
            hero.abilities
                .iter()
                .map(|ability| crate::DeltaTarget::Ability(ability.ability_id)),
        )
        .chain(
            build
                .core
                .iter()
                .chain(build.situations.iter().flat_map(|block| &block.items))
                .map(|item| crate::DeltaTarget::Item(item.item_id)),
        );
    for target in targets {
        let snapshot = snapshots
            .iter()
            .find(|snapshot| snapshot.target == target)
            .ok_or_else(|| {
                error("Spielwerte ohne API-Snapshot dürfen nicht veröffentlicht werden.")
            })?;
        if snapshot.fields.is_empty()
            || snapshot.fields.values().any(|field| {
                !field.value.is_finite()
                    || !field.source.starts_with("deadlock_assets_api/")
                    || field
                        .fetched_at
                        .is_none_or(|time| !time.is_finite() || time < patch_started_at)
            })
        {
            return Err(error(
                "API-Spielwerte sind nicht für den aktiven Patch belegt.",
            ));
        }
    }
    let rules = crate::inventory::InventoryRules::from_catalog(items)?;
    let mut inventory = crate::inventory::Inventory::default();
    let meta = crate::MetaIndex {
        by_item: Default::default(),
        sample_ok: Default::default(),
    };
    for purchase in &build.core {
        let item = items
            .iter()
            .find(|item| item.item_id == purchase.item_id)
            .ok_or_else(|| error("Kern-Item fehlt im API-Spiegel."))?;
        if purchase.tier != item.tier
            || crate::item::score_item(item, hero, &meta, &[], cfg).confidence
                == crate::Confidence::Low
        {
            return Err(error("Kernmechanik oder Item-Kategorie ist nicht belegt."));
        }
        if let Some(target) = purchase.imbue_target {
            if !item.imbueable
                || !hero
                    .abilities
                    .iter()
                    .any(|ability| ability.ability_id == target)
            {
                return Err(error("Ungültige Fähigkeitsbindung."));
            }
        }
        let mut sale_ids = Vec::new();
        let transition = loop {
            match inventory.preview_purchase(item, items, &rules, &sale_ids) {
                Ok(transition) => break transition,
                Err(failure) => {
                    let next = build
                        .core
                        .iter()
                        .filter(|previous| {
                            inventory.held_ids.contains(&previous.item_id)
                                && !sale_ids.contains(&previous.item_id)
                        })
                        .filter_map(|previous| {
                            previous
                                .sell_priority
                                .map(|priority| (priority, previous.item_id))
                        })
                        .min();
                    let Some((_, id)) = next else {
                        return Err(failure);
                    };
                    sale_ids.push(id);
                }
            }
        };
        inventory = transition.after;
        let held = items
            .iter()
            .filter(|item| inventory.held_ids.contains(&item.item_id))
            .cloned()
            .collect::<Vec<_>>();
        let bindings = build
            .core
            .iter()
            .filter(|item| inventory.held_ids.contains(&item.item_id))
            .filter_map(|item| item.imbue_target.map(|target| (item.item_id, target)))
            .collect();
        let evaluation =
            crate::combat::evaluate_inventory_with_bindings(hero, &held, cfg, &bindings);
        if !evaluation.score.is_finite() || !evaluation.unknown_effects.is_empty() {
            return Err(ReasonerError::Data(format!(
                "Kaufkurve enthält nicht belegte Kampfmechanik: {}",
                evaluation.unknown_effects.join("; ")
            )));
        }
    }
    let held = items
        .iter()
        .filter(|item| inventory.held_ids.contains(&item.item_id))
        .cloned()
        .collect::<Vec<_>>();
    let bindings = build
        .core
        .iter()
        .filter(|item| inventory.held_ids.contains(&item.item_id))
        .filter_map(|item| item.imbue_target.map(|target| (item.item_id, target)))
        .collect();
    let evaluation = crate::combat::evaluate_inventory_with_bindings(hero, &held, cfg, &bindings);
    let baseline = crate::combat::evaluate_inventory(hero, &[], cfg);
    if !evaluation.score.is_finite()
        || evaluation.score <= baseline.score
        || !evaluation.unknown_effects.is_empty()
    {
        return Err(error("Kern enthält nicht belegte Kampfmechanik."));
    }
    for candidate in build.situations.iter().flat_map(|block| &block.items) {
        let item = items
            .iter()
            .find(|item| item.item_id == candidate.item_id)
            .ok_or_else(|| error("Situations-Item fehlt im API-Spiegel."))?;
        if !item.shopable || item.disabled || item.tier != candidate.tier {
            return Err(error("Situations-Item ist im aktiven Patch nicht gültig."));
        }
        if let Some(target) = candidate.imbue_target {
            if !item.imbueable
                || !hero
                    .abilities
                    .iter()
                    .any(|ability| ability.ability_id == target)
            {
                return Err(error("Ungültige Fähigkeitsbindung im Situations-Item."));
            }
        }
        let bindings = candidate
            .imbue_target
            .map(|target| (candidate.item_id, target))
            .into_iter()
            .collect();
        let evaluation = crate::combat::evaluate_inventory_with_bindings(
            hero,
            std::slice::from_ref(item),
            cfg,
            &bindings,
        );
        if !evaluation.score.is_finite() || !evaluation.unknown_effects.is_empty() {
            return Err(ReasonerError::Data(format!(
                "Situations-Item {} enthält nicht belegte Kampfmechanik: {}",
                candidate.item_id,
                evaluation.unknown_effects.join("; ")
            )));
        }
    }
    Ok(())
}

pub async fn enqueue_publish_task(pool: &PgPool, build: &BuildObject) -> Result<i64> {
    validate_publish_current(pool, build).await?;
    let payload = publish_task_payload(build);
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

    fn model_fixture() -> (
        BuildObject,
        crate::HeroModel,
        Vec<crate::ItemModel>,
        Vec<crate::PatchSnapshot>,
    ) {
        let hero: crate::HeroModel = serde_json::from_value(serde_json::json!({
            "hero_id":700,"name":"Mechanikprüfung","archetype":"hybrid","base_health":600.0,
            "level_curve":[],"purchase_bonuses":{"weapon":[],"spirit":[],"vitality":[]},"scaling":[],
            "cost_bonuses":{"weapon":[{"gold_threshold":800,"bonus":3.0}],"spirit":[],"vitality":[]},
            "weapon":{"bullet_damage":20.0,"shots_per_second":4.0,"clip_size":16.0,"reload_duration":2.0,"range":20.0,"falloff_start_range":20.0,"falloff_end_range":50.0,"sustained_dps":50.0},
            "damage_plan":{"weapon_dps":50.0,"spirit_dps":0.0,"weapon_share":1.0,"primary_axis":"Weapon"},
            "abilities":[{"ability_id":101,"class_name":"ability_fixture","slot":1,"roles":[],"scaling":[],"channel_time":null,"charges":1,"cooldown":10.0,"scaling_step":null,"damage_type":"Spirit","base_effect":60.0,"properties":{"Damage":60.0,"AbilityCooldown":10.0},"upgrades":[{"property_upgrades":[{"name":"Damage","bonus":"35"}]}]}]
        })).unwrap();
        let models = (1..=13).map(|id| serde_json::from_value(serde_json::json!({
            "item_id":id,"name":format!("Item {id}"),"class_name":format!("item_{id}"),"slot":"Weapon","tier":2,"cost":1600,
            "is_active":false,"shopable":true,"disabled":false,"damage_axis":"Weapon","defense_kind":[],
            "properties":{"BaseAttackDamagePercent":10.0},"passive_properties":{},"condition":"None","proc_cooldown":null,"imbueable":false
        })).unwrap()).collect::<Vec<crate::ItemModel>>();
        let snapshots = std::iter::once(crate::DeltaTarget::Hero(700))
            .chain(std::iter::once(crate::DeltaTarget::Ability(101)))
            .chain(
                models
                    .iter()
                    .map(|item| crate::DeltaTarget::Item(item.item_id)),
            )
            .map(|target| crate::PatchSnapshot {
                target,
                name: "API".into(),
                fields: [(
                    "value".into(),
                    crate::SnapshotField {
                        value: 10.0,
                        fetched_at: Some(2000.0),
                        source: "deadlock_assets_api/item_or_ability".into(),
                        label: "Spielwert".into(),
                    },
                )]
                .into_iter()
                .collect(),
            })
            .collect();
        let build = BuildObject {
            hero_id: 700,
            hero_name: hero.name.clone(),
            patch_tag: "test-patch".into(),
            name: "Mechanikprüfung".into(),
            core: vec![item(1, None, None)],
            situations: vec![],
            ability_order: vec![crate::AbilityStep {
                ability_id: 101,
                currency_type: 2,
                delta: -1,
            }],
            confidence: Confidence::Low,
            rationale: "Mechanik aus aktuellen Spielwerten".into(),
            family: None,
            variants: vec![],
            family_discovery: None,
        };
        (build, hero, models, snapshots)
    }

    #[test]
    fn local_mirror_requires_complete_current_version_after_patch_start() {
        let summary =
            serde_json::json!({"client_version":6000,"mirror_complete":true,"checked_at":2000});
        assert_eq!(current_mirror_version(&summary, 1000).unwrap(), 6000);
        for invalid in [
            serde_json::json!({"client_version":6000,"mirror_complete":false,"checked_at":2000}),
            serde_json::json!({"client_version":6000,"mirror_complete":true,"checked_at":999}),
            serde_json::json!({"client_version":0,"mirror_complete":true,"checked_at":2000}),
            Value::Null,
        ] {
            assert!(current_mirror_version(&invalid, 1000).is_err());
        }
    }

    #[test]
    fn planning_without_population_authors_or_skill_sources_produces_valid_mechanics() {
        let (_, hero, models, snapshots) = model_fixture();
        let cfg = crate::ReasonerConfig {
            patch_tag: "test-patch".into(),
            use_ai: false,
            ..Default::default()
        };
        let meta = crate::meta::MetaIndexWithSources {
            index: crate::MetaIndex {
                by_item: Default::default(),
                sample_ok: Default::default(),
            },
            author_builds: vec![],
            hero_ability_orders: Default::default(),
            core_layouts: Default::default(),
            combinations: Default::default(),
            population: Default::default(),
            observations: vec![],
            family: None,
        };
        let mut build = crate::plan_build(&hero, &models, &meta, &[], &snapshots, &cfg)
            .unwrap()
            .build;
        let confidence = build.confidence.clone();
        crate::annotate_missing_authors(&mut build, &meta);
        assert_eq!(build.confidence, confidence);
        assert!(build.family.is_none());
        assert!(build.variants.is_empty());
        assert!(!build.core.is_empty());
        assert!(!build.ability_order.is_empty());
        validate_publish_input(&build).unwrap();
        validate_publish_models(&build, &hero, &models, &snapshots, 1000.0, &cfg).unwrap();
    }

    #[test]
    fn current_mechanics_publish_without_matches_family_or_high_confidence() {
        let (build, hero, models, snapshots) = model_fixture();
        validate_publish_input(&build).unwrap();
        validate_publish_models(
            &build,
            &hero,
            &models,
            &snapshots,
            1000.0,
            &crate::ReasonerConfig::default(),
        )
        .unwrap();
    }

    #[test]
    fn stale_missing_and_non_api_game_values_are_rejected() {
        let (build, hero, models, mut snapshots) = model_fixture();
        let cfg = crate::ReasonerConfig::default();
        assert!(validate_publish_models(&build, &hero, &models, &snapshots, 2001.0, &cfg).is_err());
        let missing = snapshots.pop().unwrap();
        let mut uses_missing = build.clone();
        uses_missing.core.push(item(13, None, None));
        assert!(
            validate_publish_models(&uses_missing, &hero, &models, &snapshots, 1000.0, &cfg)
                .is_err()
        );
        snapshots.push(missing);
        snapshots[0].fields.values_mut().next().unwrap().source = "deadlock_data".into();
        assert!(validate_publish_models(&build, &hero, &models, &snapshots, 1000.0, &cfg).is_err());
        assert!(
            validate_publish_models(&build, &hero, &models, &snapshots, f64::NAN, &cfg).is_err()
        );
    }

    #[test]
    fn invalid_slots_skills_bindings_and_categories_are_rejected() {
        let (build, hero, models, snapshots) = model_fixture();
        let cfg = crate::ReasonerConfig::default();
        let check = |build: &BuildObject| {
            validate_publish_models(build, &hero, &models, &snapshots, 1000.0, &cfg)
        };
        let mut invalid = build.clone();
        invalid.core = (1..=13).map(|id| item(id, None, None)).collect();
        assert!(check(&invalid).is_err());
        invalid = build.clone();
        invalid.ability_order.push(crate::AbilityStep {
            ability_id: 101,
            currency_type: 1,
            delta: -5,
        });
        assert!(check(&invalid).is_err());
        invalid = build.clone();
        invalid.core[0].imbue_target = Some(101);
        assert!(check(&invalid).is_err());
        invalid = build.clone();
        invalid.situations.push(SituationBlock {
            label: "Optional".into(),
            optional: true,
            kind: SituationKind::Optional,
            items: vec![item(1, None, None)],
        });
        assert!(validate_publish_input(&invalid).is_err());
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
