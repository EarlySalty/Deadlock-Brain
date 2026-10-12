use brain_storage::gamefacts::{limits, receipts, Entity, Fact, FactSet, FactValue, Receipt};
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
