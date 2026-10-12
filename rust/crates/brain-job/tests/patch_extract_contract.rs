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
