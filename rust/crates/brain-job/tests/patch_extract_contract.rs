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
    assert_eq!(
        keys,
        [
            "label",
            "released_at",
            "official_url",
            "patch_change_2",
            "patch_change_4"
        ]
    );
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
        "- Baba & Sinclair: Added knockback",
    ] {
        assert!(parse(content).is_empty(), "{content}");
    }
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
        assert_eq!(set.facts.len(), 3);
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
        ("SELECT count(*) FROM serve.current_fact WHERE entity_id IN(SELECT id FROM serve.current_entity WHERE key='patch_43')", 4),
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
