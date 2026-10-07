status: administrativer Endbeleg abgeschlossen
Datum: 2026-10-03
Vertrag: wiki-spielwissen-v1
Modell: GPT 6.1 Sol high, geerbt, ohne Override oder Fallback

# A: tatsächlicher Datenlauf und Endbeleg Fix6

Der korrigierte Bestand ist vollständig gegen Vertragsfelder, Typen, Identitäten und Herkunftszeiger geprüft. Der lesende Endcheck endete sicher am 03.10.2026 um 15:50:12 UTC. Keine unabhängige Gesamtfreigabe und kein Importgrün: frische unabhängige Datenabnahme und eigener Modulcommit liegen weiterhin bei A, produktive Verbraucherprüfung, Git-Integration und Deployment bei C3.

Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`, Branch `feat/brain-wiki-spielwissen-a`, erhaltener HEAD `2734c2da4e814ff79953e8e825275b0216a6af16`. Vier eigene Module weiterhin uncommittiert. Kein Git-Schreibschritt durch diesen Operator.

## Tatsächliche Zeiten und Exits

| Vorgang | UTC am 03.10.2026 | Ergebnis |
| --- | --- | --- |
| Früherer Vorstart, Task brbdh9mfm | Ende 13:01:30 | Fehlendes `/usr/bin/time`, Startaufruf 127, Wrapper 1; Harness nicht gestartet, Locks geschlossen |
| Vollharness, Task bwrrp3l2b, Wrapper 1297541 | Binarystart 13:32:11, Ende 13:57:09 | Harness 0, Wrapper 0, Locks geschlossen |
| Abgeschlossener Altbestandvergleich, Task b4ngjbh5s, Admin 1662920 | 14:11:30 bis 14:12:20 | Alle sechs Einzelvergleiche 0, ADMIN_EXIT 0 |
| Dieser administrative Endbeleg | Beginn 15:34:38 | 15 Sourcefreeze- und drei Binaryfreeze-Einträge vorab unverändert, jeweils 0 |
| Vollständiger Vertrags-/Typencheck mit vorhandenem jq | 15:40:31 bis 15:46:08 | Exit 0, tatsächliche Mengen und 19 Fehlerzähler dokumentiert |
| Sechs Bytezeilenchecks und erhaltene Ausgabemanifeste | 15:47:14 bis 15:47:18 | Sechs Größenexits 0, Hashcheck sämtlicher 38 Ausgabedateien 0, beide Freezes 0 |
| Sichere Endprobe | 15:50:12 | Keine eigenen Shellkinder, FD 8/9 nicht vorhanden; beide Freezes erneut 0 |

Der erste administrative jq-Ausdruck um 15:39:20 endete vor dem Lesen der Eingaben mit Exit 3, weil `allkeys` vor seiner Definition referenziert wurde. Mit dem vorhandenen `keys` korrigiert, danach der oben belegte vollständige Lauf. Kein Rust-Compiler oder Quellfix. Frühere gescheiterte Operatoren wurden nicht reaktiviert.

## Vollständiger korrigierter Bestand

38.273 Dokumentversionen, 1.109.153 Fakten und 851.127.133 JSONL-Bytes. Fünf Quellenidentitäten, sechs Spools. 7.082 stabile Dokument-IDs und jeweils 38.273 unterschiedliche Dokument-/Revisionspaare sowie Quellenrevisionen. Keine globale Dokument-/Revisionsdublette, kein Inhaltskonflikt und keine doppelte Fakt-ID innerhalb einer Dokumentrevision. Mehrere historische Revisionen derselben Dokument-ID bleiben erhalten.

Der abgeschlossene Rust-Vollharness belegt alle 1.109.153 Fakten originalwertgleich, darunter 130.107 JSON-Zahlen; alle 38.261 XML-Originalrevisionen und zwölf API-Originaltexte exakt. Alle sechs vollständigen Wiederholungen byteidentisch. Diese bereits abgeschlossenen Belege wurden übernommen, nicht durch einen neuen Lauf ersetzt.

| Fertiger absoluter Datenpfad | Dokumente | SHA-256 |
| --- | ---: | --- |
| `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized-fix6/deadlock.wiki-20250415-history/documents.jsonl` | 22.742 | `32cdbc98fc8692812c879b7682f17c2493fd622e67fe25fe00b853a8956e65e9` |
| `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized-fix6/deadlocked.wiki-20241107-history/documents.jsonl` | 13.301 | `56a9ec76e0176ad451ed352ee1e74ae1a623bf3399a4b3cfddb9b5b7ff9e2608` |
| `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized-fix6/deadlockwiki.org_mw-20260130-history/documents.jsonl` | 2.172 | `fd17663fda7f372f2e80d7045a3c4267bd3d9aec855282ae0bdbecebb52b54a7` |
| `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized-fix6/deadlock.miraheze.org_w-20240616-history/documents.jsonl` | 45 | `16dda1666f8d5233e09a3e7dbd860fb3ae34d3bbb6fb2c4b11da1d95dc87ec0e` |
| `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized-fix6/deadlockwiki.miraheze.org_w-20231203-history/documents.jsonl` | 1 | `bc3e5ce7bcec193f6d828db882843f6465769bc5a0f223a9a28d68f0bb21e7f4` |
| `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized-fix6/legacy-2026-captures/documents.jsonl` | 12 | `74619a6af63af768cff43d5b1c7d8dbc0153ac13cc729aaa799b5ba6d62fdf10` |

Alle strukturierten Fakten liegen im ersten Spool. Die übrigen Dokumente bleiben als ursprüngliche Wissensdokumente erhalten.

## Vertrags- und Typennachweis

Vorhandenes jq-1.7 las alle sechs fertigen JSONL-Dateien direkt im JSON-Eingabemodus. Keine Stichprobe oder Rohtext-JSON-Neukodierung als Vollbeweis. Der neue Beleg `endbeleg-fix6-vertrag-typen.json` nennt sämtliche tatsächlichen Zähler, Feldprüfungen, Quellenverteilung und Grenzen.

Alle 15 äußeren Pflichtfelder und ihre Typen, Vertragsversion, Wiki-Quellenart, stabile Seiten-ID, tatsächliche Revision, UTC-Beobachtungszeit, Belegstatus, Inhalt als String, kleine 64-stellige Hashform, Lizenzobjekt und Metadaten geprüft. Quellenkennungen an ihre fünf öffentlichen Origins gebunden; `curid`, `oldid` und Attribution an tatsächliche Seiten-/Revisionskennungen gebunden. Exakte UTF-8-Inhaltshashbindung aus dem abgeschlossenen Vollharness übernommen.

Alle acht Faktpflichtfelder, Fakt-ID-Form, Dokumentbindung, Typen, Belegstatus, Einheit/null, Qualifier und Revisionszeiger vollständig geprüft. Für jede fakttragende Revision stimmt die gesamte Liste skalarer Original-JSON-Pfade einschließlich null mit den Herkunftspointern überein. RFC-6901-Escaping bleibt erhalten. Kein fehlender oder zusätzlicher Pointer, keine unbelegte Einheitenumrechnung. Zahlenwerte wurden in diesem Endcheck ausschließlich nach JSON-Typ gezählt, nicht über Floatkonvertierung verglichen.

| Faktwerttyp | Tatsächliche Menge |
| --- | ---: |
| String | 946.167 |
| Number | 130.107 |
| null | 24.127 |
| Boolean | 8.752 |

Alle 1.109.153 Einheiten sind null, alle Herkunftszeiger Strings. Faktbelegstatus `extracted_value`, `unit_inferred=false`, Spielpatchbeleg false; historische Kennzeichnung entspricht dem jeweiligen Dokument. Alle 19 Vertragsfehlerzähler null, Exit 0.

## Genau 282 gezielte Korrekturen

Die sechs bereits vorhandenen direkten `data-fix6-comparison-*.json` vollständig aggregiert, Exit 0. Kein erneuter Altbestandvergleich.

38.273 Dokumente und 1.109.153 Fakten verglichen, genau 282 numerische Korrekturen. Keine sonstigen Dokument-, Inhalts-, Herkunfts-, Autoren-, Rechte-, Faktidentitäts- oder nichtnumerischen Wertabweichungen; keine bloßen Repräsentationsänderungen. Nur 38.273 Beobachtungszeiten und 38.261 heutige Lizenzbeobachtungen geändert, keine ungültige Beobachtungsänderung. Zwölf API-Lizenzbeobachtungen erhalten. Der alte Faktenbestand bleibt mit seinen belegten Zahlenfehlern gesperrt.

## Erhaltung und Größenkorrektur

`data-fix6.log` bestätigt nach dem einzigen Vollharness alle 47 Originalinputs, Source-/Binaryfreezes, alte Normalisierung und beide Targets einschließlich Erhaltungsmetadaten unverändert. Alte Freezes, Snapshots und Diagnosen erhalten. Dieser Operator bestätigte vor und nach seinen lesenden Prüfungen alle 15 endgültigen Sourcefreeze- sowie drei Binaryfreeze-Einträge. Zusätzlich alle 38 korrigierten Ausgabedateien gegen die vorhandene Hashliste geprüft, Exit 0.

Größte echte UTF-8-JSONL-Zeile: **4.762.643 Bytes ohne LF, 4.762.644 mit LF**. Der neue `endbeleg-fix6-bytegroessen.json` zählt mit vorhandenem Perl die tatsächlich gelesenen Bytes aller sechs Dateien, ohne Unicode-Decodierung oder JSON-Interpretation. Die Summe 851.127.133 Bytes enthält LF.

Der erhaltene `data-fix6-wc-displaywidth-proof.jsonl` nennt für den ersten Spool 4.415.434. Das ist kein gültiges Bytemaximum; selbst `LC_ALL=C wc -L` liefert hier diesen anderen Längenbefund. Nicht als Bytes übernehmen. Frühere höhere Größenwerte ebenfalls nicht übernehmen. Der byteweise Endcheck bestätigt den korrigierten Übergabewert.

`data-fix6-admin-tool-diagnosis.log`, der Rohtextwrapper und sechs `data-fix6-rawmode-comparison-*.json` bleiben erhalten. Ihr jq-Rohtext-/UTF-8-Fehler erzeugte 337 scheinbare Inhaltsabweichungen und 2.724 scheinbare nichtnumerische Abweichungen. Diese Diagnosen sind keine Datenfehlerbelege. Der erfolgreiche vollständige direkte JSON-Vergleich ist der maßgebliche bestehende Beleg.

## Bereits abgeschlossene Compilerprüfungen

Vom erhaltenen endgültigen Freeze übernommen: 47 Tests bestanden, kein ignorierter Test; Clippy aller Targets mit `-D clippy::all`, Debugbau und eigener Formatcheck erfolgreich. Gesamtformatprüfung scheitert allein am unveränderten fremden `util.rs:52`. Vier vorhandene Kommentare exakt restauriert. Keine neue Compiler-, Test- oder Formatprüfung durch diesen Operator, keine eigenen Prüfergebnisse für C3 behauptet.

A-Harness bleibt `serde_json 1.0.151` mit `arbitrary_precision`. C3 behält den produktiven Pin `1.0.150` mit `arbitrary_precision` und belegt Verbraucherwirkung einschließlich Postgres und bestehendem Reader getrennt. Kein Versionsangleichen, keine `as_f64`-/Floatkonvertierung und kein zweiter Parser.

## Abdeckung und Rechte bleiben begrenzt

Alle sechs `inventory_complete`- und `content_complete`-Flags false. Tatsächliche historische Abdeckung und Namespacezähler bleiben in `data-fix6-coverage-rights-proof.jsonl` erhalten; kein heutiger Gesamtnenner belegt. Zwölf Captures vom 02.05.2026 mit Revisionen vom 07.04. bis 01.05.2026, keine heutigen Liveabrufe. Der Einseitendump von 2023 ist ausdrücklich kein belegter Spielwissenskorpus. Drei unbelegte Redirect-Ursprünge Level, Item und Ability bleiben dokumentierte Lücken.

Bei allen 38.273 Dokumenten `redistribution_allowed=false` und `license_revision_verified=false`. Lizenznamen, Attribution, URLs und Provenienz erhalten; keine öffentliche Weitergabefreigabe. C3 prüft den beauftragten internen Import und Leserzugriff im eigenen produktiven Vertrag.

## Neue Endbelege und sichere Übergabe

Alle folgenden Dateien liegen ausschließlich unter `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/bereiche/a/`:

- `endbeleg-fix6-vertrag-typen.json`: tatsächlicher vollständiger Vertrags-/Typencheck, Mengen, Fehlerzähler, Methode und Grenzen.
- `endbeleg-fix6-bytegroessen.json`: sechs byteweise Zeilengrößen und korrigierte Gesamtsumme.
- `endbeleg-fix6-freeze-ende.log`: tatsächliche Starts, Exits, vor-/nachgelagerte Freezes und sichere Endprobe.
- `endbeleg-fix6-pruefartefakte.sha256`: finale Hashliste der vorhandenen Lauf-, direkten Vergleichs-, Diagnose-, Prüf- und Erhaltungsbelege sowie der neuen Endbelege und dieses Berichts. Die Hashliste enthält sich nicht selbst.
- `DATENLAUF-FIX6.md`: dieser tatsächliche Abschlussbericht.

Alle eigenen Werkzeugaufrufe synchron beendet; keine Hintergrundtasks, Kinderagenten oder Wachtasks gestartet. Endprobe Shell 2377717 ohne Kinder und ohne FD 8/9; frühere eigene PIDs 1199101, 1297541 und 1662920 nicht vorhanden. Keine eigene Host- oder Spoolsperre erworben, somit keine gehaltene eigene Sperre. Keine globale Sperrfreiheit behauptet, keine fremden Prozesse verändert.

Nur eigene neue Endbelege geschrieben. Kein Sourcefix, Compiler, Datenlauf, erneuter Altbestandvergleich, Netzwerk-, DB- oder Git-Schreibzugriff. Quellen, Harness, Manifeste, Lockfile, Binaries, Targets, Originale, sämtliche normalisierten Daten und vorhandene Belege erhalten. Frische unabhängige Abnahme startet erst nach dieser sicheren Übergabe durch A.
