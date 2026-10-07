use super::*;
use anyhow::{ensure, Context};
use deadlock_brain_core::http::SourceHttpOptions;
use reqwest::Url;

const PATCH_FEED_URL: &str = "https://api.deadlock-api.com/v2/patches";

#[derive(Debug, Deserialize)]
struct ApiPatchPost {
    source: String,
    title: String,
    pub_date: String,
    link: String,
    content: String,
}

pub fn sync_patchnotes(
    http: &HttpClient,
    ledger: &SteamLedger,
    dsn_env: &str,
    dry_run: bool,
) -> Result<Value> {
    let response = http.get_bounded(PATCH_FEED_URL, SourceHttpOptions::default())?;
    response.ensure_success()?;
    let mut posts: Vec<ApiPatchPost> =
        serde_json::from_slice(&response.content).context("Deadlock-Patchfeed ist ungültig")?;
    ensure!(
        !posts.is_empty() && posts.len() <= 1000,
        "Deadlock-Patchfeed ist leer oder zu groß"
    );
    for post in &posts {
        validate_post(post)?;
    }
    posts.sort_by(|left, right| left.pub_date.cmp(&right.pub_date));
    let dsn = env::var(dsn_env)
        .map_err(|_| anyhow!("{dsn_env} ist nicht gesetzt; DSN wird nicht ausgegeben"))?;
    let mut client = Client::connect(&dsn, NoTls)
        .map_err(|_| anyhow!("Zentrale Postgres-DB nicht erreichbar; DSN wird nicht ausgegeben"))?;
    ensure_pg_schema(&mut client)?;
    let index = load_entity_index(&mut client)?;
    let mut imported = Vec::new();
    let mut skipped = Vec::new();
    for post in posts {
        if !is_patch_candidate(&post) {
            skipped.push(json!({"url":post.link,"reason":"announcement_without_patch_changes"}));
            continue;
        }
        let existing = client.query_opt(
            "SELECT id FROM patchnotes.changelog_posts WHERE url=$1 ORDER BY id LIMIT 1",
            &[&post.link],
        )?;
        let row = post_row(&post, existing.map(|row| row.get(0)))?;
        let mut resolved = resolve_api_source(http, ledger, &mut client, &post, &row)?;
        if !has_complete_patch_content(&resolved.raw_content) {
            skipped.push(json!({"url":post.link,"reason":"preview_requires_official_fulltext","not_imported_as_patch":true}));
            continue;
        }
        resolved.raw_content = clean_steam_content(&cleanup_patch_content(&resolved.raw_content));
        let options = ImportPatchnoteOptions {
            patch_id: row.id,
            dsn_env: dsn_env.into(),
            dry_run,
        };
        let mut prepared = prepare_patch(&row, &resolved, &index)?;
        let canonical = client.query_opt(
            "SELECT metadata->>'patch_id' FROM brain.source_documents WHERE source=$1 AND external_id=$2 \
             AND metadata->>'patch_id' IS NOT NULL ORDER BY id LIMIT 1",
            &[&SOURCE, &prepared.source_external_id])?;
        if let Some(canonical) = canonical {
            let id: String = canonical.get(0);
            let id: i64 = id.parse().context("Kanonische Patchkennung ist ungültig")?;
            if id != row.id {
                let canonical_row = PatchnoteRow { id, ..row };
                prepared = prepare_patch(&canonical_row, &resolved, &index)?;
            }
        }
        imported.push(import_prepared_patch(&mut client, &options, prepared)?);
    }
    Ok(
        json!({"source":PATCH_FEED_URL,"feed_sha256":stable_hash(&response.content),
        "observed_at":response.observed_at,"dry_run":dry_run,"imported":imported,"skipped":skipped,
        "policy":"Discovery über API; Volltext und bestehender Parser bleiben maßgeblich"}),
    )
}

fn validate_post(post: &ApiPatchPost) -> Result<()> {
    ensure!(
        matches!(post.source.as_str(), "steam" | "forum"),
        "Unbekannte Patchfeed-Quelle"
    );
    ensure!(
        !post.title.trim().is_empty() && !post.content.trim().is_empty(),
        "Patchfeed ohne Titel oder Inhalt"
    );
    DateTime::parse_from_rfc3339(&post.pub_date)
        .context("Patchfeed ohne gültige Veröffentlichungszeit")?;
    let url = trusted_original_url(&post.link)
        .ok_or_else(|| anyhow!("Nicht erlaubte Originalquelle im Patchfeed"))?;
    ensure!(
        matches!(
            (post.source.as_str(), url.host_str()),
            ("forum", Some("forums.playdeadlock.com"))
                | ("steam", Some("store.steampowered.com"))
                | ("steam", Some("steamcommunity.com"))
        ),
        "Patchfeed-Quelle und Originalhost widersprechen sich"
    );
    Ok(())
}

fn trusted_original_url(raw: &str) -> Option<Url> {
    let url = Url::parse(raw).ok()?;
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port_or_known_default() != Some(443)
    {
        return None;
    }
    match url.host_str()? {
        "forums.playdeadlock.com" if url.path().starts_with("/threads/") => Some(url),
        "store.steampowered.com" if url.path().starts_with("/news/app/1422450/view/") => Some(url),
        "steamcommunity.com"
            if url.path().starts_with("/games/1422450/announcements/")
                || url.path().starts_with("/app/1422450/") =>
        {
            Some(url)
        }
        _ => None,
    }
}

fn post_row(post: &ApiPatchPost, existing_id: Option<i64>) -> Result<PatchnoteRow> {
    let url = trusted_original_url(&post.link).ok_or_else(|| anyhow!("Ungültige Patchquelle"))?;
    let mut canonical = url;
    canonical.set_query(None);
    canonical.set_fragment(None);
    let hash = stable_hash(canonical.as_str().as_bytes());
    let id = existing_id.unwrap_or(-i64::from_str_radix(&hash[..15], 16)?.max(1));
    Ok(PatchnoteRow {
        id,
        title: Some(post.title.trim().into()),
        url: Some(canonical.to_string()),
        posted_at: Some(post.pub_date.clone()),
        raw_content: Some(post.content.clone()),
        translated_content: None,
    })
}

fn is_patch_candidate(post: &ApiPatchPost) -> bool {
    post.source == "forum"
        || post.title.to_ascii_lowercase().contains("update")
        || has_patch_changes(&clean_steam_content(&cleanup_patch_content(&post.content)))
        || post.content.contains("playdeadlock.com/")
}

fn has_complete_patch_content(raw: &str) -> bool {
    let content = cleanup_patch_content(raw);
    let lower = content.to_ascii_lowercase();
    let has_reference = ["http://", "https://", "www.", "[url=", "[url]"]
        .iter()
        .any(|marker| lower.contains(marker))
        || lower
            .split("href")
            .skip(1)
            .any(|suffix| suffix.trim_start().starts_with('='));
    !has_reference && has_patch_changes(&clean_steam_content(&content))
}

fn has_patch_changes(content: &str) -> bool {
    content.lines().any(|line| {
        let line = line.trim();
        bullet_body(line).is_some()
            && !line.contains("http")
            && matches!(
                dbrain_normalize::classify_change_type(line).as_str(),
                "buff" | "nerf" | "bugfix" | "added" | "removed" | "rework" | "rename"
            )
    })
}

fn resolve_api_source(
    http: &HttpClient,
    ledger: &SteamLedger,
    client: &mut Client,
    post: &ApiPatchPost,
    row: &PatchnoteRow,
) -> Result<PatchSourceResolution> {
    if has_complete_patch_content(&post.content) {
        return Ok(PatchSourceResolution::from_row(row));
    }
    let urls = std::iter::once(post.link.clone())
        .chain(extract_http_tokens(&post.content))
        .filter_map(|raw| {
            trusted_original_url(raw.split(['"', '\'', '<', '>']).next().unwrap_or_default())
        });
    for url in urls {
        if url.host_str() != Some("store.steampowered.com") {
            continue;
        }
        let response = http.get_bounded(
            url.as_str(),
            SourceHttpOptions {
                headers: vec![("Accept".into(), "text/html".into())],
                ..Default::default()
            },
        )?;
        response.ensure_success()?;
        let html = std::str::from_utf8(&response.content)?;
        let item = SteamAppNewsItem {
            gid: String::new(),
            title: post.title.clone(),
            url: url.to_string(),
            contents: post.content.clone(),
            date: DateTime::parse_from_rfc3339(&post.pub_date)?.timestamp(),
            author: None,
            feedlabel: None,
            feedname: Some("steam_community_announcements".into()),
        };
        if let Some(body) = extract_steam_announcement_body_from_html(html, &item) {
            if has_complete_patch_content(&body) {
                return Ok(PatchSourceResolution {
                    raw_content: body,
                    source_url: Some(url.to_string()),
                    source_kind: "steam".into(),
                    resolved_from: Some(format!("{PATCH_FEED_URL}:{}", post.link)),
                });
            }
        }
    }
    resolve_patch_source_with(http, ledger, client, row, resolve_steam_raw_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn post() -> ApiPatchPost {
        ApiPatchPost { source:"steam".into(),title:" Minor Update - 10-05-2026".into(),
            pub_date:"2026-10-05T23:05:32Z".into(),
            link:"https://store.steampowered.com/news/app/1422450/view/703281025618282632".into(),
            content:"<p class=\"bb_paragraph\">- Rat King: Scrap Grenade - Damage increased from 65 to 70</p>".into() }
    }
    #[test]
    fn api_post_reuses_parser_and_exact_publication_time() {
        let post = post();
        validate_post(&post).unwrap();
        let row = post_row(&post, None).unwrap();
        assert!(row.id < 0);
        assert_eq!(post_row(&post, None).unwrap().id, row.id);
        assert_eq!(post_row(&post, Some(17)).unwrap().id, 17);
        let content = clean_steam_content(&cleanup_patch_content(&post.content));
        assert!(has_patch_changes(&content));
        let resolved = PatchSourceResolution {
            raw_content: content,
            ..PatchSourceResolution::from_row(&row)
        };
        let prepared = prepare_patch(&row, &resolved, &EntityIndex::default()).unwrap();
        assert!(!prepared.events.is_empty());
        assert_eq!(prepared.posted_at.unwrap().timestamp(), 1791241532);
    }
    #[test]
    fn previews_and_cosmetic_announcements_are_not_gameplay_patches() {
        let mut post = post();
        post.title = "Mind the Birds!".into();
        post.content = "<p>We voted for a new bird. See you in the city!</p>".into();
        assert!(!is_patch_candidate(&post));
        assert!(!has_patch_changes(
            "Full update: https://www.playdeadlock.com/cityneversleeps"
        ));
    }
    #[test]
    fn change_bullets_do_not_make_linked_previews_complete() {
        let full = post().content;
        assert!(has_complete_patch_content(&full));
        for link in [
            "<a href=\"https://www.playdeadlock.com/cityneversleeps\">Full update</a>",
            "<a HREF = \"/full-patch\">Full patch notes</a>",
            "[url=https://forums.playdeadlock.com/threads/update.123/]Full patch[/url]",
            "[url]https://forums.playdeadlock.com/threads/update.123/[/url]",
            "Full patch: HTTPS://store.steampowered.com/news/app/1422450/view/123",
            "Full patch: www.playdeadlock.com/cityneversleeps",
            "&lt;a href=&quot;https://www.playdeadlock.com/cityneversleeps&quot;&gt;Full update&lt;/a&gt;",
        ] {
            for changes in [full.clone(), full.repeat(20)] {
                let preview = format!("{changes}\n{link}");
                let cleaned = clean_steam_content(&cleanup_patch_content(&preview));
                assert!(has_patch_changes(&cleaned));
                assert!(!has_complete_patch_content(&preview), "{link}");
            }
        }
        assert!(!has_complete_patch_content("No gameplay changes."));
    }

    #[test]
    fn steam_fallback_keeps_fulltext_links_until_completeness_check() {
        let post = post();
        let preview = format!(
            "{}<a href=\"https://www.playdeadlock.com/cityneversleeps\">Full update</a>",
            post.content
        );
        let item = SteamAppNewsItem {
            gid: "123".into(),
            title: post.title,
            url: post.link,
            contents: preview.clone(),
            date: 1791241532,
            author: None,
            feedlabel: None,
            feedname: None,
        };
        let temp = tempfile::tempdir().unwrap();
        let http = HttpClient::new("Deadlock-Brain-Test/1", temp.path()).unwrap();
        let raw = resolve_steam_raw_content(&http, &item);
        assert_eq!(raw, preview);
        assert!(!has_complete_patch_content(&raw));
        assert!(has_complete_patch_content(&resolve_steam_content(
            &http, &item
        )));
    }

    #[test]
    fn original_links_fail_closed_on_wrong_hosts_credentials_and_ports() {
        for url in [
            "http://store.steampowered.com/news/app/1422450/view/1",
            "https://store.steampowered.com.attacker.invalid/news/app/1422450/view/1",
            "https://user@store.steampowered.com/news/app/1422450/view/1",
            "https://store.steampowered.com:444/news/app/1422450/view/1",
            "https://127.0.0.1/news/app/1422450/view/1",
        ] {
            assert!(trusted_original_url(url).is_none());
        }
    }
}
