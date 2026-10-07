# Gemeinsame E-Prüfung nach den Originalquellenfixes

Stand: 07.10.2026. Tatsächlicher Integrationskandidat `501d3725e691c713f4b04468fd9d6b77977ae91c`, Tree `4cca98fe791105203b8951a4f24c6c4abaf43cab`, Basis `ca4d877f13042c9a7a7023e54f6bf2c688b69ac4`. Eigener Arbeitsbaum `/home/nathanael/.worktrees/brain-i-release-20261007`, Branch `feat/brain-i-integration-blocked-20261007`. Er enthält die Core6-Gegenprobe und beide begrenzten Patchfixes `aca42a50` und `b70dc6b6`.

## Tatsächlich abgeschlossene Prüfungen

Formatcheck Exit 0, `/tmp/brain-i-e-patch-candidate-fmt.log`.

```bash
env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --locked --manifest-path /home/nathanael/.worktrees/brain-i-release-20261007/rust/Cargo.toml -p brain-storage -p deadlock-brain-core -p dbrain-sources -p dbrain-builds -p deadlock-brain -j 2 -- --test-threads=1
```

Exit 0: 558 passed, 0 failed, 25 ignored, 0 measured, 0 filtered. Vollständiges Log `/tmp/brain-i-e-patch-candidate-tests.log`. Die ignorierte neue Bestands-ID-Probe wird nachfolgend ausdrücklich getrennt ausgeführt; die übrigen ignorierten Fälle sind durch diesen Bericht nicht als bestanden ausgewiesen.

```bash
env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo clippy --locked --manifest-path /home/nathanael/.worktrees/brain-i-release-20261007/rust/Cargo.toml -p brain-storage -p deadlock-brain-core -p dbrain-sources -p dbrain-builds -p deadlock-brain -p brain-serve --all-targets --no-deps -j 2 -- -D warnings
```

Exit 0, `/tmp/brain-i-e-patch-candidate-clippy.log`.

## Echte Bestandszuordnung in isoliertem Postgres

Der erste gezielte Aufruf ohne `DEADLOCK_BRAIN_SCRATCH_DSN` scheiterte tatsächlich mit Exit 101: 0 passed, 1 failed, 0 ignored, 116 filtered; `Scratch-DB fehlt: NotPresent`. Das war eine fehlende Prüfvoraussetzung und ist kein behaupteter vorbestehender Produktfehler. Log `/tmp/brain-i-e-patch-candidate-existing-id-pg.log`.

Anschließend eigene frische PostgreSQL-16-Instanz mit `initdb -A trust -U brain_core_test --no-locale --encoding=UTF8`, ausschließlich Unixsocket `/tmp/brain-i-patch-lookup-20261007.cnvaCS`, Port 55439 und `listen_addresses=''`. Eigene Datenbank `brain_fixer12_patch_lookup` mittels `createdb` angelegt. Keine Produktions-DSN, kein schreibendes psql, kein Zugriff auf einen fremden Testcluster. Der vorhandene Regressionstest prüft den Datenbanknamen, das Fehlen von `brain.source_documents`, führt die tatsächliche vorhandene Lookupfunktion aus und rollt seine Transaktion zurück.

```bash
env -u DEADLOCK_CENTRAL_DSN -u DATABASE_URL DEADLOCK_BRAIN_SCRATCH_DSN='host=/tmp/brain-i-patch-lookup-20261007.cnvaCS port=55439 user=brain_core_test dbname=brain_fixer12_patch_lookup' SQLX_OFFLINE=true /home/nathanael/.cargo/bin/cargo test --locked --manifest-path /home/nathanael/.worktrees/brain-i-release-20261007/rust/Cargo.toml -p deadlock-brain --bin deadlock-brain pg_patchnotes::api_sync::tests::canonical_original_lookup_preserves_existing_changelog_ids_without_brain_documents -j 2 -- --exact --ignored --test-threads=1
```

Exit 0: 1 passed, 0 failed, 0 ignored, 0 measured, 116 filtered, tatsächliche Laufzeit 0,20 Sekunden. Log `/tmp/brain-i-e-patch-candidate-existing-id-pg-final.log`. Eigene Instanz danach mit `pg_ctl -D /tmp/brain-i-patch-lookup-20261007.cnvaCS/pg -m fast -w stop` beendet, Exit 0. Keine ganze ignorierte Suite gegen Produktivdaten gestartet.

TESTNACHWEIS[TW-1]: 558 passed, 25 ignored | Baseline: keine Altfehler behauptet

Zusätzlich getrennt: echte Lookup-Scratchprobe 1 passed, 0 ignored. Die beiden Läufe nicht als ein einzelner Lauf mit weniger ignorierten Fällen ausgeben.

## Releasevorbereitung und Grenzen

Read-only-Receiptprobe `/tmp/brain-i-mirror-proof.rs` erneut gegen die Kandidatenquellen über das vorhandene Manifest `/tmp/brain-i-live-proof-20261007/Cargo.toml` mit `SQLX_OFFLINE=true cargo build --offline ... -j 2` gebaut, Exit 0; `/tmp/brain-i-live-proof-candidate501-build.log`. Noch nicht live ausgeführt, kein Receipt-/Rawhash-Livebeweis.

Der reguläre `brain-release plan` verweigerte den Quellbaum wegen ignorierter Dateien. Tatsächlich lag dort ausschließlich `rust/target/`. Dieses eigene Prüfartefakt wurde ohne Löschung nach `/tmp/brain-i-verified-target-20261007.vYUnv5/target` verschoben. Anschließendes `git status --short --ignored` leer. Kein Hook oder Wrapper geändert, keine Prüfung abgeschwächt, kein Releasebuild oder Deploy durchgeführt.

Der gemeinsame Produkt-Gate ist auf genau Kandidat 501d3725 gegen ca4d877f mit unverändert `--model claude-opus-5-5` gestartet. Sein Ergebnis wird getrennt in REVIEW dokumentiert. Das begrenzte ALLOW von b70dc6b6 ist kein Gesamt-ALLOW. Bis zum tatsächlichen Gesamt-ALLOW bleiben vollständiger E-Main-Push, regulärer Deploy, Import und analytics_runtime-Übergabe offen. G/K-Prüfsperren und Eigentum bleiben unverändert.
