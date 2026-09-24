# S11: Betriebsaufnahme und Cutover-Vorbereitung

Status: **prepare_only; kein freigegebener Cutover**.
Basis: `3032651` auf `origin/main`, gelesen am 24.09.2026.
Contract-Version, DB-Schema-Version und aktives Knowledge-Release: nicht freigegeben.
G0 bis G6 stehen im integrierten `architecture/migration/GATES.csv` auf `offen`.

S11 beginnt hier mit dem Betriebs-/Rollback-Runbook auf Grundlage einer echten,
nur lesenden Betriebsaufnahme. Die verbindliche Pfadzuordnung ist noch offen;
`CR-S11-01.md` beantragt die Integration dieser vorbereitenden Dateien durch S00.
Es gibt keine Änderungen an Root-Workspace, Lockfile, fachlichen Contracts,
DB-Schema, produktiven Units, Timern, Secrets oder Consumer-Konfigurationen.

## Tatsächlich vorhandener Betrieb

Am 24.09.2026 gegen 15:51 CEST wurden im Benutzer-Systemd sieben einschlägige
Services und fünf Timer gelesen. Build-Daten und YouTube-Learning waren
fehlgeschlagen; die fünf Timer waren aktiv. Build-Site und `dl-knowledge` liefen.
Die drei übrigen One-Shot-Services waren inaktiv nach erfolgreichem letzten Lauf.
Das ist eine Momentaufnahme, keine Funktions-, Datenvollständigkeits- oder
Pythonfreiheitsabnahme. Details stehen in `LEGACY_MATRIX.csv`.

Die eingesetzten Arbeitsverzeichnisse sind nicht einheitlich: insbesondere läuft
der YouTube-Job aus einem separaten Live-Worktree. Der uncommittete kanonische
Checkout enthält weitere Änderungen. Weder `main` noch ein Git-Branchname allein
belegt deshalb das laufende Binary oder ein kompatibles Rückfallrelease.

## Reihenfolge und feste Stopps

1. **S00/S02/S03:** G1 integrieren, reale Pfade zuordnen und Contract-/Schema-
   Versionen festlegen. S01 liefert das vollständige Host-/Consumerinventar.
2. **S03 bis S10:** G2/G3 belegen, echte Stagingumgebung und Testdaten freigeben.
   Dann die Proben aus `REHEARSAL.md` durchführen und protokollieren.
3. **S10/S11/S00:** G4 einschließlich Lastgrenzen, Restore, Rückweg und kompletter
   Rust-Laufzeitzyklen abnehmen. `RELEASE_MANIFEST.yaml` mit realen Artefakten und
   revisionsgebundener Betreiberfreigabe vervollständigen.
4. **S11 und Betreiber:** Nur im ausdrücklich freigegebenen G5-Fenster den Ablauf
   unten durchführen. Danach G6 und das separat freigegebene Legacy-Ende.

Keine grüne CI, kein fertiger Branch und kein erfolgreicher Inventaraufruf ersetzt
G4 oder die Betriebsfreigabe. Die PR-first-Entscheidung verlangt einen offenen PR.
Ein automatischer Merge oder Deploy wird in diesem Auftrag nicht freigegeben.

## Runbook für das später freigegebene Fenster

Vor jeder Stufe erfassen: zuständige Person, Uhrzeit, exakter Release-Commit,
Image-/Binaryhash, Schema, Knowledge-Release, Policy/Tombstone-Checkpoint,
Quellenwatermarks, Testbericht und Freigabe. Fehlende Angaben bedeuten STOPP.
Produktionsbefehle zum Fencing oder Aktivieren werden erst aus den integrierten
S03/S04-Verträgen übernommen; hier werden keine Ersatz-CLI oder Locktabellen
erfunden.

### Vorbereiteter Release und Rückweg

S11 baut erst den integrierten, freigegebenen Stand mit fixierter Toolchain und
Lockfile. S02 liefert das echte Binary-/Entrypointinventar und Native-Abhängigkeiten.
Der Release muss API, Worker, Ingest, Wiki, Reparse, Population/Learning, Rebuild
und freigegebene Adapter abdecken. Secret-Referenzen statt Secretwerte verwenden.
Ressourcenbudgets, Startprüfung und Shutdown-Fristen stammen aus S10-Messungen.
Keine erfundenen Health-/Readiness-URLs: S08 muss deren tatsächlichen Vertrag und
die Bindung an die aktive Knowledge-/Policygeneration liefern.

Der vorherige Stand muss mit dem aktuellen Schema und Datenrelease getestet
lesbar sein. Das vorhandene mutable Live-Verzeichnis ist kein Rollbackartefakt.
Backuphash, Restorebericht auf leerem Ziel, Index-/Replikagenerationen, aktuelle
ACL-Sperren und Tombstones werden vor dem Fenster verifiziert. Ohne belegten
Rückweg bleibt der Betrieb unverändert.

### Shadow und Canary

Zuerst interner Read-only-Canary, danach Twitch, dann Discord/MCP/CLI/Web sowie
Docs-/Buildpublishing aus dem vollständigen Consumerinventar. Shadowaufrufe dürfen
keine Nutzerschreibvorgänge, Lernereignisse, Feedbacks oder Publikationen doppeln.
Zusätzliche Provideraufrufe nur innerhalb des ausdrücklich bewilligten Budgets;
ohne Budget keine Verdopplung. Jeder Canary protokolliert Coverage, Fehler,
Latenzen, Quellenfrische und die tatsächliche Code-/Knowledge-/Policyversion.
Ein Read-only-Pfad, der intern Schreibjobs auslöst, ist nicht für Shadow geeignet.

### Writer-Wechsel

- Alle alten Schedules und manuellen Startwege aus dem bestätigten Inventar
  sperren. Bereits laufende Jobs drainen. **Timer stoppen ist kein Writer-Fencing.**
- Alten Writer am gemeinsamen Store über den von S03/S04 implementierten
  Fencingmechanismus sperren; auch verzögerte/neu gestartete alte Worker müssen
  mit ihrer alten Generation beim Write abgewiesen werden.
- Letzten akzeptierten Offset pro Feed inklusive Wiki, API/Git, Replay, Learning
  und Publishing festhalten. Delta samt Deletes und ACL-Änderungen abgleichen;
  fehlende Quelle/Partition, Lücke oder ungeklärte Dublette stoppt die Umschaltung.
- Neue Knowledge-/Indexgeneration und Rechteprüfungen validieren. Erst danach
  den neuen Writer mit der freigegebenen Generation aktivieren. Genau eine
  wirksame Writergeneration pro vereinbartem Schreibbereich nachweisen, nicht
  einfach genau einen Unix-Prozess zählen.
- Abgelehnte Altwrites, bestätigte Neuwrites und idempotente Wiederholung
  nachweisen. Erst danach Canary schrittweise ausweiten und neue Schedules
  aktivieren. Kein gleichzeitiger alter/neuer Publishjob.

### Abbruch und Rollback

Sofortiger Stopp bei Scope-/Secret-Leak, ungeklärtem Datenverlust, zwei wirksamen
Writergenerationen, veralteten ACL-/Tombstoneprüfungen, kritischer fachlicher
Regression oder Überschreitung der vorher freigegebenen S10-Betriebsgrenzen.
Keine Grenzwerte während des Tests passend ändern. Betreiber und zuständige
Modulbesitzer werden im Betriebsprotokoll benannt.

Zuerst neuen Schreibzugang und Schedules fencen/drainen; seinen letzten Offset
festhalten. Writes seit dem Umschaltpunkt exportieren oder nachweisbar in das
kompatible Rückfallrelease replayen. Bei fehlender Kompatibilität oder unsicherer
Deltaübernahme **Schreiben angehalten lassen** und nur nachweisbar sichere
Read-only-Verfügbarkeit anbieten. Kein blindes Rückkopieren eines alten Backups.
Aktuelle Löschungen, ACL-Sperren und Secret-Rotation niemals zurückdrehen.
Rückfallwriter erst nach erneuter Rechte-/Delta-/Fencingprüfung aktivieren.

### Betriebskontrolle und G6

Nach G5 vollständigen regulären Ingest-, Wiki-/Schema-Update-, Replay-Reparse-,
Learning-/Population- und Rebuildzyklus ohne notwendigen Python-/Legacypfad
nachweisen. Nur eine vorgefüllte API zu starten genügt nicht. Runtime, Netzwerk,
CI, Native-Abhängigkeiten, Schedules, direkte Consumer-Modellpfade und
Publikationsberechtigungen erneut prüfen. Legacy-Endpunkte in der isolierten
Probe sperren und alle Pflichtpfade erneut ausführen.

Erst nach bestandenem Betrieb und Ablauf des **explizit festgelegten**
Rollbackfensters alte Services/Deployments und ausschließlich ihnen zugeordnete
Keys kontrolliert stilllegen. Geteilte Secrets und fremde Dienste erhalten.
Repoarchivierung, Rohdatenlöschung und Backupvernichtung benötigen jeweils eine
separate Freigabe; nichts davon ist durch diesen Auftrag genehmigt.

## Nur lesender Betriebscheck

`runtime-audit/` ist ein eigenständiges Rust-Werkzeug ohne Drittanbieter-
Dependencies und ohne Einbindung in den produktiven Workspace. Es erfasst nur
User-Systemd-Metadaten der `deadlock-brain*.service/.timer`-Familie und
`dl-knowledge.service`, einschließlich installierter, derzeit nicht geladener
Units. Es startet und stoppt nichts, liest keine Secretdateien und gibt weder
Environment noch ExecStart-Argumente aus. Auch Fehlermeldungen des Unterprozesses
werden nicht ungefiltert weitergereicht.

```sh
cargo fmt --manifest-path infra/cutover/runtime-audit/Cargo.toml -- --check
cargo clippy --manifest-path infra/cutover/runtime-audit/Cargo.toml --all-targets --locked --offline -- -D warnings
cargo test --manifest-path infra/cutover/runtime-audit/Cargo.toml --locked --offline
cargo run --manifest-path infra/cutover/runtime-audit/Cargo.toml --locked --offline -- snapshot
```

Exit 0 bedeutet ausschließlich: Metadatenaufnahme vollständig. Ein erkannter
fehlgeschlagener Dienst kann in einer vollständig gelesenen Aufnahme stehen.
Exit 1 bedeutet Erfassungs-/Validierungsfehler; Exit 2 ungültige Bedienung.
Jeder Bericht sagt ausdrücklich `cutover_authorized=false`. Shell-Entrypoints
sind ungeklärt, native Entrypoints kein Beweis für Pythonfreiheit. Keine
System-Units, Cronjobs, anderen Hosts, DB-Leases, Netzwerkpfade oder vollständigen
Prozessbäume werden durch diesen kleinen Check abgenommen. Seine Tests gehören
nicht automatisch zum bestehenden Root-CI-Job; S02 erhält dafür einen CR.

## Erweiterte Prozessaufnahme

Der zusätzliche Befehl `runtime` erfasst Service-Prozessgruppen, Unterprozesse,
Python-/PyPy-Kandidaten, namentlich erkennbare eingebettete Pythonbibliotheken
und gelöschte ausführbare Dateien bzw. Datei-Mappings. Er unterstützt den realen
cgroup-v1-Systemd-Host und vollständige cgroup-v2-Mounts. Details, Grenzen und die
neuen Live-Befunde stehen in [RUNTIME_CHECK.md](RUNTIME_CHECK.md).

```sh
cargo run --manifest-path infra/cutover/runtime-audit/Cargo.toml --release --locked --offline -- runtime
```

Die vorstehenden Grenzen des kleinen Metadaten-Snapshots gelten für `snapshot`.
Auch der erweiterte Prozessmodus ist keine vollständige Runtime-/Zyklusabnahme
und verändert keinen Dienst. Die aktuelle Testserie umfasst 54 Tests; der
Folgebericht steht in [RUNTIME_TEST_REPORT.md](RUNTIME_TEST_REPORT.md).
