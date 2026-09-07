# Evidence: Reddit-Ingest für Deadlock-Brain

status: aktiv
datum: 2026-09-07
contract: CONTRACT.md

Repo-Aufklärung vor dem ersten Edit. Jede Zeile ist eine Fundstelle `pfad:zeile`,
keine Vermutung. Der Hook (R11) gibt Quellcode-Edits erst frei, wenn hier
mindestens 3 Fundstellen stehen. Drei ist die Untergrenze, nicht das Ziel.

## Analoge Implementierungen (wie löst das Repo so etwas schon?)

- `rust/crates/dbrain-sources/src/forum.rs:17` — `SOURCE = "playdeadlock_forum"`,
  Schonung per Delay/Limit/Skip-existing, Roh-HTML plus Snapshots.
- `rust/crates/dbrain-sources/src/forum.rs:72` — `pull_forum` öffnet Pool,
  `SourceStore`, `begin_run`, `complete_run`.
- `rust/crates/dbrain-sources/src/forum.rs:164` — `fetch_and_store_thread`
  schreibt raw, `source_documents`, Thread- und Post-Snapshots.
- `rust/crates/dbrain-sources/src/forum.rs:520` — Skip über
  `source_documents (source, external_id)`.
- `rust/crates/dbrain-normalize/src/forum_claims.rs:46` — Claims aus
  `entity_snapshots` wo `source` + `entity_type='forum_post'`, Insert in
  `brain.forum_claims` mit `ON CONFLICT (claim_hash) DO NOTHING`.
- `rust/crates/dbrain-normalize/src/forum_claims.rs:90` — `storage_policy`
  historical_only, may_override_current_game_data=false,
  default_retrieval excluded_until_explicitly_requested.

## Bestehende Abstraktionen (werden wiederverwendet, nicht nachgebaut)

- `rust/crates/dbrain-sources/src/store.rs:22` — `SourceStore`
- `rust/crates/dbrain-sources/src/store.rs:13` — `EntitySnapshotInput`
- `rust/crates/deadlock-brain-core/src/http.rs:71` — `HttpClient`
- `rust/crates/deadlock-brain-core/src/config.rs:9` — Default-User-Agent
- `rust/crates/dbrain-sources/src/lib.rs:28` — Re-Export-Muster `pull_forum`
- `rust/crates/dbrain-normalize/src/lib.rs:35` — Re-Export `parse_forum_claims`
- `rust/crates/deadlock-brain/src/main.rs:627` — CLI `PullCommands::Forum`
- `rust/crates/deadlock-brain/src/main.rs:1813` — Wiring `dbrain_sources::pull_forum`
- `rust/crates/deadlock-brain/src/main.rs:1903` — Wiring `parse_forum_claims`

## Relevante Tests (laufen vorher, laufen nachher)

- `rust/crates/dbrain-sources/src/forum.rs:623` —
  `thread_url_parser_ignores_non_threads_and_extracts_numeric_suffix`
- `rust/crates/dbrain-sources/src/forum.rs:635` —
  `parse_thread_html_extracts_title_posts_text_and_attachments`
- Analog neue Parser-Tests an Fixtures (Listing-JSON, Thread-JSON, RSS-Fallback),
  ohne Live-Reddit und ohne Live-PG.

## Öffentliche Schnittstellen und Verträge (dürfen nicht brechen)

- `README.md:57` — Forum-Import-Doku als Ton-Vorlage
- `docs/ARCHITECTURE.md:81` — Forum-Claims quarantined, Ground Truth hat Vorrang
- `docs/BUILD_LEARNING.md:71` — Reddit als zitierte Snippets, nicht als Fakt
- `Deadlock-Bots/.../0012_brain_knowledge_timeline.sql:219` — DDL
  `brain.forum_claims` (nicht ändern; wiederverwenden)
- CLI `pull forum` / `parse forum-claims` bleiben.

## Änderungsfläche (welche Dateien voraussichtlich angefasst werden)

- `rust/crates/dbrain-sources/src/reddit.rs` — neu, Klon von forum.rs
- `rust/crates/dbrain-sources/src/lib.rs` — Modul + Re-Export
- `rust/crates/dbrain-normalize/src/reddit_claims.rs` — neu, Klon von forum_claims.rs
- `rust/crates/dbrain-normalize/src/forum_claims.rs` — Rebuild-DELETE source-scoped
- `rust/crates/dbrain-normalize/src/lib.rs` — Re-Export
- `rust/crates/deadlock-brain/src/main.rs` — CLI
- `README.md` — Abschnitt Reddit

## Offene Architekturfrage

- keine
