status: aktiv
Datum: 2026-10-03
Stand: 2026-10-03T10:48:38Z

# D/B-Lesebindung: Bereitschaft N

Frischer nativer lesender Sol-high-Vorcheck `a448286025029f89e` abgeschlossen, Quellprüfstand 10:39:46 UTC; Elternsession hat kritische Aufruf-/Publizierstellen und Dateihashes um 10:48:38 UTC nachgelesen. Keine Edits, Compiler, Tests, Gates, Downloads oder Live-Aufgaben.

Urteil: **Unmittelbare D-Inventarbindung fehlt im tatsächlichen Extraktions-/Importübergang.** Kein neuer Parser nötig. Der abgenommene Eigencommit48b6ce1 bleibt unverändert. Dies ist der neue enge Integrationsrest, kein neuer Fehler im abgenommenen Git-Exportteilstand.

## Tatsächlicher vorhandener Pfad

| Eigentümer | Pfad und Fundstelle | Belegter Stand |
| --- | --- | --- |
| D | `rust/crates/steam-core/src/task/handlers/game_download.rs:184-219`, `download` | Inventar gegen damaligen Downloadbestand geprüft, anschließend Root/App/Build/Depot/Manifest, Manifesttransporthash und Dateiliste veröffentlicht. |
| D | `game_download/store.rs:227-295,334-369`, `Root::inventory`, `hashes` | Exakte Pfade einschließlich impliziter Eltern, Typ, Größe und Steam-SHA-1 geprüft; SHA-256 über gelesene Bytes erzeugt. Bindet D-Publishlauf, nicht spätere B-Lesezugriffe. |
| B | `rust/crates/dbrain-sources/src/game_files.rs:69-79,125-179,320-339,387-433`, `extract_game_files`, `visit_directory`, `write_document` | Traversiert aktuellen Rootbestand ohne D-Dateilistenabgleich. Build/Manifest/Depot/Provenienz aus Optionen übernommen. Hinzugefügte unterstützte Textdatei bekäme dieselbe deklarierte Herkunft. |
| B | `game_files/vpk.rs:17-20,193-248`, `original_file`, `read_resource` | Gehaltener Verzeichniscontainer, verankerte Begleitarchive je Ressource neu geöffnet. Bounds und CRC vorhanden, kein D-SHA-Abgleich der tatsächlich verwendeten Begleitarchivbytes. Ressourcen- und physischer Containerhash sind getrennte Ebenen. |
| C3 | `rust/crates/dbrain-sources/src/lib.rs:7-28`, `src/bin/brain-knowledge-import.rs:26-65,144-150` | Noch keine B-Modulregistrierung oder Extraktionsanbindung, CLI beginnt bei fertigem JSONL. Workspaceweit einschließlich CLI/Import/Nebenpfaden kein produktiver extract_game_files-Caller gefunden. Graphify zuerst, neue Module darin nicht enthalten. |

Absolute Worktrees: B `/home/nathanael/.worktrees/brain-wiki-spielwissen-b`, C3 `/home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration`, D `/home/nathanael/.worktrees/steam-brain-spieldepot-d`.

## Enger Restbauteil und Eigentum

**C3s Caller mit inventargebundener Byteübergabe fehlt.** Im bestehenden dbrain-sources-/CLI-/Importpfad den abgenommenen Parser registrieren und wiederverwenden. Identität und vollständige Liste mit relativen Pfaden, Typ, Größe, SHA-1/SHA-256 unmittelbar an tatsächlich verwendete Bytes binden, einschließlich physischer VPK-Container/Begleitarchive. Eine Vorprüfung mit danach separat geöffneten Dateinamen ist kein solcher Beweis. Hinzugekommene, ausgetauschte oder unbelegte Dateien dürfen keine bestätigte Manifestprovenienz bekommen. Nicht nur Optionsmetadaten kopieren.

C3 besitzt Registrierung `src/lib.rs`, bestehenden CLI/Import unter `src/bin/brain-knowledge-import.rs` und `knowledge_import`, sowie gemeinsame Integrationsschnittstellen. Fehlender enger Caller-/Lesebindungsbauteil dort zuzuweisen, ohne parallelen Extraktor. D behält Downloader/Store/Manifestinventar. B behält sieben unveränderte Parserdateien und lokalen Harness. Falls der Byteübergabevertrag einen Eingriff in B braucht, erst konkrete Zusatzpfade/Änderung mit B zuweisen; nicht verdeckt in den abgenommenen Eigencommit eingreifen.

## Bereits vorhandene Prüfbausteine

D `game_download/tests.rs:225-277`: Dateiwechsel, Fremddateien/Begleitarchive, Fehlpfade, Typwechsel, Hardlinks und Symlinks. B `game_files/vpk.rs:410-432,481-489`: verankerte Begleitarchivwechsel und Trunkierung. Diese Bausteine für den tatsächlichen Caller-/Leseübergang wiederverwenden. Vorcheck startet keine Tests und behauptet deren aktuelle Endstandfreigabe nicht.

Lokaler `pruefharness-b/src/bin/validate-jsonl.rs:311-353` bleibt Git-/Lose-Datei-Validator: Steamfelder müssen null sein, Originale separat geöffnet. Keine Steam-/VPK-Abnahme. D/B-Prüfung muss vor Steam-Deploy auf endgültigen SHAs erfolgen; echte Depotextraktion und Datenabnahme danach mit belegten Rohdaten. Keine synthetischen Fälle als echte Depotabnahme ausgeben.

## Vorläufige Prüfbindung

B HEAD48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5, sieben Git-Blobs/Arbeitsdateien identisch, Elfdatei-Freeze3cef2c79fd36c9462948f90ceb4fa28b6b82b6ad1102769a8b8812a66132055c.

D HEAD4c5621763d5f01c96d7912400517c08aa1c40df1 mit uncommittiertem Downloader. Fünfdatei-Fingerabdruckc69e7eba1d25382e188f41c6ff76364304f0ae44d05b479901aaaaea6a507bdf bei Workerwiederholung unverändert. Elternprobe: Handler SHA-256472f0e22b5febe381eb7a36d673df1c4d0cb2e79e298bcc2c090e0871da0cce5, store SHA-256345001a4c64f36ddeec15adf218b7ec34f9e06aa2b9df6a38c3446d65243997b.

C3 HEAD08a6dd78471cd6e7c43073e1c29dd0f31abb61c5 mit veränderten gemeinsamen Quellen. CLI SHA-256f68b0fb1bc609106a57d2ff3373d7dd3cb4071f062202b4ffa025086b5d8f428 bei Workerwiederholung unverändert, lib.rs SHA-256a80ecf555257f1c65665ca012878e9513c725499d307a090c18ea8c7fd04f682 bei Elternprobe.

D/C3 ausdrücklich vorläufig, keine endgültige gemeinsame SHA-Abnahme. Im D-Rohroot fehlen steam/inventories/Downloadinventare; keine tatsächlichen Depot-/Manifest-/Originaldateihashes für Datenabnahme. Kein erneuter Login, Lizenzrequest oder Live-Task.

Nächster Schritt: Root weist C3 den engen Caller-/Lesebindungsrest zu; danach gemeinsam finalen D/B-Stand prüfen. Keine Steam-Deployfreigabe aus diesem Vorcheck.
