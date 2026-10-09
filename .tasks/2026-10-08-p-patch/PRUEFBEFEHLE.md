# P: tatsächliche gemeinsame Prüfungen

08.10.2026. Quellcheckpoint ee272b60, anschließend auf frisch geholten Main 2e0de01f0da4bbb7f6630832b52b0694ea566c6f rebasiert. Vollständiger autorisierter Quellbereich nach Rebase per git diff --exit-code unverändert, Exit 0. Neue Mainänderungen betreffen ausschließlich Q-Auftragsartefakte, keine Patchquelle.

## Patchsuite mit beiden echten Postgresfällen

```bash
cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-p-delivery-20261008/rust/Cargo.toml --config 'env.SQLX_OFFLINE="true"' --config 'env.DEADLOCK_BRAIN_SCRATCH_DSN="host=/tmp/brain-p-patch-pg-20261008.UIcyXW/cluster port=55449 user=nathanael dbname=brain_fixer12_patch_lookup"' --config 'env.DEADLOCK_BRAIN_REIMPORT_SCRATCH_DSN="host=/tmp/brain-p-patch-pg-20261008.UIcyXW/cluster port=55449 user=nathanael dbname=brain_p_patch_reimport"' --jobs 3 -p deadlock-brain --bin deadlock-brain pg_patchnotes:: -- --include-ignored --test-threads=1
```

47 passed, 0 failed, 0 ignored, 73 filtered. Exit 0. Log /tmp/brain-p-delivery-patch-tests-r3.log. Beide zuvor ignorierten Lookup-/Reimportfälle liefen wirklich. Schema-only-Abzug bestehender Tabellen einschließlich FK und Constraints, keine Produktionsdaten. Synthetische offizielle Originaltexte, echter Parser und echte commitende Importtransaktionen. Zweimal Steam und zweimal Forum, zwei Gameplayereignisse je Original, konsistente Quellen-/Snapshot-/Knowledgebindungen. Fremde Originalidentität und Abruffehler ändern keine gespeicherten Ereignisse.

## Vollständige vorhandene CLI-Suite

```bash
cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-p-delivery-20261008/rust/Cargo.toml --config 'env.SQLX_OFFLINE="true"' --jobs 3 -p deadlock-brain --bin deadlock-brain -- --test-threads=1
```

117 passed, 0 failed, 3 ignored, 0 filtered. Exit 0. Log /tmp/brain-p-delivery-cli-tests-r3.log. Zwei ignorierte Patchfälle sind im separaten Lauf oben wirklich geprüft. Der dritte ignorierte Fall ist das unveränderte Steam-Abrufjournal, kein Patch-Steam-APIaufruf oder entsprechender Livebeweis behauptet. CLI-/Timerregressionen bestanden.

## Reader und Compilerprüfung

```bash
cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-p-delivery-20261008/rust/Cargo.toml --config 'env.SQLX_OFFLINE="true"' --jobs 3 -p dbrain-sources --lib forum:: -- --test-threads=1
cargo-slot +1.97.1 clippy --manifest-path /home/nathanael/.worktrees/brain-p-delivery-20261008/rust/Cargo.toml --config 'env.SQLX_OFFLINE="true"' --jobs 3 -p deadlock-brain --bin deadlock-brain --tests -p dbrain-sources -- -D warnings
```

Reader 5 passed, 0 failed, 1 ignored, 220 filtered. Exit 0, /tmp/brain-p-delivery-reader-tests.log. Der ignorierte unveränderte Browserfall verlangt Brave und wurde wegen ausdrücklichem Brave-Verbot nicht ausgeführt. Kein Browserkompatibilitätsbeweis daraus.

Finales Clippy Exit 0, /tmp/brain-p-delivery-clippy-r3.log. rustfmt +1.97.1 --edition 2021 --check --config skip_children=true ausschließlich auf main.rs, pg_patchnotes.rs, api_sync.rs und forum.rs: Exit 0. bash -n auf vorhandenem Timerskript: Exit 0. Keine globale Formatierung.

Die zunächst geforderten env -u-Aufrufe wurden vom Worktree-Isolationshook syntaktisch abgewiesen. Entsprechend dessen Hinweis plain cargo-slot mit Cargo-eigener SQLX_OFFLINE-Konfiguration und ausdrücklich eigener Scratch-DSN verwendet. Kein Hook oder Umgebungs-Skipflag verändert. Testverbindungen enthalten keine Geheimnisse und zeigen ausschließlich auf den eigenen Unixsocket-Cluster.

TESTNACHWEIS[TW-1]: 47 passed, 0 ignored | Baseline: n/a rot

Baseline n/a: keine vorbestehenden Fehler behauptet. Zwei tatsächliche Fehlversuche 46/1/0 führten zu kompaktem Mehrzeilenparserfix und SQL-Parameter-Typfix. Der frühere Lookupversuch fiel an ConnectionRefused des nicht mehr laufenden eigenen Clusters, nicht an Fachcode. Alle Protokolle erhalten. Die Läufe sind getrennt, Zahlen nicht durch Doppelzählung summiert.

## Frischer Fixer nach tatsächlichem BLOCK, Runde 1

Die Zahlen oben sind historisch. Die folgenden Läufe prüfen die korrigierte Quelle nach e7d90c3644a3a57c94ae52a21febec6e0df0eb83, mit vier zusätzlichen Regressionstests. Werkzeug jeweils /home/nathanael/.local/bin/cargo-slot, intern /home/nathanael/.cargo/bin/cargo, +1.97.1, --jobs 3 und SQLX_OFFLINE als Cargo-Konfiguration. Bestehende Suites und ignorierte Fälle sind erhalten.

- API-Vorprüfung: derselbe Patchtestbefehl, Filter pg_patchnotes::api_sync::tests::, ohne --include-ignored und ohne Scratch-Konfiguration. 25 passed, 0 failed, 2 ignored, 97 filtered, Exit 0. /tmp/brain-p-delivery-fixer-r1-unit.log. Kein Datenbankbeweis aus diesem Vorlauf.
- Vollständige Patchsuite: exakter Befehl oben mit beiden getrennten Scratch-DSNs und --include-ignored --test-threads=1. 51 passed, 0 failed, 0 ignored, 73 filtered, Exit 0. /tmp/brain-p-delivery-fixer-r1-patch.log. Lookup und commitender Reimport liefen tatsächlich. Vorher eigener Unixsocket-only-Cluster, Datenbankname, leere Lookupdatenbank und synthetische Reimportbestände lesend bestätigt. Ausschließlich brain_p_patch_reimport in diesem Cluster neu angelegt und mit /tmp/brain-p-delivery-reimport-schema.sql aufgebaut, Exit 0, /tmp/brain-p-delivery-fixer-r1-schema.log. Der Reimport prüft nun die Überschrift Holliday mit anschließendem Knockback-Bullet samt persistierten Dokument-/Snapshot-/Knowledgebindungen.
- Reader: exakter Befehl oben ohne --include-ignored. 5 passed, 0 failed, 1 ignored, 220 filtered, Exit 0. /tmp/brain-p-delivery-fixer-r1-reader.log. Der unveränderte ignorierte Brave-Fall wurde nicht ausgeführt. Keine Browserprüfung behauptet.
- Striktes Clippy: exakter Befehl oben, Exit 0. /tmp/brain-p-delivery-fixer-r1-clippy.log. rustfmt +1.97.1 --edition 2021 --check --config skip_children=true auf denselben vier autorisierten Quelldateien, Exit 0. /tmp/brain-p-delivery-fixer-r1-fmt.log. Formatierung ausschließlich im eigenen API-Modul; main.rs enthält eine Hilfezeilenänderung.
- Vollständige CLI-Suite: exakter Befehl oben mit --test-threads=1. Wiederholung 121 passed, 0 failed, 3 ignored, 0 filtered, Exit 0. /tmp/brain-p-delivery-fixer-r1-cli-retry.log. Die beiden ignorierten Patchfälle liefen im getrennten 51er-Lauf; der unveränderte Steam-Journalfall blieb ignoriert. Der erste R1-Aufruf wurde nach 600 Sekunden ohne Ausgabe aus cargo-slot durch die Werkzeugzeitgrenze beendet, nicht als bestandener Testlauf gezählt. Originalprotokoll /tmp/brain-p-delivery-fixer-r1-cli.log bleibt erhalten. Keine Rust-Testfehler als Altfehler oder Flake behauptet.

TESTNACHWEIS[TW-1]: 51 passed, 0 ignored | Baseline: n/a rot
