# S07 · Tatsächlich ausgeführte Prüfungen

Stand 24.09.2026. Getesteter Rust-/Lockfile-Stand:
`c00fc8935048bf490c1e4790f7c6195864ad49e2`.
Die S07-Änderung ergänzt ausschließlich Dokumentation und synthetische Daten.
Kein Rust-Code, Manifest oder Lockfile wurde verändert. Der finale PR-Kopf,
PR-Link und die aktuellen GitHub-Runs werden im PR mit vollständigem SHA
festgehalten; ein älterer grüner Lauf gilt nicht für einen neuen Kopf.

## Baseline im eigenen Feature-Worktree

Werkzeug: Cargo 1.97.1, bestehender Workspace, `--locked --offline`, zwei
Buildjobs. Die Commands wurden aus dem Repository-Root ausgeführt. Keine
Produktiv-DB, Provider-Credentials oder externen Modellaufrufe für diese Tests.

| Prüfung | Befehl | Tatsächliches Ergebnis |
|---|---|---|
| Compiler und Core-Unit-Tests | `cargo test --manifest-path rust/Cargo.toml -p deadlock-brain-core --lib --locked --offline -j 2` | Exit 0; **17 bestanden, 0 fehlgeschlagen, 0 ignoriert**, darunter vorhandene Loopback-HTTP-Tests |
| Linter | `cargo clippy --manifest-path rust/Cargo.toml -p deadlock-brain-core --all-targets --locked --offline -j 2 -- -D warnings` | Exit 0 |
| Bestehende Coreformatierung | `cargo fmt --manifest-path rust/Cargo.toml -p deadlock-brain-core -- --check` | **Exit 1**, vorhandene Abweichungen in `build_narration.rs`, `config.rs`, `http.rs`, `pg.rs`, `pg_secrets.rs` |

Die Baseline ist damit **nicht vollständig grün**. Die Formatierungsabweichungen
entstehen bereits ohne S07-Dateien und bleiben unverändert. Sie sind kein
Anlass, entgegen Pfadgrenzen oder Gates fremden Code umzuformatieren. Owner 02
beziehungsweise 00 entscheidet über deren gesonderte Behebung. Compiler- und
Lintererfolg ersetzen die fehlende Formaterfüllung nicht.

## Prüfung der neu erstellten Daten

Die einmalige Offlineprüfung mit Python-Standardbibliothek kontrollierte
SHA-256, UTF-8-/JSON-Lesbarkeit mit expliziter Ablehnung doppelter Schlüssel
und nicht standardkonformer Zahlen, Requestreferenzen und deaktivierte
Freigaben. Sie implementiert keinen produktiven Providerport und wurde nicht
als Laufzeitabhängigkeit hinzugefügt. Das Planpaket erlaubt dokumentierte
optionale Einmalwerkzeuge; kein Python-Paket oder Dienst wird ausgeliefert.

| Dateneigenschaft | Ergebnis |
|---|---|
| Manifestgebundene Fixture-Dateien | 48/48 SHA-256 stimmen; keine doppelte Dateireferenz |
| Syntaktisch gültige JSON-Fixtures | 45/45 parsebar; darunter absichtlich semantisch falsche Antworten |
| Roh-Negativfälle | 3/3 erwartungsgemäß abgelehnt: abgeschnittenes JSON, NaN, doppelte Question-ID |
| Integritätsgegenproben | 2/2 entdeckt: zusätzliches Byte verändert Hash; erfundene Dateireferenz existiert nicht. Nur In-Memory-Kopien, keine beschädigten Repo-Dateien |
| Geplante Transport-/Semantikfälle | 31 ausdrücklich `not_run` in `FAULTMATRIX.csv` |
| Aktivierungsprotokoll | 5/5 `off`; keine Schwelle, Labels oder Messwerte erfunden |

Integrität wurde auch im Fixture-Verzeichnis mit
`sha256sum --check SHA256SUMS` geprüft: Exit 0. Die Schema-Erwartungen im
Manifest wurden **nicht** gegen einen Rust-Provider-Validator ausgeführt.
Insbesondere beweisen 45 parsebare Dateien nicht, dass der bestehende Client
die semantischen Negativfälle ablehnt. Es gibt keine neuen Rusttests und
keinen behaupteten Rot/Grün-Nachweis für eine noch nicht implementierte Funktion.

## Nicht ausgeführt und nicht als Erfolg gewertet

Neue Rust-Fake-Server-/Faulttests, echte Jev-/Fireworks-/Embeddinganfragen,
Resolververfügbarkeitsprüfung, Performance-/Lastmessung, Labels-/Holdout-
auswertung, Shadowbetrieb, Berechtigungs-/Egressprüfung einer produktiven
Integration und G4-Aktivierung. Keine globale Workspace- oder Consumerparität.

GitHub-Actions werden nach Erstellung des offenen PR anhand des aktuellen
Head-SHA geprüft. Nicht vorhandene, wartende oder fehlgeschlagene Checks sind
keine bestandene CI. Ein erfolgreicher Dokumentationslauf würde weder G1
schließen noch die S07-Implementierung oder ein Deployment freigeben.
