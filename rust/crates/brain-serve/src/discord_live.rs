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
    if facts.schema != "discord.public-facts.v1"
        || facts.audience != "everyone"
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
    Ok((
        received + remaining,
        Evidence {
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
    fn read(&self, context: &AuthorizedContext) -> Result<(Instant, Evidence), PortError> {
        context.check_deadline()?;
        if context.budget.max_network_rounds == 0 {
            return Err(PortError::BudgetExceeded);
        }
        let mut response = self.http.post(&self.endpoint).bearer_auth(&self.token)
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
    observations: Mutex<BTreeMap<String, (Instant, Evidence)>>,
}

fn observation_key(query: &Query, context: &AuthorizedContext) -> Result<String, PortError> {
    let bytes = serde_json::to_vec(&(
        &query.request_id,
        &query.conversation_id,
        &query.text,
        &query.domain,
        &query.requested_scopes,
        &query.profile,
        &query.patch,
        &query.mode,
        &context.principal,
        &context.conversation_id,
        &context.knowledge_release,
    ))
    .map_err(|_| denied())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

impl<R> DiscordRetriever<R> {
    pub(crate) fn new(inner: R, live: Option<Arc<DiscordLive>>) -> Self {
        Self {
            inner,
            live,
            observations: Mutex::new(BTreeMap::new()),
        }
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
        context.check_deadline()?;
        let key = observation_key(query, context)?;
        let observations = self.observations.lock().map_err(|_| unavailable())?;
        match observations.get(&key) {
            Some((expires, current))
                if Instant::now() < *expires && items.iter().all(|item| item == current) =>
            {
                Ok(())
            }
            _ => Err(denied()),
        }
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
                if context.budget.max_network_rounds <= usage.network_rounds {
                    return Err(PortError::BudgetExceeded);
                }
                let key = observation_key(query, context)?;
                let mut live_context = context.clone();
                live_context.budget.max_network_rounds -= usage.network_rounds;
                let (expires, current) = live.read(&live_context)?;
                usage.network_rounds += 1;
                let mut observations = self.observations.lock().map_err(|_| unavailable())?;
                observations.retain(|_, (expires, _)| Instant::now() < *expires);
                observations.insert(key, (expires, current.clone()));
                items.insert(0, current);
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
    fn alte_ungueltige_und_zukuenftige_beobachtungen_werden_verworfen() {
        for observed_at in [
            "".to_owned(),
            "jetzt".to_owned(),
            "2026-02-30T12:00:00Z".to_owned(),
            "2020-01-01T00:00:00Z".to_owned(),
            (Utc::now() - sqlx::types::chrono::Duration::seconds(60)).to_rfc3339(),
            (Utc::now() + sqlx::types::chrono::Duration::seconds(60)).to_rfc3339(),
        ] {
            let facts = json!({"schema":"discord.public-facts.v1","guild_id":"1","observed_at":observed_at,"cache_seconds":60,"audience":"everyone","channels":[],"voice_counts":[],"bot_infos":[]});
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
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into()]),
                provider_egress: BTreeSet::from(["public".into()]),
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
                let observed_at =
                    (Utc::now() - sqlx::types::chrono::Duration::milliseconds(age_ms)).to_rfc3339();
                let facts = json!({"schema":"discord.public-facts.v1","guild_id":"1","observed_at":observed_at,"cache_seconds":1,"audience":"everyone","channels":[],"voice_counts":[],"bot_infos":[]});
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
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into()]),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            conversation_id: "c".into(),
            knowledge_release: "release".into(),
            deadline_ms: 1000,
            budget: brain_contracts::Budget::default(),
            request_deadline: None,
        };
        let public = json!({"schema":"discord.public-facts.v1","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"everyone","channels":[{"id":"2","name":"Lane","type":2,"topic":null,"position":0,"parent_id":null}],"voice_counts":[{"channel_id":"2","count":3}],"bot_infos":[]});
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
            principal: brain_contracts::Principal {
                actor_id: "bot".into(),
                channel: "test".into(),
                scopes: BTreeSet::from(["bot.public".into()]),
                provider_egress: BTreeSet::from(["public".into()]),
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
                let public = json!({"schema":"discord.public-facts.v1","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"everyone","channels":[{"id":"2","name":"Lane","type":2,"topic":null,"position":0,"parent_id":null}],"voice_counts":[{"channel_id":"2","count":count}],"bot_infos":[]});
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
        let public = json!({"schema":"discord.public-facts.v1","guild_id":"1","observed_at":Utc::now().to_rfc3339(),"cache_seconds":60,"audience":"everyone","channels":[{"id":"2","name":"Lane","type":2,"topic":null,"position":0,"parent_id":null}],"voice_counts":[{"channel_id":"2","count":3}],"bot_infos":[]});
        let fact = evidence(serde_json::from_value(public.clone()).unwrap())
            .unwrap()
            .1;
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
