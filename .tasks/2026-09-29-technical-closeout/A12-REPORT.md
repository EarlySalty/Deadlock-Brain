status: erledigt
Datum: 2026-09-29

# A12 Bericht: Match-Vertragsfixes

- Intent: 562a877b-0939-440a-964d-1145d9e9431a
- Worker-Thread: afa412f2-0c21-4c4c-9bd7-dba7550239bc
- Worktree: `/home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929`
- Branch: `fix/pre-g5-match-contract-20260929`
- Basis: `34a2507`
- Fixcommit: `8e8ce76`

## Änderungen

1. Match-Metadaten fragen `include_player_kda=true` an. Derselbe Parameter ist im Locator-Vertrag und in den positiven Fixtures enthalten. Die Upstream-Implementierung am gepinnten Stand setzt boolesche Auswahlparameter standardmäßig auf `false`. `include_player_kda` erzeugt die sechs Basisfelder sowie `kills`, `deaths` und `assists`. Beleg: [bulk_metadata.rs am Upstream-Commit](https://github.com/deadlock-api/deadlock-api/blob/290cedba6cca7d8a07015e9feefecd22de27643c/api/src/routes/v1/matches/bulk_metadata.rs#L95-L125).
2. Match-Inhalt wird vor `CoreDocument` und Batch gegen die angeforderte Projektion geprüft. Unbekannte Felder, ungültige Typen, fehlende Identitäten, abweichende IDs, mehrere Spieler und ungültige Aliase werden quarantänisiert. Gespeichert wird die neu aufgebaute, typisierte Projektion. Raw-Hash, Schema-Provenienz, Idempotenz und Revoke bleiben erhalten.
3. Geänderte Dateien: `rust/crates/brain-feeds/src/deadlock_match.rs`, `rust/crates/brain-feeds/tests/match_store.rs`.

## Prüfung

Ausgeführt im zugewiesenen Worktree. `CARGO_HOME` und `CARGO_TARGET_DIR` zeigten auf lokale Installations- und Buildpfade. `DATABASE_URL` und `PGPASSWORD` waren für Cargo entfernt.

- `CARGO_HOME=/home/nathanael/.cargo /home/nathanael/.cargo/bin/rustfmt --edition 2021 rust/crates/brain-feeds/src/deadlock_match.rs rust/crates/brain-feeds/tests/match_store.rs && /home/nathanael/.cargo/bin/cargo +stable fmt --manifest-path rust/Cargo.toml -p brain-feeds -- --check`: Exit 0.
- `unset DATABASE_URL PGPASSWORD; CARGO_HOME=/home/nathanael/.cargo CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929/rust/target /home/nathanael/.cargo/bin/cargo +stable clippy --manifest-path rust/Cargo.toml --locked --offline -p brain-feeds --all-targets -- -D warnings > /tmp/brain-match-fix.5BA56M/cargo-clippy.log 2>&1`: Exit 0.
- `unset DATABASE_URL PGPASSWORD; BRAIN_MATCH_TEST_PG_SOCKET=/tmp/brain-match-fix.5BA56M/.match-test-pg BRAIN_MATCH_TEST_PG_PORT=55489 CARGO_HOME=/home/nathanael/.cargo CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929/rust/target /home/nathanael/.cargo/bin/cargo +stable test --manifest-path rust/Cargo.toml --locked --offline -p brain-feeds -- --include-ignored > /tmp/brain-match-fix.5BA56M/cargo-test.log 2>&1`: Exit 0. Ergebnis: 19 passed, 0 failed, 0 ignored. Die ignorierte PostgreSQL-Store-, Release- und Revoke-Fixture lief gegen ein Wegwerf-PostgreSQL per Unix-Socket und Peer-Auth. Der Cluster wurde gestoppt. Keine echte Matchabfrage oder Produktion wurde verwendet.
- Vorläufe desselben Testbefehls: Exit 101 wegen eines Compile-Typfehlers und einer veralteten Fixture ohne `hero_id`. Beide Ursachen wurden behoben; der Abschlusslauf ist oben dokumentiert.
- `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929 --base 34a2507` vor Commit: `ALLOW: no reviewable changes`, da der Code noch uncommittet war. Derselbe Befehl nach Commit: `ALLOW: No blocking defect is established by the supplied diff and source snapshots.` Der nicht blockierende Hinweis zu möglichen weiteren API-Feldern wurde durch die gepinnte Upstream-Auswahl und die Projektions-Fixture geprüft.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft
TESTNACHWEIS[TW-1]: 19 passed, 0 ignored | Baseline: nicht erhoben, kein Altfehlerurteil
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: interner Taskbericht im Taskordner
ORCHESTRIERUNG[OR-1]: Stufe mittel | Schritt done | Artefakt: .tasks/2026-09-29-technical-closeout/A12-REPORT.md
INTENT[IA-1]: Stufe mittel | Modell Luna | Thread 562a877b-0939-440a-964d-1145d9e9431a | Register: /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/REGISTER.md
MERGEPROTOKOLL[MS-1]: 5 Git-Schritte einzeln | Anläufe: 2 | Gate: ALLOW nach Code-Commit; kein Merge angefordert
