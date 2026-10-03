use super::{ExistingVideo, Retention, YoutubeSource};
use crate::{sha256_hex, FeedError, Result};
use brain_contracts::SourceCheckpoint;
use brain_ingestion::document_set::{current_pins, MAX_DOCUMENTS_PER_SOURCE, MAX_DOCUMENT_BYTES};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    time::Duration,
};
use zeroize::Zeroizing;

#[derive(serde::Serialize)]
pub struct MetadataProof {
    pub uploads: usize,
    pub upload_pages_first: usize,
    pub upload_pages_confirmation: usize,
    pub probed_video_ids: usize,
    pub available_videos: usize,
    pub unavailable_videos: usize,
    pub terminal_pages_verified: bool,
}

pub struct VerifiedMetadata {
    proof: MetadataProof,
    source_hash: String,
    checkpoint_hash: String,
    pub(super) videos: Vec<ExistingVideo>,
    pub(super) raw: BTreeMap<String, Value>,
    unavailable: BTreeSet<String>,
}

impl VerifiedMetadata {
    pub fn proof(&self) -> &MetadataProof {
        &self.proof
    }

    pub fn unavailable_video_ids(&self) -> &BTreeSet<String> {
        &self.unavailable
    }

    pub(super) fn matches(
        &self,
        source: &YoutubeSource,
        previous: Option<&SourceCheckpoint>,
    ) -> bool {
        fingerprint(source).ok().as_ref() == Some(&self.source_hash)
            && fingerprint(&previous).ok().as_ref() == Some(&self.checkpoint_hash)
    }
}

fn fingerprint(value: &impl serde::Serialize) -> Result<String> {
    Ok(sha256_hex(&serde_json::to_vec(value)?))
}

fn invalid() -> FeedError {
    FeedError::Invalid("YouTube-Metadatenprüfung ist unvollständig oder widersprüchlich. Kein Batch wird geschrieben.".into())
}

fn text<'a>(value: &'a Value, path: &str) -> Result<&'a str> {
    value
        .pointer(path)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .ok_or_else(invalid)
}

fn published_at(value: &str) -> Result<i64> {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return Err(invalid());
    }
    let number = |start: usize, end: usize| -> Result<i64> {
        let part = &bytes[start..end];
        if !part.iter().all(u8::is_ascii_digit) {
            return Err(invalid());
        }
        Ok(part.iter().fold(0, |n, b| n * 10 + i64::from(b - b'0')))
    };
    let year = number(0, 4)?;
    let month = number(5, 7)?;
    let day = number(8, 10)?;
    let hour = number(11, 13)?;
    let minute = number(14, 16)?;
    let second = number(17, 19)?;
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => return Err(invalid()),
    };
    if year < 1970 || day < 1 || day > days || hour > 23 || minute > 59 || second > 59 {
        return Err(invalid());
    }
    let y = year - i64::from(month <= 2);
    let era = y / 400;
    let yoe = y - era * 400;
    let m = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * m + 2) / 5 + day - 1;
    let epoch_days = era * 146097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719468;
    Ok(epoch_days * 86400 + hour * 3600 + minute * 60 + second)
}

fn valid_id(id: &str, len: usize) -> bool {
    id.len() == len
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}

fn page<'a>(value: &'a Value, kind: &str) -> Result<(&'a Vec<Value>, usize, Option<&'a str>)> {
    if text(value, "/kind")? != kind || value.get("error").is_some() {
        return Err(invalid());
    }
    let items = value["items"].as_array().ok_or_else(invalid)?;
    let total = value
        .pointer("/pageInfo/totalResults")
        .and_then(Value::as_u64)
        .ok_or_else(invalid)?;
    let per_page = value
        .pointer("/pageInfo/resultsPerPage")
        .and_then(Value::as_u64)
        .ok_or_else(invalid)?;
    if total > MAX_DOCUMENTS_PER_SOURCE as u64
        || items.len() > 50
        || per_page > 50
        || items.len() as u64 > per_page
    {
        return Err(invalid());
    }
    let next = match value.get("nextPageToken") {
        None => None,
        Some(Value::String(s))
            if !s.is_empty() && s.len() <= 1024 && !s.chars().any(char::is_control) =>
        {
            Some(s.as_str())
        }
        _ => return Err(invalid()),
    };
    Ok((items, total as usize, next))
}

pub async fn fetch_complete(
    source: &YoutubeSource,
    existing: &[ExistingVideo],
    previous: Option<&SourceCheckpoint>,
    api_key: &str,
    timeout_seconds: u64,
) -> Result<VerifiedMetadata> {
    source.validate()?;
    if api_key.trim().is_empty() || api_key.chars().any(char::is_control) || timeout_seconds == 0 {
        return Err(FeedError::Invalid("YouTube-Metadatenzugang fehlt.".into()));
    }
    let key = Zeroizing::new(api_key.to_owned());
    let client = reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(timeout_seconds))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| invalid())?;
    collect(source, existing, previous, |resource, query| {
        let client = &client;
        let key = key.as_str();
        async move {
            let url = format!("https://www.googleapis.com/youtube/v3/{resource}");
            let response = client
                .get(url)
                .query(&query)
                .header("X-Goog-Api-Key", key)
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(|_| invalid())?;
            read_response(response).await
        }
    })
    .await
}

async fn read_response(mut response: reqwest::Response) -> Result<Value> {
    if response.status() != reqwest::StatusCode::OK
        || !response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| {
                v.split(';')
                    .next()
                    .is_some_and(|v| v.trim() == "application/json")
            })
    {
        return Err(invalid());
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| invalid())? {
        if bytes.len().saturating_add(chunk.len()) > MAX_DOCUMENT_BYTES {
            return Err(invalid());
        }
        bytes.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&bytes).map_err(|_| invalid())
}

type Query = Vec<(String, String)>;
fn query(pairs: &[(&str, &str)]) -> Query {
    pairs
        .iter()
        .map(|(k, v)| ((*k).into(), (*v).into()))
        .collect()
}

async fn uploads<F, Fut>(
    channel: &str,
    playlist: &str,
    get: &mut F,
) -> Result<(BTreeSet<String>, usize)>
where
    F: FnMut(&'static str, Query) -> Fut,
    Fut: Future<Output = Result<Value>>,
{
    let mut ids = BTreeSet::new();
    let mut tokens = BTreeSet::new();
    let mut token = None;
    let mut expected = None;
    let mut pages = 0;
    loop {
        let mut params = query(&[
            ("part", "snippet,contentDetails"),
            ("playlistId", playlist),
            ("maxResults", "50"),
        ]);
        if let Some(token) = token.take() {
            params.push(("pageToken".into(), token));
        }
        let body = get("playlistItems", params).await?;
        pages += 1;
        let (items, total, next) = page(&body, "youtube#playlistItemListResponse")?;
        if expected.is_some_and(|n| n != total) {
            return Err(invalid());
        }
        expected = Some(total);
        for item in items {
            let id = text(item, "/contentDetails/videoId")?;
            if text(item, "/kind")? != "youtube#playlistItem"
                || text(item, "/snippet/playlistId")? != playlist
                || text(item, "/snippet/channelId")? != channel
                || text(item, "/snippet/resourceId/kind")? != "youtube#video"
                || text(item, "/snippet/resourceId/videoId")? != id
                || !valid_id(id, 11)
                || !ids.insert(id.to_owned())
                || ids.len() > total
            {
                return Err(invalid());
            }
            if item.pointer("/snippet/videoOwnerChannelId").is_some()
                && text(item, "/snippet/videoOwnerChannelId")? != channel
            {
                return Err(invalid());
            }
        }
        match next {
            Some(next)
                if !items.is_empty() && ids.len() < total && tokens.insert(next.to_owned()) =>
            {
                token = Some(next.to_owned())
            }
            Some(_) => return Err(invalid()),
            None if ids.len() == total => return Ok((ids, pages)),
            None => return Err(invalid()),
        }
    }
}

async fn channel_playlist<F, Fut>(source: &YoutubeSource, get: &mut F) -> Result<String>
where
    F: FnMut(&'static str, Query) -> Fut,
    Fut: Future<Output = Result<Value>>,
{
    let body = get(
        "channels",
        query(&[
            ("part", "contentDetails"),
            ("id", &source.channel_id),
            ("maxResults", "50"),
        ]),
    )
    .await?;
    let (items, total, next) = page(&body, "youtube#channelListResponse")?;
    if items.len() != 1
        || total != 1
        || next.is_some()
        || text(&items[0], "/kind")? != "youtube#channel"
        || text(&items[0], "/id")? != source.channel_id
    {
        return Err(invalid());
    }
    let playlist = text(&items[0], "/contentDetails/relatedPlaylists/uploads")?;
    if !valid_id(playlist, 24) || !playlist.starts_with("UU") {
        return Err(invalid());
    }
    Ok(playlist.to_owned())
}

async fn collect<F, Fut>(
    source: &YoutubeSource,
    existing: &[ExistingVideo],
    previous: Option<&SourceCheckpoint>,
    mut get: F,
) -> Result<VerifiedMetadata>
where
    F: FnMut(&'static str, Query) -> Fut,
    Fut: Future<Output = Result<Value>>,
{
    source.validate()?;
    let mut old = BTreeMap::new();
    for video in existing {
        if video.channel_id != source.channel_id
            || !valid_id(&video.video_id, 11)
            || old.insert(video.video_id.clone(), video).is_some()
            || old.len() > MAX_DOCUMENTS_PER_SOURCE
        {
            return Err(invalid());
        }
    }
    let mut known: BTreeSet<String> = old.keys().cloned().collect();
    if let Some(checkpoint) = previous {
        if checkpoint.source_id != source.source_id() {
            return Err(invalid());
        }
        for logical in current_pins(checkpoint)?.keys() {
            let id = logical
                .strip_prefix("video/")
                .filter(|id| valid_id(id, 11))
                .ok_or_else(invalid)?;
            known.insert(id.to_owned());
        }
    }
    let playlist = channel_playlist(source, &mut get).await?;
    let (listed, upload_pages_first) = uploads(&source.channel_id, &playlist, &mut get).await?;
    known.extend(listed.iter().cloned());
    if known.len() > MAX_DOCUMENTS_PER_SOURCE {
        return Err(invalid());
    }
    let ids: Vec<_> = known.iter().cloned().collect();
    let mut videos = Vec::new();
    let mut raw = BTreeMap::new();
    let mut unavailable = BTreeSet::new();
    for chunk in ids.chunks(50) {
        let body = get(
            "videos",
            query(&[
                ("part", "snippet,contentDetails,status"),
                ("id", &chunk.join(",")),
                ("maxResults", "50"),
            ]),
        )
        .await?;
        let (items, total, next) = page(&body, "youtube#videoListResponse")?;
        if total != items.len() || next.is_some() {
            return Err(invalid());
        }
        let mut returned = BTreeSet::new();
        for item in items {
            let id = text(item, "/id")?;
            let title = text(item, "/snippet/title")?;
            let description = item
                .pointer("/snippet/description")
                .and_then(Value::as_str)
                .ok_or_else(invalid)?;
            if text(item, "/kind")? != "youtube#video"
                || !chunk.iter().any(|wanted| wanted == id)
                || !returned.insert(id.to_owned())
                || text(item, "/snippet/channelId")? != source.channel_id
                || title.trim().is_empty()
                || text(item, "/snippet/publishedAt")?.len() > 64
                || !matches!(
                    text(item, "/status/privacyStatus")?,
                    "public" | "unlisted" | "private"
                )
                || text(item, "/status/uploadStatus")? != "processed"
                || text(item, "/contentDetails/duration")?.len() > 128
                || !item["contentDetails"].is_object()
            {
                return Err(invalid());
            }
            let mut video = old.get(id).map_or_else(
                || ExistingVideo {
                    video_id: id.into(),
                    channel_id: source.channel_id.clone(),
                    title: title.into(),
                    description: Some(description.into()),
                    published_at: None,
                    language: None,
                    transcript: None,
                    transcript_kind: None,
                    transcript_hash: None,
                },
                |v| (*v).clone(),
            );
            video.title = title.into();
            video.description = Some(description.into());
            video.published_at = Some(published_at(text(item, "/snippet/publishedAt")?)?);
            if source.retention == Retention::MetadataOnly {
                video.language = None;
                video.transcript = None;
                video.transcript_kind = None;
                video.transcript_hash = None;
            }
            videos.push(video);
            raw.insert(id.to_owned(), item.clone());
        }
        unavailable.extend(chunk.iter().filter(|id| !returned.contains(*id)).cloned());
    }
    if channel_playlist(source, &mut get).await? != playlist {
        return Err(invalid());
    }
    let (confirmed, upload_pages_confirmation) =
        uploads(&source.channel_id, &playlist, &mut get).await?;
    if confirmed != listed {
        return Err(invalid());
    }
    let proof = MetadataProof {
        uploads: listed.len(),
        upload_pages_first,
        upload_pages_confirmation,
        probed_video_ids: ids.len(),
        available_videos: videos.len(),
        unavailable_videos: unavailable.len(),
        terminal_pages_verified: true,
    };
    Ok(VerifiedMetadata {
        proof,
        source_hash: fingerprint(source)?,
        checkpoint_hash: fingerprint(&previous)?,
        videos,
        raw,
        unavailable,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{source_sync::SourceRights, FeedPolicy};
    use brain_contracts::SourceVisibility;
    use serde_json::json;
    use std::{collections::VecDeque, future::ready};

    const CHANNEL: &str = "UCabcdefghijklmnopqrstuv";
    const PLAYLIST: &str = "UUabcdefghijklmnopqrstuv";
    const ID: &str = "abcdefghijk";

    fn source() -> YoutubeSource {
        YoutubeSource {
            channel_id: CHANNEL.into(),
            retention: Retention::TranscriptAndMetadata,
            rights: SourceRights {
                ingestion_allowed: true,
                authorization_ref: Some("fixture".into()),
                license: None,
                history_retention_months: 12,
                policy: FeedPolicy {
                    visibility: SourceVisibility::Internal,
                    allowed_scopes: BTreeSet::from(["review".into()]),
                    provider_egress_allowed: false,
                    publication_allowed: false,
                    raw_retention_allowed: true,
                },
            },
        }
    }
    fn channel() -> Value {
        json!({"kind":"youtube#channelListResponse","items":[{"kind":"youtube#channel","id":CHANNEL,
            "contentDetails":{"relatedPlaylists":{"uploads":PLAYLIST}}}],"pageInfo":{"totalResults":1,"resultsPerPage":50}})
    }
    fn listing(ids: &[&str], total: usize, next: Option<&str>) -> Value {
        let mut v = json!({"kind":"youtube#playlistItemListResponse","items":ids.iter().map(|id| json!({"kind":"youtube#playlistItem",
            "snippet":{"channelId":CHANNEL,"playlistId":PLAYLIST,"videoOwnerChannelId":CHANNEL,"resourceId":{"kind":"youtube#video","videoId":id}},
            "contentDetails":{"videoId":id}})).collect::<Vec<_>>(),"pageInfo":{"totalResults":total,"resultsPerPage":50}});
        if let Some(next) = next {
            v["nextPageToken"] = json!(next);
        }
        v
    }
    fn details(ids: &[&str]) -> Value {
        json!({"kind":"youtube#videoListResponse","items":ids.iter().map(|id| json!({"kind":"youtube#video","id":id,
            "snippet":{"channelId":CHANNEL,"title":"Fresh","description":"Uncut description","publishedAt":"2026-10-03T12:00:00Z"},
            "contentDetails":{"duration":"PT1M"},"status":{"privacyStatus":"public","uploadStatus":"processed"}})).collect::<Vec<_>>(),
            "pageInfo":{"totalResults":ids.len(),"resultsPerPage":50}})
    }
    async fn scripted(
        source: &YoutubeSource,
        existing: &[ExistingVideo],
        previous: Option<&SourceCheckpoint>,
        values: Vec<Value>,
    ) -> Result<VerifiedMetadata> {
        let mut values: VecDeque<_> = values.into();
        let result = collect(source, existing, previous, |resource, params| {
            let params: BTreeMap<_, _> = params.into_iter().collect();
            assert_eq!(params["maxResults"], "50");
            match resource {
                "channels" => {
                    assert_eq!(params["id"], CHANNEL);
                    assert_eq!(params["part"], "contentDetails");
                    assert_eq!(params.len(), 3);
                }
                "playlistItems" => {
                    assert_eq!(params["playlistId"], PLAYLIST);
                    assert_eq!(params["part"], "snippet,contentDetails");
                    assert!((3..=4).contains(&params.len()));
                }
                "videos" => {
                    assert_eq!(params["part"], "snippet,contentDetails,status");
                    assert!(params["id"].split(',').all(|id| valid_id(id, 11)));
                    assert_eq!(params.len(), 3);
                }
                _ => panic!("unexpected resource"),
            }
            ready(values.pop_front().ok_or_else(invalid))
        })
        .await;
        if result.is_ok() {
            assert!(values.is_empty());
        }
        result
    }
    fn old() -> ExistingVideo {
        ExistingVideo {
            video_id: ID.into(),
            channel_id: CHANNEL.into(),
            title: "Old".into(),
            description: None,
            published_at: Some(1),
            language: Some("de".into()),
            transcript: Some("Existing local transcript".into()),
            transcript_kind: Some("existing-caption".into()),
            transcript_hash: Some(sha256_hex(b"Existing local transcript")),
        }
    }

    #[tokio::test]
    async fn initially_empty_inventory_has_no_sentinel_and_is_bound_to_rights() {
        let source = source();
        let verified = scripted(
            &source,
            &[],
            None,
            vec![
                channel(),
                listing(&[], 0, None),
                channel(),
                listing(&[], 0, None),
            ],
        )
        .await
        .unwrap();
        let batch = super::super::prepare_fresh_batch(&source, &verified, 1, None).unwrap();
        assert!(batch.records.is_empty());
        assert!(current_pins(&batch.checkpoint).unwrap().is_empty());
        let mut altered = source;
        altered.retention = Retention::MetadataOnly;
        assert!(super::super::prepare_fresh_batch(&altered, &verified, 1, None).is_err());
    }

    #[tokio::test]
    async fn complete_empty_is_explicit_tombstones_and_is_idempotent() {
        let source = source();
        let first = super::super::prepare_batch(&source, &[old()], true, 1, None).unwrap();
        let responses = || {
            vec![
                channel(),
                listing(&[], 0, None),
                details(&[]),
                channel(),
                listing(&[], 0, None),
            ]
        };
        let verified = scripted(&source, &[old()], Some(&first.checkpoint), responses())
            .await
            .unwrap();
        assert_eq!(
            verified.unavailable_video_ids(),
            &BTreeSet::from([ID.into()])
        );
        assert_eq!(verified.proof().uploads, 0);
        assert_eq!(verified.proof().upload_pages_first, 1);
        assert_eq!(verified.proof().upload_pages_confirmation, 1);
        assert_eq!(verified.proof().probed_video_ids, 1);
        assert!(verified.proof().terminal_pages_verified);
        let cleared =
            super::super::prepare_fresh_batch(&source, &verified, 2, Some(&first.checkpoint))
                .unwrap();
        assert_eq!(cleared.records.len(), 1);
        assert!(cleared.records[0].tombstone);
        assert_eq!(
            cleared.records[0].metadata["removal_basis"],
            "not_retrievable_via_youtube_data_api_v3"
        );
        assert!(
            super::super::prepare_batch(&source, &[], true, 2, Some(&first.checkpoint)).is_err()
        );
        assert!(super::super::prepare_fresh_batch(
            &source,
            &verified,
            2,
            Some(&cleared.checkpoint)
        )
        .is_err());
        let next = scripted(&source, &[old()], Some(&cleared.checkpoint), responses())
            .await
            .unwrap();
        let batch = super::super::prepare_fresh_batch(&source, &next, 3, Some(&cleared.checkpoint))
            .unwrap();
        assert!(batch.records.is_empty());
        assert_eq!(batch.checkpoint.state, cleared.checkpoint.state);
    }

    #[tokio::test]
    async fn fresh_full_payload_preserves_only_local_transcripts_and_new_metadata() {
        let source = source();
        let second = "lmnopqrstuv";
        let verified = scripted(
            &source,
            &[old()],
            None,
            vec![
                channel(),
                listing(&[ID, second], 2, None),
                details(&[ID, second]),
                channel(),
                listing(&[ID, second], 2, None),
            ],
        )
        .await
        .unwrap();
        let batch = super::super::prepare_fresh_batch(&source, &verified, 2, None).unwrap();
        assert_eq!(batch.records.len(), 2);
        let retained: Value = serde_json::from_str(&batch.records[0].content).unwrap();
        assert_eq!(retained["video"]["title"], "Fresh");
        assert_eq!(retained["video"]["transcript"], "Existing local transcript");
        assert_eq!(
            retained["youtube_data_api_v3"]["contentDetails"]["duration"],
            "PT1M"
        );
        let added: Value = serde_json::from_str(&batch.records[1].content).unwrap();
        assert!(added["video"]["transcript"].is_null());
        assert!(
            super::super::prepare_fresh_batch(&source, &verified, 3, Some(&batch.checkpoint))
                .is_err()
        );
        let next = scripted(
            &source,
            &[old()],
            Some(&batch.checkpoint),
            vec![
                channel(),
                listing(&[ID, second], 2, None),
                details(&[ID, second]),
                channel(),
                listing(&[ID, second], 2, None),
            ],
        )
        .await
        .unwrap();
        assert!(
            super::super::prepare_fresh_batch(&source, &next, 3, Some(&batch.checkpoint))
                .unwrap()
                .records
                .is_empty()
        );
    }

    #[tokio::test]
    async fn old_ids_are_probed_even_when_absent_from_uploads_and_checkpoint_only() {
        let source = source();
        let first = super::super::prepare_batch(&source, &[old()], true, 1, None).unwrap();
        let verified = scripted(
            &source,
            &[],
            Some(&first.checkpoint),
            vec![
                channel(),
                listing(&[], 0, None),
                details(&[ID]),
                channel(),
                listing(&[], 0, None),
            ],
        )
        .await
        .unwrap();
        assert!(verified.unavailable.is_empty());
        assert_eq!(verified.videos.len(), 1);
        assert!(verified.videos[0].transcript.is_none());
    }

    #[tokio::test]
    async fn truncated_duplicates_foreign_channels_schema_errors_and_changed_sets_abort() {
        let source = source();
        let mut bad_listings = vec![
            listing(&[ID], 2, None),
            listing(&[ID, ID], 2, None),
            listing(&[ID], 1, Some("more")),
            listing(&[], 1, Some("more")),
        ];
        let mut foreign = listing(&[ID], 1, None);
        foreign["items"][0]["snippet"]["channelId"] = json!("foreign");
        bad_listings.push(foreign);
        for listing in bad_listings {
            assert!(scripted(&source, &[], None, vec![channel(), listing])
                .await
                .is_err());
        }
        for field in ["channelId", "publishedAt", "description"] {
            let mut bad = details(&[ID]);
            bad["items"][0]["snippet"][field] = json!(false);
            assert!(scripted(
                &source,
                &[],
                None,
                vec![channel(), listing(&[ID], 1, None), bad]
            )
            .await
            .is_err());
        }
        assert!(scripted(
            &source,
            &[],
            None,
            vec![channel(), listing(&[ID], 1, None), details(&[ID, ID])]
        )
        .await
        .is_err());
        assert!(scripted(
            &source,
            &[],
            None,
            vec![
                channel(),
                listing(&[ID], 1, None),
                details(&["lmnopqrstuv"])
            ]
        )
        .await
        .is_err());
        assert!(scripted(
            &source,
            &[],
            None,
            vec![
                channel(),
                listing(&[ID], 1, None),
                details(&[ID]),
                channel(),
                listing(&[], 0, None)
            ]
        )
        .await
        .is_err());
        assert!(
            scripted(&source, &[], None, vec![channel(), listing(&[ID], 1, None)])
                .await
                .is_err()
        );
        assert!(
            scripted(&source, &[], None, vec![json!({"error":{"code":403}})])
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn multiple_pages_require_terminal_count_and_reject_token_cycles() {
        let source = source();
        let second = "lmnopqrstuv";
        let pages = || vec![listing(&[ID], 2, Some("next")), listing(&[second], 2, None)];
        let mut values = vec![channel()];
        values.extend(pages());
        values.push(details(&[ID, second]));
        values.push(channel());
        values.extend(pages());
        assert_eq!(
            scripted(&source, &[], None, values)
                .await
                .unwrap()
                .videos
                .len(),
            2
        );
        assert!(scripted(
            &source,
            &[],
            None,
            vec![
                channel(),
                listing(&[ID], 3, Some("repeat")),
                listing(&[second], 3, Some("repeat"))
            ]
        )
        .await
        .is_err());
    }

    #[tokio::test]
    async fn more_than_fifty_videos_use_all_pages_and_all_details_chunks() {
        let source = source();
        let ids: Vec<_> = (0..51).map(|n| format!("{n:011}")).collect();
        let refs: Vec<_> = ids.iter().map(String::as_str).collect();
        let pages = || {
            vec![
                listing(&refs[..50], 51, Some("next")),
                listing(&refs[50..], 51, None),
            ]
        };
        let mut values = vec![channel()];
        values.extend(pages());
        values.push(details(&refs[..50]));
        values.push(details(&refs[50..]));
        values.push(channel());
        values.extend(pages());
        let verified = scripted(&source, &[], None, values).await.unwrap();
        assert_eq!(verified.proof().uploads, 51);
        assert_eq!(verified.proof().probed_video_ids, 51);
        assert_eq!(verified.proof().available_videos, 51);
        assert_eq!(verified.proof().unavailable_videos, 0);
        assert_eq!(verified.proof().upload_pages_first, 2);
        assert_eq!(verified.proof().upload_pages_confirmation, 2);
        assert_eq!(verified.raw.len(), 51);
        let batch = super::super::prepare_fresh_batch(&source, &verified, 1, None).unwrap();
        assert_eq!(batch.records.len(), 51);
        assert_eq!(current_pins(&batch.checkpoint).unwrap().len(), 51);
    }

    #[tokio::test]
    async fn metadata_only_fresh_payload_never_carries_local_transcripts() {
        let mut source = source();
        source.retention = Retention::MetadataOnly;
        let verified = scripted(
            &source,
            &[old()],
            None,
            vec![
                channel(),
                listing(&[ID], 1, None),
                details(&[ID]),
                channel(),
                listing(&[ID], 1, None),
            ],
        )
        .await
        .unwrap();
        let batch = super::super::prepare_fresh_batch(&source, &verified, 1, None).unwrap();
        let payload: Value = serde_json::from_str(&batch.records[0].content).unwrap();
        for field in [
            "transcript",
            "transcript_kind",
            "transcript_hash",
            "language",
        ] {
            assert!(payload["video"][field].is_null());
        }
        assert_eq!(payload["video"]["description"], "Uncut description");
        assert!(verified.unavailable_video_ids().is_empty());
    }

    #[tokio::test]
    async fn missing_metadata_access_fails_before_network() {
        assert!(fetch_complete(&source(), &[], None, "", 90).await.is_err());
        assert!(
            fetch_complete(&source(), &[], None, "invalid\ncredential", 90)
                .await
                .is_err()
        );
        assert!(fetch_complete(&source(), &[], None, "fixture", 0)
            .await
            .is_err());
    }

    #[test]
    fn date_schema_and_bounds_fail_closed() {
        assert_eq!(published_at("1970-01-01T00:00:00Z").unwrap(), 0);
        assert_eq!(published_at("2001-09-09T01:46:40Z").unwrap(), 1_000_000_000);
        assert!(published_at("2025-02-29T00:00:00Z").is_err());
        assert!(published_at("2024-02-29T00:00:00Z").is_ok());
        assert!(published_at("2026-10-03T24:00:00Z").is_err());
        assert!(page(
            &listing(&[], MAX_DOCUMENTS_PER_SOURCE + 1, None),
            "youtube#playlistItemListResponse"
        )
        .is_err());
    }

    #[tokio::test]
    async fn real_http_decoder_rejects_status_schema_and_interrupted_bodies() {
        use std::{
            io::{Read, Write},
            net::TcpListener,
        };
        for response in [
            "HTTP/1.1 403 Forbidden\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}",
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}",
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 99\r\nConnection: close\r\n\r\n{}",
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\nx!",
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let server = std::thread::spawn(move || {
                let (mut stream,_) = listener.accept().unwrap();
                let mut request = [0;4096];
                let received = stream.read(&mut request).unwrap();
                assert!(received > 0);
                stream.write_all(response.as_bytes()).unwrap();
            });
            let response = reqwest::Client::builder().no_proxy().build().unwrap().get(format!("http://{addr}")).send().await.unwrap();
            assert!(read_response(response).await.is_err());
            server.join().unwrap();
        }
    }
}
