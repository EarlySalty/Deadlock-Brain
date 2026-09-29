status: aktiv
Datum: 2026-09-29

# Paket C: Nachweis und Übergabe

## Stand

Basis `305df2d36ec7b5d0513d6c0769051b41538d6a1b`, Implementierung `db35673` und `f83e01f` auf `fix/pre-g5-harness-20260929`. Die Datenbanktests starten jeweils einen eigenen PostgreSQL-Cluster mit privatem Unix-Socket, Peer-Zuordnung und bereinigter Kindprozessumgebung. Sie nutzen synthetische Datensätze und bestehende Fixtures. Kein Systemdienst wurde neu gestartet. Die historischen Runner für Pilot, Serve-Checks und Legacy-Import mit produktionsnahen Daten oder Passworttransport sind gesperrt. `run_isolated_load.sh` führt den sicheren Prozess-E2E aus.

Faktenregressionen prüfen Entity und konkretes Feld, genaue und falsche Zahl, deutschen Alias, fehlende Evidenz, Patch, Modus, beide Richtungen der Rechteprüfung sowie Revoke und Tombstone. Ein separater Feed-Test prüft für jeden Fixture-Helden die fünf Starting-Stats-Felder samt Zahlenwert, JSON-Pointer, Provenienz und Quarantäne ungültiger Werte. Der neue CLI-Test liest eine synthetische `brain_legacy`-Datenbank, schreibt in eine getrennte `brain_pilot_test`-Datenbank und prüft Wiederholung, Release, Tombstone und Revoke. Der bestehende Wiki-Runner deckt Store, Release und CLI gegen den Wegwerf-Cluster ab, ohne eine Netzquelle abzurufen.

## Ausgeführte Gates

Für die folgenden Rust-Befehle gilt `cd rust` im eigenen Worktree und eine bereinigte Umgebung:

```bash
CLEAN=(env -i "PATH=$PATH" "HOME=$HOME" "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}" "CARGO_TARGET_DIR=$PWD/target" "CARGO_BUILD_JOBS=2" "SQLX_OFFLINE=true")
CARGO="$HOME/.cargo/bin/cargo"
```

| Befehl | Exit | Nachweis |
| --- | ---: | --- |
| `$CARGO fmt --all --check` | 0 | Formatprüfung nach letztem Rust-Edit |
| `"${CLEAN[@]}" "$CARGO" clippy --workspace --all-targets --locked --offline --jobs 2 -- -D warnings` | 0 | Workspace einschließlich CLI-Test |
| `"${CLEAN[@]}" "$CARGO" test --workspace --locked --offline --jobs 2` | 0 | 980 bestanden, 0 fehlgeschlagen, 73 ignoriert; die DB-Tests darin wurden nicht als bestanden gezählt |
| `flock -w 600 /tmp/deadlock-brain-release-build.lock "${CLEAN[@]}" "$CARGO" build --workspace --release --locked --offline --jobs 2` | 0 | Release-Build im eigenen Worktree, zwei Jobs |
| `./scripts/test_brain_core_postgres.sh` | 0 | 1 gezielt aktivierter PostgreSQL-Test bestanden |
| `./scripts/test_brain_storage_upgrade.sh` | 0 | 1 gezielt aktivierter Upgrade- und Restore-Test bestanden |
| `./scripts/test_wiki_runtime.sh` | 0 | Wiki-Suite und gezielt aktivierter Scratch-Test bestanden; beim Suite-Lauf blieb dieser eine Test ignoriert |
| `"${CLEAN[@]}" "$CARGO" test -p brain-feeds --test assets_starting_stats --locked --offline --jobs 2` | 0 | 2 bestanden, 0 ignoriert |
| `bash -n` und `shellcheck` für die geänderten Shellskripte | 0 | Syntax und Shell-Lint |
| `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo "$PWD" --base origin/migration/rust-integration --head HEAD --timeout 900` | 0 | ALLOW auf `f83e01f`, zwei Hinweise ohne Merge-Blocker |

Die Skriptbefehle und der Gate-Aufruf liefen im Repo-Root, die Rust-Befehle im Verzeichnis `rust`.

`./scripts/test_brain_serve.sh` führte nach der CLI-Erweiterung die beiden gezielt aktivierten Legacy-Tests jeweils erfolgreich aus, scheiterte aber zuletzt im Prozess-E2E. Bei 8 Workern wurden 596 von 600 Anfragen beantwortet; vier Antworten waren `unavailable`. Der Service meldete dazu genau vier Pool-Wartezeitüberschreitungen bei festem Poolmaximum 4 und unverändertem Wartebudget 150 ms. Andere Läufe zeigten 598/600 bei 16 und 575/600 bei 32 Workern. Der Host hatte bei der Diagnose eine Last von 29,56 auf 16 verfügbaren CPUs und mehrere fremde Builds. Der erste Prozesslauf vor der zusätzlichen CLI-Fixture und dem Diagnosezusatz bestand mit 600/600 bei 8, 16 und 32 Workern, ohne HTTP- oder Clientfehler und mit Poolspitze 4. Dieser historische Lauf ist kein Grün-Nachweis für den aktuellen Head. Das Wartebudget und das Poolmaximum wurden nicht erhöht, fehlgeschlagene Antworten nicht wiederholt oder als Erfolg gezählt. Die finale Lastabnahme bleibt offen. Das Selbstreview gab ALLOW und wies darauf hin, dass der Wechsel auf Peer-Authentisierung den alten SCRAM-Fehlpassworttest ersetzt. Außerdem liefert der neue Lastlauf JSON je Stufe mit Durchlaufzeit, aber nicht die bisherigen separaten Latenzverteilungen und Reportdateien.

## Wiederholung auf integriertem Head

Aus dem Brain-Repo nacheinander `./scripts/test_brain_serve.sh`, `./scripts/test_brain_core_postgres.sh`, `./scripts/test_brain_storage_upgrade.sh` und `./scripts/test_wiki_runtime.sh` ausführen. Der Serve-Runner startet die drei gezielt ignorierten Prozess- und Legacy-Tests mit `--ignored`; er meldet pro Laststufe `db_pool_load` mit `requests=600`, `statuses.answered=600`, `client_errors=0` und `observed_reader_connections<=4`. Am Ende muss `db_pool_metrics.stats.peak_connections=4` gelten. Bei einer roten Laststufe Pool-Wartezeiten und Hostlast ausweisen, nicht das Budget ändern. Der finale Messpunkt ist erst nach Integration von A und C zu belegen.

Keine Betreiberfreigabe für produktiven Wiki-Capture, Provider-Egress, echte Communitydaten oder Deployment wurde angenommen. Es erfolgte weder Merge noch Deploy.

TESTNACHWEIS[TW-1]: 980 passed, 73 ignored | Baseline: nicht erhoben
BESTAND[BS-1]: teilweise | Fundort: scripts/test_brain_serve.sh:1 | Anknüpfung: vorhandenen Prozess- und PostgreSQL-Harness abgesichert
WIRKUNGSPRUEFUNG[WP-1]: 1 Befund | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: .tasks/2026-09-29-technical-closeout/C-REPORT.md
ORCHESTRIERUNG[OR-1]: Stufe groß | Schritt review | Artefakt: .tasks/2026-09-29-technical-closeout/C-REPORT.md
