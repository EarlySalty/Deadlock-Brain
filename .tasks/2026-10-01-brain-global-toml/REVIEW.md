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
