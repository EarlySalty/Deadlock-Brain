# External source fixtures

The `openapi-20260925.json` file contains the unchanged HTTP entity body of one
bounded GET to `https://api.deadlock-api.com/openapi.json` on 2026-09-25.
`manifest.json` records the byte length, exact SHA-256 and separate API/OpenAPI
versions. This is a frozen observation, not an automatic dependency upgrade.
The JSON stays byte-for-byte intact, rather than being reformatted for display.

The schema is a public API contract. Its `info.license` describes the upstream
API project; it does not establish publication, redistribution or provider-egress
rights for game assets, player data or any other payload. Those authorizations
remain false/unknown in the source provenance.

Other deterministic fixtures are inline owned Rust literals and small temporary
Git repositories created by `git_source` tests. Git author/committer timestamps
are fixed; no external repository is cloned, no source code is executed, and no
player/match or game-asset corpus is vendored. The shallow-history test clones
only its own local synthetic `file://` repository.

## Deterministic suite

From `rust/`, with the repository Rust toolchain and SQLx offline metadata:

```sh
SQLX_OFFLINE=true cargo test -p dbrain-sources -p deadlock-brain-core --locked -j2
```

Default tests use fixtures and loopback HTTP sockets only. Existing tests marked
as requiring Postgres remain ignored. No provider, private credentials or source
scheduler is required. `.github/workflows/ci.yml` now runs this suite after the
existing workspace compilation check.

## Small live contract probe (explicit opt-in only)

```sh
SQLX_OFFLINE=true DBRAIN_EXTERNAL_LIVE_CONTRACT=1 \
  cargo test -p dbrain-sources --test external_sources \
  live_small_current_assets_contract --locked -- --ignored --exact --nocapture
```

This one test performs at most two GETs without retries: OpenAPI (1 MiB limit)
and `/v1/assets/colors` (64 KiB limit). It does not fetch players, matches, replays,
history, all assets, source repositories or a bulk export. Response hashes,
versions and byte counts are printed; the colors payload is not committed.
Live availability is deliberately not part of deterministic CI.

## Coverage boundaries

Runtime Assets/Match validation checks explicitly consumed identities and
containers, retaining extra fields without interpreting them as new facts.
OpenAPI drift compares consumed response schemas and relevant request/security
contracts. Unknown constraint/semantic changes are not silently accepted.
This is not a complete JSON Schema implementation or a claim that every game
field, unit and effect has been semantically verified.

The protobuf change parser accepts only its documented syntax/message/enum/field
subset. Imports, options, services, maps, `oneof`, unresolved types and unsupported
constructs quarantine the source signal rather than invoking `protoc` or guessing.
