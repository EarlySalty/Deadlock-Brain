# Parry on objectives: operator in-game observation

Recorded 2026-10-30 from the operator's in-game test on the current game build (after the 29.09.2026 "City Never Sleeps" patch).

## Observed

1. A successful parry on the Guardian or another objective disables it like a stun.
2. A successful parry on an objective does not reset the parry cooldown. This matches the patch line of 29.09.2026 "Successfully parrying objectives no longer resets your Parry cooldown".

## Superseded stored source

- 01.08.2024 "Parrying a Guardian attack will no longer cause the Guardian to be stunned" (`game-wiki/pages/deadlock-data/patchnote.md`, 08-01-2024 Update). Observation 1 contradicts it. Open: no stored source backs observation 1 yet, so get_mechanic does not state it and still returns this dated line. Stun duration and damage bonus on objectives are not verified.

## Still valid stored sources

- 22.08.2024 "Parrying a Guardian now causes it to stop trying to repeatedly melee ... and instead causes the boss to start shooting shortly there after". Compatible with observation 1.
- 28.05.2026 "Parrying a trooper now stuns the trooper" and "Parrying a trooper no longer resets your parry cooldown".
- 17.04.2025 "Players can no longer Parry troopers", later superseded by the 28.05.2026 lines.
