# G-P-R1: Gemeinsamer JSON-Eingang, Schlussprüfung offen

status: zurückgegeben; keine Produktabnahme, 07.10.2026

## Umsetzung und Eigentum

Tatsächliche Rückgabe von Task `wvlm6fn18`, Run `wf_89eb7717-56e`, Agent `a3f02c3eee712b052` gelesen. Vorhandener rekursiver Unique-Visitor wurde im gemeinsamen Vertragsbereich benutzt. Sources behält seinen privaten `parse(&[u8]) -> Result<Value, serde_json::Error)` als Delegationsadapter. Anschließende Standard-serde-Materialisierung erhält die aktive `arbitrary_precision`-Zahlendarstellung. Keine neue Parserimplementierung, Crate oder Manifestabhängigkeit.

Geändert: `brain-contracts/src/provider_input.rs`, `dbrain-sources/src/external/strict_json.rs`, `brain-providers/src/transport.rs` und `brain-providers/tests/faults.rs`. Rohe Antwortobjekte, verschachtelte Argumente, kodierte Argumentstrings, Werkzeughistorien und finales JSON werden auf doppelte Namen geprüft, einschließlich Unicode-identischer Feldnamen. Beide unveränderten ursprünglichen Wireproben liefern `InvalidResponse("invalid chat schema")`.

## Tatsächliche Prüfgrenze

Vor der letzten Ergänzung waren Compiler, striktes Clippy und Suiten grün: 71 Vertragsfälle und 32 Providerfälle, insgesamt 103 passed, 0 ignored. Danach kam die Duplikatablehnung für finales JSON ohne Belege hinzu. Diese letzte Änderung ist nur formatgeprüft. Die 103 Fälle liefern keinen aktuellen Gesamtbeweis.

Letzter Compileranlauf erreichte keinen Buildslot; der nichtblockierende Folgelauf endete mit Exit 1 ohne Compilerstart. Source-/Verbraucherläufe endeten mit Exit 101 an damals noch fehlenden `apply_scenario_bonuses` und `simulate_calculation_with_deadline` im parallelen Reasoner-WIP. Fremde Dateien wurden nicht geändert; daraus wird kein endgültiger Reasoner- oder JSON-Regressionsbefund abgeleitet.

Rohbelege: `G/pruefungen/g-p-r1/`, mit vollständigen Logs, Exits, `commands.log`, `own-r3.diff` und `tested-r3-after.sha256`. Rückgabe bindet 86/86 unveränderte Quellen; Manifest-SHA256 `16b71706f0c79c5021d3dea14eb2115ce66da3109796284f7271c7d7e0db96bf`. Kein eigenes Gate, Git oder Runtimebeweis.

## Anschluss

Nach tatsächlichem Abschluss an den einzigen neuen Vertrags-/Provider-/Kernelworker G-K-R1 übergeben. Dieser prüft die letzte JSON-Ergänzung vollständig mit und erweitert nur die notwendige typisierte Fehlerabrechnung. G-M-06:45 arbeitet disjunkt im Reasoner. Produktcheckpoint erst nach gültigen Quellenbindungen, Compiler-/Lint-/Testbelegen und eigenem regulären Gate.
