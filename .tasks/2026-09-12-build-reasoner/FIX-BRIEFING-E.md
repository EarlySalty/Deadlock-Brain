# Fix-Briefing: build-reasoner (Paket E, Live-Befund Patch-Delta)

[Orchestrator] Fixer für den ersten Live-Lauf des Reasoners nach dem Merge
4689b80 (main, Release-Binary 23:31, Migration angewendet, Timer aktiv). Der
Warden-Build ist live falsch, die Ursache liegt in der Patch-Anwendung.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-e` (existiert, Branch
  `fix/build-reasoner-patch-delta` ab main e958158, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.
- Kontext: `ARCHITEKTUR.md` Abschnitte 8, 9, 11, `MECHANIK.md` Abschnitte 13
  und 14, `REPORT-D.md` (Worktree-Stand jetzt in main unter
  `.tasks/2026-09-12-build-reasoner/`), `REVIEW-C.md`, `REVIEW-D.md`.

## Live-Befund, wörtlich aus DB und CLI (2026-09-12 23:32)

1. `reason build Warden --no-ai --json` (Release-Binary, Central-DSN):
   Kern = 19 Items, angeführt von Trophy Collector, Golden Goose Egg, Mystic
   Conduit, Frostbite Charm, Prism Blast, Omnicharge Signet, Diviner's Kevlar,
   Cursed Relic, Boundless Spirit, Seraphim Wings; Block "Optional" enthält
   144 Items; `confidence` je Item oft `High`.
2. `brain.reasoner_item_scores` (hero_id 25, echter Patch-Tag), `total`:
   Inhibitor 637,73; Trophy Collector 340,98; Golden Goose Egg 225,54;
   Veil Walker 14,28; Siphon Bullets minus 95,19; Juggernaut minus 2554,97;
   Titanic Magazine minus 2741,28; Frenzy minus 2745,84; Quicksilver Reload
   minus 3074,41; Express Shot minus 4225,00; Mercurial Magnum minus 4378,81.
   Zum Vergleich das Scoring ohne Patch-Delta (REVIEW-B Runde 2, echter
   Snapshot): Spirit Burn 104, Juggernaut 73, Express Shot 73, Frenzy 65,
   Mercurial Magnum 50, Siphon Bullets 28, Quicksilver Reload 24, Veil
   Walker 14; Trophy Collector minus 2, Golden Goose Egg minus 3.
3. `reason patch-impact Warden` meldet 314 Patch-Deltas und 20 verschobene
   Items. Aber: alle 416 Warden-Patch-Events in `brain.patch_events` liegen
   VOR dem Datum des Helden-Snapshots (Assets-API 2026-07-06,
   `deadlock_data` 2026-08-22); von 397 Events der Kern-Items liegen 395 vor
   dem Item-Snapshot (`item_or_ability` 2026-06-30, `item_card` 2026-08-22).
   Der aktuelle Snapshot enthält diese Patches also bereits; sie trotzdem als
   Delta anzuwenden zählt sie doppelt, kumuliert über Monate.
4. Dieselbe Patchzeile liegt mehrfach vor und ist widersprüchlich
   klassifiziert: "Base Bullet Damage spirit scaling increased from 0.465
   to 0.49" (Mercurial Magnum, 2026-05-22) fünfmal, davon dreimal `buff`
   und zweimal `nerf` (Duplikate durch mehrere Patchnote-Fassungen und den
   LEFT JOIN auf Enrichments, siehe Gate-Fund 4 in REVIEW-B.md).

## Was du tust (Ursache, nicht Symptom)

1. `patch.rs` `compute_patch_delta`/`apply_patch_delta`: Deltas nur für
   Events mit `posted_at` nach dem `fetched_at` des Snapshots, aus dem der
   betroffene Wert stammt (Held: Assets-Snapshot; Item: `item_or_ability`
   beziehungsweise `item_card`, je nachdem, woher das Feld kommt; die
   Zeitstempel liefert `data.rs`, sonst additiv ergänzen). Events davor
   sind Historie für `patch-impact` und den Backtest-Patch-Wechsel, nie ein
   Delta auf aktuelle Werte.
2. Events deduplizieren (Patch, Entität, normalisierte Zeile) und das
   Vorzeichen aus den Zahlen (`old_value`, `new_value`) ableiten, nicht aus
   `change_type`; widersprüchliche Labels für dieselbe Zeile dürfen nicht zu
   zwei Deltas werden.
3. Größe je Delta relativ zum Ausgangswert (Verhältnis `new/old` auf den
   Snapshot-Wert), nicht als Absolutzahl auf ein fremdes Feld; Prozentzeilen
   als Faktor. Kein Delta darf ein Item-Ergebnis unter null drücken oder um
   mehr als einen plausiblen Faktor verschieben; wenn doch, Delta verwerfen
   und als Beleg "nicht anwendbar" ausweisen.
4. `composer.rs`: Kern nur aus Items mit positivem `total`; der Block
   "Optional" wird auf das Layout des Referenzbuilds begrenzt (höchstens 12,
   nach Score), nicht "alle übrigen 144"; `confidence` darf bei negativem
   oder null `total` nie `High` sein, auch wenn Meta-Zeilen existieren.
5. Nachweis: `reason build Warden --no-ai --json` und `reason backtest
   --hero Warden --json` mit dem Debug-Binary gegen den Central-Pool
   (read-only, `--no-persist`): Kern-Items mit `total`, Vergleich zur
   Scoring-Tabelle ohne Delta, Backtest-Kennzahlen gegen den Seed und gegen
   Build 779996. Erwartung: die vier Referenz-Items (Veil Walker, Mercurial
   Magnum, Siphon Bullets, Quicksilver Reload) und Juggernaut im Kern; sonst
   Ursache je Item benennen, keine Gewichte auf das Ergebnis hin drehen.

## Regeln

- Dateien: `dbrain-reasoner/src/patch.rs`, `composer.rs`, bei Bedarf
  additiv `data.rs`, `types.rs`, `lib.rs`, Tests. Keine Änderungen an
  `mechanics.rs`, `item.rs` (Scoring ohne Delta ist reviewt und stimmt).
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `cargo fmt` (nur eigene Dateien), `cargo clippy -p
  dbrain-reasoner --all-targets -- -D warnings`, `cargo test --workspace`
  ohne DSN (Baseline 260 bestanden, 56 ignoriert), `cargo test -p
  dbrain-reasoner -- --include-ignored` mit Central-DSN read-only. Kein
  `--release` (der Delegator baut). Keine Schreibzugriffe auf den
  Central-Pool.
- Selbstprüfung vor der Fertigmeldung. Nur `fix/build-reasoner-patch-delta`
  committen und pushen, nie main. Commit-Trailer `Co-authored-by: <dein
  Modell> <modell@local>`. Echte Umlaute, keine Gedankenstriche.

## Fertigmeldung

In diesem Thread und als `FERTIG-E.md` im Task-Ordner: Ursache je Befund mit
Beleg (Datei:Zeile, Zahlen), Fix mit Datei:Zeile, Commit-SHA, Testzahlen
(Baseline und Endstand), die Warden-Tabelle aus Punkt 5 (Kern mit `total`,
Backtest-Kennzahlen).
