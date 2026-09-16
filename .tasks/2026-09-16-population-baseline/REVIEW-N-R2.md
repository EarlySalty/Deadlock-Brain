# Review Paket N, Runde 2 (Rolle review_2_4)

FREIGABE

Geprüft gegen REVIEW-N.md und die zwei neuen Commits (79b530d, da17d7b),
kein Voll-Audit. Alle drei offenen Befunde aus Runde 1 sind abgearbeitet, der
Code ist grün und clippy-sauber, Stil ist sauber. Kein blockierender Mangel,
kein neuer Mangel.

## Status je Befund aus Runde 1

### Befund 1 (Mittel, Ausreißer-Diagnose): BEHOBEN

Die zweite Fassung von `N-AUSREISSER.md` verwirft die widerlegte
Stale-Aggregate-These und benennt die echte, am Code belegte Ursache. Alle vier
Kernbehauptungen sind verifiziert:

- `run_critic` gibt verdict `recompose` (lib.rs:154-155), und die
  Fließtext-Sätze `critic.issues` (`CriticResponse.issues: Vec<String>`,
  ai_roles.rs:57) werden als `blocked` an `compose_build_with_sources`
  durchgereicht (lib.rs:161). Belegt.
- Der alte `item_order`-Filter wertete per Teilstring aus:
  `issue.contains(&lower(&item.item.name)) || issue.contains(&id)` (Diff aus
  79b530d, composer.rs). Ein Fließtext, der einen Item-Namen erwähnt, warf das
  Item still aus dem Pool. Genau das erklärt den nichtdeterministischen
  04:34-Ausreißer (LLM), die persistierte Zeile mit "Offene Kritikpunkte:" und
  defensiven Items sowie die fünf zeichengleichen `--no-ai`-Läufe. Belegt.
- Die Runde-1-Alternativhypothesen sind sauber ausgeschlossen: das 04:34-Build
  lief mit dem neuen Release-Binary 57d3a07a (gebaut 04:26, mit
  `load_population_prior`) und `used_ai=t`, also weder altes Binary noch
  Lesefilter; die Population wurde gelesen (drei Staples mit
  "Populations-Stütze"-Evidence). Die Diagnose trennt die Hypothesen jetzt
  belegt, nicht behauptet.

Fix richtig:

- `item_order` blockt jetzt nur bei exaktem, case-insensitivem und getrimmtem
  Namens- oder ID-Abgleich (`entry == name || entry == id`, composer.rs:92-95).
  Ein Kritiker-Satz entfernt kein erwähntes Item mehr.
- Gezieltes Blocken bleibt möglich, aber nur, wenn der Kritiker den exakten
  Item-Namen (oder die numerische ID) als eigenen `issues`-Eintrag ausgibt; eine
  Aufzählung mehrerer Items in einem Satz blockt nichts mehr. Das ist die sichere
  Richtung: der Kritiker-Prompt (ai_roles.rs:171) verlangt keinen
  Ein-Name-je-Eintrag, in der Praxis prunt der Kritiker also kaum noch aktiv,
  aber es entfernt nichts mehr still. Kein Mangel, nur Hinweis.
- Fließtext-Issues werden weiter sichtbar: bei `recompose` hängt der Pfad
  `critic.issues` als "Offene Kritikpunkte: ..." an die Rationale (lib.rs:173-178).
  Sie verschwinden also nicht, sie stehen als offene Kritikpunkte im Build.
- Test `free_text_issue_does_not_block_a_mentioned_item_only_exact_names_do`
  (composer.rs:1188) ist aussagekräftig: er belegt beide Richtungen. Ein
  Fließtext mit beiden Namen behält beide Items (len 2), der exakte Name
  "spiritual overflow" entfernt genau Item 1 und behält Item 2. Deckt Regression
  und Positivfall.

Härtung gegen die stille Degradierung:

- `PopulationPrior::thin_coverage_note` (population_prior.rs) feuert, wenn ein
  Build weniger als die Hälfte der Populations-Staples abdeckt
  (`covered * 2 < total`), und ist in `compose_build_with_sources` verdrahtet:
  Confidence wird `Low` gesetzt und der Hinweis an die Rationale gehängt
  (composer.rs:378-388). Eine dünne Deckung steht damit sichtbar in Confidence
  und Evidence statt still ein anderes Ergebnis zu liefern. Test
  `thin_coverage_note_fires_only_below_half_of_the_staples` prüft die Schwelle
  (3/10 feuert, 9/10 nicht, leer nicht). Belegt.
- Wichtig für genau den Bug-Pfad: die Härtung greift auch im Recompose-Zweig, da
  `compose_build_with_sources` dort erneut läuft, und die gesetzte
  `Confidence::Low` überlebt das anschließende `enrich_build` (das nur an die
  Rationale anhängt und die Confidence nicht anfasst, lib.rs:700ff). Ein
  Recompose, der Staples verliert, liefert also Low plus sichtbaren Hinweis.

### Befund 2 (gering, run_stats vor erstem sync): BEHOBEN

`run_stats` und `run_show` prüfen jetzt vor dem Zugriff `db::schema_present`
(read-only `to_regclass('brain.population_item_stats')`, db.rs:27-33) und melden
freundlich "Keine Populationsdaten. Zuerst 'population sync' laufen lassen."
statt mit rohem sqlx-Fehler abzubrechen (cli.rs run_stats/run_show). Kein DDL im
Lesepfad, `ensure_schema` bleibt allein in `run_sync` nach `assert_writable`.

### Befund 3 (gering, started_at): BEHOBEN

`started_at` wird jetzt real am Laufbeginn erfasst: `SystemTime::now()` vor der
Sync-Schleife (cli.rs), über `SyncRun.started_at_unix` an
`record_sync_run` gereicht und per `to_timestamp($10)` gesetzt (db.rs:105-127),
`finished_at` bleibt `now()`. Kein rekonstruierter, sondern ein gemessener
Startzeitpunkt. Das Sync-Fenster ist im Log korrekt.

## Bau (Punkt 3)

`export PATH=/home/nathanael/.cargo/bin:$PATH`, `pipefail` gesetzt, kein
`--release`, cargo 1.97.1.

- `cargo test -p dbrain-reasoner --lib`: 173 bestanden, 16 ignoriert, 0
  fehlgeschlagen.
- `cargo test -p dbrain-population`: 25 bestanden, 0 fehlgeschlagen.
- `cargo clippy -p dbrain-reasoner -p dbrain-population -p deadlock-brain
  --all-targets -- -D warnings`: Exit 0, keine Warnung.

Zahlen decken sich mit FERTIG-N (173, 25). Der in FERTIG-N erwähnte streuende
Timing-Test liegt in `dbrain-sources`, außerhalb des Prüfauftrags und von N nicht
berührt.

## Stil (Punkt 4)

- Kein neuer Code-Kommentar im Diff (item_order, thin_coverage_note, cli, db
  kommentarfrei).
- Echte Umlaute durchgängig, auch in den Nutzermeldungen ("laufen lassen",
  "dünn").
- Keine Gedankenstriche in nutzersichtbarem Text; die Bindestriche sind
  Komposita ("Populations-Deckung", "Spieler-Matches"), kein Em- oder En-Dash.
- Keine Modellnamen im Produktcode, keine Referenz-Itemnamen (Testnamen wie
  "Spiritual Overflow" stehen nur im Test).

## Neue Befunde

- Info, kein Mangel: Der Kritiker kann Items nur noch über einen exakt passenden
  Einzeleintrag blocken; sein Prompt garantiert das nicht. Das ist die sichere
  Richtung (kein stilles Entfernen mehr) und Fließtext-Issues bleiben als "Offene
  Kritikpunkte" sichtbar. Kein Handlungsbedarf, nur zur Kenntnis, falls jemand
  später aktives Prunen durch den Kritiker erwartet.

## Urteil

Freigabereif. Die Ausreißer-Diagnose ist in der zweiten Fassung am Code belegt
(Teilstring-Blocken im Recompose), der Fix ist korrekt (exakter Namens- oder ID-
Abgleich, Test mit beiden Richtungen), und die fehlende Selbsterkennung ist über
`thin_coverage_note` plus Confidence Low geschlossen, wirksam auch im
Recompose-Pfad. Beide geringen Nits sind sauber behoben. Tests und Clippy grün,
Stil sauber.

Blockierend: 0
Mittel: 0
Gering: 0
