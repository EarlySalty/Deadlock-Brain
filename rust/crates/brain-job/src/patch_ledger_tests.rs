use super::*;

const HOUR: i64 = 3_600_000_000;

fn post(id: i64, title: &str, url: &str, hours: i64, raw: &str) -> Post {
    Post {
        id,
        title: title.into(),
        url: url.into(),
        posted_micros: 1_700_000_000_000_000 + hours * HOUR,
        raw_content: raw.into(),
    }
}

fn notes(prefix: &str, count: usize) -> String {
    (0..count)
        .map(|index| format!("- {prefix} change {index}\n"))
        .collect()
}

fn member(ledger: &Ledger, id: i64) -> &LedgerPost {
    ledger
        .posts
        .iter()
        .find(|post| post.post_id == id)
        .expect("post classified")
}

#[test]
fn forum_mirror_joins_steam_primary_and_keeps_earliest_date() {
    let body = notes("hero", 30);
    let ledger = classify(&[
        post(1, "10-14-2026 Update", "https://forums.example/1", 0, &body),
        post(
            2,
            "Gameplay Update",
            "https://store.steampowered.com/2",
            5,
            &body,
        ),
    ]);
    assert_eq!(ledger.patches.len(), 1);
    let patch = &ledger.patches[0];
    assert_eq!(patch.ledger_id, 2);
    assert_eq!(patch.dated_post_id, 1);
    assert_eq!(patch.released_micros, 1_700_000_000_000_000);
    assert_eq!(member(&ledger, 1).disposition, "duplicate");
    assert_eq!(member(&ledger, 2).disposition, "primary");
}

#[test]
fn steam_news_stub_joins_the_post_it_announces() {
    let ledger = classify(&[
        post(
            1,
            "Minor Update",
            "https://forums.example/1",
            0,
            &notes("fix", 5),
        ),
        post(
            2,
            "Steam stub",
            "https://forums.example/2",
            2,
            "Steam News: Minor Update is live",
        ),
    ]);
    assert_eq!(ledger.patches.len(), 1);
    assert_eq!(member(&ledger, 2).ledger_id, Some(1));
}

#[test]
fn distant_posts_with_same_notes_stay_separate() {
    let body = notes("same", 10);
    let ledger = classify(&[
        post(1, "Update A", "https://forums.example/1", 0, &body),
        post(2, "Update B", "https://forums.example/2", 37, &body),
    ]);
    assert_eq!(ledger.patches.len(), 2);
}

#[test]
fn non_patch_groups_are_excluded() {
    let ledger = classify(&[
        post(1, "Community event", "https://forums.example/1", 0, "hello"),
        post(2, "Update", "http://forums.example/2", 100, &notes("x", 3)),
    ]);
    assert!(ledger.patches.is_empty());
    assert_eq!(member(&ledger, 1).disposition, "excluded");
    assert_eq!(member(&ledger, 2).reason, "no https source url");
}

#[test]
fn colliding_labels_get_the_release_date() {
    let ledger = classify(&[
        post(1, "Update", "https://forums.example/1", 0, &notes("a", 3)),
        post(2, "Update", "https://forums.example/2", 100, &notes("b", 3)),
    ]);
    let labels: Vec<_> = ledger.patches.iter().map(|p| p.label.as_str()).collect();
    assert_eq!(labels, ["Update", "Update (2023-11-19)"]);
}

#[test]
fn kind_follows_primary_size() {
    assert_eq!(kind(15_000), "major");
    assert_eq!(kind(1_500), "gameplay");
    assert_eq!(kind(1_499), "hotfix");
}

#[test]
fn titles_need_whole_words_or_dates() {
    assert!(has_word("minor update", "update"));
    assert!(!has_word("updated heroes", "update"));
    assert!(has_title_date("05-03-2024"));
    assert!(!has_title_date("5-3-2024"));
}
