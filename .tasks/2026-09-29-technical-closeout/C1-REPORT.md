status: erledigt
Datum: 2026-09-29

# C1 Prozess-Abnahme

R-AC-C1 auf dem übernommenen Worktree und Branch `fix/pre-g5-harness-20260929` behoben. Implementierungscommit: `1882419b6178174df7b7a225561a3ec653cac430`, auf Basis `e671c5b79c63857869560d551bf9202e2f81677d`.

## Änderungen

- `scripts/test_brain_serve.sh` erstellt zwei eigene Scratch-Datenbanken für die Schema-Startprüfungen. Die historischen Datenbanknamen bleiben von den neuen Fixtures getrennt.
- `rust/crates/brain-serve/tests/process_e2e.rs` prüft am echten Prozess inkompatibles Schema v99 samt unverändertem Zustand, leere Datenbank ohne selbst angelegte Schemaobjekte, englischen Alias Guardian, unbekannte Entity, Alias-Konflikt über den normalen Retrieval-Pfad mit Limit 1, die konkrete HTTP-403-Antwort `forbidden` und eine fachlich erfolgreiche Anfrage nach DB-Recovery im selben Prozess.
- Die Laststufen 600 bei 8, 16 und 32 Workern sowie das Poollimit blieben unverändert. Der historische `C-REPORT.md` blieb unberührt.

## Verifikation

| Befehl | Exit | Ergebnis |
| --- | ---: | --- |
| `$HOME/.cargo/bin/cargo +stable fmt --manifest-path rust/Cargo.toml --package brain-serve -- --check` | 0 | Format geprüft |
| `$HOME/.cargo/bin/cargo +stable clippy --manifest-path rust/Cargo.toml --locked --offline --jobs 2 -p brain-serve --all-targets -- -D warnings` | 0 | Clippy ohne Warnungen |
| `bash -n scripts/test_brain_serve.sh` | 0 | Runner-Syntax gültig |
| `./scripts/test_brain_serve.sh > /tmp/brain-c1-process.log 2>&1` | 0 | 3 bestanden, 0 fehlgeschlagen, 0 ignoriert, 8 gefiltert; Prozess-E2E bestanden |
| `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-pre-g5-harness-20260929 --base e671c5b79c63857869560d551bf9202e2f81677d --head 1882419b6178174df7b7a225561a3ec653cac430 --effort high --timeout 600` | 0 | `ALLOW: No merge-blocking defect is evident in the supplied diff.` |

Lastnachweis aus dem Prozesslauf: je 600/600 beantwortet bei 8, 16 und 32 Workern; beobachtete Reader-Verbindungen jeweils 4, Peak 4, Serverlimit 12. Pool-Wartebudget und Retrieval-Budgets wurden nicht verändert.

Der erste Runner-Anlauf endete mit Exit 101, weil der Legacy-Importtest bereits `brain_legacy_test` angelegt hatte. Die Schema-Fixtures erhielten daraufhin eigene Datenbanknamen im privaten Cluster; der vollständige Folgelauf endete mit Exit 0. Die leere Datenbank meldet derzeit präzise `database_unavailable`, während Schema v99 `core_schema_incompatible` meldet. Der bestehenden Service-Codepfad wurde außerhalb des Testauftrags nicht geändert.

Baseline nicht erhoben; es wird kein vorbestehender Testfehler behauptet. Keine echten Daten, Secrets oder produktiven Datenbanken verwendet. Kein Merge und kein Produktionszugriff.

## Nachtrag R2

Implementierungscommit: `5c594c4e8e7b869f8f01dd16df7fa37b3ea68116`, auf Basis des bereits gepushten C1-Stands `ead4a7915a8edc82b9c4b27235852c0a481fee89`.

- Die Limit-1-Prüfung läuft jetzt vor dem Warden-Widerruf auf `pilot-r2`. Warden und Abrams werden unter `game.public` einzeln mit ihren widersprüchlichen Werten 770 und 650 als beantwortet und mit je einer Citation bestätigt. Die Guardian-Anfrage wird danach mit `InsufficientEvidence` abgewiesen. Die nachfolgende Widerrufsprüfung und alle bisherigen Lastfälle bleiben erhalten.
- Ein separater Test im privaten Scratch-Cluster verwendet eine zur Laufzeit erzeugte synthetische SCRAM-Passphrase ausschließlich im Testprozessspeicher. PostgreSQL speichert einen `SCRAM-SHA-256`-Verifier, akzeptiert die richtige Passphrase und weist die falsche mit SQLSTATE `28P01` ab. Der Test bestätigt, dass weder `PGPASSWORD` noch `BRAIN_SERVE_PG_PASSWORD` gesetzt sind. Es wird keine Service-Passphrase per Umgebung übergeben.
- Die bestehenden Prozess-Redaktionstests laufen jeweils separat: fehlerhafte Inline-Konfiguration und nicht zurückgespiegelte Argumente. Beide verwenden synthetische Werte, keine PostgreSQL-Passphrase.

## Verifikation R2

| Befehl | Exit | Ergebnis |
| --- | ---: | --- |
| `$HOME/.cargo/bin/cargo +stable fmt --manifest-path rust/Cargo.toml --package brain-serve -- --check` | 0 | Format geprüft |
| `$HOME/.cargo/bin/cargo +stable clippy --manifest-path rust/Cargo.toml --locked --offline --jobs 2 -p brain-serve --all-targets -- -D warnings` | 0 | Clippy ohne Warnungen |
| `bash -n scripts/test_brain_serve.sh` | 0 | Runner-Syntax gültig |
| `./scripts/test_brain_serve.sh > /tmp/brain-c1-followup-process.log 2>&1` | 0 | 6 bestanden, 0 fehlgeschlagen, 0 ignoriert, 26 gefiltert |
| `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-pre-g5-harness-20260929 --base e671c5b79c63857869560d551bf9202e2f81677d --head 5c594c4e8e7b869f8f01dd16df7fa37b3ea68116 --effort high --timeout 600` | 0 | `ALLOW: No merge-blocking defect is evident in the supplied diff.` |

Lastnachweis R2: je 600/600 beantwortet bei 8, 16 und 32 Workern; Reader-Peak 4 bei Poollimit 4 und unverändertem Serverlimit 12. Retrieval-Limit, Abfragebudgets und Poolbudgets blieben unverändert. Die SCRAM-Probe ist ein echter Authentisierungslauf gegen die nur lokale Wegwerf-PostgreSQL-Instanz. Sie startet `brain-serve` nicht mit Passwort-Authentisierung, weil dabei eine Passwortübergabe über ENV nötig wäre und ausdrücklich ausgeschlossen ist.

INTENT[IA-1]: Stufe mittel | Modell gpt-6-luna | Thread 562a877b-0939-440a-964d-1145d9e9431a | Register: .tasks/2026-09-29-technical-closeout/REGISTER.md
ORCHESTRIERUNG[OR-1]: Stufe mittel | Schritt done | Artefakt: .tasks/2026-09-29-technical-closeout
TESTNACHWEIS[TW-1]: 6 passed, 0 ignored | Baseline: nicht erhoben
