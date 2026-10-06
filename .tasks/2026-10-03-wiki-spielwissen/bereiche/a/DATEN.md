status: aktiv
Datum: 2026-10-03

# Gesicherte Wiki-Daten

Dauerhafter Datenordner außerhalb Git:
`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/`

## Vorhandener historischer Bestand

`legacy-raw/` enthält zwölf unveränderte echte MediaWiki-Antworten. Der Vergleich `diff -qr` gegen `/home/nathanael/repos/Deadlock-Brain/data/raw/deadlock_wiki` blieb leer. Originaldateien wurden nicht verändert. Größe der Kopie: 224 KiB laut `du -sh`.

`cache-provenance/` enthält die zwölf zugehörigen Original-HTTP-Payloads und zwölf Cache-Sidecars. Die Recherche hat die Zuordnung anhand tatsächlicher Titel, Seiten-ID und Revisions-ID geprüft. `fetched_at` belegt die Abrufe vom 02.05.2026. Revisionszeit, damals erfolgter Abruf und heutige lokale Sicherung sind getrennte Angaben.

Tatsächliche Titel: Abilities, Crowd Control, Damage Resistance, Items, Boon, Mechanics, Mo & Krill, Souls, Stats, Status Effects, The Curiosity Shop und Weapon Damage. Alle liegen im Namespace 0. Es sind zwölf verschiedene Seiten-IDs mit jeweils einer bekannten Revision. Drei Redirectzuordnungen sind zusätzlich vorhanden, aber keine eigenen Redirectrevisionen.

Die Rohdateien enthalten identifizierten Wikitext und einen Klartextauszug. Der Auszug ist nicht mit allen eingebundenen Template-/Modulrevisionen gepinnt. Fehlende Revisionsabhängigkeiten müssen in der normalisierten Ausgabe erhalten bleiben. Diese historischen Quellen belegen keine aktuellen Spielwerte oder vollständige aktuelle Wiki-Abdeckung.

## Neue öffentliche Quellenprobe

`source-evidence/` enthält sieben Originaldateien aus der öffentlichen Quellenprüfung. Sie wurden vom Rechercheworker bytegleich aus dem temporären Auftragsordner gesichert. Darunter sind die erfolgreiche siteinfo-JSON-Antwort, zugehörige Header, Robots-Antworten und der Originalkörper der 403-Inventarantwort.

Die erfolgreiche Probe vom 03.10.2026 meldet CC BY-NC-SA 4.0. Der Lizenzbeleg ist eine separate neue Beobachtung. Die alten Seitenantworten enthalten selbst keine Lizenz- oder Autorenfelder. Für den 403-Abruf liegt der Originalkörper vor, aber keine getrennte Headerdatei. Die HTTP-403-Messung ist im Recherchebericht dokumentiert.

## Abdeckung

Ein vollständiges fortgesetztes Seiteninventar liegt bislang nicht vor. Die siteinfo-Statistik nennt 101914 Seiten, 939 Artikel und 97391 Bilder. Diese drei Werte werden nicht zu einem Artikel- oder Namespaceinventar umgedeutet. Daher kein Prozentwert für eine aktuelle Vollabdeckung.

Die zwölf gesicherten Seiten sind der gesamte in den konkret geprüften lokalen Wiki-Rohpfaden nachgewiesene historische Artikelbestand. Zusätzliche reguläre öffentliche Dumps und Operatorquellen werden unabhängig geprüft. Das von der Wiki gepflegte Daten-Git gehört zur Sammlung von B; seine Patchtexte und strukturierten Spieldaten müssen C zusätzlich erhalten. Synthetische S12-Fixtures zählen nicht als Echtdaten.

## Noch ausstehend

Vertragskonformes JSONL wird durch den neuen Rust-Normalisierungspfad erzeugt und unabhängig auf Hash, IDs, Revisionen und Wiederholung geprüft. C importiert danach in die vorhandene Datenbank und belegt den Zugriff aus Brain. Eine neue Datenbank, ein eigener Merge oder ein eigener Deploy von A ist nicht vorgesehen.
