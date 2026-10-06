status: aktiv
Datum: 2026-10-03

# Tatsächlicher kostenloser Lizenzvorgang

POST http://127.0.0.1:8782/tasks wurde nach nativer ALLOW-Prüfung am 2026-10-03T04:42:21.585Z exakt einmal über das freigegebene context-mode-Werkzeug ausgeführt. Request:

```json
{"type":"AUTH_REQUEST_FREE_LICENSE","payload":{"app_id":1422450},"bot_account_id":1}
```

HTTP 200 lieferte id 4931460. GET /tasks/4931460 am 2026-10-03T04:42:32.885Z antwortete HTTP 200:

```json
{"id":4931460,"status":"DONE","result":{"ok":true,"data":{"app_id":1422450,"response":{"eresult":1,"granted_appids":[1422450],"granted_packageids":[]}}}}
```

Der reguläre kostenlose Anforderungstask wurde tatsächlich ausgeführt. Steam meldet EResult 1 und gewährt App 1422450. Damit ist die konkrete Appgewährung gemessen. Die leere Paketliste beweist keine fehlenden Depotrechte. Das Ergebnis enthält weder Depot-ID, Depotkey, Manifest-ID, Steam-Build-ID noch Dateiinventar und belegt keinen vollständigen Spieldownload. Eine zusätzliche App-/Depotberechtigungsabfrage über einen bestehenden Betriebsweg bleibt nötig.

Die geheimnisfreie Beobachtung liegt unter /home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/d/license-task-4931460.json. Gespeichert wurden ausschließlich Request, HTTP-Status, Taskkennung und die oben gezeigten Ergebnisfelder. Ungeprüfte Fehlertexte, fremde Tasks und vollständige Kontoausgaben wurden nicht gelesen oder gespeichert.

## Standbindung der geprüften Quellen

/opt/deadlock/steam/current zeigt auf /opt/deadlock/steam/releases/4c5621763d5f01c96d7912400517c08aa1c40df1. SHA-256 des dortigen steam-core: ddfa57277e243d051deb07d44b787f2c7f01cf31f93920bc46c2ada944eeb349. steam-bot: 711c7f520c15e2211f571f169c8c780c0d0c8e9651bcc932160b478ceb7d3987. Das bezeichnet den Releasepfad und dessen Dateien, keinen bewiesenen Prozesscommit.

Die per git show unter diesem Releasebezeichner gelesenen Quellen haben dieselben Hashes wie die zuvor geprüften B-Quellen: free_license.rs 0aca7c1b1d8b32440a59d5714ad3cc2c6d65bbca3ae531586a899b84145ff1b3, api/mod.rs 2619c47435924ca32bef080e16ad5739309a22cb29ce55e625cd0aa9f6df1361, task/mod.rs 63f21300094c488cf56ab4c8ff2ac2be77dbfa101befca07d3a3b10568a8e3d3. Eine reproduzierbare Binär-zu-Quellstandbindung wurde dadurch nicht behauptet.

## Sicherheitsgrenzen

Kein Kauf, keine andere App angefragt, kein Loginwechsel, kein Neustart, keine Klartext-Secrets, keine Credential-/ENV-Dateien, keine Compilerläufe. Bestehende Botsitzung über den vorhandenen Handler genutzt. Die unabhängige Ergebnis-/Sicherheitsabnahme läuft separat. Die erste echte Steam-Antwort ist positiv; fehlende Depotrechte werden nicht behauptet.
