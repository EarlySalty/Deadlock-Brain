# Fixer 11: Originalbindung und Fragmentbehandlung

Stand: 07.10.2026. Frischer nativer Fixer im bestehenden I-Auftrag. Verbindlich: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/ENTSCHEIDUNG-I-PATCH-ORIGINAL.md` (16:09 UTC).

## Ziel und Umfang

Arbeite ausschließlich in `/home/nathanael/.worktrees/brain-e-deadlock-api`, Branch `feat/brain-deadlock-api-daten`, bestätigter sauberer Ausgang `16eaecdee8a0331c97c8bda78220cdf5bdd8b20b`. Der Integrationskandidat `860793d7` und sämtliche bisherigen Gegenproben bleiben erhalten. Kein neuer Thread oder Arbeitsbaum, kein fremder WIP und keine G/K-Datei. G/K-Prüfsperren dürfen weder übernommen noch über andere Werkzeuge umgangen werden.

1. Konkrete angefragte Steam-Ereignisidentität strikt binden. Kein fremder Body bei fehlender passender GID und kein Überschreiben gültiger Ereignisse. Vorhandenen HTML-/Announcement-Parservertrag prüfen, einschließlich Legacy-Aufrufern und möglicher unterschiedlicher Kennungsfelder. Keine neue Parserpipeline oder pauschale Titel-/Zeit-Freigabe.
2. Anerkannte Steam-/Forum-Fragmentlinks im bestehenden URL-Weg einheitlich kanonisieren. Für Prüfung und tatsächlichen HTTP-Abruf dieselbe fragmentfreie Anfrage-URL verwenden; ursprünglichen Feedlink/Herkunft nachvollziehbar erhalten. HTTP-Core-Guard, Origin, Schema, Redirects, Größen und Timeoutgrenzen unverändert. Andere Abruffehler nicht verschlucken.
3. Diagnostische Zeugen aus `PATCH-GATE-DIAGNOSE.rs` in reguläre Regressionen mit richtigem Sollverhalten überführen. Richtige GID bleibt positiv; falsche oder fehlende konkrete GID darf keinen fremden Body liefern. Beide Fragmentquellen funktionieren und bewahren Herkunft. Der einmalige Diagnosecode darf nicht als dauerhaft gewünschtes Fehlverhalten in die Suite eingehen.

Befunde und tatsächliche Diagnoseausgabe: `NACHWEIS-PATCH-GATE-BLOCK.md`, `REVIEW.md` Runde 15. Bildlink-NIT bleibt getrennt offen, kein Scope-Zuwachs. Keine neuen produktiven Kommentare, ENV-Konfigurationen, Connectoren, Matchablagen oder Modell-/Timeoutwechsel. Produktcode ausschließlich Rust.

## Prüfweg und Abschlussgrenze

Bestand zuerst mit geladenem code-suche und Graphify, dann gezielt Quellen lesen. Kein voller Graph-Neubau. Nur der bisher zugelassene I-Prüfweg: own Worktree, vorhandene Cargo-Slots, `SQLX_OFFLINE=true`, Produktiv-DSNs für Tests entfernen, echte isolierte Scratch-DBs statt Produktionsschreibzugriffen. Ein Deny wird nicht über Wrapper, anderen Worker, Toolweg oder Hookänderung umgangen. Format nur eigene Dateien mit rustfmt; cargo fmt ausschließlich --check. Passende Compiler-/Clippy-/bestehende Testprüfungen selbst durchführen und Ergebnisse mit passed/failed/ignored nennen.

Gezielt nur eigene Dateien stagen, Git-Einzelschritte mit absoluten Pfaden. Featurecommit mit `Co-authored-by: GPT 6.1 Sol <gpt-6.1-sol@local>`. Kein Main-Merge, Deploy, Restart, Cleanup oder Selbst-Settle durch den Fixer. Selbstprüfung nach Commit über `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review`, Basis Ausgangs-SHA `16eaecdee8a0331c97c8bda78220cdf5bdd8b20b`, fixer HEAD, zwingend `--model claude-opus-5-5`. Kein anderer Reviewer, Cache- oder Hookeingriff. Bei BLOCK Urteil und echten Restkern zurückgeben, keine eigene Kontextfixrunde oder Modell-Neuwahl. Bei ALLOW Fix-SHA und Prüfnachweise zurückgeben; nur eigenen Featurebranch sichern. Die Hauptsession integriert und prüft den gemeinsamen Produktstand danach selbst.

Keine Sessionkontakte. Keine browsergestützte Arbeit nötig; falls doch: zuerst `/home/nathanael/Documents/claude-config/wissen/agent-browser.md`, nur Moli `/home/nathanael/.local/bin/moli`, MUST NOT Brave oder persönliche Browser. Nutzer-/Community-Daten MUST NOT an externe Anbieter oder externe Codiermodelle, auch nicht über Loopbackproxy. Secrets NEVER ausgeben. Bestehende Code-/öffentliche Vertragsproben sind zulässig.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-e-deadlock-api
