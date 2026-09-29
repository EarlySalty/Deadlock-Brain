status: erledigt
Datum: 2026-09-29

# Deadlock Brain technisch vor G5 abschließen

Intent-Thread: 562a877b-0939-440a-964d-1145d9e9431a
Orchestrator: Astra. Kleine Pakete Luna, größere Implementierungen Sol. Keine erneute Planfreigabe. Der aktuelle Nutzerauftrag hat Vorrang vor alten Deploy- und Abschlussanweisungen.

## Harte Grenzen

Kein Production-Cutover, kein Merge von Deadlock Brain nach main, kein Merge von Brain PR #40, keine produktiven Consumer aktivieren, keine echten Discord-/Twitch-Nachrichten, keine echten Steam-Builds veröffentlichen. Keine Policies, Required Checks oder Hooks umgehen. Keine fremden Worktrees verändern, zurücksetzen oder löschen. Keine History-Umschreibung. Branches nicht löschen. Keine Secrets lesen/ausgeben, keine produktiven DB-Credentials und keine Passwörter über Environment transportieren. Rust für neue Werkzeuge und Anwendungscode. Keine Code-Kommentare. Keine neuen Modelle/Provider/Budgets ohne Betreiberfreigabe. Wiki-Netz-Capture nur mit Quelle-, Lizenz- und Aufbewahrungsfreigabe. Replay nicht herunterladen und keine Match-ID raten.

Jeder Implementierer arbeitet nur im zugewiesenen Worktree und ist dessen einziger schreibender Worker. Keine Unter-Threads oder Unter-Agenten. Graphify zuerst, dann Fundstellen nachlesen. Kein Refactoring außerhalb des Pakets, kein globaler Formatierungsdiff. Tests nicht abschwächen, ignorierte Tests nicht als bestanden zählen. Commits mit `Co-Authored-By: Claude Code <noreply@anthropic.com>`. PR-Beschreibungen enden mit `🤖 Generated with [Claude Code](https://claude.com/claude-code)`.

## Integrationsweg

Brain-Pakete: eigener Branch und PR nach `migration/rust-integration`. Vor Merge unabhängiges Review und lokale Gates, anschließend vollständige Branch-CI beobachten. Integration ausschließlich durch Orchestrator. PR #40 bleibt Draft nach main. Consumer-PRs Bots #459, Docs #4 und 2nd-Brain #2 nicht mergen. Anbieter-PRs Patchnotes #49 und Steam #73 sowie Schema Bots #461 zunächst nur überprüfen, kein Neubau ohne Befund. Twitch #984 nur Regression prüfen.

## Verifizierter Ausgangspunkt

Alle sieben benannten Repositories am 2026-09-29 per `git fetch --all --prune` aktualisiert. Brain `origin/migration/rust-integration` bleibt `305df2d36ec7b5d0513d6c0769051b41538d6a1b`.

Erhaltener Brain-Worktree `/home/nathanael/.worktrees/brain-pre-g5-finalize-20260929`, Branch `integration/pre-g5-finalize-20260929`: geänderte Cargo.lock, brain-feeds/lib.rs, dbrain-sources/Cargo.toml und lib.rs; neue deadlock_match.rs und analytics_runtime.rs. Kein Reset.

Erhaltener Bots-Worktree `/home/nathanael/.worktrees/bots-c9-consumer-wiring`, Branch `codex/fix-c9-consumer-wiring`: laufender konfliktaufgelöster Merge, MERGE_HEAD `42175e5fd68a1c83bf4201b4c40cca1099f00652`. Kein abort/reset. Remote-PR #459 Draft, Head `74cc114290e6490afc5dc922ef660c34025c5f36`, bisherige Checks grün, Remote meldet DIRTY.

Docs #4 CLEAN/grün, Head `ff20af7a8e3fcacc349cb2d97eda34dfa0897957`. 2nd-Brain #2 Head `ab83b691befd761a16d971af5c249a604c5d4e0d`, Typed Brain adapter fixtures rot, Ursache neu prüfen statt Billing unterstellen. Unabhängige laufende Twitch-Patch-Arbeit bleibt unberührt.

## Pflichtumfang

1. Match: Deadlock API -> validierter Response -> SourceRecordV2 -> normaler DocumentStorePort -> CorpusRelease -> brain-serve. Match- und Account-ID eindeutig, account-bound, niemals public, Schema-Version/Hash, Raw-Hash, Locator, Zeit, Parser, Provenienz, Checkpoint/Revision/Tombstone, idempotent/revoke/release. Kein DL-Main/ClickHouse. Fehlender, doppelter Match, falscher Scope, public, Schema-Drift, ungültiger Hash müssen geschlossen scheitern.
2. Meta/Population: bestehende Deadlock API, keine neue ClickHouse-Instanz/Kopie, kein DEADLOCK_CENTRAL_DSN. Kanonische Production-Origin https://api.deadlock-api.com, Loopback nur Tests. Timeout, begrenzte Retries, Gesamtdeadline, Byte-/Row-/Zeitfenster-Limit, Hero/Entity Filter, Patch-/Schema-Pin, Drift fail closed/quarantine, Raw Hash, Provenienz, observed timestamp. Kein DB-Fallback.
3. Fact-Relevance: Regressionen richtige/falsche Entity/Feld, spezifischer Feldname, Zahl aus konkreter Zeile, Alias/Mehrdeutigkeit, identische externe ID mit gleichem Namen zusammenführen, anderer Name nicht, Konflikt insufficient evidence auch retrieval limit 1, Patch/Mode/ACL/Revokes. Bestehendes nicht neu bauen.
4. V1 Starting Stats aus Assets-Core-Records vollständig nachweisen. Kein pauschaler Sheet-Import oder Legacy-Fallback; nur tatsächlich fehlenden Pflichtwert gezielt adaptieren.
5. Patchnotes brain.feed.patchnotes.v1: Hash/Post-ID/source_revision/export_revision/URL/title/timestamp, Limits, über 5000 Posts, oversized post, Auth/Config/Ordnung, exakte Brain-Fixture-Kompatibilität.
6. Steam brain.build_publish.v1 POST /builds/v1/publish und GET /builds/v1/publish/{request_id}: UNIQUE request_id + request_sha256 + steam_task in Transaktion, Wiederholung/409/Parallelität genau eine Task, Status nach Neustart, queued/running/succeeded/failed, hero_build_id, stabile Zeit, Auth, Payload/Unknown Fields. Keine quergeteilten DB-Credentials. Kein echter Publish.
7. Bots C9: unset mode Legacy-Default erlaubt, explizit leer/unbekannt fail closed; typed nur BrainClient ohne Fallback; shadow Legacy sichtbar und Probe vollständig detached ohne Antwortverzögerung; IDs kollisionsresistent; unavailable/build_rejected korrekt; Release Gate sicher/report-only; Draft erhalten. Typed Fixtures, Workspace, config, knowledge eval/hybrid, fmt/clippy.
8. Docs: docs.public, typed BrainClient, sicherer Secretpfad, kein lokales RAG/Modell/CLI-Token. 2nd-Brain trusted internal scopes, public und *.public und wildcard reject, kein Corpus-Export/RAG/Fallback/Token in Args/Logs. Twitch shadow detached, keine 115s Verzögerung, typed ohne Fallback, invalid mode fail closed, typed fixtures, Rust SQLx required.
9. Secret-sicherer Testpfad über isoliertes Wegwerf-Postgres, Unix Socket/Peer Auth und geschützte Config, keine produktiven Credentials. Keine große neue Secret-Infrastruktur.
10. Finaler E2E auf endgültigem Head: public/internal Frage, beide Leak-Richtungen blockiert, exakte Zahl, EN/DE Alias, unbekannte Entity, fehlende Evidenz, falscher Patch/Mode, invalid token, legal/illegal build, Hero Card, Providerausfall, ACL revoke, delete, DB outage/recovery. Bestehende 18/18 plus alle genannten Nachweise; kein Budgetanstieg/Fallback.
11. Last auf finalem Head: separates Brain Postgres, Pool 4, je 600 Requests bei 8/16/32 Workern. Je 600 answered, Poolpeak <=4, null too many clients, keine Explosion, null falsche unauthorized_evidence, keine unerklärten 429/503, bounded wait und Recovery. Nicht max_connections erhöhen.
12. Finaler Brain-Head: cargo fmt --all -- --check; cargo clippy --workspace --all-targets --locked --offline -- -D warnings; cargo test --workspace --locked --offline; cargo build --workspace --release --locked --offline. Dazu brain serve/storage upgrade/core postgres/wiki runtime/legacy/tombstone/sources/replay/quality/cutover audit/consumer offline. Release nur im eigenen Worktree, hostweiter bestehender Build-Lock.
13. Wiki: bounded capture -> Wiki IR -> Raw/Facts -> normaler Store -> Release. Kein zweiter Store. Fehlende Betreiberfreigabe bedeutet WIKI_REAL_PILOT_PASSED=NEIN, nicht Fake-Erfolg.
14. Provider: Freigaben für Anbieter/Modell/Egress/Budget/Credentials prüfen; nur bei Vorliegen kleiner echter Shadow-Test, sonst PROVIDER_SHADOW_PASSED=NEIN. Replay: nur freigegebene vorhandene .dem verwenden, sonst Betreiberentscheidung Replay V1 (G2 blockiert) oder deferred erforderlich.
15. GitGuardian #40 Incident 37635766 erneut prüfen, False Positive gegebenenfalls normal schließen, nie History umschreiben/Check umgehen.
16. Alte Brain-PRs einzeln auf enthalten/superseded prüfen, erst mit Beleg schließen und Kommentar `superseded by migration/rust-integration <SHA>`. #40 offen, superseded Cutover-Branch erhalten.
17. Abschlussdoku konsistent aktualisieren: PRE_G5_TECHNICAL_REVIEW.md, STATUS.md, GATES.csv, PFAD_OWNER.csv, BRAIN_DB_MIGRATION_REPORT.md, BRAIN_POSTGRES_ISOLATION.md, FINAL_LOCAL_INTEGRATION_REVIEW.md. Historische Belege klar abgrenzen, aktuelle SHA und Nachweise. Keine unbelegten JA-Marker.

## Abgabe und Eskalation

Worker liefern Datei-/Commit-/PR-Liste, konkrete Befehle und Exitcodes, bestandene/ignorierte/fehlgeschlagene Tests, unverfälschte Blocker. Fixer prüfen ihren eigenen Diff über vorhandenen gate_hook.py --review vor Abgabe. Kein Merge durch Worker. Bei echter Paketgrenze: `[Bump-up] Paket <x>: Grund: ... Erledigt: ... Worktree: ... Offen: ...` im eigenen Ergebnis für Intent-Thread 562a877b-0939-440a-964d-1145d9e9431a, dann stoppen. Keine fremde Session-Kommunikation.
