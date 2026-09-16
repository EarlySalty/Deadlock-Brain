# Review Paket N: Planner und Population im Produktivpfad (Rolle review_1)

NACHBESSERN

Grund der Nachbesserung ist allein die Ausreißer-Diagnose (`N-AUSREISSER.md`).
Der Code ist grün, sauber und freigabereif: kein blockierender Code-Mangel,
Tests und Clippy bestätigt (siehe Befund 7). Die Nachbesserung ist rein
diagnostisch plus zwei Nits.

## Befunde

1. `N-AUSREISSER.md` (Abschnitt Urteil und Belege), mittel. Die Erklärung "stale
   Aggregate, erst der Timer um 04:46 hat `population stats` neu berechnet"
   widerspricht der belegten Zeitleiste. Der Delegator hat um 04:31, direkt nach
   dem 10k-Sync, `population stats` laufen lassen; die Ausgabe zeigte Staples je
   Held und `population show Warden` mit 10 Staples. Damit standen die
   Warden-Aggregate mit 10 Staples bereits vor dem 04:34-Build. Die Behauptung,
   die Aggregate seien bis 04:46 leer gewesen, ist somit nicht belegt, sondern
   durch die 04:31-Ausgabe widerlegt. Der einzige harte Beleg im Dokument ("der
   zuletzt persistierte Warden-Build ist exakt das Leerer-Prior-Build") zeigt
   nur, dass der Build eine leere Population gelesen hat, nicht dass die Aggregate
   leer waren. Die plausiblere Ursache steht bereits im eigenen `N-BEFUND.md`:
   der schlechte Build "kam aus dem alten Release-Binary bzw. einem DB-Stand ohne
   Population". Ein altes Binary ohne `load_population_prior` liefert
   mechanik-rein, egal wie voll die Aggregate sind. Alternative, ebenfalls
   ungeprüfte Ursache: ein Lesefilter in `load_population_prior`
   (patch_tag, updated_at oder hero_id), der die um 04:31 geschriebenen Zeilen
   für den 04:34-Build unsichtbar gemacht hat. Der Beleg um 05:31 (`stats --hero
   Warden` plus Sofort-Build, 6/9) trennt diese Hypothesen nicht.
   Urteil: offen, nicht "erklärt". Erwartet: entweder belegen, dass die
   04:31-`stats`-Ausgabe die vom Build gelesenen Aggregate nicht geschrieben hat
   (anderes Binary, anderer Stand, Trockenlauf), oder die echte Lese- oder
   Binary-Diskrepanz benennen. Die betriebliche Empfehlung "nach jedem Sync
   `population stats`" bleibt richtig, deckt aber die eigentliche Frage nicht ab,
   warum der Build eine leere Population still gelesen hat, obwohl Aggregate da
   waren (fehlende Selbsterkennung, Ursache statt Symptom).

2. `rust/crates/dbrain-population/src/cli.rs:164-165` (`run_stats`), gering.
   `assert_writable` ist ergänzt (Nit aus REVIEW-P Punkt 2 behoben), aber
   `ensure_schema` läuft nach dem Fix nur noch in `sync`. Ein erstmaliges
   `population stats` vor je einem `sync` bricht damit an fehlenden Aggregat- oder
   Rohtabellen mit rohem sqlx-Fehler ab statt mit klarer Meldung. Operativ selten,
   weil `sync` die Tabellen anlegt und `stats` ohnehin Rohzeilen braucht; als
   Randfall notiert, kein Blocker.

3. `rust/crates/dbrain-population/src/db.rs:92-101` (`record_sync_run`), gering.
   `started_at` wird als `now() - make_interval(secs => duration_ms/1000)`
   rekonstruiert, nicht am echten Laufbeginn erfasst. Da `record_sync_run` direkt
   nach der Sync-Schleife läuft, ist die Rekonstruktion praktisch deckungsgleich;
   sie macht das Sync-Fenster im Log sichtbar (Ziel des Nits erfüllt). Caveat: es
   ist ein abgeleiteter, kein gemessener Startzeitpunkt. Im Bericht als
   Rekonstruktion deklariert, daher nur Hinweis.

4. `rust/crates/dbrain-population/src/aggregate.rs:118-124` (`compute_bucket`),
   info, kein Mangel. Die Deduplizierung je Zeile (`HashSet seen`) ist konsistent
   mit dem Python-Original: `clean_purchases`
   (`/tmp/dbm-eval/src/deadlock/features.py:64`) dedupliziert selbst nicht, aber
   der Prevalenz-Maßstab misst je Spieler (`counters.py:85` "Measured over
   players, not purchase rows, since no item is bought twice",
   `drop_duplicates(match_id, player_slot)`), und die README hält die Invariante
   "No item is ever bought twice by the same player" fest
   (`README.md:252`). Der zweite Schleifendurchgang nimmt Position, Kaufzeit und
   `sold` der ersten Vorkommnis; unter der Invariante ist erste gleich einzige
   Vorkommnis, ein legitimer Verkauf-und-Rückkauf entsteht in den bereinigten
   Daten nicht. Prevalenz, `weighted_buyers` und `sell_rate` zählen den Spieler
   jetzt genau einmal. `prevalence_raw > 1` ist damit ausgeschlossen. Test
   `duplicate_item_in_a_row_counts_the_player_once` deckt es ab. Semantik korrekt.

5. Refactor `5734b9d` (`plan_build`, `annotate_missing_authors`), info, kein
   Mangel. CLI und Example rufen dieselbe Bibliotheksfunktion: CLI über
   `reason_build_with_options` (`lib.rs:144`), das Mess-Example über `compose`
   (`examples/build_evaluation.rs`). `plan_build` (`lib.rs:88`) kapselt Patch-
   Delta, `apply_scored_patch_delta`, `score_items`, `finish_scores` und
   `compose_build_with_sources` verhaltensgleich zur früheren Inline-Kopie.
   `annotate_missing_authors` (`lib.rs:111`) ersetzt beide Inline-Kopien der
   Confidence-Low-plus-Rationale-Logik textgleich (`meta.core_layouts.overall.
   source_builds`). Fehlerpfade erhalten: `plan_build` gibt `Result` zurück, `?`
   propagiert im `spawn_blocking`-Closure, das äußere `map_err` behandelt weiter
   nur den JoinError; der KI-Kritiker- und Recompose-Pfad bleibt unverändert
   dahinter. Verbleibende, legitime Eingabe-Unterschiede: `meta` (inkl.
   `population`), `events` und `snapshots` werden im Produktivpfad aus der Live-DB
   (`load_reasoning_inputs`, `load_population_prior`), im Example aus FROZEN-V2
   plus `enrich_frozen_models` zusammengebaut. Das ist der Kern von live gegen
   eingefroren und keine Verhaltensänderung des geteilten Rechenkerns. Kein
   Doppelcode mehr.

6. Nit-Fixes `56308d1` gegen die Gate- und Review-Liste, info. (a) `ensure_schema`
   aus `run_population` entfernt, nur noch in `run_sync` nach `assert_writable`
   (`cli.rs:81-82`); `show` und `stats` ohne DDL: behoben (Randfall siehe Befund
   2). (b) Deduplizierung: behoben (Befund 4). (c) `started_at`: behoben mit
   Caveat (Befund 3). (d) `window_low`/`window_high` erst nach dem
   `i64::MAX`-Break gesetzt (`cli.rs:133-138`): behoben, der Sentinel-Seite kann
   das Fenster nicht mehr verfälschen. (e) `interpolate`-Klemme: bewusst keine
   Code-Änderung, im Bericht begründet; das konstante Halten des Netto-Vermögens
   nach dem letzten Sample entspricht dem Verhalten von Pythons `np.interp`
   (`features.py:networth_at`, Flach-Fortschreibung an den Rändern) und dem
   Rang-Charakter (`networth_quintile`). Nachvollziehbar und korrekt. (f)
   Marginal-Gate: Test `priority_staple_with_negative_context_margin_is_not_forced`
   ergänzt (`planner.rs:601`), Rot-Gegenprobe im Bericht beschrieben;
   `load_population_prior` prüft jetzt alle drei Populationstabellen
   (`population_item_stats`, `population_ability_order`, `population_imbue_stats`)
   und degradiert gemeinsam (`lib.rs:184-186`): behoben, deckt REVIEW-M-R2 Punkt 1.
   Zusätzlich: Hero-Filter-Pfad (REVIEW-P Punkt 4) und `average_badge = 0`
   (REVIEW-P Punkt 5) sind jetzt durch Tests belegt
   (`hero_filter_reaches_the_request_as_hero_ids`,
   `average_badge_zero_is_a_real_value_and_missing_is_neutral`).

7. Bau, info. `export PATH=/home/nathanael/.cargo/bin:$PATH`, `cargo 1.97.1`.
   `cargo test -p dbrain-reasoner --lib`: 171 bestanden, 16 ignoriert, 0
   fehlgeschlagen. `cargo test -p dbrain-population`: 25 bestanden, 0
   fehlgeschlagen. `cargo clippy -p dbrain-reasoner -p dbrain-population -p
   deadlock-brain --all-targets -- -D warnings`: Exit 0, keine Warnung. Kein
   `--release`, `pipefail` gesetzt. Zahlen decken sich mit der Fertigmeldung
   (171/16, 25). Die Workspace-Zahl 388/58 nicht nachgemessen (außerhalb des
   Prüfauftrags Punkt 4).

8. Stil, info. Kein neuer Code-Kommentar im Diff. Keine Gedankenstriche in
   nutzersichtbarem Text (die zwei `-`-Treffer sind arithmetisch: `prevalence_raw
   - 1.0` im Test und `now() - make_interval` in SQL). Echte Umlaute durchgängig.
   Keine Modellnamen, keine Referenz-Itemnamen im Produktcode (Item-IDs im Test
   sind synthetisch: 1, 2, 100).

## Urteil

Der Code ist freigabereif: geteilte Pipeline korrekt und ohne Doppelcode,
Fehlerpfade erhalten, Deduplizierung Python-konsistent, alle sechs Gate- und
Review-Nits abgearbeitet, Tests und Clippy grün, Stil sauber. Blockiert wird
allein die Ausreißer-Diagnose (`N-AUSREISSER.md`): sie ist nicht belegt und
widerspricht der 04:31-`stats`-plus-`show`-Ausgabe. Nachzubessern ist die
Diagnose (echte Ursache benennen: altes Binary oder Lesefilter in
`load_population_prior`, plus die fehlende Selbsterkennung des leeren
Populations-Reads), nicht der Code.

Mängel je Schwere:
Blockierend: 0
Mittel: 1
Gering: 2
