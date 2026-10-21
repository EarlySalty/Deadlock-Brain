# P4 source handoff

Verified component: `5685f7e72a5cde9fafb748d0565a2286146544d9`, branch `worktree-agent-ae1a42b9495225b9d`, clean and published to origin. Source version 2 and schema `c4962120` must be integrated together. This is not a live deployment.

## Behavior and configuration

The source registry routes `deadlock_patchnotes_db` to `crate::patch_extract::source::load`. Root brain-job lib.rs is unchanged. Extractor and contract file remain 413 + 386 = 799 lines. Version 2 emits the reference-fingerprint key without changing payload, entity or receipt semantics.

The loader requires `patchnotes_feed_reader`, verifies read-only privileges, uses a read-only transaction and fetches one cursor row only when Iterator.next requests it. A zero-capacity rendezvous channel eliminates queued envelopes. Drop, cancellation and server/client timeouts bound shutdown. Credentials use the existing Infisical transport. There are no new credential/token files or secret-bearing diagnostics. Authority validation rejects apparent credentials inside a URL path before SQLx parsing.

The source path is the non-secret JSON configuration file at `source_cache.join("deadlock_patchnotes_db")`, not a feed directory. Required fields:

| Field | Valid values |
| --- | --- |
| infisical_config | Absolute path to the existing Infisical configuration |
| max_posts | 1 to 5000 |
| max_row_bytes | 4195328 to 8388608 |
| timeout_seconds | 1 to 60 |

The worker recommends max_posts 5000, max_row_bytes 8388608 and timeout_seconds 15. The existing secret-manager configuration must supply `PATCHNOTES_FEED_READONLY_DSN`. No production configuration was created or installed.

Rows whose title/url/raw_content byte sum exceeds the configured transport cap are skipped inside the P4 source before reaching the Core iterator. The orchestrator explicitly approved this interim policy on 2026-10-13 and deferred a Core-level nonfatal item report until after P1r is live. SQL returns ID and actual byte-size metadata without transferring oversized text. Each skipped post emits one JSON line with event `patchnotes_source_skip`, source, post_id, reason `transport_row_too_large`, byte_size and cumulative skipped_oversized_posts. Successful source completion emits a `patchnotes_source_summary` with scanned_posts, emitted_posts and skipped_oversized_posts. A demand request continues fetching past skipped rows until the next bounded post or EOF, so skips do not consume a rendezvous response or leave the iterator waiting for another request. The existing max_posts bound counts scanned rows, including skips.

No raw object is fabricated for an oversized transport row. Do not claim lossless capture of arbitrarily large rows or a Core RunReport entry for these skips. Malformed content within the cap, including NULL title/url, ID 0, BC/infinite timestamps and values above the Core 4-MiB raw limit, is still captured before extractor/Core rejection. Infrastructure, role, timeout and configured scan-count errors remain fatal.

## Measured proof

Worst bounded workload: four rows, each exactly 8388608 source-text bytes, maximal sixfold JSON escaping to roughly 48 MiB per envelope, a 200-ms consumer and real loader-to-Core PostgreSQL writes. Peak client process VmHWM was 258748416 bytes, below the unchanged 300000000-byte requirement. This excludes the compiler and PostgreSQL process footprint. Raw objects were 4 then 4, delta 0; both runs honestly rejected all four oversized objects.

A separate 132-post scratch run wrote 132 raw objects. Repeating it left 132, delta 0. First run: 131 accepted, 1 ignored, 0 rejected. Repeat: 0 accepted, 132 ignored, 0 rejected.

Earlier real peaks of 359378944 and 309030912 bytes failed the requirement. Removal of envelope queueing and producer prefetch, plus release of row storage and excess vector capacity, fixed the cause. The threshold was not increased.

## Latest scoped verification, 2026-10-13

Assigned parent manifest: `/home/nathanael/.worktrees/Deadlock-Brain-p4-patches/rust/Cargo.toml`. All compiler commands use existing cargo-slot, pinned +1.97.1, --locked and --jobs3. No owner/Core code or migrations changed.

- Source tests: `cargo-slot +1.97.1 test --manifest-path <manifest> --locked --jobs 3 -p brain-job --lib patch_extract::source::tests -- --include-ignored --nocapture --test-threads=1`, exit0 on ionos. Six passed, zero failed/ignored, one unrelated lib test outside the filter. Log: `/tmp/p4-oversize-source-tests-20261013.log`.
- Existing ingestion/extractor contracts: same launcher/manifest with `--test ingest --test patch_extract_contract -- --include-ignored --skip cached_hero_source_yields_one_unchanged_original_blob --nocapture --test-threads=1`, exit0 on ionos. Three plus nine passed, zero failed/ignored; the unchanged optional Core2 hero-cache fixture remains explicitly filtered under the earlier scope instruction. Log: `/tmp/p4-oversize-ingest-extract-tests-20261013.log`.
- Check and Clippy: `-p brain-job --all-targets`, Clippy additionally `-- -D warnings`; both exit0 on ionos. Logs: `/tmp/p4-oversize-check-20261013.log`, `/tmp/p4-oversize-clippy-20261013.log`.
- Formatter check and diff whitespace check exit0. Extractor/contracts remain799 lines. Executable build: same launcher/manifest with `-p brain-job`, exit0 locally in verify profile; log `/tmp/p4-oversize-build-20261013.log`.

Fresh affected test total:18 passed,0 failed,0 ignored,2 filtered. The new single regression captures actual source stderr from the real PostgreSQL/Core fixture and verifies exactly one structured record per skipped post, exact UTF-8 sizes8388615/8388609, IDs1/3, reason and cumulative counts1/2; the summary is scanned4/emitted2/skipped2. Later bounded posts2/4 are accepted, the run succeeds and raw storage contains exactly those paths.

The updated worst-bounded workload measured client peak254627840 bytes, below300000000, and repeat raw4 to4, delta0. The existing132-post repeat remains covered. This is scratch verification, not a productive run or Core-level skipped-item report.

## Core3 composition checks, 2026-10-13

The parent normally composed approved19dec0c9 and then losslessly restarted its uncommitted merge at approved982645cf, preserving P4's migration4 alongside reserved core7. Core3a has no production/migration changes; final P4 adapter patches do not alter source/storage paths. The canonical adapter and owner fixture runtime are now verified separately in RUNTIME_VERIFICATION.txt. Remaining production owner coherence and final gate are still open; these source/schema results are not ALLOW or live acceptance.

Source tests on the composed Core3 tree:6 passed/0 failed/0 ignored,1 unrelated lib test outside the source filter. Real oversize JSON/count/continuation proof remains green; new worst-bounded client peak260386816 bytes and raw4→4, delta0. Log `/tmp/p4-core3-source-tests-20261013.log`. Compiler check of brain-job/storage/contracts/serve with --tests exited0; log `/tmp/p4-core3-merge-check-20261013.log`.

Fixture-backed tests used existing public paths with dotted Cargo env config: `env.BRAIN_HERO_FIXTURE.value="/tmp/p1r-hero-a314-fixture-GInzyzvn/hero-data.json"`, force=true; `env.BRAIN_CORE2_HERO_REPOSITORY.value="/tmp/brain-core2-cache-wf-7f1dca25-SbrUuw"`, force=true. Pinned Git fixture2a26e26535a390c309e300e2c7b2763fa5ad7abd was verified. Normal cargo-slot test with --locked --jobs3, --include-ignored --nocapture --test-threads=1 and targets ingest/hero/patch_extract_contract passed4+7+9,0 failed/ignored/filtered. This time the existing optional hero/cache proofs actually ran. Log `/tmp/p4-core3-ingest-hero-extract-fixture-20261013.log`.

Sparse registry tests passed3/0/0 (44 unrelated unit tests outside the filter), and P4 upgrade/idempotence test passed1/0/0 with versions[1,4,7], unchanged fact/relfilenode, null unproven patch history, FK/identity guards and unchanged grants. Logs `/tmp/p4-core3-registry-tests-20261013.log` and `/tmp/p4-core3-patch-migration-tests-20261013.log`. Fresh source/ingest/hero/extractor/registry/P4-upgrade total30 passed,0 failed,0 ignored,45 disclosed filters.

The first general hero attempt lacked BRAIN_HERO_FIXTURE on ionos:5 passed/2 failed, not a code flake or green proof. Subsequent inspection of its retained log shows the local fallback also completed with5 passed/2 failed and exit101 despite the attempted stop; it was not a zero-result run. The cause was fixed with the documented existing public fixtures, and only this exact workspace's empty fallback marker was cleared before the successful normal remote rerun. Earlier log `/tmp/p4-core3-ingest-hero-extract-20261013.log` is retained. No production secret, real feed read, external AI call or service change was involved.

Composed brain-job/storage Clippy with --all-targets -- -D warnings exited0 through the normal remote wrapper; log `/tmp/p4-core3-job-storage-clippy-20261013.log`. The brain-job executable build from the same job exited0 locally in verify profile; log `/tmp/p4-core3-job-build-20261013.log`. Background job bc3f7oezz completed with exit0.

## Earlier component verification

Manifest: `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/agent-ae1a42b9495225b9d/rust/Cargo.toml`.

Executed through `/home/nathanael/.local/bin/cargo-slot`, each with that manifest, `--locked --jobs 3`:

- test `-p brain-job -- --test-threads=1 --nocapture`: 18 passed, 0 failed, 1 existing ignored. Breakdown: source/lib 6, normal ingest 3 plus one unrelated ignored Core2 hero-cache fixture, extractor contract 9.
- clippy `-p brain-job --all-targets -- -D warnings`: exit 0.
- check `-p brain-job`: exit 0.
- build `-p brain-job`: exit 0.

Scoped rustfmt checks of all five owned files and the staged diff whitespace check passed. No source changed after the green run. Parent independently read the actual success/count/RSS markers.

Evidence directory: `/tmp/claude-1000/-home-nathanael--worktrees-Deadlock-Brain-p4-patches/58c3ac6f-3049-4932-93a8-78e1ab4d2a1e/tasks/`.

- Green test/clippy/check/build/RSS: `by2gkyu72.output`.
- Red memory attempts: `b4o4wn4im.output`, `bsg0u5wn8.output`.
- Optional activated ignored Core2 cache attempt: `b2xvl3f4o.output`, failed because BRAIN_CORE2_HERO_REPOSITORY was absent. Not a green baseline or a flake; not repeated because that untouched Core2 proof was already accepted by the orchestrator.

No main merge, gate ALLOW, deployment, timer activation, restart or live proof is claimed.
