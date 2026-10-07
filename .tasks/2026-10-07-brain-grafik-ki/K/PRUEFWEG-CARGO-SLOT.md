# K: Tatsächliche Nachprüfung über cargo-slot am 7. Oktober 2026

## Neuer zulässiger Weg und verbleibende Werkzeuggrenze

Neue ausdrückliche Nutzerfreigabe: Cargo ausschließlich über cargo-slot, Arbeitsroots ~/.worktrees, ~/repos und /tmp. Keine Wiederverwendung alter flock-/FD-Runner. Frischer nativer Fixturefixer aab063d35034d0c46 stoppte dennoch an einer tatsächlichen ctx_execute_file-Projektrootgrenze: Worktreezugriff wird auf /home/nathanael/repos/Deadlock-Brain beschränkt. Keine Fixtureänderung, Compiler-, Format-, Clippy- oder Testausführung durch diesen Worker. Keine Wiederholung oder Umgehung dieser Werkzeugablehnung. Die zulässigen direkten Cargoaufrufe der Hauptsession liefen tatsächlich; das ist kein Beweis behobener ctx-Rootbindung.

Brainquelle: eigener detached HEAD 9fc48a08, uncommittierter enger Budgetfix in provider_input.rs, brain-providers/lib.rs und answer_provider_tests.rs. Retrievalfixture unverändert. Scope ist kein G-Port, kein neuer Rechner und keine Live-/Gesamtabnahme.

## Tatsächliche Befehle und Resultate

1. `SQLX_OFFLINE=true cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-k-ki-20261007/rust/Cargo.toml -p brain-contracts -p brain-providers --locked --offline --jobs 3 --no-fail-fast -- --include-ignored`

   Log /tmp/k-budget-contracts-providers-cargo-slot-20261007.log, Task b9jobv241, Exit 0. 79 passed, 0 failed, 0 ignored, 0 filtered. Teilzahlen 17/20/23/11/8; zwei Doctesttargets jeweils 0. Tatsächlicher Compiler 36,19 s. Bestehende Contracts und Provider einschließlich Transportbudget und Accounting liefen; synthetische lokale HTTP-Fixtures, keine privaten Daten.

2. `SQLX_OFFLINE=true cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-k-ki-20261007/rust/Cargo.toml -p brain-serve --lib service::answer_provider_tests --locked --offline --jobs 3 --no-fail-fast -- --include-ignored`

   Log /tmp/k-budget-serve-enum-cargo-slot-20261007.log, Task bbrkc28mi, Exit 0. 6 passed, 0 failed, 0 ignored, 48 filtered. Tatsächlicher Compiler 60 s. Beide echten zentralen Enumzweige gegen lokale HTTP-Fixtures, alle vier Methoden, vollständige Tools/Historie, Rechte, Deadline, Accounting und die verstärkten konkreten Payload-Budgetgrenzen geprüft. Kein echter Liveproviderlauf.

3. `SQLX_OFFLINE=true cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-k-ki-20261007/rust/Cargo.toml -p dbrain-sources --test entity_semantic consumer_patch_conditions_keep_decimal_thresholds_and_withhold_unsafe_statements --locked --offline --jobs 3 -- --include-ignored`

   Log /tmp/k-source-decimal-cargo-slot-20261007.log, Task bpau0gvyw, Exit 0. 1 passed, 0 failed, 0 ignored, 12 filtered. Tatsächlicher Compiler 106 s. Fehlender bestehender Dezimalerhaltstest der bereits committed gemeinsamen Source-Parsernaht jetzt tatsächlich ausgeführt. Kein Nachweis der gesamten Source-Suite.

4. `SQLX_OFFLINE=true cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-k-ki-20261007/rust/Cargo.toml -p dbrain-retrieval --test chunked_retrieval --locked --offline --jobs 3 --no-fail-fast -- --include-ignored`

   Log /tmp/k-retrieval-cargo-slot-20261007.log, Task b2ppqwp9x, Exit 101. 3 passed, 15 failed, 0 ignored, 0 filtered. Tatsächlicher Compiler 64 s. Die roten Fälle scheitern überwiegend an InvalidResponse("Ungültiges Release-Lesemanifest"); der Budgetfehlerfall bekommt ebenfalls nicht seine erwartete Fehlerart. Die bekannte Obergrenzenassertion bei Zeile 203 wird im großen Heldenfall nicht erreicht, weil bereits retrieve bei Zeile 197 am Lesemanifest scheitert. Deshalb ist die vorher nur aus dem Source abgeleitete Differenz 31 weiterhin kein gemessener Assertionfehler. Keine Behauptung vorbestehender Fehler ohne Zahlenbaseline, keine Abschwächung oder Skips.

## Neue echte Vorfixbaseline und Budgetcheckpoint

Am 7. Oktober 2026 eigener temporärer detached Baselineworktree von unverändertem 38280ca8, vor dem Drei-Dateien-Budgetfix. Kein WIP verschoben, keine Fixture gelesen oder geändert, keine Schutzumgehung. Tatsächlicher Befehl:

`SQLX_OFFLINE=true cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-k-budget-baseline-20261007/rust/Cargo.toml --target-dir /home/nathanael/.worktrees/brain-k-ki-20261007/rust/target -p dbrain-retrieval --test chunked_retrieval --locked --offline --jobs 3 --no-fail-fast -- --include-ignored`

Task bqec8u4xj, Log /tmp/k-retrieval-prebudget-baseline-20261007.log, Exit 101. Tatsächlicher Compiler 31,14s; 3 passed, 15 failed, 0 ignored, 0 filtered. Namen der 15 roten Fälle gegen aktuelle Budgetfassung maschinell verglichen: identische Menge, keine nur-baseline oder nur-aktuell roten Fälle. Das belegt Vorbestehen gegenüber diesem Budgetfix, nicht gegenüber früherem main oder dem ganzen K-Paket. Aktuelle Fehler bleiben zu beheben; gleiche Zahlen sind kein grüner Retrievalnachweis. Die große Obergrenzenassertion wird weiter nicht erreicht.

Eigener Baselineworktree nach abgeschlossenem Lauf und tatsächlich leerem `git status --short --ignored` entfernt. Keine wertvollen ignorierten Artefakte im Baum, Logs und der bereits gesicherte Checkpoint bleiben erhalten. Kein fremder Baum oder Prozess berührt.

Budgetquellen als ca9fe4bb committed: provider_input.rs, brain-providers/lib.rs, answer_provider_tests.rs, +92/-32. Neue positive Scopeprüfungen oben 79 und 6 bestehen für diese identische Sourcefassung. Zusätzlicher tatsächlicher Compiler /tmp/k-brain-main-budget-check-20261007.log 25,05s, striktes Clippy /tmp/k-brain-budget-cargo-slot-clippy-20261007.log 19,41s und Formatcheck bet04wxab jeweils Exit 0. Gemeinsamer regulärer Folgegate b1e3bp2a0 gegen frisch geprüftes origin/main ca4d877f auf ca9fe4bb tatsächlich abgeschlossen und gelesen: `[gpt-6.1-sol] ALLOW: No confirmed merge-blocking defect in the supplied code.` Log /tmp/k-brain-main-budget-followup-gate-20261007.log. Zwei offene NITs: Retrieval-Grenzassertion noch nicht erreicht; Produktionsstart über pg_pool_from_config mit vorhandenem brain_site-Rollen-/Kommentarschema nicht belegt. Der frühere Inputbudget-BLOCK ist für diesen Codecheckpoint ersetzt; ALLOW ist keine Behauptung grüner Retrievalsuite oder erfolgreicher Produktion. Kein Brain-Mainpush oder Deploy.

TESTNACHWEIS[TW-1]: 3 passed, 0 ignored | Baseline: 15 rot

## Historisches Urteil vor der neuen Vorfixbaseline

Die neuen 86 positiven scoped Fälle ersetzen die alte pauschale Testsperrenbehauptung für diese konkreten Cargoaufrufe. Die Retrievalsuite ist tatsächlich rot, die erforderliche Fixturekorrektur weiterhin nicht umgesetzt. Letzter gemeinsamer Main-Gate bleibt BLOCK auf 9fc48a08; kein neuer ALLOW, Commit des Budgetfixes oder Brain-Mainpush. Rootbindung und vorhandene Fixtureverträge korrekt reparieren lassen, danach erneut echte Retrievalprüfung und reguläre Gatefolgeschleife mit demselben Urteilmodell. Keine bereits gesicherten Antwort-/Artefaktpfade neu bauen.

## Tatsächliche Twitch-Consumerprüfungen

Eigener unveränderter Twitch-HEAD 2ead4d55, bereits auf origin/main und im regulären Release deployed.

`SQLX_OFFLINE=true cargo-slot +1.99.0 test --manifest-path /home/nathanael/.worktrees/twitch-k-ki-20261007/rust/Cargo.toml -p tb-knowledge --locked --offline --jobs 3 --no-fail-fast -- --include-ignored`

Log /tmp/k-twitch-knowledge-cargo-slot-20261007.log, Task bzu8k8fwc, Exit 0. 33 passed, 0 failed, 0 ignored, 0 filtered, Teilzahlen 23/3/7. Doctesttarget 0. Vorhandene echte Testausführung aus warmem Cache, kein neuer Releasebau behauptet.

`SQLX_OFFLINE=true cargo-slot +1.99.0 test --manifest-path /home/nathanael/.worktrees/twitch-k-ki-20261007/rust/Cargo.toml -p tb-bot --bin tb-bot brain_chat_wiring --locked --offline --jobs 3 --no-fail-fast -- --include-ignored`

Log /tmp/k-twitch-chat-consumer-cargo-slot-20261007.log, Task b8ygie6x3, Exit 0. 13 passed, 0 failed, 0 ignored, 312 filtered. Tatsächlicher Compiler 93 s, Testausführung 19,04 s. Ein Compilerwarning im vorhandenen Testcode brain_chat_wiring.rs:1236: Atomic::fetch_update auf diesem Compiler als deprecated markiert. Kein Compiler-/Testfehler, kein ungerechtfertigter neuer Clippynachweis oder ungefragter Nebenscopefix.

Regulärer lesender Wrapper erneut Exit 0: vier aktive neue PIDs 1807853/1807722/1809122/1809132 auf Release 2ead4d55, keine gelöschten exes, NRestarts 0. Neuer Anker "Brain-Chat meldet Antwortausfall" mittels binärer exakter Suche in der tatsächlich installierten Releasebinary nachgewiesen, Exit 0. Echte Chat-/Testkontoprobe und UI-Ort weiterhin offen, kein vollständiger LIVEBEWEIS oder Cleanup.

TESTNACHWEIS[TW-1]: 135 passed, 0 ignored | Baseline: nicht neu gemessen, 15 aktuelle Retrievalfälle rot
