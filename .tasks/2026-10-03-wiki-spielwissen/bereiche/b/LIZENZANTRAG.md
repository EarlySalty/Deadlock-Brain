status: aktiv
Datum: 2026-10-03

# Autorisierte kostenlose Deadlock-Lizenzanforderung

AN_BEREICHE.md Punkt 17 ist gelesen und freigegeben. Der bestehende Rust-Handler wurde vor Ausführung vollständig geprüft: `free_license.rs:38-59` benutzt ausschließlich die bestehende `ctx.connection`, sendet `CMsgClientRequestFreeLicense` mit genau einer App-ID und führt weder Kauf noch Login oder Neustart aus. Nutzdaten sind explizit `app_id=1422450`.

Belegter Betriebsweg: `POST http://127.0.0.1:8782/tasks`, HTTP-Vertrag `EnqueueTaskRequest` mit Feld `type`, nicht `task_type`. Request:

```json
{"type":"AUTH_REQUEST_FREE_LICENSE","payload":{"app_id":1422450},"bot_account_id":1}
```

Nach erfolgreicher Einreihung liefert `GET /tasks/<zurückgegebene ID>` den Taskstatus und das Ergebnis mit `app_id`, `response.eresult`, `granted_appids`, `granted_packageids`. Ein positiver Taskstatus allein ersetzt keine Prüfung dieser Grants und keine gemessene Depotberechtigung. Es wird nicht anonym erneut heruntergeladen, weil das keine Rechte der verbundenen Botsitzung prüfen würde.

## Tatsächlicher Ausführungsblocker

Der Ausführungsversuch wurde vor jedem HTTP-Zugriff vom lokalen Bash-Hook auf context-mode umgeleitet. Der vorgeschriebene Aufruf von `ctx_execute` ist in dieser bereits laufenden Sitzung weiterhin im don't-ask-Modus verweigert. Die Änderungen an `startweg/claude-sol.json` sind nicht live übernommen. Deshalb wurde kein Task angelegt, keine Lizenz angefordert und kein Ergebnis oder EResult erfunden. Das ist eine Harnessgrenze, keine negative Steam-Antwort.

Die Sperre wird nicht durch versteckte Curl-Aufrufe, einen neuen HTTP-Helfer oder einen anderen Agenten umgangen. Eine geordnete Wiederaufnahme derselben tatsächlich beendeten eigenen Sitzung mit den aktuellen Sol-Settings ist der vorgesehene Weg aus Punkt 14; eine laufende Sitzung wird nicht per Resume dupliziert. Die unabhängige Rust-Fix-/Compilerarbeit läuft parallel weiter.

Aktuelle Zuständigkeit nach AN_BEREICHE.md Punkt 18: D übernimmt ausschließlich Lizenzanforderung, Steam-Zugang und regulären Spiel-Download in frischer Sol-high-Sitzung. B hat keinen Task angelegt und führt keine konkurrierende Anforderung oder weitere Zugangsversuche aus. Der oben beschriebene Wiederaufnahmepfad ist für den Lizenzvorgang damit durch die Übergabe an D ersetzt. B behält Parser, Compilerprüfung und vorhandene Datensätze; zusätzliche echte Rohdaten werden nach belegter Übergabe durch D extrahiert.
