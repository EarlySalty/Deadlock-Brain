# N-MESSUNG: Livebeweis nach der Pipeline-Vereinheitlichung

Stand 2026-09-16. Worktree `~/.worktrees/deadlock-brain-n`, Branch
`feat/reasoner-planner-produktiv`. Debug-Binary aus dem Worktree, kein
`--release`. Zentrale DB nur lesend (`export_gpt_secret.py --secret
DEADLOCK_CENTRAL_DSN`, `PGOPTIONS='-c default_transaction_read_only=on'`),
Aufrufe mit `--no-ai --no-persist`. Referenzwaffen-Sets je Held aus der
M-Messung (`M-SECHS-pop.json`), Warden-Set die neun Referenzwaffen aus dem
Lightbringer-Seed.

## Warden (9 Referenzwaffen)

- Kern (16): Extended Magazine, Titanic Magazine, High-Velocity Rounds, Opening
  Rounds, Extra Spirit, Improved Spirit, Swift Striker, Quicksilver Reload,
  Fleetfoot, Rusted Barrel, Mystic Regeneration, Mercurial Magnum, Burst Fire,
  Glass Cannon, Spiritual Overflow, Healing Tempo.
- Referenzwaffen live: **6/9** (Opening Rounds, Spiritual Overflow, Titanic
  Magazine, High-Velocity Rounds, Fleetfoot, Swift Striker). Verfehlt: Frenzy,
  Blood Tribute, Monster Rounds.
- `reason backtest --hero Warden` bewertet dasselbe Build: staple_count 10,
  present 9, fehlend nur Enduring Speed (Item 2447176615), Staple-Gate nicht
  bestanden, kendall_tau 0,577, jaccard@12 0,500. Konsistent zum Build und zur
  M-Deutung (der Kontext-Marginal-Gate lehnt das anti-synergetische Enduring
  Speed bewusst ab).
- Ziel mindestens 5/9 erfüllt (6/9), identisch zur M-Messung gegen FROZEN-V2.

## Sechs Helden gegen die M-Nachher-Werte

| Held | live | M-nachher | Delta | verfehlt live |
|---|---|---|---:|---|
| Infernus | 5/7 | 5/7 | +0 | Ricochet, Toxic Bullets |
| Lady Geist | 1/7 | 2/7 | -1 | Cultist Sacrifice, Monster Rounds, Berserker, Recharging Rush, Restorative Shot, Kinetic Dash |
| Abrams | 4/7 | 3/7 | +1 | Stalker, Crushing Fists, Monster Rounds |
| Vindicta | 2/3 | 2/3 | +0 | Alchemical Fire |
| Bebop | 0/4 | 0/4 | +0 | Stalker, Capacitor, Tesla Bullets, Crippling Headshot |
| Ivy | 2/3 | 2/3 | +0 | Tesla Bullets |

Keine Regression über eine Waffe: Lady Geist -1, Abrams +1, der Rest deckungs-
gleich. Warden bleibt bei 6/9.

## Ursache der ±1-Abweichungen (FROZEN-V2 gegen Live-DB)

Die M-Nachher-Werte sind gegen `FROZEN-V2.json` (Snapshot vom 2026-09-13)
gemessen, der Livebeweis gegen die aktuelle zentrale DB. Die Pipeline ist
identisch (beide `plan_build` -> `compose_build_with_sources` -> Planner); die
Abweichung stammt aus dem Snapshot-Drift der Eingaben, nicht aus dem Code. Beleg
dafür ist auch der Warden-Kendall-tau (live 0,577 gegen frozen 0,464 bei
gleichem jaccard 0,500 und gleichem fehlenden Staple): die Median-Kaufpositionen
und Autorenbuilds der Live-DB liegen anders als im eingefrorenen Stand.

- Lady Geist: Monster Rounds und Kinetic Dash liegen in der Live-DB knapp unter
  der Kostenband-Auswahlschwelle des Planners; im frozen Stand gewann Kinetic
  Dash sein Band. Snapshot-Drift der Score-Eingaben, kein Populations-Staple
  (Lady Geist trägt in der Population wenige Staples ab 70 %).
- Abrams: Berserker gewinnt in der Live-DB zusätzlich sein Kostenband (Combat-
  Marginalwert über der Schwelle), daher +1.

## Laufzeit je Held (Debug, zentrale DB)

Warden 24 s, Infernus 25 s, Lady Geist 39 s, Abrams 71 s, Vindicta 22 s,
Bebop 43 s, Ivy 29 s. Alle unter der 120-s-Grenze; Abrams mit 71 s am oberen
Rand, kein Blocker.

## Tests und Lint

- `cargo test -p dbrain-reasoner --lib`: 170 bestanden, 16 ignoriert, 0 rot.
- `cargo test --workspace` ohne DSN: 386 bestanden, 58 ignoriert, 0 rot
  (Baseline gehalten).
- `cargo clippy -p dbrain-reasoner -p deadlock-brain --all-targets -- -D
  warnings`: sauber.
