# G-S2-Restkern: genau ein frischer nativer Fixer

status: startbereit nach direkter begrenzter Fortsetzungsentscheidung, 07.10.2026

## 1. Ziel und Vertrag

Verbindlich: /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/ENTSCHEIDUNG-G-S2-RESTKERN.md, tatsächlich durch G gelesen. Derselbe offene G-Auftrag. Zwei nach Graphify bestätigte combat.rs-Kerne gemeinsam eng korrigieren: Stackbonus durch dieselbe vorhandene Schadensstufe einschließlich Rüstungsverringerung; doppelte Item-IDs vor Aufbau der Effektzustände mit der vorhandenen gemeinsamen Normalisierung behandeln. Default-/Fast-/Bindingpfade und beide öffentlichen simulate_calculation-Eingänge betrachten. Im expliziten Szenario Shred nicht zweimal anwenden. Stat-, Shop- und Effektpopulation müssen übereinstimmen. Kein zweiter Inventar-, Effekt- oder Rechenpfad.

Lies G/G-M-S2-BLOCK-NACHWEISE.md und den tatsächlichen letzten Gate unter G/pruefungen/g-m-checkpoints-r2/s2-fix-r6/gate.log. Quellorte im geprüften Arbeitsstand: evaluate_core_with_deadline :545 normalisiert IDs; simulate_calculation_inner :841 reicht Items unverändert weiter; simulate :1027 erzeugt Zustände je Vorkommen; Bulletanteil :1325 enthält im Defaultpfad Shred, Stackanteil :1927 nicht. Vor Umsetzung code-suche/Graphify und den vorhandenen Bestand prüfen, keine neue Parallelimplementierung.

## 2. Eigentum

Produktdatei ausschließlich rust/crates/dbrain-reasoner/src/combat.rs und ihre unmittelbar vorhandenen numerischen Tests. Alle fünf vorhandenen Fixcommits erhalten. Keine neue Modellformel, Kommentare, Referenz-Itemnamen im Produktcode, Konfiguration, Abhängigkeiten, Loader, Planer oder I-/K-Dateien. S3-Deadlinewrapper bleibt unstaged; andere S3-/S4-Quellen und Originalfixtures bytegleich erhalten. Keine Testabschwächung. Eigene Belege ausschließlich G/pruefungen/g-m-s2-restkern/. Zentrale REGISTER/PLAN/REVIEW/AN_HAUPT/TODO nicht bearbeiten; G ist deren Statusproduzent.

## 3. Arbeitsstand und Wirkung

Worktree /home/nathanael/.worktrees/brain-g-v2-20261007, Branch feat/brain-v2-g-20261007. Start-HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206, von G jetzt geprüft. Index zuvor leer. Originaler S1 und tatsächlicher letzter origin-Stand bd83d7abdef812a30daa47aa5f6a78de4f42263a. Endmanifest g-m-checkpoints-r2/bereichs-quellen-ende.sha256 durch G erneut 12/12 bestätigt. Nur combat.rs darf bewusst abweichen; elf andere Quellen einschließlich Originalfixtures erhalten.

Exklusives Produkt-/Git-Schreibrecht für deinen engen Fix; Elternsession führt währenddessen keine Git-Schritte aus. Eigener Fixcommit erlaubt. Push nach bestehendem Featurebranch erst nach tatsächlichem vollen S2-ALLOW UND den hier verlangten Nachweisen. Git einzeln mit literalen absoluten Pfaden, nur eigene Dateien, kein add -A, Forcepush, Reset, Stash oder Arbeitsquellrücksetzen. Trailer ausschließlich Co-authored-by: GPT 6.1 Sol <modell@local>. Bestehenden eigenen sauberen Prüfbaum brain-g-checkpoints-20261007 ausschließlich für committed Compiler/Clippy verwenden, keine Hilfsbaumtests. Kein Main, Release, Deploy, Neustart, Runtime, Cleanup, Settle oder S3/S4-Beginn in diesem Fixauftrag.

## 4. Beweisziel und Sperren

Numerische Prüfung ist diesmal ausdrücklich beauftragt, nicht durch Compiler ersetzbar. Passende bestehende Fälle tatsächlich ausführen: Stackschaden mit/ohne Shred und doppelte Item-IDs über beide öffentlichen Simulationseingänge. Default-/Fast-/Bindingpfade sowie explizites Szenario ohne doppelte Schadensstufe abdecken. Vorhandene originale öffentliche API-Fixtures benutzen, soweit geeignet. Synthetische Mutationen ausdrücklich im Beleg kennzeichnen. Keine privaten Daten oder Modellaufrufe. Tests ohne echte Wall-Clock, bestehende RequestDeadline-Helfer benutzen. Tatsächliche Befehle, passed/failed/ignored/filtered, Exit, konkrete Testnamen und Quellbindung melden; Null-Lauf ist kein Beweis. Primär-WIP-Teststand und committed S2-Stand getrennt beschreiben.

Vorhandener zugelassener Slotvertrag wurde von G über Graphify und ABLAUF.md gefunden und HOSTPROBE.md gelesen: /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md. Drei Build-Slots über FD/flock auf locks/build-slot-1.lock bis -3.lock, jobs 3, danach freigeben. check/clippy/test brauchen diesen Slot; der zusätzliche /tmp/deadlock-cargo-release.lock gehört zu Release-Builds. Keine neue Wrapper-Architektur. Die früher abgewiesene vorangestellte flock-Release-Form für cargo test nicht wiederholen, kein direkter Cargo-Test ohne Slot, keine verschachtelten Ersatzwege, keine anderen Werkzeuge oder Worker als Ausweg. Kein Hook-, Settings-, Sperrdatei- oder Fremdprozess-Eingriff.

Tatsächliche Prüfsperre einmal exakt mit Deny und fehlendem numerischem Beweisziel zurückgeben. Kein fingierter Lauf und keine S2-Sicherung allein aufgrund Compiler/Gate, solange die ausdrücklich verlangte numerische Prüfung fehlt. Format, committed Compiler und striktes Clippy all-targets einschließlich Abhängigkeiten, locked/offline, jobs 3; vorhandenes Target /tmp/brain-g-m-0645-target.

Anschließend voller gemeinsamer S2-Gate: base bd83d7abdef812a30daa47aa5f6a78de4f42263a, head tatsächlicher enger Fixstand, /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py mit `--review`. Unverändert gpt-6.1-sol als bereits urteilsgebendes Modell. Einziger Reviewer ist der Gate, keine Reviewthreads und keine bloße Deltafreigabe. Bei erneutem BLOCK pro Runde frischen nativen Fixer mit engen konkreten Funden starten, nie den eigenen Kontext fortsetzen; dieselbe ganze Gruppe und dasselbe Urteilmodell bis ALLOW, spätestens nach fünf erfolglosen Runden qualifizierte Rückgabe. Keine Denys an Fixer übertragen. Nach echtem ALLOW und passenden Nachweisen S2 auf dem bestehenden origin-Feature sichern und den tatsächlichen ls-remote-Stand belegen. S3/S4 werden durch G anschließend gesondert im erhaltenen Stand fortgesetzt.

## 5. Routing und Rückgabe

Auftraggeber G, Delegator 481426fe-b477-42b3-91c6-901811fcba1d, Haupt-Orchestrator d3a1741e-82bc-4a48-865b-2845c663dca7, native Elternsession 030a7b6f-d25c-482d-b66c-68185cd05dbb. Du bist genau der eine frische native Fixer für beide Kerne. Geerbtes Sitzungsmodell und ausdrücklich freigegebenes xhigh, keine Modellüberschreibung. Weitere native Fixer nur bei tatsächlichem BLOCK gemäß obiger Schleife. Keine T3-Threads, Sessionkoordination, ListAgents oder SendMessage an andere Sessions. Browser wäre Moli nach agent-browser.md, Brave MUST NOT benutzt werden; diese reine Rechnung braucht keinen Browser.

Fachrückgabe erst bei Task-Ende, tatsächlicher Prüfsperre oder qualifiziertem Blocker nach fünf erfolglosen Runden. Delta, beide Kernnachweise, wirkliche Testzahlen, SHA/Eltern, committed Prüfungen, voller Gate, Quellbindung und origin präzise nennen. Keine Meldung je Runde, keine Gesamt-G-/Livefreigabe. Wache 25 Minuten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
