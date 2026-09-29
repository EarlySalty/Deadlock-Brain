status: aktiv
Datum: 2026-09-29

# Bestand und Integrationsentscheidungen

## Frisch abgefragte Repositories

Alle sieben lokalen Repositories wurden zunächst mit `git fetch --all --prune` aktualisiert. Pfade sind /home/nathanael/repos/<Repo>, Remotes EarlySalty/<Repo>. Worktree-/Branch-/Dirty-Inventar und offene PRs wurden neu ermittelt. Die Zähler sind eine Momentaufnahme während laufender Arbeit und enthalten eigene neue Prüfworktrees.

| Repository | origin/main | Worktrees | Dirty | lokale Branches |
|---|---|---:|---:|---:|
| Deadlock-Brain | 25c6ed695137 | 69 | 12 | 68 |
| Deadlock-Bots | 42175e5fd68a | 56 | 13 | 43 |
| Deadlock-Twitch-Bot | cf3d77085ed3 | 100 | 40 | 81 |
| Deadlock-Docs | 75358d57e665 | 11 | 4 | 9 |
| Deadlock-2nd-Brain | 44229978e151 | 2 | 0 | 2 |
| Deadlock--Patchnotes-Bot | db1d36398d70 | 55 | 11 | 43 |
| Deadlock-Steam-Bot | 8c3fc6e0e8f5 | 17 | 4 | 14 |

Fremde Arbeitsstände wurden nicht geändert. C9-MERGE_HEAD und Brain-Dirty-Adapter sind in AUFTRAG.md ausdrücklich erhalten. Ein unabhängiger Twitch-Patch-Thread läuft; keine Koordination oder Eingriffe in dessen Worktree.

## Tatsächliche API

Ermittelt über Brain-Code, Live-API-Dokumentation und GitHub-Metadaten: https://github.com/deadlock-api/deadlock-api, öffentlicher Defaultbranch master. Kein EarlySalty-API-Repository geraten. Origin https://api.deadlock-api.com. Aktuelles read-only OpenAPI https://api.deadlock-api.com/openapi.json am 2026-09-29 geladen, OpenAPI 3.1.0, info.version 0.1.0. Keine Match-/Account-Produktdaten abgefragt.

Eigener Referenzklon ohne Checkout: /home/nathanael/.worktrees/brain-deadlock-api-reference-20260929. Nach Clone zusätzlich fetch --all --prune ausgeführt. origin/master: 2a3cbd17b2d6524505414eb0da8f83b200f06ae0. Sechs offene Upstream-PRs inventarisiert, kein Eingriff in dieses Fremdprojekt. Kein produktiver API-Dienst und keine Analytics-Kopie eingerichtet.

Für A relevante Contractdetails: /v1/matches/metadata bietet match_ids, account_ids, only_filtered_players, hero_ids, limit und format. /v1/analytics/hero-stats liefert AnalyticsHeroStats-Array, hat Zeitfensterparameter, aber keinen hero_ids-Queryparameter. /v1/analytics/item-stats hat hero_id/hero_ids. Ein Hero-Filter bei hero-stats muss deshalb tatsächlich nach Responsevalidierung erfolgen und darf nicht durch einen wirkungslosen Queryparameter behauptet werden. Schema-/Patch-Pins müssen den realen Vertrag abbilden, keine erfundenen Serverantwortfelder.

## PR-Zustand

Brain-Integration unverändert 305df2d36ec7b5d0513d6c0769051b41538d6a1b. PR #40 offen, Draft, kein Auto-Merge. Vollständige Code-CI dieses alten Heads grün außer GitGuardian; dies ist kein Nachweis für neue Pakete.

Bereits gemergt: Brain #41/42/43/44/45/47/49/50/51/52/55/56; Patchnotes #49 (06c50ab), Steam #73 (d6b6852), Schema Bots #461 (c421d61), Twitch #984 (13321934). Historisch rote Provider-CI wird von D/E anhand Logs und lokalen Läufen neu bewertet.

Bots #459 Draft/DIRTY bei Remote-Head 74cc114; lokaler Main-Merge staged. Docs #4 CLEAN/grün ff20af7; 2nd-Brain #2 UNSTABLE/Typed fixtures FAILURE ab83b69. Kein Consumer-Merge.

## Alte PRs einzeln geprüft

| PR | Befund | Aktion |
|---|---|---|
| #35 | Vollständiger PR-Head ist Vorfahr von 305df2d, merge-base --is-ancestor Exit 0 | geschlossen mit superseded-Kommentar, Branch erhalten |
| #36 | Vollständiger PR-Head ist Vorfahr von 305df2d, merge-base --is-ancestor Exit 0 | geschlossen mit superseded-Kommentar, Branch erhalten |
| #37 | Vollständiger PR-Head ist Vorfahr von 305df2d, merge-base --is-ancestor Exit 0 | geschlossen mit superseded-Kommentar, Branch erhalten |
| #38 | Vollständiger PR-Head ist Vorfahr von 305df2d, merge-base --is-ancestor Exit 0 | geschlossen mit superseded-Kommentar, Branch erhalten |
| #39 | Vollständiger PR-Head ist Vorfahr von 305df2d, merge-base --is-ancestor Exit 0 | geschlossen mit superseded-Kommentar, Branch erhalten |
| #48 | Zwei noch nicht enthaltene Commits ändern nur C6_DOMAIN_KERNEL.md mit Tests auf 3b86d3c. C6-Code bereits integriert, neuere kombinierte Abnahmen vorhanden | geschlossen mit begründetem superseded-Kommentar, Branch erhalten |
| #46 | Eindeutig NICHT vollständig enthalten: validation.rs, validation-Tests und CLI validate fehlen in Integration. Vertrag im alten Stand teilweise überholt | offen lassen, gezielten technischen Nutzen und Integration prüfen; keine falsche superseded-Behauptung |
| #16/17/18/19/21/22/27/28 | Je PR unabhängig im Orchestrator nachgerechnet: 5/5, 3/3, 6/6, 5/5, 60/60, 10/10, 27/27, 7/7 geänderte Dateien blob-identisch im Integrationsstand | einzeln mit konkretem Nachweis geschlossen, Branches erhalten |
| #3/4/5/6/7/8/9/10/25/26/29/30 | F1 hat jeden PR geprüft; Ersatzbelege werden gezielt nachgebessert, unklare Gleichwertigkeit wird nicht behauptet | noch keine Schließung aus bloßer Ähnlichkeit |

## GitGuardian

Aktueller Check 108480492324 meldet Incident 37635766 weiterhin Triggered. Historischer Fund in df2d61fa116eb209e7516e80bddc957ac8546d21, scripts/run_isolated_serve_checks.sh:24. Kontrollierte Auswertung der Quelle: auth-Modus password und password_env mit einem Environment-Variablennamen, kein dort einkompilierter Passwortwert. Der tatsächliche Testprozess nutzt später einen separat geladenen Wert; dieser wurde nicht ausgegeben.

Dashboard: https://dashboard.gitguardian.com/workspace/755270/incidents/37635766?occurrence=299625425

Normale Aktion: Incident öffnen, anhand des genannten Commits den Variablennamen bestätigen, als False Positive beziehungsweise Test Credential schließen und Check regulär neu prüfen. Kein Rebase/Force-Push/History-Rewrite, kein Check-Bypass. Automation aktuell nicht verfügbar: preview_open meldet No preview automation host is available. Damit ist die Dashboard-Aktion noch NICHT erfolgt.
