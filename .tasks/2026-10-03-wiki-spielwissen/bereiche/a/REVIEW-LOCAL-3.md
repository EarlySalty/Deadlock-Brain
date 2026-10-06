status: aktiv
Datum: 2026-10-03

# Lokale unabhängige Prüfung A, Runde 3

Tatsächliche native Rückmeldung des frischen Prüfers abe0eb93f8bf0b595, vom Teil-Orchestrator gesichert. Nur-Lese-Prüfung abgeschlossen: zwei bestätigte Funde, Fix erforderlich, kein Gesamt-ALLOW. Keine Compiler-, Test-, Netzwerk-, Git- oder Dateiänderungen durch den Prüfer.

WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft

## Snapshot

Alle vier Dateien vollständig gelesen. Eigene Eingangs- und Abschlusshashs des Prüfers stimmen mit dem Freeze aus FIX-2.md überein:

| Datei | SHA-256 |
| --- | --- |
| wiki_inventory.rs | 2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80 |
| wiki_inventory/normalize.rs | 27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc |
| wiki_inventory/storage.rs | 88ff521c77032215ba2aef8b4dc4abee20fd43bedbcf5ab218620acbb43e7498 |
| wiki_inventory/tests.rs | 82356a793f8981f021ef6ab9fcf420bfc529fa445243dcff04df2ed2749e7d91 |

## 1. P2: Konfliktdateien umgehen Gesamtgrenze und können Wiederaufnahme verhindern

Fundstellen: `rust/crates/dbrain-sources/src/wiki_inventory/storage.rs:47-77,186-194,290-305`.

Statisches Szenario: Seite 1, Revision 101 und kleiner Originaltext sind gespeichert. Weitere zulässige API-Aufzeichnungen enthalten dieselbe ID/Revision mit jeweils anderem Text. Jeder Versuch schreibt vor der Fehlermeldung eine neue Konfliktdatei. Größenprüfung und Fortschreibung von stored_bytes fehlen; beim Wiederöffnen werden Konflikte ebenfalls nicht gezählt. Der Konfliktbestand kann dadurch die Gesamtgrenze beliebig überschreiten.

Eine Eingabe unterhalb der Antwortgrenze kann nach Anreicherung größer als max_total_bytes sein. Der Konfliktpfad schreibt sie trotzdem. Beim nächsten Aufruf verweigert die neue Quellenprüfung das Lesen dieser Konfliktdatei wegen ihrer Größe. Dann ist auch die Originalquelle mit unverändertem Originaltext unter denselben Grenzen nicht wiederaufnehmbar.

Zwillingssuche: Neue Originaldokumente und Herkunftsergänzungen haben geprüfte Größenberechnungen; der gemeinsame Konfliktpfad nicht. API, XML und Live verwenden diesen Pfad. Die bereits vorhandene Schreiblücke wird durch die verpflichtende Konfliktprüfung beim Binden besonders relevant.

## 2. P2: Erste Herkunftsergänzung vor Checkpoint nicht vollständig crash-dauerhaft

Fundstellen: `rust/crates/dbrain-sources/src/wiki_inventory/storage.rs:404-406,680-693,729-732`.

Statisches Stromausfall-/Kernel-Crash-Szenario: Das Original ist dauerhaft vorhanden; provenance/ fehlt. Die erste Autorenherkunft erzeugt dieses Verzeichnis, synchronisiert Datei und provenance/, aber vor erfolgreicher Rückkehr nicht dessen neu angelegten Eintrag im Spoolwurzelverzeichnis. Ein Ausfall vor dem anschließenden Checkpoint kann die als dauerhaft behandelte Ergänzung verlieren.

Zwillingssuche: write_atomic und write_immutable synchronisieren nur den unmittelbaren Dateielternpfad. Auch erstmals angelegtes conflicts/ hat die Lücke. Ein später erfolgreicher Checkpoint synchronisiert die Spoolwurzel, schützt aber nicht das beanspruchte Unterbrechungsfenster davor. Der Ergänzungstest simuliert normales Wiederöffnen, keinen Systemausfall.

## Begrenzte Bestätigung

Bei gewöhnlichen zulässigen Größen prüfen Quellenmarker, tatsächlicher Checkpoint, Dokumente und Konflikte Herkunft vor neuen Dokument-, Marker-, Checkpoint- oder Transporteffekten. Fehlender Checkpoint ist kein Herkunftsbeleg mehr. Zusätzliche Autorenherkunft ist dauerhaft gespeichert, dedupliziert und veröffentlicht; Original, Hash, Revision, Erstbeobachtung und Rechte bleiben erhalten, Herkunftswidersprüche sichtbar. Neue Marker-/Herkunftsleser sind tatsächlich begrenzt. Dokumente und Ergänzungen werden zusammen gezählt. Vorherige Revisions-, Inventar-, Checkpoint-, Domain- und Artikelpfadfixes bleiben statisch erhalten. 29 Testmarkierungen vorhanden, nicht ausgeführt.

Alle Szenarien sind statische Ableitungen, keine ausgeführten Reproduktionen. Compiler-, Echtdaten-, Crash- und integrierte Gate-Nachweise fehlen. C2 prüft separat die tatsächliche Anbindung und Größenprobe des bereits vorhandenen get_bounded; daraus wird kein fehlender neuer Corepfad abgeleitet. Neue Funde ausschließlich an frischen Fixer, nicht an vorige Autoren.
