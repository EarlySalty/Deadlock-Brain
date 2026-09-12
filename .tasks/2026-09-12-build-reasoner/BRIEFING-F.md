# Briefing: Kern als Einkaufskurve nach Kostenband (Paket F, mittel)

[Orchestrator] Kalibrierungspaket nach dem Merge von E. Der Kern ist jetzt
rechnerisch sauber (0 falsche Deltas, nur positive Scores), aber er verletzt
die Grundlogik des Spiels: 15 der 19 Kern-Items sind Tier 4 oder 5, nur ein
Tier-1- und ein Tier-2-Item. Der Referenzbuild (Seed Lightbringer x Situation,
Build 779996) hat im Kern 3 Tier 1, 6 Tier 2, 2 Tier 3, 8 Tier 4. Ein Kern
ist eine Einkaufskurve über das Spiel, kein globales Ranking nach Slot-Wert;
teure Items gewinnen den Slot-Wert immer, aber niemand kauft 15 teure Items,
bevor die Lane vorbei ist.

Befund (aus `WARDEN-E.json`, Ränge nach `total`): Titanic Magazine 14,
Quicksilver Reload 37, Monster Rounds 44, Enduring Speed 53, Opening Rounds 65,
Veil Walker 75, Fleetfoot 89, Swift Striker 97, Extra Regen 133,
High-Velocity Rounds 139. Alle liegen unter dem globalen Cutoff 19, obwohl sie
in ihrem Kostenband vorne stehen.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-f` (Branch
  `feat/build-reasoner-f` ab main 80417b9 mit E, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.
- Kontext: `ARCHITEKTUR.md` Abschnitt 8 und 9, `MECHANIK.md` Abschnitt 13,
  `FERTIG-E.md`, `REVIEW-E.md`, `REPORT-D.md` (Kalibrierungspunkte), Seed
  `referenz/lightbringer-warden.json`.

## Was du tust

1. Kostenband-Layout aus echten Autoren-Builds ableiten, nicht raten: über
   alle Builds in `tierlist.hero_build_sources` (read-only, alle Helden,
   aktuelle Version je Build) je Build die Kern-Kategorie erkennen (die
   erste Kategorie beziehungsweise Kategorien, deren Name Core, Kern,
   Standard oder ähnlich enthält; sonst die größte Kategorie) und die Anzahl
   Items je Tier zählen. Ergebnis: Median und Interquartil je Tier, gesamt
   und je Held. Das ist die Zielverteilung; für Warden mit den vorhandenen
   Warden-Builds, sonst der Gesamtmedian. Werte in `FERTIG-F.md` als
   Tabelle, plus als `brain.reasoner_layouts` (neue Migration, Upsert je
   Held und Patch-Tag) oder als reine Ableitung zur Laufzeit; entscheide nach
   Laufzeit und begründe.
2. Composer: der Kern wird bandweise gefüllt. Je Tier so viele Plätze wie
   das Layout vorgibt, Auswahl im Band nach `total` (Tier 3 und 4) und nach
   `per_soul_value` (Tier 1 und 2, weil dort der Soul-Ertrag zählt und die
   Lane-Phase). Reihenfolge im Kern nach Kaufphase (Lane, Mid, Core, Late)
   und innerhalb der Phase nach Score. Slot-Grenzen (4 Waffe, 4 Vitalität,
   4 Spirit, Flex ab Tier-Regel des Snapshots) dürfen nicht überschritten
   werden; ist ein Band voll, rückt das nächste Item des Bandes in Optional.
3. Der bisherige globale Cutoff 19 fällt weg; die Kerngröße folgt der Summe
   des Layouts. Die Blöcke Can buy 1, Tryhard, Shields, Optional bleiben wie
   in E (Optional höchstens 12).
4. Zustandsfaktor 0,6 auf Imbue-Items und der Ausschluss der Lane-Phase vom
   Kern (REPORT-D) prüfen: sind sie mit Punkt 2 noch nötig? Wenn ja, lassen
   und begründen; wenn sie das Ergebnis verzerren, ändern und die Wirkung
   je Referenz-Item beziffern. Keine sonstigen Gewichtsänderungen.
5. Nachweis mit dem Debug-Binary, read-only, `--no-persist`: `reason build
   Warden --no-ai --json` und `reason backtest --hero Warden --json`.
   Tabelle Kern je Tier vorher (E) und nachher, Rang und Band-Rang je
   Seed-Kern-Item, Backtest Kern-Überdeckung und Jaccard gegen Seed, Build
   779996 und das Aggregat (E: 0,2105 / 0,1176 Seed, 0,2105 / 0,0930 779996,
   Aggregat 0,1763). Zusätzlich Backtest für zwei weitere Helden mit vielen
   Autoren-Builds, damit die Regel nicht nur Warden passt. Verschlechtert
   sich ein Held, steht das im Bericht, keine Sonderregel je Held.

## Regeln

- Dateien: `dbrain-reasoner/src/composer.rs`, `types.rs`, `data.rs`
  (Layout-Ableitung), `lib.rs`, bei Bedarf neue Migration unter
  `scripts/migrations/`, Tests. `item.rs`, `mechanics.rs`, `patch.rs` nur
  mit Begründung im Bericht (Punkt 4).
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `rustfmt` nur auf eigene Dateien, Clippy auf
  `dbrain-reasoner` mit `--all-targets -- -D warnings`, Workspace-Tests ohne
  DSN (Baseline nach E: 273 bestanden, 58 ignoriert) und die Reasoner-Tests
  mit `--include-ignored` gegen den Central-Pool read-only plus lokale
  Scratch-DB wie in E (101). Kein `--release`. Keine Schreibzugriffe auf den
  Central-Pool.
- Selbstprüfung vor der Fertigmeldung. Nur `feat/build-reasoner-f` committen
  und pushen, nie main. Commit-Trailer `Co-authored-by: <dein Modell>
  <modell@local>`. Echte Umlaute, keine Gedankenstriche.

## Fertigmeldung

In diesem Thread und als `FERTIG-F.md` im Task-Ordner: Layout-Tabelle mit
Quelle (Anzahl Builds), Änderung mit Datei:Zeile, Commit-SHA, Testzahlen
(Baseline und Endstand), die Nachweis-Tabellen aus Punkt 5.
