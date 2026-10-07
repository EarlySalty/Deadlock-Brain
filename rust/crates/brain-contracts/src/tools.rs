use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::entity_profile::EntityKind;
use crate::{AuthorizedContext, Evidence, PortError, ProviderAnswer, Query, Usage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolName {
    EntityFind,
    EntityProfile,
    HeroCompare,
    DamageCalculate,
    PatchHistory,
    BuildPlan,
    GameRules,
    ServerKnowledge,
}

impl ToolName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntityFind => "entity_find",
            Self::EntityProfile => "entity_profile",
            Self::HeroCompare => "hero_compare",
            Self::DamageCalculate => "damage_calculate",
            Self::PatchHistory => "patch_history",
            Self::BuildPlan => "build_plan",
            Self::GameRules => "game_rules",
            Self::ServerKnowledge => "server_knowledge",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolDefinition {
    pub name: ToolName,
    pub description: String,
    pub input_schema: Value,
}

impl ToolDefinition {
    pub fn validate(&self) -> Result<(), PortError> {
        if self.input_schema["type"] != "object" {
            return Err(invalid("Werkzeugschema benötigt ein geschlossenes Objekt"));
        }
        validate_schema(&self.input_schema)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCall {
    pub id: String,
    pub name: ToolName,
    pub arguments: Value,
}

impl ToolCall {
    pub fn validate(&self, definitions: &[ToolDefinition]) -> Result<ToolRequest, PortError> {
        validate_id(&self.id)?;
        validate_definitions(definitions)?;
        let definition = definitions
            .iter()
            .find(|definition| definition.name == self.name)
            .ok_or_else(|| invalid("Werkzeug ist nicht freigegeben"))?;
        validate_arguments(&self.arguments, &definition.input_schema)?;
        let subrequest = ToolSubrequest::parse(self.name, &self.arguments)?;
        subrequest.validate()?;
        Ok(ToolRequest {
            name: self.name,
            arguments: self.arguments.clone(),
            subrequest,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolLanguage {
    German,
    English,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolEntityRef {
    pub kind: EntityKind,
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ToolProgression {
    Souls(u64),
    Boons(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ToolItemTransition {
    Buy { item_id: u64 },
    Sell { item_id: u64 },
    Upgrade { from_item_id: u64, to_item_id: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolImbue {
    pub item_id: u64,
    pub ability_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolAbilityRank {
    pub ability_id: u64,
    pub rank: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolDistanceUnit {
    GameUnits,
    Meters,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolDistance {
    pub value: f64,
    pub unit: ToolDistanceUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolTargetKind {
    Player,
    Npc,
    Objective,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolTargetValues {
    pub health: Option<f64>,
    pub health_regen: Option<f64>,
    pub bullet_shield: Option<f64>,
    pub spirit_shield: Option<f64>,
    pub bullet_resist: Option<f64>,
    pub spirit_resist: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolTargetState {
    pub at_seconds: f64,
    pub values: ToolTargetValues,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolTarget {
    pub kind: ToolTargetKind,
    pub values: ToolTargetValues,
    pub distance: Option<ToolDistance>,
    pub hit_chance: Option<f64>,
    pub headshot_fraction: Option<f64>,
    #[serde(default)]
    pub states: Vec<ToolTargetState>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolScenario {
    pub progression: ToolProgression,
    pub level: Option<u32>,
    pub ability_points: Option<u32>,
    pub total_spirit: Option<f64>,
    #[serde(default)]
    pub item_ids: Vec<u64>,
    #[serde(default)]
    pub item_transitions: Vec<ToolItemTransition>,
    #[serde(default)]
    pub imbues: Vec<ToolImbue>,
    #[serde(default)]
    pub ability_ranks: Vec<ToolAbilityRank>,
    pub target: Option<ToolTarget>,
    pub horizon_seconds: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityFindRequest {
    pub query: String,
    pub kind: Option<EntityKind>,
    pub language: ToolLanguage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolAnalyticsSelection {
    pub min_average_badge: Option<u32>,
    pub max_average_badge: Option<u32>,
    pub min_unix_timestamp: i64,
    pub max_unix_timestamp: i64,
}

impl ToolAnalyticsSelection {
    fn validate(&self) -> Result<(), PortError> {
        if self
            .min_average_badge
            .into_iter()
            .chain(self.max_average_badge)
            .any(|badge| badge > 116)
            || self
                .min_average_badge
                .zip(self.max_average_badge)
                .is_some_and(|(min, max)| min > max)
        {
            return Err(invalid("Ungültiger Analytics-Rangbereich"));
        }
        if self.min_unix_timestamp < 0 || self.max_unix_timestamp <= self.min_unix_timestamp {
            return Err(invalid("Ungültiger Analytics-Zeitraum"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityProfileRequest {
    pub entity: ToolEntityRef,
    pub fields: Vec<String>,
    pub scenario: Option<ToolScenario>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analytics: Option<ToolAnalyticsSelection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolRankPopulation {
    ActiveHeroes,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolBoonRange {
    pub min_boons: u32,
    pub max_boons: u32,
}

impl ToolBoonRange {
    fn validate(&self) -> Result<(), PortError> {
        if self.min_boons > self.max_boons {
            return Err(invalid("Ungültiger Boonbereich"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroCompareRequest {
    pub hero_ids: Vec<u64>,
    pub metrics: Vec<String>,
    pub scenario: ToolScenario,
    pub ranking_population: Option<ToolRankPopulation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boon_range: Option<ToolBoonRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analytics: Option<ToolAnalyticsSelection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DamageCalculateRequest {
    pub hero_id: u64,
    pub scenario: ToolScenario,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchHistoryRequest {
    pub entity: ToolEntityRef,
    pub ability_id: Option<u64>,
    #[serde(default)]
    pub fields: Vec<String>,
    pub from_patch: Option<String>,
    pub to_patch: Option<String>,
    #[serde(default)]
    pub historical_client_versions: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolPlaystyle {
    Weapon,
    Spirit,
    Tank,
}

impl ToolPlaystyle {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Weapon => "weapon",
            Self::Spirit => "spirit",
            Self::Tank => "tank",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildPlanRequest {
    pub hero_id: u64,
    pub playstyle: ToolPlaystyle,
    pub budget: Option<u64>,
    #[serde(default)]
    pub imbues: Vec<ToolImbue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolGameRuleTopic {
    KillBounty,
    Comeback,
    Urn,
    Midboss,
    ResistStacking,
    Resources,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameRulesRequest {
    pub topic: ToolGameRuleTopic,
    pub entity: Option<ToolEntityRef>,
    pub game_time_seconds: Option<f64>,
    pub scenario: Option<ToolScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerKnowledgeRequest {
    pub question: String,
    pub public_channel_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "name", content = "arguments", rename_all = "snake_case")]
pub enum ToolSubrequest {
    EntityFind(EntityFindRequest),
    EntityProfile(EntityProfileRequest),
    HeroCompare(HeroCompareRequest),
    DamageCalculate(DamageCalculateRequest),
    PatchHistory(PatchHistoryRequest),
    BuildPlan(BuildPlanRequest),
    GameRules(GameRulesRequest),
    ServerKnowledge(ServerKnowledgeRequest),
}

impl ToolSubrequest {
    fn parse(name: ToolName, arguments: &Value) -> Result<Self, PortError> {
        let parsed = match name {
            ToolName::EntityFind => serde_json::from_value(arguments.clone()).map(Self::EntityFind),
            ToolName::EntityProfile => {
                serde_json::from_value(arguments.clone()).map(Self::EntityProfile)
            }
            ToolName::HeroCompare => {
                serde_json::from_value(arguments.clone()).map(Self::HeroCompare)
            }
            ToolName::DamageCalculate => {
                serde_json::from_value(arguments.clone()).map(Self::DamageCalculate)
            }
            ToolName::PatchHistory => {
                serde_json::from_value(arguments.clone()).map(Self::PatchHistory)
            }
            ToolName::BuildPlan => serde_json::from_value(arguments.clone()).map(Self::BuildPlan),
            ToolName::GameRules => serde_json::from_value(arguments.clone()).map(Self::GameRules),
            ToolName::ServerKnowledge => {
                serde_json::from_value(arguments.clone()).map(Self::ServerKnowledge)
            }
        };
        parsed.map_err(|_| invalid("Werkzeugargumente passen nicht zur typisierten Unteranfrage"))
    }

    fn validate(&self) -> Result<(), PortError> {
        match self {
            Self::EntityFind(request) => validate_text(&request.query),
            Self::EntityProfile(request) => {
                validate_entity(&request.entity)?;
                validate_fields(&request.fields, true)?;
                if let Some(analytics) = &request.analytics {
                    analytics.validate()?;
                }
                request
                    .scenario
                    .as_ref()
                    .map_or(Ok(()), ToolScenario::validate)
            }
            Self::HeroCompare(request) => {
                validate_numeric_ids(&request.hero_ids)?;
                if request.hero_ids.len() < 2 {
                    return Err(invalid("Heldenvergleich benötigt mindestens zwei Helden"));
                }
                validate_fields(&request.metrics, true)?;
                if let Some(range) = &request.boon_range {
                    range.validate()?;
                }
                if let Some(analytics) = &request.analytics {
                    analytics.validate()?;
                }
                request.scenario.validate()
            }
            Self::DamageCalculate(request) => {
                validate_numeric_id(request.hero_id)?;
                request.scenario.validate()
            }
            Self::PatchHistory(request) => {
                validate_entity(&request.entity)?;
                if let Some(id) = request.ability_id {
                    validate_numeric_id(id)?;
                }
                validate_fields(&request.fields, false)?;
                for patch in request.from_patch.iter().chain(&request.to_patch) {
                    validate_text(patch)?;
                }
                let mut versions = BTreeSet::new();
                for version in &request.historical_client_versions {
                    if *version <= 0 || !versions.insert(version) {
                        return Err(invalid("Ungültige oder doppelte historische Version"));
                    }
                }
                Ok(())
            }
            Self::BuildPlan(request) => {
                validate_numeric_id(request.hero_id)?;
                if request.budget == Some(0) {
                    return Err(invalid("Buildbudget muss positiv sein"));
                }
                validate_imbues(&request.imbues)
            }
            Self::GameRules(request) => {
                if let Some(entity) = &request.entity {
                    validate_entity(entity)?;
                }
                validate_optional_number(request.game_time_seconds, true)?;
                request
                    .scenario
                    .as_ref()
                    .map_or(Ok(()), ToolScenario::validate)
            }
            Self::ServerKnowledge(request) => {
                validate_text(&request.question)?;
                if let Some(id) = request.public_channel_id {
                    validate_numeric_id(id)?;
                }
                Ok(())
            }
        }
    }
}

impl ToolScenario {
    fn validate(&self) -> Result<(), PortError> {
        if self.level == Some(0) {
            return Err(invalid("Heldenlevel muss positiv sein"));
        }
        validate_optional_number(self.total_spirit, true)?;
        validate_optional_number(self.horizon_seconds, true)?;
        if self.horizon_seconds == Some(0.0) {
            return Err(invalid("Auswertungshorizont muss positiv sein"));
        }
        validate_numeric_ids(&self.item_ids)?;
        validate_imbues(&self.imbues)?;
        for transition in &self.item_transitions {
            match transition {
                ToolItemTransition::Buy { item_id } | ToolItemTransition::Sell { item_id } => {
                    validate_numeric_id(*item_id)?
                }
                ToolItemTransition::Upgrade {
                    from_item_id,
                    to_item_id,
                } => {
                    validate_numeric_id(*from_item_id)?;
                    validate_numeric_id(*to_item_id)?;
                    if from_item_id == to_item_id {
                        return Err(invalid("Itemupgrade benötigt verschiedene Items"));
                    }
                }
            }
        }
        let mut abilities = BTreeSet::new();
        for rank in &self.ability_ranks {
            validate_numeric_id(rank.ability_id)?;
            if !abilities.insert(rank.ability_id) {
                return Err(invalid("Doppelter Fähigkeitsrang"));
            }
        }
        if let Some(target) = &self.target {
            target.values.validate()?;
            if let Some(distance) = &target.distance {
                validate_optional_number(Some(distance.value), true)?;
            }
            for fraction in [target.hit_chance, target.headshot_fraction]
                .into_iter()
                .flatten()
            {
                if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
                    return Err(invalid("Trefferanteil muss zwischen null und eins liegen"));
                }
            }
            let mut previous = None;
            for state in &target.states {
                validate_optional_number(Some(state.at_seconds), true)?;
                if previous.is_some_and(|time| state.at_seconds <= time)
                    || self
                        .horizon_seconds
                        .is_some_and(|horizon| state.at_seconds > horizon)
                {
                    return Err(invalid("Ungültige Reihenfolge oder Zeit des Zielzustands"));
                }
                previous = Some(state.at_seconds);
                state.values.validate()?;
            }
        }
        Ok(())
    }
}

impl ToolTargetValues {
    fn validate(&self) -> Result<(), PortError> {
        for value in [
            self.health,
            self.health_regen,
            self.bullet_shield,
            self.spirit_shield,
        ] {
            validate_optional_number(value, true)?;
        }
        for value in [self.bullet_resist, self.spirit_resist] {
            validate_optional_number(value, false)?;
        }
        Ok(())
    }
}

fn validate_optional_number(value: Option<f64>, nonnegative: bool) -> Result<(), PortError> {
    if value.is_some_and(|value| !value.is_finite() || (nonnegative && value < 0.0)) {
        return Err(invalid("Ungültige Szenariozahl"));
    }
    Ok(())
}

fn validate_numeric_id(id: u64) -> Result<(), PortError> {
    if id == 0 || id > i64::MAX as u64 {
        return Err(invalid("Ungültige Entitäts-ID"));
    }
    Ok(())
}

fn validate_numeric_ids(ids: &[u64]) -> Result<(), PortError> {
    let mut seen = BTreeSet::new();
    for id in ids {
        validate_numeric_id(*id)?;
        if !seen.insert(id) {
            return Err(invalid("Doppelte Entitäts-ID"));
        }
    }
    Ok(())
}

fn validate_entity(entity: &ToolEntityRef) -> Result<(), PortError> {
    validate_numeric_id(entity.id)
}

fn validate_imbues(imbues: &[ToolImbue]) -> Result<(), PortError> {
    let mut items = BTreeSet::new();
    for imbue in imbues {
        validate_numeric_id(imbue.item_id)?;
        validate_numeric_id(imbue.ability_id)?;
        if !items.insert(imbue.item_id) {
            return Err(invalid("Doppelte Itembindung"));
        }
    }
    Ok(())
}

fn validate_text(text: &str) -> Result<(), PortError> {
    if text.trim().is_empty() || text.len() > 32_768 {
        return Err(invalid("Ungültiger Werkzeugtext"));
    }
    Ok(())
}

fn validate_fields(fields: &[String], required: bool) -> Result<(), PortError> {
    if required && fields.is_empty() {
        return Err(invalid("Werkzeugfelder fehlen"));
    }
    validate_ids(fields)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ToolRequest {
    name: ToolName,
    arguments: Value,
    #[serde(skip)]
    subrequest: ToolSubrequest,
}

impl ToolRequest {
    pub fn name(&self) -> ToolName {
        self.name
    }

    pub fn arguments(&self) -> &Value {
        &self.arguments
    }

    pub fn subrequest(&self) -> &ToolSubrequest {
        &self.subrequest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolResult {
    pub call_id: String,
    pub name: ToolName,
    pub result: Value,
    pub evidence_ids: Vec<String>,
    pub is_error: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ModelBlock {
    Text { text: String },
    ToolUse { call: ToolCall },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFinishReason {
    EndTurn,
    ToolUse,
    MaxTokens,
    Refusal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProviderTurn {
    Final {
        answer: ProviderAnswer,
        finish_reason: ProviderFinishReason,
    },
    ToolCalls {
        blocks: Vec<ModelBlock>,
        finish_reason: ProviderFinishReason,
        usage: Usage,
    },
}

impl From<ProviderAnswer> for ProviderTurn {
    fn from(answer: ProviderAnswer) -> Self {
        Self::Final {
            answer,
            finish_reason: ProviderFinishReason::EndTurn,
        }
    }
}

impl ProviderTurn {
    pub fn usage(&self) -> &Usage {
        match self {
            Self::Final { answer, .. } => &answer.usage,
            Self::ToolCalls { usage, .. } => usage,
        }
    }

    pub fn validate(&self, definitions: &[ToolDefinition]) -> Result<(), PortError> {
        validate_definitions(definitions)?;
        match self {
            Self::Final {
                answer,
                finish_reason: ProviderFinishReason::EndTurn,
            } => validate_ids(&answer.cited_evidence_ids),
            Self::ToolCalls {
                blocks,
                finish_reason: ProviderFinishReason::ToolUse,
                ..
            } => validate_calls(blocks, definitions).map(|_| ()),
            _ => Err(invalid("Abschlussgrund passt nicht zum Providerturn")),
        }
    }

    pub fn into_answer(self) -> Result<ProviderAnswer, PortError> {
        match self {
            Self::Final {
                answer,
                finish_reason: ProviderFinishReason::EndTurn,
            } => {
                validate_ids(&answer.cited_evidence_ids)?;
                Ok(answer)
            }
            _ => Err(invalid(
                "Werkzeugaufruf oder unvollständiger Turn ist keine Antwort",
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "snake_case", deny_unknown_fields)]
pub enum ToolMessage {
    Assistant { blocks: Vec<ModelBlock> },
    ToolResults { results: Vec<ToolResult> },
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolConversation {
    pub messages: Vec<ToolMessage>,
}

impl ToolConversation {
    pub fn validate(&self, definitions: &[ToolDefinition]) -> Result<(), PortError> {
        validate_definitions(definitions)?;
        let mut seen = BTreeSet::new();
        let mut pending = BTreeMap::new();
        for message in &self.messages {
            match message {
                ToolMessage::Assistant { blocks } if pending.is_empty() => {
                    for call in validate_calls(blocks, definitions)? {
                        if !seen.insert(call.id.as_str()) {
                            return Err(invalid("Doppelte Werkzeugaufruf-ID"));
                        }
                        pending.insert(call.id.as_str(), call.name);
                    }
                }
                ToolMessage::ToolResults { results } if !pending.is_empty() => {
                    for result in results {
                        if pending.remove(result.call_id.as_str()) != Some(result.name) {
                            return Err(invalid("Werkzeugergebnis gehört nicht zum Aufruf"));
                        }
                        validate_ids(&result.evidence_ids)?;
                    }
                    if !pending.is_empty() {
                        return Err(invalid("Werkzeugergebnis fehlt"));
                    }
                }
                _ => return Err(invalid("Ungültige Werkzeuggesprächsfolge")),
            }
        }
        if !pending.is_empty() {
            return Err(invalid("Werkzeugergebnis fehlt"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PinnedGameContext {
    pub client_version: i64,
    pub language: ToolLanguage,
    pub mechanic_revision: String,
}

impl PinnedGameContext {
    pub fn validate(&self) -> Result<(), PortError> {
        if self.client_version <= 0 {
            return Err(invalid("Ungültige gebundene Spielversion"));
        }
        validate_id(&self.mechanic_revision)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ToolEvidenceDependency {
    pub request: ToolRequest,
    pub game_context: Option<PinnedGameContext>,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ToolExecution {
    pub result: ToolResult,
    pub dependencies: Vec<ToolEvidenceDependency>,
    pub usage: Usage,
}

impl ToolExecution {
    pub fn validate_for(
        &self,
        call: &ToolCall,
        request: &ToolRequest,
        game_context: Option<&PinnedGameContext>,
    ) -> Result<(), PortError> {
        let expected_game = if request.name() == ToolName::ServerKnowledge {
            None
        } else {
            let game = game_context.ok_or_else(|| invalid("Gebundene Spielversion fehlt"))?;
            game.validate()?;
            if let ToolSubrequest::EntityFind(find) = request.subrequest() {
                if find.language != game.language {
                    return Err(invalid("Werkzeugsprache weicht von der Anfragebindung ab"));
                }
            }
            Some(game)
        };
        if self.result.call_id != call.id
            || self.result.name != call.name
            || request.name != call.name
            || request.arguments != call.arguments
        {
            return Err(invalid("Werkzeugausführung gehört nicht zur Unteranfrage"));
        }
        validate_id(&self.result.call_id)?;
        validate_ids(&self.result.evidence_ids)?;
        let mut ids = BTreeSet::new();
        for dependency in &self.dependencies {
            if &dependency.request != request || dependency.game_context.as_ref() != expected_game {
                return Err(invalid(
                    "Belegabhängigkeit gehört nicht zur gebundenen Unteranfrage",
                ));
            }
            for evidence in &dependency.evidence {
                validate_id(&evidence.evidence_id)?;
                validate_id(&evidence.source_id)?;
                validate_id(&evidence.logical_id)?;
                evidence
                    .validate()
                    .map_err(|_| invalid("Ungültige Werkzeugbelegabhängigkeit"))?;
                if !ids.insert(evidence.evidence_id.as_str()) {
                    return Err(invalid("Doppelte Werkzeugbelegabhängigkeit"));
                }
            }
        }
        if self
            .result
            .evidence_ids
            .iter()
            .any(|id| !ids.contains(id.as_str()))
        {
            return Err(invalid("Werkzeugbeleg hat keine Unteranfrageabhängigkeit"));
        }
        if !self.result.is_error && ids.is_empty() {
            return Err(invalid("Werkzeugausführung hat keine Belegabhängigkeit"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolValidationPurpose {
    Provider,
    Publication,
    Cache,
}

pub trait ToolExecutionPort: Send + Sync {
    fn definitions(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _game_context: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        Err(PortError::Unavailable("Werkzeugdefinitionen fehlen".into()))
    }

    fn execute(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        game_context: Option<&PinnedGameContext>,
        call_id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError>;

    fn validate_dependencies(
        &self,
        _query: &Query,
        _context: &AuthorizedContext,
        _game_context: Option<&PinnedGameContext>,
        _dependencies: &[ToolEvidenceDependency],
        _purpose: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        Err(PortError::Unavailable(
            "Kanonische Werkzeugbelegprüfung fehlt".into(),
        ))
    }
}

pub fn validate_definitions(definitions: &[ToolDefinition]) -> Result<(), PortError> {
    let mut names = BTreeSet::new();
    for definition in definitions {
        definition.validate()?;
        if !names.insert(definition.name) {
            return Err(invalid("Doppelte Werkzeugdefinition"));
        }
    }
    Ok(())
}

fn validate_calls<'a>(
    blocks: &'a [ModelBlock],
    definitions: &[ToolDefinition],
) -> Result<Vec<&'a ToolCall>, PortError> {
    let mut calls = Vec::new();
    let mut ids = BTreeSet::new();
    for block in blocks {
        if let ModelBlock::ToolUse { call } = block {
            call.validate(definitions)?;
            if !ids.insert(call.id.as_str()) {
                return Err(invalid("Doppelte Werkzeugaufruf-ID"));
            }
            calls.push(call);
        }
    }
    if calls.is_empty() {
        return Err(invalid("Werkzeugturn enthält keinen Aufruf"));
    }
    Ok(calls)
}

pub(crate) fn validate_id(id: &str) -> Result<(), PortError> {
    if id.trim().is_empty() || id.len() > 512 || id.chars().any(char::is_control) {
        return Err(invalid("Ungültige Werkzeug-ID"));
    }
    Ok(())
}

fn validate_ids(ids: &[String]) -> Result<(), PortError> {
    let mut seen = BTreeSet::new();
    for id in ids {
        validate_id(id)?;
        if !seen.insert(id) {
            return Err(invalid("Doppelte Beleg-ID"));
        }
    }
    Ok(())
}

pub(crate) fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}

fn forbidden_argument(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "actor"
            | "actor_id"
            | "principal"
            | "context"
            | "authorized_context"
            | "scopes"
            | "requested_scopes"
            | "provider_egress"
            | "knowledge_release"
            | "release"
            | "release_id"
            | "client_version"
            | "mechanic_revision"
            | "patch_membership"
            | "version"
            | "deadline"
            | "deadline_ms"
            | "request_deadline"
            | "request_id"
            | "conversation_id"
            | "dsn"
            | "url"
            | "urls"
            | "sql"
            | "provider"
            | "model"
            | "endpoint"
            | "base_url"
            | "expression"
            | "expressions"
    )
}

fn validate_schema(schema: &Value) -> Result<(), PortError> {
    let object = schema
        .as_object()
        .ok_or_else(|| invalid("Ungültiges Werkzeugschema"))?;
    let kind = schema["type"]
        .as_str()
        .ok_or_else(|| invalid("Schematyp fehlt"))?;
    for (key, value) in object {
        let valid = match key.as_str() {
            "type" => true,
            "description" | "title" => value.is_string(),
            "enum" => value.as_array().is_some_and(|values| !values.is_empty()),
            "properties" | "required" | "additionalProperties" => kind == "object",
            "items" | "minItems" | "maxItems" => kind == "array",
            "minLength" | "maxLength" => kind == "string",
            "minimum" | "maximum" => kind == "integer" || kind == "number",
            _ => false,
        };
        if !valid {
            return Err(invalid("Nicht unterstütztes Werkzeugschemafeld"));
        }
    }
    match kind {
        "object" => {
            if schema["additionalProperties"] != false {
                return Err(invalid("Werkzeugschema muss unbekannte Felder ablehnen"));
            }
            let properties = schema["properties"]
                .as_object()
                .ok_or_else(|| invalid("Schemafelder fehlen"))?;
            for (name, property) in properties {
                if forbidden_argument(name) {
                    return Err(invalid(
                        "Serverbindung oder ausführbares Argument im Werkzeugschema",
                    ));
                }
                validate_schema(property)?;
            }
            if let Some(required) = schema.get("required") {
                let mut seen = BTreeSet::new();
                for name in required
                    .as_array()
                    .ok_or_else(|| invalid("Ungültige Schemapflichtfelder"))?
                {
                    let name = name
                        .as_str()
                        .ok_or_else(|| invalid("Ungültiges Schemapflichtfeld"))?;
                    if !properties.contains_key(name) || !seen.insert(name) {
                        return Err(invalid("Unbekanntes oder doppeltes Schemapflichtfeld"));
                    }
                }
            }
        }
        "array" => validate_schema(&schema["items"])?,
        "string" | "integer" | "number" | "boolean" | "null" => {}
        _ => return Err(invalid("Nicht unterstützter Werkzeugschematyp")),
    }
    for (min, max) in [("minLength", "maxLength"), ("minItems", "maxItems")] {
        for key in [min, max] {
            if schema
                .get(key)
                .is_some_and(|value| value.as_u64().is_none())
            {
                return Err(invalid("Ungültige Schemalängengrenze"));
            }
        }
        if let (Some(min), Some(max)) = (schema[min].as_u64(), schema[max].as_u64()) {
            if min > max {
                return Err(invalid("Widersprüchliche Schemalängengrenzen"));
            }
        }
    }
    for key in ["minimum", "maximum"] {
        if schema.get(key).is_some_and(|value| {
            if kind == "integer" {
                integer(value).is_none()
            } else {
                !value.as_f64().is_some_and(f64::is_finite)
            }
        }) {
            return Err(invalid("Ungültige Schemazahlengrenze"));
        }
    }
    let contradictory = if kind == "integer" {
        match (integer(&schema["minimum"]), integer(&schema["maximum"])) {
            (Some(min), Some(max)) => min > max,
            _ => false,
        }
    } else {
        match (schema["minimum"].as_f64(), schema["maximum"].as_f64()) {
            (Some(min), Some(max)) => min > max,
            _ => false,
        }
    };
    if contradictory {
        return Err(invalid("Widersprüchliche Schemazahlengrenzen"));
    }
    if let Some(values) = schema.get("enum").and_then(Value::as_array) {
        let mut base = schema.clone();
        base.as_object_mut()
            .ok_or_else(|| invalid("Ungültiges Werkzeugschema"))?
            .remove("enum");
        for (index, value) in values.iter().enumerate() {
            if values[..index].contains(value) {
                return Err(invalid("Doppelter Schemawert"));
            }
            validate_arguments(value, &base)?;
        }
    }
    Ok(())
}

fn integer(value: &Value) -> Option<i128> {
    value
        .as_i64()
        .map(i128::from)
        .or_else(|| value.as_u64().map(i128::from))
}

fn validate_arguments(value: &Value, schema: &Value) -> Result<(), PortError> {
    if schema.get("enum").is_some_and(|allowed| {
        !allowed
            .as_array()
            .is_some_and(|values| values.contains(value))
    }) {
        return Err(invalid(
            "Werkzeugargument außerhalb des erlaubten Wertsatzes",
        ));
    }
    let length = match schema["type"].as_str() {
        Some("object") => {
            let object = value
                .as_object()
                .ok_or_else(|| invalid("Werkzeugargument ist kein Objekt"))?;
            let properties = schema["properties"]
                .as_object()
                .ok_or_else(|| invalid("Schemafelder fehlen"))?;
            for (name, value) in object {
                let property = properties
                    .get(name)
                    .ok_or_else(|| invalid("Unbekanntes Werkzeugargument"))?;
                validate_arguments(value, property)?;
            }
            if let Some(required) = schema["required"].as_array() {
                if required
                    .iter()
                    .any(|name| !object.contains_key(name.as_str().unwrap_or_default()))
                {
                    return Err(invalid("Werkzeugpflichtargument fehlt"));
                }
            }
            None
        }
        Some("array") => {
            let array = value
                .as_array()
                .ok_or_else(|| invalid("Werkzeugargument ist keine Liste"))?;
            for item in array {
                validate_arguments(item, &schema["items"])?;
            }
            Some((array.len() as u64, "minItems", "maxItems"))
        }
        Some("string") => Some((
            value
                .as_str()
                .ok_or_else(|| invalid("Werkzeugargument ist kein Text"))?
                .chars()
                .count() as u64,
            "minLength",
            "maxLength",
        )),
        Some("integer") => {
            let number =
                integer(value).ok_or_else(|| invalid("Werkzeugargument ist keine Ganzzahl"))?;
            if integer(&schema["minimum"]).is_some_and(|min| number < min)
                || integer(&schema["maximum"]).is_some_and(|max| number > max)
            {
                return Err(invalid("Werkzeugzahl außerhalb der Schemagrenzen"));
            }
            None
        }
        Some("number") => {
            let number = value
                .as_f64()
                .filter(|number| number.is_finite())
                .ok_or_else(|| invalid("Werkzeugargument ist keine endliche Zahl"))?;
            if schema["minimum"].as_f64().is_some_and(|min| number < min)
                || schema["maximum"].as_f64().is_some_and(|max| number > max)
            {
                return Err(invalid("Werkzeugzahl außerhalb der Schemagrenzen"));
            }
            None
        }
        Some("boolean") if value.is_boolean() => None,
        Some("null") if value.is_null() => None,
        _ => return Err(invalid("Werkzeugargument passt nicht zum Schematyp")),
    };
    if let Some((length, min, max)) = length {
        if schema[min].as_u64().is_some_and(|min| length < min)
            || schema[max].as_u64().is_some_and(|max| length > max)
        {
            return Err(invalid(
                "Werkzeugargument außerhalb der Schemalängengrenzen",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    pub(super) fn definition() -> ToolDefinition {
        ToolDefinition {
            name: ToolName::EntityFind,
            description: "Entität suchen".into(),
            input_schema: json!({"type":"object","properties":{"query":{"type":"string","minLength":1},"language":{"type":"string","enum":["german","english"]}},"required":["query","language"],"additionalProperties":false}),
        }
    }

    fn call() -> ToolCall {
        ToolCall {
            id: "call-1".into(),
            name: ToolName::EntityFind,
            arguments: json!({"query":"Warden","language":"german"}),
        }
    }

    fn closed_schema(value: &Value) -> Value {
        match value {
            Value::Object(fields) => {
                let properties: serde_json::Map<_, _> = fields
                    .iter()
                    .map(|(name, value)| (name.clone(), closed_schema(value)))
                    .collect();
                json!({"type":"object", "properties":properties, "required":fields.keys().collect::<Vec<_>>(), "additionalProperties":false})
            }
            Value::Array(values) => {
                json!({"type":"array", "items":closed_schema(&values[0])})
            }
            Value::String(_) => json!({"type":"string"}),
            Value::Number(number) if number.is_i64() || number.is_u64() => {
                json!({"type":"integer"})
            }
            Value::Number(_) => json!({"type":"number"}),
            Value::Bool(_) => json!({"type":"boolean"}),
            Value::Null => json!({"type":"null"}),
        }
    }

    fn evidence() -> Evidence {
        Evidence {
            evidence_id: "e1".into(),
            source_id: "assets".into(),
            logical_id: "hero/warden".into(),
            revision: 1,
            kind: crate::EvidenceKind::Fact,
            content: "Warden".into(),
            citation: "assets:hero/warden".into(),
            visibility: crate::SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            score: 1.0,
            provenance: None,
            patch: None,
        }
    }

    #[test]
    fn sieben_typisierte_unteranfragen_aus_validierten_aufrufen() {
        let scenario = json!({"progression":{"kind":"boons","value":35},"total_spirit":38.0});
        for (name, arguments) in [
            (
                ToolName::EntityFind,
                json!({"query":"Warden","language":"german"}),
            ),
            (
                ToolName::EntityProfile,
                json!({"entity":{"kind":"hero","id":1},"fields":["weapon_dps"],"scenario":scenario}),
            ),
            (
                ToolName::HeroCompare,
                json!({"hero_ids":[1,2],"metrics":["weapon_dps"],"scenario":scenario,"ranking_population":"active_heroes"}),
            ),
            (
                ToolName::DamageCalculate,
                json!({"hero_id":1,"scenario":scenario}),
            ),
            (
                ToolName::PatchHistory,
                json!({"entity":{"kind":"ability","id":2},"historical_client_versions":[6758,6759]}),
            ),
            (
                ToolName::BuildPlan,
                json!({"hero_id":1,"playstyle":"weapon","budget":10000,"imbues":[{"item_id":3,"ability_id":2}]}),
            ),
            (
                ToolName::ServerKnowledge,
                json!({"question":"Welche Lanes sind offen?","public_channel_id":4}),
            ),
        ] {
            let definition = ToolDefinition {
                name,
                description: name.as_str().into(),
                input_schema: closed_schema(&arguments),
            };
            let call = ToolCall {
                id: "typed-call".into(),
                name,
                arguments,
            };
            let request = call.validate(&[definition]).unwrap();
            assert_eq!(request.name(), name);
            assert_eq!(request.arguments(), &call.arguments);
            match (name, request.subrequest()) {
                (ToolName::EntityFind, ToolSubrequest::EntityFind(request)) => {
                    assert_eq!(request.language, ToolLanguage::German);
                }
                (ToolName::EntityProfile, ToolSubrequest::EntityProfile(request)) => {
                    assert_eq!(request.entity.id, 1);
                }
                (ToolName::HeroCompare, ToolSubrequest::HeroCompare(request)) => {
                    assert_eq!(request.hero_ids, [1, 2]);
                }
                (ToolName::DamageCalculate, ToolSubrequest::DamageCalculate(request)) => {
                    assert_eq!(request.scenario.progression, ToolProgression::Boons(35));
                }
                (ToolName::PatchHistory, ToolSubrequest::PatchHistory(request)) => {
                    assert_eq!(request.historical_client_versions, [6758, 6759]);
                }
                (ToolName::BuildPlan, ToolSubrequest::BuildPlan(request)) => {
                    assert_eq!(request.playstyle.as_str(), "weapon");
                }
                (ToolName::ServerKnowledge, ToolSubrequest::ServerKnowledge(request)) => {
                    assert_eq!(request.public_channel_id, Some(4));
                }
                _ => panic!("Unteranfrage passt nicht zum Werkzeugnamen"),
            }
        }
    }

    #[test]
    fn typisierte_argumente_verwerfen_unbekannte_felder_und_ungueltige_szenarien() {
        let scenario = json!({"progression":{"kind":"boons","value":35}});
        let mut scenarios = Vec::new();
        for (field, value) in [
            ("level", json!(0)),
            ("total_spirit", json!(-1.0)),
            ("horizon_seconds", json!(0.0)),
            ("item_ids", json!([1, 1])),
            ("item_ids", json!([0])),
            (
                "ability_ranks",
                json!([{"ability_id":2,"rank":1},{"ability_id":2,"rank":2}]),
            ),
            (
                "imbues",
                json!([{"item_id":1,"ability_id":2},{"item_id":1,"ability_id":3}]),
            ),
            (
                "item_transitions",
                json!([{"kind":"upgrade","from_item_id":1,"to_item_id":1}]),
            ),
            ("target", json!({"kind":"player","values":{"health":-1.0}})),
            (
                "target",
                json!({"kind":"player","values":{},"hit_chance":1.1}),
            ),
            (
                "target",
                json!({"kind":"player","values":{},"states":[{"at_seconds":1.0,"values":{}},{"at_seconds":1.0,"values":{}}]}),
            ),
            ("unknown", json!(1)),
        ] {
            let mut invalid = scenario.clone();
            invalid[field] = value;
            scenarios.push(invalid);
        }
        let mut outside_horizon = scenario.clone();
        outside_horizon["horizon_seconds"] = json!(1.0);
        outside_horizon["target"] =
            json!({"kind":"player","values":{},"states":[{"at_seconds":2.0,"values":{}}]});
        scenarios.push(outside_horizon);
        for scenario in scenarios {
            let arguments = json!({"hero_id":1,"scenario":scenario});
            let definition = ToolDefinition {
                name: ToolName::DamageCalculate,
                description: "Schaden berechnen".into(),
                input_schema: closed_schema(&arguments),
            };
            let call = ToolCall {
                id: "invalid-call".into(),
                name: definition.name,
                arguments,
            };
            assert!(call.validate(&[definition]).is_err());
        }
        for (name, arguments) in [
            (
                ToolName::EntityFind,
                json!({"query":" ","language":"german"}),
            ),
            (
                ToolName::EntityFind,
                json!({"query":"Warden","language":"german","unknown":true}),
            ),
            (
                ToolName::EntityProfile,
                json!({"entity":{"kind":"hero","id":0},"fields":["weapon_dps"]}),
            ),
            (
                ToolName::HeroCompare,
                json!({"hero_ids":[1,1],"metrics":["weapon_dps"],"scenario":scenario}),
            ),
            (
                ToolName::HeroCompare,
                json!({"hero_ids":[1],"metrics":["weapon_dps"],"scenario":scenario}),
            ),
            (
                ToolName::HeroCompare,
                json!({"hero_ids":[1,2],"metrics":["weapon_dps","weapon_dps"],"scenario":scenario}),
            ),
            (
                ToolName::BuildPlan,
                json!({"hero_id":1,"playstyle":"weapon","budget":0}),
            ),
            (
                ToolName::BuildPlan,
                json!({"hero_id":1,"playstyle":"unknown"}),
            ),
            (
                ToolName::PatchHistory,
                json!({"entity":{"kind":"hero","id":1},"historical_client_versions":[0]}),
            ),
            (
                ToolName::PatchHistory,
                json!({"entity":{"kind":"hero","id":1},"historical_client_versions":[6759,6759]}),
            ),
            (
                ToolName::ServerKnowledge,
                json!({"question":"Lanes","public_channel_id":0}),
            ),
        ] {
            let definition = ToolDefinition {
                name,
                description: name.as_str().into(),
                input_schema: closed_schema(&arguments),
            };
            let call = ToolCall {
                id: "invalid-call".into(),
                name,
                arguments,
            };
            assert!(call.validate(&[definition]).is_err());
        }
    }

    #[test]
    fn werkzeugausfuehrung_bindet_ergebnis_und_alle_belege_an_die_unteranfrage() {
        let call = call();
        let request = call.validate(&[definition()]).unwrap();
        let game = PinnedGameContext {
            client_version: 6759,
            language: ToolLanguage::German,
            mechanic_revision: "mechanic-v1".into(),
        };
        let execution = ToolExecution {
            result: ToolResult {
                call_id: call.id.clone(),
                name: call.name,
                result: json!({"id":1}),
                evidence_ids: vec!["e1".into()],
                is_error: false,
            },
            dependencies: vec![ToolEvidenceDependency {
                request: request.clone(),
                game_context: Some(game.clone()),
                evidence: vec![evidence()],
            }],
            usage: Usage::default(),
        };
        assert!(execution.validate_for(&call, &request, Some(&game)).is_ok());
        assert!(execution.validate_for(&call, &request, None).is_err());
        let mut invalid_game = game.clone();
        invalid_game.client_version = 0;
        assert!(execution
            .validate_for(&call, &request, Some(&invalid_game))
            .is_err());
        let mut alternate_call = call.clone();
        alternate_call.arguments["query"] = json!("Haze");
        let alternate_request = alternate_call.validate(&[definition()]).unwrap();
        assert!(execution
            .validate_for(&alternate_call, &request, Some(&game))
            .is_err());
        assert!(execution
            .validate_for(&call, &alternate_request, Some(&game))
            .is_err());
        let mut invalid_executions = Vec::new();
        let mut invalid = execution.clone();
        invalid.result.call_id = "other".into();
        invalid_executions.push(invalid);
        let mut invalid = execution.clone();
        invalid.result.name = ToolName::BuildPlan;
        invalid_executions.push(invalid);
        let mut invalid = execution.clone();
        invalid.result.evidence_ids.push("missing".into());
        invalid_executions.push(invalid);
        let mut invalid = execution.clone();
        invalid.result.evidence_ids.push("e1".into());
        invalid_executions.push(invalid);
        let mut invalid = execution.clone();
        invalid.dependencies[0].request = alternate_request;
        invalid_executions.push(invalid);
        let mut invalid = execution.clone();
        invalid.dependencies[0]
            .game_context
            .as_mut()
            .unwrap()
            .client_version += 1;
        invalid_executions.push(invalid);
        let mut invalid = execution.clone();
        invalid.dependencies[0].evidence[0].revision = 0;
        invalid_executions.push(invalid);
        let mut invalid = execution.clone();
        invalid.dependencies[0].evidence.push(evidence());
        invalid_executions.push(invalid);
        let mut invalid = execution.clone();
        invalid.dependencies.clear();
        invalid_executions.push(invalid);
        for invalid in invalid_executions {
            assert!(invalid.validate_for(&call, &request, Some(&game)).is_err());
        }
        let mut full = execution.clone();
        let mut extra = evidence();
        extra.evidence_id = "uncited".into();
        full.dependencies[0].evidence.push(extra);
        assert!(full.validate_for(&call, &request, Some(&game)).is_ok());
        let mut error = execution;
        error.result.is_error = true;
        error.result.evidence_ids.clear();
        error.dependencies.clear();
        assert!(error.validate_for(&call, &request, Some(&game)).is_ok());
    }

    #[test]
    fn regeln_verwerfen_nichtendliche_spielzeit_und_szenariowerte() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let request = ToolSubrequest::GameRules(GameRulesRequest {
                topic: ToolGameRuleTopic::ResistStacking,
                entity: None,
                game_time_seconds: Some(value),
                scenario: None,
            });
            assert!(request.validate().is_err());
            let scenario: ToolScenario = serde_json::from_value(json!({
                "progression":{"kind":"boons","value":0},
                "target":{"kind":"player","values":{}}
            }))
            .unwrap();
            let mut scenario = scenario;
            scenario.target.as_mut().unwrap().values.bullet_resist = Some(value);
            let request = ToolSubrequest::GameRules(GameRulesRequest {
                topic: ToolGameRuleTopic::ResistStacking,
                entity: None,
                game_time_seconds: Some(0.0),
                scenario: Some(scenario),
            });
            assert!(request.validate().is_err());
        }
    }

    #[test]
    fn geschlossene_namen_und_bloecke() {
        let names = [
            ToolName::EntityFind,
            ToolName::EntityProfile,
            ToolName::HeroCompare,
            ToolName::DamageCalculate,
            ToolName::PatchHistory,
            ToolName::BuildPlan,
            ToolName::ServerKnowledge,
        ];
        for name in names {
            assert_eq!(serde_json::to_value(name).unwrap(), name.as_str());
        }
        assert!(serde_json::from_value::<ToolName>(json!("sql_execute")).is_err());
        assert!(
            serde_json::from_value::<ModelBlock>(json!({"type":"thinking","text":"hidden"}))
                .is_err()
        );
        assert!(serde_json::from_value::<ToolCall>(
            json!({"id":"x","name":"entity_find","arguments":{},"actor_id":"other"})
        )
        .is_err());
    }

    #[test]
    fn schemapruefung_und_serverbindungen() {
        let definition = definition();
        let request = call().validate(std::slice::from_ref(&definition)).unwrap();
        assert_eq!(request.name(), ToolName::EntityFind);
        assert_eq!(request.arguments()["query"], "Warden");
        for arguments in [
            json!({}),
            json!({"query":1,"language":"german"}),
            json!({"query":"x","language":"french"}),
            json!({"query":"x","language":"german","actor_id":"other"}),
        ] {
            let mut call = call();
            call.arguments = arguments;
            assert!(call.validate(std::slice::from_ref(&definition)).is_err());
        }
        for field in [
            "actor_id",
            "scopes",
            "knowledge_release",
            "url",
            "sql",
            "expression",
            "request_deadline",
        ] {
            let mut definition = definition.clone();
            definition.input_schema["properties"][field] = json!({"type":"string"});
            assert!(definition.validate().is_err());
        }
        let mut unsupported = definition.clone();
        unsupported.input_schema["$ref"] = json!("other");
        assert!(unsupported.validate().is_err());
        assert!(call().validate(&[]).is_err());
        assert!(validate_definitions(&[definition.clone(), definition.clone()]).is_err());
        for schema in [
            json!({"type":"object","properties":{},"additionalProperties":true}),
            json!({"type":"object","properties":{},"required":["missing"],"additionalProperties":false}),
            json!({"type":"object","properties":{"query":{"type":"string"}},"required":["query","query"],"additionalProperties":false}),
            json!({"type":"object","properties":{"query":{"type":"string","enum":["same","same"]}},"additionalProperties":false}),
            json!({"type":"object","properties":{"query":{"type":"string","enum":[1]}},"additionalProperties":false}),
            json!({"type":"object","properties":{"query":{"type":"string","minLength":2,"maxLength":1}},"additionalProperties":false}),
            json!({"type":"object","properties":{"nested":{"type":"object","properties":{"client_version":{"type":"integer"}},"additionalProperties":false}},"additionalProperties":false}),
        ] {
            assert!(ToolDefinition {
                input_schema: schema,
                ..definition.clone()
            }
            .validate()
            .is_err());
        }
        let schema =
            json!({"type":"integer", "minimum":9007199254740993u64, "maximum":9007199254740995u64});
        assert!(validate_schema(&schema).is_ok());
        assert!(validate_arguments(&json!(9007199254740992u64), &schema).is_err());
        assert!(validate_arguments(&json!(9007199254740993u64), &schema).is_ok());
        assert!(validate_arguments(&json!(9007199254740996u64), &schema).is_err());
    }

    #[test]
    fn ergebniszuordnung_und_wiederholte_ids() {
        let assistant = ToolMessage::Assistant {
            blocks: vec![ModelBlock::ToolUse { call: call() }],
        };
        let result = ToolResult {
            call_id: "call-1".into(),
            name: ToolName::EntityFind,
            result: json!({"entities":[]}),
            evidence_ids: vec!["e1".into()],
            is_error: false,
        };
        let conversation = ToolConversation {
            messages: vec![
                assistant.clone(),
                ToolMessage::ToolResults {
                    results: vec![result.clone()],
                },
            ],
        };
        assert!(conversation.validate(&[definition()]).is_ok());
        for results in [
            vec![],
            vec![result.clone(), result.clone()],
            vec![ToolResult {
                call_id: "other".into(),
                ..result.clone()
            }],
            vec![ToolResult {
                name: ToolName::BuildPlan,
                ..result.clone()
            }],
        ] {
            assert!(ToolConversation {
                messages: vec![assistant.clone(), ToolMessage::ToolResults { results }]
            }
            .validate(&[definition()])
            .is_err());
        }
        assert!(ToolConversation {
            messages: vec![assistant.clone()]
        }
        .validate(&[definition()])
        .is_err());
        let mut repeated = conversation.clone();
        repeated.messages.extend(conversation.messages);
        assert!(repeated.validate(&[definition()]).is_err());
        assert!(ToolConversation {
            messages: vec![ToolMessage::Assistant {
                blocks: vec![
                    ModelBlock::ToolUse { call: call() },
                    ModelBlock::ToolUse { call: call() }
                ]
            }]
        }
        .validate(&[definition()])
        .is_err());
    }

    #[test]
    fn werkzeugturn_ist_keine_finale_antwort() {
        let turn = ProviderTurn::ToolCalls {
            blocks: vec![ModelBlock::ToolUse { call: call() }],
            finish_reason: ProviderFinishReason::ToolUse,
            usage: Usage {
                input_tokens: 100,
                ..Usage::default()
            },
        };
        assert_eq!(turn.usage().input_tokens, 100);
        assert!(turn.validate(&[definition()]).is_ok());
        assert_eq!(
            serde_json::from_value::<ProviderTurn>(serde_json::to_value(&turn).unwrap()).unwrap(),
            turn
        );
        for finish_reason in [
            ProviderFinishReason::EndTurn,
            ProviderFinishReason::MaxTokens,
            ProviderFinishReason::Refusal,
        ] {
            assert!(ProviderTurn::ToolCalls {
                blocks: vec![ModelBlock::ToolUse { call: call() }],
                finish_reason,
                usage: Usage::default()
            }
            .validate(&[definition()])
            .is_err());
        }
        assert!(ProviderTurn::ToolCalls {
            blocks: vec![ModelBlock::Text {
                text: "noch keine Antwort".into()
            }],
            finish_reason: ProviderFinishReason::ToolUse,
            usage: Usage::default()
        }
        .validate(&[definition()])
        .is_err());
        assert!(turn.into_answer().is_err());
        let answer = ProviderAnswer {
            text: "Antwort".into(),
            cited_evidence_ids: vec!["e1".into()],
            usage: Usage::default(),
        };
        assert_eq!(
            ProviderTurn::Final {
                answer: answer.clone(),
                finish_reason: ProviderFinishReason::EndTurn
            }
            .into_answer()
            .unwrap(),
            answer
        );
        assert!(ProviderTurn::from(ProviderAnswer {
            cited_evidence_ids: vec!["e1".into(), "e1".into()],
            ..answer.clone()
        })
        .validate(&[])
        .is_err());
        assert_eq!(ProviderTurn::from(answer.clone()).usage(), &answer.usage);
        for finish_reason in [
            ProviderFinishReason::ToolUse,
            ProviderFinishReason::MaxTokens,
            ProviderFinishReason::Refusal,
        ] {
            let turn = ProviderTurn::Final {
                answer: answer.clone(),
                finish_reason,
            };
            assert!(turn.validate(&[]).is_err());
            assert!(turn.into_answer().is_err());
        }
    }

    #[test]
    fn unimplementierte_abhaengigkeitspruefung_schliesst_sicher() {
        struct NoValidation;
        impl ToolExecutionPort for NoValidation {
            fn execute(
                &self,
                _query: &Query,
                _context: &AuthorizedContext,
                _game_context: Option<&PinnedGameContext>,
                _call_id: &str,
                _request: &ToolRequest,
            ) -> Result<ToolExecution, PortError> {
                Err(PortError::Unavailable("fehlt".into()))
            }
        }
        let query = Query {
            request_id: "r".into(),
            conversation_id: "c".into(),
            text: "Frage".into(),
            domain: None,
            requested_scopes: BTreeSet::new(),
            profile: Default::default(),
            patch: None,
            mode: None,
        };
        let context = AuthorizedContext {
            discord: None,
            principal: crate::Principal {
                actor_id: "a".into(),
                channel: "test".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::new(),
            },
            conversation_id: "c".into(),
            knowledge_release: "r1".into(),
            deadline_ms: 1000,
            budget: Default::default(),
            request_deadline: None,
        };
        for purpose in [
            ToolValidationPurpose::Provider,
            ToolValidationPurpose::Publication,
            ToolValidationPurpose::Cache,
        ] {
            assert!(NoValidation
                .validate_dependencies(&query, &context, None, &[], purpose)
                .is_err());
        }
        assert!(NoValidation.definitions(&query, &context, None).is_err());
    }
}
