use super::invalid;
use std::{
    fs::{File, OpenOptions},
    io,
    path::{Component, Path, PathBuf},
};

#[cfg(target_os = "linux")]
fn open(path: &Path, directory: bool) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    // Linux O_NOFOLLOW | O_NONBLOCK. Directory opens also require O_DIRECTORY.
    let flags = 0x20000 | 0x800 | if directory { 0x10000 } else { 0 };
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(flags)
        .open(path)?;
    let metadata = file.metadata()?;
    if directory && !metadata.is_dir() || !directory && !metadata.is_file() {
        return Err(invalid("Keine reguläre Spieldatei oder kein Verzeichnis"));
    }
    Ok(file)
}
#[cfg(not(target_os = "linux"))]
fn open(_: &Path, _: bool) -> io::Result<File> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Sicher verankerte Extraktion benötigt Linux und /proc/self/fd",
    ))
}

pub(super) fn directory(path: &Path) -> io::Result<File> {
    open(path, true)
}
pub(super) fn regular(path: &Path) -> io::Result<File> {
    open(path, false)
}
#[cfg(target_os = "linux")]
pub(super) fn location(file: &File) -> io::Result<PathBuf> {
    use std::os::fd::AsRawFd;
    let path = PathBuf::from(format!("/proc/self/fd/{}", file.as_raw_fd()));
    if !path.is_dir() {
        return Err(invalid("/proc/self/fd ist nicht verfügbar"));
    }
    Ok(path)
}
#[cfg(not(target_os = "linux"))]
pub(super) fn location(_: &File) -> io::Result<PathBuf> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Linux erforderlich",
    ))
}

// Open every root ancestor relative to a held descriptor. No canonicalize /
// absolute reopen after validation, and no symlink component is followed.
pub(super) fn root(path: &Path) -> io::Result<File> {
    let mut handle = directory(if path.is_absolute() {
        Path::new("/")
    } else {
        Path::new(".")
    })?;
    for part in path.components() {
        match part {
            Component::RootDir | Component::CurDir => {}
            Component::Normal(name) => {
                handle = directory(&location(&handle)?.join(name))?;
            }
            _ => return Err(invalid("Ungültiger Root-Pfad")),
        }
    }
    Ok(handle)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replaced_ancestor_cannot_redirect_held_directory() {
        let t = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        std::fs::create_dir(t.path().join("root")).unwrap();
        std::fs::write(t.path().join("root/data.txt"), "inside").unwrap();
        std::fs::write(outside.path().join("data.txt"), "outside").unwrap();
        let held = root(&t.path().join("root")).unwrap();
        std::fs::rename(t.path().join("root"), t.path().join("old")).unwrap();
        std::os::unix::fs::symlink(outside.path(), t.path().join("root")).unwrap();
        let mut file = regular(&location(&held).unwrap().join("data.txt")).unwrap();
        let mut text = String::new();
        std::io::Read::read_to_string(&mut file, &mut text).unwrap();
        assert_eq!(text, "inside");
        assert!(root(&t.path().join("root")).is_err());
    }
}
