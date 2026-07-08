# Analysis Notes

`deadlock_brain.analysis_notes` persists review contexts and later AI results.
It does not call a model.

## CLI

```bash
./rust/target/release/deadlock-brain analysis save-review Stalker --pretty
./rust/target/release/deadlock-brain analysis run-fireworks Stalker --dry-run --pretty
infisical run -- ./rust/target/release/deadlock-brain analysis run-fireworks Stalker --pretty
./rust/target/release/deadlock-brain analysis list --pretty
```

`analysis save-review` builds the same deterministic review context as
`deadlock-brain review`, then stores it in `analysis_notes` with:

- query
- matched entity type/name
- context kind
- context hash
- prompt version
- prompt text
- optional result text
- optional model and confidence
- source references
- full context JSON

This gives later model calls an audit trail: for every generated analysis we can
see exactly which data and prompt version were used.

## Fireworks

`analysis run-fireworks` builds the review context, sends it to DeepSeek through Fireworks'
the OpenAI-compatible `/chat/completions` API, strips hidden thinking blocks from
the model text, and stores the generated analysis as an `analysis_ready` note.

The adapter reads configuration from environment variables:

- `FIREWORK_API_KEY` or `FIREWORKS_API_KEY`
- `FIREWORK_BASE_URL` or `FIREWORKS_BASE_URL`, default `https://api.fireworks.ai/inference/v1`
- `FIREWORK_MODEL` or `FIREWORKS_MODEL`, default `accounts/fireworks/models/deepseek-v4-flash`
- `FIREWORKS_TIMEOUT_SECONDS`
- `FIREWORKS_MAX_TOKENS`
- `FIREWORKS_TEMPERATURE`
- `FIREWORKS_TOP_P`

`--dry-run` never calls the API. It is meant for request inspection and smoke
tests, and only reports whether an API key is present.
