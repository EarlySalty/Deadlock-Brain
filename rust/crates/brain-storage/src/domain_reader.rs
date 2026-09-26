//! Typed domain views reuse canonical release storage rather than a second mutable truth store.
use crate::memory_repository::validate_release;
use brain_contracts::{
    domain::{
        DomainObject, DomainSnapshot, DomainStorePort, StoredDomainObject, Validity,
        DOMAIN_CONTRACT_VERSION, LEGACY_DOMAIN_CONTRACT_VERSION,
    },
    AuthorizedContext, DocumentRevision, PortError, SnapshotReadPort, SourceRecordV2,
};
use std::collections::{BTreeMap, BTreeSet};
#[path = "domain_knowledge.rs"]
mod knowledge;
impl<S: SnapshotReadPort> DomainStorePort for DomainReader<S> {
    fn read_domain(
        &self,
        context: &AuthorizedContext,
        validity: &Validity,
    ) -> Result<DomainSnapshot, PortError> {
        self.read_with_egress(context, validity, false)
    }
}

#[derive(Clone)]
pub struct DomainReader<S> {
    store: S,
}
impl<S> DomainReader<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }
}
fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}
fn stable(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 512 && !value.chars().any(char::is_control)
}
fn source_visible(
    source: &DocumentRevision,
    visible: &[SourceRecordV2],
) -> Result<bool, PortError> {
    let Some(record) = visible
        .iter()
        .find(|r| r.source_id == source.source_id && r.logical_id == source.logical_id)
    else {
        // A source withdrawn by its CURRENT ACL cannot authorize a derived public object.
        return Ok(false);
    };
    if source.revision != record.revision || source.content_hash != record.content_hash {
        return Err(invalid("domain provenance revision or hash mismatch"));
    }
    Ok(true)
}
impl<S: SnapshotReadPort> DomainReader<S> {
    pub fn read_with_egress(
        &self,
        context: &AuthorizedContext,
        validity: &Validity,
        for_provider: bool,
    ) -> Result<DomainSnapshot, PortError> {
        if !(1..=60000).contains(&context.deadline_ms)
            || !stable(&validity.patch)
            || !stable(&validity.mode)
        {
            return Err(invalid("invalid domain context"));
        }
        let snapshot = self.store.read_snapshot(&context.knowledge_release)?;
        validate_release(&snapshot.release)?;
        if snapshot.release.release_id != context.knowledge_release
            || snapshot.release.patch != validity.patch
        {
            return Err(invalid("domain release or patch mismatch"));
        }
        let visible: Vec<_> = snapshot
            .authorized(&context.principal, for_provider)?
            .into_iter()
            .filter(|r| knowledge::validity_matches(r, validity))
            .collect();
        let mut cards = BTreeMap::new();
        let mut catalogs = BTreeMap::new();
        let mut object_sources = BTreeMap::new();
        let mut facts = BTreeMap::new();
        let mut rules = BTreeMap::new();
        let mut predicates = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for record in &visible {
            let Some(schema) = record.metadata.get("domain_contract") else {
                continue;
            };
            if ![DOMAIN_CONTRACT_VERSION, LEGACY_DOMAIN_CONTRACT_VERSION].contains(&schema.as_str())
                || record.content.len() > 256 * 1024
            {
                return Err(invalid("unsupported or oversized domain object"));
            }
            let stored: StoredDomainObject = serde_json::from_str(&record.content)
                .map_err(|_| invalid("invalid typed domain object"))?;
            if stored.contract_version != *schema {
                return Err(invalid("domain envelope version mismatch"));
            }
            if let Some(encoded) = record
                .metadata
                .get(brain_contracts::domain::DOMAIN_DEPENDENCIES_METADATA_KEY)
            {
                if encoded.len() > 64 * 1024 {
                    return Err(invalid("oversized domain dependencies"));
                }
                let dependencies: brain_contracts::source::Versioned<Vec<DocumentRevision>> =
                    serde_json::from_str(encoded)
                        .map_err(|_| invalid("invalid domain dependency contract"))?;
                if dependencies.data.len() > 64 {
                    return Err(invalid("domain dependency budget exceeded"));
                }
                let mut identities = BTreeSet::new();
                let mut authorized = true;
                for dependency in &dependencies.data {
                    if !identities.insert((&dependency.source_id, &dependency.logical_id)) {
                        return Err(invalid("ambiguous domain dependency"));
                    }
                    authorized &= source_visible(dependency, &visible)?;
                }
                if !authorized {
                    continue;
                }
            }
            match stored.object {
                DomainObject::NumericFact(fact) => {
                    if !fact.verified || fact.validity != *validity {
                        continue;
                    }
                    if !stable(&fact.fact_id)
                        || !stable(&fact.subject_id)
                        || !stable(&fact.predicate)
                        || !fact.quantity.value.is_finite()
                        || !stable(&fact.quantity.unit)
                    {
                        return Err(invalid("invalid verified numeric fact"));
                    }
                    if !source_visible(&fact.source, &visible)? {
                        continue;
                    }
                    if !seen.insert(("fact", fact.fact_id.clone())) {
                        return Err(invalid("ambiguous fact identity"));
                    }
                    let predicate = (fact.subject_id.clone(), fact.predicate.clone());
                    if let Some(quantity) = predicates.insert(predicate, fact.quantity.clone()) {
                        if quantity != fact.quantity {
                            return Err(invalid("conflicting verified domain facts"));
                        }
                    }
                    object_sources.insert(
                        format!("fact:{}", fact.fact_id),
                        knowledge::reference(record),
                    );
                    facts.insert(fact.fact_id.clone(), fact);
                }
                DomainObject::Rule(rule) => {
                    if !rule.verified || rule.validity != *validity {
                        continue;
                    }
                    if !stable(&rule.rule_id) || !stable(&rule.rule_version) {
                        return Err(invalid("invalid verified rule"));
                    }
                    if !source_visible(&rule.source, &visible)? {
                        continue;
                    }
                    if !seen.insert(("rule", rule.rule_id.clone())) {
                        return Err(invalid("ambiguous rule identity"));
                    }
                    object_sources.insert(
                        format!("rule:{}", rule.rule_id),
                        knowledge::reference(record),
                    );
                    rules.insert(rule.rule_id.clone(), rule);
                }
                DomainObject::HeroCard(card) => {
                    let card = *card;
                    if schema != DOMAIN_CONTRACT_VERSION {
                        return Err(invalid("v2 card in legacy envelope"));
                    }
                    if card.validity != *validity {
                        continue;
                    }
                    if card.card.knowledge_release != snapshot.release.release_id {
                        return Err(invalid("card release mismatch"));
                    }
                    let Some(refs) = knowledge::card_sources(&card, &visible)? else {
                        continue;
                    };
                    if !refs
                        .iter()
                        .map(|r| source_visible(r, &visible))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .all(|v| v)
                    {
                        continue;
                    }
                    if !seen.insert(("card", card.card.hero_id.clone())) {
                        return Err(invalid("ambiguous hero revision"));
                    }
                    object_sources.insert(
                        format!("card:{}", card.card.hero_id),
                        knowledge::reference(record),
                    );
                    cards.insert(card.card.hero_id.clone(), card);
                }
                DomainObject::BuildCatalog(catalog) => {
                    let catalog = *catalog;
                    if schema != DOMAIN_CONTRACT_VERSION {
                        return Err(invalid("v2 catalog in legacy envelope"));
                    }
                    if catalog.validity != *validity {
                        continue;
                    }
                    let refs = knowledge::catalog_sources(&catalog)?;
                    if !refs
                        .iter()
                        .map(|r| source_visible(r, &visible))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .all(|v| v)
                    {
                        continue;
                    }
                    if !seen.insert(("catalog", catalog.catalog_id.clone())) {
                        return Err(invalid("ambiguous build catalog"));
                    }
                    object_sources.insert(
                        format!("catalog:{}", catalog.catalog_id),
                        knowledge::reference(record),
                    );
                    catalogs.insert(catalog.catalog_id.clone(), catalog);
                }
            }
        }
        Ok(DomainSnapshot {
            release: snapshot.release,
            validity: validity.clone(),
            facts: facts.into_values().collect(),
            rules: rules.into_values().collect(),
            cards: cards.into_values().collect(),
            catalogs: catalogs.into_values().collect(),
            records: visible,
            object_sources,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryRepository;
    use brain_contracts::{
        domain::{NumericFact, Quantity, RuleExpr, TypedRule},
        Budget, DocumentStorePort, Principal, SourceVisibility,
    };
    fn record(id: &str, content: &str) -> SourceRecordV2 {
        SourceRecordV2 {
            source_id: "domain-fixture".into(),
            logical_id: id.into(),
            revision: 1,
            content_hash: format!("hash-{id}"),
            content: content.into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            tombstone: false,
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        }
    }
    fn validity() -> Validity {
        Validity {
            patch: "p1".into(),
            mode: "ranked".into(),
        }
    }
    fn context() -> AuthorizedContext {
        AuthorizedContext {
            principal: Principal {
                actor_id: "fixture".into(),
                channel: "test".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::new(),
            },
            conversation_id: "fixture".into(),
            knowledge_release: "r1".into(),
            deadline_ms: 1000,
            budget: Budget::default(),
        }
    }
    fn fact() -> NumericFact {
        NumericFact {
            fact_id: "damage".into(),
            subject_id: "hero".into(),
            predicate: "damage".into(),
            quantity: Quantity {
                value: 40.0,
                unit: "damage".into(),
            },
            validity: validity(),
            source: DocumentRevision {
                source_id: "domain-fixture".into(),
                logical_id: "raw".into(),
                revision: 1,
                content_hash: "hash-raw".into(),
            },
            verified: true,
        }
    }
    fn typed(id: &str, object: DomainObject) -> SourceRecordV2 {
        let mut r = record(
            id,
            &serde_json::to_string(&StoredDomainObject {
                contract_version: DOMAIN_CONTRACT_VERSION.into(),
                object,
            })
            .unwrap(),
        );
        r.metadata
            .insert("domain_contract".into(), DOMAIN_CONTRACT_VERSION.into());
        r
    }
    async fn store(objects: Vec<SourceRecordV2>) -> MemoryRepository {
        let s = MemoryRepository::default();
        s.apply_record(record("raw", "verified source fixture"))
            .unwrap();
        for object in objects {
            s.apply_record(object).unwrap();
        }
        let release = s.release_from_heads("r1", "v1", "p1").unwrap();
        s.publish(&release).await.unwrap();
        s
    }
    #[tokio::test]
    async fn typed_facts_rules_are_pinned_and_source_acl_is_rechecked() {
        let f = fact();
        let r = TypedRule {
            rule_id: "base".into(),
            rule_version: "1".into(),
            validity: validity(),
            source: f.source.clone(),
            verified: true,
            expression: RuleExpr::Fact {
                fact_id: f.fact_id.clone(),
            },
        };
        let s = store(vec![
            typed("fact", DomainObject::NumericFact(f.clone())),
            typed("rule", DomainObject::Rule(r.clone())),
        ])
        .await;
        let reader = DomainReader::new(s.clone());
        let initial = reader.read_domain(&context(), &validity()).unwrap();
        assert_eq!(initial.facts, vec![f]);
        assert_eq!(initial.rules, vec![r]);
        let mut changed = record("raw", "newer text");
        changed.revision = 2;
        changed.content_hash = "new-hash".into();
        s.apply_record(changed.clone()).unwrap();
        assert_eq!(
            reader.read_domain(&context(), &validity()).unwrap(),
            initial
        );
        changed.revision = 3;
        changed.visibility = SourceVisibility::Private;
        changed.allowed_scopes.insert("private".into());
        s.apply_record(changed).unwrap();
        let hidden = reader.read_domain(&context(), &validity()).unwrap();
        assert!(hidden.facts.is_empty());
        assert!(hidden.rules.is_empty());
    }
    #[tokio::test]
    async fn forged_hash_and_ambiguous_facts_fail_closed() {
        let mut forged = fact();
        forged.source.content_hash = "forged".into();
        let reader =
            DomainReader::new(store(vec![typed("fact", DomainObject::NumericFact(forged))]).await);
        assert!(reader.read_domain(&context(), &validity()).is_err());
        let reader = DomainReader::new(
            store(vec![
                typed("fact-a", DomainObject::NumericFact(fact())),
                typed("fact-b", DomainObject::NumericFact(fact())),
            ])
            .await,
        );
        assert!(reader.read_domain(&context(), &validity()).is_err());
    }
    #[tokio::test]
    async fn unverified_wrong_mode_and_wrong_patch_are_not_promoted() {
        let mut unverified = fact();
        unverified.verified = false;
        let reader = DomainReader::new(
            store(vec![typed("fact", DomainObject::NumericFact(unverified))]).await,
        );
        assert!(reader
            .read_domain(&context(), &validity())
            .unwrap()
            .facts
            .is_empty());
        let reader =
            DomainReader::new(store(vec![typed("fact", DomainObject::NumericFact(fact()))]).await);
        let mut v = validity();
        v.mode = "casual".into();
        assert!(reader.read_domain(&context(), &v).unwrap().facts.is_empty());
        v.patch = "p2".into();
        assert!(reader.read_domain(&context(), &v).is_err());
    }
    #[tokio::test]
    async fn wiki_dependency_pins_reject_unknown_versions_forgery_and_duplicates() {
        use brain_contracts::{domain::DOMAIN_DEPENDENCIES_METADATA_KEY, source::Versioned};
        let dependency = fact().source;
        for encoded in [
            serde_json::json!({"contract_version":"brain.ir.v999","data":[]}).to_string(),
            serde_json::to_string(&Versioned::new(vec![DocumentRevision {
                content_hash: "forged".into(),
                ..dependency.clone()
            }]))
            .unwrap(),
            serde_json::to_string(&Versioned::new(vec![
                dependency.clone(),
                dependency.clone(),
            ]))
            .unwrap(),
        ] {
            let mut record = typed("fact", DomainObject::NumericFact(fact()));
            record
                .metadata
                .insert(DOMAIN_DEPENDENCIES_METADATA_KEY.into(), encoded);
            assert!(DomainReader::new(store(vec![record]).await)
                .read_domain(&context(), &validity())
                .is_err());
        }
        let mut record = typed("fact", DomainObject::NumericFact(fact()));
        record.metadata.insert(
            DOMAIN_DEPENDENCIES_METADATA_KEY.into(),
            serde_json::to_string(&Versioned::new(vec![dependency])).unwrap(),
        );
        assert_eq!(
            DomainReader::new(store(vec![record]).await)
                .read_domain(&context(), &validity())
                .unwrap()
                .facts
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn legacy_v1_numeric_facts_remain_readable_without_value_or_identity_changes() {
        let expected = fact();
        let mut legacy = typed("fact", DomainObject::NumericFact(expected.clone()));
        legacy.content = serde_json::to_string(&StoredDomainObject {
            contract_version: LEGACY_DOMAIN_CONTRACT_VERSION.into(),
            object: DomainObject::NumericFact(expected.clone()),
        })
        .unwrap();
        legacy.metadata.insert(
            "domain_contract".into(),
            LEGACY_DOMAIN_CONTRACT_VERSION.into(),
        );
        let view = DomainReader::new(store(vec![legacy]).await)
            .read_domain(&context(), &validity())
            .unwrap();
        assert_eq!(view.facts, vec![expected]);
    }

    #[tokio::test]
    async fn future_contract_and_malformed_domain_payload_are_rejected() {
        let mut future = typed("fact", DomainObject::NumericFact(fact()));
        future
            .metadata
            .insert("domain_contract".into(), "brain.domain.v999".into());
        assert!(DomainReader::new(store(vec![future]).await)
            .read_domain(&context(), &validity())
            .is_err());
        let mut malformed = typed("fact", DomainObject::NumericFact(fact()));
        malformed.content = "{}".into();
        assert!(DomainReader::new(store(vec![malformed]).await)
            .read_domain(&context(), &validity())
            .is_err());
    }
}
