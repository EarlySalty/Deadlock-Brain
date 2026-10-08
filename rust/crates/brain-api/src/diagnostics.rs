use crate::ApiResponse;
use serde_json::{json, Value};
use std::{io::Write, sync::OnceLock};

pub(crate) struct RequestTrace {
    route: &'static str,
    request_id: OnceLock<String>,
}

impl RequestTrace {
    pub(crate) fn new(route: &'static str) -> Self {
        Self {
            route,
            request_id: OnceLock::new(),
        }
    }

    pub(crate) fn bind(&self, body: &[u8]) {
        if body.len() > 64 * 1024 {
            return;
        }
        let Ok(value) = serde_json::from_slice::<Value>(body) else {
            return;
        };
        let Some(id) = value.get("request_id").and_then(Value::as_str) else {
            return;
        };
        if !id.trim().is_empty() && id.len() <= 512 && !id.chars().any(char::is_control) {
            let _ = self
                .request_id
                .set(brain_contracts::response_audit::diagnostic_text(id, None));
        }
    }

    pub(crate) fn event(&self, response: &ApiResponse) -> Option<Value> {
        let body = serde_json::from_str::<Value>(&response.body).ok();
        let class = if response.status >= 400 {
            match body
                .as_ref()
                .and_then(|body| body.pointer("/error/code"))
                .and_then(Value::as_str)
                .unwrap_or_default()
            {
                "unauthorized" => "unauthorized",
                "forbidden" => "forbidden",
                "invalid_request" => "invalid_request",
                "invalid_kernel_response" => "invalid_kernel_response",
                "invalid_retrieval_response" => "invalid_retrieval_response",
                "invalid_internal_evidence" => "invalid_internal_evidence",
                "invalid_internal_response" => "invalid_internal_response",
                "serialization_error" => "serialization_error",
                "worker_unavailable" => "worker_unavailable",
                "context_unavailable" => "context_unavailable",
                "unavailable" => "unavailable",
                "not_ready" => "not_ready",
                "deadline_exceeded" => "deadline_exceeded",
                "overloaded" => "overloaded",
                "payload_too_large" => "payload_too_large",
                "unsupported_media_type" => "unsupported_media_type",
                "unsupported_domain" => "unsupported_domain",
                "not_found" => "not_found",
                _ => "http_error",
            }
        } else {
            match body
                .as_ref()
                .and_then(|body| body.get("status"))
                .and_then(Value::as_str)
                .unwrap_or_default()
            {
                "unavailable" => "unavailable",
                "provider_error" => "provider_error",
                "budget_exceeded" => "budget_exceeded",
                "unauthorized_evidence" => "unauthorized_evidence",
                "answered" | "insufficient_evidence" | "build_rejected" => return None,
                _ => "invalid_api_response",
            }
        };
        Some(json!({
            "event": "request_failure",
            "route": self.route,
            "request_id": self.request_id.get(),
            "error_class": class,
            "http_status": response.status,
        }))
    }

    fn write(&self, response: &ApiResponse, writer: &mut impl Write) -> std::io::Result<()> {
        if let Some(event) = self.event(response) {
            serde_json::to_writer(&mut *writer, &event)?;
            writeln!(writer)?;
        }
        Ok(())
    }

    pub(crate) fn emit(&self, response: &ApiResponse) {
        let _ = self.write(response, &mut std::io::stderr().lock());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failures_log_only_request_binding_and_static_classes() {
        let trace = RequestTrace::new("/v1/answer");
        trace.bind(br#"{"request_id":"fixture-request","text":"private-question","user_id":"123456789012345678","answer_context":{"channel_name":"private-channel"},"token":"private-secret"}"#);
        for (status, body, class) in [
            (
                200,
                json!({"status":"unavailable","request_id":"foreign-id","text":"private-answer","citations":["private-citation"]}),
                "unavailable",
            ),
            (
                200,
                json!({"status":"provider_error","text":"private-answer"}),
                "provider_error",
            ),
            (
                502,
                json!({"error":{"code":"invalid_kernel_response","message":"private-error"}}),
                "invalid_kernel_response",
            ),
            (
                400,
                json!({"error":{"code":"invalid_request","message":"private-error"}}),
                "invalid_request",
            ),
            (
                503,
                json!({"error":{"code":"worker_unavailable","message":"private-error"}}),
                "worker_unavailable",
            ),
            (
                500,
                json!({"error":{"code":"private-secret","message":"private-error"}}),
                "http_error",
            ),
        ] {
            let response = ApiResponse {
                status,
                content_type: "application/json",
                body: body.to_string(),
            };
            let mut log = Vec::new();
            trace.write(&response, &mut log).unwrap();
            let event: Value = serde_json::from_slice(&log).unwrap();
            assert_eq!(
                event,
                json!({"event":"request_failure","route":"/v1/answer","request_id":"fixture-request","error_class":class,"http_status":status})
            );
            let log = String::from_utf8(log).unwrap();
            for private in [
                "private-",
                "123456789012345678",
                "foreign-id",
                "user_id",
                "citations",
                "token",
            ] {
                assert!(!log.contains(private), "{private}");
            }
            assert_eq!(log.lines().count(), 1);
        }
    }

    #[test]
    fn missing_or_invalid_request_ids_stay_absent_and_success_is_quiet() {
        for body in [
            br#"{"request_id":"bad\nrequest"}"#.as_slice(),
            br#"{"request_id":3}"#,
            b"{invalid-json",
            br#"{}"#,
        ] {
            let trace = RequestTrace::new("/v1/operator/query");
            trace.bind(body);
            let failed = crate::json_error(400, "invalid_request", "private-error");
            assert_eq!(trace.event(&failed).unwrap()["request_id"], Value::Null);
            let succeeded = ApiResponse {
                status: 200,
                content_type: "application/json",
                body: json!({"status":"answered","text":"private-answer"}).to_string(),
            };
            let mut log = Vec::new();
            trace.write(&succeeded, &mut log).unwrap();
            assert!(log.is_empty());
        }
        let trace = RequestTrace::new("/v1/answer");
        let large = json!({"request_id":"fixture", "text":"x".repeat(64*1024)}).to_string();
        trace.bind(large.as_bytes());
        assert!(trace.request_id.get().is_none());
        trace.bind(br#"{"request_id":"first"}"#);
        trace.bind(br#"{"request_id":"second"}"#);
        assert_eq!(trace.request_id.get().unwrap(), "first");
    }
}
