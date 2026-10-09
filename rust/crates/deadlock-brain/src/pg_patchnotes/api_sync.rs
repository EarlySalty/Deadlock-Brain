use super::*;
use anyhow::{ensure, Context};
use deadlock_brain_core::http::SourceHttpOptions;
use reqwest::Url;

const PATCH_FEED_URL: &str = "https://api.deadlock-api.com/v2/patches";
const GENERAL_PATCH_SUBJECTS: &[&str] = &[
    "bullet spread",
    "bullet velocity",
    "weapon damage",
    "fire rate",
    "reload time",
    "health regen",
    "max health",
    "movement speed",
    "stamina",
    "troopers",
    "guardians",
    "walkers",
    "patrons",
    "ziplines",
];

#[derive(Debug, Deserialize)]
struct ApiPatchPost {
    source: String,
    title: String,
    pub_date: String,
    link: String,
    content: String,
}

pub fn sync_patchnotes(http: &HttpClient, dsn_env: &str, dry_run: bool) -> Result<Value> {
    let response = http.get_bounded(PATCH_FEED_URL, SourceHttpOptions::default())?;
    response.ensure_success()?;
    let mut posts: Vec<ApiPatchPost> =
        serde_json::from_slice(&response.content).context("Deadlock-Patchfeed ist ungültig")?;
    ensure!(
        !posts.is_empty() && posts.len() <= 1000,
        "Deadlock-Patchfeed ist leer oder zu groß"
    );
    for post in &posts {
        validate_post(post).inspect_err(|error| {
            eprintln!(
                "{}",
                json!({"title": post.title, "error": error.to_string(),
                "not_imported_as_patch": true})
            );
        })?;
    }
    posts.sort_by_cached_key(|post| {
        DateTime::parse_from_rfc3339(&post.pub_date).expect("Veröffentlichungszeit wurde geprüft")
    });
    let dsn = env::var(dsn_env)
        .map_err(|_| anyhow!("{dsn_env} ist nicht gesetzt; DSN wird nicht ausgegeben"))?;
    let mut client = Client::connect(&dsn, NoTls)
        .map_err(|_| anyhow!("Zentrale Postgres-DB nicht erreichbar; DSN wird nicht ausgegeben"))?;
    ensure_pg_schema(&mut client)?;
    let index = load_entity_index(&mut client)?;
    let mut imported = Vec::new();
    let mut skipped = Vec::new();
    let mut failed = Vec::new();
    for post in posts {
        let row = load_post_row(&mut client, &post)?;
        let resolution = resolve_api_source(&post, |url| {
            let response = http.get_bounded(
                url,
                SourceHttpOptions {
                    headers: vec![("Accept".into(), "text/html".into())],
                    ..Default::default()
                },
            )?;
            response.ensure_success()?;
            Ok(std::str::from_utf8(&response.content)?.to_string())
        });
        let resolved = match resolution {
            Ok(resolved) => resolved,
            Err(error) => {
                let entry = json!({"url": post.link, "error": error.to_string(), "not_imported_as_patch": true});
                eprintln!("{entry}");
                failed.push(entry);
                continue;
            }
        };
        let Some(resolved) = resolved else {
            skipped.push(log_skipped_post(
                &post,
                "preview_requires_official_fulltext",
            ));
            continue;
        };
        let mut prepared = prepare_api_patch(&row, &resolved, &index)?;
        for event in prepared
            .events
            .iter()
            .filter(|event| event.entity_name.is_none())
        {
            eprintln!(
                "{}",
                json!({"url": prepared.url, "patch_id": prepared.row_id,
                "line_index": event.line_index, "line": event.raw_line, "binding": "unbound"})
            );
        }
        if !is_patch_candidate(&prepared, &index) {
            skipped.push(log_skipped_post(
                &post,
                "announcement_without_known_patch_changes",
            ));
            continue;
        }
        let canonical = client.query_opt(
            "SELECT metadata->>'patch_id' FROM brain.source_documents WHERE source=$1 AND external_id=$2 \
             AND metadata->>'patch_id' IS NOT NULL ORDER BY id LIMIT 1",
            &[&SOURCE, &prepared.source_external_id])?;
        if let Some(canonical) = canonical {
            let id: String = canonical.get(0);
            let id: i64 = id.parse().context("Kanonische Patchkennung ist ungültig")?;
            if id != row.id {
                let canonical_row = PatchnoteRow { id, ..row };
                prepared = prepare_api_patch(&canonical_row, &resolved, &index)?;
            }
        }
        let options = ImportPatchnoteOptions {
            patch_id: prepared.row_id,
            dsn_env: dsn_env.into(),
            dry_run,
        };
        imported.push(import_prepared_patch(&mut client, &options, prepared)?);
    }
    let summary = json!({"source":PATCH_FEED_URL,"feed_sha256":stable_hash(&response.content),
        "observed_at":response.observed_at,"dry_run":dry_run,"imported":imported,"skipped":skipped,
        "failed":failed,"complete":failed.is_empty(),
        "policy":"Discovery über API; Volltext und bestehender Parser bleiben maßgeblich"});
    if !failed.is_empty() {
        eprintln!("{summary}");
        anyhow::bail!("Patch-Erkennung unvollständig: {} Originalquellen nicht erreichbar; bestätigte Originale wurden verarbeitet", failed.len());
    }
    Ok(summary)
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
    let mut url = Url::parse(raw).ok()?;
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port_or_known_default() != Some(443)
    {
        return None;
    }
    url.set_fragment(None);
    match url.host_str()? {
        "forums.playdeadlock.com" if forum_thread_id(&url).is_some() => Some(url),
        "store.steampowered.com" | "steamcommunity.com" if steam_original_gid(&url).is_some() => {
            Some(url)
        }
        _ => None,
    }
}

fn steam_original_gid(url: &Url) -> Option<String> {
    let path = url.path().strip_suffix('/').unwrap_or(url.path());
    let prefixes: &[&str] = match url.host_str()? {
        "store.steampowered.com" => &["/news/app/1422450/view/"],
        "steamcommunity.com" => &[
            "/app/1422450/event/",
            "/games/1422450/announcements/detail/",
            "/app/1422450/announcements/detail/",
            "/app/1422450/externalpost/steam_community_announcements/",
        ],
        _ => return None,
    };
    let gid = prefixes
        .iter()
        .find_map(|prefix| path.strip_prefix(prefix))?;
    (!gid.is_empty() && gid.bytes().all(|byte| byte.is_ascii_digit())).then(|| gid.to_string())
}

fn forum_thread_id(url: &Url) -> Option<u64> {
    let slug = url.path().strip_prefix("/threads/")?.trim_end_matches('/');
    if slug.contains('/')
        || url
            .query_pairs()
            .any(|(key, value)| key != "page" || value != "1")
    {
        return None;
    }
    let (_, id) = slug.rsplit_once('.')?;
    id.parse::<u64>().ok().filter(|id| *id > 0)
}

fn bound_forum_original(html: &str, url: &Url) -> Result<Option<String>> {
    let Some(thread_id) = forum_thread_id(url) else {
        return Ok(None);
    };
    if extract_html_attribute(html, "data-content-key")
        != Some(format!("thread-{thread_id}").as_str())
    {
        return Ok(None);
    }
    let Some(script) = html
        .split("<script type=\"application/ld+json\">")
        .nth(1)
        .and_then(|script| script.split("</script>").next())
    else {
        return Ok(None);
    };
    let Ok(document) = serde_json::from_str::<Value>(script) else {
        return Ok(None);
    };
    let original = &document["mainEntity"];
    if original["@type"] != "DiscussionForumPosting"
        || original["url"]
            .as_str()
            .and_then(trusted_original_url)
            .and_then(|url| forum_thread_id(&url))
            != Some(thread_id)
    {
        return Ok(None);
    }
    let Some(text) = original["text"].as_str() else {
        return Ok(None);
    };
    let body = dbrain_sources::forum::first_post_html(html)?;
    let normalize =
        |text: &str| normalize_patch_line(&clean_steam_content(&cleanup_patch_content(text)));
    if normalize(text).is_empty() || normalize(text) != normalize(&body) {
        return Ok(None);
    }
    Ok(Some(body))
}

fn load_post_row(
    client: &mut impl postgres::GenericClient,
    post: &ApiPatchPost,
) -> Result<PatchnoteRow> {
    let url = trusted_original_url(&post.link).ok_or_else(|| anyhow!("Ungültige Patchquelle"))?;
    let canonical = canonical_post_url(&url);
    let mut identities = vec![canonical.to_string()];
    if let Some(gid) = extract_steam_event_gid(url.as_str()) {
        identities.push(format!(
            "https://steamcommunity.com/app/1422450/event/{gid}"
        ));
    } else if let Some(gid) =
        extract_steam_gid(url.as_str()).filter(|_| url.host_str() == Some("steamcommunity.com"))
    {
        identities.push(format!(
            "https://steamcommunity.com/app/1422450/announcements/detail/{gid}"
        ));
        identities.push(format!("https://steamcommunity.com/app/1422450/externalpost/steam_community_announcements/{gid}"));
    }
    let existing = client.query_opt(
        "SELECT id FROM patchnotes.changelog_posts \
         WHERE rtrim(split_part(split_part(url, '#', 1), '?', 1), '/') = ANY($1) \
         ORDER BY (url=$2) DESC, (url=$3) DESC, (url=$4) DESC, id LIMIT 1",
        &[
            &identities,
            &canonical.as_str(),
            &url.as_str(),
            &post.link.as_str(),
        ],
    )?;
    post_row(post, existing.map(|row| row.get(0)))
}

fn canonical_post_url(url: &Url) -> Url {
    let mut canonical = url.clone();
    canonical.set_query(None);
    canonical.set_fragment(None);
    let path = canonical.path().trim_end_matches('/').to_string();
    canonical.set_path(&path);
    if let Some(gid) = extract_steam_event_gid(url.as_str()) {
        canonical
            .set_host(Some("store.steampowered.com"))
            .expect("Fester Steam-Host");
        canonical.set_path(&format!("/news/app/1422450/view/{gid}"));
    } else if let Some(gid) =
        extract_steam_gid(url.as_str()).filter(|_| url.host_str() == Some("steamcommunity.com"))
    {
        canonical.set_path(&format!("/games/1422450/announcements/detail/{gid}"));
    }
    canonical
}

fn post_row(post: &ApiPatchPost, existing_id: Option<i64>) -> Result<PatchnoteRow> {
    let mut url =
        trusted_original_url(&post.link).ok_or_else(|| anyhow!("Ungültige Patchquelle"))?;
    let canonical = canonical_post_url(&url);
    let hash = stable_hash(canonical.as_str().as_bytes());
    let id = existing_id.unwrap_or(-i64::from_str_radix(&hash[..15], 16)?.max(1));
    url.set_query(None);
    Ok(PatchnoteRow {
        id,
        title: Some(post.title.trim().into()),
        url: Some(url.to_string()),
        posted_at: Some(post.pub_date.clone()),
        raw_content: Some(post.content.clone()),
        translated_content: None,
    })
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
    !has_reference && !clean_steam_content(&content).is_empty()
}

fn prepare_api_patch(
    row: &PatchnoteRow,
    resolved: &PatchSourceResolution,
    index: &EntityIndex,
) -> Result<PreparedPatch> {
    let mut prepared = prepare_patch(row, resolved, index)?;
    let content = clean_steam_content(&cleanup_patch_content(&resolved.raw_content));
    let mut lines = Vec::new();
    for line in patch_lines_with(&content, true) {
        let trimmed = line.trim();
        if bullet_body(trimmed).is_some() {
            lines.push(line);
        } else if index
            .exact(trimmed.trim_matches(['[', ']', ':']).trim())
            .is_some()
            || (trimmed.starts_with('[') && trimmed.ends_with(']'))
            || !has_change_action(trimmed)
        {
            if let Some(heading) = section_heading(trimmed) {
                lines.push(heading);
            }
        } else {
            lines.push(format!("- {trimmed}"));
        }
    }
    prepared.events = parse_event_lines(
        row,
        prepared.url.as_deref(),
        &prepared.source_kind,
        prepared.posted_at,
        lines,
        index,
    );
    Ok(prepared)
}

fn is_patch_candidate(patch: &PreparedPatch, index: &EntityIndex) -> bool {
    patch.events.iter().any(|event| {
        event.entity_name.is_some()
            || (event.subject.is_none()
                && has_change_action(&event.normalized_line)
                && (event.section.as_deref().is_some_and(|section| {
                    matches!(
                        section.to_ascii_lowercase().as_str(),
                        "general" | "general changes" | "gameplay" | "map changes" | "objectives"
                    )
                }) || GENERAL_PATCH_SUBJECTS.iter().any(|subject| {
                    event
                        .normalized_line
                        .to_ascii_lowercase()
                        .strip_prefix(subject)
                        .is_some_and(|rest| rest.starts_with(char::is_whitespace))
                })))
            || event.section.as_deref().is_some_and(|section| {
                index.exact(section).is_some()
                    || section_subject(Some(section))
                        .and_then(|subject| index.exact(subject))
                        .is_some()
            })
    })
}

fn log_skipped_post(post: &ApiPatchPost, reason: &str) -> Value {
    let entry = json!({"url": post.link, "title": post.title, "reason": reason,
        "not_imported_as_patch": true});
    eprintln!("{entry}");
    entry
}

fn resolve_api_source(
    post: &ApiPatchPost,
    mut fetch_html: impl FnMut(&str) -> Result<String>,
) -> Result<Option<PatchSourceResolution>> {
    let url = trusted_original_url(&post.link)
        .ok_or_else(|| anyhow!("Ungültige Originalquelle im Patchfeed"))?;
    let html = fetch_html(url.as_str())?;
    let body = if post.source == "forum" {
        bound_forum_original(&html, &url)?
    } else {
        let item = SteamAppNewsItem {
            gid: steam_original_gid(&url)
                .ok_or_else(|| anyhow!("Originalquelle ohne konkrete Steam-Kennung"))?,
            title: post.title.clone(),
            url: url.to_string(),
            contents: String::new(),
            date: DateTime::parse_from_rfc3339(&post.pub_date)?.timestamp(),
            author: None,
            feedlabel: None,
            feedname: Some("steam_community_announcements".into()),
        };
        extract_steam_announcement_body_from_html(&html, &item)
    };
    Ok(body
        .filter(|body| has_complete_patch_content(body))
        .map(|body| PatchSourceResolution {
            raw_content: body,
            source_url: Some(url.to_string()),
            source_kind: post.source.clone(),
            resolved_from: Some(format!("{PATCH_FEED_URL}:{}", post.link)),
        }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patch_index() -> EntityIndex {
        let mut index = EntityIndex::default();
        for (kind, name) in [
            ("hero", "Rat King"),
            ("hero", "Holliday"),
            ("hero", "Sinclair"),
            ("item", "Metal Skin"),
            ("item", "Improved Spirit"),
            ("ability", "Scrap Grenade"),
            ("mechanic", "Weapon damage"),
            ("mechanic", "Bullet velocity"),
        ] {
            index.insert(kind, name, name);
        }
        index.finish()
    }

    fn candidate(original: &str, index: &EntityIndex) -> bool {
        let row = post_row(&post(), Some(17)).unwrap();
        let resolved = PatchSourceResolution {
            raw_content: original.into(),
            ..PatchSourceResolution::from_row(&row)
        };
        is_patch_candidate(&prepare_api_patch(&row, &resolved, index).unwrap(), index)
    }

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
        assert!(candidate(&content, &patch_index()));
        let resolved = PatchSourceResolution {
            raw_content: post.content.clone(),
            ..PatchSourceResolution::from_row(&row)
        };
        let prepared = prepare_api_patch(&row, &resolved, &patch_index()).unwrap();
        assert!(!prepared.events.is_empty());
        assert_eq!(prepared.posted_at.unwrap().timestamp(), 1791241532);
    }
    #[test]
    fn announcements_without_change_lines_are_not_patches() {
        let index = patch_index();
        for original in [
            "<p>Mind the Birds!</p><p>We voted for our favourite bird. See you in the city!</p>",
            "<p>Holliday skins and item icons for everyone.</p>",
            "<p>Holliday artwork and animations in this update.</p>",
            "<p>Holliday is our favourite hero.</p>",
            "<p>Holliday: our favourite hero with artwork, animations and item icons for everyone.</p>",
            "[ Holliday ] Artwork and icons for everyone.",
            "[ General Changes ] Holliday: our favourite hero with artwork and animations.",
            "<p>Holliday is our favourite hero. We love the artwork, the attack animation and the sound, and we voted for our favourite bird before meeting everyone in the city.</p>",
        ] {
            assert!(!candidate(original, &index), "{original}");
            for source in ["steam", "forum"] {
                let mut post = post();
                post.source = source.into();
                if source == "forum" {
                    post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
                }
                let html = if source == "forum" {
                    forum_html(&post, original)
                } else {
                    steam_html(&post, original)
                };
                let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                    .unwrap()
                    .unwrap();
                let prepared =
                    prepare_api_patch(&post_row(&post, Some(17)).unwrap(), &resolved, &index)
                        .unwrap();
                assert!(!is_patch_candidate(&prepared, &index), "{source}/{original}");
            }
        }
    }

    #[test]
    fn post_recognition_requires_changes_and_a_known_entity() {
        let index = patch_index();
        for original in [
            "<p>Scrap Grenade damage increased from 65 to 70.</p>",
            "<p>Today we introduce six new heroes, including Holliday.</p>",
            "<p>- Rat King: Rule, Ratannia! - Now falls faster while charging</p>",
            "<p>- Holliday: T1 +1m/s Move Speed to +2m/s Move Speed</p>",
            "<p>- Holliday: adjusted targeting</p>",
            "[ Holliday ] Added knockback",
            "[ Holliday ] • adjusted targeting",
            "[ Holliday ] - Unknown: adjusted targeting",
        ] {
            assert!(candidate(original, &index), "{original}");
            assert!(!candidate(original, &EntityIndex::default()), "{original}");
        }
        for original in [
            "- Unknown: damage increased from 10 to 20",
            "Holliday",
            "Improved Spirit",
        ] {
            assert!(!candidate(original, &index), "{original}");
        }
    }

    #[test]
    fn general_patch_changes_do_not_require_or_invent_entity_bindings() {
        let index = EntityIndex::default();
        for original in [
            "Bullet spread reduced from 4 to 3",
            "Bullet velocity increased from 1000 to 1200.",
            "Fire rate increased from 5 to 6",
            "[ General Changes ]\n- Added knockdown",
        ] {
            assert!(candidate(original, &index), "{original}");
            for source in ["steam", "forum"] {
                let mut post = post();
                post.source = source.into();
                if source == "forum" {
                    post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
                }
                let html = if source == "forum" {
                    forum_html(&post, original)
                } else {
                    steam_html(&post, original)
                };
                let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                    .unwrap()
                    .unwrap();
                let prepared =
                    prepare_api_patch(&post_row(&post, Some(17)).unwrap(), &resolved, &index)
                        .unwrap();
                assert!(is_patch_candidate(&prepared, &index), "{source}/{original}");
                assert_eq!(prepared.events.len(), 1, "{source}/{original}");
                let event = &prepared.events[0];
                assert_eq!(event.entity_type, "general");
                assert_eq!(event.entity_name, None);
                assert_eq!(event.subject, None);
                assert_eq!(event.metadata["source_kind"], source);
                assert_eq!(event.metadata["url"], post.link);
            }
        }
        for original in [
            "- Unknown: damage increased from 10 to 20",
            "Sale price reduced from 4 to 3",
            "Added unknown artwork",
            "[ General Changes ]\nHolliday is our favourite hero.",
        ] {
            assert!(!candidate(original, &index), "{original}");
        }
    }

    #[test]
    fn bound_entity_changes_reach_prepared_events_with_and_without_numbers() {
        let mut index = EntityIndex::default();
        index.insert("hero", "Holliday", "Holliday");
        index.insert("ability", "Scrap Grenade", "Scrap Grenade");
        let index = index.finish();
        for (original, entity, change_type, old, new) in [
            (
                "- Holliday: increased from 1 to 2",
                "Holliday",
                "buff",
                Some("1"),
                Some("2"),
            ),
            (
                "- Holliday: hitbox increased from 1m to 2m",
                "Holliday",
                "buff",
                Some("1m"),
                Some("2m"),
            ),
            (
                "- Holliday: slow increased from 20% to 30%",
                "Holliday",
                "buff",
                Some("20%"),
                Some("30%"),
            ),
            (
                "- Holliday: invulnerability increased from 3 to 4",
                "Holliday",
                "buff",
                Some("3"),
                Some("4"),
            ),
            (
                "- Holliday: added a double jump",
                "Holliday",
                "added",
                None,
                None,
            ),
            (
                "- Scrap Grenade: added knockback",
                "Scrap Grenade",
                "added",
                None,
                None,
            ),
            (
                "- Holliday: increased from 1 to 2 charges",
                "Holliday",
                "buff",
                Some("1"),
                Some("2 charges"),
            ),
            (
                "- Holliday: reworked primary attack",
                "Holliday",
                "rework",
                None,
                None,
            ),
        ] {
            assert!(!candidate(original, &EntityIndex::default()), "{original}");
            assert!(candidate(original, &index), "{original}");
            assert!(has_complete_patch_content(original), "{original}");
            for source in ["steam", "forum"] {
                let mut post = post();
                post.source = source.into();
                post.content = "Preview only".into();
                let html = if source == "forum" {
                    post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
                    forum_html(&post, original)
                } else {
                    steam_html(&post, original)
                };
                let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                    .unwrap()
                    .unwrap();
                let row = post_row(&post, Some(17)).unwrap();
                let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
                assert_eq!(prepared.events.len(), 1, "{source}/{original}");
                let event = &prepared.events[0];
                assert_eq!(event.entity_name.as_deref(), Some(entity));
                assert_eq!(event.change_type, change_type);
                assert_eq!(event.old_value.as_deref(), old);
                assert_eq!(event.new_value.as_deref(), new);
                assert_eq!(event.metadata["patch_external_id"], "patch_17");
                assert_eq!(event.metadata["url"], post.link);
                assert_eq!(event.metadata["source_kind"], source);
            }
        }
    }

    #[test]
    fn latest_public_patch_keeps_every_gameplay_line_and_its_entity() {
        let lines = [
            "- Fixed bugs that were causing troopers to walk too slowly in lane",
            "- Rat King: Scrap Grenade - Fixed some bugs with bouncing targeting",
            "- Rat King: Scrap Grenade - Damage increased from 65 to 70",
            "- Rat King: Scrap Grenade - T2 Damage increased from 65 to 70",
            "- Rat King: Rat Swarm - Base Cooldown reduced from 28s to 24s",
            "- Rat King: Rule, Ratannia! - Cooldown increased from 130 to 160",
            "- Rat King: Rule, Ratannia! - Damage Reduction reduced from 20% to 15%",
            "- Rat King: Rule, Ratannia! - Radius reduced from 24m to 20m",
            "- Rat King: Rule, Ratannia! - Charge Duration reduced from 12s to 10s",
            "- Rat King: Rule, Ratannia! - Bonus Move speed no longer scales with Spirit",
            "- Rat King: Rule, Ratannia! - Now falls faster while charging",
            "- Rat King: Rule, Ratannia! - T1 +1m/s Move Speed to +2m/s Move Speed",
            "- Rat King: Rule, Ratannia! - T1 No longer gives -20s cooldown (Total cooldown from 110s to 160s)",
            "- Rat King: Rule, Ratannia! - T2 No longer grants +8s Charge Duration (Total charge duration from 20s to 10s)",
            "- Rat King: Rule, Ratannia! - T2 Flag Duration reduced from +8s to +5s",
            "- Rat King: Rule, Ratannia! - T3 Damage Reduction reduced from 15% to 10% (Total Damage Reduction from 35% to 25%)",
            "- Sinclair: Base HP Regen reduced from 2 to 1",
            "- Sinclair: Gun - Falloff range reduced from 25-61 to 20-60",
            "- Sinclair: Gun - Reduced bullet base damage from 17.76 to 16.5",
            "- Sinclair: Vexing Bolt - Now does half damage to objectives",
            "- Sinclair: Spectral Assistant - Gun falloff range reduced from 25-61 to 20-60",
            "- Sinclair: Rabbit Hex - Radius reduced from 6.5m to 6m",
            "- Sinclair: Rabbit Hex - T3 reduced from +3m Radius to +2m",
            "- Sinclair: Audience Participation - Copy duration reduced from 12s to 9s",
            "- Sinclair: Audience Participation - Copied cooldown increased from 40% to 60%",
        ];
        let original = lines
            .iter()
            .map(|line| format!("<p class=\"bb_paragraph\">{line}</p>"))
            .collect::<String>();
        let mut index = EntityIndex::default();
        index.insert("hero", "Rat King", "Rat King");
        index.insert("hero", "Sinclair", "Sinclair");
        let index = index.finish();
        for source in ["steam", "forum"] {
            let mut post = post();
            post.source = source.into();
            post.content = "Preview only".into();
            if source == "forum" {
                post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
            }
            let html = if source == "forum" {
                forum_html(&post, &original)
            } else {
                steam_html(&post, &original)
            };
            let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                .unwrap()
                .unwrap();
            let row = post_row(&post, Some(17)).unwrap();
            let baseline = prepare_patch(&row, &resolved, &index).unwrap();
            let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
            assert_eq!(
                prepared.events.len(),
                lines.len(),
                "{source}: missing {:?}",
                lines
                    .iter()
                    .filter(|line| !prepared
                        .events
                        .iter()
                        .any(|event| { event.normalized_line == line.trim_start_matches("- ") }))
                    .collect::<Vec<_>>()
            );
            assert_eq!(prepared.raw_payload_text, baseline.raw_payload_text);
            assert_eq!(
                prepared.snapshot_payload_hash,
                baseline.snapshot_payload_hash
            );
            for (line, event) in lines.iter().zip(&prepared.events) {
                assert_eq!(
                    event.normalized_line,
                    line.trim_start_matches("- "),
                    "{source}/{line}"
                );
                let entity = if line.starts_with("- Rat King:") {
                    Some("Rat King")
                } else if line.starts_with("- Sinclair:") {
                    Some("Sinclair")
                } else {
                    None
                };
                assert_eq!(event.entity_name.as_deref(), entity, "{source}/{line}");
            }
            let flat = PatchSourceResolution {
                raw_content: format!("{} {}", lines[10], lines[19]),
                ..resolved.clone()
            };
            let prepared = prepare_api_patch(&row, &flat, &index).unwrap();
            assert_eq!(prepared.events.len(), 2, "{source}/flat");
            for (line, event) in [lines[10], lines[19]].iter().zip(&prepared.events) {
                assert_eq!(event.normalized_line, line.trim_start_matches("- "));
                assert_eq!(event.subject.as_deref(), event.entity_name.as_deref());
            }
            for first in [lines[10], lines[11], lines[16], lines[19]] {
                let flat = PatchSourceResolution {
                    raw_content: format!("{first} {}", lines[0]),
                    ..resolved.clone()
                };
                let prepared = prepare_api_patch(&row, &flat, &index).unwrap();
                assert_eq!(prepared.events.len(), 2, "{source}/{first}");
                assert_eq!(
                    prepared.events[0].normalized_line,
                    first.trim_start_matches("- ")
                );
                assert_eq!(
                    prepared.events[1].normalized_line,
                    lines[0].trim_start_matches("- ")
                );
                assert_eq!(prepared.events[1].entity_name, None);
            }
        }
    }

    #[test]
    fn entity_headings_bind_all_gameplay_siblings_to_prepared_events() {
        let mut index = EntityIndex::default();
        index.insert("hero", "Holliday", "Holliday");
        index.insert("hero", "Holliday", "Powder Keg");
        index.insert("item", "Metal Skin", "Metal Skin");
        index.insert("item", "Improved Spirit", "Improved Spirit");
        index.insert("item", "Improved Burst", "Improved Burst");
        let index = index.finish();
        let mut changes = [
            "charge",
            "charges",
            "jump",
            "jumps",
            "dash",
            "dashes",
            "knockback",
            "knockdown",
            "stun",
            "stuns",
            "root",
            "roots",
            "silence",
            "cast",
            "casting",
            "projectile",
            "projectiles",
            "bounce",
            "bounces",
            "stack",
            "stacks",
            "slow",
            "slows",
            "invulnerability",
            "invulnerable",
            "immunity",
            "immune",
        ]
        .map(|word| format!("Added {word}"))
        .to_vec();
        changes.extend([
            "Increased from 1 to 2".into(),
            "Reduced from 2 to 1".into(),
            "Decreased from 2% to 1%".into(),
            "Slow increased from 20% to 30%".into(),
            "Invulnerability increased from 3 to 4".into(),
            "Bullet velocity increased from 1000 to 1200".into(),
            "Bullet spread reduced from 4 to 3".into(),
            "Fire rate increased from 5 to 6".into(),
            "Hitbox increased from 1m to 2m".into(),
            "Hitbox redesigned".into(),
            "Dispersion improved".into(),
        ]);
        for (heading, entity, section) in [
            ("Holliday", "Holliday", "Holliday"),
            ("Holliday:", "Holliday", "Holliday"),
            ("[ Holliday ]", "Holliday", "Holliday"),
            (
                "Powder Keg: Ability Changes",
                "Holliday",
                "Powder Keg: Ability Changes",
            ),
            ("Metal Skin", "Metal Skin", "Metal Skin"),
            ("Improved Spirit", "Improved Spirit", "Improved Spirit"),
            ("[ Improved Burst ]", "Improved Burst", "Improved Burst"),
            (
                "Improved Spirit: Item Changes",
                "Improved Spirit",
                "Improved Spirit: Item Changes",
            ),
        ] {
            for change in &changes {
                for source in ["steam", "forum"] {
                    let mut post = post();
                    post.source = source.into();
                    if source == "forum" {
                        post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
                    }
                    let original =
                        format!("{heading}\n- Damage increased from 50 to 60\n- {change}");
                    let html = if source == "forum" {
                        forum_html(&post, &original)
                    } else {
                        steam_html(&post, &original)
                    };
                    let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                        .unwrap()
                        .unwrap();
                    let row = post_row(&post, Some(17)).unwrap();
                    let baseline = prepare_patch(&row, &resolved, &index).unwrap();
                    let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
                    assert_eq!(prepared.events.len(), 2, "{source}/{original}");
                    assert_eq!(prepared.raw_payload_text, baseline.raw_payload_text);
                    assert_eq!(
                        prepared.snapshot_payload_hash,
                        baseline.snapshot_payload_hash
                    );
                    assert!(
                        prepared.events.iter().all(|event| {
                            event.entity_name.as_deref() == Some(entity)
                                && event.section.as_deref() == Some(section)
                        }),
                        "{source}/{original}"
                    );
                    assert!(prepared.events[1].normalized_line.ends_with(change));
                    let only_sibling = format!("{heading}\n- {change}");
                    assert!(candidate(&only_sibling, &index), "{only_sibling}");
                }
            }
        }
    }

    #[test]
    fn recognized_patch_keeps_bound_and_unbound_changes_without_clause_vetoes() {
        let index = patch_index();
        let lines = [
            "- Holliday: Fixed damage being applied twice during the attack animation",
            "- Holliday: Fixed damage being applied twice at the end of the attack animation",
            "- Holliday: Fixed damage being applied twice right as the attack animation ends",
            "- Holliday: Attack animation no longer interrupts reload",
            "- Holliday: added artwork; added knockback; added new hero skins",
            "- Metal Skin: improved damage animations",
            "- Added new artwork and increased weapon damage from 50 to 60",
            "- Added knockdown",
            "Added unknown icons",
            "- Unknown: adjusted targeting",
            "- Improved unknown visuals",
        ];
        let original = format!("{}\nHolliday skins and item icons.", lines.join("\n"));
        for source in ["steam", "forum"] {
            let mut post = post();
            post.source = source.into();
            if source == "forum" {
                post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
            }
            let html = if source == "forum" {
                forum_html(&post, &original)
            } else {
                steam_html(&post, &original)
            };
            let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                .unwrap()
                .unwrap();
            let row = post_row(&post, Some(17)).unwrap();
            let baseline = prepare_patch(&row, &resolved, &index).unwrap();
            let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
            assert!(is_patch_candidate(&prepared, &index));
            assert_eq!(prepared.events.len(), lines.len(), "{source}");
            assert_eq!(prepared.raw_payload_text, baseline.raw_payload_text);
            assert_eq!(prepared.raw_payload_hash, baseline.raw_payload_hash);
            assert_eq!(
                prepared.snapshot_payload_text,
                baseline.snapshot_payload_text
            );
            assert_eq!(
                prepared.snapshot_payload_hash,
                baseline.snapshot_payload_hash
            );
            for (position, (line, event)) in lines.iter().zip(&prepared.events).enumerate() {
                assert_eq!(
                    event.normalized_line,
                    line.trim_start_matches("- "),
                    "{source}/{position}"
                );
                assert_eq!(event.metadata["patch_external_id"], "patch_17");
                assert_eq!(event.metadata["source_kind"], source);
                assert_eq!(event.metadata["url"], post.link);
                if position < 5 {
                    assert_eq!(event.entity_name.as_deref(), Some("Holliday"));
                } else if position == 5 {
                    assert_eq!(event.entity_name.as_deref(), Some("Metal Skin"));
                } else if position >= 7 {
                    assert_eq!(event.entity_name, None, "{source}/{line}");
                }
            }
        }
    }

    #[test]
    fn change_qualifiers_stay_with_their_change() {
        let index = patch_index();
        let row = post_row(&post(), Some(17)).unwrap();
        for line in [
            "- Sinclair: Damage increased from 50 to 60 - only against objectives",
            "- Sinclair: Damage increased from 50 to 60 - only after fixing a reload bug",
            "- Sinclair: Vexing Bolt - Now does half damage to objectives - only while charging",
        ] {
            let resolved = PatchSourceResolution {
                raw_content: line.into(),
                ..PatchSourceResolution::from_row(&row)
            };
            for prepared in [
                prepare_patch(&row, &resolved, &index).unwrap(),
                prepare_api_patch(&row, &resolved, &index).unwrap(),
            ] {
                assert!(is_patch_candidate(&prepared, &index));
                assert_eq!(prepared.events.len(), 1, "{line}");
                assert_eq!(prepared.events[0].entity_name.as_deref(), Some("Sinclair"));
                assert_eq!(
                    prepared.events[0].normalized_line,
                    line.trim_start_matches("- ")
                );
            }
        }
    }

    #[test]
    fn section_changes_keep_unknown_subjects_unbound() {
        let index = patch_index();
        let row = post_row(&post(), Some(17)).unwrap();
        let resolved = PatchSourceResolution {
            raw_content: "Holliday\n- Added knockback\n- Unknown: added a stun\nGeneral Changes\n- Added knockdown\nadded unknown artwork\nHolliday\n[ New Heroes ]\n- Added an unknown effect".into(),
            ..PatchSourceResolution::from_row(&row)
        };
        let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
        assert!(is_patch_candidate(&prepared, &index));
        assert_eq!(prepared.events.len(), 5);
        assert_eq!(prepared.events[0].entity_name.as_deref(), Some("Holliday"));
        assert!(prepared.events[1..]
            .iter()
            .all(|event| event.entity_name.is_none()));
        let resolved = PatchSourceResolution {
            raw_content: "Holliday\n- Unknown: adjusted targeting".into(),
            ..resolved
        };
        let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
        assert!(is_patch_candidate(&prepared, &index));
        assert_eq!(prepared.events.len(), 1);
        assert!(prepared.events[0].entity_name.is_none());
    }

    #[test]
    fn complete_original_and_patch_recognition_are_independent() {
        assert!(has_complete_patch_content("No gameplay changes."));
        assert!(!candidate("No gameplay changes.", &patch_index()));
        assert!(!has_complete_patch_content(""));
        assert!(!has_complete_patch_content(
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
                assert!(candidate(&cleaned, &patch_index()));
                assert!(!has_complete_patch_content(&preview), "{link}");
            }
        }
        assert!(has_complete_patch_content("No gameplay changes."));
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

    fn forum_html(post: &ApiPatchPost, body: &str) -> String {
        let url = trusted_original_url(&post.link).unwrap();
        let thread_id = forum_thread_id(&url).unwrap();
        let original = json!({"mainEntity": {"@type":"DiscussionForumPosting",
            "url":canonical_post_url(&url).as_str(),
            "text":clean_steam_content(&cleanup_patch_content(body))}});
        format!("<html data-content-key=\"thread-{thread_id}\"><script type=\"application/ld+json\">{original}</script><article class=\"js-post\" data-content=\"post-1\"><div class=\"bbWrapper\">{body}</div></article></html>")
    }

    #[test]
    fn forum_original_binding_rejects_replies_foreign_threads_and_missing_metadata() {
        let mut post = post();
        post.source = "forum".into();
        post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
        let original = "- Weapon damage increased from 50 to 60";
        let valid = forum_html(&post, original);
        assert!(resolve_api_source(&post, |_| Ok(valid.clone()))
            .unwrap()
            .is_some());
        for invalid in [
            valid.replace("thread-75046", "thread-75047"),
            valid.replace("application/ld+json", "text/plain"),
            valid.replace("DiscussionForumPosting", "Comment"),
            valid.replace(
                "\"url\":\"https://forums.playdeadlock.com/threads/update.75046\"",
                "\"url\":\"https://forums.playdeadlock.com/threads/update.75047\"",
            ),
            valid.replace(
                "<div class=\"bbWrapper\">- Weapon damage increased from 50 to 60",
                "<div class=\"bbWrapper\">- Weapon damage increased from 70 to 80",
            ),
        ] {
            assert_ne!(valid, invalid);
            assert!(resolve_api_source(&post, |_| Ok(invalid.clone()))
                .unwrap()
                .is_none());
        }
        for suffix in ["page-2", "?page=2", "unread", "?order=desc"] {
            post.link = format!("https://forums.playdeadlock.com/threads/update.75046/{suffix}");
            assert!(validate_post(&post).is_err(), "{suffix}");
        }
    }

    #[test]
    fn bound_changes_include_cosmetics_and_animation_fixes() {
        let mut index = EntityIndex::default();
        index.insert("item", "Metal Skin", "Metal Skin");
        index.insert("hero", "Holliday", "Holliday");
        let index = index.finish();
        let mut post = post();
        post.source = "forum".into();
        post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
        for original in [
            "- Metal Skin: cooldown reduced from 22 to 20",
            "* Metal Skin: cooldown reduced from 22 to 20",
            "Metal Skin: cooldown reduced from 22 to 20",
            "[ Items ] * Metal Skin: cooldown reduced from 22 to 20",
            "Holliday: added knockback",
            "* Holliday: added knockback",
            "• Holliday: adjusted targeting",
            "- Holliday: Fixed damage being applied twice during the attack animation",
            "- Holliday: Fixed damage being applied twice at the end of the attack animation",
            "- Holliday: Fixed damage being applied twice right as the attack animation ends",
            "- Holliday: Attack animation no longer applies damage twice",
            "- Holliday: Attack animation no longer deals damage twice",
            "- Holliday: Attack animation no longer causes damage twice",
            "- Holliday: Attack animation no longer interrupts reload",
            "- Holliday: Fixed health not increasing with the icon on screen",
            "- Holliday: Fixed damage being applied twice after the attack animation",
            "- Holliday: Fixed healing not working when the sound plays",
            "- Holliday: Fixed damage being applied twice while the animation plays",
        ] {
            let html = forum_html(&post, original);
            let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                .unwrap()
                .unwrap();
            let prepared =
                prepare_api_patch(&post_row(&post, Some(17)).unwrap(), &resolved, &index).unwrap();
            assert_eq!(prepared.events.len(), 1, "{original}");
            assert_eq!(
                prepared.events[0].entity_name.as_deref(),
                Some(if original.contains("Metal Skin") {
                    "Metal Skin"
                } else {
                    "Holliday"
                })
            );
        }
        for original in [
            "- Metal Skin: added artwork",
            "* Metal Skin: added new hero skins",
            "Metal Skin: improved animations",
            "Metal Skin: improved animations during attacks",
            "Metal Skin: improved damage animations",
            "Metal Skin: fixed damage number animations",
            "Metal Skin: added artwork after taking damage",
        ] {
            assert!(candidate(original, &index), "{original}");
        }
    }

    fn steam_html(post: &ApiPatchPost, body: &str) -> String {
        let events = json!([{
            "gid":"703281025618282632",
            "event_name":post.title,
            "rtime32_start_time":1791241532_i64,
            "announcement_body":{"gid":"184467000000000001","headline":post.title,"body":body}
        }]);
        let encoded = events
            .to_string()
            .replace('&', "&amp;")
            .replace('"', "&quot;");
        format!(
            "<meta property=\"og:url\" content=\"{}\"><div data-partnereventstore=\"{encoded}\"></div>",
            post.link
        )
    }

    #[test]
    fn steam_original_urls_require_exact_numeric_identity_before_fetching() {
        let mut post = post();
        let html = format!(
            "<a href=\"https://store.steampowered.com/news/app/1422450/view/703281025618282632\">Other event</a>{}",
            steam_html(&post, "- Weapon damage increased from 50 to 60")
        );
        for path in [
            "https://store.steampowered.com/news/app/1422450/view/",
            "https://store.steampowered.com/news/app/1422450/view/abc",
            "https://store.steampowered.com/news/app/1422450/view/703281025618282632/other",
            "https://store.steampowered.com/news/app/1422450/view/703281025618282632//",
            "https://steamcommunity.com/games/1422450/announcements/",
            "https://steamcommunity.com/games/1422450/announcements/detail/",
            "https://steamcommunity.com/games/1422450/announcements/detail/abc",
            "https://steamcommunity.com/games/1422450/announcements/detail/184467000000000001/other",
            "https://steamcommunity.com/app/1422450/",
            "https://steamcommunity.com/app/1422450/discussions/",
            "https://steamcommunity.com/app/1422450/event/",
            "https://steamcommunity.com/app/1422450/event/abc",
            "https://steamcommunity.com/app/1422450/event/703281025618282632/other",
            "https://steamcommunity.com/app/1422450/other/announcements/detail/184467000000000001",
            "https://steamcommunity.com/app/1422450/announcements/detail/",
            "https://steamcommunity.com/app/1422450/externalpost/steam_community_announcements/",
            "https://steamcommunity.com/app/1422450/externalpost/steam_community_announcements/184467000000000001/other",
        ] {
            for suffix in ["", "?l=english#notes"] {
                post.link = format!("{path}{suffix}");
                assert!(trusted_original_url(&post.link).is_none(), "{}", post.link);
                assert!(validate_post(&post).is_err(), "{}", post.link);
                let mut fetched = false;
                assert!(resolve_api_source(&post, |_| {
                    fetched = true;
                    Ok(html.clone())
                }).is_err(), "{}", post.link);
                assert!(!fetched);
            }
        }
    }

    #[test]
    fn steam_url_forms_preserve_event_and_announcement_identity_mapping() {
        let event = "703281025618282632";
        let announcement = "184467000000000001";
        for (link, gid, canonical, wrong_gid) in [
            (format!("https://store.steampowered.com/news/app/1422450/view/{event}"), event,
                format!("https://store.steampowered.com/news/app/1422450/view/{event}"), event),
            (format!("https://steamcommunity.com/app/1422450/event/{event}"), event,
                format!("https://store.steampowered.com/news/app/1422450/view/{event}"), event),
            (format!("https://steamcommunity.com/games/1422450/announcements/detail/{announcement}"), announcement,
                format!("https://steamcommunity.com/games/1422450/announcements/detail/{announcement}"), announcement),
            (format!("https://steamcommunity.com/app/1422450/announcements/detail/{announcement}"), announcement,
                format!("https://steamcommunity.com/games/1422450/announcements/detail/{announcement}"), announcement),
            (format!("https://steamcommunity.com/app/1422450/externalpost/steam_community_announcements/{announcement}"), announcement,
                format!("https://steamcommunity.com/games/1422450/announcements/detail/{announcement}"), announcement),
        ] {
            for suffix in ["", "/", "?l=english#notes", "/?l=english#notes"] {
                let mut post = post();
                post.link = format!("{link}{suffix}");
                validate_post(&post).unwrap();
                let url = trusted_original_url(&post.link).unwrap();
                assert_eq!(steam_original_gid(&url).as_deref(), Some(gid));
                assert_eq!(canonical_post_url(&url).as_str(), canonical);
                let body = "- Weapon damage increased from 50 to 60";
                let html = format!("<a href=\"/view/123\">Other event</a>{}", steam_html(&post, body));
                assert_eq!(resolve_api_source(&post, |_| Ok(html.clone()))
                    .unwrap().unwrap().raw_content, body);
                let foreign = html.replace(wrong_gid, "123");
                assert!(resolve_api_source(&post, |_| Ok(foreign.clone()))
                    .unwrap().is_none(), "{}", post.link);
            }
        }
    }

    #[test]
    fn concrete_steam_event_never_resolves_a_foreign_or_missing_gid() {
        let index = EntityIndex::default();
        for link in [
            "https://store.steampowered.com/news/app/1422450/view/703281025618282632",
            "https://steamcommunity.com/app/1422450/event/703281025618282632",
        ] {
            let mut post = post();
            post.link = link.into();
            let body = "[p]- Weapon damage increased from 50 to 60[/p]";
            let correct = steam_html(&post, body);
            let resolved = resolve_api_source(&post, |_| Ok(correct.clone()))
                .unwrap()
                .unwrap();
            assert_eq!(resolved.raw_content, body);
            let row = post_row(&post, Some(17)).unwrap();
            let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
            assert_eq!(prepared.events.len(), 1);
            assert_eq!(prepared.events[0].metadata["url"], link);
            for invalid in [
                correct.replace("703281025618282632", "703281025618282631"),
                correct.replace("&quot;gid&quot;:&quot;703281025618282632&quot;,", ""),
                correct.replace(body, ""),
                correct.replace("data-partnereventstore", "data-other-store"),
            ] {
                assert!(resolve_api_source(&post, |_| Ok(invalid.clone()))
                    .unwrap()
                    .is_none());
            }
            let foreign = correct.replace("703281025618282632", "703281025618282631");
            assert!(resolve_api_source(&post, |_| Ok(format!(
                "<meta property=\"og:url\" content=\"{link}\">{foreign}"
            )))
            .unwrap()
            .is_none());
        }
    }

    #[test]
    fn original_fragments_use_one_request_url_and_preserve_feed_provenance() {
        let index = EntityIndex::default();
        for (source, link) in [
            ("steam", "https://store.steampowered.com/news/app/1422450/view/703281025618282632?l=english#notes"),
            ("steam", "https://steamcommunity.com/app/1422450/event/703281025618282632#notes"),
            ("forum", "https://forums.playdeadlock.com/threads/update.75046/#post-1"),
        ] {
            let mut post = post();
            post.source = source.into();
            post.link = link.into();
            validate_post(&post).unwrap();
            let request_url = trusted_original_url(link).unwrap();
            assert!(request_url.fragment().is_none());
            assert_eq!(request_url.as_str(), link.split('#').next().unwrap());
            let body = "[p]- Weapon damage increased from 50 to 60[/p]";
            let html = if source == "forum" {
                forum_html(&post, body)
            } else {
                steam_html(&post, body)
            };
            let mut requests = Vec::new();
            let resolved = resolve_api_source(&post, |url| {
                assert_eq!(url, request_url.as_str());
                requests.push(url.to_string());
                Ok(html.clone())
            })
            .unwrap()
            .unwrap();
            assert_eq!(requests, vec![request_url.to_string()]);
            assert_eq!(resolved.source_url.as_deref(), Some(request_url.as_str()));
            assert_eq!(resolved.raw_content, body);
            assert_eq!(resolved.resolved_from, Some(format!("{PATCH_FEED_URL}:{link}")));
            let row = post_row(&post, Some(17)).unwrap();
            let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
            assert_eq!(prepared.events.len(), 1);
            assert_eq!(prepared.events[0].metadata["url"], request_url.as_str());
            assert_eq!(prepared.source_external_id, request_url.as_str());
            let payload: Value = serde_json::from_str(&prepared.raw_payload_text).unwrap();
            assert_eq!(payload["source_url"], request_url.as_str());
            assert_eq!(payload["resolved_from"], format!("{PATCH_FEED_URL}:{link}"));
            assert!(resolve_api_source(&post, |_| Err(anyhow!("Abruf fehlgeschlagen"))).is_err());
        }
    }

    #[test]
    #[ignore = "needs isolated Postgres via DEADLOCK_BRAIN_SCRATCH_DSN"]
    fn canonical_original_lookup_preserves_existing_changelog_ids_without_brain_documents() {
        let dsn = env::var("DEADLOCK_BRAIN_SCRATCH_DSN").expect("Scratch-DB fehlt");
        let mut client = Client::connect(&dsn, NoTls).unwrap();
        let database: String = client
            .query_one("SELECT current_database()", &[])
            .unwrap()
            .get(0);
        assert_eq!(database, "brain_fixer12_patch_lookup");
        let mut tx = client.transaction().unwrap();
        let brain_documents: Option<String> = tx
            .query_one("SELECT to_regclass('brain.source_documents')::text", &[])
            .unwrap()
            .get(0);
        assert!(brain_documents.is_none());
        tx.batch_execute("CREATE SCHEMA patchnotes; CREATE TABLE patchnotes.changelog_posts (id BIGINT PRIMARY KEY, url TEXT NOT NULL)").unwrap();
        for (source, link) in [
            ("steam", "https://store.steampowered.com/news/app/1422450/view/703281025618282632#notes"),
            ("steam", "https://store.steampowered.com/news/app/1422450/view/703281025618282632?l=english#notes"),
            ("steam", "https://steamcommunity.com/app/1422450/event/703281025618282632#notes"),
            ("forum", "https://forums.playdeadlock.com/threads/update.75046/#post-1"),
        ] {
            let mut post = post();
            post.source = source.into();
            post.link = link.into();
            validate_post(&post).unwrap();
            let canonical = trusted_original_url(link).unwrap();
            tx.execute("INSERT INTO patchnotes.changelog_posts VALUES (17, $1)", &[&canonical.as_str()]).unwrap();
            let row = load_post_row(&mut tx, &post).unwrap();
            assert_eq!(row.id, 17, "{link}");
            let body = "[p]- Weapon damage increased from 50 to 60[/p]";
            let html = if source == "forum" {
                forum_html(&post, body)
            } else {
                steam_html(&post, body)
            };
            let index = EntityIndex::default();
            let resolved = resolve_api_source(&post, |url| {
                assert_eq!(url, canonical.as_str());
                Ok(html.clone())
            }).unwrap().unwrap();
            let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
            assert_eq!(prepared.patch_external_id, "patch_17");
            assert_eq!(prepared.events[0].metadata["patch_id"], 17);
            assert_eq!(prepared.source_external_id, canonical.as_str());
            let payload: Value = serde_json::from_str(&prepared.raw_payload_text).unwrap();
            assert_eq!(payload["resolved_from"], format!("{PATCH_FEED_URL}:{link}"));
            tx.execute("INSERT INTO patchnotes.changelog_posts VALUES (11, $1)", &[&link]).unwrap();
            assert_eq!(load_post_row(&mut tx, &post).unwrap().id, 17);
            tx.execute("DELETE FROM patchnotes.changelog_posts WHERE id=17", &[]).unwrap();
            assert_eq!(load_post_row(&mut tx, &post).unwrap().id, 11);
            tx.execute("DELETE FROM patchnotes.changelog_posts", &[]).unwrap();
            let undiscovered = load_post_row(&mut tx, &post).unwrap();
            assert!(undiscovered.id < 0);
            assert_eq!(undiscovered.id, post_row(&post, None).unwrap().id);
        }
        let store = "https://store.steampowered.com/news/app/1422450/view/703281025618282632";
        let event = "https://steamcommunity.com/app/1422450/event/703281025618282632";
        let announcement =
            "https://steamcommunity.com/games/1422450/announcements/detail/184467000000000001";
        let app_announcement =
            "https://steamcommunity.com/app/1422450/announcements/detail/184467000000000001";
        let forum = "https://forums.playdeadlock.com/threads/update.75046";
        for (stored, link) in [
            (store.to_string(), format!("{store}?l=english#notes")),
            (format!("{store}?l=english#notes"), store.to_string()),
            (store.to_string(), format!("{store}/")),
            (format!("{store}/"), store.to_string()),
            (
                format!("{store}/?l=english#notes"),
                format!("{event}#notes"),
            ),
            (format!("{event}/#notes"), format!("{store}?l=english")),
            (
                announcement.to_string(),
                format!("{app_announcement}/?l=english#notes"),
            ),
            (format!("{app_announcement}/"), announcement.to_string()),
            (forum.to_string(), format!("{forum}/?page=1#post-1")),
            (format!("{forum}/?page=1#post-1"), forum.to_string()),
            (forum.to_string(), format!("{forum}/")),
            (format!("{forum}/"), forum.to_string()),
        ] {
            let mut post = post();
            post.source = if link.starts_with("https://forums.") {
                "forum"
            } else {
                "steam"
            }
            .into();
            post.link = link.clone();
            validate_post(&post).unwrap();
            tx.execute(
                "INSERT INTO patchnotes.changelog_posts VALUES (17, $1)",
                &[&stored],
            )
            .unwrap();
            let row = load_post_row(&mut tx, &post).unwrap();
            assert_eq!(row.id, 17, "{stored} -> {link}");
            let mut row_url = trusted_original_url(&link).unwrap();
            row_url.set_query(None);
            assert_eq!(row.url.as_deref(), Some(row_url.as_str()));
            let body = "[p]- Weapon damage increased from 50 to 60[/p]";
            let html = if post.source == "forum" {
                forum_html(&post, body)
            } else {
                steam_html(&post, body)
            };
            let index = EntityIndex::default();
            let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                .unwrap()
                .unwrap();
            assert_eq!(resolved.raw_content, body);
            let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
            assert_eq!(prepared.patch_external_id, "patch_17");
            assert_eq!(prepared.events.len(), 1);
            assert_eq!(prepared.events[0].metadata["patch_id"], 17);
            let request_url = trusted_original_url(&link).unwrap();
            assert_eq!(prepared.source_external_id, request_url.as_str());
            let payload: Value = serde_json::from_str(&prepared.raw_payload_text).unwrap();
            assert_eq!(payload["source_url"], request_url.as_str());
            assert_eq!(payload["resolved_from"], format!("{PATCH_FEED_URL}:{link}"));
            let new_id = post_row(&post, None).unwrap().id;
            post.link = stored;
            validate_post(&post).unwrap();
            assert_eq!(post_row(&post, None).unwrap().id, new_id);
            tx.execute("DELETE FROM patchnotes.changelog_posts", &[])
                .unwrap();
            assert_eq!(load_post_row(&mut tx, &post).unwrap().id, new_id);
        }
        let mut known_post = post();
        known_post.link = store.into();
        let store_id = post_row(&known_post, None).unwrap().id;
        known_post.link = announcement.into();
        let announcement_id = post_row(&known_post, None).unwrap().id;
        assert_ne!(store_id, announcement_id);
        tx.execute(
            "INSERT INTO patchnotes.changelog_posts VALUES (17, $1), (19, $2)",
            &[&store, &announcement],
        )
        .unwrap();
        for link in [
            "https://store.steampowered.com/news/app/1422450/view/703281025618282631?l=english#notes",
            "https://steamcommunity.com/app/1422450/event/703281025618282631/",
            "https://steamcommunity.com/games/1422450/announcements/detail/703281025618282632",
            "https://steamcommunity.com/app/1422450/announcements/detail/703281025618282632/",
            "https://store.steampowered.com/news/app/1422450/view/184467000000000001",
            "https://steamcommunity.com/app/1422450/event/184467000000000001",
            "https://steamcommunity.com/games/1422450/announcements/detail/184467000000000002",
        ] {
            let mut post = post();
            post.link = link.into();
            validate_post(&post).unwrap();
            let row = load_post_row(&mut tx, &post).unwrap();
            assert!(row.id < 0, "{link}");
            assert_ne!(row.id, store_id);
            assert_ne!(row.id, announcement_id);
            let html = steam_html(&post, "[p]- Weapon damage increased from 50 to 60[/p]");
            assert!(resolve_api_source(&post, |_| Ok(html.clone())).unwrap().is_none());
        }
        tx.rollback().unwrap();
    }

    #[test]
    #[ignore = "needs schema-only disposable Postgres via DEADLOCK_BRAIN_REIMPORT_SCRATCH_DSN"]
    fn original_reimport_preserves_full_events_and_persistence_bindings() {
        let dsn = env::var("DEADLOCK_BRAIN_REIMPORT_SCRATCH_DSN").expect("Scratch-DB fehlt");
        let mut client = Client::connect(&dsn, NoTls).unwrap();
        let database: String = client
            .query_one("SELECT current_database()", &[])
            .unwrap()
            .get(0);
        assert_eq!(database, "brain_p_patch_reimport");
        ensure_pg_schema(&mut client).unwrap();
        for table in [
            "source_documents",
            "entity_snapshots",
            "patch_events",
            "knowledge_events",
        ] {
            let count: i64 = client
                .query_one(&format!("SELECT count(*) FROM brain.{table}"), &[])
                .unwrap()
                .get(0);
            assert_eq!(count, 0, "Scratch-Schema muss leer sein: {table}");
        }
        client.batch_execute("CREATE SCHEMA patchnotes; CREATE TABLE patchnotes.changelog_posts (id BIGINT PRIMARY KEY, url TEXT NOT NULL);
            INSERT INTO brain.entities(entity_type, canonical_name, source, created_at, updated_at)
            VALUES ('item', 'Metal Skin', 'scratch', now(), now()), ('hero', 'Holliday', 'scratch', now(), now())").unwrap();
        let index = load_entity_index(&mut client).unwrap();
        let full = "<p>[ Items ] * Metal Skin: cooldown reduced from 22 to 20</p><p>Holliday</p><p>- Added knockback</p><p>- Holliday: Fixed damage being applied twice during the attack animation</p><p>General Changes</p><p>- Added unknown icons</p>";
        for (source, id, link) in [
            (
                "steam",
                17_i64,
                "https://store.steampowered.com/news/app/1422450/view/703281025618282632",
            ),
            (
                "forum",
                19_i64,
                "https://forums.playdeadlock.com/threads/update.75046/",
            ),
        ] {
            let mut post = post();
            post.source = source.into();
            post.link = link.into();
            post.title = format!("Scratch {source} update");
            post.content = "- Metal Skin: cooldown reduced from 22 to 20".into();
            client
                .execute(
                    "INSERT INTO patchnotes.changelog_posts VALUES ($1, $2)",
                    &[&id, &link],
                )
                .unwrap();
            let html = if source == "forum" {
                forum_html(&post, full)
            } else {
                steam_html(&post, full)
            };
            let row = load_post_row(&mut client, &post).unwrap();
            assert_eq!(row.id, id);
            let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                .unwrap()
                .unwrap();
            let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
            assert_eq!(prepared.events.len(), 4);
            let payload_hash = prepared.snapshot_payload_hash.clone();
            let event_hashes = prepared
                .events
                .iter()
                .map(|event| event.event_hash.clone())
                .collect::<Vec<_>>();
            let options = ImportPatchnoteOptions {
                patch_id: id,
                dsn_env: "DEADLOCK_BRAIN_REIMPORT_SCRATCH_DSN".into(),
                dry_run: false,
            };
            for teaser in [
                "- Metal Skin: cooldown reduced from 22 to 20",
                "Preview only",
            ] {
                post.content = teaser.into();
                let row = load_post_row(&mut client, &post).unwrap();
                let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                    .unwrap()
                    .unwrap();
                let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
                assert_eq!(prepared.snapshot_payload_hash, payload_hash);
                assert_eq!(
                    prepared
                        .events
                        .iter()
                        .map(|event| event.event_hash.clone())
                        .collect::<Vec<_>>(),
                    event_hashes
                );
                let summary = import_prepared_patch(&mut client, &options, prepared).unwrap();
                assert_eq!(summary["written"]["patch_events"], 4);
                assert_eq!(summary["written"]["knowledge_events_from_patch_events"], 4);
                let rows = client.query("SELECT pe.event_hash, pe.entity_name, pe.patch_url, sd.external_id,
                    es.payload->>'raw_content', es.payload_hash, ke.source_url,
                    ke.snapshot_id = es.id AND ke.source_document_id = sd.id AND ke.patch_event_id = pe.id AS bound,
                    pe.normalized_line
                    FROM brain.patch_events pe
                    JOIN brain.entity_snapshots es ON es.id = pe.patch_snapshot_id
                    JOIN brain.source_documents sd ON sd.id = es.source_document_id
                    JOIN brain.knowledge_events ke ON ke.patch_event_id = pe.id
                    WHERE pe.patch_external_id = $1 ORDER BY pe.line_index", &[&format!("patch_{id}")]).unwrap();
                assert_eq!(rows.len(), 4);
                for (position, stored) in rows.iter().enumerate() {
                    assert_eq!(stored.get::<_, String>(0), event_hashes[position]);
                    assert_eq!(
                        stored.get::<_, Option<String>>(1).as_deref(),
                        match position {
                            0 => Some("Metal Skin"),
                            1 | 2 => Some("Holliday"),
                            _ => None,
                        }
                    );
                    assert_eq!(stored.get::<_, String>(2), link);
                    assert_eq!(stored.get::<_, String>(3), link);
                    assert_eq!(stored.get::<_, String>(4), full);
                    assert_eq!(stored.get::<_, String>(5), payload_hash);
                    assert_eq!(stored.get::<_, String>(6), link);
                    assert!(stored.get::<_, bool>(7));
                    if position == 2 {
                        assert_eq!(stored.get::<_, String>(8), "Holliday: Fixed damage being applied twice during the attack animation");
                    } else if position == 3 {
                        assert_eq!(stored.get::<_, String>(8), "Added unknown icons");
                    }
                }
                let counts = client
                    .query_one(
                        "SELECT
                    (SELECT count(*) FROM brain.source_documents WHERE external_id=$1),
                    (SELECT count(*) FROM brain.entity_snapshots WHERE external_id=$1)",
                        &[&link],
                    )
                    .unwrap();
                assert_eq!(counts.get::<_, i64>(0), 1);
                assert_eq!(counts.get::<_, i64>(1), 1);
            }
            let invalid = if source == "forum" {
                html.replace("thread-75046", "thread-75047")
            } else {
                html.replace("703281025618282632", "703281025618282631")
            };
            assert!(resolve_api_source(&post, |_| Ok(invalid.clone()))
                .unwrap()
                .is_none());
            assert!(
                resolve_api_source(&post, |_| Err(anyhow!("Original nicht erreichbar"))).is_err()
            );
            let count: i64 = client
                .query_one(
                    "SELECT count(*) FROM brain.patch_events WHERE patch_external_id=$1",
                    &[&format!("patch_{id}")],
                )
                .unwrap()
                .get(0);
            assert_eq!(count, 4);
        }
    }

    #[test]
    fn canonical_fragment_requests_pass_the_unchanged_http_core_guard() {
        let cache = tempfile::tempdir().unwrap();
        let http = HttpClient::new("Deadlock-Brain-Test/1", cache.path()).unwrap();
        for link in [
            "https://store.steampowered.com/news/app/1422450/view/703281025618282632#notes",
            "https://forums.playdeadlock.com/threads/update.75046/#post-1",
        ] {
            let deadline = std::time::Instant::now() - Duration::from_secs(1);
            let rejected = http
                .get_bounded_until(link, SourceHttpOptions::default(), deadline)
                .unwrap_err();
            assert!(rejected
                .to_string()
                .contains("without credentials or fragment"));
            let canonical = trusted_original_url(link).unwrap();
            let after_guard = http
                .get_bounded_until(canonical.as_str(), SourceHttpOptions::default(), deadline)
                .unwrap_err();
            assert!(after_guard
                .to_string()
                .contains("source HTTP total timeout"));
        }
    }

    #[test]
    fn unlinked_feed_teaser_never_replaces_original_events() {
        let post = post();
        let full = format!(
            "{}<p>- Rat King: Scrap Grenade cooldown reduced from 20 to 18</p>",
            post.content
        );
        let html = steam_html(&post, &full);
        let mut requests = Vec::new();
        let resolved = resolve_api_source(&post, |url| {
            requests.push(url.to_string());
            Ok(html.clone())
        })
        .unwrap()
        .unwrap();
        assert_eq!(requests, vec![post.link.clone()]);
        assert_eq!(resolved.raw_content, full);
        assert_ne!(resolved.raw_content, post.content);
        let row = post_row(&post, Some(17)).unwrap();
        let prepared = prepare_api_patch(&row, &resolved, &patch_index()).unwrap();
        assert_eq!(prepared.events.len(), 2);
        assert!(prepared
            .events
            .iter()
            .any(|event| event.normalized_line.contains("cooldown reduced")));
    }

    #[test]
    fn unreadable_or_linked_original_never_falls_back_to_feed_teaser() {
        let post = post();
        assert!(has_complete_patch_content(&post.content));
        assert!(
            resolve_api_source(&post, |_| Ok("<html>No original body</html>".into()))
                .unwrap()
                .is_none()
        );
        assert!(
            resolve_api_source(&post, |_| Err(anyhow!("Originalabruf fehlgeschlagen"))).is_err()
        );
        let linked = format!(
            "{}<a href=\"https://www.playdeadlock.com/cityneversleeps\">Full patch</a>",
            post.content
        );
        assert!(
            resolve_api_source(&post, |_| Ok(steam_html(&post, &linked)))
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn original_prose_reaches_existing_parser_for_steam_and_forum() {
        let prose = "[h3]The Hideout[/h3][p]Welcome to the Hideout! The Hideout replaces the existing Dashboard UI and is your personal area to play around in while waiting for a match.[/p][h3]Hero Voting[/h3][p]Today we introduce the first of the six new heroes, Mina, with another new hero unlocking every two days.[/p][h3]Mina: Hero Spotlight[/h3][p]Killing enemies has never looked better. Mina is a glass cannon that delivers quick bursts of Spirit damage at range with her passive, Love Bites.[/p]";
        let mut index = EntityIndex::default();
        index.insert("hero", "Mina", "Mina");
        let index = index.finish();
        for source in ["steam", "forum"] {
            let mut post = post();
            post.title = "Six New Heroes".into();
            post.content = "Six New Heroes".into();
            post.source = source.into();
            let html = if source == "forum" {
                post.link = "https://forums.playdeadlock.com/threads/six-new-heroes.75046/".into();
                format!("{}<article class=\"js-post\" data-content=\"post-2\"><div class=\"bbWrapper\">- Mina: Damage increased from 10 to 20</div></article>", forum_html(&post, prose))
            } else {
                steam_html(&post, prose)
            };
            let resolved = resolve_api_source(&post, |_| Ok(html.clone()))
                .unwrap()
                .unwrap();
            assert!(candidate(&resolved.raw_content, &index));
            let prepared =
                prepare_api_patch(&post_row(&post, None).unwrap(), &resolved, &index).unwrap();
            assert_eq!(prepared.events.len(), 1, "{source}");
            assert!(prepared.events.iter().any(|event| {
                event.section.as_deref() == Some("Hero Voting")
                    && event.entity_name.as_deref() == Some("Mina")
                    && event.change_type == "added"
            }));
            let payload: Value = serde_json::from_str(&prepared.raw_payload_text).unwrap();
            assert_eq!(payload["raw_content"], prose);
        }
    }

    #[test]
    fn incidental_image_link_in_original_still_fails_closed() {
        let post = post();
        let body = format!(
            "{}<img src=\"https://cdn.steamstatic.com/patch.png\">",
            post.content
        );
        assert!(resolve_api_source(&post, |_| Ok(steam_html(&post, &body)))
            .unwrap()
            .is_none());
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
