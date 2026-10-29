use crate::{sources::InputObjects, InputObject};
use anyhow::{ensure, Context};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub const SOURCE: &str = "deadlock-api-meta";
pub const STAT_SETS: [(&str, &str); 2] = [
    (
        "hero-stats.json",
        "https://api.deadlock-api.com/v1/analytics/hero-stats?game_mode=normal&bucket=no_bucket",
    ),
    (
        "item-stats.json",
        "https://api.deadlock-api.com/v1/analytics/item-stats?game_mode=normal&bucket=no_bucket&min_matches=20",
    ),
];
pub const MAX_BYTES: usize = 1024 * 1024;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(90);
const TOTAL_TIMEOUT: Duration = Duration::from_secs(270);
const BACKOFF: Duration = Duration::from_secs(5);

pub fn window_start(unix_seconds: u64) -> u64 {
    (unix_seconds / 86_400 * 86_400).saturating_sub(30 * 86_400)
}

pub fn window_end(unix_seconds: u64) -> u64 {
    unix_seconds / 3_600 * 3_600
}

pub fn hour(unix_seconds: u64) -> String {
    let days = (unix_seconds / 86_400) as i64;
    let hour = (unix_seconds % 86_400) / 3_600;
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:00:00Z")
}

fn valid_hour(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 20
        && value.ends_with(":00:00Z")
        && bytes.iter().enumerate().all(|(index, byte)| match index {
            4 | 7 => *byte == b'-',
            10 => *byte == b'T',
            13 | 16 => *byte == b':',
            19 => *byte == b'Z',
            _ => byte.is_ascii_digit(),
        })
}

fn read_bounded(path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() <= MAX_BYTES,
        "Metastatistik überschreitet die Größengrenze"
    );
    Ok(bytes)
}

fn objects(directory: &Path, revision: &str) -> anyhow::Result<Vec<InputObject>> {
    ensure!(valid_hour(revision), "Ungültige Snapshot-Stunde");
    STAT_SETS
        .iter()
        .map(|(path, _)| {
            Ok(InputObject {
                revision: revision.to_owned(),
                path: (*path).to_owned(),
                bytes: read_bounded(&directory.join(path))?,
            })
        })
        .collect()
}

fn download(url: &str, cache: &Path) -> anyhow::Result<Vec<u8>> {
    let client = HttpClient::new("deadlock-brain-meta-stats/1", cache)?;
    let response = client.get_bounded(
        url,
        SourceHttpOptions {
            max_bytes: MAX_BYTES,
            request_timeout: REQUEST_TIMEOUT,
            total_timeout: TOTAL_TIMEOUT,
            backoff: BACKOFF,
            ..SourceHttpOptions::default()
        },
    )?;
    response.ensure_success()?;
    ensure!(
        response.content_type().starts_with("application/json"),
        "Metastatistik liefert kein JSON"
    );
    Ok(response.content)
}

fn store(path: &Path, bytes: &[u8]) -> anyhow::Result<()> {
    let partial = path.with_extension("partial");
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&partial)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    std::fs::rename(&partial, path)?;
    Ok(())
}

fn prune(cache: &Path, keep: &str) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(cache)? {
        let entry = entry?;
        let name = entry.file_name();
        if name
            .to_str()
            .is_some_and(|name| valid_hour(name) && name != keep)
            && entry.file_type()?.is_dir()
        {
            std::fs::remove_dir_all(entry.path())?;
        }
    }
    Ok(())
}

pub fn capture(cache: &Path, unix_seconds: u64) -> anyhow::Result<Vec<InputObject>> {
    let revision = hour(unix_seconds);
    ensure!(
        cache.is_absolute() && valid_hour(&revision),
        "Ungültiger Quellen-Cache"
    );
    std::fs::create_dir_all(cache)?;
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(cache.join(".lock"))?;
    lock.lock()
        .context("Quellen-Cache konnte nicht gesperrt werden")?;
    let directory = cache.join(&revision);
    std::fs::create_dir_all(&directory)?;
    let since = window_start(unix_seconds);
    let until = window_end(unix_seconds);
    for (path, url) in STAT_SETS {
        let target = directory.join(path);
        if !target.exists() {
            let url = format!("{url}&min_unix_timestamp={since}&max_unix_timestamp={until}");
            let bytes = std::thread::scope(|scope| {
                scope
                    .spawn(|| download(&url, &cache.join(".http")))
                    .join()
                    .unwrap_or_else(|_| Err(anyhow::anyhow!("Abruf abgebrochen")))
            })?;
            store(&target, &bytes)?;
        }
    }
    prune(cache, &revision)?;
    objects(&directory, &revision)
}

fn now() -> anyhow::Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

fn live(directory: &Path, opened: anyhow::Result<u64>) -> anyhow::Result<Vec<InputObject>> {
    let opened = opened?;
    let unix_seconds = now()?;
    ensure!(
        window_end(opened) == window_end(unix_seconds),
        "Metastatistik-Lauf hat eine Stundengrenze überschritten; der nächste Lauf erfasst neu"
    );
    capture(directory, unix_seconds)
}

pub fn deadlock_api_meta(directory: &Path, fetch: bool) -> InputObjects {
    if !fetch {
        return Box::new(std::iter::once(Err(anyhow::anyhow!(
            "Metastatistiken werden nur live erfasst; nachträgliches Einspielen ist gesperrt, damit die Snapshot-Zeit der Erfassungszeit entspricht."
        ))));
    }
    let directory = directory.to_path_buf();
    let opened = now();
    Box::new(
        std::iter::once_with(move || live(&directory, opened)).flat_map(|loaded| match loaded {
            Ok(objects) => objects.into_iter().map(Ok).collect::<Vec<_>>(),
            Err(error) => vec![Err(error)],
        }),
    )
}
