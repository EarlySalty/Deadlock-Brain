status: aktiv | 2026-09-29

# C1: Selbst-Gate, Runden 1 bis 4

1. `rust/crates/deadlock-brain/src/steam_web_api.rs:120`: Nach einem Prozessabbruch mit unklarer Steam-Antwort durfte die Reservierung nicht als Transportfehler gemeldet werden. Behoben durch dauerhaft gespeicherten Versandbeginn und eine Sperre für unbekannte Antworten, auch bei Cache-Treffern. Der PostgreSQL-Test prüft den Zustand nach erneuter Verbindung.
2. `rust/crates/deadlock-brain/src/steam_web_api.rs:166`: Die Freigabe der Advisory-Sperre verwendete `execute` für eine `SELECT`-Abfrage und verwarf Fehler. Auf `query_one` mit geprüftem Rückgabewert und Fehlermeldung umgestellt. Der PostgreSQL-Test prüft die Freigabe bei noch offener erster Verbindung.
3. `rust/crates/deadlock-brain/src/steam_web_api.rs:14`: Der lokale Transport war im Review nicht durch die Diensttopologie belegt. Auf dem Diensthost sind der Brain-Patchnote-Dienst und `dl-bot` vorhanden; `dl-bot` lauscht auf `127.0.0.1:8901`. Kein externer Proxy ergänzt.

Runde 2 bestätigte die ersten drei Punkte als behoben. Neuer Befund: `scripts/migrations/2026-09-29-steam-web-api-journal.sql:6` ergänzte `dispatch_started` nicht in einer bereits vorhandenen Journal-Tabelle. Die Migration ergänzt die Spalte jetzt wiederholbar, markiert Altzeilen mit unbekanntem Versand konservativ als begonnen und stellt den Standard für neue Zeilen auf `false`. Der Test prüft die Aktualisierung einer alten Tabelle und das Verhalten nach erneutem Verbinden.

Runde 3: `ALLOW`. Der Kritiker bestätigte die Aktualisierung bestehender Tabellen und keine blockierende Regression im Fix-Diff. Der erneute Gate-Lauf auf dem Dokumentationscommit blockierte dennoch: Bei fehlgeschlagenem Journaleintrag und fehlgeschlagener Ausgleichsmeldung konnte eine Reservierungs-ID nach Neustart verloren gehen. Vor jeder Reservierung wird nun ein langlebiger Sperrvermerk geschrieben. Er bleibt bei unbestätigter Ausgleichsmeldung erhalten und verhindert weitere Steam-Aufrufe; ein PostgreSQL-Test prüft diese Neustartsperre. Vierte Gate-Runde steht aus.
