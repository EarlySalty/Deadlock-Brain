# Fix-Briefing: build-reasoner (Paket C, Fixrunde 1)

[Orchestrator] Fixer für Paket C nach Review Runde 1. Mängelliste:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-C.md`
(vollständig lesen). Spec `ARCHITEKTUR.md` Abschnitte 5, 8, 10, 11,
`MECHANIK.md` Abschnitte 13 und 14, Worker-Briefing `BRIEFING-C.md`,
Fertigmeldung `FERTIG-C.md`.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-c` (Branch
  `feat/build-reasoner-c`, Commit 1e23609, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Was du tust

Mängel 1 bis 5 aus REVIEW-C.md. Mängel 6 und 7 liegen bei Paket D und sind
nicht dein Thema.

1. `patch.rs:317` (blockierend): den `sustained_dps`-Guard in
   `apply_patch_delta` richtig herum drehen, so dass Waffen-Buffs
   (bullet_damage, fire_rate, reload) in `weapon_dps` und `primary_axis`
   ankommen. Regressionstest mit einem Warden-ähnlichen Modell: vor dem Fix
   rot, danach grün, Zahlen in der Fertigmeldung.
2. `backtest.rs:49` (wichtig): "nicht messbar" muss ausdrückbar sein.
   Entscheidung des Delegators: `BacktestMetrics.order_proximity` in
   `types.rs` wird `Option<f64>` (`None` = nicht messbar); Aggregate mitteln
   nur über `Some`, der Report zeigt "nicht messbar" statt einer Zahl. Das ist
   die einzige erlaubte Änderung an `types.rs`; sie ist additiv für Paket B
   (B nutzt `BacktestMetrics` nicht) und wird in der Fertigmeldung als
   Schnittstellenänderung für D ausgewiesen.
3. `composer.rs:137` (wichtig): `sell_priority` je Item aus der Phase setzen
   (Lane- und frühe Items zuerst verkaufen, Kern nie), `ability_order` aus
   der besten Quelle füllen: erst der am höchsten gewichtete Autoren-Build
   des Helden aus dem `MetaIndex` (`details.abilityOrder`), sonst
   `brain.hero_ability_orders`, sonst leer mit Beleg "keine Quelle". Beides
   muss bis in den Publish-Payload durchkommen (Roundtrip-Test).
4. `meta.rs` (nit): Verbreitung auf höchstens 1,0 klemmen.
5. `composer.rs` (nit): Situationsblock-Heuristik so nachziehen, dass der
   Warden-Referenzbuild (`referenz/lightbringer-warden.json`) in seine fünf
   Blöcke fällt; Test gegen den Seed.

## Regeln

- Nur `patch.rs`, `meta.rs`, `composer.rs`, `backtest.rs`, `publish.rs`,
  `types.rs` (nur Punkt 2) und Tests. `lib.rs` nur temporär für den Testlauf
  ergänzen, nicht committen.
- Keine Code-Kommentare. Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`,
  im Verzeichnis `rust/`: `cargo fmt` (nur eigene Dateien), `cargo clippy -p
  dbrain-reasoner -p dbrain-builds --all-targets -- -D warnings`, `cargo
  test -p dbrain-reasoner -p dbrain-builds`. Kein `--release`.
- Selbstprüfung vor der Fertigmeldung: jeden Mangel gegen den eigenen Diff
  abhaken.
- Nur `feat/build-reasoner-c` committen und pushen, nie main. Commit-Trailer:
  `Co-authored-by: <dein Modell> <modell@local>`.
- Echte Umlaute, englische Spielnamen, keine Gedankenstriche.

## Fertigmeldung

In diesem Thread und als Anhang unten in `REVIEW-C.md` (Abschnitt
"Fixrunde 1"): je Mangel Nummer, Datei:Zeile, was geändert, Commit-SHA,
Testlauf mit Zahlen (Baseline und Endstand), die Schnittstellenänderung an
`types.rs` ausdrücklich.
