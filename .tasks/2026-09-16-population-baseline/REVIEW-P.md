# Review Paket P: Populations-Baseline (Runde 1)

**Urteil: FREIGABE**

Branch `feat/population-baseline` (Spitze 42d11ec, 4 Commits ab main a57382a).
Gegen Spec (AUFTRAG.md, BRIEFING-P.md), das Python-Original unter
`/tmp/dbm-eval/src/deadlock/` und die echten Testdaten geprüft. Der Code baut,
22 Tests grün, Clippy `-D warnings` sauber (nach erzwungenem Neubau), und die
Aggregate wurden unabhängig per SQL gegen die lokale `population_dev` (119248
Spieler-Matches, 38 Helden) bestätigt. Es gibt keinen blockierenden und keinen
wichtigen Mangel. Alle drei in FERTIG-P deklarierten ABWEICHUNGEN sind belegt
und tragfähig. Die folgenden Punkte sind Nits.

## Verifikation (belegt)

- Build und Tests: `cargo test -p dbrain-population` = 22 bestanden, 0 fehlgeschlagen.
  `cargo clippy -p dbrain-population -p deadlock-brain --all-targets -- -D warnings`
  nach `touch` der Quellen ohne Warnung, Exit 0. Kein `--release`.
- Feldnamen gegen echtes Payload (`testdata/metadata_sample.json`): Item-Felder
  `game_time_s, imbued_ability_id, item_id, sold_time_s, upgrade_info`, Stats
  `net_worth, time_stamp_s`, Spieler `player_match_outcome, team, account_id,
  hero_id`, Match `winning_team, match_outcome, average_badge, duration_s,
  start_time, match_mode, game_mode`. Alle vom Code gelesenen Felder stimmen.
- SQL-Gegencheck (Warden, hero_id 25, Bucket all, 4017 Spieler): die fünf
  stärksten Staples aus `population_item_stats` (prevalence_raw 0.9676 / 0.9539
  / 0.9338 / 0.9094 / 0.9069) sind mit einer unabhängigen Neuberechnung aus
  `population_player_matches` identisch. Median-Position des Top-Items (gespeichert
  7.0) stimmt mit `percentile_cont(0.5)` über `array_position - 1` (7.00) überein.
  Deckt sich mit P-WARDEN-POPULATION (Quicksilver Reload 96,8 %).
- kendall_tau (`metrics.rs`) ist tau-b (Nenner mit Bindungen als
  `sqrt((c+d+ties_build)*(c+d+ties_pop))`), das entspricht dem Default von
  `scipy.stats.kendalltau` in `evaluate.order_distance`. jaccard_at entspricht
  Pythons `_jaccard` (Schnitt/Vereinigung der Top-k, NaN bei zwei leeren Mengen).
  Randfälle getestet.
- Konstanten gegen `sequence.py`: BADGE_CENTER 80, BADGE_HALFWIDTH 25, Kernel
  `exp(-((badge-80)/25)^2)`; STAPLE 0.70 (evaluate PREVALENCE_THRESHOLD), Imbue
  split unter 0.50 (imbue MAJORITY), thin unter 30 (state THIN_EVIDENCE); NEIGUNG
  0.25, ABILITY_ORDER_PREFIX 16. Prevalence roh = ungewichtete Käuferquote wie
  `evaluate.item_prevalence`, Median-Position = `evaluate.population_order`.
- Ingest gegen `api.py`/`ingest.py`: Ratenlimits 9/50 (matches) und 30 (assets),
  User-Agent und Header `X-API-KEY`, Base-URL, BASE_PARAMS (match_mode Ranked,
  game_mode normal, include_player_items/stats/info/objectives/mid_boss,
  order_by match_id desc), Cursor `max_match_id = lowest - 1`, Kurz-Seite bricht
  ab: alles deckungsgleich. 429 liest `Retry-After` und
  `error.quota.next_request_in` wie Pythons `_retry_after`. Idempotenz über
  `ON CONFLICT (match_id, account_id) DO NOTHING`. Read-only-Abbruch mit klarer
  Meldung (`db.rs:13`). Kein Schlüssel im Log, kein Plattencache, keine Rohseiten
  in `source_documents`/`entity_snapshots`.
- Migration: Schema `brain`, PK `(match_id, account_id)`, Index auf `hero_id`,
  Aggregat-PKs `(hero_id, bucket[, item_id|position])` decken die Lese-Abfragen.
  `IF NOT EXISTS` durchgängig. Keine GRANTs in der Datei, genau wie
  `2026-09-12-reasoner.sql`; Rechtevergabe bleibt beim Delegator (gleich gehalten).
- DSN: `deadlock_brain_core::pg::pg_pool()` mit `DEADLOCK_CENTRAL_DSN`, kein
  zweiter Env-Name. Keine Secrets, Modell- oder Anbieternamen, keine
  Code-Kommentare, keine Gedankenstriche; echte Umlaute in nutzersichtbaren
  Strings.
- net_worth-Quintil (`clean.rs:232`) entspricht Pythons
  `features.within_match_position` (`(worths < mine).sum() / (n-1)`), in 5
  Quintile gelegt.

## Mängel

1. `rust/crates/dbrain-population/src/cli.rs:180`, nit. `--rebuild` ist ein
   No-op (`let _ = args.rebuild;`); `stats` rechnet ohnehin immer neu. Erwartet:
   Flag entfernen oder es zwischen Neuberechnung und Anzeige gecachter Aggregate
   unterscheiden lassen. In FERTIG-P bereits als offener Punkt deklariert.

2. `rust/crates/dbrain-population/src/cli.rs:166-201`, nit. `population stats`
   schreibt Aggregate (`rebuild_hero` ruft `write_hero`), ruft aber kein
   `assert_writable`. Gegen eine read-only-Verbindung bricht es erst spät mit
   rohem sqlx-Fehler ab statt mit der klaren Meldung, die `sync` liefert.
   Erwartet: `assert_writable` auch vor `stats`.

3. `rust/crates/dbrain-population/src/lib.rs:32`, nit. `WIN_WEIGHT = 1.25` ist
   ein selbst gewählter Faktor; Pythons Default (`sequence.row_weights`) ist 1.0,
   Win-Gewichtung dort standardmäßig aus und nur konfigurierbar. Der AUFTRAG
   verlangt "Sieg als zweites Gewicht", eine aktive Gewichtung ist also
   spec-konform, aber der konkrete Wert 1,25 hat keine Python-Grundlage und keine
   Wirkungsmessung. Erwartet: Wahl im Bericht oder in der Doku begründen
   (Kommentare sind verboten).

4. `rust/crates/dbrain-population/src/api.rs:69-76` und `cli.rs:88-96`, nit
   (unverifiziert). `--hero` reicht `hero_ids` an `/v1/matches/metadata`; ob der
   Endpunkt diesen Parameter kennt, ist nicht belegt (10k-Lauf und
   Warden-Baseline liefen ohne `--hero`). Ignoriert der Endpunkt den Parameter,
   filtert der Lauf still nicht. Erwartet: den Hero-Filter-Pfad einmal live gegen
   die API prüfen oder als ungeprüft kennzeichnen.

5. `rust/crates/dbrain-population/src/clean.rs:64-66`, nit. `average_badge` wird
   mit `> 0` auf `None` gefiltert; ein legitimes Durchschnitts-Badge von exakt 0
   (Obscurus) liefe damit als fehlend mit neutralem Gewicht 1.0, während Python
   nur `NaN` neutral behandelt. Praktisch vernachlässigbar, weil ein Schnitt über
   zwölf Ranked-Spieler kaum exakt 0 wird.

## Deklarierte ABWEICHUNGEN (bestätigt, keine Mängel)

- Katalogquelle `/v1/assets/items` statt `brain.item_catalog`: das echte
  Schema von `brain.item_catalog` trägt weder `cost` noch ein Imbue-Flag, die
  Assets-Quelle liefert beide echt. Begründung trägt, kein zentraler DB-Zugriff
  nötig, gleiche Quelle wie `dbrain-builds::sync`.
- Eigener `ApiClient` statt `deadlock-brain-core::HttpClient`: dessen `get`
  cached jede Antwort auf die Platte (verletzt "keine Rohseiten auf der Platte",
  rund 92 MB je Seite) und kennt weder Endpunkt-Ratengrenzen noch 429. Der eigene
  Client ist die spec-konforme Wahl.
- net_worth_rank match-lokal statt phasen-global (`dataset.add_networth_quintiles`):
  deklariert; die Spalte wird in Paket P von keinem Aggregat konsumiert
  (`AggRow` lädt sie nicht), also keine Wirkung auf die Baseline. Für Paket M ist
  die abweichende Semantik zu beachten.

---

Urteil: FREIGABE
Blockierend: 0
Wichtig: 0
Nit: 5
