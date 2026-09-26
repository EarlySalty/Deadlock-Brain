# C7/C8 — Runtime-Konfiguration und Prüfwerkzeuge

Stand: 26.09.2026. Repository: `EarlySalty/Deadlock-Brain`. Ausschließlich C7/C8 aus `INTEGRATION_REVIEW.md`; keine Bearbeitung der übrigen Integrationsaufträge.

Branch: `codex/fix-c7-c8-runtime-tooling`.
Basis: `origin/migration/rust-integration`, `087c522deda58ecf4bd6843167f51c54f681e944`.
Implementierungscommit: `395c38e5a8cbe1fee1979c07404a503b04021cc6`.
Isolierter Worktree: `/home/nathanael/.worktrees/brain-fix-c7-c8-runtime-tooling`.

Der bestehende schmutzige Hauptcheckout wurde nicht geändert, zurückgesetzt oder gestasht. Kein Merge, Deployment, Produktionsimport und keine Installation, Aktivierung, Ausführung oder Neustart einer Unit. Die tatsächliche Systemd-/Procfs-Beobachtung war ausschließlich lesend.

## C7: explizite Quellenkonfiguration

`dbrain-sources::source_pins` ergänzt die bestehende Quellen-Crate um `SourcePins` und `DeadlockDataPin`. Config-Schema 1, vollständiger lowercase 40/64-Hex-Commit und die tatsächlich implementierte Parserrevision sind Pflicht. Optional können die erwartete SourceIr-Schemaversion und die exakte `ClientVersion` des gepinnten `data/version.txt` festgelegt werden. Unbekannte Felder und doppelte Schlüssel in der eigenständigen Pin-Datei werden abgewiesen; deren Lesen ist auf 64 KiB begrenzt.

Der Import prüft Pin, Commitobjekttyp, lokale Verfügbarkeit, Repository-Origin und optionale Data-Version vor Pool-/SourceStore-Schreibzugriffen. Der CLI- und Wiki-Job führen diese Prüfung zusätzlich vor ihrem Poolaufbau aus. Es gibt keinen HEAD-/Branch-/Tag-Fallback, automatischen Fetch oder neuesten Commit. Vorhandene Git-Objekt-Sicherungen bleiben aktiv; der Arbeitsbaum ist nicht Datenautorität.

`pull deadlock-data --source-config PATH` beziehungsweise `DBRAIN_SOURCE_PINS_CONFIG` liest die neue JSON-Config. Die explizite Kompatibilitätsvariante `--commit`/`DBRAIN_DEADLOCK_DATA_COMMIT` benötigt zusätzlich `--parser-revision`/`DBRAIN_DEADLOCK_DATA_PARSER_REVISION`. Bei Verwendung einer Pin-Datei sind zusätzliche CLI-/Environment-Pinfelder ein Fehler, kein versteckter Override. Einmal aufgelöste Optionen werden unverändert weitergereicht. Die bestehende Patchnote-Normalisierung nach dem Import bleibt erhalten und verwendet denselben Pool. `pull patchnotes` aus dem Postgres-Patchnotes-Schema ist kein Git-Quellenabruf und erhält keinen künstlichen Commit-Pin.

`wiki refresh --config PATH` verlangt dieselbe Struktur unter `source_pins` direkt in seiner JSON-Datei. Für diesen Pfad gibt es keine Umgebungs-Pinüberschreibung. Beide CLI-Einstiege unterstützen `--check-config`: lokale Pinprüfung, `valid:true`/`writes:false`, kein Pool, kein HTTP-Import, keine Publikation. Die bestehenden Settings des Pull-CLI bleiben erhalten; der neue Preflight ruft keinen Credential-Provider auf. `--skip-source-update` bleibt ausdrücklich Export des vorhandenen Datenbankstands, kein reproduzierbarer Neuimport dieses Pins.

### Konfigurations- und Unit-Beispiele

`config/source-pins.json`, `config/wiki-refresh.json`, `.env.example`, `ops/deadlock-brain-wiki-refresh.service` und die Erläuterungen unter `ops/source-pinning.md` sind aktualisiert. Die Unit-Vorlage enthält nur einen zusätzlichen read-only `ExecStartPre --check-config`; sie wurde nicht installiert oder ausgeführt. Der bestehende Infisical-/Secret-Exec-Mechanismus bleibt bestehen, keine Secrets stehen in den neuen Dateien.

Der Beispielpin `66d0832dc30148ca66984f3f4c4a4d340af6f7d0` wurde ausschließlich aus dem im Integrationsreview bereits dokumentierten `66d0832d` aufgelöst. `ClientVersion=6698` wurde direkt aus diesem Commitobjekt gelesen. Es wurde **kein neuer oder aktuellster Commit ausgewählt**. Parserpin: `dbrain-deadlock-data/2`; SourceIr-Version: 1. Ein späteres Update benötigt ausdrückliche Configänderung und gegebenenfalls einen separaten Abruf des freigegebenen exakten Gitobjekts durch den Betreiber.

### Optionaler Wiki-Korpus

Bei `wiki.enabled=true` ist `wiki.source_pin` Pflicht: Parser `dbrain-wiki-corpus/1`, optionale Schema-Version 1, freigegebene Lizenzangaben sowie eine endliche Liste exakter Seiten-/Revisions-IDs und Renderantwort-Hashes. Der Runtimepfad fragt nur konfigurierte `oldid` ab, nicht neueste Revisionen oder ein neues Allpages-Inventar.

`response_sha256` ist `external::normalized_hash` der vollständigen freigegebenen Parse-JSON-Antwort. Das ist eine deterministische JSON-Repräsentation, kein HTTP-Bytehash und kein RFC-8785-Versprechen. Ändert sich eine transkludierte Vorlage bei gleicher Artikelrevision, wird die abweichende Renderantwort abgewiesen. `rendered_templates_pinned:false` bleibt korrekt: einzelne Vorlagenabhängigkeiten sind dadurch nicht separat revisionsgepinnte Quellen. `rendered_content_hash_pinned:true` belegt die Inhaltsprüfung. Ist der alte Render nicht mehr lieferbar, gibt es einen sichtbaren Jobfehler statt stiller Datenaktualisierung. Der S12-Capture-/Discovery-Auftrag C5 wird hier nicht vorweggenommen.

## C8: nicht mutierende Checks und gelöschte Runtimes

`architecture/migration/s12/check-completion.sh` ersetzt das entfernte Python-Programm. Bash/Coreutils koordinieren die neun bestehenden Offline-Prüfschritte mit derselben Fehlerweitergabe. Jeder Lauf erhält ein eigenes `rust/target/wiki-completion.*/` mit Einzelprotokollen, exakten Befehlen/Exitcodes in `completion-results.tsv`, Quellhashes, Toolversionen und Abschlussstatus. Testprozesse erhalten eine kleine Build-Environment-Allowlist und ein isoliertes HOME. Die versionierten `reports/` bleiben unveränderte historische Evidenz.

`check.sh` verwendet bei gesetztem `CARGO_TARGET_DIR` auch das dort gebaute Probe-Binary. Der integrierte Wiki-CI-Schritt ruft das Shellwerkzeug auf, archiviert seine neuen Artefakte und verlangt anschließend einen leeren Git-Status. Historische Erwähnungen des alten Python-Skripts in Review/Handoff/Golden-Berichten bleiben bewusst historisch, aktive Aufrufe verwenden Bash.

Das Runtime-Audit klassifiziert zusätzlich den Prozessnamen aus dem ohnehin gelesenen `/proc/<pid>/stat`, versionierte Python-/PyPy-Namen einschließlich üblicher ABI-Suffixe sowie benannte Python-/PyPy-Bibliotheken und ausführbare Interpreter-Mappings. Gelöschte Pfade werden bei jeder Erkennung berücksichtigt. Die bestehende Suffixbehandlung allein reichte für den dokumentierten Fall nicht aus. Namen ähnlicher Hilfsprogramme und lediglich nicht ausführbar gemappte Python-Dateien werden nicht pauschal zu Kandidaten.

Bestehende Größen-/Zeitbudgets, Cgroup-v1/v2-Beschränkungen, PID-/Startzeit-/Exe- und Membership-Nachprüfungen sowie fehlerschließende Metadatenfehler bleiben erhalten. Kein `/proc/*/environ`, `/proc/*/cmdline`, `/proc/*/mem`, `ptrace`, Speicherinhalt oder erhöhte Rechte werden verwendet. Ausgegeben werden feste Befundkategorien, keine rohen Prozesspfade. Ein statisch eingebetteter, vollständig umbenannter oder zwischen zwei Beobachtungen verschwundener Interpreter kann ohne benannte Metadaten weiterhin unsichtbar sein. Deshalb bleiben `point_in_time_only=true`, `full_runtime_verification=not_performed` und keine Python-frei-/Cutover-Zusage.

## Verifikation

Die folgenden Befehle wurden am oben genannten **sauberen Implementierungscommit** ausgeführt. Rust/Cargo 1.97.1 auf PATH, `SQLX_OFFLINE=true`, zwei Buildjobs, keine DSN-/Provider-Secrets an die Prüfprozesse übergeben. Der eigenständige Release-Nachweis verwendet zusätzlich `CARGO_NET_OFFLINE=true`; die angeforderte Befehlszeile bleibt unverändert. Der anschließende Handoff-/README-/Betriebsdokumentationscommit ändert keinen Rust- oder Shellcode.

| Prüfung | Ergebnis | Lokale Evidenz unter `.consumer-ci-reports/` |
|---|---|---|
| `cargo fmt --all -- --check` in `rust/` | Exit 0 | `verification/results.tsv`, `verification/fmt.log` |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Exit 0 | `verification/results.tsv`, `verification/clippy.log` |
| `cargo test --workspace --locked` | Exit 0; 721 bestanden, 0 fehlgeschlagen, 67 ignoriert | `verification/test.log` |
| `cargo build --workspace --release --locked` | Exit 0 | `release-standalone.log`, `.exit`, `.commit` |
| Eigenständiges Runtime-Audit: fmt, Clippy `-D warnings`, Tests, Release | jeweils Exit 0; 59 Tests bestanden | `verification-tooling/results.tsv`, `audit-*.log` darunter |
| `bash architecture/migration/s12/check-completion.sh` | alle neun Schritte Exit 0, `complete=true`, `success=1`; 223 bestanden, 0 fehlgeschlagen, 6 ignoriert | tatsächlicher Lauf `rust/target/wiki-completion.UlzI89/` |
| Release-Binary: Quellen- und Wiki-`--check-config` gegen den vorhandenen, exakt gepinnten Cache | jeweils Exit 0, `valid:true`, `writes:false` | `preflights-release.tsv`, `source-preflight-release.json`, `wiki-preflight-release.json` |
| `git status --short` vor der Handoff-Erstellung, nach den Prüfungen | leer, 0 Bytes | `C7_C8_VERIFIED_STATUS.txt` |

Die 67 ignorierten Workspace-Tests sind kein erfolgreicher Live-/DB-/Provider-Nachweis. Die Ergebnisse des Wiki-Checks wiederholen teilweise Workspace-Tests und werden nicht als zusätzliche eindeutige Tests addiert. Historische Golden-Berichte wurden nicht geändert.

Die Evidenz wird schrittweise statt über einen einzigen langen Sammellauf gebunden: Die ersten drei Pflichtprüfungen stehen in `verification/`, der separat abgeschlossene Release-Build in `release-standalone.*`, die Audit- und Wiki-Nachweise in den genannten eigenen Verzeichnissen. Unvollständige zusätzliche Wiederholungsprotokolle werden nicht als erfolgreiche Gesamtläufe ausgegeben. Der vorhandene `dbrain-reasoner/build.rs` erzwingt über seinen Sentinel bei jedem Build neue Git-Provenienz und Folgekompilationen; diese Schutzfunktion wurde nicht entfernt.


Die gezielten Quellen-/CLI-Tests bestanden mit 160 Tests, 6 vorhandenen ignorierten Tests, 0 Fehlern. Abgedeckt sind gültige, falsche und fehlende Pins; lokal fehlende Commitobjekte; Nicht-Commitobjekte; falsche Origin/Parser-/Schema-/Data-Version; reproduzierbare Gitblob-Materialisierung trotz neuerem HEAD und geändertem Arbeitsbaum; Update erst nach Configänderung; Wiki-Renderdrift; frühe Jobfehler ohne Pool-/Publikationszugriffe. Beobachtungs-, Run- und Renderzeitstempel sind keine reproduzierbare Inhaltsidentität.

Der Check-Runner-Vertragstest verwendet ausdrücklich Toolstubs nur für Orchestrierung, Fehlerpropagation und Git-Unverändertheit; er wird nicht als echter Compilerlauf ausgegeben. Die oben ausgewiesenen Workspace-/Wiki-Prüfungen verwenden die echte Rust-Toolchain.

### Tatsächlicher read-only Runtimebefund

Der neue Auditlauf auf dem Arbeitsserver endete mit Exit 0 und meldete `python_candidates=1`, `deleted_executables=1`. `deadlock-brain-site.service` wird trotz gelöschter Executable als `python_candidate` erkannt. Das ist der zuvor dokumentierte Falschnegativ-Fall, nun mit positivem Metadatenbefund. Keine Unit wurde hierfür gestartet oder verändert. Lokale Protokolle: `.consumer-ci-reports/runtime-readonly.txt`, `runtime-readonly-error.txt`, `runtime-readonly.exit`.

## Grenzen und weitere Betreiberarbeit

Kein Unit-Starttest, produktiver Quellenimport oder Datenbank-/Wiki-Cutover wurde durchgeführt. Der sichere CLI-Preflight ersetzt keine spätere, gesondert freigegebene Runtime-/DB-Abnahme. Der neue Wiki-Pin enthält keinen erfundenen Live-Capture; Aktivierung benötigt separat geprüfte Revisions-/Renderdaten. Die übrigen Integrationsgates und C1–C6/C9–C11 bleiben außerhalb dieses PRs. Dieser Handoff ist keine Produktionsfreigabe.
