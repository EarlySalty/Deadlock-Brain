# Answer audit report, 2026-10-30

Window: last 24h of answered traces before 2026-10-30. Probe-question traces skipped.

```
answer_audit traces=305 skipped=803 claims=590 flagged=3 flags=3

#1422 10-29 09:31 stale_history
  C: Nicht ganz. Laut aktuellen Spieldaten gibt ein Lane-Trooper 100 Seelen als Grundwert, dazu kommen 2 Seelen pro Spielminute. Zum Spielstart sind das also 100 Seelen je Trooper und 400 für eine ganze Welle mit 4 Troopern. Davon bekommst du 50 sofort beim letzten Treffer, die anderen 50 kommen als Seelenorb, den der Gegner auch denien kann. Früher waren die Werte anders: Mit dem 06-17-2025 Update wurde der Grundwert von 132 auf 120 gesenkt und der Bonus pro Minute von 1 auf 1.5 erhöht. [Sta
  E: 2025-07-29 07-29-2025 Update: Trooper bounty growth changed from 1.5 to 1.2
  S: https://forums.playdeadlock.com/threads/07-29-2025-update.72760/
--
#2205 10-29 21:31 stale_history
  C: Hier sind die Treffer: - Fire Rate Powerup: von +20% auf +15% reduziert, die Dauer wurde von 25s auf 35s erhöht. [Stand 2024-06-06 06-06-2024 Update]
  E: 2024-06-27 06-27-2024 Update: Fire Rate powerup Duration changed from 35 to 45s and +15% Fire Rate to +18%
  S: https://forums.playdeadlock.com/threads/06-27-2024-update.8486/
--
#2411 10-29 22:06 stale_history
  C: Welches Item meinst du genau? Ich habe keinen eindeutigen Treffer für Fire Rate gefunden, deshalb kann ich noch nicht sicher sagen, was früher Fire Rate hatte. In den Patchnotes stehen Fire-Rate-Werte bei Ricochet, Frenzy und Spiritual Overflow, zum Beispiel Ricochet: Fire Rate reduced from 25% to 20% (07-04-2025 Update). Auch der Fire Rate Powerup wurde von +20% auf +15% gesenkt (08-08-2025 Update). Sag mir den Namen, dann schaue ich dir die genauen Werte raus. [Stand 2025-07-04 07-04-20
  E: 2026-03-06 03-06-2026 Update: Fire Rate changed from 18 to 20
  S: https://forums.playdeadlock.com/threads/03-06-2026-update.114328/
```

Review: 2205 lists the 06-06-2024 Fire Rate Powerup values as the change and misses the 06-27-2024 follow-up; 1422 cites history up to 06-17-2025 and misses the 07-29-2025 bounty growth change; 2411 is a clarifying answer that quotes old fire rate lines (borderline).

Earlier draft rules flagged 12 traces on the same window; 9 were false positives (unrelated later rows, numbers bound to the wrong entity, dated history lines) and were removed by requiring the later row to supersede the exact anchor value, same entity on both sides, and number-to-name proximity.
