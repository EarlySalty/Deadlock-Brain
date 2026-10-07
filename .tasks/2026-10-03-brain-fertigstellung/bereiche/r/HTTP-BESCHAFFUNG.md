status: aktiv
Datum: 2026-10-03

# Öffentliche Replay-Beschaffung

Am 03.10.2026 war genau eine Demo in der begrenzten Prüfung von 30 unterschiedlichen gespeicherten Match-Salts verfügbar. Alle Salts-Aufrufe setzten `disable_steam=true`. 29 weitere geprüfte Matches lieferten HTTP 200 mit `demo_url=null`. Keine GC-Anfragen, kein Versuch, fehlende Salts zu erzwingen.

Die öffentlichen Routen sind im OpenAPI-Vertrag unter `https://api.deadlock-api.com/openapi.json` dokumentiert. `GET /v1/matches/demo/schema` ohne Match-Parameter nennt die zuletzt vorhandene Demo; ein Match-Parameter wurde nicht verwendet, weil dieser einen Steam-Fallback auslösen könnte.

## Verfügbare Demo

- Match: `110001910`.
- Identitätsbeleg: `https://api.deadlock-api.com/v1/matches/110001910/salts?disable_steam=true`, HTTP 200 mit passender numerischer Match-ID und Demo-URL.
- CDN: `http://replay271.valve.net/1422450/110001910_1897304728.dem.bz2`. HEAD 200 und Range 206, keine Umleitung.
- Vollständiger Download um etwa 15:20 UTC: HTTP 200, 45.014.647 Bytes. Obergrenze 128 MiB, kein Konto oder Schlüssel erforderlich.
- Komprimierter SHA-256: `3fe2095e66c4d6c1b8ac2ac58898dd84662c6e970d373556c4d2cd1566d77443`.
- Tatsächliche Kompression: Zstandard, Signatur `28b52ffd`. Die Dateiendung `.bz2` beschreibt den Inhalt nicht korrekt. Der alte Bzip2-Versuch verwarf seine Eingabedatei nach dem Integritätsfehler; kein Decoder-Erfolg wurde daraus abgeleitet.
- Privater Zwischenstand: `/tmp/brain-replay-r-20261003/public-demo-110001910-worker/110001910.compressed`, Verzeichnis 0700, Datei 0600. Nach Decode und Wiederholungsnachweis entfernen.

## Geprüfte Grenzen

Die jüngsten neun vorher geprüften Matches und weitere 20 ältere Matches aus `/v1/matches/recently-fetched` hatten keine gespeicherte Demo-URL. Die Stichprobe liegt damit unter den angestrebten 10 bis 50 Demos. Weitere GC- oder Brute-Force-Beschaffung wird nicht betrieben. Ein eigenes vom Nutzer beigebrachtes Match liegt nicht vor. Der bestehende Steam-GC-Weg wurde lesend geprüft, nicht aufgerufen.

Download und Kompressionsbeleg beweisen noch keine erfolgreiche DEM-Dekodierung, keine feldweise unabhängige Referenzprüfung und keine Store-/Abfragewirkung. Diese Nachweise folgen gesondert. Die Daten bleiben lokal, externe Weitergabe und Veröffentlichung sind nicht freigegeben.
