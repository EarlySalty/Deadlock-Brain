# Fix-Briefing: build-reasoner (Paket A, Fixrunde 1)

[Orchestrator] Fixer für Paket A nach Review Runde 1. Mängelliste:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-A.md`
(vollständig lesen). Spec `ARCHITEKTUR.md` Abschnitte 5, 6, 9, Vorcheck
`VORCHECK-ERGEBNIS.md` Punkte 1, 4, 6, Worker-Briefing `BRIEFING-A.md`.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-a` (Branch
  `feat/build-reasoner-a`, Commit 3bef602, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Was du tust

Mangel 1 (Paketschnitt) ist entschieden und nicht dein Thema: das HeroModel
in `data.rs` bleibt, Paket B baut darauf auf. Du bearbeitest Mängel 2 bis 7
aus REVIEW-A.md:

2. `load_patch_events`: Alias-Auflösung über `brain.entities` und
   `brain.entity_aliases` wie in `dbrain-learn/src/build_optimizer.rs:2403`
   (Namensmenge aus canonical_name plus alias_kind canonical, snapshot_name,
   class_name_short), nicht nur der kanonische Name.
3. `load_author_builds`: ORDER BY auf die echten Spalten von
   `tierlist.hero_build_sources` (`last_updated_at`, `published_at`,
   `version`), kein Mischen von timestamptz und int; `table_exists` darf
   keinen Schemafehler verdecken.
4. Spalten von `brain.hero_stat_values` und `brain.hero_stat_profiles` am
   echten Schema verifizieren (DB-Zugang wie in `scripts/`, Secrets aus
   Infisical, nie ausgeben) und die Queries daran ausrichten; den bisher
   ignorierten Echtdaten-Test mit DSN einmal wirklich laufen lassen und die
   Zahl nennen.
5. `reasoning_effort: "none"` gegen den Fireworks-Endpunkt belegen: ein
   kleiner echter Aufruf über den bestehenden Client (Key aus Infisical) oder
   der Nachweis aus der Fireworks-Doku; wenn der Endpunkt den Wert ablehnt,
   auf den Wert umstellen, den er akzeptiert, und das in der Fertigmeldung
   festhalten.
6. Item-Analyst-Filter ("Item nicht im Build, Text verwerfen") in
   `ai_roles.rs` nur als Parser-Regel belassen, wenn der Reviewer das so
   akzeptiert; sonst als klar benannte `pub fn` bereitstellen, die Paket C
   aus `composer.rs` aufruft. Entscheide nach dem Wortlaut in REVIEW-A.md.
7. Snapshot-Suche: Exakttreffer auf `canonical_name` schlägt jede
   Fuzzy-Übereinstimmung.

Keine Änderungen an `types.rs`-Signaturen (B und C bauen dagegen). Keine
neuen Dateien außer Tests.

## Regeln

- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `cargo fmt` (nur eigene Dateien), `cargo clippy -p
  dbrain-reasoner --all-targets -- -D warnings`, `cargo test -p
  dbrain-reasoner` mit und ohne `DEADLOCK_CENTRAL_DSN`. Kein `--release`.
- Selbstprüfung vor der Fertigmeldung: jeden Mangel aus REVIEW-A.md gegen den
  eigenen Diff abhaken.
- Nur `feat/build-reasoner-a` committen und pushen, nie main. Commit-Trailer:
  `Co-authored-by: <dein Modell> <modell@local>`.
- Echte Umlaute, englische Spielnamen, keine Gedankenstriche.

## Fertigmeldung

In diesem Thread und als Anhang unten in `REVIEW-A.md` (Abschnitt
"Fixrunde 1"): je Mangel Nummer, Datei:Zeile, was geändert, Commit-SHA,
Testlauf mit Zahlen (ohne DSN und mit DSN).
