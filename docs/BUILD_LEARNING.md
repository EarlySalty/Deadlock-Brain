# Build Learning

`deadlock_brain.build_learning` is the first self-learning layer for builds.
It treats popular Steam/GC builds as weak labels, not as truth.

## Commands

```bash
./rust/target/release/deadlock-brain learn import-steam-builds --pretty --limit-per-hero 10
./rust/target/release/deadlock-brain learn list-builds --hero Haze --pretty
./rust/target/release/deadlock-brain learn analyze-build 28 --dry-run --pretty
./rust/target/release/deadlock-brain learn analyze-next --dry-run --pretty --limit 3
infisical run -- ./rust/target/release/deadlock-brain learn analyze-build 28 --pretty
```

For normal repeated runs use the wrapper:

```bash
DRY_RUN=1 LIMIT=3 ./scripts/run_build_learning.sh
infisical run -- ./scripts/run_build_learning.sh
HERO=Haze LIMIT=2 infisical run -- ./scripts/run_build_learning.sh
```

The wrapper imports the newest Steam/GC builds from the Bot DB first, then
analyzes the next builds without an `analysis_ready` note for the active model
and prompt version. `DRY_RUN=1` stores only auditable context notes and does not
call Fireworks.

## Automation

For a simple scheduled run, call the wrapper from cron or a user systemd timer
inside the same environment that provides `FIREWORKS_API_KEY`.

Example cron entry for every two hours:

```cron
0 */2 * * * cd /home/naniadm/Documents/Deadlock-Brain && infisical run -- ./scripts/run_build_learning.sh >> /home/naniadm/Documents/Deadlock-Brain/data/build_learning.log 2>&1
```

Useful environment switches:

- `LIMIT=5`: how many pending builds Fireworks should analyze per run.
- `HERO=Haze`: restrict import and analysis to one hero.
- `DRY_RUN=1`: test the full queue without calling Fireworks.
- `IMPORT_LIMIT_PER_HERO=10`: how many top Steam/GC builds to sync per hero.
- `DELAY_SECONDS=2`: pause between real Fireworks calls.

## Stored Data

- `learned_builds`: imported Steam/GC builds, source rank, quality label,
  item list, item categories, ability order and raw source metadata.
- `build_learning_notes`: model contexts and later Fireworks analyses for audit.

## Label Assumption

Steam/GC ranking is useful but noisy:

- rank 1-3: `likely_good`
- rank 4-10: `usable_noisy`
- lower ranks: `low_confidence`

The model prompt asks DeepSeek via Fireworks to explain why a build works, what variant it
represents, which items are lane/core/late/situational, and which rules the
Brain should learn. It must keep English game names and write German analysis.

## Future Sources

The same storage shape can take:

- curated Discord exports
- Reddit discussion notes
- manual seed builds
- Fireworks-generated critique

Discord/Reddit should be imported as cited discussion snippets, not blindly as
facts. They are useful for hypotheses, matchup notes and build variants.
