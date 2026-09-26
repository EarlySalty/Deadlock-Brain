# Frozen v1 storage fixture (C11)

This is synthetic data in the **historical v1 wire format**, not a production dump and not a serialization of today's Rust structs.

- `schema.sql` is byte-for-byte `scripts/migrations/2026-09-24-brain-contract-v1.sql` at commit `f18502f46fcb403fa5998c550a3191efc29e9544` (24 September 2026). The schema was introduced by `1db885d3b2a4a200c80385d3602ce74c17f8562b` and was unchanged by `f18502f`.
- The historical `CONTRACT_VERSION` at `f18502f` is `brain.v1`; its `CorpusRelease.source_revisions` already contains per-document pins. The type named `SourceRecordV2` existed even in that v1 contract. Its name is not a database schema version.
- `data.json` freezes that corrected v1 representation: three source identities, eleven revisions, seven heads, two releases, UTF-8/newlines, unequal per-document revision counters, source/parser revisions and locators, patch/mode/language/unit metadata, public/internal/private ACLs, multi-scope restrictions, ACL tightening after publication, deny-by-default private data, a no-egress record, and a tombstone.
- A fact-shaped document body is preserved as opaque text. There was no persisted Fact table, source registry, job/lease table, or checkpoint table in this v1 schema. Inventing one would test a fictional upgrade. Source identities live in records; ACL/provenance live in `record_json`. The integration test asserts the actual three-table inventory, then verifies empty new checkpoint tables and preservation/resume after v2 writes and repeated migrations.
- `SHA256SUMS` pins both files. Content hashes are separately recomputed from exact UTF-8 content bytes in Rust. The migrator does not rehash arbitrary legacy data or assume every historical hash label used this algorithm.

## Earlier, ambiguous v1 generation

At `1db885d`, `CorpusRelease.source_revisions` was `BTreeMap<String,u64>`: only the **maximum revision per source**. Revision counters belong to individual documents, so this does not identify an exact historical corpus. `f18502f` corrected it to nested per-document pins without changing SQL.

The scratch test also persists this earlier shape (`{"wiki":4,"docs":3,"private":1}`). The migrator must reject it atomically and leave the original data/schema untouched. It must not manufacture document pins from today's heads, timestamps, or maximum revisions. Such a deployment needs an independently verified original release manifest and an explicitly reviewed repair before upgrading. This fixture does not claim that those missing pins can be recovered from the old maximum alone.

## Reproduction

From the repository root, as a non-root user with PostgreSQL tools and the workspace Rust toolchain:

```sh
bash scripts/test_brain_storage_upgrade.sh /path/to/cargo
```

The runner creates a unique Unix-socket-only PostgreSQL cluster under `/tmp/brain-c11.*`, proves the connection identity and data directory before writes, creates fresh databases, executes the real migration binary, and removes only its own cluster on exit. It accepts no database address or DSN. The opt-in Rust test never silently skips a missing cluster.
