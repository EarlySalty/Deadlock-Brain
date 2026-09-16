# N-AUSREISSER: Warum der 04:34-Warden-Build schlecht war

Stand 2026-09-16, Branch `feat/reasoner-planner-produktiv`. Untersucht: der
Release-Lauf 04:34:58 (Binary 57d3a07a, main 7277ff5) gab für Warden einen
schlechten Kern (Extra Health, Rapid Rounds, Swift Striker, Fleetfoot, Duration
Extender, Restorative Shot, Enduring Speed, ...), Läufe ab 05:06 mit demselben
Binary das gute 6/9-Build.

## Urteil

Kein Rennen und keine Nichtdeterminismus-Quelle. Der Ausreißer sind **stale
Aggregate**: Der 10k-`population sync` (Lauf 1, fertig 04:31) hat nur die
Rohzeilen `brain.population_player_matches` geladen, aber die Aggregat-Tabellen
`population_item_stats` und `population_hero_buckets` nicht neu berechnet. Der
Build-Pfad liest ausschließlich die Aggregate (`load_population_prior` ->
`PopulationIndex::load`), nie die Rohzeilen. Zwischen dem Roh-Ingest (04:31) und
dem `population stats`-Neuaufbau des Timers (04:46) las der 04:34-Build also
Aggregate aus einem älteren, staple-armen Stand und lieferte das mechanik-reine
Build ohne Populations-Staples. Ab 04:46 stehen die 10 Warden-Staples, seitdem
6/9.

## Belege

- Zeitachse (lokal CEST): `population_sync_runs` Lauf 1 `requested 10000`
  (`matches_seen 10000`, `player_matches_inserted 118572`, `duration_ms 274461`)
  endete 04:31:25; Lauf 2 `requested 2000` (Timer) endete 04:45:57. Der aktuelle
  `updated_at` aller Aggregat-Zeilen für Held 25 ist 04:46:01 = der
  Timer-`population stats`, nicht der 10k-Sync.
- Aktueller Aggregat-Stand Held 25: Buckets `all`/`weapon`/`vitality`, 10
  Staples im `all`-Bucket, `max(prevalence_raw)=0.969`. Passt zum guten
  05:06-Build.
- Der zuletzt persistierte Warden-Build in `brain.reasoner_builds` (Confidence
  `Low`, Kern Extra Health, Rapid Rounds, Swift Striker, Fleetfoot, Duration
  Extender, ...) ist exakt das Leerer-Prior-Build. Der Bad-Build ist also der
  Build ohne wirksame Populations-Staples, nicht ein zufällig anderes Ergebnis.

## Ausgeschlossen

- **Torn Read / Rennen:** `aggregate::write_hero` löscht und schreibt je Held in
  einer einzigen Transaktion mit einem Commit (`aggregate.rs`); ein paralleler
  Build sieht per MVCC entweder den alten oder den neuen committeten Stand,
  nie einen halbleeren. Der geforderte atomare Je-Held-Neuaufbau ist bereits so
  umgesetzt, kein Code-Fix nötig.
- **Planner-Nichtdeterminismus:** Drei Läufe `reason build Warden --no-ai
  --no-persist` gegen die zentrale DB liefern zeichengleiche Kerne. Die
  Bucket-Wahl im Build-Pfad ist fest `BUCKET_ALL` (`load_population_prior`),
  keine bucket-abhängige Streuung. Der Planner ist bei festen Aggregaten
  deterministisch.
- **Doppelzählung:** `SELECT count(*) FROM brain.population_item_stats WHERE
  prevalence_raw>1` = 0; die in Nit (b) gefundene Doppelzählung schlägt im
  Live-Bestand nicht durch (bereinigte Daten ohne doppelte item_ids je Zeile),
  ist aber als latenter Defekt gefixt.

## Folge und Empfehlung

`population sync` darf die Aggregate nicht stale zurücklassen. Der Timer macht es
richtig (sync, dann stats). Der manuelle 10k-Ingest hat `population stats`
ausgelassen bzw. vor dem 04:34-Build noch nicht laufen lassen. Betrieblich gilt:
nach jedem Sync immer `population stats` laufen lassen, bevor Builds vertraut
werden. Nit (c) (started_at beim Laufbeginn) macht genau diese Sync-Fenster im
`population_sync_runs`-Log künftig sichtbar; vorher verdeckte
`started_at = finished_at` das 4,5-Minuten-Ingest-Fenster, das die Diagnose
erschwert hat.
