# G-M-S1-R2: endliche Waffenrate am gemeinsamen Parser

status: vorbereitet nach tatsächlichem S1-ALLOW mit bestätigtem numerischem Randfall, 07.10.2026

## 1. Ziel und Vertrag

S1 samt Boolfix 4df1eb5aeeeb36b8fa4f68a320167be11bce68e6 ist committed und isoliert Compiler/Clippy Exit 0. Gemeinsamer regulärer Gate gegen F2 tatsächlich ALLOW, Task bd0t5yhab, Log /tmp/brain-g-s1-fix-gesamt-gate-20261007.log. NIT: data.rs:859 im committed Stand teilt bullets_per_second durch ungeprüfte Pelletzahl. Bei bullets 0, vorhandener Projektilrate und fehlender Schussrate/Zykluszeit entsteht Infinity.

Bereichsführung befragte Graphify und bestätigte die Arbeitsquelle :904. calculation_models_from_payloads legt den so erzeugten WeaponProfile direkt im öffentlichen SourcedWeaponModel ab, kein nachgelagerter endlicher Guard an dieser Grenze. Diesen numerischen Kern eng korrigieren, ohne Datenlücken durch erfundene 1 oder einen behaupteten bekannten Wert 0 zu ersetzen. Fehlende/ungültige Rate muss im bestehenden öffentlichen Known/Unknown-Vertrag ehrlich bleiben; Originalrohwerte erhalten. Vorhandene gültige direkte Schussrate, Zyklus-/Burstrate und Projektilrate mit positiver gültiger Pelletzahl dürfen sich nicht ändern. Zwillinge der abgeleiteten Quotienten auf Endlichkeit prüfen, keine allgemeinen Waffenrefactorings.

## 2. Eigentum

Nur reiner Waffenparser und unmittelbar zugehörige Datentests in dbrain-reasoner/src/data.rs. Falls der bestehende öffentliche Unknown-Anschluss zwingend einen unmittelbar zugehörigen Guard in mechanics.rs benötigt, zuerst den konkreten vorhandenen Pfad belegen und auf diesen Guard beschränken; keine neue Formel/Typen oder Scopeerweiterung. Kein Loader, SQL, Planer, Build, Quelle, Konfiguration, Kommentare oder neue Abhängigkeiten. R1-Boolfix und drei Tests unverändert. S3-Helfer bleiben im Arbeitsbaum unstaged; Originalfixtures unverändert. Keine neuen Referenz-Itemnamen oder Heldengewichte in Produktcode.

Eigene Belege unter G/pruefungen/g-m-s1-r2/. Zentrale Akten und TODO nicht bearbeiten. Vor Bestandssuche code-suche/Graphify; vorhandenen Parser und seine Unknown-Ansicht benutzen, keinen zweiten Parser.

## 3. Arbeitsstand und Befugnisse

Primärworktree /home/nathanael/.worktrees/brain-g-v2-20261007, Branch feat/brain-v2-g-20261007, tatsächlicher Start-HEAD 4df1eb5aeeeb36b8fa4f68a320167be11bce68e6. Eigener sauberer Prüfbaum brain-g-checkpoints-20261007 auf demselben Stand. Origin zuletzt F2 2b67796f; 4df1eb5a noch ungepusht. Vorheriger Fixer abgeschlossen. Bereichsmanifest g-m-s1-r1/bereichs-quellen-fix.sha256 vor Start 12/12 bestätigt.

Direkter Nutzerauftrag verlangt geprüfte gesicherte G-Featurecheckpoints und autonomen Gitabschluss. Eigener enger Fixcommit und nach gemeinsamem S1-ALLOW Featurepush sind daher erlaubt; kein Main. Git-Schritte einzeln, literale absolute Pfade, ausschließlich eigene Dateien, kein add -A/Forcepush. Exklusives Git-Schreibrecht. Trailer nur Co-authored-by: GPT 6.1 Sol <modell@local>, keine weitere Attribution. Wenn du eine wirkliche Berechtigungsablehnung erhältst, Befehlsform gemäß Regel korrigieren, nicht umgehen und nicht wiederholt identisch retryen. Kein Release/Runtime/Cleanup/Settle.

## 4. Beweisziel

Keine Infinity/NaN in öffentlichen konvertierten Waffenwerten; ungültige beziehungsweise nicht ableitbare Rate bleibt unbekannt statt bekannter Null. Reale vorhandene Payloads zum Ausgang nehmen; gezielte Null-/Randwertmutationen sind Parserfälle, kein Produktionsnachweis. Bestehende gültige Werte erhalten und ursprüngliche Sourcebytes außerhalb des engen Deltas binden.

Compiler und striktes Clippy auf dem konkreten committed Fixstand im bestehenden eigenen Prüfbaum all-targets einschließlich Abhängigkeiten über flock /tmp/deadlock-cargo-release.lock, cargo mit locked/offline/jobs 3 und target-dir /tmp/brain-g-m-0645-target. KEINE Tests im Hilfsbaum. Tests im Primärbaum nur über tatsächlich zugelassenen bestehenden Cargo-Slotweg; blockiert dieser, keine identischen Wiederholungen und nicht durch direkten Cargoaufruf ohne Slot umgehen. Compiler ist der Mindestbeweis, neue Tests keine allgemeine Zusatzpflicht. Bei tatsächlich gelaufenem Test Zahlen/Exits/Filter nennen, kein fingierter Lauf.

S3-Holdbacks im Index erhalten, nur eigenes Fixdelta committen. Danach gesamtes S1 gegen F2 2b67796fb80ae3440a0c9e76671dfc8169032844 regulär mit `gate_hook.py --review` prüfen, nicht bloß neues Fixdelta. Bestehendes Urteilmodell gpt-6.1-sol unverändert. Bei BLOCK tatsächlichen neuen Fund zurückgeben, frischer Folgefixer durch G. S2/S3/S4 hier nicht beginnen. Erst ALLOW plus committed Prüfungen gestatten Featurepush und origin-Beweis.

## 5. Routing

Auftraggeber G, Delegator 481426fe-b477-42b3-91c6-901811fcba1d, Haupt-Orchestrator d3a1741e-82bc-4a48-865b-2845c663dca7, Elternsession 030a7b6f-d25c-482d-b66c-68185cd05dbb. Du bist neuer nativer Fixer, keine weitere Delegation/T3-Threads oder Sessionkoordination. Fachrückgabe mit Delta, Unknown-Semantik, konkreten SHAs, Staginggrenze, Compiler-/Clippybeweisen, tatsächlichen Tests, gemeinsamem Gate, Quellbindung und origin-Stand. Statusproduzent G. Wache 25 Minuten, spätestens 30. Kein Gesamt-G-/Live-ALLOW.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
