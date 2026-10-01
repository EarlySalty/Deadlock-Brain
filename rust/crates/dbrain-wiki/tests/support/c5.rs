#![allow(dead_code)]
//! Synthetic source fixtures only. No real game claims, wiki requests or Lua execution.
use dbrain_s12_wiki_probe::{
    capture::{CaptureOptions, Params},
    knowledge::{FieldMapping, MappingProfile, ValueKind},
    model::*,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub fn options() -> CaptureOptions {
    CaptureOptions {
        source_key: "c5_synthetic_wiki".into(),
        retrieved_at: 1_790_280_000,
        policy: CapturePolicy {
            offline_review_allowed: true,
            decision_ref: Some("C5 synthetic fixture approval".into()),
            source_license: Some("synthetic fixture; no upstream content".into()),
            publication_allowed: false,
            provider_egress_allowed: false,
            media_download_allowed: false,
            raw_retention_allowed: true,
        },
        scope: WikiScope {
            namespace_allowlist: [0, 10].into(),
            page_ids: [2, 3, 4].into(),
            pages: BTreeSet::new(),
            categories: BTreeSet::new(),
            heroes: vec![HeroScope {
                title: "Pilot Hero".into(),
                entity_id: "hero:pilot".into(),
                locale: "en".into(),
                pages: BTreeSet::new(),
            }],
        },
        max_requests: 40,
        max_pages: 4,
        max_total_bytes: 1_000_000,
        request_interval_ms: 0,
        deadline_ms: 10_000,
    }
}
pub fn siteinfo() -> Value {
    json!({"query":{"general":{"lang":"en"},"namespaces":[
        {"id":0,"name":""},{"id":6,"name":"File"},{"id":10,"name":"Template"},{"id":828,"name":"Module"}],
        "statistics":{"pages":85932},"rightsinfo":{"text":"untrusted self-reported rights"}}})
}
pub fn page(id: i64, hero_head: i64, template_head: i64) -> Value {
    let (ns, title, head) = match id {
        1 => (0, "Pilot Hero", hero_head),
        2 => (10, "Template:Stats", template_head),
        3 => (0, "Pilot Alias", 301),
        4 => (0, "Other Hero", 401),
        _ => panic!("out-of-scope ID {id}"),
    };
    let mut p = json!({"pageid":id,"ns":ns,"title":title,"lastrevid":head,"pagelanguage":"en",
        "fullurl":format!("https://wiki.invalid/{title}"),"templates":[],"links":[]});
    if id == 1 {
        p["templates"] = json!([{"ns":10,"title":"Template:Stats"}]);
    }
    if id == 3 {
        p["redirect"] = json!(true);
        p["links"] = json!([{"ns":0,"title":"Pilot Hero"}]);
    }
    let content = |revision| {
        match id {
        1 => json!({"health": if revision >= 102 {"650"} else {"600"},"unit":"health","conditional":"20","condition":"night","variant":"boosted"}).to_string(),
        2 => json!({"damage": if revision >= 202 {"3"} else {"2"},"unit":"damage"}).to_string(),
        3 => "#REDIRECT [[Pilot Hero]]".into(), _ => "{\"health\":\"500\",\"unit\":\"health\"}".into(),
    }
    };
    p["revisions"] = json!([head,head-1].into_iter().map(|revision| json!({"revid":revision,
        "timestamp":"2026-09-20T00:00:00Z","slots":{"main":{"contentmodel":if id==3 {"wikitext"} else {"json"},"content":content(revision)}}})).collect::<Vec<_>>());
    p
}
pub fn reply(p: &Params, hero_head: i64, template_head: i64) -> Value {
    assert_eq!(p["action"], "query");
    assert!(!p.contains_key("redirects"));
    if p.get("meta").map(String::as_str) == Some("siteinfo") {
        return siteinfo();
    }
    assert!(
        !p.contains_key("list"),
        "fixture does not permit allpages or unplanned discovery"
    );
    let id = p
        .get("pageids")
        .map(|s| s.parse().unwrap())
        .unwrap_or_else(|| match p["titles"].as_str() {
            "Pilot Hero" => 1,
            "Template:Stats" => 2,
            "Pilot Alias" => 3,
            "Other Hero" => 4,
            _ => panic!("unexpected title"),
        });
    let mut value = page(id, hero_head, template_head);
    match p["prop"].as_str() {
        "info" => {
            value.as_object_mut().unwrap().remove("revisions");
        }
        "info|revisions" => {}
        "info|templates|links" => {
            value.as_object_mut().unwrap().remove("revisions");
        }
        other => panic!("unexpected prop {other}"),
    }
    json!({"query":{"pages":[value]}})
}
pub fn capture(hero_head: i64, template_head: i64) -> Capture {
    dbrain_s12_wiki_probe::capture::collect(&options(), |p| Ok(reply(p, hero_head, template_head)))
        .unwrap()
}
pub fn bytes(hero_head: i64, template_head: i64) -> Vec<u8> {
    serde_json::to_vec(&capture(hero_head, template_head)).unwrap()
}
pub fn mapping() -> MappingProfile {
    let field = |page_id, predicate: &str, path: &str| FieldMapping {
        page_id,
        predicate: predicate.into(),
        value_locator: path.into(),
        kind: ValueKind::Quantity,
        unit_locator: Some("json:/unit".into()),
        condition_locator: None,
        variant_locator: None,
    };
    let mut conditional = field(1, "conditional_health", "json:/conditional");
    conditional.condition_locator = Some("json:/condition".into());
    conditional.variant_locator = Some("json:/variant".into());
    MappingProfile {
        version: "c5-mapping-v1".into(),
        review_ref: "synthetic mapping only".into(),
        source_key: options().source_key,
        entities: BTreeMap::from([
            (1, "hero:pilot".into()),
            (2, "template:stats".into()),
            (4, "hero:other".into()),
        ]),
        fields: vec![
            field(1, "health", "json:/health"),
            field(1, "unknown_health", "json:/missing"),
            conditional,
            field(2, "damage", "json:/damage"),
            field(4, "health", "json:/health"),
        ],
    }
}
