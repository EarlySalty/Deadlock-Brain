# Deadlock-Brain: Voll-Cutover SQLite → zentrale Postgres (2026-07-04)

Status: Design freigegeben (Owner). Umsetzung phasenweise, native Agents, jede Phase von Claude reviewt.

## Ziel

Das Brain läuft **100 % auf der zentralen Postgres**. `rusqlite` und die Datei
`data/deadlock_brain.sqlite3` werden **vollständig entfernt** (Daten *und* Code). Kein
Sonderweg: Zugriff über **sqlx** (async), wie die übrigen zentralen-PG-Dienste
(Steam/Twitch/Web). Das `brain.*`-Schema gehört `Deadlock-Bots/dl-central-db` und wird per
**sqlx-Migrationen** dort verwaltet.

**Nicht-Ziel:** Fachlogik ändern. Der Cutover ist ein reiner Speicher-/Zugriffs-Umbau;
Parse-/Normalize-/Enrich-/Retrieval-Semantik bleibt identisch (verifiziert per Parität).

## Entschiedene Weichen

- **Voll-Cutover** (nicht nur „Datei weg"): `rusqlite` raus aus allen Crates.
- **sqlx (async)** statt sync-`postgres`. Grund: Ökosystem-Konsistenz, compile-geprüfte Queries
  sichern die 9-Crate-SQL-Migration ab, Schema zentral über dl-central-db.
- **Reconcile:** Der aktuelle **Live-SQLite-Stand ist die Wahrheit**. Der stale PG-Seed
  (2026-07-02: 25.689 patch_events, `knowledge_events`) wird von der Migration **überschrieben**.
  Begründung: Die Divergenz ist rein *abgeleitet* — jede Erzeuger-Stufe (`parse patchnotes`,
  `normalize entities`, `enrich patch-events/legacy`) baut per `--rebuild` deterministisch neu,
  und `knowledge_events` ist nur aus `patch_events` materialisiert. Kein Quellverlust.
- **Roh-Blobs** bleiben auf Disk (`data/raw/`, `data/youtube_transcripts/`); nur die DB-Tabellen
  (`source_documents`-Metadaten, `entity_snapshots`, alle abgeleiteten) ziehen nach PG.

## Ausgangslage (verifiziert 2026-07-04)

- Brain-Laufzeit öffnet für alle Befehle SQLite (`deadlock-brain/src/main.rs:860`,
  `deadlock-brain-core/src/db.rs::open_connection`). Einziger PG-Writer: `pg_patchnotes.rs`
  (`pg import-patchnote`, ein Patch), Branch `feat/pg-patchnote-direct-import`, nicht auf main.
- `rusqlite` in **9/9** funktionalen Crates. Zentraler Punkt: `deadlock-brain-core/{db.rs,schema.rs}`;
  alle Befehle reichen `&Connection` durch. Retrieval (`dbrain-retrieval/src/lib.rs`, >1400 Zeilen)
  baut jede Query auf `&Connection`.
- PG `brain.*` (Schema aus `dl-central-db/migrations/0012_brain_knowledge_timeline.sql` +
  `0013_*`) ist ein **einmaliger, stale Hand-Seed**; kein Job hält es synchron; **kein Live-Leser**.
- Kein automatischer SQLite→PG-Mirror. Der 5-Min-Timer
  (`~/.local/bin/deadlock-brain-patchnotes-sync.sh`) schreibt nur SQLite.

## Phasenplan (Strangler; Binary erst nach Phase 3 wieder komplett → Deploy als ein Flip)

### Phase 0 — Schema-Abgleich
Brain-SQLite-Schema (`deadlock-brain-core/src/schema.rs`) tabellen-/spaltenweise gegen das
`brain.*`-PG-Schema (`dl-central-db/migrations`) mappen. Lücken beidseitig auflisten (SQLite-Tabellen
ohne PG-Pendant; PG-Zusatztabellen). Fehlende per **dl-central-db-Migration** ergänzen.
**DoD:** vollständige Tabellen-/Spalten-Mapping-Matrix + Migrationsbedarf dokumentiert.

### Phase 1 — Daten-Migration (einmalig, deterministisch)
Rust/sqlx One-Shot-Tool: kopiert jede Brain-SQLite-Tabelle → `brain.*` in PG (überschreibt Seed,
idempotent/truncate-then-load in TX). Typ-Mapping (SQLite INTEGER-Sekunden → TIMESTAMPTZ, TEXT-JSON
→ JSONB). **SQLite-Backup vorher.** Danach Parität Zahl-gegen-Zahl je Tabelle.
**DoD:** PG `brain.*` == aktueller Live-SQLite-Stand (Counts + Stichproben-Diff), Backup gesichert.

### Phase 2 — DB-Kern auf sqlx
`deadlock-brain-core/src/db.rs` von rusqlite auf einen **sqlx `PgPool`** umstellen; eine gemeinsame
async DB-Abstraktion, die alle Crates konsumieren. `schema.rs` (SQLite-DDL) entfällt (Schema lebt in
dl-central-db). Async-Fundament (`tokio`) etablieren.
**DoD:** core baut auf sqlx, PgPool aus `DEADLOCK_CENTRAL_DSN`, Tests grün.

### Phase 3 — 9 Crates cutover
`dbrain-sources`, `dbrain-normalize`, `dbrain-enrich`, `dbrain-learn`, `dbrain-builds`,
`dbrain-retrieval`, `deadlock-brain-yt`: jede `&Connection`-Query auf sqlx/PG. SQLite-Dialekt → PG:
`?` → `$1`, `INSERT OR IGNORE` → `ON CONFLICT DO NOTHING`, `INTEGER`-Zeit → `TIMESTAMPTZ`,
`json_extract` → `->>`, Tabellen mit `brain.`-Schema-Präfix. Pro Crate: umbauen → Tests grün → weiter.
**DoD je Crate:** rusqlite entfernt, sqlx-Queries compile-geprüft, Crate-Tests grün.

### Phase 4 — CLI + Timer + Consumer
`deadlock-brain/src/main.rs` async, alle Befehle auf PgPool (der `pg`-Dispatch verschmilzt mit dem
Normalpfad). `~/.local/bin/deadlock-brain-patchnotes-sync.sh` auf das PG-Binary (kein `--db`-SQLite-Pfad
mehr). `!brain`-Consumer (`Deadlock-Bots/crates/dl-brain`) prüfen/auf PG-Read umstellen, falls er heute
indirekt SQLite nutzt.
**DoD:** `deadlock-brain status/context/review/build` laufen gegen PG; Timer-Wrapper PG-only.

### Phase 5 — Cutover-Deploy + Parität
Live-Parität: für eine Stichprobe (`context`/`review`/`build`/`timeline` mehrerer Entities) alte
SQLite-Antworten vs neue PG-Antworten diffen (JSON-Gleichheit). Dann Service/Timer auf PG-Binary
flippen, Burn-in, Live-Beweis (Timer-Lauf gegen PG, `!brain` antwortet aus PG). SQLite-Backup bleibt.
**DoD:** Parität grün, Live-Betrieb auf PG bewiesen (Journal/Query).

### Phase 6 — SQLite entfernen
`rusqlite` aus allen `Cargo.toml`, `schema.rs`/`db.rs`-SQLite-Reste, `data/deadlock_brain.sqlite3`
+ `data/backups/*.sqlite3` (nach Burn-in) löschen. `--db`-CLI-Arg entfernen.
**DoD:** `grep -r rusqlite` leer, keine SQLite-Datei, Brain läuft rein auf PG.

## Risiken & Sicherungen

- **Live-System:** 5-Min-Timer + `!brain` laufen. Umbau komplett auf Branch; Deploy erst nach Phase 5
  als *ein* getesteter Flip mit SQLite-Backup + Rollback (Binary+Timer zurück auf SQLite-Stand).
- **Schema-Drift PG↔SQLite:** in Phase 0 explizit gemappt; Migration (Phase 1) macht PG deckungsgleich.
- **Async-Umbau:** größter mechanischer Posten (Phase 2–4); durch compile-geprüfte sqlx-Queries + Tests
  pro Crate abgesichert.
- **Fremde uncommittete Arbeit** im Repo (`dbrain-retrieval/lib.rs`, `build_narration.rs` = Build-Narration,
  nicht Teil dieses Cutovers): NICHT anfassen, vor Merge klären.

## Ausführung & Verifikation

- Native Agents, Phase für Phase; Claude reviewt jede `changed_files`, schreibt finale Doku/Texte selbst.
- Verifizierbares Signal je Phase (Counts, Tests, Live-Diff) statt Kritiker-Urteil.
- Emoji-Feature (`Patchnotes-Bot/feature/brain-emoji-integration`) liest bereits `brain.*` aus PG →
  unabhängig, profitiert nach dem Cutover automatisch von frischen Daten.
