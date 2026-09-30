# B1: unabhängige statische Harness-Abnahme

Datum: 2026-09-30. fertig: J (Review abgeschlossen). Fix: J (bestehende Aufrufer nachziehen).

**Urteil: BLOCK für die vollständige B1-Integration, ein konkreter Argumentbefund.** Die fünf direkt aufgerufenen Runner erfüllen statisch den neuen Cache-/Compilervertrag. Ihre bestehenden Aufrufer wurden aber nicht auf das neue Pflichtargument umgestellt. Kein Cargo-, Harness-, DB-, Modell-, Dienst- oder Lastlauf wurde ausgeführt. Keine automatische Folgeausführung.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

## Bindung und tatsächlich ausgeführte Prüfung

- Quellworktree ausschließlich als Git-Objektquelle: `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`. Geprüft wurden eingefrorene Blobs und der Diff `ca4a8f236a75ba89ce60d2140c735acd5d1f5bee..0a6e09517770c0a016594c3580ed34bd20f96d37`, nicht HEAD oder Arbeitsdateien. Genau fünf vorhandene Shellrunner, 85 Einfügungen und 35 Löschungen. Unter `rust/` kein Delta, damit auch kein Rust-/Lock-/Testquelldelta. Keine B2-Importerabnahme.
- Eigene Berichtssenke: `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`, sauberer Ausgangshead `ae2abe1a954eca03d863b1370df4a03496eac9b0`.
- Nach Graphify-Abfrage vollständige fünf Blobfassungen, Diff und unmittelbar betroffene Aufrufer gelesen. Die Graphantwort lieferte keine verlässlichen Runnerstellen; maßgeblich sind die eingefrorenen Git-Quellen. Gezielte Zwillingssuche unter `scripts/`, `rust/` und `.github/`, keine neue Vollinventur.
- Jeder geprüfte Blob wurde unverändert über stdin an `/bin/bash -n` übergeben: fünfmal Exit 0. Kein Sourcen, kein `--help`, keine Argumentprobe mit ausgeführtem Runner. Die nachstehenden Argumentfolgen sind statisch hergeleitet, nicht gemessene Harness-Exits.

| Geprüfter Blob unter `scripts/` | SHA256 | Syntax |
| --- | --- | --- |
| `run_local_pilot.sh` | `93271dcb114f3485d420e22708b254da73d073b33e482e965e076277b62d1655` | Exit 0 |
| `test_brain_core_postgres.sh` | `5af6333ba58d29af5b7531d658b6c73c81aca309e286ef18fdd122a0fb7d4911` | Exit 0 |
| `test_brain_serve.sh` | `c96e4127cc0b7aa1c4a1e685adf20314a9937226b2690161be05d02ef0bd21e9` | Exit 0 |
| `test_brain_storage_upgrade.sh` | `0f1925a813f5e8ce041c290cc273ec502ba30b4eec095c8bb728c8ed2fcbdae5` | Exit 0 |
| `test_wiki_runtime.sh` | `0801823fe138d711acd8255db1944504500dd0268d0a840e091b7665a16cda66` | Exit 0 |

## B1-R1: Pflichtargument erreicht bestehende Eintrittspfade nicht

**Priorität P2, Argumentweitergabe/Integration.** Änderungsanker `scripts/test_brain_serve.sh:5-13` und `scripts/test_brain_core_postgres.sh:5-13`; derselbe neue Vertrag gilt für Upgrade und Wiki. Fehlende Argumente werden jetzt korrekt mit Exit 2 vor `pg_config`, Scratchanlage und `initdb` abgewiesen. Die bestehenden Aufrufer liefern weiterhin genau diesen ungültigen Aufruf:

| Vorhandener Eintrittspfad bei `0a6e095` | Konkrete Folge |
| --- | --- |
| `scripts/run_isolated_load.sh:4` | `exec` ruft Serve ohne Argument auf und verwirft auch ein dem Wrapper ausdrücklich übergebenes Cacheargument. Daher immer Usage/Exit 2 statt Lastprüfung. |
| `scripts/check_brain_core.sh:38-46` | Beide Zweige `postgres` und `all` rufen Core-PG ohne Cache auf. Der PG-Check liefert Exit 2, der Sammelrunner protokolliert den Fehler und endet mit Exit 1. Ein zusätzliches äußeres Argument wird nicht weitergereicht. |
| `.github/workflows/brain-serve-c1.yml:75` | Serve-Aufruf ohne Cache, daher Usage/Exit 2, sofern der Schritt erreicht wird. |
| `.github/workflows/rust-core-verification.yml:69,76,85` | Core-PG-, Upgrade- und Serve-Schritt jeweils ohne Cache; keiner erreicht seinen PG-/Prozessvertrag. |
| `.github/workflows/wiki-runtime-pilot.yml:48` | Wiki-Runner ohne Cache; Scratch-Staging und CLI-Prüfung finden nicht statt. |

Die Zwillingssuche bestätigt dieselbe Ursache an allen genannten aktiven Aufrufstellen. Die bereits deaktivierten historischen Runner bleiben ausdrücklich deaktiviert und sind kein zusätzlicher Laufpfad. Das Reproduktionsbeispiel `rust/crates/brain-storage/tests/fixtures/storage_v1/README.md:22` zeigt außerdem noch nur `/path/to/cargo`; dieser einzelne Dateipfad wird jetzt als Targetverzeichnis interpretiert und abgewiesen.

**Enge Korrektur:** Den expliziten vorhandenen absoluten Cache durch die bestehenden Wrapper bis zu den Runnern weiterreichen und die direkten Workflow-/Reproduktionsaufrufe anpassen. Kein Defaultcache, kein neuer Runner, keine Testentfernung und kein Wiederöffnen deaktivierter Pfade. Bei den Workflows den dort tatsächlich vorhandenen Cache verwenden, nicht den hostlokalen Worktreepfad kopieren. Die nötigen Aufruferdateien liegen außerhalb des ursprünglichen Fünfdatei-Autorenscopes; den Anschlussfix gezielt um diese Stellen erweitern, nicht still im Review ändern.

Dies ist ein nachgewiesener Bruch bestehender Prüfeinstiege, **kein GitHub-Actions-Merge-Gate**. Ein roter Actions-Lauf wird nicht zur Mergebedingung erklärt. Die unten genannten direkten Aufrufe umgehen keinen Guard, sondern bedienen den neuen Vertrag korrekt. Sie bleiben als spätere Einzelprüfungen technisch vorbereitet, ohne den Anschlussbefund zu schließen.

## Unauffällige geprüfte B1-Verträge

1. **Argumente vor Wirkung:** Pilot `:7-15`, Core/Serve `:5-13`, Wiki `:7-15` verlangen genau ein Argument. Upgrade `:6-24` verlangt eines oder zwei, erhält bei zwei Argumenten die Reihenfolge `[cargo-executable] <target-dir>` und prüft das Programm mit `command -v`. Leer, relativ oder nicht vorhandenes Verzeichnis wird vor Clustererzeugung abgewiesen. Alle Pfade sind gequotet. Es gibt weder Target-Fallback noch Target-`mkdir`. Die Guards prüfen Verzeichnisexistenz, nicht den speziellen Hostpfad oder dessen Schreibbarkeit; den genehmigten Cache bindet der konkrete spätere Aufruf.
2. **Compilervertrag:** Alle 14 statischen Cargo-Aufrufstellen verwenden `+1.97.1`, `--locked --offline --jobs 1 --target-dir "$TARGET_DIR"`: Serve 6, Core 1, Upgrade 1, Wiki 4, Pilot 2 im Phasenrumpf. Kein versteckter paralleler Cargoaufruf hinzugefügt. Die beiden Pilotstellen laufen je Phase nacheinander, standardmäßig also 8 Cargoaufrufe über vier Phasen. Bei Vorbaufehler wird die Phase als fehlgeschlagen vermerkt und deren Testaufruf ausgelassen. Das Upgrade-Zusatzargument ist für den Rustup-Cargo-Einstieg zu verwenden, der `+1.97.1` verarbeitet; die späteren Befehle binden ausdrücklich `/home/nathanael/.cargo/bin/cargo`, keinen direkten Toolchain-Cargo ohne Rustup-Dispatch.
3. **Isolation und Identität unverändert:** Core/Serve/Upgrade/Wiki behalten `env -i`, Scratch-HOME, ausdrückliche Cargo-/Rustup-Homes und synthetische interne Testkoordinaten. Marker `brain-c11-scratch-v1` und `C5_SCRATCH_ONLY`, Peer-Zuordnungen, synthetische SCRAM-Fixture, Unix-Socketbindung, Rollen und DB-Namen bleiben im Diff unverändert. Der Pilot behält seinen bisherigen lokalen Dokument-/Scratchpfad und die vorhandenen internen Variablen; er ist nicht nachträglich zu einem `env -i`-Runner geworden. Keine neue produktive ENV-Konfiguration oder echten Credentials eingeführt.
4. **Tests, Status und Cleanup unverändert:** Alle Filter und `--ignored`-Grenzen bleiben bestehen; kein Workspace-`--ignored`. Kein Budget oder `max_connections` erhöht. Core/Serve/Wiki stoppen ihren eigenen Cluster und behalten bei Fehler ihre Scratchbelege; Upgrade stoppt und entfernt sein Verzeichnis auch bei Testfehler, deshalb äußeren vollständigen Log sichern. Pilot erhält Phasenstatus, Clusterneustart und Stop-Trap sowie die bestehenden dauerhaften Scratch-/Berichtspfade. Es wurden nur Cache-/Argument-/Cargozeilen geändert, keine zusätzliche Härtung dieser alten Pilotpfade behauptet.
5. **Kindprogramme und Last:** Die geprüften Rust-Prozessstarts verwenden `env!("CARGO_BIN_EXE_...")`, nicht einen hart codierten `ROOT/rust/target`-Binarypfad: Serve `tests/common/mod.rs:57-64`, Importer `tests/cli_scratch.rs:31,224`, Migration `tests/storage_upgrade.rs:198`. Der Serve-Kindprozess leert seine Umgebung. `brain-serve/tests/process_e2e.rs:692-695` enthält weiterhin 600 Anfragen je 8/16/32 Worker. Ein Cargojob begrenzt weder diese Last noch alle Test-/Linkerthreads oder den RAMverbrauch. Cargo offline ist kein Prozess-Egress-Sandboxbeweis.

## Exakte spätere Direktaufrufe und tatsächliche Laufklassen

**Nicht ausgeführt und nicht mit diesem Bericht zugeteilt.** Voraussetzung ist ein sauberer, revisionsgebundener Quellstand mit den geprüften Runnerblobs und separat geprüftem Ruststand. Der Autor arbeitet im genannten Quellworktree an B2 weiter; die folgenden Pfade dürfen daher nicht ungeprüft gegen den beweglichen Arbeitsbaum gestartet werden. Der Integrator muss den tatsächlich zugeteilten Stand und Diff vor dem Lauf binden. Kein Checkout, Build oder Lauf durch den Reviewer.

Für alle fünf Aufrufe: unprivilegierte vorhandene Identität, PostgreSQL-Werkzeuge, bereits installierte Rustup-Toolchain 1.97.1, vorhandenes Cargo-Home `/home/nathanael/.cargo`, vorhandener Targetcache `/home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target`. Keine Umleitung auf neue Caches, kein Fetch, kein nachgelagerter automatischer Lauf. Die vorhandenen Test-Overrides dürfen diese konkrete Werkzeugbindung nicht abändern. Vollständige Ausgabe und echter Exit müssen durch den zugeteilten Aufrufer erhalten werden, besonders beim Upgrade.

### Core-PG

```bash
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_core_postgres.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Ein Cargoaufruf, gezielter Atomicity-/Fencing-/Release-/Restart-Test. Echter privater Peer-PG unter temporärem Scratchpfad, nur Unix-Socket, Port 55439, `max_connections=12`, `shared_buffers=16MB`. DB-Schreiben in der Fixture, kein Produktionszugriff.

### Storage-Upgrade

```bash
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_storage_upgrade.sh /home/nathanael/.cargo/bin/cargo /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Ein Cargoaufruf, echter Migrationsprozess, v1-Upgrade, Dump/Restore und Least-Privilege in `/tmp/brain-c11.*`. Nur Unix-Socket, Port 55441, `max_connections=24`, `shared_buffers=16MB`. Dies ist der bestehende historische Fixture-Restore, kein vollständiger Produktionsrückweg einschließlich späterer Datenänderungen.

### Serve

```bash
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_serve.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Sechs serielle Cargoaufrufe: Import-CLI, Import-Library, Serve-E2E, synthetischer SCRAM-Nachweis und zwei Konfigurations-/Argument-Prozessfälle. Privater Peer-/SCRAM-PG, Unix-Socket Port 55439, `max_connections=12`, `shared_buffers=16MB`; echte lokale Importer-/Serveprozesse und Loopback-Providerfixture. **Enthält fest 1.800 Lastanfragen in drei Blöcken zu 600 bei 8/16/32 Workern, zusätzlich zu den funktionalen Anfragen.** Nicht als leichter Smoke zuteilen. Kein externer Modell- oder Produktionsdienststart.

### Wiki-Runtime

```bash
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_wiki_runtime.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Vier serielle Cargoaufrufe: reguläre Tests der vier angegebenen Pakete, gezielter Scratch-Wiki-Test sowie CLI `plan` und `stage`. Privater Peer-PG `/tmp/brain-c5-pg.*`, Unix-Socket Port 55441, `max_connections=12`, `shared_buffers=16MB`. Lokale Offline-Fixtures, DB-Schreiben und reale CLI-Prozesse, kein Live-Wiki-Capture.

### Lokaler Pilot

```bash
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/run_local_pilot.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Dieser Befehl ist nur mit dem **bereits ausdrücklich gebundenen und genehmigten** lokalen Dokumentsetup ausführbar: vorhandenes `BRAIN_PILOT_ROOT` mit `public/` und `internal/` außerhalb des Repos sowie die geprüften vorhandenen Bericht-/Scratchziele. Kein konkreter freigegebener Corpuspfad liegt diesem Reviewauftrag bei; keinen erfinden und keine echte Konfiguration auslesen. Das ist eine Pilot-Voraussetzung, kein weiterer B1-Codebefund und keine Forderung nach neuer produktiver ENV-Konfiguration.

Vier Standardphasen, je Vorbau und Test, insgesamt acht serielle Cargoaufrufe; die vorhandene optionale Diagnosephase würde zwei weitere hinzufügen und gehört nicht zum Vierphasenaufruf. Eigener Cluster `ROOT/.core-test-pg`, Unix-Socket Port 55439, `max_connections=12`, `shared_buffers=16MB`, bestehende lokale Trust-Authentisierung. Bestehenden laufenden Cluster lehnt der Runner ab. Er erzeugt beziehungsweise verwendet seinen Scratchcluster, ersetzt seinen bisherigen Berichtspfad und erstellt die Pilotdatenbanken neu, einschließlich sofortigem Scratch-PG-Stop/Neustart zwischen Phasen. Daher diese Ziele vorher konkret binden; keine pauschale Gleichsetzung mit den jeweils neuen Peer-Tempclustern. Reale Serveprozesse mit lokalem Providerfixture, kein externer Modelllauf. Bericht und Scratchdaten bleiben nach Clusterstop erhalten.

## Freigabepunkt

Der ursprüngliche interne Cache-/Zweijob-/Pilot-Offlinebefund ist an den fünf direkten Runnern statisch behoben. Die vollständige Quellintegration bleibt wegen **B1-R1** offen. Nächste konkrete Aktion: den bestehenden Autor mit der Argumentweitergabe an den oben belegten Aufrufstellen beauftragen. Danach nur dieses Anschlussdelta unabhängig nachprüfen. Kein neuer Modellprozess, keine allgemeine Deployfrage und keine Wiederholung des Workspace-Tests allein für diese statische Abnahme.
