use deadlock_brain_core::ai::{
    extract_ai_text, AiClient, ChatCompletionRequest, ChatMessage, DEFAULT_SYSTEM_PROMPT,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    BuildObject, DamagePlan, HeroModel, MetaIndex, PatchDelta, ReasonerError, Result, ScoredItem,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbilityRoleNote {
    pub ability_id: i64,
    pub role: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeroAnalystResponse {
    pub ability_roles: Vec<AbilityRoleNote>,
    pub playstyle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemAnalystNote {
    pub item_id: i64,
    pub why: String,
    pub condition_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemAnalystResponse {
    pub items: Vec<ItemAnalystNote>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchAnalystNote {
    pub target_kind: String,
    pub target_id: i64,
    pub label: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchAnalystResponse {
    pub notes: Vec<PatchAnalystNote>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaAnalystResponse {
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CriticResponse {
    pub verdict: String,
    pub issues: Vec<String>,
}

fn request(
    system: &str,
    context: Value,
    config: &deadlock_brain_core::ai::AiConfig,
) -> ChatCompletionRequest {
    let user = format!(
        "{system}\n\nAntworte ausschließlich mit gültigem JSON ohne Markdown-Codezaun. Zahlen und Auswahlwerte stammen aus dem Kontext und dürfen nicht verändert werden.\n\nKontext:\n{}",
        serde_json::to_string(&context).unwrap_or_else(|_| "{}".to_string())
    );
    let mut request = ChatCompletionRequest::new(
        vec![
            ChatMessage::system(DEFAULT_SYSTEM_PROMPT),
            ChatMessage::user(user),
        ],
        config,
    );
    request.response_format = Some(json!({"type": "json_object"}));
    request.reasoning_effort = Some("none".to_string());
    request
}

fn request_value(request: &ChatCompletionRequest) -> Result<Value> {
    serde_json::to_value(request).map_err(|error| ReasonerError::Ai(error.to_string()))
}

fn parse_json<T: for<'de> Deserialize<'de>>(text: &str) -> Result<T> {
    let text = text.trim();
    let candidate = text
        .strip_prefix("```json")
        .and_then(|value| value.strip_suffix("```"))
        .or_else(|| {
            text.strip_prefix("```")
                .and_then(|value| value.strip_suffix("```"))
        })
        .unwrap_or(text)
        .trim();
    let start = candidate
        .find('{')
        .ok_or_else(|| ReasonerError::Ai("KI-Antwort enthält kein JSON-Objekt".to_string()))?;
    let end = candidate
        .rfind('}')
        .filter(|end| *end >= start)
        .ok_or_else(|| {
            ReasonerError::Ai("KI-Antwort enthält kein vollständiges JSON-Objekt".to_string())
        })?;
    serde_json::from_str(&candidate[start..=end])
        .map_err(|error| ReasonerError::Ai(format!("ungültiges Rollen-JSON: {error}")))
}

fn call<T>(
    client: &AiClient,
    request: ChatCompletionRequest,
    parse: fn(&str) -> Result<T>,
) -> Result<T> {
    let response = client
        .chat_value(&request_value(&request)?)
        .map_err(|error| ReasonerError::Ai(error.to_string()))?;
    parse(&extract_ai_text(&response))
}

pub fn build_hero_analyst_request(
    hero: &HeroModel,
    damage_plan: &DamagePlan,
    config: &deadlock_brain_core::ai::AiConfig,
) -> ChatCompletionRequest {
    request(
        "Rolle Hero-Analyst. Liefere ability_roles und playstyle nach dem festgelegten Schema. Beschreibe nur Rollen und Spielstil, erfinde keine Werte.",
        json!({"hero": hero, "damage_plan": damage_plan}),
        config,
    )
}

pub fn build_item_analyst_request(
    items: &[ScoredItem],
    config: &deadlock_brain_core::ai::AiConfig,
) -> ChatCompletionRequest {
    request(
        "Rolle Item-Analyst. Liefere für die gegebenen Items je einen Warum-Text und einen Bedingungshinweis. Wähle keine neuen Items und ändere keine Zahlen.",
        json!({"items": items}),
        config,
    )
}

pub fn build_patch_analyst_request(
    deltas: &[PatchDelta],
    raw_events: &[Value],
    config: &deadlock_brain_core::ai::AiConfig,
) -> ChatCompletionRequest {
    request(
        "Rolle Patch-Analyst. Erkläre die gegebenen Patch-Verschiebungen im Klartext. Vorzeichen und Größenordnung kommen ausschließlich aus den Deltas.",
        json!({"deltas": deltas, "raw_events": raw_events}),
        config,
    )
}

pub fn build_meta_analyst_request(
    meta: &MetaIndex,
    config: &deadlock_brain_core::ai::AiConfig,
) -> ChatCompletionRequest {
    request(
        "Rolle Meta-Analyst. Fasse die Meta-Signale und ihre Stichprobenqualität einordnend zusammen. Erzeuge keine Rangfolge und keine Zahlen.",
        json!({"meta": meta}),
        config,
    )
}

pub fn build_critic_request(
    build: &BuildObject,
    config: &deadlock_brain_core::ai::AiConfig,
) -> ChatCompletionRequest {
    request(
        "Rolle Kritiker. Prüfe das fertige Build gegen die gelieferten Fakten. Gib verdict pass oder recompose und konkrete issues aus. Wähle keine Items und ändere keine Zahlen.",
        json!({"build": build}),
        config,
    )
}

pub fn parse_hero_analyst_response(text: &str) -> Result<HeroAnalystResponse> {
    parse_json(text)
}

pub fn parse_item_analyst_response(text: &str) -> Result<ItemAnalystResponse> {
    parse_json(text)
}

pub fn parse_patch_analyst_response(text: &str) -> Result<PatchAnalystResponse> {
    parse_json(text)
}

pub fn parse_meta_analyst_response(text: &str) -> Result<MetaAnalystResponse> {
    parse_json(text)
}

pub fn parse_critic_response(text: &str) -> Result<CriticResponse> {
    let response: CriticResponse = parse_json(text)?;
    if !matches!(response.verdict.as_str(), "pass" | "recompose") {
        return Err(ReasonerError::Ai(
            "Kritiker verdict ist weder pass noch recompose".to_string(),
        ));
    }
    Ok(response)
}

pub fn run_hero_analyst(
    client: &AiClient,
    hero: &HeroModel,
    damage_plan: &DamagePlan,
) -> Result<HeroAnalystResponse> {
    let request = build_hero_analyst_request(hero, damage_plan, client.config());
    call(client, request, parse_hero_analyst_response)
}

pub fn run_item_analyst(client: &AiClient, items: &[ScoredItem]) -> Result<ItemAnalystResponse> {
    call(
        client,
        build_item_analyst_request(items, client.config()),
        parse_item_analyst_response,
    )
}

pub fn run_patch_analyst(
    client: &AiClient,
    deltas: &[PatchDelta],
    raw_events: &[Value],
) -> Result<PatchAnalystResponse> {
    call(
        client,
        build_patch_analyst_request(deltas, raw_events, client.config()),
        parse_patch_analyst_response,
    )
}

pub fn run_meta_analyst(client: &AiClient, meta: &MetaIndex) -> Result<MetaAnalystResponse> {
    call(
        client,
        build_meta_analyst_request(meta, client.config()),
        parse_meta_analyst_response,
    )
}

pub fn run_critic(client: &AiClient, build: &BuildObject) -> Result<CriticResponse> {
    call(
        client,
        build_critic_request(build, client.config()),
        parse_critic_response,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::TcpListener;
    use std::time::Duration;

    #[test]
    fn run_critic_validates_verdict_over_http() {
        let build = BuildObject {
            hero_id: 25,
            hero_name: "Warden".to_string(),
            patch_tag: "test".to_string(),
            name: "Test build".to_string(),
            core: Vec::new(),
            situations: Vec::new(),
            ability_order: Vec::new(),
            confidence: crate::Confidence::Low,
            rationale: String::new(),
        };
        for verdict in ["pass", "recompose", "maybe"] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let mut settings = test_settings();
            settings.ai_base_url = format!("http://{}", listener.local_addr().unwrap());
            settings.ai_api_key = Some("test-key".to_string());
            let client = AiClient::from_settings(&settings).unwrap();
            let server = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                stream
                    .set_write_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let mut reader = BufReader::new(&mut stream);
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                assert_eq!(line, "POST /chat/completions HTTP/1.1\r\n");
                let mut length = None;
                loop {
                    line.clear();
                    assert!(reader.read_line(&mut line).unwrap() > 0);
                    if line == "\r\n" {
                        break;
                    }
                    if let Some((name, value)) = line.split_once(':') {
                        if name.eq_ignore_ascii_case("content-length") {
                            length = Some(value.trim().parse::<usize>().unwrap());
                        }
                    }
                }
                let mut body = vec![0; length.unwrap()];
                reader.read_exact(&mut body).unwrap();
                let request: Value = serde_json::from_slice(&body).unwrap();
                assert!(request["messages"][1]["content"]
                    .as_str()
                    .unwrap()
                    .contains("Rolle Kritiker"));
                let response = json!({"choices": [{"message": {"content":
                    json!({"verdict": verdict, "issues": ["fixture issue"]}).to_string()
                }}]})
                .to_string();
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response.len(), response).unwrap();
            });
            let result = run_critic(&client, &build);
            server.join().unwrap();
            if verdict == "maybe" {
                assert!(
                    matches!(result, Err(ReasonerError::Ai(message)) if message.contains("verdict"))
                );
            } else {
                assert_eq!(
                    result.unwrap(),
                    CriticResponse {
                        verdict: verdict.to_string(),
                        issues: vec!["fixture issue".to_string()],
                    }
                );
            }
        }
    }

    #[test]
    fn parses_fenced_hero_fixture() {
        let response = parse_hero_analyst_response(
            "```json\n{\"ability_roles\":[{\"ability_id\":25,\"role\":\"Damage\",\"note\":\"Kern\"}],\"playstyle\":\"Waffen-Kern\"}\n```",
        )
        .unwrap();
        assert_eq!(response.ability_roles[0].ability_id, 25);
        assert_eq!(response.playstyle, "Waffen-Kern");
    }

    #[test]
    fn parses_all_role_fixtures() {
        let item = parse_item_analyst_response(
            r#"{"items":[{"item_id":1,"why":"Waffenwert","condition_note":"Schussgebunden"}]}"#,
        )
        .unwrap();
        assert_eq!(item.items[0].item_id, 1);
        let patch = parse_patch_analyst_response(
            r#"{"notes":[{"target_kind":"Hero","target_id":25,"label":"Buff","rationale":"Waffe skaliert besser"}]}"#,
        )
        .unwrap();
        assert_eq!(patch.notes[0].target_id, 25);
        let meta = parse_meta_analyst_response(r#"{"summary":"Nebensignal"}"#).unwrap();
        assert_eq!(meta.summary, "Nebensignal");
    }

    #[test]
    fn rejects_unknown_critic_verdict() {
        let error = parse_critic_response(r#"{"verdict":"maybe","issues":[]}"#).unwrap_err();
        assert!(error.to_string().contains("verdict"));
    }

    fn test_settings() -> deadlock_brain_core::config::Settings {
        deadlock_brain_core::config::Settings {
            project_root: "/tmp/reasoner-test".into(),
            data_dir: "/tmp/reasoner-test/data".into(),
            raw_dir: "/tmp/reasoner-test/raw".into(),
            cache_dir: "/tmp/reasoner-test/cache".into(),
            user_agent: "test".to_string(),
            sheet_id: "sheet".to_string(),
            sheet_gid: "0".to_string(),
            wiki_enabled: false,
            wiki_min_delay_seconds: 0.0,
            wiki_cache_ttl_seconds: 0,
            ai_api_key: None,
            ai_base_url: "http://localhost".to_string(),
            ai_model: "configured-model".to_string(),
            ai_timeout_seconds: 1,
            ai_max_completion_tokens: 100,
            ai_temperature: 0.2,
            ai_top_p: 0.9,
            ai_use_token_plan: false,
        }
    }

    #[test]
    fn request_disables_reasoning_without_hard_coding_a_model() {
        let config = deadlock_brain_core::ai::AiConfig::from_settings(&test_settings());
        let request = build_meta_analyst_request(
            &MetaIndex {
                by_item: Default::default(),
                sample_ok: Default::default(),
            },
            &config,
        );
        let value = request_value(&request).unwrap();
        assert_eq!(value["reasoning_effort"], "none");
        assert_eq!(value["response_format"]["type"], "json_object");
        assert_eq!(value["model"], "configured-model");
    }
}
