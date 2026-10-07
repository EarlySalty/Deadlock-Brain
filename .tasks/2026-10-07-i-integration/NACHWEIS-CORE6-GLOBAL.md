# Gegenprobe zum Produkt-Gate vom 07.10.2026

## Urteil und Grenze

Der konkrete BLOCK-Kern aus `/tmp/brain-i-e-product-integration-gate.log` ist widerlegt: Ein neuer vollständiger Core6-Lauf derselben Version ohne globale Endpoints verdeckt vorhandene globale Assets nicht. Der bestehende gemeinsame SQL-Leser hat bereits einen INNER JOIN auf das angefragte Endpoint-Dokument, bevor ORDER BY und LIMIT wirken (`brain-storage/src/asset_mirror.rs:159-164`). Ein Lauf ohne diesen Endpoint kommt nicht in die Ergebnismenge.

Kein Produktfix und kein Gate-Override. Nur eine zusätzliche Prüfung in `dbrain-sources/src/asset_receipt_tests.rs`. Der Produkt-BLOCK bleibt wirksam. Nach fünf weiteren erfolglosen Fortsetzungsurteilen erfolgt die Fachrückgabe; kein anderer Reviewer und kein weiteres Urteil angefordert.

## Tatsächliche Probe

`assets_api::receipt_tests::later_core_mirror_preserves_globals_from_the_actual_older_run` verwendet den vorhandenen ScratchPg, echte SourceStore-/SourceIr-Dokumente und sämtliche öffentlichen Mirror-Leser. Vor Datenzugriff werden eigener Unixsocket, Rolle `brain_core_test` und eigenes data_directory geprüft. Keine Produktivdaten oder produktive Datenbank verwendet.

1. Vollständigen Lauf mit 13 Endpoint-/Sprachkombinationen und Originalmanifest speichern.
2. Einen späteren erfolgreichen Core6-Lauf derselben Version mit nur sechs gebundenen Endpoints speichern. Feste SQL-Zeiten liegen 25 Stunden auseinander, Spiegelzeiten mehr als 24 Stunden. Keine neue Wall-Clock-Steuerung.
3. Alle 13 Kombinationen über den vorhandenen Value- und Receipt-Leser lesen. Core6 stammt aus dem neuen Lauf; globale Assets stammen samt tatsächlichem Receipt aus dem älteren Lauf. Payloads sind identisch.
4. Explizite Run-Bindung prüfen: ältere globale Daten funktionieren mit dem älteren Run. Der neue Core6-Run weist fehlende globale Daten ab und erhält niemals den Receipt des falschen Runs.

```text
env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --locked --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p dbrain-sources --lib later_core_mirror_preserves_globals_from_the_actual_older_run -j 2 -- --test-threads=1
```

Harness-Exit 0. Kompilation 22.65s; tatsächlicher Testlauf 4.65s. Original `/tmp/brain-i-core6-global-gate-reproduction-final.log`:

```text
running 1 test
test assets_api::receipt_tests::later_core_mirror_preserves_globals_from_the_actual_older_run ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 225 filtered out; finished in 4.65s
```

Formatcheck `cargo fmt --check --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p dbrain-sources`, Exit 0. Striktes Clippy `env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo clippy --locked --manifest-path /home/nathanael/.worktrees/brain-e-deadlock-api/rust/Cargo.toml -p dbrain-sources --all-targets --no-deps -j 2 -- -D warnings`, Exit 0. Originale `/tmp/brain-i-core6-global-fmt-final.log` und `/tmp/brain-i-core6-global-clippy-final.log`.

TESTNACHWEIS[TW-1]: 1 passed, 0 ignored | Baseline: keine Altfehler behauptet

## Aussagekraft

Dies ist eine echte PostgreSQL-Gegenprobe des genannten Reader-Szenarios mit kontrollierten Daten, kein Liveimport und kein allgemeiner Vollständigkeitsbeweis. Die Produktimplementation ist unverändert. Die ältere Core6-Kompatibilität bleibt erhalten. Ungültige vorhandene Endpoint-Dokumente werden weiterhin nicht stillschweigend durch ältere Daten ersetzt. Die übrigen Gate-NITs sind damit nicht geprüft oder geschlossen.
