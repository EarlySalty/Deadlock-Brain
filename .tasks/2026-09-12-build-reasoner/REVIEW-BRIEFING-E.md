# Review-Briefing: build-reasoner (Paket E, Runde 1)

[Orchestrator] Review Runde 1 für Paket E (Patch-Delta Live-Fix plus
Spirit-Feuerrate). Lesend, kein Code, kein Branch. Du bist der einzige Thread
für dieses Review. Keine Unter-Threads oder Unter-Agenten spawnen.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-e` (Branch
  `fix/build-reasoner-patch-delta`, Code 4434a52, Nachweis 2993780, Basis
  e958158 = main vor dem Live-Befund)
- Diff: `git -C /home/nathanael/.worktrees/deadlock-brain-e diff e958158..4434a52 -- rust/`
- Auftrag: `FIX-BRIEFING-E.md` im Hauptordner
  `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/`,
  Befund 13 in `PAKETE.md`, Spec `ARCHITEKTUR.md` Abschnitte 8, 9, 11,
  `MECHANIK.md` Abschnitte 13 und 14. Fertigmeldung und Belege im Worktree
  unter `.tasks/2026-09-12-build-reasoner/FERTIG-E.md` und `WARDEN-E*.json`.
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Verbindliche Nutzervorgabe: "immer nur aktuelle Warden-Daten nehmen, damit
  wir ein sauberes Heldenmodell haben, und alle Besonderheiten, weil Warden
  zum Beispiel die Feuerrate mit Spirit Power skaliert."

## Was du prüfst

1. `patch.rs`: Deltas nur für Events mit `posted_at` nach dem `fetched_at`
   des Snapshot-Feldes, aus dem der Wert stammt (Held: Assets; Item:
   `item_or_ability` oder `item_card`). Dedup nach Patchtag, Entität und
   normalisierter Rohzeile; Vorzeichen aus Zahlen, nie aus `change_type`;
   Rohzeile `from/to` schlägt fehlerhafte Spalten (Mercurial 0,465 zu 0
   statt 0,49). Faktor `new/old`, Grenzen 0,5 bis 2 einzeln und kumuliert,
   Score-Schutz gegen Ergebnis unter null. Prüfe, ob der Kalendertag als
   Dedup-Schlüssel echte Mehrfach-Updates am selben Tag verschluckt und ob
   "frühester Zeitpunkt der Fassungen" ein Event fälschlich vor den Snapshot
   schiebt.
2. `data.rs`: Snapshot-Zeitpunkte je Feld korrekt aus dem tatsächlich
   gewählten Snapshot (Waffen-Fallback, Card-Proc-Cooldown); Heldenmodell nur
   aus Assets-Snapshots ohne ältere Profil-Skalierungen; der Loader lädt
   jetzt auch Events der modellierten Items und Abilities (7762 statt 416):
   Laufzeit und Speicher am echten Central-Pool (read-only, Zugang wie in
   `scripts/`, Secrets aus Infisical, nie ausgeben).
3. `item.rs` (additiv erlaubt für die Feuerrate): `ERoundsPerSecond`
   direkt in Schuss/s, `EFireRate` nur als Prozent-Fallback, nie addiert;
   Formel `DPS = Schaden × Magazin / (Magazin / r + Nachladezeit)`; Item-SP
   plus Kaufbonus-SP; keine Doppelzählung bei doppelten Properties/Passive;
   Zustandsfaktoren und Gewichte unverändert; `mechanics.rs` unverändert
   (`git diff --stat` bestätigen).
4. `composer.rs` und `lib.rs`: Patch wirkt genau einmal (Modellwert), kein
   zweiter Bonus in `score_items` oder beim Sortieren; Kern nur bei
   positivem endlichem `total`; Optional höchstens 12 nach Score;
   Confidence nie `High` bei `total <= 0`. Prüfe, ob bestehende Aufrufer
   von `score_items` (Backtest, Publish) durch die Signaturänderung stumm
   anderes Verhalten bekommen.
5. `types.rs`: additiver Snapshot-Beleg; alte JSON-Zeilen in
   `brain.reasoner_patch_deltas` bleiben deserialisierbar (Serde-Default).
6. Nachweis: die Warden-Tabellen in `FERTIG-E.md` gegen `WARDEN-E-BUILD.json`
   und `WARDEN-E-BACKTEST.json` stichprobenartig abgleichen (Kern 19 positiv,
   Optional 12, Juggernaut 73,95, Mercurial 51,82, Backtest Kern-Überdeckung
   0,2105 gegen Seed und 779996). Die drei fehlenden Referenz-Items (Siphon
   Bullets, Quicksilver Reload, Veil Walker) sind ausdrücklich kein Mangel
   von E, sondern Kalibrierungsthema (Kern-Cutoff, Paket F); prüfe nur, dass
   der Fixer dafür keine Gewichte gedreht hat.

Tests selbst laufen lassen und Zahlen nennen: `cargo test --workspace` ohne
DSN (Fixer: 273 bestanden, 58 ignoriert), `cargo test -p dbrain-reasoner --
--include-ignored` mit Central-DSN read-only (Fixer: 101 mit lokaler
Scratch-DB; ohne Scratch nenne die Zahl), Clippy `-D warnings`. Toolchain
`export PATH=/home/nathanael/.cargo/bin:$PATH`, im Verzeichnis `rust/`. Kein
`--release`. Keine Schreibzugriffe auf den Central-Pool.

## Ergebnis

`REVIEW-E.md` im Hauptordner: Mängelliste mit Datei:Zeile, Schwere
(blockierend, wichtig, nit), je Mangel Fix-Vorschlag; Punkte, die nur
Kalibrierung sind, getrennt als "für Paket F" ausweisen. Urteil: FREIGABE
oder NACHBESSERN. Fertigmeldung in diesem Thread mit Urteil. Deutsch, echte
Umlaute, keine Gedankenstriche.
