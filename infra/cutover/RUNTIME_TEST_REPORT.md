# S11: Testbericht zur Prozessprüfung

Datum: 24.09.2026. Fortsetzung von PR #26.
Ausgangs-Head: `5b11aa5e77fa0a513f5ea3b68b41227da6a7129e`.
Integrierte Basis: `30326512568b7370524956839100462ba71bdb92`.
Der vollständige neue Commit und die Wiederholung nach Commit werden im
PR-Prüfkommentar zusammen mit dem aktuellen GitHub-Lauf dokumentiert.
Dieser Bericht übernimmt keine Freigabe eines alten Heads.

## Implementierter Umfang

Der bestehende dependencyfreie S11-Metadatenleser erhält den Befehl `runtime`.
Systemd-Servicegruppen und ihre Untergruppen werden read-only über cgroup v1
bzw. v2 erfasst; procfs liefert Prozessidentität, ausführbare Datei und geladene
Datei-Mappings. Die Tests prüfen Klassifikation, Datenminimierung, Grenzen,
Race-Abbruch und fehlende Metadaten. Gemeinsame Contracts, produktive Crates,
Root-CI, DB-Schema und Runtimekonfiguration bleiben unverändert.

## Tatsächlich ausgeführte Prüfungen

Alle Befehle laufen im S11-Worktree. Das Auditmanifest liegt unter
`infra/cutover/runtime-audit/Cargo.toml`.

| Prüfung | Befehl | Ergebnis |
|---|---|---|
| Ausgangsstand | `cargo test --manifest-path infra/cutover/runtime-audit/Cargo.toml --locked --offline` vor Änderungen | 28 bestanden, 0 fehlgeschlagen, 0 ignoriert |
| Formatierung | `cargo fmt --manifest-path infra/cutover/runtime-audit/Cargo.toml -- --check` | Nach Formatierung bestanden; ein vorheriger Lauf fand ausschließlich Formatabweichungen in zwei neuen CLI-Tests, diese wurden behoben |
| Lint/Compiler | `cargo clippy --manifest-path infra/cutover/runtime-audit/Cargo.toml --all-targets --locked --offline -- -D warnings` | Bestanden, keine Warnungen |
| Vollständige Audit-Testserie | `cargo test --manifest-path infra/cutover/runtime-audit/Cargo.toml --locked --offline` | 54 bestanden: 25 bestehende Inventartests, 5 CLI-Tests und 24 neue Prozess-/Mounttests; 0 fehlgeschlagen, 0 ignoriert |
| Angegebene Mindestversion | `CARGO_TARGET_DIR=infra/cutover/runtime-audit/target/msrv RUSTC=/usr/bin/rustc /usr/bin/cargo test --manifest-path infra/cutover/runtime-audit/Cargo.toml --locked --offline --quiet` | Dieselben 54 Tests mit rustc 1.75.0 bestanden, 0 ignoriert |
| Aktuelle lokale Toolchain | `/home/nathanael/.cargo/bin/rustc --version` | rustc 1.97.1; Format, Clippy, Tests und Releasebuild damit ausgeführt |
| Audit-Releasebuild | `cargo build --manifest-path infra/cutover/runtime-audit/Cargo.toml --release --locked --offline` | Bestanden; nur das Prüfwerkzeug, kein Brain-Releaseimage |
| Echte Prozessaufnahme | `cargo run --manifest-path infra/cutover/runtime-audit/Cargo.toml --release --locked --offline -- runtime` | Exit 0 nach expliziter v1-Ergänzung; zwölf Units, zwei beobachtete Serviceprozesse, fünf Services ohne laufende Prozessbeobachtung |

Die erste neue Live-Aufnahme scheiterte mit Exit 1 statt eines Teilberichts:
Der Host besitzt kein vereinheitlichtes v2-Layout, sondern cgroup v1 mit
`name=systemd`. Danach wurden Mount-Erkennung und exakte benannte
Hierarchieprüfung ergänzt, erneut getestet und tatsächlich ausgeführt.
Die Prozessaufnahme findet ein gelöschtes laufendes Site-Binary und gelöschte
Datei-Mappings in beiden laufenden Services. Genaue Einordnung: RUNTIME_CHECK.md.

## Beobachtete rote Gegenproben

1. Vor Implementierung gab der neue Collector nur einen Fehler zurück.
   **7 von zunächst 19 Prozessprüfungen waren rot**, 12 Ablehnungstests bereits
   grün. Ein grundsätzlich ablehnender Stub ist kein Beleg aller Fehlerzweige.
2. Vor Einbindung des neuen CLI-Befehls waren **beide neuen CLI-Tests rot**:
   `runtime` wurde als Bedienfehler abgewiesen und die Hilfe enthielt den neuen
   Momentaufnahmevertrag nicht. Nach Einbindung bestanden alle fünf CLI-Tests.
3. Der zunächst immer auf v2 festgelegte Mountdetektor ließ **2 von 22 damaligen
   Prozess-/Mounttests scheitern**: echter v1-Mount und Ablehnung unbekannter,
   eingeschränkter oder mehrdeutiger Mounts.
4. Zusätzlich wurde unter dem ignorierten
   `runtime-audit/target/runtime-counterexample/` eine separate Crate mit
   unverändert kopierten 24 Prozess-/Mounttests angelegt. Ihr bewusst defekter
   Collector akzeptiert jede Eingabe und liefert einen leeren Bericht; ihr
   Mountdetektor lehnt pauschal alles ab. Ergebnis: **23 rot, 1 grün**, Exit 101.
   Der verbleibende Ablehnungstest für ungültige Mounts war in Gegenprobe 3 rot.
   Damit wurde jeder der 24 neuen Prozess-/Mounttests mindestens einmal rot
   beobachtet. Die zwei neuen CLI-Tests haben Gegenprobe 2.

Die defekte Kopie ist nicht im Produktcode und wird nicht committed oder
veröffentlicht. Das sind gezielte Implementierungsgegenproben, keine exhaustive
Mutationstestabdeckung, kein Wirksamkeitsmaß eines KI-Reviews und keine echte
Writer-/Restore-Ausfallprobe.

Wiederholung der separaten Gegenprobe: Cargo.toml, Cargo.lock, src/lib.rs,
src/main.rs und tests/runtime.rs in ein ignoriertes Testverzeichnis kopieren.
Nur dort src/runtime.rs durch folgende vereinfachte Implementierung ersetzen;
anschließend `cargo test --manifest-path <Kopie>/Cargo.toml --locked --offline
--test runtime` ausführen. Nie den echten Quellstand für die Gegenprobe ersetzen.

```rust
use crate::{AuditResult, Unit};
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CgroupLayout { #[default] UnifiedV2, SystemdV1 }
pub trait RuntimeReader {
    fn layout(&mut self) -> AuditResult<CgroupLayout> { Ok(CgroupLayout::UnifiedV2) }
    fn text(&mut self, path: &str) -> AuditResult<String>;
    fn link(&mut self, path: &str) -> AuditResult<String>;
    fn directories(&mut self, path: &str) -> AuditResult<Vec<String>>;
}
pub struct LinuxReader;
impl RuntimeReader for LinuxReader {
    fn text(&mut self, _: &str) -> AuditResult<String> { Err("unavailable") }
    fn link(&mut self, _: &str) -> AuditResult<String> { Err("unavailable") }
    fn directories(&mut self, _: &str) -> AuditResult<Vec<String>> { Err("unavailable") }
}
#[derive(Debug)]
pub struct RuntimeReport;
pub fn detect_layout(_: &str) -> AuditResult<CgroupLayout> { Err("unavailable") }
pub fn collect_runtime(_: &[Unit], _: &mut impl RuntimeReader) -> AuditResult<RuntimeReport> {
    Ok(RuntimeReport)
}
pub fn render_runtime(_: &RuntimeReport) -> String { String::new() }
```

## Abgrenzung zur CI und zum Gesamtauftrag

Die vorhandene Root-CI führt die eigenständige Audit-Testserie noch nicht aus.
Ihr aktueller Head-/Merge-SHA und ihre tatsächlichen Ergebnisse werden getrennt
im PR-Kommentar dokumentiert. Ein grüner Root-Lauf bedeutet nicht, dass dort
diese 54 Tests liefen. Die angeforderte Einbindung durch S02/S10 bleibt im CR.

Keine vollständige lokale Root-Workspace-Suite, kein Produktionsimage, kein
Restore/Rebuild einer Datenbank, kein Writer-Fencing, keine Neuwrite-Rückführung,
kein Consumer-/Provider-E2E, kein Last-/Readinessnachweis und kein vollständiger
regulärer Ingest-/Learning-/Reparsezyklus wurden in dieser Fortsetzung ausgeführt.
Keine Dienste oder Timer gestartet/gestoppt, keine Secrets geändert, keine
Archive angelegt und keine Daten gelöscht. Die G0–G6-Zustände werden nicht durch
S11 überschrieben. Vollständiger S11-Abschluss bleibt von den integrierten
Verträgen, echten Betriebsgates und ausdrücklicher Cutoverfreigabe abhängig.
