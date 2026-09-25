use brain_contracts::{CorpusRelease, HeroKnowledgeCard};
use dbrain_s12_wiki_probe::{analyze, compare, knowledge::*, literal, sha256};
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../fixtures/pilot.capture.json")).unwrap()
}
fn raw_page(v: &mut Value, id: i64) -> &mut Value {
    &mut v["pages"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["page_id"] == id)
        .unwrap()["response"]["query"]["pages"][0]
}
fn latest(v: &mut Value, id: i64) -> &mut Value {
    let page = raw_page(v, id);
    let head = page["lastrevid"].clone();
    page["revisions"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["revid"] == head)
        .unwrap()
}
fn content(v: &mut Value, id: i64, value: Value) {
    latest(v, id)["slots"]["main"]["content"] = json!(value.to_string());
}
fn mapping() -> MappingProfile {
    serde_json::from_str(include_str!("../fixtures/completion.mapping.json")).unwrap()
}
fn ir(v: &Value) -> WikiIr {
    extract(&serde_json::to_vec(v).unwrap(), &mapping()).unwrap()
}
fn release(ir: &WikiIr) -> CorpusRelease {
    CorpusRelease {
        release_id: "fixture-release-one".into(),
        knowledge_version: "fixture-knowledge-v1".into(),
        patch: "unknown".into(),
        created_at_epoch: ir.report().retrieved_at,
        source_revisions: BTreeMap::from([(
            ir.report().source_key.clone(),
            ir.report()
                .pages
                .iter()
                .filter_map(|p| Some((p.source_id.clone(), p.latest_revision? as u64)))
                .collect(),
        )]),
    }
}
fn reviewed(ir: &WikiIr) -> ProjectionReview {
    ProjectionReview {
        decision_ref: "synthetic-test-review-not-production".into(),
        approved_fields: ir
            .fields()
            .iter()
            .map(|f| (f.id.clone(), f.source_revision.clone()))
            .collect(),
        source_revisions: ir
            .report()
            .pages
            .iter()
            .filter_map(|p| {
                let r = p.latest()?;
                Some((
                    p.source_id.clone(),
                    brain_contracts::DocumentRevision {
                        logical_id: p.source_id.clone(),
                        source_id: ir.report().source_key.clone(),
                        revision: r.revision_id as u64,
                        content_hash: r.raw_sha256.clone()?,
                    },
                ))
            })
            .collect(),
    }
}
fn card(ir: &WikiIr) -> CardProjection {
    project_card(ir, 101, "de", &release(ir), &reviewed(ir)).unwrap()
}

#[test]
fn exact_decimals_preserve_units_without_rounding_or_float_coercion() {
    for (raw, expected) in [
        ("1000.0", "1000"),
        ("42.125", "42.125"),
        ("+000.0100", "0.01"),
        ("-0.0", "0"),
        ("-12.500", "-12.5"),
    ] {
        assert_eq!(decimal(raw).as_deref(), Some(expected));
    }
    for raw in [
        "NaN",
        "1e3",
        "1,5",
        "25%",
        "1.0000000001",
        ".5",
        "1.",
        "",
        "--1",
    ] {
        assert!(decimal(raw).is_none(), "{raw}");
    }
    let x = ir(&fixture());
    let units: Vec<_> = x
        .fields()
        .iter()
        .filter_map(|f| {
            if let IrValue::Quantity { unit, .. } = f.value {
                Some(unit)
            } else {
                None
            }
        })
        .collect();
    for unit in [
        Unit::Health,
        Unit::Percent,
        Unit::PercentagePoint,
        Unit::Multiplier,
    ] {
        assert!(units.contains(&unit));
    }
}
#[test]
fn shared_source_contract_keeps_both_revisions_and_does_not_invent_game_validity() {
    let x = ir(&fixture());
    let sources: Vec<_> = x
        .sources()
        .iter()
        .filter(|s| s.logical_id.ends_with(":101"))
        .collect();
    assert_eq!(sources.len(), 2);
    for s in sources {
        s.validate().unwrap();
        assert_eq!(s.content_hash, sha256(s.content.as_bytes()));
        assert!(s.valid_from.is_none());
        assert!(s.valid_to.is_none());
        assert_eq!(s.metadata["patch"], "null");
        assert!(s.metadata.contains_key("policy"));
    }
}
#[test]
fn conditions_and_variants_survive_ir_but_cannot_be_flattened_into_legacy_facts() {
    let mut input = fixture();
    let mut payload: Value = serde_json::from_str(
        latest(&mut input, 201)["slots"]["main"]["content"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    payload["unit"] = json!("damage");
    content(&mut input, 201, payload);
    let x = ir(&input);
    let pulse = x.fields().iter().find(|f| f.predicate == "damage").unwrap();
    assert_eq!(pulse.condition.as_deref(), Some("while_airborne"));
    assert_eq!(pulse.variant.as_deref(), Some("charged"));
    let projected = card(&x);
    assert!(!projected.card.facts.iter().any(|f| f.key == "damage"));
    assert!(projected
        .unknowns
        .iter()
        .any(|s| s.contains("context_not_representable")));
}
#[test]
fn missing_null_and_false_remain_distinct_and_unknown_units_do_not_become_zero() {
    let mut v = fixture();
    content(
        &mut v,
        101,
        json!({"flag":false,"nullable":null,"amount":"25","unit":"furlongs"}),
    );
    let x = ir(&v);
    let value = |p| &x.fields().iter().find(|f| f.predicate == p).unwrap().value;
    assert_eq!(value("flag"), &IrValue::Boolean { value: false });
    assert!(matches!(value("nullable"),IrValue::Unknown{reason} if reason=="explicit_null"));
    assert!(matches!(value("missing"),IrValue::Unknown{reason} if reason=="missing_field"));
    assert!(
        matches!(value("amount"),IrValue::Unknown{reason} if reason=="missing_or_unknown_unit")
    );
}
#[test]
fn a_canonical_card_roundtrips_existing_contract_and_every_fact_is_grounded() {
    let x = ir(&fixture());
    let projected = card(&x);
    assert!(!projected.card.facts.is_empty());
    assert!(!projected.published);
    let wire = serde_json::to_vec(&projected.card).unwrap();
    let decoded: HeroKnowledgeCard = serde_json::from_slice(&wire).unwrap();
    assert_eq!(decoded, projected.card);
    for f in &projected.card.facts {
        let locators = &projected.locators[&f.fact_id];
        assert!(!locators.is_empty());
        for l in locators {
            let page = x
                .report()
                .pages
                .iter()
                .find(|p| p.page_id == l.page_id)
                .unwrap();
            let r = page
                .revisions
                .iter()
                .find(|r| r.revision_id == l.revision_id)
                .unwrap();
            assert_eq!(r.raw_sha256.as_deref(), Some(l.content_hash.as_str()));
            assert!(r.candidates.iter().any(|c| c.locator == l.locator));
        }
        assert!(x
            .sources()
            .iter()
            .any(|s| s.logical_id == f.source_revision.logical_id
                && s.revision == f.source_revision.revision
                && s.content_hash == f.source_revision.content_hash));
    }
}
#[test]
fn rebuild_hash_is_independent_of_mapping_order_and_retrieval_time() {
    let v = fixture();
    let a = ir(&v);
    let first = card(&a);
    let mut profile = mapping();
    profile.fields.reverse();
    let mut later = v.clone();
    later["retrieved_at"] = json!(v["retrieved_at"].as_i64().unwrap() + 5);
    let b = extract(&serde_json::to_vec(&later).unwrap(), &profile).unwrap();
    assert_eq!(
        serde_json::to_value(first).unwrap(),
        serde_json::to_value(card(&b)).unwrap()
    );
}
#[test]
fn alias_locale_is_source_locale_not_the_requested_card_locale() {
    let x = ir(&fixture());
    assert_eq!(
        x.resolve_alias("old_fixture_ranger", "en"),
        Some("fixture:hero:one")
    );
    assert_eq!(x.resolve_alias("old_fixture_ranger", "de"), None);
    assert_eq!(x.resolve_alias("random hero", "en"), None);
}
#[test]
fn newer_revision_needs_new_review_even_when_field_id_is_stable() {
    let v = fixture();
    let current = ir(&v);
    let mut previous = v;
    let p = raw_page(&mut previous, 101);
    p["lastrevid"] = json!(9101);
    p["revisions"]
        .as_array_mut()
        .unwrap()
        .retain(|r| r["revid"] == 9101);
    let old = ir(&previous);
    let old_card = card(&old);
    let new_card = card(&current);
    assert_ne!(old_card.rebuild_sha256, new_card.rebuild_sha256);
    let mut stale_review = reviewed(&old);
    stale_review.source_revisions = reviewed(&current).source_revisions;
    let unreviewed = project_card(&current, 101, "de", &release(&current), &stale_review).unwrap();
    assert!(!unreviewed.card.facts.iter().any(|f| f.key == "health"));
    assert!(unreviewed
        .unknowns
        .iter()
        .any(|s| s.contains("unreviewed_revision")));
}
#[test]
fn stale_release_and_unknown_hero_or_locale_are_rejected() {
    let x = ir(&fixture());
    let mut r = release(&x);
    let logical = x
        .report()
        .pages
        .iter()
        .find(|p| p.page_id == 101)
        .unwrap()
        .source_id
        .clone();
    r.source_revisions
        .get_mut(&x.report().source_key)
        .unwrap()
        .insert(logical, 9101);
    assert!(project_card(&x, 101, "de", &r, &reviewed(&x)).is_err());
    assert!(project_card(&x, 999, "de", &release(&x), &reviewed(&x)).is_err());
    assert!(project_card(&x, 101, "zz", &release(&x), &reviewed(&x)).is_err());
}
#[test]
fn current_acl_denial_erases_raw_ir_and_canonical_facts() {
    let mut v = fixture();
    v["pages"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["page_id"] == 101)
        .unwrap()["access_allowed"] = json!(false);
    let x = ir(&v);
    assert!(!x.sources().iter().any(|s| s.logical_id.ends_with(":101")));
    assert!(!card(&x)
        .card
        .facts
        .iter()
        .any(|f| f.subject_id == "fixture:hero:one"));
}
#[test]
fn dynamic_dependencies_quarantine_dependent_facts_not_just_the_bad_page() {
    let mut v = fixture();
    let r = latest(&mut v, 701);
    r["slots"]["main"]["contentmodel"] = json!("Scribunto");
    r["slots"]["main"]["content"] = json!("return dangerous()");
    let x = ir(&v);
    let card = card(&x);
    assert!(!card.card.facts.iter().any(|f| f.key == "health"));
}
#[test]
fn global_policy_revocation_invalidates_cards_without_reembedding_raw_bytes() {
    let a = fixture();
    let mut b = a.clone();
    b["policy"]["publication_allowed"] = json!(true);
    let before = analyze(&serde_json::to_vec(&b).unwrap()).unwrap();
    let after = analyze(&serde_json::to_vec(&a).unwrap()).unwrap();
    let delta = compare(&before, &after).unwrap();
    assert!(!delta.reproject_heroes.is_empty());
    assert!(delta.raw_content_changed.is_empty());
    assert_eq!(delta.embedding_jobs_scheduled, 0);
}
#[test]
fn static_lua_data_has_exact_utf8_locators_and_no_executable_grammar() {
    let text = include_str!("../fixtures/literal-data.lua");
    let values = literal::lua(text).unwrap();
    assert!(values.iter().any(|c| c.value == json!(false)));
    assert!(values.iter().any(|c| c.value.is_null()));
    for c in values {
        let span = c.locator.split_once("@utf8:").unwrap().1;
        let (a, b) = span.split_once("..").unwrap();
        let raw = &text[a.parse::<usize>().unwrap()..b.parse::<usize>().unwrap()];
        assert!(!raw.is_empty());
        if c.value == json!("42.125") {
            assert_eq!(raw, "\"42.125\"");
        }
    }
    for bad in [
        "return require('x')",
        "return { x = math.huge }",
        "return { x = 1 + 2 }",
        "return { x = function() end }",
        "return { x=1,x=2 }",
        "return { [dangerous()] = 1 }",
        "return { 'a', x=2 }",
        "return {nil}",
        "return {} dangerous()",
        "return {x=1/0}",
    ] {
        assert!(literal::lua(bad).is_err(), "{bad}");
    }
}
#[test]
fn flat_templates_are_data_parameters_not_executed_programs() {
    let text = include_str!("../fixtures/literal-template.wiki");
    let values = literal::template(text).unwrap();
    assert_eq!(values.len(), 3);
    for c in values {
        let span = c.locator.split_once("@utf8:").unwrap().1;
        let (a, b) = span.split_once("..").unwrap();
        assert_eq!(
            c.value.as_str().unwrap(),
            &text[a.parse::<usize>().unwrap()..b.parse::<usize>().unwrap()]
        );
    }
    for bad in [
        "{{#invoke:foo|bar}}",
        "{{#if:x|1|2}}",
        "{{Data|x={{Nested}}}}",
        "{{Data|x=1|x=2}}",
        "before {{Data|x=2}}",
        "{{Data|x={{{a}}}}}",
    ] {
        assert!(literal::template(bad).is_err(), "{bad}");
    }
}
#[test]
fn safe_literal_formats_feed_the_same_ir_and_shared_card_adapter() {
    for (model, text, value_path, unit_path) in [
        (
            "Scribunto",
            "return { amount=\"42.125\", unit=\"damage\" }",
            "lua:/amount",
            "lua:/unit",
        ),
        (
            "wikitext",
            "{{Data|amount=42.125|unit=damage}}",
            "template:Data/amount",
            "template:Data/unit",
        ),
    ] {
        let mut v = fixture();
        let r = latest(&mut v, 101);
        r["slots"]["main"]["contentmodel"] = json!(model);
        r["slots"]["main"]["content"] = json!(text);
        let mut p = mapping();
        p.fields = vec![FieldMapping {
            page_id: 101,
            predicate: "test_damage".into(),
            value_locator: value_path.into(),
            kind: ValueKind::Quantity,
            unit_locator: Some(unit_path.into()),
            condition_locator: None,
            variant_locator: None,
        }];
        let x = extract(&serde_json::to_vec(&v).unwrap(), &p).unwrap();
        assert_eq!(
            x.fields()[0].value,
            IrValue::Quantity {
                decimal: "42.125".into(),
                unit: Unit::Damage
            }
        );
        assert_eq!(card(&x).card.facts[0].value, json!("42.125"));
    }
}
#[test]
fn unreviewed_profiles_and_duplicate_fields_are_rejected() {
    let v = fixture();
    let mut p = mapping();
    p.review_ref.clear();
    assert!(extract(&serde_json::to_vec(&v).unwrap(), &p).is_err());
    let mut p = mapping();
    p.fields.push(p.fields[0].clone());
    assert!(extract(&serde_json::to_vec(&v).unwrap(), &p).is_err());
}
#[test]
fn shared_contract_card_matches_the_reviewed_synthetic_golden() {
    let projected = card(&ir(&fixture()));
    let expected: Value =
        serde_json::from_str(include_str!("../fixtures/completion-card.golden.json")).unwrap();
    assert_eq!(serde_json::to_value(projected).unwrap(), expected);
}
