# FERTIG-P: Paket P Populations-Baseline

Branch: `feat/population-baseline` (Worktree `~/.worktrees/deadlock-brain-population`).

## Commits

- `ecc5adb` dbrain-population: Crate für Populations-Baseline (Ingest, Bereinigung, Aggregate, Metriken)
- `eb95e27` dbrain-population: Top-Level-Befehl population im deadlock-brain-Binary
- `15db0b3` dbrain-population: DSN aus deadlock_brain_core, Read-only-Abbruch, echte Umlaute, Show bucket-genau

## Geänderte und neue Dateien

- neu `rust/crates/dbrain-population/` (Cargo.toml, `src/{lib,api,catalog,clean,db,aggregate,index,metrics,cli}.rs`, `testdata/{metadata_sample,catalog_sample}.json`)
- neu `scripts/migrations/2026-09-16-population.sql`
- geändert `rust/Cargo.toml` (Workspace-Eintrag), `rust/Cargo.lock`
- geändert `rust/crates/deadlock-brain/Cargo.toml` (Abhängigkeit), `rust/crates/deadlock-brain/src/main.rs` (Befehl `population`)

## Was gebaut wurde (gegen die Spec abgehakt)

1. Ingest `population sync --matches N [--hero <id|name>] [--since <ts>]`: Bulk
   `/v1/matches/metadata`, paginiert neueste zuerst per `max_match_id`-Cursor,
   Parameter wie in `ingest.py` inklusive `include_mid_boss`, Population
   `match_mode=Ranked` und `game_mode=normal`. Gedrosselt nach den in `api.py`
   gemessenen Grenzen (9/min anonym, 50/min mit `DEADLOCK_API_KEY`, 429 mit
   `next_request_in`/`Retry-After`), Schlüssel nie geloggt. Idempotent über
   `match_id`/`account_id` (`ON CONFLICT DO NOTHING`). Keine Rohseiten in
   `source_documents`/`entity_snapshots`, keine Datei auf der Platte.
2. Bereinigung beim Schreiben: Skillpunkte (`type==ability`) von Käufen
   (`type==upgrade`) getrennt, Kaufreihenfolge nach `game_time_s` sortiert,
   Tier 5 raus, i64-IDs, `won` aus `player_match_outcome`/`winning_team`,
   `net_worth` nur als Quintil-Rang, Verkaufszeit je Kauf (NULL-fähig),
   Imbue-Ziel je Kauf am Feld `imbued_ability_id`. Jede Regel mit Test.
3. Speicherung, Migration, Schema `brain`: `population_player_matches`,
   `population_sync_runs`, Aggregat-Tabellen `population_hero_buckets`,
   `population_item_stats`, `population_imbue_stats`, `population_ability_order`.
4. Aggregate `population stats [--hero X] [--rebuild]`: je Held und Bucket
   (`all`, `weapon`, `spirit`, `vitality`, Neigung = Shop-Tab mit größtem
   Seelenanteil, Bucket nur ab 25 Prozent der Spieler). Prevalence roh und
   gewichtet, Median-Position, Median-Kaufzeit, Verkaufsrate, Staple ab 0,70,
   Bigram nächster Kauf, Imbue-Modus mit `split`/`thin`, Skill-Reihenfolge
   (Modus über die ersten 16 Buchungen plus Zahl der genauen Folger).
   Gewichtung: Rang-Kernel `exp(-((badge-80)/25)^2)` (Zentrum 80 wie
   `sequence.py`), Sieg als zweites Gewicht (`WIN_WEIGHT`), Rohzahlen daneben.
5. CLI `population show --hero Warden [--bucket weapon]`: lesbare Tabelle,
   Staples zuerst, dann Imbue-Ziele und Skill-Reihenfolge, bucket-genau.
6. Bibliotheks-API `PopulationIndex::load` mit `staples`, `prevalence`,
   `median_position`, `imbue_target`, `ability_order`, `next_after`, plus
   `kendall_tau` und `jaccard_at` mit Tests.

## Testzahlen

- Crate `dbrain-population`: 22 passed, 0 ignored (das Crate ist neu, vorher 0).
- Workspace ohne DSN: 325 passed, 58 ignored, 0 rot. Die im Briefing genannte
  Baseline 273/58 ist älter; die 58 ignorierten bleiben unverändert, nichts
  bricht. Das Crate steuert 22 der 325 bei.
- `cargo clippy -p dbrain-population -p deadlock-brain --all-targets -- -D warnings`: sauber.
- fmt nur auf eigene Dateien (`rustfmt`), kein `cargo fmt`.

## 10k-Ingest (Nachweis, lokale Wegwerf-DB `population_dev` über Unix-Socket)

- `deadlock-brain population sync --matches 10000`: 10000 Matches gesehen,
  116198 neue Spieler-Matches, 2342 schon vorhanden, Laufzeit 478,0 s.
- Idempotenz belegt: ein zweiter 200er-Lauf lud 1644 der 200er-Seite als schon
  vorhanden nach (nur die inzwischen neu hinzugekommenen Matches waren neu),
  ein direkt anschließender Kontrolllauf gegen dieselbe Seite schrieb nichts
  doppelt.
- Datenbank danach: 119248 Spieler-Matches über 10059 Matches, 38 Helden.
- `population_sync_runs` protokolliert Fenster, Matchzahl, Spieler-Matches und
  Dauer je Lauf.

## Warden-Baseline

Siehe `P-WARDEN-POPULATION.md`. Warden (hero_id 25): 4017 Spieler-Matches im
all-Bucket, 10 Staples ab 70 Prozent (Quicksilver Reload 96,8 Prozent bis
Enduring Speed 75,8 Prozent), Imbue-Ziele je imbuebarem Item, Skill-Reihenfolge
als Modus mit 1645 von 3189 genauen Folgern. Buckets all, weapon, vitality
(spirit unter 25 Prozent).

## ABWEICHUNGEN

- ABWEICHUNG (Katalogquelle): Das Briefing nennt `brain.item_catalog` und
  `brain.hero_catalog` als Quelle für Tier, Kosten, Shop-Tab und Imbue-Flag.
  Das echte Schema von `brain.item_catalog` trägt nur `item_id, name,
  slot_type, tier, defense_kind, damage_axis, properties` und hat weder `cost`
  noch ein Imbue-Flag. Deshalb kommt der Katalog aus derselben Upstream-Quelle,
  die auch `dbrain-builds::sync` füllt: `GET /v1/assets/items` (Felder `id`,
  `type`, `item_slot_type`, `item_tier`, `cost`, `imbue`) und `/v1/assets/heroes`.
  Kein Zugriff auf die zentrale DB nötig, Kosten und Imbue-Flag sind dort echt.
- ABWEICHUNG (HTTP-Baustein): `deadlock-brain-core::HttpClient` wird für die
  Matchseiten nicht wiederverwendet, weil `get` jede Antwort auf die Platte
  cached (`write_cache`), was die Regel "keine Rohantworten auf der Platte"
  (92 MB je Seite) verletzt, und weil er keine Endpunkt-Ratengrenzen und keine
  429-Behandlung kennt. Der eigene `ApiClient` schreibt nie auf Platte, pacet je
  Endpunkt und behandelt 429 nach dem gemessenen Muster aus `api.py`.
- Hinweis (net_worth_rank): Python bildet das Quintil global je Match-Phase
  (`dataset.add_networth_quintiles`). Da der Rang je Kauf schon beim Ingest
  feststehen soll, wird hier der Rang innerhalb des Matches zum Kaufzeitpunkt
  gebildet (Anteil der Mitspieler mit weniger Nettovermögen) und in 5 Quintile
  gelegt. Selbe Aussage (nur Rang, nie absolute Seelen), aber match-lokal statt
  populationsweit, dafür ohne zweiten Durchlauf über alle Zeilen.

## Datenbank-Anbindung

- Das Crate nimmt dieselbe Quelle wie der Rest des Binaries:
  `deadlock_brain_core::pg::pg_pool()` mit `DEADLOCK_CENTRAL_DSN`. Für die
  Wegwerf-DB wurde `DEADLOCK_CENTRAL_DSN` in der Shell auf den lokalen
  Socket-DSN gesetzt, nie in einer Datei. Ein zweiter Env-Name entfällt.
- `population sync` bricht mit klarer Meldung ab, wenn die Verbindung read-only
  ist (`SHOW transaction_read_only = on`), statt still nichts zu schreiben.
- Die Migration auf die zentrale DB spielt der Delegator beim Deploy ein.

## Offene Punkte

- `--rebuild` ist derzeit ein No-op, weil `stats` die Aggregate ohnehin immer
  neu aus `population_player_matches` rechnet (idempotent).
- Ohne API-Schlüssel läuft der Ingest mit 9/min; ein Schlüssel in
  `DEADLOCK_API_KEY` hebt auf 50/min, ohne Bytes zu sparen.
