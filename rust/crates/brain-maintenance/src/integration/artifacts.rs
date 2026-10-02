use anyhow::{ensure, Result};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

pub struct Artifacts {
    root: PathBuf,
}

impl Artifacts {
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
            ["json", "html", "md"].contains(&extension),
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
                && ["json", "html", "md"].contains(&extension),
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
