# Paket E-Fix1: Gate-BLOCK im API-Spiegel beheben

Rolle: Blatt-Worker (GPT 6.1 Sol), frischer Fixer. Keine weiteren Threads, native Subagenten nur für Recherche.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Regeln: `AUFTRAG.md`, Steuerung `VON_HAUPT.md` (neueste oben).

## Ausgangslage

Paket E (Thread `082da08a`, gestoppt, nicht anfassen) hat den lokalen API-Spiegel gebaut. Worktree `/home/nathanael/.worktrees/brain-e-deadlock-api`, Branch `feat/brain-deadlock-api-daten`, HEAD `6d06633e`. Gate-Stand: Schema-Commit `5e70da3a` ALLOW, API-Commit `e65efae2` BLOCK. Befund und Einordnung: `.tasks/2026-10-06-brain-abschluss/AN_HAUPT-E.md` im E-Worktree, Abschnitt „Einordnung der API-Gate-Funde“, Gate-Logs daneben.

## Auftrag

1. **BLOCK beheben:** `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:177` (Zwillinge Zeile 55 und 210) wertet schon eine erkannte Änderungszeile als vollständig und überspringt die Originalauflösung. Eine Vorschau mit Volltextlink darf nie als vollständiger Patch importiert werden oder einen vollständigen Patch ersetzen. Einen gemeinsamen Vollständigkeitsmaßstab für alle drei Stellen, Zwillingssuche per Graphify (Skill `code-suche`) zuerst.
2. **Regressionstest** für den Vorschau-Fall und eine echte Produzent-/Leserprobe von `SourceStore::persist_ir` bis `brain_storage::asset_mirror::load_mirrored_assets` (Test-DB, nicht Prod).
3. Breiten Dependency-Lint nur, wenn er in E-Dateien liegt; `dbrain-enrich` nicht anfassen.
4. fmt, Clippy, Tests der betroffenen Crates, dann Selbstprüfung `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-e-deadlock-api --base 5e70da3a6f40c0f1eedc78565641d8dfce582f56 --head <neuer HEAD>`. Bei Exit 2 ohne Urteil genau einmal wiederholen.
5. Commit und Push nur auf den Featurebranch.

## Grenzen

- Release-Hold (`A/RELEASEFENSTER.md`): kein Main-Push, kein Release, keine Installation, kein Neustart, kein Tick.
- Keine Matchdaten speichern, keine neue Pipeline, kein zweiter Patchparser. Der Lesevertrag für G (`asset_mirror`) bleibt unverändert.
- Rust only, Postgres only, keine Code-Kommentare, keine ENV-Konfiguration, Secrets nie im Klartext. Deutsch ohne Em-Dashes, echte Umlaute.

## Bericht

Oben in `AN_HAUPT-E.md` im E-Worktree einen Abschnitt „E-Fix1“: Ursache, Commits, Testzahlen, Gate-Antwort wörtlich. Danach stehen bleiben (Integration erst nach Hold-Ende durch den Haupt-Orchestrator).
