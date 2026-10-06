status: fixbedarf
Datum: 2026-10-03
Statischer Snapshot: 2026-10-03T06:42:29Z
Eigene Nachkontrolle relevanter D-Quellen: 2026-10-03T06:50:36Z

# Unabhängige statische Prüfung D, Runde 1

Frischer nativer Sol-high-Prüfer ae5dd4780dc5d7217, ausschließlich lesend nach Punkt 28. Keine Compiler, Tests, Dateiänderungen oder Steam-Aufrufe. Keine endgültige ALLOW-/Deployment-/SHA-Abnahme. Die vier nachkontrollierten betroffenen D-Quellen game_download.rs, manifest.rs, store.rs und net.rs entsprechen weiter den unten genannten Prüfsnapshots. B bleibt ein WIP-Lesesnapshot.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 8/8 geprüft

## Bestätigte statische Befunde

### 1. P1: Hintergrundempfang kann Depotgeheimnisse protokollieren

F6. game_download.rs:30 schützt nur das Polling der einzelnen Anfrage mit NoSubscriber. Die bestehende Verbindung empfängt Antworten über den bereits separat gestarteten Filtertask in vendor/steam-vent-0.4.2/src/connection/filter.rs:76. Dieser zieht über connection/raw.rs:52 und transport/websocket.rs:33 den Transportstream unabhängig vom lokalen Anfragedispatcher.

Reproduzierbarer Zustand: vorhandene WebSocket-Verbindung, vom Validator erlaubter Filter tungstenite::protocol::frame=trace und reguläre Depotkey- oder CDN-Freigabeantwort. Tungstenite 0.27.0 protokolliert den eingehenden Frameinhalt hexadezimal vollständig. Daraus sind die Depot-/Transportgeheimwerte zurückgewinnbar. Es wurden keine echten Geheimwerte gelesen, ausgegeben oder künstlich erzeugt.

Zwillingssuche: Anfrageencoding ist unter NoSubscriber geschützt; sein Test prüft nur Encoding, nicht den Hintergrundempfang. HTTP-Wrapping beweist ebenfalls keine Hintergrundtaskabdeckung. Empfangsschutz muss vor dem niedrigeren Frame-/Decoderlogging greifen, sichere Status-/Fehlerlogs außerhalb erhalten.

Zusätzliche gezielte Callernachprüfung: websocket.rs:29 hängt derzeit KeyLogFile an rustls. Der Fixplan muss auch mögliche TLS-Schlüsseldateiausgabe verhindern, ohne ENV-/Secretinhalte zu lesen. Das ist ein vorhandener bedingter Debugpfad, keine beobachtete tatsächliche Schlüsseldatei.

### 2. P1: Manifestgrenze erst nach vollständiger Objektallokation

F6/F11. game_download/manifest.rs:60 decodiert ContentManifestPayload vollständig; die 200000-Einträge-Grenze folgt erst in Zeile 71. Ein formal gültiger Payload aus 200001 leeren Mappingnachrichten, jeweils 0a 00, benötigt nur 400002 Bytes und erzeugt alle Objekte vor Ablehnung. Viele Millionen solcher Einträge passen unter die 64-MiB-Transportgrenze und führen zu wesentlich höherer Objektallokation.

Zwillingssuche: ZipArchive::new verarbeitet ebenfalls das vollständige Zentralverzeichnis vor Prüfung auf einen Eintrag in manifest.rs:31/32. Der CM-Empfang hat zudem bestehende vorgelagerte Allokationen: net.rs:109 reserviert die behauptete Protobufheadergröße vor ausreichender Eingangsprüfung; message.rs:223 reserviert einen behaupteten Multi-Unterrahmen, ehe vorhandene Daten/Budget geprüft sind. Diese Bestandsdefekte wurden nicht durch den neuen EResult-Guard eingeführt. Nachträgliche Appinfo-/Ausgabegrenzen schützen nicht davor.

Erforderliche Wirkung: Grenzen vor und während Decodierung, Eintragszählung und ZIP-Verzeichnisaufnahme; reale Header-/Multi-Längen und Gesamtbudget vor Speicherreservation. Keine selbst geschriebene Ersatzkryptografie oder Dekompressionsbibliothek. Positive legitime Decoder-/Callerfälle erhalten.

### 3. P1: Zusatzdateien erhalten beim B-Lesen unbelegte Manifestprovenienz

F10. game_download.rs:96/117 prüft gewählte Manifestdateien, lässt zusätzliche reguläre Dateien im Root stehen und meldet vollständigen Root. B game_files.rs:133/387 traversiert anschließend den gesamten Root ohne Manifest-Dateiliste und übernimmt die vorgegebenen Depot-/Manifestwerte.

Reproduzierbarer Zustand: extra game/citadel/scripts/fremd.txt liegt im späteren Depotroot, gehört aber nicht zum gewählten Manifest. Alle echten Manifestdateien sind korrekt. D kann Erfolg melden, B extrahiert zusätzlich die fremde Datei mit der Steam-Manifestkennung. Zusätzliche VPK-Directorydateien und deren nachbarschaftlich geöffnete nummerierte Archive haben denselben fehlenden Abgleich.

Erforderliche Wirkung: fremde Dateien erhalten, aber exakte Manifestzuordnung vor erfolgreicher Übergabe und Extraktion erzwingen oder betroffenen Root ablehnen. D kann vorhandene Extras im eigenen Schreiber ohne Löschung ablehnen; C2/B müssen die Bindung am tatsächlichen Lese-/Importübergang ebenfalls bestätigen. Keine B-Parserneuentwicklung und keine B-Quelländerung durch D.

## Unauffällig statisch geprüft

Zentraler EResult-Guard erhält unbekannte i32-Headercodes unverändert; bekannte Codes einschließlich Invalid/OK sowie fehlende Headerwerte behalten den bisherigen Ablauf. Nachverfolgt: job/job_multi, service_method, one/on, unauthentifizierte Antworten, Login, GC-Welcome/-Antworten, Notifications und gepufferte Friends-/Persona-Nachrichten. Vorhandene Fehlerverwerfungen wurden nicht als neuer Erfolgspfad durch den Guard bewertet.

Acht Fremddienstpfade: PICS-Zugang, Appinfo, Serververzeichnis, Depotkey, Manifestfreigabe, CDN-Freigabe, Manifest-HTTP und Chunk-HTTP. Feste App 1422450/Payload {}/Ziel, vorhandene ctx.connection, keine Login-/Kauf-/Exportstrecke. HTTPS-Hostliste, keine Proxys/Redirects und begrenztes Lesen. Getrennte Originalroots und VPK-Nachbarschaft passen grundsätzlich zu resource_paths.

NOFOLLOW je Pfadkomponente, reguläre Dateien mit einem Hardlink, exklusives Staging und NOREPLACE schützen die untersuchten Pfad-/Überschreibfälle. Resume über Größe/SHA-1, abweichende Dateien bleiben erhalten. Chunktransport, Dekompressionsausgabe/Decoderfenster, Datei-/Chunkhashes, Speicherprüfung, Bestand/Reserve, Lane 1 und bestehende Zeitgrenze sind im geprüften Code vorhanden. Diese Feststellungen sind keine bestandenen Laufzeitprüfungen.

## Fehlende echte Nachweise

Keine Cargoauflösung, Compiler, Tests, Clippy oder Rustfmt. Zwölf Testfälle vorbereitet, nicht bestanden behauptet. Cargo.lock ohne Diff. Keine Steam-Aufrufe/Downloads; tatsächliche Depotwahl, Plattform-/Branch-/Sprachabdeckung und Spielvollständigkeit ungeprüft. Primärquellenvergleich ersetzt keinen Download. Jede folgende Änderung entwertet betroffene statische Prüfung; finale Prüfung auf realem eigenen Commit erforderlich.

## Relevante Snapshot-Hashes

| Datei relativ zum Steam-Worktree, soweit nicht anders genannt | SHA-256 |
| --- | --- |
| rust/Cargo.toml | 4a9b9c7259fed3aa682ba1fa59f4db28b483509f422ff0bff89111f38d59b7c5 |
| rust/crates/steam-core/Cargo.toml | 8dabb5cca0a9f959e57a9f047471167fc7db10787a1102bdbfe2ff2188deab5c |
| game_download.rs unter handlers | 512cefc1373b4a2ca281af91ca350c0b2c6f11dcef704b8c76b0106ef78f6c19 |
| game_download/manifest.rs | 548d523f084b466f438f6c652bc8a97c72126f944c5a80e17b6198c7080d4c81 |
| game_download/protocol.rs | 512b0456ecbb306a6d13678ee72f4afafac386cfd31014a01452a6e9026c759c |
| game_download/store.rs | 22c15e77429512bbefb61630d9a40bd326fb5f15507b0c74f448ebc3b1d43d00 |
| game_download/tests.rs | 857283349be8c778c3606bf36bb4d4bf1761378afc59a2d9127504398a20ea7f |
| rust/vendor/steam-vent-0.4.2/src/net.rs | 31b4e7e6f03ab15f2c621f74f30790f3cd5f9d45a2ce094c4e3288fb26a8784a |
| rust/Cargo.lock | 82f7309a075592c598c2af2c62b5532edc1115134c6bf2149425b9aa4414d72f |
| rust/vendor/steam-vent-0.4.2/src/connection/filter.rs | fe83a240d86df0e1ee84c18cdd712f443f0c1fee01606dc387716d072fcf203b |
| rust/vendor/steam-vent-0.4.2/src/transport/websocket.rs | 58e3ae2a54cd529eefb1c1782b9af5d5c08fb3e8cb61408cc7dc9175d551e0a6 |
| rust/vendor/steam-vent-0.4.2/src/message.rs | 4250164846c99bd8809696ebc2171368175ed8508c66ff373337c31a3da414be |
| B game_files.rs | 8ca3b514ac2fd4fc0301193114f381560db1322de851ffcb2f7c842f23e45fac |
| B game_files/vpk.rs | feeec228eea64e008ba09dd70032245c32e366bcb7c4c2e891c3466310f9f368 |
| B game_files/anchored.rs | e6bd4e5221d8806e85f043dc90a116db87848f38a45bad57dd37047858e09474 |

Bestandsdateien teilweise nur in relevanten Ausschnitten geprüft. Vertragsdokumente zu Prüfungsbeginn gehasht, nicht beim genannten Abschlusszeitpunkt. Sammelhashalgorithmus des Implementierers wurde nicht gleichgesetzt.

Primärquellen: SteamKit Callbacks.cs (PICS), DepotManifest.cs, CDN/Client.cs, CDN/DepotChunk.cs im SteamRE/SteamKit-Master; Tungstenite v0.27.0 protocol/frame/mod.rs und protocol/frame/frame.rs im snapview/tungstenite-rs-Repository. Keine tatsächliche Geheimnisausgabe überprüft oder ausgelöst.
