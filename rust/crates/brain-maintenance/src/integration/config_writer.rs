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
            .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_CLOEXEC)
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
        read_bounded(&self.path, 65536)
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
    let replacement = read_bounded(input, 65536)?;
    brain_serve::Config::parse(&replacement)
        .map_err(|_| anyhow::anyhow!("serve_config_invalid"))?;
    let writer = ConfigWriter::lock(&runtime.serve_config)?;
    let expected = writer.read()?;
    ensure!(
        crate::digest(&expected) == expected_sha256,
        "serve_config_changed"
    );
    writer.replace(&expected, &replacement)?;
    Ok(serde_json::json!({"status":"configured","config_sha256":crate::digest(&replacement)}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn operator_config_and_activation_share_lock_and_preserve_changed_rollback_basis() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let path = dir.path().join("serve.json");
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
