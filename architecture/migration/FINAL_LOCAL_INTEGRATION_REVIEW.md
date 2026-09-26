# Finale lokale Integrations- und Review-Notiz (DB-Pooling #49, C9 #50, Consumer)

Stand: 26.09.2026. Integrator: lokale Integrationssession.
Integrationsbranch: `migration/rust-integration`, eigener Worktree `~/.worktrees/brain-final-local-review-20260926`, Branch `integration/final-local-review-20260926`.

Integrierter Code-Commit: **`ed06e13`** (`a24c80d` = Merge PR #49, `ed06e13` = Merge PR #50, beide auf Basis `3b86d3c`). Dokumentations-Commit: siehe Historie des Branches.

Kein Production-Cutover, keine produktiven Bots/Consumer aktiviert, keine öffentlichen Nachrichten gesendet.

## 1. PR #49 `codex/fix-brain-db-pooling` (Pool/Backpressure)

Ergebnis: **integriert, semantisch reviewet, lokal gegen die echte getrennte Brain-Instanz nachgewiesen.**

Semantischer Review vor dem Merge:

- kein `postgres::Client::connect()` je Request/Operation mehr im `brain-serve`-Pfad; `LocalPgReader` ist ein gemeinsamer, hart begrenzter Pool (`Arc<ClientPool>`, Condvar-Acquire).
- geteilt von Startup-Preflight, DB-Permissions, `read_snapshot`, `read_heads`, Retrieval-Index, Evidence-/Cache-Revalidierung, Conversation-Ownership und `/readyz` (Readiness nutzt denselben Pool).
- Poollimit wird auch während des Connection-Aufbaus nicht überschritten (Slot-Reservation vor dem Connect); `max_connections` hart 1..=64, Service default 4.
- bounded acquire wait (`postgres_pool_wait_ms`, default 250 ms); Erschöpfung => `PortError::Unavailable` => typisiertes `AnswerStatus::Unavailable` (kein `unauthorized_evidence`, kein HTTP-503-Rohfehler im Anfragepfad).
- defekte Sessions werden beim Drop erkannt (`is_closed`), aus `open` entfernt und nicht wiederverwendet; Recovery nach DB-Störung innerhalb desselben Limits (Prozess-E2E und echter Instanz-Neustart belegen es).
- kein connect-per-operation-Fallback; Rest-`Client::connect` nur in Legacy-CLI-Werkzeugen außerhalb des Requestpfads (`pg_insights`, `pg_patchnotes`, `pg_steam_news`).
- `PolicyError::StatePoisoned` wird jetzt als `AnswerStatus::Unavailable` geantwortet statt rohem 503.
- C5-Wiki-Scratch-Test nach dem Async/Sync-Fix grün (`scripts/test_wiki_runtime.sh`: `scratch_release_written=true fact_records=3 provider_calls=0`).

## 2. PR #50 `codex/fix-c9-consumer-wiring` (C9)

Ergebnis: **integriert, semantisch reviewet.** Merge `ed06e13`, konfliktfrei auf #49-Basis; keine historischen C2/C3-/C6-Versionen wieder eingeführt (Diff: `brain-mcp.rs` neu, `deadlock-brain`-CLI `answer`, Cargo-Tokio-Makros, Handoff-Doku).

Brain CLI/MCP erhalten:

- typed `AsyncBrainClient` als einziger Antworttransport; `Answer`-Subcommand läuft vor jeder lokalen DB-Initialisierung (kein Legacy-Pfad im typed Command).
- `unavailable` und `build_rejected` bleiben Wire-Statuswerte (CLI-Test auf unveränderte JSON-Projektion; MCP mappt `build_rejected` als Domain-Ergebnis, `unavailable`/Transportfehler als `isError`).
- explizite Scopes: CLI `--scope` (pflicht), MCP `BRAIN_MCP_SCOPES`; Tool-Argumente können Scopes nicht liefern (Test gesichert).
- Token ausschließlich über Environment (`BRAIN_CLIENT_TOKEN`, `BRAIN_MCP_TOKEN`); kein Token per CLI-Argument.

## 3. Consumer-PRs (lokaler Review)

| Repo | PR | Branch-Head | lokaler Review |
|---|---|---|---|
| Deadlock-Twitch-Bot | #984 | `8e6ee4c` + Fix `d877a9d` | legacy = Bestandsverhalten; typed = sichtbare Antwort ausschließlich über BrainClient (Fehler => `unavailable_answer`, kein Legacy-Fallback); shadow = Legacy sichtbar. **Befund und Fix:** die Typed-Probe lief im Shadow-Zweig über `tokio::join!` und konnte die sichtbare Legacy-Antwort bis an ihre 115-s-Frist verzögern. Fix `d877a9d`: Probe abgekoppelt (`tokio::spawn`, eigene Frist), unbekannte `TWITCH_BRAIN_CLIENT_MODE`-Werte werden ausdrücklich geloggt; Regressionstest ergänzt; `check-brain-consumer.sh self-explainer` grün (fmt+test, 20 passed). |
| Deadlock-Bots | #459 | `0e3cf65` | Alle vier Handoff-Befunde im Diff bestätigt: 1) unbekannter `BRAIN_CLIENT_MODE` fail-closed (Test vorhanden), 2) kollisionsresistente IDs (128-Bit-Instance-Nonce vor Lokalsequenz), 3) Shadow-Probe abgekoppelt mit eigener Frist (Blocking-Probe-Regressionstest), 4) Auto-Merge-Pfad entfernt, Gate report-only mit Nur-Lese-Rechten (2 Policy-Tests grün). `check-brain-consumer.sh knowledge` fmt/test/clippy grün. Der rote GitHub-„Semantic review"-Check ist Quota-bedingt (`monthly quota exceeded`) und kein Codebefund; er wurde nicht umgangen, kein Force-Merge. |
| Deadlock-Docs | #4 | `ba4143f` | Adapter fest auf `docs.public` beschränkt,typed `brain-client` gepinnt auf `3b86d3c`; Live-Staging grün (siehe Abschnitt 6). `tools/brain-adapter/check.sh` fmt/test/clippy grün (6 Tests). |
| Deadlock-2nd-Brain | #2 | `46aa2d3` | Interne Bindung verweigert `public`/`*.public`/Wildcards, Subset-Prüfung gegen trusted Scopes; Live-Staging grün (siehe Abschnitt 6). `tools/brain-adapter/check.sh` fmt/test/clippy grün. GitHub-Runner-Problem (`runner_id=0`, `steps=[]`) ist infrastrukturbedingt, kein belegter Codefeffer; kein Required-Check umgangen. |

## 4. Workspace-Prüfung auf `ed06e13`

- `cargo fmt --all -- --check`: grün
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`: grün
- `cargo test --workspace --locked --offline`: **943 passed, 0 failed**
- `cargo build --workspace --release --locked --offline`: grün
- `scripts/test_brain_serve.sh`: grün (Scratch-Cluster max_connections=12, Pool 4; 600 Requests je 8/16/32 Worker: je 600 answered, 0 client errors, 0 `unauthorized_evidence`, beobachtete Reader-Connections 4, 0 „too many clients"; Fault-/Recovery-Phase grün)
- `scripts/test_brain_storage_upgrade.sh`: grün
- `scripts/test_brain_core_postgres.sh`: grün
- `scripts/test_wiki_runtime.sh`: grün (C5 offline, kein Live-Capture)
- `architecture/migration/s12/check-completion.sh`: grün

## 5. 600-Request-Test gegen die echte getrennte Brain-Instanz

Setup: `scripts/run_isolated_load.sh` über `run_isolated_pilot.sh` gegen PostgreSQL 5446 (nur Unix-Socket, `max_connections=40`, unverändert), `brain-serve` aus `ed06e13` mit `postgres.max_connections=4`, je Stufe frischer Ingest in `brain_pilot`, erzwungener PostgreSQL-Neustart, danach E2E + 600 Requests. Report: `~/.local/share/deadlock-brain/loadtest-20260926-final-local/`.

| Messgröße | 8 Worker | 16 Worker | 32 Worker |
|---|---|---|---|
| answered | 600 | 600 | 600 |
| insufficient_evidence / unavailable / unauthorized_evidence (Load) | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 |
| HTTP 429/503 (Load) | 0 | 0 | 0 |
| Pool hard max | 4 | 4 | 4 |
| Peak-Pool-Verbindungen | 4 | 4 | 4 |
| created connections | 4 | 4 | 4 |
| reused checkouts | 3051 | 3051 | 3051 |
| wait_count | 1279 | 1814 | 2060 |
| wait_timeout_count | 0 | 0 | 0 |
| wait_max | 11,2 ms | 70,2 ms | 57,2 ms |
| neue `brain_service`-Verbindungen laut DB-Log | 8 | 8 | 8 |
| „too many clients already" | 0 | 0 | 0 |
| `pg_stat_activity` Peak (brain_service) | 4 | 4 | 4 |
| p50 / p95 / p99 | 13,7 / 21,3 / 27,0 ms | 41,8 / 81,0 / 110,7 ms | 47,8 / 102,7 / 173,8 ms |
| Brain CPU (utime+stime) / RAM (RSS) | ~3,8 s / 25,3 MiB | ~4,6 s / 27,5 MiB | ~3,9 s / 27,0 MiB |
| PostgreSQL CPU / RAM-peak | 6,9 s / 112 MiB | 7,4 s / 102 MiB | 7,4 s / 107 MiB |

Messverbindungen neben dem Pool: der Taster (`sample_brain_pg.sh`) verbindet als Superuser `deadlock-brain-pg` für `pg_stat_activity`; er gehört nicht zum `brain_service`-Kontingent und ist in der Spalte „Peak" nicht enthalten.

Pflichtbedingungen: 1) kein „too many clients" ✓ 2) Peak 4 = Poollimit 4, administrative Messverbindungen erklärt ✓ 3) 8 Neuverbindungen je Stufe statt hunderten ✓ 4) Wiederverwendung belegt (3051 Checkouts auf 4 Sessions) ✓ 5) 0 falsche `unauthorized_evidence` ✓ 6) bounded wait vorhanden; Erschöpfungspfad => `Unavailable` im Prozess-E2E (ACCESS EXCLUSIVE Lock) nachgewiesen ✓ 7) Recovery nach erzwungenem Instanz-Neustart und nach Verbindungsabbruch grün ✓ 8) `max_connections=40` unverändert ✓.

**600_REQUEST_TEST_PASSED: JA**

## 6. Default-E2E nach Pooling (Abschnitt 8)

Je Stufe gegen die eigene Brain-PostgreSQL => `brain-serve` => typed `BrainClient`: **18/18 bestanden, `failed_cases: []`** (public question, internal question, Scope-Leak-Schutz in beiden Richtungen, exact number, Alias EN, Alias DE, unknown entity, missing evidence, wrong patch, wrong mode, invalid token, legal build, illegal build => `build_rejected`, Hero Card, provider failure, ACL revoke, delete, DB-unavailable/Recovery durch echten Neustart zwischen Ingest und Serve). Kein Diagnosebudget erhöht, kein Legacy-Fallback.

**DEFAULT_E2E_PASSED: JA**

## 7. Consumer-Staging (Abschnitt 9)

Nur isolierte/testweise Pfade gegen die lokale `brain-serve`-Staging-Instanz (Port 18790, brain_pilot, Loopback-Provider-Fixture, Nichtproduktionstoken):

- Brain CLI (`deadlock-brain answer`): answered mit Zitat ✓; falscher Token => HTTP 401, Abbruch ✓; nicht freigegebener Scope => HTTP 403, Abbruch ✓.
- Brain MCP (`brain-mcp`): `initialize`/`tools/list`/`tools/call` answered ✓; falscher Token => `isError`/unavailable ✓; falscher Scope => `isError`/unavailable ✓; 1-ms-Timeout => `isError`/unavailable ✓.
- Docs-Adapter: answered ✓; falscher Token => Transportfehler, Exit 64 ✓; Scope fest `docs.public` (Server-ACL entscheidet) ✓.
- 2nd-Brain-Adapter: intern answered ✓; `public`/`docs.*`-Bindung abgelehnt ✓; interner Scope mit Public-Token => Server-ACL-Ablehnung ✓.
- Conversation-Isolation live: öffentlicher Actor an einer vom internen Actor beanspruchten Conversation => 403 forbidden, kein Leck ✓.
- Provider-Ausfall/Recovery: Provider-Fixture gestoppt => `provider_error` (typisiert); wieder gestartet => answered ✓.
- Twitch: Fixture-/Testpfad (`check-brain-consumer.sh self-explainer` inkl. Shadow-Entkopplungs-Regression), keine öffentliche Route gestartet ✓.
- Deadlock-Bots: Fake-Transport (Fixture-HTTP-Server) in `dl-brain`-Tests, Modus-/ID-/Shadow-Regressionen grün; kein Discord-Transport gestartet ✓.
- `build_rejected`, ACL revoke, delete: live im E2E je Stufe beobachtet (Abschnitt 6).
- Shadow: sichtbare Legacy-Antwort wartet nach `d877a9d` nicht auf die Typed-Probe (Bots- und Twitch-Regressionstests) ✓.

Keine echten Discord-/Twitch-Nachrichten gesendet.

**CONSUMER_STAGING_PASSED: JA**

## 8. DB-Isolation und Backup/Restore

- `ops/brain-postgres/verify-isolation.sh` am 26.09. (nach Integration) erneut: alle Proben PASS, 0 Fehlschläge — `brain_service`/`brain_ingest`/`brain_readonly` erreichen DL-Main weder per Socket noch TCP, kein `postgres_fdw`/`dblink`/Foreign-Server, DL-Main kennt keine Brain-Rolle, DDL-/Owner-/Rollenrechte getrennt.
- Instanz: eigener Prozess (systemd `deadlock-brain-postgresql`), eigener OS-User, eigenes PGDATA `/var/lib/deadlock-brain/postgresql`, eigener Socket `/run/deadlock-brain-postgresql`, Port 5446, eigene Rollen mit Connection-Limits, nur Unix-Socket.
- Backup: täglich (`deadlock-brain-postgresql-backup.timer`, letzter Lauf Exit 0), Basebacks im `brain-20260926T031943Z`.
- Restore-Probe frisch ausgeführt: `brain` fingerprint-gleich; `brain_pilot` weicht erwartbar ab, weil der Wegwerf-Pilot heute nach der Sicherung neu befüllt wurde (Runtime-Tabelle `conversation_owners_v1` 279 => 629 durch Last-/Staging-Conversations); Schema-, ACL-, Release-, Tombstone- und übrige Zeilen-Hashes identisch.

Legacy-Abhängigkeiten zu DL-Main wurden nicht als Cross-DB-Zugriff wieder eingeführt; bestehende Übergänge laufen als Feeds/APIs (dokumentiert in `BRAIN_POSTGRES_ISOLATION.md` bzw. Legacy-Runtime). Deadlock API nutzt vorhandene Analytics-/ClickHouse-Daten; kein neues ClickHouse.

## 9. Wiki, Provider, Replay

- Wiki: C5 repo-seitig fertig und offline grün; echter Wiki-Pilot bleibt gesperrt, weil die Rechte-/Lizenzentscheidung für Capture und Raw-Aufbewahrung fehlt. Keine Eigenmächtigkeit, kein Full-Wiki-Capture, kein zweiter Produktionsstore (Instanz enthält weiterhin nur `brain` und `brain_pilot`).
- Provider: kein Modell/Provider freigegeben; nur Loopback-Fixtures. Keine Auswahl getroffen.
- Replay: keine freigegebene echte `.dem`; C10 bleibt offen.

## 10. Marker

BRAIN_DB_ISOLATED: JA
BRAIN_DATA_MIGRATION_VERIFIED: JA
DB_POOLING_VERIFIED: JA
600_REQUEST_TEST_PASSED: JA
DEFAULT_E2E_PASSED: JA
CONSUMER_STAGING_PASSED: JA
WIKI_REAL_PILOT_PASSED: NEIN
PROVIDER_SHADOW_PASSED: NEIN
REAL_REPLAY_PASSED: NEIN
G1_READY: JA
G2_READY: NEIN (weiterhin offen: echter Wiki-Pilot, echter Provider, echte Replays)
G3_READY: NEIN (Consumer-Parität lokal belegt; Gesamtdatenübernahme fehlt)
G4_READY: JA (lokal: Last, Isolation, Backup/Return bestanden)
PRODUCTION_CUTOVER_READY: NEIN

Kein Production-Cutover ausgeführt; G5/G6 bleiben der Betreiberfreigabe vorbehalten.

## 11. Superseded/ersetzt

- „Pooling fehlt"-Aussagen in `STATUS.md`, `WAVE1_WAVE2_LOCAL_REVIEW.md`, `INTEGRATION_REVIEW.md` und `GATES.csv` sind durch diesen Nachweis ersetzt.
- „C9 fehlt/nicht integriert" ebenda ersetzt.
- Remote-Zahlen aus den PR-Beschreibungen (#49: 600/8/16/32 auf Scratch) wurden nicht als lokale Abnahme übernommen, sondern gegen die echte Instanz neu gemessen (Abschnitt 5).
- Der frühere lokale Arbeitsstand `integration/final-local-20260926` (Worktree `brain-final-local-20260926`, uncommittete Idle-Prune-Änderung) wird hiermit ersetzt; die Prune-Idee ist überflüssig, weil defekte Sessions beim Drop bereits aus dem Pool entfernt werden und Recovery nachweislich funktioniert.
