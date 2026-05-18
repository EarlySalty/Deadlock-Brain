# Statlocker Source

Statlocker is an optional source for public meta signals. It is not included in
`pull all`, because these endpoints should be used deliberately and lightly.

## Commands

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind wpa-patches
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind wpa-items --patch patch_129989 --hero "Mo & Krill" --min-sample-size 50
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind leaderboard --leaderboard-page-size 100
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind leaderboard-player-matches --players-from-leaderboard 3 --matches-per-player 4
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind leaderboard-player-matches --players-from-leaderboard 2 --matches-per-player 3 --include-match-details --include-build-analysis
PYTHONPATH=src python3 -m deadlock_brain.cli player list-matches --pretty --limit 10
PYTHONPATH=src python3 -m deadlock_brain.cli player analyze-match <account_id> <match_id> --dry-run --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli player analyze-next --dry-run --pretty --limit 3
DRY_RUN=1 PLAYERS_FROM_LEADERBOARD=2 MATCHES_PER_PLAYER=2 ./scripts/run_player_match_learning.sh
infisical run -- PYTHONPATH=src python3 -m deadlock_brain.cli player analyze-match <account_id> <match_id> --pretty
infisical run -- ./scripts/run_player_match_learning.sh
```

Without `--patch`, the newest Statlocker WPA patch is used. Hero names are
converted to Statlocker format, e.g. `Mo & Krill` becomes `Mo_and_Krill`.

## Stored Data

- `statlocker_wpa_patch`: available WPA patch metadata.
- `statlocker_wpa_item`: item WPA rows for the selected patch/hero/rank filter.
- `statlocker_leaderboard_player`: one requested leaderboard page.
- `statlocker_player_profile`: targeted player profile snapshots.
- `statlocker_player_match`: concise recent matches for one player.
- `statlocker_match_detail`: one targeted match detail payload.
- `statlocker_player_build_analysis`: Statlocker's player/hero build analysis.

The build optimizer reads `statlocker_wpa_item` as a weak, correlational signal.
Positive WPA can lift an item slightly, negative WPA can lower it slightly, but
the deterministic hero understanding, shop economy and item mechanics still
remain the primary decision layer.

The player commands add a second learning lane. Leaderboard players are used as
observed high-skill samples, then MiniMax can turn one account/match into
reusable notes about item timing, standard core, adaptation and coaching rules.
This is still evidence, not truth: keep limits small, pull details only when
needed, and mark missing timestamps or team-comp data as uncertainty.

`scripts/run_player_match_learning.sh` is the cautious automation entrypoint.
Defaults are deliberately small: 3 leaderboard players, 3 matches each, match
details and build analysis enabled, then up to 5 MiniMax analyses. Set
`DRY_RUN=1` to store only contexts without calling MiniMax.

The automation does not require the `infisical` CLI. It follows the
Deadlock-Bots pattern: source
`/home/naniadm/.config/deadlock-bots/infisical.env`, call the Infisical HTTP API
with `scripts/export_infisical_env.py`, then inject the returned secrets into
the process environment. `MINIMAX_TOKEN_PLAN_KEY` is mirrored to
`MINIMAX_API_KEY` for compatibility, but the Brain uses the token-plan endpoint
when that key is present.

The automation protects against hanging MiniMax calls with
`ANALYSIS_TIMEOUT_SECONDS` (default 600 seconds per task, `0` disables it). Output tokens
are not used as a cost brake; `MINIMAX_MAX_COMPLETION_TOKENS` is only the
technical ceiling required by the API.

## Notes

WPA is not causal. Treat it as a meta hint:

- Good: "This item appears to perform well for this hero in this patch/sample."
- Not enough: "This item is automatically correct."
- Risk: small hero/item samples can be noisy, especially right after patches.
