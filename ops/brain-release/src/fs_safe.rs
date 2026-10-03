use anyhow::{bail, ensure, Context, Result};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Component, Path};

pub fn uid() -> u32 {
    unsafe { libc::geteuid() }
}

fn private_operator_group() -> bool {
    static PRIVATE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *PRIVATE.get_or_init(|| {
        let query = |args: &[&str]| -> Option<String> {
            let output = std::process::Command::new("/usr/bin/getent")
                .args(args)
                .env_clear()
                .current_dir("/")
                .output()
                .ok()?;
            if !output.status.success() {
                return None;
            }
            String::from_utf8(output.stdout).ok()
        };
        let Some(passwd) = query(&["passwd"]) else {
            return false;
        };
        let Some(group) = query(&["group", "1000"]) else {
            return false;
        };
        let users = passwd
            .lines()
            .map(|line| line.split(':').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if users
            .iter()
            .any(|user| user.len() != 7 || (user[3] == "1000" && user[2] != "1000"))
        {
            return false;
        }
        let fields = group.trim().split(':').collect::<Vec<_>>();
        fields.len() == 4
            && fields[2] == "1000"
            && fields[3]
                .split(',')
                .filter(|name| !name.is_empty())
                .all(|name| {
                    users
                        .iter()
                        .any(|user| user[0] == name && user[2] == "1000")
                })
    })
}

fn safe_mode(meta: &fs::Metadata) -> bool {
    meta.mode() & 0o002 == 0
        && (meta.mode() & 0o020 == 0
            || (meta.uid() == 1000 && meta.gid() == 1000 && private_operator_group()))
}

pub fn checked_path(path: &Path, owner: u32, directory: bool) -> Result<()> {
    ensure!(path.is_absolute(), "Absoluter Pfad erforderlich");
    let mut current = std::path::PathBuf::from("/");
    for component in path.components() {
        match component {
            Component::RootDir => continue,
            Component::Normal(part) => current.push(part),
            _ => bail!("Nicht normalisierter Pfad"),
        }
        let meta = fs::symlink_metadata(&current)?;
        ensure!(
            !meta.file_type().is_symlink(),
            "Symlink im Pfad: {}",
            current.display()
        );
        ensure!(
            meta.uid() == 0 || meta.uid() == owner,
            "Fremder Eigentümer: {}",
            current.display()
        );
        ensure!(
            safe_mode(&meta),
            "Fremd beschreibbarer Pfad: {}",
            current.display()
        );
        if current != path || directory {
            ensure!(meta.is_dir(), "Kein Verzeichnis: {}", current.display());
        }
    }
    let meta = fs::symlink_metadata(path)?;
    ensure!(
        meta.uid() == owner,
        "Unerwarteter Eigentümer: {}",
        path.display()
    );
    Ok(())
}

pub fn regular(path: &Path, owner: u32) -> Result<File> {
    checked_path(path.parent().context("Elternpfad fehlt")?, owner, true)?;
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)?;
    let meta = file.metadata()?;
    ensure!(
        meta.is_file() && meta.uid() == owner && meta.nlink() == 1 && safe_mode(&meta),
        "Unsichere Datei: {}",
        path.display()
    );
    Ok(file)
}

pub fn cargo_artifact(target: &Path, name: &str, owner: u32) -> Result<File> {
    checked_path(target, owner, true)?;
    let release = target.join("release");
    checked_path(&release, owner, true)?;
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(release.join(name))?;
    let meta = file.metadata()?;
    ensure!(
        meta.is_file()
            && meta.uid() == owner
            && safe_mode(&meta)
            && meta.mode() & 0o111 != 0
            && [1, 2].contains(&meta.nlink()),
        "Unsicheres Cargoartefakt: {name}"
    );
    let deps = release.join("deps");
    checked_path(&deps, owner, true)?;
    let prefix = format!("{}-", name.replace('-', "_"));
    let mut aliases = 0;
    for entry in fs::read_dir(&deps)? {
        let entry = entry?;
        let alias = fs::symlink_metadata(entry.path())?;
        if alias.dev() != meta.dev() || alias.ino() != meta.ino() {
            continue;
        }
        let filename = entry.file_name();
        let filename = filename.to_str().context("Ungültiger Cargoalias")?;
        let suffix = filename
            .strip_prefix(&prefix)
            .context("Fremder Cargo-Hardlink")?;
        ensure!(
            alias.is_file()
                && alias.uid() == owner
                && safe_mode(&alias)
                && !suffix.is_empty()
                && suffix.bytes().all(|b| b.is_ascii_hexdigit())
                && alias.nlink() == meta.nlink(),
            "Fremder Cargo-Hardlink"
        );
        aliases += 1;
    }
    ensure!(
        aliases + 1 == meta.nlink(),
        "Cargoartefakt besitzt Hardlinks außerhalb seines Buildlayouts"
    );
    Ok(file)
}

pub fn bytes(path: &Path, owner: u32, limit: u64) -> Result<Vec<u8>> {
    let file = regular(path, owner)?;
    ensure!(file.metadata()?.len() <= limit, "Datei zu groß");
    let mut data = Vec::new();
    file.take(limit + 1).read_to_end(&mut data)?;
    ensure!(data.len() as u64 <= limit, "Datei gewachsen");
    Ok(data)
}

pub fn hash(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}

pub fn file_hash(path: &Path, owner: u32) -> Result<String> {
    let mut file = regular(path, owner)?;
    let mut digest = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        digest.update(&buf[..n]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub fn new_dir(path: &Path) -> Result<()> {
    fs::create_dir(path)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    Ok(())
}

pub fn new_file(path: &Path, data: &[u8], mode: u32) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)?;
    file.write_all(data)?;
    file.sync_all()?;
    Ok(())
}

pub fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;
    Ok(())
}

pub struct Lock(File);
impl Lock {
    pub fn acquire(path: &Path, owner: u32) -> Result<Self> {
        Self::open(path, owner, false)?.context("Sperre nicht erworben")
    }
    pub fn try_acquire(path: &Path, owner: u32) -> Result<Option<Self>> {
        Self::open(path, owner, true)
    }
    fn open(path: &Path, owner: u32, nonblocking: bool) -> Result<Option<Self>> {
        if path != Path::new("/tmp/deadlock-cargo-release.lock") {
            checked_path(path.parent().context("Lock-Elternpfad fehlt")?, owner, true)?;
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(path)?;
        let before = file.metadata()?;
        ensure!(
            before.is_file() && before.uid() == owner && before.nlink() == 1 && safe_mode(&before),
            "Unsichere Sperrdatei"
        );
        use std::os::fd::AsRawFd;
        let flags = libc::LOCK_EX | if nonblocking { libc::LOCK_NB } else { 0 };
        if unsafe { libc::flock(file.as_raw_fd(), flags) } != 0 {
            let error = std::io::Error::last_os_error();
            if nonblocking && error.kind() == std::io::ErrorKind::WouldBlock {
                return Ok(None);
            }
            return Err(error).context("flock fehlgeschlagen");
        }
        let after = fs::symlink_metadata(path)?;
        ensure!(
            before.dev() == after.dev() && before.ino() == after.ino(),
            "Sperrdatei ausgetauscht"
        );
        Ok(Some(Self(file)))
    }
}
impl Drop for Lock {
    fn drop(&mut self) {
        use std::os::fd::AsRawFd;
        unsafe {
            libc::flock(self.0.as_raw_fd(), libc::LOCK_UN);
        }
    }
}

pub fn sha(value: &str) -> Result<()> {
    ensure!(
        value.len() == 40
            && value
                .bytes()
                .all(|x| x.is_ascii_hexdigit() && !x.is_ascii_uppercase()),
        "Ungültiger Commit-SHA"
    );
    Ok(())
}
