status: erledigt
Datum: 2026-10-03

# Native Prüfung vor Lizenzaktion

Quelle: abgeschlossener nativer Worker a68a3580b952ec661, geerbtes gpt-6.1-sol. Der Worker lieferte die Abnahme direkt zurück und schrieb keine Datei. Teil-D hält diese Rückmeldung hier fest. Keine Lizenzaktion durch den Worker.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft

Urteil: ALLOW für den geprüften Quellpfad mit app_id=1422450 und bot_account_id=1. Der Handler free_license.rs:38 verwendet ausschließlich die vorhandene Steam-Verbindung, fordert genau eine kostenlose App-Lizenz an und führt keinen Kauf, Login oder Neustart aus. Ohne Verbindung bricht er ab. Antworttimeout: 30 Sekunden.

POST /tasks erwartet type, payload und bot_account_id und liefert id. Belegt durch rust/crates/api-contract/src/lib.rs:83 und rust/crates/steam-core/src/api/mod.rs:326 im Steam-Bot-Repo. GET /tasks/<id> liest nur das gespeicherte Ergebnis. Sichere Ergebnisfelder: id, status, result.ok, result.data.app_id und result.data.response mit eresult, granted_appids, granted_packageids. Eine Steam-Ablehnung bewahrt die Antwortdaten bei FAILED; Transportfehler können ohne data enden. DONE beziehungsweise EResult 1 allein ersetzt weder die Grantprüfung noch einen Depotdownloadnachweis.

Beide HTTP-Routen nutzen die bestehende Zugangsmiddleware. Ohne konfigurierten Schlüssel lässt diese Anfragen durch. Der Worker prüfte den Konfigurationszustand nicht und las keine Geheimwerte. Ein Zugangshindernis wäre eine konkrete API-Grenze, keine Aufforderung zum Umgehen.

SHA-256 der tatsächlich geprüften Quellen unter /home/nathanael/repos/Deadlock-Steam-Bot:

| Datei | SHA-256 |
| --- | --- |
| rust/crates/steam-core/src/task/handlers/free_license.rs | 0aca7c1b1d8b32440a59d5714ad3cc2c6d65bbca3ae531586a899b84145ff1b3 |
| rust/crates/steam-core/src/api/mod.rs | 2619c47435924ca32bef080e16ad5739309a22cb29ce55e625cd0aa9f6df1361 |

Graphify und lokale Regeln wurden berücksichtigt. Keine Secrets, Cargo-Aufrufe, eigene Git-Aktionen oder produktiven Änderungen. Die früher blockierte B-Sitzung hatte keinen Task angelegt.
