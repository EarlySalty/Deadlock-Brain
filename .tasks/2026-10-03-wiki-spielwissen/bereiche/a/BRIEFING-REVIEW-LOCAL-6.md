# A: Fix5 und vollständige Speicher-/Retry-Familie unabhängig nachprüfen

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-a

## Ziel und Vertrag

Frischer unabhängiger nativer Blattprüfer, ausschließlich lesend. Geerbtes GPT6.1Sol, höchstens high/medium, kein Modellwechsel/Fallback. Keine Compiler/Test/Format/Gate/Netz/Git, keine Dateiänderung und keine weiteren Agenten oder fremden Sessions. Bestehender Datenworker besitzt echte Compiler-/Datenläufe. Statische Nachprüfung ist keine Laufzeitfreigabe.

CONTRACT.md und AN_BEREICHE.md zentral unter /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-wiki-spielwissen lesen. Bereichsberichte unter eigenem Worktree .tasks/2026-10-03-wiki-spielwissen/bereiche/a/: FIX-5.md, REVIEW-LOCAL-5.md und BRIEFING-FIX-5.md. Vorhandenen Eigenanteil erhalten. Keine neue Implementierung/Doppelwriter oder zweiter Corepfad. C2 besitzt existierende get_bounded-Anbindung und finale Integration/Gate/Deploy. Quellentexte, Revisionen, Autoren, Capture- und Rechtebelege originaltreu erhalten; historische Abdeckung getrennt.

Ausdrücklicher Hauptauftrag: gesamte Speicher-/Retry-Familie systematisch prüfen, keine Zweizeilenprüfung. Nach Fix5 bei erneut untragfähigem Urteil muss A vor Fix6 Ursache, Restfunde und konkreten begrenzten Vorschlag an Haupt melden. Keine automatische weitere Fixrunde.

## Umfang und konkrete Fehlerfamilie

Lies alle vier A-Module, tatsächliche öffentliche Aufrufer, Speicherhelfer und alle41 Testdefinitionen. Prüfe systematisch Dokument, Herkunft, Konflikt, Quellenbindung, Checkpoint und abgeleitete JSONL-/Inventarpublikation von Write/Flush/File-sync über Hardlink/Rename und Directory-sync bis Rückgabebestätigung. Was geschieht nach sichtbarer Installation und fehlgeschlagenem letzten Sync? Wo synchronisiert Retry vor Deduplizierung/Erfolgsbestätigung? Originale bleiben unverändert, neue Belege dürfen nicht verloren gehen, Widersprüche erhalten und keine Lizenzaufwertung.

Fix5-Kern1: zusätzlicher Autor/Contributor/Capture/Lizenz eines bereits gespeicherten Konfliktinhalts wurde früher öffentlich verworfen. Jetzt bestehender Herkunftspfad mit eindeutig inhaltsgebundenem Ergänzungsschlüssel. Prüfe Erstinstallation, wiederholten identischen Konflikt, neue Ergänzung, Widerspruch, Quellbindung, Wiederöffnung, öffentliche API/XML-Captures und Berichtszusammenführung. Kein erfolgreicher/„erhalten“-Pfad vor benötigtem Sync, keine Herkunftsvermischung.

Fix5-Kern2: privater Same-Spool-Budgetzähler war nach sichtbarer Installation/letztem Syncfehler veraltet. Jetzt sofort nach Installation fortschreiben, Fehler vor Installation dürfen nicht erhöhen. Prüfe Dokument, Konflikt und Herkunftsersetzung samt Deduplizierungsretry. Öffentliche API/XML-Retries öffnen neu und zählen echte Dateien: klar von privatem Fall trennen, keine unbelegte öffentliche Budgetüberschreitung behaupten.

Vollständige kompakte Pfadmatrix, jeweilige konkrete Anker/Fehlerfolgen. Keine pauschalen fremden Audits/Stilfunde. Tatsächlich unbehebbare beziehungsweise erneut tragfähigkeitsverhindernde Funde samt Ursache und begrenztem Lösungsvorschlag nativ zurückgeben, Rohbericht bleibt bei A. Keinen echten Crash-/Testlauf behaupten, geschriebene Regressionen nicht bestandene nennen.

Bestandssuche zuerst geladener Skill code-suche/Graphify, dann konkrete Stellen lesen. Globaler Graph /home/nathanael/.graphify/global-graph.json; neue untracked A-Module können fehlen, kein Neuaufbau. Secrets/ENV nicht lesen, keine Schutzumgehung.

## Eigentum und bestätigter Freeze

Worktree /home/nathanael/.worktrees/brain-wiki-spielwissen-a, Branch feat/brain-wiki-spielwissen-a, HEAD2734c2da4e814ff79953e8e825275b0216a6af16. Neue eigene Module uncommittiert. Fixer5 abgeschlossen und ausdrücklich eingefroren, Parent hat alle vier Hashs tatsächlich gemessen. Keine weiteren Sourcewriter. Exakt vor/nach nachmessen, Abweichung melden, nicht fremde Basis übernehmen:

- rust/crates/dbrain-sources/src/wiki_inventory.rs: 2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80
- rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs: 27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc
- rust/crates/dbrain-sources/src/wiki_inventory/storage.rs: edd384e621d9a0df11d1a36ad6cac44678285e342f68d4c363db42f1646bd7ea
- rust/crates/dbrain-sources/src/wiki_inventory/tests.rs: e26f97b9f3a9d666889cefff3445d17a8e07511c7fd527591c12e172356e3f18

Kein Berichts-/Register-/Status-/TODO-Schreibrecht. Native Rückmeldung an A, A sichert Bericht.

## Tatsächliche Beweislage und Stopbedingungen

41 Regressionen geschrieben, null ausgeführt. Laut Fixer gezielter rustfmt-check0. Einziger tatsächlicher Cargo-Start06:47:12UTC, test4 Exit101 wegen eigenem Harness-Kindmodulpfad, keine Tests. Harness korrigiert, tatsächlicher Retest noch offen. test5/test6 nie Compiler. Derselbe Datenworker nach Fix5 wieder freigegeben, wartet regulär auf beide Hostlocks, keine neue Implementierung. Stabile Wartetasks nicht timerbedingt stoppen. Alle Archive/Captures/Harness/Core/eigenes Lockfile erhalten, bisher null bestätigte normalisierte Vertragsdokumente/Fakten, kein Datenimport/Commit/Push/Deploy.

Bei Freezeabweichung sofort A melden; ansonsten gesamte Familie bis zur Rückgabe lesen. Tatsächliche Compiler-/Datenresultate nur als separat zitierte genuine Workerbelege, keine Eigenbehauptung.

## Routing und Rückgabe

Auftraggeber A f01cce67-209b-468e-8abb-ec2070beeaa2. Haupt /root. Paket a, Versuch1, alleiniger Statusproduzent teil-a. Unabhängiges statisches Urteil zu beiden Fix5-Kernen und vollständiger Matrix mit konkreten Zeilenankern, Hashmessung und Grenzen. Nur verifizierte Defekte mit Input/Zustand und tatsächlichem Fehlerpfad, insbesondere öffentliche und private Fälle getrennt. Kein allgemeines ALLOW. Keine Fixes starten. Bei Restfund begrenzten Vorschlag liefern, damit A vor Runde6 gemäß Haupt eskaliert.
