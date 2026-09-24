//! Bounded, network-free S12 preparation. Nothing here publishes knowledge or
//! implements a substitute for the unapproved S02/S03/S05 runtime contracts.
mod discovery;
mod impact;
pub mod model;
mod pages;
mod preview;
pub use impact::compare;
use model::*;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub type Result<T> = std::result::Result<T, String>;

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

// serde_json::Value alone silently accepts duplicate keys. Reject them at every
// depth before interpreting policies, API captures or structured source content.
struct UniqueJson(Value);
impl<'de> Deserialize<'de> for UniqueJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct UniqueVisitor;
        impl<'de> Visitor<'de> for UniqueVisitor {
            type Value = UniqueJson;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("JSON with unique object keys")
            }
            fn visit_bool<E: serde::de::Error>(
                self,
                value: bool,
            ) -> std::result::Result<Self::Value, E> {
                Ok(UniqueJson(value.into()))
            }
            fn visit_i64<E: serde::de::Error>(
                self,
                value: i64,
            ) -> std::result::Result<Self::Value, E> {
                Ok(UniqueJson(value.into()))
            }
            fn visit_u64<E: serde::de::Error>(
                self,
                value: u64,
            ) -> std::result::Result<Self::Value, E> {
                Ok(UniqueJson(value.into()))
            }
            fn visit_f64<E: serde::de::Error>(
                self,
                value: f64,
            ) -> std::result::Result<Self::Value, E> {
                serde_json::Number::from_f64(value)
                    .map(|n| UniqueJson(Value::Number(n)))
                    .ok_or_else(|| E::custom("non-finite JSON number"))
            }
            fn visit_str<E: serde::de::Error>(
                self,
                value: &str,
            ) -> std::result::Result<Self::Value, E> {
                Ok(UniqueJson(value.into()))
            }
            fn visit_string<E: serde::de::Error>(
                self,
                value: String,
            ) -> std::result::Result<Self::Value, E> {
                Ok(UniqueJson(value.into()))
            }
            fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Self::Value, E> {
                Ok(UniqueJson(Value::Null))
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> std::result::Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(UniqueJson(value)) = seq.next_element()? {
                    values.push(value);
                }
                Ok(UniqueJson(Value::Array(values)))
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> std::result::Result<Self::Value, A::Error> {
                let mut values = serde_json::Map::new();
                while let Some((key, UniqueJson(value))) = map.next_entry::<String, UniqueJson>()? {
                    if values.insert(key, value).is_some() {
                        return Err(serde::de::Error::custom("duplicate JSON key"));
                    }
                }
                Ok(UniqueJson(Value::Object(values)))
            }
        }
        deserializer.deserialize_any(UniqueVisitor)
    }
}

pub fn parse_json(bytes: &[u8]) -> Result<Value> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("input byte budget exceeded".into());
    }
    serde_json::from_slice::<UniqueJson>(bytes)
        .map(|value| value.0)
        .map_err(|_| "invalid, duplicate-key or over-depth JSON".into())
}

pub fn analyze(bytes: &[u8]) -> Result<Report> {
    let capture: Capture = serde_json::from_value(parse_json(bytes)?)
        .map_err(|_| "invalid capture schema (s12-capture-v1 required)".to_string())?;
    if capture.format != CAPTURE_VERSION {
        return Err("unsupported capture version".into());
    }
    if capture.source_key.is_empty()
        || capture.source_key.len() > 128
        || !capture
            .source_key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err("source_key must be a stable ASCII identifier".into());
    }
    if capture.retrieved_at <= 0 {
        return Err("retrieved_at must be a positive Unix timestamp".into());
    }
    if !capture.policy.offline_review_allowed
        || capture
            .policy
            .decision_ref
            .as_ref()
            .is_none_or(|v| v.trim().is_empty())
    {
        return Err("offline review not approved: explicit decision required".into());
    }
    if capture.pages.len() > MAX_PAGES
        || capture.discovery.len() > MAX_PAGES
        || capture.classification.len() > MAX_PAGES
        || capture.hero_bindings.len() > 32
    {
        return Err("capture entry budget exceeded".into());
    }
    let mut discovered = discovery::discover(&capture)?;
    pages::inspect(&capture, &mut discovered.pages)?;
    let mut coverage = BTreeMap::<PageKind, Coverage>::new();
    let mut blockers: BTreeSet<String> = [
        "G0_not_accepted",
        "G1_contracts_not_integrated",
        "canonical_fact_rule_card_ports_missing",
        "live_store_worker_domain_api_pilot_not_verified",
        "offline_probe_not_a_publisher",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    let complete = discovered.missing.is_empty()
        && discovered.pending.is_empty()
        && discovered.diagnostics.is_empty();
    if !complete {
        blockers.insert("discovery_incomplete".into());
    }
    if capture
        .policy
        .source_license
        .as_ref()
        .is_none_or(|v| v.trim().is_empty())
    {
        blockers.insert("source_license_not_recorded".into());
    }
    if !capture.policy.publication_allowed {
        blockers.insert("publication_not_approved".into());
    }
    for page in discovered.pages.values() {
        let counts = coverage.entry(page.kind).or_default();
        counts.discovered += 1;
        counts.fetched += usize::from(page.revisions.iter().any(|r| r.raw_sha256.is_some()));
        counts.parsed += usize::from(page.stage == Stage::Parsed);
        match page.stage {
            Stage::Quarantined => counts.quarantined += 1,
            Stage::Unavailable => counts.unavailable += 1,
            Stage::PolicyBlocked => counts.policy_blocked += 1,
            Stage::ApprovedExclusion => counts.approved_exclusion += 1,
            _ => {}
        }
        if page.build_critical {
            counts.build_critical_unvalidated += 1;
            blockers.insert("build_critical_facts_unvalidated".into());
        }
        if page.kind == PageKind::Unclassified {
            blockers.insert("unclassified_pages".into());
        }
        if !page.dependencies_complete && page.stage != Stage::ApprovedExclusion {
            blockers.insert("dependency_manifest_incomplete".into());
        }
    }
    let card_previews = preview::project(&capture.hero_bindings, &discovered.pages)?;
    Ok(Report {
        source_policy: capture.policy,
        site_default_language: capture
            .siteinfo
            .pointer("/query/general/lang")
            .and_then(Value::as_str)
            .map(str::to_string),
        report_version: "s12-offline-report-v1".into(),
        parser_version: PARSER_VERSION.into(),
        capture_sha256: sha256(bytes),
        source_key: capture.source_key,
        retrieved_at: capture.retrieved_at,
        discovery_complete: complete,
        namespace_names: discovered.namespaces,
        missing_namespaces: discovered.missing,
        continuation_pending: discovered.pending,
        pages: discovered.pages.into_values().collect(),
        coverage,
        card_previews,
        diagnostics: discovered.diagnostics,
        blockers,
        mode: "prepare_only".into(),
        integration_verified: false,
        production_publishable: false,
    })
}
