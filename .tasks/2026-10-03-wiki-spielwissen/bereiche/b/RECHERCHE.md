status: erledigt
Datum: 2026-10-03

## Rechercheauftrag B

Lesende Bestandsprüfung im nativen GPT-6.1-Sol-Worker. Kein produktiver Code, kein Compiler, kein Commit, keine Secrets gelesen und keine Dienste angefasst. Der Hauptcheckout und alle bestehenden Daten bleiben unverändert.

## Lokaler Spiel- und Werkzeugbestand

Die geprüften üblichen Installationspfade existieren nicht: `~/.steam`, `~/.local/share/Steam`, `~/Steam`, `~/.var/app/com.valvesoftware.Steam`, `~/.config/Steam`, `~/.wine/drive_c/Program Files (x86)/Steam`, `/opt/steam`, `/opt/steamcmd`, `~/steamcmd`, `~/.local/share/steamcmd`. Auch `/usr/games/steam`, `/usr/games/steamcmd`, `/usr/bin/steam`, `/usr/bin/steamcmd` und `/usr/local/bin/steamcmd` fehlen.

`command -v` fand keine ausführbaren Programme für `steam`, `steamcmd`, `DepotDownloader`, `dotnet`, `vpk` oder `Source2Viewer`. In `Deadlock-Brain/data` wurden weder `.vpk` noch `.vdata` gefunden. Eine unbeschränkte Suche im gesamten Home-Verzeichnis lief in einen Timeout; deshalb bedeutet diese Prüfung keine vollständige Abwesenheitsbehauptung für sämtliche Datenträger.

`~/.local/share/deadlock` enthält Community-Bot-Releaseartefakte, keine hier belegte Spielinstallation. `/opt/deadlock/steam` ist der vorhandene Botdienst. Die reine Prozessnamenprüfung zeigte laufende `steam-core` und `steam-bot`, aber keinen Steam-Client, SteamCMD, DepotDownloader oder Source2Viewer. Keine Prozessumgebung oder vollständige Argumentliste wurde gelesen.

Der vorhandene Steam-Bot besitzt eine über Infisical betreute Steam-Sitzung. `Deadlock-Steam-Bot/README.md` nennt localhost 8782 für `steam-core` und 8783 für `steam-bot`. Eine gezielte Rust-Suche nach `DepotDownloader|steamcmd|download_depot|GetManifest|GetCDN|ContentServer|depot|manifest` blieb leer. Ein vorhandener GC-/Profil-/Builddienst beweist keinen Depotdownloadpfad oder nutzbare Spielelizenz. Die laufenden Dienste wurden weder gestoppt noch für einen Login verwendet.

## Bereits vorhandene abgeleitete Daten

Pfad: `/home/nathanael/repos/Deadlock-Brain/data/external/deadlock-data`.
Git-SHA: `0d46cdecfccf77adec16aac01af6d30173e0ebb8`, Arbeitsbaum bei Prüfung sauber.
Quelle: [deadlock-wiki/deadlock-data](https://github.com/deadlock-wiki/deadlock-data).
README: Daten automatisch mit [deadlock-wiki/deadbot](https://github.com/deadlock-wiki/deadbot) erzeugt und für deadlock.wiki verwendet.
Lizenz im Repository: MIT, Copyright 2024 deadlock-wiki. Die Lizenz des Generatorcodes oder Datenrepos darf nicht pauschal als Lizenz für Valve-Assets ausgegeben werden.

`data/version.txt` enthält `ClientVersion=6731`, `ServerVersion=6731`, `appID=1422450`, `ServerAppID=1422460`, `ToolsAppID=211`, `SourceRevision=11070267`, Datum `Oct 01 2026`, Zeit `15:31:54`. Diese Werte sind keine belegten Steam-Depotmanifest-IDs oder Steam-Build-ID.

Vorhanden sind 17 JSON-Sammlungen mit 4.111.617 Bytes, zwei CSV-Dateien mit 312.494 Bytes und 29 Lokalisierungsdateien mit 19.622.148 Bytes. Zwölf Lokalisierungsdateien sind leere JSON-Objekte. Die Einträge sind veröffentlichte Ableitungen und kein eigener Spielclientdownload.

| Sammlung | Oberste Einträge | Relevanz |
| --- | ---: | --- |
| ability-data.json | 358 | Fähigkeiten und Rohwerte |
| hero-data.json | 68 | Heldendefinitionen, auch interne und unbestätigt spielbare Einträge |
| item-data.json | 279 | Item-/Upgrade-Definitionen, auch Basisklassen |
| npc-data.json | 90 | Einheiten und Ziele |
| generic-data.json | 51 | Bewegung, Ökonomie und allgemeine Konfiguration |
| misc-data.json | 106 | Pickups und weitere Definitionen |
| convars.json | 4.255 | Konfigurationswerte, keine belegte Laufzeitwirkung |
| hero-meaningful-stats.json | 53 | vom Generator ausgewählte Statistiknamen |
| soul-unlock-data.json | 36 | Fortschrittsdaten |
| attribute-data.json | 3 | Attributgruppen |
| item-investment-data.json | 1 | Iteminvestitionen |
| midtown-metadata.json | 2 | begrenzte Kartenmetadaten |
| ability-cards.json | 68 | präsentationsnahe Ableitung |
| item-cards.json | 279 | präsentationsnahe Ableitung |
| resource-lookup.json | 432 | Zuordnung, keine eigenständige Mechanik |
| street-brawl-data.json | 2 | separates Spielmodusprofil |
| stat-infobox-order.json | 4 | Darstellung |

Repräsentative SHA-256:

- version.txt: `cacaf4c7b4def9dbde39533a2cf161c4379e6121368e4879f8c1ee0d6ea2be3d`
- generic-data.json: `f8c4601602ee89217ba45f7404230b2278abfa5b5f54ba30109a2cfa77805e29`
- hero-data.json: `5bfc1e4fe3f35dfaefd49e814f8eb410bb3e42243a2e9233f0f7434d8a5cc2df`
- npc-data.json: `e794b4cbf66d094da16742ac24fa342a492d1878458e56e94498ffad9624f42f`
- item-data.json: `992743a93afb8a671d76b460da5ca65acd2c48e889598f0d37aa8012819b8bfc`
- ability-data.json: `ee0d29838f0bedd81efdc3e841d4f8c30c60477e31e630dcfc107cece6f954e1`

## Erreichbare veröffentlichte Spieldateien

Die GitHub-API war ohne Ausgabe von Zugangsdaten erreichbar. [SteamDatabase/GameTracking-Deadlock](https://github.com/SteamDatabase/GameTracking-Deadlock) bietet veröffentlichte, aus dem Spielbestand abgeleitete Dateien. Es ist ein Drittanbieterarchiv und keine offizielle Valve-Downloadquelle.

Bei der Prüfung: Commit `4c6431ccdb816d2911bbbaa4335169cc34140386`, Commitzeit `2026-10-03T00:53:23Z`. Die rekursive Git-Tree-Antwort war `truncated=false`: 14.352 Dateien, darunter 108 `.vdata`-Dateien. Unter `game/citadel/pak01_dir/scripts/` liegen 97 Dateien mit insgesamt 11.260.351 Bytes. Der Baum enthält zehn Pfade mit Kartenbezug; diese Zählung beweist keine vollständige Kartenabdeckung.

Besonders relevante konkrete Pfade:

- `game/citadel/pak01_dir/scripts/abilities.vdata`, `heroes.vdata`, `generic_data.vdata`, `misc.vdata`, `npc_units.vdata`, `modifiers.vdata`, `scale_functions.vdata`, `loot_tables.vdata`.
- `game/citadel/pak01_dir/scripts/items/items_game.txt`, `bots/bot_difficulty.vdata`, `nav_hulls.vdata`, `nav_hulls_presets.vdata`, `navlinks.vdata`, `collision_properties.txt`, `surfaceproperties_game.txt`.
- `game/citadel/pak01_dir/scripts/tarot/` und `ranked_seasons.vdata`; als eigene Definitionsklassen behandeln.
- `game/citadel/resource/`, `game/citadel/pak01_dir/resource/` sowie Sprachverzeichnisse wie `game/citadel_german/`.
- `game/citadel/steam.inf`, `game/citadel/gameinfo.gi`, `game/citadel/gameinfo_branchspecific.gi`, `game/citadel/pak01_dir.txt` und das top-level `files.json` für Bestand und Versionskontext.

Die per API gelesene `game/citadel/steam.inf` enthält `ClientVersion=6745`, `ServerVersion=6745`, `appID=1422450`, `ServerAppID=1422460`, `ToolsAppID=211`, `SourceRevision=11078118`, `VersionDate=Oct 02 2026`, `VersionTime=17:45:04`. Der Stand unterscheidet sich damit ausdrücklich vom lokalen deadlock-data-Stand 6731. Nicht unter einer gemeinsamen Spielversion vermischen.

Im rekursiven Baum wurde keine Root-Datei LICENSE oder COPYING gefunden, nur `game/thirdpartylegalnotices.txt`. Öffentlich erreichbar bedeutet keine pauschale Erlaubnis, geschützte Assets zu veröffentlichen. In dieser Recherche wurden lediglich Metadaten und die kleine Versionsdatei gelesen. Es gab keinen großen Download und keine öffentliche Weiterverteilung.

## Offizielle Serverquelle und tatsächliche Grenze

Die lokale veröffentlichte Versionsdatei und das aktuelle Tracking nennen ServerAppID 1422460. Eine App-ID im Versionsfile beweist weder einen verfügbaren Dedicated-Server noch Zugang per Anonymous-SteamCMD. Die Suchprüfung fand dafür keine autoritative Bestätigung.

[Valve: Distributing Your Dedicated Game Server](https://partner.steamgames.com/doc/sdk/uploading/distributing_gs?l=english) erklärt, dass anonyme Downloads die Aufnahme der Server-App in das Paket 17906 voraussetzen. [Steam Support: standalone dedicated server](https://help.steampowered.com/en/faqs/view/5E9C-65C2-C968-98DC) beschreibt das offizielle Verfahren. [Deadworks Server Hosting](https://docs.deadworks.net/guides/server-hosting/) benutzt `app_update 1422450 validate`, ohne anonymen Zugang zu belegen. [Steam-Store Deadlock](https://store.steampowered.com/app/1422450/) nennt die Client-App.

Deadbots README beschreibt die Verwendung von GameTracking-Deadlock, DepotDownloader für nichtenglische Lokalisierungen, eigene Eingabepfade über `--dldir` und Decompile/Parse-Schritte. Diese Python-Anwendung wurde nur als Legacy-/Provenienzreferenz gelesen und weder ausgeführt noch erweitert. Binäre Spiellogik und Laufzeitwirkung werden durch dieses Dateninventar nicht bewiesen.

## Geprüfte Grenze des Rechercheauftrags

Kein vorhandener nutzbarer Depotdownloadclient gefunden; berechtigter Steam-Clientdownload bleibt unbelegt. Die veröffentlichte GameTracking-Quelle ist dagegen erreichbar und hat ausreichend kompakte Gameplay-Rohdefinitionen für eine getrennt versionierte, lokale Auswertung. Build-/Depotmanifest-IDs fehlen in den hier geprüften Quellen und müssen als unbekannt erhalten bleiben. Kein CONTRACT.md in dieser Recherche verändert.
