# Briefing: Loader läuft für alle Helden (Paket G, klein)

[Orchestrator] Befund 15 in `PAKETE.md` und Abschnitt "Separates Paket" in
`REVIEW-F.md`: `reason build "Lady Geist"` und `reason build Infernus`
(je `--no-ai --no-persist --json`) brechen mit Exit 1 und "Datenfehler:
Ability 0 gehört nicht zum geladenen Helden" ab. Ursache laut Review:
`hero.rs:20` `integer(ability.get("id"))` liefert 0, wenn eine
Ability-Snapshot-Zeile kein oder ein leeres `id`-Feld hat; `position()` an
`hero.rs:24` findet keine Ability mit id 0 und wirft den Data-Fehler. Warden
hat vollständige ids und läuft. Der Timer synchronisiert alle 38 Helden, der
Reasoner muss also für alle laufen oder je Held sauber sagen, was fehlt.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-g` (Branch
  `feat/build-reasoner-g` ab main 80417b9, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.
- Kontext: `ARCHITEKTUR.md` Abschnitt 8 (Ladeschicht), `MECHANIK.md`
  Abschnitt 13, `FERTIG-E.md` (Snapshot-Herkunft je Feld).

## Was du tust

1. Ursache am echten Snapshot belegen (read-only, Zugang wie in `scripts/`,
   Secrets aus Infisical, nie ausgeben): für Lady Geist und Infernus die
   Ability-Snapshot-Zeilen ansehen; welches Feld trägt die Ability-ID
   (`id`, `ability_id`, `class_name`, Schlüssel im Payload), und warum
   liefert es bei diesen Helden nichts. Prüfe alle 38 Helden mit einer
   Schleife `reason build <Held> --no-ai --no-persist --json` (Debug-Binary)
   und liste je Held Exit-Code und Fehlertext in `FERTIG-G.md`.
2. Fix an der Ursache in `hero.rs` beziehungsweise `data.rs`: die
   Ability-Zuordnung darf nicht an einem einzigen Schlüssel hängen, wenn der
   Snapshot die ID anders trägt; fehlt die ID wirklich, ist die Fehlermeldung
   je Held eindeutig (Heldenname, Ability-Slot, gefundene Schlüssel) statt
   "Ability 0". "Unvollständiges Waffenprofil" (FERTIG-F) genauso: Ursache
   belegen und beheben oder als klare Datenmeldung ausweisen.
3. Nachweis: die Schleife aus Punkt 1 nach dem Fix, Ziel Exit 0 für alle 38
   Helden; Helden, die weiter scheitern, mit Grund. Test mit Fixture, das
   den kaputten Schlüssel nachstellt.

## Regeln

- Dateien: `dbrain-reasoner/src/hero.rs`, `data.rs` (nur Ladepfad der
  Abilities und Waffe), Tests. Keine Änderungen an `composer.rs`, `meta.rs`,
  `item.rs`, `mechanics.rs`, `patch.rs` (Paket F arbeitet dort parallel).
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `rustfmt` nur auf eigene Dateien, Clippy auf
  `dbrain-reasoner` mit `--all-targets -- -D warnings`, Workspace-Tests ohne
  DSN (Baseline 273 bestanden, 58 ignoriert), Reasoner-Tests mit
  `--include-ignored` gegen Central read-only (Baseline: 9 Scratch-Tests
  ohne `REASONER_SCRATCH_DSN` rot, Rest grün). Kein `--release`. Keine
  Schreibzugriffe auf den Central-Pool.
- Selbstprüfung vor der Fertigmeldung. Nur `feat/build-reasoner-g`
  committen und pushen, nie main. Commit-Trailer `Co-authored-by: <dein
  Modell> <modell@local>`. Echte Umlaute, keine Gedankenstriche.

## Fertigmeldung

In diesem Thread und als `FERTIG-G.md` im Task-Ordner: Ursache mit Beleg
(Snapshot-Zeile, Schlüssel), Fix mit Datei:Zeile, Commit-SHA, Testzahlen
(Baseline und Endstand), Tabelle aller 38 Helden vorher und nachher.
