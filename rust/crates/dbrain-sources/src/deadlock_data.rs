use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde_json::{json, Map, Value};
use sqlx::PgPool;

use crate::{
    store::{
        complete_run, json_string, open_pool, stable_hash_text, EntitySnapshotInput,
        SourceDocumentInput, SourceStore,
    },
    Result, SourcesError,
};

pub const SOURCE: &str = "deadlock_data";
const REPO_URL: &str = "https://github.com/deadlock-wiki/deadlock-data.git";
const REPO_WEB_URL: &str = "https://github.com/deadlock-wiki/deadlock-data";

#[derive(Debug, Clone)]
pub struct PullDeadlockDataOptions {
    pub repo_dir: PathBuf,
    pub update_repo: bool,
}

pub async fn pull_deadlock_data(
    raw_dir: &Path,
    options: PullDeadlockDataOptions,
) -> Result<Value> {
    let pool = open_pool().await?;
    let store = SourceStore::new(&pool, raw_dir)?;
    let run_id = store.begin_run(SOURCE).await?;
    let outcome = pull_deadlock_data_inner(&store, &options).await;
    complete_run(&store, run_id, outcome).await
}

async fn pull_deadlock_data_inner(
    store: &SourceStore<'_>,
    options: &PullDeadlockDataOptions,
) -> Result<Value> {
    let previous = previous_run_metadata(store.pool()).await?;
    let git_summary = if options.update_repo {
        sync_repository(&options.repo_dir)?
    } else {
        json!({"updated": false, "mode": "local"})
    };
    let repo = read_repo_info(&options.repo_dir, git_summary)?;

    let resource_lookup = read_json_optional(&options.repo_dir, "data/json/resource-lookup.json")?
        .map(|value| build_resource_lookup(&value))
        .unwrap_or_default();
    let localizations = read_localizations(&options.repo_dir)?;
    let ability_cards_value = read_json_optional(&options.repo_dir, "data/json/ability-cards.json")?
        .unwrap_or(Value::Null);
    let ability_cards = build_ability_card_index(&ability_cards_value);

    let mut import = ImportSummary::default();
    import_version_document(store, &repo, &mut import).await?;
    import_resource_lookup(store, &repo, &resource_lookup, &mut import).await?;
    import_localizations(store, &repo, &localizations, &mut import).await?;
    import_heroes(
        store,
        &repo,
        &resource_lookup,
        &localizations,
        &mut import,
    )
    .await?;
    import_abilities(
        store,
        &repo,
        &resource_lookup,
        &localizations,
        &ability_cards,
        &mut import,
    )
    .await?;
    import_ability_cards(
        store,
        &repo,
        &resource_lookup,
        &localizations,
        &ability_cards_value,
        &mut import,
    )
    .await?;
    import_items(store, &repo, &resource_lookup, &localizations, &mut import).await?;
    import_item_cards(store, &repo, &resource_lookup, &localizations, &mut import).await?;
    import_npcs(store, &repo, &resource_lookup, &localizations, &mut import).await?;
    import_supporting_documents(store, &repo, &mut import).await?;
    import_changelogs(store, &repo, &mut import).await?;

    Ok(json!({
        "source": SOURCE,
        "trusted": true,
        "repo_dir": options.repo_dir.to_string_lossy(),
        "repo_url": REPO_URL,
        "commit_sha": repo.commit_sha,
        "commit_time": repo.commit_time,
        "previous_commit_sha": previous.commit_sha,
        "commit_changed": previous.commit_sha.as_deref() != Some(repo.commit_sha.as_str()),
        "version": repo.version,
        "previous_client_version": previous.client_version,
        "version_changed": previous.client_version.as_deref() != repo.version.get("ClientVersion").map(String::as_str),
        "git": repo.git_summary,
        "documents": import.documents,
        "snapshots": import.snapshots,
        "snapshots_by_type": import.snapshots_by_type,
        "entities": import.entities,
        "aliases": import.aliases,
        "hero_stat_profiles": import.hero_stat_profiles,
        "hero_stat_values": import.hero_stat_values,
        "changelogs": import.changelogs,
        "source_trust": "trusted",
    }))
}

#[derive(Debug, Default)]
struct PreviousRunMetadata {
    commit_sha: Option<String>,
    client_version: Option<String>,
}

async fn previous_run_metadata(pool: &PgPool) -> Result<PreviousRunMetadata> {
    let summary = sqlx::query_scalar!(
        r#"
        SELECT summary::text AS "summary!"
        FROM brain.source_runs
        WHERE source=$1 AND status='ok'
        ORDER BY id DESC
        LIMIT 1
        "#,
        SOURCE,
    )
    .fetch_optional(pool)
    .await?;

    let Some(summary) = summary else {
        return Ok(PreviousRunMetadata::default());
    };
    let value: Value = serde_json::from_str(&summary).unwrap_or(Value::Null);
    Ok(PreviousRunMetadata {
        commit_sha: text_at(&value, "commit_sha"),
        client_version: value
            .get("version")
            .and_then(|version| text_at(version, "ClientVersion")),
    })
}

#[derive(Debug)]
struct RepoInfo {
    repo_dir: PathBuf,
    commit_sha: String,
    commit_time: Option<String>,
    version: BTreeMap<String, String>,
    version_text: String,
    git_summary: Value,
}

fn sync_repository(repo_dir: &Path) -> Result<Value> {
    if repo_dir.join(".git").is_dir() {
        let before = git_output(repo_dir, &["rev-parse", "HEAD"]).ok();
        run_git(repo_dir, &["pull", "--ff-only"])?;
        let after = git_output(repo_dir, &["rev-parse", "HEAD"]).ok();
        return Ok(json!({
            "updated": true,
            "mode": "pull",
            "before": before,
            "after": after,
            "changed": before != after,
        }));
    }

    if repo_dir.exists() {
        return Err(SourcesError::invalid_input(format!(
            "deadlock-data Pfad ist kein Git-Repo: {}",
            repo_dir.to_string_lossy()
        )));
    }

    if let Some(parent) = repo_dir.parent() {
        fs::create_dir_all(parent)?;
    }
    let target = repo_dir.to_string_lossy().to_string();
    run_git(Path::new("."), &["clone", "--depth", "1", REPO_URL, &target])?;
    let after = git_output(repo_dir, &["rev-parse", "HEAD"]).ok();
    Ok(json!({
        "updated": true,
        "mode": "clone",
        "after": after,
        "changed": true,
    }))
}

fn run_git(cwd: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git").args(args).current_dir(cwd).output()?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(SourcesError::invalid_input(format!(
        "git {} fehlgeschlagen: {}",
        args.join(" "),
        stderr
    )))
}

fn git_output(cwd: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).current_dir(cwd).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(SourcesError::invalid_input(format!(
            "git {} fehlgeschlagen: {}",
            args.join(" "),
            stderr
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn read_repo_info(repo_dir: &Path, git_summary: Value) -> Result<RepoInfo> {
    let version_text = fs::read_to_string(repo_dir.join("data/version.txt"))?;
    let version = parse_version_txt(&version_text);
    let commit_sha = if repo_dir.join(".git").is_dir() {
        git_output(repo_dir, &["rev-parse", "HEAD"])?
    } else {
        "local-fixture".to_string()
    };
    let commit_time = if repo_dir.join(".git").is_dir() {
        git_output(repo_dir, &["log", "-1", "--format=%cI"]).ok()
    } else {
        None
    };
    Ok(RepoInfo {
        repo_dir: repo_dir.to_path_buf(),
        commit_sha,
        commit_time,
        version,
        version_text,
        git_summary,
    })
}

fn parse_version_txt(input: &str) -> BTreeMap<String, String> {
    let mut version = BTreeMap::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            version.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    version
}

#[derive(Debug, Default)]
struct ImportSummary {
    documents: i64,
    snapshots: i64,
    snapshots_by_type: BTreeMap<String, i64>,
    entities: i64,
    aliases: i64,
    hero_stat_profiles: i64,
    hero_stat_values: i64,
    changelogs: BTreeMap<String, i64>,
}

async fn import_version_document(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    summary: &mut ImportSummary,
) -> Result<()> {
    let rel = "data/version.txt";
    let raw_path = store.write_raw(SOURCE, rel, repo.version_text.as_bytes(), "txt")?;
    let metadata = metadata_for(repo, rel);
    let external_id = document_external_id(rel, repo);
    store.upsert_source_document(SourceDocumentInput {
        source: SOURCE,
        external_id: &external_id,
        title: Some("deadlock-data version.txt"),
        url: github_url(repo, rel).as_deref(),
        content_type: "text/plain",
        raw_path: &raw_path,
        content: repo.version_text.as_bytes(),
        metadata: &metadata,
    })
    .await?;
    summary.documents += 1;
    Ok(())
}

async fn import_resource_lookup(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    lookup: &ResourceLookup,
    summary: &mut ImportSummary,
) -> Result<()> {
    let Some((document_id, _payload)) = import_json_document(store, repo, "data/json/resource-lookup.json", summary).await? else {
        return Ok(());
    };

    for entry in &lookup.entries {
        let payload = json!({
            "lookup": entry.lookup,
            "name": entry.name,
            "key": entry.key,
            "type": entry.kind,
            "hero_name": entry.hero_name,
            "hero_key": entry.hero_key,
            "_deadlock_data": payload_metadata(repo, "data/json/resource-lookup.json"),
        });
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "resource_lookup".to_string(),
            external_id: entry.lookup.clone(),
            canonical_name: Some(entry.name.clone()),
            payload,
        };
        let _ = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, "resource_lookup");
    }

    Ok(())
}

async fn import_localizations(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    localizations: &Localizations,
    summary: &mut ImportSummary,
) -> Result<()> {
    for language in &localizations.languages {
        let rel = format!("data/localizations/{language}.json");
        let Some((document_id, payload)) = import_json_document(store, repo, &rel, summary).await? else {
            continue;
        };
        let payload = json!({
            "language": language,
            "values": payload,
            "_deadlock_data": payload_metadata(repo, &rel),
        });
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "localization".to_string(),
            external_id: language.clone(),
            canonical_name: Some(language.clone()),
            payload,
        };
        let _ = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, "localization");
    }
    Ok(())
}

async fn import_heroes(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    lookup: &ResourceLookup,
    localizations: &Localizations,
    summary: &mut ImportSummary,
) -> Result<()> {
    let Some((document_id, payload)) = import_json_document(store, repo, "data/json/hero-data.json", summary).await? else {
        return Ok(());
    };

    for (key, value) in object_entries(&payload) {
        let mut payload = payload_with_key(&key, &value, payload_metadata(repo, "data/json/hero-data.json"));
        let canonical = display_name(&key, &payload, lookup, localizations)
            .unwrap_or_else(|| key.clone());
        let entity_type = if hero_is_public(&payload) { "hero" } else { "hero_internal" };
        attach_lookup_metadata(&mut payload, lookup.by_key.get(&key));
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: entity_type.to_string(),
            external_id: key.clone(),
            canonical_name: Some(canonical.clone()),
            payload: Value::Object(payload.clone()),
        };
        let snapshot_id = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, entity_type);

        let entity_id = upsert_domain_entity(
            store.pool(),
            DomainEntityInput {
                entity_type,
                canonical_name: &canonical,
                external_id: &key,
                snapshot_id: Some(snapshot_id),
                metadata: domain_metadata(repo, "hero", &key, &payload),
                aliases: aliases_for_key(&key, &canonical, lookup, localizations),
            },
        )
        .await?;
        summary.entities += 1;
        summary.aliases +=
            insert_bound_ability_aliases(store.pool(), snapshot_id, &payload).await?;

        let (profiles, values) = upsert_hero_stats(
            store.pool(),
            snapshot_id,
            Some(entity_id),
            &canonical,
            &key,
            &Value::Object(payload),
        )
        .await?;
        summary.hero_stat_profiles += profiles;
        summary.hero_stat_values += values;
    }

    Ok(())
}

async fn import_abilities(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    lookup: &ResourceLookup,
    localizations: &Localizations,
    ability_cards: &HashMap<String, AbilityCardMeta>,
    summary: &mut ImportSummary,
) -> Result<()> {
    let Some((document_id, payload)) = import_json_document(store, repo, "data/json/ability-data.json", summary).await? else {
        return Ok(());
    };

    for (key, value) in object_entries(&payload) {
        let mut payload = payload_with_key(&key, &value, payload_metadata(repo, "data/json/ability-data.json"));
        if let Some(card) = ability_cards.get(&key) {
            payload.insert(
                "_deadlock_data_card".to_string(),
                json!({
                    "hero_key": card.hero_key,
                    "hero_name": card.hero_name,
                    "slot": card.slot,
                    "card_name": card.name,
                }),
            );
        }
        attach_lookup_metadata(&mut payload, lookup.by_key.get(&key));
        let canonical = display_name(&key, &payload, lookup, localizations)
            .unwrap_or_else(|| key.clone());
        let entity_type = if ability_is_public(&payload, lookup, ability_cards.get(&key)) {
            "ability"
        } else {
            "ability_internal"
        };
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: entity_type.to_string(),
            external_id: key.clone(),
            canonical_name: Some(canonical.clone()),
            payload: Value::Object(payload.clone()),
        };
        let snapshot_id = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, entity_type);
        let aliases = aliases_for_key(&key, &canonical, lookup, localizations);
        let _ = upsert_domain_entity(
            store.pool(),
            DomainEntityInput {
                entity_type,
                canonical_name: &canonical,
                external_id: &key,
                snapshot_id: Some(snapshot_id),
                metadata: domain_metadata(repo, "ability", &key, &payload),
                aliases,
            },
        )
        .await?;
        summary.entities += 1;
    }

    Ok(())
}

async fn import_ability_cards(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    lookup: &ResourceLookup,
    localizations: &Localizations,
    ability_cards_value: &Value,
    summary: &mut ImportSummary,
) -> Result<()> {
    let Some((document_id, _payload)) = import_json_document(store, repo, "data/json/ability-cards.json", summary).await? else {
        return Ok(());
    };

    for (hero_key, hero_cards) in object_entries(ability_cards_value) {
        let Some(hero_cards) = hero_cards.as_object() else {
            continue;
        };
        let hero_name = text_field(hero_cards, "Name");
        let mut slots = hero_cards
            .iter()
            .filter_map(|(slot, value)| slot.parse::<u8>().ok().map(|number| (number, slot, value)))
            .collect::<Vec<_>>();
        slots.sort_by_key(|(number, _, _)| *number);
        for (_number, slot, value) in slots {
            let Some(card) = value.as_object() else {
                continue;
            };
            let Some(key) = text_field(card, "Key") else {
                continue;
            };
            let mut payload = card.clone();
            payload.insert("HeroKey".to_string(), Value::String(hero_key.clone()));
            if let Some(hero_name) = &hero_name {
                payload.insert("HeroName".to_string(), Value::String(hero_name.clone()));
            }
            payload.insert("Slot".to_string(), Value::String(slot.clone()));
            payload.insert(
                "_deadlock_data".to_string(),
                payload_metadata(repo, "data/json/ability-cards.json"),
            );
            attach_lookup_metadata(&mut payload, lookup.by_key.get(&key));
            let canonical = display_name(&key, &payload, lookup, localizations)
                .unwrap_or_else(|| key.clone());
            let snapshot = EntitySnapshotInput {
                source: SOURCE.to_string(),
                entity_type: "ability_card".to_string(),
                external_id: key.clone(),
                canonical_name: Some(canonical.clone()),
                payload: Value::Object(payload.clone()),
            };
            let snapshot_id = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
            increment_snapshot(summary, "ability_card");
            let _ = upsert_domain_entity(
                store.pool(),
                DomainEntityInput {
                    entity_type: "ability",
                    canonical_name: &canonical,
                    external_id: &key,
                    snapshot_id: Some(snapshot_id),
                    metadata: domain_metadata(repo, "ability_card", &key, &payload),
                    aliases: aliases_for_key(&key, &canonical, lookup, localizations),
                },
            )
            .await?;
            summary.entities += 1;
        }
    }

    Ok(())
}

async fn import_items(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    lookup: &ResourceLookup,
    localizations: &Localizations,
    summary: &mut ImportSummary,
) -> Result<()> {
    let Some((document_id, payload)) = import_json_document(store, repo, "data/json/item-data.json", summary).await? else {
        return Ok(());
    };

    for (key, value) in object_entries(&payload) {
        let mut payload = payload_with_key(&key, &value, payload_metadata(repo, "data/json/item-data.json"));
        attach_lookup_metadata(&mut payload, lookup.by_key.get(&key));
        let canonical = display_name(&key, &payload, lookup, localizations)
            .unwrap_or_else(|| key.clone());
        let entity_type = if item_is_public(&payload, lookup.by_key.get(&key)) {
            "item"
        } else {
            "item_special"
        };
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: entity_type.to_string(),
            external_id: key.clone(),
            canonical_name: Some(canonical.clone()),
            payload: Value::Object(payload.clone()),
        };
        let snapshot_id = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, entity_type);
        let _ = upsert_domain_entity(
            store.pool(),
            DomainEntityInput {
                entity_type,
                canonical_name: &canonical,
                external_id: &key,
                snapshot_id: Some(snapshot_id),
                metadata: domain_metadata(repo, "item", &key, &payload),
                aliases: aliases_for_key(&key, &canonical, lookup, localizations),
            },
        )
        .await?;
        summary.entities += 1;
    }

    Ok(())
}

async fn import_item_cards(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    lookup: &ResourceLookup,
    localizations: &Localizations,
    summary: &mut ImportSummary,
) -> Result<()> {
    let Some((document_id, payload)) = import_json_document(store, repo, "data/json/item-cards.json", summary).await? else {
        return Ok(());
    };

    for (key, value) in object_entries(&payload) {
        let mut payload = payload_with_key(&key, &value, payload_metadata(repo, "data/json/item-cards.json"));
        attach_lookup_metadata(&mut payload, lookup.by_key.get(&key));
        let canonical = display_name(&key, &payload, lookup, localizations)
            .unwrap_or_else(|| key.clone());
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "item_card".to_string(),
            external_id: key.clone(),
            canonical_name: Some(canonical.clone()),
            payload: Value::Object(payload.clone()),
        };
        let snapshot_id = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, "item_card");
        let entity_type = if item_is_public(&payload, lookup.by_key.get(&key)) {
            "item"
        } else {
            "item_special"
        };
        let _ = upsert_domain_entity(
            store.pool(),
            DomainEntityInput {
                entity_type,
                canonical_name: &canonical,
                external_id: &key,
                snapshot_id: Some(snapshot_id),
                metadata: domain_metadata(repo, "item_card", &key, &payload),
                aliases: aliases_for_key(&key, &canonical, lookup, localizations),
            },
        )
        .await?;
        summary.entities += 1;
    }

    Ok(())
}

async fn import_npcs(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    lookup: &ResourceLookup,
    localizations: &Localizations,
    summary: &mut ImportSummary,
) -> Result<()> {
    let Some((document_id, payload)) = import_json_document(store, repo, "data/json/npc-data.json", summary).await? else {
        return Ok(());
    };

    for (key, value) in object_entries(&payload) {
        let mut payload = payload_with_key(&key, &value, payload_metadata(repo, "data/json/npc-data.json"));
        attach_lookup_metadata(&mut payload, lookup.by_key.get(&key));
        let canonical = display_name(&key, &payload, lookup, localizations)
            .unwrap_or_else(|| key.clone());
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "npc_unit".to_string(),
            external_id: key,
            canonical_name: Some(canonical),
            payload: Value::Object(payload),
        };
        let _ = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, "npc_unit");
    }

    Ok(())
}

async fn import_supporting_documents(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    summary: &mut ImportSummary,
) -> Result<()> {
    for rel in [
        "data/json/attribute-data.json",
        "data/json/generic-data.json",
        "data/json/hero-meaningful-stats.json",
        "data/json/midtown-metadata.json",
        "data/json/misc-data.json",
        "data/json/soul-unlock-data.json",
        "data/json/stat-infobox-order.json",
        "data/changelogs/tag_tree.json",
        "data/changelogs/hotfixes.json",
    ] {
        let Some((document_id, payload)) = import_json_document(store, repo, rel, summary).await? else {
            continue;
        };
        let entity_type = rel
            .trim_start_matches("data/")
            .trim_end_matches(".json")
            .replace(['/', '-'], "_");
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: entity_type.clone(),
            external_id: entity_type.clone(),
            canonical_name: Some(entity_type.clone()),
            payload: json!({
                "values": payload,
                "_deadlock_data": payload_metadata(repo, rel),
            }),
        };
        let _ = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, &entity_type);
    }

    import_component_tree(store, repo, summary).await
}

async fn import_component_tree(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    summary: &mut ImportSummary,
) -> Result<()> {
    let rel = "data/item-component-tree.txt";
    let path = repo_file_path(repo, rel);
    if !path.exists() {
        return Ok(());
    }
    let content = fs::read_to_string(&path)?;
    let raw_path = store.write_raw(SOURCE, rel, content.as_bytes(), "txt")?;
    let metadata = metadata_for(repo, rel);
    let external_id = document_external_id(rel, repo);
    let document_id = store.upsert_source_document(SourceDocumentInput {
        source: SOURCE,
        external_id: &external_id,
        title: Some("deadlock-data item-component-tree.txt"),
        url: github_url(repo, rel).as_deref(),
        content_type: "text/plain",
        raw_path: &raw_path,
        content: content.as_bytes(),
        metadata: &metadata,
    })
    .await?;
    summary.documents += 1;

    let edges = parse_component_tree(&content);
    let snapshot = EntitySnapshotInput {
        source: SOURCE.to_string(),
        entity_type: "item_component_tree".to_string(),
        external_id: "item_component_tree".to_string(),
        canonical_name: Some("item_component_tree".to_string()),
        payload: json!({
            "edges": edges,
            "raw_content": content,
            "_deadlock_data": payload_metadata(repo, rel),
        }),
    };
    let _ = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
    increment_snapshot(summary, "item_component_tree");
    Ok(())
}

async fn import_changelogs(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    summary: &mut ImportSummary,
) -> Result<()> {
    let configs = import_changelog_configs(store, repo, summary).await?;
    import_patchnote_raw_files(store, repo, &configs, summary).await?;
    import_patchnote_sidecar_files(store, repo, "data/changelogs/wiki", "patchnote_wikitext", "wiki_content", summary).await?;
    import_patchnote_sidecar_files(store, repo, "data/changelogs/versions", "patchnote_structured", "events", summary).await?;
    Ok(())
}

async fn import_changelog_configs(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    summary: &mut ImportSummary,
) -> Result<HashMap<String, ChangelogConfig>> {
    let Some((document_id, payload)) = import_json_document(store, repo, "data/changelogs/changelog_configs.json", summary).await? else {
        return Ok(HashMap::new());
    };
    let configs = parse_changelog_configs(&payload);
    let snapshot = EntitySnapshotInput {
        source: SOURCE.to_string(),
        entity_type: "changelog_config".to_string(),
        external_id: "changelog_configs".to_string(),
        canonical_name: Some("changelog_configs".to_string()),
        payload: json!({
            "configs": payload,
            "_deadlock_data": payload_metadata(repo, "data/changelogs/changelog_configs.json"),
        }),
    };
    let _ = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
    increment_snapshot(summary, "changelog_config");
    Ok(configs)
}

async fn import_patchnote_raw_files(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    configs: &HashMap<String, ChangelogConfig>,
    summary: &mut ImportSummary,
) -> Result<()> {
    let dir = repo_file_path(repo, "data/changelogs/raw");
    for path in sorted_files(&dir, Some("txt"))? {
        let Some(stem) = file_stem_string(&path) else {
            continue;
        };
        let rel = format!("data/changelogs/raw/{stem}.txt");
        let content = fs::read_to_string(&path)?;
        let raw_path = store.write_raw(SOURCE, &rel, content.as_bytes(), "txt")?;
        let config = config_for_stem(configs, &stem);
        let metadata = metadata_for(repo, &rel);
        let document_external_id = document_external_id(&rel, repo);
        let title = patch_title(&stem, config.as_ref());
        let document_id = store.upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id: &document_external_id,
            title: Some(&title),
            url: config.as_ref().and_then(|value| value.link.as_deref()),
            content_type: "text/plain",
            raw_path: &raw_path,
            content: content.as_bytes(),
            metadata: &metadata,
        })
        .await?;
        summary.documents += 1;

        let wiki_path = repo_file_path(repo, &format!("data/changelogs/wiki/{stem}.txt"));
        let version_path = repo_file_path(repo, &format!("data/changelogs/versions/{stem}.json"));
        let payload = json!({
            "id": stem,
            "title": title,
            "url": config.as_ref().and_then(|value| value.link.clone()),
            "posted_at": config.as_ref().and_then(|value| value.date.clone()).or_else(|| date_from_stem(&stem)),
            "raw_content": content,
            "_deadlock_data": {
                "source_trust": "trusted",
                "source_origin": "deadlock-wiki/deadlock-data",
                "file_path": rel,
                "forum_id": config.as_ref().and_then(|value| value.forum_id.clone()),
                "is_hero_lab": config.as_ref().and_then(|value| value.is_hero_lab),
                "wiki_wikitext_path": if wiki_path.exists() { Some(format!("data/changelogs/wiki/{stem}.txt")) } else { None },
                "structured_events_path": if version_path.exists() { Some(format!("data/changelogs/versions/{stem}.json")) } else { None },
            }
        });
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: "patchnote".to_string(),
            external_id: format!("changelogs/raw/{stem}"),
            canonical_name: Some(title),
            payload,
        };
        let _ = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, "patchnote");
        increment_counter(&mut summary.changelogs, "raw");
    }
    Ok(())
}

async fn import_patchnote_sidecar_files(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    directory_rel: &str,
    entity_type: &str,
    payload_key: &str,
    summary: &mut ImportSummary,
) -> Result<()> {
    let dir = repo_file_path(repo, directory_rel);
    let extension = if entity_type == "patchnote_structured" { "json" } else { "txt" };
    for path in sorted_files(&dir, Some(extension))? {
        let Some(stem) = file_stem_string(&path) else {
            continue;
        };
        let rel = format!("{directory_rel}/{stem}.{extension}");
        let content = fs::read(&path)?;
        let raw_path = store.write_raw(SOURCE, &rel, &content, extension)?;
        let metadata = metadata_for(repo, &rel);
        let external_id = document_external_id(&rel, repo);
        let title = format!("deadlock-data {rel}");
        let content_type = if extension == "json" {
            "application/json"
        } else {
            "text/plain"
        };
        let document_id = store.upsert_source_document(SourceDocumentInput {
            source: SOURCE,
            external_id: &external_id,
            title: Some(&title),
            url: github_url(repo, &rel).as_deref(),
            content_type,
            raw_path: &raw_path,
            content: &content,
            metadata: &metadata,
        })
        .await?;
        summary.documents += 1;

        let payload_value = if extension == "json" {
            serde_json::from_slice::<Value>(&content)?
        } else {
            Value::String(String::from_utf8_lossy(&content).into_owned())
        };
        let payload = json!({
            "id": stem,
            payload_key: payload_value,
            "_deadlock_data": payload_metadata(repo, &rel),
        });
        let snapshot = EntitySnapshotInput {
            source: SOURCE.to_string(),
            entity_type: entity_type.to_string(),
            external_id: format!("{directory_rel}/{stem}"),
            canonical_name: Some(stem),
            payload,
        };
        let _ = store.upsert_entity_snapshot_id(&snapshot, Some(document_id)).await?;
        increment_snapshot(summary, entity_type);
        increment_counter(&mut summary.changelogs, entity_type);
    }
    Ok(())
}

async fn import_json_document(
    store: &SourceStore<'_>,
    repo: &RepoInfo,
    rel: &str,
    summary: &mut ImportSummary,
) -> Result<Option<(i64, Value)>> {
    let path = repo_file_path(repo, rel);
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read(&path)?;
    let payload = serde_json::from_slice::<Value>(&content)?;
    let raw_path = store.write_raw(SOURCE, rel, &content, "json")?;
    let metadata = metadata_for(repo, rel);
    let external_id = document_external_id(rel, repo);
    let title = format!("deadlock-data {rel}");
    let document_id = store.upsert_source_document(SourceDocumentInput {
        source: SOURCE,
        external_id: &external_id,
        title: Some(&title),
        url: github_url(repo, rel).as_deref(),
        content_type: "application/json",
        raw_path: &raw_path,
        content: &content,
        metadata: &metadata,
    })
    .await?;
    summary.documents += 1;
    Ok(Some((document_id, payload)))
}

fn repo_file_path(repo: &RepoInfo, rel: &str) -> PathBuf {
    repo.repo_dir.join(rel)
}

#[derive(Debug, Clone)]
struct LookupEntry {
    lookup: String,
    name: String,
    key: String,
    kind: String,
    hero_name: Option<String>,
    hero_key: Option<String>,
}

#[derive(Debug, Default)]
struct ResourceLookup {
    entries: Vec<LookupEntry>,
    by_key: HashMap<String, Vec<LookupEntry>>,
}

fn build_resource_lookup(value: &Value) -> ResourceLookup {
    let mut lookup = ResourceLookup::default();
    let Some(object) = value.as_object() else {
        return lookup;
    };
    for (lookup_key, row) in object {
        let Some(row) = row.as_object() else {
            continue;
        };
        let Some(key) = text_field(row, "key") else {
            continue;
        };
        let Some(name) = text_field(row, "name") else {
            continue;
        };
        let entry = LookupEntry {
            lookup: lookup_key.clone(),
            name,
            key: key.clone(),
            kind: text_field(row, "type").unwrap_or_else(|| "unknown".to_string()),
            hero_name: text_field(row, "hero_name"),
            hero_key: text_field(row, "hero_key"),
        };
        lookup.by_key.entry(key).or_default().push(entry.clone());
        lookup.entries.push(entry);
    }
    lookup
}

#[derive(Debug, Default)]
struct Localizations {
    languages: Vec<String>,
    values: HashMap<String, HashMap<String, String>>,
}

fn read_localizations(repo_dir: &Path) -> Result<Localizations> {
    let dir = repo_dir.join("data/localizations");
    let mut localizations = Localizations::default();
    for path in sorted_files(&dir, Some("json"))? {
        let Some(language) = file_stem_string(&path) else {
            continue;
        };
        let payload = serde_json::from_slice::<Value>(&fs::read(&path)?)?;
        let mut values = HashMap::new();
        if let Some(object) = payload.as_object() {
            for (key, value) in object {
                if let Some(text) = value.as_str().map(str::trim).filter(|text| !text.is_empty()) {
                    values.insert(key.clone(), text.to_string());
                }
            }
        }
        localizations.languages.push(language.clone());
        localizations.values.insert(language, values);
    }
    localizations.languages.sort();
    Ok(localizations)
}

#[derive(Debug, Clone)]
struct AbilityCardMeta {
    hero_key: String,
    hero_name: Option<String>,
    slot: String,
    name: Option<String>,
}

fn build_ability_card_index(value: &Value) -> HashMap<String, AbilityCardMeta> {
    let mut index = HashMap::new();
    let Some(heroes) = value.as_object() else {
        return index;
    };
    for (hero_key, hero_cards) in heroes {
        let Some(hero_cards) = hero_cards.as_object() else {
            continue;
        };
        let hero_name = text_field(hero_cards, "Name");
        for (slot, card) in hero_cards {
            if slot.parse::<u8>().is_err() {
                continue;
            }
            let Some(card) = card.as_object() else {
                continue;
            };
            if let Some(key) = text_field(card, "Key") {
                index.insert(
                    key,
                    AbilityCardMeta {
                        hero_key: hero_key.clone(),
                        hero_name: hero_name.clone(),
                        slot: slot.clone(),
                        name: text_field(card, "Name"),
                    },
                );
            }
        }
    }
    index
}

fn read_json_optional(repo_dir: &Path, rel: &str) -> Result<Option<Value>> {
    let path = repo_dir.join(rel);
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(serde_json::from_slice::<Value>(&fs::read(path)?)?))
}

fn object_entries(value: &Value) -> Vec<(String, Value)> {
    let mut entries = Vec::new();
    if let Some(object) = value.as_object() {
        for (key, value) in object {
            entries.push((key.clone(), value.clone()));
        }
    } else if let Some(array) = value.as_array() {
        for (index, value) in array.iter().enumerate() {
            let key = value
                .as_object()
                .and_then(|object| text_field(object, "Key"))
                .unwrap_or_else(|| index.to_string());
            entries.push((key, value.clone()));
        }
    }
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
}

fn payload_with_key(key: &str, value: &Value, metadata: Value) -> Map<String, Value> {
    let mut payload = value.as_object().cloned().unwrap_or_default();
    payload
        .entry("Key".to_string())
        .or_insert_with(|| Value::String(key.to_string()));
    payload.insert("_deadlock_data".to_string(), metadata);
    payload
}

fn attach_lookup_metadata(payload: &mut Map<String, Value>, entries: Option<&Vec<LookupEntry>>) {
    let Some(entries) = entries else {
        return;
    };
    let rows = entries
        .iter()
        .map(|entry| {
            json!({
                "lookup": entry.lookup,
                "name": entry.name,
                "type": entry.kind,
                "hero_name": entry.hero_name,
                "hero_key": entry.hero_key,
            })
        })
        .collect::<Vec<_>>();
    payload.insert("_deadlock_data_lookup".to_string(), json!(rows));
}

fn hero_is_public(payload: &Map<String, Value>) -> bool {
    bool_field(payload, "IsDisabled") != Some(true)
        && bool_field(payload, "InDevelopment") != Some(true)
        && bool_field(payload, "IsSelectable") != Some(false)
        && text_field(payload, "Name").is_some()
}

fn ability_is_public(
    payload: &Map<String, Value>,
    lookup: &ResourceLookup,
    card: Option<&AbilityCardMeta>,
) -> bool {
    if bool_field(payload, "IsDisabled") == Some(true) {
        return false;
    }
    let key = text_field(payload, "Key").unwrap_or_default();
    card.is_some()
        || lookup
            .by_key
            .get(&key)
            .is_some_and(|entries| entries.iter().any(|entry| entry.kind == "ability"))
        || text_field(payload, "Name").is_some()
}

fn item_is_public(payload: &Map<String, Value>, lookup_entries: Option<&Vec<LookupEntry>>) -> bool {
    if bool_field(payload, "IsDisabled") == Some(true) {
        return false;
    }
    let has_lookup = lookup_entries
        .is_some_and(|entries| entries.iter().any(|entry| entry.kind == "item"));
    has_lookup
        || (text_field(payload, "Name").is_some()
            && (payload.get("Cost").is_some() || payload.get("Tier").is_some() || payload.get("Slot").is_some()))
}

fn display_name(
    key: &str,
    payload: &Map<String, Value>,
    lookup: &ResourceLookup,
    localizations: &Localizations,
) -> Option<String> {
    text_field(payload, "Name")
        .or_else(|| {
            lookup
                .by_key
                .get(key)
                .and_then(|entries| entries.first())
                .map(|entry| entry.name.clone())
        })
        .or_else(|| localized(localizations, "english", key))
        .or_else(|| localized(localizations, "german", key))
}

fn localized(localizations: &Localizations, language: &str, key: &str) -> Option<String> {
    localizations
        .values
        .get(language)
        .and_then(|values| values.get(key))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn aliases_for_key(
    key: &str,
    canonical: &str,
    lookup: &ResourceLookup,
    localizations: &Localizations,
) -> Vec<(String, String)> {
    let mut aliases = Vec::new();
    push_alias(&mut aliases, canonical, "canonical");
    push_alias(&mut aliases, key, "deadlock_data_key");
    if let Some(entries) = lookup.by_key.get(key) {
        for entry in entries {
            push_alias(&mut aliases, &entry.lookup, "resource_lookup");
            push_alias(&mut aliases, &entry.name, "resource_lookup");
            if let Some(hero_name) = &entry.hero_name {
                push_alias(&mut aliases, hero_name, "owner_hero");
            }
        }
    }
    if let Some(value) = localized(localizations, "english", key) {
        push_alias(&mut aliases, &value, "localized_en");
    }
    if let Some(value) = localized(localizations, "german", key) {
        push_alias(&mut aliases, &value, "localized_de");
    }
    dedupe_aliases(aliases)
}

fn push_alias(aliases: &mut Vec<(String, String)>, value: &str, kind: &str) {
    let value = value.trim();
    if !value.is_empty() {
        aliases.push((value.to_string(), kind.to_string()));
    }
}

fn dedupe_aliases(aliases: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut seen = HashSet::new();
    aliases
        .into_iter()
        .filter(|(alias, kind)| seen.insert((normalize_alias(alias), kind.clone())))
        .collect()
}

struct DomainEntityInput<'a> {
    entity_type: &'a str,
    canonical_name: &'a str,
    external_id: &'a str,
    snapshot_id: Option<i64>,
    metadata: Value,
    aliases: Vec<(String, String)>,
}

async fn upsert_domain_entity(pool: &PgPool, input: DomainEntityInput<'_>) -> Result<i64> {
    let existing =
        find_existing_entity(pool, input.entity_type, input.canonical_name, &input.aliases).await?;
    let metadata_json = if let Some((_, existing_metadata)) = &existing {
        merged_metadata_json(existing_metadata.as_deref(), &input.metadata)?
    } else {
        json_string(&input.metadata)?
    };
    let entity_id = if let Some((entity_id, _)) = existing {
        sqlx::query!(
            r#"
            UPDATE brain.entities
            SET primary_external_id=COALESCE(primary_external_id, $1),
                first_snapshot_id=COALESCE(first_snapshot_id, $2),
                metadata=$3::text::jsonb,
                updated_at=now()
            WHERE id=$4
            "#,
            input.external_id,
            input.snapshot_id,
            metadata_json,
            entity_id,
        )
        .execute(pool)
        .await?;
        entity_id
    } else {
        sqlx::query_scalar!(
            r#"
            INSERT INTO brain.entities(
              entity_type, canonical_name, primary_external_id, source,
              first_snapshot_id, metadata, created_at, updated_at
            )
            VALUES($1,$2,$3,$4,$5,$6::text::jsonb,now(),now())
            RETURNING id
            "#,
            input.entity_type,
            input.canonical_name,
            input.external_id,
            SOURCE,
            input.snapshot_id,
            metadata_json,
        )
        .fetch_one(pool)
        .await?
    };

    for (alias, kind) in input.aliases {
        insert_alias(pool, entity_id, &alias, &kind, input.external_id, input.snapshot_id).await?;
    }
    Ok(entity_id)
}

async fn find_existing_entity(
    pool: &PgPool,
    entity_type: &str,
    canonical_name: &str,
    aliases: &[(String, String)],
) -> Result<Option<(i64, Option<String>)>> {
    let direct = sqlx::query!(
        r#"
        SELECT id AS "id!", metadata::text AS "metadata_json!"
        FROM brain.entities
        WHERE entity_type=$1 AND canonical_name=$2
        "#,
        entity_type,
        canonical_name,
    )
    .fetch_optional(pool)
    .await?
    .map(|row| (row.id, Some(row.metadata_json)));
    if direct.is_some() {
        return Ok(direct);
    }

    for (alias, _) in aliases {
        let alias_norm = normalize_alias(alias);
        if alias_norm.is_empty() {
            continue;
        }
        let matched = sqlx::query!(
            r#"
            SELECT e.id AS "id!", e.metadata::text AS "metadata_json!"
            FROM brain.entity_aliases a
            JOIN brain.entities e ON e.id=a.entity_id
            WHERE e.entity_type=$1 AND a.alias_norm=$2
            ORDER BY
              CASE a.alias_kind WHEN 'canonical' THEN 0 WHEN 'snapshot_name' THEN 1 ELSE 2 END,
              e.id
            LIMIT 1
            "#,
            entity_type,
            alias_norm,
        )
        .fetch_optional(pool)
        .await?
        .map(|row| (row.id, Some(row.metadata_json)));
        if matched.is_some() {
            return Ok(matched);
        }
    }
    Ok(None)
}

async fn insert_alias(
    pool: &PgPool,
    entity_id: i64,
    alias: &str,
    kind: &str,
    external_id: &str,
    snapshot_id: Option<i64>,
) -> Result<bool> {
    let alias_norm = normalize_alias(alias);
    if alias_norm.is_empty() {
        return Ok(false);
    }
    let result = sqlx::query!(
        r#"
        INSERT INTO brain.entity_aliases(
          entity_id, alias, alias_norm, alias_kind, source,
          external_id, snapshot_id, created_at
        )
        VALUES($1,$2,$3,$4,$5,$6,$7,now())
        ON CONFLICT (entity_id, alias_norm, alias_kind) DO NOTHING
        "#,
        entity_id,
        alias,
        alias_norm,
        kind,
        SOURCE,
        external_id,
        snapshot_id,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

async fn insert_bound_ability_aliases(
    pool: &PgPool,
    snapshot_id: i64,
    hero_payload: &Map<String, Value>,
) -> Result<i64> {
    let Some(bound) = hero_payload.get("BoundAbilities").and_then(Value::as_object) else {
        return Ok(0);
    };
    let mut inserted = 0_i64;
    for ability in bound.values().filter_map(Value::as_object) {
        let Some(key) = text_field(ability, "Key") else {
            continue;
        };
        let Some(name) = text_field(ability, "Name") else {
            continue;
        };
        let aliases = vec![
            (name.clone(), "hero_bound_ability".to_string()),
            (key.clone(), "deadlock_data_key".to_string()),
        ];
        if let Some((entity_id, _)) = find_existing_entity(pool, "ability", &name, &aliases).await? {
            if insert_alias(pool, entity_id, &name, "hero_bound_ability", &key, Some(snapshot_id))
                .await?
            {
                inserted += 1;
            }
            if insert_alias(pool, entity_id, &key, "deadlock_data_key", &key, Some(snapshot_id))
                .await?
            {
                inserted += 1;
            }
        }
    }
    Ok(inserted)
}

fn merged_metadata_json(existing: Option<&str>, new_metadata: &Value) -> Result<String> {
    let mut merged = existing
        .and_then(|text| serde_json::from_str::<Value>(text).ok())
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();
    if let Some(new_object) = new_metadata.as_object() {
        for (key, value) in new_object {
            merged.insert(key.clone(), value.clone());
        }
    }
    json_string(&Value::Object(merged))
}

async fn upsert_hero_stats(
    pool: &PgPool,
    snapshot_id: i64,
    entity_id: Option<i64>,
    hero_name: &str,
    external_id: &str,
    payload: &Value,
) -> Result<(i64, i64)> {
    let payload_json = json_string(payload)?;
    let payload_hash = stable_hash_text(&payload_json);
    // Frisch geschriebene Zeilen haben keinen SQLite-Ursprung; die NOT-NULL
    // `legacy_*`-Spalten werden mit den echten PG-IDs belegt (konsistent mit
    // dem FK-COALESCE-Muster der migrierten Daten).
    let profile_id = sqlx::query_scalar!(
        r#"
        INSERT INTO brain.hero_stat_profiles(
          snapshot_id, legacy_snapshot_id, entity_id, legacy_entity_id, hero_name,
          source, external_id, payload_hash, row_number, created_at, updated_at
        )
        VALUES($1, $1, $2, $2, $3, $4, $5, $6, NULL, now(), now())
        ON CONFLICT(snapshot_id) DO UPDATE SET
          entity_id=excluded.entity_id,
          legacy_entity_id=excluded.legacy_entity_id,
          hero_name=excluded.hero_name,
          source=excluded.source,
          external_id=excluded.external_id,
          payload_hash=excluded.payload_hash,
          updated_at=now()
        RETURNING id
        "#,
        snapshot_id,
        entity_id,
        hero_name,
        SOURCE,
        external_id,
        payload_hash,
    )
    .fetch_one(pool)
    .await?;

    let values = collect_hero_stat_values(payload);
    let mut upserts = 0_i64;
    for stat in values {
        let result = sqlx::query!(
            r#"
            INSERT INTO brain.hero_stat_values(
              profile_id, legacy_profile_id, entity_id, legacy_entity_id, hero_name,
              stat_key, stat_label, numeric_value, raw_value, created_at, updated_at
            )
            VALUES($1, $1, $2, $2, $3, $4, $5, $6, $7, now(), now())
            ON CONFLICT(profile_id, stat_key) DO UPDATE SET
              entity_id=excluded.entity_id,
              legacy_entity_id=excluded.legacy_entity_id,
              hero_name=excluded.hero_name,
              stat_label=excluded.stat_label,
              numeric_value=excluded.numeric_value,
              raw_value=excluded.raw_value,
              updated_at=now()
            "#,
            profile_id,
            entity_id,
            hero_name,
            stat.key,
            stat.label,
            stat.numeric,
            stat.raw,
        )
        .execute(pool)
        .await?;
        if result.rows_affected() > 0 {
            upserts += 1;
        }
    }
    Ok((1, upserts))
}

#[derive(Debug)]
struct StatValue {
    key: String,
    label: String,
    numeric: Option<f64>,
    raw: String,
}

fn collect_hero_stat_values(payload: &Value) -> Vec<StatValue> {
    let mut values = Vec::new();
    let Some(object) = payload.as_object() else {
        return values;
    };
    for (key, value) in object {
        if should_skip_root_stat(key) {
            continue;
        }
        if let Some(number) = value.as_f64() {
            push_stat(&mut values, &stat_key(key), number, value);
        }
    }
    for (prefix, field) in [
        ("level_scaling", "LevelScaling"),
        ("spirit_scaling", "SpiritScaling"),
        ("weapon", "WeaponInfo"),
        ("weapon", "Weapon"),
    ] {
        if let Some(nested) = object.get(field).and_then(Value::as_object) {
            collect_nested_numeric_stats(&mut values, prefix, nested);
        }
    }
    values
}

fn collect_nested_numeric_stats(values: &mut Vec<StatValue>, prefix: &str, object: &Map<String, Value>) {
    for (key, value) in object {
        if let Some(number) = value.as_f64() {
            push_stat(values, &format!("{prefix}.{}", stat_key(key)), number, value);
        }
    }
}

fn push_stat(values: &mut Vec<StatValue>, key: &str, number: f64, raw: &Value) {
    values.push(StatValue {
        key: key.to_string(),
        label: stat_label(key),
        numeric: Some(number),
        raw: value_to_raw(raw),
    });
}

fn should_skip_root_stat(key: &str) -> bool {
    matches!(
        key,
        "BoundAbilities"
            | "Key"
            | "Lore"
            | "Name"
            | "Playstyle"
            | "Role"
            | "Type"
            | "Weapon"
            | "WeaponInfo"
            | "LevelScaling"
            | "SpiritScaling"
            | "_deadlock_data"
            | "_deadlock_data_lookup"
    )
}

fn stat_key(value: &str) -> String {
    let mut output = String::new();
    let mut previous_separator = true;
    let mut previous_lower_or_digit = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && previous_lower_or_digit && !output.ends_with('_') && !output.ends_with('.') {
                output.push('_');
            }
            output.push(ch.to_ascii_lowercase());
            previous_separator = false;
            previous_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        } else if ch == '.' {
            if output.ends_with('_') {
                let _ = output.pop();
            }
            output.push('.');
            previous_separator = true;
            previous_lower_or_digit = false;
        } else if !previous_separator {
            output.push('_');
            previous_separator = true;
            previous_lower_or_digit = false;
        }
    }
    output.trim_matches('_').to_string()
}

fn stat_label(key: &str) -> String {
    key.split('.')
        .map(|part| {
            part.split('_')
                .filter(|piece| !piece.is_empty())
                .map(title_ascii)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn title_ascii(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut out = String::new();
    out.push(first.to_ascii_uppercase());
    out.extend(chars);
    out
}

fn value_to_raw(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        Value::Bool(flag) => flag.to_string(),
        Value::Null => String::new(),
        other => json_string(other).unwrap_or_else(|_| other.to_string()),
    }
}

fn metadata_for(repo: &RepoInfo, rel: &str) -> Value {
    json!({
        "repo": "deadlock-wiki/deadlock-data",
        "repo_url": REPO_URL,
        "commit_sha": repo.commit_sha,
        "commit_time": repo.commit_time,
        "version": repo.version,
        "file_path": rel,
        "source_trust": "trusted",
        "source_origin": "deadlock-wiki/deadlock-data",
        "generated_by": "deadbot",
    })
}

fn payload_metadata(repo: &RepoInfo, rel: &str) -> Value {
    metadata_for(repo, rel)
}

fn domain_metadata(repo: &RepoInfo, kind: &str, key: &str, payload: &Map<String, Value>) -> Value {
    json!({
        "source_trust": "trusted",
        "source_origin": "deadlock-wiki/deadlock-data",
        "source": SOURCE,
        "kind": kind,
        "deadlock_data_key": key,
        "commit_sha": repo.commit_sha,
        "client_version": repo.version.get("ClientVersion"),
        "is_disabled": bool_field(payload, "IsDisabled"),
        "in_development": bool_field(payload, "InDevelopment"),
        "in_hero_labs": bool_field(payload, "InHeroLabs"),
        "slot": text_field(payload, "Slot"),
        "tier": payload.get("Tier").cloned(),
        "cost": payload.get("Cost").cloned(),
    })
}

fn document_external_id(rel: &str, repo: &RepoInfo) -> String {
    format!("{rel}@{}", repo.commit_sha)
}

fn github_url(repo: &RepoInfo, rel: &str) -> Option<String> {
    if repo.commit_sha == "local-fixture" {
        None
    } else {
        Some(format!("{REPO_WEB_URL}/blob/{}/{rel}", repo.commit_sha))
    }
}

fn increment_snapshot(summary: &mut ImportSummary, entity_type: &str) {
    summary.snapshots += 1;
    increment_counter(&mut summary.snapshots_by_type, entity_type);
}

fn increment_counter(map: &mut BTreeMap<String, i64>, key: &str) {
    *map.entry(key.to_string()).or_default() += 1;
}

fn sorted_files(dir: &Path, extension: Option<&str>) -> Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(extension) = extension {
            if path.extension().and_then(|value| value.to_str()) != Some(extension) {
                continue;
            }
        }
        files.push(path);
    }
    files.sort();
    Ok(files)
}

fn file_stem_string(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .map(ToString::to_string)
}

fn text_field(object: &Map<String, Value>, key: &str) -> Option<String> {
    object
        .get(key)
        .and_then(value_text)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty() && value != "null")
}

fn text_at(value: &Value, key: &str) -> Option<String> {
    value
        .as_object()
        .and_then(|object| object.get(key))
        .and_then(value_text)
}

fn value_text(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        Value::Null => None,
        _ => None,
    }
}

fn bool_field(object: &Map<String, Value>, key: &str) -> Option<bool> {
    match object.get(key) {
        Some(Value::Bool(flag)) => Some(*flag),
        Some(Value::Number(number)) => number.as_i64().map(|value| value != 0),
        Some(Value::String(text)) => match text.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" => Some(true),
            "false" | "0" | "no" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

fn normalize_alias(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_component_tree(content: &str) -> Vec<Value> {
    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line == "graph" {
                return None;
            }
            let (from, to) = line.split_once("--->")?;
            Some(json!({
                "from": from.trim(),
                "to": to.trim(),
            }))
        })
        .collect()
}

#[derive(Debug, Clone)]
struct ChangelogConfig {
    forum_id: Option<String>,
    date: Option<String>,
    link: Option<String>,
    is_hero_lab: Option<bool>,
}

fn parse_changelog_configs(value: &Value) -> HashMap<String, ChangelogConfig> {
    let mut configs = HashMap::new();
    let Some(object) = value.as_object() else {
        return configs;
    };
    for (key, value) in object {
        let Some(row) = value.as_object() else {
            continue;
        };
        configs.insert(
            key.clone(),
            ChangelogConfig {
                forum_id: text_field(row, "forum_id"),
                date: text_field(row, "date"),
                link: text_field(row, "link"),
                is_hero_lab: bool_field(row, "is_hero_lab"),
            },
        );
    }
    configs
}

fn config_for_stem(configs: &HashMap<String, ChangelogConfig>, stem: &str) -> Option<ChangelogConfig> {
    configs
        .get(stem)
        .cloned()
        .or_else(|| date_from_stem(stem).and_then(|date| configs.get(&date).cloned()))
}

fn date_from_stem(stem: &str) -> Option<String> {
    let date = stem.chars().take(10).collect::<String>();
    if date.len() == 10
        && date.chars().enumerate().all(|(index, ch)| {
            matches!(index, 4 | 7) && ch == '-' || !matches!(index, 4 | 7) && ch.is_ascii_digit()
        })
    {
        Some(date)
    } else {
        None
    }
}

fn patch_title(stem: &str, config: Option<&ChangelogConfig>) -> String {
    let date = config
        .and_then(|value| value.date.clone())
        .or_else(|| date_from_stem(stem));
    let mut title = date
        .as_deref()
        .map(us_date_title)
        .unwrap_or_else(|| stem.to_string());
    title.push_str(" Update");
    if config.and_then(|value| value.is_hero_lab).unwrap_or(false) || stem.contains("HeroLab") {
        title.push_str(" Hero Lab");
    }
    title
}

fn us_date_title(date: &str) -> String {
    let mut parts = date.split('-');
    let year = parts.next();
    let month = parts.next();
    let day = parts.next();
    match (month, day, year) {
        (Some(month), Some(day), Some(year)) => format!("{month}-{day}-{year}"),
        _ => date.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::{PgPool, PgPoolOptions};

    async fn test_pool() -> Option<PgPool> {
        let dsn = std::env::var("DEADLOCK_CENTRAL_DSN").ok()?;
        PgPoolOptions::new()
            .max_connections(2)
            .connect(&dsn)
            .await
            .ok()
    }

    /// Synthetische Entity-/Alias-Typen, die in echten Daten nicht vorkommen —
    /// erlaubt kollisionsfreies Aufraeumen (Zaehler netto unveraendert).
    const TEST_ENTITY_TYPE: &str = "__dbrain_sources_test_type__";

    async fn cleanup_entity(pool: &PgPool) {
        let _ = sqlx::query(
            "DELETE FROM brain.entity_aliases WHERE entity_id IN (SELECT id FROM brain.entities WHERE entity_type=$1)",
        )
        .bind(TEST_ENTITY_TYPE)
        .execute(pool)
        .await;
        let _ = sqlx::query("DELETE FROM brain.entities WHERE entity_type=$1")
            .bind(TEST_ENTITY_TYPE)
            .execute(pool)
            .await;
    }

    /// PG-Integration des Entity-/Alias-Schreibpfads (frueher Teil des vollen
    /// deadlock-data-Import-Tests). Nutzt einen synthetischen Entity-Typ, prueft
    /// Idempotenz und raeumt restlos wieder auf. Der volle Fixture-Import (der
    /// reale Entitaeten wie "Abrams" upserten wuerde) laeuft nicht gegen die
    /// geteilte Scratch-PG.
    #[tokio::test]
    #[ignore = "needs scratch Postgres via DEADLOCK_CENTRAL_DSN"]
    async fn upsert_domain_entity_roundtrip_is_idempotent_and_cleans_up() {
        let Some(pool) = test_pool().await else {
            return;
        };
        cleanup_entity(&pool).await;

        let aliases = vec![
            ("__DbrainTestEntity__".to_string(), "canonical".to_string()),
            ("__dbrain_alias_one__".to_string(), "resource_lookup".to_string()),
        ];
        let entity_id = upsert_domain_entity(
            &pool,
            DomainEntityInput {
                entity_type: TEST_ENTITY_TYPE,
                canonical_name: "__DbrainTestEntity__",
                external_id: "__dbrain_ext__",
                snapshot_id: None,
                metadata: json!({"source_trust": "trusted", "test": true}),
                aliases: aliases.clone(),
            },
        )
        .await
        .expect("insert entity");
        assert!(entity_id > 0);

        // Zweiter Aufruf trifft den find-existing-Pfad -> gleiche Entity-ID.
        let entity_id_again = upsert_domain_entity(
            &pool,
            DomainEntityInput {
                entity_type: TEST_ENTITY_TYPE,
                canonical_name: "__DbrainTestEntity__",
                external_id: "__dbrain_ext__",
                snapshot_id: None,
                metadata: json!({"extra": 1}),
                aliases,
            },
        )
        .await
        .expect("upsert entity again");
        assert_eq!(entity_id, entity_id_again);

        let alias_count: i64 =
            sqlx::query_scalar("SELECT count(*)::int8 FROM brain.entity_aliases WHERE entity_id=$1")
                .bind(entity_id)
                .fetch_one(&pool)
                .await
                .expect("alias count");
        assert_eq!(alias_count, 2);

        // Merge behaelt bestehende Metadaten und fuegt neue Schluessel hinzu.
        let metadata: String =
            sqlx::query_scalar("SELECT metadata::text FROM brain.entities WHERE id=$1")
                .bind(entity_id)
                .fetch_one(&pool)
                .await
                .expect("metadata");
        assert!(metadata.contains("trusted"));
        assert!(metadata.contains("extra"));

        cleanup_entity(&pool).await;
    }

    /// SONDERFALL (deferred): Dieser Cross-Crate-Test rief frueher
    /// `dbrain_normalize::parse_patchnotes_with_conn` gegen eine In-Memory-SQLite
    /// auf. Da `dbrain_normalize` in diesem Worktree noch synchron (SQLite) ist,
    /// wird er beim Seam-Schluss (normalize async) reaktiviert und auf PG-Pool +
    /// async normalize umgestellt.
    #[tokio::test]
    #[ignore = "cross-crate PG-Integration: wird beim Seam-Schluss (normalize async) reaktiviert"]
    async fn patchnote_events_do_not_duplicate_when_commit_metadata_changes() {
        // TODO(seam-close): auf PG-Pool + async normalize umstellen.
    }

    #[test]
    fn version_parser_accepts_crlf() {
        let parsed = parse_version_txt("ClientVersion=6592\r\nVersionDate=Jun 19 2026\r\n");
        assert_eq!(parsed.get("ClientVersion"), Some(&"6592".to_string()));
        assert_eq!(parsed.get("VersionDate"), Some(&"Jun 19 2026".to_string()));
    }
}
