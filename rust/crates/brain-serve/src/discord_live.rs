use brain_contracts::{
    AuthorizedContext, Evidence, EvidenceKind, PortError, Query, RetrievalPort, SourceVisibility,
    Usage,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::types::chrono::{DateTime, Utc};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    io::Read,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

const SOURCE: &str = "discord.public-live.v1";
const ENDPOINT: &str = "http://127.0.0.1:8890/mcp/public";

fn unavailable() -> PortError {
    PortError::Unavailable("Öffentliche Discord-Live-Fakten sind derzeit nicht verfügbar.".into())
}
fn denied() -> PortError {
    PortError::PermissionDenied("Öffentliche Discord-Live-Fakten sind nicht freigegeben.".into())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Facts {
    schema: String,
    guild_id: String,
    observed_at: String,
    cache_seconds: u64,
    audience: String,
    channels: Vec<Channel>,
    voice_counts: Vec<VoiceCount>,
    bot_infos: Vec<BotInfo>,
    #[serde(default)]
    tempvoice: TempVoice,
    #[serde(default)]
    messages: Vec<History>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct History {
    channel_id: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Message {
    id: String,
    text: String,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct TempVoice {
    category_ids: Vec<String>,
    join_channel_ids: Vec<String>,
    open_lanes: Vec<OpenLane>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OpenLane {
    channel_id: String,
    count: u64,
    mode: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Channel {
    id: String,
    name: String,
    #[serde(rename = "type")]
    kind: u64,
    topic: Option<String>,
    position: i64,
    parent_id: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VoiceCount {
    channel_id: String,
    count: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BotInfo {
    channel_id: String,
    message_id: String,
    text: String,
}

fn evidence(facts: Facts) -> Result<(Instant, Evidence), PortError> {
    let received = Instant::now();
    let observed = DateTime::parse_from_rfc3339(&facts.observed_at).map_err(|_| denied())?;
    let age = Utc::now()
        .signed_duration_since(observed)
        .to_std()
        .map_err(|_| denied())?;
    let remaining = Duration::from_secs(facts.cache_seconds)
        .checked_sub(age)
        .filter(|remaining| !remaining.is_zero())
        .ok_or_else(denied)?;
    if facts.schema != "discord.public-facts.v2"
        || !matches!(facts.audience.as_str(), "verified_members" | "requester")
        || facts.cache_seconds > 60
        || facts.cache_seconds == 0
        || facts.guild_id.parse::<u64>().ok().is_none_or(|id| id == 0)
    {
        return Err(denied());
    }
    let ids: HashSet<_> = facts.channels.iter().map(|c| c.id.as_str()).collect();
    if ids.len() != facts.channels.len()
        || facts.channels.iter().any(|c| {
            c.id.parse::<u64>().ok().is_none_or(|id| id == 0)
                || !matches!(c.kind, 0 | 2 | 4 | 5 | 13 | 15 | 16)
                || c.parent_id.as_ref().is_some_and(|parent| {
                    !facts
                        .channels
                        .iter()
                        .any(|p| p.id == *parent && p.kind == 4)
                })
        })
        || facts.voice_counts.iter().any(|v| {
            !facts
                .channels
                .iter()
                .any(|c| c.id == v.channel_id && matches!(c.kind, 2 | 13))
        })
        || facts.bot_infos.iter().any(|i| {
            !ids.contains(i.channel_id.as_str())
                || i.message_id.parse::<u64>().is_err()
                || i.text.contains("<@")
                || i.text.contains("discord.com/users/")
        })
    {
        return Err(denied());
    }
    let mut lines = vec!["Aktuelle öffentliche Discord-Kanäle und Voice-Anzahlen.".to_owned()];
    for id in &facts.tempvoice.category_ids {
        let category = facts
            .channels
            .iter()
            .find(|channel| channel.id == *id && channel.kind == 4)
            .ok_or_else(denied)?;
        lines.push(format!("TempVoice-Kategorie: {}.", category.name));
    }
    for id in &facts.tempvoice.join_channel_ids {
        let channel = facts
            .channels
            .iter()
            .find(|channel| channel.id == *id && channel.kind == 2)
            .ok_or_else(denied)?;
        lines.push(format!("Join-to-create: {}.", channel.name));
    }
    for lane in &facts.tempvoice.open_lanes {
        let channel = facts
            .channels
            .iter()
            .find(|channel| channel.id == lane.channel_id && channel.kind == 2)
            .ok_or_else(denied)?;
        lines.push(format!(
            "Offene Lane: {}, Art {}, {} anwesend.",
            channel.name, lane.mode, lane.count
        ));
    }
    for c in &facts.channels {
        let kind = match c.kind {
            0 => "Textkanal",
            2 => "Voice",
            4 => "Kategorie",
            5 => "Ankündigungen",
            13 => "Stage",
            15 => "Forum",
            16 => "Medien",
            _ => unreachable!(),
        };
        let parent = c
            .parent_id
            .as_ref()
            .and_then(|id| facts.channels.iter().find(|p| p.id == *id))
            .map(|p| format!(", Kategorie {}", p.name))
            .unwrap_or_default();
        let count = facts
            .voice_counts
            .iter()
            .find(|v| v.channel_id == c.id)
            .map(|v| format!(", {} anwesend", v.count))
            .unwrap_or_default();
        lines.push(format!(
            "{kind}: {}{parent}{count}, Reihenfolge {}. {}",
            c.name,
            c.position,
            c.topic.as_deref().unwrap_or_default()
        ));
    }
    for info in &facts.bot_infos {
        lines.push(format!(
            "Bot-Infotext in {}: {}",
            facts
                .channels
                .iter()
                .find(|c| c.id == info.channel_id)
                .map(|c| c.name.as_str())
                .ok_or_else(denied)?,
            info.text
        ));
    }
    for history in &facts.messages {
        let channel = facts
            .channels
            .iter()
            .find(|channel| channel.id == history.channel_id && matches!(channel.kind, 0 | 5))
            .ok_or_else(denied)?;
        for message in &history.messages {
            if message.id.parse::<u64>().ok().is_none_or(|id| id == 0) {
                return Err(denied());
            }
            lines.push(format!("Nachricht in {}: {}", channel.name, message.text));
        }
    }
    let content = lines.join("\n");
    let digest = format!("{:x}", Sha256::digest(content.as_bytes()));
    let expires = received + remaining;
    if Instant::now() >= expires {
        return Err(denied());
    }
    Ok((
        expires,
        Evidence {
            evidence_id: format!("discord-live-{digest}"),
            source_id: SOURCE.into(),
            logical_id: format!("discord-guild:{}", facts.guild_id),
            revision: 1,
            kind: EvidenceKind::Prose,
            content,
            citation: format!("https://discord.com/channels/{}", facts.guild_id),
            visibility: SourceVisibility::RequestScoped,
            allowed_scopes: BTreeSet::from(["discord.request:unbound".into()]),
            score: 100.0,
            provenance: None,
            patch: None,
        },
    ))
}

pub(crate) struct DiscordLive {
    http: reqwest::blocking::Client,
    token: String,
    endpoint: String,
}
impl DiscordLive {
    pub(crate) fn new(token: String) -> Result<Self, crate::Error> {
        let http = reqwest::blocking::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|_| crate::Error::ConfigInvalid("discord_live_http"))?;
        Ok(Self {
            http,
            token,
            endpoint: ENDPOINT.into(),
        })
    }
    #[cfg(test)]
    fn read(&self, context: &AuthorizedContext) -> Result<(Instant, Evidence), PortError> {
        self.read_channel(context, None)
    }

    fn read_channel(
        &self,
        context: &AuthorizedContext,
        channel_id: Option<u64>,
    ) -> Result<(Instant, Evidence), PortError> {
        let request = context.discord.as_ref().ok_or_else(denied)?;
        if !request.allow_discord_reads {
            return Err(denied());
        }
        let text = self.call(
            context,
            "public_server_facts",
            channel_id
                .map(|id| json!({"channel_id":id}))
                .unwrap_or_else(|| json!({})),
        )?;
        let (expires, mut item) = evidence(serde_json::from_str(&text).map_err(|_| denied())?)?;
        item.allowed_scopes = BTreeSet::from([request.scope.clone()]);
        Ok((expires, item))
    }

    fn read_invite(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<(Instant, Evidence), PortError> {
        let request = brain_contracts::invite::authorize(query, context, true)?;
        let text = self.call(context, "self_invite_status", json!({}))?;
        let status = serde_json::from_str(&text).map_err(|_| denied())?;
        let item = brain_contracts::invite::status_evidence(&status, request.scope.clone())?;
        Ok((Instant::now() + context.remaining_time()?, item))
    }

    fn call(
        &self,
        context: &AuthorizedContext,
        name: &str,
        arguments: Value,
    ) -> Result<String, PortError> {
        context.check_deadline()?;
        if context.budget.max_network_rounds == 0 {
            return Err(PortError::BudgetExceeded);
        }
        let request = context.discord.as_ref().ok_or_else(denied)?;
        let mut builder = self
            .http
            .post(&self.endpoint)
            .bearer_auth(&self.token)
            .header("x-discord-request-id", &request.request_id);
        if let Some(user) = request.user_id {
            builder = builder.header("x-discord-user-id", user);
        }
        let mut response = builder
            .timeout(context.remaining_time()?.min(Duration::from_secs(15)))
            .json(&json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":arguments}}))
            .send().map_err(|_| unavailable())?.error_for_status().map_err(|_| unavailable())?;
        let mut bytes = Vec::new();
        response
            .by_ref()
            .take(262_145)
            .read_to_end(&mut bytes)
            .map_err(|_| unavailable())?;
        context.check_deadline()?;
        if bytes.len() > 262_144 {
            return Err(unavailable());
        }
        let rpc: Value = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
        let result = &rpc["result"];
        if rpc["id"] != 1
            || result["isError"] != false
            || result["content"].as_array().is_none_or(|c| c.len() != 1)
            || result["content"][0]["type"] != "text"
        {
            return Err(unavailable());
        }
        result["content"][0]["text"]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(unavailable)
    }
}

fn allowed(query: &Query, context: &AuthorizedContext, provider: bool) -> bool {
    query.requested_scopes.contains("bot.public")
        && context.principal.scopes.contains("bot.public")
        && context
            .discord
            .as_ref()
            .is_some_and(|request| request.allow_discord_reads)
        && (!provider || context.principal.provider_egress.contains("public"))
}

pub(crate) struct DiscordRetriever<R> {
    inner: R,
    live: Option<Arc<DiscordLive>>,
    pricing: Option<brain_providers::PriceCeiling>,
    retry_attempts: usize,
    observations: Mutex<BTreeMap<String, (Instant, Vec<Evidence>)>>,
}

fn observation_key(query: &Query, context: &AuthorizedContext) -> Result<String, PortError> {
    let bytes = serde_json::to_vec(&(
        &query.request_id,
        &query.conversation_id,
        &query.text,
        &query.answer_context,
        &query.domain,
        &query.requested_scopes,
        &query.profile,
        &query.patch,
        &query.mode,
        &context.principal,
        &context.conversation_id,
        &context.knowledge_release,
        &context.discord.as_ref().map(|request| {
            (
                &request.user_id,
                &request.request_id,
                &request.scope,
                request.allow_discord_reads,
            )
        }),
    ))
    .map_err(|_| denied())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

impl<R> DiscordRetriever<R> {
    pub(crate) fn new(inner: R, live: Option<Arc<DiscordLive>>) -> Self {
        Self {
            inner,
            live,
            pricing: None,
            retry_attempts: 1,
            observations: Mutex::new(BTreeMap::new()),
        }
    }
    pub(crate) fn with_provider(mut self, provider: &crate::config::Provider) -> Self {
        self.pricing = provider.pricing.map(|price| brain_providers::PriceCeiling {
            input_micros_per_token: price.input_micros_per_token,
            output_micros_per_token: price.output_micros_per_token,
        });
        self.retry_attempts = provider.retry_attempts;
        self
    }
    fn packing_context(
        &self,
        context: &AuthorizedContext,
        usage: &Usage,
    ) -> Result<AuthorizedContext, PortError> {
        let mut remaining = context.clone();
        remaining.budget.max_input_tokens = remaining
            .budget
            .max_input_tokens
            .checked_sub(u32::try_from(usage.input_tokens).map_err(|_| PortError::BudgetExceeded)?)
            .ok_or(PortError::BudgetExceeded)?;
        remaining.budget.max_output_tokens = remaining
            .budget
            .max_output_tokens
            .checked_sub(u32::try_from(usage.output_tokens).map_err(|_| PortError::BudgetExceeded)?)
            .ok_or(PortError::BudgetExceeded)?;
        remaining.budget.max_cost_micros = remaining
            .budget
            .max_cost_micros
            .checked_sub(usage.cost_micros)
            .ok_or(PortError::BudgetExceeded)?;
        let rounds = context
            .budget
            .max_network_rounds
            .checked_sub(usage.network_rounds)
            .ok_or(PortError::BudgetExceeded)?;
        let attempts = self.retry_attempts.min(rounds as usize).max(1) as u64;
        let mut ceiling = remaining.budget.max_input_tokens as u64 / attempts;
        if let Some(price) = self.pricing {
            let output_cost = (remaining.budget.max_output_tokens as u64)
                .checked_mul(price.output_micros_per_token)
                .ok_or(PortError::BudgetExceeded)?;
            let input_cost = remaining
                .budget
                .max_cost_micros
                .checked_sub(output_cost)
                .ok_or(PortError::BudgetExceeded)?;
            if let Some(cost_ceiling) = input_cost.checked_div(price.input_micros_per_token) {
                ceiling = ceiling.min(cost_ceiling / attempts);
            }
        }
        remaining.budget.max_input_tokens = ceiling as u32;
        Ok(remaining)
    }
    fn retrieve_invite(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<(Vec<Evidence>, Usage), PortError> {
        brain_contracts::invite::authorize(query, context, true)?;
        let live = self.live.as_ref().ok_or_else(unavailable)?;
        let (expires, item) = live.read_invite(query, context)?;
        let items = vec![item];
        let usage = Usage {
            network_rounds: 1,
            ..Usage::default()
        };
        let packing = self.packing_context(context, &usage)?;
        if brain_contracts::provider_input::grounded_input_ceiling(query, &items)
            > packing.budget.max_input_tokens as u64
        {
            return Err(PortError::BudgetExceeded);
        }
        let key = observation_key(query, context)?;
        let mut observations = self.observations.lock().map_err(|_| unavailable())?;
        let now = Instant::now();
        if now >= expires {
            return Err(denied());
        }
        observations.retain(|_, (expires, _)| now < *expires);
        observations.insert(key, (expires, items.clone()));
        Ok((items, usage))
    }

    fn validate_invite(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        items: &[Evidence],
        provider: bool,
    ) -> Result<(), PortError> {
        brain_contracts::invite::validate_projection(query, context, items, provider)?;
        self.validate_observation(query, context, items)
    }

    fn validate_observation(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        items: &[Evidence],
    ) -> Result<(), PortError> {
        context.check_deadline()?;
        let key = observation_key(query, context)?;
        let observations = self.observations.lock().map_err(|_| unavailable())?;
        match observations.get(&key) {
            Some((expires, current))
                if Instant::now() < *expires && items.iter().all(|item| current.contains(item)) =>
            {
                Ok(())
            }
            _ => Err(denied()),
        }
    }

    fn validate_live(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        items: &[Evidence],
        provider: bool,
    ) -> Result<(), PortError> {
        if !allowed(query, context, provider) {
            return Err(denied());
        }
        self.validate_observation(query, context, items)
    }
}
impl<R: RetrievalPort> RetrievalPort for DiscordRetriever<R> {
    fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        self.retrieve_with_usage(query, context)
            .map(|(items, _)| items)
    }
    fn retrieve_with_usage(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<(Vec<Evidence>, Usage), PortError> {
        if brain_contracts::invite::requested(query) {
            return self.retrieve_invite(query, context);
        }
        let (mut items, mut usage) = self.inner.retrieve_with_usage(query, context)?;
        context.check_deadline()?;
        if query.domain.is_none() && allowed(query, context, false) {
            if let Some(live) = &self.live {
                let key = observation_key(query, context)?;
                self.observations
                    .lock()
                    .map_err(|_| unavailable())?
                    .remove(&key);
                let rounds = context
                    .budget
                    .max_network_rounds
                    .checked_sub(usage.network_rounds)
                    .ok_or(PortError::BudgetExceeded)?;
                if !items.is_empty()
                    && rounds <= u32::from(query.profile != brain_contracts::AnswerProfile::Fact)
                {
                    eprintln!(
                        "{}",
                        json!({"event":"brain_discord_live_skipped","request_id":query.request_id,"reason":"network_budget"})
                    );
                    return Ok((items, usage));
                }
                if rounds == 0 {
                    return Err(PortError::BudgetExceeded);
                }
                let mut live_context = context.clone();
                live_context.budget.max_network_rounds = rounds;
                let channel_id = query
                    .text
                    .split("<#")
                    .nth(1)
                    .and_then(|suffix| suffix.split('>').next())
                    .and_then(|id| id.parse::<u64>().ok())
                    .filter(|id| *id != 0);
                usage.network_rounds += 1;
                let (expires, current) = match live.read_channel(&live_context, channel_id) {
                    Ok(current) => current,
                    Err(error @ PortError::Unavailable(_)) if !items.is_empty() => {
                        context.check_deadline()?;
                        eprintln!(
                            "{}",
                            json!({"event":"brain_discord_live_skipped","request_id":query.request_id,"reason":"unavailable","error":error.to_string()})
                        );
                        return Ok((items, usage));
                    }
                    Err(error) => return Err(error),
                };
                let packing_context =
                    if matches!(query.profile, brain_contracts::AnswerProfile::Fact) {
                        context.clone()
                    } else {
                        self.packing_context(context, &usage)?
                    };
                let mut candidates = vec![current.clone()];
                candidates.extend(items.iter().cloned());
                if !matches!(query.profile, brain_contracts::AnswerProfile::Fact)
                    && brain_contracts::provider_input::grounded_input_ceiling(query, &candidates)
                        > packing_context.budget.max_input_tokens as u64
                {
                    let terms = brain_contracts::lexical::terms(&query.text);
                    let mut lines: Vec<_> = current.content.lines().collect();
                    lines.sort_by_key(|line| {
                        let line = line.to_lowercase();
                        !terms.iter().any(|term| line.contains(term))
                    });
                    let mut seen = HashSet::new();
                    lines.retain(|line| seen.insert(*line));
                    candidates = lines
                        .into_iter()
                        .filter(|line| !line.trim().is_empty())
                        .map(|line| {
                            let mut item = current.clone();
                            item.content = line.to_owned();
                            item.evidence_id =
                                format!("discord-live-{:x}", Sha256::digest(line.as_bytes()));
                            item
                        })
                        .collect();
                }
                candidates.retain(|item| item.source_id == SOURCE);
                let additional_documents = items.split_off(items.len().min(1));
                items.append(&mut candidates);
                items.extend(additional_documents);
                candidates = items;
                items = dbrain_retrieval::pack(query, &packing_context, candidates)?;
                let selected_live = items
                    .iter()
                    .filter(|item| item.source_id == SOURCE)
                    .cloned()
                    .collect();
                let mut observations = self.observations.lock().map_err(|_| unavailable())?;
                let now = Instant::now();
                if now >= expires {
                    return Err(denied());
                }
                observations.retain(|_, (expires, _)| now < *expires);
                observations.insert(key, (expires, selected_live));
            }
        }
        Ok((items, usage))
    }
    fn validate_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        provider: bool,
    ) -> Result<(), PortError> {
        if brain_contracts::invite::requested(query)
            || evidence
                .iter()
                .any(|item| item.source_id == brain_contracts::invite::SOURCE)
        {
            return self.validate_invite(query, context, evidence, provider);
        }
        if evidence.is_empty() {
            return Err(denied());
        }
        let (live, stored): (Vec<_>, Vec<_>) = evidence
            .iter()
            .cloned()
            .partition(|e| e.source_id == SOURCE);
        if !stored.is_empty() {
            self.inner
                .validate_evidence(query, context, &stored, provider)?;
        }
        if !live.is_empty() {
            self.validate_live(query, context, &live, provider)?;
        }
        Ok(())
    }
    fn validate_publication(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<(), PortError> {
        if brain_contracts::invite::requested(query)
            || evidence
                .iter()
                .any(|item| item.source_id == brain_contracts::invite::SOURCE)
        {
            return self.validate_invite(query, context, evidence, false);
        }
        if evidence.is_empty() {
            return Err(denied());
        }
        let (live, stored): (Vec<_>, Vec<_>) = evidence
            .iter()
            .cloned()
            .partition(|e| e.source_id == SOURCE);
        if !stored.is_empty() {
            self.inner.validate_publication(query, context, &stored)?;
        }
        if !live.is_empty() {
            self.validate_live(query, context, &live, false)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statusbeobachtungen_bleiben_request_person_scope_und_readpolicy_getrennt() {
        let query: Query = serde_json::from_value(json!({"request_id":"fixture-request","conversation_id":"fixture-conversation","text":brain_contracts::invite::QUESTION,"requested_scopes":["bot.public"]})).unwrap();
        let mut context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: Some(42),
                request_id: query.request_id.clone(),
                scope: "discord.request:fixture".into(),
                allow_discord_reads: false,
            }),
            principal: brain_contracts::Principal {
                actor_id: "fixture".into(),
                channel: "discord".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:fixture".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: query.conversation_id.clone(),
            knowledge_release: "fixture".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget::default(),
            request_deadline: None,
        };
        let item = brain_contracts::invite::status_evidence(
            &brain_contracts::invite::SelfInviteStatus {
                status: brain_contracts::invite::InviteStatus::Pending,
                at: None,
            },
            "discord.request:fixture".into(),
        )
        .unwrap();
        let items = vec![item];
        let adapter = DiscordRetriever::new(Stored, None);
        let key = observation_key(&query, &context).unwrap();
        adapter.observations.lock().unwrap().insert(
            key.clone(),
            (
                Instant::now() + context.remaining_time().unwrap(),
                items.clone(),
            ),
        );
        context.budget.max_network_rounds = 0;
        for provider in [false, true] {
            adapter
                .validate_evidence(&query, &context, &items, provider)
                .unwrap();
        }
        adapter
            .validate_publication(&query, &context, &items)
            .unwrap();
        for field in 0..4 {
            let mut other = context.clone();
            match field {
                0 => other.discord.as_mut().unwrap().user_id = Some(43),
                1 => other.discord.as_mut().unwrap().allow_discord_reads = true,
                2 => other.discord.as_mut().unwrap().request_id = "other".into(),
                _ => {
                    other.discord.as_mut().unwrap().scope = "discord.request:other".into();
                    other
                        .principal
                        .scopes
                        .insert("discord.request:other".into());
                }
            }
            for provider in [false, true] {
                assert!(adapter
                    .validate_evidence(&query, &other, &items, provider)
                    .is_err());
            }
            assert!(adapter
                .validate_publication(&query, &other, &items)
                .is_err());
        }
        let mut forged = items.clone();
        forged[0].content = "{\"status\":\"sent\",\"at\":null}".into();
        assert!(adapter
            .validate_evidence(&query, &context, &forged, true)
            .is_err());
        assert!(adapter
            .validate_publication(&query, &context, &forged)
            .is_err());
        adapter
            .observations
            .lock()
            .unwrap()
            .get_mut(&key)
            .unwrap()
            .0 = Instant::now();
        assert!(adapter
            .validate_publication(&query, &context, &items)
            .is_err());
    }

    #[test]
    fn eigener_status_nutzt_private_leserechte_ohne_nachrichten_und_beide_transporte() {
        use brain_contracts::{AnswerStatus, Principal};
        use brain_kernel::{AnswerKernelPort, Kernel};
        use brain_providers::{OpenAiCompatibleProvider, ProviderConfig};
        use std::{
            io::{BufRead, BufReader, Write},
            net::TcpListener,
        };

        fn request(stream: &mut std::net::TcpStream) -> (String, Value) {
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut reader = BufReader::new(stream);
            let mut headers = String::new();
            let mut length = 0;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_lowercase().strip_prefix("content-length:") {
                    length = value.trim().parse::<usize>().unwrap();
                }
                headers.push_str(&line);
            }
            let mut bytes = vec![0; length];
            reader.read_exact(&mut bytes).unwrap();
            (headers, serde_json::from_slice(&bytes).unwrap())
        }
        fn reply(stream: &mut std::net::TcpStream, body: Value) {
            let body = body.to_string();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            )
            .unwrap();
        }
        struct NoStoredReads;
        impl RetrievalPort for NoStoredReads {
            fn retrieve(
                &self,
                _: &Query,
                _: &AuthorizedContext,
            ) -> Result<Vec<Evidence>, PortError> {
                panic!("Eigener Status darf keine weiteren Quellen lesen");
            }
        }
        for native in [false, true] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let endpoint = format!("http://{}", listener.local_addr().unwrap());
            let mut live = DiscordLive::new("fixture-mcp".into()).unwrap();
            live.endpoint = format!("{endpoint}/mcp");
            let mut config = if native {
                ProviderConfig::codex_subscription(format!("{endpoint}/v1"))
            } else {
                ProviderConfig::new("fixture-model-key", endpoint, "fixture-model")
            };
            config.retry_attempts = 1;
            let model = config.model.clone();
            let server = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                let (headers, rpc) = request(&mut stream);
                assert!(headers.to_lowercase().contains("x-discord-user-id: 42"));
                assert!(headers
                    .to_lowercase()
                    .contains("x-discord-request-id: fixture-request"));
                assert_eq!(
                    rpc["params"],
                    json!({"name":"self_invite_status","arguments":{}})
                );
                reply(
                    &mut stream,
                    json!({"jsonrpc":"2.0","id":1,"result":{"isError":false,"content":[{"type":"text","text":"{\"status\":\"sent\",\"at\":\"2026-10-02T12:00:00Z\"}"}]}}),
                );
                drop(stream);
                let (mut stream, _) = listener.accept().unwrap();
                let (headers, payload) = request(&mut stream);
                assert!(headers.starts_with(if native {
                    "POST /v1/messages "
                } else {
                    "POST /chat/completions "
                }));
                let raw = payload.to_string();
                for private in [
                    "private Zusatzkennung",
                    "76561197960265839",
                    "discord.request:",
                    "fixture-mcp",
                ] {
                    assert!(!raw.contains(private));
                }
                let messages = payload["messages"].as_array().unwrap();
                let data: Value =
                    serde_json::from_str(messages.last().unwrap()["content"].as_str().unwrap())
                        .unwrap();
                assert_eq!(data["query"], brain_contracts::invite::QUESTION);
                assert_eq!(data["evidence"].as_array().unwrap().len(), 1);
                let status: brain_contracts::invite::SelfInviteStatus =
                    serde_json::from_str(data["evidence"][0]["content"].as_str().unwrap()).unwrap();
                assert_eq!(status.status, brain_contracts::invite::InviteStatus::Sent);
                let answer = json!({"text":"Deine Einladung ist verschickt.","cited_evidence_ids":[brain_contracts::invite::EVIDENCE_ID]}).to_string();
                reply(
                    &mut stream,
                    if native {
                        json!({"model":model,"stop_reason":"end_turn","content":[{"type":"text","text":answer}],"usage":{"input_tokens":100,"output_tokens":10}})
                    } else {
                        json!({"model":model,"choices":[{"finish_reason":"stop","message":{"content":answer}}],"usage":{"prompt_tokens":100,"completion_tokens":10}})
                    },
                );
            });
            let context = AuthorizedContext {
                discord: Some(brain_contracts::DiscordRequestContext {
                    user_id: Some(42),
                    request_id: "fixture-request".into(),
                    scope: "discord.request:fixture-owner".into(),
                    allow_discord_reads: false,
                }),
                principal: Principal {
                    actor_id: "fixture".into(),
                    channel: "discord".into(),
                    scopes: BTreeSet::from([
                        "bot.public".into(),
                        "discord.request:fixture-owner".into(),
                    ]),
                    provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
                },
                conversation_id: "fixture-conversation".into(),
                knowledge_release: "fixture".into(),
                deadline_ms: 10000,
                budget: brain_contracts::Budget::default(),
                request_deadline: None,
            };
            let query: Query = serde_json::from_value(json!({"request_id":"fixture-request","conversation_id":"fixture-conversation","text":"Bin ich eingeladen? Chatkontext: private Zusatzkennung 76561197960265839","requested_scopes":["bot.public"]})).unwrap();
            let adapter = DiscordRetriever::new(NoStoredReads, Some(Arc::new(live)));
            let unobserved = brain_contracts::invite::status_evidence(
                &brain_contracts::invite::SelfInviteStatus {
                    status: brain_contracts::invite::InviteStatus::Sent,
                    at: None,
                },
                "discord.request:fixture-owner".into(),
            )
            .unwrap();
            assert!(adapter
                .validate_evidence(&query, &context, &[unobserved], true)
                .is_err());
            let result = Kernel::new(adapter, OpenAiCompatibleProvider::new(config).unwrap())
                .answer_for_publication(&query, &context);
            assert_eq!(result.status, AnswerStatus::Answered);
            assert_eq!(result.citations.len(), 1);
            assert_eq!(result.usage.network_rounds, 2);
            server.join().unwrap();
        }
    }

    #[test]
    #[ignore = "Prüft den laufenden Statusweg nur mit einer synthetischen, unbekannten Person."]
    fn live_statuswerkzeug_ist_gelistet_und_lehnt_unbekannte_person_ab() {
        let config = crate::Config::load(std::path::Path::new(
            "/home/nathanael/.config/deadlock-brain/brain-serve.json",
        ))
        .expect("Bestehende Brainkonfiguration muss gültig sein.");
        let secrets = crate::Secrets::load_until(
            &config,
            std::path::Path::new("/etc/deadlock-brain/infisical.json"),
            Instant::now() + Duration::from_secs(30),
        )
        .expect("Bestehender Infisicalresolver muss verfügbar sein.");
        let live = DiscordLive::new(
            secrets
                .discord_live_token
                .expect("Bestehende interne Zugangsreferenz muss verfügbar sein."),
        )
        .unwrap();
        let response = live
            .http
            .post(&live.endpoint)
            .bearer_auth(&live.token)
            .json(&json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}))
            .send()
            .expect("MCP muss erreichbar sein.");
        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(response.headers()["content-type"], "application/json");
        let rpc: Value = response.json().expect("MCP muss JSON liefern.");
        let tool = rpc["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|tool| tool["name"] == "self_invite_status")
            .expect("Eigener Status muss im laufenden MCP angeboten werden.");
        assert_eq!(
            tool["inputSchema"],
            json!({"type":"object","properties":{},"additionalProperties":false})
        );
        let response = live
            .http
            .post(&live.endpoint)
            .bearer_auth(&live.token)
            .header("x-discord-request-id", "synthetic-v-live-check")
            .header("x-discord-user-id", "42")
            .json(&json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"self_invite_status","arguments":{}}}))
            .send()
            .expect("MCP muss erreichbar sein.");
        assert_eq!(response.status().as_u16(), 403);
        assert_eq!(response.headers()["content-type"], "application/json");
        assert_eq!(
            response.json::<Value>().unwrap(),
            json!({"error":"forbidden"})
        );
        println!("Live-MCP: self_invite_status gelistet; synthetische Person sicher abgelehnt.");
    }

    #[test]
    fn private_read_gate_verhindert_io_und_erhaelt_gespeichertes_wissen() {
        use brain_contracts::{AnswerProviderPort, AnswerStatus, ProviderAnswer};
        use brain_kernel::{AnswerKernelPort, Kernel};
        use std::net::TcpListener;

        struct SafeStore(Evidence);
        impl RetrievalPort for SafeStore {
            fn retrieve(
                &self,
                _: &Query,
                context: &AuthorizedContext,
            ) -> Result<Vec<Evidence>, PortError> {
                let request = context.discord.as_ref().unwrap();
                assert_eq!(request.user_id, Some(42));
                assert!(context.principal.scopes.contains(&request.scope));
                Ok(vec![self.0.clone()])
            }
            fn validate_evidence(
                &self,
                _: &Query,
                _: &AuthorizedContext,
                items: &[Evidence],
                _: bool,
            ) -> Result<(), PortError> {
                if items == std::slice::from_ref(&self.0) {
                    Ok(())
                } else {
                    Err(denied())
                }
            }
            fn validate_publication(
                &self,
                query: &Query,
                context: &AuthorizedContext,
                items: &[Evidence],
            ) -> Result<(), PortError> {
                self.validate_evidence(query, context, items, false)
            }
        }
        struct Answer;
        impl AnswerProviderPort for Answer {
            fn answer(
                &self,
                query: &Query,
                _: &AuthorizedContext,
                items: &[Evidence],
            ) -> Result<ProviderAnswer, PortError> {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].source_id, "docs.public");
                let payload = brain_contracts::provider_input::grounded_messages(query, items);
                let data: Value = serde_json::from_str(&payload[1].content).unwrap();
                assert_eq!(data["query"], query.text);
                assert!(!payload[1].content.contains("discord.request:"));
                Ok(ProviderAnswer {
                    text: "Antwort aus gespeichertem Wissen".into(),
                    cited_evidence_ids: vec![items[0].evidence_id.clone()],
                    usage: Usage::default(),
                })
            }
        }
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let mut live = DiscordLive::new("fixture".into()).unwrap();
        live.endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
        let live = Arc::new(live);
        let context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: Some(42),
                request_id: "r".into(),
                scope: "discord.request:fixture".into(),
                allow_discord_reads: false,
            }),
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "fixture".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:fixture".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: "c".into(),
            knowledge_release: "fixture".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget::default(),
            request_deadline: None,
        };
        for channel in [None, Some(2)] {
            assert!(matches!(
                live.read_channel(&context, channel),
                Err(PortError::PermissionDenied(_))
            ));
        }
        let stored = Evidence {
            evidence_id: "stored".into(),
            source_id: "docs.public".into(),
            logical_id: "fixture".into(),
            revision: 1,
            kind: EvidenceKind::Prose,
            content: "Neutrales gespeichertes Wissen".into(),
            citation: "https://example.invalid/fixture".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            score: 1.0,
            provenance: None,
            patch: None,
        };
        let adapter = DiscordRetriever::new(SafeStore(stored.clone()), Some(live.clone()));
        let kernel = Kernel::new(adapter, Answer);
        for text in [
            "Was macht Abrams?",
            "Wo stehen die Discord-Serverregeln?",
            "Wie funktioniert der Invite-Bot?",
            "Welche Lanes gibt es?",
        ] {
            let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":text,"requested_scopes":["bot.public"]})).unwrap();
            let result = kernel.answer_for_publication(&query, &context);
            assert_eq!(result.status, AnswerStatus::Answered);
            assert_eq!(result.citations, vec![stored.clone()]);
            assert_eq!(result.usage.network_rounds, 0);
        }
        for rounds in [0, 1] {
            let mut context = context.clone();
            context.discord.as_mut().unwrap().allow_discord_reads = true;
            context.budget.max_network_rounds = rounds;
            let adapter = DiscordRetriever::new(SafeStore(stored.clone()), Some(live.clone()));
            for text in [
                "Was macht Abrams?",
                "Welche Lanes gibt es?",
                "What is this?",
            ] {
                let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":text,"requested_scopes":["bot.public"]})).unwrap();
                let (items, usage) = adapter.retrieve_with_usage(&query, &context).unwrap();
                assert_eq!(items, vec![stored.clone()]);
                assert_eq!(usage.network_rounds, 0);
                adapter
                    .validate_evidence(&query, &context, &items, true)
                    .unwrap();
                adapter
                    .validate_publication(&query, &context, &items)
                    .unwrap();
            }
        }
        let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":"Welche Lanes gibt es?","requested_scopes":["bot.public"],"domain":{"kind":"rule","rule_id":"fixture-rule"}})).unwrap();
        let mut context = context.clone();
        context.discord.as_mut().unwrap().allow_discord_reads = true;
        let adapter = DiscordRetriever::new(SafeStore(stored.clone()), Some(live.clone()));
        let (items, usage) = adapter.retrieve_with_usage(&query, &context).unwrap();
        assert_eq!(items, vec![stored.clone()]);
        assert_eq!(usage.network_rounds, 0);
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
        drop(listener);
        let mut query = query;
        query.domain = None;
        let facts = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"requester","channels":[],"voice_counts":[],"bot_infos":[]});
        let (expires, mut old) = evidence(serde_json::from_value(facts).unwrap()).unwrap();
        old.allowed_scopes = BTreeSet::from([context.discord.as_ref().unwrap().scope.clone()]);
        let key = observation_key(&query, &context).unwrap();
        adapter
            .observations
            .lock()
            .unwrap()
            .insert(key.clone(), (expires, vec![old.clone()]));
        adapter
            .validate_evidence(&query, &context, std::slice::from_ref(&old), true)
            .unwrap();
        let (items, usage) = adapter.retrieve_with_usage(&query, &context).unwrap();
        assert_eq!(items, vec![stored.clone()]);
        assert_eq!(usage.network_rounds, 1);
        assert!(!adapter.observations.lock().unwrap().contains_key(&key));
        for provider in [false, true] {
            adapter
                .validate_evidence(&query, &context, &items, provider)
                .unwrap();
            assert!(adapter
                .validate_evidence(&query, &context, std::slice::from_ref(&old), provider)
                .is_err());
        }
        adapter
            .validate_publication(&query, &context, &items)
            .unwrap();
        assert!(adapter
            .validate_publication(&query, &context, &[old])
            .is_err());
        let mut invite = query.clone();
        invite.text = brain_contracts::invite::QUESTION.into();
        assert!(adapter.retrieve_with_usage(&invite, &context).is_err());
        let result = Kernel::new(
            DiscordRetriever::new(SafeStore(stored.clone()), Some(live)),
            Answer,
        )
        .answer_for_publication(&query, &context);
        assert_eq!(result.status, AnswerStatus::Answered);
        assert_eq!(result.citations, vec![stored]);
        assert_eq!(result.usage.network_rounds, 1);
        let start = Instant::now();
        context.request_deadline = Some(brain_contracts::RequestDeadline::after_with_clock(
            Duration::ZERO,
            move || start,
        ));
        assert!(matches!(
            adapter.retrieve_with_usage(&query, &context),
            Err(PortError::BudgetExceeded)
        ));
    }

    #[test]
    fn private_read_gate_sperrt_live_evidence_auch_bei_vorhandener_beobachtung() {
        let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":"Welche Discord-Lanes gibt es?","requested_scopes":["bot.public"]})).unwrap();
        let mut context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: Some(42),
                request_id: "r".into(),
                scope: "discord.request:fixture".into(),
                allow_discord_reads: true,
            }),
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "fixture".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:fixture".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: "c".into(),
            knowledge_release: "fixture".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget::default(),
            request_deadline: None,
        };
        let facts = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"requester","channels":[],"voice_counts":[],"bot_infos":[]});
        let (expires, item) = evidence(serde_json::from_value(facts).unwrap()).unwrap();
        let items = vec![item];
        let adapter = DiscordRetriever::new(Stored, None);
        let public_key = observation_key(&query, &context).unwrap();
        adapter
            .observations
            .lock()
            .unwrap()
            .insert(public_key.clone(), (expires, items.clone()));
        adapter
            .validate_evidence(&query, &context, &items, true)
            .unwrap();
        context.discord.as_mut().unwrap().allow_discord_reads = false;
        let private_key = observation_key(&query, &context).unwrap();
        assert_ne!(public_key, private_key);
        adapter
            .observations
            .lock()
            .unwrap()
            .insert(private_key, (expires, items.clone()));
        for provider in [false, true] {
            assert!(matches!(
                adapter.validate_evidence(&query, &context, &items, provider),
                Err(PortError::PermissionDenied(_))
            ));
        }
        assert!(matches!(
            adapter.validate_publication(&query, &context, &items),
            Err(PortError::PermissionDenied(_))
        ));
    }

    #[test]
    fn grosse_live_panels_und_doku_passen_gemeinsam_ins_providerbudget() {
        use brain_contracts::{provider_input::grounded_input_ceiling, AnswerProviderPort};
        use brain_providers::{OpenAiCompatibleProvider, PriceCeiling, ProviderConfig};
        use std::{
            io::{BufRead, BufReader, Write},
            net::{TcpListener, TcpStream},
        };

        fn request(stream: &mut TcpStream) -> Value {
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut reader = BufReader::new(stream);
            let mut length = 0;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_lowercase().strip_prefix("content-length:") {
                    length = value.trim().parse::<usize>().unwrap();
                }
            }
            let mut bytes = vec![0; length];
            reader.read_exact(&mut bytes).unwrap();
            serde_json::from_slice(&bytes).unwrap()
        }
        fn reply(stream: &mut TcpStream, body: Value) {
            let body = body.to_string();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            )
            .unwrap();
        }
        struct Documents(Vec<Evidence>);
        impl RetrievalPort for Documents {
            fn retrieve(
                &self,
                _: &Query,
                _: &AuthorizedContext,
            ) -> Result<Vec<Evidence>, PortError> {
                Ok(self.0.clone())
            }
            fn validate_evidence(
                &self,
                _: &Query,
                _: &AuthorizedContext,
                items: &[Evidence],
                _: bool,
            ) -> Result<(), PortError> {
                if items.iter().all(|item| self.0.contains(item)) {
                    Ok(())
                } else {
                    Err(denied())
                }
            }
            fn validate_publication(
                &self,
                query: &Query,
                context: &AuthorizedContext,
                items: &[Evidence],
            ) -> Result<(), PortError> {
                self.validate_evidence(query, context, items, false)
            }
        }
        for panel_bytes in [850, 3000] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let endpoint = format!("http://{}", listener.local_addr().unwrap());
            let mut live = DiscordLive::new("test".into()).unwrap();
            live.endpoint = format!("{endpoint}/mcp");
            let channels = vec![
                json!({"id":"2","name":"Anleitungen","type":0,"topic":null,"position":0,"parent_id":null}),
                json!({"id":"3","name":"Anfänger-Lane","type":2,"topic":null,"position":1,"parent_id":null}),
                json!({"id":"4","name":"Ranked-Lane","type":2,"topic":null,"position":2,"parent_id":null}),
                json!({"id":"5","name":"Turnier-Lane","type":2,"topic":null,"position":3,"parent_id":null}),
                json!({"id":"6","name":"AFK","type":2,"topic":null,"position":4,"parent_id":null}),
                json!({"id":"7","name":"Team 1","type":2,"topic":null,"position":5,"parent_id":null}),
                json!({"id":"8","name":"Caster Channel","type":2,"topic":null,"position":6,"parent_id":null}),
            ];
            let panels: Vec<_> = (0..5).map(|i| json!({"channel_id":"2","message_id":format!("{}", 10+i),"text":format!("Öffentliches Panel {i}: {}", "a".repeat(panel_bytes))})).collect();
            let facts = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"verified_members","channels":channels,"voice_counts":[{"channel_id":"3","count":3},{"channel_id":"4","count":4},{"channel_id":"5","count":5}],"bot_infos":panels});
            let full_live = evidence(serde_json::from_value(facts.clone()).unwrap())
                .unwrap()
                .1;
            let documents: Vec<_> = (0..6)
                .map(|i| {
                    let mut item = full_live.clone();
                    item.source_id = "docs.public".into();
                    item.citation = "https://example.invalid/serverwissen".into();
                    item.evidence_id = format!("doc-{i}");
                    let prefix = if i == 0 {
                        "In der Willkommens-DM kannst du Casual, Ranked oder Street Brawl wählen.\n"
                    } else {
                        ""
                    };
                    item.content = format!(
                        "{prefix}{}",
                        "d".repeat((if i == 0 { 992 } else { 991 }) - prefix.len())
                    );
                    item
                })
                .collect();
            assert_eq!(
                documents
                    .iter()
                    .map(|item| item.content.len())
                    .sum::<usize>(),
                5947
            );
            let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":"Welche Lanes gibt es auf dem Discord-Server?","requested_scopes":["bot.public"], "answer_context":{"platform":"discord", "channel_name":"Hilfe", "category_name":"Community", "is_thread":false, "is_direct_message":false, "input_kind":"mention"}})).unwrap();
            let mut context = AuthorizedContext {
                discord: Some(brain_contracts::DiscordRequestContext {
                    user_id: None,
                    request_id: "r".into(),
                    scope: "discord.request:unbound".into(),
                    allow_discord_reads: true,
                }),
                principal: brain_contracts::Principal {
                    actor_id: "bot".into(),
                    channel: "test".into(),
                    scopes: BTreeSet::from(["bot.public".into(), "discord.request:unbound".into()]),
                    provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
                },
                conversation_id: "c".into(),
                knowledge_release: "release".into(),
                deadline_ms: 10000,
                budget: brain_contracts::Budget {
                    max_network_rounds: 2,
                    max_input_tokens: 12000,
                    max_output_tokens: 4096,
                    max_cost_micros: 12768,
                },
                request_deadline: None,
            };
            let mut unpacked = vec![full_live];
            unpacked.extend(documents.clone());
            assert!(grounded_input_ceiling(&query, &unpacked) + 4096 > 12768);
            let worker = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                assert_eq!(
                    request(&mut stream)["params"]["name"],
                    "public_server_facts"
                );
                reply(
                    &mut stream,
                    json!({"jsonrpc":"2.0","id":1,"result":{"isError":false,"content":[{"type":"text","text":facts.to_string()}]}}),
                );
                drop(stream);
                let (mut stream, _) = listener.accept().unwrap();
                let payload = request(&mut stream);
                let instructions = payload["messages"][0]["content"].as_str().unwrap();
                assert!(instructions.contains("aktuell offenen Lanes"));
                assert!(instructions.contains("Erkläre diese Unterscheidung nicht im Antworttext"));
                assert!(!instructions.contains("Casual"));
                assert_eq!(payload["max_tokens"], 4096);
                assert!(
                    brain_contracts::provider_input::transport_input_ceiling(&payload, true)
                        .unwrap()
                        <= 8672
                );
                let supplied: Value =
                    serde_json::from_str(payload["messages"][1]["content"].as_str().unwrap())
                        .unwrap();
                assert_eq!(supplied["answer_context"]["channel_name"], "Hilfe");
                assert_eq!(supplied["answer_context"]["input_kind"], "mention");
                assert!(!payload
                    .to_string()
                    .contains("https://discord.com/channels/1"));
                println!("Provider-Input: Ortskontext Hilfe / Community / mention angekommen");
                let ids: Vec<_> = supplied["evidence"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|item| item["content"].as_str().unwrap().contains("Lane"))
                    .map(|item| item["id"].clone())
                    .collect();
                assert!(!ids.is_empty());
                assert!(supplied["evidence"].as_array().unwrap().iter().any(|item| {
                    item["id"] == "doc-0"
                        && item["content"]
                            .as_str()
                            .unwrap()
                            .contains("Casual, Ranked oder Street Brawl")
                }));
                let answer = json!({"text":"Es gibt Anfänger-Lane, Ranked-Lane und Turnier-Lane.","cited_evidence_ids":ids});
                reply(
                    &mut stream,
                    json!({"model":"fixture-model","choices":[{"message":{"content":answer.to_string()},"finish_reason":"stop"}],"usage":{"prompt_tokens":1000,"completion_tokens":40}}),
                );
            });
            let price = PriceCeiling {
                input_micros_per_token: 1,
                output_micros_per_token: 1,
            };
            let mut adapter = DiscordRetriever::new(Documents(documents), Some(Arc::new(live)));
            adapter.pricing = Some(price);
            let (items, usage) = adapter.retrieve_with_usage(&query, &context).unwrap();
            assert_eq!(usage.network_rounds, 1);
            assert!(grounded_input_ceiling(&query, &items) <= 8672);
            assert_eq!(items[0].evidence_id, "doc-0");
            assert!(items.iter().any(|item| item.evidence_id == "doc-0"));
            let content = items
                .iter()
                .map(|item| item.content.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            for lane in [
                "Anfänger-Lane",
                "Ranked-Lane",
                "Turnier-Lane",
                "3 anwesend",
                "4 anwesend",
                "5 anwesend",
            ] {
                assert!(
                    content.contains(lane),
                    "panel_bytes={panel_bytes}, fehlend={lane}, Eingabeobergrenze={}, Dokumente={}, Live-Belege={}",
                    grounded_input_ceiling(&query, &items),
                    items.iter().filter(|item| item.source_id == "docs.public").count(),
                    items.iter().filter(|item| item.source_id == SOURCE).count()
                );
            }
            for provider in [false, true] {
                adapter
                    .validate_evidence(&query, &context, &items, provider)
                    .unwrap();
            }
            adapter
                .validate_publication(&query, &context, &items)
                .unwrap();
            let mut changed = items.clone();
            changed[0].content.push_str("verändert");
            assert!(adapter
                .validate_publication(&query, &context, &changed)
                .is_err());
            context.budget.max_network_rounds -= usage.network_rounds;
            let mut config = ProviderConfig::new("fixture", endpoint, "fixture-model");
            config.pricing = Some(price);
            let answer = OpenAiCompatibleProvider::new(config)
                .unwrap()
                .answer(&query, &context, &items)
                .unwrap();
            assert!(answer.text.contains("Turnier-Lane"));
            assert_eq!(answer.usage.network_rounds + usage.network_rounds, 2);
            worker.join().unwrap();
            let mut other_query = query.clone();
            other_query.answer_context = None;
            assert!(adapter
                .validate_evidence(&other_query, &context, &items, true)
                .is_err());
            other_query = query.clone();
            other_query.request_id = "anderer-Aufruf".into();
            assert!(adapter
                .validate_evidence(&other_query, &context, &items, true)
                .is_err());
            adapter
                .observations
                .lock()
                .unwrap()
                .get_mut(&observation_key(&query, &context).unwrap())
                .unwrap()
                .0 = Instant::now();
            assert!(adapter
                .validate_publication(&query, &context, &items)
                .is_err());
        }
    }

    #[test]
    fn packing_reserviert_kosten_nach_bisherigem_verbrauch() {
        let mut adapter = DiscordRetriever::new(Stored, None);
        adapter.pricing = Some(brain_providers::PriceCeiling {
            input_micros_per_token: 2,
            output_micros_per_token: 3,
        });
        let context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: None,
                request_id: "r".into(),
                scope: "discord.request:unbound".into(),
                allow_discord_reads: true,
            }),
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::new(),
            },
            conversation_id: "c".into(),
            knowledge_release: "release".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget {
                max_network_rounds: 2,
                max_input_tokens: 12000,
                max_output_tokens: 4096,
                max_cost_micros: 12768,
            },
            request_deadline: None,
        };
        let usage = Usage {
            network_rounds: 1,
            input_tokens: 100,
            output_tokens: 10,
            cost_micros: 50,
            ..Usage::default()
        };
        let packed = adapter.packing_context(&context, &usage).unwrap();
        assert_eq!(
            packed.budget.max_input_tokens,
            (12768 - 50 - (4096 - 10) * 3) / 2
        );
        adapter.pricing.as_mut().unwrap().output_micros_per_token = u64::MAX;
        assert!(matches!(
            adapter.packing_context(&context, &usage),
            Err(PortError::BudgetExceeded)
        ));
    }

    #[test]
    #[ignore = "Liest ausschließlich öffentliche Live-Fakten über den bestehenden Infisicalzugang."]
    fn oeffentliche_live_fakten_ueber_bestehenden_resolver() {
        let config = crate::Config::load(std::path::Path::new(
            "/home/nathanael/.config/deadlock-brain/brain-serve.json",
        ))
        .expect("Bestehende Brainkonfiguration muss gültig sein.");
        let secrets = crate::Secrets::load_until(
            &config,
            std::path::Path::new("/etc/deadlock-brain/infisical.json"),
            std::time::Instant::now() + Duration::from_secs(30),
        )
        .expect("Bestehender Infisicalresolver muss verfügbar sein.");
        let live = DiscordLive::new(
            secrets
                .discord_live_token
                .expect("Bestehende interne Tokenreferenz muss verfügbar sein."),
        )
        .unwrap();
        let context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: None,
                request_id: "r".into(),
                scope: "discord.request:unbound".into(),
                allow_discord_reads: true,
            }),
            principal: brain_contracts::Principal {
                actor_id: "interne-live-pruefung".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:unbound".into()]),
                provider_egress: BTreeSet::new(),
            },
            conversation_id: "interne-live-pruefung".into(),
            knowledge_release: "live-pruefung".into(),
            deadline_ms: 15000,
            budget: brain_contracts::Budget::default(),
            request_deadline: Some(brain_contracts::RequestDeadline::after(
                Duration::from_secs(15),
            )),
        };
        let facts = live
            .read(&context)
            .expect("Öffentliche MCP-Fakten müssen lesbar sein.");
        println!("{}", facts.1.content);
    }

    struct Stored;
    impl RetrievalPort for Stored {
        fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
            Ok(Vec::new())
        }
        fn validate_evidence(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            items: &[Evidence],
            _: bool,
        ) -> Result<(), PortError> {
            self.validate_publication(query, context, items)
        }
        fn validate_publication(
            &self,
            _: &Query,
            _: &AuthorizedContext,
            _: &[Evidence],
        ) -> Result<(), PortError> {
            Err(denied())
        }
    }

    #[test]
    fn reine_live_packs_werden_lokal_und_leere_packs_ablehnend_geprueft() {
        let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":"Wo finde ich Leute zum Spielen?","requested_scopes":["bot.public"]})).unwrap();
        let context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: None,
                request_id: "r".into(),
                scope: "discord.request:unbound".into(),
                allow_discord_reads: true,
            }),
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:unbound".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: "c".into(),
            knowledge_release: "release".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget::default(),
            request_deadline: None,
        };
        let facts = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"verified_members","channels":[],"voice_counts":[],"bot_infos":[]});
        let (expires, item) = evidence(serde_json::from_value(facts).unwrap()).unwrap();
        let adapter = DiscordRetriever::new(Stored, None);
        let items = vec![item.clone()];
        adapter.observations.lock().unwrap().insert(
            observation_key(&query, &context).unwrap(),
            (expires, items.clone()),
        );
        for provider in [false, true] {
            assert!(Stored
                .validate_evidence(&query, &context, &[], provider)
                .is_err());
            assert!(adapter
                .validate_evidence(&query, &context, &[], provider)
                .is_err());
            adapter
                .validate_evidence(&query, &context, &items, provider)
                .unwrap();
        }
        assert!(adapter.validate_publication(&query, &context, &[]).is_err());
        adapter
            .validate_publication(&query, &context, &items)
            .unwrap();
        let mut stored = item;
        stored.source_id = "docs.public".into();
        let mut mixed = items;
        mixed.push(stored);
        for provider in [false, true] {
            assert!(adapter
                .validate_evidence(&query, &context, &mixed, provider)
                .is_err());
        }
        assert!(adapter
            .validate_publication(&query, &context, &mixed)
            .is_err());
    }

    #[test]
    fn historische_live_fragen_erreichen_den_bestehenden_nichts_fall() {
        use brain_contracts::{AnswerProviderPort, AnswerStatus, ProviderAnswer};
        use brain_kernel::{AnswerKernelPort, Kernel};
        use std::{
            io::{BufRead, BufReader, Write},
            net::TcpListener,
        };

        struct KeineHistorie;
        impl AnswerProviderPort for KeineHistorie {
            fn answer(
                &self,
                query: &Query,
                _: &AuthorizedContext,
                items: &[Evidence],
            ) -> Result<ProviderAnswer, PortError> {
                assert!(!items.is_empty());
                assert!(items.iter().all(|item| item.source_id == SOURCE));
                let messages = brain_contracts::provider_input::grounded_messages(query, items);
                assert!(messages[0]
                    .content
                    .contains("Aktuelle Live-Fakten belegen keinen historischen Zustand"));
                Ok(ProviderAnswer {
                    text: String::new(),
                    cited_evidence_ids: Vec::new(),
                    usage: Usage {
                        network_rounds: 1,
                        ..Usage::default()
                    },
                })
            }
        }
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut live = DiscordLive::new("test".into()).unwrap();
        live.endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            for _ in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut reader = BufReader::new(&mut stream);
                let mut length = 0;
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line == "\r\n" {
                        break;
                    }
                    if let Some(value) = line.to_lowercase().strip_prefix("content-length:") {
                        length = value.trim().parse::<usize>().unwrap();
                    }
                }
                reader.read_exact(&mut vec![0; length]).unwrap();
                let facts = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"verified_members","channels":[{"id":"2","name":"Coaching Lane","type":2,"topic":null,"position":0,"parent_id":null}],"voice_counts":[],"bot_infos":[]});
                let reply = json!({"jsonrpc":"2.0","id":1,"result":{"isError":false,"content":[{"type":"text","text":facts.to_string()}]}}).to_string();
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    reply.len(),
                    reply
                )
                .unwrap();
            }
        });
        let kernel = Kernel::new(
            DiscordRetriever::new(Stored, Some(Arc::new(live))),
            KeineHistorie,
        );
        let context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: None,
                request_id: "r".into(),
                scope: "discord.request:unbound".into(),
                allow_discord_reads: true,
            }),
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:unbound".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: "c".into(),
            knowledge_release: "release".into(),
            deadline_ms: 10000,
            budget: brain_contracts::Budget {
                max_network_rounds: 2,
                max_input_tokens: 12000,
                max_output_tokens: 4096,
                max_cost_micros: 12768,
            },
            request_deadline: None,
        };
        for date in ["2018", "am 1. Januar 2019"] {
            let mut context = context.clone();
            context.discord.as_mut().unwrap().request_id = date.into();
            let query: Query = serde_json::from_value(json!({"request_id":date,"conversation_id":"c","text":format!("Welche Lanes gab es auf dem Discord-Server {date}?"),"requested_scopes":["bot.public"]})).unwrap();
            let answer = kernel.answer_for_publication(&query, &context);
            assert_eq!(answer.status, AnswerStatus::InsufficientEvidence);
            assert_eq!(
                answer.text,
                "Dazu hab ich gerade nichts Genaues, frag am besten direkt im Discord nach."
            );
            assert!(answer.citations.is_empty());
            assert_eq!(answer.usage.network_rounds, 2);
        }
        server.join().unwrap();
    }

    #[test]
    fn alte_ungueltige_und_zukuenftige_beobachtungen_werden_verworfen() {
        for observed_at in [
            "".to_owned(),
            "jetzt".to_owned(),
            "2026-02-30T12:00:00Z".to_owned(),
            "2020-01-01T00:00:00Z".to_owned(),
            (Utc::now() - Duration::from_secs(60)).to_rfc3339(),
            (Utc::now() + Duration::from_secs(60)).to_rfc3339(),
        ] {
            let facts = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":observed_at,"cache_seconds":60,"audience":"verified_members","channels":[],"voice_counts":[],"bot_infos":[]});
            assert!(evidence(serde_json::from_value(facts).unwrap()).is_err());
        }
    }

    #[test]
    fn kurze_cachezeit_beginnt_beim_beobachten_und_laeuft_lokal_ab() {
        use std::{
            io::{BufRead, BufReader, Write},
            net::TcpListener,
        };
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut live = DiscordLive::new("test".into()).unwrap();
        live.endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
        let adapter = DiscordRetriever::new(Stored, Some(Arc::new(live)));
        let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":"Welche Lanes gibt es?","requested_scopes":["bot.public"]})).unwrap();
        let mut context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: None,
                request_id: "r".into(),
                scope: "discord.request:unbound".into(),
                allow_discord_reads: true,
            }),
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:unbound".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: "c".into(),
            knowledge_release: "release".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget::default(),
            request_deadline: None,
        };
        let worker = std::thread::spawn(move || {
            for age_ms in [250, 2000] {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut reader = BufReader::new(&mut stream);
                let mut length = 0;
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line == "\r\n" {
                        break;
                    }
                    if let Some(value) = line.to_lowercase().strip_prefix("content-length:") {
                        length = value.trim().parse::<usize>().unwrap();
                    }
                }
                reader.read_exact(&mut vec![0; length]).unwrap();
                let observed_at = (Utc::now() - Duration::from_millis(age_ms)).to_rfc3339();
                let facts = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":observed_at,"cache_seconds":1,"audience":"verified_members","channels":[],"voice_counts":[],"bot_infos":[]});
                let reply = json!({"jsonrpc":"2.0","id":1,"result":{"isError":false,"content":[{"type":"text","text":facts.to_string()}]}}).to_string();
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    reply.len(),
                    reply
                )
                .unwrap();
            }
        });
        let (items, usage) = adapter.retrieve_with_usage(&query, &context).unwrap();
        assert_eq!(usage.network_rounds, 1);
        let expires =
            adapter.observations.lock().unwrap()[&observation_key(&query, &context).unwrap()].0;
        assert!(expires.saturating_duration_since(Instant::now()) <= Duration::from_millis(750));
        context.budget.max_network_rounds = 0;
        for provider in [false, true] {
            adapter
                .validate_evidence(&query, &context, &items, provider)
                .unwrap();
        }
        adapter
            .validate_publication(&query, &context, &items)
            .unwrap();
        std::thread::sleep(
            expires.saturating_duration_since(Instant::now()) + Duration::from_millis(20),
        );
        for provider in [false, true] {
            assert!(adapter
                .validate_evidence(&query, &context, &items, provider)
                .is_err());
        }
        assert!(adapter
            .validate_publication(&query, &context, &items)
            .is_err());
        context.budget.max_network_rounds = 1;
        assert!(adapter.retrieve_with_usage(&query, &context).is_err());
        worker.join().unwrap();
    }

    #[test]
    fn netzwerkbudget_erfasst_den_abruf_und_validierung_bleibt_lokal() {
        use std::{
            io::{BufRead, BufReader, Write},
            net::TcpListener,
        };
        struct ChargedStored(u32);
        impl RetrievalPort for ChargedStored {
            fn retrieve(
                &self,
                _: &Query,
                _: &AuthorizedContext,
            ) -> Result<Vec<Evidence>, PortError> {
                Ok(Vec::new())
            }
            fn retrieve_with_usage(
                &self,
                _: &Query,
                _: &AuthorizedContext,
            ) -> Result<(Vec<Evidence>, Usage), PortError> {
                Ok((
                    Vec::new(),
                    Usage {
                        network_rounds: self.0,
                        ..Usage::default()
                    },
                ))
            }
        }
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut live = DiscordLive::new("test".into()).unwrap();
        live.endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
        let live = Arc::new(live);
        let adapter = DiscordRetriever::new(Stored, Some(live.clone()));
        let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":"Welche Lanes gibt es?","requested_scopes":["bot.public"]})).unwrap();
        let mut context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: None,
                request_id: "r".into(),
                scope: "discord.request:unbound".into(),
                allow_discord_reads: true,
            }),
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:unbound".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: "c".into(),
            knowledge_release: "release".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget::default(),
            request_deadline: None,
        };
        let public = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"verified_members","channels":[{"id":"2","name":"Lane","type":2,"topic":null,"position":0,"parent_id":null}],"voice_counts":[{"channel_id":"2","count":3}],"bot_infos":[]});
        let item = evidence(serde_json::from_value(public.clone()).unwrap())
            .unwrap()
            .1;
        context.budget.max_network_rounds = 0;
        assert!(matches!(
            live.read(&context),
            Err(PortError::BudgetExceeded)
        ));
        assert!(matches!(
            adapter.retrieve_with_usage(&query, &context),
            Err(PortError::BudgetExceeded)
        ));
        assert!(adapter
            .validate_evidence(&query, &context, std::slice::from_ref(&item), false)
            .is_err());
        assert!(adapter
            .validate_evidence(&query, &context, std::slice::from_ref(&item), true)
            .is_err());
        assert!(adapter
            .validate_publication(&query, &context, &[item])
            .is_err());
        for rounds in [1, u32::MAX] {
            context.budget.max_network_rounds = rounds;
            let charged = DiscordRetriever::new(ChargedStored(rounds), Some(live.clone()));
            assert!(matches!(
                charged.retrieve_with_usage(&query, &context),
                Err(PortError::BudgetExceeded)
            ));
        }
        listener.set_nonblocking(true).unwrap();
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
        listener.set_nonblocking(false).unwrap();
        let server = listener.try_clone().unwrap();
        let reply = json!({"jsonrpc":"2.0","id":1,"result":{"isError":false,"content":[{"type":"text","text":public.to_string()}]}}).to_string();
        let worker = std::thread::spawn(move || {
            let (mut stream, _) = server.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut reader = BufReader::new(&mut stream);
            let mut length = 0;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_lowercase().strip_prefix("content-length:") {
                    length = value.trim().parse::<usize>().unwrap();
                }
            }
            reader.read_exact(&mut vec![0; length]).unwrap();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                reply.len(),
                reply
            )
            .unwrap();
        });
        context.budget.max_network_rounds = 1;
        let (items, usage) = adapter.retrieve_with_usage(&query, &context).unwrap();
        assert_eq!(usage.network_rounds, 1);
        worker.join().unwrap();
        context.budget.max_network_rounds = 0;
        adapter
            .validate_evidence(&query, &context, &items, false)
            .unwrap();
        adapter
            .validate_evidence(&query, &context, &items, true)
            .unwrap();
        adapter
            .validate_publication(&query, &context, &items)
            .unwrap();
        let mut changed = items.clone();
        changed[0].content.push_str("privat");
        assert!(adapter
            .validate_publication(&query, &context, &changed)
            .is_err());
        let mut restricted = context.clone();
        restricted.principal.provider_egress.clear();
        assert!(adapter
            .validate_evidence(&query, &restricted, &items, true)
            .is_err());
        restricted = context.clone();
        restricted.principal.scopes.clear();
        assert!(adapter
            .validate_publication(&query, &restricted, &items)
            .is_err());
        let mut other_query = query.clone();
        other_query.text.push_str(" Voice");
        assert!(adapter
            .validate_evidence(&other_query, &context, &items, false)
            .is_err());
        let key = observation_key(&query, &context).unwrap();
        adapter
            .observations
            .lock()
            .unwrap()
            .get_mut(&key)
            .unwrap()
            .0 = Instant::now() - Duration::from_secs(61);
        assert!(adapter
            .validate_evidence(&query, &context, &items, false)
            .is_err());
        assert!(adapter
            .validate_publication(&query, &context, &items)
            .is_err());
        listener.set_nonblocking(true).unwrap();
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }

    #[test]
    fn parallele_anfragen_behalten_getrennte_gueltige_beobachtungen() {
        use std::{
            io::{BufRead, BufReader, Write},
            net::TcpListener,
            sync::Barrier,
        };
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut live = DiscordLive::new("test".into()).unwrap();
        live.endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
        let adapter = Arc::new(DiscordRetriever::new(Stored, Some(Arc::new(live))));
        let context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: None,
                request_id: "r".into(),
                scope: "discord.request:unbound".into(),
                allow_discord_reads: true,
            }),
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:unbound".into()]),
                provider_egress: BTreeSet::from(["public".into(), "discord_request".into()]),
            },
            conversation_id: "c".into(),
            knowledge_release: "release".into(),
            deadline_ms: 10000,
            budget: brain_contracts::Budget::default(),
            request_deadline: None,
        };
        let server = std::thread::spawn(move || {
            for count in 1..=130 {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let mut reader = BufReader::new(&mut stream);
                let mut length = 0;
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line == "\r\n" {
                        break;
                    }
                    if let Some(value) = line.to_lowercase().strip_prefix("content-length:") {
                        length = value.trim().parse::<usize>().unwrap();
                    }
                }
                reader.read_exact(&mut vec![0; length]).unwrap();
                let public = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"verified_members","channels":[{"id":"2","name":"Lane","type":2,"topic":null,"position":0,"parent_id":null}],"voice_counts":[{"channel_id":"2","count":count}],"bot_infos":[]});
                let reply = json!({"jsonrpc":"2.0","id":1,"result":{"isError":false,"content":[{"type":"text","text":public.to_string()}]}}).to_string();
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    reply.len(),
                    reply
                )
                .unwrap();
            }
        });
        let barrier = Arc::new(Barrier::new(2));
        let workers: Vec<_> = (0..2)
            .map(|worker| {
                let adapter = adapter.clone();
                let context = context.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    let mut results = Vec::new();
                    for request in 0..65 {
                        let query: Query = serde_json::from_value(json!({"request_id":format!("r-{worker}-{request}"),"conversation_id":"c","text":"Welche Lanes gibt es?","requested_scopes":["bot.public"]})).unwrap();
                        barrier.wait();
                        let (items, usage) = adapter.retrieve_with_usage(&query, &context).unwrap();
                        assert_eq!(usage.network_rounds, 1);
                        results.push((query, items));
                    }
                    results
                })
            })
            .collect();
        let results: Vec<_> = workers
            .into_iter()
            .flat_map(|worker| worker.join().unwrap())
            .collect();
        server.join().unwrap();
        assert_eq!(adapter.observations.lock().unwrap().len(), 130);
        let mut local_context = context;
        local_context.budget.max_network_rounds = 0;
        for (query, items) in &results {
            for provider in [false, true] {
                adapter
                    .validate_evidence(query, &local_context, items, provider)
                    .unwrap();
            }
            adapter
                .validate_publication(query, &local_context, items)
                .unwrap();
        }
        let (first_query, first_items) = &results[0];
        let (other_query, other_items) = &results[65];
        assert_ne!(first_items, other_items);
        assert!(adapter
            .validate_evidence(other_query, &local_context, first_items, true)
            .is_err());
        assert!(adapter
            .validate_publication(first_query, &local_context, other_items)
            .is_err());
    }

    #[test]
    fn statische_sperre_und_providerfreigabe_bleiben_verbindlich() {
        let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":"Welche Lanes gibt es?","requested_scopes":["bot.public"]})).unwrap();
        let context = AuthorizedContext {
            discord: Some(brain_contracts::DiscordRequestContext {
                user_id: None,
                request_id: "r".into(),
                scope: "discord.request:unbound".into(),
                allow_discord_reads: true,
            }),
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into(), "discord.request:unbound".into()]),
                provider_egress: BTreeSet::new(),
            },
            conversation_id: "c".into(),
            knowledge_release: "release".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget::default(),
            request_deadline: None,
        };
        assert!(allowed(&query, &context, false));
        assert!(!allowed(&query, &context, true));
        let adapter = DiscordRetriever::new(Stored, None);
        let mut item = Evidence {
            evidence_id: "e".into(),
            source_id: "docs".into(),
            logical_id: "l".into(),
            revision: 1,
            kind: EvidenceKind::Prose,
            content: "Lane".into(),
            citation: "Discord".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["bot.public".into()]),
            score: 1.0,
            provenance: None,
            patch: None,
        };
        assert!(adapter
            .validate_publication(&query, &context, &[item.clone()])
            .is_err());
        item.source_id = SOURCE.into();
        assert!(adapter
            .validate_publication(&query, &context, &[item])
            .is_err());
    }
    #[test]
    fn private_felder_und_falsche_zulassung_werden_verworfen() {
        let public = json!({"schema":"discord.public-facts.v2","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"verified_members","channels":[{"id":"2","name":"Lane","type":2,"topic":null,"position":0,"parent_id":null}],"voice_counts":[{"channel_id":"2","count":3}],"bot_infos":[]});
        let fact = evidence(serde_json::from_value(public.clone()).unwrap())
            .unwrap()
            .1;
        assert!(fact.content.contains("3 anwesend"));
        assert_eq!(fact.visibility, SourceVisibility::RequestScoped);
        assert_eq!(
            fact.allowed_scopes,
            BTreeSet::from(["discord.request:unbound".into()])
        );
        let mut private = public.clone();
        private["voice_counts"][0]["username"] = json!("privat");
        assert!(serde_json::from_value::<Facts>(private).is_err());
        let mut private = public;
        private["audience"] = json!("administrator");
        assert!(evidence(serde_json::from_value(private).unwrap()).is_err());
    }
}
