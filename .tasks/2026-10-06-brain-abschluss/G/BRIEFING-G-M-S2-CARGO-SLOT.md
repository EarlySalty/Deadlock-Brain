# G-S2-Restkern: Weiterbau mit cargo-slot

status: frischer Fixer wunjz2v6f abgeschlossen; cargo-slot-Aufruf Exit 101, Logauswertung durch context-mode blockiert; Fixabschluss offen, 07.10.2026

## 1. Ziel und Vorrang

Verbindlich sind ENTSCHEIDUNG-WEITERBAU-2015.md und der Vorrangabschnitt in PAKETE.md unter /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/. G hat beide tatsächlich gelesen. Derselbe offene Auftrag; der gestoppte frühere Haupt-Orchestrator wird nicht reaktiviert. Delegator 481426fe-b477-42b3-91c6-901811fcba1d bleibt zuständig. Alle bisherigen FD/flock-Anweisungen sind für neue Cargoaufrufe überholt: ausschließlich cargo-slot.

Beide erhaltenen combat.rs-Restkerne numerisch prüfen und eng korrigieren: Stackbonus durch dieselbe vorhandene Schadensstufe einschließlich Shred führen, Default-/Fast-/Bindingpfade gemeinsam; explizites Szenario ohne doppelten Shred. Doppelte Item-IDs vor Effektzuständen mit vorhandener gemeinsamer Normalisierung behandeln, Stat-/Shop-/Effektpopulation identisch. Vor Umsetzung code-suche/Graphify und Bestand prüfen, keinen zweiten Inventar-, Effekt- oder Rechenpfad bauen. Fachfund und letzte Gruppenprüfung: G-M-S2-BLOCK-NACHWEISE.md und pruefungen/g-m-checkpoints-r2/s2-fix-r6/gate.log.

## 2. Eigentum

Produktdatei ausschließlich rust/crates/dbrain-reasoner/src/combat.rs samt unmittelbaren vorhandenen numerischen Fällen. Fünf bisherige Fixcommits und alle S3/S4-Quellen erhalten. S3-Deadlinewrapper bleibt unstaged. Elf andere Quell-/Fixturedateien bytegleich; keine Kommentare, neuen Modelle/Formeln, Konfiguration, Abhängigkeiten, Referenz-Itemnamen im Produktcode, Loader-, Planer- oder I-/K-Dateien. Bestehende Suites nicht abschwächen oder ignorieren. Eigene Belege ausschließlich G/pruefungen/g-m-s2-cargo-slot/. Zentrale G-Statusdateien/TODO nicht bearbeiten, Statusproduzent bleibt G.

## 3. Arbeitsstand und Rechte

Tatsächliches CWD und zugewiesener Worktree /home/nathanael/.worktrees/brain-g-v2-20261007, Branch feat/brain-v2-g-20261007. Start-HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206 jetzt durch G geprüft, Produktstatus erhalten, Startmanifest pruefungen/g-m-checkpoints-r2/bereichs-quellen-ende.sha256 erneut 12/12 bestätigt. Kein anderer eigener Produktwriter. Letzter geprüfter origin-Stand S1 bd83d7abdef812a30daa47aa5f6a78de4f42263a.

Frischer nativer Fixerkontext mit den neu freigegebenen Arbeitsroots ~/.worktrees, ~/repos und /tmp. Kein bisher gestoppter Kontext und kein Ersatz-T3-Thread. Exklusives Produkt-/Git-Schreibrecht für deinen engen Fix; Elternsession führt währenddessen keine Git-Schritte aus. Fixcommit erlaubt, Featurepush erst nach echtem vollen S2-ALLOW und passenden Prüfbelegen. Git-Schritte einzeln, literale absolute Pfade, nur eigene Dateien, kein add -A/Forcepush/Reset/Stash oder Arbeitsquellrücksetzen. Trailer ausschließlich Co-authored-by: GPT 6.1 Sol <modell@local>. S3/S4 werden nach konsumierter Rückgabe gesondert im erhaltenen Stand fortgeführt. Kein Main, Release, Deploy, Neustart, Runtime, Cleanup oder Settle in diesem engen Fixauftrag.

## 4. Tatsächliche Beweise und Schutzgrenzen

Die früher abgewiesenen Ausführungen sind keine numerischen Beweise. Jetzt den ausdrücklich zugelassenen neuen Weg tatsächlich benutzen: /home/nathanael/.local/bin/cargo-slot, Quelle claude-config/bin/cargo-slot von G nachgelesen. Argumente direkt als Cargoargumente, kein zusätzliches cargo und keine FD/flock-Schleife. Der Wrapper übernimmt drei Slots und bei optimierten Profilen die Release-Sperre. Keine neuen Wrapper, Hook-/Settingsänderungen, direkten Cargo-Prüfläufe oder fremden Prozesse/Sperrdateien.

Bekannter konkreter primärer Verhaltenslauf:

```text
/home/nathanael/.local/bin/cargo-slot test --manifest-path /home/nathanael/.worktrees/brain-g-v2-20261007/rust/Cargo.toml --package dbrain-reasoner --lib --locked --offline --jobs 3 --target-dir /tmp/brain-g-m-0645-target combat::tests -- --include-ignored --test-threads=1
```

Passende bestehende Fälle und unmittelbare Regressionen tatsächlich ausführen: Stack mit/ohne Shred, explizites Szenario ohne Doppel-Shred, doppelte Item-IDs über beide öffentlichen Simulationseingänge, gleiche Stat-/Shop-/Effektpopulation. Originale öffentliche API-Fixtures benutzen, soweit geeignet; synthetische Mutationen im Beleg ausdrücklich kennzeichnen. Keine privaten Daten oder externen Modellaufrufe zur Herkunftsanalyse. Keine echte Wall-Clock in Tests; bestehende Deadlinehilfen benutzen. Befehl, tatsächliche passed/failed/ignored/filtered-Zahlen, Exit, Testnamen und Quellbindung nennen. Null-Lauf ist kein Beweis. Primär-WIP-Teststand vom committed S2-Compilerstand getrennt ausweisen; keine Gesamt-G- oder Produktionsabnahme daraus.

Formatcheck und committed Compiler/striktes Clippy all-targets einschließlich Abhängigkeiten über cargo-slot, locked/offline/jobs 3, vorhandenes Target /tmp/brain-g-m-0645-target. Vorhandenen eigenen sauberen Prüfbaum brain-g-checkpoints-20261007 nur auf eigene konkrete committed Stände setzen, keine Hilfsbaumtests. Eigene Formatänderungen ausschließlich kontrolliert an combat.rs; keine globale mutierende Formatierung.

Anschließend voller S2-Gate gegen S1 bd83d7abdef812a30daa47aa5f6a78de4f42263a bis tatsächlichem Fix-HEAD: /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py mit `--review`. Unverändert bisheriges Urteilmodell gpt-6.1-sol, kein Delta-ALLOW statt Gruppe und kein Modellwürfeln. Einziger Reviewer ist der Gate. Bei BLOCK autonome Schleife mit je frischem nativen Fixer und engem Dateieigentum, nie eigener fortgesetzter Kontext; gleiches Modell und dieselbe ganze Gruppe bis ALLOW, spätestens nach fünf erfolglosen Runden qualifizierte Rückgabe. Keine separate Review-Threads. Native Fixer dafür ausdrücklich erlaubt.

Bei neuem tatsächlichem Schutzproblem präzisen Deny und fehlendes Beweisziel zurückgeben, keinen Umgehungs- oder Wiederholungsweg. Erst tatsächliches ALLOW plus numerischer und committed Compiler-/Clippy-/Formatbeweis gestatten S2-Sicherung auf bestehendem origin-Feature und echten ls-remote-Beleg. Keine Wartepflicht auf fremden Build, keine Übernahme fremden WIPs.

## 5. Routing und Abschlussgrenze

Direkter Auftraggeber G im bestehenden Thread a867ef50-88e6-41ac-a852-724f5184c6e6, native Elternsession 030a7b6f-d25c-482d-b66c-68185cd05dbb, Delegator 481426fe-b477-42b3-91c6-901811fcba1d. Geerbtes Sitzungsmodell, ausdrücklich freigegebenes xhigh, keine Modellüberschreibung. Keine T3-Threads oder Sessionkoordination, kein ListAgents/SendMessage an andere Sessions. Browser wäre ausschließlich Moli nach agent-browser.md, Brave MUST NOT benutzt werden; reine Rechnung benötigt keinen Browser.

Rückgabe bei Task-Ende, echtem neuen Schutzproblem oder qualifiziertem Blocker nach fünf erfolglosen Runden. Keine Meldung je Runde. Tatsächliche Delta-, Test-, Compiler-, Gate-, SHA-, Quellen- und origin-Belege nennen. Spiegel/F/G/K bleibt Abschlussreihenfolge; der geprüfte G-Featurevertrag für F wird unabhängig davon vorbereitet. Private Verarbeitung bleibt gesperrt und blockiert die getrennte öffentliche Lieferung nicht. analytics_runtime bleibt bis expliziter I-Übergabe gesperrt. Wache 25 Minuten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
