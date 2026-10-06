//! Synthetic checks only. Never sends community content, a database row or credentials
//! in a prompt. The source of this file is part of the policy fingerprint.
use serde_json::{json, Value};

use super::{Result, SelectionError};

fn request(model: &str, max_tokens: u64, prompt: &str) -> Value {
    json!({"model":model,"messages":[{"role":"user","content":prompt}],
        "max_tokens":max_tokens,"temperature":0.0,"top_p":1.0,"reasoning_effort":"none","stream":false})
}

fn message<'a>(model: &str, response: &'a Value, finish_reason: &str) -> Result<&'a Value> {
    let reported = response.get("model").and_then(Value::as_str).ok_or(SelectionError::ProbeFailed)?;
    if reported != model && Some(reported) != model.strip_prefix("accounts/fireworks/models/") {
        return Err(SelectionError::ProbeFailed);
    }
    let choices = response.get("choices").and_then(Value::as_array).ok_or(SelectionError::ProbeFailed)?;
    if choices.len() != 1 || choices[0].get("finish_reason").and_then(Value::as_str) != Some(finish_reason) {
        return Err(SelectionError::ProbeFailed);
    }
    let message = choices[0].get("message").ok_or(SelectionError::ProbeFailed)?;
    if message.get("role").and_then(Value::as_str) != Some("assistant") { return Err(SelectionError::ProbeFailed); }
    Ok(message)
}

fn content(message: &Value) -> Result<&str> {
    message.get("content").and_then(Value::as_str).filter(|s| !s.trim().is_empty())
        .ok_or(SelectionError::ProbeFailed)
}

pub(super) fn run(model: &str, token_limit: u64, mut chat: impl FnMut(&Value) -> Result<Value>) -> Result<()> {
    let text = chat(&request(model, token_limit.min(512), "Reply with exactly BRAIN_TEXT_OK and nothing else."))?;
    if content(message(model, &text, "stop")?)?.trim() != "BRAIN_TEXT_OK" { return Err(SelectionError::ProbeFailed); }

    // Known patch/build fixture mirrors the existing Mystic Shot cooldown parser case.
    // Unknown winrate must remain unknown, the only allowed item and source must survive.
    let mut domain = request(model, token_limit.min(1024),
        "Synthetischer Test, keine echte Patchbehauptung: Held Lash, einzig erlaubtes Build-Item Mystic Shot. Der vorgelegte Patchtext lautet: Mystic Shot cooldown reduced from 12s to 10s. Quelle: brain-config-fixture. Es gibt keine Winrate-Daten. Gib exakt ein JSON-Objekt mit hero, items (Array), cooldown_seconds (Zahl), winrate (null), source aus. Keine weiteren Felder und keine erfundenen Items oder Quellen.");
    domain["response_format"] = json!({"type":"json_object"});
    let response = chat(&domain)?;
    let parsed: Value = serde_json::from_str(content(message(model, &response, "stop")?)?)
        .map_err(|_| SelectionError::ProbeFailed)?;
    if parsed != json!({"hero":"Lash","items":["Mystic Shot"],"cooldown_seconds":10,"winrate":null,"source":"brain-config-fixture"}) {
        return Err(SelectionError::ProbeFailed);
    }

    let mut tools = request(model, token_limit.min(1024), "Call brain_add with x=2 and y=3. Do not calculate the answer yourself.");
    tools["tools"] = json!([{"type":"function","function":{"name":"brain_add","description":"Add two integers for a synthetic availability check", "parameters":{"type":"object","properties":{"x":{"type":"integer"},"y":{"type":"integer"}},"required":["x","y"],"additionalProperties":false}}}]);
    tools["tool_choice"] = json!({"type":"function","function":{"name":"brain_add"}});
    let response = chat(&tools)?;
    let tool_message = message(model, &response, "tool_calls")?;
    let calls = tool_message.get("tool_calls").and_then(Value::as_array).ok_or(SelectionError::ProbeFailed)?;
    if calls.len() != 1 || calls[0].get("type").and_then(Value::as_str) != Some("function")
        || calls[0].pointer("/function/name").and_then(Value::as_str) != Some("brain_add")
    { return Err(SelectionError::ProbeFailed); }
    let id = calls[0].get("id").and_then(Value::as_str)
        .filter(|s| !s.is_empty() && s.len() <= 256 && !s.chars().any(char::is_control))
        .ok_or(SelectionError::ProbeFailed)?;
    let arguments: Value = serde_json::from_str(calls[0].pointer("/function/arguments").and_then(Value::as_str).ok_or(SelectionError::ProbeFailed)?)
        .map_err(|_| SelectionError::ProbeFailed)?;
    if arguments != json!({"x":2,"y":3}) { return Err(SelectionError::ProbeFailed); }
    tools["messages"].as_array_mut().ok_or(SelectionError::ProbeFailed)?.extend([
        tool_message.clone(), json!({"role":"tool","tool_call_id":id,"content":"5"}),
        json!({"role":"user","content":"Reply with the tool result, exactly the number 5 and nothing else."}),
    ]);
    tools["tool_choice"] = json!("none");
    let response = chat(&tools)?;
    if content(message(model, &response, "stop")?)?.trim() != "5" { return Err(SelectionError::ProbeFailed); }

    let mut reasoning = request(model, token_limit.min(4096), "Compute 17 * 19 - 318. The final answer must be exactly the integer 5, without formatting.");
    reasoning["reasoning_effort"] = json!("low");
    let response = chat(&reasoning)?;
    let message = message(model, &response, "stop")?;
    let has_reasoning = message.get("reasoning_content").and_then(Value::as_str).is_some_and(|s| !s.trim().is_empty())
        || message.get("content").and_then(Value::as_str).is_some_and(|s| {
            s.split_once("<think>").and_then(|(_,s)| s.split_once("</think>"))
                .is_some_and(|(thinking,_)| !thinking.trim().is_empty())
        });
    if !has_reasoning || crate::ai::extract_ai_text(&response).trim() != "5" { return Err(SelectionError::ProbeFailed); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const MODEL: &str = "accounts/fireworks/models/deepseek-v4-flash";
    fn reply(content: Value, finish: &str) -> Value {
        json!({"model":MODEL,"choices":[{"message":{"role":"assistant","content":content},"finish_reason":finish}]})
    }
    fn replies() -> Vec<Value> {
        let mut tool = reply(Value::Null, "tool_calls");
        tool["choices"][0]["message"]["tool_calls"] = json!([{"id":"probe-call","type":"function","function":{"name":"brain_add","arguments":"{\"x\":2,\"y\":3}"}}]);
        let mut reason = reply(json!("5"), "stop");
        reason["choices"][0]["message"]["reasoning_content"] = json!("Synthetic calculation check.");
        vec![reply(json!("BRAIN_TEXT_OK"),"stop"),
            reply(json!("{\"hero\":\"Lash\",\"items\":[\"Mystic Shot\"],\"cooldown_seconds\":10,\"winrate\":null,\"source\":\"brain-config-fixture\"}"),"stop"),
            tool, reply(json!("5"),"stop"),reason]
    }
    fn test_responses(responses: Vec<Value>) -> Result<()> {
        let mut replies = responses.into_iter();
        run(MODEL, 16000, |payload| {
            assert_eq!(payload["model"], MODEL);
            replies.next().ok_or(SelectionError::ProbeFailed)
        })
    }
    #[test]
    fn complete_synthetic_protocol_passes() { assert!(test_responses(replies()).is_ok()); }
    #[test]
    fn http_200_empty_wrong_model_or_truncated_answer_fails() {
        for position in 0..5 {
            let mut r=replies();r[position]=json!({}); assert!(test_responses(r).is_err());
            let mut r=replies();r[position]["model"]=json!("different-model"); assert!(test_responses(r).is_err());
            let mut r=replies();r[position]["choices"][0]["finish_reason"]=json!("length"); assert!(test_responses(r).is_err());
        }
    }
    #[test]
    fn fabricated_patch_or_missing_reasoning_is_rejected() {
        let mut r=replies();r[1]["choices"][0]["message"]["content"] = json!("{\"winrate\":0.8}");
        assert!(test_responses(r).is_err());
        let mut r=replies();r[4]["choices"][0]["message"].as_object_mut().unwrap().remove("reasoning_content");
        assert!(test_responses(r).is_err());
        let mut r=replies();r[2]["choices"][0]["message"]["tool_calls"][0]["function"]["arguments"] = json!("{\"x\":20,\"y\":3}");
        assert!(test_responses(r).is_err());
    }
}
