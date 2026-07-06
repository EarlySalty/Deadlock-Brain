use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPool;

use crate::queue::Video;

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
    if let Ok(fenced) = Regex::new(r"(?is)```(?:json)?\s*(.*?)```") {
        if let Some(captures) = fenced.captures(&text) {
            if let Some(inner) = captures.get(1) {
                text = inner.as_str().trim().to_string();
            }
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

pub async fn save_claims(
    pool: &PgPool,
    video: &Video,
    claims: &[Claim],
    prompt_text: &str,
    model_response_text: &str,
) -> anyhow::Result<usize> {
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
        let provider_metadata_json = serde_json::to_string(&json!({ "worker": "gemini_browser" }))?;
        sqlx::query!(
            r#"
            INSERT INTO brain.youtube_learning_claims(
              video_id, claim_hash, claim_index, entity_type, entity_name, claim_type,
              claim_text, evidence_quote, timestamp_seconds, model_confidence,
              verifier_confidence, status, model, prompt_version, prompt_text,
              model_response_text, provider_metadata, verifier, created_at, updated_at
            )
            VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17::text::jsonb,$18::text::jsonb,now(),now())
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
              provider_metadata=excluded.provider_metadata,
              verifier=excluded.verifier,
              updated_at=now()
            "#,
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
            provider_metadata_json,
            verifier_json,
        )
        .execute(pool)
        .await?;
        saved += 1;
    }
    Ok(saved)
}

pub async fn query_claims(pool: &PgPool, entity: &str) -> anyhow::Result<Vec<ClaimRow>> {
    let rows = sqlx::query!(
        r#"
        SELECT c.entity_name AS "entity!", c.claim_type, c.claim_text,
               c.model_confidence, v.channel_title,
               v.published_at::text AS "published_at?", v.title, v.url
        FROM brain.youtube_learning_claims c
        JOIN brain.youtube_videos v ON v.video_id=c.video_id
        WHERE lower(c.entity_name)=lower($1)
          AND c.prompt_version=$2
          AND COALESCE(c.model, '')=COALESCE($3, '')
        ORDER BY v.published_at DESC NULLS LAST, c.created_at DESC
        "#,
        entity,
        PROMPT_VERSION,
        MODEL,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| ClaimRow {
            entity: row.entity,
            claim_type: row.claim_type,
            assertion: row.claim_text,
            confidence: row.model_confidence,
            source_channel: row.channel_title,
            published_at: row.published_at,
            video_title: row.title,
            url: row.url,
        })
        .collect())
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

    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn query_claims_returns_rows_for_known_entity_parity() {
        let Some(pool) = crate::testutil::test_pool().await else {
            return;
        };
        // Paritäts-Beweis gegen den Scratch-Snapshot (11941 Claims gesamt).
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM brain.youtube_learning_claims")
            .fetch_one(&pool)
            .await
            .expect("total count");
        assert_eq!(
            total, 11941,
            "youtube_learning_claims Gesamtzahl (Scratch-Snapshot)"
        );

        // query_claims filtert prompt_version=youtube_claims_de_v2 + model=gemini-web.
        let walker = query_claims(&pool, "Walker").await.expect("query walker");
        assert_eq!(walker.len(), 7, "gemini-web/v2 Claims für Entity 'Walker'");
        assert!(walker
            .iter()
            .all(|row| row.entity.eq_ignore_ascii_case("Walker")));
    }
}
