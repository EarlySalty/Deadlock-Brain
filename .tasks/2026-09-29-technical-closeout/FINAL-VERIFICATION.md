status: erledigt
Datum: 2026-09-29

# Finale lokale Verifikation des integrierten Brain-Heads

## Urteil und Geltungsbereich

**Verifikation fertig: JA. Produktfix auf dem geprüften Head nötig: NEIN. G5-Freigabe: NEIN.** Die lokale, synthetische Prüfung bestand auf `022f8a981c2164f6d8d4302bae2194e100c4f65c`, Branch `verification/pre-g5-final-20260929`, im eigenen Worktree. Die drei verbindlichen Laststufen beantworteten je 600 von 600 Anfragen bei höchstens vier gleichzeitigen Reader-Verbindungen. Diese Aussage gilt für den geprüften Code und die dokumentierten Fixtures. Echter Wiki-Pilot, Provider-Shadow, realer Replay-Korpus und Produktions-Cutover bleiben ohne Freigabe und echte Daten ungeprüft. Es gab keinen Produktcode-Edit, keinen Merge, keinen Deploy, keinen Service-Neustart und keine Nachricht an Community oder Streamer.

Werkzeuge: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `cargo 1.97.1 (c980f4866 2026-06-30)`, PostgreSQL `16.15`. Workspace-Befehle liefen in `rust/` mit `$HOME/.cargo/bin/cargo`, `CARGO_BUILD_JOBS=2`, `SQLX_OFFLINE=true`, `--locked --offline` für Clippy, Tests und Build sowie einem bereinigten Kindprozessumfeld. Die Test-Runner legen Wegwerfcluster mit privatem Unix-Socket und Peer-Authentisierung an. Die Cluster werden nach erfolgreichem Lauf gestoppt und entfernt. Kein produktiver Datenbankzugang und keine Zugangsdaten wurden an die Tests weitergereicht. Der separate SCRAM-Fall verwendet eine synthetische Testrolle; er ist kein Nachweis für einen Produktionsstart mit Service-Passwort.

## Workspace-Gates auf genau diesem Head

Im folgenden Befehlsblock bezeichnet `CARGO` die lokale Cargo-Binary. `CLEAN` entfernt geerbte Laufzeitkonfiguration; `PWD` ist hier das Verzeichnis `rust/`. Die vier Befehle wurden ohne Ausgabe-Pipe ausgeführt und ihre vollständige Ausgabe getrennt unter `final-logs/` gesichert.

```bash
CARGO="$HOME/.cargo/bin/cargo"
CLEAN=(env -i "PATH=$HOME/.cargo/bin:$PATH" "HOME=$HOME" "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}" "CARGO_TARGET_DIR=$PWD/target" "CARGO_BUILD_JOBS=2" "SQLX_OFFLINE=true")
"${CLEAN[@]}" "$CARGO" fmt --all -- --check
"${CLEAN[@]}" "$CARGO" clippy --workspace --all-targets --locked --offline --jobs 2 -- -D warnings
"${CLEAN[@]}" "$CARGO" test --workspace --locked --offline --jobs 2
flock -w 600 /tmp/deadlock-brain-release-build.lock "${CLEAN[@]}" "$CARGO" build --workspace --release --locked --offline --jobs 2
```

| Gate | Exit | Ergebnis | Log |
| --- | ---: | --- | --- |
| Format | 0 | Workspace formatiert | `final-logs/fmt.log` |
| Clippy mit `-D warnings` | 0 | Keine Warnung | `final-logs/clippy.log` |
| Workspace-Tests | 0 | **1.011 bestanden, 0 fehlgeschlagen, 75 ignoriert, 0 herausgefiltert** aus 99 Ergebnisblöcken | `final-logs/workspace-test.log` |
| Release-Build | 0 | Release-Profil in 2 min 18 s abgeschlossen | `final-logs/release-build.log` |

Die 75 ignorierten Fälle gehören nicht zur Zahl der bestandenen Workspace-Tests. Gezielt aktivierte Fälle sind unten separat ausgewiesen. Wiederholte Tests aus ergänzenden Runnern werden nicht zu den 1.011 Workspace-Fällen addiert.

## Datenbank, Fakten und Wiki

Die folgenden Runner wurden aus dem Repo-Root ausgeführt. Ihre Cargo-Kindprozesse nutzen `env -i`, Offline-Abhängigkeiten und den lokalen Build-Cache. In den PG-Runnern läuft ein eigener PostgreSQL-Cluster mit Unix-Socket und Peer-Zuordnung, ohne produktiven Dienst anzuhalten. `final-logs/` enthält die getrennten Ausgaben.

| Befehl | Exit | Tatsächlicher Testbefund |
| --- | ---: | --- |
| `./scripts/test_brain_serve.sh` | 0 | 6 gezielte Tests bestanden, 0 fehlgeschlagen, 0 ignoriert, 26 herausgefiltert. Darunter Legacy-Import per CLI und Bibliothek, Prozess-E2E, synthetischer SCRAM-Fall sowie zwei Konfigurations- und Startprüfungen. |
| `./scripts/test_brain_core_postgres.sh` | 0 | 1 Scratch-PG-Test bestanden, 0 fehlgeschlagen, 0 ignoriert, 3 herausgefiltert. |
| `./scripts/test_brain_storage_upgrade.sh` | 0 | 1 Upgrade-/Restore-Test bestanden, 0 fehlgeschlagen, 0 ignoriert, 1 herausgefiltert. |
| `./scripts/test_wiki_runtime.sh` | 0 | Reguläre Suiten: 294 bestanden, 0 fehlgeschlagen, 9 ignoriert. Anschließend gezielter Wiki-IR/Store/Release-Scratch-Test: 1 bestanden, 0 fehlgeschlagen, 0 ignoriert, 7 herausgefiltert. |
| `cargo test --manifest-path rust/Cargo.toml -p brain-feeds --test assets_starting_stats --locked --offline --jobs 2` mit `env -i`, lokalem Cargo und `CARGO_TARGET_DIR=$PWD/rust/target` | 0 | 2 bestanden, 0 fehlgeschlagen, 0 ignoriert, 0 herausgefiltert. Für jeden Fixture-Helden werden fünf `starting_stats`-Felder mit konkretem Zahlenwert, JSON-Pointer, Herkunft und Sichtbarkeit geprüft. Fehlende oder nicht numerische Werte sperren den gesamten Batch. |

Der Match-Fall wurde zusätzlich gezielt mit `cargo test --manifest-path rust/Cargo.toml -p brain-feeds --test match_store postgres_match_commit_release_readback_replay_and_revoke --locked --offline --jobs 2 -- --ignored --exact` in `env -i` ausgeführt: **Exit 0, 1 bestanden, 0 fehlgeschlagen, 0 ignoriert, 1 herausgefiltert**. Dafür wurde ein eigener Cluster unter einem privaten `/tmp/brain-final-match.*/.match-test-pg`-Socket auf Port `55443` mit Peer-Authentisierung gestartet; `BRAIN_MATCH_TEST_PG_SOCKET`, `BRAIN_MATCH_TEST_PG_PORT` und `USER` zeigten auf diese Fixture. `listen_addresses=''` schloss TCP-Verbindungen aus. Geprüft wurden privater Match-Readback, Release und Replay, Demo-Evidenz, CLI-Revoke, Entzug auch für historische Releases und der atomare Rollback eines Mehrquellen-Commits mit ungültigem zweitem Lease-Fence. Der Cluster wurde gestoppt und entfernt. Nachweis: `final-logs/match-store.log`.

Der Prozess-E2E-Test in `rust/crates/brain-serve/tests/process_e2e.rs:230` umfasst die bestehenden 18 Szenarien und die ergänzten Pflichtfälle: öffentliche und private Fragen, Rechte in beiden Richtungen, Zahlenwert und EN/DE-Alias, unbekannte Entity und fehlende Evidenz, falscher Patch oder Modus, ungültige Berechtigung, gültiger und ungültiger Build, Hero Card und Providerfehler. Er prüft zusätzlich Konflikt bei `limit=1` vor Widerruf, Widerruf und Tombstone, Ausfall und Antwort nach Wiederherstellung im selben Prozess, Start bei leerem oder falschem Schema ohne Selbstmigration sowie Health, Readiness und Shutdown. Der eine Testfall ist nicht mit 18 separat gemeldeten Cargo-Testergebnissen zu verwechseln.

Der zusätzliche Offline-Abschlusslauf `flock -w 600 /tmp/deadlock-brain-release-build.lock env -i ... bash architecture/migration/s12/check-completion.sh` endete mit **Exit 0** und `complete=true, success=1`. Sämtliche neun Teilschritte meldeten Exit 0: Wiki-Offline-Suite (117 bestanden), Contracts und Sources (157 bestanden, 7 ignoriert), Clippy, Release-Build, Hero-Dossier-Filter (7 bestanden, 58 herausgefiltert), Workspace-All-Targets-Check, zwei Formatprüfungen und `git diff --check`. Der Runner nutzte `PATH=$HOME/.cargo/bin:$PATH`, `CARGO_TARGET_DIR=$PWD/rust/target`, `CARGO_BUILD_JOBS=2` und `SQLX_OFFLINE=true`. Seine flüchtigen Detailnachweise liegen lokal in `rust/target/wiki-completion.maNj9f/`; Exit-Überblick, Befehlsliste, Status und Head-Provenienz sind zusätzlich unter `final-logs/wiki-completion.log`, `final-logs/wiki-completion-results.tsv`, `final-logs/wiki-completion-status.txt` und `final-logs/wiki-completion-environment.txt` gesichert. `complete=true` bezeichnet die **Offline-Prüfschritte**, nicht einen echten Wiki-Capture.

## Prozesslast und Poolgrenze

Der einzige hier als bindend ausgewiesene Lastlauf stammt aus `./scripts/test_brain_serve.sh` auf `022f8a9`. Die drei Stufen liefen im Prozess-E2E gegen den privaten PG-Cluster. Die Werte sind tatsächlich beobachtete Antworten, keine hochgerechnete Quote.

| Worker | Anfragen | `answered` | Clientfehler | HTTP-/Statusfehler | beobachtete Reader-Verbindungen | Dauer |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 8 | 600 | 600 | 0 | 0 | 4 | 1.148 ms |
| 16 | 600 | 600 | 0 | 0 | 4 | 772 ms |
| 32 | 600 | 600 | 0 | 0 | 4 | 800 ms |

Poolmaximum 4, beobachtete Spitze 4, insgesamt 5 erstellte Verbindungen und 9.115 wiederverwendete Checkouts über den gesamten Prozess-E2E. `wait_count=5351`, `wait_max_micros=150070`, `wait_total_micros=20231810`; `wait_timeout_count=1` stammt aus dem **separaten, absichtlich gesättigten Pool-Test**, nicht aus einem der drei 600er-Läufe. Keine unerklärte 429-/503-Antwort, kein `unauthorized_evidence`-Antwortstatus, keine Clientfehler und kein „too many clients“ im Scratch-PostgreSQL-Log. Poollimit, Wartebudget und Retrygrenzen blieben unverändert. Rohwerte: `final-logs/serve.log`.

## Replay, Qualität, Cutover und Consumer

| Befehl aus dem Repo-Root | Exit | Befund |
| --- | ---: | --- |
| `cargo test --manifest-path rust/Cargo.toml -p dbrain-replay --all-targets --locked --offline --jobs 2` mit `env -i` und lokalem Cache | 0 | 57 bestanden, 0 fehlgeschlagen, 1 ignoriert, 0 herausgefiltert (`final-logs/replay-offline.log`). |
| `flock -w 600 /tmp/deadlock-brain-release-build.lock env -i ... bash architecture/migration/replays/s14/audit/check.sh` | 0 | 56 Tests bestanden, 0 fehlgeschlagen, 0 ignoriert. Der reale Korpus-Audit meldete erwartungsgemäß Exit 2 und `status=blocked`; der Wrapper prüfte genau diesen Ausgang und endete mit Exit 0 (`final-logs/replay-audit.log`). |
| `PATH="$HOME/.cargo/bin:$PATH" bash scripts/ci/check_consumer_offline.sh client` | 0 | Format, Tests und Clippy jeweils Exit 0; 14 Tests bestanden. |
| `PATH="$HOME/.cargo/bin:$PATH" bash scripts/ci/check_consumer_offline.sh quality` | 0 | Format, Tests und Clippy jeweils Exit 0; 85 Tests bestanden. |
| `PATH="$HOME/.cargo/bin:$PATH" bash scripts/ci/check_consumer_offline.sh cutover` | 0 | Format, Tests und Clippy jeweils Exit 0; 59 Tests bestanden. |

Die drei Consumer-Modi leeren intern das geerbte Umfeld und speichern getrennte `results.tsv`, Testlogs sowie Toolchain- und Head-Provenienz in `.consumer-ci-reports/{client,quality,cutover}/`. Die kurzen Exit-Ausgaben sind in `final-logs/consumer-client.log`, `quality.log` und `cutover.log` enthalten. Die zugehörigen Prüfungen sind offline; produktive Consumer wurden nicht aktiviert. Beim S14-Audit stehen `manifest_rows=0`, `real_matches=0` und `integration_verified=false`. Damit ist die Replay-Vorbereitung grün, die reale Replay-Integration aber ausdrücklich **NEIN**.

## Ignorierte Fälle und Grenzen

`cargo test --workspace --locked --offline --jobs 2 -- --ignored --list` endete mit Exit 0 und inventarisierte **75 Namen, ohne sie auszuführen**. Die vollständige nach Binary gruppierte Liste liegt in `final-logs/ignored-inventory.log`. Eine pauschale Ausführung mit `--include-ignored` fand nicht statt.

Gezielt unter Wegwerf-PG ausgeführt wurden **8** zuvor ignorierte Fälle: Match `postgres_match_commit_release_readback_replay_and_revoke`, Legacy `scratch_import_release_tombstone_and_revoke` und `cli_reads_archive_and_replays_tombstones_and_revokes`, Serve `binary_loopback_health_readiness_shutdown_and_no_fallback` und `private_scratch_scram_accepts_runtime_password_and_rejects_wrong_password`, Core `postgres_atomicity_fences_release_and_restart_contract`, Storage `v1_upgrade_restore_and_least_privilege` sowie Wiki `scratch_raw_ir_facts_release_delta_reparse_and_acl`. Diese acht Ergebnisse stammen aus den gezielten Aufrufen; in der unveränderten Workspace-Suite blieben sie korrekt als ignoriert markiert.

Die **67 übrigen** sind nach Voraussetzungen aufgeteilt: 58 Datenbank-/Snapshot-Fälle in Builds, Enrich, Learn, Normalize, Reasoner, Retrieval, Sources und YouTube benötigen `DEADLOCK_CENTRAL_DSN`, `REASONER_SCRATCH_DSN` oder dazu passende, nicht bereitgestellte Datenbestände; 4 lokale Pilotphasen benötigen einen separat freigegebenen Aufbau. 2 Wiki-Fälle benötigen echten Live-Capture, 1 Assets-Fall einen aktuellen externen Live-Vertrag und 1 Replay-Fall ist ein vom Subprozess verwendeter `sandbox_probe`-Helfer. Der einzelne Wiki-Manifest-Fall benötigt eine **leere** `brain_wiki_test_*`-Datenbank und verlangt im Test selbst TCP `127.0.0.1` (`rust/crates/dbrain-retrieval/src/wiki_manifest_tests.rs:13-32`); er wurde nicht fälschlich als Peer-/Unix-Socket-Nachweis ausgegeben oder gegen eine andere DB gestartet. Diese Gruppen ergeben 58 + 4 + 2 + 1 + 1 + 1 = 67. Ignoriert bedeutet in keinem dieser Fälle bestanden.

Ein echter Wiki-Pilot, Provider-Shadow, reale Replay-Partitionen und die frische externe CI-Abhängigkeitsauflösung bleiben **NEIN**; die lokalen Offline-Gates ersetzen diese Voraussetzungen nicht. Es wurden keine Quellen nachgeladen und keine echten Steam-, Discord- oder Twitch-Aktionen ausgelöst. Ein späterer G5-Entscheid benötigt eigene Freigaben und echte Daten, nicht eine Umdeutung dieses Berichts.

Die 17 getrennten `final-logs/*.log` wurden vor dem Commit auf Passwort-/Token-Werte, Datenbank-URLs mit Zugangsdaten, Kontrollzeichen, Compilerwarnungen und Testfehler geprüft; die geprüften Muster hatten jeweils 0 Treffer. `rust/target/` und `.consumer-ci-reports/` sind flüchtige Build- beziehungsweise Runner-Artefakte und gehören nicht zum Commit.

TESTNACHWEIS[TW-1]: 1011 passed, 75 ignored | Baseline: kein Altfehler behauptet
BESTAND[BS-1]: ja | Fundort: scripts/test_brain_serve.sh:1 | Anknüpfung: vorhandene sichere Runner und Offline-Audits verwendet
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: .tasks/2026-09-29-technical-closeout/FINAL-VERIFICATION.md
ORCHESTRIERUNG[OR-1]: Stufe groß | Schritt done | Artefakt: .tasks/2026-09-29-technical-closeout/FINAL-VERIFICATION.md
