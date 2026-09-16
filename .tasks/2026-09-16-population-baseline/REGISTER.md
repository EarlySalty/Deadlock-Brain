# Register: population-baseline

Intent: Claude-Session Fable (kein T3-Thread, Nutzervorgabe: Opus-Subagenten).
Stufe groß, Worker-Modell Opus 4.8 (`opus48-coder`). Status: abgeschlossen 2026-09-16.

## Worker-Register (Claude-Subagenten)

| Paket | Datum | Modell | Worktree | Branch | Status | Ergebnis |
|---|---|---|---|---|---|---|
| W Welle-2-Audit | 2026-09-16 | opus48 | keiner (lesend) | keiner | fertig | `WELLE2-AUDIT.md`: fc71b5d Fast-Forward, grün, Warden 3/9; Merge-Gate BLOCK wegen verfehlter eigener Abnahme, Welle 2 erst mit M gemergt |
| P Population-Crate | 2026-09-16 | opus48 | ~/.worktrees/deadlock-brain-population | feat/population-baseline | fertig (42d11ec) | `FERTIG-P.md`: 22 Tests, 10k-Ingest 478 s, Warden 10 Staples |
| Review P R1 | 2026-09-16 | opus48 | keiner | keiner | fertig | `REVIEW-P.md`: FREIGABE, 5 Nits |
| M Messlatte + Warden | 2026-09-16 | opus48 | ~/.worktrees/deadlock-brain-m | feat/build-reasoner-population (von fc71b5d, P gemergt) | fertig (23e0935) | `FERTIG-M.md`: Warden 3/9 auf 6/9 |
| Review M R1 | 2026-09-16 | opus48 | keiner | keiner | fertig | `REVIEW-M.md`: NACHBESSERN, 4 wichtig, 3 Nits |
| Fix M R1+R2 | 2026-09-16 | opus48 | ~/.worktrees/deadlock-brain-m | feat/build-reasoner-population | fertig (be9a982) | Prior produktiv, Timer-Schritte, P-Nits, Staple-Hebel am Kontext-Marginalwert; Warden 6/9, Staple-Gate fällt wegen Enduring Speed (Mobilität unmodelliert) |
| Review M R2 | 2026-09-16 | opus48 | keiner | keiner | fertig | `REVIEW-M-R2.md`: FREIGABE, 2 Nits, 1 Abnahme-Punkt |
| Merge 1 | 2026-09-16 | Fable | Haupt-Checkout | main | fertig | P 42d11ec (Gate ALLOW, 5 Nits), Welle 2 plus M 7277ff5 (Gate ALLOW); Nachweis-JSONs nach `~/.local/share/deadlock-brain/nachweise/`, Bundle `reasoner-branches-20260916.bundle`, 13 Worktrees und 12 Branches gelöscht (`BRANCH-BACKUP.md`) |
| Deploy 1 | 2026-09-16 | Fable | Haupt-Checkout | main 7277ff5 | fertig | Release 57d3a07a, Migration `2026-09-16-population.sql` zentral, Ingest 118572 Spieler-Matches / 10000 Matches / 38 Helden, Timer-Oneshot Result=success mit Populations-Schritten |
| N Planner produktiv | 2026-09-16 | opus48 | ~/.worktrees/deadlock-brain-n | feat/reasoner-planner-produktiv | fertig (da17d7b) | `FERTIG-N.md`, `N-BEFUND.md`, `N-MESSUNG.md`, `N-AUSREISSER.md`: Planner war schon produktiv, Doppelcode CLI/Example gebündelt; Ausreißer 04:34 = KI-Recompose blockte per Teilstring erwähnte Items, gefixt; Gate- und Review-Nits erledigt |
| Review N R1+R2 | 2026-09-16 | opus48 | keiner | keiner | fertig | `REVIEW-N.md` NACHBESSERN (Diagnose), `REVIEW-N-R2.md` FREIGABE ohne Mängel |
| Merge 2 + Deploy 2 | 2026-09-16 | Fable | Haupt-Checkout | main da17d7b | fertig | Gate ALLOW (Merge und Push), Release 84a0c998, Livebeweis: drei KI-Läufe `reason build Warden` zeichengleich 6/9, Backtest tau 0,577 Jaccard@12 0,50, persistierte Zeile in `brain.reasoner_builds` ist das gute Build; Worktree und Branch gelöscht |

Status-Werte: geplant, läuft, fertig, gestoppt, gebumpt.

## Endstand

- Warden: 6 von 9 Referenzwaffen (High-Velocity Rounds, Opening Rounds, Swift
  Striker, Titanic Magazine, Fleetfoot, Spiritual Overflow), 9 von 10
  Populations-Staples, Maßstab mindestens 5/9 erfüllt. Offen: Enduring Speed
  (Mobilität in der Combat-Bewertung unmodelliert), Frenzy (kein Staple, im
  T4-Band verdrängt), Monster Rounds (Farm-Nutzen unmodelliert), Blood Tribute
  (negativer Mechanikwert).
- Sechs Helden ohne Regression (`N-MESSUNG.md`), Ivy ohne Staples ab 70 %.
- Kein Steam-Upload in dieser Akte; Publish-Payload lesend geprüft.
