status: aktiv | 2026-09-29

# C1: Selbst-Gate, erste Runde

1. `rust/crates/deadlock-brain/src/steam_web_api.rs:120`: Nach einem Prozessabbruch mit unklarer Steam-Antwort durfte die Reservierung nicht als Transportfehler gemeldet werden. Behoben durch dauerhaft gespeicherten Versandbeginn und eine Sperre für unbekannte Antworten, auch bei Cache-Treffern. Der PostgreSQL-Test prüft den Zustand nach erneuter Verbindung.
2. `rust/crates/deadlock-brain/src/steam_web_api.rs:166`: Die Freigabe der Advisory-Sperre verwendete `execute` für eine `SELECT`-Abfrage und verwarf Fehler. Auf `query_one` mit geprüftem Rückgabewert und Fehlermeldung umgestellt. Der PostgreSQL-Test prüft die Freigabe bei noch offener erster Verbindung.
3. `rust/crates/deadlock-brain/src/steam_web_api.rs:14`: Der lokale Transport war im Review nicht durch die Diensttopologie belegt. Auf dem Diensthost sind der Brain-Patchnote-Dienst und `dl-bot` vorhanden; `dl-bot` lauscht auf `127.0.0.1:8901`. Kein externer Proxy ergänzt.

Zweite Gate-Runde steht aus.
