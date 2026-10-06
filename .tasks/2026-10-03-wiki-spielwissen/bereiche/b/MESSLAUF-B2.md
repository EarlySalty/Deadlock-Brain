status: aktiv
Datum: 2026-10-03
Stand: 2026-10-03T09:02:23Z

# B2: Messlauf mit rotem Clippy

Task `b9tza4noz` startete 06:50:12 UTC und endete regulär 08:54:28 UTC mit Gesamt-Exit 1, 7.456 Sekunden einschließlich Sperrwartezeit. Das tatsächliche Log enthält `RUN_EXIT 1`, `LOCK_RELEASE both released` und erfolgreiche Vorher-/Nachher-Quell- und Artefakthashbindung. Kein Gesamterfolg.

fmt und vier Rust-Compilerläufe grün. 27 Extraktortests, fünf Validatortests und ein zusätzlicher begrenzter 51-Byte-VPK-Test erfolgreich, keine ignoriert. Beide vollständigen Git-Export-/Validatorläufe grün; Dokumente/Fakten unverändert gegenüber Erstlauf. Keine neue Steam-/VPK-Rohdatenabnahme.

TESTNACHWEIS[TW-1]: 33 passed, 0 ignored | Baseline: unbekannt rot

## Tatsächlich gemessene Ressourcen

Linux-Prozesspeak `VmHWM` in KiB, monotone Dauer des jeweiligen vollständigen Laufs, nicht gesamte Host-RAM-Grenze und nicht Sperrwartezeit. Originalmessdateien außerhalb Git in round3-proof, durch artifacts.sha256 gebunden.

| Quelle | Lauf | Dauer in Sekunden | Peak-RSS in KiB |
| --- | --- | ---: | ---: |
| GameTracking | Extraktor | 29,905512983 | 394.968 |
| GameTracking | Validator | 17,382368559 | 499.708 |
| deadlock-data | Extraktor | 34,229362360 | 42.040 |
| deadlock-data | Validator | 20,484072983 | 51.900 |

## Verifizierte Clippy-Funde

Extraktor im Test- und Normalmodus: drei `needless_borrow` bei `game_files/vpk.rs:107,114,121`, ein `manual_is_multiple_of` bei `game_files.rs:657`. Validator im Test- und Normalmodus: ein `manual_is_multiple_of` bei `pruefharness-b/src/bin/validate-jsonl.rs:89`. Fünf unterschiedliche Stellen, vier Clippy-Aufrufe mit Exit 1 unter `-D warnings`. Keine Lints unterdrücken; vorhandene Ausdrücke minimal korrigieren, keine Parser-Neuimplementierung.

Messworker besitzt nur den Validator-/Messharness und korrigiert seine eine Stelle selbst. Die vier Parserstellen gehören an einen frischen gezielten Fixer nach bestätigter Eigentumsabgabe. Seine bereits gestartete erneute Wartetask `bubtwgxd0` wurde für diese konkret nötigen Parserfixes vor Compilerstart geordnet beendet, nicht wegen Timer. Endbericht/Hashstand des Workers noch ausstehend.

Eigene unabhängige Prozessprobe 09:00:17 UTC: ursprüngliche PIDs 3412237/3412240/3412244 und Folge-PIDs 4163398/4163402/4163406 sowie Nachkommen verschwunden, keine eigenen Lockeinträge. Kein fremder Prozess gestoppt, keine globale Sperrfreiheit behauptet. Parent hat bis zur Eigentumsabgabe keinen weiteren Writer oder Compiler gestartet.

Nachweisroot `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/round3-proof/`: check-b3-run.log, je Quelle JSONL/Inventar/Validatorlog und extract.resources.log/validate.resources.log, source-before.sha256/source-after.sha256, artifacts.sha256. Ältere round2-proof-Beweise erhalten. Weitere Änderungen entwerten die betroffenen Hashnachweise; nach minimalen Fixes vollständiger neuer Lauf im vorhandenen Wrapper und frische unabhängige Eigenabnahme.

Echter D-Depotlauf und unmittelbare Punkt-31-Inventarbindung separat offen. Acht Nichtdokumentdateien unverändert als Originale/Inventar/Herkunft mit Gründen erhalten. C2 allein Integration/Deploy, B2 liefert nach wirklich grünen Restchecks nur Parser-Eigencommits.

## Grüner Endstandlauf nach den fünf minimalen Lintkorrekturen

Task `bcc4xxitp`, 09:09:20 bis 09:46:16 UTC, regulärer Exit 0, 2.216 Sekunden einschließlich Sperrwartezeit. Tatsächlicher Nachweisroot `round3-proof/run-5gzlJFjs`. fmt, vier Compiler und vier Clippy-Modi mit `-D warnings` erfolgreich, Testflags `--include-ignored --test-threads=2`. 27 Extraktortests, fünf Validatortests und eine zusätzliche VPK-Probe unter 128 MiB bestanden. Keine Lintunterdrückung, keine neue Parserimplementierung.

TESTNACHWEIS[TW-1]: 33 passed, 0 ignored | Baseline: 0 rot

| Quelle | Lauf | Dauer in Sekunden | Peak-RSS in KiB |
| --- | --- | ---: | ---: |
| GameTracking | Extraktor | 41,122702289 | 395.036 |
| GameTracking | Validator | 23,421946167 | 499.680 |
| deadlock-data | Extraktor | 41,437847723 | 42.216 |
| deadlock-data | Validator | 26,285260107 | 51.204 |

Beide Quellen vollständig extrahiert und validiert, JSONL und Inventare bytegleich zu Vorläufen. Ressourcenlogs mit `success=true` in Artefaktmanifest gebunden. Prozesspeak VmHWM, keine globale Host-RAM-Grenze. Quellmanifest enthält zehn Ausführungsdateien und ist vor/nach identisch, SHA-256 `80fd9cc6e812a570934d7ecc1bae765d4f5719a30e9452c3d716febd46a2a386`. Separat nachgemessener Elfdatei-Freeze einschließlich unverändertem Harness-Cargo `3cef2c79fd36c9462948f90ceb4fa28b6b82b6ad1102769a8b8812a66132055c`. Artefaktmanifest SHA-256 `f9183fa1dd2fe47cd6ea24f7d396452f42ce862cd1e34705741b21ab5b880ea6`.

Elternprobe 10:03:51 UTC bestätigt eigenes Prozess-/Kinderende; beide Locks laut Log freigegeben, keine eigenen Sperreinträge. Fixer hat Eigentum abgegeben. Frische unabhängige lesende Eigenabnahme `a80e83655289d465f` gestartet, Ergebnis noch offen. Seit Punkt 41 ist C3 aktiver alleiniger Integrator/Deployer, C2 beendet. Historische rote Vorlaufaussagen oben bleiben erhalten und sind durch diesen grünen Endstandlauf abgelöst. Echte D-Depotabnahme weiterhin offen.

