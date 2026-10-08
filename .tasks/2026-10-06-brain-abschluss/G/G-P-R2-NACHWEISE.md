# G-P-R2: transienten Status und Abrechnung erhalten

status: gebaut, lokal geprüft, Fixdelta ALLOW, 07.10.2026

Frischer Fixer `wrq0z0aau` / `wf_670d3b4c-132` tatsächlich abgeschlossen. Commit `1b5ea4527576c1b8b622e86e81067b396889f2b0`, Base `45f51d6f`. Genau `brain-providers/src/transport.rs` und `tests/faults.rs` geändert; Bereichsführung bestätigte beide Quellen gegen Commit mit Exit 0.

Fehler beim begrenzten Lesen eines transienten HTTP-Diagnosekörpers beenden nicht mehr sofort den Retryloop. Vorhandene Frist, Versuchszahl und kumulierte Reservierung begrenzen weitere Aufrufe. Usage bleibt beobachtet beziehungsweise reserviert, kein neuer Ledger. Loopback deckt übergroße, abgeschnittene und zeitlich gescheiterte Körper sowie ursprüngliche Frist und Accounting ab. In der geprüften nativen Konfiguration bleibt ein Versuch; dort ist Statuserhalt, kein anschließender Retry-Erfolg belegt.

Format, Compiler, Clippy mit -D warnings und vollständige Providersuite Exit 0. Tatsächliche Marker durch Bereichsführung gelesen: 11 Unit- und 33 Faultfälle, 44 passed/0 failed/0 ignored/0 filtered; Doc-Tests 0. Vollständige Befehle und unverdeckte Exits `G/pruefungen/g-p-r2/commands.log`, tatsächliche Läufe in check.log, clippy.log und test.log. Vorherige G-K-R1-Providerabdeckung: 11 Unit/29 Fault, 40 passed/0 failed; keine eigene neu ausgeführte Rotbaseline behauptet.

Regulärer Fixdelta-Gate `45f51d6f..1b5ea452`, Exit 0, tatsächliches gate.log: `[gpt-6.1-sol] ALLOW: No blocking defects found in the supplied diff and revision snapshots.` Zwei eigene Git-Schreibschritte einzeln: gezieltes add und Featurecommit. Kein Push, Main, Deploy oder Liveprovider. Vollständiger ursprünglicher Provider-/Kernelumfang wird nach Kernelkorrektur gemeinsam nachgeprüft; Delta-ALLOW ist keine Gesamt-G-Abnahme.

TESTNACHWEIS[TW-1]: 44 passed, 0 ignored | Baseline: 0 rot

MERGEPROTOKOLL[MS-1]: 2 Git-Schritte einzeln | Anläufe: 1 | Gate: [gpt-6.1-sol] ALLOW; kein Main-Merge
