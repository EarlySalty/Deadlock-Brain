# G-S2: kontrollierte Zeitbasis und vollständiger Abschluss

## 1. Ziel und vorhandener Vertrag

Fortsetzung des vorhandenen Auftrags, kein Neubau. Beende beide S2-Rechenfehler einschließlich echter Prüfung beider öffentlichen Simulationsgrenzen. Lies BRIEFING-G-M-S2-FORTSETZUNG-2100.md und G-M-S2-BLOCK-NACHWEISE.md. Dieser Nachtrag erweitert dessen Eigentum genau um den nötigen bestehenden Deadlinevertrag; alle übrigen Grenzen bleiben.

Workflow we899a9xq / wf_a74b258f-f61 wurde durch G regulär gestoppt, bevor weitere identische Scopeblocker wiederholt werden. Runde 1 hinterließ echte Produktfixes und korrigierten Testhelfer als combat.rs-WIP. Stackteilmenge: 2 passed/0 failed/0 ignored/341 filtered. Getrennter expliziter Shredfall: 1 passed/0 failed/0 ignored/342 filtered. Compiler/Clippy/Format dort Exit 0 auf WIP, noch kein committed S2-Beweis. Runde 2 und 3 stoppten an derselben fehlenden kontrollierbaren Uhr, kein Gateurteil. Diese Scopegrenze ist kein inhaltlicher Gate-BLOCK und zählt nicht als neue Gate-Fixrunde.

Tatsächlich geprüfter Deadlinevertrag brain-contracts/src/deadline.rs benutzt std::time::Instant::now() in after und remaining; privater State besitzt keine kontrollierbare Zeitquelle. Eine lokale Uhr nur im combat-Test kontrolliert den tatsächlichen with_deadline-Eingang deshalb nicht. Die Forderung nach kontrollierter Zeitbasis bleibt erhalten. G besitzt diesen zentralen Vertragsbereich und gibt hiermit die nötige eng kompatible Ergänzung ausdrücklich frei.

## 2. Exklusives Eigentum

Erlaubt: rust/crates/brain-contracts/src/deadline.rs samt unmittelbaren vorhandenen Modultests und rust/crates/dbrain-reasoner/src/combat.rs samt unmittelbaren Regressionen. Keine anderen Produktdateien oder Manifeste. Kein anderer G-Writer aktiv. Alle früheren Fixcommits, S3-Deadlinewrapper, data.rs/lib.rs und calculation.rs/calculation_tests.rs erhalten. Keine Änderung an K/I-Files, service.rs, analytics_runtime oder Quellenrechten.

Ergänze den bestehenden RequestDeadline kompatibel um eine injizierbare monotone Zeitquelle, sodass ein Test denselben konkreten öffentlichen Deadlinepfad vollständig kontrollieren kann. Bestehende after/remaining/check/wait/expires_at/cancel und Arc-Klonidentität bewahren. Standard in Produktion bleibt die bisherige monotone Systemzeit. Kein neues Timeout, Modell, Requestbudget oder vom Modell steuerbarer Uhrparameter. Ein fixer Instant-Anker plus explizit kontrollierbare Uhr ist zulässig, keine elapsed-/sleepabhängige Erfolgsmessung. Falls nötig nur den jetzt freigegebenen Deadlinevertrag erweitern, keinen zweiten RequestDeadline oder Erfolgsshim im combat-Code anlegen.

## 3. Zustand und Gitrechte

Eigener Worktree /home/nathanael/.worktrees/brain-g-v2-20261007, Branch feat/brain-v2-g-20261007. Elternsession prüfte nach erfolgreichem TaskStop: HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206, 60 Statuszeilen, leerer Index; alle ursprünglichen WIP-Dateien erhalten. Combat-SHA256 d8a34a344238977c7eab261c14cdc66a880345c41b87a8695ca79c2052652c18; deadline.rs unverändert 1c56ba8005599b15115004348de7ac50ce1eb27b86a00033516583d84db71584. Weitere erhaltene Sourcehashes in Runde-3/quellbeleg.json.

Gezielte kompatible Deadline- und S2-Fixcommits erlaubt, Git-Schritte einzeln mit literalen absoluten Pfaden. Nur eigene Dateien. S3-Holdback unverändert anhand bestehenden S2-HOLDBACK.patch aus dem Index zurückhalten, Arbeitsquellen nicht zurücksetzen. Kein add -A/Reset/Stash/Forcepush. Trailer Co-authored-by: GPT 6.1 Sol <modell@local>. Featurepush erst nach vollständiger untenstehender Prüfung und ALLOW. Elternsession ohne Git-Schritte während deiner Arbeit. Kein Main, Release, Deploy, Runtime, Cleanup oder Settle in diesem Paket.

## 4. Zahlenbeweis und regulärer Gate

Vor Codebestandssuche code-suche/Graphify. Nur cargo-slot, locked/offline/jobs 3, vorhandenes Target /tmp/brain-g-m-0645-target. Keine FD/flock-Schleifen, Umgehungswrapper, direkten Cargoprüfungen oder Hook-/Settingsänderungen. Die drei erhaltenen Fälle prüfen: Stack-Shred auf Default/Fast/Binding, explizites Szenario ohne Doppel-Shred und Duplikatnormalisierung über beide öffentlichen Simulationseingänge bei gleicher Stat-/Shop-/Effektpopulation. Öffentliche Originalfixtures nutzen, synthetische Änderungen getrennt kennzeichnen. Erfolgs- und Abbruchfälle der kontrollierten Uhr sowie Klon-/Abbruchbindung tatsächlich testen.

Primärlauf:
/home/nathanael/.local/bin/cargo-slot test --manifest-path /home/nathanael/.worktrees/brain-g-v2-20261007/rust/Cargo.toml --package dbrain-reasoner --lib --locked --offline --jobs 3 --target-dir /tmp/brain-g-m-0645-target combat::tests -- --include-ignored --test-threads=1 --nocapture

Daneben passende vorhandene brain-contracts-Deadlinetests über denselben Wrapper. Formatcheck, Compiler und striktes Clippy all-targets einschließlich Abhängigkeiten. Committed S2-Prüfstand im eigenen vorhandenen sauberen Checkpointbaum nach dessen Zustandsprüfung belegen. Originalfixtures und bestehende Suites nicht schwächen oder still überspringen. Keine Prüfsperre durch einen anderen Werkzeugweg umgehen.

Einziger Reviewer:
python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-g-v2-20261007 --base bd83d7abdef812a30daa47aa5f6a78de4f42263a --head <tatsächlicher gesamter Fix-SHA>

Unverändert gpt-6.1-sol, volle S2-Gruppe samt nötiger kompatibler Deadlineergänzung. Kein Delta-ALLOW, Modellwürfeln oder Reviewthread. Nach echtem BLOCK nächster frischer nativer Fixer durch steuernden Workflow. Scope-/Abhängigkeitsblocker separat NEEDS_CONTRACT melden, nicht als Gate-BLOCK, damit sie nicht blind fünfmal wiederholt werden. Schutzblocker mit präzisem Deny sofort melden. Rohlogs, Exits, Zahlen, Git-Schrittanzahl, SHA und Quellenbindung nach G/pruefungen/g-m-s2-zeitbasis-2135/runde-N/. Gewöhnlicher Worker delegiert nicht weiter.

## 5. Routing und Übergabe

G ee3de2ba-30ab-4558-a57c-6c1de154891e, Delegator 481426fe-b477-42b3-91c6-901811fcba1d. Keine T3-Threads oder fremde Sessionverwaltung. G bleibt Statusproduzent; keine TODO-/REGISTERänderungen. Native Workflowrückgabe nur bei Task-Ende, wirklichem Schutz-/Vertragsproblem oder maximal fünf echten Gate-Fixrunden. S3/S4 folgt nach deiner abgeschlossenen Rückgabe. Parallelentscheidung hebt starre Mergefolge auf, aber keine Eigentumsgrenze.

Nur Rust, bestehende Provider, keine privaten Originale/Community-Rohdaten an Codiermodelle oder Git. Secrets NEVER ausgeben. Kein Browser nötig; andernfalls ausschließlich Moli nach Guide, Brave MUST NOT benutzt werden. Bericht mit tatsächlichem TESTNACHWEIS[TW-1] und MERGEPROTOKOLL[MS-1]; gebaut/reviewt/gepusht/gemergt/live getrennt.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 30 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
