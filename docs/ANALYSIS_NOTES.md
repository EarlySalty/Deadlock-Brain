# Analysis Notes

`deadlock_brain.analysis_notes` persists review contexts and later AI results.
It does not call a model.

## CLI

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli analysis save-review Stalker --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli analysis run-minimax Stalker --dry-run --pretty
infisical run -- PYTHONPATH=src python3 -m deadlock_brain.cli analysis run-minimax Stalker --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli analysis list --pretty
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

## MiniMax

`analysis run-minimax` builds the review context, sends it to MiniMax through
the OpenAI-compatible `/chat/completions` API, strips hidden thinking blocks from
the model text, and stores the generated analysis as an `analysis_ready` note.

The adapter reads configuration from environment variables:

- `MINIMAX_API_KEY` or `MINIMAX_TOKEN_PLAN_KEY`
- `MINIMAX_BASE_URL`, default `https://api.minimax.io/v1`
- `MINIMAX_MODEL`, default `MiniMax-M3`
- `MINIMAX_TIMEOUT_SECONDS`
- `MINIMAX_MAX_COMPLETION_TOKENS`
- `MINIMAX_TEMPERATURE`
- `MINIMAX_TOP_P`

`--dry-run` never calls the API. It is meant for request inspection and smoke
tests, and only reports whether an API key is present.
