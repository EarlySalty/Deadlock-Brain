# Übergabe · Chat 05 / Domain-Vorprüfung

Status: blockiert für Implementierung; Preflight-Artefakte getestet und zum Review vorgeschlagen
Basis-Commit: `c00fc8935048bf490c1e4790f7c6195864ad49e2`
Ergebnis-Commit / PR: Test-/Entwurfscommit `19b4e26763e5b1f3a3f8b3d145a004e5258d4145`; ergänzender Evidenzcommit ist der diese Übergabe enthaltende Commit. Draft-PR auf Branch `feat/brain-s05-domain-audit-20260924` vorgesehen.
Tatsächlich getesteter Commit: `19b4e26763e5b1f3a3f8b3d145a004e5258d4145`
Contract-/Schema-Version: offen / offen; nicht durch S05 erfunden
Source-/Corpus-/Modellversion: Quellcodebasis wie oben; synthetische Fixture `S05-ALIAS-20260924-v1`; kein Corpusrelease oder Modell verwendet
Betroffene Requirement-IDs: R03, R04, R10, R23, R24, R28–R34, R36–R37, R43, R47, R52–R53, R58, R60. Nur R10/R28/R34/R58 erhalten eine kleine konkrete Differentialprobe; keine vollständige Erfüllung dieser Anforderungen behauptet.

## Ergebnis und konkrete Änderungen

Ausschließlich neue Dateien unter `architecture/migration/s05/`:

- `probes/run.rs`, `probes/alias_cases.rs`, `probes/checks.rs`: eigenständige Rust-Prüfung des unveränderten vorhandenen Aliasnormalisierers; 18 eingefrorene Alt-/Rust-Fälle, 4.684 Eigenschaftsfälle und vier Tests der Quellenextraktion.
- `DOMAIN_PARITAET.csv`: 16 offene Fach-/Abnahmebereiche; nur der Aliasbaustein wurde differential ausgeführt. Kein vollständiger Funktionsscan oder Gesamtparitätsnachweis.
- `README.md`: Gatebefund, Reproduktion, fachliche Toleranzvorschläge und Messgrenzen.
- `CHANGE_REQUEST.md`: CR-S05-001 an 00/02/03; Unicode-/Alias-Semantik, G1-Verträge, Schlüsselmigration und Ownerüberschneidung 05/14.
- `SOURCES.sha256`: sieben Quell-/Koordinations-/Lockfilehashes der geprüften Basis.
- `TEST_RESULTS.json`: Befehle, tatsächliche Exitcodes, vollständige Ausgaben und Audit-Wandzeiten am Testcommit. `domain_parity_accepted` ist ausdrücklich `false`.
- `LEGACY_REFERENCE.json`: alle 18 Goldens erneut gegen die tatsächliche gepinnte Python-FunctionDef verifiziert; Python 3.12.3 / Unicode 15.0.0, auch zweite Normalisierung zum Nachweis der Alt-Idempotenzabweichung dokumentiert.
- Diese Übergabe.

Keine neuen öffentlichen Produktinterfaces, keine Root-Dependencies, keine Änderungen an Cargo-Workspace/-Lockfile oder Produktionsquellen. Die Auditquellen sind keine zweite Domainengine.

## Nachweise

Arbeitsverzeichnis: isolierter S05-Worktree der genannten Codebasis. Native Audit-Toolchain `rustc 1.75.0`, Host `x86_64-unknown-linux-gnu`, LLVM 17.0.6. Keine Aussage über Eignung dieser Toolchain für den ganzen Workspace.

| Prüfung | Befehl / Umgebung | Tatsächliches Resultat | Artefakt |
|---|---|---|---|
| Audit kompilieren | `rustc --edition=2021 -Dwarnings architecture/migration/s05/probes/run.rs -o /tmp/s05-probe-19b4e26` | Exit 0 | TEST_RESULTS.json |
| Audit-Hilfstests kompilieren | `rustc --edition=2021 -Dwarnings --test architecture/migration/s05/probes/run.rs -o /tmp/s05-probe-tests-19b4e26` | Exit 0 | TEST_RESULTS.json |
| Hilfstests | `/tmp/s05-probe-tests-19b4e26 --nocapture` | 4 bestanden, 0 fehlgeschlagen/ignoriert | TEST_RESULTS.json |
| Verhalten reproduzieren | `/tmp/s05-probe-19b4e26 /tmp/s05-characterization-19b4e26 --characterize` | Exit 0; 11/18 identisch, 7 bekannte Unterschiede, 0 unerwartete Baselineänderungen; 4.684 Eigenschaftsfälle bestanden | TEST_RESULTS.json |
| Strenge Fachparität | `/tmp/s05-probe-19b4e26 /tmp/s05-strict-19b4e26 --strict` | **Exit 1, NICHT BESTANDEN** wegen sieben ungeklärten Unterschieden | TEST_RESULTS.json |
| Eingefrorene Quellen | `sha256sum --check architecture/migration/s05/SOURCES.sha256` | sieben Hashes unverändert | TEST_RESULTS.json |
| Matrixstruktur/Pfade | einmalige Standardbibliotheksprüfung mit csv und pathlib | 16 eindeutige Feature-IDs, konsistente Spalten, alle konkret genannten Rust-Pfade vorhanden | TEST_RESULTS.json |
| Whitespace | `git diff --check`; vor erstem Commit zusätzlich `git diff --cached --check` | Exit 0 | Testprotokoll / Git |
| Legacy-Goldens | einmalige AST-Ausführung nur der gepinnten FunctionDef; Abgleich aller 18 Rust-Fixture-Erwartungen | 18/18 Referenzausgaben bestätigt | LEGACY_REFERENCE.json |
| Formatter | `rustfmt --edition 2021` für die drei Probequellen | nicht verfügbar (`rustfmt: not found`, Exit 127); nicht als bestanden gewertet | diese Übergabe |
| Gesamter Cargo-Workspace, DB, Provider, Learning, Last | nicht ausgeführt | offen | README / Matrix |

Die anfängliche zusätzliche JSON-Erfassung der Legacy-Referenz scheiterte im Audit-Konvertierer an einem nicht JSON-escaped Steuerzeichen. Nach Korrektur dieser ausschließlich einmaligen Erfassung wurden alle 18 Ausgaben erfolgreich überprüft; Produktionsquellen und Rust-Goldens wurden nicht geändert.

## Folgen

Datenmigration/Kompatibilität/Wiederanlauf: keine Daten geschrieben. Eine spätere Aliasänderung kann gespeicherte Schlüssel/Indizes betreffen; Kollisionen, Backfill, Reindex und Rollback gehören zu 03 nach gemeinsamer Entscheidung.

Berechtigungen/Secrets/Egress: ausschließlich Repositoryquellen und synthetische Zeichenfolgen. Keine Spieler-/Replay-/Privatdaten, keine Secretwerte und keine externen Datenquellen in die Artefakte aufgenommen. Die native Probe benötigt keine Datenbank und kein Netzwerk. Die Ausgabe lokaler Audit-Zeitmessungen ist kein Runtime-/Egress-Audit der gesamten Anwendung.

Latenz/Ressourcen/Kosten und Messgrenzen: keine Produktionsmessung. Korpus begrenzt; Compiler/Prüfprozess werden vom Rust-Treiber jeweils nach 60 Sekunden beendet. Kein nachgewiesenes produktives RAM-/CPU-/Deadlinebudget. Keine Providerkosten verursacht.

Vorhandene Funktionen erhalten / bewusste Abweichung: Produktcode unverändert. Sieben Unterschiede dokumentiert, **keine** davon genehmigt. Vier betreffen Casefolding, zwei nicht idempotentes Altverhalten an Rand-Unterstrichen, einer Whitespace/Steuerzeichen. Alte Fehler werden nicht automatisch konserviert.

Python-/Legacyfreiheit: produktiver Code unverändert; reguläre Probe vollständig Rust. Python wurde ausschließlich einmalig für die Ausführung der alten reinen Funktion, die Erfassung von Referenzen und die Sammlung lokaler Testbelege verwendet. Kein installierter Worker, Timer, Sidecar, PyO3 oder HTTP-Rückdelegationspfad. Die Pythonfreiheit des gesamten bestehenden Backends bleibt ungetestet.

## Grenzen und Blocker

G0/G1 stehen auf der integrierten Basis offen; S01-Inventar und S10-Testprofil sind für diese Basis nicht als integrierte Gatefreigabe belegt. Vertrag, Schema, DomainStore-/Artefaktports und eindeutige Population-Pfadzuständigkeit fehlen. Kein `implement` erteilt, auch kein regulärer S05-Start vorweggenommen.

Noch ausstehend sind vollständige Entity-, Build-, Optimizer-, Lernzyklus-, Coaching-, Meta-, Analytics-, Lineage-, Effect-/Rule-/Graph-, historische/Mode-/Varianten- sowie Holdout-Abnahmen. Die entsprechenden Zeilen bleiben offen. Keine Freigabe echter Pflichtquellen oder Replays wird aus den synthetischen Tests abgeleitet.

## Übergabe an nächsten Besitzer

00: diesen Draft und CR-S05-001 prüfen; S01/S10 integrieren, Gateentscheidung und reale Pfade bestätigen. Gemeinsamen STATUS ausschließlich selbst aktualisieren.

02/03: gemeinsamen Normalisierungs-/Fact-/Rule-/Evidence-/Artefaktvertrag samt Versionen und Alias-Migrationsbedarf entscheiden. Keine ad-hoc-Schnittstelle aus diesem Audit übernehmen.

05 nach G1: auf den neu integrierten Contract-/Storecommit rebasen; vorhandene 18 Fälle erneut prüfen, genehmigte Semantik implementieren und erst danach weitere deterministische Fachgruppen bearbeiten. Gegenwarts-/Historien-/Unknown- und kleine exakte Buildreferenzen ergänzen, anschließend vollständige Lernzyklen und Datenpfade prüfen.

10/12/13/14: fachspezifische Referenzen und freigegebene Daten für Folgeabnahmen bereitstellen; keine Arbeit dieser Besitzer durch diesen PR als erledigt markieren.

## Integration durch Chat 00

Merge-Commit: offen
Gate-/STATUS-Änderung: keine
Freigegeben von: niemand / ausstehend
Neue nächste Arbeitswelle: nicht durch S05 freigegeben

## Zusätzliche Angaben v1.0

Arbeitsmodus / freigegebenes Gate: nichtproduktive Vorprüfung; `implement` gesperrt, G0/G1 offen
Verwendete Plan-/Source-/Knowledge-/Rule-/Parserrevisionen: Plan 1.0, Basiscommit und Sourcehashes wie oben; Knowledge/Rule/Parser-Releases offen
Echte Integration oder nur Mock/Fixture: echte vorhandene reine Funktion auf synthetischer Fixture; keine gesamte Crate- oder Storeintegration
Betroffene Pfade/Owner und Folge-PRs: nur neue S05-Übergabepfade; 00 integriert Dokumentation. Vertrags-/Schema-/Produktänderungen später bei 02/03/05 nach Gateentscheidung.
Sensible Quell-/Replay-/Publikationsrechte: keine solchen Daten genutzt; reale Pflichtquellenrechte bleiben offen
