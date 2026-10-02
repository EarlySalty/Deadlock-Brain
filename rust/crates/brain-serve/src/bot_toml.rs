//! Normale Bot-TOML wird begrenzt gelesen; Parserfehler enthalten keine Eingabewerte.
use crate::Error;
use serde::de::DeserializeOwned;
use std::{fs::OpenOptions, io::Read, os::unix::fs::OpenOptionsExt, path::Path};
use zeroize::Zeroizing;

pub const MAX_BYTES: usize = 256 * 1024;

pub fn read(path: &Path) -> Result<Vec<u8>, Error> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NONBLOCK | nix::libc::O_NOFOLLOW | nix::libc::O_CLOEXEC)
        .open(path)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                Error::ConfigMissing
            } else {
                Error::ConfigIo
            }
        })?;
    if !file.metadata().map_err(|_| Error::ConfigIo)?.is_file() {
        return Err(Error::ConfigInvalid("file"));
    }
    let mut bytes = Vec::new();
    file.take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::ConfigIo)?;
    if bytes.len() > MAX_BYTES {
        return Err(Error::ConfigInvalid("size"));
    }
    Ok(bytes)
}

pub fn document(bytes: &[u8]) -> Result<toml::Value, Error> {
    if bytes.len() > MAX_BYTES {
        return Err(Error::ConfigInvalid("size"));
    }
    let text = std::str::from_utf8(bytes).map_err(|_| Error::ConfigSyntax)?;
    toml::from_str(text).map_err(|_| Error::ConfigSyntax)
}

pub fn value(document: &toml::Value, path: &[&str]) -> Result<toml::Value, Error> {
    let mut value = document;
    for key in path {
        value = value.get(*key).ok_or(Error::ConfigSyntax)?;
    }
    if !value.is_table() {
        return Err(Error::ConfigSyntax);
    }
    Ok(value.clone())
}

pub fn section<T: DeserializeOwned>(bytes: &[u8], path: &[&str]) -> Result<T, Error> {
    value(&document(bytes)?, path)?
        .try_into()
        .map_err(|_| Error::ConfigSyntax)
}

pub async fn infisical_snapshot(path: &Path) -> Result<Vec<(String, Zeroizing<String>)>, Error> {
    let root = document(&read(path)?)?;
    let mut source = value(&root, &["brain", "infisical"])?;
    validate_secret_source(&source)?;
    // Der geprüfte private Descriptor bleibt bis zum Ende des bestehenden Loaders offen.
    let _credential = bind_credential(&mut source)?;
    // Der bestehende sichere Loader erhält dieselben typisierten Metadaten im Speicher.
    // Es entsteht keine weitere normale JSON-Datei und keine neue Secretquelle.
    let metadata = serde_json::to_vec(&source).map_err(|_| Error::ConfigSyntax)?;
    dl_token_secrets::values_from_config(&metadata)
        .await
        .map_err(|_| Error::SecretSource)
}

fn bind_credential(source: &mut toml::Value) -> Result<Option<std::fs::File>, Error> {
    use nix::fcntl::{fcntl, FcntlArg, FdFlag};
    use std::os::{
        fd::AsRawFd,
        unix::fs::{MetadataExt, PermissionsExt},
    };
    let file = if let Some(fd) = source.get("credential_fd") {
        let fd = i32::try_from(fd.as_integer().ok_or(Error::SecretSource)?)
            .map_err(|_| Error::SecretSource)?;
        let flags = fcntl(fd, FcntlArg::F_GETFD).map_err(|_| Error::SecretSource)?;
        fcntl(
            fd,
            FcntlArg::F_SETFD(FdFlag::from_bits_retain(flags) | FdFlag::FD_CLOEXEC),
        )
        .map_err(|_| Error::SecretSource)?;
        filedescriptor::FileDescriptor::dup(&fd)
            .and_then(|descriptor| descriptor.as_file())
            .map_err(|_| Error::SecretSource)?
    } else if let Some(path) = source.get("credential_path") {
        OpenOptions::new()
            .read(true)
            .custom_flags(nix::libc::O_NONBLOCK | nix::libc::O_NOFOLLOW | nix::libc::O_CLOEXEC)
            .open(path.as_str().ok_or(Error::SecretSource)?)
            .map_err(|_| Error::SecretSource)?
    } else {
        return Ok(None);
    };
    let metadata = file.metadata().map_err(|_| Error::SecretSource)?;
    if !metadata.is_file()
        || metadata.permissions().mode() & 0o077 != 0
        || ![0, nix::unistd::geteuid().as_raw()].contains(&metadata.uid())
        || metadata.len() == 0
        || metadata.len() > 8192
    {
        return Err(Error::SecretSource);
    }
    let fd = file.as_raw_fd();
    fcntl(fd, FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC)).map_err(|_| Error::SecretSource)?;
    let table = source.as_table_mut().ok_or(Error::SecretSource)?;
    table.remove("credential_path");
    table.insert("credential_fd".into(), toml::Value::Integer(fd.into()));
    Ok(Some(file))
}

fn validate_secret_source(source: &toml::Value) -> Result<(), Error> {
    let fields = ["credential_fd", "credential_path", "secret_values_fd"];
    if fields
        .iter()
        .filter(|field| source.get(**field).is_some())
        .count()
        != 1
    {
        return Err(Error::SecretSource);
    }
    for field in ["credential_fd", "secret_values_fd"] {
        if let Some(fd) = source.get(field) {
            if !fd
                .as_integer()
                .is_some_and(|fd| (3..=i32::MAX as i64).contains(&fd))
            {
                return Err(Error::SecretSource);
            }
        }
    }
    if let Some(path) = source.get("credential_path") {
        if !path
            .as_str()
            .is_some_and(|path| Path::new(path).is_absolute())
        {
            return Err(Error::SecretSource);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn writerlose_config_fifo_wird_begrenzt_abgewiesen() {
        const FIXTURE_PATH: &str = "BRAIN_CONFIG_FIFO_TEST_PATH";
        if let Some(path) = std::env::var_os(FIXTURE_PATH) {
            assert!(read(Path::new(&path)).is_err());
            let mut source = toml::Value::Table(toml::map::Map::from_iter([(
                "credential_path".into(),
                toml::Value::String(path.to_str().unwrap().into()),
            )]));
            assert!(bind_credential(&mut source).is_err());
            return;
        }
        let directory = tempfile::tempdir().unwrap();
        let fifo = directory.path().join("config.fifo");
        nix::unistd::mkfifo(&fifo, nix::sys::stat::Mode::S_IRUSR).unwrap();
        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "bot_toml::tests::writerlose_config_fifo_wird_begrenzt_abgewiesen",
            ])
            .env(FIXTURE_PATH, &fifo)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        loop {
            if let Some(status) = child.try_wait().unwrap() {
                assert!(status.success());
                break;
            }
            if std::time::Instant::now() >= deadline {
                let _ = child.kill();
                let _ = child.wait();
                panic!("Config-FIFO blockiert den Loader");
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    #[test]
    fn json_fremde_tabelle_und_parserdetails_bleiben_geschlossen() {
        for input in [
            br#"{"brain":{"serve":{}}}"#.as_slice(),
            b"[brain.serve]\nvalue='nicht-protokollieren".as_slice(),
        ] {
            let error = document(input).unwrap_err();
            assert!(!format!("{error:?} {error}").contains("nicht-protokollieren"));
        }
        let root = document(b"[brain.docs]\ntimeout_ms=5000\n").unwrap();
        assert!(value(&root, &["brain", "serve"]).is_err());
        assert!(document(&vec![b' '; MAX_BYTES + 1]).is_err());
        for input in [
            "credential_fd=5",
            "credential_path='/run/private/credential'",
            "secret_values_fd=3",
        ] {
            assert!(validate_secret_source(&document(input.as_bytes()).unwrap()).is_ok());
        }
        for input in [
            "credential_fd=-1",
            "credential_fd=5\ncredential_path='/run/private/credential'",
            "credential_path='relativ'",
            "credential_fd='5'",
            "credential_fd=5\nsecret_values_fd=3",
            "",
        ] {
            assert!(validate_secret_source(&document(input.as_bytes()).unwrap()).is_err());
        }
        use std::{
            io::{Seek, Write},
            os::{fd::AsRawFd, unix::fs::PermissionsExt},
        };
        let mut file = tempfile::tempfile().unwrap();
        file.set_permissions(std::fs::Permissions::from_mode(0o400))
            .unwrap();
        file.write_all(b"synthetic-bootstrap").unwrap();
        let offset = file.stream_position().unwrap();
        let mut source =
            document(format!("credential_fd={}", file.as_raw_fd()).as_bytes()).unwrap();
        let bound = bind_credential(&mut source).unwrap().unwrap();
        assert_eq!(file.stream_position().unwrap(), offset);
        use nix::fcntl::{fcntl, FcntlArg, FdFlag};
        for fd in [file.as_raw_fd(), bound.as_raw_fd()] {
            assert!(
                FdFlag::from_bits_retain(fcntl(fd, FcntlArg::F_GETFD).unwrap())
                    .contains(FdFlag::FD_CLOEXEC)
            );
        }
        file.set_permissions(std::fs::Permissions::from_mode(0o644))
            .unwrap();
        assert!(bind_credential(&mut source).is_err());
        let mut invalid = document(b"credential_fd=2147483647").unwrap();
        assert!(bind_credential(&mut invalid).is_err());
    }
}
