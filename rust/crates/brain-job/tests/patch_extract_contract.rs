use brain_job::{
    extract::{self, Assertion, Atom},
    patch_extract::{self, AliasBinding, AliasIndex, KnownEntity, StatBinding},
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn envelope(content: Option<&str>) -> Vec<u8> {
    serde_json::to_vec(&json!({"id":42,"title":"Update","url":"https://store.steampowered.com/news/app/1422450/view/703281025618282632","posted_at":"2026-10-05T18:00:00+00:00","raw_content":content})).unwrap()
}

fn alias(name: &str, key: &str, kind: &'static str, scope: Option<&str>) -> AliasBinding {
    AliasBinding {
        alias: name.into(),
        entity: KnownEntity {
            key: key.into(),
            kind,
        },
        scope: scope.map(str::to_owned),
    }
}

fn catalog() -> AliasIndex {
    AliasIndex::new(
        vec![
            alias("Baba", "hero_baba", "hero", None),
            alias("Sinclair", "hero_sinclair", "hero", None),
            alias(
                "Bubbling Brew",
                "ability_baba_brew",
                "ability",
                Some("hero_baba"),
            ),
        ],
        vec![
            StatBinding {
                entity: "hero_baba".into(),
                label: "Max Health".into(),
                stat: "stats.max_health".into(),
            },
            StatBinding {
                entity: "ability_baba_brew".into(),
                label: "Cooldown".into(),
                stat: "ability.AbilityCooldown".into(),
            },
        ],
    )
    .unwrap()
}

fn assertions(bytes: &[u8], aliases: &AliasIndex) -> Vec<Assertion> {
    let post = patch_extract::parse_post(bytes).unwrap();
    let assertions = patch_extract::extract(&post, aliases)
        .unwrap()
        .changes
        .into_iter()
        .map(|change| change.assertion)
        .collect::<Vec<_>>();
    patch_extract::prove(&post, aliases, &assertions).unwrap();
    extract::prove(patch_extract::SOURCE, "42", bytes, &assertions).unwrap();
    assertions
}

#[test]
fn scoped_statements_and_complete_conditions_prove_original_bytes() {
    let bytes = envelope(Some("Heroes\nBaba\n- Max Health increased from 780.0 to 800.0\nBubbling Brew\n- Cooldown reduced from 26s to 24s\n- Now falls faster while charging\n  only against objectives\nGeneral\n- Added a feature"));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let result = patch_extract::extract(&post, &catalog()).unwrap();
    assert_eq!(result.changes.len(), 3);
    assert_eq!(result.changes[0].old_value.as_deref(), Some("780.0"));
    assert_eq!(result.changes[0].new_value.as_deref(), Some("800.0"));
    assert_eq!(result.changes[0].stat.as_deref(), Some("stats.max_health"));
    assert_eq!(result.changes[1].assertion.entity, "ability_baba_brew");
    assert_eq!(
        result.changes[1].stat.as_deref(),
        Some("ability.AbilityCooldown")
    );
    assert!(
        matches!(&result.changes[2].assertion.value, Atom::Text(value) if value == "- Now falls faster while charging\n  only against objectives")
    );
    let assertions = assertions(&bytes, &catalog());
    for receipt in &assertions {
        assert_eq!(receipt.locator_kind, "json_pointer");
        assert_eq!(receipt.locator, "/raw_content");
        assert_eq!(
            receipt.hash,
            Sha256::digest(post.lexical_value("/raw_content").unwrap().as_bytes()).to_vec()
        );
    }
}

#[test]
fn explicit_inline_names_preserve_qualifiers_without_stat_guessing() {
    let statement =
        "- Sinclair: Vexing Bolt - Now does half damage to objectives - only while charging";
    let bytes = envelope(Some(statement));
    let result = assertions(&bytes, &catalog());
    assert_eq!(result.len(), 1);
    assert!(matches!(&result[0].value, Atom::Text(value) if value == statement));
    assert!(result[0].condition["old_value"].is_null());
    assert!(result[0].condition["new_value"].is_null());
    assert!(result[0].condition["stat"].is_null());
}

#[test]
fn exact_supplied_stat_mapping_is_optional_and_ambiguity_is_null() {
    let bytes = envelope(Some(
        "- Baba: Max Health increased from 780.0 to 800.0\n- Baba: Damage increased from 1 to 2",
    ));
    let facts = assertions(&bytes, &catalog());
    assert_eq!(facts.len(), 2);
    assert_eq!(facts[0].condition["stat"], "stats.max_health");
    assert!(facts[1].condition["stat"].is_null());
    let aliases = AliasIndex::new(
        vec![alias("Baba", "hero_baba", "hero", None)],
        vec![
            StatBinding {
                entity: "hero_baba".into(),
                label: "Max Health".into(),
                stat: "stats.a".into(),
            },
            StatBinding {
                entity: "hero_baba".into(),
                label: "Max Health".into(),
                stat: "stats.b".into(),
            },
        ],
    )
    .unwrap();
    let facts = assertions(&bytes, &aliases);
    assert!(facts[0].condition["stat"].is_null());
}

#[test]
fn ambiguous_unknown_and_unscoped_names_do_not_manufacture_entities() {
    let aliases = AliasIndex::new(
        vec![
            alias("Baba", "hero_baba", "hero", None),
            alias("Shared", "ability_a", "ability", Some("hero_baba")),
            alias("Shared", "ability_b", "ability", Some("hero_baba")),
        ],
        Vec::new(),
    )
    .unwrap();
    let bytes = envelope(Some("- Shared: Damage increased from 1 to 2\nBaba\n- Shared: Damage increased from 1 to 2\nUnknown\n- Damage increased from 1 to 2"));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let result = patch_extract::extract(&post, &aliases).unwrap();
    assert!(result.changes.is_empty());
    assert!(result
        .skipped
        .iter()
        .any(|entry| entry.reason == "unscoped_alias"));
    assert!(result
        .skipped
        .iter()
        .any(|entry| entry.reason == "ambiguous_alias"));
    assert!(result
        .skipped
        .iter()
        .any(|entry| entry.reason == "unknown_or_unscoped_entity"));
}

#[test]
fn origin_validation_rejects_lookalikes_other_apps_and_url_rewriting() {
    for url in [
        "http://forums.playdeadlock.com/threads/update.42/",
        "https://forums.playdeadlock.com.evil.test/threads/update.42/",
        "https://evil.test/?u=store.steampowered.com/news/app/1422450/view/42",
        "https://store.steampowered.com/news/app/570/view/42",
        "https://store.steampowered.com/news/app/1422450/view/42/../43",
        "https://user@forums.playdeadlock.com/threads/update.42/",
        "https://forums.playdeadlock.com:443/threads/update.42/",
        "https://forums.playdeadlock.com/threads/update.42/page-2",
        "https://forums.playdeadlock.com/threads/update.42/?redirect=evil",
        "https://forums.playdeadlock.com/threads/update.%34%32/",
    ] {
        let mut value: Value =
            serde_json::from_slice(&envelope(Some("- Baba: Damage increased from 1 to 2")))
                .unwrap();
        value["url"] = json!(url);
        let bytes = serde_json::to_vec(&value).unwrap();
        let post = patch_extract::parse_post(&bytes).unwrap();
        let result = patch_extract::extract(&post, &catalog()).unwrap();
        assert!(result.changes.is_empty(), "{url}");
        assert_eq!(result.skipped[0].reason, "untrusted_origin");
        assert_eq!(post.bytes(), bytes);
    }
}

#[test]
fn official_steam_and_forum_paths_are_accepted_exactly() {
    for url in [
        "https://forums.playdeadlock.com/threads/10-05-2026-update.42/",
        "https://steamcommunity.com/games/1422450/announcements/detail/42",
        "https://store.steampowered.com/news/app/1422450/view/42",
    ] {
        let mut value: Value =
            serde_json::from_slice(&envelope(Some("- Baba: Added knockback"))).unwrap();
        value["url"] = json!(url);
        let bytes = serde_json::to_vec(&value).unwrap();
        assert_eq!(assertions(&bytes, &catalog()).len(), 1);
    }
}

#[test]
fn explicit_date_is_required_and_titles_are_never_chronology() {
    let mut value: Value =
        serde_json::from_slice(&envelope(Some("- Baba: Added knockback"))).unwrap();
    value["title"] = json!("10-05-2026 Update");
    value["posted_at"] = Value::Null;
    let bytes = serde_json::to_vec(&value).unwrap();
    let post = patch_extract::parse_post(&bytes).unwrap();
    let result = patch_extract::extract(&post, &catalog()).unwrap();
    assert!(result.changes.is_empty());
    assert_eq!(result.skipped[0].reason, "missing_posted_at");
    value["posted_at"] = json!("2026-02-30T00:00:00+00:00");
    assert!(patch_extract::parse_post(&serde_json::to_vec(&value).unwrap()).is_err());
}

#[test]
fn duplicate_titles_keep_distinct_post_identities_and_correction_receipts() {
    let first = envelope(Some("- Baba: Added knockback"));
    let mut value: Value = serde_json::from_slice(&first).unwrap();
    value["id"] = json!(43);
    let second = serde_json::to_vec(&value).unwrap();
    let first_post = patch_extract::parse_post(&first).unwrap();
    let second_post = patch_extract::parse_post(&second).unwrap();
    assert_eq!(first_post.title(), second_post.title());
    assert_ne!(first_post.id(), second_post.id());
    let facts = assertions(&first, &catalog());
    assert!(patch_extract::prove(&second_post, &catalog(), &facts).is_err());
    let correction = envelope(Some("- Baba: Removed knockback"));
    let correction = patch_extract::parse_post(&correction).unwrap();
    assert!(patch_extract::prove(&correction, &catalog(), &facts).is_err());
}

#[test]
fn strict_five_field_envelope_preserves_lexical_strings_and_rejects_duplicates() {
    let bytes = br#"{ "id" : 42, "title" : "Update", "url" : "https://forums.playdeadlock.com/threads/update.42/", "posted_at" : "2026-10-05T18:00:00+00:00", "raw_content" : "- Baba: Added knockback" }"#;
    let post = patch_extract::parse_post(bytes).unwrap();
    assert_eq!(post.title(), "Update");
    assert_eq!(post.lexical_value("/title"), Some(r#""Update""#));
    let facts = assertions(bytes, &catalog());
    assert_ne!(
        facts[0].hash,
        Sha256::digest(post.raw_content().unwrap().as_bytes()).to_vec()
    );
    for invalid in [br#"{"id":42,"id":43,"title":"Update","url":"https://forums.playdeadlock.com/threads/update.42/","posted_at":null,"raw_content":null}"#.as_slice(),
        br#"{"id":42,"title":"Update","url":"x","posted_at":null,"raw_content":null,"translated_content":"x"}"#.as_slice(),
        br#"{"id":42,"title":"Update","url":"x","raw_content":null}"#.as_slice()] {
        assert!(patch_extract::parse_post(invalid).is_err());
    }
}

#[test]
fn receipt_and_entity_context_tampering_are_rejected() {
    let bytes = envelope(Some("Baba\n- Max Health increased from 780.0 to 800.0"));
    for mode in 0..6 {
        let mut facts = assertions(&bytes, &catalog());
        match mode {
            0 => facts[0].entity = "hero_sinclair".into(),
            1 => facts[0].value = Atom::Text("- Removed a limitation".into()),
            2 => facts[0].condition["context_indices"] = json!([1]),
            3 => facts[0].condition["old_value"] = json!("900"),
            4 => facts[0].hash[0] ^= 1,
            _ => facts[0].locator = "/title".into(),
        }
        let post = patch_extract::parse_post(&bytes).unwrap();
        assert!(patch_extract::prove(&post, &catalog(), &facts).is_err());
        assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &facts).is_err());
    }
}

#[test]
fn bounds_skip_oversized_statements_and_reject_unbounded_rows() {
    let bytes = envelope(Some(&format!(
        "Baba\n- Added {}",
        "x".repeat(patch_extract::MAX_STATEMENT_BYTES)
    )));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let result = patch_extract::extract(&post, &catalog()).unwrap();
    assert!(result.changes.is_empty());
    assert_eq!(result.skipped[0].reason, "statement_limit");
    assert!(patch_extract::parse_post(&vec![b' '; patch_extract::MAX_ENVELOPE_BYTES + 1]).is_err());
    let content = format!(
        "Baba\n{}",
        "- Added knockback\n".repeat(patch_extract::MAX_CHANGES + 1)
    );
    let bytes = envelope(Some(&content));
    let post = patch_extract::parse_post(&bytes).unwrap();
    assert!(patch_extract::extract(&post, &catalog()).is_err());
}

#[test]
fn missing_stubs_announcements_and_combined_entities_are_retained_not_refetched() {
    for content in [
        None,
        Some(""),
        Some("Coming soon"),
        Some("Baba\nA new event is now available"),
        Some("- Baba: Added knockback - Sinclair: Removed knockback"),
    ] {
        let bytes = envelope(content);
        let post = patch_extract::parse_post(&bytes).unwrap();
        let result = patch_extract::extract(&post, &catalog()).unwrap();
        assert!(result.changes.is_empty(), "{content:?}");
        assert!(!result.skipped.is_empty());
        assert_eq!(post.bytes(), bytes);
    }
}

#[test]
fn compound_entity_continuations_are_refused_by_both_proof_paths() {
    let control = envelope(Some("- Baba: Added knockback"));
    let mut receipt = assertions(&control, &catalog());
    assert_eq!(receipt.len(), 1);
    for statement in [
        "- Baba: Added knockback\n  Sinclair: Removed knockback",
        "- Baba: Added knockback\nSinclair: Removed knockback",
        "- Baba: Added knockback\r\n\tSinclair: Removed knockback",
        "- Baba: Added knockback\n  - Sinclair: Removed knockback",
        "- Baba: Added knockback; Sinclair: Removed knockback",
        "- Baba: Added knockback and Sinclair removed knockback",
        "- Baba: Added knockback & Sinclair removed knockback",
        "- Baba: Added knockback / Sinclair added knockback",
        "- Baba: Added knockback. Sinclair now takes less damage",
        "- Baba: Added knockback\n  Sinclair removed knockback",
        "- Baba: Added knockback\n  Unknown: Removed knockback",
        "- Baba: Added knockback\n  Unknown removed knockback",
        "- Baba: Added knockback\n  sinclair recoil 10%",
        "- Baba: Added knockback\n  unknown passive effect",
        "- Baba: Added knockback\n  - Unknown's passive effect",
    ] {
        let bytes = envelope(Some(statement));
        let post = patch_extract::parse_post(&bytes).unwrap();
        let result = patch_extract::extract(&post, &catalog()).unwrap();
        assert!(result.changes.is_empty(), "{statement}");
        assert_eq!(result.skipped.len(), 1, "{statement}");
        receipt[0].hash =
            Sha256::digest(post.lexical_value("/raw_content").unwrap().as_bytes()).to_vec();
        receipt[0].condition["post_hash"] = json!(hex::encode(Sha256::digest(&bytes)));
        receipt[0].value = Atom::Text(statement.into());
        assert!(
            patch_extract::prove(&post, &catalog(), &receipt).is_err(),
            "{statement}"
        );
        assert!(
            extract::prove(patch_extract::SOURCE, "42", &bytes, &receipt).is_err(),
            "{statement}"
        );
    }
}

#[test]
fn unindented_qualifiers_cannot_publish_a_truncated_numeric_change() {
    let change = "- Baba: Max Health increased from 780.0 to 800.0";
    let control = envelope(Some(change));
    let mut receipt = assertions(&control, &catalog());
    assert_eq!(receipt.len(), 1);
    assert_eq!(receipt[0].condition["old_value"], "780.0");
    assert_eq!(receipt[0].condition["new_value"], "800.0");
    for qualifier in [
        "when attacking objectives",
        "against airborne enemies",
        "while the ability is active",
        "unless the target is immune",
        "only against objectives",
    ] {
        for separator in ["\n", "\r\n", "\n\n"] {
            let statement = format!("{change}{separator}{qualifier}");
            let bytes = envelope(Some(&statement));
            let post = patch_extract::parse_post(&bytes).unwrap();
            let result = patch_extract::extract(&post, &catalog()).unwrap();
            assert!(result.changes.is_empty(), "{statement}");
            assert_eq!(result.skipped[0].reason, "ambiguous_continuation");
            receipt[0].hash =
                Sha256::digest(post.lexical_value("/raw_content").unwrap().as_bytes()).to_vec();
            receipt[0].condition["post_hash"] = json!(hex::encode(Sha256::digest(&bytes)));
            for value in [change, statement.as_str()] {
                receipt[0].value = Atom::Text(value.into());
                assert!(
                    patch_extract::prove(&post, &catalog(), &receipt).is_err(),
                    "{statement}"
                );
                assert!(
                    extract::prove(patch_extract::SOURCE, "42", &bytes, &receipt).is_err(),
                    "{statement}"
                );
            }
        }
    }
}

#[test]
fn continuation_bullets_with_action_words_cannot_release_a_truncated_change() {
    let change = "- Baba: Max Health increased from 780.0 to 800.0";
    let control = envelope(Some(change));
    for qualifier in [
        "when Damage increased from 1 to 2",
        "When Damage increased from 1 to 2",
        "WHEN DAMAGE increased from 1 to 2",
        "**When** Damage increased from 1 to 2",
        "[When] Damage increased from 1 to 2",
        "`When` Damage increased from 1 to 2",
        "Every Time Damage increased from 1 to 2",
        "Not Damage increased from 1 to 2",
        "When damage is increased",
        "while removed effects are active",
        "unless knockback is now applied",
    ] {
        for marker in ["", "- ", "* ", "• "] {
            let statement = format!("{change}\n{marker}{qualifier}");
            let bytes = envelope(Some(&statement));
            let post = patch_extract::parse_post(&bytes).unwrap();
            let result = patch_extract::extract(&post, &catalog()).unwrap();
            assert!(result.changes.is_empty(), "{statement}");
            assert!(!result.skipped.is_empty(), "{statement}");
            let mut receipt = assertions(&control, &catalog());
            receipt[0].hash =
                Sha256::digest(post.lexical_value("/raw_content").unwrap().as_bytes()).to_vec();
            receipt[0].condition["post_hash"] = json!(hex::encode(Sha256::digest(&bytes)));
            for value in [change, statement.as_str()] {
                receipt[0].value = Atom::Text(value.into());
                assert!(
                    patch_extract::prove(&post, &catalog(), &receipt).is_err(),
                    "{statement}"
                );
                assert!(
                    extract::prove(patch_extract::SOURCE, "42", &bytes, &receipt).is_err(),
                    "{statement}"
                );
            }
            receipt[0].value = Atom::Text(change.into());
            for boundary in [
                json!(1),
                json!({"position":1,"key":"hero_fabricated","kind":"hero","scope":null}),
            ] {
                receipt[0].condition["next_heading"] = boundary;
                assert!(patch_extract::prove(&post, &catalog(), &receipt).is_err());
                assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &receipt).is_err());
            }
        }
    }
}

#[test]
fn qualified_numeric_statements_keep_complete_text_without_unqualified_values() {
    let change = "- Baba: Max Health increased from 780.0 to 800.0";
    for suffix in [
        "\n  when attacking objectives",
        "\n\n  against airborne enemies",
        "\n  - while the ability is active",
        " - only against objectives",
        ", only against objectives",
        "; only against objectives",
    ] {
        let statement = format!("{change}{suffix}");
        let bytes = envelope(Some(&statement));
        let post = patch_extract::parse_post(&bytes).unwrap();
        let mut facts = assertions(&bytes, &catalog());
        assert_eq!(facts.len(), 1, "{statement}");
        assert!(matches!(&facts[0].value, Atom::Text(value) if value == &statement));
        assert!(facts[0].condition["old_value"].is_null());
        assert!(facts[0].condition["new_value"].is_null());
        assert!(facts[0].condition["stat"].is_null());
        facts[0].condition["old_value"] = json!("780.0");
        facts[0].condition["new_value"] = json!("800.0");
        facts[0].condition["stat"] = json!("stats.max_health");
        facts[0].condition["stat_label"] = json!("Max Health");
        assert!(patch_extract::prove(&post, &catalog(), &facts).is_err());
        assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &facts).is_err());
    }
}

#[test]
fn ambiguous_continuations_clear_context_and_cannot_evade_statement_limits() {
    let bytes = envelope(Some(
        "Baba\n- Added knockback\nUnknown\n- Damage increased from 1 to 2",
    ));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let result = patch_extract::extract(&post, &catalog()).unwrap();
    assert!(result.changes.is_empty());
    assert_eq!(result.skipped[0].reason, "ambiguous_continuation");
    assert_eq!(result.skipped[1].reason, "unknown_or_unscoped_entity");
    let bytes = envelope(Some(&format!(
        "- Baba: Added knockback\n{}",
        "x".repeat(patch_extract::MAX_STATEMENT_BYTES)
    )));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let result = patch_extract::extract(&post, &catalog()).unwrap();
    assert!(result.changes.is_empty());
    assert_eq!(result.skipped[0].reason, "statement_limit");
}

#[test]
fn independent_statements_and_catalog_headings_remain_provable() {
    for content in [
        "Baba\n- Max Health increased from 780.0 to 800.0\nSinclair\n- Added knockback",
        "- Baba: Max Health increased from 780.0 to 800.0\n- Sinclair: Added knockback",
        "Baba\n  - Max Health increased from 780.0 to 800.0\nSinclair\n  - Added knockback",
    ] {
        let bytes = envelope(Some(content));
        let facts = assertions(&bytes, &catalog());
        assert_eq!(facts.len(), 2, "{content}");
        assert_eq!(facts[0].entity, "hero_baba");
        assert_eq!(facts[0].condition["old_value"], "780.0");
        assert_eq!(facts[1].entity, "hero_sinclair");
    }
}

#[test]
fn receipt_less_headings_preserve_source_positions_with_declared_catalog_bindings() {
    for content in [
        "Baba\n- Added knockback\nSinclair\n",
        "Baba\n- Added knockback\nSinclair\n- Recoil 10%",
        "Baba\n- Added knockback\n## **Sinclair**\n- Recoil 10%",
        "Baba\n- Added knockback\n- Sinclair\n- Recoil 10%",
        "Baba\n- Added knockback\nBubbling Brew\n- Recoil 10%",
        "Baba\r\n\r\n- Added knockback\r\n## **Sinclair**\r\n- Recoil 10%",
        "Sinclair\n- Recoil 10%\nBaba\n- Added knockback",
        "Unknown\nBaba\n- Added knockback",
    ] {
        let bytes = envelope(Some(content));
        let post = patch_extract::parse_post(&bytes).unwrap();
        let facts = assertions(&bytes, &catalog());
        assert_eq!(facts.len(), 1, "{content}");
        assert_eq!(facts[0].entity, "hero_baba");
        assert_eq!(post.bytes(), bytes);
        assert!(facts[0].condition.to_string().len() <= 1024);
    }
    let bytes = envelope(Some("Baba\n- Added knockback\nSinclair\n- Recoil 10%"));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let facts = assertions(&bytes, &catalog());
    assert_eq!(facts[0].order, 1);
    assert_eq!(facts[0].condition["statement_index"], 1);
    assert_eq!(facts[0].condition["context_indices"], json!([0]));
    assert_eq!(
        facts[0].condition["next_heading"],
        json!({"position":2,"key":"hero_sinclair","kind":"hero","scope":null})
    );
    for position in [0, 1, 3, 9999] {
        let mut forged = assertions(&bytes, &catalog());
        forged[0].condition["next_heading"]["position"] = json!(position);
        assert!(patch_extract::prove(&post, &catalog(), &forged).is_err());
        assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &forged).is_err());
    }
    let reduced = AliasIndex::new(
        vec![alias("Baba", "hero_baba", "hero", None)],
        Vec::new(),
    )
    .unwrap();
    assert!(patch_extract::prove(&post, &reduced, &facts).is_err());
}

#[test]
fn index_only_unknown_heading_cannot_truncate_the_source_statement() {
    let control = envelope(Some("Baba\n- Added knockback"));
    let bytes = envelope(Some(
        "Baba\n- Added knockback\nUnknown\n- Damage increased from 1 to 2",
    ));
    let post = patch_extract::parse_post(&bytes).unwrap();
    assert!(patch_extract::extract(&post, &catalog())
        .unwrap()
        .changes
        .is_empty());
    let mut forged = assertions(&control, &catalog());
    forged[0].hash =
        Sha256::digest(post.lexical_value("/raw_content").unwrap().as_bytes()).to_vec();
    forged[0].condition["post_hash"] = json!(hex::encode(Sha256::digest(&bytes)));
    forged[0].condition["next_heading"] = json!(2);
    assert!(patch_extract::prove(&post, &catalog(), &forged).is_err());
    assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &forged).is_err());
}

#[test]
fn boundary_binding_mutations_must_match_source_and_other_receipts() {
    let bytes = envelope(Some("Baba\n- Added knockback\nSinclair\n- Added knockback"));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let facts = assertions(&bytes, &catalog());
    assert_eq!(facts.len(), 2);
    for boundary in [
        json!(2),
        json!({"position":2}),
        json!({"position":2,"key":"hero_sinclair","kind":"hero","scope":"hero_unknown"}),
        json!({"position":2,"key":"hero_other","kind":"hero","scope":null}),
        json!({"position":2,"key":"hero_sinclair","kind":"ability","scope":null}),
        json!({"position":2,"key":"hero_sinclair","kind":"hero","scope":null,"alias":"Sinclair"}),
    ] {
        let mut forged = assertions(&bytes, &catalog());
        forged[0].condition["next_heading"] = boundary;
        assert!(patch_extract::prove(&post, &catalog(), &forged).is_err());
        assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &forged).is_err());
    }
}

#[test]
fn ambiguous_or_inapplicable_catalog_heading_cannot_release_the_previous_statement() {
    let bytes = envelope(Some("Baba\n- Added knockback\nShared\n- Recoil 10%"));
    let post = patch_extract::parse_post(&bytes).unwrap();
    for bindings in [
        vec![
            alias("Shared", "ability_a", "ability", Some("hero_baba")),
            alias("Shared", "ability_b", "ability", Some("hero_baba")),
        ],
        vec![alias("Shared", "ability_a", "ability", Some("hero_other"))],
    ] {
        let aliases = AliasIndex::new(
            std::iter::once(alias("Baba", "hero_baba", "hero", None))
                .chain(bindings)
                .collect(),
            Vec::new(),
        )
        .unwrap();
        let result = patch_extract::extract(&post, &aliases).unwrap();
        assert!(result.changes.is_empty());
        assert!(result
            .skipped
            .iter()
            .any(|entry| matches!(entry.reason, "ambiguous_alias" | "unscoped_alias")));
    }
}

#[test]
fn fabricated_self_consistent_boundaries_require_the_public_catalog_prover() {
    let bytes = envelope(Some("Baba\n- Added knockback\nUnknown\n- Recoil 10%"));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let fabricated = AliasIndex::new(
        vec![
            alias("Baba", "hero_baba", "hero", None),
            alias("Unknown", "hero_fabricated", "hero", None),
        ],
        Vec::new(),
    )
    .unwrap();
    let facts = assertions(&bytes, &fabricated);
    assert_eq!(facts.len(), 1);
    assert_eq!(
        facts[0].condition["next_heading"],
        json!({"position":2,"key":"hero_fabricated","kind":"hero","scope":null})
    );
    extract::prove(patch_extract::SOURCE, "42", &bytes, &facts).unwrap();
    assert!(patch_extract::prove(&post, &catalog(), &facts).is_err());
}

#[test]
fn capitalized_words_and_arbitrary_source_labels_do_not_infer_entities() {
    let bytes = envelope(Some("Objectives\nBaba\n- Added Knockback against Objectives"));
    let facts = assertions(&bytes, &catalog());
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].entity, "hero_baba");
    assert_eq!(facts[0].order, 2);
    assert!(
        matches!(&facts[0].value, Atom::Text(value) if value == "- Added Knockback against Objectives")
    );
}

#[test]
fn receipt_less_secondary_subjects_cannot_disappear_from_replay_checks() {
    let control = envelope(Some("Baba\n- Added knockback\nSinclair\n- Recoil 10%"));
    for statement in [
        "- Added knockback against Sinclair",
        "- Added knockback while Sinclair's effect is active",
        "- Added knockback\n  Sinclair: Recoil 10%",
        "- Added knockback\n  when Sinclair's effect is active",
        "- Added knockback\nSinclair: Removed knockback",
    ] {
        let content = format!("Baba\n{statement}\nSinclair\n- Recoil 10%");
        let bytes = envelope(Some(&content));
        let post = patch_extract::parse_post(&bytes).unwrap();
        let result = patch_extract::extract(&post, &catalog()).unwrap();
        assert!(result.changes.is_empty(), "{statement}");
        let mut forged = assertions(&control, &catalog());
        forged[0].hash =
            Sha256::digest(post.lexical_value("/raw_content").unwrap().as_bytes()).to_vec();
        forged[0].condition["post_hash"] = json!(hex::encode(Sha256::digest(&bytes)));
        forged[0].condition["next_heading"]["position"] = json!(1 + statement.lines().count());
        forged[0].value = Atom::Text(statement.into());
        assert!(patch_extract::prove(&post, &catalog(), &forged).is_err());
        assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &forged).is_err());
    }
    let aliases = AliasIndex::new(
        vec![
            alias("Baba", "hero_baba", "hero", None),
            alias("secondary", "hero_secondary", "hero", None),
        ],
        Vec::new(),
    )
    .unwrap();
    let control = envelope(Some("Baba\n- Added knockback\nsecondary\n- Recoil 10%"));
    let bytes = envelope(Some(
        "Baba\n- Added knockback against secondary\nsecondary\n- Recoil 10%",
    ));
    let post = patch_extract::parse_post(&bytes).unwrap();
    assert!(patch_extract::extract(&post, &aliases)
        .unwrap()
        .changes
        .is_empty());
    let mut forged = assertions(&control, &aliases);
    forged[0].hash =
        Sha256::digest(post.lexical_value("/raw_content").unwrap().as_bytes()).to_vec();
    forged[0].condition["post_hash"] = json!(hex::encode(Sha256::digest(&bytes)));
    forged[0].value = Atom::Text("- Added knockback against secondary".into());
    assert!(patch_extract::prove(&post, &aliases, &forged).is_err());
    assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &forged).is_err());
}

#[test]
fn conditional_numeric_subjects_never_become_unqualified_numeric_values() {
    for qualifier in ["When", "WHEN", "**When**", "[When]", "`When`", "While", "Unless"] {
        let statement = format!("- Baba: {qualifier} Damage increased from 1 to 2");
        let bytes = envelope(Some(&statement));
        let post = patch_extract::parse_post(&bytes).unwrap();
        let mut facts = assertions(&bytes, &catalog());
        assert_eq!(facts.len(), 1, "{statement}");
        assert!(matches!(&facts[0].value, Atom::Text(value) if value == &statement));
        assert!(facts[0].condition["old_value"].is_null());
        assert!(facts[0].condition["new_value"].is_null());
        assert!(facts[0].condition["stat"].is_null());
        facts[0].condition["old_value"] = json!("1");
        facts[0].condition["new_value"] = json!("2");
        assert!(patch_extract::prove(&post, &catalog(), &facts).is_err());
        assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &facts).is_err());
    }
    for content in [
        "Baba\n- Max Health increased from 780.0 to 800.0\n- Damage increased from 1 to 2",
        "Baba\n- Max Health increased from 780.0 to 800.0\n- Added knockback",
        "- Baba: Max Health increased from 780.0 to 800.0\n- Sinclair: Added knockback",
    ] {
        let bytes = envelope(Some(content));
        let facts = assertions(&bytes, &catalog());
        assert_eq!(facts.len(), 2, "{content}");
        assert_eq!(facts[0].condition["old_value"], "780.0");
        assert_eq!(facts[0].condition["new_value"], "800.0");
    }
}

#[test]
fn complete_statement_byte_limit_includes_qualified_bullet_continuations() {
    let prefix = "- Baba: Added ";
    let statement = format!(
        "{prefix}{}",
        "x".repeat(patch_extract::MAX_STATEMENT_BYTES - prefix.len())
    );
    let bytes = envelope(Some(&statement));
    let facts = assertions(&bytes, &catalog());
    assert_eq!(facts.len(), 1);
    assert!(
        matches!(&facts[0].value, Atom::Text(value) if value.len() == patch_extract::MAX_STATEMENT_BYTES)
    );
    let statement = format!(
        "- Baba: Max Health increased from 780.0 to 800.0\n  - **When** Damage increased from 1 to 2 {}",
        "ä".repeat(patch_extract::MAX_STATEMENT_BYTES / 2)
    );
    let bytes = envelope(Some(&statement));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let result = patch_extract::extract(&post, &catalog()).unwrap();
    assert!(result.changes.is_empty());
    assert_eq!(result.skipped[0].reason, "statement_limit");
    let control = envelope(Some("- Baba: Max Health increased from 780.0 to 800.0"));
    let mut forged = assertions(&control, &catalog());
    forged[0].hash =
        Sha256::digest(post.lexical_value("/raw_content").unwrap().as_bytes()).to_vec();
    forged[0].condition["post_hash"] = json!(hex::encode(Sha256::digest(&bytes)));
    assert!(patch_extract::prove(&post, &catalog(), &forged).is_err());
    assert!(extract::prove(patch_extract::SOURCE, "42", &bytes, &forged).is_err());
}

#[test]
fn condition_byte_limit_includes_catalog_boundary_metadata() {
    let owner_alias = "a".repeat(128);
    let owner_key = "h".repeat(128);
    let entity_alias = "b".repeat(128);
    let entity_key = "e".repeat(128);
    let boundary_alias = "c".repeat(128);
    let boundary_key = "n".repeat(128);
    let aliases = AliasIndex::new(
        vec![
            alias(&owner_alias, &owner_key, "hero", None),
            alias(&entity_alias, &entity_key, "ability", Some(&owner_key)),
            alias(&boundary_alias, &boundary_key, "ability", Some(&owner_key)),
        ],
        Vec::new(),
    )
    .unwrap();
    let content = format!("{owner_alias}\n{entity_alias}\n- Added knockback");
    let control = envelope(Some(&content));
    let facts = assertions(&control, &aliases);
    assert_eq!(facts.len(), 1);
    assert!(facts[0].condition.to_string().len() <= 1024);
    let bytes = envelope(Some(&format!("{content}\n{boundary_alias}\n- Recoil 10%")));
    let post = patch_extract::parse_post(&bytes).unwrap();
    assert_eq!(
        patch_extract::extract(&post, &aliases)
            .unwrap_err()
            .to_string(),
        "patch_condition_limit"
    );
}

#[test]
fn complete_single_scope_statements_are_not_rejected_by_action_word_count() {
    for statement in [
        "- Baba: Fixed a bug where knockback is now applied twice",
        "- Baba: Changed how removed effects are displayed",
        "- Baba: Now deals increased damage against objectives",
        "- Baba: Fixed Baba's knockback",
    ] {
        let bytes = envelope(Some(statement));
        let post = patch_extract::parse_post(&bytes).unwrap();
        let facts = assertions(&bytes, &catalog());
        assert_eq!(facts.len(), 1, "{statement}");
        assert!(matches!(&facts[0].value, Atom::Text(value) if value == statement));
        assert!(facts[0].condition["old_value"].is_null());
        assert!(facts[0].condition["new_value"].is_null());
        assert_eq!(
            facts[0].hash,
            Sha256::digest(post.lexical_value("/raw_content").unwrap().as_bytes()).to_vec()
        );
    }
    let bytes = envelope(Some(
        "Baba\n- Max Health increased from 780.0 to 800.0\n- Damage increased from 1 to 2",
    ));
    let facts = assertions(&bytes, &catalog());
    assert_eq!(facts.len(), 2);
    assert_eq!(facts[1].entity, "hero_baba");
    assert_eq!(facts[1].condition["old_value"], "1");
    assert_eq!(facts[1].condition["new_value"], "2");
    assert!(facts[1].condition["stat"].is_null());
}

#[test]
fn entity_generic_timer_catalog_entries_use_existing_extraction_and_proof_apis() {
    let aliases = AliasIndex::new(
        vec![alias("Objective Timer", "timer_objective", "timer", None)],
        vec![StatBinding {
            entity: "timer_objective".into(),
            label: "Respawn".into(),
            stat: "timer.respawn".into(),
        }],
    )
    .unwrap();
    let bytes = envelope(Some("- Objective Timer: Respawn increased from 60s to 90s"));
    let post = patch_extract::parse_post(&bytes).unwrap();
    let facts = assertions(&bytes, &aliases);
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].entity, "timer_objective");
    assert_eq!(facts[0].kind, "timer");
    assert_eq!(facts[0].condition["old_value"], "60s");
    assert_eq!(facts[0].condition["new_value"], "90s");
    assert_eq!(facts[0].condition["stat"], "timer.respawn");
    let unmapped = AliasIndex::new(
        vec![alias("Objective Timer", "timer_objective", "timer", None)],
        Vec::new(),
    )
    .unwrap();
    assert!(patch_extract::prove(&post, &unmapped, &facts).is_err());
    let bytes = envelope(Some("- timer_objective: Respawn increased from 60s to 90s"));
    let identities = std::collections::BTreeMap::from([("timer_objective".into(), "timer")]);
    let facts =
        extract::extract_with_identities(patch_extract::SOURCE, "42", &bytes, &identities).unwrap();
    assert_eq!(facts.len(), 1);
    extract::prove(patch_extract::SOURCE, "42", &bytes, &facts).unwrap();
}

#[test]
fn lexical_escape_spelling_is_proven_without_reserializing_the_envelope() {
    let fixture = include_str!("fixtures/patchnotes_lexical.json");
    let escaped = fixture.replace("Baba", &format!("{}u0042aba", char::from(92)));
    let bytes = escaped.as_bytes();
    let post = patch_extract::parse_post(bytes).unwrap();
    let facts = assertions(bytes, &catalog());
    assert_eq!(facts.len(), 1);
    assert_eq!(post.bytes(), bytes);
    let raw_lexeme = post.lexical_value("/raw_content").unwrap();
    assert!(raw_lexeme.contains("u0042aba"));
    assert_eq!(
        facts[0].hash,
        Sha256::digest(raw_lexeme.as_bytes()).to_vec()
    );
    assert_ne!(
        facts[0].hash,
        Sha256::digest(serde_json::to_vec(post.raw_content().unwrap()).unwrap()).to_vec()
    );
}

#[test]
fn shared_dispatch_checks_source_identity_and_preserves_p1_branches() {
    let bytes = envelope(Some("- hero_baba: Added knockback"));
    let path = patch_extract::object_path(42);
    let identities = std::collections::BTreeMap::from([("hero_baba".into(), "hero")]);
    let facts = extract::extract_with_identities(patch_extract::SOURCE, &path, &bytes, &identities)
        .unwrap();
    assert_eq!(facts.len(), 1);
    extract::prove(patch_extract::SOURCE, &path, &bytes, &facts).unwrap();
    assert!(
        extract::extract_with_identities(patch_extract::SOURCE, "43", &bytes, &identities).is_err()
    );
    assert!(!extract::supported(patch_extract::SOURCE, "042"));
    assert!(extract::supported(
        brain_job::git::Source::ALL[0],
        "data/json/hero-data.json"
    ));
}
