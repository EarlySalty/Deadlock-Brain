use brain_storage::gamefacts::{
    limits, receipts, Alias, Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, Receipt,
    Relation,
};

#[path = "support/gamefacts_pg.rs"]
pub mod gamefacts_pg;
use sha2::{Digest, Sha256};

fn receipt(kind: &str, locator: &str, lexeme: &str) -> Receipt {
    Receipt {
        locator_kind: kind.into(),
        locator: locator.into(),
        value_sha256: Sha256::digest(lexeme.as_bytes()).into(),
    }
}

#[test]
fn decimal_bounds_are_exact_including_scientific_notation_and_long_exponents() {
    for text in [
        "0",
        "-0",
        "1e15",
        "-1E+15",
        "1000000000000000.0000",
        "0.0001e19",
        "9.999999999999999999e14",
        "1e-99999999999999999999999",
        "0e99999999999999999999",
    ] {
        assert!(limits::validate_numeric_lexeme(text).is_ok(), "{text}");
    }
    for text in [
        "1000000000000000.00000001",
        "-1000000000000000.00000001",
        "1.00000000000000001e15",
        "10.0000000000000001e14",
        "1000000000000001",
        "1e999999999999999999",
        "NaN",
        "Infinity",
        "-inf",
        "",
        "01",
        "+1",
        "1.",
        ".1",
        "1e",
        "1e+-2",
        "1 2",
        " 1",
        "1\0",
    ] {
        assert_eq!(
            limits::validate_numeric_lexeme(text).unwrap_err().code,
            "invalid_number",
            "{text:?}"
        );
    }
    assert_eq!(
        "1000000000000000.00000001".parse::<f64>().unwrap(),
        limits::NUMBER_ABS
    );
    assert!(
        limits::validate_numeric_lexeme(&format!("0.{}1", "0".repeat(limits::TEXT_BYTES))).is_err()
    );
    assert!(limits::validate_numeric_lexeme(&format!("0.{}1e{}", "0".repeat(1000), 1016)).is_ok());
    assert!(limits::validate_numeric_lexeme(&format!("0.{}2e{}", "0".repeat(1000), 1016)).is_err());
}

#[test]
fn rounded_numbers_cannot_bypass_original_lexeme_validation() {
    let bytes = br#"{"n":1000000000000000.00000001}"#;
    let set = FactSet {
        facts: vec![Fact {
            entity_key: "hero_test".into(),
            stat_key: "health".into(),
            value: FactValue::Number(limits::NUMBER_ABS),
            receipt: receipt("json_pointer", "/n", "1000000000000000.00000001"),
        }],
        ..FactSet::default()
    };
    assert!(limits::validate(&set).is_ok());
    assert_eq!(
        receipts::verify(bytes, &set).unwrap_err().code,
        "invalid_number"
    );
}

#[test]
fn kv_paths_keep_occurrences_arrays_escaped_keys_and_original_scalar_tokens() {
    let bytes = br#"<!-- kv3 encoding:text --> { "a/b~c" = { x = 1.2300e+2 x = -0 } arr = [ true, "a\nb" ] "[0]" = 42 }"#;
    for (path, lexeme) in [
        ("/a~1b~0c/0/x/0", "1.2300e+2"),
        ("/a~1b~0c/0/x/1", "-0"),
        ("/arr/0/[0]", "true"),
        ("/arr/0/[1]", r#""a\nb""#),
        ("/[0]/0", "42"),
    ] {
        assert_eq!(
            receipts::value_bytes(bytes, &receipt("kv_path", path, lexeme)).unwrap(),
            lexeme.as_bytes()
        );
    }
    for path in [
        "/a/b~c/0/x/0",
        "/a~1b~0c/x",
        "/a~1b~0c/0/x/01",
        "/arr/0/0",
        "/arr/0/[01]",
        "/[0]",
        "/a~2b/0",
        "/arr/0/[2]",
    ] {
        assert!(
            receipts::value_bytes(bytes, &receipt("kv_path", path, "")).is_err(),
            "{path}"
        );
    }
    assert!(receipts::value_bytes(b"{ x = 1", &receipt("kv_path", "/x/0", "1")).is_err());
    let set = FactSet {
        facts: vec![Fact {
            entity_key: "hero_test".into(),
            stat_key: "health".into(),
            value: FactValue::Number(123.0),
            receipt: receipt("kv_path", "/a~1b~0c/0/x/0", "1.2300e+2"),
        }],
        ..FactSet::default()
    };
    receipts::verify(bytes, &set).unwrap();
}

#[test]
fn byte_ranges_concatenate_ordered_disjoint_bounded_original_bytes() {
    let bytes = b"# Title\nintro\n## Section\nbody text\n";
    for (locator, value) in [
        ("0-7", "# Title"),
        ("0-7,14-24", "# Title## Section"),
        ("2-7,7-8", "Title\n"),
        ("25-34", "body text"),
    ] {
        assert_eq!(
            receipts::value_bytes(bytes, &receipt("byte_ranges", locator, value)).unwrap(),
            value.as_bytes()
        );
    }
    for locator in [
        "",
        "7-0",
        "3-3",
        "0-7,5-9",
        "14-24,0-7",
        "0-36",
        "00-7",
        "-1-7",
        "0-7,",
        "0_7",
        "0-7 ",
        "0-99999999999999999999999",
    ] {
        assert!(
            receipts::value_bytes(bytes, &receipt("byte_ranges", locator, "")).is_err(),
            "{locator:?}"
        );
    }
    let many = vec![b'x'; limits::BYTE_RANGE_BYTES + 2];
    let ranges = |count: usize| {
        (0..count)
            .map(|i| format!("{}-{}", i * 2, i * 2 + 1))
            .collect::<Vec<_>>()
            .join(",")
    };
    let at_cap = ranges(limits::BYTE_RANGES);
    assert_eq!(
        receipts::value_bytes(&many, &receipt("byte_ranges", &at_cap, ""))
            .unwrap()
            .len(),
        limits::BYTE_RANGES
    );
    let over = ranges(limits::BYTE_RANGES + 1);
    assert!(receipts::value_bytes(&many, &receipt("byte_ranges", &over, "")).is_err());
    let full = format!("0-{}", limits::BYTE_RANGE_BYTES);
    assert!(receipts::value_bytes(&many, &receipt("byte_ranges", &full, "")).is_ok());
    let too_long = format!("0-{}", limits::BYTE_RANGE_BYTES + 1);
    assert!(receipts::value_bytes(&many, &receipt("byte_ranges", &too_long, "")).is_err());
    let set = FactSet {
        facts: vec![Fact {
            entity_key: "doc_test".into(),
            stat_key: "section".into(),
            value: FactValue::Text("# Title## Section".into()),
            receipt: receipt("byte_ranges", "0-7,14-24", "# Title## Section"),
        }],
        ..FactSet::default()
    };
    receipts::verify(bytes, &set).unwrap();
}

#[test]
fn a_full_factset_uses_no_extra_success_marker_row() {
    let mut set = FactSet {
        entities: (0..limits::ROWS)
            .map(|i| Entity {
                kind: "hero".into(),
                key: format!("hero_{i}"),
                receipt: receipt("raw_bytes", "", "{}"),
            })
            .collect(),
        ..FactSet::default()
    };
    receipts::verify(b"{}", &set).unwrap();
    set.entities.push(set.entities[0].clone());
    assert_eq!(limits::validate(&set).unwrap_err().code, "too_many_rows");
}

fn ambiguous_json() -> [String; 6] {
    let mut values = [
        r#"{"x":1,"x":2}"#,
        r#"{"x":1,"x":2}"#,
        r#"{"x":2,"outside":{"a":1,"a":2}}"#,
        r#"{"x":2,"outside":[{"a":1,"a":2}]}"#,
        r#"{"x":2,"outside":{"a/b~c":1,"a\/b~c":2}}"#,
        r#"{"x":2,"outside":{"ß":1,"ß":2}}"#,
    ]
    .map(str::to_owned);
    let slash = char::from(92);
    values[1] = format!(r#"{{"x":1,"{slash}u0078":2}}"#);
    values[5] = format!(r#"{{"x":2,"outside":{{"ß":1,"{slash}u00df":2}}}}"#);
    values
}

#[test]
fn duplicate_decoded_json_keys_reject_every_locator_and_empty_sets() {
    for text in ambiguous_json() {
        for proof in [
            receipt("json_pointer", "/x", "2"),
            receipt("json_pointer", "", &text),
            receipt("json_pointer", "/outside", ""),
            receipt("raw_bytes", "", &text),
            receipt("byte_ranges", "0-1", "{"),
        ] {
            let rejection = receipts::value_bytes(text.as_bytes(), &proof).unwrap_err();
            assert_eq!(rejection.code, "locator_collision");
            assert_eq!(
                rejection.detail,
                "Rohobjekt enthält mehrdeutige JSON-Schlüssel"
            );
            let set = FactSet {
                entities: vec![Entity {
                    kind: "hero".into(),
                    key: "hero_test".into(),
                    receipt: proof,
                }],
                ..FactSet::default()
            };
            assert_eq!(
                receipts::verify(text.as_bytes(), &set).unwrap_err().code,
                "locator_collision"
            );
        }
        assert_eq!(
            receipts::verify(text.as_bytes(), &FactSet::default())
                .unwrap_err()
                .code,
            "locator_collision"
        );
    }
}

#[test]
fn unique_json_keeps_source_lexemes_and_distinct_object_keys() {
    let text = r#"  {"a/b~c":["a\nb",1.2300e+2,-0,18446744073709551616,1e999],"object": { "x" : true },"other":{"x":false},"X":1,"é":2,"é":3}  "#;
    for (pointer, token) in [
        ("", text.trim()),
        (
            "/a~1b~0c",
            r#"["a\nb",1.2300e+2,-0,18446744073709551616,1e999]"#,
        ),
        ("/object", r#"{ "x" : true }"#),
        ("/a~1b~0c/0", r#""a\nb""#),
        ("/a~1b~0c/+1", "1.2300e+2"),
        ("/a~1b~0c/2", "-0"),
        ("/a~1b~0c/3", "18446744073709551616"),
        ("/a~1b~0c/4", "1e999"),
        ("/other/x", "false"),
        ("/X", "1"),
        ("/é", "2"),
        ("/e\u{0301}", "3"),
    ] {
        let proof = receipt("json_pointer", pointer, token);
        assert_eq!(
            receipts::value_bytes(text.as_bytes(), &proof).unwrap(),
            token.as_bytes()
        );
        receipts::verify(
            text.as_bytes(),
            &FactSet {
                entities: vec![Entity {
                    kind: "hero".into(),
                    key: "hero_control".into(),
                    receipt: proof,
                }],
                ..FactSet::default()
            },
        )
        .unwrap();
    }
    assert_eq!(
        receipts::value_bytes(text.as_bytes(), &receipt("raw_bytes", "", text)).unwrap(),
        text.as_bytes()
    );
    for bytes in [
        b"not JSON".as_slice(),
        b"{ x = 1 x = 2 }",
        b"{\"x\":1,\"x\":2} trailing",
        &[0xff, 1, 2],
    ] {
        let proof = Receipt {
            locator_kind: "raw_bytes".into(),
            locator: String::new(),
            value_sha256: Sha256::digest(bytes).into(),
        };
        assert_eq!(receipts::value_bytes(bytes, &proof).unwrap(), bytes);
        receipts::verify(
            bytes,
            &FactSet {
                entities: vec![Entity {
                    kind: "hero".into(),
                    key: "hero_control".into(),
                    receipt: proof,
                }],
                ..FactSet::default()
            },
        )
        .unwrap();
        assert_eq!(
            receipts::value_bytes(bytes, &receipt("json_pointer", "", ""))
                .unwrap_err()
                .code,
            "invalid_receipt"
        );
    }
}

#[test]
fn json_validation_is_stack_safe_and_shared_by_distinct_receipts_near_raw_cap() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let text = format!(
                "{{\"deep\":{}1{},\"x\":1}}",
                "[".repeat(2_000_000),
                "]".repeat(2_000_000)
            );
            assert!(text.len() < limits::RAW_BYTES);
            receipts::verify(text.as_bytes(), &FactSet::default()).unwrap();
            assert_eq!(
                receipts::value_bytes(text.as_bytes(), &receipt("json_pointer", "/x", "1"))
                    .unwrap(),
                b"1"
            );
            let duplicate = text.replace(",\"x\":1}", ",\"other\":{\"a\":1,\"a\":2},\"x\":1}");
            assert_eq!(
                receipts::verify(duplicate.as_bytes(), &FactSet::default())
                    .unwrap_err()
                    .code,
                "locator_collision"
            );
        })
        .unwrap()
        .join()
        .unwrap();
    let text = format!("[{}1]", "1,".repeat(1_999_999));
    let set = FactSet {
        facts: (0..limits::ROWS)
            .map(|i| Fact {
                entity_key: "hero_test".into(),
                stat_key: format!("stat_{i}"),
                value: FactValue::Number(1.0),
                receipt: receipt("json_pointer", &format!("/{}", 1_999_999 - i), "1"),
            })
            .collect(),
        ..FactSet::default()
    };
    receipts::verify(text.as_bytes(), &set).unwrap();
}

#[tokio::test]
async fn duplicate_json_store_rejection_preserves_raw_and_all_canonical_boundaries() {
    let (_pg, admin, writer) = gamefacts_pg::fixture(2).await;
    let role: (String, bool, bool) = sqlx::query_as(
        "SELECT current_user,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user",
    )
    .fetch_one(&writer)
    .await
    .unwrap();
    assert_eq!(role, ("brain_ingest".into(), false, false));
    let denied = sqlx::query("SELECT 1 FROM serve.current_fact LIMIT 0")
        .fetch_all(&writer)
        .await
        .unwrap_err();
    assert_eq!(
        denied.as_database_error().and_then(|e| e.code()).as_deref(),
        Some("42501")
    );
    let version: String = sqlx::query_scalar("SHOW server_version")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert!(version.starts_with("16."));
    let store = GameFactsStore::new(writer.clone());
    let run = store.begin_run("collision").await.unwrap();
    let mut rejected = 0;
    for text in ambiguous_json() {
        let raw = store
            .capture(run, "collision", "r1", "bad.json", text.as_bytes().to_vec())
            .await
            .unwrap();
        let stored: (Vec<u8>, Vec<u8>) =
            sqlx::query_as("SELECT bytes,content_sha256 FROM raw.object WHERE id=$1")
                .bind(raw.id)
                .fetch_one(&admin)
                .await
                .unwrap();
        assert_eq!(
            stored,
            (
                text.as_bytes().to_vec(),
                Sha256::digest(text.as_bytes()).to_vec()
            )
        );
        let root = receipt("raw_bytes", "", &text);
        for set in [
            FactSet {
                entities: vec![Entity {
                    kind: "hero".into(),
                    key: "hero_bad".into(),
                    receipt: root.clone(),
                }],
                ..FactSet::default()
            },
            FactSet {
                aliases: vec![Alias {
                    entity_key: "hero_bad".into(),
                    alias: "Bad".into(),
                    lang: "en".into(),
                    receipt: root.clone(),
                }],
                ..FactSet::default()
            },
            FactSet {
                facts: vec![Fact {
                    entity_key: "hero_bad".into(),
                    stat_key: "health".into(),
                    value: FactValue::Number(2.0),
                    receipt: receipt("json_pointer", "/x", "2"),
                }],
                ..FactSet::default()
            },
            FactSet {
                relations: vec![Relation {
                    from_key: "hero_bad".into(),
                    kind: "related_to".into(),
                    to_key: "hero_bad".into(),
                    receipt: root,
                }],
                ..FactSet::default()
            },
            FactSet::default(),
            FactSet {
                entities: vec![Entity {
                    kind: "hero".into(),
                    key: "hero_bad".into(),
                    receipt: receipt("json_pointer", "", text.trim()),
                }],
                ..FactSet::default()
            },
            FactSet {
                entities: vec![Entity {
                    kind: "hero".into(),
                    key: "hero_bad".into(),
                    receipt: receipt("json_pointer", "/outside", ""),
                }],
                ..FactSet::default()
            },
        ] {
            let rejection = store.apply(run, &raw, "collision", &set).await.unwrap_err();
            assert_eq!(rejection.code, "locator_collision");
            store
                .reject(run, raw.id, "collision", &rejection)
                .await
                .unwrap();
            rejected += 1;
        }
    }
    let canonical: (i64, i64, i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM canon.entity),(SELECT count(*) FROM canon.alias),(SELECT count(*) FROM canon.fact),(SELECT count(*) FROM canon.relation),(SELECT count(*) FROM canon.receipt)").fetch_one(&admin).await.unwrap();
    assert_eq!(canonical, (0, 0, 0, 0, 0));
    let good = r#"  {"x":2,"values":[1.2300e+2,-0,0.12345678901234567890123456789]}  "#;
    let raw = store
        .capture(
            run,
            "collision",
            "r1",
            "good.json",
            good.as_bytes().to_vec(),
        )
        .await
        .unwrap();
    let set = FactSet {
        entities: vec![Entity {
            kind: "hero".into(),
            key: "hero_good".into(),
            receipt: receipt("raw_bytes", "", good),
        }],
        facts: [
            ("health", FactValue::Number(2.0), "/x", "2"),
            (
                "decimal",
                FactValue::Number(123.0),
                "/values/0",
                "1.2300e+2",
            ),
            ("negative_zero", FactValue::Number(-0.0), "/values/1", "-0"),
            (
                "precision",
                FactValue::Text("0.12345678901234567890123456789".into()),
                "/values/2",
                "0.12345678901234567890123456789",
            ),
        ]
        .into_iter()
        .map(|(stat_key, value, pointer, lexeme)| Fact {
            entity_key: "hero_good".into(),
            stat_key: stat_key.into(),
            value,
            receipt: receipt("json_pointer", pointer, lexeme),
        })
        .collect(),
        ..FactSet::default()
    };
    store.apply(run, &raw, "collision", &set).await.unwrap();
    for proof in
        std::iter::once(&set.entities[0].receipt).chain(set.facts.iter().map(|f| &f.receipt))
    {
        let stored: Vec<u8> = sqlx::query_scalar("SELECT value_sha256 FROM canon.receipt WHERE raw_object_id=$1 AND locator_kind=$2 AND locator=$3").bind(raw.id).bind(&proof.locator_kind).bind(&proof.locator).fetch_one(&admin).await.unwrap();
        assert_eq!(stored, proof.value_sha256);
    }
    store
        .finish_run(
            run,
            true,
            Counts {
                accepted: 1,
                rejected,
                ignored: 0,
            },
        )
        .await
        .unwrap();
    assert_eq!(gamefacts_pg::growth(&admin).await, (7, 4, 5));
    assert_eq!(
        gamefacts_pg::rejection_codes(&admin, run).await,
        vec!["locator_collision"; 42]
    );
    gamefacts_pg::assert_count(
        &admin,
        "SELECT count(*) FROM serve.current_fact WHERE value_num=2",
        1,
    )
    .await;
    writer.close().await;
    admin.close().await;
}

#[test]
#[ignore = "requires the read-only GameTracking cache at the attested revision"]
fn cached_gametracking_kv3_lookup_and_batch_verification_use_original_bytes() {
    let repository = std::env::var_os("BRAIN_CORE2_KV3_REPOSITORY").expect("cache path required");
    let result = std::process::Command::new("git")
        .arg("-C")
        .arg(repository)
        .args([
            "show",
            "e0b9830a7f69726c933da90446e669ad276e73f8:game/citadel/pak01_dir/scripts/heroes.vdata",
        ])
        .output()
        .unwrap();
    assert!(result.status.success());
    let bytes = result.stdout;
    assert_eq!(bytes.len(), 2_567_685);
    assert!(bytes.len() < limits::RAW_BYTES);
    let mut facts = Vec::new();
    for (key, lexeme, number) in [
        ("EMaxMoveSpeed", "7.2", 7.2),
        ("EMaxHealth", "780.0", 780.0),
        ("EBaseHealthRegen", "2.0", 2.0),
    ] {
        let proof = receipt(
            "kv_path",
            &format!("/hero_base/0/m_mapStartingStats/0/{key}/0"),
            lexeme,
        );
        assert_eq!(
            receipts::value_bytes(&bytes, &proof).unwrap(),
            lexeme.as_bytes()
        );
        facts.push(Fact {
            entity_key: "hero_base".into(),
            stat_key: limits::normalize_key(key),
            value: FactValue::Number(number),
            receipt: proof,
        });
    }
    receipts::verify(
        &bytes,
        &FactSet {
            facts,
            ..FactSet::default()
        },
    )
    .unwrap();
}
