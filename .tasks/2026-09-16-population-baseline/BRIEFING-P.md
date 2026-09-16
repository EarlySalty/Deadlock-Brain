# Briefing Paket P: Crate `dbrain-population`

Auftrag: `AUFTRAG.md` in diesem Ordner, Abschnitt "Was aus dem Python-Repo
übernommen wird" ist die fachliche Spec. Du bist der einzige Worker für dieses
Paket. Keine Unter-Agenten spawnen.

## Arbeitsort

- Worktree anlegen: `git -C /home/nathanael/repos/Deadlock-Brain worktree add
  /home/nathanael/.worktrees/deadlock-brain-population -b feat/population-baseline main`
- Nur dort arbeiten. Fremde Worktrees unter `~/.worktrees/deadlock-brain-*` und
  `~/repos/wt/*` nicht anfassen, den Haupt-Checkout nicht auschecken.
- Referenzcode lesen (nicht kopieren, nicht ausführen): `/tmp/dbm-eval/src/deadlock/`
  (`api.py` Ratenlimits, `ingest.py` Pagination und `BASE_PARAMS`, `features.py`
  Bereinigung, `dataset.py` Spalten und Abdeckung, `economy.py`, `evaluate.py`
  Staple-Gate und Ordnungsmetriken, `imbue.py`, `abilityorder.py`), dazu
  `/tmp/dbm-eval/README.md`, `CONTEXT.md`, `docs/game-mechanics.md`.
- Bestehende Bausteine wiederverwenden: `deadlock-brain-core` (`HttpClient`,
  `pg.rs` mit `DEADLOCK_CENTRAL_DSN`), `dbrain-sources/src/deadlock_api.rs`
  (Aufbau der Metadaten-URL, `get_deadlock_api_json`, `safe_ids`), Item-Katalog
  und Helden-Katalog aus `brain.item_catalog` und `brain.hero_catalog` (Tier,
  Kosten, Shop-Tab, Imbue-Flag; Spalten am echten Schema verifizieren, lesend).
  Graphify zuerst (`graphify query`), dann grep.

## Was zu bauen ist

1. **Ingest** `population sync --matches N [--hero <id|name>] [--since <ts>]`:
   Bulk-Endpunkt `GET /v1/matches/metadata` paginiert (neueste zuerst, Parameter
   wie in `ingest.py` inklusive `include_player_items`, `include_player_info`,
   `include_player_stats`, `include_objectives`, `include_mid_boss` falls die API
   ihn kennt), gedrosselt nach den in `api.py` gemessenen Grenzen, optionaler
   Schlüssel aus Env `DEADLOCK_API_KEY` (nie loggen). Population: Ranked und
   Normal. Idempotent über `match_id` (ON CONFLICT DO NOTHING). Keine Rohseiten
   in `source_documents`/`entity_snapshots`, keine Dateien auf der Platte.
2. **Bereinigung** beim Schreiben (Regeln aus AUFTRAG.md): Skillpunkte von
   Käufen trennen, nach Zeit sortieren, Tier 5 raus, `i64`-IDs, `won` aus
   `winning_team`, `net_worth_at_buy` nur als Rang (Quintil), Verkaufszeit je
   Kauf, Imbue-Ziel je Kauf (Feld am Item-Eintrag; Name am echten Payload prüfen).
   Jede Regel mit einem Test an einem kleinen eingefrorenen Payload-Ausschnitt
   (Testdaten unter `testdata/`, anonymisierte Account-IDs).
3. **Speicherung**, neue Migration `scripts/migrations/2026-09-16-population.sql`,
   Schema `brain`:
   - `population_player_matches`: `match_id`, `account_id`, `hero_id`, `team`,
     `won`, `average_badge`, `duration_s`, `start_time`, `items i64[]` in
     Kaufreihenfolge, `buy_times_s int4[]`, `sold_times_s int4[]` (NULL-fähig),
     `net_worth_rank int2[]`, `imbue_targets i64[]` (0 = keins),
     `ability_points i64[]`, `ability_times_s int4[]`, `fetched_at`.
     Primärschlüssel `(match_id, account_id)`.
   - `population_sync_runs`: Lauf, Fenster, Matches, Spieler-Matches, Dauer.
   - Aggregat-Tabellen aus Punkt 4, je `(hero_id, bucket)`.
4. **Aggregate** `population stats [--hero X] [--rebuild]`: je Held und Bucket
   (`all`, `weapon`, `spirit`, `vitality`; Neigung = Shop-Tab mit dem größten
   Seelenanteil der Käufe, Bucket nur ab 25 % der Spieler des Helden): Spieler
   (gewichtet und roh), je Item Kaufanteil, Median-Position, Median-Kaufzeit,
   Verkaufsrate, Staple-Flag ab 0,70; Bigram nächster Kauf; Imbue-Ziel-Modus je
   imbuebarem Item mit Zählung und Flags `split`/`thin`; Skillpunkt-Reihenfolge
   (Modus über die ersten 16 Buchungen, wie viele Spieler ihr genau folgen).
   Gewichtung: Rang-Gewicht wie in Python (Zentrum 80, Formel aus `dataset.py`
   oder `evaluate.py` übernehmen und im Code benennen), Sieg als zweites
   Gewicht; Rohzahlen immer daneben.
5. **CLI** `population show --hero Warden [--bucket weapon]`: lesbare Tabelle
   (Staples zuerst, dann nach Kaufanteil), Imbue-Ziele, Skill-Reihenfolge.
   Neuer Top-Level-Befehl in `deadlock-brain/src/main.rs`, nichts unter `Reason`
   ändern.
6. **Bibliotheks-API** für Paket M: `PopulationIndex::load(pool, hero_id)` mit
   `staples(bucket)`, `prevalence(item_id)`, `median_position(item_id)`,
   `imbue_target(item_id)`, `ability_order()`, `next_after(item_id)`; plus reine
   Funktionen `kendall_tau(build_order, population_positions)` und
   `jaccard_at(k, build_items, player_items)` mit Tests.

## Datenbank für Entwicklung

Zentrale DB nur lesend (Katalog). Für Migration, Ingest und Aggregate eine
lokale Wegwerf-DB über Unix-Socket anlegen (`createdb population_dev`, Muster
in `.tasks/2026-09-12-build-reasoner/FERTIG-E.md`), DSN nur in der eigenen
Shell, nie in Dateien. Der echte Lauf für den Nachweis: `population sync
--matches 10000` gegen die Wegwerf-DB, danach `stats` und `show --hero Warden`.
Ausgabe des Warden-Show-Laufs als `P-WARDEN-POPULATION.md` in diesen Ordner,
mit Laufzeit, Matchzahl, Spieler-Matches je Bucket.

## Regeln

- Kein Python, keine Code-Kommentare, echte Umlaute, keine Gedankenstriche.
- `export PATH=/home/nathanael/.cargo/bin:$PATH`; im Verzeichnis `rust/`:
  `cargo fmt` nur auf eigene Dateien, `cargo clippy -p dbrain-population
  -p deadlock-brain --all-targets -- -D warnings`, `cargo test -p dbrain-population`,
  Workspace-Tests ohne DSN (Baseline 273 bestanden, 58 ignoriert, nichts davon
  brechen). Kein `--release`. `set -o pipefail` bei Pipes hinter cargo.
- Modellnamen, Secrets, DSNs nie in Code oder Doku. Keine Referenz-Itemnamen
  und keine Warden-Sondergewichte im Produktcode.
- Selbstprüfung vor der Fertigmeldung: Diff nochmal lesen, jede Spec-Zeile
  gegen den Code abhaken.
- Nur `feat/population-baseline` committen (kleine Commits, deutsche
  Betreffzeilen wie im Repo üblich, Trailer `Co-Authored-By: Claude Opus 4.8
  <noreply@anthropic.com>`) und mit `git push -u origin feat/population-baseline`
  pushen. Nie main.
- Fertigmeldung als `FERTIG-P.md` in diesem Ordner: Commits, geänderte Dateien,
  Testzahlen vorher/nachher, Laufzeit und Zahlen des 10k-Ingests, offene Punkte.
  Wenn etwas nicht baubar ist (API-Feld fehlt, Ratenlimit unhaltbar): als
  `ABWEICHUNG:` in FERTIG-P.md melden und den Rest fertig bauen.
