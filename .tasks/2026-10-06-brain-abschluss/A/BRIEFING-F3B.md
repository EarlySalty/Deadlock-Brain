# A-F3b: Site-Port mit PostgreSQL-Kommentaren fortsetzen

Die Rückgabe `F3-RUECKGABE.md` ist angenommen. A-F3 hat den Eigentumsblocker korrekt gemeldet und ist abgeschlossen. Frischer nativer Blatt-Implementierer übernimmt denselben sauberen Worktree `/home/nathanael/.worktrees/brain-a-site-20261006` und Branch `fix/brain-a-site-20261006`. Bestehendes `BRIEFING-F3.md` gilt mit dieser begrenzten Erweiterung. Nicht neu starten oder die 201 Git-Refs erneut ablaufen: fachlichen Befund an den konkreten Fundstellen bestätigen und weiterbauen.

## Eigentumserweiterung

Zusätzlich darf dieser Worker die neue PostgreSQL-Kommentarpersistenz im bestehenden Brain-Migrationsweg umsetzen: eigene neue SQL-Migration, ausschließlich deren vorhandene Registrierung in `brain-storage`/`brain-migrate` und die genau nötigen bestehenden Grantdefinitionen. Neue Migrationskennung erst nach frischem Fetch. Bereits angewandte SQL-Dateien bleiben bytegleich. Keine andere Tabelle, globale Schemaänderung oder fremde Migrationsnummer belegen. Diese Dateien besitzt aktuell kein anderer Paket-A-Worker. Wenn ein tatsächlich fremder neuer Stand kollidiert, vor Mutation melden.

PostgreSQL ist verbindlich und keine offene Architekturfrage. Kommentar-API und bestehende Nutzerfunktionen erhalten: GET `{key: [{text, ts}]}`, POST `{ok, comments}`, Standardgruppe `general`, 4000 Zeichen Text und 40 Zeichen Gruppe, append-only Reihenfolge. Kein SQLite, kein produktiver JSON-Schreibweg, kein Weglassen bei leerem Altbestand. Bestehende PostgreSQL-Konfigurations-/Secretlader nutzen. Rechte auf genau die nötige Kommentar-Tabelle und Sequenz begrenzen, keine allgemeinen Ingest-/Adminrechte als bequemes Sitekonto vergeben. Zugangsdaten nie in Code, Migration, Artefakt oder Ausgabe. Tatsächlich nötige neue Betriebsfreigabe beziehungsweise nicht verfügbare Secretprovisionierung präzise an Paket A geben, nicht die Grenze umgehen.

## Umsetzung und Beweis

Python nur lesen. Bestehende UI/Assets, Dossiers, Meta, Stärkeberichte und Methodik weiterverwenden. Aktuelle echte URL ist `/brain/site/`; Backend `/site/`, kein neuer `/brain`-Handler nötig. Vorhandener Renderer liefert später `/brain/site/entities/...`, vorgelagert A-F1. Öffentliche Dateiverträge aus den tatsächlich verwendeten Assets ableiten; Quellcodeauslieferung wie `HEAD /site/server.py` im alten Server ist kein zu erhaltendes Nutzerfeature. Interne Originale, Pfade und Raw-Wikitexte sperren.

Normale Compiler-, fmt-, Clippy- und bestehende passende Prüfungen mit realem lokalem HTTP-Server und eigenem Scratch-Postgres. Kommentar-POST/Persistenz/GET auf diesem isolierten echten Datenpfad beweisen, keine öffentlichen Testkommentare erzeugen. Migration, Rollen und Deployment werden erst durch Paket A nach Main ausgeführt. Arbeit in Source/Tests belegt prüfen, Feature committen und pushen, normales Gate gegen frisches Main; BLOCK vollständig zurückgeben für frischen Fixer. Kein Main/Deploy/Restart durch F3b. Kommentar-API nicht ändern und keine neue Moderations-/Loginfunktion bauen.

Native vollständige Rückgabe mit Worktree/Branch/SHA, Umfang und Migration/Grantbedarf, Prüfungen/Zahlen, Gatewortlaut/Modell/Exit/Log und konkretem Installations-/Dienstvertrag. Root-Akte nicht schreiben, keine weiteren Agenten oder T3-Threads. Statusproduzent F3b, Wache nach 20 Minuten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-site-20261006
