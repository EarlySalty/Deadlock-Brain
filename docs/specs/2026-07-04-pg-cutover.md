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
- **Reconcile (Owner: „alles migrieren, nichts löschen"):** Der aktuelle **Live-SQLite-Stand ist
  die Wahrheit** für die 10 gemappten + 26 fehlenden Tabellen (der stale, divergente Teil des
  PG-Seeds wird dort ersetzt). Die 3 **PG-only kuratierten/derived** Tabellen bleiben **erhalten**:
  `insight_records` (51 kuratiert) + `knowledge_events`/`current_entity_state` — vor der Migration
  dumpen, nach dem Reload rematerialisieren bzw. verbatim mit ID-Remap wieder einspielen. **Kein
  Datenverlust.**
- **Sauber anlegen (Owner):** Die Migration ist die Gelegenheit, `brain.*` PG-idiomatisch zu
  gestalten statt SQLite 1:1 zu kopieren — durchgängig `TIMESTAMPTZ`/`JSONB`/`bigint`/`double
  precision`, konsistenter `_json`-Suffix-Drop, echte `PK/FK/UNIQUE` + Indizes für die Read-Pfade,
  und SQLite-Warzen fixen (`mechanic_notes.rowid`-Spalte, `learned_builds.language INTEGER`,
  TEXT-vs-INTEGER-Zeitquellen vereinheitlichen). Kein Daten-/Semantik-Redesign — nur sauberes Schema.
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

### Phase 0 — Schema-Abgleich (ERLEDIGT 2026-07-04, native Agent)
Ergebnis (Mapping-Matrix im Agent-Report):
- SQLite hat **36 reguläre Tabellen** (+ 5 `vector_embeddings*`, brauchen `vec0` — separat, kein
  Teil dieses Cutovers). PG `brain.*` hat **13**.
- **10 Tabellen mappen 1:1** (PG ist reine Obermenge: `_json→jsonb`, INTEGER/TEXT-Zeit→TIMESTAMPTZ,
  BIGSERIAL id + `legacy_sqlite_id`/`legacy_<fk>_id` fürs FK-Remap).
- **GAP A: 26 SQLite-Tabellen fehlen komplett in `brain.*`** (Katalog hero/item, Sheet, Builds,
  Notes, YouTube-Lern-Layer inkl. 11.941 `youtube_learning_claims`) → Phase 0.5.
- **GAP B: 3 PG-only Tabellen** (`knowledge_events` 26.685 abgeleitet; `current_entity_state` 5.804
  Projektion; `insight_records` 51 **kuratiert, kein Rebuilder**) → erhalten (s. Reconcile).
- Stolpersteine: TEXT-ISO-Zeitfelder (`posted_at`, `first/last_seen_at`, `published_at`) ≠ Unix-Sek.;
  Materializer für `knowledge_events` deckt nur die patch_event-Scheibe ab (forum_claim/insight fehlen).

### Phase 0.5 — Schema-Migrationen (dl-central-db), SAUBER angelegt
Die **26 GAP-A-Tabellen** als neue `dl-central-db`-Migration(en) anlegen, PG-idiomatisch (kein
1:1-SQLite-Copy): BIGSERIAL id + `legacy_sqlite_id` (nur bei Autoincrement-PK) + `legacy_<fk>_id` je
FK; `*_json → JSONB`; alle Zeit → `TIMESTAMPTZ`; `bigint`/`double precision`; natürliche/komposite PKs
(Katalog, youtube_*) erhalten; SQLite-UNIQUE übernehmen; Cross-Schema-FK per Barrier-Muster
(`0011_barrier_orphans_and_cross_fks.sql`). Warzen fixen: `mechanic_notes.rowid`, `learned_builds.language`.
Sinnvolle Indizes für die Read-Pfade (Phase 3) gleich mitanlegen.
**DoD:** Migration(en) compilen/`sqlx migrate` grün gegen Scratch-DB; alle 36 SQLite-Tabellen haben ein
sauberes PG-Ziel; Review durch Claude vor Apply auf Live-PG.

### Phase 1 — Daten-Migration (einmalig, deterministisch)
Rust/sqlx One-Shot-Tool lädt **alle 36** SQLite-Tabellen → `brain.*`, FK-sicher (Ladereihenfolge
Tier 0→4 aus dem Phase-0-Report; Truncate = umgekehrt), in einer TX. Muster je Tabelle: erst mit
`legacy_sqlite_id` + `legacy_<fk>_id` laden, dann FK per
`UPDATE child SET fk = parent.id FROM parent WHERE parent.legacy_sqlite_id = child.legacy_<fk>_id`
auflösen. Typ-Mapping: INTEGER-Sek. → TIMESTAMPTZ, **TEXT-ISO → TIMESTAMPTZ** (eigener Parser!),
TEXT-JSON → JSONB. Natürliche/Komposit-PK-Tabellen (Katalog, youtube_*) ohne id-Remap.
**Nichts löschen:** Vor der Migration die 3 PG-only Tabellen (`insight_records`,
`knowledge_events`, `current_entity_state`) dumpen. Nach dem Reload der Basis:
`current_entity_state` + `knowledge_events` aus der neuen Basis **rematerialisieren** (patch_event-Scheibe
via portiertem `pg_patchnotes`-Materializer; forum_claim-/insight-Scheiben implementieren oder verbatim
mit ID-Remap wiederherstellen); `insight_records` **verbatim** wieder einspielen (`source_patch_event_ids`
auf neue IDs remappen). **SQLite-Backup vorher.**
**DoD:** PG `brain.*` == aktueller Live-SQLite-Stand für alle 36 Tabellen (Counts + Stichproben-Diff);
die 3 kuratierten/derived Tabellen vollständig erhalten (Counts vor==nach); Backup gesichert.

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
