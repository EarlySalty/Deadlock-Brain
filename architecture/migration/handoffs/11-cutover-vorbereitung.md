# Übergabe: S11 / Produktivumstellung, Betrieb und Legacy-Ende

Status: Vorbereitung getestet; vollständiges S11 **blockiert**, nicht integriert.
Basis-Commit: `30326512568b7370524956839100462ba71bdb92` (`origin/main`).
Ergebnis-Branch: `migration/s11-cutover-preparation-20260924`.
Implementierungscommit: `d4e911a66542339a0babaf70311e9a4e3254995d`.
PR: https://github.com/EarlySalty/Deadlock-Brain/pull/26 (Draft, offen).
Tatsächlich getesteter Commit: `d4e911a66542339a0babaf70311e9a4e3254995d`;
Format, Clippy, 28 Tests und Releasebuild nach dem Commit erneut bestanden.
Die nachfolgende Änderung ergänzt nur diese Abschlussdokumentation. Der jeweils
aktuelle vollständige PR-Head und dessen erneute Prüfungen stehen im PR-Protokoll.
Contract-/Schema-Version: beide im integrierten STATUS offen.
Source-/Corpus-/Modellversion: nicht freigegeben, nicht geändert.
Betroffene Requirements: R01, R03, R04, R07, R18, R22, R40, R56, nur Vorbereitung.

## Ergebnis und konkrete Änderungen

`infra/cutover/README.md`: Betriebs-/Rollback-Runbook auf realem Inventar.
`LEGACY_MATRIX.csv`: zwölf live erfasste User-Systemd-Units plus sichtbare Lücken,
Zuständigkeiten und Voraussetzungen für ihre spätere kontrollierte Stilllegung.
`REHEARSAL.md`: vierzehn konkrete Staging-/Restore-/Ausfallproben, alle als noch
nicht durchgeführt markiert.
`RELEASE_MANIFEST.yaml`: unverändert unfreigegebene Vorlage aus dem Nutzerpaket,
keine erfundenen Image-, Schema-, Knowledge- oder Writerfreigaben.
`CR-S11-01.md`: nötige Owner-, CI-, Contract-, Job-, Consumer- und Freigabeübergaben.
`TEST_REPORT.md`: echte Befehle, Ergebnisse, Rotgegenproben und Prüfgrenzen.

`infra/cutover/runtime-audit/`: eigenständiges Rust-Inventarwerkzeug samt lokalem
Cargo.toml/Cargo.lock, drei CLI- und 25 Inventartests. Kein neuer produktiver
Workspace, kein Providerzugang, keine Datenpersistenz und keine Drittanbieter-
Dependency. Nur feste lesende Systemd-Aufrufe, keine Shellausführung. Ausgabe
enthält keine Environmentwerte, ExecStart-Argumente oder rohen Fehlertexte.
Timeout und Ausgabemenge pro Metadatenaufruf sind begrenzt. Exit 0 bestätigt nur
eine vollständige Metadatenaufnahme, niemals eine Cutoverfreigabe.

## Nachweise

| Prüfung | Befehl / Umgebung | Tatsächliches Ergebnis | Artefakt |
|---|---|---|---|
| Format | Cargo fmt des eigenständigen Auditmanifests | bestanden | infra/cutover/TEST_REPORT.md |
| Clippy/Compiler | alle Audit-Targets, locked/offline, Warnungen als Fehler | bestanden | infra/cutover/TEST_REPORT.md |
| Tests | Audit-Crate, Rust 1.97.1 | 28 bestanden, 0 fehlgeschlagen, 0 ignoriert | infra/cutover/TEST_REPORT.md |
| Rotkontrolle | separate bewusst falsche Library / leerer CLI-Stub | 25/25 bzw. 3/3 erwartungsgemäß rot | infra/cutover/TEST_REPORT.md |
| Releasebuild | nur Auditwerkzeug | bestanden | infra/cutover/TEST_REPORT.md |
| Bestehende GitHub-CI | Run 36010634455, Versuch 1; Head d4e911a; Checkout-Merge 5454d76466c5fe9e185e8d20ccde98108f24141f | completed/success; keine Audit-Tests im Root-CI | infra/cutover/TEST_REPORT.md |
| Live-Metadaten | manuell aufgerufener Snapshot im User-Systemd | 12 Units; Build-Daten und YouTube fehlgeschlagen, 5 Timer aktiv | infra/cutover/LEGACY_MATRIX.csv |
| Restore/Fencing/G4/G5/G6 | keine freigegebenen Voraussetzungen | nicht durchgeführt | infra/cutover/REHEARSAL.md |

## Folgen

Datenmigration/Kompatibilität/Wiederanlauf: keine Änderung. Das Tool liest weder
Anwendungsdatenbanken noch produktive Rohdaten und verändert keine Schedules.
Berechtigungen/Secrets/Egress: keine Änderungen, keine Secretdateien gelesen;
keine Provider-/Veröffentlichungsaufrufe. Metadaten bleiben als solche begrenzt.
Latenz/Ressourcen/Kosten: manuelle lokale Aufnahme ohne Providerkosten; keine
Performancezusage für den Brainkern. Kein permanenter Worker oder Timer ergänzt.
Vorhandene Funktionen: unverändert; fremde Worktrees/Änderungen nicht übernommen.
Python-/Legacyfreiheit: Audit selbst Rust ohne Dependencies. Der Brain-Gesamtbetrieb
ist damit ausdrücklich nicht als Python-/Legacyfrei abgenommen.

## Grenzen und Blocker

S000 hat G0 bis G6 offen gelassen und S11-Pfade noch nicht bestätigt. S11 erteilt
sich keine G1-/G2-/G4-/Produktivfreigabe. Es werden nur additive Vorbereitungs-
artefakte zur Integration angeboten, keine gemeinsamen Contracts verändert.

Fehlgeschlagene periodische Jobs und unterschiedliche Live-Checkoutstände müssen
durch ihre Besitzer aufgelöst werden. Auf der integrierten Basis benutzt der
Build-Wrapper einen Pythonexporter. Er wurde nicht eigenmächtig geändert. Keine
Root-Workspace-Komplettsuite oder volle Runtime-/Netzwerkprüfung ausgeführt.
Das neue Audittool wird noch nicht durch die bestehende Root-CI ausgeführt;
Integration ist im CR an S02/S10 übergeben.

Nicht durch den Snapshot geprüft: System-Units, Cron, andere Hosts, vollständige
Prozess-/Native-Abhängigkeiten, Parent-Cgroups, DB-Writergenerationen, Daten-
Vollständigkeit, Health/Readiness, Provider-/Consumerfunktion und Publikation.
Es gibt noch keine gebauten Brain-Releaseimages oder installierte neue
Deploymentkonfiguration. Sie benötigen die integrierten G1-/Releaseverträge.

## Übergabe an nächste Besitzer

S00 integriert nur den geprüften PR unter den geltenden Regeln, ordnet
`infra/cutover/**` zu und aktualisiert STATUS/GATES selbst. S02/S03 klären
Contracts, Schema, Release-/Fencingports und die CI-Einbindung. S04/S05 beheben
Job-/Zyklusblocker. S08/S09 liefern Readiness, Consumer- und Publishnachweise.
S10 ergänzt echte Last-/Sicherheitsgrenzen. Danach S11-Proben in isoliertem
Staging; produktiver G5-Wechsel ausschließlich nach G4 und Betreiberfreigabe.

## Integration durch S00

Merge-Commit: keiner.
Gate-/STATUS-Änderung: keine.
Freigegeben von: keine Cutoverfreigabe.

## Zusätzliche Angaben v1.0

Arbeitsmodus: vorbereitend, keine freigegebene nächste Betriebswelle.
Planrevision: Nutzerpaket v1.0 vom 24.09.2026.
Echte Integration oder Fixture: echte lesende User-Systemd-Metadatenaufnahme;
synthetische Audit-Tests, keine integrierte Brain-/Restoreprobe.
Owner/Folge-PRs: siehe CR-S11-01. Gemeinsame Owner-/Gate-/Schema-/CI-Dateien
wurden nicht geändert. Quellen-/Replay-/Publikationsrechte bleiben unverändert;
keine neuen Quelldaten oder Replays abgerufen/veröffentlicht.

## Fortsetzung von PR #26: Prozessaufnahme

Ausgangs-Head dieser Fortsetzung:
`5b11aa5e77fa0a513f5ea3b68b41227da6a7129e`.
Der genaue neue Head und seine nach dem Commit wiederholten Prüfungen stehen
im aktuellen PR-Prüfkommentar. Die vorstehenden 28-Test-/CI-Nachweise bleiben
historisch und werden nicht als Ergebnis für den neuen Head übernommen.

Neu: `runtime-audit runtime` prüft rekursiv Service-Prozessgruppen einschließlich
Untergruppen, Prozessidentität, namentlich erkennbare Python-/PyPy-Laufzeiten,
eingebettete Pythonbibliotheken und gelöschte laufende Dateien. Der reale Host
nutzt cgroup v1; v1/v2-Mounts und ihre Mitgliedschaften werden explizit unterschieden.
Der Collector bricht bei erkannten Wechseln oder fehlender Sichtbarkeit ab.
Kein Lesen von Environment, Kommandozeilen, Secretdateien oder Prozessspeicher.

Artefakte: `infra/cutover/RUNTIME_CHECK.md`, `RUNTIME_TEST_REPORT.md`,
`runtime-audit/src/runtime.rs`, ergänzte CLI und 24 neue Prozess-/Mounttests.
Endstand lokal: 54 Tests unter Rust 1.97.1 und Rust 1.75.0 bestanden;
Formatierung, Clippy mit Warnungen als Fehler sowie Audit-Releasebuild bestanden.
Alle 26 neuen Tests haben eine dokumentierte rote Gegenprobe. Es wurde keine
exhaustive Mutationstest- oder KI-Review-Qualitätsabnahme durchgeführt.

Echte zusätzliche Befunde: zwei laufende Serviceprozesse erfasst, fünf Services
ohne Prozessbeobachtung; ein als gelöscht markiertes laufendes Site-Binary und
in beiden laufenden Services gelöschte Datei-Mappings. In den zwei beobachteten
Prozessen keine erkannten Pythonhinweise, aber ausdrücklich kein Nachweis für
die fünf inaktiven/fehlgeschlagenen Jobs oder vollständige reguläre Zyklen.
Build-Daten und YouTube-Learning weiterhin fehlgeschlagen.

Next-owner: S04/S05 für die Jobfehler; S02/S10 für die weiterhin fehlende
Einbindung der Audit-Testserie in die gemeinsame CI; S00/S02/S03/S08 für die
bestehenden Contract-/Schema-/Fencing-/Readinessfreigaben. S11 muss vor späterem
Deploy die tatsächliche Releaseherkunft der laufenden Dateien bestimmen.
Keine neuen Betriebs-/Daten-/Keyänderungen, keine Gateänderung und kein
Produktivwechsel. S11 als Gesamtpaket bleibt blockiert, nicht fertig abgenommen.
