# Review-Briefing: build-reasoner (Paket C, Runde 1)

[Orchestrator] Review Runde 1 für Paket C. Lesend, kein Code, kein Branch.
Du bist der einzige Thread für dieses Review. Keine Unter-Threads oder
Unter-Agenten spawnen.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-c` (Branch
  `feat/build-reasoner-c`, Commit 1e23609, Basis 3bef602 = Paket A)
- Diff: `git -C /home/nathanael/.worktrees/deadlock-brain-c diff 3bef602..HEAD`
- Auftrag: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/AUFTRAG.md`,
  Spec `ARCHITEKTUR.md` (Abschnitte 5, 8, 9, 10, 11, 13) und `MECHANIK.md`
  (Patch-Delta, Meta-Signale, Grenzen), Worker-Briefing `BRIEFING-C.md`,
  Fertigmeldung `FERTIG-C.md`, Befunde 6 bis 10 in `PAKETE.md`, Vorcheck
  `VORCHECK-ERGEBNIS.md` (Punkte 3, 4, 5, 6), Review von A `REVIEW-A.md`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`

## Was du prüfst

1. Spec-Treue gegen die Signaturen aus `types.rs` (Paket A) und Abschnitt 8:
   `compute_patch_delta`, `apply_patch_delta`, `build_meta_index`,
   `compose_build`, Backtest, Publish-Mapping. Abweichungen mit Zeile und
   Folge für Paket D. Die vier Entscheidungen in FERTIG-C.md bewertest du
   inhaltlich.
2. Patch-Delta: Vorzeichen und Größe kommen aus der geparsten Patchzeile,
   nie geschätzt; Item- und Helden-Ziele werden über `entities` und
   `entity_aliases` aufgelöst; Events ohne numerische Item-ID werden nicht
   geraten (Entscheidung 1). Prüfe an den echten Warden-Events vom
   30.06.2026 und den Spiritual-Overflow-Events vom 28.07.2026 (DB-Zugang
   wie in `scripts/`, Secrets aus Infisical, nie ausgeben).
3. Meta-Index: Gewichte und Mindeststichproben wie MECHANIK.md Abschnitt 14,
   Dämpfung statt Nullung, Autoren-Builds aus `tierlist.hero_build_sources`
   (details.modCategories, Patch-Stand über `last_updated_at`), Seed-Builds
   aus `referenz/*.json`, nichts wird in `tierlist` geschrieben.
4. Composer: Kern plus Situationsblöcke wie der Warden-Referenzbuild,
   Reihenfolge, Konfidenz je Item, Belege, genau eine Recompose-Runde mit
   Sperrliste, KI wählt nie Items.
5. Backtest: die drei Kennzahlen aus Abschnitt 11 korrekt gerechnet
   (Kern-Überdeckung, Jaccard, Reihenfolge-Nähe, Patch-Wechsel), ehrlich
   "nicht messbar" statt Zahl, wenn Reihenfolge oder Vergleichsbasis fehlt;
   der Warden-Test in FERTIG-C.md prüft nur die Formel, sag klar, was ein
   echter Lauf zusätzlich braucht.
6. Publish: Abbildung auf `BuildSpecPayload`, Erweiterung von `spec.rs`
   (`imbue`, `sell_priority`, Kategorie-Layout) rückwärtskompatibel, Enqueue
   in `steam.steam_tasks` mit Typ `BUILD_PUBLISH_ORIGINAL` in der Form, die
   `publish_original.rs` im Steam-Bot wirklich liest (lesend prüfen unter
   `/home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/task/handlers/builds/publish_original.rs`).
   Kein Schreibpfad darf beim `--no-publish`-Lauf erreicht werden.
7. Hygiene: keine Secrets, keine Code-Kommentare, keine neuen externen
   Crates, sqlx-Offline-Cache falls compile-geprüfte Queries dazukamen;
   `cargo fmt`, `cargo clippy -p dbrain-reasoner -p dbrain-builds
   --all-targets -- -D warnings` und `cargo test -p dbrain-reasoner -p
   dbrain-builds` selbst laufen lassen (dazu die C-Module temporär in
   `lib.rs` deklarieren, ohne zu committen) und Zahlen nennen (Worker:
   Reasoner 15 bestanden, 1 ignoriert; dbrain-builds 7 bestanden, 5
   ignoriert).

## Ergebnis

Datei `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-C.md`:
vollständige Mängelliste, je Mangel Nummer, Datei:Zeile, Befund, Schwere
(blockierend, wichtig, nit), Vorschlag. Am Ende ein Urteil: FREIGABE oder
NACHBESSERN, plus die Punkte, die Paket D bei der Integration wissen muss.
Fertigmeldung in diesem Thread mit dem Pfad und dem Urteil. Deutsch, echte
Umlaute, keine Gedankenstriche.
