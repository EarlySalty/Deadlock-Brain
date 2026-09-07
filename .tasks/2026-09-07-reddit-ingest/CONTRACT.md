# Contract: Reddit-Ingest für Deadlock-Brain

status: aktiv
datum: 2026-09-07
klasse: hoch
repo: Deadlock-Brain

Dieser Contract ist der Maßstab für Implementierung und Merge-Kritiker. Nach dem
Anlegen ist er unveränderlich: der Hook lässt nur noch die `status:`-Zeile und
Anhänge unter `## Amendments` zu. Wer ein REQ oder INV ändern will, schreibt ein
Amendment mit Begründung; Produkt-, API- oder Datenänderungen entscheidet der User.

## Ziel

Öffentliche Reddit-Threads aus `r/Deadlock` (und per Flag weitere Subs) landen
automatisch als Rohdokumente und quarantined Claims im Brain, analog zum
Playdeadlock-Forum, ohne dass jemand auf Reddit schreibt.

## Anforderungen (user-sichtbares Verhalten)

- REQ-01: `deadlock-brain pull reddit` zieht öffentliche Posts und sichtbare
  Kommentare aus Default-Subreddit `Deadlock`, speichert Rohdaten unter
  `data/raw/reddit/` und schreibt `brain.source_documents` plus
  `brain.entity_snapshots` (`entity_type` `reddit_thread` / `reddit_post`).
- REQ-02: Bereits gespeicherte Thread-IDs werden standardmäßig übersprungen;
  `--refresh-existing` holt sie erneut. `--limit` (Default 25, 0 = ohne Limit)
  und `--delay-seconds` (Default 2.0) steuern Wellen und Abstand.
- REQ-03: `deadlock-brain parse reddit-claims` baut aus den Snapshots Claims in
  `brain.forum_claims`. Jeder Claim trägt `ingest_source=reddit` in `metadata`,
  einen konkreten Permalink in `source_url` / `source_references`,
  `currentness=historical_quarantine` und darf aktuelle API-/Patch-/Sheet-Daten
  nicht überschreiben.
- REQ-04: `--rebuild` bei Reddit-Claims löscht nur Reddit-Zeilen
  (`metadata->>'ingest_source' = 'reddit'`). Forum-Claims bleiben.
- REQ-05: 403/429/Netzfehler sind fail-soft: betroffener Thread wird
  übersprungen und in der Summary gezählt, der Lauf bricht nicht die übrigen
  ab. Kein Captcha-Bypass, kein Headless-Browser, kein Login.
- REQ-06: README bekommt denselben Abschnittston wie „Forum schonend
  importieren“: Befehle, Schonung, Claims sind quarantined.

## Invarianten (darf sich nicht ändern)

- INV-01: Ingest nur über `SourceStore` (`begin_run` / `write_raw` /
  `upsert_source_document` / `insert_many_snapshots` / `complete_run`).
- INV-02: HTTP nur über `deadlock_brain_core::http::HttpClient` mit bestehendem
  Default-User-Agent `DeadlockBrain/0.1 contact=admin@earlysalty.com`.
- INV-03: Kein Posten, Voten, Kommentieren, Messaging, kein OAuth, kein Secret.
- INV-04: Forum-Pull, Forum-Claims-Parser und bestehende Tests bleiben
  verhaltensgleich, außer dass `parse forum-claims --rebuild` nur noch Forum-Zeilen
  löscht (`ingest_source` fehlt oder gleich `playdeadlock_forum`).
- INV-05: Keine Migration in Deadlock-Bots, keine neue Tabelle, keine Änderung
  an `brain.forum_claims`-DDL. Claims nutzen die vorhandene Tabelle.
- INV-06: Bestehende Tests nicht löschen oder abschwächen. Keine Code-Kommentare.

## Nicht-Ziele

- Schreiben auf Reddit (Bot-Account, getarnte Fragen, Auto-Replies).
- OAuth / Reddit-App / Infisical-Secret.
- Pushshift, Arctic Shift, inoffizielle Archive, Browser-Automation, Captcha.
- Trust-Engine, Retrieval-Einbindung der Claims in `ask`/`review` (bleiben
  excluded_until_explicitly_requested wie Forum).
- YouTube-Pipeline, Wiki-Crawler, `pull all`.
- Neue systemd-Unit / Cron in diesem Slice (CLI reicht, Doku nennt das Kommando).

## Erlaubter Änderungsbereich

- `rust/crates/dbrain-sources/src/` (neues `reddit.rs`, `lib.rs`)
- `rust/crates/dbrain-normalize/src/` (`reddit_claims.rs`, `forum_claims.rs` nur
  Rebuild-DELETE source-scoped, `lib.rs`)
- `rust/crates/deadlock-brain/src/main.rs` (CLI `pull reddit`, `parse reddit-claims`)
- `README.md`
- `.tasks/2026-09-07-reddit-ingest/`
- `rust/.sqlx/` nur falls ein neues `query!` zwingend nötig ist (vermeiden:
  `sqlx::query` wie in `forum_claims.rs`)

## Verbotene Änderungen

- Deadlock-Bots / `dl-central-db/migrations`
- `brain.forum_claims` Schema, andere Brain-Tabellen
- YouTube-Crate, Wiki-Pull, Fireworks-Client, Infisical, Config-ENV für Flags
- Lint-/CI-Config, `Cargo.toml` Workspace-Abhängigkeiten außer Nutzung bereits
  vorhandener Crates (`serde_json`, `sqlx`, `roxmltree` falls RSS)
- Fremde Repos, Deploy, Merge nach main

## Offene Produktfragen

- keine (User 2026-09-07: Reddit-Ingest nur lesen, analog Forum, bauen)

## Amendments
