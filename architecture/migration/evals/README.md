# S10: Qualität, Sicherheit und Performance

Status: ausführbares Offline-Testwerkzeug im freigegebenen Vorbereitungsumfang. Keine G4-Abnahme und kein produktiver Cutover.

Basis: `30326512568b7370524956839100462ba71bdb92`. Im integrierten `STATUS.md` ist S10 für `prepare_only` reserviert. Contract-, DB-Schema- und Knowledge-Version sind dort noch nicht freigegeben. Deshalb verändert dieses Paket weder Produktcode noch zentrale CI, Datenbank oder Dienste. Der eigenständige Cargo-Workspace liegt ausschließlich im reservierten S10-Pfad; er ist kein zweiter produktiver Backendkern und definiert keine neue Brain-API.

## Ausführen

Vom Repository-Root mit der vorhandenen Rust-Toolchain 1.97.1:

```sh
cargo fmt --manifest-path architecture/migration/evals/Cargo.toml --check
cargo clippy --manifest-path architecture/migration/evals/Cargo.toml --all-targets --locked -- -D warnings
cargo test --manifest-path architecture/migration/evals/Cargo.toml --locked
cargo build --release --manifest-path architecture/migration/evals/Cargo.toml --locked
```

Bei vollständig vorhandenen Cargo-Abhängigkeiten zusätzlich `--offline`. Der produktive Workspace unter `rust/` bleibt unverändert. Dessen Standard-CI führt diesen separaten Workspace noch nicht aus; siehe `CR-S10-001.md`.

Das Binary liegt unter `architecture/migration/evals/target/release/dbrain-s10-evals`:

```sh
# Ausgabeziele müssen neu sein. Vorhandene Berichte werden nie überschrieben.
architecture/migration/evals/target/release/dbrain-s10-evals design architecture/migration/evals/profile.json architecture/migration/evals/cases.json /tmp/s10-design.json
architecture/migration/evals/target/release/dbrain-s10-evals baseline /tmp/s10-stage.json
architecture/migration/evals/target/release/dbrain-s10-evals support-inventory /pfad/zum/Deadlock-Docs /tmp/s10-support.json
architecture/migration/evals/target/release/dbrain-s10-evals assess /pfad/zum/freigegebenen-profil.json architecture/migration/evals/cases.json /pfad/zum/lauf.json VOLLSTAENDIGER_GEPRUEFTER_COMMIT /tmp/s10-assessment.json
```

`design` prüft ausschließlich den Testentwurf. `baseline` misst lokale, unveränderte Population-Metriken anhand exakter kleiner Referenzen. `support-inventory` erfasst die tatsächlich vorhandenen sechs Docs-Evaldateien mit Git-Revision, Hashes und Fallzahlen, ohne Fragen oder Dokumentinhalte zu veröffentlichen. `assess` wertet bereitgestellte Messdaten aus. Es erzeugt selbst weder HTTP-Last noch Provideraufrufe.

Exitcodes: `0` erfolgreicher Werkzeuglauf beziehungsweise bestandene Einzellauf-Prüfung; `2` nicht bestandene oder blockierte Einzellauf-Prüfung; `64` ungültige Eingabe oder Dateifehler. Kein Exitcode erlaubt Merge oder Release. `release_approved` bleibt immer `false`. JSON-Eingaben sind auf 4 MiB begrenzt; neue Ausgaben haben unter Unix Rechte 0600.

## Was geprüft wird

`cases.json` enthält 40 verbindliche Szenariospezifikationen für öffentliche, Domain-, interne, Code-/Betriebs-, Sicherheits-, Fehler-, Wiki-/Build-, Schema- und Replay-/Population-Fälle. Das sind Anforderungen an spätere echte Läufe, nicht 40 bereits ausgeführte Sicherheitstests. Entfernte Fälle, Wiederholungen, Cachezustände oder Assertions werden nicht stillschweigend akzeptiert.

Die ausführbaren Werkzeugtests prüfen insbesondere Versionsbindung, fehlende Belege, falsche Ergebnisse, fehlende Kosten, abgeschnittene Last, Zeitstempel, Quantile und Dateischutz. Wartezeit bleibt in der Ende-zu-Ende-Latenz. Fehler und Abbrüche bleiben in Fehlerquote und Kosten. Unbekannte Werte werden nicht als Nullkosten oder erfolgreiche Prüfung ausgegeben.

`quality::paired_noninferiority` ist eine Bibliotheksfunktion für gepaarte binäre Referenzlabels mit konservativer einseitiger 95-Prozent-Untergrenze. Sie ist noch kein vollständiger A/B-Bericht und setzt unabhängige Match-/Quellenfamilien voraus. `temporal_holdout` kontrolliert Match-Trennung sowie Ereignis- und Verfügbarkeitszeit. Modellselbstbewertungen ersetzen keine unabhängigen Labels.

## Bedeutung der Kennzahlen

Die Einzellauf-Zusammenfassung zählt akzeptierte Szenarioresultate, auch erwartete Ablehnungen. `error_rate` bezeichnet nicht akzeptierte Fallresultate, keine Transportfehlerquote. Throughput und Kosten beziehen sich ebenfalls auf akzeptierte Fallresultate. Ein späterer Produktionsbericht muss Transportfehler und beantwortete Anfragen zusätzlich getrennt ausweisen.

## Aussagegrenzen

Ein vollständiger Einzellauf benötigt freigegebenes Profil, G0–G3-Verweise, exakten Commit, Quellen-/Modell-/Datenstände, Hardware, alle Pflichtfälle und Messgrößen. Die Auswertung überprüft deren Form und Konsistenz, nicht die Wahrhaftigkeit eines vom Aufrufer gelieferten Reports. Hashes sind Referenzen, keine Signaturen und kein Nachweis, dass die referenzierten Dateien wirklich unabhängig geprüft wurden. `kind=real_e2e` allein macht Fixtures nicht zu echter Evidenz.

G4 verlangt zusätzlich unabhängige Prüfung der Rohartefakte, A/B/C/D-Vergleich einschließlich Nichtunterlegenheit, tatsächliche Sicherheits-/Fehlertests und Restore/Rollback mit S11. Die vorhandenen Tools `rust/crates/dbrain-retrieval/examples/ask_latency.rs` und `rust/crates/dbrain-reasoner/examples/build_evaluation.rs` sollen dafür weiterverwendet werden. Keine produktiven Zugangsdaten in PR-Tests und keine Last gegen die laufenden Bots.
