//! Reiner Leseweg mit derselben Veröffentlichungsschranke wie öffentliche Antworten.
use super::{bearer_token, deadline_response, json_error, ApiResponse, ApiService};
use brain_contracts::{
    public_api::{PublicEvidence, PublicRetrievalResponse},
    AnswerStatus, PortError, Query, RequestDeadline, SourceVisibility, PUBLIC_API_VERSION,
};
use brain_kernel::AnswerKernelPort;
use brain_policy::PolicyError;

impl<K: AnswerKernelPort> ApiService<K> {
    pub fn handle_retrieve(&self, authorization: Option<&str>, body: &[u8]) -> ApiResponse {
        self.handle_retrieve_until(
            authorization,
            body,
            RequestDeadline::after(std::time::Duration::from_millis(
                self.deadline_ms.clamp(1, 60000),
            )),
        )
    }

    pub(crate) fn handle_retrieve_until(
        &self,
        authorization: Option<&str>,
        body: &[u8],
        deadline: RequestDeadline,
    ) -> ApiResponse {
        if deadline.check().is_err() {
            return deadline_response();
        }
        if body.len() > 64 * 1024 {
            return json_error(413, "payload_too_large", "Anfrage ist zu groß");
        }
        if self.knowledge_release.trim().is_empty()
            || self.knowledge_release == "current"
            || !(1..=60000).contains(&self.deadline_ms)
        {
            return json_error(503, "not_ready", "Kein gültiger Wissensstand konfiguriert");
        }
        let Some(token) = bearer_token(authorization) else {
            return json_error(401, "unauthorized", "Bearer-Token fehlt oder ist ungültig");
        };
        let query: Query = match serde_json::from_slice(body) {
            Ok(query) => query,
            Err(_) => return json_error(400, "invalid_request", "Query-Contract ist ungültig"),
        };
        if query.validate().is_err() {
            return json_error(400, "invalid_request", "Query-Contract ist ungültig");
        }
        // Domänenbeweise enthalten interne Eingangsrevisionen und Quelllocator.
        // Ihr eigener Auswertungsweg bleibt ausschließlich bei /v1/answer.
        if query.domain.is_some() || query.profile == brain_contracts::AnswerProfile::Build {
            return json_error(
                400,
                "unsupported_domain",
                "Belegsuche unterstützt ausschließlich Textfragen",
            );
        }
        let knowledge_release = match self.release_for_token(token) {
            Ok(release) => release,
            Err(response) => return response,
        };
        let context = match self.policy.authorize_query_until(
            token,
            &query,
            knowledge_release,
            deadline.clone(),
            self.budget.clone(),
        ) {
            Ok(context) => context,
            Err(PolicyError::BudgetExceeded) => return deadline_response(),
            Err(PolicyError::InvalidCredentials) => {
                return json_error(401, "unauthorized", "Zugangsdaten sind ungültig")
            }
            Err(PolicyError::StatePoisoned) => {
                return json_error(503, "unavailable", "Datenbankkapazität ist nicht verfügbar")
            }
            Err(_) => return json_error(403, "forbidden", "Anfrage ist nicht freigegeben"),
        };
        let Some(retrieval) = &self.retrieval else {
            return json_error(503, "unavailable", "Belegsuche ist nicht verfügbar");
        };
        let evidence = match retrieval.retrieve(&query, &context) {
            Ok(evidence) => evidence,
            Err(error) => return retrieval_error(error),
        };
        if deadline.check().is_err() {
            return deadline_response();
        }
        // Die gesamte Abhängigkeit wird erneut gegen aktuelle Freigaben geprüft.
        if !evidence.is_empty() {
            if let Err(error) = retrieval.validate_publication(&query, &context, &evidence) {
                return retrieval_error(error);
            }
        }
        if deadline.check().is_err() {
            return deadline_response();
        }
        if evidence.len() > 100
            || evidence.iter().any(|item| {
                item.validate().is_err()
                    || item.visibility != SourceVisibility::Public
                    || !brain_policy::evidence_allowed(&context.principal, item)
            })
        {
            return json_error(
                502,
                "invalid_retrieval_response",
                "Ungültige öffentliche Belege",
            );
        }
        let response = PublicRetrievalResponse {
            contract_version: PUBLIC_API_VERSION.into(),
            request_id: query.request_id,
            knowledge_release: context.knowledge_release,
            status: if evidence.is_empty() {
                AnswerStatus::InsufficientEvidence
            } else {
                AnswerStatus::Answered
            },
            evidence: evidence
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let citation = brain_contracts::PublicCitation::from_evidence(item, index);
                    PublicEvidence {
                        citation_id: citation.citation_id,
                        label: citation.label,
                        text: item.content.clone(),
                        kind: item.kind,
                    }
                })
                .collect(),
            truncated: false,
            out_of_domain: false,
        };
        match serde_json::to_string(&response) {
            Ok(body) if body.len() <= 2 * 1024 * 1024 => ApiResponse {
                status: 200,
                content_type: "application/json",
                body,
            },
            Ok(_) => json_error(502, "invalid_retrieval_response", "Belegpaket ist zu groß"),
            Err(_) => json_error(
                500,
                "serialization_error",
                "Belege konnten nicht serialisiert werden",
            ),
        }
    }
}

fn retrieval_error(error: PortError) -> ApiResponse {
    match error {
        PortError::BudgetExceeded => deadline_response(),
        PortError::PermissionDenied(_) => json_error(
            403,
            "forbidden",
            "Belege sind nicht zur Veröffentlichung freigegeben",
        ),
        PortError::InvalidResponse(_) => {
            json_error(502, "invalid_retrieval_response", "Ungültige Belegantwort")
        }
        PortError::Unavailable(_) => json_error(
            503,
            "unavailable",
            "Belegsuche ist vorübergehend nicht verfügbar",
        ),
    }
}
