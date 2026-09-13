use std::cmp::Ordering;

use crate::{
    BuildItem, BuildObject, BuyPhase, Confidence, CoreLayoutStats, Evidence, EvidenceKind,
    HeroModel, PatchDelta, ReasonerConfig, ScoredItem, SituationBlock, SituationKind,
};

fn confidence_rank(confidence: &Confidence) -> u8 {
    match confidence {
        Confidence::Low => 0,
        Confidence::Medium => 1,
        Confidence::High => 2,
    }
}

fn lower(value: &str) -> String {
    value.to_ascii_lowercase()
}

fn is_shield(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    name.contains("shield") || name.contains("reactive barrier")
}

fn is_can_buy_one(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    [
        "metal skin",
        "dispel magic",
        "counterspell",
        "vampiric burst",
        "spirit resilience",
        "bullet resilience",
    ]
    .iter()
    .any(|needle| name.contains(needle))
}

fn is_tryhard(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    name.contains("slowing hex")
}

fn is_optional(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    [
        "healing booster",
        "silencer",
        "spellslinger",
        "toxic bullets",
        "split shot",
        "ricochet",
        "armor piercing rounds",
        "crippling headshot",
        "spellbreaker",
        "plated armor",
        "inhibitor",
    ]
    .iter()
    .any(|needle| name.contains(needle))
}

fn is_counter(item: &ScoredItem) -> bool {
    let name = lower(&item.item.name);
    [
        "anti-heal",
        "silencer",
        "spellbreaker",
        "inhibitor",
        "crippling",
        "toxic bullets",
    ]
    .iter()
    .any(|needle| name.contains(needle))
}

fn item_order<'a>(scored: &'a [ScoredItem], blocked: &[String]) -> Vec<&'a ScoredItem> {
    let mut items = scored
        .iter()
        .filter(|item| item.item.shopable && !item.item.disabled)
        .filter(|item| item.score.total.is_finite())
        .filter(|item| {
            let id = item.item.item_id.to_string();
            !blocked.iter().any(|issue| {
                let issue = lower(issue);
                issue.contains(&lower(&item.item.name)) || issue.contains(&id)
            })
        })
        .collect::<Vec<_>>();
    items.sort_by(|left, right| {
        right
            .score
            .total
            .partial_cmp(&left.score.total)
            .unwrap_or(Ordering::Equal)
            .then_with(|| right.item.tier.cmp(&left.item.tier))
            .then_with(|| left.item.item_id.cmp(&right.item.item_id))
    });
    items
}

fn is_situation_item(item: &ScoredItem) -> bool {
    is_shield(item)
        || is_can_buy_one(item)
        || is_tryhard(item)
        || is_optional(item)
        || is_counter(item)
}

#[derive(Default)]
struct AuthorEvidence {
    core: std::collections::BTreeSet<i64>,
    sales: std::collections::BTreeMap<i64, u32>,
    ability_order: Vec<crate::AbilityStep>,
    skill_notes: Vec<String>,
}

fn author_evidence(hero_id: i64, sources: &[crate::meta::AuthorBuildSource]) -> AuthorEvidence {
    let mut counts = std::collections::BTreeMap::<i64, (usize, usize)>::new();
    let mut evidence = AuthorEvidence::default();
    for source in sources.iter().filter(|source| source.hero_id == hero_id) {
        let core = crate::meta::core_item_ids(&source.details)
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        let mut seen = std::collections::BTreeSet::new();
        for category in source
            .details
            .get("modCategories")
            .or_else(|| source.details.get("mod_categories"))
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
        {
            for item in category
                .get("mods")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
            {
                let id = item
                    .get("abilityId")
                    .or_else(|| item.get("ability_id"))
                    .and_then(|value| value.as_i64().or_else(|| value.as_str()?.parse().ok()));
                let Some(id) = id.filter(|id| *id > 0) else {
                    continue;
                };
                if seen.insert(id) {
                    let entry = counts.entry(id).or_default();
                    entry.0 += usize::from(core.contains(&id));
                    entry.1 += 1;
                }
                if let Some(priority) = item
                    .get("sellPriority")
                    .or_else(|| item.get("sell_priority"))
                    .and_then(|value| value.as_u64().or_else(|| value.as_str()?.parse().ok()))
                    .and_then(|value| u32::try_from(value).ok())
                    .filter(|priority| *priority > 0)
                {
                    evidence
                        .sales
                        .entry(id)
                        .and_modify(|existing| *existing = (*existing).min(priority))
                        .or_insert(priority);
                }
            }
        }
    }
    evidence.core = counts
        .into_iter()
        .filter_map(|(id, (core, all))| (core > all - core).then_some(id))
        .collect();
    evidence
}

fn core_candidates<'a>(
    ordered: &[&'a ScoredItem],
    authors: &AuthorEvidence,
) -> Vec<&'a ScoredItem> {
    ordered
        .iter()
        .copied()
        .filter(|item| !is_situation_item(item) || authors.core.contains(&item.item.item_id))
        .collect()
}

fn item_why(item: &ScoredItem) -> String {
    let mut details = Vec::new();
    for (name, label, unit) in [
        ("BaseAttackDamagePercent", "Waffenschaden", "%"),
        ("BonusClipSizePercent", "Magazingröße", "%"),
        ("BonusClipSize", "Schuss im Magazin", ""),
        ("BonusFireRate", "Feuerrate", "%"),
        ("TechPower", "Spirit", ""),
        ("SpiritPower", "Spirit", ""),
        ("BonusHealth", "Leben", ""),
        ("BulletResist", "Kugelresistenz", "%"),
        ("TechResist", "Spirit-Resistenz", "%"),
        ("CooldownReduction", "Abklingzeitverkürzung", "%"),
        (
            "BaseAttackDamagePercentBonus",
            "zusätzlicher Waffenschaden",
            "%",
        ),
        ("BonusDamagePercent", "zusätzlicher Schaden", "%"),
        ("DPS", "Schaden pro Sekunde während des Effekts", ""),
        ("ExplosionDamage", "Explosionsschaden", ""),
    ] {
        if let Some(value) = item
            .item
            .properties
            .get(name)
            .filter(|value| **value != 0.0)
        {
            let condition = if item.item.conditional_properties.contains(name)
                || item.item.passive_properties.contains_key(name)
            {
                " bei Auslösung"
            } else {
                ""
            };
            details.push(format!("{value:+.1}{unit} {label}{condition}"));
        }
    }
    let benefits = if details.is_empty() {
        "Der Nutzen hängt von der besonderen Wirkung und ihrer Auslösebedingung ab.".to_string()
    } else {
        format!("{}.", details.join(", "))
    };
    let condition = match &item.item.condition {
        crate::ConditionKind::None => String::new(),
        crate::ConditionKind::ActiveCooldown { cooldown, .. } => format!(" Die Aktivierung hat {cooldown:.1} Sekunden Abklingzeit; ihre Wirkung gilt nicht dauerhaft."),
        crate::ConditionKind::StateBound { threshold } => format!(" Der bedingte Bonus hängt an einer Lebensschwelle von {:.0}%; die angenommene Verfügbarkeit ist keine gemessene Trefferquote.", threshold * 100.0),
        crate::ConditionKind::MeleeBound => " Der zusätzliche Effekt setzt Nahkampftreffer voraus.".to_string(),
        crate::ConditionKind::ShotBound => " Der zusätzliche Effekt setzt passende Waffentreffer voraus.".to_string(),
        crate::ConditionKind::RampUp { ramp_seconds } => format!(" Der Effekt braucht {ramp_seconds:.1} Sekunden zum Aufbau."),
        crate::ConditionKind::ActionBound { action } if action == "parry" => " Heilung und Schadensbonus setzen eine erfolgreiche Parade voraus.".to_string(),
        crate::ConditionKind::ActionBound { .. } => " Der zusätzliche Effekt benötigt seine Auslöseaktion und gilt nicht dauerhaft.".to_string(),
    };
    let confidence = if matches!(item.confidence, Confidence::Low) {
        " Für die Empfehlung liegen noch wenige belastbare Vergleichsdaten vor."
    } else {
        ""
    };
    format!(
        "{} Seelen. {benefits}{condition}{confidence}",
        item.item.cost
    )
}

fn build_item(
    item: &ScoredItem,
    hero: &HeroModel,
    cfg: &ReasonerConfig,
    mut sources: Vec<Evidence>,
) -> BuildItem {
    let imbue_target = crate::mechanics::imbue_target(&item.item, hero, cfg);
    sources.push(Evidence {
        kind: EvidenceKind::Mechanic,
        detail: item_why(item),
    });
    BuildItem {
        item_id: item.item.item_id,
        name: item.item.name.clone(),
        tier: item.item.tier,
        buy_phase: item.buy_phase.clone(),
        why: sources
            .iter()
            .filter(|source| {
                matches!(source.kind, EvidenceKind::Meta)
                    && source.detail.starts_with("Zusammen mit ")
            })
            .fold(item_why(item), |mut why, source| {
                why.push(' ');
                why.push_str(&source.detail);
                why
            }),
        confidence: if item.score.total <= 0.0 {
            Confidence::Low
        } else {
            item.confidence.clone()
        },
        imbue_target,
        sell_priority: match item.buy_phase {
            BuyPhase::Lane => Some(if item.item.tier <= 1 { 1 } else { 2 }),
            BuyPhase::Mid | BuyPhase::Core | BuyPhase::Late => None,
        },
        sources,
    }
}

fn confidence(items: &[BuildItem]) -> Confidence {
    items
        .iter()
        .map(|item| item.confidence.clone())
        .min_by_key(confidence_rank)
        .unwrap_or(Confidence::Low)
}

fn patch_sources(item: &ScoredItem, deltas: &[PatchDelta]) -> Vec<Evidence> {
    let mut sources = item
        .sources
        .iter()
        .filter(|source| !matches!(source.kind, EvidenceKind::Mechanic))
        .cloned()
        .collect::<Vec<_>>();
    for delta in deltas {
        if matches!(delta.target, crate::DeltaTarget::Item(item_id) if item_id == item.item.item_id)
        {
            sources.push(Evidence {
                kind: EvidenceKind::Patch,
                detail: delta.note.clone(),
            });
        }
    }
    sources
}

pub fn compose_build(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
) -> crate::Result<BuildObject> {
    compose_build_with_blocklist(hero, scored, deltas, cfg, &[])
}

pub fn compose_build_with_sources(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
    blocked: &[String],
    meta: &crate::meta::MetaIndexWithSources,
) -> crate::Result<BuildObject> {
    let mut authors = author_evidence(hero.hero_id, &meta.author_builds);
    let (raw_order, source) = meta.ability_order(hero.hero_id);
    let (order, notes) = crate::progression::coherent_order(hero, &raw_order);
    authors.skill_notes = notes;
    authors.ability_order = order.clone();
    let mut build = compose_build_with_author_evidence(
        hero,
        scored,
        deltas,
        cfg,
        blocked,
        (
            meta.core_layouts.for_hero(hero.hero_id),
            &meta.combinations,
            &authors,
        ),
    )?;
    build.ability_order = order;
    build.rationale = format!(
        "{} {}",
        source.detail,
        build
            .rationale
            .strip_prefix("Skill-Order: keine Quelle")
            .unwrap_or(&build.rationale)
            .trim()
    );
    Ok(build)
}

pub fn purchase_plan_with_sources(
    hero: &HeroModel,
    scored: &[ScoredItem],
    cfg: &ReasonerConfig,
    meta: &crate::meta::MetaIndexWithSources,
) -> crate::Result<crate::planner::PurchasePlan> {
    let mut authors = author_evidence(hero.hero_id, &meta.author_builds);
    (authors.ability_order, authors.skill_notes) =
        crate::progression::coherent_order(hero, &meta.ability_order(hero.hero_id).0);
    plan_core(
        hero,
        scored,
        cfg,
        &[],
        (
            meta.core_layouts.for_hero(hero.hero_id),
            &meta.combinations,
            &authors,
        ),
    )
}

fn plan_core(
    hero: &HeroModel,
    scored: &[ScoredItem],
    cfg: &ReasonerConfig,
    blocked: &[String],
    context: (
        &CoreLayoutStats,
        &std::collections::BTreeMap<(i64, i64), crate::meta::CombinationSupport>,
        &AuthorEvidence,
    ),
) -> crate::Result<crate::planner::PurchasePlan> {
    let (layout, combinations, authors) = context;
    let catalog = scored
        .iter()
        .map(|item| item.item.clone())
        .collect::<Vec<_>>();
    let rules = crate::inventory::InventoryRules::from_catalog(&catalog)?;
    let ordered = item_order(scored, blocked);
    let mut plan = crate::planner::plan_with_economy(
        hero,
        scored,
        &core_candidates(&ordered, authors),
        cfg,
        crate::planner::PlanningContext {
            layout,
            rules: &rules,
            combinations,
            order: &authors.ability_order,
            economy: &crate::planner::EconomyPolicy::default(),
        },
    );
    plan.assumptions.extend(authors.skill_notes.iter().cloned());
    Ok(plan)
}

pub fn compose_build_with_blocklist(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    _cfg: &ReasonerConfig,
    blocked: &[String],
) -> crate::Result<BuildObject> {
    compose_build_with_layout_and_blocklist(
        hero,
        scored,
        deltas,
        _cfg,
        blocked,
        &CoreLayoutStats::default(),
        &Default::default(),
    )
}

pub fn compose_build_with_layout(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
    layout: &CoreLayoutStats,
) -> crate::Result<BuildObject> {
    compose_build_with_layout_and_blocklist(
        hero,
        scored,
        deltas,
        cfg,
        &[],
        layout,
        &Default::default(),
    )
}

fn compose_build_with_layout_and_blocklist(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
    blocked: &[String],
    layout: &CoreLayoutStats,
    combinations: &std::collections::BTreeMap<(i64, i64), crate::meta::CombinationSupport>,
) -> crate::Result<BuildObject> {
    compose_build_with_author_evidence(
        hero,
        scored,
        deltas,
        cfg,
        blocked,
        (layout, combinations, &AuthorEvidence::default()),
    )
}

fn compose_build_with_author_evidence(
    hero: &HeroModel,
    scored: &[ScoredItem],
    deltas: &[PatchDelta],
    cfg: &ReasonerConfig,
    blocked: &[String],
    context: (
        &CoreLayoutStats,
        &std::collections::BTreeMap<(i64, i64), crate::meta::CombinationSupport>,
        &AuthorEvidence,
    ),
) -> crate::Result<BuildObject> {
    let (_layout, combinations, authors) = context;
    let ordered = item_order(scored, blocked);
    let plan = plan_core(hero, scored, cfg, blocked, context)?;
    let selected = plan
        .steps
        .iter()
        .filter_map(|step| {
            ordered
                .iter()
                .copied()
                .find(|item| item.item.item_id == step.transition.purchased_id)
        })
        .collect::<Vec<_>>();
    let sales = plan
        .steps
        .iter()
        .flat_map(|step| {
            step.transition
                .sold_ids
                .iter()
                .map(|id| (*id, step.transition.purchased_id))
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let selected_ids = selected
        .iter()
        .map(|item| item.item.item_id)
        .collect::<std::collections::BTreeSet<_>>();
    let core = selected
        .iter().enumerate()
        .map(|(index, item)| {
            let mut sources = patch_sources(item, deltas);
            let step = &plan.steps[index];
            for other in selected[..index].iter().filter(|other| step.transition.after.held_ids.contains(&other.item.item_id)) {
                if let Some(support) = combinations.get(&(item.item.item_id.min(other.item.item_id), item.item.item_id.max(other.item.item_id))) {
                    sources.push(Evidence { kind: EvidenceKind::Meta, detail: format!("Zusammen mit {}: beobachtete Siegquote {:+.2} Prozentpunkte gegenüber dem stärkeren Einzel-Item, {} gemeinsame Spiele im aktuellen Patch. Statistisches Nebensignal, kein Beweis für eine mechanische Wechselwirkung.", other.item.name, support.relative_lift * 100.0, support.matches) });
                }
            }
            let (progressed, _) = crate::progression::at_souls(hero, &plan.ability_order, step.progression.earned_souls, cfg);
            let mut built = build_item(item, &progressed, cfg, sources);
            built.imbue_target = step.imbue_targets.get(&item.item.item_id).copied();
            built.sell_priority = None;
            let detail = format!("Im bisherigen Build: {:+.1} gemeinsamer Kampfnutzen. Kaufpreis {} Seelen, Verkaufserlös {}, zusätzliche Ausgabe {}, insgesamt {} Seelen. Danach {} Items im Inventar; im Kampffenster {:.0} Waffenschaden, {:.0} Fähigkeitsschaden und {:.0} Auslöserschaden.", step.marginal_value, step.transition.purchase_cost, step.transition.sale_return, step.transition.net_cost, step.transition.after.spent_souls, step.transition.after.held_ids.len(), step.evaluation.weapon_damage, step.evaluation.ability_damage, step.evaluation.proc_damage);
            built.why.push(' ');
            built.why.push_str(&detail);
            built.sources.push(Evidence { kind: EvidenceKind::Mechanic, detail });
            let detail = format!("Erwerb bei {} verdienten Seelen, Level {} und {} bereits angewandten Schritten der veröffentlichten Skillfolge; Fähigkeitsränge {:?}. Nach dem Kauf bleiben {} Seelen verfügbar.", step.progression.earned_souls, step.progression.reached_level, step.progression.applied_order_steps, step.progression.ability_ranks, step.progression.earned_souls-step.transition.after.spent_souls);
            built.why.push(' ');
            built.why.push_str(&detail);
            built.sources.push(Evidence { kind: EvidenceKind::Mechanic, detail });
            let previous_bonuses = if index == 0 { std::collections::BTreeMap::new() } else { plan.steps[index-1].evaluation.shop_bonuses.clone() };
            for (category, bonus) in &step.evaluation.shop_bonuses {
                let previous = previous_bonuses.get(category).copied().unwrap_or(0.0);
                if (*bonus-previous).abs() > f64::EPSILON {
                    let label = match category.as_str() { "weapon" => "% Waffenschaden", "vitality" => "% Leben", "spirit" => " Spirit", _ => category.as_str() };
                    let detail = format!("Der Shopbonus aus dem gehaltenen Kategorienwert ändert sich von {previous:.1} auf {bonus:.1}{label}; die Schwelle stammt aus dem aktuellen Helden-Snapshot.");
                    built.why.push(' ');
                    built.why.push_str(&detail);
                    built.sources.push(Evidence { kind: EvidenceKind::Mechanic, detail });
                }
            }
            for consumed in &step.transition.consumed_ids {
                if let Some(previous) = scored.iter().find(|candidate| candidate.item.item_id == *consumed) {
                    let detail = format!("{} geht im Upgrade auf; dessen Preis ist angerechnet und seine Einzelwirkung wird nicht zusätzlich behalten.", previous.item.name);
                    built.why.push(' ');
                    built.why.push_str(&detail);
                    built.sources.push(Evidence { kind: EvidenceKind::Mechanic, detail });
                }
            }
            if authors.core.contains(&item.item.item_id) {
                let detail = "In den beobachteten Builds dieses Helden steht dieses Item überwiegend im Kern.".to_string();
                built.why.push(' ');
                built.why.push_str(&detail);
                built.sources.push(Evidence { kind: EvidenceKind::Author, detail });
            }
            if let Some(next_id) = sales.get(&item.item.item_id) {
                if let Some(next) = selected.iter().find(|next| next.item.item_id == *next_id) {
                    built.sell_priority = Some(authors.sales.get(&item.item.item_id).copied().unwrap_or(index as u32 + 1));
                    let detail = format!("Verkaufe {} vor dem Kauf von {}, damit der benötigte Platz frei wird. Die gemeinsame Bewertung berücksichtigt den dabei verlorenen Nutzen.", item.item.name, next.item.name);
                    built.why.push(' ');
                    built.why.push_str(&detail);
                    built.sources.push(Evidence { kind: EvidenceKind::Mechanic, detail });
                }
            }
            for (sold_id, next_id) in &sales {
                if *next_id == item.item.item_id {
                    if let Some(sold) = selected.iter().find(|sold| sold.item.item_id == *sold_id) {
                        built.why.push_str(&format!(" Vorher {} verkaufen; sonst fehlt der Platz.", sold.item.name));
                    }
                }
            }
            built
        })
        .collect::<Vec<_>>();
    let mut can_buy = Vec::new();
    let mut tryhard = Vec::new();
    let mut shields = Vec::new();
    let mut counters = Vec::new();
    let mut optional = Vec::new();
    for item in ordered {
        if selected_ids.contains(&item.item.item_id) {
            continue;
        }
        if is_shield(item) {
            shields.push(build_item(item, hero, cfg, patch_sources(item, deltas)));
        } else if is_can_buy_one(item) {
            can_buy.push(build_item(item, hero, cfg, patch_sources(item, deltas)));
        } else if is_tryhard(item) {
            tryhard.push(build_item(item, hero, cfg, patch_sources(item, deltas)));
        } else if is_optional(item) {
            optional.push(build_item(item, hero, cfg, patch_sources(item, deltas)));
        } else if is_counter(item) {
            counters.push(build_item(item, hero, cfg, patch_sources(item, deltas)));
        } else {
            optional.push(build_item(item, hero, cfg, patch_sources(item, deltas)));
        }
    }
    can_buy.truncate(6);
    optional.truncate(12);
    let mut situations = Vec::new();
    if !can_buy.is_empty() {
        situations.push(SituationBlock {
            label: "Ein Item nach Bedarf".to_string(),
            optional: true,
            kind: SituationKind::CanBuyN(1),
            items: can_buy,
        });
    }
    if !tryhard.is_empty() {
        situations.push(SituationBlock {
            label: "Für schwere Gegner".to_string(),
            optional: true,
            kind: SituationKind::Tryhard,
            items: tryhard,
        });
    }
    if !shields.is_empty() {
        situations.push(SituationBlock {
            label: "Schutz".to_string(),
            optional: true,
            kind: SituationKind::Shields,
            items: shields,
        });
    }
    if !optional.is_empty() {
        situations.push(SituationBlock {
            label: "Optional".to_string(),
            optional: true,
            kind: SituationKind::Optional,
            items: optional,
        });
    }
    if !counters.is_empty() {
        situations.push(SituationBlock {
            label: "Gegen bestimmte Gegner".to_string(),
            optional: true,
            kind: SituationKind::Counters,
            items: counters,
        });
    }
    let mut all_items = core.clone();
    all_items.extend(situations.iter().flat_map(|block| block.items.clone()));
    Ok(BuildObject {
        hero_id: hero.hero_id,
        hero_name: hero.name.clone(),
        patch_tag: cfg.patch_tag.clone(),
        name: format!("{} Reasoner Build", hero.name),
        core,
        situations,
        ability_order: Vec::new(),
        confidence: confidence(&all_items),
        rationale: std::iter::once(&"Skill-Order: keine Quelle".to_string()).chain(plan.assumptions.iter()).chain(plan.final_evaluation.assumptions.iter())
            .chain(plan.final_evaluation.unknown_effects.iter()).cloned()
            .chain(plan.final_evaluation.scenarios.iter().map(|scenario| format!("Ablauf {}: {}. {:.0} Schüsse, {} Nachladungen, {:.1} Sekunden Kanalzeit; Fähigkeiten {:?}.", scenario.name, scenario.sequence.join(" → "), scenario.shots, scenario.reloads, scenario.channel_seconds, scenario.casts)))
            .collect::<Vec<_>>().join(" "),
    })
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn missing_upgrade_component_fails_before_any_persistence() {
        let mut broken = item(1, "Kaputtes Upgrade", 10.0, false, &[]);
        broken.item.component_items.push("missing_component".into());
        let cfg = ReasonerConfig::default();
        let result =
            compose_build_with_layout(&hero(), &[broken], &[], &cfg, &layout(&[(2, 1)], 0));
        assert!(
            matches!(&result, Err(crate::ReasonerError::Data(message)) if message.contains("missing_component"))
        );
        let writes = std::cell::Cell::new(0);
        let failed = crate::finish_build_with_persistence(
            result.map(|build| (build, Vec::new())),
            true,
            |build, _| {
                writes.set(writes.get() + 1);
                async { Ok(build) }
            },
        )
        .await;
        assert!(failed.is_err());
        assert_eq!(writes.get(), 0);

        let legal = compose_build_with_layout(
            &hero(),
            &[item(2, "Legal", 10.0, false, &[])],
            &[],
            &cfg,
            &layout(&[(2, 1)], 0),
        );
        let saved = crate::finish_build_with_persistence(
            legal.map(|build| (build, Vec::new())),
            true,
            |build, _| {
                writes.set(writes.get() + 1);
                async { Ok(build) }
            },
        )
        .await
        .unwrap();
        assert_eq!(saved.core.len(), 1);
        assert_eq!(writes.get(), 1);
        let incomplete =
            compose_build_with_layout(&hero(), &[], &[], &cfg, &layout(&[(2, 1)], 0)).unwrap();
        assert!(incomplete.core.is_empty());
        assert!(incomplete.rationale.contains("ungefüllte Layoutplätze"));
    }

    #[test]
    fn author_core_evidence_overrides_global_names_only_for_the_same_hero() {
        let source = |hero_id, name: &str| crate::meta::AuthorBuildSource {
            hero_id,
            author: name.to_string(),
            weight: 1.0,
            details: serde_json::json!({"modCategories":[{"name":"CORE","mods":[{"abilityId":1}]}]}),
        };
        let items = [
            item(1, "Ricochet", 10.0, false, &[]),
            item(2, "Andere Wahl", 5.0, false, &[]),
        ];
        let layout = layout(&[(2, 1)], 0);
        let other_hero = author_evidence(25, &[source(26, "Anderer Held")]);
        let baseline = compose_build_with_author_evidence(
            &hero(),
            &items,
            &[],
            &ReasonerConfig::default(),
            &[],
            (&layout, &Default::default(), &other_hero),
        )
        .unwrap();
        assert_eq!(baseline.core[0].item_id, 2);
        let same_hero = author_evidence(25, &[source(25, "Beobachtet")]);
        let build = compose_build_with_author_evidence(
            &hero(),
            &items,
            &[],
            &ReasonerConfig::default(),
            &[],
            (&layout, &Default::default(), &same_hero),
        )
        .unwrap();
        assert_eq!(build.core[0].item_id, 1);
        assert!(build.core[0].why.contains("überwiegend im Kern"));
    }

    #[test]
    fn actual_sales_release_slots_and_sold_pairs_do_not_rank_or_explain() {
        let mut items = (1..=12)
            .map(|id| {
                let mut early = item(
                    id,
                    &format!("Früh {id}"),
                    if id == 12 { 1.0 } else { 100.0 },
                    false,
                    &[],
                );
                early.item.tier = 1;
                early.item.cost = 800;
                early.buy_phase = BuyPhase::Lane;
                early
            })
            .collect::<Vec<_>>();
        let mut ghost_partner = item(13, "Partner des verkauften Items", 1.0, false, &[]);
        ghost_partner.item.tier = 3;
        ghost_partner.item.cost = 3200;
        ghost_partner.score.per_slot_value = 100.0;
        let mut replacement = item(14, "Später Kauf", 50.0, false, &[]);
        replacement.item.tier = 3;
        replacement.item.cost = 3200;
        items.extend([ghost_partner, replacement]);
        let layout = layout(&[(1, 12), (3, 1)], 0);
        let combinations = BTreeMap::from([(
            (12, 13),
            crate::meta::CombinationSupport {
                relative_lift: 1.0,
                matches: 1000,
            },
        )]);
        let no_sale = compose_build_with_author_evidence(
            &hero(),
            &items,
            &[],
            &ReasonerConfig::default(),
            &[],
            (&layout, &combinations, &AuthorEvidence::default()),
        )
        .unwrap();
        assert_eq!(no_sale.core.len(), 13);
        assert_eq!(no_sale.core.last().unwrap().item_id, 14);
        let authors = author_evidence(
            25,
            &[crate::meta::AuthorBuildSource {
                hero_id: 25,
                author: "Verkaufsbeleg".to_string(),
                weight: 1.0,
                details: serde_json::json!({"modCategories":[{"name":"CORE","mods":[{"abilityId":12,"sellPriority":3}]}]}),
            }],
        );
        let build = compose_build_with_author_evidence(
            &hero(),
            &items,
            &[],
            &ReasonerConfig::default(),
            &[],
            (&layout, &combinations, &authors),
        )
        .unwrap();
        assert_eq!(build.core.len(), 13);
        assert_eq!(build.core.last().unwrap().item_id, 14);
        let sold = build.core.iter().find(|item| item.item_id == 12).unwrap();
        assert_eq!(sold.sell_priority, Some(3));
        assert!(sold
            .why
            .contains("Verkaufe Früh 12 vor dem Kauf von Später Kauf"));
        assert!(!build
            .core
            .last()
            .unwrap()
            .sources
            .iter()
            .any(|source| source.detail.starts_with("Zusammen mit Früh 12:")));
        let payload = crate::publish::to_publish_payload(&build);
        assert!(payload.mod_categories[0]
            .mods
            .iter()
            .any(|item| item.annotation.contains("Verkaufe Früh 12")));
    }

    #[test]
    fn imbue_uses_mechanics_gain_and_why_does_not_leak_score_fields() {
        let mut hero = hero();
        let ability = |id, effect| {
            serde_json::from_value(serde_json::json!({
            "ability_id":id,"class_name":"damage","slot":1,"roles":[],"scaling":[],"channel_time":null,
            "charges":1,"cooldown":10.0,"scaling_step":null,"damage_type":"Weapon","base_effect":effect,"tick_rate":null,"duration":null
        })).unwrap()
        };
        hero.abilities = vec![ability(1, 10.0), ability(42, 100.0), ability(0, 10000.0)];
        let mut candidate = item(1, "Prüfung", 5.0, false, &[]);
        candidate.item.imbueable = true;
        candidate
            .item
            .properties
            .insert("BonusClipSizePercent".to_string(), 100.0);
        candidate
            .item
            .properties
            .insert("CooldownReduction".to_string(), 10.0);
        let built = build_item(
            &candidate,
            &hero,
            &ReasonerConfig::default(),
            vec![Evidence {
                kind: EvidenceKind::Mechanic,
                detail: "per_soul_value und scaling_step".to_string(),
            }],
        );
        assert_eq!(built.imbue_target, Some(42));
        assert!(built.why.contains("100.0% Magazingröße"));
        assert!(!built.why.contains("per_soul_value"));
        assert!(!built.why.contains("scaling_step"));
    }

    #[test]
    fn selected_pair_changes_the_next_purchase_and_explains_observation() {
        let mut anchor = item(1, "Anker", 30.0, false, &[]);
        anchor.item.tier = 1;
        anchor.score.per_soul_value = 30.0;
        let mut solo = item(2, "Einzeln stärker", 10.1, false, &[]);
        solo.item.tier = 3;
        let mut partner = item(3, "Passender Partner", 10.0, false, &[]);
        partner.item.tier = 3;
        partner.score.per_slot_value = 10.0;
        let items = [anchor, solo, partner];
        let layout = layout(&[(1, 1), (3, 1)], 0);
        let combinations = BTreeMap::from([(
            (1, 3),
            crate::meta::CombinationSupport {
                relative_lift: 0.2,
                matches: 1000,
            },
        )]);
        let build = compose_build_with_layout_and_blocklist(
            &hero(),
            &items,
            &[],
            &ReasonerConfig::default(),
            &[],
            &layout,
            &combinations,
        )
        .unwrap();
        let baseline =
            compose_build_with_layout(&hero(), &items, &[], &ReasonerConfig::default(), &layout)
                .unwrap();
        assert_eq!(
            baseline
                .core
                .iter()
                .map(|item| item.item_id)
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_eq!(
            build
                .core
                .iter()
                .map(|item| item.item_id)
                .collect::<Vec<_>>(),
            [1, 3]
        );
        assert!(build.core[1].why.contains("Zusammen mit Anker"));
        assert!(build.core[1].why.contains("kein Beweis"));
        let absent = compose_build_with_layout_and_blocklist(
            &hero(),
            &items[1..],
            &[],
            &ReasonerConfig::default(),
            &[],
            &layout,
            &combinations,
        )
        .unwrap();
        assert_eq!(absent.core[0].item_id, 2);
    }

    #[test]
    fn matching_class_only_ability_never_becomes_imbue_zero() {
        let mut hero = hero();
        let mut ability: crate::AbilityModel = serde_json::from_value(serde_json::json!({
            "ability_id": 0, "class_name": "class_only", "slot": 1, "roles": [], "scaling": [], "channel_time": null,
            "charges": 0, "cooldown": 0.0, "scaling_step": null, "damage_type": "Weapon", "base_effect": 0.0, "tick_rate": null, "duration": null
        })).unwrap();
        hero.abilities.push(ability.clone());
        ability.ability_id = 42;
        ability.damage_type = DamageType::Spirit;
        hero.abilities.push(ability);
        let mut candidate = item(1, "Imbue", 1.0, false, &[]);
        candidate.item.imbueable = true;
        assert_eq!(
            build_item(&candidate, &hero, &ReasonerConfig::default(), vec![]).imbue_target,
            Some(42)
        );
        hero.abilities.pop();
        assert_eq!(
            build_item(&candidate, &hero, &ReasonerConfig::default(), vec![]).imbue_target,
            None
        );
    }

    #[test]
    fn fix_e_nonpositive_scores_never_enter_core_or_keep_high_confidence() {
        let scored = vec![
            item(1, "Zero", 0.0, false, &[]),
            item(2, "Negative", -2.0, false, &[]),
        ];
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(2, 19)], 48),
        )
        .unwrap();
        assert!(build.core.is_empty());
        assert!(build
            .situations
            .iter()
            .flat_map(|block| &block.items)
            .all(|item| item.confidence != Confidence::High));
    }

    #[test]
    fn fix_e_optional_keeps_only_twelve_highest_scores() {
        let scored = (1..=60)
            .map(|id| item(id, &format!("Item {id}"), id as f64, false, &[]))
            .collect::<Vec<_>>();
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(2, 19)], 48),
        )
        .unwrap();
        let optional = build
            .situations
            .iter()
            .find(|block| block.label == "Optional")
            .unwrap();
        assert_eq!(optional.items.len(), 12);
        assert_eq!(
            optional
                .items
                .iter()
                .map(|item| item.item_id)
                .collect::<Vec<_>>(),
            scored
                .iter()
                .rev()
                .filter(|item| !build
                    .core
                    .iter()
                    .any(|chosen| chosen.item_id == item.item.item_id))
                .take(12)
                .map(|item| item.item.item_id)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn fix_e_composer_does_not_apply_deltas_to_scores_again() {
        let scored = vec![
            item(1, "First", 10.0, false, &[]),
            item(2, "Second", 9.0, false, &[]),
        ];
        let deltas = crate::patch::compute_patch_delta(
            &hero(),
            &[
                serde_json::json!({"item_id":2,"change_type":"buff","old_value":1,"new_value":100,"raw_line":"Damage increased from 1 to 100"}),
            ],
        );
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &deltas,
            &ReasonerConfig::default(),
            &layout(&[(2, 2)], 0),
        )
        .unwrap();
        assert_eq!(build.core[0].item_id, 1);
    }
    use super::*;
    use crate::{
        BuyPhase, CoreLayoutBand, CoreLayoutStats, DamagePlan, DamageType, ItemModel, ItemScore,
        PurchaseBonuses, SlotType, WeaponProfile,
    };
    use std::collections::BTreeMap;

    fn hero() -> HeroModel {
        HeroModel {
            hero_id: 25,
            name: "Warden".to_string(),
            archetype: "brawler".to_string(),
            base_health: 1.0,
            level_curve: Vec::new(),
            purchase_bonuses: PurchaseBonuses {
                spirit: Vec::new(),
                weapon: Vec::new(),
                vitality: Vec::new(),
            },
            cost_bonuses: Default::default(),
            level_rewards: Default::default(),
            standard_level_up_upgrades: Default::default(),
            standard_upgrade_levels: Default::default(),
            base_spirit_power: 0.0,
            scaling: Vec::new(),
            weapon: WeaponProfile {
                bullet_damage: 1.0,
                shots_per_second: 1.0,
                clip_size: 1.0,
                reload_duration: 1.0,
                range: 1.0,
                falloff_start_range: 1.0,
                falloff_end_range: 1.0,
                sustained_dps: 1.0,
            },
            abilities: Vec::new(),
            damage_plan: DamagePlan {
                weapon_dps: 1.0,
                spirit_dps: 0.0,
                weapon_share: 1.0,
                primary_axis: DamageType::Weapon,
            },
        }
    }

    fn item(id: i64, name: &str, score: f64, active: bool, defense: &[&str]) -> ScoredItem {
        ScoredItem {
            item: ItemModel {
                item_id: id,
                class_name: format!("item_{id}"),
                component_items: Vec::new(),
                property_damage_types: Default::default(),
                property_spirit_scaling: Default::default(),
                description: String::new(),
                name: name.to_string(),
                slot: SlotType::Weapon,
                tier: 2,
                cost: 1,
                is_active: active,
                shopable: true,
                disabled: false,
                damage_axis: DamageType::Weapon,
                defense_kind: defense.iter().map(|value| value.to_string()).collect(),
                properties: BTreeMap::from([("BaseAttackDamagePercent".into(), score)]),
                passive_properties: BTreeMap::new(),
                conditional_properties: Default::default(),
                condition: crate::ConditionKind::None,
                proc_cooldown: None,
                imbueable: false,
            },
            score: ItemScore {
                combat_value: score,
                per_slot_value: 0.0,
                per_soul_value: 0.0,
                purchase_bonus_value: 0.0,
                condition_factor: 1.0,
                active_value: 0.0,
                passive_value: 0.0,
                meta_support: 0.0,
                total: score,
            },
            confidence: Confidence::High,
            buy_phase: BuyPhase::Core,
            sources: vec![Evidence {
                kind: EvidenceKind::Mechanic,
                detail: "fixture".to_string(),
            }],
        }
    }

    fn layout(targets: &[(i64, usize)], flex_slots: usize) -> CoreLayoutStats {
        CoreLayoutStats {
            source_builds: 1,
            total_median: targets.iter().map(|(_, target)| *target).sum::<usize>() as f64,
            total_lower_quartile: 0.0,
            total_upper_quartile: 0.0,
            flex_slots,
            bands: targets
                .iter()
                .map(|(tier, target)| {
                    (
                        *tier,
                        CoreLayoutBand {
                            tier: *tier,
                            median: *target as f64,
                            lower_quartile: 0.0,
                            upper_quartile: 0.0,
                            target: *target,
                        },
                    )
                })
                .collect(),
        }
    }

    #[test]
    fn does_not_invent_sales_for_lane_items_when_inventory_has_room() {
        let mut lane = item(1, "Lane", 3.0, false, &[]);
        lane.buy_phase = BuyPhase::Lane;
        lane.item.tier = 1;
        let mut early = item(2, "Early", 2.0, false, &[]);
        early.buy_phase = BuyPhase::Lane;
        let mut core = item(3, "Core", 1.0, false, &[]);
        core.item.tier = 1;
        let build = compose_build_with_layout(
            &hero(),
            &[lane, early, core],
            &[],
            &ReasonerConfig::default(),
            &layout(&[(1, 2), (2, 1)], 0),
        )
        .unwrap();
        let payload = crate::publish::publish_task_payload(&build);
        assert!(build.core.iter().all(|item| item.sell_priority.is_none()));
        assert!(payload["mod_categories"][0]["mods"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item
                .get("sell_priority")
                .is_none_or(serde_json::Value::is_null)));
    }

    #[test]
    fn keeps_reference_situation_blocks_without_copying_the_seed_core() {
        let seed: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../.tasks/2026-09-12-build-reasoner/referenz/lightbringer-warden.json"
        ))
        .unwrap();
        let blocks = seed["bloecke"].as_array().unwrap();
        let mut scored = Vec::new();
        for block in blocks {
            for entry in block["items"].as_array().unwrap() {
                let name = entry["name"].as_str().unwrap();
                let defense = match name {
                    "Spirit Resilience" | "Bullet Resilience" | "Metal Skin"
                    | "Reactive Barrier" | "Witchmail" | "Plated Armor" => vec!["resist"],
                    "Veil Walker" | "Spirit Shielding" | "Weapon Shielding" => vec!["shield"],
                    _ => vec![],
                };
                let mut candidate = item(
                    scored.len() as i64 + 1,
                    name,
                    100.0 - scored.len() as f64,
                    entry["active"].as_bool().unwrap_or(false) || name == "Reactive Barrier",
                    &defense,
                );
                candidate.item.tier = entry["tier"].as_i64().unwrap();
                if name == "Blood Tribute" {
                    candidate.item.damage_axis = DamageType::Spirit;
                }
                candidate.item.slot = match name {
                    "High-Velocity Rounds"
                    | "Opening Rounds"
                    | "Monster Rounds"
                    | "Swift Striker"
                    | "Titanic Magazine"
                    | "Fleetfoot"
                    | "Spiritual Overflow"
                    | "Blood Tribute" => SlotType::Weapon,
                    "Quicksilver Reload"
                    | "Mercurial Magnum"
                    | "Boundless Spirit"
                    | "Transcendent Cooldown" => SlotType::Spirit,
                    _ => SlotType::Vitality,
                };
                scored.push(candidate);
            }
        }
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(1, 3), (2, 6), (3, 2), (4, 8)], 7),
        )
        .unwrap();
        assert_eq!(build.situations.len(), 4);
        let actual = std::iter::once(("Core Items", &build.core)).chain(
            build
                .situations
                .iter()
                .map(|block| (block.label.as_str(), &block.items)),
        );
        for ((label, items), expected) in actual.zip(blocks) {
            let expected_label = match expected["name"].as_str().unwrap() {
                "Can buy 1" => "Ein Item nach Bedarf",
                "Tryhard" => "Für schwere Gegner",
                "Shields" => "Schutz",
                other => other,
            };
            assert_eq!(label, expected_label);
            let mut expected_items = expected["items"]
                .as_array()
                .unwrap()
                .iter()
                .collect::<Vec<_>>();
            if label == "Core Items" {
                expected_items.sort_by_key(|item| item["tier"].as_i64().unwrap());
            }
            if label == "Core Items" {
                let reference_names = expected_items
                    .iter()
                    .map(|item| item["name"].as_str().unwrap())
                    .collect::<std::collections::BTreeSet<_>>();
                assert!(items
                    .iter()
                    .all(|item| reference_names.contains(item.name.as_str())));
                assert!(!items.is_empty());
                continue;
            }
            if label == "Optional" {
                assert!(items.len() <= 12);
                continue;
            }
            assert_eq!(
                items
                    .iter()
                    .map(|item| item.name.as_str())
                    .collect::<Vec<_>>(),
                expected_items
                    .iter()
                    .map(|item| item["name"].as_str().unwrap())
                    .collect::<Vec<_>>(),
                "{label}"
            );
        }
        let listed = build
            .core
            .iter()
            .chain(build.situations.iter().flat_map(|block| &block.items))
            .map(|item| item.item_id)
            .collect::<Vec<_>>();
        assert_eq!(
            listed.len(),
            listed
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
        );
        assert!(listed
            .iter()
            .all(|id| scored.iter().any(|item| item.item.item_id == *id)));
    }

    #[test]
    fn ability_sources_and_sell_priorities_survive_publish_roundtrip() {
        use crate::meta::{AuthorBuildSource, MetaIndexWithSources};
        use crate::{AbilityStep, CoreLayoutIndex, MetaIndex};
        let steps = |id| {
            vec![AbilityStep {
                ability_id: id,
                currency_type: 2,
                delta: -1,
            }]
        };
        let source = |hero_id, weight, author: &str, id| AuthorBuildSource {
            hero_id,
            weight,
            author: author.to_string(),
            details: serde_json::json!({"abilityOrder":{"currencyChanges":[
                {"abilityId":id,"currencyType":2,"delta":-1},
                {"abilityId":id,"currencyType":1,"delta":-2}
            ]}}),
        };
        let mut meta = MetaIndexWithSources {
            index: MetaIndex {
                by_item: BTreeMap::new(),
                sample_ok: Default::default(),
            },
            author_builds: vec![
                source(25, 1.0, "Lower", 100),
                source(25, 3.0, "Best", 101),
                source(99, 9.0, "Other hero", 999),
            ],
            hero_ability_orders: BTreeMap::from([(25, steps(102))]),
            core_layouts: CoreLayoutIndex {
                by_hero: BTreeMap::from([(25, layout(&[(1, 1)], 0))]),
                overall: CoreLayoutStats::default(),
            },
            combinations: BTreeMap::new(),
        };
        let mut lane = item(1, "Lane", 1.0, false, &[]);
        lane.buy_phase = BuyPhase::Lane;
        lane.item.tier = 1;
        let compose = |meta: &MetaIndexWithSources| {
            compose_build_with_sources(
                &hero(),
                std::slice::from_ref(&lane),
                &[],
                &ReasonerConfig::default(),
                &[],
                meta,
            )
            .unwrap()
        };
        let build = compose(&meta);
        let restored_build: BuildObject =
            serde_json::from_value(serde_json::to_value(&build).unwrap()).unwrap();
        let payload = crate::publish::publish_task_payload(&restored_build);
        let restored: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&payload).unwrap()).unwrap();
        assert_eq!(payload["ability_order"][0]["ability_id"], 101);
        assert_eq!(payload["ability_order"][1]["delta"], -2);
        assert!(restored["mod_categories"][0]["mods"][0]
            .get("sell_priority")
            .is_none_or(serde_json::Value::is_null));
        assert_eq!(restored["ability_order"].as_array().unwrap().len(), 2);
        assert!(restored["description"].as_str().unwrap().contains("Best"));
        meta.author_builds.reverse();
        assert_eq!(compose(&meta).ability_order, build.ability_order);
        meta.author_builds
            .push(source(25, f64::NAN, "Invalid weight", 998));
        assert_eq!(compose(&meta).ability_order, build.ability_order);
        meta.author_builds.retain(|source| source.hero_id != 25);
        let mut malformed = source(25, 5.0, "Incomplete", 103);
        malformed.details["abilityOrder"]["currencyChanges"][1]
            .as_object_mut()
            .unwrap()
            .remove("delta");
        meta.author_builds.push(malformed);
        let fallback = compose(&meta);
        assert_eq!(fallback.ability_order, steps(102));
        assert!(fallback.rationale.contains("brain.hero_ability_orders"));
        meta.hero_ability_orders.clear();
        let missing = compose(&meta);
        assert!(missing.ability_order.is_empty());
        assert!(missing.rationale.contains("keine Quelle"));
        meta.author_builds.push(AuthorBuildSource {
            hero_id: 25,
            author: "Snake case".to_string(),
            weight: 1.0,
            details: serde_json::json!({"ability_order": [
                {"ability_id":"104", "currency_type":"2", "delta":"-1"}
            ]}),
        });
        assert_eq!(compose(&meta).ability_order, steps(104));
        let default_build = compose_build(&hero(), &[], &[], &ReasonerConfig::default()).unwrap();
        assert!(default_build.rationale.contains("keine Quelle"));
    }

    #[test]
    fn keeps_core_first_and_emits_reference_situation_order() {
        let scored = vec![
            item(1, "Core Gun", 10.0, false, &[]),
            item(2, "Metal Skin", 9.0, true, &["bullet_resist"]),
            item(3, "Spirit Shielding", 8.0, false, &["shield"]),
            item(4, "Slowing Hex", 7.0, true, &[]),
        ];
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(2, 1)], 0),
        )
        .unwrap();
        assert_eq!(
            build
                .core
                .iter()
                .map(|item| item.name.as_str())
                .collect::<Vec<_>>(),
            vec!["Core Gun"]
        );
        assert_eq!(
            build
                .situations
                .iter()
                .map(|block| block.label.as_str())
                .collect::<Vec<_>>(),
            vec!["Ein Item nach Bedarf", "Für schwere Gegner", "Schutz"]
        );
        assert_eq!(
            build.situations[0].items[0].sources[0].kind,
            EvidenceKind::Mechanic
        );
    }

    #[test]
    fn fills_each_cost_band_with_the_band_specific_score() {
        let mut cheap_mechanic = item(1, "Cheap Mechanic", 100.0, false, &[]);
        cheap_mechanic.item.tier = 1;
        cheap_mechanic.score.per_soul_value = 1.0;
        let mut cheap_lane = item(2, "Cheap Lane", 10.0, false, &[]);
        cheap_lane.item.tier = 1;
        cheap_lane.score.per_soul_value = 2.0;
        let mut expensive_low = item(3, "Expensive Low", 20.0, false, &[]);
        expensive_low.item.tier = 3;
        let mut expensive_high = item(4, "Expensive High", 30.0, false, &[]);
        expensive_high.item.tier = 3;
        let build = compose_build_with_layout(
            &hero(),
            &[cheap_mechanic, cheap_lane, expensive_low, expensive_high],
            &[],
            &ReasonerConfig::default(),
            &layout(&[(1, 1), (3, 1)], 0),
        )
        .unwrap();
        let names = build
            .core
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["Cheap Mechanic", "Expensive High"]);
    }

    #[test]
    fn preserves_funded_acquisition_order_instead_of_resorting_by_item_phase() {
        let mut items = [
            (1, "Core", BuyPhase::Core),
            (2, "Lane", BuyPhase::Lane),
            (3, "Late", BuyPhase::Late),
            (4, "Mid", BuyPhase::Mid),
        ]
        .into_iter()
        .map(|(id, name, phase)| {
            let mut item = item(id, name, id as f64, false, &[]);
            item.item.tier = match phase {
                BuyPhase::Lane => 1,
                BuyPhase::Mid => 2,
                BuyPhase::Core => 3,
                BuyPhase::Late => 4,
            };
            item.buy_phase = phase;
            item
        })
        .collect::<Vec<_>>();
        let build = compose_build_with_layout(
            &hero(),
            &items,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(1, 1), (2, 1), (3, 1), (4, 1)], 0),
        )
        .unwrap();
        let plan = plan_core(
            &hero(),
            &items,
            &ReasonerConfig::default(),
            &[],
            (
                &layout(&[(1, 1), (2, 1), (3, 1), (4, 1)], 0),
                &Default::default(),
                &AuthorEvidence::default(),
            ),
        )
        .unwrap();
        assert_eq!(
            build
                .core
                .iter()
                .map(|item| item.item_id)
                .collect::<Vec<_>>(),
            plan.steps
                .iter()
                .map(|step| step.transition.purchased_id)
                .collect::<Vec<_>>()
        );
        assert!(plan
            .steps
            .windows(2)
            .all(|pair| pair[0].progression.earned_souls <= pair[1].progression.earned_souls));
        let payload = crate::publish::to_publish_payload(&build);
        assert_eq!(
            payload.mod_categories[0]
                .mods
                .iter()
                .map(|item| item.ability_id)
                .collect::<Vec<_>>(),
            build
                .core
                .iter()
                .map(|item| item.item_id)
                .collect::<Vec<_>>()
        );
        items.clear();
    }

    #[test]
    fn historical_flex_does_not_limit_universal_slots() {
        let mut scored = (1..=7)
            .map(|id| item(id, &format!("Weapon {id}"), id as f64, false, &[]))
            .collect::<Vec<_>>();
        let mut other_band = item(8, "Other band", 100.0, false, &[]);
        other_band.item.tier = 3;
        other_band.item.slot = SlotType::Spirit;
        scored.push(other_band);
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(2, 7)], 1),
        )
        .unwrap();
        assert_eq!(build.core.len(), 7);
        assert!(build.core.iter().all(|item| item.tier == 2));
        let optional = build
            .situations
            .iter()
            .find(|block| block.kind == SituationKind::Optional)
            .unwrap();
        assert_eq!(
            optional
                .items
                .iter()
                .map(|item| item.item_id)
                .collect::<std::collections::BTreeSet<_>>(),
            [8].into_iter().collect()
        );
    }

    #[test]
    fn enforces_twelve_universal_slots_even_when_layout_requests_more() {
        let scored = (1..=15)
            .map(|id| item(id, &format!("Weapon {id}"), id as f64, false, &[]))
            .collect::<Vec<_>>();
        let build = compose_build_with_layout(
            &hero(),
            &scored,
            &[],
            &ReasonerConfig::default(),
            &layout(&[(2, 15)], 2),
        )
        .unwrap();
        let plan = plan_core(
            &hero(),
            &scored,
            &ReasonerConfig::default(),
            &[],
            (
                &layout(&[(2, 15)], 2),
                &Default::default(),
                &AuthorEvidence::default(),
            ),
        )
        .unwrap();
        assert!(plan
            .steps
            .iter()
            .all(|step| step.transition.after.held_ids.len() <= 12));
        assert_eq!(
            plan.steps.last().unwrap().transition.after.held_ids.len(),
            12
        );
        assert_eq!(
            build
                .core
                .iter()
                .map(|item| item.item_id)
                .collect::<Vec<_>>(),
            plan.steps
                .iter()
                .map(|step| step.transition.purchased_id)
                .collect::<Vec<_>>()
        );
        assert!(build
            .situations
            .iter()
            .flat_map(|block| block.items.iter())
            .any(|item| item.item_id == 1));
    }
}
