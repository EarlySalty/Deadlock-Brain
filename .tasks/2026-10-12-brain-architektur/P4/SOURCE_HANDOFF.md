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

Rows whose title/url/raw_content byte sum exceeds the configured transport cap fail the source explicitly before capture. The vector-based frozen Core contract requires complete bytes in memory. Do not claim lossless capture of arbitrarily large rows. Malformed content within this cap, including NULL title/url, ID 0, BC/infinite timestamps and values above the Core 4-MiB raw limit, is captured before extractor/Core rejection; following valid posts remain processable.

## Measured proof

Worst bounded workload: four rows, each exactly 8388608 source-text bytes, maximal sixfold JSON escaping to roughly 48 MiB per envelope, a 200-ms consumer and real loader-to-Core PostgreSQL writes. Peak client process VmHWM was 258748416 bytes, below the unchanged 300000000-byte requirement. This excludes the compiler and PostgreSQL process footprint. Raw objects were 4 then 4, delta 0; both runs honestly rejected all four oversized objects.

A separate 132-post scratch run wrote 132 raw objects. Repeating it left 132, delta 0. First run: 131 accepted, 1 ignored, 0 rejected. Repeat: 0 accepted, 132 ignored, 0 rejected.

Earlier real peaks of 359378944 and 309030912 bytes failed the requirement. Removal of envelope queueing and producer prefetch, plus release of row storage and excess vector capacity, fixed the cause. The threshold was not increased.

## Verification

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
