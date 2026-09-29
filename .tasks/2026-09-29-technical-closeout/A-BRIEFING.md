status: aktiv
Datum: 2026-09-29

# Paket A: Match und Meta/Population produktionsreif verdrahten

Lies den gemeinsamen verbindlichen Auftrag unter /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/AUFTRAG.md, besonders Punkte 1 und 2 und sämtliche Grenzen. Du bist Sol, einziger Thread für A, keine Unter-Threads/Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a; Bump-up genau wie dort.

Worktree: /home/nathanael/.worktrees/brain-pre-g5-finalize-20260929
Branch: integration/pre-g5-finalize-20260929
HEAD: 305df2d36ec7b5d0513d6c0769051b41538d6a1b
Dirty: rust/Cargo.lock, rust/crates/brain-feeds/src/lib.rs, rust/crates/dbrain-sources/Cargo.toml, rust/crates/dbrain-sources/src/lib.rs. Neue Dateien: brain-feeds/src/deadlock_match.rs und dbrain-sources/src/analytics_runtime.rs. rust/graphify-out untracked ist Suchartefakt und gehört nicht in den Commit. Den gesamten bestehenden Fachstand übernehmen, nicht resetten/verwerfen.

Referenz aus Nutzerauftrag, wörtlich:
„Deadlock API → validierter Match Response → SourceRecordV2 → normaler DocumentStorePort → CorpusRelease → brain-serve“
„Meta und Population sind bounded Runtime Sources.“
„KEIN neues ClickHouse. KEINE lokale Analytics-Kopie nur aus Bequemlichkeit. KEIN DEADLOCK_CENTRAL_DSN.“

Exklusiver Scope: beide bestehenden Source-Adapter, Exports, erforderliche Core/Runtime/CLI-Anbindung und gezielte Tests, Dependency-/Lockfileänderungen. Keine Test-Harness-Skripte und keine große Migrationsstatusdoku bearbeiten, Paket C/F übernehmen diese. Keine unrelated Refactors/Formatierungsdiffs. Falls dieselbe Datei wie ein notwendiger Harness betroffen ist, erst genaue Pfade im Bericht melden statt fremde Arbeit anfassen.

Nicht bloß unaufgerufene Adapter bauen. Beweise den normalen Store-/Release-Weg und die Runtime-Nutzung der Analytics-Observation. Match-ID und Account-Bindung/ACL, Hash-/Schema-Pins/Locator/Zeit/Revision/Checkpoint/Tombstone/Idempotenz/revoke/release vollständig. Meta/Population mit Production-Allowlist, Loopback nur Tests, Deadline/Timeout/Retry/Byte/Row/Window-Bounds, Hero/Entity/Patch/Schema-Filter und Provenienz. Kein Live-Provider oder zusätzliche Freigabe erfinden. Tests mit Fixtures und isoliertem PostgreSQL ohne Passwort-ENV.

Vollständige Pflicht-Testfälle aus AUFTRAG.md 1/2 abdecken. Relevante fmt/check/clippy/tests mit --locked --offline. Keine ignorierten Tests als grün melden. Keine produktiven Dienste anfassen. Pro Cargo-Lauf begrenzte Jobs und vorhandene Build-Sperren nutzen; kein Release-Build parallel erzwingen.

Commit und Push ausschließlich auf eigenem Branch ausdrücklich erlaubt. PR nach migration/rust-integration anlegen und in T3 verlinken, niemals selbst mergen. Unabhängiges Review übernimmt Orchestrator. Selbstprüfung mit vorhandenem gate_hook.py --review vor Abgabe, Gates nicht umgehen. Paketbericht unter .tasks/2026-09-29-technical-closeout/A-REPORT.md im eigenen Worktree, konkrete SHA/Befehle/Exitcodes/Verhaltensnachweise und Restpunkte. Keine Code-Kommentare. Fertigmeldung mit PR-URL und Diffstat.
