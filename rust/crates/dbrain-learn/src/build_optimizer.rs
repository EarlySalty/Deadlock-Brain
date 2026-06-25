use std::collections::{BTreeMap, BTreeSet, HashMap};

use rusqlite::{params_from_iter, Connection};
use serde_json::{json, Map, Value};

use crate::{
    util::{
        as_array, as_object, bool_value, clean_html_text, clamp_usize,
        decode_json_fields, dedupe_strings, get, get_any, get_any_string, get_string, int_or_none,
        int_or_zero, json_loads, numeric_value, query_json_rows, query_one_json,
        sorted_counts, statlocker_key, statlocker_hero_name, table_exists,
        value_to_non_empty_string, value_to_string,
    },
    LearnError, Result,
};

const PUBLIC_ITEM_SLOTS: &[&str] = &["weapon", "vitality", "spirit"];
const SHOP_BONUS_SPIKE: i64 = 4800;
const SHOP_BONUS_CAP: i64 = 28800;
const UNKNOWN_COST_SENTINEL: i64 = 9999;
const SHOP_EARLY_ECONOMY_SCORE: f64 = 2.0;
const SHOP_CORE_WINDOW_SCORE: f64 = 1.0;
const SHOP_LANE_FARM_SCORE: f64 = 3.0;
const SHOP_LANE_TRADE_SCORE: f64 = 2.0;
const LEARNING_CORE_ITEM_BONUS: f64 = 68.0;
const LEARNING_CORE_ITEM_COUNT_BONUS: f64 = 7.0;
const LEARNING_AVOID_ITEM_MALUS: f64 = 34.0;
const LEARNED_BUILD_FREQUENCY_BONUS: f64 = 26.0;
const LEARNED_BUILD_FREQUENCY_THRESHOLD: f64 = 0.67;
const SPIRIT_DOMINANT_WEAPON_CORE_MALUS: f64 = 120.0;
const PASSIVE_ECONOMY_GENERALIST_MALUS: f64 = 42.0;

const SITUATIONAL_ITEMS: &[&str] = &[
    "Healbane",
    "Healing Booster",
    "Debuff Reducer",
    "Dispel Magic",
    "Reactive Barrier",
    "Metal Skin",
    "Return Fire",
    "Knockdown",
    "Slowing Hex",
    "Silence Wave",
    "Curse",
    "Cursed Relic",
    "Indomitable",
    "Unstoppable",
    "Spellbreaker",
    "Counterspell",
];

const LATE_ITEMS: &[&str] = &[
    "Boundless Spirit",
    "Phantom Strike",
    "Magic Carpet",
    "Arctic Blast",
    "Colossus",
    "Leech",
    "Diviner's Kevlar",
];

const ENGAGE_ACTIVE_HINTS: &[&str] = &[
    "teleport", "dash", "silence", "stun", "slow", "disarm", "root", "knock", "hex",
];

const PURE_DEFENSE_HINTS: &[&str] = &["bonus health", "resist", "armor", "regen"];

const SPIRIT_DOMINANT_WEAPON_CORE_ITEMS: &[&str] = &[
    "Active Reload",
    "Burst Fire",
    "Glass Cannon",
    "Crushing Fists",
    "Kinetic Dash",
];

const SPIRIT_AFFINITY_ITEMS: &[&str] = &[
    "Extra Spirit",
    "Improved Spirit",
    "Boundless Spirit",
    "Spirit Lifesteal",
    "Duration Extender",
    "Superior Duration",
    "Mystic Vulnerability",
    "Escalating Exposure",
    "Spirit Shredder Bullets",
    "Spirit Rend",
    "Spirit Resilience",
    "Spirit Shielding",
    "Spirit Burn",
    "Spirit Sap",
    "Mystic Burst",
    "Mystic Expansion",
    "Mystic Slow",
    "Extra Charge",
    "Rapid Recharge",
    "Compress Cooldown",
    "Superior Cooldown",
    "Transcendent Cooldown",
    "Surge of Power",
    "Tankbuster",
    "Enchanter's Emblem",
    "Toxic Bullets",
];

const WEAPON_AFFINITY_ITEMS: &[&str] = &[
    "Active Reload",
    "Burst Fire",
    "Kinetic Dash",
    "Glass Cannon",
    "Crushing Fists",
    "Extended Magazine",
    "Titanic Magazine",
    "Rapid Rounds",
    "Swift Striker",
    "Intensifying Magazine",
    "Lucky Shot",
    "Sharpshooter",
    "Long Range",
    "Ricochet",
    "Spiritual Overflow",
    "Quicksilver Reload",
];

const COOLDOWN_AFFINITY_ITEMS: &[&str] = &[
    "Compress Cooldown",
    "Superior Cooldown",
    "Transcendent Cooldown",
    "Rapid Recharge",
    "Recharging Rush",
];

const DURATION_AFFINITY_ITEMS: &[&str] = &["Duration Extender", "Superior Duration"];

#[derive(Debug, Clone, Default)]
struct BuildLearningSignals {
    core_items: BTreeMap<String, ItemLearningSignal>,
    avoid_items: BTreeMap<String, ItemLearningSignal>,
    rows_used: usize,
}

#[derive(Debug, Clone, Default)]
struct ItemLearningSignal {
    display_name: String,
    count: i64,
    weight: f64,
}

#[derive(Debug, Clone, Default)]
struct LearnedBuildItemProfile {
    total_builds: i64,
    item_counts: BTreeMap<String, LearnedItemFrequency>,
    spirit_item_hits: i64,
    weapon_item_hits: i64,
    cooldown_item_hits: i64,
    duration_item_hits: i64,
}

#[derive(Debug, Clone, Default)]
struct LearnedItemFrequency {
    count: i64,
}

struct ScoreSupport<'a> {
    review_context: &'a Value,
    hero_needs: &'a Value,
    statlocker_signals: &'a BTreeMap<String, Value>,
    learning_signals: &'a BuildLearningSignals,
    learned_item_profile: &'a LearnedBuildItemProfile,
    vs_heroes: &'a [String],
}

#[derive(Debug, Clone)]
pub struct BuildSuggestOptions {
    pub query: String,
    pub vs_heroes: Vec<String>,
    pub limit_events: usize,
}

impl BuildSuggestOptions {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            vs_heroes: Vec::new(),
            limit_events: 80,
        }
    }
}

pub fn build_suggest(conn: &Connection, options: BuildSuggestOptions) -> Result<Value> {
    build_hero_build_context(
        conn,
        &options.query,
        &options.vs_heroes,
        options.limit_events,
    )
}

pub fn build_hero_build_context(
    conn: &Connection,
    query: &str,
    vs_heroes: &[String],
    limit_events: usize,
) -> Result<Value> {
    let review_context = build_review_context(conn, query, limit_events)?;
    let entity = get(&review_context, "entity_summary")
        .and_then(as_object)
        .cloned()
        .unwrap_or_default();
    if entity.get("entity_type").and_then(Value::as_str) != Some("hero") {
        let hit = entity
            .get("entity_type")
            .and_then(Value::as_str)
            .unwrap_or("none");
        return Err(LearnError::InvalidInput(format!(
            "Builds sind aktuell nur fuer Heroes gedacht. Treffer: {hit}"
        )));
    }

    let hero_name = entity
        .get("name")
        .and_then(value_to_non_empty_string)
        .unwrap_or_else(|| query.to_string());
    let Some(hero_payload) = load_entity_payload(conn, &hero_name, "hero")? else {
        return Err(LearnError::InvalidInput(format!(
            "Kein Hero-Payload fuer {query} gefunden."
        )));
    };
    let abilities = load_hero_abilities(conn, &hero_payload)?;
    let learning_signals = load_build_learning_signals(conn, &hero_name)?;
    let learned_item_profile = load_learned_build_item_profile(conn, &hero_name)?;
    let mut hero_needs = infer_hero_needs(&hero_payload, &abilities);
    apply_learned_item_profile(&mut hero_needs, &learned_item_profile);
    let wiki_summary = load_wiki_summary(conn, &hero_name)?;
    let statlocker_signals = load_statlocker_wpa_signals(conn, &hero_name)?;
    let items = load_public_items(conn)?;
    let score_support = ScoreSupport {
        review_context: &review_context,
        hero_needs: &hero_needs,
        statlocker_signals: &statlocker_signals,
        learning_signals: &learning_signals,
        learned_item_profile: &learned_item_profile,
        vs_heroes,
    };
    let mut scored_items = Vec::new();
    for item in items {
        scored_items.push(score_item(&hero_payload, &item, &score_support));
    }
    scored_items.sort_by(|left, right| {
        score_of(right)
            .partial_cmp(&score_of(left))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let build = build_recommendation(&hero_payload, &scored_items, &hero_needs);
    let timeline = get(&review_context, "timeline_signals").cloned().unwrap_or_else(|| json!({}));
    let review_signals = json!({
        "change_type_counts": get(&timeline, "change_type_counts").cloned().unwrap_or(Value::Null),
        "ability_mentions": get(&timeline, "ability_mentions").cloned().unwrap_or(Value::Null),
        "current_stat_hints": get(&review_context, "current_stat_hints")
            .and_then(|value| get(value, "hints"))
            .cloned()
            .unwrap_or(Value::Null),
        "open_questions": get(&review_context, "open_questions").cloned().unwrap_or(Value::Null),
    });
    Ok(json!({
        "query": query,
        "vs_heroes": vs_heroes,
        "hero": hero_summary(&hero_payload, &review_context, wiki_summary, &abilities, &hero_needs),
        "economy": economy_summary(&hero_payload),
        "build": build,
        "top_items": scored_items.iter().take(40).cloned().collect::<Vec<_>>(),
        "rejected_expensive_items": rejected_expensive_items(&scored_items),
        "learning_signals": compact_learning_signals(&learning_signals, &learned_item_profile),
        "statlocker_signals": compact_statlocker_signals(&statlocker_signals, 12),
        "review_signals": review_signals,
    }))
}

fn build_recommendation(
    _hero_payload: &Value,
    scored_items: &[Value],
    hero_needs: &Value,
) -> Value {
    let needs = string_set(get(hero_needs, "needs"));
    let mut early_pool: Vec<Value> = scored_items
        .iter()
        .filter(|item| {
            let cost = item_cost(item);
            cost <= 1600
                && !tag(item, "situational")
                && !tag(item, "late_only")
                && (!tag(item, "pure_defense") || needs.contains("frontline_survival"))
        })
        .cloned()
        .collect();
    early_pool.sort_by(|left, right| {
        (
            item_cost(left),
            if hero_bucket(left) == "Good" { 0 } else { 1 },
        )
            .cmp(&(
                item_cost(right),
                if hero_bucket(right) == "Good" { 0 } else { 1 },
            ))
            .then_with(|| {
                score_of(right)
                    .partial_cmp(&score_of(left))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });

    let core_pool: Vec<Value> = scored_items
        .iter()
        .filter(|item| {
            let cost = item_cost(item);
            (1600..=SHOP_BONUS_SPIKE).contains(&cost)
                && (!tag(item, "situational") || tag(item, "learning_core"))
                && !tag(item, "questionable_core")
                && (!tag(item, "pure_defense") || needs.contains("frontline_survival"))
        })
        .cloned()
        .collect();
    let late_pool: Vec<Value> = scored_items
        .iter()
        .filter(|item| {
            item_cost(item) >= 6400 && score_of(item) >= 42.0 && !tag(item, "questionable_core")
        })
        .cloned()
        .collect();
    let situational_pool: Vec<Value> = scored_items
        .iter()
        .filter(|item| tag(item, "situational") || tag(item, "counter_item") || tag(item, "pure_defense"))
        .cloned()
        .collect();

    let early = diversified_pick(&early_pool, 7, 3, &BTreeSet::new());
    let early_names = item_names_from_scored(&early);
    let core = diversified_pick(&core_pool, 8, 4, &early_names);
    let mut core_excludes = early_names.clone();
    core_excludes.extend(item_names_from_scored(&core));
    let late = diversified_pick(&late_pool, 5, 3, &core_excludes);
    let mut situational_excludes = core_excludes;
    situational_excludes.extend(item_names_from_scored(&late));
    let situational = diversified_pick(&situational_pool, 10, 5, &situational_excludes);

    json!({
        "plan": build_plan(hero_needs),
        "lane_decision_model": lane_decision_model(),
        "early": compact_recommendations(&early),
        "core": compact_recommendations(&core),
        "late": compact_recommendations(&late),
        "situational": compact_recommendations(&situational),
        "shop_routes_to_4800": shop_routes_to_4800(scored_items),
    })
}

fn score_item(
    hero_payload: &Value,
    item_payload: &Value,
    support: &ScoreSupport<'_>,
) -> Value {
    let item = item_summary(item_payload);
    let archetypes = string_set(get(&item, "archetypes"));
    let bucket = hero_item_bucket(hero_payload, item_payload);
    let mut reasons: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut tags = Map::new();
    let mut score = bucket_score(&bucket);
    if bucket != "Normal" {
        reasons.push(format!("Hero bucket: {bucket}"));
    }

    let name = get_string(&item, "name").unwrap_or_default();
    let learning_core = learning_signal_for_item(&support.learning_signals.core_items, &name).cloned();
    let learning_avoid = learning_signal_for_item(&support.learning_signals.avoid_items, &name).cloned();
    let learned_frequency = learned_item_frequency(support.learned_item_profile, &name);
    let passive_generalist_without_quality_signal = is_passive_economy_generalist(&item)
        && learned_frequency.map(|(frequency, _)| frequency < 0.34).unwrap_or(true);

    if let Some(signal) = learning_core.as_ref().filter(|_| !passive_generalist_without_quality_signal) {
        let bonus = LEARNING_CORE_ITEM_BONUS + (signal.count.saturating_sub(1) as f64 * LEARNING_CORE_ITEM_COUNT_BONUS);
        score += bonus.min(LEARNING_CORE_ITEM_BONUS + 28.0);
        tags.insert("learning_core".to_string(), json!(true));
        tags.insert("learning_core_count".to_string(), json!(signal.count));
        reasons.push("Core-Item laut Build-Analyse aehnlicher Quality-Builds".to_string());
    } else if learning_core.is_some() && passive_generalist_without_quality_signal {
        tags.insert("learning_core_suppressed".to_string(), json!(true));
    }
    if let Some(signal) = learning_avoid.as_ref() {
        score -= LEARNING_AVOID_ITEM_MALUS + (signal.count.saturating_sub(1) as f64 * 4.0).min(16.0);
        tags.insert("learning_avoid".to_string(), json!(true));
        tags.insert("questionable_core".to_string(), json!(true));
        tags.insert("learning_avoid_count".to_string(), json!(signal.count));
        warnings.push("Build-Analyse stuft dieses Item als fraglich ein".to_string());
    }
    if let Some((frequency, count)) = learned_frequency.filter(|(frequency, _)| *frequency >= LEARNED_BUILD_FREQUENCY_THRESHOLD) {
        score += LEARNED_BUILD_FREQUENCY_BONUS * frequency;
        tags.insert("learned_quality_frequency".to_string(), json!(true));
        tags.insert("learned_quality_count".to_string(), json!(count));
        reasons.push("haeufig in echten Quality-Builds dieses Heroes".to_string());
    }

    if let Some(phase_hint) = phase_hint(&name) {
        tags.insert(phase_hint.to_string(), json!(true));
        if phase_hint == "situational" {
            warnings.push("situational statt blindem Core-Kauf".to_string());
        }
    }

    let cost = known_summary_cost(&item);
    let tier = int_or_zero(get(&item, "tier"));
    let slot = get_string(&item, "slot").unwrap_or_default();
    if let Some(cost) = cost {
        if cost <= 1600 {
            score += SHOP_EARLY_ECONOMY_SCORE;
            tags.insert("early_economy".to_string(), json!(true));
            reasons.push("hilft frueh beim Shop-Bonus-Aufbau".to_string());
            if archetypes.contains("lane_farm") {
                score += SHOP_LANE_FARM_SCORE;
                reasons.push("Lane/Farm-Item: verbessert Souls oder NPC/Orb-Druck".to_string());
            }
            if archetypes.contains("lane_trade") {
                score += SHOP_LANE_TRADE_SCORE;
                reasons.push("Lane-Trade-Item: verbessert fruehe Trades oder Druck".to_string());
            }
        } else if cost <= SHOP_BONUS_SPIKE {
            score += SHOP_CORE_WINDOW_SCORE;
            tags.insert("core_window".to_string(), json!(true));
            reasons.push("liegt vor/bei dem wichtigen 4800-Shop-Bonus-Fenster".to_string());
        } else if cost >= 6400 {
            score -= 10.0;
            tags.insert("late_only".to_string(), json!(true));
            warnings.push("teuer; nur kaufen, wenn der Effekt wirklich der Gameplan ist".to_string());
        }
    }

    let needs = string_set(get(support.hero_needs, "needs"));
    if !support.vs_heroes.is_empty() && SITUATIONAL_ITEMS.contains(&name.as_str()) {
        if support.vs_heroes.iter().any(|hero| hero == "Infernus") && name == "Debuff Reducer" {
            score += 30.0;
            tags.insert("counter_item".to_string(), json!(true));
            reasons.push("ESSENTIELLER COUNTER: Reduziert Infernus' DoT stark.".to_string());
        }
        if support.vs_heroes.iter().any(|hero| hero == "Abrams") && matches!(name.as_str(), "Healbane" | "Toxic Bullets") {
            score += 30.0;
            tags.insert("counter_item".to_string(), json!(true));
            reasons.push("ESSENTIELLER COUNTER: Stoppt Abrams' Heal.".to_string());
        }
        if support.vs_heroes.iter().any(|hero| hero == "Kelvin") && name == "Unstoppable" {
            score += 20.0;
            tags.insert("counter_item".to_string(), json!(true));
            reasons.push("Guter Counter: Verhindert Kelvins Slows/Roots.".to_string());
        }
    }

    if slot == "vitality" {
        score += 6.0;
        reasons.push("Vitality-Slot gibt Health/Defense/Movement-Shop-Bonus".to_string());
        if needs.contains("frontline_survival") {
            score += 4.0;
            reasons.push("passt zu Frontline-/Channel-Risiko".to_string());
        }
    } else if slot == "spirit" {
        score += 5.0;
        reasons.push("Spirit-Slot gibt Ability-/Spirit-Power-Shop-Bonus".to_string());
    } else if slot == "weapon" {
        score += 2.0;
        reasons.push("Weapon-Slot gibt Gun-Damage-Shop-Bonus".to_string());
        if needs.contains("weapon_damage") {
            score += 8.0;
            reasons.push("passt zu erkanntem Weapon-/Gun-Fenster".to_string());
        }
    }

    let (property_score, property_reasons, property_tags) = score_properties(&item, support.hero_needs);
    score += property_score;
    reasons.extend(property_reasons.into_iter().take(4));
    tags.extend(property_tags);

    let (archetype_score, archetype_reasons, archetype_tags, archetype_warnings) =
        score_archetypes(&item, support.hero_needs);
    score += archetype_score;
    reasons.extend(archetype_reasons);
    warnings.extend(archetype_warnings);
    tags.extend(archetype_tags);

    let description = get_string(&item, "description").unwrap_or_default().to_lowercase();
    if bool_value(get(&item, "is_active")) && ENGAGE_ACTIVE_HINTS.iter().any(|hint| description.contains(hint)) {
        score += 14.0;
        tags.insert("counter_item".to_string(), json!(true));
        reasons.push("Active gibt Engage/CC/Counterplay statt nur Stats".to_string());
    }
    if bool_value(get(&item, "is_active")) && description.contains("reset the cooldown") {
        score -= 34.0;
        tags.insert("questionable_core".to_string(), json!(true));
        warnings.push("Cooldown-Reset ist teuer und braucht einen klaren Hero-spezifischen Gameplan".to_string());
    }

    if passive_generalist_without_quality_signal {
        score -= PASSIVE_ECONOMY_GENERALIST_MALUS;
        tags.insert("questionable_core".to_string(), json!(true));
        warnings.push("Economy-Generalist ohne Rueckhalt in echten Builds: nur situativ".to_string());
    }

    if name == "Refresher" {
        score -= 24.0;
        tags.insert("questionable_core".to_string(), json!(true));
        warnings.push("teurer Cooldown-Reset; nur kaufen, wenn der Hero konkret von einem zweiten vollen Ability-Zyklus gewinnt".to_string());
    }

    if is_pure_defense(&item) {
        score -= 7.0;
        tags.insert("pure_defense".to_string(), json!(true));
        warnings.push("viel Sustain/Defense, aber wenig direkter Impact".to_string());
    }

    if tier >= 4 && bucket == "Normal" {
        score -= 12.0;
        tags.insert("questionable_core".to_string(), json!(true));
        warnings.push("T4 ohne Hero-Good-Bucket braucht klare Begruendung".to_string());
    }

    let (patch_bonus, patch_reasons) = patch_synergy_bonus(&item, support.review_context);
    score += patch_bonus;
    reasons.extend(patch_reasons);

    let (need_bonus, need_reasons) = hero_need_bonus(&item, support.hero_needs);
    score += need_bonus;
    reasons.extend(need_reasons);

    if is_spirit_dominant(support.hero_needs) && is_spirit_dominant_weapon_core_item(&name) {
        score -= SPIRIT_DOMINANT_WEAPON_CORE_MALUS;
        tags.insert("questionable_core".to_string(), json!(true));
        warnings.push("Weapon-Core passt nicht zum Spirit-Profil dieses Heroes".to_string());
    }

    let (statlocker_bonus, statlocker_reasons, statlocker_tags, statlocker_warnings) =
        statlocker_wpa_bonus(&item, support.statlocker_signals);
    score += statlocker_bonus;
    reasons.extend(statlocker_reasons.clone());
    tags.extend(statlocker_tags);
    warnings.extend(statlocker_warnings);

    let mut ordered_reasons = statlocker_reasons;
    ordered_reasons.extend(reasons);
    json!({
        "item": item,
        "hero_bucket": bucket,
        "score": (score * 100.0).round() / 100.0,
        "reasons": dedupe_strings(ordered_reasons).into_iter().take(7).collect::<Vec<_>>(),
        "warnings": dedupe_strings(warnings).into_iter().take(4).collect::<Vec<_>>(),
        "tags": Value::Object(tags),
    })
}

fn score_properties(item: &Value, hero_needs: &Value) -> (f64, Vec<String>, Map<String, Value>) {
    let mut score: f64 = 0.0;
    let mut reasons = Vec::new();
    let mut tags = Map::new();
    let priority_scaling_stats = string_set(get(hero_needs, "priority_scaling_stats"));
    for prop in get(item, "properties").and_then(as_array).into_iter().flatten() {
        let label_blob = ["name", "label", "provided_property_type"]
            .iter()
            .filter_map(|key| get(prop, key))
            .map(value_to_string)
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        if let Some((reason, value)) = property_hint_score(prop, &label_blob, hero_needs) {
            score += value;
            reasons.push(reason);
        }
        let provided = get_string(prop, "provided_property_type").unwrap_or_default();
        let scales = string_set(get(prop, "scales_with"));
        let matched = priority_scaling_stats
            .iter()
            .any(|stat| stat == &provided || scales.contains(stat));
        if matched {
            score += 7.0;
            reasons.push("matched konkrete Ability-Scaling-Stats".to_string());
        }
    }
    if reasons.iter().any(|reason| reason.to_lowercase().contains("mobility")) {
        tags.insert("mobility".to_string(), json!(true));
    }
    if reasons.iter().any(|reason| reason.to_lowercase().contains("cooldown")) {
        tags.insert("cooldown".to_string(), json!(true));
    }
    (score.min(36.0), dedupe_strings(reasons), tags)
}

fn property_hint_score(prop: &Value, label_blob: &str, hero_needs: &Value) -> Option<(String, f64)> {
    if is_non_scoring_stat_property(prop) {
        return None;
    }
    let needs = string_set(get(hero_needs, "needs"));
    if is_cooldown_reduction_property(prop, label_blob) {
        return Some(("Cooldown-Reduktion: haeufigere Casts der zentralen Abilities".to_string(), if needs.contains("cooldown_reliability") { 16.0 } else { 5.0 }));
    }
    if is_spirit_resist_shred_property(prop, label_blob) {
        return Some(("Spirit-Resist-Shred verstaerkt den gesamten Spirit-Schaden".to_string(), if needs.contains("spirit_damage") { 15.0 } else { 4.0 }));
    }
    if is_spirit_amp_property(label_blob) {
        return Some(("Spirit-Amp skaliert den Spirit-Schaden hoch".to_string(), if needs.contains("spirit_damage") { 13.0 } else { 4.0 }));
    }
    if label_blob.contains("duration") {
        return Some(("Duration/value uptime".to_string(), if needs.contains("ability_uptime") { 9.0 } else { 3.0 }));
    }
    if label_blob.contains("range") {
        return Some(("Engage/ability reach".to_string(), if needs.contains("engage_reach") { 8.0 } else { 3.0 }));
    }
    if label_blob.contains("radius") {
        return Some((
            "AoE reliability".to_string(),
            if needs.contains("engage_reach") || needs.contains("ability_uptime") { 7.0 } else { 3.0 },
        ));
    }
    if label_blob.contains("techpower") || label_blob.contains("spirit power") || label_blob.contains("tech power") {
        return Some(("Spirit Power".to_string(), if needs.contains("spirit_damage") { 10.0 } else { 1.0 }));
    }
    if label_blob.contains("health") {
        return Some(("Frontline durability".to_string(), if needs.contains("frontline_survival") { 9.0 } else { 3.0 }));
    }
    if label_blob.contains("resist") || label_blob.contains("armor") {
        return Some(("Frontline durability".to_string(), if needs.contains("frontline_survival") { 8.0 } else { 3.0 }));
    }
    if label_blob.contains("move speed") {
        return Some(("Engage mobility".to_string(), if needs.contains("engage_reach") { 11.0 } else { 7.0 }));
    }
    if label_blob.contains("sprint") {
        return Some(("Map/engage mobility".to_string(), if needs.contains("engage_reach") { 9.0 } else { 5.0 }));
    }
    if label_blob.contains("stamina") {
        return Some((
            "Chase/escape economy".to_string(),
            if needs.contains("engage_reach") || needs.contains("weapon_damage") { 8.0 } else { 4.0 },
        ));
    }
    if label_blob.contains("lifesteal") {
        return Some((
            "Fight sustain".to_string(),
            if needs.contains("frontline_survival") || needs.contains("weapon_damage") { 7.0 } else { 2.0 },
        ));
    }
    if label_blob.contains("healing") {
        return Some((
            "Fight sustain".to_string(),
            if needs.contains("frontline_survival") || needs.contains("team_support") { 6.0 } else { 2.0 },
        ));
    }
    if label_blob.contains("weapon damage") {
        return Some(("Weapon scaling".to_string(), if needs.contains("weapon_damage") { 8.0 } else { 2.0 }));
    }
    if label_blob.contains("fire rate") {
        return Some(("Weapon scaling".to_string(), if needs.contains("weapon_damage") { 8.0 } else { 2.0 }));
    }
    if label_blob.contains("bullet") && !label_blob.contains("resist") && !label_blob.contains("armor") {
        return Some(("Weapon pressure".to_string(), if needs.contains("weapon_damage") { 6.0 } else { 2.0 }));
    }
    None
}

fn hero_need_bonus(item: &Value, hero_needs: &Value) -> (f64, Vec<String>) {
    let properties = get(item, "properties")
        .and_then(as_array)
        .cloned()
        .unwrap_or_default();
    let property_blob = properties
        .iter()
        .map(|prop| {
            get_any_string(prop, &["label", "name", "provided_property_type"]).unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let description = get_string(item, "description").unwrap_or_default().to_lowercase();
    let needs = string_set(get(hero_needs, "needs"));
    let mut score: f64 = 0.0;
    let mut reasons = Vec::new();
    let has_cooldown_reduction = properties.iter().any(|prop| {
        let blob = ["name", "label", "provided_property_type"]
            .iter()
            .filter_map(|key| get(prop, key))
            .map(value_to_string)
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        is_cooldown_reduction_property(prop, &blob)
    });
    if needs.contains("engage_reach")
        && ["range", "move speed", "teleport", "dash"]
            .iter()
            .any(|word| property_blob.contains(word) || description.contains(word))
    {
        score += 9.0;
        reasons.push("loest ein Hero-Reichweiten-/Engage-Problem".to_string());
    }
    if needs.contains("cooldown_reliability") && has_cooldown_reduction {
        score += 10.0;
        reasons.push("senkt zentrale Ability-Cooldowns".to_string());
    }
    if needs.contains("ability_uptime") && property_blob.contains("duration") {
        score += 6.0;
        reasons.push("verlaengert wichtige CC-/Channel-Fenster".to_string());
    }
    if needs.contains("spirit_damage")
        && ["spirit power", "techpower", "tech power"]
            .iter()
            .any(|word| property_blob.contains(word))
    {
        score += 7.0;
        reasons.push("skaliert Ability-Spirit-Damage".to_string());
    }
    if needs.contains("weapon_damage")
        && ["weapon damage", "fire rate", "bullet", "ammo", "reload"]
            .iter()
            .any(|word| property_blob.contains(word))
    {
        score += 8.0;
        reasons.push("skaliert erkannte Weapon-/Gun-Fenster".to_string());
    }
    if needs.contains("frontline_survival")
        && ["health", "resist", "armor"]
            .iter()
            .any(|word| property_blob.contains(word))
    {
        score += 5.0;
        reasons.push("hilft beim Channel-/Frontline-Risiko".to_string());
    }
    (score, reasons)
}

fn score_archetypes(item: &Value, hero_needs: &Value) -> (f64, Vec<String>, Map<String, Value>, Vec<String>) {
    let archetypes = string_set(get(item, "archetypes"));
    let needs = string_set(get(hero_needs, "needs"));
    let mut score: f64 = 0.0;
    let mut reasons = Vec::new();
    let mut tags = Map::new();
    let mut warnings = Vec::new();

    if archetypes.contains("kill_setup")
        && ["engage_reach", "cooldown_reliability", "ability_uptime"]
            .iter()
            .any(|need| needs.contains(*need))
    {
        score += 12.0;
        reasons.push("Kill-Setup: hilft zentrale CC-/Pick-Fenster zu erzwingen".to_string());
    }
    if archetypes.contains("waveclear") {
        score += 7.0;
        reasons.push("Waveclear: stabilisiert Lane/Farm und Map-Tempo".to_string());
    }
    if archetypes.contains("orb_secure") {
        score += 7.0;
        reasons.push("Orb-Secure: hilft Souls in der Lane wirklich zu sichern".to_string());
    }
    if archetypes.contains("duel") && ["weapon_damage", "frontline_survival"].iter().any(|need| needs.contains(*need)) {
        score += 8.0;
        reasons.push("Duel-Item: verbessert direkte 1v1-/Trade-Fenster".to_string());
    }
    if archetypes.contains("burst") && ["spirit_damage", "weapon_damage"].iter().any(|need| needs.contains(*need)) {
        score += 7.0;
        reasons.push("Burst: verstaerkt kurze Kill-Fenster".to_string());
    }
    if archetypes.contains("sustained_dps") && needs.contains("weapon_damage") {
        score += 8.0;
        reasons.push("Sustained DPS: passt zu wiederholten Gun-Damage-Fenstern".to_string());
    }
    if archetypes.contains("escape") {
        score += 5.0;
        reasons.push("Escape/Reposition: senkt Risiko nach Engage oder Trade".to_string());
    }
    if archetypes.contains("teamfight_engage") && ["engage_reach", "ability_uptime"].iter().any(|need| needs.contains(*need)) {
        score += 9.0;
        reasons.push("Teamfight-Engage: macht zentrale Fight-Eröffnung verlaesslicher".to_string());
    }
    if archetypes.contains("objective_damage") {
        score += 4.0;
        reasons.push("Objective-Druck: hilft Map-/Guardian-/Boss-Timings".to_string());
    }
    if archetypes.contains("core_scaling") {
        score += 8.0;
        reasons.push("Core-Scaling: verbessert wiederholbare Hero-Wincondition".to_string());
    }
    if archetypes.contains("counter") {
        warnings.push("Counter-/Defense-Anteil pruefen; als Core nur kaufen, wenn der Haupt-Effekt zum Hero-Plan passt".to_string());
    }
    if archetypes.contains("defensive_active") || archetypes.contains("defensive_proc") {
        if needs.contains("frontline_survival") || needs.contains("team_support") {
            score += 11.0;
            reasons.push("Defensiv-Active gegen Burst/CC: haelt den Hero im Fight".to_string());
        } else {
            tags.insert("situational".to_string(), json!(true));
            warnings.push("Defensiv-Item nur bei konkretem Gegnerdruck kaufen".to_string());
        }
    }
    if archetypes.contains("support") {
        if needs.contains("team_support") {
            score += 8.0;
            reasons.push("Support-Value passt zur erkannten Team-/Ally-Rolle".to_string());
        } else {
            tags.insert("situational".to_string(), json!(true));
            reasons.push("Team-/Support-Value statt reinem Solo-Scaling".to_string());
        }
    }
    if archetypes.contains("anti_heal") || archetypes.contains("anti_carry") {
        tags.insert("situational".to_string(), json!(true));
        warnings.push("Anti-Heal/Anti-Carry: nach Gegnerdraft und Problemziel kaufen".to_string());
    }
    if archetypes.contains("save") && !needs.contains("team_support") {
        tags.insert("situational".to_string(), json!(true));
        warnings.push("Save-Item: stark, aber nur mit klarer Team-/Peel-Aufgabe".to_string());
    }
    if archetypes.contains("luxury") {
        tags.insert("late_only".to_string(), json!(true));
        warnings.push("Luxury/T4: erst nach Core-Plan und Shop-Bonus-Route rechtfertigen".to_string());
    }
    if archetypes.contains("active_burden") {
        warnings.push("Active-Slot beachten; maximal vier Active Items".to_string());
    }
    (score.min(34.0), reasons, tags, warnings)
}

fn patch_synergy_bonus(item: &Value, review_context: &Value) -> (f64, Vec<String>) {
    let timeline = get(review_context, "timeline_signals").cloned().unwrap_or_else(|| json!({}));
    let ability_mentions = get(&timeline, "ability_mentions").cloned().unwrap_or_else(|| json!({}));
    let top_abilities: BTreeSet<String> = ability_mentions
        .as_object()
        .into_iter()
        .flat_map(|object| object.iter())
        .filter(|(_, count)| int_or_zero(Some(count)) >= 2)
        .map(|(name, _)| name.to_lowercase())
        .collect();
    let property_blob = get(item, "properties")
        .and_then(as_array)
        .into_iter()
        .flatten()
        .map(|prop| get_any_string(prop, &["label", "name"]).unwrap_or_default())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    if !top_abilities.is_empty()
        && ["cooldown", "duration", "range", "radius"]
            .iter()
            .any(|word| property_blob.contains(word))
    {
        (6.0, vec!["passt zu oft gepatchten/zentralen Ability-Signalen".to_string()])
    } else {
        (0.0, Vec::new())
    }
}

fn statlocker_wpa_bonus(
    item: &Value,
    statlocker_signals: &BTreeMap<String, Value>,
) -> (f64, Vec<String>, Map<String, Value>, Vec<String>) {
    let key = statlocker_key(&get_string(item, "name").unwrap_or_default());
    let Some(signal) = statlocker_signals.get(&key) else {
        return (0.0, Vec::new(), Map::new(), Vec::new());
    };
    let sample_size = int_or_zero(get_any(signal, &["sampleSize", "sample_size"]));
    if sample_size <= 0 {
        return (0.0, Vec::new(), Map::new(), Vec::new());
    }
    let wpa = numeric_value(get(signal, "wpaValue"));
    let cost_relative = get(signal, "costRelativeWpa")
        .map(|value| numeric_value(Some(value)))
        .unwrap_or(wpa);
    let value = if cost_relative != 0.0 { cost_relative } else { wpa };
    let mut score = 0.0;
    let mut reasons = Vec::new();
    let mut warnings = Vec::new();
    let mut tags = Map::new();
    tags.insert("statlocker_wpa".to_string(), json!(true));
    if value >= 0.03 {
        score += 12.0;
        tags.insert("statlocker_positive".to_string(), json!(true));
    } else if value >= 0.015 {
        score += 8.0;
        tags.insert("statlocker_positive".to_string(), json!(true));
    } else if value >= 0.005 {
        score += 4.0;
        tags.insert("statlocker_positive".to_string(), json!(true));
    } else if value <= -0.015 {
        score -= 8.0;
        tags.insert("statlocker_negative".to_string(), json!(true));
    } else if value <= -0.005 {
        score -= 4.0;
        tags.insert("statlocker_negative".to_string(), json!(true));
    }
    if score != 0.0 {
        reasons.push(format!(
            "Statlocker WPA-Signal {value:+.3} bei n={sample_size} (korrelativ, nicht causal)"
        ));
    }
    if sample_size < 100 {
        warnings.push("Statlocker-Sample fuer dieses Hero/Item ist klein".to_string());
    }
    (score, reasons, tags, warnings)
}

fn economy_summary(hero_payload: &Value) -> Value {
    let cost_bonuses = get(hero_payload, "cost_bonuses").cloned().unwrap_or_else(|| json!({}));
    let purchase_bonuses = get(hero_payload, "purchase_bonuses").cloned().unwrap_or_else(|| json!({}));
    json!({
        "wiki_rules": {
            "souls_are_currency_and_xp": true,
            "item_tiers": [800, 1600, 3200, 6400],
            "shop_bonus_categories": {
                "weapon": "Weapon Damage",
                "vitality": "Health",
                "spirit": "Spirit Power",
            },
            "significant_investment_bonus": SHOP_BONUS_SPIKE,
            "shop_bonus_cap": SHOP_BONUS_CAP,
            "component_discount": true,
            "active_item_limit": 4,
            "ability_unlock_souls": [600, 1200, 2100, 3600],
            "ability_upgrade_costs": [1, 2, 5],
            "mechanic_notes": {
                "silence": "verhindert Abilities und Active Items",
                "stun": "verhindert Bewegung, Schiessen und Actions",
                "immobilize": "rootet Bewegung, verhindert aber nicht automatisch Abilities oder Waffen",
                "resistance": "Bullet/Spirit/Melee Resist reduzieren Schaden; mehrere Quellen stacken diminishing/multiplikativ",
                "resistance_reduction": "Resist-Reduction wird separat verrechnet und kann effektive Resistenz negativ machen",
            },
        },
        "important_shop_spike": SHOP_BONUS_SPIKE,
        "cost_bonuses": {
            "weapon": get(&cost_bonuses, "weapon").cloned().unwrap_or_else(|| json!([])),
            "vitality": get(&cost_bonuses, "vitality").cloned().unwrap_or_else(|| json!([])),
            "spirit": get(&cost_bonuses, "spirit").cloned().unwrap_or_else(|| json!([])),
        },
        "purchase_bonuses": {
            "weapon": get(&purchase_bonuses, "weapon").cloned().unwrap_or_else(|| json!([])),
            "vitality": get(&purchase_bonuses, "vitality").cloned().unwrap_or_else(|| json!([])),
            "spirit": get(&purchase_bonuses, "spirit").cloned().unwrap_or_else(|| json!([])),
        },
        "interpretation": "Souls sind Geld und XP. Ein Build muss Item-Effekt, Shop-Kategorie-Bonus, Boon-/Ability-Timing und Komponenten zusammen betrachten; 4800 Souls pro Kategorie ist ein wichtiger frueher Investment-Spike.",
    })
}

fn hero_summary(
    hero_payload: &Value,
    review_context: &Value,
    wiki_summary: Option<Value>,
    abilities: &[Value],
    hero_needs: &Value,
) -> Value {
    let desc = get(hero_payload, "description").cloned().unwrap_or_else(|| json!({}));
    json!({
        "name": get(hero_payload, "name").cloned().unwrap_or(Value::Null),
        "hero_type": get(hero_payload, "hero_type").cloned().unwrap_or(Value::Null),
        "role": get(&desc, "role").cloned().unwrap_or(Value::Null),
        "playstyle": get(&desc, "playstyle").cloned().unwrap_or(Value::Null),
        "gun_tag": get(hero_payload, "gun_tag").cloned().unwrap_or(Value::Null),
        "starting_stats": compact_starting_stats(get(hero_payload, "starting_stats")),
        "level_up_stats": get(hero_payload, "standard_level_up_upgrades").cloned().unwrap_or_else(|| json!({})),
        "abilities": abilities,
        "inferred_gameplan": hero_needs,
        "wiki_summary": wiki_summary.unwrap_or(Value::Null),
        "sheet_hints": get(review_context, "current_stat_hints")
            .and_then(|value| get(value, "hints"))
            .cloned()
            .unwrap_or_else(|| json!([])),
    })
}

pub(crate) fn item_summary(payload: &Value) -> Value {
    let desc = get(payload, "description").cloned().unwrap_or_else(|| json!({}));
    let description = if desc.is_object() {
        clean_html_text(&get_string(&desc, "desc").unwrap_or_default())
    } else {
        clean_html_text(&value_to_string(&desc))
    };
    let mut item = json!({
        "name": get_any(payload, &["name", "class_name"]).cloned().unwrap_or(Value::Null),
        "class_name": get(payload, "class_name").cloned().unwrap_or(Value::Null),
        "slot": get(payload, "item_slot_type").cloned().unwrap_or(Value::Null),
        "tier": int_or_zero(get(payload, "item_tier")),
        "cost": known_payload_cost(payload).map(Value::from).unwrap_or(Value::Null),
        "is_active": bool_value(get(payload, "is_active_item")),
        "activation": get(payload, "activation").cloned().unwrap_or(Value::Null),
        "description": description,
        "component_items": get(payload, "component_items").cloned().unwrap_or_else(|| json!([])),
        "properties": compact_properties(get(payload, "properties")),
        "upgrades": get(payload, "upgrades").cloned().unwrap_or_else(|| json!([])),
    });
    let archetypes = classify_item_archetypes(&item);
    if let Some(object) = item.as_object_mut() {
        object.insert("archetypes".to_string(), json!(archetypes));
    }
    item
}

fn classify_item_archetypes(item: &Value) -> Vec<String> {
    let props = get(item, "properties").and_then(as_array).cloned().unwrap_or_default();
    let blob = [
        get_string(item, "name").unwrap_or_default(),
        get_string(item, "description").unwrap_or_default(),
        props
            .iter()
            .map(|prop| {
                ["name", "label", "provided_property_type"]
                    .iter()
                    .filter_map(|key| get(prop, key))
                    .map(value_to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join(" "),
    ]
    .join(" ")
    .to_lowercase();
    let mut archetypes = BTreeSet::new();
    if int_or_zero(get(item, "cost")) >= 6400 {
        archetypes.insert("luxury".to_string());
    }
    if bool_value(get(item, "is_active")) {
        archetypes.insert("active_burden".to_string());
    }
    add_if_any(&mut archetypes, &blob, "lane_farm", &["npc", "nonplayer", "non-player", "trooper", "bonus souls", "souls", "creep"]);
    add_if_any(&mut archetypes, &blob, "waveclear", &["npc damage", "trooper", "non-player", "nonplayer", "creep", "chain", "ricochet"]);
    add_if_any(&mut archetypes, &blob, "orb_secure", &["bonus souls", "secure", "claim", "confirm", "last hit", "orb"]);
    add_if_any(&mut archetypes, &blob, "lane_trade", &["close range", "weapon damage", "fire rate", "max ammo", "reload", "bonus damage", "current health damage", "bullet lifesteal", "out of combat regen"]);
    add_if_any(&mut archetypes, &blob, "duel", &["close range", "bullet lifesteal", "melee", "duel", "weapon damage", "fire rate", "slow resist"]);
    add_if_any(&mut archetypes, &blob, "burst", &["burst", "bonus damage", "damage amp", "amplification", "current health damage", "execute", "crit"]);
    add_if_any(&mut archetypes, &blob, "sustained_dps", &["fire rate", "weapon damage", "max ammo", "reload", "bullet procs", "ricochet", "sustained"]);
    add_if_any(&mut archetypes, &blob, "kill_setup", &["slow", "stun", "silence", "disarm", "root", "immobil", "knock", "teleport", "dash distance", "gravity"]);
    add_if_any(&mut archetypes, &blob, "escape", &["teleport", "dash", "move speed", "sprint speed", "stamina", "escape", "barrier"]);
    add_if_any(&mut archetypes, &blob, "teamfight_engage", &["stun", "silence", "disarm", "knock", "hex", "curse", "area", "nearby enemies", "radius"]);
    add_if_any(&mut archetypes, &blob, "counter", &["debuff", "cleanse", "dispel", "unstoppable", "immune", "barrier", "shield", "return fire", "healing reduction", "metal skin"]);
    if bool_value(get(item, "is_active")) {
        add_if_any(&mut archetypes, &blob, "defensive_active", &["immune", "unstoppable", "metal skin", "negative status", "status effects", "bullet resist", "debuff resist", "barrier", "shield"]);
    }
    add_if_any(&mut archetypes, &blob, "defensive_proc", &["reactive barrier", "gain a barrier", "when you are stunned", "when you are chained", "when you are immobilized", "when you are slept", "when you are silenced"]);
    add_if_any(&mut archetypes, &blob, "anti_heal", &["healing reduction", "anti-heal", "healbane"]);
    add_if_any(&mut archetypes, &blob, "anti_carry", &["disarm", "return fire", "metal skin", "bullet resist", "weapon damage reduction", "fire rate slow"]);
    add_if_any(&mut archetypes, &blob, "save", &["rescue", "barrier", "shield", "cleanse", "dispel", "heal yourself and nearby allies"]);
    add_if_any(&mut archetypes, &blob, "core_scaling", &["stack", "escalat", "amp", "cooldown", "duration", "ability range", "radius", "spirit power", "techpower", "charges", "ricochet", "bullet procs", "max weapon damage"]);
    add_if_any(&mut archetypes, &blob, "support", &["nearby allies", "friendly", "ally", "aura", "rescue", "heal yourself and nearby allies", "healing output"]);
    add_if_any(&mut archetypes, &blob, "objective_damage", &["guardian", "walker", "patron", "mid boss", "midboss", "objective", "non-player", "nonplayer", "npc damage"]);
    add_if_any(&mut archetypes, &blob, "splitpush", &["split push", "splitpush", "lane pressure", "trooper", "wave"]);
    archetypes.into_iter().collect()
}

fn compact_properties(properties: Option<&Value>) -> Value {
    let Some(object) = properties.and_then(as_object) else {
        return json!([]);
    };
    let mut rows = Vec::new();
    for (name, prop) in object {
        if !prop.is_object() {
            continue;
        }
        if is_property_sentinel(name, prop) {
            continue;
        }
        let value = get(prop, "value").cloned().unwrap_or(Value::Null);
        let disable = get(prop, "disable_value").cloned().unwrap_or(Value::Null);
        if matches!(value_to_string(&value).as_str(), "" | "0" | "0.0" | "-1.0")
            && matches!(value_to_string(&disable).as_str(), "0" | "-1" | "-2")
        {
            continue;
        }
        rows.push(json!({
            "name": name,
            "label": get_any(prop, &["label", "postvalue_label"]).cloned().unwrap_or_else(|| json!(name)),
            "value": value,
            "prefix": get(prop, "prefix").cloned().unwrap_or(Value::Null),
            "postfix": get(prop, "postfix").cloned().unwrap_or(Value::Null),
            "provided_property_type": get(prop, "provided_property_type").cloned().unwrap_or(Value::Null),
            "tooltip_section": get(prop, "tooltip_section").cloned().unwrap_or(Value::Null),
            "important": bool_value(get(prop, "tooltip_is_important")),
            "elevated": bool_value(get(prop, "tooltip_is_elevated")),
            "scales_with": scale_hint(get(prop, "scale_function")),
        }));
    }
    Value::Array(rows)
}

fn load_public_items(conn: &Connection) -> Result<Vec<Value>> {
    let rows = query_json_rows(
        conn,
        r#"
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='deadlock_assets_api'
          AND entity_type='item_or_ability'
        "#,
        &[],
    )?;
    let mut items = Vec::new();
    let mut seen = BTreeSet::new();
    for row in rows {
        let payload = json_loads(get(&row, "payload_json").and_then(Value::as_str), json!({}));
        let class_name = get_string(&payload, "class_name").unwrap_or_default();
        if class_name.is_empty() || !seen.insert(class_name) {
            continue;
        }
        if !bool_value(get(&payload, "shopable")) {
            continue;
        }
        if bool_value(get(&payload, "disabled")) {
            continue;
        }
        let slot = get_string(&payload, "item_slot_type").unwrap_or_default();
        if !PUBLIC_ITEM_SLOTS.contains(&slot.as_str()) {
            continue;
        }
        let Some(cost) = known_payload_cost(&payload) else {
            continue;
        };
        if cost <= 0 {
            continue;
        }
        items.push(payload);
    }
    Ok(items)
}

fn load_hero_abilities(conn: &Connection, hero_payload: &Value) -> Result<Vec<Value>> {
    let hero_items = get(hero_payload, "items").cloned().unwrap_or_else(|| json!({}));
    let mut ability_classes = Vec::new();
    for key in ["signature1", "signature2", "signature3", "signature4"] {
        if let Some(value) = get_string(&hero_items, key) {
            ability_classes.push(value);
        }
    }
    let mut abilities = Vec::new();
    for class_name in ability_classes {
        let pattern_spaced = format!("%\"class_name\": \"{class_name}\"%");
        let pattern_compact = format!("%\"class_name\":\"{class_name}\"%");
        let row = query_one_json(
            conn,
            r#"
            SELECT payload_json
            FROM entity_snapshots
            WHERE source='deadlock_assets_api'
              AND entity_type='item_or_ability'
              AND (payload_json LIKE ?1 OR payload_json LIKE ?2)
            LIMIT 1
            "#,
            &[&pattern_spaced, &pattern_compact],
        )?;
        if let Some(row) = row {
            let payload = json_loads(get(&row, "payload_json").and_then(Value::as_str), json!({}));
            abilities.push(ability_summary(&payload));
        }
    }
    Ok(abilities)
}

fn ability_summary(payload: &Value) -> Value {
    let props = compact_properties(get(payload, "properties"));
    let desc = get(payload, "description").cloned().unwrap_or_else(|| json!({}));
    let text = ["desc", "quip", "t1_desc", "t2_desc", "t3_desc"]
        .iter()
        .filter_map(|key| get(&desc, key))
        .map(value_to_string)
        .collect::<Vec<_>>()
        .join(" ");
    let clean_text = clean_html_text(&text);
    let mut scaling_stats: Vec<String> = ability_scaling_stats(&props).into_iter().collect();
    scaling_stats.sort();
    json!({
        "name": get(payload, "name").cloned().unwrap_or(Value::Null),
        "class_name": get(payload, "class_name").cloned().unwrap_or(Value::Null),
        "description": clean_html_text(&get_string(&desc, "desc").unwrap_or_default()),
        "role_tags": infer_ability_role_tags(&clean_text, &props),
        "key_properties": key_ability_properties(&props),
        "scaling_stats": scaling_stats,
        "damage_profile": ability_damage_profile(&props),
        "upgrades": compact_ability_upgrades(get(payload, "upgrades")),
    })
}

fn infer_hero_needs(hero_payload: &Value, abilities: &[Value]) -> Value {
    let role = get_string(hero_payload, "hero_type").unwrap_or_default().to_lowercase();
    let gun_tag = get_string(hero_payload, "gun_tag").unwrap_or_default().to_lowercase();
    let mut ability_tags_set = BTreeSet::new();
    let mut scaling_stats_set = BTreeSet::new();
    for ability in abilities {
        ability_tags_set.extend(string_set(get(ability, "role_tags")));
        scaling_stats_set.extend(string_set(get(ability, "scaling_stats")));
    }
    let ability_tags: Vec<String> = ability_tags_set.into_iter().collect();
    let scaling_stats: Vec<String> = scaling_stats_set.into_iter().collect();
    let damage_profile = hero_damage_profile(abilities, &ability_tags, &role, &gun_tag);
    let mut needs = BTreeSet::new();
    if role.contains("brawler") {
        needs.insert("frontline_survival".to_string());
        needs.insert("engage_reach".to_string());
    }
    if ability_tags.iter().any(|tag| {
        matches!(
            tag.as_str(),
            "engage" | "pick" | "disarm" | "knockup" | "stun" | "silence" | "immobilize" | "sleep" | "tether"
        )
    }) {
        needs.insert("engage_reach".to_string());
        needs.insert("cooldown_reliability".to_string());
    }
    if ability_tags.iter().any(|tag| {
        matches!(
            tag.as_str(),
            "channel" | "stun" | "disarm" | "slow" | "silence" | "immobilize" | "sleep" | "tether"
        )
    }) {
        needs.insert("ability_uptime".to_string());
    }
    let damage_plan = get_string(&damage_profile, "damage_plan").unwrap_or_default();
    if matches!(damage_plan.as_str(), "spirit" | "hybrid") {
        needs.insert("spirit_damage".to_string());
    }
    if matches!(damage_plan.as_str(), "weapon" | "hybrid") {
        needs.insert("weapon_damage".to_string());
    }
    if ability_tags.iter().any(|tag| matches!(tag.as_str(), "self_heal" | "health_stacking")) {
        needs.insert("frontline_survival".to_string());
    }
    if ability_tags.iter().any(|tag| tag == "support_team") {
        needs.insert("team_support".to_string());
    }
    let needs_vec: Vec<String> = needs.into_iter().collect();
    json!({
        "summary": gameplan_summary(hero_payload, &ability_tags, &damage_profile),
        "needs": needs_vec,
        "ability_role_tags": ability_tags,
        "scaling_stats": scaling_stats,
        "priority_scaling_stats": priority_scaling_stats(&needs_vec, &damage_profile),
        "damage_profile": damage_profile,
        "lane_priorities": lane_priorities(),
        "build_implications": build_implications(&needs_vec, &scaling_stats, &damage_profile),
        "decision_framework": hero_decision_framework(),
    })
}

fn infer_ability_role_tags(text: &str, props: &Value) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut tags = BTreeSet::new();
    let checks: &[(&str, &[&str])] = &[
        ("self_heal", &["heal yourself", "heal", "lifesteal"]),
        ("engage", &["burrow", "moving faster", "teleport", "jump out"]),
        ("knockup", &["knockup", "knock up"]),
        ("disarm", &["disarm"]),
        ("silence", &["silence", "silenced"]),
        ("stun", &["stun", "stunning", "stunned"]),
        ("immobilize", &["immobilize", "immobilized", "root", "rooted"]),
        ("sleep", &["sleep", "drowsy"]),
        ("tether", &["tether", "binds", "binding"]),
        ("bleed", &["bleed"]),
        ("burn", &["burn", "burning"]),
        ("slow", &["slow", "dash distance"]),
        ("pick", &["hold the target", "stunning", "stun", "sleep", "immobilized"]),
        ("health_stacking", &["permanently gain max health", "bonus max health"]),
        ("channel", &["channel"]),
        ("damage_amp", &["damage to them", "+15% damage"]),
        ("support_team", &["ally", "allies", "friendly", "teammate", "nearby allies", "heal an ally", "targeted ally"]),
        ("weapon_scaling", &["weapon damage", "bullet damage", "fire rate", "ammo", "reload", "headshot"]),
        ("mobility", &["dash", "leap", "fly", "jump", "sprint", "move speed"]),
        ("burst", &["burst", "explode", "detonate", "nuke"]),
    ];
    for (tag, needles) in checks {
        if needles.iter().any(|needle| lower.contains(needle)) {
            tags.insert((*tag).to_string());
        }
    }
    for prop in props.as_array().into_iter().flatten() {
        let label = get_any_string(prop, &["label", "name"]).unwrap_or_default().to_lowercase();
        if label.contains("cooldown") && !is_non_scoring_stat_property(prop) {
            tags.insert("cooldown_bound".to_string());
        }
        if label.contains("cast range") || label == "radius" {
            tags.insert("range_sensitive".to_string());
        }
        if meaningful_property_value(prop)
            && (["weapon damage", "fire rate", "ammo", "reload", "headshot"]
                .iter()
                .any(|word| label.contains(word))
                || (label.contains("bullet") && !label.contains("resist") && !label.contains("armor")))
        {
            tags.insert("weapon_scaling".to_string());
        }
    }
    tags.into_iter().collect()
}

fn key_ability_properties(props: &Value) -> Value {
    let important_names: BTreeSet<&str> = [
        "AbilityCooldown",
        "AbilityCastRange",
        "AbilityChannelTime",
        "AbilityDuration",
        "Radius",
        "Damage",
        "DPS",
        "BonusMoveSpeed",
        "BulletResist",
        "TechResist",
        "DamageHealMult",
        "DamageHealMultNonHero",
        "BonusHealthOnKill",
        "SlowPercent",
        "GroundDashReductionPercent",
    ]
    .into_iter()
    .collect();
    Value::Array(
        props
            .as_array()
            .into_iter()
            .flatten()
            .filter(|prop| get_string(prop, "name").map(|name| important_names.contains(name.as_str())).unwrap_or(false))
            .take(14)
            .cloned()
            .collect(),
    )
}

fn ability_scaling_stats(props: &Value) -> BTreeSet<String> {
    let mut stats = BTreeSet::new();
    for prop in props.as_array().into_iter().flatten() {
        if let Some(provided) = get_string(prop, "provided_property_type") {
            stats.insert(provided);
        }
        stats.extend(string_set(get(prop, "scales_with")));
    }
    stats
}

fn ability_damage_profile(props: &Value) -> Value {
    let mut damage_sources = 0;
    let mut spirit_scaling_damage_sources = 0;
    let mut weapon_stat_sources = 0;
    let mut control_sources = 0;
    let mut survival_sources = 0;
    for prop in props.as_array().into_iter().flatten() {
        let name = get_string(prop, "name").unwrap_or_default();
        let label = get_any_string(prop, &["label", "name"]).unwrap_or_default().to_lowercase();
        let scales = string_set(get(prop, "scales_with"));
        if damage_property_name(&name) && numeric_value(get(prop, "value")) > 0.0 {
            damage_sources += 1;
            if scales.contains("ETechPower") {
                spirit_scaling_damage_sources += 1;
            }
        }
        if meaningful_property_value(prop)
            && ["weapon damage", "fire rate", "ammo", "reload", "headshot"]
            .iter()
            .any(|word| label.contains(word))
        {
            weapon_stat_sources += 1;
        }
        if matches!(name.as_str(), "SlowPercent" | "GroundDashReductionPercent")
            || ["stun", "slow", "silence", "disarm", "immobil"]
                .iter()
                .any(|word| label.contains(word))
        {
            control_sources += 1;
        }
        if ["heal", "lifesteal", "resist", "barrier", "bonus health"]
            .iter()
            .any(|word| label.contains(word))
        {
            survival_sources += 1;
        }
    }
    json!({
        "damage_sources": damage_sources,
        "spirit_scaling_damage_sources": spirit_scaling_damage_sources,
        "weapon_stat_sources": weapon_stat_sources,
        "control_sources": control_sources,
        "survival_sources": survival_sources,
    })
}

fn hero_damage_profile(abilities: &[Value], ability_tags: &[String], role: &str, gun_tag: &str) -> Value {
    let mut ability_damage_sources = 0;
    let mut spirit_scaling_damage_sources = 0;
    let mut weapon_stat_sources = 0;
    for ability in abilities {
        let profile = get(ability, "damage_profile").cloned().unwrap_or_else(|| json!({}));
        ability_damage_sources += int_or_zero(get(&profile, "damage_sources"));
        spirit_scaling_damage_sources += int_or_zero(get(&profile, "spirit_scaling_damage_sources"));
        weapon_stat_sources += int_or_zero(get(&profile, "weapon_stat_sources"));
    }
    if ability_tags.iter().any(|tag| tag == "weapon_scaling") {
        weapon_stat_sources += 1;
    }
    if gun_tag.contains("rapid") || role.contains("gun") || role.contains("assassin") {
        weapon_stat_sources += 1;
    }
    let spirit_signal = spirit_scaling_damage_sources + if ability_damage_sources >= 3 { 1 } else { 0 };
    let damage_plan = if weapon_stat_sources >= 3 && weapon_stat_sources > spirit_signal {
        "weapon"
    } else if spirit_signal >= 3 && spirit_signal > weapon_stat_sources {
        "spirit"
    } else if spirit_signal >= 2 && weapon_stat_sources >= 1 {
        "hybrid"
    } else if weapon_stat_sources >= 2 {
        "weapon"
    } else if spirit_signal >= 2 || ability_damage_sources >= 3 {
        "spirit"
    } else {
        "utility"
    };
    json!({
        "damage_plan": damage_plan,
        "ability_damage_sources": ability_damage_sources,
        "spirit_scaling_damage_sources": spirit_scaling_damage_sources,
        "weapon_stat_sources": weapon_stat_sources,
        "spirit_signal": spirit_signal,
    })
}

fn priority_scaling_stats(needs: &[String], damage_profile: &Value) -> Vec<String> {
    let needs: BTreeSet<&str> = needs.iter().map(String::as_str).collect();
    let mut stats = BTreeSet::new();
    if needs.contains("spirit_damage") {
        stats.insert("ETechPower".to_string());
    }
    if needs.contains("weapon_damage") {
        stats.extend([
            "MODIFIER_VALUE_ATTACK_DAMAGE_ADDITIVE_ONLY".to_string(),
            "MODIFIER_VALUE_BASEATTACK_DAMAGE_PERCENT".to_string(),
            "MODIFIER_VALUE_FIRE_RATE".to_string(),
            "MODIFIER_VALUE_AMMO_CLIP_SIZE_PERCENT".to_string(),
        ]);
    }
    if needs.contains("cooldown_reliability") {
        stats.insert("ETechCooldown".to_string());
    }
    if needs.contains("ability_uptime") {
        stats.insert("ETechDuration".to_string());
        stats.insert("EChannelDuration".to_string());
    }
    if needs.contains("engage_reach") {
        stats.insert("ETechRange".to_string());
        stats.insert("ETechRadius".to_string());
    }
    if get_string(damage_profile, "damage_plan").as_deref() == Some("weapon") {
        stats.remove("ETechPower");
    }
    stats.into_iter().collect()
}

fn compact_ability_upgrades(upgrades: Option<&Value>) -> Value {
    let rows = upgrades.and_then(as_array).cloned().unwrap_or_default();
    Value::Array(
        rows.iter()
            .enumerate()
            .map(|(index, upgrade)| {
                let changes = get(upgrade, "property_upgrades")
                    .and_then(as_array)
                    .into_iter()
                    .flatten()
                    .map(|change| {
                        json!({
                            "name": get(change, "name").cloned().unwrap_or(Value::Null),
                            "bonus": get(change, "bonus").cloned().unwrap_or(Value::Null),
                            "scale_stat_filter": get(change, "scale_stat_filter").cloned().unwrap_or(Value::Null),
                        })
                    })
                    .collect::<Vec<_>>();
                json!({"tier_index": index + 1, "changes": changes})
            })
            .collect(),
    )
}

fn gameplan_summary(hero_payload: &Value, ability_tags: &[String], damage_profile: &Value) -> String {
    if get_string(damage_profile, "damage_plan").as_deref() == Some("spirit") {
        return "Spirit-dominanter Plan: Ability-Schaden ueber Spirit-Skalierung, Cooldown-Reduktion und Resist-Shred maximieren".to_string();
    }
    if ability_tags.iter().any(|tag| tag == "pick") && ability_tags.iter().any(|tag| tag == "engage") {
        return "Engage-/Pick-Hero: Reichweitenfenster erzwingen, Control landen und Team-Follow-up auf ein priorisiertes Ziel ermoeglichen.".to_string();
    }
    if ability_tags.iter().any(|tag| tag == "weapon_scaling") && ability_tags.iter().any(|tag| tag == "mobility") {
        return "Mobile Weapon- oder Hybrid-Damage-Rolle: Farm/Positioning halten und Items kaufen, die Waffenfenster verlaengern oder sicherer machen.".to_string();
    }
    if ability_tags.iter().any(|tag| tag == "self_heal" || tag == "frontline_survival") {
        return "Brawler/Sustain-Rolle: Trades und laengere Fights ueberleben, aber defensive Kaeufe muessen weiter Impact erzeugen.".to_string();
    }
    get(hero_payload, "description")
        .and_then(|desc| get_string(desc, "playstyle"))
        .unwrap_or_default()
}

fn lane_priorities() -> Vec<&'static str> {
    vec![
        "Souls sind gleichzeitig Geld und XP/Boons; Lane-Items muessen Farm, Orb-Sicherung, Trades oder Kill-Setup verbessern.",
        "Fruehe 800/1600-Kaeufe sind nicht nur Item-Effekte, sondern auch Schritte zu Shop-Bonus-Schwellen pro Kategorie.",
        "Ein Lane-Kauf ist gut, wenn er ein konkretes Hero-Problem loest: Reichweite, Waveclear, Sustain, Ammo, Mobility, Cooldown, CC oder Kill-Druck.",
        "Reine Defensive ist nur dann frueh gut, wenn sie den Hero in seiner eigentlichen Aufgabe haelt; sonst verzögert sie Impact.",
    ]
}

fn build_implications(needs: &[String], scaling_stats: &[String], damage_profile: &Value) -> Vec<String> {
    let needs_set: BTreeSet<&str> = needs.iter().map(String::as_str).collect();
    let mut implications = Vec::new();
    implications.push(format!(
        "Damage-Plan: {} (ability_damage={}, spirit_scaling={}, weapon_hooks={}).",
        get_string(damage_profile, "damage_plan").unwrap_or_default(),
        int_or_zero(get(damage_profile, "ability_damage_sources")),
        int_or_zero(get(damage_profile, "spirit_scaling_damage_sources")),
        int_or_zero(get(damage_profile, "weapon_stat_sources")),
    ));
    if needs_set.contains("engage_reach") {
        implications.push("Range/Mobility ist wertvoll, wenn sie zentrale Control- oder Damage-Fenster erreichbar macht.".to_string());
    }
    if needs_set.contains("cooldown_reliability") {
        implications.push("Cooldown-Items sind nur gut, wenn sie echte zweite Engage-/Control-Fenster schaffen.".to_string());
    }
    if needs_set.contains("spirit_damage") {
        implications.push("Spirit Power hat Wert, weil mehrere Abilities mit ETechPower skalieren.".to_string());
    }
    if needs_set.contains("weapon_damage") {
        implications.push("Weapon-Kaeufe brauchen entweder starke Gun-Skalierung, Orb-/Lane-Kontrolle oder ein klares Damage-Fenster.".to_string());
    }
    if needs_set.contains("frontline_survival") {
        implications.push("Defensive Items brauchen einen Zweck: Channel ueberleben, Re-Engage schaffen oder Counter beantworten.".to_string());
    }
    if needs_set.contains("team_support") {
        implications.push("Team-/Ally-Items koennen Core sein, wenn sie die erkannte Support-Aufgabe direkt staerken.".to_string());
    }
    let _ = scaling_stats;
    implications
}

fn build_plan(hero_needs: &Value) -> &'static str {
    let needs = string_set(get(hero_needs, "needs"));
    if needs.contains("engage_reach") && needs.contains("cooldown_reliability") {
        "Erst Lane und Shop-Boni stabilisieren, dann Items kaufen, die zentrale Engage-/Control-Fenster verlaesslicher machen."
    } else if needs.contains("weapon_damage") && needs.contains("spirit_damage") {
        "Hybrid-Plan: 4800-Shop-Boni bewusst balancen und nur Items kaufen, die entweder Gun-Fenster oder Ability-Fenster klar verbessern."
    } else if needs.contains("weapon_damage") {
        "Weapon-Plan: Souls sichern, Gun-Shop-Bonus effizient aufbauen und defensive/aktive Items nur kaufen, wenn sie Damage-Fenster ermoeglichen."
    } else if needs.contains("spirit_damage") {
        "Spirit-Plan: fruehe Economy mit Spirit-Shop-Bonus verbinden, danach Cooldown/Range/Duration passend zu den wichtigsten Abilities kaufen."
    } else {
        "Generischer Plan: Lane stabilisieren, 4800-Shop-Boni bewusst planen und jedes teure Item gegen den Hero-Gameplan begruenden."
    }
}

fn lane_decision_model() -> Vec<&'static str> {
    vec![
        "1. Kann das Item Souls sichern, Trades gewinnen oder einen fruehen Shop-Bonus sinnvoll fuellen?",
        "2. Passt der Shop-Slot zum geplanten 4800-Fenster oder blockiert er bessere Core-Kaeufe?",
        "3. Loest es ein konkretes Hero-Problem: Reichweite, Cooldown, Channel-Sicherheit, Disarm/Pick-Follow-up?",
        "4. Wenn es nur Sustain/Defense gibt: nur kaufen, wenn der Gegnerdruck das erzwingt.",
    ]
}

fn hero_decision_framework() -> Vec<&'static str> {
    vec![
        "1. Hero-Aufgabe aus Abilities ableiten: poke, burst, pick, frontline, support, objective, gun-carry oder hybrid.",
        "2. Scaling lesen: welche Abilities skalieren mit Spirit Power, Weapon Damage, Duration, Range, Cooldown, Charges oder Boons?",
        "3. Fruehe Oekonomie planen: 800/1600-Items sollen Lane-Problem und Shop-Bonus-Fortschritt gleichzeitig bedienen.",
        "4. Core-Items muessen den Gameplan staerker machen, nicht nur gute generische Stats geben.",
        "5. T4/Luxury nur kaufen, wenn das Item einen konkreten Win-Condition-Schritt liefert.",
    ]
}

pub(crate) fn load_entity_payload(conn: &Connection, query: &str, entity_type: &str) -> Result<Option<Value>> {
    let wanted_snapshot_type = if entity_type == "hero" { "hero" } else { "item_or_ability" };
    let lower = query.to_lowercase();
    let like = format!("%{query}%");
    let row = query_one_json(
        conn,
        r#"
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='deadlock_assets_api'
          AND entity_type=?1
          AND (
            lower(canonical_name)=?2
            OR canonical_name LIKE ?3
            OR payload_json LIKE ?3
          )
        ORDER BY
          CASE WHEN lower(canonical_name)=?2 THEN 0 ELSE 1 END,
          fetched_at DESC,
          id DESC
        LIMIT 1
        "#,
        &[&wanted_snapshot_type, &lower, &like],
    )?;
    Ok(row
        .and_then(|item| get(&item, "payload_json").and_then(Value::as_str).map(|raw| json_loads(Some(raw), json!({}))))
        .filter(Value::is_object))
}

fn load_wiki_summary(conn: &Connection, title: &str) -> Result<Option<Value>> {
    let row = query_one_json(
        conn,
        r#"
        SELECT payload_json, fetched_at
        FROM entity_snapshots
        WHERE source='deadlock_wiki' AND external_id=?1
        ORDER BY id DESC
        LIMIT 1
        "#,
        &[&title],
    )?;
    let Some(row) = row else {
        return Ok(None);
    };
    let payload = json_loads(get(&row, "payload_json").and_then(Value::as_str), json!({}));
    let pages = get(&payload, "query")
        .and_then(|query| get(query, "pages"))
        .and_then(as_object)
        .cloned()
        .unwrap_or_default();
    let first_page = pages.values().next().cloned().unwrap_or_else(|| json!({}));
    let extract = get_string(&first_page, "extract").unwrap_or_default();
    let paragraphs: Vec<_> = extract
        .split('\n')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();
    let gameplay = paragraphs.first().copied().unwrap_or("");
    let trivia = paragraphs
        .iter()
        .find(|part| part.to_lowercase().contains("highest base health"))
        .copied()
        .unwrap_or("");
    Ok(Some(json!({
        "source": "deadlock_wiki",
        "fetched_at": get(&row, "fetched_at").cloned().unwrap_or(Value::Null),
        "gameplay_extract": gameplay.chars().take(700).collect::<String>(),
        "notable_trivia": if trivia.is_empty() { Value::Null } else { json!(trivia.chars().take(300).collect::<String>()) },
    })))
}

fn load_statlocker_wpa_signals(conn: &Connection, hero_name: &str) -> Result<BTreeMap<String, Value>> {
    let hero_key = statlocker_key(hero_name);
    let hero_web_key = statlocker_key(&statlocker_hero_name(hero_name));
    let rows = query_json_rows(
        conn,
        r#"
        SELECT payload_json, fetched_at
        FROM entity_snapshots
        WHERE source='statlocker'
          AND entity_type='statlocker_wpa_item'
        ORDER BY fetched_at DESC, id DESC
        "#,
        &[],
    )?;
    let mut signals = BTreeMap::new();
    let mut fallback = BTreeMap::new();
    for row in rows {
        let mut payload = json_loads(get(&row, "payload_json").and_then(Value::as_str), json!({}));
        let item_name = get_string(&payload, "item").unwrap_or_default();
        if item_name.is_empty() {
            continue;
        }
        let requested_hero = get(&payload, "_deadlock_brain")
            .and_then(|meta| get_string(meta, "requested_hero"))
            .or_else(|| get_string(&payload, "heroName"))
            .unwrap_or_else(|| "all".to_string());
        if let Some(object) = payload.as_object_mut() {
            object.insert(
                "fetched_at".to_string(),
                get(&row, "fetched_at").cloned().unwrap_or(Value::Null),
            );
        }
        let key = statlocker_key(&item_name);
        let requested_key = statlocker_key(&requested_hero);
        if requested_key == hero_key || requested_key == hero_web_key {
            signals.entry(key).or_insert(payload);
        } else if requested_hero == "all" {
            fallback.entry(key).or_insert(payload);
        }
    }
    if signals.is_empty() {
        Ok(fallback)
    } else {
        Ok(signals)
    }
}

fn load_build_learning_signals(conn: &Connection, hero_name: &str) -> Result<BuildLearningSignals> {
    if !table_exists(conn, "build_learning_notes")? {
        return Ok(BuildLearningSignals::default());
    }
    let rows = query_json_rows(
        conn,
        r#"
        SELECT insights_json, updated_at, created_at, id
        FROM build_learning_notes
        WHERE lower(hero_name)=lower(?1)
          AND COALESCE(insights_json, '') NOT IN ('', '{}', 'null')
        ORDER BY updated_at DESC, created_at DESC, id DESC
        LIMIT 24
        "#,
        &[&hero_name],
    )?;
    let mut signals = BuildLearningSignals::default();
    for (index, row) in rows.iter().enumerate() {
        let parsed = json_loads(get(row, "insights_json").and_then(Value::as_str), json!({}));
        let insights = get(&parsed, "insights")
            .filter(|value| value.is_object())
            .cloned()
            .unwrap_or(parsed);
        let core_items = insight_string_values(get(&insights, "core_items"));
        if core_items.is_empty() {
            continue;
        }
        signals.rows_used += 1;
        let recency_weight = 1.0 + (12_usize.saturating_sub(index.min(12)) as f64 * 0.05);
        for item in core_items {
            record_learning_signal(&mut signals.core_items, &item, recency_weight);
        }
        for item in insight_string_values(get(&insights, "avoid_or_question")) {
            record_learning_signal(&mut signals.avoid_items, &item, recency_weight);
        }
    }
    Ok(signals)
}

fn load_learned_build_item_profile(conn: &Connection, hero_name: &str) -> Result<LearnedBuildItemProfile> {
    if !table_exists(conn, "learned_builds")? {
        return Ok(LearnedBuildItemProfile::default());
    }
    let rows = query_json_rows(
        conn,
        r#"
        SELECT item_names_json, quality_tier, quality_score, source_rank, updated_at
        FROM learned_builds
        WHERE lower(hero_name)=lower(?1)
          AND quality_tier IN ('likely_good', 'usable_noisy')
        ORDER BY quality_score DESC, source_rank ASC, updated_at DESC, id ASC
        LIMIT 12
        "#,
        &[&hero_name],
    )?;
    let mut profile = LearnedBuildItemProfile::default();
    for row in rows {
        let items = json_loads(get(&row, "item_names_json").and_then(Value::as_str), json!([]));
        let mut seen = BTreeSet::new();
        for item in insight_string_values(Some(&items)) {
            let candidate = learning_item_candidate_name(&item);
            let key = normalized_item_key(&candidate);
            if key.is_empty() || !seen.insert(key.clone()) {
                continue;
            }
            let entry = profile.item_counts.entry(key).or_default();
            entry.count += 1;
            if item_name_in(&candidate, SPIRIT_AFFINITY_ITEMS) {
                profile.spirit_item_hits += 1;
            }
            if item_name_in(&candidate, WEAPON_AFFINITY_ITEMS) {
                profile.weapon_item_hits += 1;
            }
            if item_name_in(&candidate, COOLDOWN_AFFINITY_ITEMS) {
                profile.cooldown_item_hits += 1;
            }
            if item_name_in(&candidate, DURATION_AFFINITY_ITEMS) {
                profile.duration_item_hits += 1;
            }
        }
        profile.total_builds += 1;
    }
    Ok(profile)
}

fn apply_learned_item_profile(hero_needs: &mut Value, profile: &LearnedBuildItemProfile) {
    if profile.total_builds <= 0 {
        return;
    }
    let spirit_dominant = learned_profile_spirit_dominant(profile);
    let weapon_dominant = learned_profile_weapon_dominant(profile);
    if spirit_dominant {
        add_need(hero_needs, "spirit_damage");
        if !weapon_dominant {
            remove_need(hero_needs, "weapon_damage");
        }
    }
    if weapon_dominant {
        add_need(hero_needs, "weapon_damage");
    }
    if profile.cooldown_item_hits >= profile.total_builds {
        add_need(hero_needs, "cooldown_reliability");
    }
    if profile.duration_item_hits >= profile.total_builds {
        add_need(hero_needs, "ability_uptime");
    }
    if let Some(object) = hero_needs.as_object_mut() {
        if let Some(damage_profile) = object.get_mut("damage_profile").and_then(Value::as_object_mut) {
            damage_profile.insert("learned_spirit_item_hits".to_string(), json!(profile.spirit_item_hits));
            damage_profile.insert("learned_weapon_item_hits".to_string(), json!(profile.weapon_item_hits));
            damage_profile.insert("learned_spirit_dominant".to_string(), json!(spirit_dominant));
            damage_profile.insert("learned_weapon_dominant".to_string(), json!(weapon_dominant));
            if spirit_dominant && !weapon_dominant {
                damage_profile.insert("damage_plan".to_string(), json!("spirit"));
                let current = int_or_zero(damage_profile.get("spirit_scaling_damage_sources"));
                damage_profile.insert("spirit_scaling_damage_sources".to_string(), json!(current.max(3)));
            } else if weapon_dominant && !spirit_dominant {
                damage_profile.insert("damage_plan".to_string(), json!("weapon"));
            } else if spirit_dominant && weapon_dominant {
                damage_profile.insert("damage_plan".to_string(), json!("hybrid"));
            }
        }
        if spirit_dominant && !weapon_dominant {
            object.insert(
                "summary".to_string(),
                json!("Spirit-dominanter Plan: Ability-Schaden ueber Spirit-Skalierung, Cooldown-Reduktion und Resist-Shred maximieren"),
            );
        }
    }
    refresh_priority_scaling_stats(hero_needs);
}

fn compact_learning_signals(
    learning_signals: &BuildLearningSignals,
    learned_item_profile: &LearnedBuildItemProfile,
) -> Value {
    json!({
        "insight_rows_used": learning_signals.rows_used,
        "core_items": compact_learning_map(&learning_signals.core_items, 16),
        "avoid_items": compact_learning_map(&learning_signals.avoid_items, 12),
        "learned_builds_used": learned_item_profile.total_builds,
        "learned_spirit_item_hits": learned_item_profile.spirit_item_hits,
        "learned_weapon_item_hits": learned_item_profile.weapon_item_hits,
    })
}

fn compact_learning_map(values: &BTreeMap<String, ItemLearningSignal>, limit: usize) -> Value {
    let mut rows = values.values().cloned().collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| right.weight.partial_cmp(&left.weight).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| left.display_name.cmp(&right.display_name))
    });
    Value::Array(
        rows.into_iter()
            .take(limit)
            .map(|row| json!({"name": row.display_name, "count": row.count, "weight": (row.weight * 100.0).round() / 100.0}))
            .collect(),
    )
}

fn insight_string_values(value: Option<&Value>) -> Vec<String> {
    match value {
        Some(Value::String(text)) => vec![text.trim().to_string()],
        Some(Value::Array(items)) => items.iter().flat_map(|item| insight_string_values(Some(item))).collect(),
        Some(Value::Object(object)) => object.values().flat_map(|item| insight_string_values(Some(item))).collect(),
        Some(Value::Number(_)) | Some(Value::Bool(_)) | Some(Value::Null) | None => Vec::new(),
    }
    .into_iter()
    .filter(|item| !item.trim().is_empty())
    .collect()
}

fn record_learning_signal(values: &mut BTreeMap<String, ItemLearningSignal>, raw_name: &str, weight: f64) {
    let display_name = learning_item_candidate_name(raw_name);
    let key = normalized_item_key(&display_name);
    if key.is_empty() {
        return;
    }
    let entry = values.entry(key).or_insert_with(|| ItemLearningSignal {
        display_name: display_name.clone(),
        count: 0,
        weight: 0.0,
    });
    entry.count += 1;
    entry.weight += weight;
}

fn learning_item_candidate_name(raw_name: &str) -> String {
    let mut candidate = raw_name
        .trim()
        .trim_matches(|character: char| {
            matches!(character, '-' | '*' | '"' | '\'' | '`' | '[' | ']' | '{' | '}')
        })
        .trim()
        .to_string();
    for delimiter in [" (", " - ", " – ", " — ", ":", ";", " + ", ","] {
        if let Some(index) = candidate.find(delimiter) {
            candidate.truncate(index);
        }
    }
    candidate.trim().to_string()
}

fn normalized_item_key(value: &str) -> String {
    statlocker_key(value)
}

fn learning_signal_for_item<'a>(
    values: &'a BTreeMap<String, ItemLearningSignal>,
    item_name: &str,
) -> Option<&'a ItemLearningSignal> {
    values.get(&normalized_item_key(item_name))
}

fn learned_item_frequency(profile: &LearnedBuildItemProfile, item_name: &str) -> Option<(f64, i64)> {
    if profile.total_builds <= 0 {
        return None;
    }
    profile
        .item_counts
        .get(&normalized_item_key(item_name))
        .map(|entry| (entry.count as f64 / profile.total_builds as f64, entry.count))
}

fn learned_profile_spirit_dominant(profile: &LearnedBuildItemProfile) -> bool {
    profile.total_builds > 0
        && profile.spirit_item_hits >= profile.total_builds * 3
        && profile.spirit_item_hits >= profile.weapon_item_hits
}

fn learned_profile_weapon_dominant(profile: &LearnedBuildItemProfile) -> bool {
    profile.total_builds > 0
        && profile.weapon_item_hits >= profile.total_builds * 3
        && profile.weapon_item_hits > profile.spirit_item_hits
}

fn item_name_in(name: &str, values: &[&str]) -> bool {
    let key = normalized_item_key(name);
    values.iter().any(|value| normalized_item_key(value) == key)
}

fn add_need(hero_needs: &mut Value, need: &str) {
    let Some(object) = hero_needs.as_object_mut() else {
        return;
    };
    let needs = object.entry("needs".to_string()).or_insert_with(|| json!([]));
    let Some(items) = needs.as_array_mut() else {
        return;
    };
    if !items.iter().any(|item| item.as_str() == Some(need)) {
        items.push(json!(need));
    }
    items.sort_by_key(value_to_string);
}

fn remove_need(hero_needs: &mut Value, need: &str) {
    let Some(items) = hero_needs
        .as_object_mut()
        .and_then(|object| object.get_mut("needs"))
        .and_then(Value::as_array_mut)
    else {
        return;
    };
    items.retain(|item| item.as_str() != Some(need));
}

fn refresh_priority_scaling_stats(hero_needs: &mut Value) {
    let needs = get(hero_needs, "needs")
        .and_then(as_array)
        .into_iter()
        .flatten()
        .filter_map(value_to_non_empty_string)
        .collect::<Vec<_>>();
    let damage_profile = get(hero_needs, "damage_profile").cloned().unwrap_or_else(|| json!({}));
    let priority = priority_scaling_stats(&needs, &damage_profile);
    if let Some(object) = hero_needs.as_object_mut() {
        object.insert("priority_scaling_stats".to_string(), json!(priority));
    }
}

fn compact_statlocker_signals(signals: &BTreeMap<String, Value>, limit: usize) -> Value {
    let mut rows = signals
        .values()
        .map(|signal| {
            json!({
                "item": get(signal, "item").cloned().unwrap_or(Value::Null),
                "hero_name": get(signal, "heroName").cloned().unwrap_or(Value::Null),
                "wpa": get(signal, "wpaValue").cloned().unwrap_or(Value::Null),
                "cost_relative_wpa": get(signal, "costRelativeWpa").cloned().unwrap_or(Value::Null),
                "sample_size": get(signal, "sampleSize").cloned().unwrap_or(Value::Null),
                "mean_purchase_time_min": get(signal, "mean_purchase_time_min").cloned().unwrap_or(Value::Null),
            })
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        let left_value = get(left, "cost_relative_wpa")
            .filter(|value| !value.is_null())
            .or_else(|| get(left, "wpa"));
        let right_value = get(right, "cost_relative_wpa")
            .filter(|value| !value.is_null())
            .or_else(|| get(right, "wpa"));
        numeric_value(right_value)
            .partial_cmp(&numeric_value(left_value))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Value::Array(rows.into_iter().take(limit).collect())
}

fn hero_item_bucket(hero_payload: &Value, item_payload: &Value) -> String {
    let buckets = get(hero_payload, "item_draft_bucketing").cloned().unwrap_or_else(|| json!({}));
    let class_name = get_string(item_payload, "class_name").unwrap_or_default();
    get(&buckets, &class_name)
        .and_then(|meta| get_string(meta, "bucket"))
        .unwrap_or_else(|| "Normal".to_string())
}

fn diversified_pick(
    items: &[Value],
    max_items: usize,
    max_per_slot: usize,
    exclude_names: &BTreeSet<String>,
) -> Vec<Value> {
    let mut picked = Vec::new();
    let mut by_slot: HashMap<String, usize> = HashMap::new();
    for item in items {
        let name = scored_item_name(item);
        let slot = scored_item_slot(item);
        if exclude_names.contains(&name) || *by_slot.get(&slot).unwrap_or(&0) >= max_per_slot {
            continue;
        }
        picked.push(item.clone());
        *by_slot.entry(slot).or_default() += 1;
        if picked.len() >= max_items {
            break;
        }
    }
    picked
}

fn phase_hint(name: &str) -> Option<&'static str> {
    if SITUATIONAL_ITEMS.contains(&name) {
        Some("situational")
    } else if LATE_ITEMS.contains(&name) {
        Some("late")
    } else {
        None
    }
}

fn compact_recommendations(items: &[Value]) -> Value {
    Value::Array(
        items
            .iter()
            .map(|item| {
                let inner = get(item, "item").cloned().unwrap_or_else(|| json!({}));
                json!({
                    "name": get(&inner, "name").cloned().unwrap_or(Value::Null),
                    "slot": get(&inner, "slot").cloned().unwrap_or(Value::Null),
                    "tier": get(&inner, "tier").cloned().unwrap_or(Value::Null),
                    "cost": get(&inner, "cost").cloned().unwrap_or(Value::Null),
                    "archetypes": get(&inner, "archetypes").cloned().unwrap_or_else(|| json!([])),
                    "score": get(item, "score").cloned().unwrap_or(Value::Null),
                    "why": get(item, "reasons")
                        .and_then(as_array)
                        .map(|rows| rows.iter().take(3).cloned().collect::<Vec<_>>())
                        .unwrap_or_default(),
                    "warnings": get(item, "warnings").cloned().unwrap_or_else(|| json!([])),
                })
            })
            .collect(),
    )
}

fn rejected_expensive_items(items: &[Value]) -> Value {
    let mut rows = Vec::new();
    for item in items {
        if item_cost(item) >= 6400
            && (score_of(item) < 35.0
                || tag(item, "questionable_core")
                || scored_item_name(item) == "Refresher")
        {
            rows.push(json!({
                "name": scored_item_name(item),
                "cost": item_cost(item),
                "score": score_of(item),
                "warnings": get(item, "warnings").cloned().unwrap_or_else(|| json!([])),
                "bucket": hero_bucket(item),
            }));
        }
    }
    rows.sort_by(|left, right| {
        let left_refresher = if get_string(left, "name").as_deref() == Some("Refresher") { 0 } else { 1 };
        let right_refresher = if get_string(right, "name").as_deref() == Some("Refresher") { 0 } else { 1 };
        left_refresher
            .cmp(&right_refresher)
            .then_with(|| numeric_value(get(left, "score")).partial_cmp(&numeric_value(get(right, "score"))).unwrap_or(std::cmp::Ordering::Equal))
    });
    Value::Array(rows.into_iter().take(12).collect())
}

fn is_pure_defense(item: &Value) -> bool {
    let labels = get(item, "properties")
        .and_then(as_array)
        .into_iter()
        .flatten()
        .map(|prop| get_any_string(prop, &["label", "name"]).unwrap_or_default().to_lowercase())
        .collect::<Vec<_>>();
    if labels.is_empty() {
        return false;
    }
    let defensive = labels
        .iter()
        .filter(|label| PURE_DEFENSE_HINTS.iter().any(|hint| label.contains(hint)))
        .count();
    let offensive = labels
        .iter()
        .filter(|label| ["damage", "cooldown", "duration", "range", "radius", "slow", "fire"].iter().any(|hint| label.contains(hint)))
        .count();
    defensive >= 2 && offensive == 0 && !bool_value(get(item, "is_active"))
}

fn shop_routes_to_4800(scored_items: &[Value]) -> Value {
    let mut by_slot: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for item in scored_items {
        let slot = scored_item_slot(item);
        if PUBLIC_ITEM_SLOTS.contains(&slot.as_str()) && item_cost(item) <= 3200 {
            by_slot.entry(slot).or_default().push(item.clone());
        }
    }
    let mut routes = Map::new();
    for slot in ["vitality", "spirit", "weapon"] {
        let mut candidates = by_slot.remove(slot).unwrap_or_default();
        candidates.sort_by(|left, right| {
            (
                if tag(left, "early_economy") { 0 } else { 1 },
                if hero_bucket(left) == "Good" { 0 } else { 1 },
                item_cost(left),
            )
                .cmp(&(
                    if tag(right, "early_economy") { 0 } else { 1 },
                    if hero_bucket(right) == "Good" { 0 } else { 1 },
                    item_cost(right),
                ))
                .then_with(|| {
                    score_of(right)
                        .partial_cmp(&score_of(left))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });
        let mut route = Vec::new();
        let mut route_names = BTreeSet::new();
        let mut spend = 0;
        for item in candidates {
            let name = scored_item_name(&item);
            if !route_names.insert(name) {
                continue;
            }
            spend += item_cost(&item);
            route.push(item);
            if spend >= SHOP_BONUS_SPIKE || route.len() >= 5 {
                break;
            }
        }
        routes.insert(
            slot.to_string(),
            json!({
                "label": slot_label(slot),
                "spend": spend,
                "target": SHOP_BONUS_SPIKE,
                "items": compact_recommendations(&route),
                "target_reached": spend >= SHOP_BONUS_SPIKE,
            }),
        );
    }
    Value::Object(routes)
}

fn compact_starting_stats(stats: Option<&Value>) -> Value {
    let keys = [
        "max_health",
        "base_health_regen",
        "max_move_speed",
        "sprint_speed",
        "stamina",
        "heavy_melee_damage",
        "light_melee_damage",
    ];
    let mut object = Map::new();
    for key in keys {
        if let Some(value) = stats.and_then(|stats| get(stats, key)) {
            object.insert(key.to_string(), value.clone());
        }
    }
    Value::Object(object)
}

fn scale_hint(scale_function: Option<&Value>) -> Value {
    let Some(scale_function) = scale_function else {
        return json!([]);
    };
    if let Some(stats) = get(scale_function, "scaling_stats").filter(|value| value.is_array()) {
        return stats.clone();
    }
    if let Some(subclass) = get(scale_function, "subclass") {
        if let Some(stats) = get(subclass, "scaling_stats").filter(|value| value.is_array()) {
            return stats.clone();
        }
        if let Some(stat) = get(subclass, "specific_stat_scale_type") {
            return json!([value_to_string(stat)]);
        }
    }
    get(scale_function, "specific_stat_scale_type")
        .map(|stat| json!([value_to_string(stat)]))
        .unwrap_or_else(|| json!([]))
}

fn build_review_context(conn: &Connection, query: &str, limit_events: usize) -> Result<Value> {
    let limit = clamp_usize(limit_events, 1, 500);
    let best_match = find_best_entity_match(conn, query)?;
    let aliases = if let Some(entity_id) = best_match
        .as_ref()
        .and_then(|value| int_or_none(get(value, "id")))
    {
        load_aliases(conn, entity_id)?
    } else {
        Vec::new()
    };
    let events = load_patch_events(conn, query, best_match.as_ref(), &aliases, limit)?;
    let event_ids: Vec<i64> = events
        .iter()
        .filter_map(|event| int_or_none(get(event, "id")))
        .collect();
    let enrichments = load_enrichments(conn, &event_ids)?;
    let current_stat_hints = build_current_stat_hints(conn, query, best_match.as_ref(), &aliases)?;
    let entity_summary = build_entity_summary(query, best_match.as_ref(), &aliases);
    let timeline_signals = build_timeline_signals(&events, &enrichments);
    let open_questions = build_open_questions(
        best_match.is_none(),
        &entity_summary,
        &current_stat_hints,
        &timeline_signals,
        !enrichments.is_empty(),
    );
    Ok(json!({
        "query": query,
        "context_kind": "analysis_review",
        "entity_summary": entity_summary,
        "lineage": {
            "available": false,
            "relations": [],
            "related_names": [],
            "relation_counts": {},
            "omitted_relation_count": 0,
        },
        "current_stat_hints": current_stat_hints,
        "timeline_signals": timeline_signals,
        "open_questions": open_questions,
        "source_references": build_source_references(best_match.as_ref(), &events, &current_stat_hints),
        "prompt_de": build_prompt_de(best_match.as_ref(), query),
        "retrieval_meta": {
            "limit_events_requested": limit_events,
            "events_loaded": events.len(),
            "enrichments_loaded": enrichments.len(),
            "fallback_used": best_match.is_none(),
            "sheet_stats_available": get(&current_stat_hints, "available").cloned().unwrap_or(Value::Bool(false)),
        },
    }))
}

fn find_best_entity_match(conn: &Connection, query: &str) -> Result<Option<Value>> {
    let query_norm = normalize_alias(query);
    if table_exists(conn, "entities")? && table_exists(conn, "entity_aliases")? {
        let like_norm = format!("%{query_norm}%");
        let like_query = format!("%{query}%");
        let rows = query_json_rows(
            conn,
            r#"
            SELECT
              e.id,
              e.entity_type,
              e.canonical_name,
              e.primary_external_id,
              e.source,
              e.metadata_json,
              MAX(
                CASE
                  WHEN lower(e.canonical_name)=lower(?1) THEN 120
                  WHEN a.alias_norm=?2 AND a.alias_kind='canonical' THEN 115
                  WHEN a.alias_norm=?2 THEN 110
                  WHEN a.alias_norm LIKE ?3 THEN 80
                  WHEN lower(e.canonical_name) LIKE lower(?4) THEN 70
                  ELSE 50
                END
              ) AS score,
              GROUP_CONCAT(DISTINCT a.alias_kind) AS matched_alias_kinds
            FROM entities e
            LEFT JOIN entity_aliases a ON a.entity_id=e.id
            WHERE
              lower(e.canonical_name)=lower(?1)
              OR lower(e.canonical_name) LIKE lower(?4)
              OR a.alias_norm=?2
              OR a.alias_norm LIKE ?3
            GROUP BY e.id
            ORDER BY score DESC, e.entity_type, length(e.canonical_name), e.canonical_name
            LIMIT 1
            "#,
            &[&query, &query_norm, &like_norm, &like_query],
        )?;
        if let Some(mut row) = rows.into_iter().next() {
            if int_or_zero(get(&row, "score")) >= 100 {
                if let Some(object) = row.as_object_mut() {
                    let metadata = json_loads(object.get("metadata_json").and_then(Value::as_str), json!({}));
                    object.remove("metadata_json");
                    object.insert("metadata".to_string(), metadata);
                    let alias_kinds = object
                        .get("matched_alias_kinds")
                        .and_then(Value::as_str)
                        .map(split_group_concat)
                        .unwrap_or_default();
                    object.insert("matched_alias_kinds".to_string(), json!(alias_kinds));
                }
                return Ok(Some(row));
            }
        }
    }

    let like = format!("%{query}%");
    let row = query_one_json(
        conn,
        r#"
        SELECT
          NULL AS id,
          entity_type,
          canonical_name,
          external_id AS primary_external_id,
          source,
          payload_json,
          100 AS score
        FROM entity_snapshots
        WHERE source='deadlock_assets_api'
          AND (
            lower(canonical_name)=lower(?1)
            OR canonical_name LIKE ?2
            OR payload_json LIKE ?2
          )
        ORDER BY CASE WHEN lower(canonical_name)=lower(?1) THEN 0 ELSE 1 END, fetched_at DESC, id DESC
        LIMIT 1
        "#,
        &[&query, &like],
    )?;
    Ok(row.map(|mut value| {
        if let Some(object) = value.as_object_mut() {
            let payload = json_loads(object.get("payload_json").and_then(Value::as_str), json!({}));
            object.remove("payload_json");
            object.insert("metadata".to_string(), json!({"disabled": get(&payload, "disabled").cloned().unwrap_or(Value::Null), "document_kinds": []}));
            object.insert("matched_alias_kinds".to_string(), json!([]));
            if object.get("canonical_name").and_then(Value::as_str).unwrap_or("").is_empty() {
                if let Some(name) = get_string(&payload, "name") {
                    object.insert("canonical_name".to_string(), json!(name));
                }
            }
        }
        value
    }))
}

fn load_aliases(conn: &Connection, entity_id: i64) -> Result<Vec<Value>> {
    let rows = query_json_rows(
        conn,
        r#"
        SELECT alias, alias_norm, alias_kind, source, external_id, snapshot_id
        FROM entity_aliases
        WHERE entity_id=?1
        ORDER BY
          CASE alias_kind
            WHEN 'canonical' THEN 0
            WHEN 'snapshot_name' THEN 1
            WHEN 'class_name' THEN 2
            WHEN 'class_name_short' THEN 3
            WHEN 'external_id' THEN 4
            ELSE 5
          END,
          length(alias),
          alias
        LIMIT 80
        "#,
        &[&entity_id],
    )?;
    Ok(rows)
}

fn load_patch_events(
    conn: &Connection,
    query: &str,
    best_match: Option<&Value>,
    aliases: &[Value],
    limit: usize,
) -> Result<Vec<Value>> {
    if !table_exists(conn, "patch_events")? {
        return Ok(Vec::new());
    }
    let mut names = BTreeSet::new();
    if let Some(best_match) = best_match {
        if let Some(name) = get_string(best_match, "canonical_name") {
            names.insert(name.to_lowercase());
        }
        for alias in aliases {
            let kind = get_string(alias, "alias_kind").unwrap_or_default();
            if matches!(kind.as_str(), "canonical" | "snapshot_name" | "class_name_short") {
                if let Some(alias_value) = get_string(alias, "alias") {
                    names.insert(alias_value.to_lowercase());
                }
            }
        }
    }
    let mut rows = if names.is_empty() {
        let like = format!("%{query}%");
        query_json_rows(
            conn,
            r#"
            SELECT *
            FROM patch_events
            WHERE entity_name LIKE ?1
            ORDER BY patch_snapshot_id DESC, line_index
            LIMIT 500
            "#,
            &[&like],
        )?
    } else {
        let placeholders = (0..names.len()).map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            r#"
            SELECT *
            FROM patch_events
            WHERE lower(entity_name) IN ({placeholders})
            ORDER BY patch_snapshot_id DESC, line_index
            LIMIT 500
            "#
        );
        let params = names.into_iter().collect::<Vec<_>>();
        let mut stmt = conn.prepare(&sql)?;
        let iter = stmt.query_map(params_from_iter(params.iter()), crate::util::row_to_json)?;
        iter.collect::<rusqlite::Result<Vec<_>>>()?
    };
    for row in &mut rows {
        if let Some(object) = row.as_object_mut() {
            let metadata = json_loads(object.get("metadata_json").and_then(Value::as_str), json!({}));
            object.remove("metadata_json");
            object.insert("metadata".to_string(), metadata);
        }
    }
    rows.sort_by_key(|row| std::cmp::Reverse(event_sort_key(row)));
    rows.truncate(limit);
    Ok(rows)
}

fn load_enrichments(conn: &Connection, event_ids: &[i64]) -> Result<Vec<Value>> {
    if event_ids.is_empty() || !table_exists(conn, "patch_event_enrichments")? {
        return Ok(Vec::new());
    }
    let placeholders = (0..event_ids.len()).map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT * FROM patch_event_enrichments WHERE patch_event_id IN ({placeholders})"
    );
    let mut stmt = conn.prepare(&sql)?;
    let iter = stmt.query_map(params_from_iter(event_ids.iter()), |row| {
        crate::util::row_to_json(row).map(|value| decode_json_fields(&value))
    })?;
    Ok(iter.collect::<rusqlite::Result<Vec<_>>>()?)
}

fn build_current_stat_hints(
    conn: &Connection,
    query: &str,
    best_match: Option<&Value>,
    aliases: &[Value],
) -> Result<Value> {
    let mut names = BTreeSet::new();
    names.insert(query.to_string());
    if let Some(best_match) = best_match.and_then(|value| get_string(value, "canonical_name")) {
        names.insert(best_match);
    }
    for alias in aliases {
        let kind = get_string(alias, "alias_kind").unwrap_or_default();
        if matches!(kind.as_str(), "canonical" | "snapshot_name" | "class_name_short") {
            if let Some(value) = get_string(alias, "alias").filter(|value| !value.chars().all(|character| character.is_ascii_digit())) {
                names.insert(value);
            }
        }
    }
    let lowered = names.iter().map(|name| name.to_lowercase()).collect::<Vec<_>>();
    let mut profile = Value::Null;
    let mut value_rows = Vec::new();
    if !lowered.is_empty() && table_exists(conn, "hero_stat_profiles")? {
        let placeholders = (0..lowered.len()).map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            "SELECT hero_name, source, external_id, snapshot_id, row_number, id FROM hero_stat_profiles WHERE lower(hero_name) IN ({placeholders}) ORDER BY updated_at DESC LIMIT 1"
        );
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query_map(params_from_iter(lowered.iter()), crate::util::row_to_json)?;
        if let Some(row) = rows.next() {
            profile = row?;
        }
    }
    if !profile.is_null() && table_exists(conn, "hero_stat_values")? {
        if let Some(profile_id) = int_or_none(get(&profile, "id")) {
            value_rows = query_json_rows(
                conn,
                r#"
                SELECT stat_key, stat_label, raw_value, numeric_value
                FROM hero_stat_values
                WHERE profile_id=?1
                ORDER BY stat_key
                "#,
                &[&profile_id],
            )?;
        }
    }
    let hints = merged_sheet_stat_hints(&value_rows);
    Ok(json!({
        "available": !profile.is_null() || !hints.is_empty(),
        "source_count": if profile.is_null() { 0 } else { 1 },
        "profile": if profile.is_null() { Value::Null } else {
            json!({
                "hero_name": get(&profile, "hero_name").cloned().unwrap_or(Value::Null),
                "source": get(&profile, "source").cloned().unwrap_or(Value::Null),
                "external_id": get(&profile, "external_id").cloned().unwrap_or(Value::Null),
                "snapshot_id": get(&profile, "snapshot_id").cloned().unwrap_or(Value::Null),
                "row_number": get(&profile, "row_number").cloned().unwrap_or(Value::Null),
            })
        },
        "hints": hints.iter().take(18).cloned().collect::<Vec<_>>(),
        "omitted_hint_count": hints.len().saturating_sub(18),
        "source_tables": if profile.is_null() { json!([]) } else { json!(["hero_stat_values"]) },
    }))
}

fn build_entity_summary(query: &str, best_match: Option<&Value>, aliases: &[Value]) -> Value {
    let mut human_aliases = Vec::new();
    for alias in aliases {
        let kind = get_string(alias, "alias_kind").unwrap_or_default();
        let value = get_string(alias, "alias").unwrap_or_default();
        if !value.is_empty()
            && matches!(kind.as_str(), "canonical" | "snapshot_name" | "class_name_short")
            && !human_aliases.contains(&value)
        {
            human_aliases.push(value);
        }
    }
    let metadata = best_match
        .and_then(|value| get(value, "metadata"))
        .cloned()
        .unwrap_or_else(|| json!({}));
    json!({
        "matched": best_match.is_some(),
        "name": best_match
            .and_then(|value| get(value, "canonical_name"))
            .cloned()
            .unwrap_or_else(|| json!(query)),
        "entity_type": best_match.and_then(|value| get(value, "entity_type")).cloned().unwrap_or(Value::Null),
        "source": best_match.and_then(|value| get(value, "source")).cloned().unwrap_or(Value::Null),
        "external_id": best_match.and_then(|value| get(value, "primary_external_id")).cloned().unwrap_or(Value::Null),
        "match_score": best_match.and_then(|value| get(value, "score")).cloned().unwrap_or(Value::Null),
        "matched_alias_kinds": best_match
            .and_then(|value| get(value, "matched_alias_kinds"))
            .cloned()
            .unwrap_or_else(|| json!([])),
        "aliases": human_aliases.into_iter().take(12).collect::<Vec<_>>(),
        "metadata_hints": {
            "disabled": get(&metadata, "disabled").cloned().unwrap_or(Value::Null),
            "document_kinds": get(&metadata, "document_kinds").cloned().unwrap_or_else(|| json!([])),
        },
    })
}

fn build_timeline_signals(events: &[Value], enrichments: &[Value]) -> Value {
    let mut enrichments_by_event: BTreeMap<i64, Vec<&Value>> = BTreeMap::new();
    for enrichment in enrichments {
        if let Some(id) = int_or_none(get(enrichment, "patch_event_id").or_else(|| get(enrichment, "event_id"))) {
            enrichments_by_event.entry(id).or_default().push(enrichment);
        }
    }
    let dates = events
        .iter()
        .filter_map(|event| get_string(event, "posted_at"))
        .collect::<Vec<_>>();
    let mut recent_events = Vec::new();
    let mut stat_changes = Vec::new();
    let mut ability_mentions: BTreeMap<String, i64> = BTreeMap::new();
    let mut low_confidence_events = 0;
    for event in events {
        let event_id = int_or_none(get(event, "id")).unwrap_or_default();
        let event_enrichments = enrichments_by_event.get(&event_id).cloned().unwrap_or_default();
        if event_enrichments.iter().any(|enrichment| numeric_value(get(enrichment, "confidence")) < 0.5) {
            low_confidence_events += 1;
        }
        recent_events.push(compact_event(event, &event_enrichments));
        for enrichment in event_enrichments {
            if let Some(ability) = get_string(enrichment, "ability_name") {
                *ability_mentions.entry(ability).or_default() += 1;
            }
            if get(enrichment, "stat_name").is_some()
                && (get(enrichment, "old_value").is_some() || get(enrichment, "new_value").is_some())
            {
                stat_changes.push(compact_stat_change(event, enrichment));
            }
        }
    }
    let mut ability_rows = ability_mentions.into_iter().collect::<Vec<_>>();
    ability_rows.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    json!({
        "event_count": events.len(),
        "date_range": {
            "newest": dates.iter().max().cloned(),
            "oldest": dates.iter().min().cloned(),
        },
        "latest_patch": events.first().map(latest_patch).unwrap_or(Value::Null),
        "change_type_counts": sorted_counts(events.iter().map(|event| get_string(event, "change_type").unwrap_or_else(|| "unknown".to_string())).collect()),
        "source_counts": sorted_counts(events.iter().map(|event| get_string(event, "source_kind").unwrap_or_else(|| "unknown".to_string())).collect()),
        "top_sections": sorted_counts(events.iter().map(|event| get_string(event, "section").unwrap_or_else(|| "Unsectioned".to_string())).collect()),
        "recent_events": recent_events.iter().take(24).cloned().collect::<Vec<_>>(),
        "stat_changes": stat_changes.iter().take(24).cloned().collect::<Vec<_>>(),
        "omitted_recent_event_count": recent_events.len().saturating_sub(24),
        "omitted_stat_change_count": stat_changes.len().saturating_sub(24),
        "ability_mentions": Value::Object(ability_rows.into_iter().take(12).map(|(key, count)| (key, json!(count))).collect()),
        "low_confidence_event_count": low_confidence_events,
    })
}

fn build_open_questions(
    fallback_used: bool,
    entity_summary: &Value,
    current_stat_hints: &Value,
    timeline_signals: &Value,
    enrichments_available: bool,
) -> Vec<&'static str> {
    let mut questions = Vec::new();
    if !bool_value(get(entity_summary, "matched")) {
        questions.push("Entity konnte nicht kanonisch gematcht werden; Patch-Treffer sind nur Fallback-Suche.");
    }
    if fallback_used {
        questions.push("Retrieval nutzt Fallback-Matching; Namen und Aliase vor einer Review-Aussage pruefen.");
    }
    if int_or_zero(get(timeline_signals, "event_count")) == 0 {
        questions.push("Keine Patch-Events gefunden; Trend- oder Balance-Aussagen waeren spekulativ.");
    }
    if !enrichments_available {
        questions.push("Patch-Event-Enrichments fehlen; Stat-Aenderungen sind nur aus Rohzeilen ableitbar.");
    } else if int_or_zero(get(timeline_signals, "low_confidence_event_count")) != 0 {
        questions.push("Ein Teil der Patchzeilen ist unstrukturiert oder low-confidence; Rohzeilen gegenlesen.");
    }
    if get_string(entity_summary, "entity_type").as_deref() == Some("hero")
        && get(current_stat_hints, "hints").and_then(as_array).map(Vec::is_empty).unwrap_or(true)
    {
        questions.push("Keine aktuellen Sheet-Stat-Hints fuer den Hero gefunden.");
    }
    if int_or_zero(get(current_stat_hints, "omitted_hint_count")) != 0 {
        questions.push("Sheet-Stats wurden gekuerzt; fuer Detailanalyse ggf. Rohkontext nachladen.");
    }
    if int_or_zero(get(timeline_signals, "omitted_recent_event_count")) != 0
        || int_or_zero(get(timeline_signals, "omitted_stat_change_count")) != 0
    {
        questions.push("Timeline wurde gekuerzt; fuer historische Vollanalyse hoeheres limit_events nutzen.");
    }
    questions
}

fn build_source_references(best_match: Option<&Value>, events: &[Value], current_stat_hints: &Value) -> Value {
    let mut references = Vec::new();
    if let Some(best_match) = best_match {
        references.push(json!({
            "kind": "entity",
            "source": get(best_match, "source").cloned().unwrap_or(Value::Null),
            "label": get(best_match, "canonical_name").cloned().unwrap_or(Value::Null),
            "external_id": get(best_match, "primary_external_id").cloned().unwrap_or(Value::Null),
        }));
    }
    for event in events.iter().take(40) {
        references.push(json!({
            "kind": "patch_event",
            "source": get(event, "source_kind").cloned().unwrap_or(Value::Null),
            "label": get(event, "patch_title").cloned().unwrap_or(Value::Null),
            "url": get(event, "patch_url").cloned().unwrap_or(Value::Null),
            "posted_at": get(event, "posted_at").cloned().unwrap_or(Value::Null),
            "patch_event_id": get(event, "id").cloned().unwrap_or(Value::Null),
            "line_index": get(event, "line_index").cloned().unwrap_or(Value::Null),
        }));
    }
    if let Some(profile) = get(current_stat_hints, "profile").filter(|value| value.is_object()) {
        references.push(json!({
            "kind": "sheet_stats",
            "source": get(profile, "source").cloned().unwrap_or(Value::Null),
            "label": get(profile, "hero_name").cloned().unwrap_or(Value::Null),
            "external_id": get(profile, "external_id").cloned().unwrap_or(Value::Null),
            "snapshot_id": get(profile, "snapshot_id").cloned().unwrap_or(Value::Null),
            "row_number": get(profile, "row_number").cloned().unwrap_or(Value::Null),
        }));
    }
    Value::Array(references.into_iter().take(40).collect())
}

fn build_prompt_de(best_match: Option<&Value>, query: &str) -> String {
    let name = best_match
        .and_then(|value| get_string(value, "canonical_name"))
        .unwrap_or_else(|| query.to_string());
    let entity_type = best_match
        .and_then(|value| get_string(value, "entity_type"))
        .unwrap_or_else(|| "Entity".to_string());
    format!(
        "Du bist ein Deadlock-Analyseassistent. Nutze ausschliesslich den bereitgestellten Review-Kontext und kennzeichne Unsicherheiten klar. Behalte Namen von Items, Heroes und Abilities exakt auf Englisch; erklaere Bewertung, Patch-Interpretation und offene Fragen auf Deutsch. Ziel: Erstelle eine kompakte Review fuer {name} ({entity_type}) mit aktueller Stat-Einordnung, relevanten Timeline-Signalen, moeglichen Balance- oder Build-Implikationen und Quellenhinweisen. Erfinde keine Zahlen oder Patchdetails, die nicht im Kontext stehen."
    )
}

fn compact_event(event: &Value, enrichments: &[&Value]) -> Value {
    json!({
        "patch_event_id": get(event, "id").cloned().unwrap_or(Value::Null),
        "posted_at": get(event, "posted_at").cloned().unwrap_or(Value::Null),
        "patch_title": get(event, "patch_title").cloned().unwrap_or(Value::Null),
        "source_kind": get(event, "source_kind").cloned().unwrap_or(Value::Null),
        "section": get(event, "section").cloned().unwrap_or(Value::Null),
        "change_type": get(event, "change_type").cloned().unwrap_or(Value::Null),
        "line": get_any(event, &["normalized_line", "raw_line"]).cloned().unwrap_or(Value::Null),
        "structured_changes": enrichments.iter().map(|enrichment| compact_enrichment(enrichment)).collect::<Vec<_>>(),
    })
}

fn compact_enrichment(enrichment: &Value) -> Value {
    json!({
        "stat_name": get(enrichment, "stat_name").cloned().unwrap_or(Value::Null),
        "old_value": get(enrichment, "old_value").cloned().unwrap_or(Value::Null),
        "new_value": get(enrichment, "new_value").cloned().unwrap_or(Value::Null),
        "unit": get(enrichment, "unit").cloned().unwrap_or(Value::Null),
        "ability_name": get(enrichment, "ability_name").cloned().unwrap_or(Value::Null),
        "secondary_entity_name": get(enrichment, "secondary_entity_name").cloned().unwrap_or(Value::Null),
        "confidence": get(enrichment, "confidence").cloned().unwrap_or(Value::Null),
    })
}

fn compact_stat_change(event: &Value, enrichment: &Value) -> Value {
    let mut compact = compact_enrichment(enrichment);
    if let Some(object) = compact.as_object_mut() {
        object.insert("patch_event_id".to_string(), get(event, "id").cloned().unwrap_or(Value::Null));
        object.insert("posted_at".to_string(), get(event, "posted_at").cloned().unwrap_or(Value::Null));
        object.insert("patch_title".to_string(), get(event, "patch_title").cloned().unwrap_or(Value::Null));
        object.insert("change_type".to_string(), get(event, "change_type").cloned().unwrap_or(Value::Null));
        object.insert("line".to_string(), get_any(event, &["normalized_line", "raw_line"]).cloned().unwrap_or(Value::Null));
    }
    compact
}

fn latest_patch(event: &Value) -> Value {
    json!({
        "posted_at": get(event, "posted_at").cloned().unwrap_or(Value::Null),
        "patch_title": get(event, "patch_title").cloned().unwrap_or(Value::Null),
        "patch_url": get(event, "patch_url").cloned().unwrap_or(Value::Null),
        "source_kind": get(event, "source_kind").cloned().unwrap_or(Value::Null),
    })
}

fn merged_sheet_stat_hints(value_rows: &[Value]) -> Vec<Value> {
    let primary = [
        "base_hp",
        "max_level_hp",
        "hp_gain",
        "base_regen",
        "base_move_speed",
        "base_sprint",
        "base_stamina",
        "base_ammo",
        "base_bullet_dmg",
        "base_fire_rate",
        "base_dps",
        "max_gun_dps",
        "max_gun_damage",
        "dmg_gain",
        "spirit_gain",
        "total_bullet_ratio",
        "total_spirit_ratio",
        "aggregate_growth",
        "hp_growth_increase",
        "dps_growth_increase",
    ];
    let mut by_key = BTreeMap::new();
    for row in value_rows {
        if let Some(key) = get_string(row, "stat_key") {
            by_key.insert(
                key,
                json!({
                    "stat_key": get(row, "stat_key").cloned().unwrap_or(Value::Null),
                    "stat_label": get(row, "stat_label").cloned().unwrap_or(Value::Null),
                    "value": get(row, "raw_value").cloned().unwrap_or(Value::Null),
                    "numeric_value": get(row, "numeric_value").cloned().unwrap_or(Value::Null),
                }),
            );
        }
    }
    let mut ordered = Vec::new();
    for key in primary {
        if let Some(value) = by_key.remove(key) {
            ordered.push(value);
        }
    }
    ordered.extend(by_key.into_values());
    ordered
}

pub(crate) fn hero_name_from_id(conn: &Connection, hero_id: Option<&str>) -> Result<Option<String>> {
    let Some(hero_id) = hero_id.filter(|value| !value.trim().is_empty()) else {
        return Ok(None);
    };
    let id_int = hero_id.parse::<i64>().ok();
    let row = if let Some(id_int) = id_int {
        query_one_json(
            conn,
            r#"
            SELECT canonical_name, payload_json
            FROM entity_snapshots
            WHERE source='deadlock_assets_api' AND entity_type='hero'
              AND (external_id=?1 OR payload_json LIKE ?2)
            ORDER BY
              CASE
                WHEN payload_json LIKE '%"name"%' AND payload_json NOT LIKE '%hero_%'
                THEN 0 ELSE 1
              END,
              fetched_at DESC,
              id DESC
            LIMIT 1
            "#,
            &[&hero_id, &format!("%\"id\": {id_int}%")],
        )?
    } else {
        query_one_json(
            conn,
            r#"
            SELECT canonical_name, payload_json
            FROM entity_snapshots
            WHERE source='deadlock_assets_api' AND entity_type='hero'
              AND external_id=?1
            ORDER BY fetched_at DESC, id DESC
            LIMIT 1
            "#,
            &[&hero_id],
        )?
    };
    Ok(row.and_then(|row| {
        let payload = json_loads(get(&row, "payload_json").and_then(Value::as_str), json!({}));
        get_string(&payload, "name").or_else(|| get_string(&row, "canonical_name"))
    }))
}

pub(crate) fn asset_payload_by_id(conn: &Connection, asset_id: Option<&str>) -> Result<Option<Value>> {
    let Some(asset_id) = asset_id.filter(|value| !value.trim().is_empty()) else {
        return Ok(None);
    };
    let row = query_one_json(
        conn,
        r#"
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='deadlock_assets_api'
          AND entity_type='item_or_ability'
          AND external_id=?1
        ORDER BY fetched_at DESC, id DESC
        LIMIT 1
        "#,
        &[&asset_id],
    )?
    .or_else(|| {
        let pattern = format!("%\"id\": {asset_id}%");
        query_one_json(
            conn,
            r#"
            SELECT payload_json
            FROM entity_snapshots
            WHERE source='deadlock_assets_api'
              AND entity_type='item_or_ability'
              AND payload_json LIKE ?1
            ORDER BY fetched_at DESC, id DESC
            LIMIT 1
            "#,
            &[&pattern],
        )
        .ok()
        .flatten()
    });
    Ok(row.and_then(|row| get(&row, "payload_json").and_then(Value::as_str).map(|raw| json_loads(Some(raw), json!({})))))
}

pub(crate) fn specific_item_purpose(item: &Value) -> Option<&'static str> {
    let name = get_string(item, "name").unwrap_or_default();
    let archetypes = string_set(get(item, "archetypes"));
    match name.as_str() {
        "Transcendent Cooldown" => Some("T4 Spirit cooldown economy fuer Abilities und Active Items; stark, wenn Hero/Build ueber wiederholte Ability- und Active-Zyklen gewinnt."),
        "Superior Cooldown" => Some("Ability-spezifische Cooldown-Verkuerzung; stark fuer eine zentrale, cooldown-limitierte Ability, aber nicht fuer Active-Item-CDs."),
        "Warp Stone" => Some("Active-Reposition fuer Engage, Escape, Angle-Wechsel und Pick-Setup; kein reines Damage-Item."),
        "Ethereal Shift" => Some("Defensives Active fuer Dodge/Survive/Reset gegen Burst oder Catch; kann riskante Engage-Fenster absichern."),
        "Extra Stamina" => Some("Fruehe Stamina-Oekonomie fuer Chase, Escape, Lane-Dodges und Map-Bewegung."),
        "Healing Rite" => Some("Fruehes Lane-/Rotation-Sustain-Active; stabilisiert HP und Tempo, wird spaeter oft verkauft, wenn Slots/Core wichtiger werden."),
        "Cold Front" => Some("Fruehes/mittleres Spirit-Active fuer AoE-Slow, Wave/Fight-Setup und sichere Trefferfenster."),
        "Mystic Burst" => Some("Frueher Spirit-Burst-Verstaerker fuer Abilities mit verlaesslichen Schadensfenstern; oft Komponente fuer spaetere Spirit-Burst-Linie."),
        "Compress Cooldown" => Some("Fruehe Cooldown-Oekonomie fuer einen cooldown-limitierten Ability-Plan; Vorbereitung auf Superior/Transcendent Cooldown-Linien."),
        "Mystic Expansion" => Some("Fruehes Spirit-Scaling ueber Range/Radius/Flaechen-Zuverlaessigkeit; gut, wenn Abilities mehrere Ziele oder sichere Treffer brauchen."),
        "Restorative Locket" => Some("Defensives Sustain-/Burst-Heal-Item gegen Poke und Fight-Schaden; kauft Zeit fuer riskantere Engage- oder Poke-Heroes."),
        "Swift Striker" => Some("Weapon-DPS und Bewegungsdruck; sinnvoll, wenn der Hero zwischen Ability-Zyklen mit Gun-Trades weiter Impact erzeugen soll."),
        "Close Quarters" => Some("Frueher Nahdistanz-Weapon-Druck; stark bei Heroes, die ohnehin in kurze Distanzen gehen."),
        "Point Blank" => Some("Weapon-Core fuer Nahdistanz-Burst und Chase; passt, wenn der Hero regelmaessig in kurze Reichweite kommt."),
        "Tankbuster" => Some("Spirit-Damage-Upgrade gegen hohe HP/Frontline-Ziele; kein generisches Burst-Item, sondern Anti-Health-Skalierung."),
        "Improved Spirit" => Some("Effizientes Spirit-Stat-Upgrade fuer Ability-Schaden und Spirit-Shop-Bonus-Fortschritt."),
        "Stamina Mastery" => Some("Midgame-Stamina-Core fuer Chase, Escape, Dodge-Frequenz und laengere Fight-Pattern."),
        "Weakening Headshot" => Some("Situativer Gun-Debuff gegen Ziele, die durch Resist/Frontline-Wert schwer sterben."),
        "Crippling Headshot" => Some("T4 Weapon-Debuff fuer Resist-Break und Ziel-Fokus; stark, wenn der Hero genug Headshot-/Gun-Fenster bekommt."),
        "Majestic Leap" => Some("Makro-/Engage-Mobility fuer Winkel, Initiation und schnelle Rotations; besonders wertvoll fuer Heroes mit starkem Eintritt in Fights."),
        _ if archetypes.contains("counter") => Some("Situatives Counter-Item gegen konkrete gegnerische Mechaniken, nicht automatisch Core."),
        _ if archetypes.contains("kill_setup") => Some("Setup-Item fuer Pick-Potenzial, Catch oder verlaessliche Ability-Treffer."),
        _ if archetypes.contains("core_scaling") => Some("Core-Scaling-Item; muss zum Hero-Schadensprofil oder Cooldown-/Uptime-Plan passen."),
        _ if archetypes.contains("escape") => Some("Mobilitaets-/Survival-Item zur Fehlervermeidung, Jagd oder Reposition."),
        _ => None,
    }
}

pub(crate) fn key_item_properties(properties: Option<&Value>) -> Value {
    let rows = properties.and_then(as_array).cloned().unwrap_or_default();
    let important = rows
        .iter()
        .filter(|row| bool_value(get(row, "important")) || bool_value(get(row, "elevated")))
        .cloned()
        .collect::<Vec<_>>();
    let selected = if important.is_empty() { rows } else { important };
    Value::Array(selected.into_iter().take(10).collect())
}

fn known_payload_cost(payload: &Value) -> Option<i64> {
    let cost = int_or_none(get(payload, "cost"))?;
    if cost >= UNKNOWN_COST_SENTINEL {
        None
    } else {
        Some(cost)
    }
}

fn known_summary_cost(item: &Value) -> Option<i64> {
    let cost = int_or_none(get(item, "cost"))?;
    if cost >= UNKNOWN_COST_SENTINEL {
        None
    } else {
        Some(cost)
    }
}

fn is_property_sentinel(name: &str, prop: &Value) -> bool {
    let label = get_any_string(prop, &["label", "postvalue_label"]).unwrap_or_default();
    let blob = format!("{name} {label}").to_lowercase();
    let value = numeric_value(get(prop, "value"));
    if value >= 9999.0 && (blob.contains("channel duration") || blob.contains("abilitychanneltime")) {
        return true;
    }
    name == "ChannelMoveSpeed" && value <= -1.0
}

fn is_non_scoring_stat_property(prop: &Value) -> bool {
    let name = get_string(prop, "name").unwrap_or_default();
    matches!(
        name.as_str(),
        "AbilityCastDelay"
            | "AbilityCastRange"
            | "AbilityChannelTime"
            | "AbilityCooldown"
            | "AbilityCooldownBetweenCharge"
            | "AbilityDuration"
            | "AbilityPostCastDuration"
            | "AbilityResourceCost"
            | "AbilityUnitTargetLimit"
            | "ChannelMoveSpeed"
            | "TechPower"
            | "WeaponPower"
    ) && !meaningful_property_value(prop)
}

fn is_cooldown_reduction_property(prop: &Value, label_blob: &str) -> bool {
    let name = get_string(prop, "name").unwrap_or_default().to_lowercase();
    let provided = get_string(prop, "provided_property_type").unwrap_or_default().to_lowercase();
    meaningful_property_value(prop)
        && (name.contains("cooldownreduction")
            || provided.contains("cooldown_reduction")
            || label_blob.contains("cooldown reduction")
            || label_blob.contains("faster time between charges"))
}

fn is_spirit_resist_shred_property(prop: &Value, label_blob: &str) -> bool {
    let provided = get_string(prop, "provided_property_type").unwrap_or_default().to_lowercase();
    let value = numeric_value(get(prop, "value"));
    meaningful_property_value(prop)
        && (provided.contains("tech_armor_damage_resist_reduction")
            || label_blob.contains("spirit resist on spirit damage")
            || (label_blob.contains("spirit resist") && (label_blob.contains("reduction") || value < 0.0)))
}

fn is_spirit_amp_property(label_blob: &str) -> bool {
    label_blob.contains("spirit amp")
        || label_blob.contains("tech damage percent")
        || label_blob.contains("outgoing tech damage")
        || label_blob.contains("magic increase")
}

fn meaningful_property_value(prop: &Value) -> bool {
    let value = get(prop, "value");
    let numeric = numeric_value(value);
    let text = value.map(value_to_string).unwrap_or_default();
    !text.trim().is_empty() && numeric != 0.0 && numeric > -9999.0 && numeric < 9999.0
}

fn damage_property_name(name: &str) -> bool {
    matches!(name, "Damage" | "DPS") || name.contains("DamagePer") || name.contains("DPS")
}

fn is_spirit_dominant(hero_needs: &Value) -> bool {
    let damage_profile = get(hero_needs, "damage_profile").unwrap_or(&Value::Null);
    if bool_value(get(damage_profile, "learned_spirit_dominant")) {
        return true;
    }
    get_string(damage_profile, "damage_plan").as_deref() == Some("spirit")
        && int_or_zero(get(damage_profile, "spirit_scaling_damage_sources")) >= 3
}

fn is_spirit_dominant_weapon_core_item(name: &str) -> bool {
    item_name_in(name, SPIRIT_DOMINANT_WEAPON_CORE_ITEMS)
}

fn is_passive_economy_generalist(item: &Value) -> bool {
    let name = get_string(item, "name").unwrap_or_default();
    if name == "Trophy Collector" {
        return true;
    }
    let description = get_string(item, "description").unwrap_or_default().to_lowercase();
    description.contains("passive soul generation") || description.contains("souls per minute")
}

fn add_if_any(archetypes: &mut BTreeSet<String>, blob: &str, tag: &str, needles: &[&str]) {
    if needles.iter().any(|needle| blob.contains(needle)) {
        archetypes.insert(tag.to_string());
    }
}

fn string_set(value: Option<&Value>) -> BTreeSet<String> {
    value
        .and_then(as_array)
        .into_iter()
        .flatten()
        .filter_map(value_to_non_empty_string)
        .collect()
}

fn bucket_score(bucket: &str) -> f64 {
    match bucket {
        "Best" => 45.0,
        "Good" => 28.0,
        "Bad" => -35.0,
        "Avoid" => -50.0,
        _ => 0.0,
    }
}

fn item_cost(item: &Value) -> i64 {
    get(item, "item")
        .and_then(known_summary_cost)
        .unwrap_or(SHOP_BONUS_CAP + 1)
}

fn score_of(item: &Value) -> f64 {
    numeric_value(get(item, "score"))
}

fn tag(item: &Value, name: &str) -> bool {
    get(item, "tags")
        .and_then(|tags| get(tags, name))
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn hero_bucket(item: &Value) -> String {
    get_string(item, "hero_bucket").unwrap_or_default()
}

fn scored_item_name(item: &Value) -> String {
    get(item, "item")
        .and_then(|item| get_string(item, "name"))
        .unwrap_or_default()
}

fn scored_item_slot(item: &Value) -> String {
    get(item, "item")
        .and_then(|item| get_string(item, "slot"))
        .unwrap_or_default()
}

fn item_names_from_scored(items: &[Value]) -> BTreeSet<String> {
    items.iter().map(scored_item_name).collect()
}

fn slot_label(slot: &str) -> &str {
    match slot {
        "weapon" => "Gun",
        "vitality" => "HP",
        "spirit" => "Spirit",
        _ => slot,
    }
}

fn normalize_alias(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|character| if character.is_alphanumeric() { character } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn split_group_concat(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect()
}

fn event_sort_key(event: &Value) -> (String, i64, i64) {
    (
        get_string(event, "posted_at").unwrap_or_default(),
        int_or_zero(get(event, "patch_snapshot_id")),
        int_or_zero(get(event, "line_index")),
    )
}
