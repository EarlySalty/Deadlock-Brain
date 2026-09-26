# Wiki parser and contract adapter

The productive S12 parser now belongs to the main Rust workspace at `rust/crates/dbrain-wiki`. Its Cargo package, Rust import and CLI names remain `dbrain-s12-wiki-probe`, `dbrain_s12_wiki_probe` and `dbrain-s12-wiki-probe` for compatibility. There is no second contract crate or second publisher.

`knowledge::extract` creates a trusted local `WikiIr` wrapper. `WikiIr::contract()` exposes `brain_contracts::source::Versioned<brain_contracts::wiki::WikiIr>` (`brain.ir.v1`). All semantic fields, source namespace, raw-source provenance, conditions, variants, aliases, dependencies and missingness belong to `brain-contracts`. Only capture diagnostics and trusted extraction/projection operations remain here. Deserializing a shared DTO is not a projection review or an authorization.

The CLI `extract` output now contains `report` and `contract`; semantic fields are under `contract.data`, with the required `contract.contract_version`. The historical flat `wiki-ir-v1` dump was diagnostic output, not an accepted import format. Existing leaf serialization, field IDs, mapping review pins and the reviewed card golden are retained. The private trusted wrapper has no `Deserialize` implementation.

`project_card` remains loss-aware: the old `brain.v1::Fact` cannot carry source conditions or variants. Such fields stay in the common IR and are not flattened into unconditional facts. C5/C6 must consume the shared IR and supply their own explicit domain/release validation; this move does not implement it.

## Checks

From the repository root:

```sh
cargo +1.97.1 test --manifest-path rust/Cargo.toml -p dbrain-s12-wiki-probe --all-targets --locked
RUSTUP_TOOLCHAIN=1.97.1 bash architecture/migration/s12/check.sh
```

The full workspace checks now include this package. The historical check entry point resolves the new workspace manifest and respects `CARGO_TARGET_DIR`.

## Retained evidence

Historical fixtures and reports remain byte-for-byte under `architecture/migration/s12/{fixtures,reports}`. Tests use those files explicitly; productive parser code does not depend on that documentation directory. The old standalone manifest is retained as `Cargo.toml.historical` beside its historical lockfile. It is not an active workspace or dependency.

The original source paths and hashes in old reports describe their original commits, not this refactor. They remain recoverable through Git at the C4 base `087c522` and earlier commits. Current implementation and tests have moved with their Git history; the stored reports have not been regenerated to conceal the move.
