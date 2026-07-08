use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{config::Settings, CoreError, Result};

pub const DEFAULT_SYSTEM_PROMPT: &str = "Du bist ein Deadlock-Analyseassistent fuer einen deutschen Discord. Nutze ausschliesslich den bereitgestellten Kontext. Behalte Namen von Items, Heroes, Abilities, Stats und Quellen exakt auf Englisch. Schreibe die Analyse auf Deutsch. Markiere Unsicherheiten und fehlende Daten klar. Erfinde keine Winrates, Pickrates oder Patchdetails.";

#[derive(Clone)]
pub struct MiniMaxConfig {
    api_key: Option<String>,
    pub base_url: String,
    pub model: String,
    pub timeout_seconds: u64,
    pub max_completion_tokens: u64,
    pub temperature: f64,
    pub top_p: f64,
    pub use_token_plan: bool,
}

impl std::fmt::Debug for MiniMaxConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MiniMaxConfig")
            .field("api_key", &self.api_key.as_ref().map(|_| "<redacted>"))
            .field("base_url", &self.base_url)
            .field("model", &self.model)
            .field("timeout_seconds", &self.timeout_seconds)
            .field("max_completion_tokens", &self.max_completion_tokens)
            .field("temperature", &self.temperature)
            .field("top_p", &self.top_p)
            .field("use_token_plan", &self.use_token_plan)
            .finish()
    }
}

impl MiniMaxConfig {
    pub fn from_env() -> Result<Self> {
        let settings = crate::config::load_settings()?;
        Ok(Self::from_settings(&settings))
    }

    pub fn from_settings(settings: &Settings) -> Self {
        Self {
            api_key: settings.minimax_api_key.clone(),
            base_url: settings.minimax_base_url.clone(),
            model: settings.minimax_model.clone(),
            timeout_seconds: settings.minimax_timeout_seconds,
            max_completion_tokens: settings.minimax_max_completion_tokens,
            temperature: settings.minimax_temperature,
            top_p: settings.minimax_top_p,
            use_token_plan: settings.minimax_use_token_plan,
        }
    }

    pub fn api_key_present(&self) -> bool {
        self.api_key
            .as_ref()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
    }

    fn api_key(&self) -> Result<&str> {
        self.api_key
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .ok_or(CoreError::MissingFireworksApiKey)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(rename = "max_tokens")]
    pub max_completion_tokens: u64,
    pub temperature: f64,
    pub top_p: f64,
    pub stream: bool,
}

impl ChatCompletionRequest {
    pub fn new(messages: Vec<ChatMessage>, config: &MiniMaxConfig) -> Self {
        Self {
            model: config.model.clone(),
            messages,
            max_completion_tokens: config.max_completion_tokens,
            temperature: config.temperature,
            top_p: config.top_p,
            stream: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MiniMaxClient {
    client: Client,
    config: MiniMaxConfig,
}

impl MiniMaxClient {
    pub fn new(config: MiniMaxConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .user_agent("DeadlockBrain/0.1")
            .build()?;
        Ok(Self { client, config })
    }

    pub fn from_settings(settings: &Settings) -> Result<Self> {
        Self::new(MiniMaxConfig::from_settings(settings))
    }

    pub fn from_env() -> Result<Self> {
        Self::new(MiniMaxConfig::from_env()?)
    }

    pub fn config(&self) -> &MiniMaxConfig {
        &self.config
    }

    pub fn chat(&self, request: &ChatCompletionRequest) -> Result<Value> {
        self.chat_value(&serde_json::to_value(request)?)
    }

    pub fn chat_value(&self, request_payload: &Value) -> Result<Value> {
        self.call_openai_compatible(request_payload)
    }

    fn call_openai_compatible(&self, request_payload: &Value) -> Result<Value> {
        let url = format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        );
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.config.api_key()?)
            .json(request_payload)
            .send()?;
        parse_response(response, "fireworks_openai_compatible")
    }
}

pub fn build_review_request(
    prompt_de: &str,
    compact_context: &Value,
    config: &MiniMaxConfig,
) -> ChatCompletionRequest {
    let user_content = format!(
        "{prompt_de}\n\nErstelle die Antwort mit dieser Struktur:\n1. Kurzfazit\n2. Patch-Verlauf und relevante Reworks/Renames\n3. Aktuelle Einordnung anhand der Daten\n4. Build-/Gameplay-Implikationen\n5. Unsicherheiten / offene Punkte\n\nReview-Kontext als JSON:\n{}",
        serde_json::to_string(compact_context).unwrap_or_else(|_| "{}".to_string())
    );
    ChatCompletionRequest::new(
        vec![
            ChatMessage::system(DEFAULT_SYSTEM_PROMPT),
            ChatMessage::user(user_content),
        ],
        config,
    )
}

pub fn extract_minimax_text(response: &Value) -> String {
    if let Some(items) = response.get("content").and_then(Value::as_array) {
        let fragments = items
            .iter()
            .filter_map(|item| {
                if item.get("type").and_then(Value::as_str) == Some("text") {
                    item.get("text").and_then(Value::as_str)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if !fragments.is_empty() {
            return strip_thinking(&fragments.join("")).trim().to_string();
        }
        if items
            .iter()
            .any(|item| item.get("type").and_then(Value::as_str) == Some("thinking"))
        {
            return String::new();
        }
    }

    response
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(Value::as_str)
        .map(strip_thinking)
        .unwrap_or_default()
        .trim()
        .to_string()
}

pub fn minimax_usage_summary(response: &Value) -> Value {
    json!({
        "id": response.get("id").cloned().unwrap_or(Value::Null),
        "model": response.get("model").cloned().unwrap_or(Value::Null),
        "http_status": response.get("_http_status").cloned().unwrap_or(Value::Null),
        "created": response
            .get("created")
            .cloned()
            .unwrap_or_else(|| json!(crate::now_epoch_seconds().unwrap_or_default())),
        "usage": response.get("usage").cloned().unwrap_or_else(|| json!({})),
        "input_sensitive": response.get("input_sensitive").cloned().unwrap_or(Value::Null),
        "output_sensitive": response.get("output_sensitive").cloned().unwrap_or(Value::Null),
        "base_resp": response.get("base_resp").cloned().unwrap_or(Value::Null),
        "provider": response.get("_provider").cloned().unwrap_or_else(|| json!("fireworks")),
        "mode": response.get("_minimax_mode").cloned().unwrap_or_else(|| json!("fireworks_openai_compatible")),
    })
}

pub fn strip_thinking(text: &str) -> String {
    let mut remaining = text.to_string();
    loop {
        let lower = remaining.to_ascii_lowercase();
        let Some(start) = lower.find("<think>") else {
            break;
        };
        let after_start = start + "<think>".len();
        let Some(relative_end) = lower[after_start..].find("</think>") else {
            remaining.truncate(start);
            break;
        };
        let end = after_start + relative_end + "</think>".len();
        remaining.replace_range(start..end, "");
    }
    remaining.trim().to_string()
}

fn parse_response(response: reqwest::blocking::Response, mode: &str) -> Result<Value> {
    let status = response.status();
    let body = response.text()?;
    if !status.is_success() {
        return Err(CoreError::HttpStatus {
            status,
            url: "Fireworks".to_string(),
            body: body.chars().take(1000).collect(),
        });
    }
    let mut value: Value = serde_json::from_str(&body)?;
    if let Some(object) = value.as_object_mut() {
        object.insert("_http_status".to_string(), json!(status.as_u16()));
        object.insert("_provider".to_string(), json!("fireworks"));
        object.insert("_minimax_mode".to_string(), json!(mode));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_thinking_removes_hidden_blocks() {
        assert_eq!(strip_thinking("a <think>secret</think> b"), "a  b");
    }

    #[test]
    fn extracts_openai_compatible_text() {
        let value = json!({"choices":[{"message":{"content":"<think>x</think>Antwort"}}]});
        assert_eq!(extract_minimax_text(&value), "Antwort");
    }

    #[test]
    fn serializes_fireworks_token_field() {
        let config = MiniMaxConfig {
            api_key: Some("redacted".to_string()),
            base_url: "http://localhost".to_string(),
            model: "accounts/fireworks/models/deepseek-v4-flash".to_string(),
            timeout_seconds: 1,
            max_completion_tokens: 123,
            temperature: 0.2,
            top_p: 0.9,
            use_token_plan: false,
        };
        let value = serde_json::to_value(ChatCompletionRequest::new(
            vec![ChatMessage::user("hi")],
            &config,
        ))
        .unwrap();
        assert_eq!(value.get("max_tokens").and_then(Value::as_u64), Some(123));
        assert!(value.get("max_completion_tokens").is_none());
    }
}
