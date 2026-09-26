//! Deterministic domain dispatch in the existing release retriever. Derived packs
//! are recomputed from canonical authorized data, never trusted by their kind tag.
use brain_contracts::{
    domain::*, value::Observed, AnswerProfile, AuthorizedContext, CorpusSnapshot, DocumentRevision,
    Evidence, EvidenceKind, PortError, Query, SnapshotReadPort, SourceRecordV2, SourceVisibility,
};
use brain_storage::DomainReader;
use dbrain_reasoner::{
    core_rules::CoreRuleEvaluator,
    domain_builds::{evaluate_build, resolve_hero},
};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub(crate) fn handles(query: &Query) -> bool {
    query.profile == AnswerProfile::Build || query.domain.is_some()
}
struct Reader<'a, S>(&'a S);
impl<S: SnapshotReadPort> SnapshotReadPort for Reader<'_, S> {
    fn read_snapshot(&self, id: &str) -> Result<CorpusSnapshot, PortError> {
        self.0.read_snapshot(id)
    }
}
fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}

pub(crate) fn retrieve<S: SnapshotReadPort>(
    store: &S,
    query: &Query,
    context: &AuthorizedContext,
    provider: bool,
) -> Result<Vec<Evidence>, PortError> {
    query
        .validate()
        .map_err(|_| invalid("invalid domain request"))?;
    if query.conversation_id != context.conversation_id
        || !query.requested_scopes.is_subset(&context.principal.scopes)
    {
        return Err(invalid("invalid domain authorization context"));
    }
    let Some(request) = query.domain.clone() else {
        return Ok(Vec::new());
    };
    if matches!(request, DomainRequest::Build { .. }) && query.profile != AnswerProfile::Build
        || query.profile == AnswerProfile::Build
            && !matches!(
                request,
                DomainRequest::Build { .. } | DomainRequest::Rule { .. }
            )
    {
        return Ok(Vec::new());
    }
    // No guessed game validity and no automatic current/latest patch.
    let (Some(patch), Some(mode)) = (&query.patch, &query.mode) else {
        return Ok(Vec::new());
    };
    let validity = Validity {
        patch: patch.clone(),
        mode: mode.clone(),
    };
    let snapshot =
        DomainReader::new(Reader(store)).read_with_egress(context, &validity, provider)?;
    let mut proof = Proof::new(&snapshot);
    let mut answer = DomainAnswer {
        contract_version: DOMAIN_ANSWER_VERSION.into(),
        route: DomainRoute::Fact,
        verdict: DomainVerdict::Proven,
        text: String::new(),
        knowledge_release: context.knowledge_release.clone(),
        validity,
        inputs: Vec::new(),
        input_fact_ids: BTreeSet::new(),
        rule_evaluation: None,
        build_evaluation: None,
    };
    match request {
        DomainRequest::Fact {
            hero,
            locale,
            predicate,
        } => {
            let Some(card) = resolve_hero(&snapshot.cards, &hero, &locale) else {
                return Ok(Vec::new());
            };
            proof.card(card)?;
            let facts: Vec<_> = card
                .card
                .facts
                .iter()
                .filter(|f| f.subject_id == card.card.hero_id && f.key == predicate)
                .collect();
            if facts.len() == 1 {
                let fact = facts[0];
                let value = match &fact.value {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
                answer.text = format!(
                    "{}: {} = {} {}. [Beleg 1]",
                    card.card.hero_id,
                    fact.key,
                    value,
                    fact.unit.as_deref().unwrap_or("")
                );
                answer.input_fact_ids.insert(fact.fact_id.clone());
            } else {
                answer.verdict = DomainVerdict::Unknown;
                answer.text = "Der angefragte Wert ist unbekannt oder nur unter nicht aufgelösten Bedingungen belegt. Er wird nicht als 0 eingesetzt. [Beleg 1]".into();
            }
        }
        DomainRequest::Rule { rule_id } => {
            answer.route = DomainRoute::Rule;
            let Some(rule) = snapshot.rules.iter().find(|r| r.rule_id == rule_id) else {
                return Ok(Vec::new());
            };
            proof.object(&format!("rule:{rule_id}"))?;
            proof.document(&rule.source)?;
            match CoreRuleEvaluator.evaluate(
                rule,
                &snapshot.facts,
                &snapshot.release,
                &snapshot.validity,
            ) {
                Ok(evaluation) => {
                    for id in &evaluation.input_fact_ids {
                        let fact = snapshot
                            .facts
                            .iter()
                            .find(|f| &f.fact_id == id)
                            .ok_or_else(|| invalid("rule fact disappeared"))?;
                        proof.object(&format!("fact:{id}"))?;
                        proof.document(&fact.source)?;
                    }
                    answer.text = format!("Regel {} (Version {}): {} {}. Berechnet aus {} verifizierten Eingabefakten, nicht durch ein Sprachmodell. [Beleg 1]", evaluation.rule_id, evaluation.rule_version, evaluation.quantity.value, evaluation.quantity.unit, evaluation.input_fact_ids.len());
                    answer.input_fact_ids = evaluation.input_fact_ids.clone();
                    answer.rule_evaluation = Some(evaluation);
                }
                Err(_) => {
                    answer.verdict = DomainVerdict::Unknown;
                    answer.text = "Die Regel ist belegt, aber ihre Eingaben, Einheiten oder Bedingungen reichen für eine sichere Berechnung nicht aus. Kein Ersatzwert 0. [Beleg 1]".into();
                }
            }
        }
        DomainRequest::Build {
            hero,
            locale,
            catalog_id,
            items,
        } => {
            answer.route = DomainRoute::Build;
            let Some(card) = resolve_hero(&snapshot.cards, &hero, &locale) else {
                return Ok(Vec::new());
            };
            let Some(catalog) = snapshot
                .catalogs
                .iter()
                .find(|c| c.catalog_id == catalog_id)
            else {
                return Ok(Vec::new());
            };
            proof.card(card)?;
            proof.object(&format!("catalog:{catalog_id}"))?;
            proof.located(&catalog.models)?;
            proof.located(&catalog.inventory_rules)?;
            for dependency in &catalog.dependencies {
                proof.located(dependency)?;
            }
            for alias in &catalog.aliases {
                proof.located(&alias.alias.provenance)?;
            }
            if let Some(fact) = snapshot
                .facts
                .iter()
                .find(|f| f.fact_id == catalog.active_limit_fact_id)
            {
                proof.object(&format!("fact:{}", fact.fact_id))?;
                proof.document(&fact.source)?;
                answer.input_fact_ids.insert(fact.fact_id.clone());
            }
            let result = evaluate_build(&snapshot, catalog, &items, &locale);
            let (verdict, label) = match result.legal {
                Observed::Known { value: true } => (DomainVerdict::Proven, "Build legal"),
                Observed::Known { value: false } => (DomainVerdict::Rejected, "Build abgelehnt"),
                Observed::Unknown { .. } => (DomainVerdict::Unknown, "Build nicht prüfbar"),
            };
            answer.verdict = verdict;
            let cost = match result.spent_souls {
                Observed::Known { value } => format!(" Gesamtausgaben: {value} Seelen."),
                Observed::Unknown { .. } => String::new(),
            };
            answer.text = format!("{label}: {}{cost} Prüfung für Patch {} und Modus {}; Kaufreihenfolge ohne Verkäufe. [Beleg 1]", result.reason, snapshot.validity.patch, snapshot.validity.mode);
            answer.build_evaluation = Some(result);
        }
        DomainRequest::Card { hero, locale } => {
            answer.route = DomainRoute::Card;
            let Some(card) = resolve_hero(&snapshot.cards, &hero, &locale) else {
                return Ok(Vec::new());
            };
            proof.card(card)?;
            // A compact generated view, not a separately maintained stats store.
            // Retain conditions/Unknown and distinguish descriptive from executable rules.
            answer.text = serde_json::to_string(&serde_json::json!({
                "hero_id": card.card.hero_id, "facts": card.card.facts,
                "descriptive_rules": card.card.rules, "effects": card.effects,
                "synergies": card.card.synergies, "fields": card.fields, "unknowns": card.unknowns
            }))
            .map_err(|_| invalid("card projection encoding"))?;
        }
    }
    answer.inputs = proof.inputs.clone();
    if answer.inputs.is_empty() || answer.inputs.len() > 512 {
        return Err(invalid("domain proof size"));
    }
    let primary = proof
        .primary
        .as_ref()
        .ok_or_else(|| invalid("domain proof missing primary object"))?;
    let content = serde_json::to_string(&answer).map_err(|_| invalid("domain answer encoding"))?;
    if content.len() > 256 * 1024 {
        return Err(PortError::BudgetExceeded);
    }
    let identity = serde_json::to_vec(&(
        &query.text,
        &query.domain,
        &context.knowledge_release,
        &content,
    ))
    .map_err(|_| invalid("domain identity encoding"))?;
    let id = format!("domain-{:x}", Sha256::digest(identity));
    Ok(vec![Evidence {
        evidence_id: id.clone(),
        source_id: primary.source_id.clone(),
        logical_id: primary.logical_id.clone(),
        revision: primary.revision,
        kind: match answer.route {
            DomainRoute::Fact => EvidenceKind::Fact,
            DomainRoute::Card => EvidenceKind::Mechanic,
            _ => EvidenceKind::Rule,
        },
        content,
        citation: format!("brain:{id}@{}", primary.revision),
        visibility: proof.visibility,
        allowed_scopes: proof.scopes,
        score: 100.0,
        provenance: None,
        patch: Some(snapshot.validity.patch),
    }])
}
struct Proof<'a> {
    snapshot: &'a DomainSnapshot,
    inputs: Vec<LocatedRevision>,
    primary: Option<DocumentRevision>,
    visibility: SourceVisibility,
    scopes: BTreeSet<String>,
}
impl<'a> Proof<'a> {
    fn new(snapshot: &'a DomainSnapshot) -> Self {
        Self {
            snapshot,
            inputs: Vec::new(),
            primary: None,
            visibility: SourceVisibility::Public,
            scopes: BTreeSet::new(),
        }
    }
    fn record(&self, source: &DocumentRevision) -> Result<&'a SourceRecordV2, PortError> {
        self.snapshot
            .records
            .iter()
            .find(|r| {
                r.source_id == source.source_id
                    && r.logical_id == source.logical_id
                    && r.revision == source.revision
                    && r.content_hash == source.content_hash
            })
            .ok_or_else(|| invalid("domain proof source not authorized"))
    }
    fn located(&mut self, located: &LocatedRevision) -> Result<(), PortError> {
        let record = self.record(&located.source)?;
        self.scopes.extend(record.allowed_scopes.iter().cloned());
        self.visibility = match (self.visibility, record.visibility) {
            (SourceVisibility::Private, _) | (_, SourceVisibility::Private) => {
                SourceVisibility::Private
            }
            (SourceVisibility::Internal, _) | (_, SourceVisibility::Internal) => {
                SourceVisibility::Internal
            }
            _ => SourceVisibility::Public,
        };
        if !self.inputs.contains(located) {
            self.inputs.push(located.clone());
        }
        Ok(())
    }
    fn document(&mut self, source: &DocumentRevision) -> Result<(), PortError> {
        // v1 NumericFact/TypedRule have a document-level locator. The parser here
        // identifies this projection, not an invented original source parser.
        self.located(&LocatedRevision {
            source: source.clone(),
            locator: "document".into(),
            parser_revision: DOMAIN_CONTRACT_VERSION.into(),
        })
    }
    fn object(&mut self, key: &str) -> Result<(), PortError> {
        let source = self
            .snapshot
            .object_sources
            .get(key)
            .ok_or_else(|| invalid("domain object record missing"))?;
        if self.primary.is_none() {
            self.primary = Some(source.clone());
        }
        self.document(source)
    }
    fn card(&mut self, card: &DomainKnowledgeCard) -> Result<(), PortError> {
        self.object(&format!("card:{}", card.card.hero_id))?;
        for location in std::iter::once(&card.source)
            .chain(&card.dependencies)
            .chain(card.aliases.iter().map(|a| &a.provenance))
        {
            self.located(location)?;
        }
        for fact in &card.card.facts {
            self.document(&fact.source_revision)?;
        }
        for rule in &card.card.rules {
            self.document(&rule.source_revision)?;
        }
        for effect in &card.effects {
            self.document(&effect.source_revision)?;
        }
        for field in &card.fields {
            self.document(&field.source_revision)?;
            for locator in &field.locators {
                self.located(&LocatedRevision {
                    source: field.source_revision.clone(),
                    locator: locator.locator.clone(),
                    parser_revision: card.source.parser_revision.clone(),
                })?;
            }
        }
        for synergy in &card.card.synergies {
            for evidence in &synergy.evidence {
                let source = self
                    .snapshot
                    .records
                    .iter()
                    .find(|r| {
                        r.source_id == evidence.source_id
                            && r.logical_id == evidence.logical_id
                            && r.revision == evidence.revision
                    })
                    .ok_or_else(|| invalid("synergy source not authorized"))?;
                self.document(&DocumentRevision {
                    source_id: source.source_id.clone(),
                    logical_id: source.logical_id.clone(),
                    revision: source.revision,
                    content_hash: source.content_hash.clone(),
                })?;
            }
        }
        Ok(())
    }
}
