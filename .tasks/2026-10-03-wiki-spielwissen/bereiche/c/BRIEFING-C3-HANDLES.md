status: aktiv
Datum: 2026-10-03

# C3: bestätigten Steam-Handle-Eingang anbinden

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration

## Ziel und bestehender Vertrag

Gewöhnlicher nativer Sol-high-Worker ohne Weiterdelegation oder Modellwechsel. C3 orchestriert und ist alleiniger Integrator/Deployer, Root Hauptorchestrator. Keine fremden Sessions. Lies zentrales BRIEFING-C3.md, CONTRACT.md, AN_BEREICHE.md Punkte43 bis46 und bereiche/b/B-C3-HANDLE-API-B2.md vollständig. Graphify zuerst vor jeder Bestandssuche, globaler Graph bereits vorhanden, keine Vollindexierung.

Root bestätigt Punkt46: B baut den zusätzlichen vorhandenen Extraktor ausschließlich in game_files.rs und game_files/vpk.rs, anchored.rs bleibt unverändert. Die API konsumiert BoundGameFiles { options:GameFileOptions, manifest_transport_sha256:[u8;32], files:Vec<BoundGameFile> }; BoundGameFile enthält relative_path:String, gehaltenes File, expected_bytes:u64, steam_sha1:[u8;20], expected_sha256:[u8;32]. extract_game_files_from_handles(input, output) hält diese Objekte, setzt Cursor und prüft tatsächliche Typ-/Größen-/SHA256-Bindung. Steam-SHA1-Prüfung, vollständiger D-Inventarbeleg und private unveränderliche Lesesicherungen sind C3-Eigentum. Keine Aliasdeskriptoren parallel nutzen. Sämtliche regulären Dateien einschließlich übersprungener Assets und VPK-Begleitarchive bereitstellen; Verzeichnisse/sonstige Typen im vollständigen Beleg erhalten. Physische Referenzen extraction.physical_sources, Ressourcenhash getrennt. Bei Err niemals teilweise Ausgabe importieren.

## Nachgelesener Bestand und konkreter Bau

D-Quellen sind nur lesbare Referenz, keine D-Edits: /home/nathanael/.worktrees/steam-brain-spieldepot-d/rust/crates/steam-core/src/task/handlers/game_download.rs und game_download/store.rs. Der aktuelle Writer veröffentlicht Gesamtinventar mit depots und je Depot app_id/build_id/depot_id/manifest_id/manifest_sha256/root/observed_at/status/expected_bytes/files. Dateien haben path/kind=file/size/sha1/sha256; explizite Verzeichnisse kind=directory,size0. SourceInventory-Provenienz Steam/resource_paths; Gesamtdownload höchstens96GiB, Bestand192GiB, Reserve16GiB, Datei-/Pfadgrenzen vor Aufnahme. Manifesttransportbytes werden gehasht, nicht im Inventar als Rohdaten ausgegeben. manifest_sha256 ist der D-belegte Transporthash; keinen unabhängigen neuen Steamtransportabruf erfinden oder dessen Rohbyteprüfung vortäuschen.

Baue den kleinsten vorhandenen Caller-/Bindungsrest als neues Rust-Modul unter dbrain-sources. Lies tatsächliche D-Formate und vorhandene sichere Datei-/Temp-/Hashmechanik, wiederverwenden statt zweiten Downloader/Parser bauen. Konkrete schmale API für Importer und B-Abnahme früh an C3 melden. Eingabe aus normaler expliziter Datei mit gepinntem Inventarhash und erwarteter Root/App/Build/Depot/Manifest-Identität; keine ENV-Konfig oder Secret-/Steamzugriffe. Gesamtinventar und Depotbeleg widerspruchsfrei vergleichen, vollständige normalisierte relative Liste, Typen, Größen, SteamSHA1/SHA256 und Summe an tatsächlich kopierte Bytes binden. Unterinventar niemals als kompletter Depotbeleg ausgeben. Fremde Dateien/Typen/Extras sichtbar fail-closed beziehungsweise getrennt als nicht manifestbelegt führen, niemals löschen oder öffnen als Manifestbestand.

Private bytegenaue Sicherungen müssen skalieren: alle Dateien können insgesamt96GiB groß sein. Nicht pauschal RAM-/memfd-Kopien des ganzen Depots. Vor Sicherung echten freien Speicher, konfigurierte Gesamt-/Dateianzahl-/Pfad-/FD-Grenzen prüfen. Originale verankert und ohne Symlink-/Hardlink-/Typ-/Pfadwechsel öffnen, streamend kopieren und beide Hashes am Kopierstrom prüfen. Eigene private Sicherung read-only halten, Schreibobjekte vor Übergabe schließen, keine externen Pfadöffnungen beim späteren Parser. Kein mutable Namenscheck als Ersatz für tatsächliche Bytes. Keine Aliasdeskriptoren parallel, keine Löschung von Originalen. Eigene temporäre Sicherungen dürfen regulär nach exklusivem Ende freigegeben werden; verbleibende Belege/JSONL privat atomar und erst nach vollständigem Erfolg veröffentlichen. Bei ExtraktorErr Ausgabe als ungültig behandeln, nicht in bestehenden Import weitergeben. Bestehender B-Parser konsumiert gebundene Objekte; keine Kopie seiner Logik oder eigene VPK-Extraktion.

## Exklusives Eigentum

Nur neue rust/crates/dbrain-sources/src/steam_game_input.rs und neue tests/steam_game_input.rs; bei begründetem Umfang eigene Unterdateien unter src/steam_game_input/. Keine vorhandene Datei oder Cargo/lib.rs/CLI ändern. Alle sieben B-Parserdateien bleiben ausschließlich B. C3-CLI-Worker a48b3e3dfac497915 besitzt lib.rs/Cargo/Lock/Importer/Validator/PG-Retry/ChunkIndex; Reader-Worker a813e5d645f3f8e09 nur runner.rs/runner_tests.rs. Zusätzlichen minimalen Abhängigkeitsbedarf melden. Bereits vorhanden sha2/tempfile/serde/serde_json, konkrete SHA1 und verankerte rustix/fs sind im D/B-Bestand benutzt, werden ausschließlich CLI-Worker integrieren. Keine eigenen Implementierungen von Hash-/Pfad-/Manifestparsern.

Worktree/Branch unverändert, aktuell HEAD7168574626cd1eeb960de750470d2d593722295b nach B-Eigencommitübernahme. Fix2ad3eaca regulär geprüft und Sol-high-ALLOW, kein Gesamt-Allow. Fünf geerbte/CLI-Format-/Zwischenstandsdateien erhalten. Gemeinsame Quellenphase ist offen, kein Prüfer aktiv. Fremde Arbeit/Writer/Locks nicht anfassen, keine globale Formatierung oder Rücksetzung.

## Beweisziel und Grenzen

Zunächst Quellen bis Schreibabschluss und konkrete Caller-API melden. KEIN Cargo/Rustharness/Commit/Push/Deploy oder Produktivlauf vor ausdrücklicher gemeinsamer Prüffreigabe. Danach bestehende echte Datei-I/O-/Race-/Symlink-/Hardlink-/Hash-/Größen-/fehlendesArchiv-/Extras-/Trunkierung-/Teiloutputprüfungen, keine behauptete echte Steamdepotabnahme mit Fixtures. Sämtliche Compiler unter beiden blockierenden Hostlocks nach HOSTPROBE.md, frische NonZombie-Probe, höchstens zweiJobs, keine Timerabbrüche stabiler Tasks. Kein Releasebau durch dich.

A-Fix6 läuft getrennt wegen282 numerischer Faktenabweichungen; alte A-Fakten nicht importfrei, konkrete serde_json-Features über Root ausstehend. Keine Epsilon/Rundung. Keine produktiven Imports/DB-/Konfig-/Unit-/Credentialzugriffe, zweite Steam-Sitzung, Appgrant oder Download. Gemeinsame D/B/C-Endabnahme vor Steam-Deploy, echte Depotextraktion danach bleibt Pflicht.

## Bericht und Routing

Knapp an C3 per eigenem SendMessage main: bestätigte vorhandene Komponenten, konkrete API/Dependencybedarf, Quellenabschluss, eigene Dateiliste und ehrlich fehlende Beweise. Deutsche Texte/echte Umlaute, keine Gedankenstriche. Keine Register-/TODO-/Status-/Berichtdateien außerhalb eigener Quellen schreiben. Bei echter Vertragsabweichung konkret Ursache und kleinste Lösungsrichtung melden, keine neue Plattform oder unautorisierte Scope-Erweiterung.
