# A: unabhängige statische Nachprüfung 4

Datum: 03.10.2026
Produzent des Urteils: frischer nativer lesender Prüfer `a54f6e5cadb34657e`, GPT 6.1 Sol, höchstens high geerbt.
Dieser Bericht sichert die tatsächliche native Rückmeldung. Kein zusätzlicher Prüflauf durch das Schreiben dieses Berichts. Erster spezialisierter Rollenversuch `a71f5cc8ce0b66c3c` endete ohne Hash-/Codeurteil wegen einer unvereinbaren verpflichtenden Compilerrolle; daraus wird kein Nachweis abgeleitet.

Urteil: begrenzte statische Prüfung abgeschlossen, ein Restdefekt, Fix erforderlich. Keine allgemeine Freigabe.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

## Freeze

Der unabhängige Prüfer meldet alle vier Hashs vor und nach der Prüfung identisch und briefingkonform:

| Datei im A-Worktree | SHA-256 |
| --- | --- |
| rust/crates/dbrain-sources/src/wiki_inventory.rs | 2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80 |
| rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs | 27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc |
| rust/crates/dbrain-sources/src/wiki_inventory/storage.rs | a16a36d1880a5eb3c07856c39e4c7f6b18b9a168aa4cac0de93961ae34f6827b |
| rust/crates/dbrain-sources/src/wiki_inventory/tests.rs | 5e71ade9d106a9adffc5935a9d141741b58805a3d4cf15eb01d324901eb9bef4 |

## Die beiden ursprünglichen Funde

1. Konfliktbudget statisch behoben: Öffnen zählt Dokumente, Herkunft und Konflikte gemeinsam. Neue Konflikte vor Verzeichnisanlage/Schreiben begrenzt, identische Konflikte nicht doppelt gezählt. Größenablehnung lässt Original wiederaufnehmbar.
2. Ursprünglich fehlende Elternsynchronisierung statisch behoben: gemeinsamer Helfer synchronisiert Verzeichnis und sämtliche absoluten Vorfahren, auch vorhandene Verzeichnisse; Fehler weitergegeben. Wiederholung nach fehlgeschlagenem abschließendem Synchronisieren bleibt jedoch unvollständig.

## P2: Retry bestätigt Herkunft ohne nachgeholte Dateielternsynchronisierung

Pfad: `rust/crates/dbrain-sources/src/wiki_inventory/storage.rs`, Zeilen 387 bis 388, 425 und 712 bis 713. Zwilling: bestehender Konflikt, Zeilen 303 bis 325.

1. Original vorhanden, identische Revision ergänzt erstmals Autorenherkunft.
2. `write_atomic` synchronisiert die Datei und benennt erfolgreich nach `provenance/<key>.json` um. Danach schlägt `sync_parent` fehl; erster Aufruf meldet Fehler.
3. Öffentlicher API-/XML-Schreibaufruf wird wiederholt. Öffnen synchronisiert Spoolwurzel und `documents/`, nicht Einträge innerhalb `provenance/`.
4. `persist_provenance` findet die bereits sichtbare identische Aussage und kehrt erfolgreich zurück. Checkpoint/Veröffentlichung können erfolgreich sein, ohne die fehlgeschlagene Synchronisierung der Herkunftsdatei nachzuholen.

Falsches Ergebnis: erfolgreiche Wiederaufnahme ohne nachgeholte Dauerhaftigkeitsbestätigung. `conflict.exists()` überspringt dieselbe Synchronisierung und meldet „Konflikt erhalten“. Wiederaufnahme muss vor der jeweiligen Bestätigung nachholen.

A hat nach dem Bericht Graphify abgefragt und die konkreten gemeinsamen Speicherpfade selbst gelesen. Dieser Kontrollfluss ist bestätigt. Kein Crash oder Datenverlust wurde reproduziert. Der Datenworker wurde ausschließlich wegen dieses konkreten notwendigen Fixes um eine geordnete eigene Prüfpause gebeten; Schreibfreigabe erst nach belegtem sicheren Punkt, keine Timerpause und keine fremden Prozesse.

## Vier statisch geprüfte Regressionen

In `wiki_inventory/tests.rs`:

- Zeilen 97 bis 159: Gesamtbudget/Deduplizierung/exaktes Wiederöffnungsbudget und unverändertes Original.
- Zeilen 162 bis 190: übergroßer angereicherter Konflikt über öffentlichen Pfad, Ablehnung ohne Konfliktdatei und Originalwiederaufnahme.
- Zeilen 193 bis 227: tatsächlicher Verzeichnishelfer, Fehlerweitergabe/Wiederholung; Injektion vor Dateischreiben erfasst den Restdefekt nicht.
- Zeilen 230 bis 282: gemeinsamer Pfad ohne Checkpoint, normales Wiederöffnen und Original-/Erstbeobachtungs-/Autoren-/Konflikterhaltung; kein fehlgeschlagenes abschließendes Synchronisieren.

Quellenbindung und unveränderliche Originalspeicherung bleiben in den geprüften Pfaden erhalten.

## Beweisgrenzen

Prüfer ausschließlich lesend: keine Dateien, Compiler, Tests, Formatierung, Gate, Netzwerk, Git, Crashs oder weiteren Agenten. Statisches Urteil ist kein Compiler-/Echtdaten-/endgültiger Gate- oder SHA-Beweis. Datenworker und C2 bleiben zuständig. Neue Änderungen entwerten diesen Snapshotnachweis.
