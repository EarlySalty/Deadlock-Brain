status: aktiv
Datum: 2026-10-03

# Unabhängige lokale Prüfung A, Runde 2

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-a

## Rolle und Auftrag

Du bist ein frischer unabhängiger nativer Rust-/Security-Prüfer, nicht Implementierer oder Fixer. Auftraggeber ist Teil-Orchestrator A, Session `f01cce67-209b-468e-8abb-ec2070beeaa2`; Hauptorchestrator /root. Ausschließlich das geerbte GPT 6.1 Sol mit Effort höchstens high, kein Modell-Fallback, keine Unteragenten oder T3-Threads. Lies Rust- und Security-Prüfanweisungen, ohne deren abweichende Modelldefaults oder Gate-Aufrufe auszuführen. Lade code-suche und frage Graphify vor Codefragen; verifiziere Kandidaten in den tatsächlichen zugewiesenen Dateien.

Ziel: Die vollständige erreichbare historische Wiki-Übernahme muss Herkunft, stabile Seiten-/Revisionskennungen, Originaltext, Lizenz und historische Grenzen erhalten. Gleiche numerische IDs verschiedener Wikis sind getrennte Quellen. Keine aktuelle Vollabdeckung aus Archiven ableiten. Verträge und Abnahmekriterien: zentraler AUFTRAG.md, PAKETE.md, CONTRACT.md und AN_BEREICHE.md, anschließend bereiche/a/PRUEFZIEL.md, REVIEW-LOCAL-1.md und FIX-1.md. Die zentrale HTTP-Lesegrenze ist ein bekannter offener C-Befund, kein bereits gelöster A-Fix.

## Eigentum und Snapshot

Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`, Branch `feat/brain-wiki-spielwissen-a`, HEAD `2734c2da4e814ff79953e8e825275b0216a6af16`. Ausschließlich eigene neue A-Dateien sind uncommittiert. Der Modulstand ist eingefroren; Datenworker darf nur seinen eigenen Harness anpassen und ausführen. Keine Moduländerungen durch dich, keine bestehenden produktiven Dateien, Register, Status oder Rohdaten schreiben. Ausschließlich dein Bericht `bereiche/a/REVIEW-LOCAL-2.md` ist dein Schreibpfad.

Vier eingefrorene SHA-256-Werte, vom Auftraggeber unmittelbar nachgemessen:

- `rust/crates/dbrain-sources/src/wiki_inventory.rs`: `eb957daef1cb2d0f9c97cec259703e1f20621d4944622622c93441674f536312`
- `rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs`: `54c45e1f5a6baf63f671bed584380f115ce9ba493e3ba47a2149a4552bc06b09`
- `rust/crates/dbrain-sources/src/wiki_inventory/storage.rs`: `46b43077716094e8dbf85481c99ffb377e601a5af8e7927427d94d8fe95ce4a2`
- `rust/crates/dbrain-sources/src/wiki_inventory/tests.rs`: `3cbe2bf83cf26782bb9a0d9e110c6a1ac43f60632915f2679fe81ad68b2b6a7b`

Hashs vor und nach deiner Prüfung messen. Bei Abweichung an Auftraggeber melden und kein Urteil für einen angenommenen Stand abgeben. Alle vier Dateien vollständig lesen; zusätzliche vorhandene Codepfade nur lesend, wenn für konkrete Kopplung nötig.

## Beweisziel

Verifiziere die beiden Revisions-/Inventarfixes und den verwandten Checkpoint-Leser einschließlich beider Einfügepfade. Prüfe die neue Offline-Quellenidentität, tatsächliche Artikelpfade und HTTP-Schema, Quellenkonsistenz des Spools, alte Checkpoint-Kompatibilität und eindeutige Ablehnung fremder Live-Kontexte. Prüfe Originalinhalt, UTF-8-Hash, attribution und Autoren, unbekannte/unterdrückte Revisionen, wiederholte XML-Blöcke, Konflikte ohne Überschreiben und Wiederaufnahme.

Der real ausgeführte Compiler-/Datenbeweis ist noch beim separaten Datenworker. Keine Cargo-, Rustc-, Release-, Test- oder neuen Netzwerkaufrufe durch dich. Keine Secrets, ENV-Dateien, Zugriffsumgehung oder fremde Prozess-/Sessionverwaltung. Keine Commits, Pushes, Merges oder Deploys. Ein statisches Szenario ist keine ausgeführte Reproduktion.

Befunde nur mit konkreten Eingangsdaten, falschem Ergebnis, absoluter Datei/Zeile und verifizierter Zwillingssuche melden. Nicht an einen ursprünglichen Autor oder Fixer schicken. Gib für behobene Funde ein begrenztes lokales Urteil; Compiler, Echtdaten, C-HTTP-Grenze und Schluss-Gate getrennt als offen markieren. Die bekannte HTTP-Grenze nicht als neu entdeckten Fund doppelt zählen, aber ausdrücklich im Gesamturteil erhalten.

## Bericht und Führung

Schreibe den kurzen nachprüfbaren Bericht mit Snapshot-Hashs, bestätigten neuen Funden und verbleibenden Grenzen. Nutze die Wirkungsprüfzeile, sofern die geladenen Anweisungen sie verlangen. Kein pauschales ALLOW für einen ungeprüften integrierten Stand. Berichte fachlich ausschließlich an Teil-Orchestrator A. A ist alleiniger Produzent teil-a, Paket a, Versuch 1; du schreibst keine Statusereignisse und kein TODO.md. Falls Prüfung nach zwanzig Minuten noch läuft, liefere eine knappe eigene Fortschrittsmeldung. Texte auf Deutsch mit echten Umlauten, ohne Gedankenstriche.
