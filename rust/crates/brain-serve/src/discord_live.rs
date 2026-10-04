use brain_contracts::{
    AuthorizedContext, Evidence, EvidenceKind, PortError, Query, RetrievalPort, SourceVisibility,
    Usage,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeSet, HashSet},
    io::Read,
    sync::Arc,
    time::Duration,
};

const SOURCE: &str = "discord.public-live.v1";
const ENDPOINT: &str = "http://127.0.0.1:8890/mcp";

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

fn evidence(facts: Facts) -> Result<Evidence, PortError> {
    if facts.schema != "discord.public-facts.v1"
        || facts.audience != "everyone"
        || facts.cache_seconds > 60
        || facts.cache_seconds == 0
        || facts.observed_at.is_empty()
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
    let content = lines.join("\n");
    let digest = format!("{:x}", Sha256::digest(content.as_bytes()));
    Ok(Evidence {
        evidence_id: format!("discord-live-{digest}"),
        source_id: SOURCE.into(),
        logical_id: format!("discord-guild:{}", facts.guild_id),
        revision: 1,
        kind: EvidenceKind::Prose,
        content,
        citation: format!("https://discord.com/channels/{}", facts.guild_id),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::from(["bot.public".into()]),
        score: 100.0,
        provenance: None,
        patch: None,
    })
}

pub(crate) struct DiscordLive {
    http: reqwest::blocking::Client,
    token: String,
}
impl DiscordLive {
    pub(crate) fn new(token: String) -> Result<Self, crate::Error> {
        let http = reqwest::blocking::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|_| crate::Error::ConfigInvalid("discord_live_http"))?;
        Ok(Self { http, token })
    }
    fn read(&self, context: &AuthorizedContext) -> Result<Evidence, PortError> {
        context.check_deadline()?;
        let mut response = self.http.post(ENDPOINT).bearer_auth(&self.token)
            .timeout(context.remaining_time()?.min(Duration::from_secs(15)))
            .json(&json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"public_server_facts","arguments":{}}}))
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
        evidence(
            serde_json::from_str(
                result["content"][0]["text"]
                    .as_str()
                    .ok_or_else(unavailable)?,
            )
            .map_err(|_| denied())?,
        )
    }
}

fn relevant(query: &Query) -> bool {
    let text = query.text.to_lowercase();
    query.domain.is_none()
        && ["discord", "lane", "voice", "kanal", "kanäle", "router"]
            .iter()
            .any(|term| text.contains(term))
}
fn allowed(query: &Query, context: &AuthorizedContext, provider: bool) -> bool {
    query.requested_scopes.contains("bot.public")
        && context.principal.scopes.contains("bot.public")
        && (!provider || context.principal.provider_egress.contains("public"))
}

pub(crate) struct DiscordRetriever<R> {
    inner: R,
    live: Option<Arc<DiscordLive>>,
}
impl<R> DiscordRetriever<R> {
    pub(crate) fn new(inner: R, live: Option<Arc<DiscordLive>>) -> Self {
        Self { inner, live }
    }
    fn validate_live(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        items: &[Evidence],
        provider: bool,
    ) -> Result<(), PortError> {
        if !allowed(query, context, provider) || !relevant(query) {
            return Err(denied());
        }
        let current = self.live.as_ref().ok_or_else(denied)?.read(context)?;
        if items.iter().any(|item| item != &current) {
            return Err(denied());
        }
        Ok(())
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
        let (mut items, mut usage) = self.inner.retrieve_with_usage(query, context)?;
        if relevant(query) && allowed(query, context, false) {
            if let Some(live) = &self.live {
                if context.budget.max_network_rounds < usage.network_rounds.saturating_add(1) {
                    return Err(PortError::BudgetExceeded);
                }
                items.insert(0, live.read(context)?);
                usage.network_rounds += 1;
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
        let (live, stored): (Vec<_>, Vec<_>) = evidence
            .iter()
            .cloned()
            .partition(|e| e.source_id == SOURCE);
        self.inner
            .validate_evidence(query, context, &stored, provider)?;
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
        let (live, stored): (Vec<_>, Vec<_>) = evidence
            .iter()
            .cloned()
            .partition(|e| e.source_id == SOURCE);
        self.inner.validate_publication(query, context, &stored)?;
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
            principal: brain_contracts::Principal {
                actor_id: "interne-live-pruefung".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into()]),
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
        println!("{}", facts.content);
    }

    struct Stored;
    impl RetrievalPort for Stored {
        fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
            Ok(Vec::new())
        }
        fn validate_publication(
            &self,
            _: &Query,
            _: &AuthorizedContext,
            items: &[Evidence],
        ) -> Result<(), PortError> {
            if items.is_empty() {
                Ok(())
            } else {
                Err(denied())
            }
        }
    }

    #[test]
    fn statische_sperre_und_providerfreigabe_bleiben_verbindlich() {
        let query: Query = serde_json::from_value(json!({"request_id":"r","conversation_id":"c","text":"Welche Lanes gibt es?","requested_scopes":["bot.public"]})).unwrap();
        let context = AuthorizedContext {
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into()]),
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
        let public = json!({"schema":"discord.public-facts.v1","guild_id":"1","observed_at":"jetzt","cache_seconds":60,"audience":"everyone","channels":[{"id":"2","name":"Lane","type":2,"topic":null,"position":0,"parent_id":null}],"voice_counts":[{"channel_id":"2","count":3}],"bot_infos":[]});
        let fact = evidence(serde_json::from_value(public.clone()).unwrap()).unwrap();
        assert!(fact.content.contains("3 anwesend"));
        assert_eq!(fact.allowed_scopes, BTreeSet::from(["bot.public".into()]));
        let mut private = public.clone();
        private["voice_counts"][0]["username"] = json!("privat");
        assert!(serde_json::from_value::<Facts>(private).is_err());
        let mut private = public;
        private["audience"] = json!("administrator");
        assert!(evidence(serde_json::from_value(private).unwrap()).is_err());
    }
}
