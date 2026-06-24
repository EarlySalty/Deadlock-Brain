use regex::Regex;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::{db, queue::Video};

pub const MODEL: &str = "gemini-web";
pub const PROMPT_VERSION: &str = "youtube_claims_de_v2";

pub const GEMINI_PROMPT_TEMPLATE: &str = r#"Schau dir dieses YouTube-Video von einem Deadlock-Creator genau an: {URL}

Fasse anschließend zusammen, welche konkreten Gameplay-Erkenntnisse der Creator vermittelt – zum Beispiel Item-Builds, Item-Timings, Hero-Matchups, Mechaniken, Combos oder Meta-Einschätzungen. Ignoriere Smalltalk, Intros, Werbung und Spendenaufrufe. Antworte auf Deutsch in klaren Stichpunkten."#;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    pub entity: String,
    pub claim_type: String,
    pub assertion: String,
    pub patch_context: Option<String>,
    pub confidence: f64,
}

pub fn build_prompt(url: &str) -> String {
    GEMINI_PROMPT_TEMPLATE.replace("{URL}", url)
}

pub fn parse_model_claims(response_text: &str) -> Vec<Claim> {
    let mut text = response_text.trim().to_string();
    let fenced = Regex::new(r"(?is)```(?:json)?\s*(.*?)```").expect("valid regex");
    if let Some(captures) = fenced.captures(&text) {
        if let Some(inner) = captures.get(1) {
            text = inner.as_str().trim().to_string();
        }
    }

    let parsed = parse_json_value(&text);
    let Some(value) = parsed else {
        return Vec::new();
    };
    let claims_value = if let Some(claims) = value.get("claims") {
        claims
    } else {
        &value
    };
    let Some(claims) = claims_value.as_array() else {
        return Vec::new();
    };

    claims.iter().filter_map(parse_claim).collect()
}

fn parse_json_value(text: &str) -> Option<Value> {
    serde_json::from_str(text).ok().or_else(|| {
        let start = text.find('{')?;
        let end = text.rfind('}')?;
        if end <= start {
            return None;
        }
        serde_json::from_str(&text[start..=end]).ok()
    })
}

fn parse_claim(value: &Value) -> Option<Claim> {
    let object = value.as_object()?;
    let entity = object.get("entity")?.as_str()?.trim().to_string();
    if entity.is_empty() {
        return None;
    }
    let claim_type = object.get("claim_type")?.as_str()?.trim().to_string();
    if !matches!(
        claim_type.as_str(),
        "build" | "item_timing" | "matchup" | "mechanic" | "combo" | "meta"
    ) {
        return None;
    }
    let assertion = object.get("assertion")?.as_str()?.trim().to_string();
    if assertion.is_empty() {
        return None;
    }
    let patch_context = object
        .get("patch_context")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let confidence = object
        .get("confidence")
        .and_then(|value| {
            value
                .as_f64()
                .or_else(|| value.as_str().and_then(|text| text.parse::<f64>().ok()))
        })
        .unwrap_or(0.0)
        .clamp(0.0, 1.0);
    Some(Claim {
        entity,
        claim_type,
        assertion,
        patch_context,
        confidence,
    })
}

pub fn save_claims(
    conn: &Connection,
    video: &Video,
    claims: &[Claim],
    prompt_text: &str,
    model_response_text: &str,
) -> anyhow::Result<usize> {
    let now = db::now_epoch_seconds();
    let mut saved = 0;
    for (index, claim) in claims.iter().enumerate() {
        let claim_hash = stable_hash_text(&serde_json::to_string(&json!({
            "video_id": &video.video_id,
            "claim_text": &claim.assertion,
            "evidence_quote": "",
            "prompt_version": PROMPT_VERSION,
        }))?);
        let verifier_json = serde_json::to_string(&json!({
            "patch_context": &claim.patch_context,
            "verifier": "not_run",
        }))?;
        conn.execute(
            r#"
            INSERT INTO youtube_learning_claims(
              video_id, claim_hash, claim_index, entity_type, entity_name, claim_type,
              claim_text, evidence_quote, timestamp_seconds, model_confidence,
              verifier_confidence, status, model, prompt_version, prompt_text,
              model_response_text, provider_metadata_json, verifier_json, created_at, updated_at
            )
            VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            ON CONFLICT(claim_hash) DO UPDATE SET
              claim_index=excluded.claim_index,
              entity_type=excluded.entity_type,
              entity_name=excluded.entity_name,
              claim_type=excluded.claim_type,
              claim_text=excluded.claim_text,
              evidence_quote=excluded.evidence_quote,
              timestamp_seconds=excluded.timestamp_seconds,
              model_confidence=excluded.model_confidence,
              verifier_confidence=excluded.verifier_confidence,
              status=excluded.status,
              model_response_text=excluded.model_response_text,
              provider_metadata_json=excluded.provider_metadata_json,
              verifier_json=excluded.verifier_json,
              updated_at=excluded.updated_at
            "#,
            params![
                video.video_id,
                claim_hash,
                index as i64,
                Option::<String>::None,
                claim.entity,
                claim.claim_type,
                claim.assertion,
                "",
                Option::<f64>::None,
                claim.confidence,
                0.0_f64,
                "unverified",
                MODEL,
                PROMPT_VERSION,
                prompt_text,
                model_response_text,
                serde_json::to_string(&json!({ "worker": "gemini_browser" }))?,
                verifier_json,
                now,
                now
            ],
        )?;
        saved += 1;
    }
    Ok(saved)
}

pub fn query_claims(conn: &Connection, entity: &str) -> rusqlite::Result<Vec<ClaimRow>> {
    let mut statement = conn.prepare(
        r#"
        SELECT c.entity_name, c.claim_type, c.claim_text, c.model_confidence,
               v.channel_title, v.published_at, v.title, v.url
        FROM youtube_learning_claims c
        JOIN youtube_videos v ON v.video_id=c.video_id
        WHERE lower(c.entity_name)=lower(?)
          AND c.prompt_version=?
          AND COALESCE(c.model, '')=COALESCE(?, '')
        ORDER BY COALESCE(v.published_at, '') DESC, c.created_at DESC
        "#,
    )?;
    let rows = statement
        .query_map(params![entity, PROMPT_VERSION, MODEL], |row| {
            Ok(ClaimRow {
                entity: row.get(0)?,
                claim_type: row.get(1)?,
                assertion: row.get(2)?,
                confidence: row.get(3)?,
                source_channel: row.get(4)?,
                published_at: row.get(5)?,
                video_title: row.get(6)?,
                url: row.get(7)?,
            })
        })?
        .collect();
    rows
}

#[derive(Debug, Serialize)]
pub struct ClaimRow {
    pub entity: String,
    pub claim_type: String,
    pub assertion: String,
    pub confidence: f64,
    pub source_channel: Option<String>,
    pub published_at: Option<String>,
    pub video_title: String,
    pub url: String,
}

fn stable_hash_text(content: &str) -> String {
    hex::encode(Sha256::digest(content.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_claim_json() {
        let claims = parse_model_claims(
            r#"{"claims":[{"entity":"Bebop","claim_type":"combo","assertion":"Bebop kann Hook in Bomb sicher vorbereiten.","patch_context":null,"confidence":0.8}]}"#,
        );
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].entity, "Bebop");
        assert_eq!(claims[0].claim_type, "combo");
        assert_eq!(claims[0].confidence, 0.8);
    }

    #[test]
    fn parses_markdown_wrapped_json() {
        let claims = parse_model_claims(
            r#"```json
{"claims":[{"entity":"Ivy","claim_type":"mechanic","assertion":"Ivy kann Verbündete mit Air Drop repositionieren.","patch_context":"aktueller Patch","confidence":1.2}]}
```"#,
        );
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].entity, "Ivy");
        assert_eq!(claims[0].patch_context.as_deref(), Some("aktueller Patch"));
        assert_eq!(claims[0].confidence, 1.0);
    }

    #[test]
    fn parses_realistic_two_turn_json_response() {
        let claims = parse_model_claims(
            r#"Hier ist das strukturierte JSON aus der Analyse:

```json
{
  "claims": [
    {
      "entity": "Mo & Krill",
      "claim_type": "item_timing",
      "assertion": "Mo & Krill sollte Slowing Hex frueh kaufen, wenn der Gegner stark auf Mobilitaet spielt.",
      "patch_context": null,
      "confidence": "0.74"
    },
    {
      "entity": "",
      "claim_type": "build",
      "assertion": "Diese unvollstaendige Aussage wird verworfen.",
      "patch_context": null,
      "confidence": 0.5
    }
  ]
}
```"#,
        );
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].entity, "Mo & Krill");
        assert_eq!(claims[0].claim_type, "item_timing");
        assert_eq!(claims[0].patch_context, None);
        assert_eq!(claims[0].confidence, 0.74);
    }

    #[test]
    fn garbage_input_returns_no_claims() {
        assert!(parse_model_claims("kein json").is_empty());
        assert!(parse_model_claims(r#"{"claims":"nope"}"#).is_empty());
        assert!(parse_model_claims(r#"{"claims":[{"entity":"x"}]}"#).is_empty());
    }
}
