//! Synthetic adapter fixture only. These are NOT Deadlock gameplay stats.
//! The pilot and regression suite share this producer; no production catalog.
#![allow(dead_code)]
use brain_contracts::{
    domain::*,
    wiki::{IrField, IrValue, SourceLocator, Unit},
    *,
};
use dbrain_reasoner::ItemModel;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const SOURCE: &str = "c6-synthetic-domain";
pub fn revision(record: &SourceRecordV2) -> DocumentRevision {
    DocumentRevision {
        source_id: record.source_id.clone(),
        logical_id: record.logical_id.clone(),
        revision: record.revision,
        content_hash: record.content_hash.clone(),
    }
}
pub fn located(record: &SourceRecordV2, pointer: &str) -> LocatedRevision {
    LocatedRevision {
        source: revision(record),
        locator: format!("json:{pointer}"),
        parser_revision: "c6-synthetic-fixture-v1".into(),
    }
}
pub fn record(id: &str, rev: u64, value: &impl serde::Serialize, patch: &str) -> SourceRecordV2 {
    let content = serde_json::to_string(value).unwrap();
    SourceRecordV2 {
        source_id: SOURCE.into(),
        logical_id: id.into(),
        revision: rev,
        content_hash: format!("{:x}", Sha256::digest(content.as_bytes())),
        content,
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::from([
            ("patch".into(), patch.into()),
            ("mode".into(), "ranked".into()),
            ("kind".into(), "domain_input".into()),
        ]),
    }
}
pub fn typed(id: &str, rev: u64, object: DomainObject, patch: &str) -> SourceRecordV2 {
    let mut r = record(
        id,
        rev,
        &StoredDomainObject {
            contract_version: DOMAIN_CONTRACT_VERSION.into(),
            object,
        },
        patch,
    );
    r.metadata
        .insert("domain_contract".into(), DOMAIN_CONTRACT_VERSION.into());
    r
}
pub fn item(id: i64, active: bool) -> ItemModel {
    serde_json::from_value(json!({
        "item_id":id,"name":format!("Fixture Item {id}"),"class_name":format!("fixture_item_{id}"),"component_items":[],
        "slot":"Weapon","tier":1,"cost":800,"is_active":active,"shopable":true,"disabled":false,
        "damage_axis":"Weapon","defense_kind":[],"properties":{},"passive_properties":{},"condition":"None","proc_cooldown":null,"imbueable":false
    })).unwrap()
}
pub fn records(
    release: &str,
    patch: &str,
    hero_revision: u64,
    health: &str,
) -> Vec<SourceRecordV2> {
    let validity = Validity {
        patch: patch.into(),
        mode: "ranked".into(),
    };
    let hero = record(
        "hero",
        hero_revision,
        &json!({"name":"Fixture Hero", "health":health,"mystery":null}),
        patch,
    );
    let mut models = vec![item(101, false), item(102, false), item(103, false)];
    models[1].cost = 1600;
    models[1].component_items = vec!["fixture_item_101".into()];
    models.extend((201..=205).map(|id| item(id, true)));
    let models = record("models", 1, &models, patch);
    let inventory = record(
        "inventory",
        1,
        &json!({"max_slots":2,"upgrade_components":{"102":[101]},"resale_fraction":0.5}),
        patch,
    );
    let mechanics = record(
        "mechanics",
        1,
        &json!({"active_limit":1,"damage":40,"interval":5,"effect":"only after a hit","synergy":"synthetic provenance test"}),
        patch,
    );
    let rule_source = record(
        "rule-source",
        1,
        &json!({"rule":"damage divided by interval", "version":"fixture-1"}),
        patch,
    );
    let hero_source = revision(&hero);
    let field = |predicate: &str, value: IrValue| IrField {
        id: format!("fixture-{predicate}"),
        subject_id: "hero:fixture".into(),
        predicate: predicate.into(),
        value,
        condition: None,
        variant: None,
        source_revision: hero_source.clone(),
        locators: vec![SourceLocator {
            page_id: 1,
            revision_id: hero_revision as i64,
            content_hash: hero.content_hash.clone(),
            locator: format!("json:/{predicate}"),
        }],
        unknowns: BTreeSet::new(),
    };
    let synergy = SynergyEvidence {
        synergy_id: "fixture-synergy".into(),
        left_id: "hero:fixture".into(),
        right_id: "item:101".into(),
        evidence: vec![Evidence {
            evidence_id: "fixture-mechanic-evidence".into(),
            source_id: SOURCE.into(),
            logical_id: mechanics.logical_id.clone(),
            revision: mechanics.revision,
            kind: EvidenceKind::Mechanic,
            content: mechanics.content.clone(),
            citation: "fixture:mechanic".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            score: 1.0,
            patch: Some(patch.into()),
        }],
    };
    let card = DomainKnowledgeCard {
        card: HeroKnowledgeCard {
            hero_id: "hero:fixture".into(),
            knowledge_release: release.into(),
            facts: vec![Fact {
                fact_id: format!("health-r{hero_revision}"),
                subject_id: "hero:fixture".into(),
                key: "health".into(),
                value: json!(health),
                unit: Some("health".into()),
                source_revision: hero_source.clone(),
            }],
            rules: vec![Rule {
                rule_id: "descriptive-hit".into(),
                subject_id: "hero:fixture".into(),
                expression: "Effect requires a hit; not executable source.".into(),
                source_revision: revision(&mechanics),
            }],
            synergies: vec![synergy],
        },
        validity: validity.clone(),
        source: located(&hero, ""),
        dependencies: vec![located(&mechanics, "")],
        aliases: [("Fixture Hero", "en"), ("Prüfheld", "de"), ("Straße", "de")]
            .into_iter()
            .map(|(text, locale)| LocalizedAlias {
                text: text.into(),
                locale: locale.into(),
                provenance: located(&hero, "/name"),
            })
            .collect(),
        fields: vec![
            field(
                "health",
                IrValue::Quantity {
                    decimal: health.into(),
                    unit: Unit::Health,
                },
            ),
            field(
                "mystery",
                IrValue::Unknown {
                    reason: "explicit_null".into(),
                },
            ),
        ],
        effects: vec![Effect {
            effect_id: "fixture-effect".into(),
            subject_id: "hero:fixture".into(),
            expression: "condition=hit; effect unknown outside that condition".into(),
            source_revision: revision(&mechanics),
        }],
        unknowns: BTreeSet::from(["mystery".into()]),
        review_ref: "synthetic-test-review-only".into(),
    };
    let catalog = BuildCatalogRef {
        catalog_id: "fixture-catalog".into(),
        validity: validity.clone(),
        models: located(&models, ""),
        inventory_rules: located(&inventory, ""),
        dependencies: Vec::new(),
        unknown_legality_fields: BTreeSet::new(),
        active_limit_fact_id: "active-limit".into(),
        aliases: (101..=103)
            .map(|id| ItemAlias {
                item_id: id,
                alias: LocalizedAlias {
                    text: format!("Prüfobjekt {id}"),
                    locale: "de".into(),
                    provenance: located(&models, ""),
                },
            })
            .collect(),
        review_ref: "synthetic-model-review-only".into(),
    };
    let facts = [
        ("active-limit", "active_limit", 1.0, "1"),
        ("damage", "damage", 40.0, "damage"),
        ("interval", "interval", 5.0, "s"),
    ];
    let mut result = vec![
        hero,
        models,
        inventory,
        mechanics.clone(),
        rule_source.clone(),
        typed(
            "card",
            hero_revision,
            DomainObject::HeroCard(Box::new(card)),
            patch,
        ),
        typed(
            "catalog",
            1,
            DomainObject::BuildCatalog(Box::new(catalog)),
            patch,
        ),
    ];
    for (id, predicate, value, unit) in facts {
        result.push(typed(
            &format!("fact-{id}"),
            1,
            DomainObject::NumericFact(NumericFact {
                fact_id: id.into(),
                subject_id: "hero:fixture".into(),
                predicate: predicate.into(),
                quantity: Quantity {
                    value,
                    unit: unit.into(),
                },
                validity: validity.clone(),
                source: revision(&mechanics),
                verified: true,
            }),
            patch,
        ));
    }
    result.push(typed(
        "rule",
        1,
        DomainObject::Rule(TypedRule {
            rule_id: "fixture-dps".into(),
            rule_version: "fixture-1".into(),
            validity,
            source: revision(&rule_source),
            verified: true,
            expression: RuleExpr::Divide {
                left: Box::new(RuleExpr::Fact {
                    fact_id: "damage".into(),
                }),
                right: Box::new(RuleExpr::Fact {
                    fact_id: "interval".into(),
                }),
            },
        }),
        patch,
    ));
    result
}
pub fn build(hero: &str, locale: &str, items: &[&str]) -> DomainRequest {
    DomainRequest::Build {
        hero: hero.into(),
        locale: locale.into(),
        catalog_id: "fixture-catalog".into(),
        items: items.iter().map(|s| (*s).into()).collect(),
    }
}
pub fn query(request: &DomainRequest, patch: &str) -> Query {
    Query {
        domain: Some(request.clone()),
        request_id: "c6-request".into(),
        conversation_id: "c6-conversation".into(),
        text: "Prüfe die typisierte Domainanfrage.".into(),
        requested_scopes: BTreeSet::new(),
        profile: match request {
            DomainRequest::Build { .. } | DomainRequest::Rule { .. } => AnswerProfile::Build,
            DomainRequest::Fact { .. } => AnswerProfile::Fact,
            DomainRequest::Card { .. } => AnswerProfile::Explain,
        },
        patch: Some(patch.into()),
        mode: Some("ranked".into()),
    }
}
