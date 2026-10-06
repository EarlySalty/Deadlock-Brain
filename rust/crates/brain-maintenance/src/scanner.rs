use crate::{
    config::{safe_relative, validate_ref, Bounds, MaintenanceConfig, RepositoryConfig},
    digest, process,
};
use anyhow::{ensure, Context, Result};
use brain_contracts::{
    source::{GameValidity, OriginArtifact, SourceIdentity, SourceRevision, SourceTimestamp},
    value::{Observed, UnknownReason},
};
use dbrain_sources::git_source::{validate_commit, PinnedRepository};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceBlob {
    pub path: String,
    pub sha256: String,
    pub content: String,
    pub origin: OriginArtifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanResult {
    pub repo_id: String,
    pub source_sha: String,
    pub deployed_sha: Option<String>,
    pub source_changed: bool,
    pub changed_paths: Vec<String>,
    pub source_blobs: Vec<EvidenceBlob>,
    pub docs_sha: String,
    pub documents: BTreeMap<String, Option<String>>,
    pub document_paths: BTreeMap<String, String>,
    pub source_fingerprint: String,
    pub assets: Vec<crate::config::AssetProvenance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredRepository {
    pub path: PathBuf,
    pub state: String,
}

pub(crate) fn approved_code_path(path: &str) -> bool {
    // Nur Code und bewusst registrierte Dokumentbelege. Keine Datenablagen.
    let extension = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    matches!(
        extension,
        "rs" | "ts" | "tsx" | "js" | "jsx" | "css" | "md" | "html"
    ) && !path.split('/').any(|p| {
        matches!(
            p,
            "data"
                | "logs"
                | "downloads"
                | "uploads"
                | "fixtures"
                | "snapshots"
                | ".tasks"
                | "node_modules"
        ) || p.starts_with(".env")
            || p.to_lowercase().contains("secret")
            || p.to_lowercase().contains("credential")
    })
}

pub async fn resolve_ref(repo: &Path, reference: &str, bounds: &Bounds) -> Result<String> {
    validate_ref(reference)?;
    let bytes = process::run(
        Path::new("git"),
        &[
            "-c".into(),
            "core.hooksPath=/dev/null".into(),
            "rev-parse".into(),
            "--verify".into(),
            format!("{reference}^{{commit}}"),
        ],
        repo,
        &[],
        bounds.git_timeout_ms,
        128,
    )
    .await?;
    let sha = std::str::from_utf8(&bytes)?.trim().to_owned();
    validate_commit(&sha)?;
    Ok(sha)
}

/// Der registrierte Online-Leseweg füllt fehlende Blobs eines Partial-Clones.
/// Der nachfolgende gepinnte Parser behält sein Netzwerkverbot.
pub async fn materialize_game_source(
    repo: &Path,
    origin: &str,
    commit: &str,
    prefixes: &[String],
    bounds: &Bounds,
) -> Result<()> {
    validate_commit(commit)?;
    let pinned = PinnedRepository::open(repo, commit)?;
    pinned.require_origin(&[origin])?;
    let mut files = BTreeMap::new();
    for prefix in prefixes {
        safe_relative(prefix)?;
        let mut args = vec![
            "--no-optional-locks".into(),
            "--no-replace-objects".into(),
            "-c".into(),
            "core.hooksPath=/dev/null".into(),
            "--literal-pathspecs".into(),
            "ls-tree".into(),
            "-r".into(),
            "-z".into(),
            commit.into(),
            "--".into(),
            prefix.clone(),
        ];
        let tree = process::run(
            Path::new("/usr/bin/git"),
            &args,
            repo,
            &[],
            bounds.git_timeout_ms,
            1024 * 1024,
        )
        .await?;
        let entries: Vec<_> = tree
            .split(|byte| *byte == 0)
            .filter(|entry| !entry.is_empty())
            .collect();
        ensure!(
            entries.len() <= dbrain_sources::git_source::MAX_GIT_FILES,
            "game_source_file_limit"
        );
        for entry in entries {
            let entry = std::str::from_utf8(entry)?;
            let (header, path) = entry.split_once('\t').context("game_source_tree_record")?;
            safe_relative(path)?;
            let fields: Vec<_> = header.split_whitespace().collect();
            ensure!(
                fields.len() == 3
                    && matches!(fields[0], "100644" | "100755")
                    && fields[1] == "blob",
                "game_source_blob_required"
            );
        }
        args.insert(8, "-l".into());
        process::run(
            Path::new("/usr/bin/git"),
            &args,
            repo,
            &[],
            bounds.git_timeout_ms,
            1024 * 1024,
        )
        .await?;
        for blob in pinned.files(prefix)? {
            files.insert(blob.path, blob.size);
        }
    }
    ensure!(
        files.len() <= dbrain_sources::git_source::MAX_GIT_FILES
            && files
                .values()
                .try_fold(0usize, |total, size| total.checked_add(*size))
                .is_some_and(|total| total <= dbrain_sources::git_source::MAX_GIT_TOTAL_BYTES),
        "game_source_snapshot_limit"
    );
    Ok(())
}

pub fn discover(config: &MaintenanceConfig) -> Result<Vec<DiscoveredRepository>> {
    config.validate()?;
    let mut known: BTreeSet<_> = config
        .repositories
        .iter()
        .map(|r| std::fs::canonicalize(&r.path))
        .collect::<std::io::Result<_>>()?;
    known.insert(std::fs::canonicalize(&config.docs_repo)?);
    let mut found = Vec::new();
    for root in &config.repo_roots {
        for entry in std::fs::read_dir(root)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name == "rs-relay"
                || name.contains("tradingbot")
                || name.contains("ai-coach")
                || name.contains("ai_coach")
            {
                continue;
            }
            if path.join(".git").exists() && !known.contains(&std::fs::canonicalize(&path)?) {
                let git_config = path.join(".git/config");
                let Ok(metadata) = std::fs::symlink_metadata(&git_config) else {
                    continue;
                };
                if !metadata.is_file() || metadata.len() > 65536 {
                    continue;
                }
                let remote_config = std::fs::read_to_string(&git_config)?;
                if !remote_config.lines().any(|line| {
                    line.trim().strip_prefix("url = ").is_some_and(|url| {
                        url.starts_with("git@github.com:EarlySalty/")
                            || url.starts_with("https://github.com/EarlySalty/")
                    })
                }) {
                    continue;
                }
                ensure!(
                    found.len() + known.len() < config.bounds.max_repositories,
                    "Entdeckung überschreitet die Repo-Grenze"
                );
                found.push(DiscoveredRepository {
                    path,
                    state: "policy_pending".into(),
                });
            }
        }
    }
    found.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(found)
}

fn read_sources(
    repo: &RepositoryConfig,
    pin: &PinnedRepository,
    bounds: &Bounds,
    changed_paths: &[String],
) -> Result<Vec<EvidenceBlob>> {
    ensure!(
        repo.code_only_export_approved && repo.policy.provider_egress_allowed,
        "Externe Codeprüfung ist nicht freigegeben"
    );
    let mut paths = BTreeSet::new();
    let mut size = 0usize;
    let mut blobs = Vec::new();
    let commit_time = pin.revision_info()?.commit_time;
    let prefixes: BTreeSet<_> = repo.evidence_paths.iter().chain(changed_paths).collect();
    for prefix in prefixes {
        safe_relative(prefix)?;
        for file in pin.files(prefix)? {
            if !approved_code_path(&file.path) || !paths.insert(file.path.clone()) {
                continue;
            }
            ensure!(
                paths.len() <= bounds.max_files && file.size <= bounds.max_blob_bytes,
                "Quellenausschnitt überschreitet die Grenze"
            );
            size = size
                .checked_add(file.size)
                .context("Kontextgröße überschritten")?;
            ensure!(
                size <= bounds.max_bundle_bytes,
                "Quellenkontext überschreitet die Grenze"
            );
            let raw = pin.read_blob(&file.path)?;
            let content = String::from_utf8(raw).context("Quellbeleg ist kein UTF-8-Text")?;
            ensure!(
                !contains_platform_identifier(&content),
                "context_platform_identifier"
            );
            let sha256 = digest(content.as_bytes());
            let origin = OriginArtifact {
                identity: SourceIdentity {
                    source_id: format!("repo:{}", repo.id),
                    logical_id: file.path.clone(),
                },
                source_revision: SourceRevision::Git {
                    commit: pin.commit().into(),
                },
                raw_sha256: sha256.clone(),
                locator: format!(
                    "{}/blob/{}/{}",
                    repo.origin.trim_end_matches(".git"),
                    pin.commit(),
                    file.path
                ),
                parser_revision: "brain-maintenance.v1".into(),
                parser_family: "git-code".into(),
                schema_version: Observed::unknown(UnknownReason::NotPresent),
                schema_sha256: Observed::unknown(UnknownReason::NotPresent),
                retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(
                    chrono::Utc::now().timestamp(),
                )),
                source_time: Observed::known(SourceTimestamp::UnixSeconds(commit_time)),
                language: Observed::unknown(UnknownReason::NotPresent),
                origin_artifacts: BTreeSet::from([format!(
                    "{}@{}:{}",
                    repo.id,
                    pin.commit(),
                    file.path
                )]),
                derivation_family: Observed::known(format!("git:{}", repo.id)),
                policy: repo.policy.clone(),
                validity: GameValidity::unknown(),
            };
            origin.validate().map_err(anyhow::Error::msg)?;
            blobs.push(EvidenceBlob {
                path: file.path,
                sha256,
                content,
                origin,
            });
        }
    }
    ensure!(
        !blobs.is_empty(),
        "Keine zulässigen vollständigen Codebelege gefunden"
    );
    blobs.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(blobs)
}

fn contains_platform_identifier(content: &str) -> bool {
    content
        .split(|c: char| !c.is_ascii_digit() && c != '_')
        .any(|part| {
            let count = part.bytes().filter(u8::is_ascii_digit).count();
            (16..=20).contains(&count)
        })
}

pub async fn scan(
    config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    previous_sha: Option<&str>,
) -> Result<ScanResult> {
    scan_pinned(config, repo, previous_sha, None).await
}

pub async fn relevant_source_matches(
    config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    sha: &str,
) -> Result<bool> {
    let current = resolve_ref(&repo.path, &repo.source_ref, &config.bounds).await?;
    if current == sha {
        return Ok(true);
    }
    let pin = PinnedRepository::open(&repo.path, sha)?;
    pin.require_origin(&[&repo.origin])?;
    Ok(!pin
        .diff(&current)?
        .into_iter()
        .flat_map(|change| change.old_path.into_iter().chain(change.new_path))
        .any(|path| {
            approved_code_path(&path)
                && repo
                    .source_paths
                    .iter()
                    .any(|scope| path == *scope || path.starts_with(&format!("{scope}/")))
        }))
}

pub async fn scan_pinned(
    config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    previous_sha: Option<&str>,
    required_sha: Option<&str>,
) -> Result<ScanResult> {
    config.validate()?;
    crate::config::require_registered(config, repo)?;
    let source_sha = if let Some(sha) = required_sha {
        ensure!(
            relevant_source_matches(config, repo, sha).await?,
            "Quellstand hat relevante Änderungen"
        );
        sha.to_owned()
    } else {
        resolve_ref(&repo.path, &repo.source_ref, &config.bounds).await?
    };
    let pin = PinnedRepository::open(&repo.path, &source_sha)?;
    pin.require_origin(&[&repo.origin])?;
    let mut inventory = BTreeMap::new();
    for scope in &repo.source_paths {
        for blob in pin.files(scope)? {
            if approved_code_path(&blob.path) {
                inventory.insert(blob.path.clone(), blob.oid);
            }
            ensure!(
                inventory.len() <= config.bounds.max_files,
                "Quellinventar überschreitet die Grenze"
            );
        }
    }
    let changed_paths = match previous_sha {
        Some(previous) if previous != source_sha => PinnedRepository::open(&repo.path, previous)?
            .diff(&source_sha)?
            .into_iter()
            .flat_map(|c| c.old_path.into_iter().chain(c.new_path))
            .filter(|path| {
                repo.source_paths
                    .iter()
                    .any(|p| path == p || path.starts_with(&format!("{p}/")))
                    && approved_code_path(path)
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        Some(_) => Vec::new(),
        None => inventory.keys().cloned().collect(),
    };
    let source_changed = !changed_paths.is_empty();
    let source_fingerprint = digest(&serde_json::to_vec(&inventory)?);
    let evidence_changes = if previous_sha.is_some() {
        changed_paths.as_slice()
    } else {
        &[]
    };
    let source_blobs = read_sources(repo, &pin, &config.bounds, evidence_changes)?;
    let docs_sha = resolve_ref(&config.docs_repo, &config.docs_ref, &config.bounds).await?;
    let docs = PinnedRepository::open(&config.docs_repo, &docs_sha)?;
    docs.require_origin(&[&config.docs_origin])?;
    for asset in &config.registered_assets {
        let files = docs.files(&asset.git_path)?;
        let file = files
            .iter()
            .find(|f| f.path == asset.git_path)
            .context("Registrierte Bildquelle fehlt")?;
        ensure!(
            file.size <= config.bounds.max_blob_bytes
                && digest(&docs.read_blob(&asset.git_path)?) == asset.sha256,
            "Registrierte Bildquelle wurde verändert"
        );
    }
    let mut documents = BTreeMap::new();
    let mut document_paths = BTreeMap::new();
    let mut total = source_blobs.iter().map(|b| b.content.len()).sum::<usize>();
    for target in &repo.doc_targets {
        let output = repo.output_paths.get(target).unwrap_or(target);
        let output_exists = docs.files(output)?.iter().any(|file| file.path == *output);
        let artifact = if output_exists { output } else { target };
        let files = docs.files(artifact)?;
        let content = if let Some(document) = config.canonical_documents.get(target) {
            ensure!(
                document.logical_id == *target
                    && document.origin.raw_sha256 == digest(document.content.as_bytes())
                    && canonical_policy_bound(config, repo, target),
                "Kanonisches Dokument hat andere Rechte oder Identität"
            );
            Some(document.content.clone())
        } else if target.starts_with("internal/") && config.private_document_root.is_some() {
            read_private_document(config, artifact)?
        } else if let Some(file) = files.iter().find(|f| f.path == *artifact) {
            ensure!(
                file.size <= config.bounds.max_blob_bytes,
                "Dokument überschreitet die Grenze"
            );
            let content = String::from_utf8(docs.read_blob(artifact)?)?;
            total += content.len();
            ensure!(
                total <= config.bounds.max_bundle_bytes,
                "Gesamter Kontext überschreitet die Grenze"
            );
            Some(content)
        } else {
            None
        };
        documents.insert(target.clone(), content);
        document_paths.insert(
            target.clone(),
            if config.canonical_documents.contains_key(target) {
                config.canonical_documents[target]
                    .metadata
                    .get("output_path")
                    .unwrap_or(output)
                    .clone()
            } else {
                artifact.clone()
            },
        );
    }
    let deployed_sha = match &repo.deployed_sha_file {
        Some(path) => {
            let metadata = std::fs::symlink_metadata(path)?;
            ensure!(
                metadata.file_type().is_file() && metadata.len() <= 128,
                "Deploy-Beleg ist keine kleine reguläre SHA-Datei"
            );
            let value = std::fs::read_to_string(path)?.trim().to_owned();
            validate_commit(&value)?;
            Some(value)
        }
        None => None,
    };
    Ok(ScanResult {
        repo_id: repo.id.clone(),
        source_sha,
        deployed_sha,
        source_changed,
        changed_paths,
        source_blobs,
        docs_sha,
        documents,
        document_paths,
        source_fingerprint,
        assets: config.registered_assets.clone(),
    })
}

pub(crate) fn canonical_policy_bound(
    config: &MaintenanceConfig,
    repo: &RepositoryConfig,
    target: &str,
) -> bool {
    let Some(document) = config.canonical_documents.get(target) else {
        return true;
    };
    let expected = repo.document_policy_for(target);
    if &document.origin.policy == expected {
        return true;
    }
    // Eine neue explizite Exportfreigabe gilt nur für die folgende codebasierte Fassung.
    let mut previous = expected.clone();
    previous.provider_egress_allowed = false;
    previous.authorization_ref = document.origin.policy.authorization_ref.clone();
    repo.code_only_migration_targets.contains(target)
        && target.starts_with("internal/")
        && !target.starts_with("internal/public-candidates/")
        && repo.code_only_export_approved
        && expected.provider_egress_allowed
        && document.origin.policy == previous
}

pub fn exportable_document<'a>(
    config: &MaintenanceConfig,
    scan: &'a ScanResult,
    target: &str,
) -> Option<&'a String> {
    if config
        .canonical_documents
        .get(target)
        .is_some_and(|d| !d.origin.policy.provider_egress_allowed)
    {
        return None;
    }
    scan.documents.get(target).and_then(Option::as_ref)
}

pub(crate) fn read_private_document(
    config: &MaintenanceConfig,
    target: &str,
) -> Result<Option<String>> {
    crate::config::safe_doc_target(target)?;
    let root = config
        .private_document_root
        .as_ref()
        .context("Privater Dokumentpfad fehlt")?;
    use std::os::unix::fs::PermissionsExt;
    let metadata = std::fs::symlink_metadata(root)?;
    ensure!(
        metadata.is_dir()
            && metadata.permissions().mode() & 0o077 == 0
            && std::fs::canonicalize(root)? == *root,
        "Privater Dokumentpfad ist nicht geschützt"
    );
    let path = root.join(target);
    if !path.exists() {
        return Ok(None);
    }
    let metadata = std::fs::symlink_metadata(&path)?;
    ensure!(
        metadata.is_file()
            && metadata.permissions().mode() & 0o077 == 0
            && metadata.len() <= config.bounds.max_blob_bytes as u64
            && std::fs::canonicalize(&path)? == path,
        "Privates Dokument ist nicht geschützt"
    );
    Ok(Some(std::fs::read_to_string(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn partial_clone_wird_vor_offline_parser_nur_im_registrierten_pfad_gefuellt() {
        let directory = tempfile::tempdir().unwrap();
        let upstream = directory.path().join("upstream");
        let clone = directory.path().join("clone");
        std::fs::create_dir(&upstream).unwrap();
        let git = |cwd: &Path, args: &[&str]| {
            let output = std::process::Command::new("git")
                .current_dir(cwd)
                .args(args)
                .output()
                .unwrap();
            assert!(output.status.success());
            String::from_utf8(output.stdout).unwrap()
        };
        git(&upstream, &["init", "-q"]);
        git(&upstream, &["config", "uploadpack.allowFilter", "true"]);
        git(
            &upstream,
            &["config", "uploadpack.allowAnySHA1InWant", "true"],
        );
        std::fs::create_dir(upstream.join("scripts")).unwrap();
        std::fs::write(upstream.join("scripts/game.vdata"), "Spielquelle").unwrap();
        std::fs::write(upstream.join("anderer-pfad.txt"), "Nicht importieren").unwrap();
        git(&upstream, &["add", "."]);
        git(
            &upstream,
            &[
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
                "commit",
                "-qm",
                "Fixture",
            ],
        );
        let commit = git(&upstream, &["rev-parse", "HEAD"]).trim().to_owned();
        let origin = format!("file://{}", upstream.display());
        git(
            directory.path(),
            &[
                "clone",
                "-q",
                "--filter=blob:none",
                "--no-checkout",
                &origin,
                "clone",
            ],
        );
        let pinned = PinnedRepository::open(&clone, &commit).unwrap();
        assert!(pinned.files("scripts").is_err());
        let bounds = Bounds {
            max_repositories: 1,
            max_files: 100,
            max_blob_bytes: 8388608,
            max_bundle_bytes: 33554432,
            max_output_bytes: 1048576,
            git_timeout_ms: 10000,
        };
        assert!(materialize_game_source(
            &clone,
            "file:///nicht-die-quelle",
            &commit,
            &["scripts".into()],
            &bounds
        )
        .await
        .is_err());
        assert!(pinned.files("scripts").is_err());
        materialize_game_source(&clone, &origin, &commit, &["scripts".into()], &bounds)
            .await
            .unwrap();
        let files = pinned.files("scripts").unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            pinned.read_blob(&files[0].path).unwrap(),
            "Spielquelle".as_bytes()
        );
        assert!(pinned.files("anderer-pfad.txt").is_err());
    }
    #[test]
    fn sensitive_paths_and_data_are_never_exported() {
        for path in [
            ".env",
            "logs/chat.md",
            "data/users.rs",
            "src/secrets.rs",
            "uploads/x.ts",
            "src/auth.json",
            "snapshots/chat.md",
        ] {
            assert!(!approved_code_path(path), "{path}");
        }
        assert!(approved_code_path("rust/crates/service/src/lib.rs"));
    }
    #[test]
    fn platform_identifiers_in_code_remain_local() {
        assert!(contains_platform_identifier(
            "UserId::new(123_456_789_012_345_678)"
        ));
        assert!(contains_platform_identifier("\"123456789012345678\""));
        assert!(!contains_platform_identifier("timeout_ms: 30000"));
    }
}
