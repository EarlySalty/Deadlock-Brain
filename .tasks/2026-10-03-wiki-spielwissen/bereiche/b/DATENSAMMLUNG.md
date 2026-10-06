status: erledigt
Datum: 2026-10-03

# Tatsächliche Datensammlung

## Offizieller Downloadversuch

Nach belegter Bestandsprüfung wurde SteamCMD aus der offiziellen Quelle `https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz` lokal unter `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/tools/steamcmd` installiert. SHA-256 des Installationsarchivs: `cebf0046bfd08cf45da6bc094ae47aa39ebf4155e5ede41373b579b8f1071e7c`.

Anonymer offizieller Login funktionierte. Der normale Downloadversuch für Server-App 1422460 endete mit Exit 8 und `ERROR! Failed to install app '1422460' (No subscription)`. Der normale Downloadversuch für Client-App 1422450 endete ebenfalls mit Exit 8 und `No subscription`. Beide Resultate wurden geprüft; der äußere Auswertungsbefehl endete mit 0, was kein erfolgreicher Spieldownload ist.

Client-Appinfo bestätigt Deadlock und App 1422450. Server-Appinfo liefert ein leeres Objekt. Keine Depotmanifest-ID oder Steam-Build-ID ist damit belegt. Kein Steamkonto mit Geheimnissen benutzt, keine vorhandene Botsitzung angefasst, keine Sperre umgangen, kein offizieller Client-/Serverbestand als heruntergeladen ausgegeben.

## Veröffentlichte Rohdateien

Quelle: https://github.com/SteamTracking/GameTracking-Deadlock . Der bisherige Name SteamDatabase/GameTracking-Deadlock leitet offiziell dorthin um. Gepinnter Commit: `4c6431ccdb816d2911bbbaa4335169cc34140386`, Commitzeit `2026-10-03T00:53:23Z`.

Vollständige Git-Baummetadaten: 14.352 Blobdateien, `truncated=false`. Pfad: `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/gametracking-4c6431ccdb816d2911bbbaa4335169cc34140386/tree.json`.

237 gezielt ausgewählte Dateien mit 32.311.304 Bytes wurden heruntergeladen und gegen Größe und Git-Blob-SHA-1 aus dem festen Baum geprüft. Darunter alle 108 veröffentlichten VData, die Gameplay-Skripte, englische Ressourcenlokalisierung, Core-Bewegungsdefinitionen, textuelle VPulse-Kartenmetadaten, cfg/gi/inf und Bestandsdateien. Kein Steam-Depotdownload: veröffentlichter Drittanbieterexport mit unbekannter Veröffentlichungslizenz. `redistribution_allowed=false`.

Zusätzliche lesende Abdeckungsprobe gegen den vollständigen gepinnten Baum: 456 Blobdateien mit Endungen json/txt/kv/kv3/vdata/lua/cfg/gi/vpulse/inf/csv, davon 237 gesammelt und 219 nicht gesammelt. Keine fehlende VData/VPulse und keine fehlende textuelle Datei unter `game/citadel/pak01_dir/scripts/` oder `resource/` in diesem Endungssatz. Die übrigen 219 enthalten Binär-Stringdumps, Modulmetadaten, Werkzeuge sowie Grafik-/Audio-/Buildkontext; sie sind nicht als extrahiert ausgewiesen. Kein deutscher Lokalisierungspfad gefunden; einziger Treffer für german/de_de/de-DE ist `game/citadel_german/gameinfo.gi`. Diese Probe behauptet keine Vollständigkeit anderer Dateitypen oder binärer Spiellogik.

Rohdatenroot: `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/gametracking-4c6431ccdb816d2911bbbaa4335169cc34140386/files`.

steam.inf belegt ClientVersion 6745, SourceRevision 11078118 und appID 1422450. Diese Clientversion ist keine Steam-Build-ID. Fehlende Depot-/Manifestwerte bleiben unbekannt.

## Erhaltener vorhandener Export

Quelle: https://github.com/deadlock-wiki/deadlock-data , Git-Commit `0d46cdecfccf77adec16aac01af6d30173e0ebb8`. Unveränderlicher Snapshot aus `git archive`, Originalcheckout unverändert.

Root: `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/deadlock-data-0d46cdecfccf77adec16aac01af6d30173e0ebb8/files`.

431 Dateien mit 34.530.366 Bytes einschließlich vollständigem data-Baum, README und MIT-Lizenz. Davon 158 JSON, 263 Textdateien, zwei CSV und sechs PNG; neben den 17 Gameplay-Sammlungen und 29 Lokalisierungen enthält der gesamte Baum auch historische Changelogs. PNG sind Inventar, kein Mechanikwissen. README bestätigt die Deadbot-Ableitung. MIT des Repositories ist keine pauschale Weiterverteilungserlaubnis für Valve-Assets.

ClientVersion 6731, SourceRevision 11070267, appID 1422450. Dieser Stand bleibt vom GameTracking-Stand 6745 getrennt.

Erneute vollständige lesende Git-Blob-Prüfung am 2026-10-03 um 04:57:43 UTC: GameTracking 237 Dateien/32.311.304 Bytes, deadlock-data 431 Dateien/34.530.366 Bytes, jeweils null Abweichungen gegen genau die oben genannten Revisionen. Sichere reguläre Originalpfade unter den jeweiligen Rohdatenroots geprüft; keine Originale verändert. Das bestätigt die erhaltenen Snapshots, ersetzt weiterhin keinen JSONL-Extraktionslauf.

## Verbleibende Grenze

Kein vollständiger lizenzierter Spieleclient oder offizielles VPK vor Ort. Deshalb keine Behauptung vollständiger Spiel-/Binärlogik und kein echter VPK-Originaltest. Technische Rohdatenextraktion und interne Importfreigabe bleiben getrennte Schritte; C prüft die Lizenzgrenze vor produktiver Datenhaltung.
