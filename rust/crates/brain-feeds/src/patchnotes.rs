use crate::{clean, configuration, FeedError, FeedPolicy, Result};
use brain_contracts::{
    feeds::{PatchnotesFeedV1, PatchnotesPostV1, PATCHNOTES_FEED_VERSION},
    source::{
        GameValidity, OriginArtifact, SourceIdentity, SourcePolicy, SourceRevision, SourceTimestamp,
    },
    value::{Observed, UnknownReason},
    SourceBatch, SourceCheckpoint,
};
use brain_ingestion::document_set::{prepare_document_batch, CoreDocument, DocumentSetSource};
use std::collections::{BTreeMap, BTreeSet};

pub const SOURCE: &str = "patchnotes-feed";
pub const PARSER_REVISION: &str = "brain-feeds-patchnotes.v1";
pub const MAX_FEED_BYTES: usize = 64 * 1024 * 1024;
pub const OFFICIAL_FACTUAL_AUTHORIZATION: &str = "orchestrator:829d42f1-18eb-4f1d-9a14-a409c57242a5:2026-10-11:official-patch-factual-publication-and-egress:P2-8cb38338:P3-e782a46a";

fn official_post(feed: &PatchnotesFeedV1, post: &PatchnotesPostV1) -> bool {
    let Ok(url) = reqwest::Url::parse(&post.url) else {
        return false;
    };
    feed.provider == "deadlock-patchnotes-bot"
        && post.post_id.starts_with("changelog-")
        && post.source_revision.starts_with("changelog-posts/")
        && post.raw_sha256 == crate::sha256_hex(post.raw_text.as_bytes())
        && url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.port().is_none()
        && match url.host_str() {
            Some("forums.playdeadlock.com") => {
                url.path().starts_with("/threads/") || url.path().starts_with("/posts/")
            }
            Some("store.steampowered.com") => url.path().starts_with("/news/app/1422450/"),
            Some("steamcommunity.com") => url
                .path()
                .starts_with("/games/1422450/announcements/detail/"),
            _ => false,
        }
}

pub fn parse_feed(bytes: &[u8]) -> Result<PatchnotesFeedV1> {
    if bytes.len() > MAX_FEED_BYTES {
        return Err(FeedError::Invalid("feed exceeds byte limit".into()));
    }
    let feed: PatchnotesFeedV1 = serde_json::from_slice(bytes)?;
    feed.validate().map_err(FeedError::Invalid)?;
    Ok(feed)
}

pub fn documents(feed: &PatchnotesFeedV1, policy: &FeedPolicy) -> Result<Vec<CoreDocument>> {
    feed.validate().map_err(FeedError::Invalid)?;
    let mut out = Vec::with_capacity(feed.posts.len());
    for post in &feed.posts {
        let official = official_post(feed, post);
        if (policy.publication_allowed || policy.provider_egress_allowed) && !official {
            return Err(FeedError::Invalid(
                "Für öffentliche Antworten fehlt ein geprüftes offizielles Patchnotes-Original"
                    .into(),
            ));
        }
        let logical_id = format!("post/{}", clean(&post.post_id));
        let content = format!(
            "Patch: {}\nURL: {}\n\n{}\n",
            clean(&post.title),
            clean(&post.url),
            post.raw_text.trim()
        );
        out.push(CoreDocument {
            logical_id: logical_id.clone(),
            metadata: BTreeMap::from([
                ("connector".into(), SOURCE.into()),
                ("kind".into(), "prose".into()),
                ("locator".into(), post.url.clone()),
                ("title".into(), clean(&post.title)),
                ("provider_raw_sha256".into(), post.raw_sha256.clone()),
                ("feed_export_revision".into(), feed.export_revision.clone()),
            ]),
            origin: OriginArtifact {
                identity: SourceIdentity {
                    source_id: SOURCE.into(),
                    logical_id,
                },
                source_revision: SourceRevision::Api {
                    api_version: PATCHNOTES_FEED_VERSION.into(),
                    original_revision: Some(post.source_revision.clone()),
                },
                raw_sha256: post.raw_sha256.clone(),
                locator: post.url.clone(),
                parser_revision: PARSER_REVISION.into(),
                parser_family: "patchnotes_feed".into(),
                schema_version: Observed::known(PATCHNOTES_FEED_VERSION.into()),
                schema_sha256: Observed::unknown(UnknownReason::NotPresent),
                retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(feed.exported_at)),
                source_time: post.published_at.map_or_else(
                    || Observed::unknown(UnknownReason::NotPresent),
                    |t| Observed::known(SourceTimestamp::UnixSeconds(t)),
                ),
                language: post.language.clone().map_or_else(
                    || Observed::unknown(UnknownReason::NotPresent),
                    Observed::known,
                ),
                origin_artifacts: BTreeSet::from([format!(
                    "{}:{}@sha256:{}",
                    feed.provider, post.post_id, post.raw_sha256
                )]),
                derivation_family: Observed::known(format!("{}:changelog_posts", feed.provider)),
                policy: SourcePolicy {
                    visibility: policy.visibility,
                    allowed_scopes: policy.allowed_scopes.clone(),
                    authorization_ref: if official {
                        Observed::known(OFFICIAL_FACTUAL_AUTHORIZATION.into())
                    } else {
                        Observed::unknown(UnknownReason::NotPresent)
                    },
                    license: Observed::unknown(UnknownReason::NotPresent),
                    publication_allowed: policy.publication_allowed,
                    provider_egress_allowed: policy.provider_egress_allowed,
                    raw_retention_allowed: policy.raw_retention_allowed,
                },
                validity: GameValidity::unknown(),
            },
            content,
        });
    }
    Ok(out)
}

pub fn prepare_batch(
    feed: &PatchnotesFeedV1,
    policy: &FeedPolicy,
    previous: Option<&SourceCheckpoint>,
) -> Result<SourceBatch> {
    let set = DocumentSetSource {
        source_id: SOURCE.into(),
        configuration: configuration(SOURCE, PARSER_REVISION, policy)?,
        visibility: policy.visibility,
        allowed_scopes: policy.allowed_scopes.clone(),
        tombstone_metadata: BTreeMap::from([("connector".into(), SOURCE.into())]),
    };
    Ok(prepare_document_batch(
        &set,
        &documents(feed, policy)?,
        previous,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{
        feeds::PatchnotesPostV1, source::origin_from_record, DocumentStorePort, SourceVisibility,
    };
    use brain_storage::MemoryRepository;

    fn post(id: &str, text: &str) -> PatchnotesPostV1 {
        PatchnotesPostV1 {
            post_id: id.into(),
            title: format!("Update {id}"),
            url: format!("https://example.invalid/{id}"),
            published_at: Some(1_700_000_000),
            language: Some("en".into()),
            raw_text: text.into(),
            raw_sha256: crate::sha256_hex(text.as_bytes()),
            source_revision: format!("{id}-r1"),
        }
    }
    fn feed(posts: Vec<PatchnotesPostV1>) -> Vec<u8> {
        serde_json::to_vec(&PatchnotesFeedV1 {
            contract_version: PATCHNOTES_FEED_VERSION.into(),
            provider: "deadlock-patchnotes-bot".into(),
            export_revision: "export-1".into(),
            exported_at: 1_790_000_000,
            posts,
        })
        .unwrap()
    }
    fn policy() -> FeedPolicy {
        FeedPolicy {
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["game.public".into()]),
            provider_egress_allowed: false,
            publication_allowed: false,
            raw_retention_allowed: true,
        }
    }

    #[tokio::test]
    async fn feed_is_idempotent_revises_changes_and_tombstones_removed_posts() {
        let repo = MemoryRepository::default();
        let first = parse_feed(&feed(vec![post("a", "Abrams +5"), post("b", "Haze -2")])).unwrap();
        let batch = prepare_batch(&first, &policy(), None).unwrap();
        assert_eq!(batch.records.len(), 2);
        let origin = origin_from_record(&batch.records[0]).unwrap();
        assert_eq!(origin.raw_sha256, batch.records[0].content_hash);
        assert!(origin
            .origin_artifacts
            .iter()
            .next()
            .unwrap()
            .ends_with(&crate::sha256_hex(b"Abrams +5")));
        assert_eq!(origin.validity, GameValidity::unknown());
        let lease = repo.claim(SOURCE, "one", 30_000).await.unwrap();
        repo.commit(&batch, &lease).await.unwrap();
        let cp = repo.checkpoint(SOURCE).await.unwrap().unwrap();
        assert!(prepare_batch(&first, &policy(), Some(&cp))
            .unwrap()
            .records
            .is_empty());
        let second = parse_feed(&feed(vec![post("a", "Abrams +6")])).unwrap();
        let update = prepare_batch(&second, &policy(), Some(&cp)).unwrap();
        let states: Vec<_> = update
            .records
            .iter()
            .map(|r| (r.logical_id.as_str(), r.revision, r.tombstone))
            .collect();
        assert_eq!(states, vec![("post/a", 2, false), ("post/b", 2, true)]);
    }

    #[test]
    fn tampered_or_unknown_feeds_fail_closed() {
        let mut bad = post("a", "x");
        bad.raw_sha256 = crate::sha256_hex(b"y");
        assert!(parse_feed(&feed(vec![bad])).is_err());
        let mut value: serde_json::Value =
            serde_json::from_slice(&feed(vec![post("a", "x")])).unwrap();
        value["database_dsn"] = serde_json::json!("postgres://");
        assert!(parse_feed(&serde_json::to_vec(&value).unwrap()).is_err());
        assert!(parse_feed(&feed(Vec::new())).is_err());
    }

    #[test]
    fn official_originals_bind_approved_rights_across_refreshes() {
        let feed = parse_feed(include_bytes!("../tests/fixtures/patchnotes_feed.json")).unwrap();
        let restricted = prepare_batch(&feed, &policy(), None).unwrap();
        let mut public = policy();
        public.allowed_scopes = BTreeSet::from(["bot.public".into()]);
        public.publication_allowed = true;
        public.provider_egress_allowed = true;
        let approved = prepare_batch(&feed, &public, Some(&restricted.checkpoint)).unwrap();
        assert_eq!(approved.records.len(), feed.posts.len());
        for (old, new) in restricted.records.iter().zip(&approved.records) {
            let origin = origin_from_record(new).unwrap();
            assert_eq!(new.content, old.content);
            assert_eq!(new.content_hash, old.content_hash);
            assert_eq!(new.revision, old.revision + 1);
            assert_eq!(
                origin.policy.authorization_ref,
                Observed::known(OFFICIAL_FACTUAL_AUTHORIZATION.into())
            );
            assert!(origin.policy.publication_allowed && origin.policy.provider_egress_allowed);
            assert_eq!(
                origin.policy.license,
                Observed::unknown(UnknownReason::NotPresent)
            );
        }
        assert!(prepare_batch(&feed, &public, Some(&approved.checkpoint))
            .unwrap()
            .records
            .is_empty());
        let mut refreshed = feed.clone();
        let mut new_post = refreshed.posts[0].clone();
        new_post.post_id = "changelog-99".into();
        new_post.url = "https://forums.playdeadlock.com/posts/322289/".into();
        new_post.source_revision = "changelog-posts/99@next".into();
        refreshed.posts.push(new_post);
        let next = prepare_batch(&refreshed, &public, Some(&approved.checkpoint)).unwrap();
        assert_eq!(next.records.len(), 1);
        assert_eq!(
            origin_from_record(&next.records[0])
                .unwrap()
                .policy
                .authorization_ref,
            Observed::known(OFFICIAL_FACTUAL_AUTHORIZATION.into())
        );
    }

    #[test]
    fn answer_rights_reject_unofficial_or_corrupt_originals() {
        let feed = parse_feed(include_bytes!("../tests/fixtures/patchnotes_feed.json")).unwrap();
        let mut public = policy();
        public.publication_allowed = true;
        public.provider_egress_allowed = true;
        for url in [
            "https://forums.playdeadlock.com.evil.invalid/threads/update",
            "https://steamcommunity.com/games/440/announcements/detail/1",
            "https://steamcommunity.com/profiles/1",
            "http://forums.playdeadlock.com/threads/update",
            "https://user@forums.playdeadlock.com/threads/update",
        ] {
            let mut changed = feed.clone();
            changed.posts[0].url = url.into();
            assert!(prepare_batch(&changed, &public, None).is_err());
            let retained = prepare_batch(&changed, &policy(), None).unwrap();
            assert_eq!(
                origin_from_record(&retained.records[0])
                    .unwrap()
                    .policy
                    .authorization_ref,
                Observed::unknown(UnknownReason::NotPresent)
            );
        }
        let mut foreign = feed.clone();
        foreign.provider = "foreign-provider".into();
        assert!(prepare_batch(&foreign, &public, None).is_err());
        let mut corrupted = feed;
        corrupted.posts[0].raw_sha256 = "a".repeat(64);
        assert!(prepare_batch(&corrupted, &public, None).is_err());
    }

    #[test]
    fn provider_export_fixture_is_accepted_and_prepares_a_batch() {
        // Echter Export des Patchnotes-Providers (test_brain_feed.py):
        // gleicher Vertrag, inhaltsgebundene Export-Revision, gueltige Raw-Hashes.
        let bytes = include_str!("../tests/fixtures/patchnotes_feed.json").as_bytes();
        let parsed = parse_feed(bytes).unwrap();
        assert_eq!(parsed.provider, "deadlock-patchnotes-bot");
        assert_eq!(parsed.posts.len(), 2);
        assert!(parsed
            .posts
            .iter()
            .all(|p| p.raw_sha256 == crate::sha256_hex(p.raw_text.as_bytes())));
        // Der Export bindet auch Metadaten an die Source-Revision. Die
        // Fixture muss nach einer Provider-Aenderung erneut echt passen.
        for post in &parsed.posts {
            let fields = serde_json::json!({
                "title": post.title,
                "url": post.url,
                "published_at": post.published_at,
                "language": post.language,
                "raw_sha256": post.raw_sha256,
            });
            let digest = crate::sha256_hex(&serde_json::to_vec(&fields).unwrap());
            let id = post.post_id.strip_prefix("changelog-").unwrap();
            assert_eq!(
                post.source_revision,
                format!("changelog-posts/{id}@{}", &digest[..12])
            );
        }
        let mut posts = parsed.posts.clone();
        posts.sort_by(|left, right| left.post_id.cmp(&right.post_id));
        let canonical: Vec<serde_json::Value> = posts
            .into_iter()
            .map(|post| serde_json::to_value(post).unwrap())
            .collect();
        let digest = crate::sha256_hex(&serde_json::to_vec(&canonical).unwrap());
        assert_eq!(parsed.export_revision, format!("export-{}", &digest[..16]));
        let batch = prepare_batch(&parsed, &policy(), None).unwrap();
        assert_eq!(batch.records.len(), 2);
        assert!(batch
            .records
            .iter()
            .all(|r| r.logical_id.starts_with("post/changelog-")));
    }
}
