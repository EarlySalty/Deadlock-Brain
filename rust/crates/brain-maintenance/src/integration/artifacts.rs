use anyhow::{ensure, Result};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct Artifacts {
    root: PathBuf,
}

impl Artifacts {
    fn call_path(&self, id: &str, suffix: &str) -> Result<PathBuf> {
        ensure!(
            id.len() == 64 && id.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "call_identity"
        );
        Ok(self.root.join(format!("call-{id}.{suffix}")))
    }

    pub fn claim_call(&self, id: &str) -> Result<bool> {
        let path = self.call_path(id, "intent")?;
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
        {
            Ok(mut file) => {
                file.write_all(id.as_bytes())?;
                file.sync_all()?;
                fs::File::open(&self.root)?.sync_all()?;
                Ok(true)
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
            Err(error) => Err(error.into()),
        }
    }

    pub fn call_claimed(&self, id: &str) -> Result<bool> {
        let path = self.call_path(id, "intent")?;
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        ensure!(
            metadata.is_file()
                && metadata.permissions().mode() & 0o077 == 0
                && metadata.len() == 64
                && fs::read(path)? == id.as_bytes(),
            "call_intent_marker"
        );
        Ok(true)
    }

    pub fn call_receipt(&self, id: &str) -> Result<Option<Vec<u8>>> {
        if let Some(receipt) = self.read_call_index(id, "validated")? {
            return Ok(Some(receipt));
        }
        self.read_call_index(id, "receipt")
    }

    fn read_call_index(&self, id: &str, phase: &str) -> Result<Option<Vec<u8>>> {
        let path = self.call_path(id, phase)?;
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        ensure!(
            metadata.is_file()
                && metadata.permissions().mode() & 0o077 == 0
                && metadata.len() <= 69,
            "call_receipt_index"
        );
        Ok(Some(self.read(&fs::read_to_string(path)?)?))
    }

    pub fn save_call_receipt(&self, id: &str, bytes: &[u8]) -> Result<()> {
        self.save_call_index(id, "receipt", bytes)
    }

    pub fn save_call_validation(&self, id: &str, bytes: &[u8]) -> Result<()> {
        self.save_call_index(id, "validated", bytes)
    }

    fn save_call_index(&self, id: &str, phase: &str, bytes: &[u8]) -> Result<()> {
        let reference = self.put(bytes, "json")?;
        let path = self.call_path(id, phase)?;
        let mut file = tempfile::NamedTempFile::new_in(&self.root)?;
        file.write_all(reference.as_bytes())?;
        file.as_file().sync_all()?;
        match file.persist_noclobber(path) {
            Ok(_) => {}
            Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
                ensure!(
                    self.read_call_index(id, phase)?.as_deref() == Some(bytes),
                    "call_receipt_conflict"
                );
            }
            Err(_) => anyhow::bail!("call_receipt_commit"),
        }
        fs::File::open(&self.root)?.sync_all()?;
        Ok(())
    }
    pub fn open(root: &Path) -> Result<Self> {
        if !root.exists() {
            fs::create_dir_all(root)?;
            fs::set_permissions(root, fs::Permissions::from_mode(0o700))?;
        }
        let metadata = fs::symlink_metadata(root)?;
        ensure!(
            metadata.is_dir() && metadata.permissions().mode() & 0o077 == 0,
            "private_artifact_directory"
        );
        ensure!(fs::canonicalize(root)? == root, "artifact_symlink");
        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    pub fn put(&self, bytes: &[u8], extension: &str) -> Result<String> {
        ensure!(bytes.len() <= 8 * 1024 * 1024, "artifact_size");
        ensure!(
            ["json", "toml", "html", "md", "raw"].contains(&extension),
            "artifact_extension"
        );
        let name = format!("{:x}.{extension}", Sha256::digest(bytes));
        let path = self.root.join(&name);
        if path.exists() {
            ensure!(self.read(&name)? == bytes, "immutable_artifact_conflict");
            return Ok(name);
        }
        let mut staging = tempfile::NamedTempFile::new_in(&self.root)?;
        staging.as_file_mut().write_all(bytes)?;
        staging.as_file().sync_all()?;
        match staging.persist_noclobber(&path) {
            Ok(_) => {}
            Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
                ensure!(self.read(&name)? == bytes, "immutable_artifact_conflict");
            }
            Err(_) => anyhow::bail!("artifact_commit"),
        }
        fs::File::open(&self.root)?.sync_all()?;
        Ok(name)
    }

    pub fn read(&self, name: &str) -> Result<Vec<u8>> {
        let (hash, extension) = name
            .split_once('.')
            .ok_or_else(|| anyhow::anyhow!("artifact_reference"))?;
        ensure!(
            hash.len() == 64
                && hash
                    .bytes()
                    .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
                && ["json", "toml", "html", "md", "raw"].contains(&extension),
            "artifact_reference"
        );
        let path = self.root.join(name);
        ensure!(fs::symlink_metadata(&path)?.is_file(), "artifact_file");
        let mut bytes = Vec::new();
        fs::File::open(path)?
            .take(8 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)?;
        ensure!(
            bytes.len() <= 8 * 1024 * 1024 && format!("{:x}", Sha256::digest(&bytes)) == hash,
            "artifact_hash"
        );
        Ok(bytes)
    }

    pub fn publication_lock(&self) -> Result<fs::File> {
        let path = self.root.join("publication.lock");
        if path.exists() {
            ensure!(
                fs::symlink_metadata(&path)?.is_file(),
                "publication_lock_file"
            );
        }
        let file = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .read(true)
            .open(path)?;
        file.lock()?;
        Ok(file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn content_addressed_artifacts_replay_without_overwrite_and_detect_damage() {
        let dir = tempfile::tempdir().unwrap();
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let artifacts = Artifacts::open(dir.path()).unwrap();
        let name = artifacts.put(b"<h1>Geprueft</h1>", "html").unwrap();
        assert_eq!(name, artifacts.put(b"<h1>Geprueft</h1>", "html").unwrap());
        assert!(artifacts.read("../secret.json").is_err());
        fs::write(dir.path().join(&name), "falsch").unwrap();
        assert!(artifacts.read(&name).is_err());
        assert!(artifacts.put(b"<h1>Geprueft</h1>", "html").is_err());
    }
}
