use serde::Serialize;
use serde_json::{json, Value};
use sqlx::postgres::PgPool;

const VISUAL_KEYWORDS: &[&str] = &[
    "wall jump",
    "walljump",
    "movement",
    "tech",
    "combo",
    "mantle",
    "animation cancel",
    "dash",
    "slide",
    "parry",
    "mechanics demo",
    "vent boost",
    "ledge",
];

const VERBAL_KEYWORDS: &[&str] = &[
    "build",
    "items",
    "item",
    "lane",
    "laning",
    "matchup",
    "counter",
    "souls",
    "macro",
    "guide",
    "tips",
    "rank",
    "strategy",
    "meta",
    "economy",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    VisualTech,
    VerbalStrategy,
}

impl ContentType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VisualTech => "visual_tech",
            Self::VerbalStrategy => "verbal_strategy",
        }
    }

    fn transcript_low_value(self) -> bool {
        matches!(self, Self::VisualTech)
    }
}

#[derive(Debug, Serialize)]
pub struct ClassificationSummary {
    pub selected: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub videos: Vec<ClassifiedVideoSummary>,
}

#[derive(Debug, Serialize)]
pub struct ClassifiedVideoSummary {
    pub video_id: String,
    pub title: String,
    pub content_type: String,
    pub transcript_low_value: bool,
    pub status: String,
}

#[derive(Debug)]
struct VideoForClassification {
    video_id: String,
    title: String,
    metadata_json: String,
    transcript_text: Option<String>,
}

pub async fn classify_videos(pool: &PgPool, limit: usize) -> anyhow::Result<ClassificationSummary> {
    let videos = select_videos(pool, limit).await?;
    let mut summary = ClassificationSummary {
        selected: videos.len(),
        updated: 0,
        unchanged: 0,
        videos: Vec::new(),
    };

    for video in videos {
        let content_type = classify_content_type(&video.title, video.transcript_text.as_deref());
        let updated = update_video_metadata(pool, &video, content_type).await?;
        if updated {
            summary.updated += 1;
        } else {
            summary.unchanged += 1;
        }
        summary.videos.push(ClassifiedVideoSummary {
            video_id: video.video_id,
            title: video.title,
            content_type: content_type.as_str().to_string(),
            transcript_low_value: content_type.transcript_low_value(),
            status: if updated {
                "updated".to_string()
            } else {
                "unchanged".to_string()
            },
        });
    }

    Ok(summary)
}

async fn select_videos(
    pool: &PgPool,
    limit: usize,
) -> anyhow::Result<Vec<VideoForClassification>> {
    let limit = i64::try_from(limit.max(1)).unwrap_or(i64::MAX);
    let rows = sqlx::query!(
        r#"
        SELECT v.video_id, v.title, v.metadata::text AS "metadata_json!",
               t.transcript_text AS "transcript_text?"
        FROM brain.youtube_videos v
        LEFT JOIN brain.youtube_transcripts t ON t.video_id=v.video_id
        ORDER BY
          CASE WHEN v.metadata->>'content_type' IS NOT NULL THEN 1 ELSE 0 END,
          v.published_at DESC NULLS LAST,
          v.discovered_at DESC
        LIMIT $1
        "#,
        limit,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| VideoForClassification {
            video_id: row.video_id,
            title: row.title,
            metadata_json: row.metadata_json,
            transcript_text: row.transcript_text,
        })
        .collect())
}

pub fn classify_content_type(title: &str, transcript_text: Option<&str>) -> ContentType {
    let title_normalized = normalize_for_keywords(title);
    let transcript_normalized = normalize_for_keywords(transcript_text.unwrap_or_default());
    let visual_score = weighted_score(&title_normalized, &transcript_normalized, VISUAL_KEYWORDS);
    let verbal_score = weighted_score(&title_normalized, &transcript_normalized, VERBAL_KEYWORDS);
    if visual_score > 0 && visual_score >= verbal_score {
        ContentType::VisualTech
    } else {
        ContentType::VerbalStrategy
    }
}

fn weighted_score(title: &str, transcript: &str, keywords: &[&str]) -> usize {
    keywords
        .iter()
        .map(|keyword| keyword_score(title, keyword) * 3 + keyword_score(transcript, keyword))
        .sum()
}

fn keyword_score(text: &str, keyword: &str) -> usize {
    let normalized_keyword = normalize_for_keywords(keyword);
    if normalized_keyword.contains(' ') {
        let haystack = format!(" {text} ");
        let needle = format!(" {normalized_keyword} ");
        if haystack.contains(&needle) {
            return 1;
        }
        return 0;
    }

    text.split_whitespace()
        .filter(|token| {
            *token == normalized_keyword
                || token.ends_with(&normalized_keyword)
                || token.starts_with(&normalized_keyword)
        })
        .count()
}

fn normalize_for_keywords(text: &str) -> String {
    let mut normalized = String::new();
    for character in text.chars().flat_map(|character| character.to_lowercase()) {
        if character.is_ascii_alphanumeric() {
            normalized.push(character);
        } else {
            normalized.push(' ');
        }
    }
    normalized.split_whitespace().collect::<Vec<_>>().join(" ")
}

async fn update_video_metadata(
    pool: &PgPool,
    video: &VideoForClassification,
    content_type: ContentType,
) -> anyhow::Result<bool> {
    let mut metadata =
        serde_json::from_str::<Value>(&video.metadata_json).unwrap_or_else(|_| json!({}));
    if !metadata.is_object() {
        metadata = json!({});
    }
    let desired_content_type = content_type.as_str();
    let desired_low_value = content_type.transcript_low_value();
    let already_current = metadata
        .get("content_type")
        .and_then(Value::as_str)
        .map(|value| value == desired_content_type)
        .unwrap_or(false)
        && metadata
            .get("transcript_low_value")
            .and_then(Value::as_bool)
            .map(|value| value == desired_low_value)
            .unwrap_or(false);
    if already_current {
        return Ok(false);
    }
    metadata["content_type"] = Value::String(desired_content_type.to_string());
    metadata["transcript_low_value"] = Value::Bool(desired_low_value);
    let metadata_json = serde_json::to_string(&metadata)?;
    sqlx::query!(
        "UPDATE brain.youtube_videos SET metadata=$1::text::jsonb, updated_at=now() WHERE video_id=$2",
        metadata_json,
        video.video_id,
    )
    .execute(pool)
    .await?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heuristic_classifies_visual_and_verbal_titles() {
        assert_eq!(
            classify_content_type(
                "HOW TO PERFORM VENT BOOST EVERY TIME #deadlock #deadlocktips",
                None
            ),
            ContentType::VisualTech
        );
        assert_eq!(
            classify_content_type("Wall-Jump tutorial for every Deadlock hero", None),
            ContentType::VisualTech
        );
        assert_eq!(
            classify_content_type(
                "Best Pocket build and laning guide for ranked games",
                Some("This video explains items, souls, macro strategy and lane matchups.")
            ),
            ContentType::VerbalStrategy
        );
    }
}
