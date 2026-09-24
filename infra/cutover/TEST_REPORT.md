# S11: Tatsächlich ausgeführte Prüfungen

Datum: 24.09.2026. Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`.
Basis: `30326512568b7370524956839100462ba71bdb92`.
Geprüft: eigener S11-Worktree auf dieser Basis. Die exakten nach dem Commit erneut
geprüften SHAs und GitHub-Läufe werden in der Übergabe und im zugehörigen Draft-PR
festgehalten. Kein Testergebnis einer fremden Session wurde übernommen.

## Ergebnis

| Prüfung | Tatsächlicher Befehl | Ergebnis |
|---|---|---|
| Format | `cargo fmt --manifest-path infra/cutover/runtime-audit/Cargo.toml -- --check` | Exit 0 |
| Lint/Compiler | `cargo clippy --manifest-path infra/cutover/runtime-audit/Cargo.toml --all-targets --locked --offline -- -D warnings` | Exit 0, keine Warnungen |
| Funktionale Tests | `cargo test --manifest-path infra/cutover/runtime-audit/Cargo.toml --locked --offline` | 25 Inventartests + 3 CLI-Tests bestanden, 0 fehlgeschlagen, 0 ignoriert |
| Releasebuild des Inventarwerkzeugs | `cargo build --manifest-path infra/cutover/runtime-audit/Cargo.toml --release --locked --offline` | Exit 0; kein Produktivimage und kein Brain-Releasebuild |
| Echte Metadatenaufnahme | `cargo run --manifest-path infra/cutover/runtime-audit/Cargo.toml --locked --offline -- snapshot` | Exit 0, zwölf Units: sieben Services und fünf Timer |

Die echte Aufnahme bestätigt zwei fehlgeschlagene Services, fünf aktive Timer,
die laufende Site und `dl-knowledge`. Alle sieben Services melden ihre eigenen
Memory-/CPU-/Tasksgrenzen als `infinity`; Grenzen aus Eltern-Slices sind damit
nicht ausgeschlossen. Fünf Entrypoints sind Shellaufrufe, zwei andere ungeprüfte
Entrypoints. Das beweist keine Pythonfreiheit. Der kleine Auditcode meldet immer
`cutover_authorized=false` und `full_runtime_verification=not_performed`.

## Rote Gegenproben

1. Vor der Implementierung lieferte der Library-Stub nur Fehler/leere Ausgabe.
   `cargo test ... --offline`: **15 von 25 Tests rot**, 10 Ablehnungstests schon
   grün, weil der Stub grundsätzlich alles ablehnte. Das wird nicht als
   vollständiger Rotnachweis ausgegeben.
2. Ein leerer CLI-Stub `fn main() {}` ließ mit `cargo test ... --test cli`
   **alle 3 CLI-Tests rot** werden, bevor die CLI implementiert wurde.
3. Eine separate bewusst falsche Library unter dem ignorierten
   `runtime-audit/target/red-control/` akzeptierte fehlerhafte Eingaben und gab
   einen synthetischen Marker ungefiltert aus. Die unveränderten 25 Inventartests
   wurden hineinkopiert. `cargo test --manifest-path
   infra/cutover/runtime-audit/target/red-control/Cargo.toml --offline`:
   **0 bestanden, 25 fehlgeschlagen**, Exit 101.
4. Anschließend am richtigen Quellstand wieder Format, Clippy, alle Tests und
   Releasebuild ausgeführt: **28 bestanden**, Exit 0.

Damit gibt es für jeden neu eingeführten Test eine beobachtete rote Gegenprobe.
Die falsche Library ist kein Sicherheits-Mutationstest jeder einzelnen Codezeile
und keine Writer-/Restoreprobe. Sie wird nicht committed oder ausgeliefert.
Zur Wiederholung eine isolierte Crate gleichen Namens ohne Dependencies anlegen,
`tests/inventory.rs` unverändert kopieren und folgenden Teststub als `src/lib.rs`
verwenden; die echte Implementierung dabei nicht ersetzen:

```rust
pub type AuditResult<T> = Result<T, &'static str>;
#[derive(Debug)]
pub struct Unit;
pub fn discover(_: &str, _: &str) -> AuditResult<Vec<String>> { Ok(Vec::new()) }
pub fn parse_unit(_: &str, _: &str) -> AuditResult<Unit> { Ok(Unit) }
pub fn collect(_: &mut impl FnMut(&[&str]) -> AuditResult<String>) -> AuditResult<Vec<Unit>> {
    Err("NEVER_PRINT_THIS")
}
pub fn render(_: &[Unit]) -> String { "NEVER_PRINT_THIS".to_owned() }
```

## Nicht durchgeführt und nicht behauptet

Produktive Umschaltung, Datenmigration, Restore/Rebuild einer Datenbank,
Writer-Fencing, Provider-/Consumer-E2E, Lastmessung, SLO-Abnahme, komplette
Runtime-/Netzwerk-/Native-/Pythonfreiheitsprüfung, regelmäßiger Learning-/Reparse-
Zyklus, Keyrotation, Abschaltung, Archivierung und Datenlöschung wurden **nicht**
ausgeführt. Keine G4-/G5-/G6-Abnahme. Die Root-Workspace-Komplettsuite wurde lokal
nicht ausgeführt, weil diese Änderungen keine produktiven Crates verändern.

Die 28 Tests decken Parser, Fehlerbehandlung, feste nur lesende Befehlsauswahl,
Redaktion und CLI-Bedienung ab. Fake-Systemd-Antworten sind synthetische
Metadatentests; nur der zusätzliche Snapshot ist eine echte Systemd-Abfrage.
Keine Live-Mutation wird durch die Tests ausgelöst. Prozess-Timeouts und alle
möglichen Systemd-Versionen sind nicht separat als Ausfallmatrix getestet.

Die bestehende Root-CI prüft das neue eigenständige Werkzeug noch nicht mit.
`CR-S11-01.md` weist diese Integrationsaufgabe S02/S10 zu. Ein grünes Root-CI-
Ergebnis darf nicht als automatisch ausgeführte Audit-Testserie angegeben werden.
