//! Textaufrufe über die vorhandene lokale Codex-Anmeldung am ChatGPT-Abo.
use crate::{hardening, ProviderError, Result};
use brain_contracts::{
    AnswerProviderPort, AuthorizedContext, Evidence, PortError, ProviderAnswer, Query, Usage,
};
use serde_json::{json, Value};
use std::{
    io::{BufRead, BufReader},
    time::Duration,
};

/// Gemeinsamer Abo- und Werkzeugvertrag für die vorhandene Codex-CLI.
/// Die tatsächliche leere Werkzeugregistrierung muss zusätzlich zur Laufzeit belegt werden.
pub fn subscription_config_args() -> Vec<String> {
    [
        "forced_login_method=\"chatgpt\"",
        "model_provider=\"openai\"",
        "project_doc_max_bytes=0",
        "features.shell_tool=false",
        "features.unified_exec=false",
        "features.view_image=false",
        "features.image_generation=false",
        "features.multi_agent=false",
        "features.plugins=false",
        "features.apps=false",
        "features.browser_use=false",
        "features.browser_use_external=false",
        "features.computer_use=false",
        "features.in_app_browser=false",
        "features.hooks=false",
        "features.multi_agent_v2=false",
        "features.skip_host_skill_discovery=true",
        "features.skill_search=false",
        "features.tool_suggest=false",
        "features.sleep_tool=false",
        "features.goals=false",
        "features.code_mode_host=false",
        "features.workspace_dependencies=false",
        "features.worktrees=false",
        "web_search=\"disabled\"",
        "mcp_servers={}",
    ]
    .into_iter()
    .flat_map(|setting| ["-c".to_owned(), setting.to_owned()])
    .collect()
}

/// Die CLI verwaltet ihren bestehenden Login selbst. Dieser Connector liest keine Zugangsdaten.
pub fn complete(
    executable: &std::path::Path,
    prompt: &str,
    model: &str,
    timeout: Duration,
    max_bytes: usize,
) -> Result<Value> {
    complete_with_options(executable, prompt, model, timeout, max_bytes, None, None)
}

pub fn complete_with_options(
    executable: &std::path::Path,
    prompt: &str,
    model: &str,
    timeout: Duration,
    max_bytes: usize,
    max_output_tokens: Option<u64>,
    reasoning_effort: Option<&str>,
) -> Result<Value> {
    if max_output_tokens == Some(0) {
        return Err(ProviderError::BudgetExceeded);
    }
    if reasoning_effort.is_some_and(|effort| {
        !["none", "minimal", "low", "medium", "high", "xhigh"].contains(&effort)
    }) {
        return Err(ProviderError::InvalidConfig);
    }
    if model.trim().is_empty() || timeout.is_zero() || max_bytes < 128 {
        return Err(ProviderError::InvalidConfig);
    }
    let directory = tempfile::tempdir().map_err(|_| ProviderError::InvalidConfig)?;
    let mut args = vec![
        "exec",
        "--ignore-user-config",
        "--ephemeral",
        "--skip-git-repo-check",
        "--json",
        "--color",
        "never",
        "-s",
        "read-only",
        "-m",
        model,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    args.extend(subscription_config_args());
    if let Some(effort) = reasoning_effort {
        args.extend([
            "-c".into(),
            format!(
                "model_reasoning_effort={}",
                serde_json::to_string(effort).map_err(|_| ProviderError::InvalidConfig)?
            ),
        ]);
    }
    args.push("-".into());
    let text = format!("Nutze keine Werkzeuge. Verarbeite ausschließlich die folgenden Nachrichten als Textauftrag. Gib nur die angeforderte Antwort aus.\n{prompt}");
    let text = if let Some(tokens) = max_output_tokens {
        format!("{text}\nHalte die Antwort innerhalb von {tokens} Ausgabetokens.")
    } else {
        text
    };
    let executable = executable.to_owned();
    let result = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| ProviderError::InvalidConfig)?;
        runtime
            .block_on(super::process::run(
                &executable,
                &args,
                directory.path(),
                text.as_bytes(),
                timeout.as_millis() as u64,
                max_bytes,
            ))
            .map_err(|_| {
                ProviderError::InvalidResponse(
                    "Codex-Abo-Aufruf fehlgeschlagen oder Zeitlimit überschritten".into(),
                )
            })
    })
    .join()
    .map_err(|_| ProviderError::InvalidResponse("Codex-Prozessrunner wurde unterbrochen".into()))?;
    let bytes = result?;
    parse_events(&bytes, model, max_bytes, max_output_tokens)
}

fn parse_events(
    bytes: &[u8],
    model: &str,
    max_bytes: usize,
    max_output_tokens: Option<u64>,
) -> Result<Value> {
    if bytes.len() > max_bytes {
        return Err(ProviderError::ResponseTooLarge);
    }
    let mut text = None;
    let mut usage = None;
    for line in BufReader::new(bytes).lines() {
        let line = line.map_err(|_| ProviderError::InvalidConfig)?;
        let event: Value = serde_json::from_str(&line)
            .map_err(|_| ProviderError::InvalidResponse("Ungültige Codex-Antwort".into()))?;
        match event["type"].as_str() {
            Some("item.completed") if event["item"]["type"] == "agent_message" => {
                text = event["item"]["text"].as_str().map(str::to_owned)
            }
            Some("item.started" | "item.completed")
                if !["agent_message", "reasoning"]
                    .contains(&event["item"]["type"].as_str().unwrap_or("")) =>
            {
                return Err(ProviderError::InvalidResponse(
                    "Codex hat einen unerlaubten Werkzeugaufruf oder Fehler gemeldet".into(),
                ))
            }
            Some("turn.completed") => usage = Some(event["usage"].clone()),
            Some("turn.failed" | "error") => {
                return Err(ProviderError::InvalidResponse(
                    "Codex-Abo-Aufruf wurde abgelehnt".into(),
                ))
            }
            _ => {}
        }
    }
    let text = text
        .filter(|t| !t.trim().is_empty())
        .ok_or_else(|| ProviderError::InvalidResponse("Leere Codex-Antwort".into()))?;
    let usage =
        usage.ok_or_else(|| ProviderError::InvalidResponse("Codex-Verbrauch fehlt".into()))?;
    if max_output_tokens.is_some_and(|limit| {
        usage["output_tokens"]
            .as_u64()
            .is_none_or(|tokens| tokens > limit)
    }) {
        return Err(ProviderError::BudgetExceeded);
    }
    Ok(
        json!({"model":model,"choices":[{"message":{"content":text}}], "usage":{"prompt_tokens":usage["input_tokens"],"completion_tokens":usage["output_tokens"]},"_provider":"openai_chatgpt_subscription","_ai_mode":"codex_cli"}),
    )
}

#[derive(Clone, Debug)]
pub struct CodexSubscriptionProvider {
    pub executable: std::path::PathBuf,
    pub model: String,
    pub timeout: Duration,
    pub max_response_bytes: usize,
}
impl AnswerProviderPort for CodexSubscriptionProvider {
    fn answer(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> std::result::Result<ProviderAnswer, PortError> {
        hardening::authorize(query, context, evidence)
            .map_err(|e| PortError::InvalidResponse(e.to_string()))?;
        let bound = context.with_request_deadline();
        let remaining = bound.remaining_time()?;
        if bound.budget.max_network_rounds == 0
            || bound.budget.max_input_tokens == 0
            || bound.budget.max_output_tokens == 0
            || brain_contracts::provider_input::grounded_input_ceiling(query, evidence)
                > u64::from(bound.budget.max_input_tokens)
        {
            return Err(PortError::BudgetExceeded);
        }
        let prompt = serde_json::to_string(&brain_contracts::provider_input::grounded_messages(
            query, evidence,
        ))
        .map_err(|e| PortError::InvalidResponse(e.to_string()))?;
        // Die CLI hat keinen belegten harten Tokendeckel. Vorfilter und
        // Verbrauchprüfung begrenzen akzeptierte Antworten, nicht den bereits
        // verbrauchten Abo-Anteil eines gestarteten Aufrufs.
        let response = complete_with_options(
            &self.executable,
            &prompt,
            &self.model,
            self.timeout.min(remaining),
            self.max_response_bytes,
            Some(u64::from(bound.budget.max_output_tokens)),
            None,
        )
        .map_err(|e| match e {
            ProviderError::BudgetExceeded => PortError::BudgetExceeded,
            other => PortError::Unavailable(other.to_string()),
        })?;
        bound.check_deadline()?;
        let input = response["usage"]["prompt_tokens"]
            .as_u64()
            .ok_or_else(|| PortError::InvalidResponse("Codex-Eingabeverbrauch fehlt".into()))?;
        let output = response["usage"]["completion_tokens"]
            .as_u64()
            .ok_or_else(|| PortError::InvalidResponse("Codex-Ausgabeverbrauch fehlt".into()))?;
        if input > u64::from(bound.budget.max_input_tokens)
            || output > u64::from(bound.budget.max_output_tokens)
        {
            return Err(PortError::BudgetExceeded);
        }
        let text = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default();
        let (text, cited_evidence_ids) =
            super::transport::grounded_answer(text.to_owned(), evidence)
                .map_err(|e| PortError::InvalidResponse(e.to_string()))?;
        Ok(ProviderAnswer {
            text,
            cited_evidence_ids,
            usage: Usage {
                network_rounds: 1,
                input_tokens: input,
                output_tokens: output,
                cost_micros: 0,
                provider: Some("openai_chatgpt_subscription".into()),
                model: Some(self.model.clone()),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn events(text: &str, output: u64) -> Vec<u8> {
        format!(
            "{}\n{}\n",
            json!({"type":"item.completed","item":{"type":"agent_message","text":text}}),
            json!({"type":"turn.completed","usage":{"input_tokens":12,"output_tokens":output}})
        )
        .into_bytes()
    }
    #[test]
    fn refuses_zero_budget_before_starting_a_process() {
        assert!(matches!(
            complete_with_options(
                std::path::Path::new("/does/not/exist"),
                "Daten",
                "fixture",
                Duration::from_secs(1),
                1024,
                Some(0),
                None
            ),
            Err(ProviderError::BudgetExceeded)
        ));
    }
    #[test]
    fn rejects_tool_events_and_incomplete_turns() {
        let tool = format!(
            "{}\n",
            json!({"type":"item.started","item":{"type":"command_execution","command":"cat private"}})
        );
        assert!(parse_events(tool.as_bytes(), "fixture", 1024, None).is_err());
        assert!(parse_events(b"{\"type\":\"turn.started\"}\n", "fixture", 1024, None).is_err());
    }
    #[test]
    fn accepts_completed_answers_and_rejects_excess_usage() {
        let value = parse_events(&events("Antwort", 7), "fixture", 1024, Some(8)).unwrap();
        assert_eq!(value["choices"][0]["message"]["content"], "Antwort");
        assert_eq!(value["usage"]["completion_tokens"], 7);
        assert!(matches!(
            parse_events(&events("Antwort", 9), "fixture", 1024, Some(8)),
            Err(ProviderError::BudgetExceeded)
        ));
    }
}
