# Patch history answers for "war das nicht mal" follow-ups

## Incident

2026-10-29 07:30, request `dl-bot-discord-781be0f389c8293106f45e03db3e84a1-3`.
DM follow-up "War das nicht mal 60/40? Dachte das wäre immer noch so" after a question about trooper souls at minute 0/1.
One tool call, six value-gate rejections (`unsupported_numbers=2`), `insufficient_evidence` after about 55 s, bot silent.

Correct answer: 11-21-2025 Update (21.11.2025) reduced the flying orb share from 50 % to 40 % (60 % drops to the ground).
Minor Update 06-30-2026 (30.06.2026) changed it back from 40 to 50, so 50/50 since then.

## Root causes

1. Patch extraction only kept hero sections and subject-prefixed lines, so `[ General ]` lines like the orb split never reached canon.
2. The Discord DM context resolver in brain-serve returned empty turns, so follow-ups lost their topic.
3. The value gate rejected German dates of cited patches and the 100-x complement of a cited split.
4. Identical rejections were retried until the round limit.

## Changes

- brain-job `patchnotes` extractor v6 emits `patch_note_<line>_<piece>` facts for non-hero bullet lines.
- `patch_history` takes an optional `text` search across all patches and sections (terms ranked, then date).
- brain-serve DM context projects prior questions to a fixed list of public game terms.
- Value gate: German dates of cited patches; the 100-x complement of a cited percentage only in split notation next to it (40/60). Numbers from the question stay unsupported unless evidenced.
- Kernel stops retrying when the unsupported set is unchanged from the previous round.

## Verification

See REPORT.md once deployed.
