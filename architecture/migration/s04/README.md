# S04: Ingestion und Feeder – Startprüfung

Stand: 24. September 2026. Plan: 1.0.

**Ergebnis: dokumentarische Vorbereitung; Implementierung blockiert.** Dies ist keine G0-/G1-Freigabe, kein implementierter Worker und keine bestandene Ingestion-Abnahme. Auftrag dieser Arbeit: ausschließlich S04. Die Vorprüfung ersetzt weder S02 noch S03 oder S10.

## Geprüfte Basis und Grenzen

Integrierter, frisch von `origin/main` gelesener Basis-Commit: `c00fc8935048bf490c1e4790f7c6195864ad49e2`. Eigener Branch: `migration/s04-ingestion-preparation-20260924`.

`../STATUS.md` lässt Contract-Version, DB-Schema-Version, G0 und G1 offen. `../GATES.csv` bestätigt das. Der Paketauftrag erlaubt S04-Implementierung erst nach G1. Das Planpaket liegt auf dieser integrierten Basis nur als `Deadlock-Brain_Rust-Daten_Planpaket_v1.0.zip` im Repository-Root; das entpackte Verzeichnis unter `architecture/migration/` ist dort noch nicht integriert. Der S04-Auftrag ist das ZIP-Mitglied `deadlock-brain-rust-planpaket-v1.0/chats/04_INGESTION_FEEDER.md`. Diese Vorgaben sind Plananforderungen, keine bereits implementierten Rust-Ports. Die koordinierte Paketablage bleibt Aufgabe von 00; S04 kopiert nicht ungefragt das Gesamtpaket.

SHA256 des vom Nutzer bereitgestellten ZIPs: `945be6983ff0235eb0ec36b451f9613ea567e0962ca5658364e8665980177716`. SHA256 des S04-Mitglieds: `b1278465416fa0acc9bd5f0a3142ac113459d96cf07c1fb89a6dbfeb1c7588a1`. Der Abgleich mit dem versionierten ZIP wird in der Übergabe protokolliert.

PR #15 enthält S01-Inventar auf `76205b5f8e110fa1741f73cd29274a6600828498`, ist bei dieser Prüfung aber offen und basiert auf `dependency/s00-planpaket-20260924`. Seine Aussagen werden nicht als integrierte Freigabe oder aktuelle Runtimeprüfung übernommen. Insbesondere ist dessen älterer Befund eines fehlenden STATUS durch die hier gelesene Koordinationsbasis überholt. G0 bleibt laut aktueller Koordination dennoch offen.

Geändert werden ausschließlich diese neuen Dateien:

- `architecture/migration/s04/README.md`
- `architecture/migration/s04/SOURCE_JOB_INVENTORY.csv`
- `architecture/migration/s04/CONTRACT_REQUESTS.md`
- `architecture/migration/s04/PILOT_ACCEPTANCE.md`
- `architecture/migration/handoffs/04-ingestion-feeder.md`

Gemeinsamer STATUS, Gates, Ownerregister, ADRs, Verträge, Cargo-Manifeste, Lockfile, SQL, Produktcode, Dienste und produktive Daten bleiben unverändert. Die Dokumente sind Vorschläge zur Integration durch Chat 00, keine selbst erteilte Pfadfreigabe.

## Wiederverwendbarer Bestand und konkrete Lücken

Die Belege beziehen sich ausschließlich auf den oben fixierten Git-Stand. Zeilenangaben werden gegen diesen Commit geprüft. Vorhandene Funktionen werden nicht als neue S04-Leistung ausgegeben.

| Befund | Beleg im Basis-Commit | Konsequenz für S04 |
|---|---|---|
| Rust-Quellenmodule für Assets, Deadlock API, Git-Daten, Forum, Sheet, Patchnotes, Reddit, Statlocker und Wiki sind exportiert. | `rust/crates/dbrain-sources/src/lib.rs:7–37` | Bestehende Module nutzen; keine zusätzlichen gleichartigen Adapter oder kosmetischen Crate-Umbenennungen. Fachliche Parser bleiben bei 12/13/14. |
| SourceStore schreibt Run-Status, Raw-Dateien, Dokumente und Entity-Snapshots. | `rust/crates/dbrain-sources/src/store.rs:40–222` | Es gibt einen Ausgangspunkt, aber noch keinen hier belegten gemeinsamen Lease-/Checkpoint-/Ack-Vertrag. Run-Summary ist kein solcher Nachweis. |
| Bei identischem `(source, external_id, content_hash)` erfolgt `DO NOTHING`; anschließend wird die vorhandene ID gelesen. Titel, URL und Metadaten werden auf diesem Pfad nicht aktualisiert. | `rust/crates/dbrain-sources/src/store.rs:95–145` | Reine Metadaten-/Policyänderungen dürfen nicht als unverändert verschwinden. Das ist eine konkrete Schnittstellenlücke, kein Nachweis eines tatsächlich geschehenen Rechtelecks. |
| Raw-Dateien werden direkt mit `fs::write` angelegt; existierende Pfade werden nicht erneut auf vollständigen Inhalt geprüft. | `rust/crates/dbrain-sources/src/store.rs:76–92` | Abbruch während des Schreibens und Wiederanlauf müssen vor einer Durable-Raw-Zusage geprüft werden. Keine behauptete atomare Raw-/DB-Transaktion. |
| Dokument, Snapshots und Run-Abschluss sind getrennte Aufrufe über den Pool. | `rust/crates/dbrain-sources/src/store.rs:95–222,241–257` | Persistenzgrenze und Wiederholungssicherheit gemeinsam mit 03 festlegen. Weder Exactly-once noch crashsicheres Ack ist dadurch bereits belegt. |
| HTTP hat Versuchsgrenze, Timeout und Status-Retries; der Cachepfad wird nur aus der URL abgeleitet. | `rust/crates/deadlock-brain-core/src/http.rs:18–51,152–240,243–279` | Private/Session-Ingestion erst mit freigegebenem Auth-/Policy-/Cache-Vertrag. Nicht bloß andere Auth-Header bei gleichem Cacheverzeichnis ergänzen. Core nicht eigenmächtig ändern. |
| Git-Import führt die Importfunktionen aus und weist `commit_changed` erst in der Ergebniszusammenfassung aus. | `rust/crates/dbrain-sources/src/deadlock_data.rs:49–127` | Gleicher Commit ist hier kein vorgeschalteter No-op. Reparse-/Dependency-/Policyversionen mit 13 berücksichtigen; Hash-Deduplikation ist nicht gleich Arbeitsvermeidung. |
| Wiki-Refresh enthält Lock, Staging, Validierung und Aktivierung mit Rücknahme bei Statusfehler. | `rust/crates/deadlock-brain/src/wiki_refresh.rs:60–196` | Vorhandenen Ablauf erhalten und mit 03/12 in den gemeinsamen Releasevertrag einordnen; kein zweiter unabhängiger Knowledge-Zeiger. Kein neuer Crash-/Rebuildnachweis durch bloßes Lesen. |
| Versionierter Build-Data-Timer startet einen Shell-Wrapper. Dessen Standard-Secrets-Pfad ruft Python auf. | `service/systemd/deadlock-brain-build-data.service:7–11`; `service/systemd/deadlock-brain-build-data.timer:4–6`; `scripts/run_build_data_with_infisical.sh:10–14,23–55` | Pythonfreiheit des Betriebs ist nicht erreicht oder hier nachgewiesen. Dies ist ein Codebefund, keine Feststellung zum aktuell installierten Dienst. Wrapper/Writer-Cutover gehört in die abgestimmte Arbeit mit 11; nicht in diesem PR ändern. |

Das vorhandene `python_json_spacing_and_ascii` in Rust ist hingegen allein aufgrund seines Namens keine Python-Laufzeitabhängigkeit. Entscheidend sind tatsächlich gestartete Prozesse.

## Voraussetzungen für die Implementierung

Chat 00 integriert G0 und anschließend G1 mit einem neuen festen Basis-Commit. 02/03 liefern die versionierten Source-/Policy-/Store-/Checkpoint-/Publish-Verträge und isolierte Testdatenbank-Anleitung. 00 löst die Pfadüberschneidungen: Das aktuelle Register reserviert `dbrain-sources/**` zugleich für 04/13 und Teile für 12/14; die CLI liegt ebenfalls bei 04/09. Diese Reservierungen sind keine eindeutige Freigabe für parallele Edits.

Die kleinsten konkreten Anforderungen stehen in `CONTRACT_REQUESTS.md`. `PILOT_ACCEPTANCE.md` beschreibt die anschließend zu implementierenden Prüfungen, nicht bereits ausgeführte Tests. Es werden absichtlich keine eigenen Traits, SQL-Migrationen, JSON-Wireformate oder Ersatz-Checkpointdateien erfunden.

## Versorgungsstatus und Nachweise

`SOURCE_JOB_INVENTORY.csv` trennt vorhandene Codepfade, nur vorgeschlagene Quellen und nicht geprüfte Betriebseigenschaften. Rechte, Datenvollständigkeit, aktiver Writer und Versorgung sind nicht aus Modulnamen ableitbar. Insbesondere sind Dokument-/GitHub-Ereignis-/Session-Feeder in diesem S04-Lauf nicht bis zu ihrer realen Implementierung verifiziert; kein vorhandener Legacy-Feeder wird als bereits portiert behauptet.

Kein Quellenabruf, Datenbankzugriff, Provideraufruf, Reembedding, Rebuild, Live-Lasttest, Dienstneustart, Merge oder Deploy wurde ausgeführt. Für diese rein dokumentarische Änderung werden Inhalts-/Pfad-/Diffprüfungen protokolliert; Rust-Tests bleiben ausdrücklich nicht ausgeführt. Den genauen Prüfstand und die nächsten Besitzer nennt `../handoffs/04-ingestion-feeder.md`.
