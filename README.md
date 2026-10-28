# Deadlock Brain

Rust services that answer questions about Deadlock, the Deadlock-Docs knowledge base and the community, backed by their own Postgres database.

Start with [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md): components, binaries, units, schemas, data flow, deploy and rollback.

## Layout

- `rust/crates/`: all services and CLIs (Cargo workspace).
- `ops/`: systemd units, Postgres configuration, release helper sources.
- `docs/`: current documentation; `architecture/` holds migration history, marked historical.
- `scripts/migrations/`: applied SQL migrations, never edited.

## Quickstart

Type-check the workspace (production binaries are built only by the release helper, see Operations):

```sh
cargo check --manifest-path rust/Cargo.toml --workspace
```

Check the database schema against the expected state:

```sh
brain-migrate check --config <non-secret-config.json>
```

Ingest one source (sources are listed in `rust/crates/brain-job/src/sources.rs`):

```sh
brain-job --config /etc/deadlock-brain/brain-job.json ingest deadlock-docs
```

Run the answer API locally (secrets come from Infisical, not from files):

```sh
brain-serve --config ~/.config/deadlock-brain/brain-serve.json --infisical-config /etc/deadlock-brain/infisical-serve.json
```

Check that the answer API is ready, show local database and source counts, and score answers against the gold set:

```sh
curl -s http://127.0.0.1:8788/readyz
deadlock-brain status
brain-eval --help
```

## Operations

Production runs from `/opt/deadlock-brain/current`. Deploy, verify and roll back with `/usr/local/libexec/brain-release` as described in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md#6-deploy).

## Rules

- No secrets in files or commits. Runtime secrets come from Infisical.
- Applied migrations are not edited; add a new one.
- Internal documentation is in English.
