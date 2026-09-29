status: erledigt
Datum: 2026-09-29

# CI-Beobachtung am finalen integrierten Codehead

GitHub-Abfrage am 2026-09-29 um16:01UTC: PR40 ist OPEN, Draft=true, base=main, head=022f8a981c2164f6d8d4302bae2194e100c4f65c. Kein Merge oder Ready-Schritt für PR40.

Erfolgreiche Checks: scratch-pilot, Migration composition, Rust compile, Wiki contracts and offline regression, Integrated audit tooling (quality-pr27), Integrated audit tooling (cutover-audit), Python syntax. Semantic review ist skipped. Dies ist keine vollständig grüne Workspace-CI.

Die Core-Matrix und integrierten Wiki/Source/Replay-Suiten im Lauf36591388159 sind fehlgeschlagen. Eigene erneute Prüfung des Core-Jobs109485085537 im fehlgeschlagenen Schritt Lockfile acquisition:

```text
2026-09-29T15:35:58.7922116Z error: failed to get `haste_core` as a dependency of package `dbrain-replay`
2026-09-29T15:35:58.7925677Z revision bfb292d4798031350861ad297aa26753267a1ea6 not found
```

Andere Matrixjobs zeigen denselben Erwerbfehler; fehlende Upload-Artefakte sind ein Folgefehler vor Teststart. Der Consumer Offline Gate ist ebenfalls FAILURE. Das ist kein Billing-Fall für diesen Brain-Lauf. G-REPORT.md dokumentiert exakte Quellzugangs-/Lizenzgrenzen; Pins unverändert, kein unlizenzierter Ersatz.

GitGuardian Security Checks bleiben FAILURE mit vorhandenem Dashboardverweis. Incident37635766 ist nicht normal geschlossen, Browserautomation weiterhin ohne Host. Kein Security-Bypass oder History-Rewrite. Lokale Gates werden getrennt dokumentiert und machen diese externen Prüfungen nicht grün.

Quellen: https://github.com/EarlySalty/Deadlock-Brain/pull/40 und https://github.com/EarlySalty/Deadlock-Brain/actions/runs/36591388159 .
