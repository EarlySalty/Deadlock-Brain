#![forbid(unsafe_code)]

use brain_contracts::{AnswerResponse, Budget, Query};
use brain_kernel::AnswerKernelPort;
use brain_policy::{PolicyEngine, PolicyError};
use sha2::{Digest, Sha256};
mod http;
pub use http::router;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiResponse {
    pub status: u16,
    pub content_type: &'static str,
    pub body: String,
}

pub struct ApiService<K> {
    policy: PolicyEngine,
    kernel: K,
    knowledge_release: String,
    deadline_ms: u64,
    budget: Budget,
}

impl<K> ApiService<K>
where
    K: AnswerKernelPort,
{
    pub fn new(
        policy: PolicyEngine,
        kernel: K,
        knowledge_release: impl Into<String>,
        deadline_ms: u64,
        budget: Budget,
    ) -> Self {
        Self {
            policy,
            kernel,
            knowledge_release: knowledge_release.into(),
            deadline_ms,
            budget,
        }
    }

    pub fn handle_answer(&self, authorization: Option<&str>, body: &[u8]) -> ApiResponse {
        if body.len() > 64 * 1024 {
            return json_error(413, "payload_too_large", "Request ist zu groß");
        }
        if self.knowledge_release.trim().is_empty()
            || self.knowledge_release == "current"
            || !(1..=60000).contains(&self.deadline_ms)
        {
            return json_error(503, "not_ready", "Kein gültiger Core-Kontext konfiguriert");
        }
        let Some(token) = bearer_token(authorization) else {
            return json_error(401, "unauthorized", "Bearer Token fehlt oder ist ungültig");
        };

        let query: Query = match serde_json::from_slice(body) {
            Ok(query) => query,
            Err(_) => return json_error(400, "invalid_request", "Query Contract ist ungültig"),
        };
        if query.validate().is_err() {
            return json_error(400, "invalid_request", "Query Contract ist ungültig");
        }

        let context = match self.policy.authorize_query(
            token,
            &query,
            self.knowledge_release.clone(),
            self.deadline_ms,
            self.budget.clone(),
        ) {
            Ok(context) => context,
            Err(PolicyError::InvalidCredentials) => {
                return json_error(401, "unauthorized", "Zugangsdaten sind ungültig")
            }
            Err(
                PolicyError::ScopeDenied(_)
                | PolicyError::ConversationOwnerMismatch
                | PolicyError::EvidenceDenied,
            ) => return json_error(403, "forbidden", "Anfrage ist nicht freigegeben"),
            Err(PolicyError::StatePoisoned) => {
                return json_error(
                    503,
                    "policy_unavailable",
                    "Policy Status ist nicht verfügbar",
                )
            }
        };

        let answer = self.kernel.answer(&query, &context);
        if answer.contract_version != brain_contracts::CONTRACT_VERSION
            || answer.request_id != query.request_id
            || answer.knowledge_release != context.knowledge_release
            || answer.citations.iter().any(|e| {
                e.validate().is_err() || !brain_policy::evidence_allowed(&context.principal, e)
            })
        {
            return json_error(502, "invalid_kernel_response", "Ungültige Core-Antwort");
        }
        answer_response(&answer)
    }
}

fn bearer_token(header: Option<&str>) -> Option<&str> {
    let header = header?.trim();
    let (scheme, token) = header.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") || token.trim().is_empty() {
        return None;
    }
    Some(token.trim())
}

fn answer_response(answer: &AnswerResponse) -> ApiResponse {
    let public = brain_contracts::PublicAnswerResponse {
        contract_version: brain_contracts::PUBLIC_API_VERSION.into(),
        request_id: answer.request_id.clone(),
        knowledge_release: answer.knowledge_release.clone(),
        status: answer.status,
        text: answer.text.clone(),
        citations: answer
            .citations
            .iter()
            .enumerate()
            .map(|(index, item)| brain_contracts::PublicCitation {
                citation_id: format!("cite-{:x}", Sha256::digest(item.evidence_id.as_bytes())),
                label: format!("Beleg {}", index + 1),
            })
            .collect(),
    };
    if public.validate(&answer.request_id).is_err() {
        return json_error(
            502,
            "invalid_kernel_response",
            "Ungültige öffentliche Antwort",
        );
    }
    match serde_json::to_string(&public) {
        Ok(body) => ApiResponse {
            status: 200,
            content_type: "application/json",
            body,
        },
        Err(_) => json_error(
            500,
            "serialization_error",
            "Antwort konnte nicht serialisiert werden",
        ),
    }
}

fn json_error(status: u16, code: &str, message: &str) -> ApiResponse {
    let body = serde_json::json!({
        "error": {
            "code": code,
            "message": message,
        }
    })
    .to_string();
    ApiResponse {
        status,
        content_type: "application/json",
        body,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeSet,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };

    use brain_contracts::{
        AnswerProfile, AnswerStatus, AuthorizedContext, Principal, Usage, CONTRACT_VERSION,
    };
    use brain_policy::{AuthGrant, CredentialRegistry};

    use super::*;

    #[derive(Clone)]
    struct FixedKernel {
        calls: Arc<AtomicUsize>,
    }

    impl AnswerKernelPort for FixedKernel {
        fn answer(&self, query: &Query, context: &AuthorizedContext) -> AnswerResponse {
            self.calls.fetch_add(1, Ordering::SeqCst);
            AnswerResponse {
                contract_version: CONTRACT_VERSION.into(),
                request_id: query.request_id.clone(),
                knowledge_release: context.knowledge_release.clone(),
                status: AnswerStatus::Answered,
                text: "Antwort".into(),
                citations: vec![brain_contracts::Evidence {
                    evidence_id: "fixture-evidence".into(),
                    source_id: "private-source-path".into(),
                    logical_id: "internal-id".into(),
                    revision: 1,
                    kind: brain_contracts::EvidenceKind::Fact,
                    content: "fixture content".into(),
                    citation: "internal:fixture".into(),
                    visibility: brain_contracts::SourceVisibility::Public,
                    allowed_scopes: BTreeSet::new(),
                    score: 1.0,
                    provenance: None,
                    patch: None,
                }],
                usage: Usage::default(),
            }
        }
    }

    fn scopes(values: &[&str]) -> BTreeSet<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    fn service(calls: Arc<AtomicUsize>) -> ApiService<FixedKernel> {
        let policy = PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
            "test-token",
            "actor-1",
            "mcp",
            scopes(&["docs.public"]),
            scopes(&["public"]),
        )]));
        ApiService::new(
            policy,
            FixedKernel { calls },
            "release-1",
            2_000,
            Budget::default(),
        )
    }

    fn query(requested_scopes: &[&str]) -> Query {
        Query {
            request_id: "request-1".into(),
            conversation_id: "conversation-1".into(),
            text: "Was macht Abrams?".into(),
            requested_scopes: scopes(requested_scopes),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        }
    }

    #[test]
    fn valid_bearer_reaches_kernel_with_server_side_context() {
        let calls = Arc::new(AtomicUsize::new(0));
        let response = service(calls.clone()).handle_answer(
            Some("Bearer test-token"),
            &serde_json::to_vec(&query(&["docs.public"])).unwrap(),
        );

        assert_eq!(response.status, 200);
        let answer: brain_contracts::PublicAnswerResponse =
            serde_json::from_str(&response.body).unwrap();
        assert!(!response.body.contains("private-source-path"));
        assert!(!response.body.contains("internal-id"));
        assert!(!response.body.contains("fixture content"));
        assert!(!response.body.contains("allowed_scopes"));
        answer.validate("request-1").unwrap();
        assert_eq!(answer.status, AnswerStatus::Answered);
        assert_eq!(answer.knowledge_release, "release-1");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn body_scope_escalation_is_rejected_before_kernel() {
        let calls = Arc::new(AtomicUsize::new(0));
        let response = service(calls.clone()).handle_answer(
            Some("Bearer test-token"),
            &serde_json::to_vec(&query(&["admin"])).unwrap(),
        );

        assert_eq!(response.status, 403);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn malformed_or_missing_bearer_is_unauthorized() {
        let calls = Arc::new(AtomicUsize::new(0));
        for header in [None, Some("Basic abc"), Some("Bearer ")] {
            let response = service(calls.clone())
                .handle_answer(header, &serde_json::to_vec(&query(&[])).unwrap());
            assert_eq!(response.status, 401);
        }
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn unknown_query_fields_are_rejected() {
        let calls = Arc::new(AtomicUsize::new(0));
        let body = br#"{
            "request_id":"r1",
            "conversation_id":"c1",
            "text":"Abrams",
            "profile":"explain",
            "requested_scopes":[],
            "actor_id":"admin"
        }"#;
        let response = service(calls.clone()).handle_answer(Some("Bearer test-token"), body);
        assert_eq!(response.status, 400);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn api_does_not_accept_principal_from_request_body() {
        let principal = Principal {
            actor_id: "body-actor".into(),
            channel: "body".into(),
            scopes: scopes(&["admin"]),
            provider_egress: scopes(&["internal"]),
        };
        let value = serde_json::to_value(principal).unwrap();
        assert!(value.get("actor_id").is_some());

        let calls = Arc::new(AtomicUsize::new(0));
        let mut query = serde_json::to_value(query(&[])).unwrap();
        query["principal"] = value;
        let response = service(calls.clone()).handle_answer(
            Some("Bearer test-token"),
            &serde_json::to_vec(&query).unwrap(),
        );

        assert_eq!(response.status, 400);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }
}
