# Plan: Reddit-Ingest für Deadlock-Brain

status: aktiv
datum: 2026-09-07
contract: CONTRACT.md

Ziel steht im Contract. Jeder Milestone hat Validierung und Stop-Regel.

## Milestone 1 — Parser ohne Netz

Änderungen: `dbrain-sources/src/reddit.rs` mit `SOURCE="reddit"`, Listing- und
Thread-Parser für Reddit-JSON und RSS, Unit-Tests an Fixtures. Noch kein CLI.

Zwischenzustand: `cargo test -p dbrain-sources reddit::` grün.

Stop-Regel: Parser rät Felder oder braucht Live-HTTP.

## Milestone 2 — Pull über SourceStore

Änderungen: `pull_reddit` analog `pull_forum` (Limit, Delay, Skip-existing,
`--refresh-existing`, `--subreddit` wiederholbar, Default `Deadlock`).
`lib.rs` Re-Export. HTTP über `HttpClient`. 403/429 fail-soft.

Zwischenzustand: Tests für Skip-Logik, Delay-Guard, URL-Bau ohne Netz.

Stop-Regel: eigener HTTP-Client, eigener Store, Captcha/Browser.

## Milestone 3 — Claims

Änderungen: `reddit_claims.rs` analog `forum_claims.rs`. Rebuild nur Reddit.
`forum_claims.rs` DELETE nur Forum. Heuristik kopieren, AutoModerator
überspringen, Dev-Flair analog `is_developer`.

Zwischenzustand: Parser-Tests für Hash-Prefix/`ingest_source` und dass
Forum-Rebuild-SQL nicht mehr unscoped ist.

Stop-Regel: `DELETE FROM brain.forum_claims` ohne WHERE, neue Tabelle,
Migration.

## Milestone 4 — CLI + README

Änderungen: `pull reddit`, `parse reddit-claims` in `main.rs`. README-Abschnitt
wie Forum. Keine Code-Kommentare.

Zwischenzustand:
`SQLX_OFFLINE=1 cargo test -p dbrain-sources -p dbrain-normalize -p deadlock-brain --bin deadlock-brain`
und Clippy auf angefasste Crates grün.

Stop-Regel: Workspace-weite fmt-Orgie, ungefasste Dateien.

## Milestone 5 — Selbstprüfung

`gate_hook.py --review --repo Deadlock-Brain --base main --head HEAD`.
BLOCKING beheben, dann fertig melden mit Kritiker-Wortlaut.

Nicht in diesem Slice: Merge nach main, Deploy, Live-Cron.
