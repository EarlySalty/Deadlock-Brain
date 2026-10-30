use brain_job::{
    ingest, limits,
    patch_extract::{PatchExtractor, SOURCE},
    Extractor, FactSet, FactValue, GameFactsStore, InputObject, Outcome, RawObject,
};
use brain_storage::{gamefacts::receipts, PgStore};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

const ADDED: &str = "- Baba: Added knockback";
const HEALTH_CHANGE: &str = "- Baba: Max Health increased from 780.0 to 800.0";
const COLLISION: &[u8] = include_bytes!("fixtures/patchnotes_normalized_key_collision.json");

fn oversized_content() -> String {
    format!(
        "Baba\n- Added knockback\n- Added {}",
        "ä".repeat(limits::TEXT_BYTES / 2 + 1)
    )
}

fn envelope(content: &str) -> Vec<u8> {
    serde_json::to_vec(&json!({"id":42,"title":"Update",
        "url":"https://forums.playdeadlock.com/threads/update.42/",
        "posted_at":"2026-10-05T18:00:00+00:00","raw_content":content}))
    .unwrap()
}

fn modified(field: &str, value: Value, content: &str) -> Vec<u8> {
    let mut fields: Value = serde_json::from_slice(&envelope(content)).unwrap();
    fields[field] = value;
    serde_json::to_vec(&fields).unwrap()
}

fn extract(path: &str, bytes: &[u8]) -> Outcome {
    PatchExtractor.extract(&RawObject {
        id: 1,
        source: SOURCE.into(),
        revision: "r1".into(),
        path: path.into(),
        content_sha256: Sha256::digest(bytes).into(),
        bytes: bytes.to_vec(),
    })
}

fn accepted(path: &str, bytes: &[u8]) -> FactSet {
    let Outcome::Accepted(set) = extract(path, bytes) else {
        panic!("unexpected outcome");
    };
    assert!(limits::validate(&set).is_ok());
    assert!(receipts::verify(bytes, &set).is_ok());
    set
}

fn text(value: &FactValue) -> &str {
    let FactValue::Text(text) = value else {
        panic!("expected text");
    };
    text
}

fn changes(set: &FactSet) -> Vec<Value> {
    set.facts
        .iter()
        .filter(|fact| fact.stat_key.starts_with("patch_change_"))
        .map(|fact| serde_json::from_str(text(&fact.value)).unwrap())
        .collect()
}
fn subject_line(fact: &brain_job::Fact) -> bool {
    fact.stat_key.starts_with("patch_line_r") || fact.stat_key.starts_with("patch_rename_r")
}

fn subject_lines(content: &str, prefix: &str, name: &str) -> Vec<Value> {
    let fingerprint = Sha256::digest(name.as_bytes());
    accepted("42", &envelope(content))
        .facts
        .iter()
        .filter(|fact| {
            fact.stat_key
                .starts_with(&format!("{prefix}{fingerprint:x}_"))
        })
        .map(|fact| serde_json::from_str(text(&fact.value)).unwrap())
        .collect()
}

#[test]
fn item_subject_lines_skip_hero_sections_and_record_renames() {
    let content = "[ General ] - Debuff Remover: Now renamed to Dispel Magic - Curse: Renamed Curse to Cursed Relic [ Spirit Items ] - Dispel Magic: Cooldown increased from 45s to 50s - only on allies [ Heroes ] - Abrams: Siphon Life renamed to Drain";
    let lines = subject_lines(content, "patch_line_r", "debuff remover");
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0]["renamed_to"], "Dispel Magic");
    assert_eq!(
        lines[0]["line_text"],
        "Debuff Remover: Now renamed to Dispel Magic"
    );
    let renamed = subject_lines(content, "patch_rename_r", "dispel magic");
    assert_eq!(renamed[0]["subject"], "Debuff Remover");
    assert_eq!(
        subject_lines(content, "patch_rename_r", "cursed relic")[0]["subject"],
        "Curse"
    );
    let current = subject_lines(content, "patch_line_r", "dispel magic");
    assert_eq!(
        current[0]["line_text"],
        "Dispel Magic: Cooldown increased from 45s to 50s - only on allies"
    );
    assert!(subject_lines(content, "patch_line_r", "abrams").is_empty());
}

#[test]
fn general_section_lines_become_patch_notes_outside_hero_sections() {
    let content = "[ General ]\n- Amount of souls in the flying orb that can be secured/denied reduced from 50% to 40%\n- Debuff Remover: Cooldown increased from 45s to 50s\n\n[ Heroes ]\n- Trooper bounty ratio changed from 40 to 50";
    let notes: Vec<Value> = accepted("42", &envelope(content))
        .facts
        .iter()
        .filter(|fact| fact.stat_key.starts_with("patch_note_"))
        .map(|fact| serde_json::from_str(text(&fact.value)).unwrap())
        .collect();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0]["section"], "General");
    assert_eq!(notes[0]["line_index"], 1);
    assert_eq!(
        notes[0]["line_text"],
        "Amount of souls in the flying orb that can be secured/denied reduced from 50% to 40%"
    );
}

#[test]
fn patch_note_continuations_stay_with_their_statement_and_drop_values() {
    let content = "[ General ]\n- Trooper bounty increased from 40 to 50\n  only against objectives\n- Walker souls increased from 10 to 20\n  - when defended\n- Urn souls increased from 1 to 2\n- Shrine souls increased from 5 to 6\n  Note: only against objectives\n- Vault souls increased from 7 to 8\n- Note: when defended\n\n  Guardian souls reduced from 3 to 2";
    let notes: Vec<Value> = accepted("42", &envelope(content))
        .facts
        .iter()
        .filter(|fact| fact.stat_key.starts_with("patch_note_"))
        .map(|fact| serde_json::from_str(text(&fact.value)).unwrap())
        .collect();
    assert_eq!(notes.len(), 5);
    assert_eq!(
        notes[0]["line_text"],
        "Trooper bounty increased from 40 to 50 only against objectives"
    );
    assert_eq!(notes[0]["old"], Value::Null);
    assert_eq!(
        notes[1]["line_text"],
        "Walker souls increased from 10 to 20 when defended"
    );
    assert_eq!(notes[1]["new"], Value::Null);
    assert_eq!(notes[2]["line_text"], "Urn souls increased from 1 to 2");
    assert_eq!(notes[2]["old"], "1");
    assert_eq!(
        notes[3]["line_text"],
        "Shrine souls increased from 5 to 6 Note: only against objectives"
    );
    assert_eq!(notes[3]["old"], Value::Null);
    assert_eq!(
        notes[4]["line_text"],
        "Vault souls increased from 7 to 8 Note: when defended"
    );
    assert_eq!(notes[4]["new"], Value::Null);
}

#[test]
fn flattened_posts_keep_every_general_bullet_of_long_sections() {
    let filler = " - Underground tunnels have more light".repeat(60);
    let content = format!(
        "[ Misc Gameplay ] - Movement powerup duration increased from 30s to 50s{filler} - Fire Rate powerup duration increased from 35s to 45s and +15% Fire Rate to +18% [ Item Gameplay ] - Extra Charge: CD reduction increased from +7% to +8%"
    );
    let notes: Vec<Value> = accepted("42", &envelope(&content))
        .facts
        .iter()
        .filter(|fact| fact.stat_key.starts_with("patch_note_"))
        .map(|fact| serde_json::from_str(text(&fact.value)).unwrap())
        .collect();
    assert_eq!(notes.len(), 62);
    assert_eq!(
        notes[0]["line_text"],
        "Movement powerup duration increased from 30s to 50s"
    );
    assert_eq!(notes[61]["section"], "Misc Gameplay");
    assert_eq!(
        notes[61]["line_text"],
        "Fire Rate powerup duration increased from 35s to 45s and +15% Fire Rate to +18%"
    );
}

#[test]
fn flattened_bullets_keep_qualifiers_and_skip_subject_lines() {
    for repeat in [0, 60] {
        flattened_bullets_case(repeat);
    }
}

fn flattened_bullets_case(repeat: usize) {
    let filler = " - Underground tunnels have more light".repeat(repeat);
    let content = format!(
        "[ General ] - Fleetfoot: Stamina increased from 2 to 3{filler} - Trooper bounty increased from 40 to 50 - only against objectives - Note: applies after 10 minutes - Respawn delay now 5 - 10 seconds - 4 Strong camps replaced"
    );
    let notes: Vec<Value> = accepted("42", &envelope(&content))
        .facts
        .iter()
        .filter(|fact| fact.stat_key.starts_with("patch_note_"))
        .map(|fact| serde_json::from_str(text(&fact.value)).unwrap())
        .collect();
    assert!(notes
        .iter()
        .all(|note| !note["line_text"].as_str().unwrap().contains("Fleetfoot")));
    let bounty = notes
        .iter()
        .find(|note| {
            note["line_text"]
                .as_str()
                .unwrap()
                .starts_with("Trooper bounty")
        })
        .unwrap();
    assert_eq!(
        bounty["line_text"],
        "Trooper bounty increased from 40 to 50 only against objectives Note: applies after 10 minutes"
    );
    assert!(bounty["old"].is_null());
    assert_eq!(
        notes[notes.len() - 2]["line_text"],
        "Respawn delay now 5 - 10 seconds"
    );
    assert_eq!(
        notes.last().unwrap()["line_text"],
        "4 Strong camps replaced"
    );
    assert_eq!(notes.len(), repeat + 3);
}

fn parse(content: &str) -> Vec<Value> {
    changes(&accepted("42", &envelope(content)))
}

fn reject(path: &str, bytes: &[u8]) -> &'static str {
    let Outcome::Rejected(rejection) = extract(path, bytes) else {
        panic!("unexpected outcome");
    };
    rejection.code
}

#[test]
fn complete_statements_emit_patch_owned_alias_text_and_source_stat_labels() {
    let content =
        "Heroes\nBaba\n- Max Health increased from 780.0 to 800.0\nSinclair\n- Added knockback";
    let set = accepted("42", &envelope(content));
    let rows = changes(&set);
    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[0],
        json!({"reference":"baba","subject":"Baba","statement_index":2,
        "context_index":1,"stat_label":"Max Health","old":"780.0","new":"800.0",
        "line_text":"- Max Health increased from 780.0 to 800.0"})
    );
    assert_eq!(rows[1]["reference"], "sinclair");
    assert_eq!(set.entities.len(), 1);
    assert_eq!(set.entities[0].kind, "patch");
    assert_eq!(set.entities[0].key, "patch_42");
    assert!(set.aliases.is_empty() && set.relations.is_empty());
    let keys: Vec<_> = set
        .facts
        .iter()
        .map(|fact| fact.stat_key.as_str())
        .collect();
    assert_eq!(keys.len(), 5);
    assert_eq!(&keys[..3], ["label", "released_at", "official_url"]);
    for (key, reference, index) in [(&keys[3], "baba", 2), (&keys[4], "sinclair", 4)] {
        let fingerprint = Sha256::digest(reference.as_bytes());
        assert_eq!(*key, format!("patch_change_r{fingerprint:x}_{index}"));
    }
    assert_eq!(PatchExtractor.version(), "7");
    let rows = parse("- Baba: Damage increased from 1 to 2");
    assert_eq!(rows[0]["stat_label"], "Damage");
}

#[test]
fn qualifiers_preserve_complete_text_without_unqualified_numeric_values() {
    for suffix in [
        "\n  when attacking objectives",
        "\n  - **When** Damage increased from 1 to 2",
        " - only against objectives",
        ", only against objectives",
        "; only against objectives",
    ] {
        let statement = format!("- Baba: Max Health increased from 780.0 to 800.0{suffix}");
        let rows = parse(&statement);
        assert_eq!(rows.len(), 1, "{statement}");
        assert_eq!(rows[0]["line_text"], statement);
        for field in ["old", "new", "stat_label"] {
            assert!(rows[0][field].is_null());
        }
    }
    for qualifier in ["When", "WHEN", "**When**", "[When]", "`When`", "Unless"] {
        let rows = parse(&format!("- Baba: {qualifier} Damage increased from 1 to 2"));
        assert_eq!(rows.len(), 1);
        assert!(rows[0]["old"].is_null());
    }
}

#[test]
fn multiple_subjects_and_unsafe_boundaries_cannot_truncate_statements() {
    for content in [
        "- Baba: Added knockback; Sinclair: Removed knockback",
        "- Baba: Added knockback against sinclair",
        "- Baba: Added knockback and Unknown removed knockback",
        "- Baba: Added knockback\n  Unknown removed knockback",
    ] {
        assert!(parse(content).is_empty(), "{content}");
    }
    let rows = parse("- Baba & Sinclair: Added knockback");
    assert!(rows
        .iter()
        .all(|row| row["reference"] != "baba" && row["reference"] != "sinclair"));
    for qualifier in [
        "when attacking objectives",
        "When Damage increased from 1 to 2",
        "**When** damage is increased",
    ] {
        for marker in ["", "- ", "* ", "• "] {
            let content =
                format!("- Baba: Max Health increased from 780.0 to 800.0\n{marker}{qualifier}");
            assert!(parse(&content).is_empty(), "{content}");
        }
    }
}

#[test]
fn ampersand_heroes_and_ability_prefixed_lines_keep_their_subject() {
    let rows = parse("- Mo & Krill: Scorn damage increased from 40 to 45");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["reference"], "mo & krill");
    let rows = parse("- Rat King: Rat Swarm - Base Cooldown reduced from 28s to 24s");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["reference"], "rat king");
    assert_eq!(rows[0]["stat_label"], "Base Cooldown");
    let rows = parse(
        "- Baba: Added knockback\n- Rat King: Rule, Ratannia! - no longer scales with spirit",
    );
    assert_eq!(rows[0]["line_text"], "- Baba: Added knockback");
}

#[test]
fn sibling_note_bullets_stay_with_their_statement() {
    let rows = parse(
        "- Baba: Damage increased from 1 to 2\n- Note: applies to both forms\n- Bebop: Added knockback",
    );
    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[0]["line_text"],
        "- Baba: Damage increased from 1 to 2\n- Note: applies to both forms"
    );
    assert_eq!(rows[0]["stat_label"], "Damage");
    assert_eq!(rows[1]["reference"], "bebop");
}

#[test]
fn foreign_aliases_remain_unresolved_without_catalogue_or_invented_hero_entities() {
    let rows = parse("- Unknown: Added knockback");
    assert_eq!(rows[0]["reference"], "unknown");
    let rows = parse("Baba\n- Added knockback\nUnknown\n- Damage increased from 1 to 2");
    assert_eq!(rows.len(), 2);
    assert_eq!(
        (&rows[0]["reference"], &rows[1]["reference"]),
        (&json!("baba"), &json!("unknown"))
    );
    assert_eq!(rows[1]["context_index"], 2);
    let set = accepted("42", &envelope("Shared\n- Added knockback"));
    assert_eq!(changes(&set)[0]["reference"], "shared");
    assert_eq!(set.entities.len(), 1);
    assert!(set.aliases.is_empty() && set.relations.is_empty());
}

#[test]
fn lexical_receipts_preserve_escape_spelling_and_source_identity() {
    let bytes = br#"{ "id":42, "title":"Update", "url":"https://forums.playdeadlock.com/threads/update.42/", "posted_at":"2026-10-05T18:00:00+00:00", "raw_content":"- Baba: Added knockback" }"#;
    let encoded = String::from_utf8(bytes.to_vec())
        .unwrap()
        .replace("Baba", &format!("{}u0042aba", char::from(92)));
    let bytes = encoded.as_bytes();
    let mut set = accepted("42", bytes);
    let hash = |bytes: &[u8]| <[u8; 32]>::from(Sha256::digest(bytes));
    let token = format!("\"- {}u0042aba: Added knockback\"", char::from(92));
    assert_eq!(set.facts[3].receipt.value_sha256, hash(token.as_bytes()));
    assert_ne!(
        set.facts[3].receipt.value_sha256,
        hash(br#""- Baba: Added knockback""#)
    );
    assert_eq!(changes(&set)[0]["line_text"], "- Baba: Added knockback");
    assert_eq!(set.entities[0].receipt.locator_kind, "raw_bytes");
    assert_eq!(
        receipts::value_bytes(bytes, &set.entities[0].receipt).unwrap(),
        bytes
    );
    for (fact, pointer) in set
        .facts
        .iter()
        .zip(["/title", "/posted_at", "/url", "/raw_content"])
    {
        assert_eq!(fact.receipt.locator, pointer);
    }
    set.facts[3].receipt.value_sha256 = [0; 32];
    assert!(receipts::verify(bytes, &set).is_err());
    assert_eq!(reject("43", bytes), "source_identity");
    assert!(PatchExtractor.accepts(SOURCE, "42"));
    assert!(!PatchExtractor.accepts(SOURCE, "042"));
    assert!(!PatchExtractor.accepts("other", "42"));
    assert!(matches!(extract("042", bytes), Outcome::Ignored));
}

#[test]
fn official_origin_and_five_field_envelope_are_strict() {
    for url in [
        "http://forums.playdeadlock.com/threads/update.42/",
        "https://forums.playdeadlock.com.evil.test/threads/update.42/",
        "https://store.steampowered.com/news/app/570/view/42",
        "https://user@forums.playdeadlock.com/threads/update.42/",
        "https://forums.playdeadlock.com:443/threads/update.42/",
        "https://forums.playdeadlock.com/threads/update.%34%32/",
        "https://store.steampowered.com/news/app/1422450/view/42/../43",
    ] {
        let bytes = modified("url", json!(url), ADDED);
        assert_eq!(reject("42", &bytes), "untrusted_origin");
    }
    for (extra, code) in [
        (r#", "id":43"#, "key_collision"),
        (r#", "translated_content":"x""#, "envelope_invalid"),
        (r#", " Title ":"x""#, "key_collision"),
    ] {
        let mut bytes = envelope("- Baba: Added knockback");
        bytes.pop();
        bytes.extend_from_slice(format!("{extra}}}").as_bytes());
        assert_eq!(reject("42", &bytes), code);
    }
    assert_eq!(reject("42", COLLISION), "key_collision");
    for field in ["posted_at", "raw_content"] {
        let bytes = modified(field, Value::Null, ADDED);
        assert!(matches!(extract("42", &bytes), Outcome::Ignored));
    }
}

fn forum_post(title: &str, url: &str, posted_at: &str) -> Vec<u8> {
    serde_json::to_vec(&json!({"id":42,"title":title,"url":url,
        "posted_at":posted_at,"raw_content":ADDED}))
    .unwrap()
}

fn identity(set: &FactSet) -> (&str, &str, &str) {
    (
        text(&set.facts[0].value),
        text(&set.facts[1].value),
        text(&set.facts[2].value),
    )
}

#[test]
fn official_update_thread_posts_get_unique_dated_labels_and_canonical_urls() {
    for (title, url, posted_at, label, canonical) in [
        (
            "11-21-2025 Update",
            "https://forums.playdeadlock.com/posts/172057/",
            "2025-11-23T17:11:20.000000Z",
            "11-21-2025 Update (Post 172057, 2025-11-23)",
            "https://forums.playdeadlock.com/posts/172057/",
        ),
        (
            "09-29-2026",
            "https://forums.playdeadlock.com/posts/330973",
            "2026-10-15T03:12:00.000000Z",
            "09-29-2026 (Post 330973, 2026-10-15)",
            "https://forums.playdeadlock.com/posts/330973/",
        ),
        (
            "12-29-2025 Update",
            "https://forums.playdeadlock.com/posts/180519/",
            "2025-12-30T06:33:52.000000Z",
            "12-29-2025 Update (Post 180519, 2025-12-30)",
            "https://forums.playdeadlock.com/posts/180519/",
        ),
        (
            "03-06-2026 Update",
            "https://forums.playdeadlock.com/threads/03-06-2026-update.114328/#post-216766",
            "2026-03-06T21:37:48.000000Z",
            "03-06-2026 Update (Post 216766, 2026-03-06)",
            "https://forums.playdeadlock.com/posts/216766/",
        ),
        (
            "03-06-2026 Update",
            "https://forums.playdeadlock.com/threads/03-06-2026-update.114328/page-2#post-217001",
            "2026-03-08T10:00:00.000000Z",
            "03-06-2026 Update (Post 217001, 2026-03-08)",
            "https://forums.playdeadlock.com/posts/217001/",
        ),
    ] {
        let bytes = forum_post(title, url, posted_at);
        let set = accepted("42", &bytes);
        assert_eq!(identity(&set), (label, posted_at, canonical));
        assert_eq!(set.facts[0].receipt.locator, "/title");
        assert_eq!(set.facts[2].receipt.locator, "/url");
        assert_eq!(changes(&set).len(), 1);
    }
}

#[test]
fn forum_replies_outside_official_update_threads_stay_untrusted() {
    for (title, url, posted_at, code) in [
        (
            "General Discussion",
            "https://forums.playdeadlock.com/posts/172057/",
            "2025-11-23T17:11:20Z",
            "unofficial_thread",
        ),
        (
            "Re: 11-21-2025 Update",
            "https://forums.playdeadlock.com/posts/172057/",
            "2025-11-23T17:11:20Z",
            "unofficial_thread",
        ),
        (
            "11-21-2025 Update feedback",
            "https://forums.playdeadlock.com/posts/172057/",
            "2025-11-23T17:11:20Z",
            "unofficial_thread",
        ),
        (
            "13-40-2025 Update",
            "https://forums.playdeadlock.com/posts/172057/",
            "2025-11-23T17:11:20Z",
            "unofficial_thread",
        ),
        (
            "11-21-2025 Update",
            "https://forums.playdeadlock.com/posts/172057/",
            "2025-11-20T23:59:59Z",
            "post_before_thread",
        ),
        (
            "03-06-2026 Update",
            "https://forums.playdeadlock.com/threads/hero-feedback.114328/#post-216766",
            "2026-03-06T21:37:48Z",
            "unofficial_thread",
        ),
        (
            "03-06-2026 Update",
            "https://forums.playdeadlock.com/threads/03-06-2026-update.114328/page-0#post-216766",
            "2026-03-06T21:37:48Z",
            "untrusted_origin",
        ),
        (
            "11-21-2025 Update",
            "https://forums.playdeadlock.com/posts/172057/?page=2",
            "2025-11-23T17:11:20Z",
            "untrusted_origin",
        ),
        (
            "11-21-2025 Update",
            "https://forums.playdeadlock.com/posts/0/",
            "2025-11-23T17:11:20Z",
            "untrusted_origin",
        ),
        (
            "11-21-2025 Update",
            "http://forums.playdeadlock.com/posts/172057/",
            "2025-11-23T17:11:20Z",
            "untrusted_origin",
        ),
        (
            "11-21-2025 Update",
            "https://forums.playdeadlock.com.evil.test/posts/172057/",
            "2025-11-23T17:11:20Z",
            "untrusted_origin",
        ),
        (
            "11-21-2025 Update",
            "https://steamcommunity.com/posts/172057/",
            "2025-11-23T17:11:20Z",
            "untrusted_origin",
        ),
    ] {
        assert_eq!(
            reject("42", &forum_post(title, url, posted_at)),
            code,
            "{title} {url} {posted_at}"
        );
    }
}

#[test]
fn oversized_complete_value_rejects_instead_of_accepting_partial_changes() {
    let content = oversized_content();
    assert_eq!(reject("42", &envelope(&content)), "value_too_large");
    let prefix = "- Baba: Added ";
    for length in [512, 513] {
        assert_eq!(
            parse(&format!("{prefix}{}", "x".repeat(length - prefix.len()))).len(),
            1
        );
    }
    let overhead = parse(prefix)[0].to_string().len();
    let exact = format!("{prefix}{}", "x".repeat(limits::TEXT_BYTES - overhead));
    let set = accepted("42", &envelope(&exact));
    assert_eq!(text(&set.facts[3].value).len(), limits::TEXT_BYTES);
    assert_eq!(
        reject("42", &envelope(&format!("{exact}x"))),
        "value_too_large"
    );
}

#[test]
fn original_public_baba_roster_and_release_are_not_balance_changes() {
    let fixtures: [&[u8]; 3] = [
        include_bytes!("fixtures/patchnotes_public_286.json"),
        include_bytes!("fixtures/patchnotes_public_287.json"),
        include_bytes!("fixtures/patchnotes_public_290.json"),
    ];
    for (id, bytes) in ["286", "287", "290"].into_iter().zip(fixtures) {
        let set = accepted(id, bytes);
        assert!(changes(&set).is_empty(), "{id}");
        assert_eq!(set.entities[0].key, format!("patch_{id}"));
        assert_eq!(
            set.facts
                .iter()
                .filter(|fact| !subject_line(fact) && !fact.stat_key.starts_with("patch_note_"))
                .count(),
            3
        );
    }
}

async fn count(pool: &PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}

#[tokio::test]
async fn real_store_poison_rejections_finish_and_leave_following_run_available() {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|c, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_ingest").execute(c).await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .unwrap();
    let store = GameFactsStore::new(writer.clone());
    let large = modified("id", json!(41), &oversized_content());
    let collision = COLLISION.to_vec();
    let good = modified("id", json!(43), HEALTH_CHANGE);
    let input = |path: &str, bytes: &[u8]| {
        Ok(InputObject {
            revision: "r1".into(),
            path: path.into(),
            bytes: bytes.to_vec(),
        })
    };
    let objects = [
        input("41", &large),
        input("42", &collision),
        input("43", &good),
    ];
    let first = ingest(&store, SOURCE, objects, &[&PatchExtractor])
        .await
        .unwrap();
    let counts = first.counts;
    assert_eq!(
        (counts.accepted, counts.rejected, counts.ignored),
        (1, 2, 0)
    );
    let reasons: Vec<String> =
        sqlx::query_scalar("SELECT code FROM ops.ingest_rejection WHERE run_id=$1 ORDER BY id")
            .bind(first.run_id)
            .fetch_all(&admin)
            .await
            .unwrap();
    assert_eq!(reasons, ["value_too_large", "key_collision"]);
    for (path, bytes) in [("41", large), ("42", collision), ("43", good)] {
        let stored: Vec<u8> = sqlx::query_scalar("SELECT bytes FROM raw.object WHERE path=$1")
            .bind(path)
            .fetch_one(&admin)
            .await
            .unwrap();
        assert_eq!(stored, bytes);
    }
    let succeeded =
        "SELECT count(*) FROM ops.ingest_run WHERE status='succeeded' AND finished IS NOT NULL";
    for (sql, expected) in [
        (succeeded, 1),
        ("SELECT count(*) FROM canon.entity WHERE key IN('patch_41','patch_42')", 0),
        ("SELECT count(*) FROM canon.receipt r JOIN raw.object o ON o.id=r.raw_object_id WHERE o.path IN('41','42')", 0),
        ("SELECT count(*) FROM serve.current_fact WHERE entity_id IN(SELECT id FROM serve.current_entity WHERE key='patch_43') AND stat_key NOT LIKE 'patch\\_line\\_r%'", 4),
        ("SELECT count(*) FROM raw.object", 3),
    ] {
        assert_eq!(count(&admin, sql).await, expected, "{sql}");
    }
    let objects = [input("42", &envelope(ADDED))];
    let following = ingest(&store, SOURCE, objects, &[&PatchExtractor])
        .await
        .unwrap();
    assert!(following.run_id > first.run_id);
    assert_eq!(
        (following.counts.accepted, following.counts.rejected),
        (1, 0)
    );
    assert_eq!(count(&admin, succeeded).await, 2);
    println!("P4_POISON accepted=1 rejected=2 codes={reasons:?} first=succeeded following=succeeded poison_raw=2 poison_canonical=0");
    writer.close().await;
    admin.close().await;
}
