# S13: Befunde an vorhandenen Quellenadaptern

Prüfbasis: `30326512568b7370524956839100462ba71bdb92` auf `origin/main`. Stand der Quellenbeobachtung: 24. September 2026. Dies ist eine Startprüfung, keine Implementierung oder Betriebsabnahme. Fremde Worktrees und uncommittete Änderungen wurden nicht übernommen.

## S13-B01: Implementierungsfreigabe und gemeinsame Verträge fehlen

`architecture/migration/STATUS.md` führt G0 und G1 als offen. Contract- und DB-Schema-Version sind offen. `PFAD_OWNER.csv` reserviert `dbrain-sources/**` gleichzeitig für 04 und 13, Teile von `deadlock_api*` für 14; eine konkrete Schreibaufteilung fehlt. Die Suche nach `SourceRecordV2`, `ExternalSourceAdapter`, `SchemaChange`, `ReconciliationCase`, `derivation_family` und `parser_family` in den Rust-Crates der Prüfbasis liefert keine Treffer. Das ist ein Befund für diese Basis, keine Aussage über alle unintegrierten Branches.

**Folge:** Keine eigene Runtime-Vertragsdefinition, keine SQL-Migration, kein Scheduler und kein Publisher in S13 vor integriertem G1. S02/S03/S04 erhalten `CR-13-001-CONTRACT-STORE-PFADE.md`. Der Gesamtstatus und die Gate-Dateien bleiben unter Kontrolle von S00.

## S13-B02: Assets-Quelle und API-Landschaft haben sich geändert

`rust/crates/dbrain-sources/src/assets_api.rs` verwendet `BASE_URL = https://assets.deadlock-api.com` und unter anderem `/v2/items`, `/v2/heroes`, `/raw/items`, `/raw/heroes`. Der explizite HTTPS-HEAD-Versuch auf `/v2/heroes` am Prüftag scheitert mit curl Exit 6, `Could not resolve host`, HTTP-Code `000`. Das ist eine DNS-/Erreichbarkeitsbeobachtung von diesem Host, kein HTTP-404 und kein Beweis über die dauerhafte Abschaltung des Dienstes.

Der öffentliche Repositoryabruf `deadlock-api/deadlock-api-assets` ergibt GitHub HTTP 404. Ein 404 beweist weder Löschung noch eine bestimmte Sichtbarkeit. Das Projekt bleibt als ungeklärte Pflichtfamilie im Register; es wird nicht still entfernt.

Die unabhängig geladene aktuelle Spezifikation `https://api.deadlock-api.com/openapi.json` ergibt HTTP 200, 380364 Bytes und SHA-256 `c99d82d667d280e717ab58a2c78de79ff2a20f4185aa2c2bb480c342328bb737`. Sie enthält Assets unter `/v1/assets/`, einschließlich `items`, `heroes`, `ranks`, `colors`, `build-tags` und `npc-units`. Sie enthält `language` und `client_version` als Parameter bei Items/Helden; Helden zusätzlich `only_active`. Die Herkunft der Spezifikation ist durch Abrufadresse und Hash belegt, nicht durch einen behaupteten Deployment-Commit.

**Folge:** Nach G1 bestehenden Adapter umbauen, nicht duplizieren. Alle neun vorhandenen Endpoint-Varianten müssen eine geprüfte Zuordnung oder einen sichtbaren Blocker bekommen. Für `/raw/items` und `/raw/heroes` ist keine gleichwertige Ersatzroute nachgewiesen. Ein bloßer Hostnamewechsel wäre unvollständig. Downstream-Fachnormalisierung und ID-Mappings müssen an erlaubten Fixtures geprüft werden.

## S13-B03: Fehlende oder falsche Strukturen werden teilweise zu Erfolg

In `assets_api.rs::snapshots_for` wird ein Nicht-Array zu einer leeren Snapshotliste. Nicht-Objekte innerhalb eines Arrays werden übersprungen. Fehlen `id`, `class_name` und `name`, entsteht als externe Identität die aktuelle Listenposition. Der vorhandene Test `snapshots_for_items_derives_external_ids_and_names` erwartet ausdrücklich diese Positionsidentität. Eine Umstellung ist daher eine dokumentierte Verhaltensänderung mit Regressionstest, kein kosmetischer Refactor.

In `deadlock_api.rs::pull_match_metadata_inner` wird `payload.as_array().cloned().unwrap_or_default()` verwendet. Im normalen Mehrmatchpfad kann ein strukturell falsches Payload so zu null Matches/Snapshots statt zu einem Contractfehler führen. Die Einzeltarget-Prüfung ist davon getrennt zu betrachten; dieser Befund behauptet nicht, dass jede Codeverzweigung falsche Daten akzeptiert.

**Folge:** Pflichtcontainer und Identitäten vor Staging validieren. Leere, aber gültige Antworten von kaputten Antworten unterscheiden. Fehlende IDs, unbekannte Enumwerte und unvollständige Pflichtdaten dürfen nicht als bestätigte Nullwerte erscheinen. Bestehende gültige Daten nicht destruktiv überschreiben.

## S13-B04: Der als Raw gespeicherte API-Inhalt ist bereits neu serialisiert

Die Assets- und Matchadapter rufen `get_json` beziehungsweise `get_deadlock_api_json` auf und erzeugen anschließend `json_bytes(&payload)`. `store.rs::write_raw` erhält diese serialisierten Bytes. Der dortige SHA-256 ist ein Hash der übergebenen Bytes, nicht automatisch der unveränderten HTTP-Antwort.

`SourceDocumentInput` enthält Source, externe ID, Titel, URL, Content-Type, Rawpfad, Content und freies JSON-Metadatenfeld. Der Store besitzt bereits deduplizierte Source Documents, Entity Snapshots und Source Runs; diese Infrastruktur ist wiederzuverwenden. Parserrevision, Ursprungsartefakt, Parserfamilie, Nutzungs-/Egressentscheid, Schemafingerprint und Prüfstatus sind in diesem Input nicht als verpflichtend validierte Felder vorhanden.

**Folge:** S02 definiert Raw- und normalisierten Hash getrennt und eindeutig; S03 integriert die Provenienz-/Revisionserweiterung in den bestehenden Store. Gemeinsamen HTTP-Client über S02 um einen begrenzten unveränderten Response-Body-Pfad erweitern, falls das Inventar keinen geeigneten Port zeigt. Keine zusätzliche HTTP-Implementierung in S13.

## S13-B05: Aktueller Gitimport ist kein belegter vollständiger Historienimport

`deadlock_data.rs::sync_repository` klont mit `--depth 1` und aktualisiert mit `pull --ff-only`. Der Import liest den Arbeitsbaum unter `repo_dir`; `read_repo_info` erfasst HEAD und Commitzeit. `RepoInfo` erhält bei fehlendem `.git`-Verzeichnis die Revision `local-fixture`. Die `.git`-Prüfung verwendet `is_dir`, erkennt also nicht allgemein Git-Worktrees mit `.git`-Datei. `PullDeadlockDataOptions` besitzt keine explizite Zielcommit-Auswahl.

Der Rückgabereport führt `trusted: true` und `source_trust: trusted`. Das ist keine Nutzungsrechteprüfung und keine unabhängige Faktenbestätigung.

**Folge:** Nach G1 commitbezogen und ohne Arbeitsbaumdrift lesen; Shallow-Grenze, Rename, Delete, Merge-Eltern und fehlende Archive ausweisen. Unsichere lokale Fixtures dürfen nicht produktiv freigegeben werden. Parser-/Generatorrevision getrennt von Clientversion, Commitzeit und belegter Patchgültigkeit halten. Der erfasste Upstreamcommit und sein Elterncommit sind Prüfreferenzen, noch kein importierter historischer Umfang.

## S13-B06: OpenAPI allein deckt den Match-Metadatenvertrag nicht ab

Die beobachtete Spezifikation enthält `/v1/matches/metadata` samt den vom Adapter verwendeten Queryparametern. Für HTTP 200 beschreibt sie `application/octet-stream` als Array von Integerwerten, aber kein `application/json`-Objektschema für die Matchfelder. Das ist kein Beleg für einen ungültigen Endpunkt, aber auch keine vollständige JSON-Feldabdeckung durch das Contractgate.

`/v1/players/{account_id}/match-history` ist als Array von `PlayerMatchHistoryEntry` typisiert. `match_id` ist nichtnegative Ganzzahl mit Format int64; `account_id` und `hero_id` sind nichtnegative Ganzzahlen mit Format int32. `match_duration_s`, `start_time`, `game_mode`, `match_mode` und weitere Matchfelder sind separat vorhanden. Einheiten und Enumsemantik müssen dennoch unabhängig belegt werden; der JSON-Typ allein reicht nicht.

**Folge:** Für Metadaten nach Rechtefreigabe ein minimales, bereinigtes Originalbeispiel plus Quellen-/Generatorpin gewinnen. S13 gibt Fenster, Kohorte, Missingness und Abrufstatus an S05 weiter; S05 bleibt für statistische Interpretation zuständig. Es wurde kein Accountverlauf und kein Matchdatensatz live abgerufen.

## Quellen und Reichweite

- Eigener Rust-Code und Koordinationsdateien am oben genannten Basiscommit.
- Primäre API-Dokumentation: https://api.deadlock-api.com/docs und deren verlinkte OpenAPI-Datei.
- https://github.com/deadlock-wiki/deadlock-data: README benennt Deadbot als Generator; Commitmetadaten siehe `UPSTREAM_PINS.json`.
- https://github.com/ValveResourceFormat/SchemaExplorer: README nennt `schemas/deadlock.json` und die Generierung über DumpSource2/GameTracking. Das zeigt eine Ableitungskorrelation, keine zusätzliche unabhängige Gameplaybestätigung.

Nicht erfolgt: Datenbankzugriff, produktiver Import, Freigabe fremder Fixtures, Schema-Watcherbetrieb, Rust-Adapteränderung, Deploy oder Neustart. Die genannten Mängel sind nicht durch diesen Vorbereitungs-PR behoben.
