use brain_contracts::{
    store::ConversationOwnershipPort, AnswerProfile, Budget, Evidence, EvidenceKind, PortError,
    Principal, Query, SourceVisibility,
};
use brain_policy::{evidence_allowed, AuthGrant, CredentialRegistry, PolicyEngine};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};
fn query(scopes: &[&str]) -> Query {
    Query {
        request_id: "q1".into(),
        conversation_id: "c1".into(),
        text: "Abrams".into(),
        requested_scopes: scopes.iter().map(|s| (*s).into()).collect(),
        profile: AnswerProfile::Explain,
        patch: None,
        mode: None,
    }
}
fn registry(actor: &str) -> CredentialRegistry {
    CredentialRegistry::new(vec![AuthGrant::from_secret(
        "fixture-token",
        actor,
        "test",
        BTreeSet::from(["public".into(), "private".into()]),
        BTreeSet::from(["public".into()]),
    )])
}
#[test]
fn requested_scopes_narrow_the_verified_grant() {
    let engine = PolicyEngine::new(registry("actor-a"));
    let context = engine
        .authorize_query(
            "fixture-token",
            &query(&["public"]),
            "r1",
            1000,
            Budget::default(),
        )
        .unwrap();
    assert_eq!(context.principal.scopes, BTreeSet::from(["public".into()]));
    assert!(engine
        .authorize_query(
            "fixture-token",
            &query(&["admin"]),
            "r1",
            1000,
            Budget::default()
        )
        .is_err());
    assert!(engine
        .authorize_query("", &query(&[]), "r1", 1000, Budget::default())
        .is_err());
}
#[test]
fn public_visibility_cannot_override_an_explicit_object_acl() {
    let principal = Principal {
        actor_id: "actor-a".into(),
        channel: "test".into(),
        scopes: BTreeSet::new(),
        provider_egress: BTreeSet::new(),
    };
    let evidence = Evidence {
        evidence_id: "fixture".into(),
        source_id: "fixture".into(),
        logical_id: "fixture".into(),
        revision: 1,
        kind: EvidenceKind::Fact,
        content: "fixture".into(),
        citation: "fixture".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::from(["licensed".into()]),
        score: 1.0,
        provenance: None,
        patch: None,
    };
    assert!(!evidence_allowed(&principal, &evidence));
}
#[derive(Default)]
struct SharedOwners(Mutex<BTreeMap<String, String>>);
impl ConversationOwnershipPort for SharedOwners {
    fn claim_conversation(&self, conversation: &str, actor: &str) -> Result<(), PortError> {
        let mut owners = self.0.lock().unwrap();
        let owner = owners
            .entry(conversation.into())
            .or_insert_with(|| actor.into());
        if owner != actor {
            Err(PortError::InvalidResponse("owner mismatch".into()))
        } else {
            Ok(())
        }
    }
}
#[test]
fn new_policy_engine_does_not_reset_persisted_conversation_ownership() {
    let owners = Arc::new(SharedOwners::default());
    let first = PolicyEngine::with_ownership_store(registry("actor-a"), owners.clone());
    first
        .authorize_query("fixture-token", &query(&[]), "r1", 1000, Budget::default())
        .unwrap();
    drop(first);
    let restarted = PolicyEngine::with_ownership_store(registry("actor-b"), owners);
    assert!(restarted
        .authorize_query("fixture-token", &query(&[]), "r1", 1000, Budget::default())
        .is_err());
}
