status: erledigt
Datum: 2026-10-03
Beweisaufnahme abgeschlossen: 2026-10-03T04:03:51Z
Auftrag: ergänzende Zugangsprüfung zu AN_BEREICHE.md, Punkt 13

# Bestehender Steam-Zugang und Downloadgrenze

## Ergebnis

Der vorhandene Steamdienst antwortet auf seine bestätigten Leserouten und meldet Steam und Deadlock-GC als verbunden. Im Rust-Code gibt es außerdem einen registrierten Weg, über die bestehende Sitzung eine kostenlose App-Lizenz anzufordern. Das ist eine Änderung am Konto und wurde nicht ausgeführt.

Ein bestehender, nutzbarer Depotdownloadweg ist durch diese Prüfung nicht belegt. Die vollständig gelesene Task-Registry enthält keinen Depot-, Manifest-, Installations- oder Spielclientdownloadjob. Auch die zusammengesetzten HTTP-Router stellen keinen solchen Vorgang bereit. Die gelesenen Statusverträge liefern keine Auskunft über die Downloadberechtigung für App 1422450 oder 1422460. Die Berechtigung des Botkontos bleibt daher ungeprüft. Eine Accountbezeichnung, ein erreichbarer Dienst oder eine GC-Verbindung reichen dafür nicht aus.

Diese Grenze gilt für die konkret geprüften Dienstwege. Sie ist keine Behauptung, dass auf sämtlichen Datenträgern oder über jede Steam-Protokollfunktion ein Download ausgeschlossen wäre.

## Vorliegende Belege übernommen

`RECHERCHE.md` und `DATENSAMMLUNG.md` im B-Bereich wurden gelesen. Die vollständige Erstsuche wurde nicht wiederholt.

Die eigene offizielle SteamCMD-Installation ist bereits dokumentiert unter `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/tools/steamcmd`. Laut `DATENSAMMLUNG.md:8-12` funktionierte der anonyme Login; die normalen Installationsversuche für Client 1422450 und Server 1422460 scheiterten jeweils mit Exit 8 und `No subscription`. Diese Versuche wurden hier nicht erneut ausgeführt. Ihr Ergebnis betrifft den anonymen Zugang, nicht die ungeprüften Rechte des bestehenden Botkontos.

## Bestätigte Leserouten und tatsächliche Antworten

Die Graphify-Abfragen gegen den globalen Graphen und den Graphen von `/home/nathanael/repos/Deadlock-Steam-Bot` lieferten die API-, Task- und Startup-Dateien. Die Angaben wurden anschließend gezielt im Rust-Code überprüft. Fehlende oder abgeschnittene Graph-Treffer wurden nicht als Abwesenheitsbeweis verwendet.

| Leseroute | Vertragsbeleg | Antwort bei dieser Prüfung |
| --- | --- | --- |
| `GET http://127.0.0.1:8782/health` | `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/api/mod.rs:249-290` | HTTP 200, `version=0.1.0`, `steam_connected=true`, `gc_connected=true` |
| `GET http://127.0.0.1:8782/status` | dieselbe Datei, Zeilen 251 und 293-305 | Ohne Zugangsdaten HTTP 200; ausgegeben wurden nur Version und Verbindungsfelder |
| `GET http://127.0.0.1:8783/health` | `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-web/src/lib.rs:63-80` | HTTP 200, `ok=true`, `service=steam-bot`, `version=0.1.0` |

Alle drei Abrufe erfolgten ohne Login, ohne interne Zugangsdaten, ohne Redirect-Folgen und ohne Curl-Konfigurationsdatei. Die Statusausgabe wurde vor der Ausgabe auf die benötigten Felder begrenzt; Kontokennung und Freundeszahl wurden nicht ausgegeben.

Im Core-Router ist `/health` immer offen. `/status` liegt hinter der optionalen internen Schutzprüfung (`api/mod.rs:212-269`). Der Abruf ohne Zugangsdaten gelang tatsächlich. Andere geschützte Routen wurden nicht ausprobiert und keine Zugangsdaten beschafft.

Die Health-Felder entstehen im gelesenen Code aus `connection.is_some()` und `gc.is_some()`. Sie belegen die gemeldete Verfügbarkeit der Verbindungshandles, keine Spielelizenz und keinen erfolgreichen Depotzugriff. Der erweiterte Status enthält zusätzlich eine Kontokennung und Freundeszahl, aber keine App-/Paketlizenzliste oder Depotberechtigung. Die Versionsantwort `0.1.0` identifiziert keinen Produktionscommit; eine Übereinstimmung des laufenden Binaries mit dem gelesenen Checkout wurde nicht behauptet.

## Bestehende Verfahren konkret geprüft

### Task-Queue und HTTP-Verfahren

- `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/task/mod.rs:15-146` wurde vollständig gelesen. Die Registry umfasst Profil-, Statistik-, Match-, Freundschafts-, Party-/Lobby-, Playtest-/Lizenz- und Community-Buildaufgaben. Ein Spieldepotdownload ist dort nicht registriert.
- `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/main.rs:276-320` verdrahtet genau diese `default_registry()` mit dem TaskRunner. Der sichtbare Wartungsjob legt `BUILD_CATALOG_CYCLE` an, eine Community-Buildkatalogaufgabe.
- `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/task/runner.rs:343-362` lehnt nicht registrierte Tasknamen mit `unsupported_task_type` ab. Der allgemeine Enqueue-Endpunkt ist deshalb kein beliebiger ausführbarer Downloadweg. Es wurde kein Task angelegt.
- `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/api/mod.rs:249-269` registriert Status, Konfigurationsstatus, Freunde, Task-Einreihung/-Status, Scrim-Verfahren und Observer-Reservierung. Ein Depot-/Installationsendpunkt fehlt in dieser Router-Zusammenstellung. Observer- und Scrim-Schreibverfahren wurden nicht aufgerufen.
- `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-web/src/lib.rs:40-72` setzt den Bot-Router aus Link-, Rang-, Event-, Billing-, Präsenz-, Kontext- und Community-Buildrouten zusammen. `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-bot/src/main.rs:126-130` ergänzt den Konfigurationsrouter. Auch diese Zusammensetzung zeigt keinen Spielinstallationsweg.
- `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-bot/src/main.rs:175-213` nennt die gestarteten Hintergrundjobs: Freundschafts- und Rangabgleich, Bereinigung, Einladungen, Plus-Abgleich, Task-Rückstand und Ereignisaufbewahrung. Ein eigenständiger Depotdownloadjob ist dort nicht verdrahtet.

### Vorhandener Lizenzanforderungsweg

`AUTH_REQUEST_FREE_LICENSE` ist in `task/mod.rs:119-122` registriert. `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/task/handlers/free_license.rs:19-59` verwendet standardmäßig App 1422450 und sendet `CMsgClientRequestFreeLicense` über die vorhandene `Connection`. Der Antwortpfad in Zeilen 113-135 enthält `eresult`, `granted_appids` und `granted_packageids`.

Das ist ein konkreter vorhandener Steam-Protokollweg zur Lizenzanforderung. Die Codeexistenz belegt weder eine bereits erteilte Lizenz noch einen erfolgreichen Lauf für dieses Konto. Auch ein positives Anforderungsergebnis würde allein noch keinen implementierten Depotdownload belegen. Der Auftrag erlaubt keine Kontoänderung, daher wurden weder dieser Task noch ein anderer Lizenz-/Loginvorgang gestartet. Historische Taskresultate wurden nicht über erratene IDs gesucht.

### Versionsabfrage und tatsächlicher Downloadkandidat

`AUTH_REFRESH_GAME_VERSION` ruft laut `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/task/handlers/auth.rs:19-20,79-105` lediglich `GetClientVersion` ab. Der Handler braucht keine aktive Sitzung und lädt keine Spieldateien.

Der Graph-Treffer `download_metadata()` wurde gezielt überprüft. `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/task/handlers/scrim_result.rs:69-74,285-308` lädt Matchmetadaten über eine anhand der GC-Antwort gebildete Valve-Replayadresse. Das sind Matchdaten, keine Spieldepotdateien. Dieser Download wurde nicht ausgeführt.

Die gezielte Nachprüfung des bereits vom Graphen gefundenen Verbindungsbausteins unter `/home/nathanael/repos/Deadlock-Steam-Bot/rust/vendor/steam-vent-0.4.2/src` fand zu Depot-, Manifest-, Contentserver-, Download-, Lizenz- und Appinfo-Begriffen nur zwei Lizenz-Fehlercodes in `eresult.rs`. Das schließt generische Protokollnachrichten oder externe Bibliotheksfunktionen nicht aus. Es belegt keinen verdrahteten Contentdownload im vorhandenen Dienst.

Der Graph-Treffer „Installation“ gehört zu `/home/nathanael/repos/Deadlock-Steam-Bot/rust/deploy/dropins/README.md:21-32`. Er beschreibt die Installation von systemd-Drop-ins für die Bots, keine Steam-Spielinstallation. Es wurde kein dort genannter Befehl ausgeführt.

## Formulierung für Punkt 13

> Offizielles SteamCMD ist vorhanden. Anonyme Installationsversuche für Deadlock-Client 1422450 und Server 1422460 scheiterten mit Exit 8 und „No subscription“. Der bestehende Steam-Bot antwortet lesend und meldet Steam-/GC-Verbindungen; seine Downloadberechtigung wurde dadurch nicht belegt. Der Rust-Dienst besitzt einen Lizenzanforderungstask, der im lesenden Auftrag nicht ausgeführt werden darf. In der geprüften vollständigen Task-Registry, den Startup-Jobs und HTTP-Router-Zusammensetzungen ist kein Spieldepotdownload verdrahtet. Ein vorhandener berechtigter Downloadweg bleibt unbelegt; die Rechte des Botkontos werden ausdrücklich nicht als fehlend behauptet. Die Verarbeitung der bereits gesammelten Rohdaten bleibt davon unabhängig.

Ein weitergehender Rechte- oder Downloadnachweis braucht einen gesondert erlaubten Weg, der vorhandene Sitzungen erhält und die tatsächlich erteilten App-/Depotrechte prüft. Kontonamen allein ändern diesen Prüfstand nicht.

## Quellstand und eingehaltene Grenzen

SHA-256 der geprüften zentralen Quelldateien:

| Datei unter `/home/nathanael/repos/Deadlock-Steam-Bot/` | SHA-256 |
| --- | --- |
| `rust/crates/steam-core/src/api/mod.rs` | `2619c47435924ca32bef080e16ad5739309a22cb29ce55e625cd0aa9f6df1361` |
| `rust/crates/steam-core/src/task/mod.rs` | `63f21300094c488cf56ab4c8ff2ac2be77dbfa101befca07d3a3b10568a8e3d3` |
| `rust/crates/steam-core/src/task/handlers/free_license.rs` | `0aca7c1b1d8b32440a59d5714ad3cc2c6d65bbca3ae531586a899b84145ff1b3` |
| `rust/crates/steam-bot/src/main.rs` | `87274e62afb67d924082896fde610935f6536958c9c13e2704b04caa8c5a7bfd` |
| `rust/crates/steam-web/src/lib.rs` | `600ca254000beea603d0f4156c4138efa072b0e7a05cd9303467c7ce9282c956` |

Keine Passwörter, Secret-Werte, ENV-Dateien, Prozessumgebungen, vollständigen Prozessargumente oder Steam-Anmeldedaten gelesen oder ausgegeben. Keine neue Anmeldung, Lizenzanforderung, Task-Einreihung, Installation, Download, Kompilierung, Git-Aktion oder Dienständerung vorgenommen. Keine Umgehung, Delegation oder zusätzlichen Threads. Ausschließlich diese eigene Datei im B-Worktree geschrieben; andere Dateien und Arbeitsstände bleiben unverändert.
