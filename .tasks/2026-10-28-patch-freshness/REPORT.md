# Patch freshness report

Measured on 2026-10-29 against production (`brain` database, release 4c5ebb27) for patches published since 2026-08-01.

## Stages

| Stage | Meaning | n | Median | Max |
|---|---|---:|---:|---:|
| detect | patch published to first `deadlock_patchnotes_db` capture (or legacy detection) | 14 | 115.6 h (6937.9 min) | 20.4 d (29395.2 min) |
| upstream | patch published to first `deadlock-data` commit with a client build | 14 | 11.0 min | 22.3 h (1340.6 min) |
| ingest | upstream commit to first clean `deadlock-data` ingest run on that revision or later | 14 | 8.8 d (12645.5 min) | 61.9 d (89175.8 min) |
| total | patch published to game data live | 14 | 8.8 d (12682.3 min) | 61.9 d (89185.6 min) |
| silent_ingest | client update without patch notes, commit to ingest | 44 | 12.0 d (17286.0 min) | 73.9 d (106366.8 min) |

Source: `brain-job --config /etc/deadlock-brain/brain-job.json freshness [--since <date>]`, backed by the views `ops.patch_freshness` (one row per patch) and `ops.game_data_freshness` (one row per upstream client update, `silent` marks updates without patch notes).

## Reading the numbers

- **detect**: the healthy value is 1.4 to 3.1 min (patches 283 to 285). The large values from 2026-09-29 to 2026-10-20 are pipeline gaps, not latency: the legacy detector stopped on 2026-10-11 and the Rust patchnotes source only runs since 2026-10-23, so these patches were first captured days later.
- **upstream**: depends on the external deadlock-data repository (Deadbot). Usually about 10 min; outliers of 1.1 h, 1.5 h and 22.3 h (patch 293). Not avoidable on our side.
- **ingest**: the avoidable part. Before this change `deadlock-data` had no timer and ran only by hand, so every update waited days to weeks. Since 2026-10-28 `brain-job@deadlock-data.timer` runs every 5 min (`*:2/5:00`), the unit lives in `ops/brain-release/systemd` and is installed by `brain-release`. A no-change run takes about 4 s, so new ingest lag is bounded by about 5 min.
- **total**: historical values are dominated by the ingest gap. Expected for the next patch: detect up to 5 min, upstream about 10 min, ingest up to 5 min, so about 15 to 20 min end to end.

## Next real patch

No patch was published during the run (latest listed patch: 297, 2026-10-20 20:59 UTC). The end-to-end number for the next patch appears automatically in `ops.patch_freshness` and the report command once its first clean ingest run finishes.

## Data kept

- `ops.upstream_commit`: 516 deadlock-data commits backfilled (2024-10-02 to 2026-10-26).
- `ops.ingest_run_revision`: revision per deadlock-data run; backfilled runs are marked `inferred`.
- `ops.patch_detection_legacy`: legacy detection times by publish time.
- `ops.patch_observation`: which raw patch object each patchnotes run saw, recorded inside the ingest lock, so `ops.patch_listing` follows the current upstream version without the published-receipt function.
