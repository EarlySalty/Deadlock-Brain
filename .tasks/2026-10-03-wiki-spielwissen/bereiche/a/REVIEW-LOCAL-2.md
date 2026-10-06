status: aktiv
Datum: 2026-10-03

# Lokale unabhängige Prüfung A, Runde 2

Vom Teil-Orchestrator nach der abgeschlossenen Rückmeldung des frischen nativen Prüfers `a07963d84aefcea4d` gesichert. Der Prüfer schrieb wegen seiner verbindlichen Nur-Lese-Regel keine Berichtdatei. Dieses Dokument hält sein tatsächliches Urteil fest; es ist kein weiterer Prüflauf.

Urteil fertig: N. Fix nötig: J. Lokale statische Prüfung abgeschlossen: J. Compiler-, Echtdaten- und Gesamtfreigabe: N.

WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft

## Geprüfter Snapshot

Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`, Branch `feat/brain-wiki-spielwissen-a`, HEAD `2734c2da4e814ff79953e8e825275b0216a6af16`. Alle vier Dateien vollständig gelesen; Eingangs- und Abschlussmessung stimmen überein:

| Datei | SHA-256 |
| --- | --- |
| wiki_inventory.rs | eb957daef1cb2d0f9c97cec259703e1f20621d4944622622c93441674f536312 |
| wiki_inventory/normalize.rs | 54c45e1f5a6baf63f671bed584380f115ce9ba493e3ba47a2149a4552bc06b09 |
| wiki_inventory/storage.rs | 46b43077716094e8dbf85481c99ffb377e601a5af8e7927427d94d8fe95ce4a2 |
| wiki_inventory/tests.rs | 3cbe2bf83cf26782bb9a0d9e110c6a1ac43f60632915f2679fe81ad68b2b6a7b |

## 1. P2: Quellenbindung beim ersten fehlgeschlagenen Offline-Lauf zu spät

Fundstellen: `rust/crates/dbrain-sources/src/wiki_inventory.rs:454-476`, `wiki_inventory/storage.rs:101-118,225-228`.

Statisches Szenario: Ein leerer Spool erhält einen Export von deadlocked.wiki mit zwei Seitenblöcken, jeweils Seite 1/Revision 101, aber Text A beziehungsweise B. Das erste Dokument wird dauerhaft gesichert, das zweite als Inhaltskonflikt erfasst. Der Aufruf endet vor dem ersten Checkpoint-Schreiben.

Ein anschließender Export von deadlock.wiki mit Seite 1/Revision 101/Text C passiert die Quellenprüfung, weil der fehlende Checkpoint als leerer offizieller Zustand gelesen wird. Das offizielle Dokument und der offizielle Checkpoint werden geschrieben. Erst publish entdeckt die fremden Dokumente und scheitert. Anschließend verweigert die Checkpoint-Prüfung auch die Wiederaufnahme der ursprünglichen Quelle.

Falsches Ergebnis: Ein bereits belegter Spool wird teilweise umgewidmet und danach für seine ursprüngliche Quelle gesperrt. Die verspätete Prüfung verhindert eine erfolgreiche gemischte Veröffentlichung, aber nicht die falsche Zustandsänderung.

Zwillingssuche: Beide Offline-Eingänge verwenden persist_offline. Der Live-Eingang prüft ebenfalls nur den Checkpoint; verwaiste fremde Dokumente werden vor dem ersten Abruf nicht erkannt. persist_document prüft die Dokumentquelle intern, nicht ihre Zugehörigkeit zum Spool. Quellenbindung muss vor dem ersten Dokumentschreiben dauerhaft feststehen beziehungsweise beim Öffnen anhand vorhandener Dokumente geprüft werden.

## 2. P2: Idempotente Wiederholung verliert zusätzliche Autorenherkunft

Fundstelle: `rust/crates/dbrain-sources/src/wiki_inventory/storage.rs:143-169`.

Statisches Szenario: Zunächst API-Capture von Seite 1/Revision 101/Text Original ohne user speichern. Danach XML-Export derselben Quelle, Seite und Revision mit identischem Text und Contributor Actual author, ID 9, übernehmen. Die XML-Normalisierung erzeugt Autor, Contributor-ID und ergänzte Attribution korrekt. persist_document vergleicht jedoch nur Inhalt und Inhaltshash und kehrt erfolgreich zurück.

Falsches Ergebnis: Im veröffentlichten Dokument bleiben revision_author und revision_contributor leer; die neu belegte Attribution wird verworfen. Die Wiederholung meldet Erfolg, verliert aber belegte Herkunft.

Zwillingssuche: API und XML verwenden denselben Persistenzpfad; publish liest ausschließlich das zuerst gespeicherte Dokument. Die vorhandenen Tests prüfen Erstbeobachtung und eigenständige XML-Autorenübernahme, nicht deren Zusammenführung. Zusätzliche Herkunft muss erhalten bleiben, ohne Erstbeobachtung oder Originalinhalt zu überschreiben.

## Bestätigte vorherige Korrekturen

Bekannte Revision wird vor Hidden-/Missing-Abbruch erhalten. Höchste bekannte und lesbare Revision sind getrennt. Netz-, Offline- und wiederholte XML-Blöcke werden revisionsbewusst zusammengeführt. Der Checkpoint-Leser prüft die Dateigröße vorab und begrenzt tatsächliches Lesen auf Grenze plus ein Prüfbyte. Fünf Domains, ursprüngliche Artikelpfade und HTTP-Schema bleiben getrennt; vollständig gespeicherte fremde Kontexte werden vor Live-Abrufen abgewiesen. Originaltext und UTF-8-Hash bleiben erhalten; Inhaltskonflikte überschreiben keine Revision.

## Offene Grenzen

Die bekannte zentrale HTTP-Bodybegrenzung bei C bleibt offen und wurde nicht als neuer Fund gezählt. Echte Kompilierung, ausgeführte Regressionen, vollständige Echtdaten, Crash-Wiederaufnahme, C-Integration und Schluss-Gate fehlen weiterhin. Alle Szenarien sind statische Ableitungen, keine ausgeführten Reproduktionen. Der Prüfer führte keine Compiler-, Test-, Netzwerk-, Git- oder Deploy-Aufrufe aus und änderte keine Dateien. Funde gehen ausschließlich an einen neuen Fixer, nicht an vorherige Autoren.
