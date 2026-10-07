status: belegt
Datum: 2026-10-03

# Replay-Prüfung vor der Stempelkorrektur

Dieser Nachweis gilt ausschließlich für den erhaltenen Import-/Validierungsstand vor der anschließenden Korrektur des protobuf-Dateistempels. HEAD-Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`, eigene uncommittierte Änderungen. Kein Baseline-Vergleich, kein echter Replay-Erfolg.

## Tatsächlich ausgeführte Befehle

Beide Sperren wurden in der vorgeschriebenen Reihenfolge gehalten. Vor Compilerstart lief die NonZombie-Probe mit ausschließlich der ausdrücklich erlaubten engen Metadata-Ausnahme. Höchstens zwei Jobs. Der vollständige Prüfschritt `b3xfso49t` endete mit Exit 0.

```sh
SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo fmt --manifest-path /home/nathanael/.worktrees/brain-fertig-r/rust/crates/dbrain-replay/Cargo.toml --package dbrain-replay --check
SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo clippy --manifest-path /home/nathanael/.worktrees/brain-fertig-r/rust/crates/dbrain-replay/Cargo.toml --locked --offline --all-targets -j 2 -- -D warnings
SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --manifest-path /home/nathanael/.worktrees/brain-fertig-r/rust/crates/dbrain-replay/Cargo.toml --locked --offline -j 2
SQLX_OFFLINE=true REPLAY_TEST_SOCKET=/tmp/brain-replay-r-core-20261003/pg REPLAY_TEST_PORT=55439 REPLAY_TEST_DATABASE=brain_replay_test /home/nathanael/.cargo/bin/cargo test --manifest-path /home/nathanael/.worktrees/brain-fertig-r/rust/crates/dbrain-replay/Cargo.toml --locked --offline -j 2 --test import -- --ignored
```

## Ergebnisse

- Formatprüfung und Clippy bestanden.
- Reguläre Suite: 70 passed, 0 failed, 2 ignored, 0 filtered out.
- Zusätzlich ausgewählter echter Postgres-Test: 1 passed, 0 failed, 0 ignored, 3 filtered out. Laufzeit 27,11 Sekunden, nicht bloß ein übersprungener Datenbanktest.
- Das verbleibende ignored ist `supervisor::tests::sandbox_probe`, eine von vier Sandbox-/Watchdogtests direkt gestartete Subprozessfixture.

TESTNACHWEIS[TW-1]: 71 passed, 1 ignored | Baseline: nicht erhoben rot

Private Logs `/tmp/brain-replay-r-core-checks-20261003/fmt.log`, `clippy.log`, `tests.log`, `postgres.log`. Abschlussausgabe `/tmp/claude-1000/-home-nathanael--worktrees-brain-fertig-r/02a0a5a9-b45a-482c-b737-af9a6491eeac/tasks/b3xfso49t.output` mit tatsächlich bestätigtem Exit 0.

## Anschließender echter Decode

```sh
/home/nathanael/.worktrees/brain-fertig-r/rust/crates/dbrain-replay/target/debug/dbrain-replay-worker decode /tmp/brain-replay-r-20261003/unpack-rust/raw/public.dem /tmp/brain-replay-r-20261003/unpack-rust/raw/request.json
```

Ausgabe wurde ausschließlich in private Dateien umgeleitet. Exit 2, generischer Fehler `replay_quarantined:InvalidContainer`, kein Report. Lokale begrenzte Headerprüfung: Container-Magic korrekt, erster Befehl 1, Tick 4294967295, Headerpayload 197 Bytes; protobuf-Stempel genau `PBDEMS2` plus ein NUL, 8 Bytes. Der Download wird nicht als beschädigt eingestuft. Der vorhandene Adapter akzeptierte nur die Variante ohne NUL.

Die aktuelle eng begrenzte Korrektur und ihre neuen Tests sind hiervon getrennt zu prüfen. Eine vorläufige Workerabgabe oder das Warten auf Sperren ist kein Prüfabschluss.
