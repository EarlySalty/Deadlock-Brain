use super::{report_preflight, Arguments};
use dbrain_sources::{
    game_files::{extract_game_files, GameFileOptions},
    git_source::PinnedRepository,
    knowledge_contract::validate_knowledge_jsonl,
    knowledge_import::public_game as authorization,
};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom, Write},
};

pub(super) async fn inspect_or_authorize(
    args: &Arguments,
    pool: &sqlx::PgPool,
) -> Result<Value, String> {
    let heads = authorization::read_game_heads(pool, &args.sources).await?;
    let selection = authorization::select_game_heads(
        &heads,
        args.values
            .get("--authorization-ref")
            .map_or("inspect:dry-factual-projection", String::as_str),
        args.values.get("--provider-egress-ref").map(String::as_str),
    )?;
    if args.mode == "reauthorize-game" {
        verify_git_heads(args, &heads, &selection.selected)?;
        return authorization::reauthorize_game_heads(
            pool,
            &args.sources,
            &heads,
            args.required("--authorization-ref")?,
            args.values.get("--provider-egress-ref").map(String::as_str),
        )
        .await;
    }
    let empty_summary = || json!({"documents": 0, "content_bytes": 0, "licenses": {}, "visibility": {}, "game_origin_valid": 0, "game_origin_invalid": 0, "factual_projection_valid": 0, "factual_projection_blocked_documents": [], "publication_granted": 0, "provider_egress_granted": 0, "original_revisions": {}});
    let mut sources: BTreeMap<String, Value> = args
        .sources
        .iter()
        .map(|source| (source.clone(), empty_summary()))
        .collect();
    for record in heads {
        let summary = sources
            .get_mut(&record.source_id)
            .ok_or("Unerwartete Spielquelle beim Inventar")?;
        summary["documents"] = json!(summary["documents"].as_u64().unwrap_or(0) + 1);
        summary["content_bytes"] =
            json!(summary["content_bytes"].as_u64().unwrap_or(0) + record.content.len() as u64);
        let visibility =
            serde_json::to_value(record.visibility).map_err(|_| "Sichtbarkeit ist ungültig")?;
        let visibility = visibility.as_str().ok_or("Sichtbarkeit ist ungültig")?;
        summary["visibility"][visibility] =
            json!(summary["visibility"][visibility].as_u64().unwrap_or(0) + 1);
        let origin = brain_contracts::source::origin_from_record(&record)
            .map_err(|_| "Herkunft ist ungültig")?;
        summary["publication_granted"] = json!(
            summary["publication_granted"].as_u64().unwrap_or(0)
                + u64::from(origin.policy.publication_allowed)
        );
        summary["provider_egress_granted"] = json!(
            summary["provider_egress_granted"].as_u64().unwrap_or(0)
                + u64::from(origin.policy.provider_egress_allowed)
        );
        let revision = record
            .metadata
            .get(brain_storage::source_versions::ORIGINAL_VERSION_KEY)
            .cloned()
            .unwrap_or_else(|| "legacy-canonical".into());
        summary["original_revisions"][&revision] = json!(
            summary["original_revisions"][&revision]
                .as_u64()
                .unwrap_or(0)
                + 1
        );
        let license = match origin.policy.license {
            brain_contracts::value::Observed::Known { value } => value,
            brain_contracts::value::Observed::Unknown { .. } => "unverified".into(),
        };
        summary["licenses"][&license] =
            json!(summary["licenses"][&license].as_u64().unwrap_or(0) + 1);
        let validity = if authorization::validate_game_origin(&record).is_ok() {
            "game_origin_valid"
        } else {
            "game_origin_invalid"
        };
        summary[validity] = json!(summary[validity].as_u64().unwrap_or(0) + 1);
        let projectable = selection
            .selected
            .contains(&(record.source_id.clone(), record.logical_id.clone()));
        if projectable {
            summary["factual_projection_valid"] =
                json!(summary["factual_projection_valid"].as_u64().unwrap_or(0) + 1);
        } else {
            summary["factual_projection_blocked_documents"]
                .as_array_mut()
                .ok_or("Projektionsinventar ist ungültig")?
                .push(json!(record.logical_id));
        }
    }
    Ok(
        json!({"inspected": true, "sources": sources, "selection": selection.report(), "content_printed": false, "imported": false, "published": false, "activated": false}),
    )
}

fn repository_url(source: &str) -> Result<&'static str, String> {
    match source {
        "deadlock-wiki-deadlock-data" => Ok("https://github.com/deadlock-wiki/deadlock-data"),
        "steamtracking-gametracking-deadlock" => {
            Ok("https://github.com/SteamTracking/GameTracking-Deadlock")
        }
        _ => Err("Gepinnte Git-Spielquelle fehlt".into()),
    }
}

fn runtime_options(args: &Arguments, source: &str) -> Result<GameFileOptions, String> {
    let mut bytes = Vec::new();
    File::open(args.path("--runtime-config")?)
        .map_err(|_| "Runtime kann nicht geöffnet werden")?
        .take(2 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Runtime kann nicht gelesen werden")?;
    if bytes.len() > 2 * 1024 * 1024 {
        return Err("Runtime überschreitet die Größenbegrenzung".into());
    }
    let runtime: Value = serde_json::from_slice(&bytes).map_err(|_| "Runtime ist ungültig")?;
    let entries: Vec<_> = runtime["entity_profile_sources"]
        .as_array()
        .ok_or("Git-Spielquellen fehlen in der Runtime")?
        .iter()
        .filter(|entry| entry["repository_id"].as_str() == Some(source))
        .collect();
    if entries.len() != 1 {
        return Err("Runtime muss die Spielquelle genau einmal deklarieren".into());
    }
    let options: GameFileOptions = serde_json::from_value(entries[0]["extraction"].clone())
        .map_err(|_| "Standardextraktion ist ungültig")?;
    if options.source_id != source
        || options.app_id != 1422450
        || options.build_id.is_some()
        || options.manifest_id.is_some()
        || options.provenance["repository_url"].as_str() != Some(repository_url(source)?)
    {
        return Err("Standardextraktion passt nicht zur gepinnten Spielquelle".into());
    }
    Ok(options)
}

fn verify_git_heads(
    args: &Arguments,
    heads: &[brain_contracts::SourceRecordV2],
    selected: &std::collections::BTreeSet<(String, String)>,
) -> Result<(), String> {
    for record in heads.iter().filter(|record| {
        selected.contains(&(record.source_id.clone(), record.logical_id.clone()))
            && authorization::SOURCES[..2].contains(&record.source_id.as_str())
    }) {
        authorization::validate_game_origin(record)?;
        let document: dbrain_sources::knowledge_contract::KnowledgeDocument = serde_json::from_str(
            &record.metadata[brain_storage::source_versions::DOCUMENT_METADATA_KEY],
        )
        .map_err(|_| "Originaldokument ist ungültig")?;
        let mut options = runtime_options(args, &record.source_id)?;
        let pinned = PinnedRepository::open(&options.root, &document.revision)
            .map_err(|_| "Gepinnter Originalstand ist nicht mehr lokal verfügbar")?;
        let url = repository_url(&record.source_id)?;
        let origins = [
            url.to_owned(),
            format!("{url}.git"),
            format!(
                "git@github.com:{}.git",
                url.strip_prefix("https://github.com/")
                    .ok_or("Git-Herkunft ist ungültig")?
            ),
        ];
        pinned
            .require_origin(&origins.iter().map(String::as_str).collect::<Vec<_>>())
            .map_err(|_| "Originalrepository besitzt eine fremde Herkunft")?;
        let path = document.metadata["original_relative_path"]
            .as_str()
            .ok_or("Originalpfad fehlt")?;
        let bytes = pinned
            .read_blob(path)
            .map_err(|_| "Gepinntes Original kann nicht gelesen werden")?;
        let staged = tempfile::tempdir().map_err(|_| "Prüfordner kann nicht angelegt werden")?;
        let target = staged.path().join(path);
        std::fs::create_dir_all(target.parent().ok_or("Originalpfad fehlt")?)
            .map_err(|_| "Prüfpfad kann nicht angelegt werden")?;
        std::fs::write(target, bytes)
            .map_err(|_| "Original kann nicht zur Prüfung bereitgestellt werden")?;
        options.root = staged.path().into();
        options.source_revision = Some(document.revision.clone());
        options.observed_at = document.observed_at.clone();
        options.provenance["git_commit"] = json!(document.revision);
        let mut extracted = Vec::new();
        let inventory = extract_game_files(&options, &mut extracted)
            .map_err(|_| "Originalprüfung mit Standardextraktion fehlgeschlagen")?;
        if !inventory.gaps.is_empty() {
            return Err("Originalprüfung meldet Extraktionslücken".into());
        }
        let verified = validate_knowledge_jsonl(std::io::Cursor::new(extracted))
            .map_err(|_| "Originalprüfung verletzt den Wissensvertrag")?;
        if verified.documents().len() != 1 {
            return Err("Originalprüfung liefert keine eindeutige Spielquelle".into());
        }
        let actual = &verified.documents()[0].document;
        if actual.document_id != document.document_id
            || actual.content != document.content
            || actual.content_sha256 != document.content_sha256
            || actual.metadata["original_sha256"] != document.metadata["original_sha256"]
            || actual.facts != document.facts
        {
            return Err("Kanonisches Original weicht von seinem gepinnten Git-Blob ab".into());
        }
    }
    Ok(())
}

pub(super) fn extract(args: &Arguments) -> Result<Value, String> {
    let output_path = args.path("--output")?;
    report_preflight(&output_path)?;
    if output_path == args.path("--report")? {
        return Err("Ausgabe und Bericht müssen getrennte Dateien sein".into());
    }
    let source = &args.sources[0];
    let mut options = runtime_options(args, source)?;
    let repository_url = repository_url(source)?;
    let pinned = PinnedRepository::open(&options.root, args.required("--revision")?)
        .map_err(|_| "Gepinnte Git-Spielquelle ist nicht lesbar")?;
    let origins = [
        repository_url.to_owned(),
        format!("{repository_url}.git"),
        format!(
            "git@github.com:{}.git",
            repository_url
                .strip_prefix("https://github.com/")
                .ok_or("Repository-Herkunft ist ungültig")?
        ),
    ];
    pinned
        .require_origin(&origins.iter().map(String::as_str).collect::<Vec<_>>())
        .map_err(|_| "Lokales Git-Repository hat eine fremde Herkunft")?;
    let prefixes = if source == authorization::SOURCES[0] {
        vec![
            "data/json",
            "data/localizations/english.json",
            "data/localizations/german.json",
        ]
    } else {
        vec![
            "game/citadel/pak01_dir/scripts",
            "game/citadel/resource/localization",
            "game/citadel/pak01_dir/resource/localization",
        ]
    };
    let mut blobs = BTreeMap::new();
    for prefix in prefixes {
        for blob in pinned
            .files(prefix)
            .map_err(|_| "Gepinnte Spielpfade können nicht gelesen werden")?
        {
            if source == authorization::SOURCES[1]
                && !blob.path.starts_with("game/citadel/pak01_dir/scripts/")
                && !(blob.path.ends_with("_english.txt") || blob.path.ends_with("_german.txt"))
            {
                continue;
            }
            blobs.insert(blob.path.clone(), blob);
        }
    }
    if blobs.is_empty()
        || blobs.len() > dbrain_sources::git_source::MAX_GIT_FILES
        || blobs.values().map(|blob| blob.size).sum::<usize>() > 128 * 1024 * 1024
    {
        return Err("Gepinnte Spieldateien fehlen oder überschreiten die Importgrenze".into());
    }
    let staged =
        tempfile::tempdir().map_err(|_| "Eigener Extraktionsordner kann nicht angelegt werden")?;
    for path in blobs.keys() {
        let target = staged.path().join(path);
        std::fs::create_dir_all(target.parent().ok_or("Spielpfad fehlt")?)
            .map_err(|_| "Spielpfad kann nicht bereitgestellt werden")?;
        std::fs::write(
            target,
            pinned
                .read_blob(path)
                .map_err(|_| "Gepinnter Blob kann nicht gelesen werden")?,
        )
        .map_err(|_| "Gepinnter Blob kann nicht bereitgestellt werden")?;
    }
    options.root = staged.path().into();
    options.source_revision = Some(pinned.commit().into());
    options.observed_at = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    options.provenance["git_commit"] = json!(pinned.commit());
    let mut output =
        tempfile::NamedTempFile::new_in(output_path.parent().ok_or("Ausgabeordner fehlt")?)
            .map_err(|_| "Extraktionsausgabe kann nicht vorbereitet werden")?;
    let inventory = extract_game_files(&options, output.as_file_mut())
        .map_err(|_| "Standardextraktion fehlgeschlagen")?;
    if !inventory.gaps.is_empty() {
        return Err("Standardextraktion meldet Lücken; keine Ausgabe veröffentlicht".into());
    }
    output
        .as_file_mut()
        .seek(SeekFrom::Start(0))
        .map_err(|_| "Extraktionsausgabe kann nicht geprüft werden")?;
    let validated = validate_knowledge_jsonl(BufReader::new(output.as_file_mut()))
        .map_err(|_| "Extraktionsausgabe verletzt den Wissensvertrag")?;
    if !validated.conflicts().is_empty() {
        return Err("Extraktionsausgabe enthält Konflikte".into());
    }
    output
        .as_file_mut()
        .flush()
        .map_err(|_| "Extraktionsausgabe kann nicht geschrieben werden")?;
    output
        .as_file()
        .sync_all()
        .map_err(|_| "Extraktionsausgabe kann nicht gespeichert werden")?;
    output.persist_noclobber(&output_path).map_err(|_| {
        "Extraktionsausgabe existiert bereits oder kann nicht veröffentlicht werden"
    })?;
    File::open(output_path.parent().ok_or("Ausgabeordner fehlt")?)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| "Ausgabeordner kann nicht gespeichert werden")?;
    Ok(
        json!({"extracted": true, "source": source, "commit": pinned.commit(), "output": output_path,
        "inventory": inventory, "license_declaration": options.license_name,
        "redistribution_allowed": false, "imported": false, "published": false, "activated": false}),
    )
}
