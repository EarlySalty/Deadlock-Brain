# Review-Briefing: build-reasoner (Paket S, Runde 3, Live-Fix)

[Orchestrator] Review für den Live-Fix des Autoren-Scans (Steam-Bot). Lesend,
kein Code, kein Branch. Du bist der einzige Thread für dieses Review. Keine
Unter-Threads oder Unter-Agenten spawnen.

- Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-scan-2` (Branch
  `fix/autoren-scan-live`, Commit bc08e23, Basis aba7636 = main nach dem
  ersten S-Deploy)
- Diff: `git -C /home/nathanael/.worktrees/steam-bot-autoren-scan-2 diff aba7636..HEAD`
  (514 Zeilen neu, 204 entfernt: catalog.rs, discovery.rs, shared.rs,
  builds.rs, Cargo.toml, .sqlx)
- Fix-Briefing mit Live-Befund: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/FIX2-BRIEFING-S.md`,
  Fertigmeldung `FERTIG-S2.md`, Vorgeschichte `REVIEW-S.md`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`

## Was du prüfst

1. Ursache: Der Fixer belegt, dass `CMsgClientToGCFindHeroBuilds` ohne
   `hero_id` Ergebnis 0 liefert und mit `hero_id` Ergebnis 1 (Warden: 40
   Treffer, Build 779996 gefunden). Prüfe die Beweisführung in FERTIG-S2.md
   und den Code: sucht der Autoren-Scan jetzt je Held oder gezielt, und
   wie viele GC-Anfragen entstehen je Autor und je Zyklus (13 Autoren mal
   38 Helden wären rund 500 Anfragen; ist das mit `GC_REQUEST_DELAY` und dem
   Rate-Limit Code 5 verträglich, und wie lange dauert ein Zyklus dann)?
2. Laufzeit: eigene Autoren- und Build-Tasks mit höchstens 480 Sekunden je
   Handler. Prüfe, dass der Zyklus-Task selbst nur einplant und schnell
   endet, dass Teil-Tasks das Stale-Limit (600 s in `task/runner.rs:34`)
   sicher unterschreiten, dass kein Teil-Task doppelt eingeplant wird
   (Dedup), und dass ein per API eingereihter `BUILD_CATALOG_CYCLE` nicht
   mehr hinter einem laufenden Zyklus hängen bleibt (Task 4047138 im
   Befund). Neue Task-Typen müssen im Handler-Register und in der
   Lane-Zuordnung stehen.
3. Sichtbarkeit: Journalzeilen `Autoren-Scan beendet` und `Katalogzyklus
   beendet` mit Zahlen, keine Zeile je Held.
4. Persistenz: Upsert in `hero_build_sources` inklusive `details`,
   `version`, Zeitstempel; Rückschreibung `watched_build_authors.
   last_checked_*` je Autor mit ok/partial/error; kein Overwrite mit 0;
   `.sqlx`-Cache passt zu den Queries.
5. Scope und Hygiene: keine Lobby-, Rank-, Invite-, Publish-Änderungen, keine
   neue Poll-Schleife, keine Migration, keine Code-Kommentare, keine
   Secrets. Neue Abhängigkeit in `steam-core/Cargo.toml` begründet?
6. Tests selbst laufen lassen und Zahlen nennen: `SQLX_OFFLINE=true cargo
   test -p steam-core -p steam-persistence --manifest-path rust/Cargo.toml`
   (Fixer: 174) und Docker-Katalogtests über `rust/scripts/central_test_db.sh`
   mit `--features testing -- task::handlers::builds::catalog::tests
   --include-ignored` (Fixer: 34).

## Ergebnis

Datei `REVIEW-S2.md` im Task-Ordner: Mängelliste mit Nummer, Datei:Zeile,
Befund, Schwere, Vorschlag; Urteil FREIGABE oder NACHBESSERN; Deploy-Prüfung
(welche Journalzeilen und welche DB-Abfrage nach dem Neustart den Erfolg
zeigen, inklusive Build 779996). Fertigmeldung in diesem Thread mit Pfad und
Urteil. Deutsch, echte Umlaute, keine Gedankenstriche.
