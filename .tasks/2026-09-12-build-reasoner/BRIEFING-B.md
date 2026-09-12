# Briefing: build-reasoner (Paket B, deterministischer Kern)

[Orchestrator] Paket B. Auftrag:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/AUFTRAG.md`
(vollständig lesen), verbindliche Spec `ARCHITEKTUR.md` (Abschnitte 5, 7, 9,
13) und `MECHANIK.md` (alle Rechenregeln) im selben Ordner, Paketschnitt in
`PAKETE.md`, Fundstellen in `VORCHECK-ERGEBNIS.md`, Fertigmeldung von Paket A
in `FERTIG-A.md`.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-b` (existiert, Branch
  ist ausgecheckt, enthält den gemergten Stand von Paket A)
- Branch: `feat/build-reasoner-b`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Was du baust

Im Crate `rust/crates/dbrain-reasoner` die drei Dateien des deterministischen
Kerns, exakt gegen die Typen und Signaturen aus `types.rs` (Paket A):

1. `src/mechanics.rs`: reine Rechenregeln ohne IO nach MECHANIK.md: Kaufbonus
   je Slot und Tier aus `purchase_bonuses`, Tickrate gegen Proc-Cooldown,
   bedingte Items mit Abwertungsfaktor je `ConditionKind`, Aktiv- gegen
   Passivwert, Slot-Knappheit gegen Soul-Budget, Soul-Kurve und Phasen,
   Nebenwaffe, Imbue-Zuordnung, Kampffenster-Metrik. Jede Regel als eigene
   `pub fn` mit Unit-Test aus Zahlen der Spec.
2. `src/hero.rs`: `build_hero_model` aus dem Snapshot-Payload, aufbauend auf
   den bestehenden Funktionen in `dbrain-learn/src/build_optimizer.rs`
   (`infer_hero_needs`, `ability_damage_profile`, `ability_scaling_stats`,
   `infer_ability_role_tags`, `economy_summary`), nicht neu schreiben.
   Skalierungsstufe aus `property_upgrades[].name` plus `scaling_stats`;
   prüfe am echten Warden-Snapshot, ob `scale_function` vorhanden ist, und
   nimm es nur dann als Quelle (Entscheidung 1 in PAKETE.md).
3. `src/item.rs`: `build_item_model` und `score_items` (deterministisch,
   endgültige Zahlen, Meta nur als Nebensignal über das `MetaIndex`-Objekt aus
   `types.rs`). Kein starrer Pfad spirit/weapon/tank: Warden muss als
   Waffen-Kern mit Spirit-Nebenwirkung herauskommen.
4. Tests: je Regel Unit-Tests; ein Integrationstest lädt über `data.rs` den
   echten Warden-Snapshot und die Items Veil Walker, Mercurial Magnum, Siphon
   Bullets, Quicksilver Reload und prüft, dass diese vier im Scoring über der
   Kern-Schwelle liegen. Schlägt das fehl, ist das ein Befund für die
   Fertigmeldung, nicht etwas, das man am Test biegt.

## Regeln

- Nur `mechanics.rs`, `hero.rs`, `item.rs` und deren Tests. `lib.rs` nicht
  anfassen (die `mod`-Zeilen trägt der Delegator nach); zum lokalen Testen
  darfst du sie temporär ergänzen, aber nicht committen. `types.rs` ist
  eingefroren: fehlt dort etwas, Bump-up statt Änderung.
- Keine Code-Kommentare. Kein KI-Aufruf in diesem Paket.
- Toolchain: `export PATH=/home/nathanael/.cargo/bin:$PATH`, im Verzeichnis
  `rust/`. `cargo fmt` (nur eigene Dateien), `cargo clippy -p dbrain-reasoner`,
  `cargo test -p dbrain-reasoner`. Kein `--release`.
- Nur den eigenen Branch committen und pushen (`git push origin
  feat/build-reasoner-b`), nie main. Commit-Trailer:
  `Co-authored-by: GPT 5.6 Luna <luna@local>`.
- Echte Umlaute, englische Spielnamen, keine Gedankenstriche.

## Bump-up

```
[Bump-up] Paket B: Grund: ... Erledigt: ... Worktree: /home/nathanael/.worktrees/deadlock-brain-b Offen: ...
```

## Fertigmeldung

In diesem Thread und als Datei `FERTIG-B.md` im Task-Ordner: Branch,
Commit-SHAs, geänderte Dateien, Testlauf mit Zahlen (Baseline und Endstand),
Warden-Scoring der vier Referenz-Items mit Zahlen, Entscheidungen bei
Spec-Lücken, offene Punkte für C und D.
