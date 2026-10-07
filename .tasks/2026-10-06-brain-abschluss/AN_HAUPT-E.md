# Paket E: Analyse abgeschlossen, Umsetzung wartet

## Kurzfazit, 07.10.2026 etwa 04:32 CEST

**API als Hauptquelle für versionierte Spielwerte: ja. Neue Spieldaten-Datenbank oder neuer Brain-Dienst: nein. Analytics als Ersatz für 100 echte Nach-Patch-Matches derselben Buildfamilie: nein.** Echte zusätzliche Matchdaten sind verfügbar; Import und Familienabnahme müssen den bestehenden Weg benutzen.

Vollständiger Vergleich, Bestandswege, Tabellen, Limits, Quellen und Umsetzungsplan: [E/DEADLOCK-API.md](E/DEADLOCK-API.md). Reproduzierbare Abfrageparameter und bereinigte Messwerte: [E/MESSUNG.json](E/MESSUNG.json).

## Entscheidende Belege

1. Der alte Assets-Host löst nicht auf; Main hat den Wechsel schon. `api.deadlock-api.com/v1/assets` liefert 40 aktive Helden und 746 Items/Fähigkeiten/Waffen, mit `client_version` und deutscher Lokalisierung. Jüngste gemessene Version 6759. Haze, Mystic Burst und Bullet Dance tatsächlich verfügbar. Historische Versionen funktionieren teilweise, aber Warden 5044 gab 404. Keine pauschale Vollständigkeit oder Minuten-SLA behaupten.
2. `/v2/patches` liefert 30 Forum-/Steam-Einträge. Minor Updates enthalten nutzbare Zeilen, große Updates teilweise nur Vorschau bzw. Link auf die Originalseite. `big-days` endet gemessen im März 2026. Vorhandene offizielle Linkauflösung, Parser, `patch_events` und `patch_changes`-View behalten.
3. Warden-Probe seit 05.10. 23:05:32 UTC: Analytics nennt bei Mindest-Badge 91 insgesamt 218 Fälle. Die echte Metadatenabfrage liefert 116 unterschiedliche Spieler-Match-Paare aus 73 Spielern, davon 115 gewertet mit passender Kaufstruktur. Keine Mitglieder-IDs abgefragt oder gespeichert. Das ist **kein** Beweis für 100 Fälle einer Familie nach dem aktiven Brain-Patch und darf nicht zu den bisherigen 47 addiert werden.
4. Zwei Hero-Analytics-Abfragen mit unterschiedlichen Uhrzeituntergrenzen am selben Tag lieferten identische Tageszahlen. Aggregate sind hier kein exakter Patch-Cutoff-Beleg. Der vorhandene Builddatenclient setzt außerdem keine explizite Matchzeituntergrenze, schreibt aber einen lokalen Patchtag. Der Populationweg existiert bereits; Timerfehler dort können durch `|| echo` bei Exit 0 verdeckt werden.
5. MCP funktioniert read-only, begrenzt Ergebnisse auf 1024 Zeilen/50 KB und liest stündliche Snapshots. Altes REST-SQL funktioniert noch, ist aber zur Entfernung angekündigt. Kein neuer produktiver SQL-/MCP-Connector empfohlen. Direkter Brain-DB-Zugriff auf 5446 war nicht erreichbar; Tabellenmapping ist codebelegt, aktuelle DDL/DB-Zahlen bleiben offen.

## Empfehlung für eine spätere Freigabe von E

Vorhandenen Assets-Ingest versionsgebunden machen, API-Patchfeed in den bestehenden Patch-Sync einhängen, Analytics-Zeiträume ausdrücklich binden. Eine Datenbasis in den vorhandenen Brain-Postgres-Tabellen, keine zweite Katalog-/Patch-/Populationpipeline. Erst nach Betriebsfreigabe gezielt echte Einzelmatches ab dem tatsächlich aktiven Patch nachholen und dieselbe Familienprüfung erneut laufen lassen. Die 100er-Grenze und alle übrigen Publishbedingungen bleiben unverändert.

**A bleibt Eigentümer der Steckbrief-/Patch-Story-Veröffentlichung und Korpusaktivierung.** Die bestehenden Quellenrevisionen, Faktenbindungen, Ableitungsquittungen und `corpus_releases_v1` wiederverwenden. API-Verfügbarkeit allein behebt die G1-Korpuslücke nicht.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/dbrain-sources/src/assets_api.rs:16 | Anknüpfung: Assets-Ingest, SourceStore, vorhandener Population-Import und gemeinsame Profil-/Korpusveröffentlichung

**Status:** Nur diese Akte geschrieben. Kein Produktcode, DB-Schreibzugriff, Main-Push, Release, Tick, Neustart oder neuer Thread. `VON_HAUPT.md` enthält noch keine ausdrückliche Paket-E-Umsetzungsfreigabe. E steht bis dahin still; Release-Hold gilt.
