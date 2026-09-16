# FERTIG-N: Planner-Pipeline im Produktivpfad, Doppelcode weg

Branch `feat/reasoner-planner-produktiv` (Worktree `~/.worktrees/deadlock-brain-n`),
von main `7277ff5`.

## Ergebnis (TLDR)

- Am Code belegt: `reason build` läuft auf main `7277ff5` bereits über den
  Planner (`compose_build_with_sources` -> `plan_core` -> `plan_with_economy`)
  mit Progression, Combat und Population. Die Briefing-Hypothese (Welle-1-
  Composer) war überholt; der BEFUND-Bad-Build kam aus dem alten Release-Binary
  bzw. einem DB-Stand ohne Population. Details in `N-BEFUND.md`.
- Der echte Doppelcode (CLI und Mess-Example rechneten dieselbe Pipeline in zwei
  Kopien) ist entfernt: neue Bibliotheksfunktion `plan_build` plus Helfer
  `annotate_missing_authors`; CLI und Example rufen jetzt dieselbe Funktion.
- Livebeweis Warden **6/9** Referenzwaffen, Backtest bewertet dasselbe Build
  konsistent (tau 0,577, jaccard 0,500, nur Enduring Speed als fehlender
  Staple). Sechs Helden ohne Regression über eine Waffe. Zahlen in `N-MESSUNG.md`.
- Publish-Payload (`publish.rs`) und `BuildObject` unangetastet; `--no-ai`,
  `--no-persist`, `--json` funktionieren weiter. `patch-impact` läuft weiter
  (rc 0).

## Commits

- (dieser Commit) `refactor(reasoner): plan_build als geteilte Pipeline für CLI und Mess-Example`

## Geänderte Dateien

- `rust/crates/dbrain-reasoner/src/lib.rs`: `PlannedBuild`, `plan_build`,
  `annotate_missing_authors` neu; `reason_build_with_options` nutzt sie statt
  der eigenen Inline-Kopie.
- `rust/crates/dbrain-reasoner/examples/build_evaluation.rs`: `compose()` ruft
  `plan_build` und `annotate_missing_authors` statt der eigenen Pipeline-Kopie.
- `.tasks/2026-09-16-population-baseline/N-BEFUND.md`, `N-MESSUNG.md`, `FERTIG-N.md`.

## Testzahlen

- `cargo test -p dbrain-reasoner --lib`: 170 bestanden, 16 ignoriert, 0 rot
  (unverändert).
- `cargo test --workspace` ohne DSN: 386 bestanden, 58 ignoriert, 0 rot
  (Baseline gehalten).
- `cargo clippy -p dbrain-reasoner -p deadlock-brain --all-targets -- -D warnings`:
  sauber. Kein `--release`.

## Warden x/9 live

6/9 (Opening Rounds, Spiritual Overflow, Titanic Magazine, High-Velocity Rounds,
Fleetfoot, Swift Striker). Verfehlt: Frenzy, Blood Tribute, Monster Rounds.

## Sechs-Helden-Tabelle (live gegen M-nachher)

| Held | live | M-nachher | Delta |
|---|---|---|---:|
| Infernus | 5/7 | 5/7 | +0 |
| Lady Geist | 1/7 | 2/7 | -1 |
| Abrams | 4/7 | 3/7 | +1 |
| Vindicta | 2/3 | 2/3 | +0 |
| Bebop | 0/4 | 0/4 | +0 |
| Ivy | 2/3 | 2/3 | +0 |

## Laufzeit je Held

Warden 24 s, Infernus 25 s, Lady Geist 39 s, Abrams 71 s, Vindicta 22 s,
Bebop 43 s, Ivy 29 s. Alle unter 120 s.

## Offene Punkte und Abweichungen

- ABWEICHUNG (BEFUND-Prämisse): Der Produktivpfad nutzte den Planner schon vor
  Paket N (Paket M). Paket N belegt das, entfernt den Doppelcode und beweist den
  Livestand; es baut keinen fehlenden Planner-Pfad neu.
- Der deployte Release `57d3a07a` gibt den alten Bad-Build; ein frischer
  Release-Build von diesem Branch (oder main `7277ff5`) liefert 6/9. Der
  Release-Build und Deploy gehören dem Delegator.
- Warden-Staple-Gate bleibt nicht bestanden (nur Enduring Speed fehlt); das ist
  die aus M dokumentierte Grenze (Kontext-Marginal-Gate lehnt das anti-
  synergetische Item ab), keine N-Regression. Die Waffenlatte 6/9 ist erfüllt.
- Lady Geist -1 und Abrams +1 gegen die M-Nachher-Werte kommen aus dem
  Snapshot-Drift FROZEN-V2 gegen Live-DB (Ursache je Held in `N-MESSUNG.md`),
  innerhalb der erlaubten einen Waffe.
- `reason patch-impact` erzeugt weiterhin einen Score-Verschiebungsreport, kein
  Build-Objekt; unverändert und außerhalb des N-Umbaus.

## Nachträge (2026-09-16)

### 1. Ausreißer 04:34 geklärt (Commit `9385382`, `N-AUSREISSER.md`)

Kein Rennen, kein Nichtdeterminismus. Der schlechte Warden-Build um 04:34 kam aus
staleen Populations-Aggregaten: der 10k-`population sync` lud nur Rohzeilen, der
Aggregat-Neuaufbau (`population stats`) lief erst mit dem Timer um 04:46. Der
Build-Pfad liest nur die Aggregate, also griff bis 04:46 ein staple-armer Stand.
Belege: `write_hero` ist bereits atomar je Held (ein Commit, kein Torn Read),
drei Läufe `reason build Warden` sind zeichengleich, `prevalence_raw > 1` gibt es
im Bestand nicht, der zuletzt persistierte Warden-Build ist exakt das
Leerer-Prior-Build. Empfehlung: nach jedem Sync `population stats` laufen lassen
(der Timer macht das). Nit (c) macht das Sync-Fenster künftig im Log sichtbar.

### 2. Gate- und Review-Nits (Commit `56308d1`)

- (a) `ensure_schema` nur noch in `sync` und nach `assert_writable`; `show`/`stats`
  ohne DDL.
- (b) `compute_bucket` dedupliziert item_ids je Zeile; kein doppelt gezählter
  Spieler mehr, `prevalence_raw` bleibt unter 1. Test
  `duplicate_item_in_a_row_counts_the_player_once`.
- (c) `population_sync_runs.started_at` = Laufbeginn (`finished_at` minus
  Laufdauer) statt Insert-Zeit.
- (d) `run_sync` setzt `window_low`/`window_high` erst nach dem i64::MAX-Break.
- (e) `interpolate` klemmt Netto-Vermögen nach dem letzten Sample auf den letzten
  Wert. Das ist bewusst so: Netto-Vermögen dient nur als Rang je Zeitpunkt
  (`networth_quintile`), und ohne Messwert nach dem letzten Sample wäre eine
  lineare Fortschreibung erfundenes Wachstum. Konstantes Halten ist der ehrliche
  Rang-Proxy, deshalb keine Code-Änderung, nur hier dokumentiert.
- (f) Marginal-Gate im Planner mit Test `priority_staple_with_negative_context_-
  margin_is_not_forced` (Rot-Gegenprobe bestätigt: ohne den Guard erzwingt der
  Planner das anti-synergetische Item). `load_population_prior` prüft jetzt alle
  drei Populationstabellen und degradiert gemeinsam.

### Testzahlen nach den Nachträgen

- `cargo test -p dbrain-population`: 25 bestanden (vorher 24, plus Dedupe-Test),
  0 rot.
- `cargo test -p dbrain-reasoner --lib`: 171 bestanden (vorher 170, plus
  Marginal-Gate-Test), 16 ignoriert, 0 rot.
- `cargo test --workspace` ohne DSN: 388 bestanden, 58 ignoriert, 0 rot.
- `cargo clippy -p dbrain-reasoner -p dbrain-population -p deadlock-brain
  --all-targets -- -D warnings`: sauber.
- Live-Smoke nach den Änderungen: Warden 6/9, Population lädt (3-Tabellen-Guard).

## Nachbesserung Ausreißer (2026-09-16, zweite Runde)

Die erste Ausreißer-Erklärung (stale Aggregate) war falsch und ist widerlegt:
um 04:31 standen 10 Warden-Staples. `N-AUSREISSER.md` ist neu geschrieben.

### Echte Ursache (belegt)

Der 04:34-Build lief mit KI (`used_ai=t`) und las die Population korrekt (drei
Staples mit "Populations-Stütze"-Evidence). Ursache ist der KI-Kritiker-Recompose:
`run_critic` gab `recompose`, der Recompose reicht `critic.issues` (LLM-Fließtext)
als `blocked` an `compose_build_with_sources`, und `item_order` blockte jedes
Item, dessen Name als Teilstring im Fließtext vorkam. Der Kritiker erwähnt
Item-Namen in seinen Sätzen ("Spiritual Overflow, Healing Tempo"), also wurden
genau diese Staples still entfernt und der Recompose fiel auf defensive Items
zurück. Nichtdeterministisch (LLM), still ausgeliefert. Belege: die persistierte
04:34-Zeile trägt "Offene Kritikpunkte:" (nur nach Recompose) und zitiert Items,
die im Kern fehlen; `--no-ai` liefert fünfmal zeichengleich 6/9.

### Fix und Härtung (Commit s. u.)

- `item_order` blockt nur bei exaktem Namens-/ID-Abgleich statt per Teilstring in
  Fließtext. Test `free_text_issue_does_not_block_a_mentioned_item_only_exact_-
  names_do` mit Rot-Gegenprobe.
- `PopulationPrior::thin_coverage_note` plus Verdrahtung in
  `compose_build_with_sources`: deckt ein Build weniger als die Hälfte der
  Populations-Staples ab, steht das sichtbar in der Rationale und Confidence wird
  `Low`. Test `thin_coverage_note_fires_only_below_half_of_the_staples`.

### REVIEW-N-Nits

- `population stats`/`show` prüfen jetzt per `schema_present` (read-only,
  to_regclass) und melden freundlich "Zuerst 'population sync' laufen lassen"
  statt roh abzubrechen, ohne DDL.
- `population_sync_runs.started_at` wird jetzt aus der echten Startzeit gesetzt
  (`SystemTime` beim Laufbeginn, `to_timestamp`), nicht mehr aus finished minus
  Dauer rekonstruiert.

### Testzahlen nach der Nachbesserung

- `cargo test -p dbrain-reasoner --lib`: 173 bestanden (vorher 171, plus
  Blocklist- und thin-coverage-Test), 16 ignoriert, 0 rot.
- `cargo test -p dbrain-population`: 25 bestanden, 0 rot.
- `cargo test --workspace` ohne DSN: 390 bestanden, 58 ignoriert. Der Timing-Test
  `dbrain-sources::deadlock_api::tests::demo_poll_total_timeout_caps_the_next_-
  sleep` streut unter paralleler Last (isoliert fünfmal grün), fremdes Crate,
  von N nicht berührt, kein Zusammenhang mit den Änderungen.
- `cargo clippy -p dbrain-reasoner -p dbrain-population -p deadlock-brain
  --all-targets -- -D warnings`: sauber.
- Live-Smoke: Warden 6/9, Population lädt, thin-note feuert korrekt nicht (Deckung
  9/10).
