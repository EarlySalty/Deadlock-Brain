# REPORT: Brain-Release (Zwischenstand Kontext + Sync)

Branch `codex/brain-release-20260918`, Worktree `/home/nathanael/.worktrees/brain-release-20260918`, Basis `9efeb1e`. Einziger Worker, keine Unter-Agenten. Nur Feature-Branch, kein Merge/Prod-Deploy/Migration durch mich.

## TLDR

Kontextauswahl (Item 1) und revisionssicherer Patch-Sync-Prüfbefehl (Item 2) sind lauffähig, getestet und gepusht, damit der Orchestrator U1 vorbereiten kann. Offen: R4 (dritte Migration), quellidentitätsgeprüfter Source-Refresh, History-Modul/MCP (Item 3), YT-Integrationen gegen die Validierungs-DB.

## Geliefert

1. **Deterministische Kontextauswahl** (`deadlock-brain-patch-review`, `build_context`). Voller Patchoriginaltext und alle Events bleiben. Vollständige Payloads nur für vom Patch referenzierte Entities (`mechanic_snapshots`), Abgleich über normalisierten `canonical_name`/`external_id` gegen Event-`entity_name`/`subject` (exakt oder ganzes Wort). Alle übrigen vor Publikation beobachteten Entities bleiben als kompakter `mechanic_catalog` sichtbar (Snapshot-ID, Typ, external_id, Name, Inhaltshash). `item_or_ability` ist berücksichtigt. Jeder Detailwert behält Snapshot-ID, `content_hash` und `json_path`; kein String-Kürzen im Beleg; unbestätigte Vorversion bleibt `unknown`. `context_selection` weist Auswahlversion, Umfang, Ausschlussgrund und das offene Dimensionsproblem aus. Überschreitet der ausgewählte Kontext `MAX_CONTEXT_BYTES`, blockiert die CLI vor jedem Modellaufruf mit gemessenen Byte-/Entity-Zahlen statt zu kürzen.
2. **Revisionssicherer Patch-Sync** (`deadlock-brain pg sync-patchnotes [--apply] [--limit]`). Rein lesende Drifterkennung vergleicht den aktuellen `changelog_posts.raw_content` je id mit dem `raw_content` der zuletzt importierten patchnote-Snapshotfassung: gleiche id mit geändertem Inhalt = geändert, fehlende Fassung = neu, sonst unverändert. Unveränderte Quellen lösen keine teure Analyse aus. Mit `--apply` importiert der Befehl neue und geänderte Quellen über den bestehenden, revisionssicheren `import_one_patchnote`-Kern (prune + reinsert entfernt nicht mehr enthaltene Events); nach Evidenzmigration markiert der Capture-Trigger betroffene Reviews `needs_revalidation`, sodass keine alte Erkenntnis als aktuell geprüft durchgeht. Kein neuer Sammler; in den Timerweg integrierbar.

## Geänderte Dateien

- `rust/crates/deadlock-brain/src/bin/deadlock-brain-patch-review.rs` (Kontextauswahl + Tests)
- `rust/crates/deadlock-brain/src/pg_patchnotes.rs` (`import_one_patchnote`, `sync_patchnotes`, Drifterkennung)
- `rust/crates/deadlock-brain/src/main.rs` (CLI `pg sync-patchnotes`)
- `tests/patch-understanding/cli-smoke-fixture.sql`, `.github/workflows/patch-understanding.yml` (CLI-/Sync-Smoke, Überlauf-Block)

## Tests (wirklich ausgeführt)

Stable `cargo 1.97.1`, eigenes `target-rel` (nicht das geteilte `target-u0`), `--jobs 2`. DB-Prüfungen gegen isolierten `initdb`-Wegwerf-Cluster (eigener Port/Socket), nie den laufenden Server.

```
cargo test --locked --jobs 2 -p deadlock-brain --bin deadlock-brain-patch-review
  -> 11 passed; 0 failed; 0 ignored
cargo test --locked --jobs 2 -p deadlock-brain --bin deadlock-brain pg_insights::tests
  -> 6 passed; 0 failed; 42 filtered out
cargo clippy --locked --jobs 2 -p deadlock-brain -p deadlock-brain-yt -> 0 Warnungen
Smoke (isolierter Cluster, cli-smoke-fixture + beide Migrationen):
  review --patch patch_1 -> Kontext mit s1/s2, item_or_ability, deterministic_reference_projection_v1, mechanic_catalog, detail_entities=2
  review gegen 900 KB-Payload -> Block "Selected context still exceeds the safety limit: 904087 bytes (detail 900654 ... limit 768000)"
  pg sync-patchnotes -> drift=true, new=[1]; erneuter Check nach Import bleibt bis Volschema-Import offen
```

TESTNACHWEIS[TW-1]: 17 passed, 0 ignored | Baseline: 0 rot

(17 = 11 patch-review + 6 pg_insights. `sync-patchnotes --apply` braucht das vollständige Brain-Schema (`brain.entities` u.a.) und wird gegen `brain_release_validation_20260918` geprüft, nicht auf der Minimal-Fixture.)

## Offen (nächste Schritte, nicht wegprüfen)

- **R4:** dritte Migration + Regression laut `REVIEW-RELEASE.md` (rot reproduziert, Probe `/tmp/brain-release-r4-probe-20260918.sql`, Exit 3). Als nächstes.
- **Quellaktualität:** der alte Python-Patchnotes-Bot überspringt bereits verarbeitete URLs, daher bleibt id285 auf dem alten Hash `492177...` mit entfernter Abrams-Zeile. Batchdrift allein aktualisiert die Quellzeile nicht; nötig ist ein quellidentitätsgeprüfter Refresh vorhandener offizieller Quellen im bestehenden Importadapter (kein zweiter Sammler, keine Handkorrektur, erst nach Evidenzmigration revisionssicher).
- **Item 3:** History-Query in wiederverwendbares Rust-Modul auslagern und als lesendes Tool anbinden.
- **YT-Integrationen:** die vier bislang ignorierten YT-Tests gegen `brain_release_validation_20260918` real ausführen.
- Kontext/Prompt/History/Tests sind kein Nachweis autonomen Spielverständnisses; Entwürfe bleiben ungeprüft. U7/U8 (Bild/Video) nicht angebunden.
