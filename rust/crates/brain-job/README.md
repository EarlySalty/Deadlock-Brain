# Hero ingestion

The `hero` extractor accepts only source `deadlock-data` and path `data/json/hero-data.json`. It projects one original JSON blob, not separate hero files. It does not load localization, abilities, items, upgrades or scaling files.

The root keys identify heroes through `limits::normalize_key`. Literal `Name` becomes `name_en` and one English alias through `limits::normalize_alias`. The null `Name` on `hero_testhero` is skipped. Other missing, null or malformed names reject the whole blob.

Supported optional fields are `Type`, `MaxHealth`, `BaseHealthRegen`, `MaxMoveSpeed`, `SprintSpeed`, `Stamina`, `LightMeleeDamage` and `HeavyMeleeDamage`. A null `Type` means no type fact; it occurs on 26 named heroes in the inspected blob. Weapon facts come only from `Weapon/BulletDamage`, `RoundsPerSecond`, `ClipSize` and `ReloadTime`, using the `weapon_` stat prefix. A present supported field of the wrong type rejects the entire blob. Unresolved `Lore`, `Role`, `Playstyle`, `WeaponNameKey` and availability flags are not projected.

Strict parsing rejects duplicate decoded JSON keys. Normalized-key collisions are checked recursively before extraction. Receipts use JSON pointers into the original object and SHA-256 hashes of original value tokens, including string escapes and numeric spelling. Canonical limits are checked before returning accepted facts.

## Configuration and invocation

Use a normal JSON configuration, never an environment or token file. The following example contains only non-secret configuration. Replace the socket, port, database and absolute paths with the verified installation values. `source_cache` is the parent cache directory; the loader appends `deadlock-data`.

```json
{
  "socket": "/replace/with/postgresql/socket",
  "port": 5432,
  "database": "brain",
  "user": "brain_ingest",
  "source_cache": "/replace/with/source-cache",
  "auth": "infisical",
  "infisical_config": "/replace/with/existing/infisical.json"
}
```

The existing central Infisical provider supplies the ingest credential at runtime. Do not add password fields or write credential files. For an existing peer-authenticated installation, use `"auth": "peer"` and omit `infisical_config`.

The database schema must already be installed through the existing release migration path. The ingestion command checks the schema and does not apply migrations.

Core migration 7 adds `canon.write_run_fact(bigint,text,double precision,text,boolean,bigint,bigint)`. Only `brain_ingest` receives EXECUTE; no direct UPDATE permission on `canon.fact` is granted. The `brain_migrate`-owned SECURITY DEFINER function pins `search_path=pg_catalog`, locks the referenced run and requires its failed status and a receipt belonging to that run. It inserts a fact or replaces only its value columns and receipt within the same `(entity_id,stat_key,valid_from_run)` snapshot. Published facts and older receipts remain unchanged. An `ops.ingest_run` trigger prevents retracting a succeeded run. Migration 1 and its uniqueness constraint remain frozen; versions 2 through 6 remain reserved for their packages.

Prechecks include committed applications from the active run, as well as published runs, but exclude other failed runs. A nonempty source with no matching consumer fails centrally. Supported `Outcome::Ignored` results and unchanged reruns remain successful without requiring new writes.

```sh
brain-job --config /absolute/path/to/brain-job.json ingest deadlock-data --input /absolute/path/to/deadlock-data
```

`--input` reads the exact `HEAD` blob from an existing Git repository without fetching or splitting it. Omitting `--input` refreshes the configured source cache through the core source loader.

## Original-source checks

The upstream revision used for the source contract is `2a26e26535a390c309e300e2c7b2763fa5ad7abd`. Its original 226,182-byte blob has SHA-256 `947c26790e067109f6cca4085649ed56e446bc5f4c9c571cd396290f802a3456`, with 68 root objects and 67 named heroes.

The optional original-source tests require that exact blob via the non-secret `BRAIN_HERO_FIXTURE` path. The PostgreSQL proof additionally uses the pinned Git cache at `BRAIN_CORE2_HERO_REPOSITORY`, dispatches through `SOURCES` and the default `EXTRACTORS`, and checks the schema under `brain_ingest`. They are ignored by default so ordinary package checks do not need a local source cache. Explicitly run them when validating ingestion. The PostgreSQL test uses the existing temporary PostgreSQL 16 helper and never connects to a configured production database.

Place the verified public blob in an otherwise empty directory at the same absolute path locally and on ionos. Supply its path through Cargo CLI configuration so both remote tests and local fallback receive it.

```sh
cargo-slot +1.97.1 test --manifest-path rust/Cargo.toml \
  --config 'env.BRAIN_HERO_FIXTURE.value="/tmp/public-hero-fixture/hero-data.json"' \
  --config 'env.BRAIN_HERO_FIXTURE.force=true' \
  --config 'env.BRAIN_CORE2_HERO_REPOSITORY.value="/tmp/public-deadlock-data-cache"' \
  --config 'env.BRAIN_CORE2_HERO_REPOSITORY.force=true' \
  -p brain-job --test hero --locked --jobs 3 -- --include-ignored
```
