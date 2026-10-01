status: aktiv
Datum: 2026-10-01

# Brain PR61/PR9 gemeinsame Integration

## Ziel

Einen isolierten Integrationsbranch auf exakt `origin/main` `be2aa6bd5a6504e99693f7dd1edaa16e76be91b4` bilden und ausschließlich belegte, noch fehlende Änderungen aus dem PR61-Ownerbranch sowie den committed PR9-Handoff übernehmen. Den resultierenden Commit mit Baseline, vollständigem Eigen-Diff und Checks zur unabhängigen gemeinsamen Intent-Abnahme vorlegen.

## Verifizierte Eingaben

- PR61-Ownerbranch: `codex/fix-pr61-publication-enforcement-20261001` auf `2767140983f6e21b8a29e8bf2661040736c21262`, sauber. Der tatsächliche Direct-Tree-Diff gegen die Basis umfasst 97 Pfade, 2.782 Einfügungen und 7.566 Löschungen. Die 9 Steam-Ledger-Commits sind nicht zu übernehmen.
- PR9-Owner-Handoff: Branch `luna/abschluss-brain-pr9-20261001`, Commit `7deebcb779b6fa5572ebe1c1e554a3dbd9c2e569`, Parent `e752d2514249ece9b3702c5fd93a75680495db4c`. Nur den begrenzten Handoff prüfen, nicht die 41 Commit umfassende historische PR9-Reihe.
- PR9-Owner meldet den Handoff als sauber committed; Quellworktree bleibt unangetastet.

## Arbeit

1. PR61-Eigenanteil relativ zur Basis anhand Commitgrenzen, Patch-IDs und direktem Treevergleich abgrenzen. Nur verifizierte fehlende Regressionstests übernehmen. Bereits vorhandene Steam-Ledger-, PR40- und PR61-Funktionalität nicht erneut integrieren.
2. Den PR9-Handoff pfadweise gegen die Basis prüfen. Fehlende Import-, Schema- und History-Korrekturen übernehmen, die auf der Basis tragfähig sind. Keine PR9-Historie, obsolete CI-Pfade oder uncommittete Fremdarbeit übernehmen.
3. Änderungen auf disjunkten Pfaden integrieren, Konflikte mit Eigentümer- und Vertragsbeleg lösen und alle Quellen, Commits, Patch-IDs und Ausschlüsse dokumentieren.
4. Den finalen Scope direkt zwischen Basis und kombiniertem Commit bestimmen. Änderungen prüfen und verfügbare Checks ausführen, ohne einen Gate-PASS oder Live-PASS zu behaupten.

## Grenzen

- PR61-Quelle, PR9-Quelle und deren Worktrees bleiben erhalten und unverändert.
- Kein Adapter- oder Native-Neubau und kein globaler Codex-/Proof-Wechsel.
- Kein Merge, Push, Deploy, Restart oder Gruppen-Live-PASS in diesem Integrationsschritt.
- Stage unter `rust/target` nicht committen.
- AI-Coach-Produktänderungen sind außerhalb des Scopes.
- Kein Gate vor Root-Freigabe des exakt kombinierten Freeze-SHA.

## Fertig

Der isolierte Branch enthält nur belegte fehlende Änderungen und Task-Artefakte. Der kombinierte SHA, Basis, vollständiger Direct-Tree-Diff, Patch-ID-Abgleich, Ausschlussliste und Checks sind an Root gemeldet. Danach auf Intent-Abnahme warten.
