status: aktiv
Datum: 2026-09-29

# Paket F1: einzelne alte PRs auf enthaltene oder ersetzte Inhalte prüfen

Luna, einziger Thread F1, keine Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a. Gemeinsame Grenzen in AUFTRAG.md daneben. Keine Codeänderungen und keine fremden Worktrees ändern. Du prüfst konkrete historische Diffs, keine allgemeine Architekturarbeit.

Eigener Worktree /home/nathanael/.worktrees/brain-pre-g5-docs-20260929, Branch docs/pre-g5-closeout-20260929, clean auf 305df2d36ec7b5d0513d6c0769051b41538d6a1b. Nur .tasks/2026-09-29-technical-closeout/F1-PR-AUDIT.md schreiben. Später kann derselbe Thread die finale Doku übernehmen, jetzt noch KEINE finalen Gate-Marker ändern.

Referenz wörtlich: „Prüfe jeden alten PR einzeln. Nur wenn eindeutig: bereits vollständig enthalten ODER durch neueren Integrationsstand ersetzt dann als superseded schließen. NICHT blind schließen.“

Orchestrator hat #35 bis #39 anhand vollständiger Vorfahrenbeziehung bereits geschlossen. #48 ebenfalls geschlossen, verbleibender Diff ausschließlich historische C6-Abnahmedoku, Implementierung enthalten. Branches blieben erhalten.

Prüfe jetzt einzeln #3, #4, #5, #6, #7, #8, #9, #10, #16, #17, #18, #19, #21, #22, #25, #26, #27, #28, #29, #30 gegen origin/migration/rust-integration. Graphify zuerst, danach diffs/name-status/cherry/konkrete Funktionsbelege. Pro PR konkrete eindeutige Entscheidung:
- vollständig enthalten (Commit-/Patchbeleg)
- ersetzt (neuer konkreter Pfad/Vertrag plus Vergleich und kein verlorenes zwingendes V1-Verhalten)
- einzigartige weiterhin relevante Änderung (exakte Datei, Funktion, Integrationsempfehlung)
- nicht eindeutig (offen lassen)

Nicht nur „alt“ oder „anderes Design“ als Begründung. Alte Python-Pfade sind Legacy-Referenz; vorhandene Rust-Nachfolger am Code verifizieren. #7 feat/brain-rust-cutover-20260919 niemals vollständig mergen und Branch nicht löschen.

Zusätzlich #46 C10 ansehen: NICHT schon enthalten. In aktuellem Integration-Branch fehlen dbrain-replay/src/validation.rs und tests/validation.rs samt validate-CLI, PR-Head 919c790. Alte Referenz deadlock_brain_core::replay gegenüber neuer brain_contracts::replay berücksichtigen. Beurteile ob eine technisch notwendige fehlende V1-Funktion erhalten werden muss oder allein zusätzliches Testtool für noch ungeklärte Replay-Freigabe vorliegt. Keine echte .dem suchen/herunterladen/ausführen, keine Entscheidung Replay V1/deferred treffen.

Jetzt NICHT selbst PRs schließen, keine PR-Kommentare, kein Merge. Bericht mit Belegen abgeben, Commit/Push des Berichtsbranches erlaubt, noch kein PR nötig bis Doku-Finalisierung. Kein Stopp nach bloßer Dateiliste, alle genannten PRs einzeln einordnen. Keine Tests oder Code-Builds nötig, keine neuen Code-Kommentare.
