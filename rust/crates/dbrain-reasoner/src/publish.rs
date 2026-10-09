use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

const BUILD_DATA_MARKER: &str = "#build-data=";

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BuildProvenance {
    pub origin: BuildDataOrigin,
    pub input_sha256: String,
    pub plan_sha256: String,
    pub config: crate::ReasonerConfig,
    #[serde(default)]
    pub purchase_plan: Option<crate::planner::PurchasePlan>,
}

fn fingerprint(value: &impl Serialize) -> Result<String> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| ReasonerError::Data("Buildherkunft ist nicht prüfbar.".into()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

impl BuildDataOrigin {
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

    fn same_game_values(&self, other: &Self) -> bool {
        self.client_version == other.client_version
            && self.parser_revision == other.parser_revision
            && self.manifest_sha256 == other.manifest_sha256
            && self.heroes_sha256 == other.heroes_sha256
            && self.items_sha256 == other.items_sha256
    }

    pub(crate) fn snapshot_source(&self) -> Result<String> {
        self.validate()?;
        let encoded = serde_json::to_string(self)
            .map_err(|_| ReasonerError::Data("Originalspieldaten sind nicht prüfbar.".into()))?;
        Ok(format!(
            "deadlock_assets_api/hero{BUILD_DATA_MARKER}{encoded}"
        ))
    }
}

fn input_fingerprint(
    hero: &crate::HeroModel,
    items: &[crate::ItemModel],
    snapshots: &[crate::PatchSnapshot],
    config: &crate::ReasonerConfig,
) -> Result<String> {
    fingerprint(&(hero, items, snapshots, config))
}

fn game_values_fingerprint(
    hero: &crate::HeroModel,
    items: &[crate::ItemModel],
    snapshots: &[crate::PatchSnapshot],
    config: &crate::ReasonerConfig,
) -> Result<String> {
    let fields = snapshots
        .iter()
        .map(|snapshot| {
            (
                &snapshot.target,
                &snapshot.name,
                snapshot
                    .fields
                    .iter()
                    .map(|(name, field)| {
                        (
                            name,
                            field.value,
                            field
                                .source
                                .split(BUILD_DATA_MARKER)
                                .next()
                                .unwrap_or_default(),
                            &field.label,
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    fingerprint(&(hero, items, fields, config))
}

fn plan_fingerprint(
    build: &BuildObject,
    plan: Option<&crate::planner::PurchasePlan>,
) -> Result<String> {
    let item = |item: &BuildItem| {
        (
            item.item_id,
            item.tier,
            item.buy_phase.clone(),
            item.imbue_target,
            item.sell_priority,
        )
    };
    fingerprint(&(
        build.hero_id,
        &build.hero_name,
        &build.patch_tag,
        build.core.iter().map(item).collect::<Vec<_>>(),
        build
            .situations
            .iter()
            .map(|block| {
                (
                    &block.kind,
                    block.optional,
                    block.items.iter().map(item).collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>(),
        &build.ability_order,
        plan,
    ))
}

pub(crate) fn calculation_provenance(
    hero: &crate::HeroModel,
    items: &[crate::ItemModel],
    snapshots: &[crate::PatchSnapshot],
    config: &crate::ReasonerConfig,
) -> Result<Option<BuildProvenance>> {
    let mut origin = None;
    for field in snapshots
        .iter()
        .flat_map(|snapshot| snapshot.fields.values())
    {
        if let Some((source, encoded)) = field.source.split_once(BUILD_DATA_MARKER) {
            if source != "deadlock_assets_api/hero" {
                return Err(ReasonerError::Data(
                    "Buildherkunft hat keine gültige Originalquelle.".into(),
                ));
            }
            let found: BuildDataOrigin = serde_json::from_str(encoded)
                .map_err(|_| ReasonerError::Data("Buildherkunft ist ungültig.".into()))?;
            found.validate()?;
            if origin.as_ref().is_some_and(|origin| origin != &found) {
                return Err(ReasonerError::Data(
                    "Buildherkunft ist widersprüchlich.".into(),
                ));
            }
            origin = Some(found);
        }
    }
    origin
        .map(|origin| {
            Ok(BuildProvenance {
                origin,
                input_sha256: input_fingerprint(hero, items, snapshots, config)?,
                plan_sha256: String::new(),
                config: config.clone(),
                purchase_plan: None,
            })
        })
        .transpose()
}

pub(crate) fn bind_calculated_build(
    build: &mut BuildObject,
    mut provenance: BuildProvenance,
    purchase_plan: &crate::planner::PurchasePlan,
) -> Result<()> {
    provenance.purchase_plan = Some(purchase_plan.clone());
    provenance.plan_sha256 = plan_fingerprint(build, provenance.purchase_plan.as_ref())?;
    build.provenance = Some(Box::new(provenance));
    Ok(())
}

pub fn validate_build_provenance(build: &BuildObject) -> Result<&BuildProvenance> {
    let item_origins = build
        .core
        .iter()
        .chain(build.situations.iter().flat_map(|block| &block.items))
        .flat_map(|item| &item.sources)
        .filter_map(|evidence| match &evidence.kind {
            EvidenceKind::BuildProvenance(provenance) => Some(provenance.as_ref()),
            _ => None,
        });
    let mut origins = build.provenance.as_deref().into_iter().chain(item_origins);
    let provenance = origins.next().ok_or_else(|| {
        ReasonerError::Data(
            "Berechnete Buildherkunft fehlt. Bitte einen aktuellen Build berechnen.".into(),
        )
    })?;
    provenance.origin.validate()?;
    if origins.next().is_some()
        || provenance.config.patch_tag != build.patch_tag
        || provenance.purchase_plan.is_none()
        || provenance.plan_sha256 != plan_fingerprint(build, provenance.purchase_plan.as_ref())?
        || provenance.input_sha256.len() != 64
    {
        return Err(ReasonerError::Data(
            "Build stimmt nicht mit seiner berechneten Herkunft überein.".into(),
        ));
    }
    Ok(provenance)
}

pub fn publication_caller(build: &BuildObject) -> Result<String> {
    let provenance = validate_build_provenance(build)?;
    let origin = &provenance.origin;
    let identity = fingerprint(&(
        origin.client_version,
        &origin.parser_revision,
        &origin.manifest_sha256,
        &origin.heroes_sha256,
        &origin.items_sha256,
        &provenance.plan_sha256,
        &provenance.config,
    ))?;
    Ok(format!("deadlock-brain-reasoner/{identity}"))
}

pub fn equivalent_calculated_builds(first: &BuildObject, second: &BuildObject) -> Result<bool> {
    let first = validate_build_provenance(first)?;
    let second = validate_build_provenance(second)?;
    Ok(first.origin.same_game_values(&second.origin)
        && first.plan_sha256 == second.plan_sha256
        && first.config == second.config)
}

fn validate_calculated_inputs(
    build: &BuildObject,
    hero: &crate::HeroModel,
    items: &[crate::ItemModel],
    snapshots: &[crate::PatchSnapshot],
) -> Result<()> {
    let original = validate_build_provenance(build)?;
    let current =
        calculation_provenance(hero, items, snapshots, &original.config)?.ok_or_else(|| {
            ReasonerError::Data("Aktuelle Originalspieldaten sind nicht belegt.".into())
        })?;
    if original.origin != current.origin || original.input_sha256 != current.input_sha256 {
        return Err(ReasonerError::Data("Spieldaten oder Clientversion haben sich seit der Buildberechnung geändert. Es wurde nichts veröffentlicht.".into()));
    }
    Ok(())
}

fn validate_current_game_values(
    build: &BuildObject,
    original: &crate::data::MirroredModels,
    current: &crate::data::MirroredModels,
) -> Result<()> {
    let provenance = validate_build_provenance(build)?;
    let planning_items = |models: &crate::data::MirroredModels| {
        models
            .items
            .iter()
            .map(crate::item::build_item_model)
            .collect::<Result<Vec<_>>>()
    };
    let original_items = planning_items(original)?;
    let current_items = planning_items(current)?;
    validate_calculated_inputs(build, &original.hero, &original_items, &original.snapshots)?;
    let current_provenance = calculation_provenance(
        &current.hero,
        &current_items,
        &current.snapshots,
        &provenance.config,
    )?
    .ok_or_else(|| ReasonerError::Data("Aktuelle Originalspieldaten sind nicht belegt.".into()))?;
    if !provenance
        .origin
        .same_game_values(&current_provenance.origin)
        || game_values_fingerprint(
            &original.hero,
            &original_items,
            &original.snapshots,
            &provenance.config,
        )? != game_values_fingerprint(
            &current.hero,
            &current_items,
            &current.snapshots,
            &provenance.config,
        )?
    {
        return Err(ReasonerError::Data("Spieldaten oder Clientversion haben sich seit der Buildberechnung geändert. Es wurde nichts veröffentlicht.".into()));
    }
    Ok(())
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
    let original = validate_build_provenance(build)?;
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
            ..original.config.clone()
        },
    };
    let start = crate::data::load_family_policy(&ctx)
        .await?
        .patch_started_at
        .ok_or_else(|| {
            ReasonerError::Data("Beginn des aktiven Patches ist nicht belegt.".into())
        })?;
    let mirrored = crate::data::load_models_from_mirror(&ctx, &build.hero_name).await?;
    validate_mirror_provenance(&mirrored.provenance, start)?;
    let calculated =
        crate::data::load_models_from_origin(&ctx, &build.hero_name, &original.origin).await?;
    validate_current_game_values(build, &calculated, &mirrored)?;
    validate_publish_models_with_plan(
        build,
        original
            .purchase_plan
            .as_ref()
            .ok_or_else(|| ReasonerError::Data("Berechnete Kaufkurve fehlt.".into()))?,
        &mirrored.hero,
        &mirrored.items,
        &mirrored.snapshots,
        mirrored.provenance.mirrored_at as f64,
        &original.config,
    )
}

fn validate_mirror_provenance(
    mirror: &crate::data::MirrorProvenance,
    patch_started_at: i64,
) -> Result<()> {
    if mirror.client_version <= 0
        || patch_started_at <= 0
        || mirror.mirrored_at <= 0
        || mirror.mirrored_at > mirror.checked_at
        || mirror.checked_at < patch_started_at
    {
        return Err(ReasonerError::Data(
            "Vollständiger API-Spiegel wurde nicht für den aktiven Patch geprüft.".into(),
        ));
    }
    Ok(())
}

fn validate_publish_models_with_plan(
    build: &BuildObject,
    plan: &crate::planner::PurchasePlan,
    hero: &crate::HeroModel,
    items: &[crate::ItemModel],
    snapshots: &[crate::PatchSnapshot],
    mirrored_at: f64,
    cfg: &crate::ReasonerConfig,
) -> Result<()> {
    let error = |text: &str| ReasonerError::Data(text.into());
    if !mirrored_at.is_finite() || mirrored_at <= 0.0 {
        return Err(error(
            "API-Spiegel hat keinen gültigen Erfassungszeitpunkt.",
        ));
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
                        .is_none_or(|time| !time.is_finite() || time != mirrored_at)
            })
        {
            return Err(error(
                "API-Spielwerte sind nicht für den aktiven Patch belegt.",
            ));
        }
    }
    if plan.ability_order != order || plan.steps.len() != build.core.len() || plan.steps.is_empty()
    {
        return Err(error(
            "Gespeicherte Kaufkurve passt nicht zum veröffentlichten Build.",
        ));
    }
    let full_souls = hero
        .level_curve
        .iter()
        .map(|level| level.required_souls)
        .max()
        .unwrap_or(0);
    let (fully_progressed, full_progression) =
        crate::progression::at_souls(hero, &order, full_souls, cfg);
    if !full_progression.unknown_effects.is_empty()
        || (order.iter().any(|step| step.currency_type == 1)
            && full_progression.applied_order_steps != order.len())
    {
        return Err(error(
            "Veröffentlichte Skillfolge enthält nicht belegte Fortschrittsmechanik.",
        ));
    }
    let items = items
        .iter()
        .map(crate::item::build_item_model)
        .collect::<Result<Vec<_>>>()?;
    let items = items.as_slice();
    let rules = crate::inventory::InventoryRules::from_catalog(items)?;
    let mut inventory = crate::inventory::Inventory::default();
    let mut bindings = std::collections::BTreeMap::new();
    let mut earned_souls = 0;
    let meta = crate::MetaIndex {
        by_item: Default::default(),
        sample_ok: Default::default(),
    };
    for (purchase, step) in build.core.iter().zip(&plan.steps) {
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
        if step.transition.purchased_id != purchase.item_id
            || step.progression.earned_souls < earned_souls
            || step.transition.after.spent_souls > step.progression.earned_souls
            || step.transition.net_cost <= 0
            || step.transition.sold_ids.iter().any(|id| {
                !build
                    .core
                    .iter()
                    .any(|previous| previous.item_id == *id && previous.sell_priority.is_some())
            })
        {
            return Err(error(
                "Kauf oder Finanzierung stimmt nicht mit der gespeicherten Kaufkurve überein.",
            ));
        }
        let transition =
            inventory.preview_purchase(item, items, &rules, &step.transition.sold_ids)?;
        if transition != step.transition {
            return Err(error(
                "Gespeicherter Kaufübergang passt nicht zu den aktuellen Spielwerten.",
            ));
        }
        inventory.apply_transition(&transition)?;
        bindings.retain(|id, _| inventory.held_ids.contains(id));
        if let Some(target) = purchase.imbue_target {
            bindings.insert(purchase.item_id, target);
        }
        if bindings != step.imbue_targets {
            return Err(error(
                "Fähigkeitsbindungen passen nicht zur gespeicherten Kaufkurve.",
            ));
        }
        earned_souls = step.progression.earned_souls;
        let (progressed, progression) =
            crate::progression::at_souls(hero, &order, earned_souls, cfg);
        if progression != step.progression || !progression.unknown_effects.is_empty() {
            return Err(error(
                "Kaufkurve enthält nicht belegte Fortschrittsmechanik.",
            ));
        }
        let held = inventory.held_items(items)?;
        let evaluation =
            crate::combat::evaluate_inventory_with_bindings(&progressed, &held, cfg, &bindings);
        if !evaluation.score.is_finite() || !evaluation.unknown_effects.is_empty() {
            return Err(ReasonerError::Data(format!(
                "Kaufkurve enthält nicht belegte Kampfmechanik: {}",
                evaluation.unknown_effects.join("; ")
            )));
        }
    }
    let held = inventory.held_items(items)?;
    let (progressed, _) = crate::progression::at_souls(hero, &order, earned_souls, cfg);
    let evaluation =
        crate::combat::evaluate_inventory_with_bindings(&progressed, &held, cfg, &bindings);
    let baseline = crate::combat::evaluate_inventory(&progressed, &[], cfg);
    if !evaluation.score.is_finite()
        || evaluation.score <= baseline.score
        || !evaluation.unknown_effects.is_empty()
    {
        return Err(error("Kern enthält nicht belegte Kampfmechanik."));
    }
    let published_evaluation =
        crate::combat::evaluate_inventory_with_bindings(&fully_progressed, &held, cfg, &bindings);
    if !published_evaluation.score.is_finite() || !published_evaluation.unknown_effects.is_empty() {
        return Err(error(
            "Veröffentlichter Kern enthält nicht belegte Kampfmechanik der Skillfolge.",
        ));
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
            &fully_progressed,
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
            provenance: None,
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

    fn fixture_plan(
        build: &BuildObject,
        hero: &crate::HeroModel,
        items: &[crate::ItemModel],
        cfg: &crate::ReasonerConfig,
    ) -> Result<crate::planner::PurchasePlan> {
        let rules = crate::inventory::InventoryRules::from_catalog(items)?;
        let mut inventory = crate::inventory::Inventory::default();
        let mut steps = Vec::new();
        let mut earned = 0;
        for purchase in &build.core {
            let item = items
                .iter()
                .find(|item| item.item_id == purchase.item_id)
                .unwrap();
            let transition = inventory.preview_purchase(item, items, &rules, &[])?;
            inventory.apply_transition(&transition)?;
            earned += item.cost;
            let (hero, progression) =
                crate::progression::at_souls(hero, &build.ability_order, earned, cfg);
            let bindings = build
                .core
                .iter()
                .filter(|item| inventory.held_ids.contains(&item.item_id))
                .filter_map(|item| item.imbue_target.map(|target| (item.item_id, target)))
                .collect();
            let evaluation = crate::combat::evaluate_inventory_with_bindings(
                &hero,
                &inventory.held_items(items)?,
                cfg,
                &bindings,
            );
            steps.push(crate::planner::PurchaseStep {
                transition,
                evaluation,
                marginal_value: 1.0,
                progression,
                imbue_targets: bindings,
            });
        }
        Ok(crate::planner::PurchasePlan {
            final_evaluation: steps.last().unwrap().evaluation.clone(),
            steps,
            assumptions: Vec::new(),
            ability_order: build.ability_order.clone(),
            saving_decisions: Vec::new(),
        })
    }

    fn validate_publish_models(
        build: &BuildObject,
        hero: &crate::HeroModel,
        items: &[crate::ItemModel],
        snapshots: &[crate::PatchSnapshot],
        mirrored_at: f64,
        cfg: &crate::ReasonerConfig,
    ) -> Result<()> {
        let plan = fixture_plan(build, hero, items, cfg)?;
        validate_publish_models_with_plan(build, &plan, hero, items, snapshots, mirrored_at, cfg)
    }

    fn origin_fixture() -> BuildDataOrigin {
        BuildDataOrigin {
            client_version: 6000,
            source_run_id: 10,
            mirrored_at: 2000,
            parser_revision: "fixture-parser".into(),
            manifest_document_id: 11,
            manifest_sha256: "1".repeat(64),
            heroes_document_id: 12,
            heroes_sha256: "2".repeat(64),
            items_document_id: 13,
            items_sha256: "3".repeat(64),
        }
    }

    fn bound_fixture(origin: BuildDataOrigin) -> (BuildObject, crate::data::MirroredModels) {
        let (mut build, hero, items, mut snapshots) = model_fixture();
        for snapshot in &mut snapshots {
            for field in snapshot.fields.values_mut() {
                field.fetched_at = Some(origin.mirrored_at as f64);
            }
        }
        snapshots[0].fields.values_mut().next().unwrap().source = origin.snapshot_source().unwrap();
        let cfg = crate::ReasonerConfig {
            patch_tag: build.patch_tag.clone(),
            use_ai: false,
            ..Default::default()
        };
        let plan = fixture_plan(&build, &hero, &items, &cfg).unwrap();
        let provenance = calculation_provenance(&hero, &items, &snapshots, &cfg)
            .unwrap()
            .unwrap();
        bind_calculated_build(&mut build, provenance, &plan).unwrap();
        (
            build,
            crate::data::MirroredModels {
                hero,
                items,
                snapshots,
                provenance: crate::data::MirrorProvenance {
                    client_version: origin.client_version,
                    mirrored_at: origin.mirrored_at,
                    checked_at: origin.mirrored_at,
                },
                flex_slots: None,
            },
        )
    }

    #[test]
    fn build_provenance_is_independent_of_items_and_rejects_duplicate_legacy_bindings() {
        let (build, _) = bound_fixture(origin_fixture());
        let expected = validate_build_provenance(&build).unwrap().clone();
        assert!(build
            .core
            .iter()
            .flat_map(|item| &item.sources)
            .all(|evidence| { !matches!(evidence.kind, EvidenceKind::BuildProvenance(_)) }));
        let mut legacy = build.clone();
        let provenance = legacy.provenance.take().unwrap();
        legacy.core[0].sources.push(Evidence {
            kind: EvidenceKind::BuildProvenance(provenance),
            detail: String::new(),
        });
        let restored: BuildObject =
            serde_json::from_value(serde_json::to_value(&legacy).unwrap()).unwrap();
        assert_eq!(validate_build_provenance(&restored).unwrap(), &expected);
        legacy.provenance = build.provenance.clone();
        assert!(validate_build_provenance(&legacy).is_err());
    }

    #[test]
    fn fingerprints_bind_all_planning_inputs_without_new_receipt_fixtures() {
        let (_, hero, mut items, mut snapshots) = model_fixture();
        let cfg = crate::ReasonerConfig::default();
        let original = input_fingerprint(&hero, &items, &snapshots, &cfg).unwrap();
        assert_eq!(
            original,
            input_fingerprint(&hero, &items, &snapshots, &cfg).unwrap()
        );
        let mut changed = hero.clone();
        changed.weapon.bullet_damage += 1.0;
        assert_ne!(
            original,
            input_fingerprint(&changed, &items, &snapshots, &cfg).unwrap()
        );
        items.last_mut().unwrap().cost += 1;
        assert_ne!(
            original,
            input_fingerprint(&hero, &items, &snapshots, &cfg).unwrap()
        );
        items.last_mut().unwrap().cost -= 1;
        snapshots[0].fields.values_mut().next().unwrap().value += 1.0;
        assert_ne!(
            original,
            input_fingerprint(&hero, &items, &snapshots, &cfg).unwrap()
        );
        snapshots[0].fields.values_mut().next().unwrap().value -= 1.0;
        let mut changed = cfg.clone();
        changed.combat_window_seconds += 1.0;
        assert_ne!(
            original,
            input_fingerprint(&hero, &items, &snapshots, &changed).unwrap()
        );
        assert!(calculation_provenance(&hero, &items, &snapshots, &cfg)
            .unwrap()
            .is_none());
        snapshots[0].fields.values_mut().next().unwrap().source =
            "deadlock_assets_api/hero#build-data={}".into();
        assert!(calculation_provenance(&hero, &items, &snapshots, &cfg).is_err());
    }

    #[test]
    fn current_guard_keeps_exact_origin_but_accepts_new_observations_of_identical_values() {
        let (build, original) = bound_fixture(origin_fixture());
        let mut observed = origin_fixture();
        observed.source_run_id += 100;
        observed.mirrored_at += 100;
        observed.manifest_document_id += 100;
        observed.heroes_document_id += 100;
        observed.items_document_id += 100;
        let (recalculated, current) = bound_fixture(observed.clone());
        assert_ne!(validate_build_provenance(&build).unwrap().origin, observed);
        validate_current_game_values(&build, &original, &current).unwrap();
        assert!(equivalent_calculated_builds(&build, &recalculated).unwrap());
        assert_ne!(
            validate_build_provenance(&build).unwrap().input_sha256,
            validate_build_provenance(&recalculated)
                .unwrap()
                .input_sha256
        );
        let original_json = serde_json::to_value(&build).unwrap();
        let roundtrip = serde_json::from_value(original_json.clone()).unwrap();
        validate_current_game_values(&roundtrip, &original, &current).unwrap();
        assert_eq!(serde_json::to_value(&roundtrip).unwrap(), original_json);
        for changed in [
            {
                let mut value = observed.clone();
                value.client_version += 1;
                value
            },
            {
                let mut value = observed.clone();
                value.parser_revision.push_str("-new");
                value
            },
            {
                let mut value = observed.clone();
                value.items_sha256 = "4".repeat(64);
                value
            },
            {
                let mut value = observed;
                value.heroes_sha256 = "5".repeat(64);
                value
            },
        ] {
            let (changed_build, changed_models) = bound_fixture(changed);
            assert!(validate_current_game_values(&build, &original, &changed_models).is_err());
            assert!(!equivalent_calculated_builds(&build, &changed_build).unwrap());
        }
        let (_, mut changed) = bound_fixture(origin_fixture());
        changed.hero.weapon.bullet_damage += 1.0;
        assert!(validate_current_game_values(&build, &original, &changed).is_err());
        let (_, mut changed) = bound_fixture(origin_fixture());
        changed.items[0].cost += 1;
        assert!(validate_current_game_values(&build, &original, &changed).is_err());
        let (_, mut changed) = bound_fixture(origin_fixture());
        changed.snapshots[0]
            .fields
            .values_mut()
            .next()
            .unwrap()
            .value += 1.0;
        assert!(validate_current_game_values(&build, &original, &changed).is_err());
        let mut forged = build.clone();
        forged.provenance.as_mut().unwrap().origin.source_run_id += 1;
        assert!(validate_current_game_values(&forged, &original, &current).is_err());
    }

    #[test]
    fn guard_checks_published_upgrades_even_after_the_last_purchase() {
        let (mut build, mut hero, models, snapshots) = model_fixture();
        hero.level_curve = vec![
            crate::LevelPoint {
                level: 1,
                required_souls: 0,
            },
            crate::LevelPoint {
                level: 2,
                required_souls: 2000,
            },
        ];
        hero.level_rewards = [
            (1, vec!["EAbilityUnlocks".into()]),
            (2, vec!["EAbilityPoints".into()]),
        ]
        .into_iter()
        .collect();
        build.ability_order.push(crate::AbilityStep {
            ability_id: 101,
            currency_type: 1,
            delta: -1,
        });
        let cfg = crate::ReasonerConfig::default();
        let plan = fixture_plan(&build, &hero, &models, &cfg).unwrap();
        assert_eq!(plan.steps[0].progression.applied_order_steps, 1);
        validate_publish_models_with_plan(&build, &plan, &hero, &models, &snapshots, 2000.0, &cfg)
            .unwrap();
        hero.abilities[0].upgrades[0]["property_upgrades"][0]["upgrade_type"] =
            serde_json::json!("EUnknown");
        assert!(crate::combat::evaluate_inventory(&hero, &models[..1], &cfg)
            .unknown_effects
            .is_empty());
        let error = validate_publish_models_with_plan(
            &build, &plan, &hero, &models, &snapshots, 2000.0, &cfg,
        )
        .unwrap_err();
        assert!(error.to_string().contains("Fortschrittsmechanik"));
        hero.level_curve[1].required_souls = 100;
        let plan = fixture_plan(&build, &hero, &models, &cfg).unwrap();
        assert!(!plan.steps[0].progression.unknown_effects.is_empty());
        assert!(validate_publish_models_with_plan(
            &build, &plan, &hero, &models, &snapshots, 2000.0, &cfg
        )
        .is_err());
    }

    #[test]
    fn guard_replays_funding_sales_with_free_slots_and_binds_the_complete_plan() {
        let (mut build, hero, mut models, snapshots) = model_fixture();
        build.core[0].sell_priority = Some(1);
        build.core.push(item(2, None, None));
        models[1]
            .properties
            .insert("BaseAttackDamagePercent".into(), 100.0);
        let cfg = crate::ReasonerConfig {
            patch_tag: build.patch_tag.clone(),
            ..Default::default()
        };
        let mut plan = fixture_plan(&build, &hero, &models, &cfg).unwrap();
        let rules = crate::inventory::InventoryRules::from_catalog(&models).unwrap();
        let before = &plan.steps[0].transition.after;
        assert!(before.held_ids.len() < rules.max_slots);
        let direct = before
            .preview_purchase(&models[1], &models, &rules, &[])
            .unwrap();
        let funded = before
            .preview_purchase(&models[1], &models, &rules, &[1])
            .unwrap();
        plan.steps[1].transition = funded;
        plan.steps[1].progression =
            crate::progression::at_souls(&hero, &build.ability_order, 2400, &cfg).1;
        plan.steps[1].evaluation = crate::combat::evaluate_inventory(&hero, &models[1..2], &cfg);
        plan.final_evaluation = plan.steps[1].evaluation.clone();
        assert!(direct.after.spent_souls > plan.steps[1].progression.earned_souls);
        validate_publish_models_with_plan(&build, &plan, &hero, &models, &snapshots, 2000.0, &cfg)
            .unwrap();
        let mut unfunded = plan.clone();
        unfunded.steps[1].transition = direct;
        assert!(validate_publish_models_with_plan(
            &build, &unfunded, &hero, &models, &snapshots, 2000.0, &cfg
        )
        .is_err());
        let mut corrupted = plan.clone();
        corrupted.steps[1].transition.after.held_ids.insert(1);
        assert!(validate_publish_models_with_plan(
            &build, &corrupted, &hero, &models, &snapshots, 2000.0, &cfg
        )
        .is_err());
        let mut wrong_skill = plan.clone();
        wrong_skill.ability_order.clear();
        assert!(validate_publish_models_with_plan(
            &build,
            &wrong_skill,
            &hero,
            &models,
            &snapshots,
            2000.0,
            &cfg
        )
        .is_err());
        let provenance = BuildProvenance {
            origin: origin_fixture(),
            input_sha256: input_fingerprint(&hero, &models, &snapshots, &cfg).unwrap(),
            plan_sha256: String::new(),
            config: cfg.clone(),
            purchase_plan: None,
        };
        bind_calculated_build(&mut build, provenance, &plan).unwrap();
        let restored: BuildObject =
            serde_json::from_value(serde_json::to_value(&build).unwrap()).unwrap();
        let bound = validate_build_provenance(&restored).unwrap();
        assert_eq!(
            serde_json::to_value(bound.purchase_plan.as_ref().unwrap()).unwrap(),
            serde_json::to_value(&plan).unwrap()
        );
        validate_publish_models_with_plan(
            &restored,
            bound.purchase_plan.as_ref().unwrap(),
            &hero,
            &models,
            &snapshots,
            2000.0,
            &cfg,
        )
        .unwrap();
        let mut tampered = restored;
        tampered
            .provenance
            .as_mut()
            .unwrap()
            .purchase_plan
            .as_mut()
            .unwrap()
            .steps[1]
            .transition
            .sold_ids
            .clear();
        assert!(validate_build_provenance(&tampered).is_err());
    }

    #[test]
    fn calculated_plan_binding_preserves_text_but_detects_purchase_and_skill_changes() {
        let (build, _, _, _) = model_fixture();
        let original = plan_fingerprint(&build, None).unwrap();
        let mut changed = build.clone();
        changed.rationale.push_str(" Ergänzende Erläuterung");
        changed.core[0].why.push_str(" Ergänzende Erläuterung");
        assert_eq!(original, plan_fingerprint(&changed, None).unwrap());
        for changed in [
            {
                let mut changed = build.clone();
                changed.core[0].imbue_target = Some(101);
                changed
            },
            {
                let mut changed = build.clone();
                changed.core[0].sell_priority = Some(1);
                changed
            },
            {
                let mut changed = build.clone();
                changed.core.push(item(2, None, None));
                changed
            },
            {
                let mut changed = build.clone();
                changed.ability_order[0].delta = -2;
                changed
            },
        ] {
            assert_ne!(original, plan_fingerprint(&changed, None).unwrap());
        }
        let roundtrip: BuildObject =
            serde_json::from_value(serde_json::to_value(&build).unwrap()).unwrap();
        assert_eq!(build, roundtrip);
        assert!(validate_build_provenance(&roundtrip).is_err());
    }

    #[tokio::test]
    async fn legacy_build_is_rejected_before_database_or_queue_access() {
        let (build, _, _, _) = model_fixture();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgresql://localhost/unavailable_f_provenance_test")
            .unwrap();
        let error = validate_publish_current(&pool, &build).await.unwrap_err();
        assert!(error.to_string().contains("Buildherkunft fehlt"));
        let error = enqueue_publish_task(&pool, &build).await.unwrap_err();
        assert!(error.to_string().contains("Buildherkunft fehlt"));
    }

    #[test]
    fn local_mirror_requires_current_check_but_preserves_unchanged_original_values() {
        let mirror = crate::data::MirrorProvenance {
            client_version: 6000,
            mirrored_at: 500,
            checked_at: 2000,
        };
        validate_mirror_provenance(&mirror, 1000).unwrap();
        for invalid in [
            crate::data::MirrorProvenance {
                checked_at: 999,
                ..mirror.clone()
            },
            crate::data::MirrorProvenance {
                client_version: 0,
                ..mirror.clone()
            },
            crate::data::MirrorProvenance {
                mirrored_at: 2001,
                ..mirror.clone()
            },
        ] {
            assert!(validate_mirror_provenance(&invalid, 1000).is_err());
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
        let planned = crate::plan_build(&hero, &models, &meta, &[], &snapshots, &cfg).unwrap();
        let mut build = planned.build;
        let confidence = build.confidence.clone();
        crate::annotate_missing_authors(&mut build, &meta);
        assert_eq!(build.confidence, confidence);
        assert!(build.family.is_none());
        assert!(build.variants.is_empty());
        assert!(!build.core.is_empty());
        assert!(!build.ability_order.is_empty());
        validate_publish_input(&build).unwrap();
        validate_publish_models_with_plan(
            &build,
            &planned.purchase_plan,
            &hero,
            &models,
            &snapshots,
            2000.0,
            &cfg,
        )
        .unwrap();
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
            2000.0,
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
            validate_publish_models(&uses_missing, &hero, &models, &snapshots, 2000.0, &cfg)
                .is_err()
        );
        snapshots.push(missing);
        snapshots[0].fields.values_mut().next().unwrap().source = "deadlock_data".into();
        assert!(validate_publish_models(&build, &hero, &models, &snapshots, 2000.0, &cfg).is_err());
        assert!(
            validate_publish_models(&build, &hero, &models, &snapshots, f64::NAN, &cfg).is_err()
        );
    }

    #[test]
    fn invalid_slots_skills_bindings_and_categories_are_rejected() {
        let (build, hero, models, snapshots) = model_fixture();
        let cfg = crate::ReasonerConfig::default();
        let check = |build: &BuildObject| {
            validate_publish_models(build, &hero, &models, &snapshots, 2000.0, &cfg)
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
            provenance: None,
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
