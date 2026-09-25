#![forbid(unsafe_code)]

use std::{
    collections::{BTreeSet, HashMap},
    sync::{Arc, Mutex},
};

use brain_contracts::{AuthorizedContext, Budget, Evidence, Principal, Query, SourceVisibility};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PolicyError {
    #[error("ungueltige Zugangsdaten")]
    InvalidCredentials,
    #[error("angeforderter Scope ist nicht freigegeben: {0}")]
    ScopeDenied(String),
    #[error("Conversation gehoert einem anderen Principal")]
    ConversationOwnerMismatch,
    #[error("Evidence ist fuer diesen Principal nicht freigegeben")]
    EvidenceDenied,
    #[error("Policy Status konnte nicht gesperrt werden")]
    StatePoisoned,
}

pub type Result<T> = std::result::Result<T, PolicyError>;

#[derive(Clone, PartialEq, Eq)]
pub struct AuthGrant {
    token_sha256: String,
    pub actor_id: String,
    pub channel: String,
    pub scopes: BTreeSet<String>,
    pub provider_egress: BTreeSet<String>,
}

impl std::fmt::Debug for AuthGrant {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AuthGrant")
            .field("token_sha256", &"<redacted>")
            .field("actor_id", &self.actor_id)
            .field("channel", &self.channel)
            .field("scopes", &self.scopes)
            .field("provider_egress", &self.provider_egress)
            .finish()
    }
}

impl AuthGrant {
    pub fn from_secret(
        token: &str,
        actor_id: impl Into<String>,
        channel: impl Into<String>,
        scopes: BTreeSet<String>,
        provider_egress: BTreeSet<String>,
    ) -> Self {
        Self {
            token_sha256: digest_token(token),
            actor_id: actor_id.into(),
            channel: channel.into(),
            scopes,
            provider_egress,
        }
    }

    fn matches(&self, token: &str) -> bool {
        self.token_sha256 == digest_token(token)
    }

    fn principal(&self) -> Principal {
        Principal {
            actor_id: self.actor_id.clone(),
            channel: self.channel.clone(),
            scopes: self.scopes.clone(),
            provider_egress: self.provider_egress.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CredentialRegistry {
    grants: Vec<AuthGrant>,
}

impl CredentialRegistry {
    pub fn new(grants: Vec<AuthGrant>) -> Self {
        Self { grants }
    }

    pub fn authenticate(&self, token: &str) -> Result<Principal> {
        if token.is_empty() || token.len() > 4096 || token.chars().any(char::is_whitespace) {
            return Err(PolicyError::InvalidCredentials);
        }
        self.grants
            .iter()
            .find(|grant| grant.matches(token))
            .map(AuthGrant::principal)
            .ok_or(PolicyError::InvalidCredentials)
    }
}

#[derive(Clone, Default)]
pub struct PolicyEngine {
    credentials: CredentialRegistry,
    conversation_owners: Arc<Mutex<HashMap<String, String>>>,
    ownership_store: Option<Arc<dyn brain_contracts::store::ConversationOwnershipPort>>,
}

impl PolicyEngine {
    pub fn new(credentials: CredentialRegistry) -> Self {
        Self {
            credentials,
            conversation_owners: Arc::new(Mutex::new(HashMap::new())),
            ownership_store: None,
        }
    }

    pub fn with_ownership_store(
        credentials: CredentialRegistry,
        store: Arc<dyn brain_contracts::store::ConversationOwnershipPort>,
    ) -> Self {
        let mut engine = Self::new(credentials);
        engine.ownership_store = Some(store);
        engine
    }

    pub fn authorize_query(
        &self,
        bearer_token: &str,
        query: &Query,
        knowledge_release: impl Into<String>,
        deadline_ms: u64,
        budget: Budget,
    ) -> Result<AuthorizedContext> {
        let mut principal = self.credentials.authenticate(bearer_token)?;
        for requested in &query.requested_scopes {
            if !principal.scopes.contains(requested) {
                return Err(PolicyError::ScopeDenied(requested.clone()));
            }
        }

        // An explicit scope selection narrows the authenticated grant; it never widens it.
        if !query.requested_scopes.is_empty() {
            principal.scopes = query.requested_scopes.clone();
        }

        if let Some(store) = &self.ownership_store {
            store
                .claim_conversation(&query.conversation_id, &principal.actor_id)
                .map_err(|error| match error {
                    brain_contracts::PortError::InvalidResponse(_) => {
                        PolicyError::ConversationOwnerMismatch
                    }
                    _ => PolicyError::StatePoisoned,
                })?;
        } else {
            let mut owners = self
                .conversation_owners
                .lock()
                .map_err(|_| PolicyError::StatePoisoned)?;
            match owners.get(&query.conversation_id) {
                Some(owner) if owner != &principal.actor_id => {
                    return Err(PolicyError::ConversationOwnerMismatch)
                }
                Some(_) => {}
                None => {
                    if owners.len() >= 65536 {
                        return Err(PolicyError::StatePoisoned);
                    }
                    owners.insert(query.conversation_id.clone(), principal.actor_id.clone());
                }
            }
        }

        Ok(AuthorizedContext {
            principal,
            conversation_id: query.conversation_id.clone(),
            knowledge_release: knowledge_release.into(),
            deadline_ms,
            budget,
        })
    }
}

pub fn evidence_allowed(principal: &Principal, evidence: &Evidence) -> bool {
    (evidence.visibility == SourceVisibility::Public || !evidence.allowed_scopes.is_empty())
        && evidence.allowed_scopes.is_subset(&principal.scopes)
}

pub fn provider_egress_allowed(principal: &Principal, egress_class: &str) -> bool {
    principal.provider_egress.contains(egress_class)
}

pub fn require_evidence(principal: &Principal, evidence: &Evidence) -> Result<()> {
    if evidence_allowed(principal, evidence) {
        Ok(())
    } else {
        Err(PolicyError::EvidenceDenied)
    }
}

fn digest_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{AnswerProfile, EvidenceKind, SourceVisibility};

    fn scope_set(values: &[&str]) -> BTreeSet<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    fn query(scopes: &[&str], conversation_id: &str) -> Query {
        Query {
            request_id: "r1".into(),
            conversation_id: conversation_id.into(),
            text: "Abrams".into(),
            requested_scopes: scope_set(scopes),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        }
    }

    fn engine() -> PolicyEngine {
        PolicyEngine::new(CredentialRegistry::new(vec![AuthGrant::from_secret(
            "secret-token",
            "actor-1",
            "twitch",
            scope_set(&["docs.public", "docs.internal"]),
            scope_set(&["public"]),
        )]))
    }

    #[test]
    fn body_cannot_grant_a_scope() {
        let error = engine()
            .authorize_query(
                "secret-token",
                &query(&["admin"], "c1"),
                "k1",
                5000,
                Budget::default(),
            )
            .unwrap_err();
        assert_eq!(error, PolicyError::ScopeDenied("admin".into()));
    }

    #[test]
    fn conversation_owner_cannot_be_reused_by_other_principal() {
        let registry = CredentialRegistry::new(vec![
            AuthGrant::from_secret(
                "first",
                "actor-1",
                "twitch",
                scope_set(&["docs.public"]),
                BTreeSet::new(),
            ),
            AuthGrant::from_secret(
                "second",
                "actor-2",
                "mcp",
                scope_set(&["docs.public"]),
                BTreeSet::new(),
            ),
        ]);
        let engine = PolicyEngine::new(registry);
        engine
            .authorize_query(
                "first",
                &query(&[], "shared"),
                "k1",
                5000,
                Budget::default(),
            )
            .unwrap();
        assert_eq!(
            engine
                .authorize_query(
                    "second",
                    &query(&[], "shared"),
                    "k1",
                    5000,
                    Budget::default()
                )
                .unwrap_err(),
            PolicyError::ConversationOwnerMismatch
        );
    }

    #[test]
    fn private_evidence_requires_its_scope() {
        let principal = Principal {
            actor_id: "actor".into(),
            channel: "mcp".into(),
            scopes: scope_set(&["docs.public"]),
            provider_egress: BTreeSet::new(),
        };
        let evidence = Evidence {
            evidence_id: "e1".into(),
            source_id: "internal".into(),
            logical_id: "doc/1".into(),
            revision: 1,
            kind: EvidenceKind::Prose,
            content: "intern".into(),
            citation: "internal:doc/1".into(),
            visibility: SourceVisibility::Private,
            allowed_scopes: scope_set(&["docs.internal"]),
            score: 1.0,
            patch: None,
        };
        assert!(!evidence_allowed(&principal, &evidence));
    }

    #[test]
    fn private_evidence_without_explicit_scope_is_denied() {
        let principal = Principal {
            actor_id: "actor".into(),
            channel: "mcp".into(),
            scopes: scope_set(&["docs.internal"]),
            provider_egress: BTreeSet::new(),
        };
        let evidence = Evidence {
            evidence_id: "e2".into(),
            source_id: "internal".into(),
            logical_id: "doc/2".into(),
            revision: 1,
            kind: EvidenceKind::Prose,
            content: "intern".into(),
            citation: "internal:doc/2".into(),
            visibility: SourceVisibility::Internal,
            allowed_scopes: BTreeSet::new(),
            score: 1.0,
            patch: None,
        };
        assert!(!evidence_allowed(&principal, &evidence));
    }

    #[test]
    fn grant_debug_never_contains_raw_token() {
        let grant = AuthGrant::from_secret(
            "super-secret",
            "actor",
            "mcp",
            BTreeSet::new(),
            BTreeSet::new(),
        );
        let debug = format!("{grant:?}");
        assert!(!debug.contains("super-secret"));
        assert!(debug.contains("<redacted>"));
    }
}
