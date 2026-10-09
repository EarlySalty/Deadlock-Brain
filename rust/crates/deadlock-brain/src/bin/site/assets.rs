use anyhow::{ensure, Context, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::Read,
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;

pub(super) struct Assets {
    root: File,
}

impl Assets {
    pub(super) fn new(root: &Path) -> Result<Self> {
        ensure!(root.is_absolute(), "Der Corpusroot muss absolut sein.");
        let root = File::options()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(root)
            .context("Der Corpusroot ist nicht verfügbar.")?;
        let assets = Self { root };
        assets.heroes()?;
        Ok(assets)
    }

    fn heroes(&self) -> Result<BTreeSet<String>> {
        let registry = self.bytes("builds_registry.json")?;
        let registry: BTreeMap<String, serde_json::Value> = serde_json::from_slice(&registry)
            .context("Das öffentliche Heldenverzeichnis ist ungültig.")?;
        let mut heroes = BTreeSet::new();
        for hero in registry.into_keys() {
            ensure!(
                valid_hero(&hero),
                "Das Heldenverzeichnis enthält ungültige Namen."
            );
            heroes.insert(hero);
        }
        Ok(heroes)
    }

    pub(super) fn read(&self, uri_path: &str) -> Result<(&'static str, Vec<u8>)> {
        let decoded = urlencoding::decode(uri_path).context("Ungültiger Dateipfad.")?;
        let path = if decoded == "/site/" {
            "site/index.html"
        } else {
            decoded.strip_prefix('/').context("Ungültiger Dateipfad.")?
        };
        ensure!(
            !path
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
                && !path.contains(['\\', '\0', '%'])
                && !path.chars().any(char::is_control),
            "Ungültiger Dateipfad."
        );
        let content_type = self
            .content_type(path)
            .context("Nicht öffentliche Datei.")?;
        Ok((content_type, self.bytes(path)?))
    }

    fn content_type(&self, path: &str) -> Option<&'static str> {
        match path {
            "site/index.html" => Some("text/html; charset=utf-8"),
            "site/app.js" | "site/vendor/marked.min.js" => Some("text/javascript; charset=utf-8"),
            "site/style.css" => Some("text/css; charset=utf-8"),
            "builds_registry.json" | "hero_meta_compact.json" | "item_meta_compact.json" => {
                Some("application/json; charset=utf-8")
            }
            "HERO_STRENGTH.md" | "ITEM_STRENGTH.md" | "MASTER_METHODOLOGY.md" => {
                Some("text/markdown; charset=utf-8")
            }
            _ => {
                if let Some(hero) = path
                    .strip_prefix("understanding/")
                    .and_then(|name| name.strip_suffix(".md"))
                {
                    return self
                        .heroes()
                        .ok()?
                        .contains(hero)
                        .then_some("text/markdown; charset=utf-8");
                }
                let entity = path.strip_prefix("site/entities/")?;
                let (kind, filename) = entity.split_once('/')?;
                if !matches!(kind, "hero" | "ability" | "item") {
                    return None;
                }
                let key = filename.strip_suffix(".html")?;
                (!key.is_empty()
                    && key.len() <= 200
                    && key.len().is_multiple_of(2)
                    && key
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
                .then_some("text/html; charset=utf-8")
            }
        }
    }

    fn bytes(&self, path: &str) -> Result<Vec<u8>> {
        let mut directory = self.root.try_clone()?;
        let mut parts = path.split('/').peekable();
        while let Some(part) = parts.next() {
            use rustix::fs::{openat, Mode, OFlags};
            let flags = OFlags::RDONLY
                | OFlags::NOFOLLOW
                | OFlags::CLOEXEC
                | OFlags::NONBLOCK
                | if parts.peek().is_some() {
                    OFlags::DIRECTORY
                } else {
                    OFlags::empty()
                };
            let fd = openat(&directory, part, flags, Mode::empty())
                .context("Öffentliche Datei nicht verfügbar.")?;
            directory = File::from(fd);
        }
        let metadata = directory.metadata()?;
        ensure!(
            metadata.is_file() && metadata.len() <= MAX_FILE_BYTES,
            "Öffentliche Datei nicht verfügbar."
        );
        let mut bytes = Vec::new();
        directory.take(MAX_FILE_BYTES + 1).read_to_end(&mut bytes)?;
        ensure!(
            bytes.len() as u64 <= MAX_FILE_BYTES,
            "Öffentliche Datei zu groß."
        );
        Ok(bytes)
    }
}

fn valid_hero(hero: &str) -> bool {
    !hero.is_empty()
        && hero.chars().count() <= 40
        && hero
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}
