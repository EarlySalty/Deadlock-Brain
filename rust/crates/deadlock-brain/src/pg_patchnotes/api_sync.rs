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
        let Some(resolved) = resolve_api_source(&post, &index, |url| {
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
        let options = ImportPatchnoteOptions {
            patch_id: row.id,
            dsn_env: dsn_env.into(),
            dry_run,
        };
        let mut prepared = prepare_api_patch(&row, &resolved, &index)?;
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
                prepared = prepare_api_patch(&canonical_row, &resolved, &index)?;
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
    gameplay_event_content(content, index).is_some()
}

fn prepare_api_patch(
    row: &PatchnoteRow,
    resolved: &PatchSourceResolution,
    index: &EntityIndex,
) -> Result<PreparedPatch> {
    let mut prepared = prepare_patch(row, resolved, index)?;
    let content = clean_steam_content(&cleanup_patch_content(&resolved.raw_content));
    let event_content = gameplay_event_content(&content, index).unwrap_or_default();
    prepared.events = parse_events(
        row,
        prepared.url.as_deref(),
        &prepared.source_kind,
        prepared.posted_at,
        &event_content,
        index,
    );
    Ok(prepared)
}

fn gameplay_event_content(content: &str, index: &EntityIndex) -> Option<String> {
    let mut projected = Vec::new();
    let mut has_events = false;
    for line in patch_lines(content) {
        let line = line.trim();
        if line.to_ascii_lowercase().contains("http") {
            continue;
        }
        let body = bullet_body(line);
        if body.is_none() && section_heading(line).is_some() {
            projected.push(line.to_string());
            continue;
        }
        if body.is_none() && !is_narrative_event_line(line) {
            continue;
        }
        let body = normalize_patch_line(body.as_deref().unwrap_or(line));
        let lower = body.to_ascii_lowercase();
        let mut separators = [";", "!", "?", ". ", " and ", " but ", " while "]
            .into_iter()
            .flat_map(|delimiter| {
                lower
                    .match_indices(delimiter)
                    .map(move |(start, _)| (start, delimiter))
            })
            .collect::<Vec<_>>();
        separators.sort_unstable_by_key(|(start, _)| *start);
        let mut start = 0;
        let mut inherited_subject = None;
        for (end, delimiter) in separators
            .into_iter()
            .chain(std::iter::once((body.len(), "")))
        {
            if end < start {
                continue;
            }
            let clause = body[start..end].trim();
            let (subject, _) = split_subject(clause);
            let clause = if subject.is_none() {
                inherited_subject
                    .as_ref()
                    .map(|subject| format!("{subject}: {clause}"))
                    .unwrap_or_else(|| clause.to_string())
            } else {
                clause.to_string()
            };
            if has_gameplay_change(&clause, index) {
                projected.push(format!("- {clause}"));
                has_events = true;
            }
            if matches!(delimiter, " and " | " but " | " while ") {
                inherited_subject = subject.or(inherited_subject);
            } else {
                inherited_subject = None;
            }
            start = end + delimiter.len();
        }
    }
    has_events.then(|| projected.join("\n"))
}

fn has_gameplay_change(clause: &str, index: &EntityIndex) -> bool {
    let (subject, remainder) = split_subject(clause);
    let change_subject = remainder.as_deref().unwrap_or(clause).to_ascii_lowercase();
    let bound_entity = subject
        .as_deref()
        .and_then(|subject| index.exact(subject))
        .is_some_and(|entity| {
            matches!(
                entity.entity_type.as_str(),
                "hero"
                    | "item"
                    | "ability"
                    | "objective"
                    | "objective_entity"
                    | "weapon_or_internal"
            )
        });
    let words: Vec<_> = clause
        .split(|character: char| !character.is_alphabetic())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    let cosmetic = words.iter().any(|word| {
        matches!(
            word.as_str(),
            "cosmetic"
                | "cosmetics"
                | "emote"
                | "emotes"
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
    let gameplay = words.iter().any(|word| match word.as_str() {
        "hero" | "heroes" | "ability" | "abilities" | "item" | "items" | "weapon" | "weapons"
        | "attack" | "attacks" | "damage" | "dps" | "cooldown" | "recharge" | "delay" | "cost"
        | "falloff" | "health" | "regen" | "barrier" | "shield" | "armor" | "heal" | "healing"
        | "scaling" | "bounty" | "ammo" | "reload" | "range" | "radius" | "duration" | "speed"
        | "sprint" | "movement" | "resistance" | "resist" | "lifesteal" | "stamina" | "souls"
        | "lane" | "lanes" | "trooper" | "troopers" | "creep" | "creeps" | "guardian"
        | "guardians" | "walker" | "walkers" | "patron" | "patrons" | "urn" | "matchmaking"
        | "match" | "matches" | "crash" | "crashes" => true,
        "charge" | "charges" | "jump" | "jumps" | "dash" | "dashes" | "knockback" | "knockdown"
        | "stun" | "stuns" | "root" | "roots" | "silence" | "cast" | "casting" | "projectile"
        | "projectiles" | "bounce" | "bounces" | "stack" | "stacks" => bound_entity,
        _ => false,
    });
    let entity_transition = bound_entity
        && change_subject
            .split_once(" from ")
            .is_some_and(|(action, values)| {
                matches!(action.trim(), "increased" | "reduced" | "decreased")
                    && values.split_once(" to ").is_some_and(|(old, new)| {
                        [old, new].iter().all(|value| {
                            value
                                .trim()
                                .trim_end_matches('.')
                                .trim_end_matches('%')
                                .parse::<f64>()
                                .is_ok_and(f64::is_finite)
                        })
                    })
            });
    !cosmetic
        && (gameplay || entity_transition)
        && matches!(
            dbrain_normalize::classify_change_type(&change_subject).as_str(),
            "buff" | "nerf" | "bugfix" | "added" | "removed" | "rework" | "rename"
        )
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
            raw_content: post.content.clone(),
            ..PatchSourceResolution::from_row(&row)
        };
        let prepared = prepare_api_patch(&row, &resolved, &EntityIndex::default()).unwrap();
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
            assert_eq!(
                is_patch_candidate(original, &EntityIndex::default()),
                original.contains("primary attack"),
                "{original}"
            );
            assert!(is_patch_candidate(original, &index), "{original}");
            assert!(has_complete_patch_content(original, &index), "{original}");
            for source in ["steam", "forum"] {
                let mut post = post();
                post.source = source.into();
                post.content = "Preview only".into();
                let html = if source == "forum" {
                    post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
                    format!("<article class=\"js-post\" data-content=\"post-1\"><div class=\"bbWrapper\">{original}</div></article>")
                } else {
                    steam_html(&post, original)
                };
                let resolved = resolve_api_source(&post, &index, |_| Ok(html.clone()))
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
    fn mixed_changes_project_only_gameplay_without_changing_original_provenance() {
        for original in [
            "- Added new artwork and increased weapon damage from 50 to 60",
            "- Increased weapon damage from 50 to 60 and added new artwork",
            "- Added new artwork. Weapon damage increased from 50 to 60",
            "- Weapon damage increased from 50 to 60. Added new artwork",
        ] {
            for source in ["steam", "forum"] {
                let mut post = post();
                post.source = source.into();
                post.content = "Feed teaser, not the original".into();
                let fulltext = format!("[h3]Gameplay Changes[/h3][p]{original}[/p]");
                let html = if source == "forum" {
                    post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
                    format!("<article class=\"js-post\" data-content=\"post-1\"><div class=\"bbWrapper\">{fulltext}</div></article>")
                } else {
                    steam_html(&post, &fulltext)
                };
                let index = EntityIndex::default();
                let resolved = resolve_api_source(&post, &index, |_| Ok(html.clone()))
                    .unwrap()
                    .unwrap();
                assert_eq!(resolved.raw_content, fulltext);
                let discovered_row = post_row(&post, None).unwrap();
                let discovered = prepare_api_patch(&discovered_row, &resolved, &index).unwrap();
                assert_eq!(
                    discovered.patch_external_id,
                    format!("patch_{}", discovered_row.id)
                );
                for id in [17, 99] {
                    let row = PatchnoteRow {
                        id,
                        ..discovered_row.clone()
                    };
                    let baseline = prepare_patch(&row, &resolved, &index).unwrap();
                    let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
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
                    assert_eq!(
                        prepared.raw_payload_hash,
                        stable_hash(prepared.raw_payload_text.as_bytes())
                    );
                    assert_eq!(
                        prepared.snapshot_payload_hash,
                        stable_hash(prepared.snapshot_payload_text.as_bytes())
                    );
                    for text in [&prepared.raw_payload_text, &prepared.snapshot_payload_text] {
                        let payload: Value = serde_json::from_str(text).unwrap();
                        assert_eq!(payload["raw_content"], fulltext);
                        assert_eq!(payload["id"], id);
                        assert_eq!(payload["url"], post.link);
                        assert_eq!(payload["source_url"], post.link);
                        assert_eq!(payload["source_kind"], source);
                        assert_eq!(payload["posted_at"], "2026-10-05T23:05:32+00:00");
                    }
                    let payload: Value = serde_json::from_str(&prepared.raw_payload_text).unwrap();
                    assert_eq!(
                        payload["resolved_from"],
                        resolved.resolved_from.as_deref().unwrap()
                    );
                    assert_eq!(prepared.source_external_id, post.link);
                    assert_eq!(prepared.patch_external_id, format!("patch_{id}"));
                    assert_eq!(prepared.events.len(), 1, "{source}/{original}");
                    let event = &prepared.events[0];
                    assert_eq!(event.change_type, "buff", "{original}");
                    assert_eq!(event.old_value.as_deref(), Some("50"));
                    assert_eq!(event.new_value.as_deref(), Some("60"));
                    assert!(!event
                        .normalized_line
                        .to_ascii_lowercase()
                        .contains("artwork"));
                    assert!(!event.raw_line.to_ascii_lowercase().contains("artwork"));
                    assert_eq!(event.section.as_deref(), Some("Gameplay Changes"));
                    assert_eq!(event.metadata["patch_external_id"], format!("patch_{id}"));
                    assert_eq!(event.metadata["patch_id"], id);
                    assert_eq!(event.metadata["url"], post.link);
                    assert_eq!(event.metadata["source_kind"], source);
                    assert_eq!(event.event_hash, discovered.events[0].event_hash);
                    assert_eq!(
                        event.event_hash,
                        patch_event_content_hash(
                            &row,
                            &event.entity_type,
                            event.entity_name.as_deref(),
                            event.old_value.as_deref(),
                            event.new_value.as_deref(),
                            &event.normalized_line,
                        )
                    );
                    let projected_source = PatchSourceResolution {
                        raw_content: format!("Gameplay Changes\n{}", event.raw_line),
                        ..resolved.clone()
                    };
                    let expected = prepare_patch(&row, &projected_source, &index).unwrap();
                    assert_eq!(event.event_hash, expected.events[0].event_hash);
                    assert_eq!(event.metadata, expected.events[0].metadata);
                }
                assert_eq!(resolved.raw_content, fulltext);
            }
        }
    }

    #[test]
    fn clause_subjects_are_inherited_only_within_the_same_sentence() {
        let mut index = EntityIndex::default();
        index.insert("hero", "Holliday", "Holliday");
        let index = index.finish();
        let row = post_row(&post(), Some(17)).unwrap();
        for original in [
            "- Holliday: added artwork and increased from 1 to 2 charges",
            "- Holliday: increased from 1 to 2 charges and added artwork",
            "- Holliday: added a double jump and added knockback",
        ] {
            let resolved = PatchSourceResolution {
                raw_content: original.into(),
                ..PatchSourceResolution::from_row(&row)
            };
            let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
            assert!(!prepared.events.is_empty(), "{original}");
            assert!(prepared.events.iter().all(|event| {
                event.entity_name.as_deref() == Some("Holliday")
                    && !event.normalized_line.contains("artwork")
            }));
        }
        for original in [
            "- Holliday: added artwork. Increased from 1 to 2",
            "- Holliday: increased from 1 to 2 charges. Visitor count increased from 10 to 20",
            "- Added posters featuring Holliday and added knockback",
        ] {
            let resolved = PatchSourceResolution {
                raw_content: original.into(),
                ..PatchSourceResolution::from_row(&row)
            };
            let prepared = prepare_api_patch(&row, &resolved, &index).unwrap();
            assert_eq!(
                prepared.events.len(),
                usize::from(original.contains("charges")),
                "{original}"
            );
        }
    }

    #[test]
    fn numbers_and_hero_mentions_require_a_gameplay_subject_in_the_same_clause() {
        let mut index = EntityIndex::default();
        index.insert("hero", "Holliday", "Holliday");
        let index = index.finish();
        for index in [&EntityIndex::default(), &index] {
            for original in [
                "- Increased from 1 to 2 hero skins",
                "- Increased from 1 to 2 visitors",
                "- Visitor count increased from 1 to 2",
                "- Holliday: visitor count increased from 1 to 2",
                "- Holliday: increased from 1 to 2 visitors",
                "- Holliday: increased from 1 to 2 hero skins",
                "- Added Holliday emotes",
                "- Added emotes for hero Holliday",
                "- Holliday: added banners for the community",
                "- Added posters featuring Holliday",
                "- Visitor count increased from 1 to 2 and Holliday is our favourite hero",
                "- Visitor count increased from 1 to 2. Holliday is our favourite hero",
                "- Added artwork and weapon damage will be discussed tomorrow",
                "- Added artwork. Weapon damage will be discussed tomorrow",
                "- Improved artwork while weapon damage is unchanged",
                "- Visitors: increased from 1 to 2 and Holliday: new portrait",
                "- We voted for our favourite bird. See you in the city!",
                "- Added a double jump for visitors watching Holliday",
                "- Scrap Grenade is our favourite name and visitor count increased from 1 to 2",
            ] {
                assert!(!is_patch_candidate(original, index), "{original}");
                assert!(!has_complete_patch_content(original, index), "{original}");
                let row = post_row(&post(), Some(17)).unwrap();
                let resolved = PatchSourceResolution {
                    raw_content: original.into(),
                    ..PatchSourceResolution::from_row(&row)
                };
                assert!(
                    prepare_api_patch(&row, &resolved, index)
                        .unwrap()
                        .events
                        .is_empty(),
                    "{original}"
                );
                for source in ["steam", "forum"] {
                    let mut post = post();
                    post.source = source.into();
                    let html = if source == "forum" {
                        post.link = "https://forums.playdeadlock.com/threads/update.75046/".into();
                        format!("<article class=\"js-post\" data-content=\"post-1\"><div class=\"bbWrapper\">{original}</div></article>")
                    } else {
                        steam_html(&post, original)
                    };
                    assert!(
                        resolve_api_source(&post, index, |_| Ok(html.clone()))
                            .unwrap()
                            .is_none(),
                        "{source}/{original}"
                    );
                }
            }
        }
    }

    #[test]
    fn clause_scoping_keeps_gameplay_changes_and_full_numeric_values() {
        let mut index = EntityIndex::default();
        index.insert("hero", "Holliday", "Holliday");
        let index = index.finish();
        for original in [
            "- Holliday: increased from 1.05 to 1.2",
            "- Holliday: decreased from 2% to 1%",
            "- Increased weapon damage from 50 to 60 and added new artwork",
            "- Added new artwork and increased weapon damage from 50 to 60",
            "- Added new artwork. Weapon damage increased from 50 to 60",
            "- Weapon damage increased from 50 to 60; added new artwork",
            "- Weapon damage increased from 50 to 60 while new artwork is being prepared",
            "- Weapon damage: increased from 50 to 60",
            "- Increased from 50 to 60 weapon damage",
        ] {
            assert!(is_patch_candidate(original, &index), "{original}");
            assert!(has_complete_patch_content(original, &index), "{original}");
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
        let resolved = resolve_api_source(&post, &EntityIndex::default(), |url| {
            requests.push(url.to_string());
            Ok(html.clone())
        })
        .unwrap()
        .unwrap();
        assert_eq!(requests, vec![post.link.clone()]);
        assert_eq!(resolved.raw_content, full);
        assert_ne!(resolved.raw_content, post.content);
        let row = post_row(&post, Some(17)).unwrap();
        let prepared = prepare_api_patch(&row, &resolved, &EntityIndex::default()).unwrap();
        assert_eq!(prepared.events.len(), 2);
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
            let resolved = resolve_api_source(&post, &EntityIndex::default(), |_| Ok(html.clone()))
                .unwrap()
                .unwrap();
            assert!(is_patch_candidate(&resolved.raw_content, &index));
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
