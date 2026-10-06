status: erledigt
Datum: 2026-10-03
Beobachtung: 2026-10-03T04:37:55Z bis 04:38:46Z

# Bestehende Downloadwerkzeuge und Installation

Die B-Berichte wurden übernommen; anonyme SteamCMD-Fehlversuche wurden nicht wiederholt. Der globale Graph wurde zuerst nach Steam-Installation, Depotdownload, IPC und SteamCMD befragt. Seine Treffer waren überwiegend fachfremd und belegen keinen Depotdownloadweg. Der Brain-Worktree enthält keinen eigenen Graphen; der globale Graph bleibt verfügbar.

## Gezielte lokale Nachprüfung

Die üblichen Pfade ~/.steam, ~/.local/share/Steam, ~/.var/app/com.valvesoftware.Steam, ~/.config/Steam, /opt/steam, /opt/steamcmd, ~/steamcmd und ~/.local/share/steamcmd existieren weiterhin nicht.

~/Steam existiert jetzt mit appcache, config, libraryfolder.vdf, logs, steamapps und userdata. Die beiden libraryfolders.vdf unter config und steamapps nennen ausschließlich /home/nathanael/Steam. App 1422450 und 1422460 stehen dort nicht. SHA-256 beider Bibliotheksdateien: 4c9d44c0753e82289f96bd05f8afb970c432d6f3567b24ec59220010bf08f2e0. SHA-256 der root-libraryfolder.vdf: 17fcdf20765377d92a64a07c49e2007f492773c978fce8bc6b0df62e8f95ba96.

Eine auf drei Verzeichnisebenen begrenzte Suche in ~/Steam, /opt/deadlock/steam und der vorhandenen SteamCMD-Installation fand keine Deadlock-appmanifest_1422450.acf beziehungsweise appmanifest_1422460.acf, keine VPK-Spieldatei und kein Spieldepotmanifest. Die einzige manifest-Datei im Werkzeugbestand ist package/steam_cmd_linux.manifest und gehört zum SteamCMD-Werkzeug. Symlink-Ziele wurden nicht rekursiv verfolgt. Diese begrenzte Prüfung ist kein Abwesenheitsbeweis über sämtliche Datenträger.

Die vorhandene offizielle SteamCMD-Installation liegt unverändert unter /home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/tools/steamcmd. command -v/which findet steam, steamcmd, DepotDownloader, dotnet und Source2Viewer nicht im PATH. Es wurde kein neues Werkzeug installiert oder kompiliert.

Die reine Prozessnamensprüfung zeigt zwei steam-core und einen steam-bot, jedoch keinen Steam-Client, SteamCMD, DepotDownloader, dotnet oder Source2Viewer. /proc/net/unix enthält keine Steam-/Depot-benannten Unix-Sockets. Das schließt anders benannte IPC oder interne Handles nicht aus; ein vorhandener offizieller Steam-Client-IPC-Download ist dadurch nicht belegt. Keine Prozessargumente, Prozessumgebungen oder Sitzungsdateien gelesen.

## Dienst vor Lizenzvorgang

GET http://127.0.0.1:8782/health und /status antworteten um 04:38:46Z jeweils HTTP 200, version 0.1.0, steam_connected=true und gc_connected=true. Ausgabe war auf diese Felder begrenzt. Beide Werte belegen die gemeldeten Verbindungshandles, keine App- oder Depotberechtigung. Es wurde keine Produktions-SHA aus version 0.1.0 abgeleitet.

Der bereits geprüfte Core-Router zeigt Leserouten, POST /tasks und GET /tasks/{id}, Scrim- und Observer-Verfahren, jedoch keinen separaten Spieldepotdownloadendpunkt. Eine nicht dokumentierte URL wird nicht als Downloadroute geraten. Ohne vorhandenen berechtigten Werkzeugweg wird kein zweiter Steam-Login gestartet.

Die Prüfung las ausschließlich normale Bibliothekspfade und gezielte Quellen. Credential-, Token-, ssfn-, loginusers-, config.vdf- und ENV-Inhalte blieben unangetastet. Vorhandene Bibliotheken, Botsitzungen, Installationen und B-Daten wurden nicht verändert.
