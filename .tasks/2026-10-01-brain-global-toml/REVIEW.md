status: aktiv
Stand: 2026-10-01

# Review Runde 1

Review-Gate: `gpt-6.1-sol`
Basis: `main`
Head: `luna/finish-brain-global-toml-20261001`
Urteil: `BLOCK`

## Befunde

1. **BLOCKING**, `rust/crates/deadlock-brain/src/main.rs:1219`: `wiki refresh` erwirbt vor dem Aufruf seiner konfigurierten Verbindung den nicht benötigten Standardpool. Bei einem nicht verfügbaren Standardpool erreicht der Refresh seine explizite `pg_pool_from_config`-Verbindung nicht. Maßnahme: Refresh vor Pool-Erwerb dispatchen.
2. **NIT**, `rust/crates/deadlock-brain/src/wiki_refresh.rs:71`: `source_repository` wird durch das Brain-Projektverzeichnis ersetzt, obwohl der Importer dort `data/version.txt` und die Quellenablage erwartet. Maßnahme: den im Refresh-JSON konfigurierten Quellenpfad erhalten.
3. **NIT**, `rust/crates/deadlock-brain-core/src/http.rs:130`: Gleichheit mit den Standardwerten unterscheidet geerbte Defaults nicht von expliziten identischen Request-Policies. Maßnahme: Vererbung mit optionalen Timeout- und Retry-Werten ausdrücken und identische explizite Werte testen.
4. **NIT**, `rust/crates/deadlock-brain/src/main.rs:1179`: `population` erhielt den validierten Settings-Snapshot nicht. Maßnahme: Snapshot an `run_population` weiterreichen und Timeout, Retry-Anzahl sowie Backoff des API-Clients daraus beziehen. Erledigt in `rust/crates/dbrain-population/src/cli.rs` und `api.rs`.

## Nacharbeit

Befunde 1 bis 4 sind im selben Thread korrigiert.

### Validierung

- Workspace-Compile erfolgreich mit Rust 1.98 und `--locked`. Für die vier fehlenden externen Pfade wurden vorübergehend die gepinnten Brain/Bots-Abhängigkeiten verwendet; alle Manifeständerungen sind zurückgenommen.
- `RUSTC=/home/nathanael/.rustup/toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/rustc /home/nathanael/.rustup/toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/cargo test --manifest-path rust/Cargo.toml --locked -p deadlock-brain-core --lib http::tests`: 5 passed, 0 failed, 0 ignored, 27 filtered.
- `RUSTC=/home/nathanael/.rustup/toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/rustc /home/nathanael/.rustup/toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/cargo test --manifest-path rust/Cargo.toml --locked -p dbrain-population --lib api::tests`: 2 passed, 0 failed, 0 ignored, 24 filtered.
- Gezielt geänderte HTTP- und Population-Dateien bestehen `rustfmt --check`.
- Workspace-weites `cargo fmt --all -- --check` bleibt wegen Formatabweichungen in weiteren Workspace-Bereichen rot. Diese wurden nicht pauschal umformatiert.

Anschließend erneuter Gate-Lauf. Bei ALLOW folgen Merge, Push, Deployment, Dienstrestart, Livebeleg und Cleanup.

# Review Runde 2

Review-Gate: `gpt-6.1-sol`
Basis: `main`
Head: `44e8097`
Urteil: `BLOCK`

## Befunde

1. **BLOCKING**, `rust/crates/deadlock-brain/src/wiki_refresh.rs`: `RefreshArgs.config` teilte die Clap-ID `config` mit der globalen TOML-Option. Maßnahme: eine eigene Argument-ID vergeben.
2. **NIT**, `rust/crates/dbrain-retrieval/src/game_wiki.rs`: Suchpfade nutzten nicht `game_wiki_dir/current` und konnten deshalb den aktiven Snapshot verfehlen oder alte Generationen durchsuchen. Maßnahme: den `current`-Symlink auflösen und Legacy-Verzeichnisse ohne Symlink weiter unterstützen.

## Nacharbeit

- Befund 1 ist in `7c4c11f` mit `id = "refresh_config"` behoben. `deadlock-brain wiki refresh --help` zeigt getrennt `--config` und `--refresh-config`.
- Befund 2 ist in `7c4c11f` behoben. Wiki-Suche und Helden-Dossiers lösen den verwalteten `current`-Symlink auf; fehlender `current`-Symlink behält das bisherige Legacy-Verhalten.
- Regressionstest belegt die Suche im aktiven Snapshot und schließt archivierte Generationen aus.

### Validierung

- `cargo test --manifest-path rust/Cargo.toml --locked -p dbrain-retrieval --lib`: 45 passed, 0 failed, 15 ignored.
- `cargo check --manifest-path rust/Cargo.toml --locked -p deadlock-brain`: erfolgreich.
- CLI-Hilfe für `wiki refresh` zeigt beide Konfigurationsoptionen ohne ID-Kollision.
- Für den Compile- und Testlauf wurden vier fehlende externe Cargo-Pfade vorübergehend auf die gepinnten Abhängigkeiten im isolierten Layout gesetzt; alle Manifestpfade sind wiederhergestellt.
- `git diff --check` ist sauber. `rustfmt --check` meldet vorhandene Abweichungen in den angefassten Dateien; es wurden keine fremden Formatbereiche umgeschrieben.

# Review Runde 3

Review-Gate: `gpt-6.1-sol`
Basis: `origin/main` (`39710e3282c830ee9b47e90945deb71d1db44724`)
Head: `7c4c11f771fd9375cc45ec426e063179c4ebb5d3`
Urteil: `ALLOW`

Gate-Antwort: `ALLOW: Distinct Clap ID fixes the blocker; no blocking regressions found in the supplied delta.`

# Review Runde 4

Review-Gate: `gpt-6.1-sol`
Basis: `origin/main` (`39710e3282c830ee9b47e90945deb71d1db44724`)
Head: `f606b41`
Urteil: `ALLOW`, drei NITs

## Befunde und Nacharbeit

1. **NIT**, `rust/crates/dbrain-population/src/api.rs:33`: der Population-Client ignorierte `http.user_agent`. Behoben in `a46ad05`, der Client verwendet `Settings.user_agent`.
2. **NIT**, `rust/crates/deadlock-brain/src/main.rs:1317`: `wiki rebuild` schrieb direkt in die Publication-Root statt einen neuen aktiven Snapshot zu veröffentlichen. Behoben in `a46ad05`, `wiki rebuild` nutzt jetzt denselben validierten Staging- und Cutover-Pfad wie `wiki refresh`.
3. **NIT**, `rust/crates/deadlock-brain-core/src/config.rs:158`: der Repo-Root-Test verlangte ein vorhandenes Installationsverzeichnis. Behoben in `a46ad05`, der Test prüft den konfigurierten Pfad ohne Checkout-Dateien vorauszusetzen.

### Validierung

- Population-API-Tests: 3 passed, 0 failed, 0 ignored.
- Brain-Konfigurationstests: 16 passed, 0 failed, 0 ignored.
- Wiki-Publikationstests: 5 passed, 0 failed, 0 ignored.
- Retrieval-Tests: 45 passed, 0 failed, 15 ignored.
- `git diff --check` ist sauber. `rustfmt --check` meldet nur bereits vorhandene Formatabweichungen außerhalb der neuen Zeilen.

# Review Runde 5

Review-Gate: `gpt-6.1-sol`
Basis: `origin/main` (`39710e3282c830ee9b47e90945deb71d1db44724`)
Head: `b9ecfbe6c6596970679a974c05dcf3e18a4da1ba`
Urteil: `BLOCK`

## Befunde

1. **BLOCKING**, `rust/crates/deadlock-brain/src/wiki_refresh.rs`: `wiki rebuild --dir` ließ relative Pfade als Symlink-Ziel stehen. Maßnahme: Publication-Root vor Veröffentlichung absolut machen.
2. **NIT**, `rust/crates/dbrain-builds/src/engine.rs`: `[builds]`-Schwellenwerte wurden nicht an Runtime-Verbraucher weitergegeben. Maßnahme: TOML-Schwellen explizit an Builds-Engine und Reasoner-Meta-Index anbinden.

## Nacharbeit und Validierung

- Publication-Root wird vor dem Lock und Cutover absolutisiert und canonicalisiert. Test `relative_rebuild_root_becomes_absolute_before_publication` besteht.
- `BuildSampleGates` übergibt TOML-Werte an die Builds-Engine; alle drei Reasoner-CLI-Pfade übernehmen dieselben Werte für den Meta-Index.
- `cargo test --manifest-path rust/Cargo.toml --locked -p dbrain-builds --lib -- --include-ignored`: 13 passed, 0 failed, 0 ignored.
- `cargo test --manifest-path rust/Cargo.toml --locked -p deadlock-brain --bin deadlock-brain wiki_refresh::tests -- --include-ignored`: 6 passed, 0 failed, 0 ignored, 50 filtered.
- Für Cargo wurden vier fehlende externe Pfade vorübergehend auf gepinnte Abhängigkeiten im isolierten Layout gesetzt. Die Manifeständerungen sind zurückgenommen.
- Erneutes Gate auf dem finalen Freeze-SHA steht aus.
