use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use std::{
    fs::{File, OpenOptions},
    io::Read,
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::Path,
};

pub const TRUSTED_UID: u32 = 1000;
const MAX_BYTES: u64 = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionError {
    Unavailable,
    UnsafeFile,
    InvalidState,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    schema_version: u32,
    provider: String,
    model: String,
    release: Option<String>,
    probed_at: String,
    checked_at: String,
}

fn timestamp(value: &str) -> Result<DateTime<Utc>, SelectionError> {
    value
        .parse::<DateTime<Utc>>()
        .map_err(|_| SelectionError::InvalidState)
}

pub fn version(model: &str) -> Option<Vec<u32>> {
    let rest = model.strip_prefix("accounts/fireworks/models/deepseek-v")?;
    let (number, suffix) = rest.split_once("-flash")?;
    if number.is_empty() || number.len() > 24 {
        return None;
    }
    let mut parts = number
        .split(['p', '.'])
        .map(|p| {
            if p.is_empty()
                || !p.bytes().all(|b| b.is_ascii_digit())
                || (p.len() > 1 && p.starts_with('0'))
            {
                None
            } else {
                p.parse::<u32>().ok()
            }
        })
        .collect::<Option<Vec<_>>>()?;
    if parts.len() > 4 || parts.as_slice() < [4, 1].as_slice() {
        return None;
    }
    if !suffix.is_empty() {
        let date = suffix.strip_prefix('-')?;
        if !date.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let valid = match date.len() {
            4 => NaiveDate::parse_from_str(&format!("2000{date}"), "%Y%m%d").is_ok(),
            8 => NaiveDate::parse_from_str(date, "%Y%m%d").is_ok(),
            _ => false,
        };
        if !valid {
            return None;
        }
    }
    while parts.len() > 1 && parts.last() == Some(&0) {
        parts.pop();
    }
    Some(parts)
}

impl Selection {
    fn validated_model(self) -> Result<String, SelectionError> {
        let now = Utc::now() + chrono::Duration::minutes(5);
        let probed_at = timestamp(&self.probed_at)?;
        let checked_at = timestamp(&self.checked_at)?;
        let release = self.release.as_deref().map(timestamp).transpose()?;
        if self.schema_version != 1
            || self.provider != "fireworks"
            || version(&self.model).is_none()
            || probed_at > checked_at
            || checked_at > now
            || probed_at.timestamp() <= 0
            || release.is_some_and(|date| date > now)
        {
            return Err(SelectionError::InvalidState);
        }
        Ok(self.model)
    }
}

fn protected_file(path: &Path, trusted_uid: u32) -> Result<File, SelectionError> {
    if !path.is_absolute() {
        return Err(SelectionError::UnsafeFile);
    }
    for parent in path.ancestors().skip(1) {
        let meta = std::fs::symlink_metadata(parent).map_err(|_| SelectionError::Unavailable)?;
        if !meta.is_dir() || ![0, trusted_uid].contains(&meta.uid()) || meta.mode() & 0o022 != 0 {
            return Err(SelectionError::UnsafeFile);
        }
    }
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK)
        .open(path)
        .map_err(|_| SelectionError::Unavailable)?;
    let meta = file.metadata().map_err(|_| SelectionError::Unavailable)?;
    if !meta.is_file()
        || ![0, trusted_uid].contains(&meta.uid())
        || meta.mode() & 0o022 != 0
        || meta.nlink() != 1
        || meta.len() > MAX_BYTES
    {
        return Err(SelectionError::UnsafeFile);
    }
    Ok(file)
}

pub fn selected_model(path: &Path) -> Result<String, SelectionError> {
    let mut bytes = Vec::new();
    protected_file(path, TRUSTED_UID)?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| SelectionError::Unavailable)?;
    if bytes.len() > MAX_BYTES as usize {
        return Err(SelectionError::InvalidState);
    }
    serde_json::from_slice::<Selection>(&bytes)
        .map_err(|_| SelectionError::InvalidState)?
        .validated_model()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::{symlink, PermissionsExt};

    fn selection(model: &str, checked_at: DateTime<Utc>) -> serde_json::Value {
        serde_json::json!({
            "schema_version": 1,
            "provider": "fireworks",
            "model": model,
            "release": "2026-09-10T05:43:34.484755Z",
            "probed_at": checked_at.to_rfc3339(),
            "checked_at": checked_at.to_rfc3339(),
        })
    }

    #[test]
    fn numeric_versions_and_strict_family() {
        let id = |n: &str| format!("accounts/fireworks/models/deepseek-{n}");
        assert!(version(&id("v4p10-flash")) > version(&id("v4p9-flash")));
        assert!(version(&id("v10-flash")) > version(&id("v4p10-flash")));
        for bad in [
            "v4-pro",
            "v4p1-flash-preview",
            "v4p1-flash-0230",
            "v04-flash",
            "v4p0-flash",
        ] {
            assert!(version(&id(bad)).is_none(), "{bad}");
        }
        assert!(version("accounts/private/models/deepseek-v4p1-flash").is_none());
    }

    #[test]
    fn protected_file_is_read_at_call_time_and_fails_closed() {
        let base = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .find(|path| {
                path.ancestors().all(|ancestor| {
                    std::fs::symlink_metadata(ancestor).is_ok_and(|meta| {
                        meta.is_dir()
                            && [0, TRUSTED_UID].contains(&meta.uid())
                            && meta.mode() & 0o022 == 0
                    })
                })
            })
            .unwrap();
        let dir = tempfile::Builder::new()
            .prefix(".brain-selection-test-")
            .tempdir_in(base)
            .unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let path = dir.path().join("selection.json");
        let store = |value: &serde_json::Value, mode: u32| {
            let next = dir.path().join("next.json");
            std::fs::write(&next, serde_json::to_vec(value).unwrap()).unwrap();
            std::fs::set_permissions(&next, std::fs::Permissions::from_mode(mode)).unwrap();
            std::fs::rename(next, &path).unwrap();
        };
        let now = Utc::now();
        assert_eq!(selected_model(&path), Err(SelectionError::Unavailable));
        store(
            &selection("accounts/fireworks/models/deepseek-v4p1-flash", now),
            0o644,
        );
        assert_eq!(
            selected_model(&path).unwrap(),
            "accounts/fireworks/models/deepseek-v4p1-flash"
        );
        store(
            &selection("accounts/fireworks/models/deepseek-v4p2-flash", now),
            0o644,
        );
        assert_eq!(
            selected_model(&path).unwrap(),
            "accounts/fireworks/models/deepseek-v4p2-flash"
        );
        store(
            &selection("accounts/fireworks/models/deepseek-v4p2-flash", now),
            0o666,
        );
        assert_eq!(selected_model(&path), Err(SelectionError::UnsafeFile));
        store(
            &selection(
                "accounts/fireworks/models/deepseek-v4p2-flash",
                now + chrono::Duration::days(1),
            ),
            0o644,
        );
        assert_eq!(selected_model(&path), Err(SelectionError::InvalidState));
        let mut foreign = selection("accounts/fireworks/models/deepseek-v4p2-flash", now);
        foreign["provider"] = serde_json::json!("other");
        store(&foreign, 0o644);
        assert_eq!(selected_model(&path), Err(SelectionError::InvalidState));
        let mut extra = selection("accounts/fireworks/models/deepseek-v4p2-flash", now);
        extra["override"] = serde_json::json!(true);
        store(&extra, 0o644);
        assert_eq!(selected_model(&path), Err(SelectionError::InvalidState));
        store(
            &selection("accounts/fireworks/models/deepseek-v4p1-flash", now),
            0o644,
        );
        let alias = dir.path().join("alias.json");
        symlink(&path, &alias).unwrap();
        assert!(selected_model(&alias).is_err());
        std::fs::write(&path, vec![b' '; MAX_BYTES as usize + 1]).unwrap();
        assert!(selected_model(&path).is_err());
        assert_eq!(
            selected_model(Path::new("selection.json")),
            Err(SelectionError::UnsafeFile)
        );
    }
}
