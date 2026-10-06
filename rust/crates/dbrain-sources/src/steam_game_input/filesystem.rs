use std::{
    collections::BTreeMap,
    fs::File,
    io,
    os::fd::AsRawFd,
    path::{Component, Path, PathBuf},
};

use rustix::fs::{self, AtFlags, FileType, Mode, OFlags, RenameFlags};
use tempfile::TempDir;

use super::{invalid, InventoryFileKind, SteamGameInputLimits};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Identity {
    device: u64,
    inode: u64,
    mode: u32,
    links: u64,
    pub bytes: u64,
    mtime: i64,
    mtime_nsec: u64,
    ctime: i64,
    ctime_nsec: u64,
}

fn identity(stat: &fs::Stat) -> io::Result<Identity> {
    Ok(Identity {
        device: stat.st_dev,
        inode: stat.st_ino,
        mode: stat.st_mode,
        links: stat.st_nlink,
        bytes: u64::try_from(stat.st_size).map_err(|_| invalid("Negative Dateigröße"))?,
        mtime: stat.st_mtime,
        mtime_nsec: stat.st_mtime_nsec,
        ctime: stat.st_ctime,
        ctime_nsec: stat.st_ctime_nsec,
    })
}

fn open_directory(parent: &File, name: &std::ffi::OsStr) -> io::Result<File> {
    let before = fs::statat(parent, name, AtFlags::SYMLINK_NOFOLLOW)?;
    let child: File = fs::openat(
        parent,
        name,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
    )?
    .into();
    if identity(&before)? != identity(&fs::fstat(&child)?)? {
        return Err(invalid(
            "Verzeichnis wurde beim verankerten Öffnen ausgetauscht",
        ));
    }
    Ok(child)
}

pub(super) fn open_root(path: &Path) -> io::Result<File> {
    if !path.is_absolute() {
        return Err(invalid("Verankertes Öffnen verlangt einen absoluten Pfad"));
    }
    let mut root: File = fs::open(
        "/",
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    )?
    .into();
    for component in path.components() {
        match component {
            Component::RootDir => {}
            Component::Normal(name) => root = open_directory(&root, name)?,
            _ => return Err(invalid("Ungültiger verankerter Rootpfad")),
        }
    }
    Ok(root)
}

pub(super) fn check_root_binding(root: &File, path: &Path) -> io::Result<()> {
    let named = open_root(path)?;
    let current = fs::fstat(&named)?;
    let held = fs::fstat(root)?;
    if current.st_dev != held.st_dev || current.st_ino != held.st_ino {
        return Err(invalid(
            "Depotroot wurde während der Lesesicherung ausgetauscht",
        ));
    }
    Ok(())
}

fn parent_at<'a>(root: &File, path: &'a str) -> io::Result<(File, &'a str)> {
    let (parents, name) = path.rsplit_once('/').unwrap_or(("", path));
    let mut parent = root.try_clone()?;
    if !parents.is_empty() {
        for part in parents.split('/') {
            parent = open_directory(&parent, std::ffi::OsStr::new(part))?;
        }
    }
    Ok((parent, name))
}

pub(super) fn open_bound_file(root: &File, path: &str, expected: &Identity) -> io::Result<File> {
    let (parent, name) = parent_at(root, path)?;
    let before = identity(&fs::statat(&parent, name, AtFlags::SYMLINK_NOFOLLOW)?)?;
    if &before != expected {
        return Err(invalid(format!(
            "Inventardatei wurde vor dem Lesen ausgetauscht: {path}"
        )));
    }
    let file: File = fs::openat(
        &parent,
        name,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
        Mode::empty(),
    )?
    .into();
    check_file(&file, expected)?;
    Ok(file)
}

pub(super) fn open_explicit_file(path: &Path) -> io::Result<(File, Identity)> {
    let name = path
        .file_name()
        .ok_or_else(|| invalid("Expliziter Dateiname fehlt"))?;
    let parent = open_root(
        path.parent()
            .ok_or_else(|| invalid("Dateielternpfad fehlt"))?,
    )?;
    let stat = fs::statat(&parent, name, AtFlags::SYMLINK_NOFOLLOW)?;
    if FileType::from_raw_mode(stat.st_mode) != FileType::RegularFile || stat.st_nlink != 1 {
        return Err(invalid(
            "Inventar muss eine eigenständige reguläre Datei sein",
        ));
    }
    let expected = identity(&stat)?;
    let name = name
        .to_str()
        .ok_or_else(|| invalid("Inventardateiname ist kein UTF-8"))?;
    let file = open_bound_file(&parent, name, &expected)?;
    Ok((file, expected))
}

pub(super) fn check_file(file: &File, expected: &Identity) -> io::Result<()> {
    let stat = fs::fstat(file)?;
    if FileType::from_raw_mode(stat.st_mode) != FileType::RegularFile
        || stat.st_nlink != 1
        || &identity(&stat)? != expected
    {
        return Err(invalid(
            "Dateityp, Hardlinkzahl oder Dateiidentität wurde verändert",
        ));
    }
    Ok(())
}

pub(super) fn observe_tree(
    root: &File,
    expected: &BTreeMap<String, InventoryFileKind>,
    limits: &SteamGameInputLimits,
) -> io::Result<BTreeMap<String, Identity>> {
    fn walk(
        directory: &File,
        prefix: &str,
        expected: &BTreeMap<String, InventoryFileKind>,
        limits: &SteamGameInputLimits,
        observed: &mut BTreeMap<String, Identity>,
    ) -> io::Result<()> {
        let mut entries = fs::Dir::read_from(directory)?;
        for entry in &mut entries {
            let entry = entry?;
            let name = entry.file_name();
            if name.to_bytes() == b"." || name.to_bytes() == b".." {
                continue;
            }
            let name = name
                .to_str()
                .map_err(|_| invalid("Nicht-UTF-8-Depotpfad"))?;
            let path = if prefix.is_empty() {
                name.to_owned()
            } else {
                format!("{prefix}/{name}")
            };
            super::inventory::validate_relative(&path, limits)?;
            if observed.len() >= limits.max_entries {
                return Err(invalid(
                    "Tatsächlicher Bestand überschreitet die Eintragsgrenze",
                ));
            }
            let kind = expected.get(&path).ok_or_else(|| {
                invalid(format!(
                    "Nicht manifestbelegter Fremdpfad bleibt unverändert: {path}"
                ))
            })?;
            let stat = fs::statat(directory, name, AtFlags::SYMLINK_NOFOLLOW)?;
            let actual = FileType::from_raw_mode(stat.st_mode);
            if !matches!(
                (kind, actual),
                (InventoryFileKind::Directory, FileType::Directory)
                    | (InventoryFileKind::File, FileType::RegularFile)
            ) || actual == FileType::RegularFile && stat.st_nlink != 1
            {
                return Err(invalid(format!(
                    "Unzulässiger Typ oder Hardlink im Depot: {path}"
                )));
            }
            observed.insert(path.clone(), identity(&stat)?);
            if actual == FileType::Directory {
                let child = open_directory(directory, std::ffi::OsStr::new(name))?;
                walk(&child, &path, expected, limits, observed)?;
            }
        }
        Ok(())
    }
    let mut observed = BTreeMap::new();
    walk(root, "", expected, limits, &mut observed)?;
    if observed.len() != expected.len() {
        let missing = expected.keys().find(|path| !observed.contains_key(*path));
        return Err(invalid(format!(
            "Inventarisierter Depotpfad fehlt: {missing:?}"
        )));
    }
    Ok(observed)
}

pub(super) fn require_descriptors(files: usize, limits: &SteamGameInputLimits) -> io::Result<()> {
    let needed = files
        .checked_add(limits.fd_reserve)
        .ok_or_else(|| invalid("Dateideskriptorbedarf übergelaufen"))?;
    if needed > limits.max_open_files {
        return Err(invalid(
            "Lesesicherungen überschreiten die konfigurierte FD-Grenze",
        ));
    }
    let current = std::fs::read_dir("/proc/self/fd")?.try_fold(0_usize, |count, entry| {
        entry?;
        count
            .checked_add(1)
            .ok_or_else(|| invalid("Dateideskriptoranzahl übergelaufen"))
    })?;
    let required = current
        .checked_add(needed)
        .ok_or_else(|| invalid("FD-Gesamtbedarf übergelaufen"))?;
    let soft = rustix::process::getrlimit(rustix::process::Resource::Nofile).current;
    if soft.is_some_and(|limit| required as u64 > limit) {
        return Err(invalid(
            "Zu wenig echte Dateideskriptoren für vollständigen Handlebestand",
        ));
    }
    Ok(())
}

pub(super) fn require_space(directory: &File, bytes: u64, reserve: u64) -> io::Result<()> {
    let stat = fs::fstatvfs(directory)?;
    let available = stat
        .f_bavail
        .checked_mul(stat.f_frsize)
        .ok_or_else(|| invalid("Freier Speicher übergelaufen"))?;
    let required = bytes
        .checked_add(reserve)
        .ok_or_else(|| invalid("Speicherbedarf übergelaufen"))?;
    if available < required {
        return Err(invalid(
            "Zu wenig echter freier Speicher für vollständige Sicherung und Reserve",
        ));
    }
    Ok(())
}

pub(super) fn finish_private_file(file: &File) -> io::Result<()> {
    fs::fchmod(file, Mode::from_raw_mode(0o400))?;
    file.sync_all()
}

pub(super) struct PrivateDirectory {
    temporary: TempDir,
    directory: File,
    base: File,
    parent_path: PathBuf,
}

impl PrivateDirectory {
    pub fn new(parent: &Path) -> io::Result<Self> {
        let base = open_root(parent)?;
        let stat = fs::fstat(&base)?;
        if stat.st_mode & 0o077 != 0 || stat.st_uid != rustix::process::geteuid().as_raw() {
            return Err(invalid(
                "Sicherungs- und Ausgabeelternverzeichnis muss privat und eigentümereigen sein",
            ));
        }
        let location = PathBuf::from(format!("/proc/self/fd/{}", base.as_raw_fd()));
        let temporary = tempfile::Builder::new()
            .prefix(".steam-input-")
            .tempdir_in(&location)?;
        let name = temporary
            .path()
            .file_name()
            .ok_or_else(|| invalid("Temporärer Verzeichnisname fehlt"))?;
        let directory = open_directory(&base, name)?;
        fs::fchmod(&directory, Mode::from_raw_mode(0o700))?;
        Ok(Self {
            temporary,
            directory,
            base,
            parent_path: parent.to_owned(),
        })
    }

    pub fn for_destination(destination: &Path) -> io::Result<Self> {
        let name = destination
            .file_name()
            .ok_or_else(|| invalid("Artefaktname fehlt"))?;
        if name == "." || name == ".." {
            return Err(invalid("Ungültiger Artefaktname"));
        }
        Self::new(
            destination
                .parent()
                .ok_or_else(|| invalid("Artefaktelternpfad fehlt"))?,
        )
    }

    pub fn handle(&self) -> &File {
        &self.directory
    }

    pub fn create_file(&self, name: &str) -> io::Result<File> {
        Ok(fs::openat(
            &self.directory,
            name,
            OFlags::RDWR | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::from_raw_mode(0o600),
        )?
        .into())
    }

    pub fn detach_readonly(&self, writer: File, name: &str) -> io::Result<File> {
        finish_private_file(&writer)?;
        let expected = identity(&fs::fstat(&writer)?)?;
        let readonly = open_bound_file(&self.directory, name, &expected)?;
        drop(writer);
        fs::unlinkat(&self.directory, name, AtFlags::empty())?;
        let stat = fs::fstat(&readonly)?;
        if stat.st_nlink != 0 || stat.st_dev != expected.device || stat.st_ino != expected.inode {
            return Err(invalid(
                "Private Lesesicherung konnte nicht exklusiv abgelöst werden",
            ));
        }
        Ok(readonly)
    }

    fn check_parent_binding(&self) -> io::Result<()> {
        let named = open_root(&self.parent_path)?;
        let current = fs::fstat(&named)?;
        let held = fs::fstat(&self.base)?;
        if current.st_dev != held.st_dev || current.st_ino != held.st_ino {
            return Err(invalid(
                "Artefaktelternpfad wurde während der Extraktion ausgetauscht",
            ));
        }
        Ok(())
    }

    pub fn publish(self, destination: &Path) -> io::Result<()> {
        self.check_parent_binding()?;
        self.directory.sync_all()?;
        let source = self
            .temporary
            .path()
            .file_name()
            .ok_or_else(|| invalid("Stagingname fehlt"))?;
        let target = destination
            .file_name()
            .ok_or_else(|| invalid("Zielname fehlt"))?;
        fs::renameat_with(
            &self.base,
            source,
            &self.base,
            target,
            RenameFlags::NOREPLACE,
        )?;
        if let Err(error) = self
            .base
            .sync_all()
            .and_then(|()| self.check_parent_binding())
        {
            let target_stat = fs::statat(&self.base, target, AtFlags::SYMLINK_NOFOLLOW)?;
            let own_stat = fs::fstat(&self.directory)?;
            if target_stat.st_dev != own_stat.st_dev || target_stat.st_ino != own_stat.st_ino {
                return Err(invalid("Artefaktpfad wurde vor dem Rücknehmen einer fehlgeschlagenen Veröffentlichung fremd verändert"));
            }
            fs::renameat_with(
                &self.base,
                target,
                &self.base,
                source,
                RenameFlags::NOREPLACE,
            )?;
            self.base.sync_all()?;
            return Err(error);
        }
        let _retained = self.temporary.keep();
        Ok(())
    }
}
