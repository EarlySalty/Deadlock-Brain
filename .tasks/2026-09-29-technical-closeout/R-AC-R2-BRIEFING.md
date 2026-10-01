status: aktiv
Datum: 2026-09-29

# R-AC Runde 2: A1/A2/C1 nachprüfen

Derselbe unabhängige Reviewer 52c34332 nimmt den vorhandenen sauberen Reviewbaum wieder auf. Keine neuen Unteragenten, keine Produktänderungen, keine Produktionsaktivierung, keine Integration nach migration/main. Hauptauftrag und R-AC-BRIEFING gelten. Intent 562a877b-0939-440a-964d-1145d9e9431a.

Eigener Baum /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929, Branch review/pre-g5-core-abnahme-20260929, zuletzt clean fde7d5d. Im eigenen Prüfbranch darfst du folgende feste veröffentlichte Branchstände lokal mergen:
- A12: 93468b0 auf fix/pre-g5-match-contract-20260929, Produktfix 8e8ce76. Autorbericht A12-REPORT.md. Basis 34a2507, daher keine neue A34-Arbeit enthalten.
- C vollständig: ead4a791 auf fix/pre-g5-harness-20260929 (PR #60), Produktfix 1882419 auf e671c5b. Autorbericht C1-REPORT.md. Seit deinem e540e97 zusätzlich Tests zu fehlenden Assets-Werten und leerem Legacy-Archiv. Keine Produktänderung aus C.

Nur gegen die vollständige bekannte Mängelliste prüfen, kein neuer ungerichteter Vollaudit. R-AC-A1/A2: include_player_kda=true fordert die konkrete KDA-Spielerprojektion an, URL/Locator/Fixtures synchron, validierte typisierte Projektion vor Store. Die im Erstbericht reproduzierten Drift-Gegenfälle müssen jetzt abgewiesen werden; positiver realer Upstream-Projektionsvertrag muss passen. R-AC-C1: alle verlorenen ursprünglichen Start-/Nichtmigrationsfälle sowie EN-Alias, wirklich unbekannte Entity, Konflikt bei Limit1, präzise ACL und fachliche DB-Recovery im gleichen Serve-Prozess sind ergänzt.

Eigene Gegenproben und sicheren Prozessrunner auf diesem kombinierten Prüfstand ausführen; keine realen Nutzer-/Matchdaten. Exakte SHA/Befehle/Exitcodes und verbleibende Mängel in REVIEW-AC-R2.md. Urteil je A1/A2/C1. A3/A4 werden noch durch A34/Sol behoben und bleiben bis gesonderter Abgabe ausdrücklich offen. Keine Gesamtfreigabe vor A34 und der späteren finalen Messung. Pool und Budgets unverändert, rote Befunde nicht ausblenden. Berichtscommit eigenen Branch pushen, keine Produktfixes im Review.
