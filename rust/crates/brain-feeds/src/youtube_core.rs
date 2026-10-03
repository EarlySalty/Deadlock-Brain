use crate::{
    sha256_hex,
    source_sync::{origin, SourceRights},
    FeedError, Result,
};
use brain_contracts::{source::SourceTimestamp, value::Observed, SourceBatch, SourceCheckpoint};
use brain_ingestion::document_set::{
    prepare_document_batch, CoreDocument, DocumentSetSource, MAX_DOCUMENTS_PER_SOURCE,
    MAX_DOCUMENT_BYTES,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::collections::BTreeMap;

pub const PARSER: &str = "brain-youtube-existing.v1";

pub mod metadata;

pub fn prepare_fresh_batch(
    source: &YoutubeSource,
    verified: &metadata::VerifiedMetadata,
    observed_at: i64,
    previous: Option<&SourceCheckpoint>,
) -> Result<SourceBatch> {
    if !verified.matches(source, previous) {
        return Err(FeedError::Invalid(
            "YouTube-Prüfung gehört nicht zu dieser Quelle und diesem Ausgangsstand.".into(),
        ));
    }
    prepare_batch_inner(
        source,
        &verified.videos,
        true,
        observed_at,
        previous,
        Some(&verified.raw),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Retention {
    #[default]
    Disabled,
    MetadataOnly,
    TranscriptAndMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct YoutubeSource {
    pub channel_id: String,
    #[serde(default)]
    pub retention: Retention,
    pub rights: SourceRights,
}

impl YoutubeSource {
    pub fn validate(&self) -> Result<()> {
        self.rights.validate()?;
        if self.retention == Retention::Disabled
            || self.channel_id.len() != 24
            || !self.channel_id.starts_with("UC")
            || !self
                .channel_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
        {
            return Err(FeedError::Invalid(
                "YouTube-Quelle oder Aufbewahrung ist nicht freigegeben.".into(),
            ));
        }
        Ok(())
    }

    pub fn source_id(&self) -> String {
        format!("youtube-core/{}", self.channel_id)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ExistingVideo {
    pub video_id: String,
    pub channel_id: String,
    pub title: String,
    pub description: Option<String>,
    pub published_at: Option<i64>,
    pub language: Option<String>,
    pub transcript: Option<String>,
    pub transcript_kind: Option<String>,
    pub transcript_hash: Option<String>,
}

pub async fn read_existing(pool: &PgPool, source: &YoutubeSource) -> Result<Vec<ExistingVideo>> {
    source.validate()?;
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| FeedError::Invalid("Lesende YouTube-Verbindung fehlt.".into()))?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(|_| FeedError::Invalid("YouTube-Lesevertrag fehlt.".into()))?;
    let transcripts = source.retention == Retention::TranscriptAndMetadata;
    let rows = sqlx::query("SELECT v.video_id,v.channel_id,v.title,v.description,extract(epoch FROM v.published_at)::bigint AS published_at,CASE WHEN $2 THEN t.language ELSE NULL END AS language,CASE WHEN $2 THEN t.transcript_text ELSE NULL END AS transcript,CASE WHEN $2 THEN t.source_kind ELSE NULL END AS transcript_kind,CASE WHEN $2 THEN t.content_hash ELSE NULL END AS transcript_hash FROM brain.youtube_videos v LEFT JOIN brain.youtube_transcripts t ON t.video_id=v.video_id WHERE v.channel_id=$1 ORDER BY v.video_id LIMIT $3")
        .bind(&source.channel_id).bind(transcripts).bind((MAX_DOCUMENTS_PER_SOURCE + 1) as i64)
        .fetch_all(&mut *tx).await.map_err(|_| FeedError::Invalid("Vollständiger YouTube-Bestand ist nicht lesbar.".into()))?;
    if rows.len() > MAX_DOCUMENTS_PER_SOURCE {
        return Err(FeedError::Invalid(
            "YouTube-Menge überschreitet die Grenze. Kein Batch wurde erzeugt.".into(),
        ));
    }
    let mut videos = Vec::with_capacity(rows.len());
    for row in rows {
        let read = || -> std::result::Result<ExistingVideo, sqlx::Error> {
            Ok(ExistingVideo {
                video_id: row.try_get("video_id")?,
                channel_id: row.try_get("channel_id")?,
                title: row.try_get("title")?,
                description: row.try_get("description")?,
                published_at: row.try_get("published_at")?,
                language: row.try_get("language")?,
                transcript: row.try_get("transcript")?,
                transcript_kind: row.try_get("transcript_kind")?,
                transcript_hash: row.try_get("transcript_hash")?,
            })
        };
        videos.push(
            read().map_err(|_| {
                FeedError::Invalid("YouTube-Zeile verletzt den Lesevertrag.".into())
            })?,
        );
    }
    tx.commit()
        .await
        .map_err(|_| FeedError::Invalid("YouTube-Lesetransaktion wurde unterbrochen.".into()))?;
    Ok(videos)
}

pub fn prepare_batch(
    source: &YoutubeSource,
    videos: &[ExistingVideo],
    complete: bool,
    observed_at: i64,
    previous: Option<&SourceCheckpoint>,
) -> Result<SourceBatch> {
    prepare_batch_inner(source, videos, complete, observed_at, previous, None)
}

fn prepare_batch_inner(
    source: &YoutubeSource,
    videos: &[ExistingVideo],
    complete: bool,
    observed_at: i64,
    previous: Option<&SourceCheckpoint>,
    fresh: Option<&BTreeMap<String, serde_json::Value>>,
) -> Result<SourceBatch> {
    source.validate()?;
    if !complete || observed_at <= 0 || videos.len() > MAX_DOCUMENTS_PER_SOURCE {
        return Err(FeedError::Invalid(
            "YouTube-Menge ist unvollständig.".into(),
        ));
    }
    let source_id = source.source_id();
    let parser = if fresh.is_some() {
        "brain-youtube-data-api.v1"
    } else {
        PARSER
    };
    let mut documents = Vec::new();
    for video in videos {
        if video.channel_id != source.channel_id
            || video.video_id.len() != 11
            || !video
                .video_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
            || video.title.trim().is_empty()
        {
            return Err(FeedError::Invalid(
                "YouTube-Identität stimmt nicht mit der Quelle überein.".into(),
            ));
        }
        let mut retained = video.clone();
        if source.retention == Retention::MetadataOnly {
            retained.transcript = None;
            retained.transcript_kind = None;
            retained.transcript_hash = None;
            retained.language = None;
        } else if let Some(text) = &retained.transcript {
            if text.trim().is_empty()
                || retained.transcript_hash.as_deref() != Some(sha256_hex(text.as_bytes()).as_str())
                || retained
                    .transcript_kind
                    .as_ref()
                    .is_none_or(|kind| kind.trim().is_empty())
            {
                return Err(FeedError::Invalid(
                    "YouTube-Transkript hat keinen gültigen Herkunftsbeleg.".into(),
                ));
            }
        }
        let bytes = if let Some(raw) = fresh {
            serde_json::to_vec(&serde_json::json!({
                "video": retained,
                "youtube_data_api_v3": raw.get(&video.video_id).ok_or_else(|| FeedError::Invalid("Frische YouTube-Metadaten fehlen.".into()))?,
            }))?
        } else {
            serde_json::to_vec(&retained)?
        };
        if bytes.len() > MAX_DOCUMENT_BYTES {
            return Err(FeedError::Invalid(
                "YouTube-Dokument ist zu groß. Kein gekürzter Batch wird geschrieben.".into(),
            ));
        }
        let hash = sha256_hex(&bytes);
        let logical = format!("video/{}", video.video_id);
        let url = format!("https://www.youtube.com/watch?v={}", video.video_id);
        let mut provenance = origin(
            &source_id,
            &logical,
            &url,
            parser,
            &hash,
            observed_at,
            &source.rights,
        );
        if let Some(time) = video.published_at {
            provenance.source_time = Observed::known(SourceTimestamp::UnixSeconds(time));
        }
        if let Some(language) = &retained.language {
            provenance.language = Observed::known(language.clone());
        }
        documents.push(CoreDocument {
            logical_id: logical,
            content: String::from_utf8(bytes)
                .map_err(|_| FeedError::Invalid("YouTube-Dokument ist nicht UTF-8.".into()))?,
            metadata: BTreeMap::from([
                ("connector".into(), "youtube-core".into()),
                ("kind".into(), "prose".into()),
                ("locator".into(), url),
                ("title".into(), video.title.clone()),
                ("channel_id".into(), source.channel_id.clone()),
                ("raw_sha256".into(), hash),
            ]),
            origin: provenance,
        });
    }
    let set = DocumentSetSource {
        source_id,
        configuration: sha256_hex(&serde_json::to_vec(&(parser, source))?),
        visibility: source.rights.policy.visibility,
        allowed_scopes: source.rights.policy.allowed_scopes.clone(),
        tombstone_metadata: BTreeMap::from([
            ("connector".into(), "youtube-core".into()),
            (
                "removal_basis".into(),
                if fresh.is_some() {
                    "not_retrievable_via_youtube_data_api_v3"
                } else {
                    "absent_from_supplied_set"
                }
                .into(),
            ),
        ]),
    };
    if fresh.is_some() {
        Ok(brain_ingestion::document_set::prepare_confirmed_document_batch(
            brain_ingestion::document_set::ConfirmedCompleteRead::after_complete_source_verification(&set, &documents),
            previous,
        )?)
    } else {
        Ok(prepare_document_batch(&set, &documents, previous)?)
    }
}
