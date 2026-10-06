# A: gesamte Speicher- und Retry-Familie unabhängig lesend prüfen

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-a

## Ziel und ausdrücklicher Auftrag

Du bist frischer unabhängiger nativer Prüfer, ausschließlich lesend. GPT6.1Sol, geerbter Effort höchstens high, kein Wechsel/Fallback. Keine Implementierung, keine Compiler/Test/Format/Gate/Netz/Git, keine Dateiänderungen, keine weiteren Agenten oder fremden Sessions. Der vorhandene Datenworker besitzt alle echten Rust-/Datenläufe. Statische Nachprüfung ist kein Compiler- oder endgültiger SHA-/Gatebeweis.

Direkte Nutzeranweisung des Hauptorchestrators: Nach sicherem Fix4-Freeze die GANZE Speicher- und Retry-Strecke systematisch lesen, nicht nur zwei Zeilen. Dokument, Herkunft, Konflikt, Quellenbindung und Checkpoint jeweils vom Schreiben/Rename bis erfolgreicher Bestätigung. Fehlgeschlagener Sync darf bei Wiederholung keine ungeprüfte Erfolgsbestätigung erhalten. Bestehenden engen Fixer4 und Datenauftrag erhalten, keine neue Implementierung oder Doppelwriter. Kurze Bereichsübergabe mit Pfadabdeckung und echten Laufbeweisen; Rohbericht nur an A.

Lies zentrale CONTRACT.md und AN_BEREICHE.md sowie eigene FIX-4.md, REVIEW-LOCAL-4.md und erforderliche konkrete Produktionsaufrufer. Wiki-Datenziel: Originaltexte/Revisionen/Autoren/Rechte dauerhaft und originaltreu erhalten, Quelle und historische Abdeckung korrekt trennen. Kein zweiter Corepfad: get_bounded auf C2-Basis vorhanden, dortige Anbindung separat.

## Systematische Prüfmethode und Umfang

Erstelle eine überprüfbare Pfadmatrix für Dokument, Herkunft, Konflikt, Quellenbindung, Checkpoint und abgeleitete Veröffentlichung (documents.jsonl/inventory.json). Folge dem gemeinsamen tatsächlichen Pfad von öffentlichen API-/XML-Capture- und Berichtsfunktionen durch Spoolöffnung, bind_source, Datei-/Temporäranlage, Write/Flush/File-sync, Rename/Hardlink, Directory-sync, Checkpoint und Rückgabebestätigung. Prüfe zugehörige frühen Deduplizierungs-/Existenz-/Readpfade und wiederholten Aufruf derselben sowie wiedereröffneten Spool.

Für jeden Pfad: Was passiert, wenn der letzte benötigte Sync fehlschlägt, nachdem Ziel bereits sichtbar ist? Welche Wiederholungs-/Wiederöffnungsstelle holt den Sync tatsächlich nach, bevor Erfolg oder „erhalten“ bestätigt wird? Werden Syncfehler propagiert? Sind Original, zusätzliche Herkunft und Budget nach Fehler/Wiederaufnahme konsistent? Prüfe genau diese Fehlersystematik vollständig, ohne neue Features oder pauschale fremde Repo-Audits. Vergleiche die37 geschriebenen Regressionen mit der echten Produktionsstrecke; Fake für den ganzen Spool wäre kein Beweis. Kein echter Crash/Testlauf durch dich.

Code-Suche zuerst Skill code-suche/Graphify, danach gefundene Stellen lesen. Globaler vorhandener Graph /home/nathanael/.graphify/global-graph.json, kein Neuaufbau. Secrets/ENV nicht lesen, keine Schutzänderung oder umgeleitete fremde Arbeit.

## Eigentum und Freeze

Worktree /home/nathanael/.worktrees/brain-wiki-spielwissen-a, Branch feat/brain-wiki-spielwissen-a, HEAD2734c2da4e814ff79953e8e825275b0216a6af16. Eigene neue Module uncommittiert. Fixer4 abgeschlossen und ausdrücklich eingefroren; kein Writer mehr. Der Datenworker wieder freigegeben und hält Module unverändert. Miss exakt diese vier Hashs vor/nach; Abweichung melden statt andere Basis übernehmen:

- rust/crates/dbrain-sources/src/wiki_inventory.rs: 2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80
- rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs: 27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc
- rust/crates/dbrain-sources/src/wiki_inventory/storage.rs: f48f4819534870b1aa8b2c1f0042f82d1d1ca1b1056242e55dc6632271d4daea
- rust/crates/dbrain-sources/src/wiki_inventory/tests.rs: a0d7cb81ce90c7a729c4ac6e92fbe02d3b670ff5e473a307935589adaf26c6d1

Keine Berichtsdatei schreiben. Native Rückmeldung, A sichert sie. Kein Register/Status/TODO-Schreibrecht.

## Tatsächliche Beweislage

A und Fixer4 bestätigen Endhashs, Datenworker ebenfalls07:25:37UTC.37 Regressionen geschrieben, Formatcheck0; kein erfolgreicher echter Testlauf. Erster tatsächlicher Cargo-Start06:47:12UTC endete Exit101 wegen drei eigener Harness-E0583, keine Tests. Harness inzwischen ausschließlich im eigenen Eigentum korrigiert. Neuer stabiler test-6-Wrapper wartet auf beide Hostlocks; kein neuer Compilerstart bestätigt. Stabile Wrapper nicht wegen Wachtimer abbrechen. Fünf Archive/zwölf API-Artikel gesichert, null bestätigte normalisierte Vertragsdokumente/Fakten. Nicht als Laufbeweise ausgeben.

## Rückgabe und Routing

Direkter Auftraggeber Teil-Orchestrator A f01cce67-209b-468e-8abb-ec2070beeaa2; Haupt /root, Paket a/Versuch1, alleiniger Produzent teil-a. C2 alleinige finale Integration/Gate/Deploy.

Melde die kompakte vollständige Pfadmatrix mit konkreten Datei-/Zeilenankern, Hashvergleich, tatsächlichen statischen Ergebnissen/Beweisgrenzen. Nur verifizierte Defekte: konkrete Fehlerfolge und falsche Bestätigung, keine spekulativen Stilpunkte. Falls kritische Pfadlücke zuerst auffällt, melde sie präzise, arbeite die restliche beauftragte Matrix dennoch lesend zu Ende, sofern der Freeze stabil ist. Kein allgemeines ALLOW, kein behaupteter Compiler-/Test-/Crash-/Datenlauf.
