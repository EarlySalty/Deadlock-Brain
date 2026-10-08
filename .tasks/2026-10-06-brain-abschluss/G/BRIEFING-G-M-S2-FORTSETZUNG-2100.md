# G-S2: frische Fortsetzung im korrekt gebundenen Worktree

## 1. Ziel und Vertrag

Korrigiere als frischer nativer Fixer die beiden bestätigten S2-Restkerne. Stackbonus muss bullet_shred im Default-, Fast- und Bindingpfad erhalten; im expliziten Szenario wird Shred genau einmal angewandt. Doppelte Item-IDs dürfen Stat-, Shop- und Effektpopulation in beiden öffentlichen simulate_calculation-Eingängen nicht vervielfachen. Verwende die bereits vorhandene Normalisierung aus evaluate_core_with_deadline und die gemeinsame Schadensstufe. Alle fünf früheren Produktfixes erhalten. Kein zweiter Rechner oder Inventarpfad.

Lies CHECKPOINTS.md, G-M-S2-BLOCK-NACHWEISE.md und S2-HOLDBACK.patch im bestehenden G-Aktenbereich. Neueste Entscheidung ENTSCHEIDUNG-PARALLEL-FERTIGSTELLEN.md ersetzt alte Merge-Wartepflichten. S3/S4 bleibt erhalten und wird nach deiner Rückgabe fortgesetzt.

Der Elterncheck hat den alten Log jetzt tatsächlich gelesen: test-vor-fix.log, Exit 101, error[E0609] in combat.rs:3478. DamageModifiers besitzt resist und independent_resists, aber kein resistances. Kein Testfall wurde gestartet. Die drei erhaltenen Regressionen und zwei Helfer sind Test-WIP, noch kein Produktfix. Korrigiere den Helfer anhand des echten Vertrags. Prüfe insbesondere RequestDeadline::after(60s) an einer kontrollierten Zeitbasis; keine echte Wall-Clock in Tests, keine passend gemachten Messwerte. Vor Codebestandssuche code-suche laden und Graphify benutzen. Im Worktree fehlt graphify-out; globaler Graph /home/nathanael/.graphify/global-graph.json liefert combat.rs. Fehlender Symboltreffer ist kein Beweis fehlenden Codes.

## 2. Eigentum

Exklusiv rust/crates/dbrain-reasoner/src/combat.rs samt unmittelbaren numerischen Fällen. Erhaltene S3-Deadlinewrapper nicht entfernen und nicht als S2 stagen. Keine anderen Produktdateien, Manifeständerungen, Loader, Planer, I-/K-Schreibbereiche oder analytics_runtime. Keine globale mutierende Formatierung. Du schreibst deine Belege ausschließlich nach G/pruefungen/g-m-s2-fortsetzung-2100/runde-N/. Keine zentralen Akten, TODO, REGISTER oder fremden Worktrees bearbeiten.

## 3. Arbeitsstand und Git

Worktree /home/nathanael/.worktrees/brain-g-v2-20261007, Branch feat/brain-v2-g-20261007, Start-HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206. Tatsächlich durch Elternsession bestätigt: Index leer, vorhandener WIP in combat.rs/data.rs/lib.rs, untracked calculation.rs/calculation_tests.rs und sämtliche Akten erhalten. Manifest 11/12 gleich; combat.rs wegen 180 neuer Testzeilen SHA256 4c8b1376f3b7587e2eb006403f9b8b6e05d96a973894e1741aa5de419f157f28. Kein anderer eigener Writer aktiv.

Enger Fixcommit erlaubt, vollständiges S2 gegen S1 bd83d7abdef812a30daa47aa5f6a78de4f42263a prüfen. Nur eigenes combat.rs stagen, S3-Holdback über bestehenden selektiven Indexweg erhalten. Keine Änderungen im Arbeitsbaum zurücksetzen, kein reset/stash/add -A/Forcepush. Git-Schritte einzeln mit literalen absoluten Pfaden. Trailer: Co-authored-by: GPT 6.1 Sol <modell@local>. Featurepush ausschließlich nach passendem Zahlenbeweis, committed Prüfungen und vollem S2-ALLOW. Kein Main, Release, Deploy, Cleanup oder Settle in diesem Paket. Elternsession macht während deiner Git-Arbeit keine eigenen Git-Schritte.

## 4. Beweisziel und Gate

Cargo ausschließlich über /home/nathanael/.local/bin/cargo-slot, Argumente direkt. Keine FD/flock-Schleifen, Umgehungswrapper, Hook-, Settings- oder Rechteänderungen. Neue echte Schutzablehnung präzise melden und stoppen; nicht durch anderen Werkzeugweg umgehen. Kontextroot ist jetzt tatsächlich /home/nathanael/.worktrees/brain-g-v2-20261007; eigener ctx_execute_file-Logzugriff wurde durch Elternsession erfolgreich ausgeführt.

Primärer numerischer Lauf:
/home/nathanael/.local/bin/cargo-slot test --manifest-path /home/nathanael/.worktrees/brain-g-v2-20261007/rust/Cargo.toml --package dbrain-reasoner --lib --locked --offline --jobs 3 --target-dir /tmp/brain-g-m-0645-target combat::tests -- --include-ignored --test-threads=1 --nocapture

Tatsächlich prüfen: stack_bonus_shred_matches_default_fast_and_binding_paths, explicit_stack_shred_is_applied_once_with_target_resistance, duplicate_items_have_one_stat_shop_and_effect_population_in_public_simulations. Öffentliche Originalfixtures verwenden, soweit vorhanden; synthetische Mutationen ausdrücklich kennzeichnen. Alle ursprünglichen numerischen Fälle erhalten. Befehl, Exit, passed/failed/ignored/filtered, Werte und Quellbindung speichern. Format, Compiler und striktes Clippy all-targets samt Abhängigkeiten über cargo-slot mit locked/offline/jobs 3. Erhaltenen eigenen sauberen Prüfbaum brain-g-checkpoints-20261007 nur nach Zustandsprüfung auf eigenen committed Stand setzen; keine ungeprüften Tests daraus als Gesamtbeweis.

Einziger Reviewer ist der reguläre Gate:
python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-g-v2-20261007 --base bd83d7abdef812a30daa47aa5f6a78de4f42263a --head <tatsächlicher Fix-SHA>

Gleiches Urteilmodell gpt-6.1-sol wie Runde 1. Kein Modellwürfeln, kein Delta-ALLOW statt vollständiger S2-Gruppe. Speichere Rohgate und Exit. Bei BLOCK liefere strukturierte Funde und Zustand an den steuernden nativen Workflow; der Workflow startet einen frischen Fixer für die nächste Runde, niemals denselben Kontext. Gewöhnlicher Worker delegiert nicht weiter. Maximal fünf frische Fixrunden, dann qualifizierter Blocker.

## 5. Routing und Grenzen

Teil-Orchestrator G ee3de2ba-30ab-4558-a57c-6c1de154891e, Delegator 481426fe-b477-42b3-91c6-901811fcba1d. Keine neuen T3-Threads, keine fremden Sessions, kein ListAgents/SendMessage. Berichte am Paketende an den Workflow, Statusproduzent der G-Akte bleibt Elternsession. Keine Runde-für-Runde-Meldung an den Delegator.

Nur Rust und bestehende zentrale Provider. Secrets NEVER ausgeben. Private Originale und Community-Rohdaten MUST NOT an Codiermodelle oder Git gehen. Keine Browserarbeit nötig; falls doch vorab agent-browser.md lesen, ausschließlich Moli, Brave MUST NOT gestartet oder übernommen werden. analytics_runtime bleibt bis ausdrücklicher I-Übergabe geschützt. Rückgabe trennt gebaut, reviewt, gepusht, gemergt und live. Pflichtzeilen TESTNACHWEIS[TW-1] und MERGEPROTOKOLL[MS-1] mit echten Zahlen.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 30 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
