status: erledigt
Datum: 2026-10-03

# Begrenztes lokales Entpacken

Rust-Hilfswerkzeug: `/tmp/brain-replay-r-20261003/unpack-rust/target/debug/replay-unpack`. Es verwendet bestehende Zstd-/Bzip2-Bibliotheken, erkennt die Kompression an der Signatur, streamt begrenzt und entfernt eigene Teilausgaben bei Fehlern. Es überschreibt keine bestehende Ausgabedatei.

Die echte öffentliche Datei wurde erfolgreich entpackt: `/tmp/brain-replay-r-20261003/unpack-rust/raw/public.dem`, Header `PBDEMS2\0`, 70.861.451 Bytes, SHA-256 `0cc5982dcfac2da1396308bf1e259d9d76fa0e2c46d5fd2a6548a26b1568056a`. Datei 0600, Verzeichnis 0700. Gepinnter privater ReplayRequest daneben als `request.json`, ohne Geheimnisse oder öffentliche Weitergabefreigabe.

## Prüfung

Offline-Debugbuild, Formatprüfung und Clippy `-D warnings` bestanden. Beide Host-Locks wurden gehalten, Compilerprobe vor dem Start frisch, höchstens zwei Jobs. Tests prüfen unter anderem Codec-Erkennung, EOF/Integrität, Größenlimits, vorhandene Ausgaben und Fehlerbereinigung.

TESTNACHWEIS[TW-1]: 17 passed, 0 ignored | Baseline: 0 rot

Gemeldeter Befehl: `CARGO_TARGET_DIR=/tmp/brain-replay-r-20261003/unpack-rust/target /home/nathanael/.cargo/bin/cargo test --offline --jobs 2 --manifest-path /tmp/brain-replay-r-20261003/unpack-rust/Cargo.toml -- --include-ignored`.

Nachweis des nativen Workers: `/home/nathanael/.claude/projects/-home-nathanael--worktrees-brain-fertig-r/02a0a5a9-b45a-482c-b737-af9a6491eeac/subagents/workflows/wf_c3e8bda8-da3/journal.jsonl`. Der Worker änderte keine Repositorydateien und führte keine Downloads, DB-Zugriffe, Git-Aktionen oder Dienständerungen durch.

Dieser Nachweis gilt für Transport und Entpacken. Er beweist weder die Replay-Decoder-Suite noch die Feldsemantik, den Brain-Store oder eine produktive Abfrage. Die DEM und die komprimierte Datei werden nach dem Decode-/Idempotenznachweis gelöscht.
