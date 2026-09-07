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

## Status

- Milestone 1 — erledigt, Commit c4b73c3. `SQLX_OFFLINE=1 cargo test -p dbrain-sources reddit::` grün (5 Parser-Tests an Listing-JSON, Thread-JSON inkl. kind=more-Skip, Atom-RSS, Permalink-Verwerfen).
- Milestone 2 — erledigt, Commit f68a55e. 11 Tests grün: Delay-Guard, Skip-existing-Matrix, `thread:{id}`-External-ID, URL-Bau JSON/RSS, Default-Subreddit, RFC3339-Zeit. Fail-soft pro Thread, RSS-Fallback bei Listing und Thread.
- Milestone 3 — erledigt, Commit 56177f8. Forum-Rebuild-DELETE source-scoped (`metadata->>'ingest_source'` fehlt oder playdeadlock_forum), Reddit-Rebuild löscht nur reddit; Tests auf Hash-Eingabe `reddit|`, ingest_source-Metadaten, AutoModerator-Skip, Valve-Flair.
- Milestone 4 — erledigt, Commit 1b06efa. CLI `pull reddit` / `parse reddit-claims` verdrahtet, README-Abschnitt analog Forum. Validierung: `SQLX_OFFLINE=1 cargo test -p dbrain-sources -p dbrain-normalize -p deadlock-brain --bin deadlock-brain` grün (52/12/38), Clippy `-D warnings` auf allen drei Crates clean.
- Milestone 5 — erledigt. Kritiker `gate_hook.py --review --base main --head HEAD`: ALLOW, keine BLOCKING-Funde, sechs NITs. Fünf NITs in derselben Runde behoben (RSS-Fallback auch bei 200 mit unlesbarem Body, Submission-/Kommentar-Klassifikation im Thread-RSS über Permalink-Segmente statt Positionsannahme, Subreddit im Listing-RSS aus dem Permalink, `--refresh-existing` umgeht den HTTP-Cache via TTL 0, Forum-`claims_total` gescoped). Verbleibender NIT: `is_developer` stützt sich auf das nutzergesetzte `author_flair_text` — das ist laut Contract wörtlich vorgeschrieben („Reddit-Unterscheidung über author_flair/user_title analog Valve-Developer“); Live-Check der Flair-Vergabe in r/Deadlock war ohne Reddit-Zugriff nicht Teil des Slices. Final: 55 (sources) / 13 (normalize) / 38 (CLI) Tests grün, Clippy `-D warnings` clean.
