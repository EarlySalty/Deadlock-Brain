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
    _ledger: &SteamLedger,
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
        let existing = client.query_opt(
            "SELECT id FROM patchnotes.changelog_posts WHERE url=$1 ORDER BY id LIMIT 1",
            &[&post.link],
        )?;
        let row = post_row(&post, existing.map(|row| row.get(0)))?;
        let Some(mut resolved) = resolve_api_source(&post, &index, |url| {
            let response = http.get_bounded(
                url,
                SourceHttpOptions {
                    headers: vec![("Accept".into(), "text/html".into())],
                    ..Default::default()
                },
            )?;
            response.ensure_success()?;
            Ok(std::str::from_utf8(&response.content)?.to_string())
        })?
        else {
            skipped.push(json!({"url":post.link,"reason":"preview_requires_official_fulltext","not_imported_as_patch":true}));
            continue;
        };
        if !is_patch_candidate(&resolved.raw_content, &index) {
            skipped.push(json!({"url":post.link,"reason":"announcement_without_patch_changes"}));
            continue;
        }
        resolved.raw_content = clean_steam_content(&cleanup_patch_content(&resolved.raw_content));
        let options = ImportPatchnoteOptions {
            patch_id: row.id,
            dsn_env: dsn_env.into(),
            dry_run,
        };
        let mut prepared = prepare_patch(&row, &resolved, &index)?;
        if prepared.events.is_empty() {
            skipped.push(json!({"url":post.link,"reason":"original_without_parseable_patch_events","not_imported_as_patch":true}));
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

fn is_patch_candidate(original: &str, index: &EntityIndex) -> bool {
    has_patch_changes(
        &clean_steam_content(&cleanup_patch_content(original)),
        index,
    )
}

fn has_complete_patch_content(raw: &str, index: &EntityIndex) -> bool {
    let content = cleanup_patch_content(raw);
    let lower = content.to_ascii_lowercase();
    let has_reference = ["http://", "https://", "www.", "[url=", "[url]"]
        .iter()
        .any(|marker| lower.contains(marker))
        || lower
            .split("href")
            .skip(1)
            .any(|suffix| suffix.trim_start().starts_with('='));
    let cleaned = clean_steam_content(&content);
    !has_reference && has_patch_changes(&cleaned, index)
}

fn has_patch_changes(content: &str, index: &EntityIndex) -> bool {
    patch_lines(content).iter().any(|line| {
        let line = line.trim();
        let (old_value, new_value) = extract_old_new(line);
        let has_transition = old_value.is_some() && new_value.is_some();
        let lower = line.to_ascii_lowercase();
        let change_subject = if has_transition {
            lower
                .split_once(" from ")
                .map_or(lower.as_str(), |(subject, _)| subject)
        } else {
            lower.as_str()
        };
        let words: Vec<_> = change_subject
            .split(|character: char| !character.is_alphabetic())
            .filter(|word| !word.is_empty())
            .map(str::to_ascii_lowercase)
            .collect();
        let cosmetic = words.iter().any(|word| {
            matches!(
                word.as_str(),
                "cosmetic"
                    | "cosmetics"
                    | "skin"
                    | "skins"
                    | "artwork"
                    | "portrait"
                    | "portraits"
                    | "icon"
                    | "icons"
                    | "sound"
                    | "sounds"
                    | "music"
                    | "visual"
                    | "visuals"
                    | "animation"
                    | "animations"
            )
        });
        let gameplay = words.iter().any(|word| {
            matches!(
                word.as_str(),
                "hero"
                    | "heroes"
                    | "ability"
                    | "abilities"
                    | "item"
                    | "items"
                    | "weapon"
                    | "weapons"
                    | "damage"
                    | "dps"
                    | "cooldown"
                    | "recharge"
                    | "delay"
                    | "cost"
                    | "falloff"
                    | "health"
                    | "regen"
                    | "barrier"
                    | "shield"
                    | "armor"
                    | "heal"
                    | "healing"
                    | "scaling"
                    | "bounty"
                    | "ammo"
                    | "reload"
                    | "range"
                    | "radius"
                    | "duration"
                    | "speed"
                    | "sprint"
                    | "movement"
                    | "resistance"
                    | "resist"
                    | "lifesteal"
                    | "stamina"
                    | "souls"
                    | "lane"
                    | "lanes"
                    | "trooper"
                    | "troopers"
                    | "creep"
                    | "creeps"
                    | "guardian"
                    | "guardians"
                    | "walker"
                    | "walkers"
                    | "patron"
                    | "patrons"
                    | "urn"
                    | "matchmaking"
                    | "match"
                    | "matches"
                    | "crash"
                    | "crashes"
            )
        });
        !lower.contains("http")
            && !cosmetic
            && (has_transition || index.infer(change_subject).is_some() || gameplay)
            && (bullet_body(line).is_some() || is_narrative_event_line(line))
            && matches!(
                dbrain_normalize::classify_change_type(line).as_str(),
                "buff" | "nerf" | "bugfix" | "added" | "removed" | "rework" | "rename"
            )
    })
}

fn resolve_api_source(
    post: &ApiPatchPost,
    index: &EntityIndex,
    mut fetch_html: impl FnMut(&str) -> Result<String>,
) -> Result<Option<PatchSourceResolution>> {
    let url = trusted_original_url(&post.link)
        .ok_or_else(|| anyhow!("Ungültige Originalquelle im Patchfeed"))?;
    let html = fetch_html(url.as_str())?;
    let body = if post.source == "forum" {
        Some(dbrain_sources::forum::first_post_html(&html)?)
    } else {
        let item = SteamAppNewsItem {
            gid: String::new(),
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
        .filter(|body| has_complete_patch_content(body, index))
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
        assert!(has_patch_changes(&content, &EntityIndex::default()));
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
        for source in ["steam", "forum"] {
            for title in ["Mind the Birds!", "Minor Update"] {
                for original in [
                    "<p>We voted for our favourite bird. See you in the city!</p>",
                    "<p>We voted for a new bird. See you in the city!</p>",
                    "<p>Added new hero skins and updated item icons for everyone.</p>",
                    "<p>Improved the hero artwork and animations in this update.</p>",
                    "<p>- Added a new cosmetic item for every hero</p>",
                    "<p>- Hero artwork increased from 1 to 2 variants</p>",
                    "<p>We introduce the latest community event. See you in the city!</p>",
                ] {
                    let mut post = post();
                    post.source = source.into();
                    post.title = title.into();
                    let html = if source == "forum" {
                        post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
                        format!("<article class=\"js-post\" data-content=\"post-1\"><div class=\"bbWrapper\">{original}</div></article>")
                    } else {
                        steam_html(&post, original)
                    };
                    assert!(
                        !is_patch_candidate(original, &EntityIndex::default()),
                        "{source}/{title}/{original}"
                    );
                    assert!(!has_complete_patch_content(
                        original,
                        &EntityIndex::default()
                    ));
                    assert!(resolve_api_source(&post, &EntityIndex::default(), |_| Ok(
                        html.clone()
                    ))
                    .unwrap()
                    .is_none());
                }
            }
        }
        assert!(!has_patch_changes(
            "Full update: https://www.playdeadlock.com/cityneversleeps",
            &EntityIndex::default(),
        ));
    }

    #[test]
    fn gameplay_changes_remain_candidates_without_bullets_or_update_titles() {
        for original in [
            "<p>Scrap Grenade damage increased from 65 to 70.</p>",
            "<p>Enchanter's Barrier shield increased from 300 to 350.</p>",
            "<p>The shotgun DPS increased from 50 to 60.</p>",
            "<p>Today we introduce six new heroes, with another hero unlocking every two days.</p>",
            "<p>Fixed a crash when entering a match.</p>",
            "<p>- Rat King: Scrap Grenade cooldown reduced from 20 to 18</p>",
        ] {
            assert!(
                is_patch_candidate(original, &EntityIndex::default()),
                "{original}"
            );
            assert!(
                has_complete_patch_content(original, &EntityIndex::default()),
                "{original}"
            );
        }
    }
    #[test]
    fn structured_changes_and_mixed_lines_keep_existing_parser_events() {
        let mut index = EntityIndex::default();
        index.insert("hero", "Holliday", "Holliday");
        let index = index.finish();
        for original in [
            "- Holliday: increased from 1 to 2",
            "- Increased weapon damage from 50 to 60 and added new artwork",
            "- Holliday: reworked primary attack",
        ] {
            let mut post = post();
            post.content = "Preview only".into();
            let row = post_row(&post, Some(17)).unwrap();
            let expected = prepare_patch(
                &row,
                &PatchSourceResolution {
                    raw_content: original.into(),
                    ..PatchSourceResolution::from_row(&row)
                },
                &index,
            )
            .unwrap();
            assert_eq!(expected.events.len(), 1, "{original}");
            assert_ne!(expected.events[0].change_type, "changed");
            if original.contains(" from ") {
                assert!(expected.events[0].old_value.is_some());
                assert!(expected.events[0].new_value.is_some());
                assert!(is_patch_candidate(original, &EntityIndex::default()));
            } else {
                assert_eq!(expected.events[0].entity_name.as_deref(), Some("Holliday"));
            }
            assert!(is_patch_candidate(original, &index));
            assert!(has_complete_patch_content(original, &index));
            for source in ["steam", "forum"] {
                post.source = source.into();
                let html = if source == "forum" {
                    post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
                    format!("<article class=\"js-post\" data-content=\"post-1\"><div class=\"bbWrapper\">{original}</div></article>")
                } else {
                    steam_html(&post, original)
                };
                let mut resolved = resolve_api_source(&post, &index, |_| Ok(html.clone()))
                    .unwrap()
                    .unwrap();
                resolved.raw_content =
                    clean_steam_content(&cleanup_patch_content(&resolved.raw_content));
                let actual = prepare_patch(&row, &resolved, &index).unwrap();
                assert_eq!(actual.events.len(), expected.events.len());
                assert_eq!(
                    actual.events[0].normalized_line,
                    expected.events[0].normalized_line
                );
                assert_eq!(actual.events[0].old_value, expected.events[0].old_value);
                assert_eq!(actual.events[0].new_value, expected.events[0].new_value);
            }
        }
    }

    #[test]
    fn change_bullets_do_not_make_linked_previews_complete() {
        let full = post().content;
        assert!(has_complete_patch_content(&full, &EntityIndex::default()));
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
                assert!(has_patch_changes(&cleaned, &EntityIndex::default()));
                assert!(!has_complete_patch_content(&preview, &EntityIndex::default()), "{link}");
            }
        }
        assert!(!has_complete_patch_content(
            "No gameplay changes.",
            &EntityIndex::default()
        ));
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
        assert!(!has_complete_patch_content(&raw, &EntityIndex::default()));
        assert!(has_complete_patch_content(
            &resolve_steam_content(&http, &item),
            &EntityIndex::default(),
        ));
    }

    fn steam_html(post: &ApiPatchPost, body: &str) -> String {
        let events = json!([{
            "gid":"703281025618282632",
            "event_name":post.title,
            "rtime32_start_time":1791241532_i64,
            "announcement_body":{"headline":post.title,"body":body}
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
    fn unlinked_feed_teaser_never_replaces_original_events() {
        let post = post();
        let full = format!(
            "{}<p>- Rat King: Scrap Grenade cooldown reduced from 20 to 18</p>",
            post.content
        );
        let html = steam_html(&post, &full);
        let mut requests = Vec::new();
        let mut resolved = resolve_api_source(&post, &EntityIndex::default(), |url| {
            requests.push(url.to_string());
            Ok(html.clone())
        })
        .unwrap()
        .unwrap();
        assert_eq!(requests, vec![post.link.clone()]);
        assert_eq!(resolved.raw_content, full);
        assert_ne!(resolved.raw_content, post.content);
        resolved.raw_content = clean_steam_content(&cleanup_patch_content(&resolved.raw_content));
        let row = post_row(&post, Some(17)).unwrap();
        let prepared = prepare_patch(&row, &resolved, &EntityIndex::default()).unwrap();
        assert_eq!(prepared.events.len(), 3);
        assert!(prepared
            .events
            .iter()
            .any(|event| event.normalized_line.contains("cooldown reduced")));
    }

    #[test]
    fn unreadable_or_linked_original_never_falls_back_to_feed_teaser() {
        let post = post();
        assert!(has_complete_patch_content(
            &post.content,
            &EntityIndex::default()
        ));
        assert!(resolve_api_source(&post, &EntityIndex::default(), |_| Ok(
            "<html>No original body</html>".into()
        ))
        .unwrap()
        .is_none());
        assert!(
            resolve_api_source(&post, &EntityIndex::default(), |_| Err(anyhow!(
                "Originalabruf fehlgeschlagen"
            )))
            .is_err()
        );
        let linked = format!(
            "{}<a href=\"https://www.playdeadlock.com/cityneversleeps\">Full patch</a>",
            post.content
        );
        assert!(
            resolve_api_source(&post, &EntityIndex::default(), |_| Ok(steam_html(
                &post, &linked
            )))
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
                format!("<html><article class=\"js-post\" data-content=\"post-1\"><div class=\"bbWrapper\">{prose}</div></article><article class=\"js-post\" data-content=\"post-2\"><div class=\"bbWrapper\">- Mina: Damage increased from 10 to 20</div></article></html>")
            } else {
                steam_html(&post, prose)
            };
            let mut resolved =
                resolve_api_source(&post, &EntityIndex::default(), |_| Ok(html.clone()))
                    .unwrap()
                    .unwrap();
            assert!(is_patch_candidate(&resolved.raw_content, &index));
            resolved.raw_content =
                clean_steam_content(&cleanup_patch_content(&resolved.raw_content));
            assert!(resolved
                .raw_content
                .lines()
                .all(|line| bullet_body(line).is_none()));
            let prepared =
                prepare_patch(&post_row(&post, None).unwrap(), &resolved, &index).unwrap();
            assert_eq!(prepared.events.len(), 3, "{source}");
            assert!(prepared.events.iter().any(|event| {
                event.section.as_deref() == Some("Mina: Hero Spotlight")
                    && event.entity_name.as_deref() == Some("Mina")
            }));
        }
    }

    #[test]
    fn incidental_image_link_in_original_still_fails_closed() {
        let post = post();
        let body = format!(
            "{}<img src=\"https://cdn.steamstatic.com/patch.png\">",
            post.content
        );
        assert!(
            resolve_api_source(&post, &EntityIndex::default(), |_| Ok(steam_html(
                &post, &body
            )))
            .unwrap()
            .is_none()
        );
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
