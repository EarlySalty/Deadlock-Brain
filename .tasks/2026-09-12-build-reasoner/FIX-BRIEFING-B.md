# Fix-Briefing: build-reasoner (Paket B, Fixrunde 1)

[Orchestrator] Fixer für Paket B nach Review Runde 1. Mängelliste:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-B.md`
(vollständig lesen, elf Mängel plus die Liste "Formeln ohne Unit-Test").
Spec `MECHANIK.md` (alle Regeln) und `ARCHITEKTUR.md` Abschnitte 5, 7,
Worker-Briefing `BRIEFING-B.md`, Fertigmeldung `FERTIG-B.md`.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-b` (Branch
  `feat/build-reasoner-b`, Commit 68dc58c, Basis 3bef602, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Erster Schritt: A nachziehen

Paket A ist nach Review freigegeben und liegt auf `feat/build-reasoner-a`
(fc74b71, `data.rs` mit Alias-Auflösung und verifizierten Schemas; ein
weiterer Gate-Fix an `ai_roles.rs` kommt gerade dazu). Führe zuerst `git
merge feat/build-reasoner-a` in deinen Branch aus (kein Rebase), damit du
gegen den aktuellen Stand von `data.rs` arbeitest. Konflikte gibt es nicht,
B hat `data.rs` nie angefasst.

## Was du tust

Alle elf Mängel aus REVIEW-B.md, Reihenfolge nach Schwere:

1. Mangel 2 (blockierend): `scaling_step` braucht eine Datenquelle.
   Entscheidung des Delegators: `data.rs::ability_snapshots` wird als `pub
   async fn load_hero_abilities` freigegeben (additiv in `data.rs`, Datei
   gehört A, diese eine Ergänzung ist dir ausdrücklich erlaubt) und in
   `lib.rs` exportiert (auch das ausnahmsweise erlaubt, nur diese Zeile).
   `build_hero_model` verkettet damit wie in der Spec.
2. Mangel 1 (blockierend): Kern-Schwelle als benannte Größe im Scoring und
   der Echtdaten-Warden-Lauf mit `DEADLOCK_CENTRAL_DSN` (Zugang wie in
   `scripts/`, Secrets aus Infisical, nie ausgeben, Verbindung read-only):
   Scores der vier Referenz-Items Veil Walker, Mercurial Magnum, Siphon
   Bullets, Quicksilver Reload, die fünf besten und fünf schlechtesten Items
   für Warden, in die Fertigmeldung. Liegen die vier nicht über der
   Kern-Schwelle, Ursache je Regel benennen und beheben, wenn die Regel
   falsch ist; Gewichte nicht auf das Ergebnis hin drehen.
3. Mängel 3 bis 6 (wichtig): Tickrate gegen Proc-Cooldown verdrahten,
   heldenabhängige Bedingungen aus dem Modell statt Konstanten, `total`
   nicht aus `per_soul` und `per_slot` mischen (zwei Größen, eine Rangfolge
   wie in MECHANIK.md), Kaufboni je Slot auf eine Einheit bringen.
4. Mängel 7 bis 11 (nit) und die Formeln ohne Unit-Test: je Formel ein
   Test mit Zahlen aus MECHANIK.md.

## Regeln

- Dateien: `mechanics.rs`, `hero.rs`, `item.rs`, Tests, plus die eine
  Ergänzung in `data.rs` und `lib.rs`. `types.rs` bleibt unverändert; fehlt
  etwas, Bump-up.
- Keine Code-Kommentare. Kein KI-Aufruf. Toolchain `export
  PATH=/home/nathanael/.cargo/bin:$PATH`, im Verzeichnis `rust/`: `cargo fmt`
  (nur eigene Dateien), `cargo clippy -p dbrain-reasoner --all-targets -- -D
  warnings`, `cargo test -p dbrain-reasoner` ohne und mit DSN
  (`-- --include-ignored`). Kein `--release`.
- Selbstprüfung: jeden Mangel gegen den eigenen Diff abhaken.
- Nur `feat/build-reasoner-b` committen und pushen, nie main. Commit-Trailer
  `Co-authored-by: <dein Modell> <modell@local>`.
- Echte Umlaute, englische Spielnamen, keine Gedankenstriche.

## Fertigmeldung

In diesem Thread und als Anhang "Fixrunde 1" in `REVIEW-B.md`: je Mangel
Datei:Zeile, Änderung, Commit-SHA, Testzahlen (Baseline und Endstand, ohne
und mit DSN), die Warden-Zahlen aus Punkt 2 als Tabelle.
