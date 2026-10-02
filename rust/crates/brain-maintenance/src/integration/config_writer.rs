use super::{runtime_config::read_bounded, runtime_config::RuntimeConfig};
use anyhow::{ensure, Result};
use std::{
    fs::File,
    io::Write,
    os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    sync::Arc,
};

/// Gemeinsamer Schreibvertrag für Betreiber, Deployment, Aktivierung und Rückfall.
#[derive(Clone)]
pub struct ConfigWriter {
    path: PathBuf,
    _lock: Arc<File>,
}
impl ConfigWriter {
    pub async fn lock_async(path: PathBuf) -> Result<Self> {
        tokio::task::spawn_blocking(move || Self::lock(&path)).await?
    }
    pub fn lock(path: &Path) -> Result<Self> {
        let parent = path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("serve_config_parent"))?;
        let parent_metadata = std::fs::symlink_metadata(parent)?;
        ensure!(
            parent_metadata.is_dir()
                && parent_metadata.permissions().mode() & 0o077 == 0
                && parent_metadata.uid() == nix::unistd::geteuid().as_raw()
                && std::fs::canonicalize(parent)? == parent,
            "serve_config_parent_unprotected"
        );
        let mut lock_name = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("serve_config_filename"))?
            .to_os_string();
        lock_name.push(".lock");
        let lock = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .custom_flags(nix::libc::O_NONBLOCK | nix::libc::O_NOFOLLOW | nix::libc::O_CLOEXEC)
            .open(parent.join(lock_name))?;
        let metadata = lock.metadata()?;
        ensure!(
            metadata.is_file()
                && metadata.permissions().mode() & 0o077 == 0
                && metadata.uid() == nix::unistd::geteuid().as_raw(),
            "serve_config_lock_unprotected"
        );
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
        loop {
            match lock.try_lock() {
                Ok(()) => break,
                Err(std::fs::TryLockError::WouldBlock) if std::time::Instant::now() < deadline => {
                    std::thread::sleep(std::time::Duration::from_millis(10))
                }
                Err(_) => anyhow::bail!("serve_config_lock_unavailable"),
            }
        }
        let writer = Self {
            path: path.to_owned(),
            _lock: Arc::new(lock),
        };
        writer.read()?;
        Ok(writer)
    }
    pub fn require_path(&self, path: &Path) -> Result<()> {
        ensure!(self.path == path, "serve_config_lock_path");
        Ok(())
    }
    pub fn read(&self) -> Result<Vec<u8>> {
        let metadata = std::fs::symlink_metadata(&self.path)?;
        ensure!(
            metadata.is_file()
                && metadata.permissions().mode() & 0o077 == 0
                && metadata.uid() == nix::unistd::geteuid().as_raw()
                && std::fs::canonicalize(&self.path)? == self.path,
            "serve_config_unprotected"
        );
        read_bounded(&self.path, brain_serve::bot_toml::MAX_BYTES)
    }
    pub fn replace(&self, expected: &[u8], replacement: &[u8]) -> Result<()> {
        self.replace_checked(expected, replacement, || {})
    }
    fn replace_checked(
        &self,
        expected: &[u8],
        replacement: &[u8],
        checked: impl FnOnce(),
    ) -> Result<()> {
        let current = self.read()?;
        if current == replacement {
            return Ok(());
        }
        ensure!(current == expected, "serve_config_changed");
        checked();
        let parent = self
            .path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("serve_config_parent"))?;
        let mut staged = tempfile::NamedTempFile::new_in(parent)?;
        staged.as_file_mut().write_all(replacement)?;
        staged
            .as_file()
            .set_permissions(std::fs::Permissions::from_mode(0o600))?;
        staged.as_file().sync_all()?;
        staged
            .persist(&self.path)
            .map_err(|_| anyhow::anyhow!("serve_config_replace"))?;
        File::open(parent)?.sync_all()?;
        Ok(())
    }
}

pub fn write_serve_config(
    runtime: &RuntimeConfig,
    input: &Path,
    expected_sha256: &str,
) -> Result<serde_json::Value> {
    super::runner::require_operator_config(&runtime.maintenance_config)?;
    let replacement = read_bounded(input, brain_serve::bot_toml::MAX_BYTES)?;
    let writer = ConfigWriter::lock(&runtime.serve_config)?;
    let expected = writer.read()?;
    ensure!(
        crate::digest(&expected) == expected_sha256,
        "serve_config_changed"
    );
    validate_serve_replacement(&expected, &replacement)?;
    writer.replace(&expected, &replacement)?;
    Ok(serde_json::json!({"status":"configured","config_sha256":crate::digest(&replacement)}))
}

fn validate_serve_replacement(expected: &[u8], replacement: &[u8]) -> Result<()> {
    brain_serve::Config::parse(replacement).map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
    crate::config::parse_maintenance(replacement)?;
    brain_serve::bot_toml::validate_infisical(replacement)
        .map_err(|_| anyhow::anyhow!("infisical_config_invalid"))?;
    let mut old = brain_serve::bot_toml::document(expected)
        .map_err(|_| anyhow::anyhow!("shared_config_invalid"))?;
    let mut new = brain_serve::bot_toml::document(replacement)
        .map_err(|_| anyhow::anyhow!("shared_config_invalid"))?;
    for root in [&mut old, &mut new] {
        let table = root
            .get_mut("brain")
            .and_then(toml::Value::as_table_mut)
            .ok_or_else(|| anyhow::anyhow!("shared_config_invalid"))?;
        table.remove("serve");
        table.remove("operator");
    }
    ensure!(old == new, "serve_writer_changed_unrelated_config");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serve_writer_darf_maintenance_und_infisical_nicht_entfernen_oder_umleiten() {
        let mut root = brain_serve::bot_toml::document(include_bytes!(
            "../../../../../config/brain-serve.example.toml"
        ))
        .unwrap();
        let maintenance =
            brain_serve::bot_toml::document(include_bytes!("../../config/smoke.example.toml"))
                .unwrap()["brain"]["maintenance"]
                .clone();
        root["brain"]
            .as_table_mut()
            .unwrap()
            .insert("maintenance".into(), maintenance);
        root["brain"].as_table_mut().unwrap().insert("infisical".into(),toml::Value::try_from(
            serde_json::json!({"project_id":"fixture-project", "environment":"fixture", "secret_path":"/",
                "socket_path":"/run/private/api.sock", "credential_fd":5})).unwrap());
        let old = toml::to_string(&root).unwrap().into_bytes();
        let mut new = root.clone();
        new["brain"]["serve"]["provider"]["model"] = toml::Value::String("fixture-modell".into());
        assert!(
            validate_serve_replacement(&old, toml::to_string(&new).unwrap().as_bytes()).is_ok()
        );
        for table in ["maintenance", "infisical"] {
            let mut incomplete = new.clone();
            incomplete["brain"].as_table_mut().unwrap().remove(table);
            assert!(validate_serve_replacement(
                &old,
                toml::to_string(&incomplete).unwrap().as_bytes()
            )
            .is_err());
        }
        for (table, field) in [
            ("maintenance", "prompt_version"),
            ("infisical", "environment"),
        ] {
            let mut redirected = new.clone();
            redirected["brain"][table][field] = toml::Value::String("fremder-wert".into());
            assert!(validate_serve_replacement(
                &old,
                toml::to_string(&redirected).unwrap().as_bytes()
            )
            .is_err());
        }
    }
    #[tokio::test(flavor = "current_thread")]
    async fn config_lock_wait_keeps_tokio_worker_available() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let path = dir.path().join("bot.toml");
        std::fs::write(&path, b"belegte Configbytes").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        let first = ConfigWriter::lock(&path).unwrap();
        let waiting = tokio::spawn(ConfigWriter::lock_async(path));
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
        assert!(!waiting.is_finished());
        drop(first);
        tokio::time::timeout(std::time::Duration::from_secs(1), waiting)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
    }
    #[test]
    fn operator_config_and_activation_share_lock_and_preserve_changed_rollback_basis() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let path = dir.path().join("bot.toml");
        std::fs::write(&path, b"old").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        let first = ConfigWriter::lock(&path).unwrap();
        let (attempt_tx, attempt_rx) = std::sync::mpsc::channel();
        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let second_path = path.clone();
        let thread = std::thread::spawn(move || {
            attempt_tx.send(()).unwrap();
            let second = ConfigWriter::lock(&second_path).unwrap();
            assert!(second.replace(b"old", b"edited").is_err());
            second.replace(b"activated", b"operator-settings").unwrap();
            done_tx.send(()).unwrap();
        });
        first
            .replace_checked(b"old", b"activated", || {
                attempt_rx.recv().unwrap();
                assert!(done_rx
                    .recv_timeout(std::time::Duration::from_millis(100))
                    .is_err());
            })
            .unwrap();
        assert_eq!(first.read().unwrap(), b"activated");
        drop(first);
        done_rx
            .recv_timeout(std::time::Duration::from_secs(2))
            .unwrap();
        thread.join().unwrap();
        let rollback = ConfigWriter::lock(&path).unwrap();
        assert!(rollback.replace(b"activated", b"old").is_err());
        assert_eq!(rollback.read().unwrap(), b"operator-settings");
    }
}
