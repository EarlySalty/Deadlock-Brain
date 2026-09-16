# N-BEFUND: Welchen Pfad nimmt `reason build` und wo lag der Doppelcode

Stand 2026-09-16, Worktree `~/.worktrees/deadlock-brain-n`, Branch
`feat/reasoner-planner-produktiv` von main `7277ff5`.

## Am Code belegt

Der Produktivpfad der CLI `deadlock-brain reason build <held>` läuft so:

`main.rs:1599` ruft `reason_build_with_options` (`lib.rs`). Diese Funktion lädt
über `load_reasoning_inputs` (`lib.rs:452`) und rechnet in `spawn_blocking`
Patch-Delta, `item::score_items`, dann `composer::compose_build_with_sources`
(`composer.rs:334`). `compose_build_with_sources` ruft
`compose_build_with_author_evidence` (`composer.rs:491`), das über `plan_core`
(`composer.rs:409`) den Planner `planner::plan_with_economy` (`composer.rs:418`)
mit Progression, Combat-Bewertung und `PlanningContext.population` startet.

Damit ist die Hypothese aus dem Briefing (der Produktivpfad nutze weiter den
Welle-1-Composer ohne Planner) auf main `7277ff5` bereits **überholt**: Paket M
hat den Composer produktiv auf den Planner umgestellt und die Population in
`load_reasoning_inputs` verdrahtet (`load_population_prior`, `lib.rs:494`, setzt
`MetaIndexWithSources.population`). Der BEFUND-Bad-Build (4 Treffer, keine der
sechs großen Staples) stammt aus dem alten Release-Binary bzw. einem DB-Stand
ohne wirksame Population, nicht aus dem aktuellen main-Quellcode.

## Empirisch bestätigt

Das Debug-Binary aus dem Worktree (main-Stand, vor der N-Änderung) gegen die
zentrale DB (lesend, `--no-ai --no-persist`) liefert für Warden bereits
**6/9 Referenzwaffen** mit allen sechs großen Staples (Quicksilver Reload,
Mercurial Magnum, Opening Rounds, Titanic Magazine, High-Velocity Rounds,
Spiritual Overflow) im Kern. Das deckt sich exakt mit der 6/9-Messung aus M
gegen `FROZEN-V2.json`.

## Der echte Doppelcode

CLI und Mess-Example rechneten dieselbe deterministische Pipeline in zwei
getrennten Kopien:

- `reason_build_with_options` (`lib.rs`): Patch-Delta, `apply_scored_patch_delta`,
  `score_items`, Score-Nachbereitung, `compose_build_with_sources`, plus die
  Autoren-Confidence-Notiz bei fehlenden Autorenbuilds.
- `examples/build_evaluation.rs` `compose()`: exakt dieselbe Folge noch einmal
  von Hand, dazu eine eigene Kopie der Score-Nachbereitungsschleife und der
  Autoren-Confidence-Notiz.

Weil beide Kopien unabhängig gepflegt wurden, konnten CLI und Example
auseinanderlaufen; genau daraus entstand die Verwechslung im BEFUND. Paket N
hebt die gemeinsame Berechnung in eine Bibliotheksfunktion `plan_build`
(`lib.rs`) plus den Helfer `annotate_missing_authors`, die CLI und Example
gemeinsam aufrufen. Die Snapshot- und Autoren-Beschaffung bleibt getrennt
(Live-DB gegen FROZEN-V2), die Build-Berechnung ist danach dieselbe Funktion.
