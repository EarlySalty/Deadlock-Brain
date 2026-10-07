status: aktiv
Datum: 2026-10-03

# Alte Drafts: erhalten

Vergleichsstand: `origin/main` und entfernter main `511a347b653beba13c2bf130f4bead7a7196cc2a`. Bestandsprüfung durch nativen Worker in `wf_f79b579d-066`, beendet mit einer vollständigen Rückgabe ohne Startfehler. Teil-Z hat die Rückgabe hier festgehalten; der Worker selbst schrieb keinen Bericht.

## Urteil

Keiner der fünf Drafts darf derzeit als nachweislich überholt geschlossen werden. Bei keinem ist die vollständige Übernahme oder Überholung belegt. Die sichere Handlung ist, PRs und zugehörige Branches zu erhalten. Es gibt keinen Auftrag, ihre Restinhalte in Paket Z zusätzlich umzusetzen.

| PR | Head | Verwertbarer Rest laut Bestandsprüfung |
| --- | --- | --- |
| #3 | `089be38c5e4d755abb59952ba74d4e1cdfc86378` | Eigenständige Patchanalyse, Storyboard und zeitliche Caption-Belege. Main verarbeitet in `rust/crates/deadlock-brain-yt/src/transcripts.rs:364` Caption-Text; separate Evidenzrevisionen sind nicht nachgewiesen. |
| #4 | `9efeb1e44ead5cdf5d01e05f242291fee79e803e` | Patchhistorie mit Erstbeobachtung, Invalidierung gespeicherter Analysen und Trust-Korrekturen. `rust/crates/deadlock-brain/src/pg_insights.rs:206` stuft anhand der Confidence hoch; die Historienmigrationen des Branches sind nicht in main. |
| #5 | `9ead46171f3d0f3c5d0e2fe739f1a9e693e37013` | Inhaltsdrifterkennung, identitätsgeprüfter Quellenrefresh und Invalidierung beider Berichte bei Quellenwechsel. `pg_patchnotes.rs` enthält weder `sync_patchnotes` noch `detect_patchnote_drift`. Dokumentrevisionen im neuen Feed belegen keine vollständige Ersetzung dieser Analyseverträge. |
| #6 | `458d56d0845f83bb50fdb635e78c0367f3cff8a1` | Zusätzliche Claim-Verbraucherschutzfilter. `rust/crates/deadlock-brain-yt/src/claims.rs:178` berücksichtigt `needs_claim_revalidation` nicht; der entsprechende Retrieval-Filter ist nicht nachgewiesen. |
| #9 | `e752d2514249ece9b3702c5fd93a75680495db4c` | Assets-Ursprung und wesentliche Build-Familien stehen inzwischen in main. Restwert haben `float_roundtrip`, explizite Spielversionsbindung und `rebase-assets`; das Feature ist in `dbrain-reasoner/Cargo.toml:17` nicht aktiviert, der Rebase-Befehl fehlt in `examples/family_evaluation.rs`. |

## Git-Nachweise und Grenzen

- `git merge-base --is-ancestor <head> origin/main`: Exit 1 bei #3, #5, #6 und #9 in der Hauptsession. Der Worker bestätigt, dass die vollständige Übernahme auch für #4 nicht belegt ist.
- `git cherry origin/main <head>`: keine identischen Einzelpatches; 2, 18, 20 und 37 nicht äquivalente Commits bei #3, #5, #6 und #9. Ein Squash kann diese Messung verändern. Sie allein rechtfertigt weder Verlustbehauptung noch Löschung.
- Die Kette #4 nach #5 nach #6 nach #9 ist laut Worker jeweils mit Ancestor-Exitcode 0 bestätigt. Daher zunächst Restinhalte von #9 gesammelt weiterverwenden, #3 separat vergleichen, erst anschließend alte Drafts schließen.
- Der Befund beschreibt Restwert, keine bestätigte Produktivstörung und keine Abnahme dieser Altbranches. Es wurden keine Builds, Tests, Git-Schreibschritte oder PR-Kommentare ausgeführt.
- PR #46 wurde nicht untersucht oder verändert. Er bleibt bis zur Rückmeldung von R geschützt.

Rohbericht: `/home/nathanael/.claude/projects/-home-nathanael--worktrees-brain-fertig-z/bca6cf6b-c41f-4ca9-bd55-662039983c13/subagents/workflows/wf_f79b579d-066/journal.jsonl`.
