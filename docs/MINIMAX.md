# MiniMax Integration

The MiniMax adapter is intentionally small: it turns the deterministic
`review` context into a chat-completions request, sends it to MiniMax, and
stores the answer in `analysis_notes`.

## Commands

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli analysis run-minimax Indomitable --dry-run --pretty
infisical run -- PYTHONPATH=src python3 -m deadlock_brain.cli analysis run-minimax Indomitable --pretty
```

The dry-run command does not call MiniMax. The real command expects
`MINIMAX_API_KEY` or `MINIMAX_TOKEN_PLAN_KEY` in the environment, so Infisical
can inject it without the tool printing the secret.

## Prompt Contract

The system prompt tells the model to:

- use only the provided context
- keep Item, Hero, Ability, Stat, and source names in English
- write the analysis in German
- mark uncertainty and missing data clearly
- never invent winrates, pickrates, or patch details

The user prompt asks for:

- Kurzfazit
- Patch-Verlauf and relevant Reworks/Renames
- current assessment from the data
- Build-/Gameplay implications
- uncertainties and open points

## Storage

Successful calls are stored as `analysis_ready` rows in `analysis_notes`.
Provider metadata such as model id, response id, usage, and safety fields is
kept as a `model_call` source reference. The API key is never stored.
