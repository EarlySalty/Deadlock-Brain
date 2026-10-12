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
    let parsed = requests
        .iter()
        .any(|r| supported(format, r) && r.receipt.locator_kind != "raw_bytes");
    let mut leaves = BTreeMap::new();
    if parsed {
        let text = std::str::from_utf8(bytes).map_err(|_| Rejection::InvalidUtf8)?;
        let budget = Budget::with_limit(allocation);
        match format {
            Format::Json => {
                json_text::parse_with_budget(text, budget).map_err(parser_rejection)?;
            }
            Format::Kv | Format::Kv3 => {
                let requested: BTreeSet<_> = requests
                    .iter()
                    .filter(|r| supported(format, r))
                    .map(|r| r.receipt.locator.as_str())
                    .collect();
                let values: Vec<_> = match format {
                    Format::Kv => kv::parse_with_budget(text, budget)
                        .map_err(parser_rejection)?
                        .into_iter()
                        .map(|e| (e.pointer, e.value))
                        .collect(),
                    _ => kv3::parse_with_budget(text, budget)
                        .map_err(parser_rejection)?
                        .into_iter()
                        .map(|l| (l.pointer, l.lexeme))
                        .collect(),
                };
                for (pointer, lexeme) in values {
                    if requested.contains(pointer.as_str())
                        && leaves
                            .insert(pointer, Sha256::digest(lexeme.as_bytes()).into())
                            .is_some()
                    {
                        return Err(Rejection::LocatorCollision);
                    }
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
        let hash = if receipt.locator_kind == "raw_bytes" || matches!(format, Format::Json) {
            receipts::value_bytes(bytes, receipt)
                .ok()
                .map(|v| Sha256::digest(v).into())
        } else {
            leaves.get(&receipt.locator).copied()
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
}
