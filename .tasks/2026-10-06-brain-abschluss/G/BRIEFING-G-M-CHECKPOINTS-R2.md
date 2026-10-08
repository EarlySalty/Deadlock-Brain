# G-M: verbleibende Rechenkerncheckpoints S2/S3/S4

status: historischer S2/S3/S4-Startauftrag ab bd83d7ab; S2-Weiterbau nun ausschließlich nach BRIEFING-G-M-S2-CARGO-SLOT.md, 07.10.2026

Die frühere Prüfsperre ist nach verbindlicher Weiterbauentscheidung behoben. FD/flock-Befehle unten sind historische Anweisungen und werden nicht erneut benutzt. Alle neuen Cargo-Prüfungen ausschließlich über cargo-slot. S3/S4 startet erst nach tatsächlich konsumiertem vollständigem S2-ALLOW samt Nachweisen, mit einem daran gebundenen neuen Briefing und tatsächlichem S2-SHA. Der gestoppte frühere Haupt-Orchestrator wird nicht reaktiviert; Delegator 481426fe-b477-42b3-91c6-901811fcba1d bleibt zuständig.

## 1. Ziel und Vertrag

Fahre die erhaltene Checkpointarbeit fort, baue sie nicht neu. F1 bd1284ac und F2 2b67796f sind isoliert compiler-/clippygeprüft, regulär ALLOW und auf origin gesichert. Finales S1 einschließlich Bool- und endlichem Waffenratenfix ist als bd83d7abdef812a30daa47aa5f6a78de4f42263a gemeinsam gegen F2 ALLOW, committed Compiler/Clippy Exit 0 und auf origin bestätigt. Bereichsführung las die Rohbelege, den Fixdiff und bestätigte das aktuelle Manifest 12/12. R2-Tests wurden nach einer einmaligen Slotablehnung nicht ausgeführt. Nicht als grüne Tests melden. Startmanifest: G/pruefungen/g-m-s1-r2/bereichs-quellen-fix.sha256.

Lies G/CHECKPOINTS.md, G/RECHENKERN-VERTRAG.md, G/G-M-0645-NACHWEISE.md und die tatsächliche Fixrückgabe. F1/F2/S1 nicht erneut committen. Liefere S2 Simulation/Testfixture, S3 öffentlichen gemeinsamen Rechenkern und S4 vorhandene Rechenfälle. I/K benötigen einen tatsächlich geprüften gesicherten S3-Featurevertrag unabhängig von Main; kein fremder WIP oder zweiter Rechner.

## 2. Eigentum

Verbleibende gelieferte eigene Änderungen ausschließlich in dbrain-reasoner/src/combat.rs, planner.rs, calculation.rs, calculation_tests.rs, lib.rs und den beiden zurückgehaltenen S3-Helfern in data.rs. Frischen booleschen und numerischen Konverterfix samt Tests erhalten. Typen/Mechanik/Progression und drei Originalfixtures sind bereits committed. Im normalen Paketlauf keine neuen Produktänderungen, Kommentare, Formatrefactorings oder Testabschwächungen. Ausnahme ist die ausdrücklich beauftragte enge Korrektur bestätigter BLOCK-Funde durch jeweils frischen Fixer; dabei keinen I-/K-Eigentumsbereich betreten und keinen Ersatzbau anlegen.

Staging exakt nach CHECKPOINTS.md. S2-HOLDBACK.patch hält Combat-Deadlinewrapper zurück; S3-HOLDBACK.patch hält die cfg(test)-Modulerklärung für S4 zurück. Holdbacks nur im Index, keine Arbeitsquellen zurücksetzen. Tatsächlich committed Quellgrenzen vor jedem Commit prüfen. Eigene Belege unter G/pruefungen/g-m-checkpoints-r2/. Keine Änderungen an REGISTER, PLAN, REVIEW, AN_HAUPT oder TODO, keine fremden Bereiche.

## 3. Arbeitsstand und Erlaubnis

Primärworktree /home/nathanael/.worktrees/brain-g-v2-20261007, Featurebranch feat/brain-v2-g-20261007. Tatsächlicher Start-HEAD bd83d7abdef812a30daa47aa5f6a78de4f42263a, Startmanifest G/pruefungen/g-m-s1-r2/bereichs-quellen-fix.sha256. Vorheriger S1-Fixer abgeschlossen, Wache gelöscht. Exklusives Git-Schreibrecht für deine Gruppen; Elternsession schreibt währenddessen keine Git-Schritte. Gezielt eigene Dateien stagen, kein add -A. Git-Schritte einzeln, literale absolute Pfade. Trailer nur Co-authored-by: GPT 6.1 Sol <modell@local>, keine weitere Attribution.

Vorhandener eigener sauberer Prüfbaum /home/nathanael/.worktrees/brain-g-checkpoints-20261007 wird ausschließlich auf konkrete eigene committed Zwischenstände gesetzt. Kein neuer Ersatzbaum und keine fremden Arbeitsbäume. KEINE Tests dort, KEINE Umgehung abgewiesener Aktionen über andere Werkzeuge/Worker. Kein Main, Release, Deploy, Neustart, Cleanup oder Settle.

## 4. Beweisziel

S2, dann S3, dann S4. Für jede committed Gruppe isolierter Compiler und striktes Clippy all-targets einschließlich Abhängigkeiten im vorhandenen sauberen Prüfbaum. Bekannte zulässige Form: flock /tmp/deadlock-cargo-release.lock, vorhandenes cargo, absoluter manifest-path des Prüfbaums, package dbrain-reasoner, locked/offline, jobs 3, target-dir /tmp/brain-g-m-0645-target; Clippy -D warnings. Tatsächlichen Prüfbaum gegen Gruppen-SHA bestätigen. Gesamt-WIP ist kein Zwischenbeweis.

Je Gruppe regulärer `gate_hook.py --review` auf konkretem vorigen HEAD bis Gruppen-SHA, bekannte Datei /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py, unveränderte Urteilskette gpt-6.1-sol zuerst. Einziger Reviewer ist dieser Gate, keine Review-Threads. Bei BLOCK fährst du die Fixschleife autonom: pro Runde einen frischen nativen Fixer mit exklusivem Besitz der konkret betroffenen eigenen Dateien starten, niemals mit deinem Kontext fortsetzen. Fixer prüft seine Änderung und dieselbe vollständige Gruppe mit demselben Urteilmodell wie Runde 1. Nach tatsächlicher Rückgabe Quellen/Compiler/Clippy/Gate konsumieren und bis ALLOW fortsetzen. Keine Zwischenmeldung je Runde; Rückgabe erst bei Paketabschluss oder wirklichem Blocker nach fünf erfolglosen Runden. Abgewiesene Aktionen weder selbst umgehen noch an Fixer übertragen. Geprüfte Gruppe darf sofort nach origin/feat/brain-v2-g-20261007 gesichert werden; kein ungeprüfter Folgestand. Mehrere Commits ersetzen keinen kumulierten Maingate.

Nach tatsächlichem S3-ALLOW und Push einen eigenen S3-Übergabebeleg im Prüfbereich schreiben: vollständiger SHA, exakte Eltern-SHAs der abhängigen Gruppen, Gatewortlaut/Exit, committed Compiler-/Clippybeleg und ls-remote-Beweis. Dieser Prüfbeleg macht den tatsächlichen Quellenvertrag für I/K nachvollziehbar; keine erfundenen ModelSource-Rechte oder Receipts. Danach S4 fortsetzen.

Neue Tests sind keine Zusatzpflicht. Vorhandene primäre Rechenfälle und Reasonersuite dürfen nach finalem S4 im primären Worktree mit `--include-ignored --test-threads=1` und ursprünglichen drei ausdrücklich ausgeschlossenen Produktionsfällen laufen, über den bestehenden isolierten PostgreSQL-Verwaltungsweg aus G-M-0645. Der R2-Testaufruf mit vorangestelltem flock wurde einmal durch die Befehlsprüfung abgewiesen. Diese abgewiesene Form nicht wiederholen und nicht per direktem Cargo, anderem Werkzeug oder Fixer umgehen. Ein Testlauf setzt einen tatsächlich belegten zulässigen bestehenden Slotweg voraus; sonst ausdrücklich nicht ausgeführt melden. Keine Produktions-DB, kein Hilfsbaum-Test. Frühere Messung 34 Rechenfälle und 321 passed/4 DB-Fixturefehler bleibt historisch; neue Konvertertests können Fallzahlen ändern. Tatsächliche Zahlen, Exits, Filter und Fehlervergleich präzise berichten. Neue eigene Regressionen nicht zu Altfehlern erklären.

Fixmanifest vor Start prüfen und nachher erneut binden. Alle gelieferten Arbeitsquellen/Originalfixtures bleiben bytegleich mit diesem neuen Startmanifest. Rohlogs auf Geheimnisse prüfen, nicht pauschal stagen. Rückgabe mit Gruppen-SHAs, wirklichen Gates, Quellenbindung, origin-Beweisen und offenen Grenzen; kein Gesamt-G-/F-/K-/Live-ALLOW.

## 5. Routing

Auftraggeber G; Delegator 481426fe-b477-42b3-91c6-901811fcba1d, Haupt-Orchestrator d3a1741e-82bc-4a48-865b-2845c663dca7; native Elternsession 030a7b6f-d25c-482d-b66c-68185cd05dbb. Statusproduzent G. Native frische Fixer ausschließlich für die beauftragte autonome BLOCK-Schleife ausdrücklich erlaubt. Keine zusätzlichen T3-Threads, eigene Reviewer oder Sessionkoordination; kein ListAgents oder SendMessage. Vor Bestandssuche code-suche und Graphify; bei fehlendem Worktreegraph bekannte globale/Repoalternative, keine Neuextraktion. Wache nach 25 Minuten, spätestens 30. Rückfragen an G, nicht den Nutzer. Browser nur Moli; vor Browserarbeit agent-browser.md lesen, Brave MUST NOT benutzen. Für dieses reine Paket ist keine Browserarbeit nötig.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
