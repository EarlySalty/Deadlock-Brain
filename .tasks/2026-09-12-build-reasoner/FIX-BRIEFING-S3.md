# Fix-Briefing: Autoren-Scan je Held (Paket S3, Fixrunde 1)

[Orchestrator] Fixer für Paket S3 nach Review Runde 1. Mängelliste:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-S3.md`
(vollständig lesen). Auftrag `BRIEFING-S3.md`, Fertigmeldung `FERTIG-S3.md`.

- Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-je-held` (Branch
  `fix/autoren-scan-je-held`, Commit 0cd59bf, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Entscheidungen des Delegators

1. Mangel 1 (blockierend): ein Helden-Fehler (Timeout, Response-Code
   ungleich 1) wird erfasst, der nächste Held folgt. Abbruch des Blocks erst
   nach drei aufeinanderfolgenden Fehlschlägen (Indiz für tote GC-Session),
   Konstante im Code mit Namen. Scan-Fehler zählen nicht in den
   `error_count` der Autoren; der Autorenstatus leitet sich allein aus den
   eigenen Treffern ab (`ok` mit Builds, `partial` ohne Treffer, `error` nur
   bei eigenem Upsert-Fehler). Nur ein abgebrochener Block markiert alle
   Autoren mit `partial` und der Nachricht "Scan abgebrochen nach N
   Helden-Fehlern". Den Test `discovery_gc_errors_are_recorded_for_all_authors_in_a_block`
   auf diese Erwartung umschreiben, dazu ein Test für den Abbruch nach drei
   Fehlern.
2. Mangel 2 (wichtig): ein Zyklus plant genau einen Discovery-Task, solange
   alle Helden ins Budget passen: Blockgröße = alle Helden, wenn Heldenzahl
   mal 3,6 s plus 60 s Reserve unter 480 s liegt (heute 38 Helden, 197 s),
   sonst Blöcke zu höchstens 100 Helden. Bei
   mehreren Blöcken schreibt ein Block nur den Status der Autoren mit
   Treffern (`ok`); `partial` "keine Builds im Katalog" setzt allein der
   letzte Block des Zyklus für Autoren, deren `last_checked_at` vor dem
   Zyklusstart liegt (Zyklusstart als Payload-Feld `cycle_started_at`,
   `is_last_block: true` im Payload des letzten Blocks). Test mit zwei
   Blöcken: Autor mit Treffern nur in Block 1 bleibt `ok`.
3. Nit 3: `error_count` und angehängte Meldungen deckungsgleich machen.
4. Nit 4: im Blocklog nur die Zählung je Response-Code loggen, nicht die
   Liste.
5. Nit 5: Wording in `FERTIG-S3.md` auf "alle bekannten Helden" ändern,
   kein Codeumbau.

## Regeln

- Dateien: `discovery.rs`, `catalog.rs`, Tests. Keine Änderungen an Lanes,
  Runner, Proto.
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `rustfmt` nur auf eigene Dateien, Clippy auf
  `steam-core` (Baseline-Hänger `gc_health.rs:45` bekannt), Offline-Tests von
  `steam-core` mit `--features testing` (Reviewer: 191 bestanden) und die
  DB-Katalogtests wie in S3 (36) mit der Test-DB aus Deadlock-Bots. Kein
  `--release`. Keine Schreibzugriffe auf die Prod-DB.
- Selbstprüfung vor der Fertigmeldung. Neue Commits obendrauf, kein
  `--amend`. Nur `fix/autoren-scan-je-held` pushen, nie main. Commit-Trailer
  `Co-authored-by: <dein Modell> <modell@local>`. Echte Umlaute, keine
  Gedankenstriche.

## Fertigmeldung

In diesem Thread und als Anhang "Fixrunde 1" in `REVIEW-S3.md`: je Mangel
Datei:Zeile, Änderung, Commit-SHA, Testzahlen (Baseline und Endstand).
