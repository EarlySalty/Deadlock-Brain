# A-F3: tatsächlicher Sitevertrag und Eigentumsblocker

Native Rückgabe von A-F3, 06.10.2026. Kein Code geändert, Worktree `/home/nathanael/.worktrees/brain-a-site-20261006` sauber auf `d6131cc52711a3e8b02d299704244f8d7dbdbce6`.

## Korrektur der ersten Inventur

Die tatsächliche öffentliche URL `https://deutsche-deadlock-community.de/brain/site/` liefert HTTP 200. `/brain` und `/brain/` leiten mit 308 dorthin um. Caddy entfernt `/brain`, der Backendpfad ist `/site/`. Ein direkter lokaler `/brain`-404 ist daher erwartbar und kein Defekt. Öffentliches und lokales HTML bytegleich, 2755 Bytes, SHA-256 `cc1731726a10541060a41ac96451e0e67ea82c8279592756d3f50c0b22ad084d`.

Steckbriefseiten fehlen tatsächlich: Renderer schreibt atomar `site/entities/<hero|ability|item>/<hex-Schlüssel>.html`, öffentliche URL `/brain/site/entities/...`; der Ausgabeordner fehlt noch. A-F1 besitzt die vorgelagerte Profilverarbeitung.

## Vorhandener Dienst

Userunit `/home/nathanael/.config/systemd/user/deadlock-brain-site.service`, Pythonserver `/home/nathanael/Documents/deadlock-build-corpus/site/server.py`, Loopback 8087. Nach globalem Graph und Prüfung von 201 Git-Referenzen sowie 86 Brain-Worktrees kein Rust-Siteweg gefunden. Oberfläche: Helden-Dossiers, Helden-/Item-Meta, Stärkeberichte, Methodik und Kommentare. Rust-Port muss diese relevanten Funktionen erhalten, bestehende Assets/Renderer verwenden.

`app.js` verwendet GET und POST `/api/comments`. Python speichert append-only `/home/nathanael/Documents/deadlock-build-corpus/comments.json`. Aktueller Bestand in Datei und API null Kommentare. Das erlaubt keine Entfernung der Nutzerfunktion. Bestehende Antworten `{key: [{text, ts}]}` und `{ok, comments}`, Standardgruppe `general`, Grenzen 4000 Zeichen Text/40 Zeichen Gruppe. In 22 SQL-Dateien mit 41 Tabellen keine passende Kommentar-/KV-Persistenz gefunden.

## Nötige Eigentumserweiterung

Neue PostgreSQL-Persistenz samt registrierter Migration/Schemaanbindung erforderlich. Diese Dateien waren nicht im F3-Eigentum; Worker stoppte deshalb richtig vor Mutation. Kein Architektur- oder API-Wechsel nötig, PostgreSQL ist verbindlich. Paket A weist die eng auf Kommentare begrenzte Migration/Registrierung zu und setzt Port im selben eigenen Worktree fort.

## Sicherheitsgrenze

Altserver erlaubt `HEAD /site/server.py` mit HTTP 200. Rust-Port braucht einen ausdrücklichen öffentlichen Dateivertrag, keine freie Corpus- oder Quellcodeauslieferung. Raw-Originale, interne Pfade, Secrets und wörtliche Wikibestände bleiben intern. Neun read-only HTTP-Prüfungen, keine Compiler-/Test-/Gateprüfung mangels Codeänderung.
