status: aktiv
Datum: 2026-10-03

# C3: frischer enger Revisions- und Budgetfix

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration

## Ziel und Vertrag

Gewöhnlicher nativer Worker, keine weitere Delegation. Auftraggeber C3, Hauptorchestrator Root. Modell vom Elternharness GPT 6.1 Sol high erben, keine Modell- oder Effortänderung. C3-Startprozess313730, Session f83cb4c1-e8c2-4f44-8e83-ada61c2b246a, tatsächliche Parameter gpt-6.1-sol[1m]/high bestätigt.

Lies BRIEFING-C3.md und CONTRACT.md am zentralen Taskpfad sowie REVIEW-C2-2.md und C2-CHECK.md im C-Koordinationsbereich. Frische Fixrunde, nicht den alten Implementierer wieder aufnehmen. Zwei bestätigte BLOCKs: Wiki-Revisionsaliase7/07 umgehen Konfliktprüfung, Zwilling knowledge_contract.rs; chunk_index.rs erzeugt alle Projektionen vor Prüfung des Summenbudgets. Ursache systematisch über Schreiben, Konflikte, Köpfe, Lesen und Vertrag prüfen. Originalrevisionen erhalten; nicht heuristisch ordnen. Keine zweite Implementierung, Warnungsunterdrückung oder Budgeterhöhung als alleiniger Fix. Ein Alias darf nicht unvermerkt neuen Inhalt derselben numerischen Revision erzeugen.

## Eigentum

Ausschließlich rust/crates/brain-storage/src/source_versions.rs, rust/crates/dbrain-sources/src/knowledge_import.rs, rust/crates/dbrain-sources/src/knowledge_contract.rs, rust/crates/dbrain-sources/tests/knowledge_contract.rs, rust/crates/dbrain-retrieval/src/chunk_index.rs, rust/crates/dbrain-retrieval/src/knowledge_projection.rs und rust/crates/dbrain-retrieval/tests/knowledge_projection.rs. Notwendige gezielte Tests in diesen Dateien. Keine Cargo-/lib.rs-/CLI-/pg_release.rs-Änderung ohne Rückmeldung. Graphify zuerst, vorhandene Bausteine nachlesen. Andere Worker arbeiten parallel in disjunkten Dateien, keine fremde Arbeit zurücksetzen oder globale Formatierung. Acht uncommittierte C2-Formatdateien bleiben erhalten, davon gehören die genannten fünf zu diesem Paket.

## Arbeitsstand

Produktiver Worktree und Branch unverändert: /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration, feat/brain-wiki-spielwissen-c-integration. HEAD08a6dd78471cd6e7c43073e1c29dd0f31abb61c5, Basis511a347b653beba13c2bf130f4bead7a7196cc2a. Nur eigene zwei Commits seit Basis. Gezieltes git add nur eigene Dateien. Nach grünen eigenen Prüfungen eigener lauffähiger Fixcommit erlaubt, kein Push/Merge/Deploy. Git-Schritte einzeln, absolute Pfade, keine Variablen im Gatepfad, Attribution gemäß Nutzerregel Modell Version <modell@local>.

## Beweisziel

Selbstprüfung anhand des bestätigten Fehlers und seiner Zwillinge. Passende Format-, Compiler-, Clippy -D warnings-, bestehende und gezielte Tests einschließlich tatsächlicher isolierter PostgreSQL-Revisionsprüfung. Alle Cargo-/Rustharness-Läufe unter beiden blockierenden Hostlocks nach /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md: zuerst host-checks.lock, danach /tmp/deadlock-cargo-release.lock, bis Ende halten, unmittelbar vor Start frische NonZombie-Probe, maximal zwei Jobs. Keine fremden Prozesse stoppen. Eigenen stabilen Wartetask nicht nach20Minuten abbrechen. Keine Releasebauten/Produktivimporte. Bestehenden /tmp/brain-c2-fix1-check.sh nur lesen und gezielt eigenen Wrapper erstellen; nicht unverändert mit globaler Formatierung ausführen. Tests nur auf eingefrorenem eigenem Stand, falls parallele CLI noch schreibt zuerst Quellabschluss mit C3 klären, nicht denselben Code während eines Tests ändern. Vor laufendem Compiler eine sichere Rückmeldung möglich.

Nach Prüfungen und eigenem Commit regulärer vorhandener Einzelmodell-Gate zur Selbstprüfung:
python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration --base 511a347b653beba13c2bf130f4bead7a7196cc2a --head <eigener endgültiger SHA> --model gpt-6.1-sol --effort high
Keine Kette oder Rückfälle. Regulären Start und tatsächliche eigene Modellparameter nachweisen. BLOCK nicht selbst weiter würfeln; Originalfunde sichern und an C3, nächste Fixrunde bekommt frischen Kontext. Gesamtabnahme bleibt C3.

## Routing und Bericht

Eigene Fachakte /home/nathanael/.worktrees/brain-wiki-spielwissen-c/.tasks/2026-10-03-wiki-spielwissen/bereiche/c/C3-FIX-2.md. Worker schreibt nur diese eigene Fachakte, keine Register, TODO oder Statusereignisse. Statusproduzent teil-c, Paket c Versuch3, ausschließlich C3. Bericht mit eigenem SHA, Dateien, Ursachenentscheidung, Prüfbefehlen/Exits/Zahlen/Logs, tatsächlichem Gateurteil und eigenem PID-/Lockende. Gebaut, reviewt, gemergt und live getrennt. Rohberichte bleiben bei C3. Deutsche natürliche Texte mit Umlauten, keine Gedankenstriche. Bei unerwarteter Eigentumsabhängigkeit stoppen und C3 informieren, keine Nutzerfrage für Routinearbeit.
