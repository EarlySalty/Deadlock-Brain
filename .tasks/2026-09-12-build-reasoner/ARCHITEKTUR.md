# ARCHITEKTUR: build-reasoner

status: Entwurf (2026-09-12), Design-Paket P0, lesend

Alle Feldnamen unten sind gegen echte Daten geprueft: Hero- und Item-Payload aus
`data/raw/deadlock_assets_api/heroes.113bc12ea286f2a2.json` und
`items.f16f8c8dd04ab5d9.json`, Tabellen aus `dbrain-builds/src/engine.rs`
(`brain.hero_item_stats`, `brain.item_catalog`), Publish-Payload aus
`Deadlock-Steam-Bot/rust/crates/steam-core/src/task/handlers/builds/publish_original.rs`,
AI-Transport aus `deadlock-brain-core/src/ai.rs`. Wo etwas nicht verifizierbar war,
steht es unter "Offene Fragen".

## 1. Neues Crate

Arbeitsname `dbrain-reasoner` (Vorschlag beibehalten, passt zur `dbrain-*`-Reihe).
Workspace-Mitglied unter `rust/crates/dbrain-reasoner`. Abhaengigkeiten:
`deadlock-brain-core` (AiClient, Settings, pg), `dbrain-builds` (Publish-Payload-Typen),
`dbrain-learn` (bestehender Helden-Kontext, siehe Wiederverwendung), `dbrain-retrieval`
(Claims, Timeline), `sqlx`, `serde`, `serde_json`, `anyhow`.

Kein neues LLM, kein neuer Connector: die KI-Rollen laufen ausschliesslich ueber
`deadlock_brain_core::ai::AiClient` (Fireworks, Deepseek V4 Flash) mit einem
`--no-ai`-Schalter, der jeden Modellaufruf abschaltet. Ohne KI liefert der Reasoner
ein vollstaendiges Build mit Zahlen und Reihenfolge, nur ohne die deutschen
Begruendungstexte und ohne Kritiker-Runde.

## 2. Module und Dateien

Jede Datei gehoert genau einem Paket. `lib.rs` traegt nur `mod`-Zeilen und
`pub use`, sie wird von der Delegator-Integration gepflegt, nicht von B oder C.

| Datei | Paket | Inhalt |
|---|---|---|
| `Cargo.toml`, `src/lib.rs`, `src/types.rs` | A | Crate-Geruest, alle geteilten Typen, Orchestrierungs-Fassade |
| `src/data.rs` | A | Ladeschicht: liest brain.*-Tabellen und entity_snapshots-Payload, gibt A-Typen zurueck |
| `src/ai_roles.rs` | A | AiClient-Wrapper, Request-Builder, Response-Parser, JSON-Schemas der Rollen |
| `src/mechanics.rs` | B | Reine Rechenregeln, kein IO (siehe MECHANIK.md) |
| `src/hero.rs` | B | Helden-Modell aus Payload |
| `src/item.rs` | B | Item-Modell und deterministisches Scoring |
| `src/patch.rs` | C | Patch-Delta aus patch_events |
| `src/meta.rs` | C | Meta-Signale, Gewichtung, Mindeststichproben |
| `src/composer.rs` | C | Baut das Build-Objekt und die Situationsbloecke |
| `src/backtest.rs` | C | Backtest gegen Autoren-Builds |
| `src/publish.rs` | C | Abbildung Build-Objekt auf Publish-Payload |

B und C haengen nur an den in `types.rs` und dieser Datei fixierten Signaturen von
A, nicht aneinander: `composer.rs` und `backtest.rs` rufen die `pub fn` aus
`item.rs`/`hero.rs`/`mechanics.rs` ueber deren hier festgelegte Signaturen. Kein
gemeinsamer Datei-Schreibzugriff ausser `lib.rs`.

## 3. Paketschnitt und Startreihenfolge

- **Paket A (Fundament, zuerst allein):** legt Crate, Cargo-Workspace-Eintrag,
  `lib.rs`, `types.rs`, `data.rs`, `ai_roles.rs` an. Liefert alle Typen und die
  Ladeschicht. Fertig, bevor B und C starten.
- **Paket B (deterministischer Kern):** `mechanics.rs`, `hero.rs`, `item.rs`.
  Startet nach A, parallel zu C.
- **Paket C (Zusammensetzung und Belege):** `patch.rs`, `meta.rs`, `composer.rs`,
  `backtest.rs`, `publish.rs`. Startet nach A, parallel zu B. Der Integrationstest
  von `composer.rs` gegen echte B-Ausgabe laeuft erst, wenn B gemergt ist; bis
  dahin testet C gegen die in A definierten Typen mit Fixtures.

Delegator-Integration traegt nach jedem Paket die `mod`-Zeilen in `lib.rs` nach und
haelt `cargo fmt`/`clippy` gruen.

## 4. Datenfluss

### Liest (nichts davon wird geschrieben)

| Modul | Quelle | Felder |
|---|---|---|
| `data::load_hero_model` | `brain.entity_snapshots` (payload, entity_type='hero') | `id`, `class_name`, `name`, `purchase_bonuses`, `scaling_stats`, `level_info`, `starting_stats`, `items`, `disabled`, `hero_type`, `gun_tag` |
| `data::load_item_models` | `brain.entity_snapshots` (item, ability) und `brain.item_catalog` | Payload: `is_active_item`, `item_slot_type`, `item_tier`, `cost`, `shopable`, `activation`, `properties`, `upgrades`, `weapon_info`, `tooltip_sections`, `description`; Katalog: `item_id`, `name`, `slot_type`, `tier`, `defense_kind`, `damage_axis` |
| `data::load_meta_rows` | `brain.hero_item_stats` | `patch_tag`, `bracket`, `prevalence_builds`, `wins`, `losses`, `matches`, `avg_buy_time_relative`, `lift_pp` |
| `data::load_patch_events` | `brain.patch_events`, `brain.patch_event_enrichments` | Event-Zeilen und Enrichments je Held/Item/Ability |
| `data::load_author_builds` | `tierlist.hero_build_sources`, `tierlist.watched_build_authors` | `details` jsonb, `version`, `published_at`, `last_updated_at` |
| `data::load_claims` | Claims-Sicht aus `dbrain-retrieval` (YouTube, Forum, Reddit) | Creator-Claims je Held/Item |
| `data::load_hero_stat_values` | `brain.hero_stat_values` | Skalierungsstats je Held |
| `data::load_synergies` | `brain.hero_item_synergies` | Item-Item-Synergie je Held |

### Schreibt (neue Tabellen, Schema `brain`, DDL-Entwurf)

```sql
CREATE TABLE IF NOT EXISTS brain.reasoner_builds (
  hero_id        bigint      NOT NULL,
  patch_tag      text        NOT NULL,
  hero_name      text        NOT NULL,
  build          jsonb       NOT NULL,
  confidence     text        NOT NULL,
  used_ai        boolean     NOT NULL DEFAULT false,
  created_at     timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, patch_tag)
);

CREATE TABLE IF NOT EXISTS brain.reasoner_item_scores (
  hero_id        bigint      NOT NULL,
  patch_tag      text        NOT NULL,
  item_id        bigint      NOT NULL,
  combat_value      double precision NOT NULL,
  per_slot_value    double precision NOT NULL,
  purchase_bonus    double precision NOT NULL,
  condition_factor  double precision NOT NULL,
  meta_support      double precision NOT NULL,
  total          double precision NOT NULL,
  confidence     text        NOT NULL,
  buy_phase      text        NOT NULL,
  created_at     timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, patch_tag, item_id)
);

CREATE TABLE IF NOT EXISTS brain.reasoner_patch_deltas (
  hero_id        bigint      NOT NULL,
  patch_tag      text        NOT NULL,
  target_kind    text        NOT NULL,
  target_id      bigint      NOT NULL,
  mechanic       text        NOT NULL,
  sign           smallint    NOT NULL,
  magnitude      double precision NOT NULL,
  note           text        NOT NULL,
  created_at     timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, patch_tag, target_kind, target_id, mechanic)
);

CREATE TABLE IF NOT EXISTS brain.reasoner_backtests (
  run_id         uuid        NOT NULL DEFAULT gen_random_uuid(),
  hero_id        bigint      NOT NULL,
  patch_tag      text        NOT NULL,
  author         text        NOT NULL,
  core_coverage  double precision NOT NULL,
  order_proximity double precision NOT NULL,
  switch_detected boolean,
  detail         jsonb       NOT NULL,
  created_at     timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (run_id, hero_id, author)
);
```

Das Item-Score-Table macht das Fertig-Kriterium "reproduzierbar ohne KI-Aufruf"
pruefbar: jede Zahl im Build liegt als eigene Zeile mit ihren Komponenten vor.

## 5. Geteilte Typen (`types.rs`, Paket A)

```rust
pub struct ReasonerConfig {
    pub patch_tag: String,
    pub bracket: String,
    pub use_ai: bool,
    pub min_matches: i64,
    pub min_prevalence_builds: i64,
    pub combat_window_seconds: f64,
    pub channel_uptime: f64,
}

pub struct ReasonerCtx {
    pub pool: PgPool,
    pub ai: Option<AiClient>,
    pub config: ReasonerConfig,
}

pub enum ReasonerError {
    HeroNotFound(String),
    MissingSnapshot(String),
    Db(sqlx::Error),
    Ai(String),
    Data(String),
}

pub type Result<T> = std::result::Result<T, ReasonerError>;

pub enum DamageType { Weapon, Spirit, Hybrid, None }

pub enum SlotType { Weapon, Vitality, Spirit }

pub enum Confidence { Low, Medium, High }

pub enum BuyPhase { Lane, Core, Late }

pub struct TierBonus { pub tier: i64, pub value: f64, pub value_type: String }

pub struct PurchaseBonuses {
    pub spirit: Vec<TierBonus>,
    pub weapon: Vec<TierBonus>,
    pub vitality: Vec<TierBonus>,
}

pub struct LevelPoint { pub level: i64, pub required_souls: i64 }

pub struct ScalingStat { pub stat: String, pub per_level: f64, pub per_spirit: Option<f64> }

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

pub enum AbilityRole { Damage, Control, Mobility, Sustain, Utility, Ultimate }

pub struct ScalingStep { pub upgrade_index: i64, pub stat: String, pub from: f64, pub to: f64 }

pub struct AbilityModel {
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
}

pub struct DamagePlan {
    pub weapon_dps: f64,
    pub spirit_dps: f64,
    pub weapon_share: f64,
    pub primary_axis: DamageType,
}

pub struct HeroModel {
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

pub enum ConditionKind {
    None,
    ActiveCooldown { uptime: f64, cooldown: f64 },
    ActionBound { action: String },
    RampUp { ramp_seconds: f64 },
    StateBound { threshold: f64 },
    MeleeBound,
    ShotBound,
}

pub struct ItemModel {
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
    pub condition: ConditionKind,
    pub proc_cooldown: Option<f64>,
    pub imbueable: bool,
}

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

pub enum EvidenceKind { Mechanic, Meta, Author, Claim, Patch }

pub struct Evidence { pub kind: EvidenceKind, pub detail: String }

pub struct ScoredItem {
    pub item: ItemModel,
    pub score: ItemScore,
    pub confidence: Confidence,
    pub buy_phase: BuyPhase,
    pub sources: Vec<Evidence>,
}

pub enum SituationKind { CanBuyN(u32), Tryhard, Shields, Optional, Counters }

pub struct AbilityStep { pub ability_id: i64, pub currency_type: i64, pub delta: i64 }

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

pub struct SituationBlock {
    pub label: String,
    pub optional: bool,
    pub kind: SituationKind,
    pub items: Vec<BuildItem>,
}

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

pub enum DeltaTarget { Hero(i64), Item(i64), Ability(i64) }

pub struct PatchDelta {
    pub target: DeltaTarget,
    pub mechanic: String,
    pub sign: i8,
    pub magnitude: f64,
    pub note: String,
}

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

pub struct AuthorBuild {
    pub author: String,
    pub version: i64,
    pub published_at: Option<i64>,
    pub last_updated_at: Option<i64>,
    pub patch_tag: Option<String>,
    pub core_item_ids: Vec<i64>,
    pub buy_order: Vec<i64>,
}

pub struct BacktestFilter { pub hero: Option<String>, pub patch_tag: Option<String> }

pub struct BacktestMetrics {
    pub core_coverage: f64,
    pub order_proximity: f64,
    pub switch_detected: Option<bool>,
}

pub struct HeroBacktest {
    pub hero_id: i64,
    pub hero_name: String,
    pub per_author: Vec<(String, BacktestMetrics)>,
    pub aggregate: BacktestMetrics,
}

pub struct BacktestReport { pub heroes: Vec<HeroBacktest> }

pub struct PatchImpactReport {
    pub hero_id: i64,
    pub hero_name: String,
    pub deltas: Vec<PatchDelta>,
    pub shifted_items: Vec<(i64, f64)>,
    pub summary: String,
}
```

Orchestrierungs-Fassade in `lib.rs` (Signaturen fix, Body in Paket C bzw. A):

```rust
pub fn reason_build(ctx: &ReasonerCtx, hero: &str) -> Result<BuildObject>;
pub fn reason_patch_impact(ctx: &ReasonerCtx, hero: &str) -> Result<PatchImpactReport>;
pub fn reason_backtest(ctx: &ReasonerCtx, filter: BacktestFilter) -> Result<BacktestReport>;
```

## 6. Ladeschicht (`data.rs`, Paket A)

```rust
pub fn load_hero_model(ctx: &ReasonerCtx, hero: &str) -> Result<HeroModel>;
pub fn load_item_models(ctx: &ReasonerCtx) -> Result<Vec<ItemModel>>;
pub fn load_meta_rows(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<MetaRow>>;
pub fn load_patch_events(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<Value>>;
pub fn load_author_builds(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<AuthorBuild>>;
pub fn load_claims(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<Value>>;
pub fn load_hero_stat_values(ctx: &ReasonerCtx, hero_id: i64) -> Result<Vec<ScalingStat>>;
```

`load_hero_model` und `load_item_models` bauen die B-Modelle nicht selbst, sie
liefern rohe `serde_json::Value`-Payloads plus Katalogzeilen; die Modellbildung
(`hero::build_hero_model`, `item::build_item_model`) liegt in Paket B. Damit haengt
A nicht an B.

## 7. Deterministischer Kern (Paket B)

```rust
pub fn build_hero_model(payload: &Value, abilities: &[Value], stats: &[ScalingStat]) -> Result<HeroModel>;
pub fn damage_plan(hero: &HeroModel, cfg: &ReasonerConfig) -> DamagePlan;
pub fn scaling_step(ability: &Value) -> Option<ScalingStep>;

pub fn build_item_model(payload: &Value, catalog_slot: &str, catalog_axis: &str, defense_kind: &[String]) -> Result<ItemModel>;
pub fn classify_condition(desc: &str, props: &BTreeMap<String, f64>, is_active: bool) -> ConditionKind;
pub fn score_item(item: &ItemModel, hero: &HeroModel, meta: &MetaIndex, deltas: &[PatchDelta], cfg: &ReasonerConfig) -> ScoredItem;
pub fn score_items(hero: &HeroModel, items: &[ItemModel], meta: &MetaIndex, deltas: &[PatchDelta], cfg: &ReasonerConfig) -> Vec<ScoredItem>;

pub fn purchase_bonus_value(item: &ItemModel, hero: &HeroModel) -> f64;
pub fn combat_window_value(item: &ItemModel, hero: &HeroModel, cfg: &ReasonerConfig) -> f64;
pub fn condition_factor(item: &ItemModel, cfg: &ReasonerConfig) -> f64;
pub fn proc_stacks(tick_rate: f64, proc_cooldown: f64, window: f64) -> f64;
pub fn per_slot_value(combat: f64, purchase: f64) -> f64;
pub fn weapon_dps(weapon: &WeaponProfile, window: f64) -> f64;
pub fn ability_dps(ability: &AbilityModel, hero: &HeroModel, cfg: &ReasonerConfig) -> f64;
```

`MetaIndex` ist in Paket C definiert; damit `score_item` es konsumieren kann, lebt
der Typ in `types.rs` (Paket A), nur seine Konstruktion in `meta.rs` (C). Das ist
die einzige Stelle, an der B einen von C konstruierten Typ liest, und sie ist ueber
A entkoppelt.

## 8. Zusammensetzung und Belege (Paket C)

```rust
pub struct MetaIndex {
    pub by_item: BTreeMap<i64, MetaSupport>,
    pub sample_ok: BTreeSet<i64>,
}

pub struct MetaSupport {
    pub prevalence: f64,
    pub winrate_pp: Option<f64>,
    pub lift_pp: Option<f64>,
    pub author_hits: i64,
    pub claim_hits: i64,
    pub avg_buy_time_relative: Option<f64>,
}

pub fn build_meta_index(rows: &[MetaRow], authors: &[AuthorBuild], claims: &[Value], cfg: &ReasonerConfig) -> MetaIndex;

pub fn compute_patch_delta(hero: &HeroModel, events: &[Value]) -> Vec<PatchDelta>;

pub fn compose_build(hero: &HeroModel, scored: &[ScoredItem], deltas: &[PatchDelta], cfg: &ReasonerConfig) -> BuildObject;

pub fn backtest_metrics(build: &BuildObject, author: &AuthorBuild) -> BacktestMetrics;
pub fn backtest_hero(ctx: &ReasonerCtx, hero: &str) -> Result<HeroBacktest>;

pub fn to_publish_payload(build: &BuildObject) -> BuildSpecPayload;
```

`compose_build` ist der deterministische Composer aus dem Auftrag: er waehlt Kern
und Situationsbloecke nach Score, Slot-Knappheit, Damage-Plan und Patch-Delta, ohne
KI. Er erzeugt die Blocktitel `Core`, `Can buy N`, `Tryhard`, `Shields`, `Optional`,
`Counters` analog zum Referenz-Screenshot. Die deutschen Warum-Texte und die
Kritik kommen danach aus den KI-Rollen.

## 9. Agentenrollen und Zusammenspiel

Deterministisch in Rust (liefern Zahlen und Auswahl): Helden-Modell (`hero.rs`),
Item-Modell und Scoring (`item.rs`, `mechanics.rs`), Patch-Delta (`patch.rs`),
Meta-Gewichtung (`meta.rs`), Composer (`composer.rs`), Backtest (`backtest.rs`).

KI ueber `deadlock_brain_core::ai::AiClient` (liefern nur Text und
Klassifikationen, nie Zahlen):

| Rolle | Eingabe (JSON, deterministisch erzeugt) | Ausgabe (JSON-Schema) | Wirkung |
|---|---|---|---|
| Hero-Analyst | HeroModel plus DamagePlan | `{"ability_roles":[{"ability_id":int,"role":str,"note":str}],"playstyle":str}` | Rollen-Labels und Spielstil-Text, veraendert keine Zahl |
| Item-Analyst | ScoredItem-Liste mit Score-Komponenten und ConditionKind | `{"items":[{"item_id":int,"why":str,"condition_note":str}]}` | Warum-Text je Item, Bedingungshinweis |
| Patch-Analyst | PatchDelta-Liste plus rohe patch_event-Zeilen | `{"notes":[{"target_kind":str,"target_id":int,"label":str,"rationale":str}]}` | Klartext zur Verschiebung, Vorzeichen kommt aus dem Delta |
| Meta-Analyst | MetaIndex-Auszug plus Mindeststichproben-Status | `{"summary":str}` | Meta-Einordnung als Text |
| Kritiker | fertiges BuildObject | `{"verdict":"pass"|"recompose","issues":[str]}` | Gate, siehe Abbruchregeln |

Reihenfolge und Uebergabeobjekte:

1. `load_hero_model` plus `hero::build_hero_model` erzeugen HeroModel.
2. `load_item_models` plus `item::build_item_model` erzeugen ItemModel-Liste.
3. `meta::build_meta_index` und `patch::compute_patch_delta` liefern MetaIndex und
   PatchDelta.
4. `item::score_items` liefert die ScoredItem-Liste (deterministisch, endgueltige
   Zahlen).
5. `composer::compose_build` baut das BuildObject.
6. Nur wenn `use_ai`: Hero-Analyst und Item-Analyst fuellen `playstyle`,
   `AbilityRole`-Labels und `BuildItem.why`; Patch-Analyst fuellt die Delta-Notes;
   Meta-Analyst fuellt `rationale`.
7. Nur wenn `use_ai`: Kritiker prueft das BuildObject.

Abbruchregeln:
- KI-Aufruf schlaegt fehl oder `--no-ai`: Build wird ohne Texte ausgegeben, Zahlen
  bleiben, Rueckgabe ist gueltig. Kein Abbruch.
- Kritiker `recompose`: hoechstens eine erneute `compose_build`-Runde mit den
  Kritiker-Issues als Sperrliste (Items in `issues` werden abgewertet). Bleibt es
  bei `recompose`, wird das Build mit dem Vermerk der offenen Kritikpunkte
  ausgegeben, kein weiterer Loop.
- Item-Analyst nennt ein Item, das nicht im Build ist: Text wird verworfen, kein
  neuer Item kommt durch die KI ins Build (KI waehlt nie Items).

## 10. Build-Objekt und Abbildung auf den Publish-Payload

Der bestehende Steam-Bot-Handler `BuildPublishOriginalArgs` (publish_original.rs)
akzeptiert bereits pro Item ueber `ModSpec::Full`: `ability_id` (Alias `item_id`),
`annotation`, `required_flex_slots`, `imbue` (Proto-Feld 5) und `sell_priority`
(Proto-Feld 4), pro Kategorie zusaetzlich `description`, `width`, `height`,
`optional`. Der Publish-Payload-Typ in `dbrain-builds/src/spec.rs`
(`BuildSpecPayload`, `BuildSpecCategory`, `BuildSpecMod`) nutzt davon nur
`ability_id` und `annotation`.

Aenderung in `dbrain-builds/src/spec.rs` (im Brain-Repo, erlaubt):
`BuildSpecMod` um `imbue: Option<i64>` und `sell_priority: Option<u32>` erweitern
(beide `skip_serializing_if = "Option::is_none"`), `BuildSpecCategory` um
`description: Option<String>`, `width: Option<f64>`, `height: Option<f64>`. Damit
tragen Imbue und Verkaufsreihenfolge aus dem Referenz-Screenshot (Quicksilver
Reload und Mercurial Magnum sind Imbue-Items) durch. Der Steam-Bot-Code bleibt
lesend und unveraendert.

`publish::to_publish_payload` bildet `BuildObject` ab:
- `core` und jeder `SituationBlock` werden je eine `BuildSpecCategory`. Kern
  optional=false, Situationsbloecke optional gemaess `SituationKind`.
- `SituationKind::CanBuyN(n)` setzt den Kategorietitel `Can buy {n}`.
- `BuildItem.why` wird `annotation`, `imbue_target` wird `imbue`, `sell_priority`
  durchgereicht.
- Kategorie `width` mindestens 780, `height` 260 (Layout-Regel aus der Skill-Datei,
  sonst verschluckt der Client Kacheln).
- `ability_order` aus `BuildObject.ability_order`.

Der Publish-Weg selbst bleibt der bestehende: Payload als
`BUILD_PUBLISH_ORIGINAL`-Task an den Steam-Bot (`POST http://127.0.0.1:8782/tasks`).

## 11. Backtest

Kennzahlen je Held und Autor:
- **Kern-Ueberdeckung:** Anteil der Reasoner-Kernitems, die im Autoren-Kern
  vorkommen (Schnittmenge geteilt durch Reasoner-Kerngroesse), plus symmetrisch der
  Jaccard-Wert. Zielgroesse pro Held im Report.
- **Reihenfolge-Naehe:** normalisierte Rang-Verschiebung der gemeinsamen Items
  zwischen Reasoner-Buy-Order und Autoren-Buy-Order (0 = identisch, 1 = maximal
  verdreht), abgeleitet aus `avg_buy_time_relative` fuer die Reasoner-Seite.
- **Patch-Wechsel-Erkennung:** ueber zwei Patch-Staende hinweg pruefen, ob sich der
  Reasoner-Kern in dieselbe Richtung verschiebt wie der Autoren-Kern. Fuer Warden
  ist der Nightshift-Patch der Pflichtfall (tot vor Patch, Meta danach).

Vergleichsbasis: `tierlist.hero_build_sources` (107 Builds, `details` jsonb,
`version`, `published_at`, `last_updated_at`) und `tierlist.watched_build_authors`.
Patch-Stand je Autoren-Build wird ueber `last_updated_at` gegen `brain.patch_events`
zeitlich zugeordnet, weil `hero_build_sources` keinen Patch-Bezug traegt.

**Stand der Vergleichsbasis (aus Orchestrator-Nachtrag, in ARCHITEKTUR eingeplant):**
`watched_build_authors` hat 10 Autoren (Sanya Sniper, Cosmetical, ABL, Piggy,
Deathy, Heresy, JonJon69, Amerikanec, AverageJonas, einer ohne Namen), letzter Scan
2025-12-22, Status `partial`, `0 builds from 0 heroes`. Lightbringer und Situation
fehlen ganz. Der Autoren-Scan im Steam-Bot ist damit faktisch tot, und ohne frische
Autoren-Builds hat der Backtest keine Basis. Reaktivierung:
1. Steam-Bot-Autoren-Scan wieder laufen lassen (eigener Auftrag im
   Deadlock-Steam-Bot-Repo, hier nur lesend benannt).
2. Autoren ergaenzen, mindestens Lightbringer und Situation, damit der Warden-
   Pflichtfall eine Quelle hat.
3. Patch-Stand je Build ueber `last_updated_at` gegen `patch_events` ableiten.

Dieser Reaktivierungsteil traegt **Paket C** auf der Lese- und Ableitungsseite
(Zuordnung, Patch-Stand, Backtest-Metriken). Der Schreib-/Scan-Teil im Steam-Bot
ist ausdruecklich ein eigener Auftrag ausserhalb dieses Crates. Bis der Scan wieder
Daten liefert, laeuft der Backtest gegen die 107 vorhandenen `hero_build_sources`
und meldet fuer Warden mangels Autoren-Build ehrlich "keine Vergleichsbasis" statt
eine Zahl zu erfinden.

## 12. CLI

Neue Subcommand-Gruppe im Binary `deadlock-brain`
(`rust/crates/deadlock-brain/src/main.rs`, `enum Commands`), analog zu `Wiki`/`Learn`:

```
deadlock-brain reason build <hero> [--no-ai] [--patch <tag>] [--publish] [--json]
deadlock-brain reason patch-impact <hero> [--patch <tag>] [--json]
deadlock-brain reason backtest [--hero <hero>] [--patch <tag>] [--json]
```

- `build`: BuildObject, mit `--publish` Task an den Steam-Bot.
- `patch-impact`: PatchImpactReport.
- `backtest`: BacktestReport mit den drei Kennzahlen, Warden als Pflichtzeile.

## 13. Wiederverwendung und Ersatz

**Wiederverwenden (aufrufen oder heben, nicht neu bauen):**
- `deadlock_brain_core::ai` (AiClient, ChatMessage, ChatCompletionRequest,
  extract_ai_text): einziger AI-Transport fuer alle Rollen.
- `deadlock_brain_core::build_narration::narrate_build`: bestehender
  Narrations-Weg fuer Annotationstexte, statt einen zweiten zu bauen.
- `deadlock_brain_core::pg`: PgPool-Aufbau aus `DEADLOCK_CENTRAL_DSN`.
- `dbrain-builds::spec` (BuildSpecPayload, assemble_payload, AbilityOrderEntry) und
  der Publish-Weg: bleibt die Publish-Schnittstelle, nur `BuildSpecMod`/
  `BuildSpecCategory` erweitert (Abschnitt 10).
- `dbrain-builds::api::DeadlockApiClient` (item_stats, build_item_stats,
  hero_stats): deadlock-api-Signale fuer `meta.rs`.
- `dbrain-learn::build_optimizer`: `load_entity_payload`, `load_hero_abilities`,
  `infer_hero_needs`, `ability_damage_profile`, `ability_scaling_stats`,
  `infer_ability_role_tags`, `economy_summary` sind der bestehende deterministische
  Helden-Kontext. `hero::build_hero_model` baut darauf auf statt die Payload-
  Auswertung neu zu schreiben.
- `dbrain-enrich`: `load_patch_events`, `build_patch_event_enrichments`,
  `PatchImpactRequest` als Vorlage fuer `patch.rs`.
- `dbrain-retrieval`: Claims/Timeline fuer die Creator-Claims in `meta.rs`.

**Ersetzen (mit Begruendung):**
- `dbrain-builds::engine::composite_score` (engine.rs:332,
  `prevalence*0.50 + winrate*0.30 + lift*0.20`, drei starre Pfade spirit/weapon/tank
  in `fits_path`): das ist genau das Winrate-Ranking, das der Auftrag ablehnt. Ersatz
  ist das mechanische Scoring in `item.rs`/`mechanics.rs` mit Meta nur als
  Nebensignal. Der alte `engine`-Pfad und das `build`-Kommando bleiben unangetastet,
  bis der Reasoner den Backtest besteht; danach kann `build` auf den Reasoner zeigen.
- `dbrain-enrich::run_meta_trend_analysis` (lib.rs:789, Mock-Shifts,
  `meta_trend_notes` leer): ersetzt durch echtes Patch-Delta aus `patch_events` in
  `patch.rs`, keine Mock-Daten.

## 14. Offene Fragen

1. **`scale_function`/`EAddToScale` in der aktuellen Asset-Fassung.** Die Skill-Datei
   nennt `properties.<Wert>.scale_function.stat_scale` und den Stufen-Marker
   `upgrade_type: EAddToScale` als Angelpunkt des Builds. Im aktuellen Dump
   (`items.f16f8c8dd04ab5d9.json`) sind `properties`-Werte flach und `upgrades[]`
   traegt `property_upgrades[] = {bonus, name}` ohne sichtbares `scale_function`.
   *Empfehlung:* Paket B liest die Skalierungsstufe aus `property_upgrades[].name`
   (Treffer auf einen Skalierungsstat) und faellt sonst auf `scaling_stats` des
   Helden zurueck; der Implementierer verifiziert am echten Snapshot, ob
   `scale_function` in der DB-Fassung vorhanden ist, bevor er sich darauf verlaesst.
2. **DB-Zugang in dieser Session.** Der Infisical-Service-Token lag hier nicht vor
   (`DEADLOCK_CENTRAL_DSN` nicht ableitbar), Feldnamen wurden gegen die lokalen
   Rohdateien und `engine.rs` geprueft. *Empfehlung:* Paket A verifiziert
   `hero_stat_values`- und `hero_item_synergies`-Spalten einmalig am echten
   Schema, bevor `data.rs` festgezurrt wird.
3. **Kaufzeitpunkte der Autoren-Builds.** `hero_build_sources.details` enthaelt die
   Item-Liste, aber der Buy-Order-Zeitpunkt der Autoren ist unklar (kein
   Zeitstempel je Item). *Empfehlung:* Reihenfolge-Naehe auf der Autoren-Seite aus
   der Reihenfolge im `details`-Array ableiten und diese Annahme im Backtest-Report
   sichtbar machen; wenn `details` keine Reihenfolge traegt, die Kennzahl fuer
   diesen Autor als "nicht messbar" melden.
4. **Warden-Vergleichsbasis fehlt.** Bis der Autoren-Scan reaktiviert ist, gibt es
   fuer den Pflichtfall Warden keinen Autoren-Build. *Empfehlung:* den
   Lightbringer-Warden-Screenshot einmalig als manuellen Referenz-Build in
   `tierlist.hero_build_sources` oder eine kleine Seed-Datei einpflegen, damit der
   Backtest den Pflichtfall vor der Scan-Reaktivierung fahren kann.
