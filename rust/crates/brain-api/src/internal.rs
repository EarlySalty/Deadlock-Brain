//! Dieser Router wird ausschließlich an den privaten Unixsocket gebunden.
use crate::{bearer_token, deadline_response, json_error, ApiResponse};
use axum::{
    body::to_bytes,
    extract::{Request, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use brain_contracts::{
    internal_api::*, AnswerProfile, Budget, Query, RequestDeadline, RetrievalPort, SourceVisibility,
};
use brain_policy::{PolicyEngine, PolicyError};
use std::{collections::BTreeSet, sync::Arc, time::Duration};
use tokio::sync::Semaphore;

struct CancelOnDrop(RequestDeadline);
impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

pub struct InternalApiService {
    policy: PolicyEngine,
    retrieval: Arc<dyn RetrievalPort>,
    release: String,
    public_docs_release: Option<String>,
    public_docs_scope: &'static str,
    deadline_ms: u64,
    budget: Budget,
    slots: Arc<Semaphore>,
}

impl InternalApiService {
    pub fn new(
        policy: PolicyEngine,
        retrieval: impl RetrievalPort + 'static,
        release: String,
        deadline_ms: u64,
        budget: Budget,
    ) -> Self {
        Self {
            policy,
            retrieval: Arc::new(retrieval),
            release,
            public_docs_release: None,
            public_docs_scope: "docs.public",
            deadline_ms,
            budget,
            slots: Arc::new(Semaphore::new(4)),
        }
    }

    pub fn with_public_docs_release(mut self, release: String) -> Self {
        self.public_docs_release = Some(release);
        self.public_docs_scope = "docs.public";
        self
    }

    pub fn with_public_bot_release(mut self, release: String) -> Self {
        self.public_docs_release = Some(release);
        self.public_docs_scope = "bot.public";
        self
    }

    fn allowed(&self, token: &str) -> bool {
        self.policy.authenticate(token).is_ok_and(|principal| {
            principal.actor_id == "second-brain"
                && principal.channel == "internal"
                && principal.scopes == BTreeSet::from(["second_brain.internal".into()])
                && principal.provider_egress.is_empty()
        })
    }

    pub fn handle_query(
        &self,
        authorization: Option<&str>,
        body: &[u8],
        deadline: RequestDeadline,
    ) -> ApiResponse {
        let Some(token) = bearer_token(authorization) else {
            return json_error(
                401,
                "unauthorized",
                "Interne Operatoridentität erforderlich",
            );
        };
        if !self.allowed(token) {
            return json_error(403, "forbidden", "Keine interne Operatorfreigabe");
        }
        if deadline.check().is_err() {
            return deadline_response();
        }
        if self.release.is_empty() || ["current", "latest"].contains(&self.release.as_str()) {
            return json_error(503, "not_ready", "Kein interner Wissensstand freigegeben");
        }
        if body.len() > 64 * 1024 {
            return json_error(413, "payload_too_large", "Anfrage zu groß");
        }
        let query: Query = match serde_json::from_slice(body) {
            Ok(query) => query,
            Err(_) => return json_error(400, "invalid_request", "Ungültiger Operatorvertrag"),
        };
        if query.validate().is_err()
            || query.domain.is_some()
            || !matches!(query.profile, AnswerProfile::Fact | AnswerProfile::Explain)
            || query.requested_scopes != BTreeSet::from(["second_brain.internal".into()])
        {
            return json_error(
                400,
                "invalid_request",
                "Interner Scope und Textprofil erforderlich",
            );
        }
        let context = match self.policy.authorize_query_until(
            token,
            &query,
            self.release.clone(),
            deadline.clone(),
            self.budget.clone(),
        ) {
            Ok(context) => context,
            Err(PolicyError::BudgetExceeded) => return deadline_response(),
            Err(_) => return json_error(403, "forbidden", "Anfrage nicht intern freigegeben"),
        };
        let public_docs =
            self.public_docs_release.as_deref() == Some(context.knowledge_release.as_str());
        let mut retrieval_query = query.clone();
        let mut retrieval_context = context.clone();
        if public_docs {
            let scopes = BTreeSet::from([self.public_docs_scope.into()]);
            retrieval_query.requested_scopes = scopes.clone();
            retrieval_context.principal.scopes = scopes;
        }
        let evidence = match self
            .retrieval
            .retrieve(&retrieval_query, &retrieval_context)
            .and_then(|evidence| {
                if !public_docs || !evidence.is_empty() {
                    self.retrieval.validate_evidence(
                        &retrieval_query,
                        &retrieval_context,
                        &evidence,
                        false,
                    )?;
                }
                if public_docs && !evidence.is_empty() {
                    self.retrieval.validate_publication(
                        &retrieval_query,
                        &retrieval_context,
                        &evidence,
                    )?;
                }
                Ok(evidence)
            }) {
            Ok(evidence) => evidence,
            Err(_) => return json_error(503, "unavailable", "Interne Belege nicht verfügbar"),
        };
        if deadline.check().is_err() {
            return deadline_response();
        }
        if evidence.len() > 100
            || evidence.iter().any(|item| {
                item.validate().is_err()
                    || if public_docs {
                        item.visibility != SourceVisibility::Public
                    } else {
                        item.visibility != SourceVisibility::Internal
                            || item.allowed_scopes
                                != BTreeSet::from(["second_brain.internal".into()])
                    }
                    || !brain_policy::evidence_allowed(&retrieval_context.principal, item)
            })
        {
            return json_error(502, "invalid_internal_evidence", "Ungültige interne Belege");
        }
        // Originalausschnitte bleiben lokal. Dieser Weg ruft keinen Modellanbieter auf.
        let response = InternalAnswerResponse {
            contract_version: INTERNAL_API_VERSION.into(),
            audience: InternalAudience::LocalOperator,
            request_id: query.request_id.clone(),
            knowledge_release: context.knowledge_release,
            status: if evidence.is_empty() {
                InternalStatus::InsufficientEvidence
            } else {
                InternalStatus::Answered
            },
            excerpts: evidence
                .into_iter()
                .map(|item| InternalExcerpt {
                    source_id: item.source_id,
                    logical_id: item.logical_id,
                    revision: item.revision,
                    content: item.content,
                })
                .collect(),
        };
        match serde_json::to_string(&response) {
            Ok(body) if response.validate(&query.request_id) && body.len() <= 2 * 1024 * 1024 => {
                ApiResponse {
                    status: 200,
                    content_type: "application/json",
                    body,
                }
            }
            _ => json_error(
                502,
                "invalid_internal_response",
                "Ungültige Operatorantwort",
            ),
        }
    }
}

pub fn router(service: InternalApiService) -> Router {
    Router::new()
        .route("/v1/operator/query", post(dispatch))
        .with_state(Arc::new(service))
}

async fn dispatch(
    State(service): State<Arc<InternalApiService>>,
    request: Request,
) -> axum::response::Response {
    let deadline =
        RequestDeadline::after(Duration::from_millis(service.deadline_ms.clamp(1, 60_000)));
    let _cancel = CancelOnDrop(deadline.clone());
    let permit = match service.slots.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => return respond(json_error(429, "overloaded", "Operatorweg ausgelastet")),
    };
    let (parts, body) = request.into_parts();
    let authorization = parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    if parts.headers.get_all(header::AUTHORIZATION).iter().count() != 1
        || !bearer_token(authorization.as_deref()).is_some_and(|token| service.allowed(token))
    {
        return respond(json_error(
            403,
            "forbidden",
            "Keine interne Operatorfreigabe",
        ));
    }
    if !parts
        .headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| {
            v.split(';')
                .next()
                .is_some_and(|v| v.trim().eq_ignore_ascii_case("application/json"))
        })
    {
        return respond(json_error(
            415,
            "unsupported_media_type",
            "application/json erforderlich",
        ));
    }
    let expires = deadline.expires_at().into();
    let operation = async move {
        let body = match to_bytes(body, 64 * 1024).await {
            Ok(body) => body,
            Err(_) => return json_error(413, "payload_too_large", "Anfrage zu groß"),
        };
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            service.handle_query(authorization.as_deref(), &body, deadline)
        })
        .await
        .unwrap_or_else(|_| json_error(503, "unavailable", "Operatorweg nicht verfügbar"))
    };
    respond(
        tokio::time::timeout_at(expires, operation)
            .await
            .unwrap_or_else(|_| deadline_response()),
    )
}

fn respond(response: ApiResponse) -> axum::response::Response {
    (
        StatusCode::from_u16(response.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        [
            (header::CONTENT_TYPE, response.content_type),
            (header::CACHE_CONTROL, "no-store"),
        ],
        response.body,
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{
        store::StoreResult, AuthorizedContext, Evidence, EvidenceKind, PortError,
    };
    use brain_policy::{AuthGrant, CredentialRegistry};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    struct Fixture {
        revoked: Arc<AtomicBool>,
        reads: Arc<AtomicUsize>,
    }
    impl RetrievalPort for Fixture {
        fn retrieve(&self, _: &Query, context: &AuthorizedContext) -> StoreResult<Vec<Evidence>> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            if context.knowledge_release != "internal-release-fixture" {
                return Err(PortError::PermissionDenied("fixture_release".into()));
            }
            Ok(vec![Evidence {
                evidence_id: "fixture-e1".into(),
                source_id: "fixture-source".into(),
                logical_id: "fixture-page".into(),
                revision: 1,
                kind: EvidenceKind::Prose,
                content: "Erfundener lokaler Testausschnitt".into(),
                citation: "fixture-page#1".into(),
                visibility: SourceVisibility::Internal,
                allowed_scopes: BTreeSet::from(["second_brain.internal".into()]),
                score: 1.0,
                provenance: None,
                patch: None,
            }])
        }
        fn validate_evidence(
            &self,
            _: &Query,
            _: &AuthorizedContext,
            _: &[Evidence],
            for_provider: bool,
        ) -> StoreResult<()> {
            if for_provider || self.revoked.load(Ordering::SeqCst) {
                Err(PortError::PermissionDenied("fixture_acl".into()))
            } else {
                Ok(())
            }
        }
    }
    fn query() -> Query {
        Query {
            request_id: "internal-fixture-r".into(),
            conversation_id: "internal-fixture-c".into(),
            text: "Erfundene Operatorfrage".into(),
            requested_scopes: BTreeSet::from(["second_brain.internal".into()]),
            domain: None,
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        }
    }
    fn service(
        actor: &str,
        channel: &str,
        scopes: &[&str],
        release: &str,
    ) -> (InternalApiService, Arc<AtomicBool>, Arc<AtomicUsize>) {
        let revoked = Arc::new(AtomicBool::new(false));
        let reads = Arc::new(AtomicUsize::new(0));
        let registry = CredentialRegistry::new(vec![AuthGrant::from_secret(
            "fixture-token",
            actor,
            channel,
            scopes.iter().map(|scope| (*scope).into()).collect(),
            BTreeSet::new(),
        )]);
        let service = InternalApiService::new(
            PolicyEngine::new(registry),
            Fixture {
                revoked: revoked.clone(),
                reads: reads.clone(),
            },
            release.into(),
            1000,
            Budget {
                max_network_rounds: 0,
                max_input_tokens: 1000,
                max_output_tokens: 1000,
                max_cost_micros: 0,
            },
        );
        (service, revoked, reads)
    }
    fn call(service: &InternalApiService, query: &Query) -> ApiResponse {
        service.handle_query(
            Some("Bearer fixture-token"),
            &serde_json::to_vec(query).unwrap(),
            RequestDeadline::after(Duration::from_secs(1)),
        )
    }

    struct PublicDocsFixture {
        visibility: SourceVisibility,
        scope: &'static str,
        revoked: bool,
    }
    impl RetrievalPort for PublicDocsFixture {
        fn retrieve(
            &self,
            query: &Query,
            context: &AuthorizedContext,
        ) -> StoreResult<Vec<Evidence>> {
            assert_eq!(context.knowledge_release, "internal-release-fixture");
            assert_eq!(context.principal.actor_id, "second-brain");
            assert_eq!(context.principal.channel, "internal");
            assert!(context.principal.provider_egress.is_empty());
            if query.requested_scopes != BTreeSet::from(["docs.public".into()])
                && query.requested_scopes != BTreeSet::from(["bot.public".into()])
            {
                return Ok(Vec::new());
            }
            assert_eq!(context.principal.scopes, query.requested_scopes);
            Ok(vec![Evidence {
                evidence_id: "docs-fixture-e1".into(),
                source_id: "docs-fixture".into(),
                logical_id: "public/antwort.html".into(),
                revision: 1,
                kind: EvidenceKind::Prose,
                content: "Freigegebener öffentlicher Dokumentationsausschnitt".into(),
                citation: "public/antwort.html#1".into(),
                visibility: self.visibility,
                allowed_scopes: BTreeSet::from([self.scope.into()]),
                score: 1.0,
                provenance: None,
                patch: None,
            }])
        }
        fn validate_evidence(
            &self,
            _: &Query,
            _: &AuthorizedContext,
            _: &[Evidence],
            for_provider: bool,
        ) -> StoreResult<()> {
            assert!(!for_provider);
            if self.revoked {
                Err(PortError::PermissionDenied("fixture_acl".into()))
            } else {
                Ok(())
            }
        }
        fn validate_publication(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            evidence: &[Evidence],
        ) -> StoreResult<()> {
            self.validate_evidence(query, context, evidence, false)
        }
    }
    fn docs_service(
        visibility: SourceVisibility,
        scope: &'static str,
        revoked: bool,
    ) -> InternalApiService {
        let (mut service, _, _) = service(
            "second-brain",
            "internal",
            &["second_brain.internal"],
            "internal-release-fixture",
        );
        service.retrieval = Arc::new(PublicDocsFixture {
            visibility,
            scope,
            revoked,
        });
        service
    }

    #[test]
    fn operator_public_docs_require_an_explicit_matching_release() {
        for pin in [
            None,
            Some("other-release"),
            Some("internal-release-fixture"),
        ] {
            let service = docs_service(SourceVisibility::Public, "docs.public", false);
            let service = match pin {
                Some(pin) => service.with_public_docs_release(pin.into()),
                None => service,
            };
            let response = call(&service, &query());
            assert_eq!(response.status, 200);
            let answer: InternalAnswerResponse = serde_json::from_str(&response.body).unwrap();
            assert_eq!(answer.knowledge_release, "internal-release-fixture");
            assert_eq!(
                answer.excerpts.len(),
                usize::from(pin == Some("internal-release-fixture"))
            );
            if !answer.excerpts.is_empty() {
                assert!(answer.validate(&query().request_id));
                assert_eq!(answer.status, InternalStatus::Answered);
            }
        }
    }

    #[test]
    fn operator_public_docs_reject_private_evidence_foreign_scopes_and_revocation() {
        for (visibility, scope, revoked, expected) in [
            (SourceVisibility::Internal, "docs.public", false, 502),
            (SourceVisibility::Private, "docs.public", false, 502),
            (SourceVisibility::Public, "bot.public", false, 502),
            (SourceVisibility::Public, "docs.public", true, 503),
        ] {
            let service = docs_service(visibility, scope, revoked)
                .with_public_docs_release("internal-release-fixture".into());
            assert_eq!(call(&service, &query()).status, expected);
        }
    }

    #[test]
    fn operator_public_docs_preserve_authentication_and_wire_scopes() {
        let service = docs_service(SourceVisibility::Public, "docs.public", false)
            .with_public_docs_release("internal-release-fixture".into());
        let mut query = query();
        assert_eq!(
            service
                .handle_query(
                    None,
                    &serde_json::to_vec(&query).unwrap(),
                    RequestDeadline::after(Duration::from_secs(1))
                )
                .status,
            401
        );
        query.requested_scopes = BTreeSet::from(["docs.public".into()]);
        assert_eq!(call(&service, &query).status, 400);
    }

    #[test]
    fn operator_maintenance_scope_preserves_release_auth_and_publication_guards() {
        for (visibility, scope, revoked, expected) in [
            (SourceVisibility::Public, "bot.public", false, 200),
            (SourceVisibility::Private, "bot.public", false, 502),
            (SourceVisibility::Internal, "bot.public", false, 502),
            (SourceVisibility::Public, "docs.public", false, 502),
            (SourceVisibility::Public, "bot.public", true, 503),
        ] {
            let service = docs_service(visibility, scope, revoked)
                .with_public_bot_release("internal-release-fixture".into());
            let response = call(&service, &query());
            assert_eq!(response.status, expected);
            if expected == 200 {
                let answer: InternalAnswerResponse = serde_json::from_str(&response.body).unwrap();
                assert_eq!(answer.status, InternalStatus::Answered);
                assert!(!answer.excerpts.is_empty());
                assert_eq!(
                    service
                        .handle_query(
                            None,
                            &serde_json::to_vec(&query()).unwrap(),
                            RequestDeadline::after(Duration::from_secs(1))
                        )
                        .status,
                    401
                );
                let mut altered = query();
                altered.requested_scopes = BTreeSet::from(["bot.public".into()]);
                assert_eq!(call(&service, &altered).status, 400);
            }
        }
        let service = docs_service(SourceVisibility::Public, "bot.public", false)
            .with_public_bot_release("foreign-release".into());
        let response = call(&service, &query());
        let answer: InternalAnswerResponse = serde_json::from_str(&response.body).unwrap();
        assert_eq!(answer.status, InternalStatus::InsufficientEvidence);
    }

    #[test]
    fn interner_typ_quelle_release_und_aktuelle_acl_werden_geprueft() {
        let (service, revoked, _) = service(
            "second-brain",
            "internal",
            &["second_brain.internal"],
            "internal-release-fixture",
        );
        let response = call(&service, &query());
        assert_eq!(response.status, 200);
        let answer: InternalAnswerResponse = serde_json::from_str(&response.body).unwrap();
        assert!(answer.validate(&query().request_id));
        assert_eq!(answer.knowledge_release, "internal-release-fixture");
        assert_eq!(answer.excerpts[0].logical_id, "fixture-page");
        assert!(
            serde_json::from_str::<brain_contracts::public_api::PublicAnswerResponse>(
                &response.body
            )
            .is_err()
        );
        revoked.store(true, Ordering::SeqCst);
        assert_eq!(call(&service, &query()).status, 503);
    }

    #[test]
    fn docs_fremde_identitaeten_und_scopes_erhalten_keine_internen_belege() {
        for (actor, channel, scopes) in [
            ("docs-client", "docs", vec!["docs.public"]),
            ("docs-client", "internal", vec!["second_brain.internal"]),
            ("second-brain", "twitch", vec!["second_brain.internal"]),
            (
                "second-brain",
                "internal",
                vec!["docs.public", "second_brain.internal"],
            ),
        ] {
            let (service, _, reads) = service(actor, channel, &scopes, "internal-release-fixture");
            assert_eq!(call(&service, &query()).status, 403);
            assert_eq!(reads.load(Ordering::SeqCst), 0);
        }
        let (service, _, reads) = service(
            "second-brain",
            "internal",
            &["second_brain.internal"],
            "internal-release-fixture",
        );
        for scope in ["docs.public", "bot.public"] {
            let mut query = query();
            query.requested_scopes = BTreeSet::from([scope.into()]);
            assert_eq!(call(&service, &query).status, 400);
        }
        for field in ["release_id", "principal"] {
            let mut query = serde_json::to_value(query()).unwrap();
            query[field] = serde_json::json!("injected");
            assert_eq!(
                service
                    .handle_query(
                        Some("Bearer fixture-token"),
                        query.to_string().as_bytes(),
                        RequestDeadline::after(Duration::from_secs(1))
                    )
                    .status,
                400
            );
        }
        assert_eq!(reads.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn falsches_oder_fehlendes_release_hat_keinen_fallback() {
        for release in ["", "current", "latest", "foreign-release-fixture"] {
            let (service, _, _) = service(
                "second-brain",
                "internal",
                &["second_brain.internal"],
                release,
            );
            assert_eq!(call(&service, &query()).status, 503);
        }
    }
}
