status: aktiv
Datum: 2026-10-03
Stand: 2026-10-03T17:13:08Z

# C3: Steam-Vorstand für koordinierten Cutover

Read-only gemessen, kein Deploy oder Neustart. Current zeigt auf /opt/deadlock/steam/releases/4c5621763d5f01c96d7912400517c08aa1c40df1. Tatsächliche App-Prozesse über eigene Unit-MainPIDs und deren Kinder identifiziert, nicht sudo als App gewertet.

| App | Tatsächliche PIDs | SHA256 der laufenden Datei |
| --- | --- | --- |
| steam-core | 3424075, 1329694 | ddfa57277e243d051deb07d44b787f2c7f01cf31f93920bc46c2ada944eeb349 |
| steam-bot | 3424100 | 711c7f520c15e2211f571f169c8c780c0d0c8e9651bcc932160b478ceb7d3987 |

Exe-Pfade zeigen ohne deleted auf diesen Release, Hashes stimmen mit den dortigen Appdateien überein. sudo-/Launcher-Exe nicht lesbar; daraus kein Launcher-Binarybeweis. SOURCE_SHA fehlt im vorhandenen Release: Verzeichnisname und Apphash sind kein zusätzlicher Buildprovenienzbeweis.

Normale zz-private-start.conf der drei Steam-Units gelesen: Launcherpfad /opt/deadlock/steam/bootstrap/fe27ff5014db3781/dl-infisical-env, danach Apps über /opt/deadlock/steam/current; Core-Accounts 1/2 getrennt, bisherige Configpfade und Argumente erhalten. Keine Credential- oder Konfiginhalte gelesen, keine Prozessumgebungen. NRestarts gemessen: 0/1/0; kein Readiness- oder Gesundheitsurteil daraus.

Root meldet Launcherfix ae490cd auf Bots-main und akzeptiertes Diagnosepaket aeadf118 auf Basis 4c562176, extern noch nicht deployed. Diese Meldungen ersetzen keine eigene Cutover-Abnahme. D/B/C-Endstände und Steam-Eigenanteile vor Release über Root mit Z abstimmen. Direkt vor Wechsel unter Installationssperre Current, laufende Apphashes und geladenen Startvertrag neu prüfen; bei Abweichung abbrechen und neu koordinieren. Keine konkurrierenden Zeigerwechsel, keine neue Steam-Sitzung oder Grantwiederholung. Cleanup erst am Ende.
