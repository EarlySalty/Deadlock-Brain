use super::{budget::Budget, json_text, kv, kv3};
use crate::gamefacts::{receipts, RawObject, Receipt};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Json,
    Kv,
    Kv3,
}
#[derive(Clone, Copy, Debug)]
pub struct Policy {
    pub max_raw_bytes: usize,
    pub max_requests: usize,
    pub max_locator_bytes: usize,
    pub parser_allocation_bytes: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct Request<'a> {
    pub receipt: &'a Receipt,
    pub explicitly_unsupported: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptOutcome {
    Match([u8; 32]),
    Mismatch([u8; 32]),
    LocatorNotFound,
    UnsupportedLocatorKind,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rejection {
    RequestLimit,
    RawSizeLimit,
    LocatorSizeLimit,
    InvalidUtf8,
    LocatorCollision,
    ContentHashMismatch,
    Parser(&'static str),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verification {
    Receipt(ReceiptOutcome),
    ObjectRejected(Rejection),
}

pub fn supported(format: Format, request: &Request<'_>) -> bool {
    !request.explicitly_unsupported
        && match request.receipt.locator_kind.as_str() {
            "raw_bytes" => request.receipt.locator.is_empty(),
            "json_pointer" => matches!(format, Format::Json),
            "kv_path" | "byte_ranges" => true,
            "kv_pointer" => matches!(format, Format::Kv | Format::Kv3),
            _ => false,
        }
}

pub fn verify(
    raw: &RawObject,
    format: Format,
    requests: &[Request<'_>],
    policy: Policy,
) -> Result<Vec<Verification>, Rejection> {
    if requests.len() > policy.max_requests {
        return Err(Rejection::RequestLimit);
    }
    let compatible = |r: &Request<'_>| supported(format, r);
    let hashes = if !requests.iter().any(compatible) {
        Ok(BTreeMap::new())
    } else if raw.bytes.len() > policy.max_raw_bytes {
        Err(Rejection::RawSizeLimit)
    } else if requests
        .iter()
        .any(|r| compatible(r) && r.receipt.locator.len() > policy.max_locator_bytes)
    {
        Err(Rejection::LocatorSizeLimit)
    } else if <[u8; 32]>::from(Sha256::digest(&raw.bytes)) != raw.content_sha256 {
        Err(Rejection::ContentHashMismatch)
    } else {
        raw_hashes(&raw.bytes, format, requests, policy.parser_allocation_bytes)
    };
    Ok(requests
        .iter()
        .map(|request| {
            if !compatible(request) {
                return Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind);
            }
            let hashes = match &hashes {
                Ok(hashes) => hashes,
                Err(reason) => return Verification::ObjectRejected(*reason),
            };
            let key = (&request.receipt.locator_kind, &request.receipt.locator);
            Verification::Receipt(match hashes.get(&key) {
                Some(Some(hash)) if *hash == request.receipt.value_sha256 => {
                    ReceiptOutcome::Match(*hash)
                }
                Some(Some(hash)) => ReceiptOutcome::Mismatch(*hash),
                _ => ReceiptOutcome::LocatorNotFound,
            })
        })
        .collect())
}

type Hashes<'a> = BTreeMap<(&'a String, &'a String), Option<[u8; 32]>>;
fn raw_hashes<'a>(
    bytes: &[u8],
    format: Format,
    requests: &[Request<'a>],
    allocation: usize,
) -> Result<Hashes<'a>, Rejection> {
    let json_requested: BTreeSet<_> = requests
        .iter()
        .filter(|r| supported(format, r) && r.receipt.locator_kind == "json_pointer")
        .map(|r| r.receipt.locator.as_str())
        .collect();
    let json = if json_requested.is_empty() {
        BTreeMap::new()
    } else {
        let text = std::str::from_utf8(bytes).map_err(|_| Rejection::InvalidUtf8)?;
        json_text::selected_hashes(text, &json_requested, Budget::with_limit(allocation))
            .map_err(parser_rejection)?
    };
    let kv_path = requests
        .iter()
        .any(|r| supported(format, r) && r.receipt.locator_kind == "kv_path");
    let kv_pointer = requests
        .iter()
        .any(|r| supported(format, r) && r.receipt.locator_kind == "kv_pointer");
    let mut leaves = BTreeMap::new();
    if kv_path || kv_pointer {
        let text = std::str::from_utf8(bytes).map_err(|_| Rejection::InvalidUtf8)?;
        let text = text.trim_start_matches('\u{feff}').trim_start();
        let budget = Budget::with_limit(allocation);
        let requested: BTreeSet<_> = requests
            .iter()
            .filter(|r| {
                supported(format, r)
                    && matches!(r.receipt.locator_kind.as_str(), "kv_path" | "kv_pointer")
            })
            .map(|r| r.receipt.locator.as_str())
            .collect();
        if kv_path || matches!(format, Format::Kv3) {
            leaves = kv3::selected_hashes(text, &requested, budget).map_err(parser_rejection)?;
        } else {
            for entry in kv::parse_with_budget(text, budget).map_err(parser_rejection)? {
                if requested.contains(entry.pointer.as_str())
                    && leaves
                        .insert(entry.pointer, Sha256::digest(entry.value.as_bytes()).into())
                        .is_some()
                {
                    return Err(Rejection::LocatorCollision);
                }
            }
        }
    }
    let mut hashes = BTreeMap::new();
    for request in requests.iter().filter(|r| supported(format, r)) {
        let receipt = request.receipt;
        let key = (&receipt.locator_kind, &receipt.locator);
        if hashes.contains_key(&key) {
            continue;
        }
        let hash = match receipt.locator_kind.as_str() {
            "raw_bytes" => Some(Sha256::digest(bytes).into()),
            "json_pointer" => json.get(&receipt.locator).copied(),
            "byte_ranges" => receipts::value_bytes(bytes, receipt)
                .ok()
                .map(|v| Sha256::digest(v).into()),
            _ => leaves.get(&receipt.locator).copied(),
        };
        hashes.insert(key, hash);
    }
    Ok(hashes)
}
fn parser_rejection(code: &'static str) -> Rejection {
    if code == "duplicate_json_keys_preserved_as_text" {
        Rejection::LocatorCollision
    } else {
        Rejection::Parser(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice::from_ref;
    fn policy() -> Policy {
        Policy {
            max_raw_bytes: 4096,
            max_requests: 200,
            max_locator_bytes: 128,
            parser_allocation_bytes: 1024 * 1024,
        }
    }
    fn raw(bytes: &[u8]) -> RawObject {
        RawObject {
            id: 1,
            source: "test".into(),
            revision: "r1".into(),
            path: "unrelated.kv3".into(),
            content_sha256: Sha256::digest(bytes).into(),
            bytes: bytes.into(),
        }
    }
    fn receipt(kind: &str, locator: &str, token: &[u8]) -> Receipt {
        Receipt {
            locator_kind: kind.into(),
            locator: locator.into(),
            value_sha256: Sha256::digest(token).into(),
        }
    }
    fn check(
        raw: &RawObject,
        format: Format,
        values: &[Receipt],
        policy: Policy,
    ) -> Vec<Verification> {
        verify(
            raw,
            format,
            &values
                .iter()
                .map(|r| Request {
                    receipt: r,
                    explicitly_unsupported: false,
                })
                .collect::<Vec<_>>(),
            policy,
        )
        .unwrap()
    }
    fn audit_policy() -> Policy {
        Policy {
            max_raw_bytes: crate::gamefacts::limits::RAW_BYTES,
            max_requests: 200,
            max_locator_bytes: crate::gamefacts::limits::LOCATOR_BYTES,
            parser_allocation_bytes: 64 * 1024 * 1024,
        }
    }
    #[test]
    fn selected_json_scales_to_raw_limit_without_fact_expansion() {
        let array = format!("[{}1]", "1,".repeat(1_999_999));
        let text = format!("{{\"values\":{array}}}");
        let raw = raw(text.as_bytes());
        let values = [
            receipt("json_pointer", "/values", array.as_bytes()),
            receipt("json_pointer", "/values/+1999999", b"1"),
        ];
        assert_eq!(
            check(&raw, Format::Json, &values, audit_policy()),
            values.map(|r| Verification::Receipt(ReceiptOutcome::Match(r.value_sha256)))
        );
    }
    #[test]
    fn selected_json_key_validation_is_bounded_and_detects_unselected_collisions() {
        use std::fmt::Write;
        let mut text = String::from("{");
        for i in 0..280_000 {
            write!(text, "\"k{i}\":1,").unwrap();
        }
        text.push_str("\"tail\":1}");
        let values = [receipt("json_pointer", "/tail", b"1")];
        let good = raw(text.as_bytes());
        assert_eq!(
            check(&good, Format::Json, &values, audit_policy()),
            [Verification::Receipt(ReceiptOutcome::Match(
                values[0].value_sha256
            ))]
        );
        text.truncate(text.len() - 1);
        text.push_str(",\"k0\":2}");
        assert_eq!(
            check(&raw(text.as_bytes()), Format::Json, &values, audit_policy()),
            [Verification::ObjectRejected(Rejection::LocatorCollision)]
        );
    }
    #[test]
    fn selected_kv3_scales_near_core_budget_without_leaf_expansion() {
        let text = format!(
            "<!-- kv3 encoding:text --> {{ values = [{}1] x = 1.2300e+2 x = -0 }}",
            "1,".repeat(599_999)
        );
        let raw = raw(text.as_bytes());
        let values = [
            receipt("kv_path", "/values/0/[599999]", b"1"),
            receipt("kv_path", "/x/0", b"1.2300e+2"),
            receipt("kv_path", "/x/1", b"-0"),
        ];
        assert_eq!(
            check(&raw, Format::Json, &values, audit_policy()),
            values
                .clone()
                .map(|r| Verification::Receipt(ReceiptOutcome::Match(r.value_sha256)))
        );
        let mut small = audit_policy();
        small.parser_allocation_bytes = 1024 * 1024;
        assert_eq!(
            check(&raw, Format::Json, &values, small),
            [Verification::ObjectRejected(Rejection::Parser(super::super::budget::EXCEEDED)); 3]
        );
    }
    #[test]
    fn selected_depth_matches_core_without_changing_generic_guard() {
        for depth in [129, 2000] {
            let text = format!("  {}1{}  ", "[".repeat(depth), "]".repeat(depth));
            let pointer = "/0".repeat(depth);
            let requested = BTreeSet::from(["", "/+0", pointer.as_str()]);
            let hashes =
                json_text::selected_hashes(&text, &requested, Budget::with_limit(64 * 1024 * 1024))
                    .unwrap();
            for locator in requested {
                let receipt = Receipt {
                    locator_kind: "json_pointer".into(),
                    locator: locator.into(),
                    value_sha256: [0; 32],
                };
                let bytes = receipts::value_bytes(text.as_bytes(), &receipt).unwrap();
                assert_eq!(hashes[locator], <[u8; 32]>::from(Sha256::digest(bytes)));
            }
            assert_eq!(
                json_text::parse(&text).unwrap_err(),
                "json_depth_exceeded_preserved_as_text"
            );
        }
    }
    #[test]
    fn core_parity_includes_root_containers_escapes_and_numeric_lexemes() {
        let raw = raw(br#"  {"a/b~c":["a\nb",1.2300e+2,-0],"object": { "x" : true }}  "#);
        for pointer in [
            "",
            "/a~1b~0c",
            "/object",
            "/a~1b~0c/0",
            "/a~1b~0c/1",
            "/a~1b~0c/2",
        ] {
            let mut r = receipt("json_pointer", pointer, b"");
            r.value_sha256 = Sha256::digest(receipts::value_bytes(&raw.bytes, &r).unwrap()).into();
            assert_eq!(
                check(&raw, Format::Json, from_ref(&r), policy()),
                [Verification::Receipt(ReceiptOutcome::Match(r.value_sha256))]
            );
        }
        let r = receipt("raw_bytes", "", &raw.bytes);
        assert_eq!(
            check(&raw, Format::Json, from_ref(&r), policy()),
            [Verification::Receipt(ReceiptOutcome::Match(r.value_sha256))]
        );
    }
    #[test]
    fn unsupported_precedence_and_guards_do_not_poison_compatible_requests() {
        for (format, kind, incompatible, locator, bytes) in [
            (
                Format::Json,
                "json_pointer",
                "kv_pointer",
                "/x",
                br#"{"x":1}"#.as_slice(),
            ),
            (Format::Kv, "kv_pointer", "json_pointer", "/x/0", b"x 1"),
            (
                Format::Kv3,
                "kv_pointer",
                "json_pointer",
                "/x/0",
                b"{ x = 1 }",
            ),
        ] {
            let long = "/x".repeat(65536);
            let good = receipt(kind, locator, b"1");
            let values = [
                receipt(incompatible, &long, b"1"),
                receipt("future_pointer", &long, b"1"),
                good.clone(),
            ];
            assert_eq!(
                check(&raw(bytes), format, &values, policy()),
                [
                    Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind),
                    Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind),
                    Verification::Receipt(ReceiptOutcome::Match(good.value_sha256))
                ]
            );
            let tier = Request {
                receipt: &good,
                explicitly_unsupported: true,
            };
            assert_eq!(
                verify(&raw(&[0xff; 5000]), format, &[tier], policy()).unwrap(),
                [Verification::Receipt(
                    ReceiptOutcome::UnsupportedLocatorKind
                )]
            );
            let values = [
                receipt(kind, &long, b"1"),
                receipt(kind, &format!("{long}b"), b"1"),
            ];
            assert_eq!(
                check(&raw(&[0xff]), format, &values, policy()),
                [Verification::ObjectRejected(Rejection::LocatorSizeLimit); 2]
            );
            assert_eq!(
                check(&raw(&[0xff; 5000]), format, &values, policy()),
                [Verification::ObjectRejected(Rejection::RawSizeLimit); 2]
            );
            let mut p = policy();
            p.max_requests = 0;
            assert_eq!(
                verify(
                    &raw(bytes),
                    format,
                    &[Request {
                        receipt: &good,
                        explicitly_unsupported: false
                    }],
                    p
                ),
                Err(Rejection::RequestLimit)
            );
        }
    }
    #[test]
    fn collisions_budget_integrity_and_missing_are_distinct() {
        let r = receipt("json_pointer", "/x", b"1");
        for (bytes, expected) in [
            (
                br#"{"x":1,"x":2}"#.as_slice(),
                Verification::ObjectRejected(Rejection::LocatorCollision),
            ),
            (
                br#"{"other": {"a":1,"a":2},"x":1}"#,
                Verification::ObjectRejected(Rejection::LocatorCollision),
            ),
            (
                br#"{"y":1}"#,
                Verification::Receipt(ReceiptOutcome::LocatorNotFound),
            ),
            (
                br#"{"x":2}"#,
                Verification::Receipt(ReceiptOutcome::Mismatch(Sha256::digest(b"2").into())),
            ),
        ] {
            assert_eq!(
                check(&raw(bytes), Format::Json, from_ref(&r), policy()),
                [expected]
            );
        }
        let mut p = policy();
        p.parser_allocation_bytes = 1;
        assert_eq!(
            check(&raw(br#"{"x":1}"#), Format::Json, from_ref(&r), p),
            [Verification::ObjectRejected(Rejection::Parser(
                super::super::budget::EXCEEDED
            ))]
        );
        let mut bad = raw(br#"{"x":1}"#);
        bad.content_sha256 = [0; 32];
        assert_eq!(
            check(&bad, Format::Json, &[r], policy()),
            [Verification::ObjectRejected(Rejection::ContentHashMismatch)]
        );
    }
    #[test]
    fn core_kv_path_and_byte_ranges_lexemes_match_exactly() {
        for (bytes, kind, paths) in [
            (
                "\u{feff}  <!-- kv3 encoding:text --> { x = 1.2300e+2 x = -0 name = \"a\\nb\" }"
                    .as_bytes(),
                "kv_path",
                vec!["/x/0", "/x/1", "/name/0"],
            ),
            (
                &b"abc0123xyz45"[..],
                "byte_ranges",
                vec!["3-7", "3-7,10-12"],
            ),
        ] {
            let raw = raw(bytes);
            for locator in paths {
                let mut r = receipt(kind, locator, b"");
                r.value_sha256 = Sha256::digest(receipts::value_bytes(bytes, &r).unwrap()).into();
                assert_eq!(
                    check(&raw, Format::Json, from_ref(&r), policy()),
                    [Verification::Receipt(ReceiptOutcome::Match(r.value_sha256))]
                );
            }
        }
        let raw = raw(&[0xff, 1, 2]);
        let r = receipt("byte_ranges", "0-1,2-3", &[0xff, 2]);
        assert_eq!(
            check(&raw, Format::Json, from_ref(&r), policy()),
            [Verification::Receipt(ReceiptOutcome::Match(r.value_sha256))]
        );
    }
}
