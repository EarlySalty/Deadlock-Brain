# Briefing: build-reasoner (Paket C, Zusammensetzung und Belege)

[Orchestrator] Paket C. Auftrag:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/AUFTRAG.md`
(vollständig lesen), verbindliche Spec `ARCHITEKTUR.md` (Abschnitte 5, 8, 9,
10, 11, 13) und `MECHANIK.md` (Abschnitte Patch-Delta, Meta-Signale, Grenzen)
im selben Ordner, Paketschnitt und Entscheidungen in `PAKETE.md`, Fundstellen
in `VORCHECK-ERGEBNIS.md`, Fertigmeldung von Paket A in `FERTIG-A.md`.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-c` (existiert, Branch
  ist ausgecheckt, enthält den gemergten Stand von Paket A)
- Branch: `feat/build-reasoner-c`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Was du baust

Im Crate `rust/crates/dbrain-reasoner`, gegen die Typen aus `types.rs`
(Paket A) und die in ARCHITEKTUR.md Abschnitt 7 fixierten Signaturen von
`hero.rs`, `item.rs`, `mechanics.rs` (Paket B baut parallel; bis B gemergt
ist, testest du gegen Fixtures der A-Typen):

1. `src/patch.rs`: `compute_patch_delta` und `apply_patch_delta` aus
   `brain.patch_events` und `patch_event_enrichments` (Ziel, Mechanik,
   Vorzeichen, Größe aus der geparsten Patchzeile, nie geschätzt). Vorlage
   `dbrain-enrich` (`load_patch_events`, `build_patch_event_enrichments`).
   Ersetzt die Mock-Shifts in `run_meta_trend_analysis`, ohne die alte
   Funktion anzufassen.
2. `src/meta.rs`: `build_meta_index` mit Gewichtung und Mindeststichproben nach
   MECHANIK.md Abschnitt 14: `hero_item_stats`, Autoren-Builds aus
   `tierlist.hero_build_sources` (details, Patch-Stand über `last_updated_at`
   gegen `patch_events`), Seed-Builds aus
   `.tasks/2026-09-12-build-reasoner/referenz/*.json` (Pfad konfigurierbar),
   Creator-Claims über `dbrain-retrieval`. Nichts in `tierlist` schreiben.
3. `src/composer.rs`: `compose_build` baut das BuildObject mit Kern,
   Situationsblöcken (wie im Warden-Referenzbuild: Kern, "Can buy 1", Tryhard,
   Shields, Optional), Reihenfolge, Konfidenz und Quellenbelegen je Item;
   Sperrliste aus Kritiker-Issues für die eine erlaubte Recompose-Runde.
4. `src/backtest.rs`: die drei Kennzahlen aus Abschnitt 11 (Kern-Überdeckung
   plus Jaccard, Reihenfolge-Nähe, Patch-Wechsel-Erkennung), je Held und
   Autor, Warden als Pflichtzeile; Reihenfolge der Autoren-Seite aus dem
   `details`-Array, sonst "nicht messbar" (Entscheidung 3).
5. `src/publish.rs`: Abbildung BuildObject auf `dbrain-builds::spec`
   (Abschnitt 10). Erweiterung von `BuildSpecMod`/`BuildSpecCategory` in
   `rust/crates/dbrain-builds/src/spec.rs` gehört zu diesem Paket; der
   bestehende Publish-Weg und der Steam-Bot bleiben unverändert.
6. Tests: Patch-Delta an echten Warden-Patch-Events (die letzten drei Patches,
   Fundstellen im Vorcheck), Meta-Index mit Mindeststichproben-Dämpfung,
   Composer gegen Fixture-ScoredItems, Backtest gegen den Warden-Seed mit
   erwarteten Zahlen, Publish-Roundtrip in den bestehenden Payload.

## Regeln

- Nur die Dateien dieses Pakets. `lib.rs` nicht committen (temporär für
  Tests ergänzen ist erlaubt). `types.rs`, `hero.rs`, `item.rs`,
  `mechanics.rs` sind fremd: fehlt etwas, Bump-up statt Änderung.
- Keine Code-Kommentare. Kein neuer KI-Pfad: Texte kommen nur über
  `ai_roles.rs` aus Paket A.
- Toolchain: `export PATH=/home/nathanael/.cargo/bin:$PATH`, im Verzeichnis
  `rust/`. `cargo fmt` (nur eigene Dateien), `cargo clippy -p dbrain-reasoner
  -p dbrain-builds`, `cargo test -p dbrain-reasoner -p dbrain-builds`. Kein
  `--release`.
- Nur den eigenen Branch committen und pushen (`git push origin
  feat/build-reasoner-c`), nie main. Commit-Trailer:
  `Co-authored-by: GPT 5.6 Luna <luna@local>`.
- Echte Umlaute, englische Spielnamen, keine Gedankenstriche.

## Bump-up

```
[Bump-up] Paket C: Grund: ... Erledigt: ... Worktree: /home/nathanael/.worktrees/deadlock-brain-c Offen: ...
```

## Fertigmeldung

In diesem Thread und als Datei `FERTIG-C.md` im Task-Ordner: Branch,
Commit-SHAs, geänderte Dateien, Testlauf mit Zahlen (Baseline und Endstand),
Backtest-Zahlen für den Warden-Seed, Entscheidungen bei Spec-Lücken, offene
Punkte für D.
