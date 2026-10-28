use crate::discord_live::{allowed, denied, observation_key, DiscordLive};
use brain_contracts::{
    Accounted, AuthorizedContext, Evidence, PinnedGameContext, PortError, PortFailure, Query,
    ToolCall, ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolExecutionPort, ToolName,
    ToolRequest, ToolResult, ToolValidationPurpose, Usage, UsageAccounting,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Instant,
};

const RESULT_BYTES: usize = 12_000;
const PRIORITY_PREFIXES: [&str; 4] = [
    "Aktuelle öffentliche Discord-Kanäle",
    "TempVoice-Kategorie: ",
    "Join-to-create: ",
    "Offene Lane: ",
];

type Observations = BTreeMap<String, (Instant, Vec<Evidence>)>;

pub(crate) struct DiscordLiveTools {
    delegate: Arc<dyn ToolExecutionPort>,
    live: Option<Arc<DiscordLive>>,
    observations: Mutex<Observations>,
}

pub(crate) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::ServerKnowledge,
        description:
            "Liefert den aktuellen Live-Stand des Discord-Servers aus Sicht der fragenden \
            Person: offene Voice-Lanes mit Art, Belegung und Beitritt-Platzhalter, den \
            Join-to-create-Kanal zum Öffnen einer eigenen Lane, sichtbare Kanäle und \
            Voice-Anzahlen. Bei Fragen nach Mitspielern, einer Lobby, offenen Lanes oder wer \
            gerade im Voice ist, zuerst hier abfragen."
                .into(),
        input_schema: json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["question"],
            "properties": {"question": {"type": "string", "minLength": 1}}
        }),
    }
}

fn selected_lines(content: &str, query: &Query) -> Vec<String> {
    let terms = brain_contracts::lexical::terms(&query.text);
    let rank = |line: &str| {
        let lower = line.to_lowercase();
        if PRIORITY_PREFIXES
            .iter()
            .any(|prefix| line.starts_with(prefix))
        {
            0
        } else if terms.iter().any(|term| lower.contains(term)) {
            1
        } else {
            2
        }
    };
    let mut lines: Vec<&str> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    lines.sort_by_key(|line| rank(line));
    let mut seen = std::collections::HashSet::new();
    lines.retain(|line| seen.insert(*line));
    let mut used = 0;
    let mut selected = Vec::new();
    for line in lines {
        let cost = serde_json::to_string(line).map_or(usize::MAX, |text| text.len() + 1);
        if used + cost > RESULT_BYTES {
            continue;
        }
        used += cost;
        selected.push(line.to_owned());
    }
    selected
}

fn execution(
    id: &str,
    request: &ToolRequest,
    mut item: Evidence,
    lines: Vec<String>,
) -> ToolExecution {
    item.content = lines.join("\n");
    item.evidence_id = format!("discord-live-{:x}", Sha256::digest(item.content.as_bytes()));
    ToolExecution {
        result: ToolResult {
            call_id: id.into(),
            name: request.name(),
            result: json!({"matches": [{"evidence_id": item.evidence_id, "content": item.content}]}),
            evidence_ids: vec![item.evidence_id.clone()],
            is_error: false,
        },
        dependencies: vec![ToolEvidenceDependency {
            request: request.clone(),
            game_context: None,
            evidence: vec![item],
        }],
        usage: network_round(),
    }
}

fn network_round() -> Usage {
    Usage {
        network_rounds: 1,
        ..Usage::default()
    }
}

impl DiscordLiveTools {
    pub(crate) fn new(
        delegate: Arc<dyn ToolExecutionPort>,
        live: Option<Arc<DiscordLive>>,
    ) -> Self {
        Self {
            delegate,
            live,
            observations: Mutex::new(BTreeMap::new()),
        }
    }

    fn remember(&self, key: String, expires: Instant, item: &Evidence) -> Result<(), PortError> {
        let mut observations = self.observations.lock().map_err(|_| denied())?;
        let now = Instant::now();
        if now >= expires {
            return Err(denied());
        }
        observations.retain(|_, (until, _)| now < *until);
        let entry = observations.entry(key).or_insert((expires, Vec::new()));
        entry.0 = entry.0.min(expires);
        entry.1.push(item.clone());
        Ok(())
    }

    fn prepare(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        id: &str,
        request: &ToolRequest,
    ) -> Result<(ToolCall, &DiscordLive), PortError> {
        context.check_deadline()?;
        if context.budget.max_network_rounds == 0 {
            return Err(PortError::BudgetExceeded);
        }
        if !allowed(query, context, true) {
            return Err(denied());
        }
        let call = ToolCall {
            id: id.into(),
            name: request.name(),
            arguments: request.arguments().clone(),
        };
        if call.validate(&[definition()])? != *request {
            return Err(denied());
        }
        let live = self.live.as_deref().ok_or_else(denied)?;
        Ok((call, live))
    }

    fn fetch(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        request: &ToolRequest,
        call: &ToolCall,
        live: &DiscordLive,
    ) -> Result<ToolExecution, PortError> {
        let (expires, item) = live.read_channel(context, None)?;
        let lines = selected_lines(&item.content, query);
        let execution = execution(&call.id, request, item, lines);
        execution.validate_for(call, request, None)?;
        let key = observation_key(query, context)?;
        for dependency in &execution.dependencies {
            for item in &dependency.evidence {
                self.remember(key.clone(), expires, item)?;
            }
        }
        Ok(execution)
    }

    fn run(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        id: &str,
        request: &ToolRequest,
    ) -> Result<Accounted<ToolExecution>, PortFailure> {
        let (call, live) = self
            .prepare(query, context, id, request)
            .map_err(PortFailure::before_call)?;
        let dispatched = UsageAccounting::observed(network_round());
        let value = self
            .fetch(query, context, request, &call, live)
            .map_err(|error| PortFailure::accounted(error, dispatched.clone()))?;
        Ok(Accounted {
            accounting: UsageAccounting::observed(value.usage.clone()),
            value,
        })
    }

    fn validate_own(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        dependencies: &[ToolEvidenceDependency],
        purpose: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        context.check_deadline()?;
        if !allowed(query, context, purpose == ToolValidationPurpose::Provider) {
            return Err(denied());
        }
        let key = observation_key(query, context)?;
        let observations = self.observations.lock().map_err(|_| denied())?;
        let Some((expires, current)) = observations.get(&key) else {
            return Err(denied());
        };
        if Instant::now() >= *expires
            || dependencies.iter().any(|dependency| {
                dependency.game_context.is_some()
                    || dependency.evidence.is_empty()
                    || dependency
                        .evidence
                        .iter()
                        .any(|item| !current.contains(item))
            })
        {
            return Err(denied());
        }
        Ok(())
    }
}

impl ToolExecutionPort for DiscordLiveTools {
    fn required_calls(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolCall>, PortError> {
        self.delegate.required_calls(query, context, pin)
    }

    fn definitions(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        let mut definitions = self.delegate.definitions(query, context, pin)?;
        if self.live.is_some()
            && allowed(query, context, true)
            && definitions
                .iter()
                .all(|known| known.name != ToolName::ServerKnowledge)
        {
            definitions.push(definition());
        }
        Ok(definitions)
    }

    fn execute(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        if request.name() == ToolName::ServerKnowledge {
            return self
                .run(query, context, id, request)
                .map(|accounted| accounted.value)
                .map_err(|failure| failure.error);
        }
        self.delegate.execute(query, context, pin, id, request)
    }

    fn execute_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<Accounted<ToolExecution>, PortFailure> {
        if request.name() != ToolName::ServerKnowledge {
            return self
                .delegate
                .execute_accounted(query, context, pin, id, request);
        }
        self.run(query, context, id, request)
    }

    fn validate_build_plan(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: &PinnedGameContext,
        request: &brain_contracts::tools::BuildPlanRequest,
        execution: &ToolExecution,
        purpose: brain_contracts::store::AnswerPurpose,
    ) -> Result<(), PortError> {
        self.delegate
            .validate_build_plan(query, context, pin, request, execution, purpose)
    }

    fn validate_dependencies(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        purpose: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        let (own, delegated): (Vec<_>, Vec<_>) = dependencies
            .iter()
            .cloned()
            .partition(|dependency| dependency.request.name() == ToolName::ServerKnowledge);
        if !own.is_empty() {
            self.validate_own(query, context, &own, purpose)?;
            if delegated.is_empty() {
                return Ok(());
            }
        }
        self.delegate
            .validate_dependencies(query, context, pin, &delegated, purpose)
    }
}
