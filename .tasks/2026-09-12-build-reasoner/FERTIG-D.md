# FERTIG-D

Stand: 2026-09-12

## Branch und Commits

- Branch: `feat/build-reasoner-d`
- Commit: `5f438d26dcf6bd4c73257bd7fc4768772f0c8f18`
- Der Commit enthält den Trailer `Co-authored-by: GPT 5.6 Luna <luna@local>`.

Es wurde ausschließlich dieser Branch bearbeitet. `main` wurde nicht
verändert.

## Geänderte Dateien

- `rust/crates/dbrain-reasoner/src/lib.rs`: Fassade für Build, Patch-Impact und Backtest, Live-Quellen und Seed-Anbindung.
- `rust/crates/dbrain-reasoner/Cargo.toml`: Publish-Abhängigkeit `dbrain-builds`.
- `rust/crates/deadlock-brain/src/main.rs`: Reasoner-CLI mit JSON-, Patch-, Seed- und Publish-Optionen.
- `rust/crates/deadlock-brain/Cargo.toml`: CLI-Abhängigkeit auf `dbrain-reasoner`.
- `rust/crates/dbrain-builds/src/sync.rs`: echte Patch-Tags aus `brain.patch_events`, historienerhaltende Upserts.
- `rust/crates/deadlock-brain-core/src/ai.rs`: Fireworks-Requests in Worker-Threads und Resolver-Wiederholung nach 404.
- `rust/crates/deadlock-brain-core/src/model_resolver.rs`: Prozess-Resolver für die `deepseek-v4-flash`-Familie.
- `rust/crates/deadlock-brain-core/src/lib.rs`: Resolver-Modul exportiert.
- `rust/Cargo.lock`: neue Workspace-Abhängigkeiten.
- `scripts/migrations/2026-09-12-reasoner.sql`: Reasoner-Build-, Score-, Delta- und Backtest-Tabellen.
- `scripts/run_build_data_with_infisical.sh`: retry-fähiger täglicher Build-Data-Lauf.
- `service/systemd/deadlock-brain-build-data.service` und `.timer`: täglicher Lauf um 03:30 Europe/Berlin.
- `docs/BUILD_REASONER.md`: Zweck, Datenfluss, Befehle, Backtest-Lesart und Grenzen.
- `REPORT-D.md`: echte Warden-Zahlen und Ursachenanalyse.

## Testläufe

Baseline vor Paket D: Branch sauber auf `cbfbc98`, die Reasoner-Module waren
noch nicht in `lib.rs` deklariert, `dbrain-builds` fehlte in der
Reasoner-Crate und es gab keine Reasoner-CLI.

Endstand:

- `cargo test --workspace`: bestanden.
- Fokussierte Tests: `dbrain-builds` 7 bestanden, `dbrain-reasoner` 68
  bestanden, `deadlock-brain` 38 bestanden, `deadlock-brain-core` 14
  bestanden. Vorhandene ignorierte Tests bleiben als solche markiert.
- Clippy mit `-D warnings` für `dbrain-reasoner`, `dbrain-builds`,
  `deadlock-brain-core` und `deadlock-brain`: bestanden.
- Central-DSN-Test `loads_warden_and_reference_items_from_real_snapshot`: 1
  bestanden.
- Central-DSN-Test `scores_warden_reference_items_from_real_snapshot`: 1
  bestanden.
- Vollständiger Reasoner-Lauf mit `--include-ignored`: 70 bestanden und 5
  vorhandene Scratch-Tests wegen fehlendem `REASONER_SCRATCH_DSN` fehlgeschlagen.
  Die beiden Central-Tests wurden separat erfolgreich ausgeführt.
- Live-CLI mit Central-DSN: no-AI-Build, AI-Build, Patch-Impact und Warden-
  Backtest lieferten gültiges JSON. Die Kennzahlen stehen in `REPORT-D.md`.

## Deploy-Voraussetzungen

1. `scripts/migrations/2026-09-12-reasoner.sql` auf dem Central-Pool
   ausführen.
2. Release-Binary bauen und den systemd-Service mit dem tatsächlichen
   Checkout-Pfad installieren.
3. Der Service benötigt die vorhandene Infisical-Konfiguration und das
   Credential `infisical-token`; `DEADLOCK_CENTRAL_DSN` wird ausschließlich
   aus Infisical geladen.
4. Timer aktivieren. Der Lauf ruft täglich
   `pull build-data --hero all` auf und schreibt neue Patch-Stände, ohne alte
   Composite-Schlüssel zu löschen.

Publish wurde nicht ausgelöst, da es eine externe Schreiboperation zum
Steam-Bot wäre. Die CLI schreibt ohne `--publish` nichts.

## Offene Punkte

- Aktuelle `hero_item_stats` müssen nach dem neuen echten Patch-Tag synchronisiert
  werden, damit Meta-Signale in den Build einfließen.
- Patch-Wechsel braucht mindestens zwei vergleichbare Patch-Stände je Autor.
- Für einen vollständig grünen Include-Ignored-Lauf fehlt weiterhin
  `REASONER_SCRATCH_DSN`.
- Die zentrale Backtest-Struktur serialisiert Kern-Überdeckung,
  Reihenfolge-Nähe und Patch-Wechsel. Jaccard wurde für den Seed-Vergleich
  zusätzlich im `REPORT-D.md` aus `backtest::core_jaccard` ausgewertet.
