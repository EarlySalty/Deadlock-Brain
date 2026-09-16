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
