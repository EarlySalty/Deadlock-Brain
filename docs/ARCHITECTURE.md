# Deadlock Brain architecture

Status: describes origin/main after P8 (merged 2026-10-28, c7e12e1e). Units, binaries, schemas and paths were checked against `systemctl` (system and `--user`), `/opt/deadlock-brain/current/bin` and the source tree on the same day. Anything not verified is marked as such.

## 1. What the Brain is

The Deadlock Brain answers questions about Deadlock (heroes, items, patches, game facts), the Deadlock-Docs knowledge base and the community. It is a Rust service set with one Postgres database of its own. Python and SQLite components are gone; the legacy `brain_legacy` schema was dropped by migration v47.

## 2. Binaries

All binaries of the live release are in `/opt/deadlock-brain/current/bin`.

| Binary | Source crate | Role | How it runs |
| --- | --- | --- | --- |
| `brain-serve` | `rust/crates/brain-serve` | HTTP API: `/v1/answer`, `/v1/retrieve`, `/v1/operator/query`, `/v1/task`, `/v1/analytics/observation`, `/healthz`, `/readyz` | System unit `brain-serve.service`, 127.0.0.1:8788 |
| `brain-job` | `rust/crates/brain-job` | Ingests source data into the Brain DB (`brain-job --config <json> ingest <source>`), receipt audit (`audit-receipts`) | Templated system units `brain-job@<name>` |
| `brain-migrate` | `rust/crates/brain-storage` (`src/bin`) | Schema checks and migrations: `check`, `up`, `check-response-audit`, `up-response-audit`, `check-gamefacts`, `up-gamefacts` | Run by hand with a non-secret JSON config |
| `brain-mcp` | `rust/crates/deadlock-brain` (`bin/brain-mcp.rs`) | MCP adapter that forwards to `brain-serve`; scopes come from its config (for example `docs.public`); no token in the config, the secret is referenced in Infisical | Started by the MCP client; no systemd unit |
| `deadlock-brain-secret-exec` | `rust/crates/deadlock-brain` | Loads runtime secrets through the Infisical transport and execs one command | Wrapper, used by units and tools |
| `deadlock-brain-site` | `rust/crates/deadlock-brain` (`bin/deadlock-brain-site.rs`) | Site backend on loopback 127.0.0.1:8087 | System unit `deadlock-brain-site.service` |
| `brain-browser-consent` | `rust/crates/dbrain-sources` (`bin/brain-browser-consent.rs`) | Browser consent helper (`--config <json> [--once]`) | System unit `brain-browser-consent.service` |
| `deadlock-brain` | `rust/crates/deadlock-brain` | Operator CLI: `status`, `answer`, `context`, `reason`, `quality`, `site`, `ai-model`, `pg` and others | Run by hand |
| `brain-patchnotes-ingest` | `rust/crates/brain-feeds` (`bin/brain-patchnotes-ingest.rs`) | Patch-note import: `preview`, `validate`, `stage-candidate`; apply is locked | Run by hand |
| `brain-knowledge-import` | `rust/crates/dbrain-sources` (`bin/brain-knowledge-import.rs`) | Game knowledge import: `extract-game`, `export-legacy-game`, `inspect-game`, `reauthorize-game`, `register-asset-mirror`, `partition`, `validate`, `import`, `publish` (runtime config `/etc/deadlock-brain/maintenance-runtime.json`) | Run by hand |
| `brain-match-ingest` | `rust/crates/brain-feeds` (`bin/brain-match-ingest.rs`) | Match ingest with a private JSON config: `ingest`, `revoke` | Run by hand |
| `brain-eval` | `rust/crates/brain-eval` | Scores a `brain-serve` endpoint against the E1 gold set (default 127.0.0.1:8788, scope `bot.public`) | Run by hand |

Removed in P8 (2026-10-28): `brain-maintenance`, the Python `src/` tree, `dbrain-learn`, `dbrain-wiki`, `dbrain-replay`, `brain-legacy-import`, `deadlock-brain-yt`, `dbrain-population`, `dbrain-enrich`, `brain-jev`, `dbrain-session-store`. The inventory is in `.tasks/2026-10-28-p8-legacy-delete/INVENTAR.md`.

## 3. Units and timers

System units (`systemctl`):

| Unit | Purpose | Notes |
| --- | --- | --- |
| `brain-serve.service` | Answer and retrieval API | MemoryMax 512M; config `~/.config/deadlock-brain/brain-serve.json`, secrets via `/etc/deadlock-brain/infisical-serve.json` |
| `brain-browser-consent.service` | Browser consent helper | MemoryMax 64M |
| `deadlock-brain-site.service` | Site backend | loopback 8087, MemoryMax 256M, peer config for Postgres |
| `deadlock-brain-postgresql.service` | Brain database instance | Own instance, Unix socket only; `ops/brain-postgres/postgresql.conf` |
| `deadlock-brain-postgresql-backup.timer` | Daily Postgres backup | `ops/brain-postgres/backup.sh` |
| `brain-job@deadlock-data.timer` | deadlock-data game files (hero data and mechanics) | every 5 minutes at minute 2/5; own unit file installed by `brain-release`, which re-enables it on every release |
| `brain-job@assets.timer` | Asset source | hourly |
| `brain-job@deadlock-api-meta.timer` | Deadlock API meta statistics | hourly |
| `brain-job@deadlock_patchnotes_db.timer` | Patch notes database | every 5 minutes; drop-in `item-lineage.conf` runs the item-lineage ingest after success |
| `brain-job@deadlock-docs.timer` | Deadlock-Docs pages (`public/` and `internal/`) | hourly |
| `brain-job@audit-receipts.timer` | Receipt audit | daily 03:17 |

Patch freshness: every `deadlock-data` ingest records its upstream revision in `ops.ingest_run_revision` and the recent upstream commits in `ops.upstream_commit`. The view `ops.patch_listing` reads each upstream patch (number, title, publish time, first fetch) from the raw patchnotes objects, so the ingest role can read it without the serving publication gate. `ops.patch_freshness` joins each patch with its Brain detection time, the first Deadbot commit after publication and the first ingest run that carries that revision; `ops.game_data_freshness` lists every Deadbot client update and marks the ones without a patch as `silent`. A run counts as live only when it succeeded without rejections. The source cache keeps the last 16 upstream commits, so updates between two polls are recorded too. Legacy detection times come from `brain.patch_events`, matched on the publication time. `brain-job --config /etc/deadlock-brain/brain-job.json freshness [--since <date>]` prints the patch table, every client update and the median and maximum per stage. `brain-job ... freshness-history <git-repository>` backfills commits from a full clone and infers revisions of older runs (marked `*`).

The template `brain-job@.service` runs `brain-job --config /etc/deadlock-brain/brain-job.json ingest %i` with MemoryMax 512M. The template `brain-job@.timer` is hourly with a 2 minute random delay. `brain-serve.service` has a drop-in `item-lineage.conf`.

User units (`systemctl --user`):

| Unit | Purpose | Notes |
| --- | --- | --- |
| `deadlock-brain-luna-proxy.service` | GPT-Luna proxy on 127.0.0.1:18769 | Active. Not on the `brain-serve` answer path. |
| `public-corpus-refresh.timer` | Refreshes the public Deadlock-Docs corpus every 15 minutes | Runs `dl-corpus-refresh` with `/opt/deadlock-docs/current/tools/deploy_corpus.sh` and `ops/public-corpus-refresh.json`. Website corpus, not the Brain import. Relation to `brain-job` not confirmed. |
| `dl-brain-feeder.timer` | Legacy weekly feeder | Timer enabled. Its service is inactive and its last run failed. It points to a Python script in the old Deadlock-Bots tree. Not part of the Brain; see open items. |

Ports seen listening: 8788 (brain-serve), 8087 (site), 18790 (dl-ai gateway). Ports 18767, 18768 and 8787 are not documented here because their owners were not verified.

## 4. Databases and schemas

The Brain database runs in `deadlock-brain-postgresql.service`. The config sets port 5446; clients connect through the Unix socket.

| Schema | Contents |
| --- | --- |
| `raw` | `raw.object`: stored source payloads |
| `canon` | `canon.entity`, `canon.fact`, `canon.alias`, `canon.relation`, `canon.entity_identity`, `canon.doc_search`, `canon.receipt`, `canon.receipt_quarantine`, `canon.test_quarantine` |
| `serve` | Read model for the API. Views such as `serve.current_fact`, `serve.current_entity`, `serve.current_alias`, `serve.entity_card`, `serve.hero_profile`, `serve.item_card`, `serve.ability_card`, `serve.doc_page`, `serve.doc_section`, `serve.doc_search_hit`, `serve.meta_stat`, `serve.item_lineage_entry`, `serve.published_receipt`. Table `serve.current_relation`. |
| `ops` | `ops.ingest_run`, `ops.receipt_audit`, `ops.receipt_audit_run`, `ops.receipt_audit_rejection` |
| `brain` | Core schema kept from the pre-Rust store (for example `brain.core_schema_version`, `brain.source_record_revisions`). The backup fingerprint in `ops/brain-postgres/backup.sh` reads it. New objects go to `canon`, `serve` or `ops`. |

`brain_legacy` no longer exists (migration `scripts/migrations/2026-10-28-brain-legacy-drop-v47.sql`). Applied migrations are never edited; new ones are added and applied with `brain-migrate up`.

The central database `deadlock` with schema `brain` is a separate database. It is not part of this architecture and is read by the patchnotes bot, the Bots and the `dl-brain` MCP.

## 5. Data flow: sources to answers

1. **Sources.** `brain-job` reads the sources listed in `rust/crates/brain-job/src/sources.rs`: `deadlock-data` (git clone of deadlock-wiki/deadlock-data, `data/json/hero-data.json`), `gametracking`, `assets`, `deadlock_patchnotes_db`, `deadlock-api-meta`, `deadlock-docs`, `item-lineage`.
2. **Ingest.** `brain-job ... ingest <source>` stores source payloads in `raw.object`, records the run in `ops.ingest_run`, and writes entities, facts, aliases, relations and receipts to the `canon` schema.
3. **Read model.** The `serve` views expose the current state for the API.
4. **Answer.** `brain-serve` `/v1/answer` retrieves from `serve`, builds the context and calls the answer provider. The provider is `kind dl_ai_gateway` at `http://127.0.0.1:18790/v1` with slot `brain_answer` (header `x-dl-ai-slot`). AI calls go through this central gateway, not through custom connectors.
5. **Callers.** Consumers listed in `brain-serve.json` (bot task and Discord consumers) and `brain-mcp` (scope `docs.public`) call `brain-serve`. `brain-eval` checks answer quality against the gold set.

### How Deadlock-Docs reaches the Brain

- The `deadlock-docs` source of `brain-job` clones `EarlySalty/Deadlock-Docs` and takes the `.html` pages under `public/` and `internal/`. It writes them through the same raw and canon path, and the `serve.doc_page`, `serve.doc_section` and `serve.doc_search_hit` views expose them.
- Member answers (`bot.public`, `docs.public`) only search pages under `public/`. Pages under `internal/` get scope `internal` and are only returned to the internal second-brain operator identity (`second_brain.internal`, no provider egress); see `scopes()` in `rust/crates/brain-serve/src/docs_tools.rs`.
- `brain-job@deadlock-docs.timer` runs the import hourly. After a docs merge it can be started at once with `sudo -n systemctl start brain-job@deadlock-docs.service`.
- The public website corpus (`/docs`) is a separate path: `public-corpus-refresh.timer` deploys the Deadlock-Docs corpus through `deploy_corpus.sh`. It is not the Brain import.

## 6. Deploy

Deploys use the root-owned helper `/usr/local/libexec/brain-release`. Run it with `sudo -n` where needed.

1. `brain-release plan <source>`: checks that the source is at the remote `main`, prints the current and maintenance-current targets and the build plan.
2. `brain-release build <source> <new-bundle>`: builds all workspace binaries with `cargo-slot` (`CARGO_SLOT_DEPLOY=1`, shared sccache, fresh target dir that is removed afterwards). Per `ops/brain-release/src/main.rs` the build is run through the trusted helper.
3. `brain-release verify <source> <bundle>`: checks the bundle against the source and writes its manifest (format 3).
4. `brain-release install <source> <bundle>`: stages the release under `/opt/deadlock-brain/releases/<SHA>/` and `maintenance-releases/<SHA>/`, activates the gamefacts state, installs the units and moves the `current` pointer. Services are restarted separately, for example `systemctl restart brain-serve.service`.

Layout:

- `/opt/deadlock-brain/releases/<SHA>/` and `/opt/deadlock-brain/maintenance-releases/<SHA>/` with `manifest.json`.
- `/opt/deadlock-brain/current` is a symlink to the live release (`c7e12e1e50d1a24a630e997f77fcb13e3e002573` on 2026-10-28).
- `/opt/deadlock-brain/maintenance-current` is a compatibility alias for `/opt/deadlock-brain/current/bin`.
- Lock: `/opt/deadlock-brain/.deploy.lock`.

## 7. Rollback and recovery

- Roll back to a release that is still on disk: `sudo -n /usr/local/libexec/brain-release rollback <SHA>`. The previous release is `7a0c990b0a0ff50811827a5a7ab662594abc0c56`. Rollback re-points `current` and reinstalls the units, then restart the affected services.
- After an interrupted install: `sudo -n /usr/local/libexec/brain-release recover`. It reverts the partial install and does not restart services.
- Check the result with `systemctl status brain-serve.service` and `curl -s http://127.0.0.1:8788/readyz`.

## 8. Operator checks

- Schema state: `brain-migrate check --config <json>`.
- API readiness: `curl -s http://127.0.0.1:8788/readyz`. Local database and source counts: `deadlock-brain status`.
- Answer quality: `brain-eval` against 127.0.0.1:8788.

## 9. Open items

- `dl-brain-feeder.timer` (user) is enabled but failed and references a Python script in the old Deadlock-Bots tree. Owner decides whether to remove it.
- The relation between `public-corpus-refresh` and the Brain import is not confirmed.
- Historical plans under `architecture/` are marked as historical and kept for reference.
