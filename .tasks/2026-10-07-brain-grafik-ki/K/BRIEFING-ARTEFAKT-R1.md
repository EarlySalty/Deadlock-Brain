# K: Artefaktfixrunde 1

## Ziel und bestätigter Befund

Regulärer Gate `bbkgp67qp`, Basis `7cbb9fe6`, HEAD `b310e223c1fdbaed17661f10e8edebae5b1534df`, Exit 1. Tatsächliches Urteil aus `/tmp/k-compare-artifact-gate-20261007.log`: `[gpt-6.1-sol] BLOCK: Artifact fingerprints are unstable across JSONB storage.`

Bestätigter Kern: Storage hashbasiert JSON-Bytes, PostgreSQL JSONB normalisiert Zahlen. `-0.0` wird `0.0`; `1e18` kann als Integer zurückkehren. Pending-Speicherung/Veröffentlichung und erneuter Abruf müssen bei zulässigen Zahlen dieselbe ID, Rechnung und H-Ausgabe behalten. Zwillingsstellen: Storage `compare_artifact.rs:151,275,347`, Maintenance `compare_artifact.rs:190,228`. Ursache in der gemeinsamen Fingerprint-/Persistenzbindung korrigieren, keine pauschale Floatablehnung, Rundung oder Verringerung des bisherigen gültigen Zahlenbereichs. H-Renderer wiederverwenden und nicht parallel ändern. Tatsächliche JSONB-Roundtrips und dieselben HTML/SVG-Bytes prüfen, nicht nur Fake-Serialisierung.

Gate-NIT: SQL-Funktion liefert komplettes `release_json`, einschließlich möglicherweise unbeteiligter privater Dokument-IDs. Bestehende Corpus-Release-Grants und verbindliche Site-/Datenschutzgrenze lesen. Das Artefakt darf keine fremden privaten IDs an öffentliche Leser geben. Boundary anhand tatsächlicher Rollen und Ausgabe belegen; falls unerlaubt, im eigenen noch nicht produktiv angewandten SQL-/Artefaktpfad beheben. Keine allgemeine Quelldatenfreigabe, kein Verlust der Release-/Pinprüfung. Diese NIT ist bislang keine belegte öffentliche HTTP-Leakbehauptung.

## Eigentum und Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-k-ki-20261007`, Branch `feat/brain-k-ki-20261007`, HEAD `b310e223c1fdbaed17661f10e8edebae5b1534df`. Alle Produktdateien des Artefaktbaus committed; Akten sind eigener unstaged WIP. Kein Featurepush dieses Artefaktcommits. Du bist ein frischer nativer Fixer, nicht der ursprüngliche Implementierer.

Exklusiver Schreibbereich: `rust/crates/brain-storage/src/compare_artifact.rs`, zugehörige `tests/compare_artifact.rs`, `rust/crates/brain-maintenance/src/compare_artifact.rs`, zugehörige `tests/compare_artifact.rs` und nötigenfalls `rust/crates/deadlock-brain/src/bin/site/compare_tests.rs` sowie die eigene noch nicht angewandte `scripts/migrations/2026-10-07-brain-compare-artifacts-v1.sql`. Keine alten angewandten Migrationen ändern. Keine aktive G-, I-, Q- oder H-Datei, kein neuer Provider, kein zweiter Renderer. Vor Neubau vorhandene Canonicalization-/Fingerprintbausteine über Graphify suchen und wiederverwenden. Zusätzliche Manifestabhängigkeit nur, wenn zwingend und vorher lokal begründet, keine zweite Persistenz.

Kein Commit, Push, Merge, Releasebuild, Deploy oder produktive Migration. K allein integriert. Der ausdrücklich vorrangige zentrale Antwortwriter a77550ad927916411 wurde bereits gestartet; sein service.rs-/Vertrags-/Provider-/Kernelbereich bleibt für dich gesperrt. Behandle zuerst den bestätigten JSONB-Blocker, keine nichtblockierende Grafik-NIT-Runde vorziehen. Parallel läuft a92067f4cff648233 ausschließlich an vorhandener isolierter Maintenance-Testfixture, ohne Produktdatei-Schreibrecht. Keine Sessionkontakte. Die bestehenden Artefakttests sind dein eigener Beweisbereich; broad-fixture nicht duplizieren oder fremde Prozesse stoppen.

## Nachweis und erneuter Gate

Bestehende scoped Artefakt-/Site-/Rendererprüfungen nachziehen. Absolute moderne Cargo-CLI `+1.97.1`, vorhandener Cache, echte freie Buildslot-Locks `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/build-slot-{1,2,3}.lock`, höchstens `--jobs 3`. `SQLX_OFFLINE=true` nur als vorhandene Build-/Testeinstellung. Compiler, passende Formatierung und striktes scoped Clippy prüfen. Exakte Befehle, Exit-Codes, passed/failed/ignored/filtered und Logpfade melden. Keine Pipe oder nachgeschaltete Ausgabe, welche Exits maskiert.

Vor Rückgabe eigene Änderung mit dem regulären `gate_hook.py --review` prüfen. Gate akzeptiert nur committed HEAD: wenn das aktuelle Werkzeug für einen uncommittierten Fix keinen sauberen Reviewanschluss hat, keine Git-Schreibrechte erfinden, sondern diesen Mechanikblocker samt verifiziertem Diff zurückgeben; K committed gezielt und fährt den unveränderten regulären Gate. Kein Modelloverride oder zweites Reviewer-Modell nach BLOCK. Rückgabe nur bei Task-Ende oder echtem Blocker, keine Rundenmeldungen.

## Sicherheitsgrenzen und Routing

NEVER read, print or write plaintext secrets.
MUST NOT send private user/community data to remote models.
Keine echten privaten Fragen, Communitydaten oder Produktionszeilen lesen. Synthetische Fixture ist kein echter G-/Livebeweis. Modelle/Timeouts bleiben unverändert, keine neuen Code-Kommentare, Rust/Postgres. Keine weiteren Threads, Subagenten, ListAgents oder SendMessage. Keine zentralen Register/TODO-Edits. Fehlende echte G-Rechnung/Dokumentpinzuordnung nicht durch einen immer erlaubenden Produktionsverifier ersetzen.

K-Session `988eeaea-28ee-424c-b362-e250610cde91`, Produzent teil-k, Versuch 1. Delegator `481426fe-b477-42b3-91c6-901811fcba1d`, Hauptorchestrator `d3a1741e-82bc-4a48-865b-2845c663dca7`. Rückgabe direkt als native Task-Rückgabe; K schreibt Fachbericht und integriert.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-k-ki-20261007
