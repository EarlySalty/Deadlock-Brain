# Review-Briefing: Autoren-Scan je Held (Paket S3, Runde 1)

[Orchestrator] Review Runde 1 für Paket S3 (Deadlock-Steam-Bot). Lesend, kein
Code, kein Branch. Du bist der einzige Thread für dieses Review. Keine
Unter-Threads oder Unter-Agenten spawnen.

- Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-je-held` (Branch
  `fix/autoren-scan-je-held`, Commit 0cd59bf, Basis main ad5f00e)
- Diff: `git -C /home/nathanael/.worktrees/steam-bot-autoren-je-held diff ad5f00e..0cd59bf`
- Auftrag `BRIEFING-S3.md`, Fertigmeldung `FERTIG-S3.md`, Vorgeschichte
  `FERTIG-S2.md` und `REVIEW-S2.md`, alle in
  `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/`.
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`

## Was du prüfst

1. `discovery.rs`: GC-Anfrage je Held ohne `author_account_id` (Proto
   `citadel_gcmessages_client.proto:1361` beide Felder optional); Dedup
   `(hero_build_id, language)` über den ganzen Block; jede Build-Quelle wird
   wie bisher per `upsert_hero_build_source` gespeichert, keine Löschung.
2. Autorenstatus clientseitig: `update_watched_author_status` je
   beobachtetem Autor genau einmal je Block; `ok`, `partial` ("keine Builds
   im Katalog"), `error` korrekt abgeleitet; ein GC-Fehler bei einem Helden
   darf nicht alle Autoren auf `error` setzen, ein Block ohne jeden Treffer
   darf nicht `ok` melden. Was passiert bei mehreren Blöcken (über 100
   Helden): überschreibt Block 2 den Status aus Block 1 mit `partial`?
3. Planung `catalog.rs:96` und `discovery_tasks`: `ceil(Helden/100)` Tasks,
   Payload `hero_ids`, alte Payloads mit `author_account_id` weiter
   verarbeitbar; das 480-s-Budget je Task aus S2 bleibt; 100 Helden mal
   3,6 s ist 360 s, prüfe die Reserve gegen GC-Timeouts (je Anfrage
   Timeout mal 100 darf 480 s nicht sprengen, sonst Stale-Reap wie in S2).
4. Journal je Block "Helden-Scan beendet" mit Helden, Builds,
   Autoren-Treffern; keine Zeile je Held (Rauschen).
5. Tests: `catalog.rs:1185` und `:1213` prüfen Anfrageform und Zuordnung;
   Baseline 174 Offline und 34 DB-Katalogtests, Endstand 174 und 36. Führe
   die Offline-Tests selbst aus (`cargo test -p steam-core --features
   testing` ohne DB) und nenne die Zahl; DB-Tests nur, wenn die Test-DB
   erreichbar ist (Memory `steam-bot-deploy-und-test-weg`), sonst die Zahl
   des Fixers als Fremdnachweis kennzeichnen. Clippy-Warnfall
   `gc_health.rs:45` ist Baseline aus S2, kein S3-Mangel.

## Ergebnis

`REVIEW-S3.md` im Hauptordner: Mängelliste mit Datei:Zeile, Schwere
(blockierend, wichtig, nit), Fix-Vorschlag. Urteil: FREIGABE oder
NACHBESSERN. Fertigmeldung in diesem Thread mit Urteil. Deutsch, echte
Umlaute, keine Gedankenstriche.
