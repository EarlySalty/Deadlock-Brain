use anyhow::{ensure, Context};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub const PARSER_VERSION: &str = "json3_timed_v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Segment {
    pub event_index: usize,
    pub text: String,
    pub start_ms: Option<u64>,
    pub duration_ms: Option<u64>,
    pub end_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct CaptionEvidence {
    pub text: String,
    pub raw: Value,
    pub segments: Vec<Segment>,
    pub timing_status: &'static str,
}

#[derive(Deserialize)]
struct Root {
    #[serde(default)]
    events: Vec<Event>,
}

#[derive(Deserialize)]
struct Event {
    #[serde(rename = "tStartMs")]
    start_ms: Option<u64>,
    #[serde(rename = "dDurationMs")]
    duration_ms: Option<u64>,
    #[serde(default)]
    segs: Vec<TextSegment>,
}

#[derive(Deserialize)]
struct TextSegment {
    utf8: Option<String>,
}

pub fn parse(raw: &str) -> anyhow::Result<CaptionEvidence> {
    let raw: Value = serde_json::from_str(raw).context("JSON3 lesen")?;
    let root: Root = serde_json::from_value(raw.clone())?;
    let mut segments = Vec::new();
    for (event_index, event) in root.events.into_iter().enumerate() {
        let event_text: String = event.segs.into_iter().filter_map(|s| s.utf8).collect();
        let text = event_text.split_whitespace().collect::<Vec<_>>().join(" ");
        if text.is_empty() { continue; }
        let end_ms = match (event.start_ms, event.duration_ms) {
            (Some(start), Some(duration)) => Some(start.checked_add(duration).context("Zeitmarke läuft über")?),
            _ => None,
        };
        segments.push(Segment {event_index, text, start_ms:event.start_ms,
            duration_ms:event.duration_ms,end_ms});
    }
    ensure!(!segments.is_empty(),"JSON3 enthält keinen Text.");
    let text = segments.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(" ");
    let timed = segments.iter().filter(|s| s.start_ms.is_some() && s.end_ms.is_some()).count();
    let timing_status = if timed == segments.len() { "available" }
        else if segments.iter().any(|s| s.start_ms.is_some()) { "partial" } else { "missing" };
    Ok(CaptionEvidence {text, raw, segments, timing_status})
}

pub fn evidence_hash(evidence: &CaptionEvidence, language: &str, source_kind: &str) -> anyhow::Result<String> {
    let payload = serde_json::json!({"parser_version":PARSER_VERSION,"language":language,
        "source_kind":source_kind,"raw":evidence.raw,"segments":evidence.segments});
    Ok(hex::encode(Sha256::digest(serde_json::to_vec(&payload)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retains_zero_start_duration_offsets_and_raw_metadata() {
        let result = parse(r#"{"events":[{"tStartMs":0,"dDurationMs":1250,"segs":[{"utf8":"Hello","tOffsetMs":0},{"utf8":" world","tOffsetMs":80}]}],"extra":"preserved"}"#).unwrap();
        assert_eq!(result.segments[0].start_ms,Some(0));
        assert_eq!(result.segments[0].end_ms,Some(1250));
        assert_eq!(result.raw["extra"],"preserved");
        assert_eq!(result.raw["events"][0]["segs"][1]["tOffsetMs"],80);
        assert_eq!(result.timing_status,"available");
    }
    #[test]
    fn never_invents_timestamps_for_pasted_or_legacy_text() {
        let result=parse(r#"{"events":[{"segs":[{"utf8":"Ohne Zeitmarken"}]}]}"#).unwrap();
        assert_eq!(result.timing_status,"missing");
        assert_eq!(result.segments[0].start_ms,None);
        assert_eq!(result.segments[0].end_ms,None);
    }
    #[test]
    fn preserves_source_text_including_names_and_fragmented_words() {
        let result=parse(r#"{"events":[{"segs":[{"utf8":"Merc"},{"utf8":" Mac"},{"utf8":" doesn't"}]},{"segs":[{"utf8":"Änderung"}]}]}"#).unwrap();
        assert_eq!(result.text,"Merc Mac doesn't Änderung");
    }
    #[test]
    fn rejects_negative_and_overflowing_timestamps() {
        assert!(parse(r#"{"events":[{"tStartMs":-1,"segs":[{"utf8":"x"}]}]}"#).is_err());
        assert!(parse(r#"{"events":[{"tStartMs":18446744073709551615,"dDurationMs":1,"segs":[{"utf8":"x"}]}]}"#).is_err());
    }
    #[test]
    fn timing_only_revision_changes_evidence_hash() {
        let a=parse(r#"{"events":[{"tStartMs":1,"dDurationMs":10,"segs":[{"utf8":"same"}]}]}"#).unwrap();
        let b=parse(r#"{"events":[{"tStartMs":2,"dDurationMs":10,"segs":[{"utf8":"same"}]}]}"#).unwrap();
        assert_eq!(a.text,b.text);
        assert_ne!(evidence_hash(&a,"en","auto").unwrap(),evidence_hash(&b,"en","auto").unwrap());
    }
    #[test]
    fn overlapping_captions_are_kept_in_source_order() {
        let result=parse(r#"{"events":[{"tStartMs":1000,"dDurationMs":1000,"segs":[{"utf8":"one"}]},{"tStartMs":1500,"dDurationMs":1000,"segs":[{"utf8":"two"}]}]}"#).unwrap();
        assert_eq!(result.segments[1].start_ms,Some(1500));
        assert_eq!(result.text,"one two");
    }
    #[test]
    fn partial_timing_is_explicit() {
        let result=parse(r#"{"events":[{"tStartMs":0,"segs":[{"utf8":"partial"}]}]}"#).unwrap();
        assert_eq!(result.timing_status,"partial");
    }
    #[test]
    fn empty_caption_is_rejected() {
        assert!(parse(r#"{"events":[{"segs":[{"utf8":"\n"}]}]}"#).is_err());
    }
}
