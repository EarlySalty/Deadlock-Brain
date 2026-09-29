status: erledigt
Datum: 2026-09-29

# A12 Bericht: Match-Vertragsfixes

- Intent: 562a877b-0939-440a-964d-1145d9e9431a
- Worker-Thread: afa412f2-0c21-4c4c-9bd7-dba7550239bc
- Worktree: `/home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929`
- Branch: `fix/pre-g5-match-contract-20260929`
- Basis: `34a2507`
- Codecommits R1/R2: `8e8ce76`, `7023076`, `cd91de5`

## Änderungen

1. R1 fragte Match-Metadaten mit `include_player_kda=true` an. Dieser Schalter ist upstream standardmäßig `false` und selektiert die sechs Basisfelder sowie `kills`, `deaths` und `assists`. Die unabhängige R2-Nachprüfung fand zusätzlich `include_info=true` als Upstream-Default; R2 wählt es explizit ab. Beleg: [bulk_metadata.rs am Upstream-Commit](https://github.com/deadlock-api/deadlock-api/blob/290cedba6cca7d8a07015e9feefecd22de27643c/api/src/routes/v1/matches/bulk_metadata.rs#L85-L117).
2. Match-Inhalt wird vor `CoreDocument` und Batch gegen die angeforderte Projektion geprüft. Unbekannte Felder, ungültige Typen, fehlende Identitäten, abweichende IDs, mehrere Spieler und ungültige Aliase werden quarantänisiert. Gespeichert wird die neu aufgebaute, typisierte Projektion. Raw-Hash, Schema-Provenienz, Idempotenz und Revoke bleiben erhalten.
3. Geänderte Dateien: `rust/crates/brain-feeds/src/deadlock_match.rs`, `rust/crates/brain-feeds/tests/match_store.rs`.

## Prüfung

Ausgeführt im zugewiesenen Worktree. `CARGO_HOME` und `CARGO_TARGET_DIR` zeigten auf lokale Installations- und Buildpfade. `DATABASE_URL` und `PGPASSWORD` waren für Cargo entfernt.

- `CARGO_HOME=/home/nathanael/.cargo /home/nathanael/.cargo/bin/rustfmt --edition 2021 rust/crates/brain-feeds/src/deadlock_match.rs rust/crates/brain-feeds/tests/match_store.rs && /home/nathanael/.cargo/bin/cargo +stable fmt --manifest-path rust/Cargo.toml -p brain-feeds -- --check`: Exit 0.
- `unset DATABASE_URL PGPASSWORD; CARGO_HOME=/home/nathanael/.cargo CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929/rust/target /home/nathanael/.cargo/bin/cargo +stable clippy --manifest-path rust/Cargo.toml --locked --offline -p brain-feeds --all-targets -- -D warnings > /tmp/brain-match-fix.5BA56M/cargo-clippy.log 2>&1`: Exit 0.
- `unset DATABASE_URL PGPASSWORD; BRAIN_MATCH_TEST_PG_SOCKET=/tmp/brain-match-fix.5BA56M/.match-test-pg BRAIN_MATCH_TEST_PG_PORT=55489 CARGO_HOME=/home/nathanael/.cargo CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929/rust/target /home/nathanael/.cargo/bin/cargo +stable test --manifest-path rust/Cargo.toml --locked --offline -p brain-feeds -- --include-ignored > /tmp/brain-match-fix.5BA56M/cargo-test.log 2>&1`: Exit 0. Ergebnis: 19 passed, 0 failed, 0 ignored. Die ignorierte PostgreSQL-Store-, Release- und Revoke-Fixture lief gegen ein Wegwerf-PostgreSQL per Unix-Socket und Peer-Auth. Der Cluster wurde gestoppt. Keine echte Matchabfrage oder Produktion wurde verwendet.
- Vorläufe desselben Testbefehls: Exit 101 wegen eines Compile-Typfehlers und einer veralteten Fixture ohne `hero_id`. Beide Ursachen wurden behoben; der Abschlusslauf ist oben dokumentiert.
- `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929 --base 34a2507` vor Code-Commit: `ALLOW: no reviewable changes`, da der Code noch uncommittet war. Nach Code-Commit: `ALLOW: No blocking defect is established by the supplied diff and source snapshots.` Der nicht blockierende Hinweis zu möglichen weiteren API-Feldern wurde durch die gepinnte Upstream-Auswahl und die Projektions-Fixture geprüft. Nach Bericht-Commit: `ALLOW: The diff shows no established merge-blocking defect.`

## Nachprüfung Runde 2

1. Upstream-Abgleich am Pin `290cedba6cca7d8a07015e9feefecd22de27643c`, Datei `api/src/routes/v1/matches/bulk_metadata.rs`: `include_info` ist das einzige Auswahlflag mit Default `true`; es wird nun explizit auf `false` gesetzt. Alle übrigen aktiven Auswahlflags sind explizit: `include_more_info`, `include_objectives`, `include_mid_boss`, `include_player_info`, `include_player_items`, `include_player_stats`, `include_player_final_stats` und `include_player_death_details` auf `false`, `include_player_kda` auf `true`. Nicht gesetzte `extra_match_columns` und `extra_player_columns` haben den Default `None`. Ohne Match-Infoflags wählt Upstream immer `match_id`; `include_player_kda=true` selektiert im `players`-Array sechs Basisfelder sowie `kills`, `deaths` und `assists`. Die Antwort kann optionale Nullfelder auslassen. Der Validator verlangt `account_id` und `hero_id`, prüft Typen vorhandener Werte und speichert nur erlaubte Felder. Weitere Defaultfilter ändern die Treffermenge, nicht die Antwortprojektion.
2. URL-Erzeuger und Locatorvalidierung teilen dieselbe vollständige Parameter-Map. Der Vertrags-Test prüft sie exakt. Die Default-Regression enthält jetzt sämtliche zehn vom Upstream bei `include_info=true` gewählten Matchfelder und bestätigt, dass diese Antwort quarantänisiert wird. Die positive Fixture enthält die neun selektierten Spielerfelder. Der Validator verlangt `account_id` und `hero_id`; weitere Spielerfelder dürfen fehlen oder null sein. Unbekannte Felder werden nicht zugelassen.
3. Die gespeicherte, typisierte Projektion nutzt gegenüber R1 dieselbe erlaubte Feldmenge. Der Gate-Hinweis zur Parserrevision wurde am Ingestionpfad geprüft: die bestehende Revision bleibt passend, da akzeptierte Dokumente weiterhin nur aus `match_id`, `players` und den vorhandenen erlaubten Spielerfeldern bestehen. R1-Antworten mit den nun explizit abgewählten Infofeldern wurden vor Speicherung abgewiesen.
4. R2-Codecommits: `7023076` (`fix(brain-feeds): pin full match metadata projection`) und `cd91de5` (`test(brain-feeds): isolate match locator regressions`). Geändert wurden nur `rust/crates/brain-feeds/src/deadlock_match.rs` und `rust/crates/brain-feeds/tests/match_store.rs`.

### R2-Prüfung

- Upstream-Quelle am obigen Pin gelesen. Selektionsflags, Defaults, `player_columns`, Match-SELECT und JSON-Ausgabe gegen Querymap, Locator, Validator und Fixtures abgeglichen. Die Zwillingssuche (`rg`) fand den produktiven URL-Aufruf ausschließlich in `rust/crates/brain-feeds/src/bin/brain-match-ingest.rs:144`; dort wird die erstellte URL unverändert an den begrenzten HTTP-Client übergeben und die Antwort an den strikten Batchvalidator.
- `CARGO_HOME=/home/nathanael/.cargo /home/nathanael/.cargo/bin/rustfmt --edition 2021 rust/crates/brain-feeds/src/deadlock_match.rs rust/crates/brain-feeds/tests/match_store.rs && /home/nathanael/.cargo/bin/cargo +stable fmt --manifest-path rust/Cargo.toml -p brain-feeds -- --check`: Exit 0.
- `unset DATABASE_URL PGPASSWORD; CARGO_HOME=/home/nathanael/.cargo CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929/rust/target /home/nathanael/.cargo/bin/cargo +stable clippy --manifest-path rust/Cargo.toml --locked --offline -p brain-feeds --all-targets -- -D warnings > /tmp/brain-match-r2.QBCGMY/cargo-clippy-final.log 2>&1`: Exit 0, 0 Clippy-Warnungen.
- `unset DATABASE_URL PGPASSWORD; BRAIN_MATCH_TEST_PG_SOCKET=/tmp/brain-match-r2.QBCGMY/.match-test-pg BRAIN_MATCH_TEST_PG_PORT=55493 CARGO_HOME=/home/nathanael/.cargo CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929/rust/target /home/nathanael/.cargo/bin/cargo +stable test --manifest-path rust/Cargo.toml --locked --offline -p brain-feeds -- --include-ignored > /tmp/brain-match-r2.QBCGMY/cargo-test-final.log 2>&1`: Exit 0, 21 passed, 0 failed, 0 ignored, 0 filtered. Die PostgreSQL-Store-, Release-, Replay- und Revoke-Fixture lief gegen ein Wegwerf-PostgreSQL per Unix-Socket und Peer-Auth. Der Cluster wurde gestoppt. Keine echte Matchabfrage oder Produktion wurde verwendet.
- `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929 --base 34a2507` vor Commit: `ALLOW: No merge-blocking defect is established by the supplied diff.` mit einem nicht blockierenden Hinweis zur Parserrevision. Nach Codecommit: `ALLOW: The supplied diff and source snapshots establish no merge-blocking defect.` Nach Berichtcommit `ec28a72`: `ALLOW: No merge-blocking defect is established by the supplied diff.` mit Hinweis, die neun ausgewählten Spielerfelder nicht als durchgehend verpflichtend darzustellen. Die Formulierung wurde korrigiert; nach `ffbcdc9`: `ALLOW: No merge-blocking defect is established by the supplied diff and source snapshots.` Nach `e4f054f` meldete das Gate einen nicht blockierenden Befund, dass Locator-Negativtests ungültige Antwortkörper nutzten. Die Regressionen verwenden jetzt gültige Antwortkörper; nach `cd91de5`: `ALLOW: No merge-blocking defect is established by the supplied diff and source snapshots.`

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft
TESTNACHWEIS[TW-1]: 21 passed, 0 ignored | Baseline: nicht erhoben, kein Altfehlerurteil
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: interner Taskbericht im Taskordner
ORCHESTRIERUNG[OR-1]: Stufe mittel | Schritt done | Artefakt: .tasks/2026-09-29-technical-closeout/A12-REPORT.md
INTENT[IA-1]: Stufe mittel | Modell Luna | Thread 562a877b-0939-440a-964d-1145d9e9431a | Register: /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/REGISTER.md
MERGEPROTOKOLL[MS-1]: 6 Git-Schritte einzeln | Anläufe: 3 | Gate: ALLOW nach Code- und Bericht-Commit; kein Merge angefordert
