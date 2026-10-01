status: erledigt
Datum: 2026-09-29

# Paket D: Consumer-Regressionsabnahme

## Ergebnis

Befunde: 0. Codeänderungen: 0. Die drei Consumer-Pfade wurden an den zugewiesenen Heads geprüft. Docs #4 bleibt am Head `ff20af7a8e3fcacc349cb2d97eda34dfa0897957`; beide Checks sind grün. Die rote 2nd-Brain-CI wurde vor dem Start des Jobs durch GitHub Billing blockiert. Lokale Format-, Test- und Clippy-Läufe sind erfolgreich. Twitch #984 ist in `13321934f421b9a1cd81d0be8668c2e5a8fd925b` enthalten; die Regressionstests auf aktuellem `origin/main` sind erfolgreich.

## 1. Docs PR #4

- Geprüfter Head: `ff20af7a8e3fcacc349cb2d97eda34dfa0897957`, PR offen und Worktree sauber.
- `tools/brain-adapter/src/lib.rs:28-33,72-88`: Request-Scope muss der konfigurierten Menge `{docs.public}` entsprechen. Die Anfrage geht über `AsyncBrainClient`.
- `tools/brain-adapter/src/infisical.rs:79-116,123-170`: Konfiguration prüft den lokalen Brain-Endpunkt. Das Secret wird über den vorhandenen lokalen Infisical-Unix-Socket geladen. Fehler werden als `Configuration` zurückgegeben.
- `tools/brain-adapter/src/main.rs:5-9,11-45,49-64`: Die CLI lädt den Token aus der Konfiguration. Die Kommandos sind `prepare`, `answer` und `query`; die beiden Answer-Pfade rufen `DocsBrainAdapter` und dessen `AsyncBrainClient` auf. Transportfehler werden als `Transport` zurückgegeben und als generische Fehlermeldung ausgegeben, danach endet der Prozess mit Exit 64. `tools/brain-adapter/tests/cli.rs:26-33,78-83` prüft den deaktivierten Environment-Token-Pfad und verwirft frühere endpoint-/timeout-Argumente.
- Twin-Suche in `tools/brain-adapter` führte zu `main.rs` als CLI-Aufrufer und `lib.rs` als Adapterstelle. Weitere Treffer lagen in Tests und Workflow.
- GitHub-Checks am genannten Head: `Typed Brain adapter fixtures` SUCCESS, `GitGuardian Security Checks` SUCCESS. Der Branch blieb am genannten Head; die Prüfungen wurden gemäß Auftrag nicht lokal wiederholt.

## 2. 2nd-Brain PR #2

- Geprüfter Head: `ab83b691befd761a16d971af5c249a604c5d4e0d`, PR offen. Worktree war vor den lokalen Prüfungen sauber.
- GitHub-Job `109214649281` weist `steps: []`, `runner_id: 0` und ein leeres `runner_name` aus. Die Check-Annotation lautet: `The job was not started because recent account payments have failed or your spending limit needs to be increased. Please check the 'Billing & plans' section in your settings`. Das belegt einen externen Billing-Blocker beim Jobstart. Die lokalen Testergebnisse in diesem Abschnitt sind davon getrennt.
- `tools/brain-adapter/src/lib.rs:30-63,103-127`: Die Bindung verlangt genau eine `.internal`-Scope und vergleicht die Query-Scopes exakt mit der Konfiguration. Leere, öffentliche, Wildcard- und abweichende Scope-Sets scheitern vor dem Brain-Aufruf. Transportfehler propagieren als `AdapterError::Transport`.
- `tools/brain-adapter/src/infisical.rs:80-110,122-169` und `tools/brain-adapter/src/main.rs:6-50,53-68`: Die Secret-Auswahl ist auf `BRAIN_SERVE_SECOND_BRAIN_TOKEN` festgelegt. Der Credential-FD wird über den lokalen Socket verwendet. Die CLI-Ausgabe enthält den generischen Adapterfehler. Die CLI-Kommandos sind `prepare`, `answer` und `query`; die Answer-Pfade übergeben die Query an `InternalBrainAdapter`. Der `prepare`-Pfad serialisiert die Query; unbekannte Argumentformen enden mit `InvalidInput`. `tools/brain-adapter/tests/cli.rs:90-113` prüft, dass `BRAIN_ADAPTER_TOKEN` eine Antwort nicht aktiviert.
- Twin-Suche in `tools/brain-adapter` führte zu `main.rs` als produktivem Aufrufer und `lib.rs` als Stelle der Scope-Prüfung. Weitere Treffer lagen in Tests und Workflow.
- Toolchain für lokale Läufe: cargo 1.97.1, rustc 1.97.1. Ein erster Anlauf mit `/usr/bin/cargo` 1.75.0 endete vor dem Build mit Exit 101, weil dieser Cargo das Lockfile v4 nicht lesen konnte. Die Wiederholung mit der installierten 1.97.1-Toolchain war erfolgreich.
- `PATH=/home/nathanael/.cargo/bin:$PATH cargo fmt --manifest-path tools/brain-adapter/Cargo.toml -- --check` Exit 0.
- `PATH=/home/nathanael/.cargo/bin:$PATH CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true cargo test --manifest-path tools/brain-adapter/Cargo.toml --all-targets --locked --offline` Exit 0: 16 passed, 0 failed, 0 ignored, 0 filtered.
- `PATH=/home/nathanael/.cargo/bin:$PATH CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true cargo clippy --manifest-path tools/brain-adapter/Cargo.toml --all-targets --locked --offline -- -D warnings` Exit 0.
- Consumer-Quellcode und PR-Branch blieben am geprüften Stand. Lokale Prüfprotokolle wurden nach dem Erfassen der Zähler entfernt.

## 3. Twitch PR #984

- PR-Head `33ce4ac45edf93b5b455b16672faf935ad135dd5` wurde am 2026-09-26 in Merge-Commit `13321934f421b9a1cd81d0be8668c2e5a8fd925b` gemergt. Der neue Testworktree basiert auf `origin/main` `cf3d77085ed350554914b14c3d9981d37b95903a`; `git merge-base --is-ancestor` bestätigte den Merge-Commit mit Exit 0.
- GitHub-Checks am PR-Head: `Typed Brain fixtures (knowledge)`, `Typed Brain fixtures (self-explainer)` und `Rust SQLx required` SUCCESS. `Semantic review` endete mit Exit 1. Die Annotation enthält `Process completed with exit code 1.`
- `rust/crates/tb-config/src/dashboard_options.rs:10-60`: `BrainClientMode` listet `legacy`, `shadow` und `typed`. Nicht unterstützte Werte ergeben einen Fehler beim Enum-Parsing. Der Snapshot-Loader gibt Parsing- und Validierungsfehler zurück (`rust/crates/tb-config/src/file.rs:201-212`). Nicht-Legacy-Modi benötigen Endpoint und öffentliche Scopes.
- `rust/crates/tb-dashboard-api/src/handlers/self_explainer.rs:789-805,855-885`: Typed-Fehler ergeben eine `unavailable`-Antwort. Im Shadow-Zweig läuft die Brain-Probe als eigener `tokio::spawn`-Task; die sichtbare Antwort kommt aus `legacy_route_answer`. `typed_route_answer` hat ein eigenes 8-Sekunden-Limit. Der Elternpfad wartet nicht auf den Probe-Task.
- `rust/crates/tb-knowledge/src/brain.rs:92-132`: Brain-Client-Fehler werden als Backend-Fehler weitergegeben; Antwortstatus werden typisiert projiziert.
- `PATH=/home/nathanael/.cargo/bin:$PATH SQLX_OFFLINE=true CARGO_BUILD_JOBS=2 cargo test --manifest-path rust/Cargo.toml -p tb-dashboard-api --lib self_explainer::tests --locked --offline` Exit 0: 21 passed, 0 failed, 0 ignored, 1264 filtered. Enthalten sind `typed_port_does_not_drop_history_or_invoke_legacy_fallbacks` und `shadow_probe_laeuft_abgekoppelt_und_blockiert_die_legacy_antwort_nicht`.
- `PATH=/home/nathanael/.cargo/bin:$PATH SQLX_OFFLINE=true CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true cargo test --manifest-path rust/Cargo.toml -p tb-knowledge --all-targets --locked --offline` Exit 0: 31 passed, 0 failed, 0 ignored, 0 filtered.
- `PATH=/home/nathanael/.cargo/bin:$PATH cargo fmt --manifest-path rust/Cargo.toml -p tb-knowledge -- --check` und `PATH=/home/nathanael/.cargo/bin:$PATH rustfmt --edition 2021 --check rust/crates/tb-dashboard-api/src/handlers/self_explainer.rs` Exit 0.
- Kompilierung meldete eine Deprecated-Warnung in `rust/crates/tb-dashboard-api/src/uplink_config.rs:787` für `nix::sys::memfd::MemFdCreateFlag`. Die aufgeführten Tests waren erfolgreich.
- Twin-Suche führte zu Typed-, Legacy- und Shadow-Routen in `self_explainer.rs` sowie zum Adapter in `tb-knowledge/src/brain.rs`. Der gemeinsame Twitch-Checkout war bereits mit unversionierten Dateien belegt; die Prüfungen liefen im isolierten Worktree.

## Wirkung und Grenzen

Geprüfte Fremddienstpfade: Docs zu Infisical und Brain, 2nd-Brain zu Infisical und Brain sowie Twitch zu Brain. Infisical-Aufrufe setzen einen 10-Sekunden-Timeout und propagieren Fehler als `Configuration` an die CLI. Der Brain-Client setzt den konfigurierten Request-Timeout und sendet pro Aufruf einen POST (`brain-client/src/async_client.rs:59-85`). Die CLI gibt typisierte Adapterfehler aus; der Twitch-Handler loggt Fehlerpfade. Ein erneuter CLI- oder HTTP-Aufruf startet einen neuen Request. Die Prüfungen liefen offline mit lokalen Fixtures; produktive Services, Datenbanken und Nachrichten waren nicht beteiligt.

## Pflichtnachweise

INTENT[IA-1]: Stufe klein | Modell Luna | Thread 562a877b-0939-440a-964d-1145d9e9431a | Register: /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/REGISTER.md
ORCHESTRIERUNG[OR-1]: Stufe klein | Schritt done | Artefakt: .tasks/2026-09-29-technical-closeout/D-REPORT.md
WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 5/5 geprüft
TESTNACHWEIS[TW-1]: 68 passed, 0 ignored | Baseline: nicht separat gemessen, Consumer-Code unverändert
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: .tasks/2026-09-29-technical-closeout/D-REPORT.md
MERGEPROTOKOLL[MS-1]: 3 Git-Schritte einzeln | Anläufe: 0 | Gate: nicht ausgelöst, Branch-PR nach migration/rust-integration
