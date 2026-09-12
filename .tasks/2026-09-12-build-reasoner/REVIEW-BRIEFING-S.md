# Review-Briefing: build-reasoner (Paket S, Runde 1)

[Orchestrator] Review Runde 1 für Paket S (Steam-Bot, Autoren-Scan). Lesend,
kein Code, kein Branch. Du bist der einzige Thread für dieses Review. Keine
Unter-Threads oder Unter-Agenten spawnen.

- Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-scan` (Branch
  `feat/autoren-scan-reaktivieren`, Commit f82c21c, Basis 796bb3c =
  origin/main des Steam-Bots)
- Diff: `git -C /home/nathanael/.worktrees/steam-bot-autoren-scan diff 796bb3c..HEAD`
- Auftrag: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/BRIEFING-S.md`,
  Fertigmeldung `FERTIG-S.md` im selben Ordner (Ursache, Fix, Tests,
  SQL-Vorschlag, Live-Proof)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`

## Was du prüfst

1. Ursache und Fix: stimmt der Befund (Scheduler plante nur
   `MAINTAIN_BUILD_CATALOG`, Discovery-Handler waren No-ops seit 211a6474)?
   Erreicht der Fix `run_discovery` jetzt wirklich im Regelbetrieb
   (Scheduler in `steam-core/src/main.rs`, Dedup in
   `steam-persistence/src/builds.rs`), und läuft er nicht doppelt pro Tag?
2. GC-Last: bleibt der Scan seriell mit dem bestehenden Abstand, keine
   neue Poll-Schleife, keine Retry-Stürme bei `k_eTooBusy` oder Code 5;
   Verhalten des GC-Watchdogs (`supervisor.rs`) durch den Scan nicht
   verändert. Ein Fehler bei einem Autor darf die anderen nicht abbrechen,
   muss aber sichtbar sein (Rückgabe, Log, `last_checked_status`).
3. Persistenz: Upsert in `tierlist.hero_build_sources` mit `details`
   (`modCategories`, `abilityOrder`), `version`, `published_at`,
   `last_updated_at`, `last_seen_at`; keine Migration nötig, keine Spalten
   geraten (gegen das echte Schema prüfen, DB-Zugang wie im Repo, Secrets aus
   Infisical, nie ausgeben, nur lesend). Werden `watched_build_authors.
   last_checked_*` nach dem Lauf gesetzt?
4. Referenzfall: der Build "LIGHTBRINGERxSITUATION WARDEN BUILD" hat laut
   Fertigmeldung die GC-Build-ID 779996 mit GC-Autor 1650097169 (juice), unter
   Lightbringer (13446690) existiert derselbe Name als 782057. Prüfe, ob der
   Scan über die beobachteten Autoren beide Fassungen findet oder ob der
   Kollab-Autor zusätzlich in die SQL-Empfehlung gehört. Die IDs sind über
   die Deadlock-API ermittelt; verifiziere sie lesend.
5. Scope und Hygiene: nur Katalog- und Persistenzpfad angefasst, keine
   Lobby-, Rank-, Invite-, Publish-Änderungen; `.sqlx`-Cache-Dateien passen
   zu den Queries; keine Code-Kommentare, keine Secrets. Tests selbst laufen
   lassen (Offline: `SQLX_OFFLINE=true cargo test -p steam-core -p
   steam-persistence --manifest-path rust/Cargo.toml`, Worker: 174 bestanden;
   Docker-Katalogtests über `rust/scripts/central_test_db.sh` mit
   `--features testing -- task::handlers::builds::catalog::tests
   --include-ignored`, Worker: 18 bestanden) und Zahlen nennen. Der Worker
   nennt vorbestehende Clippy-Warnungen in `gc_health.rs:45` und
   `scrim_lobby.rs:852`; bestätige, dass sie nicht aus diesem Diff stammen.
6. Deploy-Risiko: was passiert beim ersten Lauf nach dem Neustart mit den 10
   bestehenden Autoren (Volumen, Dauer, GC-Anfragen)?

## Ergebnis

Datei `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-S.md`:
vollständige Mängelliste, je Mangel Nummer, Datei:Zeile, Befund, Schwere
(blockierend, wichtig, nit), Vorschlag. Am Ende ein Urteil: FREIGABE oder
NACHBESSERN, plus die Deploy-Reihenfolge (Merge, Build, Neustart, SQL,
Live-Proof). Fertigmeldung in diesem Thread mit dem Pfad und dem Urteil.
Deutsch, echte Umlaute, keine Gedankenstriche.
