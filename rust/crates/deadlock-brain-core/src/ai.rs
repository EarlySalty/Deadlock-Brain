use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{config::Settings, CoreError, Result};

pub const DEFAULT_SYSTEM_PROMPT: &str = "Du bist ein Deadlock-Analyseassistent für einen deutschen Discord. Nutze ausschließlich den bereitgestellten Kontext. Behalte Namen von Items, Heroes, Abilities, Stats und Quellen exakt auf Englisch. Schreibe die Analyse auf Deutsch. Markiere Unsicherheiten und fehlende Daten klar. Erfinde keine Winrates, Pickrates oder Patchdetails.";

#[derive(Clone)]
pub struct AiConfig {
    pub model: String,
    pub timeout_seconds: u64,
    pub max_completion_tokens: u64,
    pub temperature: f64,
    pub top_p: f64,
    pub use_token_plan: bool,
}

impl std::fmt::Debug for AiConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AiConfig")
            .field("model", &self.model)
            .field("timeout_seconds", &self.timeout_seconds)
            .field("max_completion_tokens", &self.max_completion_tokens)
            .field("temperature", &self.temperature)
            .field("top_p", &self.top_p)
            .field("use_token_plan", &self.use_token_plan)
            .finish()
    }
}

impl AiConfig {
    pub fn from_env() -> Result<Self> {
        let settings = crate::config::load_settings()?;
        Ok(Self::from_settings(&settings))
    }

    pub fn from_settings(settings: &Settings) -> Self {
        Self {
            model: settings.ai_model.clone(),
            timeout_seconds: settings.ai_timeout_seconds,
            max_completion_tokens: settings.ai_max_completion_tokens,
            temperature: settings.ai_temperature,
            top_p: settings.ai_top_p,
            use_token_plan: settings.ai_use_token_plan,
        }
    }

    pub fn subscription_configured(&self) -> bool {
        // Der lokale Abo-Login wird beim tatsächlichen Aufruf geprüft.
        !self.model.trim().is_empty() && self.timeout_seconds > 0
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
}

impl ChatCompletionRequest {
    pub fn new(messages: Vec<ChatMessage>, config: &AiConfig) -> Self {
        Self {
            model: config.model.clone(),
            messages,
            max_completion_tokens: config.max_completion_tokens,
            temperature: config.temperature,
            top_p: config.top_p,
            stream: false,
            response_format: None,
            reasoning_effort: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AiClient {
    config: AiConfig,
}

impl AiClient {
    pub fn new(config: AiConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub fn from_settings(settings: &Settings) -> Result<Self> {
        Self::new(AiConfig::from_settings(settings))
    }

    pub fn from_env() -> Result<Self> {
        Self::new(AiConfig::from_env()?)
    }

    pub fn config(&self) -> &AiConfig {
        &self.config
    }

    pub fn chat(&self, request: &ChatCompletionRequest) -> Result<Value> {
        self.chat_value(&serde_json::to_value(request)?)
    }

    pub fn chat_value(&self, request_payload: &Value) -> Result<Value> {
        let settings = crate::config::load_ai_settings()?;
        let prompt = subscription_prompt(request_payload)?;
        brain_providers::codex::complete_with_options(
            &settings.cli_path,
            &prompt,
            &settings.model,
            std::time::Duration::from_secs(settings.timeout_seconds),
            settings.max_response_bytes,
            request_payload
                .get("max_tokens")
                .and_then(Value::as_u64)
                .or(Some(settings.ai_max_completion_tokens)),
            request_payload
                .get("reasoning_effort")
                .and_then(Value::as_str)
                .or(settings.ai_reasoning_effort.as_deref()),
        )
        .map_err(|error| CoreError::ModelSelection(error.to_string()))
    }
}

fn subscription_prompt(request: &Value) -> Result<String> {
    let object = request.as_object().ok_or(CoreError::InvalidAiRequest)?;
    let messages = object
        .get("messages")
        .and_then(Value::as_array)
        .filter(|messages| !messages.is_empty())
        .ok_or(CoreError::InvalidAiRequest)?;
    let mut prompt = String::new();
    for message in messages {
        let role = message
            .get("role")
            .and_then(Value::as_str)
            .ok_or(CoreError::InvalidAiRequest)?;
        let content = message
            .get("content")
            .and_then(Value::as_str)
            .ok_or(CoreError::InvalidAiRequest)?;
        prompt.push_str(&format!("[{role}]\n{content}\n\n"));
    }
    if object
        .get("response_format")
        .and_then(|format| format.get("type"))
        .and_then(Value::as_str)
        == Some("json_object")
    {
        prompt.push_str("Antworte ausschließlich mit einem gültigen JSON-Objekt ohne Markdown.\n");
    }
    Ok(prompt)
}

pub fn build_review_request(
    prompt_de: &str,
    compact_context: &Value,
    config: &AiConfig,
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

pub fn extract_ai_text(response: &Value) -> String {
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

pub fn ai_usage_summary(response: &Value) -> Value {
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
        "provider": response.get("_provider").cloned().unwrap_or_else(|| json!("openai_chatgpt_subscription")),
        "mode": response.get("_ai_mode").cloned().unwrap_or_else(|| json!("codex_local")),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_payload_cannot_bypass_subscription() {
        assert!(matches!(
            subscription_prompt(&json!([])),
            Err(CoreError::InvalidAiRequest)
        ));
    }

    #[test]
    fn prompt_preserves_messages_and_json_contract() {
        let prompt = subscription_prompt(&json!({"model": "stale-model", "messages": [{"role": "system", "content": "Regeln"}, {"role": "user", "content": "Daten"}], "response_format": {"type": "json_object"}})).unwrap();
        assert!(prompt.contains("[system]\nRegeln"));
        assert!(prompt.contains("[user]\nDaten"));
        assert!(prompt.contains("gültigen JSON-Objekt"));
        assert!(!prompt.contains("stale-model"));
    }

    #[test]
    fn strip_thinking_removes_hidden_blocks() {
        assert_eq!(strip_thinking("a <think>secret</think> b"), "a  b");
    }

    #[test]
    fn extracts_openai_compatible_text() {
        let value = json!({"choices":[{"message":{"content":"<think>x</think>Antwort"}}]});
        assert_eq!(extract_ai_text(&value), "Antwort");
    }

    #[test]
    fn serializes_compatible_token_field() {
        let config = AiConfig {
            model: "configured-model".to_string(),
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
        assert!(value.get("response_format").is_none());
        assert!(value.get("reasoning_effort").is_none());
    }
}
