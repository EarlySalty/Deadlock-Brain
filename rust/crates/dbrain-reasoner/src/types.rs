use std::collections::{BTreeMap, BTreeSet};

use deadlock_brain_core::ai::AiClient;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReasonerConfig {
    pub patch_tag: String,
    pub bracket: String,
    pub use_ai: bool,
    pub min_matches: i64,
    pub min_prevalence_builds: i64,
    pub combat_window_seconds: f64,
    pub channel_uptime: f64,
}

impl Default for ReasonerConfig {
    fn default() -> Self {
        Self {
            patch_tag: "current".to_string(),
            bracket: "badge80".to_string(),
            use_ai: true,
            min_matches: 100,
            min_prevalence_builds: 5,
            combat_window_seconds: 40.0,
            channel_uptime: 0.55,
        }
    }
}

#[derive(Clone)]
pub struct ReasonerCtx {
    pub pool: PgPool,
    pub ai: Option<AiClient>,
    pub config: ReasonerConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum ReasonerError {
    #[error("Held nicht gefunden: {0}")]
    HeroNotFound(String),
    #[error("Snapshot fehlt: {0}")]
    MissingSnapshot(String),
    #[error("Datenbankfehler: {0}")]
    Db(#[source] sqlx::Error),
    #[error("KI-Fehler: {0}")]
    Ai(String),
    #[error("Datenfehler: {0}")]
    Data(String),
}

pub type Result<T> = std::result::Result<T, ReasonerError>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DamageType {
    Weapon,
    Spirit,
    Hybrid,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SlotType {
    Weapon,
    Vitality,
    Spirit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Confidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuyPhase {
    Lane,
    Mid,
    Core,
    Late,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TierBonus {
    pub tier: i64,
    pub value: f64,
    pub value_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PurchaseBonuses {
    pub spirit: Vec<TierBonus>,
    pub weapon: Vec<TierBonus>,
    pub vitality: Vec<TierBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LevelPoint {
    pub level: i64,
    pub required_souls: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScalingStat {
    pub stat: String,
    pub per_level: f64,
    pub per_spirit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeaponProfile {
    pub bullet_damage: f64,
    pub shots_per_second: f64,
    pub clip_size: f64,
    pub reload_duration: f64,
    pub range: f64,
    pub falloff_start_range: f64,
    pub falloff_end_range: f64,
    pub sustained_dps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AbilityRole {
    Damage,
    Control,
    Mobility,
    Sustain,
    Utility,
    Ultimate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScalingStep {
    pub upgrade_index: i64,
    pub stat: String,
    pub from: f64,
    pub to: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityModel {
    #[serde(default)]
    pub item_proc_disabled: bool,
    #[serde(default)]
    pub upgrades: Vec<serde_json::Value>,
    #[serde(default)]
    pub properties: BTreeMap<String, f64>,
    pub ability_id: i64,
    pub class_name: String,
    pub slot: i64,
    pub roles: Vec<AbilityRole>,
    pub scaling: Vec<ScalingStat>,
    pub channel_time: Option<f64>,
    pub charges: i64,
    pub cooldown: f64,
    pub scaling_step: Option<ScalingStep>,
    pub damage_type: DamageType,
    #[serde(default)]
    pub base_effect: f64,
    #[serde(default)]
    pub tick_rate: Option<f64>,
    #[serde(default)]
    pub duration: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DamagePlan {
    pub weapon_dps: f64,
    pub spirit_dps: f64,
    pub weapon_share: f64,
    pub primary_axis: DamageType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeroModel {
    #[serde(default)]
    pub base_spirit_power: f64,
    #[serde(default)]
    pub standard_level_up_upgrades: BTreeMap<String, f64>,
    #[serde(default)]
    pub standard_upgrade_levels: BTreeSet<i64>,
    #[serde(default)]
    pub level_rewards: BTreeMap<i64, Vec<String>>,
    #[serde(default)]
    pub cost_bonuses: BTreeMap<String, Vec<CostBonus>>,
    pub hero_id: i64,
    pub name: String,
    pub archetype: String,
    pub base_health: f64,
    pub level_curve: Vec<LevelPoint>,
    pub purchase_bonuses: PurchaseBonuses,
    pub scaling: Vec<ScalingStat>,
    pub weapon: WeaponProfile,
    pub abilities: Vec<AbilityModel>,
    pub damage_plan: DamagePlan,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionKind {
    None,
    ActiveCooldown { uptime: f64, cooldown: f64 },
    ActionBound { action: String },
    RampUp { ramp_seconds: f64 },
    StateBound { threshold: f64 },
    MeleeBound,
    ShotBound,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemModel {
    #[serde(default)]
    pub property_spirit_scaling: BTreeMap<String, f64>,
    #[serde(default)]
    pub property_damage_types: BTreeMap<String, DamageType>,
    #[serde(default)]
    pub component_items: Vec<String>,
    #[serde(default)]
    pub class_name: String,
    #[serde(default)]
    pub description: String,
    pub item_id: i64,
    pub name: String,
    pub slot: SlotType,
    pub tier: i64,
    pub cost: i64,
    pub is_active: bool,
    pub shopable: bool,
    pub disabled: bool,
    pub damage_axis: DamageType,
    pub defense_kind: Vec<String>,
    pub properties: BTreeMap<String, f64>,
    pub passive_properties: BTreeMap<String, f64>,
    #[serde(default)]
    pub conditional_properties: BTreeSet<String>,
    pub condition: ConditionKind,
    pub proc_cooldown: Option<f64>,
    pub imbueable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemScore {
    pub combat_value: f64,
    pub per_slot_value: f64,
    pub per_soul_value: f64,
    pub purchase_bonus_value: f64,
    pub condition_factor: f64,
    pub active_value: f64,
    pub passive_value: f64,
    pub meta_support: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EvidenceKind {
    Mechanic,
    Meta,
    Author,
    Claim,
    Patch,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Evidence {
    pub kind: EvidenceKind,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoredItem {
    pub item: ItemModel,
    pub score: ItemScore,
    pub confidence: Confidence,
    pub buy_phase: BuyPhase,
    pub sources: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SituationKind {
    CanBuyN(u32),
    Tryhard,
    Shields,
    Optional,
    Counters,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityStep {
    pub ability_id: i64,
    pub currency_type: i64,
    pub delta: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildItem {
    pub item_id: i64,
    pub name: String,
    pub tier: i64,
    pub buy_phase: BuyPhase,
    pub why: String,
    pub confidence: Confidence,
    pub imbue_target: Option<i64>,
    pub sell_priority: Option<u32>,
    pub sources: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SituationBlock {
    pub label: String,
    pub optional: bool,
    pub kind: SituationKind,
    pub items: Vec<BuildItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildObject {
    pub hero_id: i64,
    pub hero_name: String,
    pub patch_tag: String,
    pub name: String,
    pub core: Vec<BuildItem>,
    pub situations: Vec<SituationBlock>,
    pub ability_order: Vec<AbilityStep>,
    pub confidence: Confidence,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeltaTarget {
    Hero(i64),
    Item(i64),
    Ability(i64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchDelta {
    pub target: DeltaTarget,
    pub mechanic: String,
    pub sign: i8,
    pub magnitude: f64,
    pub note: String,
    #[serde(default)]
    pub application: Option<PatchApplication>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchApplication {
    pub field: String,
    pub snapshot_value: f64,
    pub fetched_at: f64,
    pub posted_at: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSnapshot {
    pub target: DeltaTarget,
    pub name: String,
    pub fields: BTreeMap<String, SnapshotField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotField {
    pub value: f64,
    pub fetched_at: Option<f64>,
    pub source: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaRow {
    pub item_id: i64,
    pub patch_tag: String,
    pub prevalence_builds: i64,
    pub wins: i64,
    pub losses: i64,
    pub matches: i64,
    pub avg_buy_time_relative: Option<f64>,
    pub lift_pp: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorBuild {
    pub author: String,
    pub version: i64,
    pub published_at: Option<i64>,
    pub last_updated_at: Option<i64>,
    pub patch_tag: Option<String>,
    pub core_item_ids: Vec<i64>,
    pub buy_order: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoreLayoutBand {
    pub tier: i64,
    pub median: f64,
    pub lower_quartile: f64,
    pub upper_quartile: f64,
    pub target: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoreLayoutStats {
    pub source_builds: usize,
    pub total_median: f64,
    pub total_lower_quartile: f64,
    pub total_upper_quartile: f64,
    pub flex_slots: usize,
    pub bands: BTreeMap<i64, CoreLayoutBand>,
}

impl Default for CoreLayoutStats {
    fn default() -> Self {
        Self {
            source_builds: 0,
            total_median: 0.0,
            total_lower_quartile: 0.0,
            total_upper_quartile: 0.0,
            flex_slots: 0,
            bands: BTreeMap::new(),
        }
    }
}

impl CoreLayoutStats {
    pub fn target_for_tier(&self, tier: i64) -> usize {
        self.bands.get(&tier).map(|band| band.target).unwrap_or(0)
    }

    pub fn total_target(&self) -> usize {
        self.bands.values().map(|band| band.target).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CoreLayoutIndex {
    pub by_hero: BTreeMap<i64, CoreLayoutStats>,
    pub overall: CoreLayoutStats,
}

impl CoreLayoutIndex {
    pub fn for_hero(&self, hero_id: i64) -> &CoreLayoutStats {
        self.by_hero.get(&hero_id).unwrap_or(&self.overall)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BacktestFilter {
    pub hero: Option<String>,
    pub patch_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BacktestMetrics {
    pub core_coverage: f64,
    #[serde(default)]
    pub reference_recall: f64,
    #[serde(default)]
    pub core_jaccard: f64,
    pub order_proximity: Option<f64>,
    pub switch_detected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeroBacktest {
    pub hero_id: i64,
    pub hero_name: String,
    pub per_author: Vec<(String, BacktestMetrics)>,
    pub aggregate: BacktestMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BacktestReport {
    pub heroes: Vec<HeroBacktest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchImpactReport {
    pub hero_id: i64,
    pub hero_name: String,
    pub deltas: Vec<PatchDelta>,
    pub shifted_items: Vec<(i64, f64)>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaIndex {
    pub by_item: BTreeMap<i64, MetaSupport>,
    pub sample_ok: BTreeSet<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaSupport {
    pub prevalence: f64,
    pub winrate_pp: Option<f64>,
    pub lift_pp: Option<f64>,
    pub author_hits: i64,
    pub claim_hits: i64,
    pub avg_buy_time_relative: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn shared_types_round_trip_without_losing_enum_fields() {
        let config = ReasonerConfig::default();
        let value = serde_json::to_value(&config).unwrap();
        assert_eq!(value["combat_window_seconds"], json!(40.0));
        assert_eq!(value["channel_uptime"], json!(0.55));
        let restored: ReasonerConfig = serde_json::from_value(value).unwrap();
        assert_eq!(restored, config);
    }

    #[test]
    fn situation_kind_serializes_as_tagged_value() {
        let value = serde_json::to_value(SituationKind::CanBuyN(1)).unwrap();
        assert_eq!(value, json!({"CanBuyN": 1}));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostBonus {
    pub gold_threshold: i64,
    pub bonus: f64,
}
