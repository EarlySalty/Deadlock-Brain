use super::{budget::Budget, json_text, kv, kv3};
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
    pub locator_kind: &'a str,
    pub locator: &'a str,
    pub expected_sha256: [u8; 32],
    pub explicitly_unsupported: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptOutcome {
    Match([u8; 32]),
    Mismatch([u8; 32]),
    LocatorNotFound,
    UnsupportedLocatorKind,
}

impl ReceiptOutcome {
    pub fn quarantines(self) -> bool {
        matches!(self, Self::Mismatch(_) | Self::LocatorNotFound)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rejection {
    RequestLimit,
    RawSizeLimit,
    LocatorSizeLimit,
    InvalidUtf8,
    LocatorCollision,
    Parser(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verification {
    Receipt(ReceiptOutcome),
    ObjectRejected(Rejection),
}

pub fn verify(
    bytes: &[u8],
    format: Format,
    requests: &[Request<'_>],
    policy: Policy,
) -> Result<Vec<Verification>, Rejection> {
    if requests.len() > policy.max_requests {
        return Err(Rejection::RequestLimit);
    }
    let supported = |request: &Request<'_>| {
        !request.explicitly_unsupported
            && matches!(request.locator_kind, "json_pointer" | "kv_pointer")
    };
    let requested: BTreeSet<_> = requests
        .iter()
        .filter(|request| supported(request))
        .map(|request| request.locator)
        .collect();
    let hashes = if requested.is_empty() {
        Ok(BTreeMap::new())
    } else if bytes.len() > policy.max_raw_bytes {
        Err(Rejection::RawSizeLimit)
    } else if requested
        .iter()
        .any(|locator| locator.len() > policy.max_locator_bytes)
    {
        Err(Rejection::LocatorSizeLimit)
    } else {
        raw_hashes(bytes, format, &requested, policy.parser_allocation_bytes)
    };
    let expected_kind = match format {
        Format::Json => "json_pointer",
        Format::Kv | Format::Kv3 => "kv_pointer",
    };
    Ok(requests
        .iter()
        .map(|request| {
            if !supported(request) {
                return Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind);
            }
            let hashes = match &hashes {
                Ok(hashes) => hashes,
                Err(reason) => return Verification::ObjectRejected(*reason),
            };
            let observed = (request.locator_kind == expected_kind)
                .then(|| hashes.get(request.locator))
                .flatten();
            Verification::Receipt(match observed {
                Some(hash) if *hash == request.expected_sha256 => ReceiptOutcome::Match(*hash),
                Some(hash) => ReceiptOutcome::Mismatch(*hash),
                None => ReceiptOutcome::LocatorNotFound,
            })
        })
        .collect())
}

fn raw_hashes(
    bytes: &[u8],
    format: Format,
    requested: &BTreeSet<&str>,
    allocation_bytes: usize,
) -> Result<BTreeMap<String, [u8; 32]>, Rejection> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| Rejection::InvalidUtf8)?
        .trim_start_matches('\u{feff}')
        .trim_start();
    let budget = Budget::with_limit(allocation_bytes);
    let mut hashes = BTreeMap::new();
    match format {
        Format::Json | Format::Kv3 => {
            let leaves = match format {
                Format::Json => json_text::parse_with_budget(text, budget),
                _ => kv3::parse_with_budget(text, budget),
            }
            .map_err(parser_rejection)?;
            for leaf in leaves {
                insert_hash(&mut hashes, requested, leaf.pointer, leaf.lexeme.as_bytes())?;
            }
        }
        Format::Kv => {
            for entry in kv::parse_with_budget(text, budget).map_err(parser_rejection)? {
                insert_hash(
                    &mut hashes,
                    requested,
                    entry.pointer,
                    entry.value.as_bytes(),
                )?;
            }
        }
    }
    Ok(hashes)
}

fn insert_hash(
    hashes: &mut BTreeMap<String, [u8; 32]>,
    requested: &BTreeSet<&str>,
    pointer: String,
    lexeme: &[u8],
) -> Result<(), Rejection> {
    if requested.contains(pointer.as_str())
        && hashes
            .insert(pointer, Sha256::digest(lexeme).into())
            .is_some()
    {
        return Err(Rejection::LocatorCollision);
    }
    Ok(())
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

    fn policy() -> Policy {
        Policy {
            max_raw_bytes: 4096,
            max_requests: 200,
            max_locator_bytes: 2048,
            parser_allocation_bytes: 1024 * 1024,
        }
    }

    fn request<'a>(kind: &'a str, locator: &'a str, lexeme: &[u8]) -> Request<'a> {
        Request {
            locator_kind: kind,
            locator,
            expected_sha256: Sha256::digest(lexeme).into(),
            explicitly_unsupported: false,
        }
    }

    #[test]
    fn exact_json_lexemes_and_escaped_pointers_match_without_semantic_membership() {
        let requests = [
            request("json_pointer", "/a~1b~0c/0", b"\"a\\nb\""),
            request("json_pointer", "/a~1b~0c/1", b"1.00e+2"),
            request("json_pointer", "/a~1b~0c/2", b"-0"),
            request("json_pointer", "/other", b"1.00e+2"),
            request("json_pointer", "/a~1b~0c/0", b"\"a\nb\""),
        ];
        let results = verify(
            br#"{"a/b~c":["a\nb",1.00e+2,-0]}"#,
            Format::Json,
            &requests,
            policy(),
        )
        .unwrap();
        for (result, request) in results[..3].iter().zip(&requests) {
            assert_eq!(
                *result,
                Verification::Receipt(ReceiptOutcome::Match(request.expected_sha256))
            );
        }
        assert_eq!(
            results[3],
            Verification::Receipt(ReceiptOutcome::LocatorNotFound)
        );
        assert_eq!(
            results[4],
            Verification::Receipt(ReceiptOutcome::Mismatch(requests[0].expected_sha256))
        );
    }

    #[test]
    fn kv_occurrences_and_source_escape_bytes_are_distinct() {
        let requests = [
            request("kv_pointer", "/root/0/a~1b~0c/0", br"a\nb"),
            request("kv_pointer", "/root/0/a~1b~0c/1", b"2"),
            request("kv_pointer", "/root/1/a~1b~0c/0", b"3"),
        ];
        let results = verify(
            br#"root { "a/b~c" "a\nb" "a/b~c" 2 } root { "a/b~c" 3 }"#,
            Format::Kv,
            &requests,
            policy(),
        )
        .unwrap();
        for (result, request) in results.iter().zip(requests) {
            assert_eq!(
                *result,
                Verification::Receipt(ReceiptOutcome::Match(request.expected_sha256))
            );
        }
    }

    #[test]
    fn kv3_quoted_numeric_multiline_and_array_lexemes_are_exact() {
        let requests = [
            request("kv_pointer", "/x/0", b"\"a\\u0062\""),
            request("kv_pointer", "/x/1", b"1e+02"),
            request("kv_pointer", "/values/0/[0]", b"true"),
            request("kv_pointer", "/text/0", b"\"\"\"a\nb\"\"\""),
        ];
        let results = verify(
            b"<!-- kv3 encoding:text --> { x = \"a\\u0062\" x = 1e+02 values = [ true ] text = \"\"\"a\nb\"\"\" }",
            Format::Kv3,
            &requests,
            policy(),
        ).unwrap();
        for (result, request) in results.iter().zip(requests) {
            assert_eq!(
                *result,
                Verification::Receipt(ReceiptOutcome::Match(request.expected_sha256))
            );
        }
    }

    #[test]
    fn poisoned_objects_are_counted_and_a_later_healthy_object_is_verified() {
        let oversized = vec![b' '; policy().max_raw_bytes + 1];
        let objects: [&[u8]; 5] = [
            &oversized,
            br#"{"x":1,"x":2}"#,
            br#"{"y":1}"#,
            br#"{"x":1}"#,
            br#"{"x":1"#,
        ];
        let mut counts = [0; 3];
        for bytes in objects {
            let results = verify(
                bytes,
                Format::Json,
                &[request("json_pointer", "/x", b"1")],
                policy(),
            )
            .unwrap();
            match results[0] {
                Verification::ObjectRejected(_) => counts[0] += 1,
                Verification::Receipt(ReceiptOutcome::LocatorNotFound) => counts[1] += 1,
                Verification::Receipt(ReceiptOutcome::Match(_)) => counts[2] += 1,
                other => panic!("unexpected verification: {other:?}"),
            }
        }
        assert_eq!(counts, [3, 1, 1]);
    }

    #[test]
    fn unsupported_kinds_and_explicit_tiers_survive_object_rejection() {
        let mut tier = request("json_pointer", "/ability/Upgrades/3/cooldown", b"1");
        tier.explicitly_unsupported = true;
        let results = verify(
            br#"{"x":1,"x":2}"#,
            Format::Json,
            &[
                request("future_pointer", "/x", b"1"),
                tier,
                request("json_pointer", "/x", b"1"),
            ],
            policy(),
        )
        .unwrap();
        assert_eq!(
            results,
            [
                Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind),
                Verification::Receipt(ReceiptOutcome::UnsupportedLocatorKind),
                Verification::ObjectRejected(Rejection::LocatorCollision),
            ]
        );
        assert!(!ReceiptOutcome::UnsupportedLocatorKind.quarantines());
        assert!(ReceiptOutcome::LocatorNotFound.quarantines());
        assert!(ReceiptOutcome::Mismatch([0; 32]).quarantines());
        assert!(!ReceiptOutcome::Match([0; 32]).quarantines());
    }

    #[test]
    fn caller_limits_are_explicit_rejections_not_false_receipt_outcomes() {
        let requests = [request("json_pointer", "/x", b"1")];
        let mut limits = policy();
        limits.parser_allocation_bytes = 1;
        assert_eq!(
            verify(br#"{"x":1}"#, Format::Json, &requests, limits).unwrap(),
            [Verification::ObjectRejected(Rejection::Parser(
                super::super::budget::EXCEEDED
            )),]
        );
        limits = policy();
        limits.max_locator_bytes = 1;
        assert_eq!(
            verify(br#"{"x":1}"#, Format::Json, &requests, limits).unwrap(),
            [Verification::ObjectRejected(Rejection::LocatorSizeLimit)]
        );
        limits = policy();
        limits.max_requests = 0;
        assert_eq!(
            verify(b"{}", Format::Json, &requests, limits),
            Err(Rejection::RequestLimit)
        );
        assert_eq!(
            verify(&[0xff], Format::Json, &requests, policy()).unwrap(),
            [Verification::ObjectRejected(Rejection::InvalidUtf8)]
        );
    }

    #[test]
    fn healthy_multi_root_document_uses_the_callers_document_envelope() {
        let requests = [
            request("json_pointer", "/a/x", b"1"),
            request("json_pointer", "/b/x", b"2"),
        ];
        let mut limits = policy();
        limits.parser_allocation_bytes = 10000;
        let results = verify(
            br#"{"a":{"x":1},"b":{"x":2}}"#,
            Format::Json,
            &requests,
            limits,
        )
        .unwrap();
        assert!(results
            .iter()
            .all(|result| matches!(result, Verification::Receipt(ReceiptOutcome::Match(_)))));
    }
}
