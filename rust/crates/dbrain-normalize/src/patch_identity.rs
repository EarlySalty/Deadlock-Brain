use std::collections::{HashMap, HashSet};

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

use crate::util::normalize_key;

const MIN_LINE_OVERLAP: f64 = 0.5;
const STEAM_TEASER_SUFFIX: &str = " - steam news";

#[derive(Debug, Clone, Default)]
pub(crate) struct SnapshotIdentity {
    pub(crate) id: i64,
    pub(crate) title_key: String,
    pub(crate) url_key: String,
    pub(crate) posted_day: Option<NaiveDate>,
    pub(crate) title_day: Option<NaiveDate>,
    pub(crate) teaser_title_key: Option<String>,
    pub(crate) line_keys: HashSet<String>,
}

impl SnapshotIdentity {
    pub(crate) fn new(
        id: i64,
        title: Option<&str>,
        url: Option<&str>,
        posted_at: Option<&str>,
        content: &str,
        line_keys: HashSet<String>,
    ) -> Self {
        Self {
            id,
            title_key: normalize_key(title.unwrap_or_default()),
            url_key: url.map(normalize_url).unwrap_or_default(),
            posted_day: posted_at.and_then(parse_posted_day),
            title_day: title.and_then(title_day),
            teaser_title_key: steam_teaser_title_key(content),
            line_keys,
        }
    }

    fn days(&self) -> impl Iterator<Item = NaiveDate> + '_ {
        self.posted_day.iter().chain(self.title_day.iter()).copied()
    }

    fn primary_day(&self) -> Option<NaiveDate> {
        self.posted_day.or(self.title_day)
    }
}

pub(crate) fn canonical_snapshot_ids(snapshots: &[SnapshotIdentity]) -> HashMap<i64, i64> {
    let mut parents = (0..snapshots.len()).collect::<Vec<_>>();
    for left in 0..snapshots.len() {
        for right in left + 1..snapshots.len() {
            if same_patch(&snapshots[left], &snapshots[right]) {
                union(&mut parents, left, right);
            }
        }
    }
    let mut clusters: HashMap<usize, Vec<usize>> = HashMap::new();
    for index in 0..snapshots.len() {
        let root = find(&mut parents, index);
        clusters.entry(root).or_default().push(index);
    }
    let mut canonical = HashMap::new();
    for members in clusters.values() {
        let Some(best) = members
            .iter()
            .copied()
            .max_by_key(|index| canonical_rank(&snapshots[*index]))
        else {
            continue;
        };
        for index in members {
            canonical.insert(snapshots[*index].id, snapshots[best].id);
        }
    }
    canonical
}

fn canonical_rank(
    snapshot: &SnapshotIdentity,
) -> (bool, usize, bool, bool, std::cmp::Reverse<i64>) {
    (
        !has_stale_title_day(snapshot),
        snapshot.line_keys.len(),
        snapshot.teaser_title_key.is_none(),
        snapshot.posted_day.is_some(),
        std::cmp::Reverse(snapshot.id),
    )
}

fn same_patch(left: &SnapshotIdentity, right: &SnapshotIdentity) -> bool {
    let same_day = left.primary_day().is_some() && left.primary_day() == right.primary_day();
    if same_day && !left.title_key.is_empty() && left.title_key == right.title_key {
        return true;
    }
    if same_day && !left.url_key.is_empty() && left.url_key == right.url_key {
        return true;
    }
    if !days_near(left, right) {
        return false;
    }
    is_teaser_of(left, right) || is_teaser_of(right, left) || lines_overlap(left, right)
}

fn has_stale_title_day(snapshot: &SnapshotIdentity) -> bool {
    match (snapshot.title_day, snapshot.posted_day) {
        (Some(title_day), Some(posted_day)) => (posted_day - title_day).num_days().abs() > 1,
        _ => false,
    }
}

fn days_near(left: &SnapshotIdentity, right: &SnapshotIdentity) -> bool {
    left.days().any(|left_day| {
        right
            .days()
            .any(|right_day| (left_day - right_day).num_days().abs() <= 1)
    })
}

fn is_teaser_of(teaser: &SnapshotIdentity, target: &SnapshotIdentity) -> bool {
    teaser
        .teaser_title_key
        .as_deref()
        .is_some_and(|title| !title.is_empty() && title == target.title_key)
}

fn lines_overlap(left: &SnapshotIdentity, right: &SnapshotIdentity) -> bool {
    let larger = left.line_keys.len().max(right.line_keys.len());
    if left.line_keys.is_empty() || right.line_keys.is_empty() {
        return false;
    }
    let shared = left.line_keys.intersection(&right.line_keys).count();
    shared as f64 / larger as f64 >= MIN_LINE_OVERLAP
}

fn find(parents: &mut [usize], index: usize) -> usize {
    let mut root = index;
    while parents[root] != root {
        root = parents[root];
    }
    let mut current = index;
    while parents[current] != root {
        let next = parents[current];
        parents[current] = root;
        current = next;
    }
    root
}

fn union(parents: &mut [usize], left: usize, right: usize) {
    let left_root = find(parents, left);
    let right_root = find(parents, right);
    if left_root != right_root {
        parents[left_root.max(right_root)] = left_root.min(right_root);
    }
}

fn normalize_url(url: &str) -> String {
    url.trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_lowercase()
}

fn steam_teaser_title_key(content: &str) -> Option<String> {
    let first_line = content
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())?;
    let lower = first_line.to_lowercase();
    let end = lower.find(STEAM_TEASER_SUFFIX)?;
    let title = lower[..end].trim().strip_prefix("deadlock - ")?;
    Some(normalize_key(title))
}

fn title_day(title: &str) -> Option<NaiveDate> {
    title.as_bytes().windows(10).find_map(|window| {
        let shaped = window.iter().enumerate().all(|(position, byte)| {
            if position == 2 || position == 5 {
                *byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        });
        if !shaped {
            return None;
        }
        let text = std::str::from_utf8(window).ok()?;
        NaiveDate::parse_from_str(text, "%m-%d-%Y").ok()
    })
}

fn parse_posted_day(value: &str) -> Option<NaiveDate> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if value.chars().all(|ch| ch.is_ascii_digit()) {
        return DateTime::<Utc>::from_timestamp(value.parse().ok()?, 0).map(|at| at.date_naive());
    }
    let normalized = if value.ends_with("+00") || value.ends_with("-00") {
        format!("{value}:00")
    } else {
        value.to_string()
    };
    if let Ok(at) = DateTime::parse_from_rfc3339(&normalized) {
        return Some(at.with_timezone(&Utc).date_naive());
    }
    for format in [
        "%Y-%m-%dT%H:%M:%S%z",
        "%Y-%m-%d %H:%M:%S%:z",
        "%Y-%m-%d %H:%M:%S%z",
    ] {
        if let Ok(at) = DateTime::parse_from_str(&normalized, format) {
            return Some(at.with_timezone(&Utc).date_naive());
        }
    }
    if let Ok(at) = NaiveDateTime::parse_from_str(&normalized, "%Y-%m-%d %H:%M:%S") {
        return Some(at.date());
    }
    NaiveDate::parse_from_str(value.get(..10)?, "%Y-%m-%d").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(
        id: i64,
        title: &str,
        url: &str,
        posted_at: &str,
        content: &str,
        lines: &[&str],
    ) -> SnapshotIdentity {
        SnapshotIdentity::new(
            id,
            Some(title),
            Some(url),
            Some(posted_at),
            content,
            lines.iter().map(|line| normalize_key(line)).collect(),
        )
    }

    #[test]
    fn forum_reply_with_a_stale_title_date_does_not_name_the_patch() {
        let lines = [
            "Radiant Regeneration: Spirit scaling reduced from 0.04 to 0.035",
            "Baba: Threadsap - Cooldown increased from 26s to 28s",
            "Baba: Granny Long Legs - Cooldown increased from 30s to 32s",
        ];
        let snapshots = vec![
            snapshot(
                294,
                " Minor Update - 10-14-2026",
                "https://store.steampowered.com/news/app/1422450/view/669505296465396213",
                "2026-10-15T03:11:58Z",
                "",
                &lines[..2],
            ),
            snapshot(
                295,
                "09-29-2026",
                "https://forums.playdeadlock.com/posts/330973/",
                "2026-10-15T03:12:00Z",
                "",
                &lines,
            ),
        ];
        let canonical = canonical_snapshot_ids(&snapshots);
        assert_eq!(canonical.get(&294), Some(&294));
        assert_eq!(canonical.get(&295), Some(&294));
    }

    #[test]
    fn steam_and_forum_copies_of_one_patch_share_a_canonical_snapshot() {
        let lines = [
            "Fixed troopers walking too slowly",
            "Parry cooldown reduced from 6s to 5s",
            "Abrams: Siphon Life damage increased from 10 to 12",
        ];
        let snapshots = vec![
            snapshot(
                1,
                " Minor Update - 05-28-2026",
                "https://steamcommunity.com/games/1422450/announcements/detail/1",
                "1779988499",
                "",
                &lines[..2],
            ),
            snapshot(
                2,
                "05-28-2026 Update",
                "https://forums.playdeadlock.com/threads/05-22-2026-update.135477/post-261657",
                "2026-05-28",
                "",
                &lines,
            ),
            snapshot(
                3,
                "05-29-2026 Update",
                "https://forums.playdeadlock.com/threads/05-29-2026-update.1/",
                "2026-05-29",
                "",
                &["Rem: Lantern radius increased from 8m to 9m"],
            ),
        ];
        let canonical = canonical_snapshot_ids(&snapshots);
        assert_eq!(canonical.get(&1), Some(&2));
        assert_eq!(canonical.get(&2), Some(&2));
        assert_eq!(canonical.get(&3), Some(&3));
    }

    #[test]
    fn forum_steam_news_teaser_joins_the_titled_steam_patch() {
        let snapshots = vec![
            snapshot(
                10,
                "05-08-2025 Update",
                "https://forums.playdeadlock.com/threads/05-08-2025-update.63133/",
                "2025-05-08T12:43:20-0700",
                "Deadlock - Shop Rework Update - Steam News\n\nFull shop rework",
                &["Full shop rework including many new items"],
            ),
            snapshot(
                11,
                "Shop Rework Update",
                "https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1799088287841594",
                "2025-05-08T19:33:12+00:00",
                "Shop rework",
                &["Added Rusted Barrel", "Removed Mystic Slow", "Reworked the shop"],
            ),
        ];
        let canonical = canonical_snapshot_ids(&snapshots);
        assert_eq!(canonical.get(&10), Some(&11));
    }

    #[test]
    fn same_forum_thread_posts_on_different_days_stay_separate() {
        let snapshots = vec![
            snapshot(
                20,
                "11-21-2025 Update",
                "https://forums.playdeadlock.com/threads/11-21-2025-update.1/",
                "2025-11-21T20:00:00+00:00",
                "",
                &["Fixed a bug with falloff calculation"],
            ),
            snapshot(
                21,
                "11-21-2025 Update",
                "https://forums.playdeadlock.com/threads/11-21-2025-update.1/",
                "2025-11-22T20:00:00+00:00",
                "",
                &["Haze: Bullet Dance duration reduced from 4s to 3s"],
            ),
        ];
        let canonical = canonical_snapshot_ids(&snapshots);
        assert_eq!(canonical.get(&20), Some(&20));
        assert_eq!(canonical.get(&21), Some(&21));
    }
}
