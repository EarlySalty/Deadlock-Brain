# A: frische unabhängige Gesamt-Datenabnahme nach Fix6

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-a

## 1. Ziel und Vertrag

Du bist ein frischer unabhängiger nativer Blattprüfer. GPT 6.1 Sol aus dieser Session geerbt, höchstens high oder medium. Kein Modelloverride, Fallback, weiterer Agent oder fremde Session. Root verlangt nach dem vollständigen korrigierten Beweis eine unabhängige Eigenabnahme, danach As eigenen lokalen Modulcommit. Du urteilst ausschließlich über As Daten- und Modulübergabe. Kein produktives Importgrün, Gate-ALLOW, Merge, Push oder Deploy.

Nutzerziel: tatsächliche regulär erreichbare Wikiquellen vollständig übernehmen, keine Beispieldaten als Gesamtlösung. Originaltexte, tatsächliche Revisionen, Identitäten, Autoren, Herkunft, Rechte und Historie unverfälscht erhalten. Historische Übernahme nicht als heutige Wiki-Vollabdeckung ausgeben. Bestehenden Rust-Pfad verwenden, keinen zweiten Parser oder Corepfad bauen.

Vertrag zentral: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-wiki-spielwissen/CONTRACT.md`, außerdem `BRIEFING-A.md` und `AN_BEREICHE.md`, insbesondere Punkte 45, 47 und 51. Contract-Version `wiki-spielwissen-v1`.

## 2. Eigentum und Verbote

Ausschließlich lesende Prüfung. Keine Dateiänderungen, neue Werkzeuge oder Parser, Normalisierung, Compiler, Tests, Formatierung, Netz, DB, Secrets, ENV, Git-Schreibschritte oder fremde Sessions. Vorhandene Werkzeuge dürfen vorhandene Ausgaben vollständig lesend validieren. Keine Agenten-JSONL lesen. Kleine abgeleitete Summen statt großer Rohdaten im Kontext. Falls eine Berechtigung verweigert wird, nicht über einen anderen Actor umgehen.

Codefragen zuerst mit code-suche und Graphify prüfen, danach konkrete Dateien lesen. Neue untracked A-Module fehlen eventuell im Graph; keinen Graphneuaufbau starten. Eigene Quellen:

- `rust/crates/dbrain-sources/src/wiki_inventory.rs` und `wiki_inventory/{normalize,storage,tests}.rs`.
- `.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/src/main.rs`, Cargo.toml und Cargo.lock.
- Bereichsberichte unter `.tasks/2026-10-03-wiki-spielwissen/bereiche/a/`: `DATENLAUF-FIX6.md`, `FIX-6.md`, `FEATURE-VERTRAG-FIX6.md`, `REVIEW-LOCAL-7.md`, `DUMPQUELLEN.md`, `AKTUELLE-ABDECKUNG.md`, `UEBERGABE.md` und echte Endbelege.

Keine Berichtdatei durch dich; native Rückgabe an A genügt. Status, REGISTER.md und TODO.md bleiben unberührt.

## 3. Arbeitsstand und sichere Startgrenze

Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`, Branch `feat/brain-wiki-spielwissen-a`, HEAD `2734c2da4e814ff79953e8e825275b0216a6af16`. Vier eigene neue Module uncommittiert, gemeinsame produktive Manifeste und Registrierung ausschließlich C3. A darf nach bestandener unabhängiger Abnahme nur eigene Module lokal committen.

Vollständige Operator-Endübergabe ist tatsächlich abgeschlossen und von A übernommen. Frischer Endoperator `af3b82db0ede1ca1d` beendet, keine eigenen Hintergrundkinder oder Locks; Artefaktende 15:55:40 UTC. A las `DATENLAUF-FIX6.md` und beide neue Endbeleg-JSONs vollständig. Bericht-SHA `8904cc27143036f412c927699e2cc7d261d7d9c07b270a37ff1b1b6952bd3716`, finale Hashliste `endbeleg-fix6-pruefartefakte.sha256` mit 35 strikt von A bestätigten Einträgen, SHA `db176a6fc4425f85639ab2fc062603d0e7e2ee3f4be404085fb8bbc2e6d8fa6d`. Du übernimmst fertige Daten und eingefrorene Quellen, keine Operatorrolle.

Endbeleg `endbeleg-fix6-vertrag-typen.json`: vollständiger Lauf 15:40:31 bis 15:46:08 UTC, Exit 0, alle 19 Fehlerzähler null; 7.082 stabile Dokument-IDs und 38.273 eindeutige Dokumentrevisionen. Tatsächliche Werttypen: 946.167 String, 130.107 Number, 24.127 null, 8.752 Boolean; alle Einheiten null und alle Herkunftszeiger Strings. Globale Dubletten null, gesamte JSON-Pointerabdeckung geprüft. Ein jq-Ausdrucksfehler vor Eingabelesen mit Exit 3 ist offen dokumentiert, danach vollständiger erfolgreicher Folgecheck.

Endbeleg `endbeleg-fix6-bytegroessen.json`: sechs echte erfolgreiche byteweise Prüfungen, 851.127.133 Gesamtbytes, größte Zeile 4.762.643 Bytes ohne LF beziehungsweise 4.762.644 mit LF. Vorherige höhere Rohtextwerte und wc-Displaybreite 4.415.434 sind keine gültigen Bytegrößen. Diagnosen bleiben erhalten. Belastbarer Altbestandvergleich liest direkt JSON, nicht fehlerhaften jq-Rohtextmodus. Diese Methoden und die tatsächlichen kompletten Ergebnisse unabhängig kritisch abnehmen, keine alte falsche Größenangabe übernehmen.

Tatsächlicher Vollharness `bwrrp3l2b`: Wrapperstart 13:04:18 UTC, erfolgreiche Binaryausführung ab 13:32:11 UTC, Ende 13:57:09 UTC. Harness-Exit 0, Wrapper-Exit 0, beide Locks geschlossen. A las `bereiche/a/data-fix6.log` vollständig und prüfte sechs strukturierte Resultate gesondert. Keine unerwarteten Resultate, kein run_error oder normalization_error. Alle Original-, Source-, Binary-, Altbestand- und Target-Erhaltungsprüfungen im Endlog erfolgreich. Ein früherer Vorstartversuch mit fehlendem `/usr/bin/time` und Exit 127 blieb ohne Binaryausführung; nicht als Datenlauf oder grüne Baseline ausgeben.

47 ungefilterte Tests bestanden, 0 failed, 0 ignored, 0 filtered. Clippy aller Targets mit `-D clippy::all`, Debugbau und eigene Formatprüfung erfolgreich. Gesamtformat scheitert allein am unveränderten fremden util.rs:52. Vier unnötig entfernte Originalkommentare exakt restauriert, nur der erlaubte collapsible_if-Logikfix bleibt. Kein fremdes Edit oder Warnungsunterdrückung.

Vor und nach deiner Prüfung den endgültigen Freeze lesend bestätigen: `fix6-handoff-source-freeze.sha256` mit 15 Dateien und `fix6-handoff-binaries.sha256` mit drei Binaries. Endgültiger Harness-SHA `818a9f5ac81cfe2104bf3fd82b1b493ff458d68a8800f9bf17bff3a66857baf5`; Hauptmodul-SHA `915ea72b8c96881fa510195c400685d90ed07de97765c93f99fd7e78f14f8fee`. Frühere Snapshots und Binaries sind erhalten.

## 4. Beweisziel am vollständigen echten Bestand

Fertiger Root `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized-fix6/`. Sechs Spools mit documents.jsonl, inventory.json und run-evidence.json. Ursprünglicher unveränderter Inputbestand: 47 Dateien, davon fünf Archive und zwölf historische API-Captures. Originalhashliste `pruefharness-a/originals-before.sha256`. Alter Root `a/normalized/` bleibt erhalten und wegen 282 Zahlenwertabweichungen gesperrt.

| Spool | Dokumentversionen | Fakten | Von A unabhängig bestätigter JSONL-SHA-256 |
| --- | ---: | ---: | --- |
| deadlock.wiki-20250415-history | 22.742 | 1.109.153 | 32cdbc98fc8692812c879b7682f17c2493fd622e67fe25fe00b853a8956e65e9 |
| deadlocked.wiki-20241107-history | 13.301 | 0 | 56a9ec76e0176ad451ed352ee1e74ae1a623bf3399a4b3cfddb9b5b7ff9e2608 |
| deadlockwiki.org_mw-20260130-history | 2.172 | 0 | fd17663fda7f372f2e80d7045a3c4267bd3d9aec855282ae0bdbecebb52b54a7 |
| deadlock.miraheze.org_w-20240616-history | 45 | 0 | 16dda1666f8d5233e09a3e7dbd860fb3ae34d3bbb6fb2c4b11da1d95dc87ec0e |
| deadlockwiki.miraheze.org_w-20231203-history | 1 | 0 | bc3e5ce7bcec193f6d828db882843f6465769bc5a0f223a9a28d68f0bb21e7f4 |
| legacy-2026-captures | 12 | 0 | 74619a6af63af768cff43d5b1c7d8dbc0153ac13cc729aaa799b5ba6d62fdf10 |

Vollharness meldet 38.273 Dokumentversionen, 1.109.153 originalwertgleiche Fakten, darunter 130.107 JSON-Zahlen. Alle sechs vollständigen Wiederholungen byteidentisch, 38.261 XML-Originalrevisionen und zwölf API-Texte exakt. Diese Meldungen kritisch unabhängig prüfen, nicht allein aus Exit 0 oder Übereinstimmung desselben fehlerhaften Parsers ableiten.

Prüfschwerpunkte:

1. Vollständige Faktenwerttreue gegen tatsächliches Originalcontent-JSON, alle Blätter und Faktenzeiger, Typen erhalten. Nicht nur die zwei bekannten Beispiele. Belegte Altbestanddifferenzen und alle 282 alten numerischen Abweichungen prüfen. Kein Epsilon, Floatvergleich, Runden oder Stichprobenurteil als Vollbeweis. Der vorhandene erhaltende serde_json-Pfad ist weiterverwendet, keine zweite Parserimplementierung.
2. Alle Originaltexte und UTF-8-Hashes, echten Revisionen, Autoren, Quellen-, Dokument- und Revisionsidentitäten, Rechte, Historie und Vertragsfelder erhalten. Keine geratenen Einheiten, abgeleiteten neuen Fakten oder Identitätsdubletten. Fünf Quellenidentitäten; zwei deadlock-wiki-Spools absichtlich dieselbe Quelle mit verschiedenen Revisionen.
3. Ganze zweite Verarbeitung aller sechs Spools belegt, nicht nur erneuter Hash desselben Outputs. Originale und alte Ausgaben unverändert, echter Datenexit und eigene Lock-/Kinderfreigabe dokumentiert. Neue Größen und maximale Zeile anhand endgültiger Artefakte geprüft; nicht alte Größen blind übernehmen.
4. Historische Grenzen ehrlich: alle inventory_complete/content_complete false. Öffentliche Weitergaberechte nicht behaupten. Einseitendump von 2023 ausdrücklich kein belegter Spielwissenskorpus. Zwölf Captures vom 02.05.2026, Revisionen 07.04. bis 01.05.2026. Redirectnamen Level, Item und Ability ohne eigene belegte Revision. Heutige robots-/Mechanics-Proben 403, anschließend gestoppt; weder neue Liveartikel noch globale Sperre aller Einzelseiten daraus behaupten.
5. Root-Punkt 51: A-Harness bleibt serde_json 1.0.151 mit arbitrary_precision eingefroren. C verwendet produktiv bestehenden Pin 1.0.150 mit arbitrary_precision und prüft sämtliche Serialize-/Deserialize-, Postgres- und Leserübergänge getrennt. Kein Produktivgrün aus As Harness, kein Versionsupgrade verlangen. Originallexeme bleiben exakt im content; JSON-Fakten bleiben Zahlen. Bibliotheksseitige Exponenten- und Ganzzahl-minus-null-Kanonisierung sowie darstellungsabhängiges PartialEq offen behandeln.

## 5. Routing und Urteil

Auftraggeber A `f01cce67-209b-468e-8abb-ec2070beeaa2`, Root übergeordnet, Paket a, Versuch 1, Produzent teil-a. A allein veröffentlicht Ereignisse. C3 übernimmt gemeinsame Integration, finalen Gate, Merge, Push, Deploy und Live-Prüfung.

Rückgabe auf natürlichem Deutsch mit echten Umlauten und Leerzeichen, humanizer/no-em-dashes anwenden. Pflichturteil: **fertig J/N für A-Übergabe, Abweichungen, Fix nötig J/N**. Konkrete bestätigte Funde mit Ort, Eingabe und Fehlerfolge nennen. Grenzen getrennt halten. Bei Restdefekt keinen Fix oder neuen Lauf starten, Ursache und begrenzten Vorschlag an A geben. Vorhandene vollständige Belege kurz nachvollziehbar nennen, keine Rohdatenflut oder ungeprüfte Importfreigabe.
