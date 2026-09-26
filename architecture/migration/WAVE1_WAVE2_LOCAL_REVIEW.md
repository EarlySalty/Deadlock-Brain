# Lokales Review Welle 1 und Welle 2 (C1 bis C11)

Stand: 26.09.2026. Integrator und Verifizierer: lokale Claude-Session im Worktree `~/.worktrees/brain-pg-isolation-20260926`, gepusht nach `migration/rust-integration` (PR #40). Basis vorher: `087c522`.

## Eingänge und Reihenfolge

| Reihenfolge | PR | Branch-Head | Merge | Konflikte und semantische Auflösung |
|---|---|---|---|---|
| 1 | #43 C4 Contracts | `ea2bd15` | konfliktfrei | Basis für C5/C6 |
| 2 | #41 C11 Storage | `661b2a5` | konfliktfrei | liefert `check_core_schema()`, `migrate_core()` nur in `brain-migrate` |
| 3 | #45 C2/C3 Retrieval | `f0cd5ad` | `brain-contracts/src/lib.rs` | Modulliste vereinigt: C4 `external`, `replay`, `source`, `wiki`, `value` plus C2 `provider_input`, `retrieval`, `DocumentHead`/`ChunkProvenance` |
| 4 | #42 C1 brain-serve | `a85fd52` | Workflow, `local_pilot.rs` | beide CI-Schritte behalten (C11-Upgrade-Probe und C1-Prozesspilot); Pilot auf den C1-Crate `brain-serve` mit Dienst-PID |
| 5 | #44 C7/C8 Runtime | `dbd0e01` | `s12/check.sh`, `check-completion.py` | Python-Prüfer entfernt (C8), Shell-Runner nutzt C4-Workspace-Pfade; `check.sh` baut per Manifest und startet das Probe-Binary aus `CARGO_TARGET_DIR` |
| 6 | DB_POOLING_BACKPRESSURE | `d020891` (PR #49) | konfliktfrei | siehe `FINAL_LOCAL_INTEGRATION_REVIEW.md` |
| 7 | #47 C5 Wiki | `f4f1e9e` | konfliktfrei | |
| 8 | #48 C6 Domain/Build | `cb76914` | fünf Dateien | siehe unten |
| 9 | C9 Consumer-Wiring | `0424790` (PR #50) | konfliktfrei auf #49-Basis | CLI/MCP über typed BrainClient, Token nur per Env, Scopes explizit |
| vorbereitend | #46 C10 Replay | `919c790` | nicht gemergt | ohne freigegebene `.dem` kein echter Nachweis; bleibt eigener PR |

## Semantische Integrationsfixes (eigene Commits)

- `a4921ab`: `brain-serve` ruft beim Start `check_core_schema()` auf (neuer Fehler `core_schema_incompatible`), `migrate_core()` bleibt ausschließlich `brain-migrate`. C6-Domain-Pfad läuft vor dem C2-Chunk-Index, typisierte Domain-Records werden aus dem lexikalischen Index gefiltert. Geteilte Single-Flight-Antworten mit Zitaten werden validiert (C6-Bedingung) und Fehler über `validation_status` abgebildet (C2), damit technische Fehler `unavailable` bleiben. Domain-Evidence ohne Chunk-Provenienz. Neue `Query.domain`-Felder in C2-Tests.
- `86d2ae7`: `CachedKernel` rechnet veraltete Domain-Antworten (PermissionDenied bei der Revalidierung) unter aktuellen ACLs neu, statt `unauthorized_evidence` auszugeben; der Prosa-Pfad aus C2 bleibt unverändert. `s12/check.sh` passend zum C8-Runner-Vertrag.

Erhalten: C2 `read_heads`, Chunking und BM25; C4 Wiki-, Source- und Replay-Verträge (C5 und C6 nutzen `brain-contracts`, keine zweite Contract-Crate); `unavailable` und `build_rejected` im öffentlichen Vertrag (beide im E2E beobachtet); alle CI-Workflows.

## Review-Befunde je Paket

| Paket | Befund | Bewertung |
|---|---|---|
| C1 | Composition Root startbar, Health/Readiness, SIGTERM-Budget, Passwort nur per Env-Port. Seit PR #49 ein gemeinsamer, hart begrenzter `LocalPgReader`-Pool für den gesamten Anfragepfad | ok (Pool nach PR #49 auf eigener Instanz nachgewiesen) |
| C11 | Migration getrennt, Upgrade/Restore-Probe, Least-Privilege-Test. `check_core_schema()` braucht SELECT auf alle sechs Kerntabellen, auch für Ingest | ok, Grants angepasst |
| C2/C3 | Chunk-Index je Release, Heads batchweise, Cache revalidiert; Prosa-Revoke im Cache ergibt `unauthorized_evidence` | ok |
| C4 | Versionierte Envelopes, gemeinsame IR | ok |
| C5 | Capture, IR und Facts nur gegen eigenen Scratch-Cluster (Marker, Superuser-Rolle, Zusatztabellen außerhalb `brain-migrate`) | Blocker für echten Wiki-Pilot auf der neuen Instanz |
| C6 | Domain-Kernel, Build-Legalität, Hero-Card; Vertrag `brain.domain.v2` mit Lesepfad für v1 | ok nach Integrationsfix |
| C7/C8 | Quellenpins, Python-freier Runner | ok nach Pfadanpassung |

## Tests auf `fe41451`

TESTNACHWEIS[TW-1]: 940 passed, 71 ignored | Baseline: 0 rot (vor der Integration 709 passed auf `087c522`)

Befehl: `SQLX_OFFLINE=true cargo test --workspace --locked --offline -j4 --no-fail-fast` mit Rust 1.97.1, 89 Testbinaries, 0 failed. Zusätzlich grün: `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`, `cargo fmt --all -- --check`, `scripts/test_brain_serve.sh` (1 passed), `scripts/run_local_pilot.sh` auf Scratch-Cluster (ingest, after_restart_default, reader_failures, rebuild: alle 0), `scripts/test_wiki_runtime.sh` (C5 offline, `scratch_release_written=true fact_records=3 provider_calls=0`), `architecture/migration/s12/check.sh`.

Gegen die eigene Brain-Instanz: Default-E2E 18/18, Betriebsprüfungen 16/16, Isolation 54/54, Backup/Restore gleich. Lasttest nicht bestanden (siehe `BRAIN_POSTGRES_ISOLATION.md`).

## Nicht durchgeführt

- Echter Wiki-Pilot mit zwei Revisionen: Rechte- und Lizenzfreigabe für Capture und Raw-Aufbewahrung fehlt (nicht automatisch gesetzt); C5-Store nicht an die neue Instanz anschließbar ohne Umbau.
- Provider-Shadow-Test: kein freigegebener Provider und kein freigegebenes Modell.
- Replay: keine freigegebene `.dem`; C10 nicht ausgeführt.
- Live-Abnahme: `LIVEBEWEIS[DV-1]` nicht ausgeführt: PR-first-Testbetrieb, kein Deploy, kein Produktivdienst.

Consumer-Staging ist inzwischen lokal nachgeholt: siehe `FINAL_LOCAL_INTEGRATION_REVIEW.md` (CLI, MCP, Docs, 2nd-Brain live gegen brain-serve-Staging; Twitch und Bots über Fixture-/Testpfade).

## Marker

BRAIN_DB_ISOLATED: JA
600_REQUEST_TEST_PASSED: JA (nach PR #49 gegen die eigene Instanz, 8/16/32 Worker)
DEFAULT_E2E_PASSED: JA (18/18 gegen die neue Instanz; Build- und Card-Fälle mit synthetischem Domain-Adapter)
WIKI_REAL_PILOT_PASSED: NEIN
CONSUMER_STAGING_PASSED: JA (nur lokale Staging-Pfade, keine produktiven Bots)
PRODUCTION_CUTOVER_READY: NEIN
