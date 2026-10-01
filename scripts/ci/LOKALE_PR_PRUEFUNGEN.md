# Lokale PR-Prüfungen

GitHub Actions sind in diesem Repository absichtlich abgeschafft. Die Löschung der Workflow-Dateien bedeutet nicht, dass diese Prüfungen gelaufen oder grün sind. Vor einem Merge führt der Integrator die passenden lokalen Prüfungen auf dem zusammengesetzten Checkout aus und hält Commit, Befehl und Ergebnis fest.

## Gemeinsamer lokaler Einstieg

Der bestehende Einstiegspunkt `scripts/check_brain_core.sh all <absoluter-vorhandener-target-cache>` bündelt die lokalen Prüfungen. Der `all`-Lauf prüft zuerst Migrationen und Backup-/Restore-Skripte, danach Formatierung, Clippy, Workspace-Tests, Release-Build und den privaten PostgreSQL-Core-Harness. Er braucht Rust 1.97.1, ShellCheck, PostgreSQL 16 und einen exklusiv genutzten vorhandenen Cargo-Target-Cache. Diesen umfangreichen Lauf nicht parallel zu einem anderen Cargo-Build starten.

```sh
scripts/check_brain_core.sh all /absolute/existing/rust-target
scripts/check_brain_core.sh all /absolute/existing/rust-target --peer <geprüfter-peer-head-sha>
```

`all` vergleicht gegen `origin/main` und den aktuellen `HEAD`. Peer-PRs werden nicht automatisch aus GitHub gesucht oder heruntergeladen. Angegebene Peer-Heads müssen zuvor geprüft und als lokale Refs oder Commit-SHAs vorhanden sein. Diese lokale Prüfung dokumentiert jeder Integrator separat; sie behauptet nicht, GitHub Actions ausgeführt zu haben.

Der Guard prüft die unveränderte Identität (Git-Modus und Blob) jeder bestehenden `scripts/migrations/**/*.sql`-Datei und Tabellenüberschneidungen gegen diese Migrationen und explizite Peer-Heads. Er kompiliert und testet den Admin-Guard mit Rust 1.97.1 und führt Regressionen in temporären Git-Repositories aus.

Der Guard ist absichtlich ein enger statischer DDL-Check, kein vollständiger PostgreSQL-Parser. Er berücksichtigt SQL-Kommentare, geschachtelte Blockkommentare, quoted identifiers, Standard-/Escape- und Dollar-Strings sowie Klammerung. Bei neuen Migrationen weist er dynamisches DDL, `DO`-/`EXECUTE`-/`CALL`-Pfad, ausführbare Funktions-/Trigger-DDL, unbekannte `CREATE TABLE`-Formen, unqualifizierte Tabellennamen, `SET search_path` und veränderte bestehende Migrationen zurück. Gleiche tokenisierte Tabellen-Definitionen dürfen nur als `CREATE TABLE IF NOT EXISTS` wiederholt werden. Andere SQL-Verhaltensänderungen müssen weiter anhand der passenden PostgreSQL-Regressionen geprüft werden.

## Weitere lokale Prüfungen

Die zugehörigen Befehle bleiben manuell verfügbar. Sie sind nicht Bestandteil eines automatischen Merge-Gates:

```sh
~/.cargo/bin/cargo +1.97.1 fmt --manifest-path rust/Cargo.toml --all -- --check
```

Der Backup-Test verwendet ausschließlich Stubs und temporäre Dateien. Der Restore-Test benötigt einen unprivilegierten Aufruf und PostgreSQL 16; er erstellt private temporäre Cluster. Der Storage-Upgrade-Test kann mit einem vorhandenen, exklusiv dafür freigegebenen Target-Cache aufgerufen werden:

```sh
bash scripts/test_brain_storage_upgrade.sh ~/.cargo/bin/cargo /absolute/existing/rust-target
```

Dieser Runner startet einen eigenen temporären Cluster und nutzt den vorhandenen Cargo-Cache. Er wird nicht parallel zu anderen Cargo-Builds gestartet. Status und Nachweise dieses Laufs müssen getrennt vom Migration-Guard dokumentiert werden.
