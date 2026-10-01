status: erledigt
Datum: 2026-09-29

# Paket E: historische Provider- und Fixture-Prüfung

Dieser Autorenbericht dokumentiert die erste Abgabe vor dem unabhängigen Review. Die darin als behoben beschriebene Steam-Diagnose wurde anschließend wegen ungefilterter Fehlerstrings zurückgewiesen, mit typisiertem `PersistenceError` korrigiert und erneut unabhängig geprüft. Maßgeblich ist REVIEW-E.md: akzeptierter Head923f6a8, lokales ALLOW, regulärer Merge von Steam #82 alsf509f85e am29.09.2026 um12:51UTC. Kein Deploy, Neustart oder Publish. Die folgenden Testwerte gelten für ihren historischen Prüfstand, nicht für die spätere Korrektur.

## Historischer Stand

- Patchnotes-Basis: `db1d36398d70e64b501d4a720bcfa2ba1b25ab10` (`origin/main` zum Prüfzeitpunkt).
- Steam-Basis: `8c3fc6e0e8f5ab1c33a43a181c1eee5667818837`; Fix-Commits `0bb39a34bfa2b4b57f2a55e8cc3ff95308799bdd` und `6ead49768008208aac6851dbe12706ddc6820d8c` auf `review/brain-provider-20260929`.
- Brain-Berichtsbranch: `review/pre-g5-providers-20260929`, Ausgang `21d8579c48e0c92bffe4290946c15658610d9fdd`.
- Draft-PR [#82](https://github.com/EarlySalty/Deadlock-Steam-Bot/pull/82) ist gegen `main` eröffnet und ungemergt. Der Steam-Testkonfigurationsdiff ist leer. Der Patchnotes-Prüfworktree und sein lokaler Branch wurden entfernt.
- Es gab keinen echten Steam-GC-Aufruf, Build-Publish oder Deploy. Kein Merge nach `main` wurde ausgeführt.

## Vertragsnachweise

1. Patchnotes `brain.feed.patchnotes.v1`: Provider validiert die Feed-Felder, prüft Raw-Hash, Post-ID sowie `source_revision` und `export_revision`, sortiert stabil und begrenzt Posts und Antwortgröße. Die Brain-Fixture wird vom Brain-Adapter erneut gehasht und erfolgreich verarbeitet.
2. Steam `brain.build_publish.v1`: POST und GET, Authentifizierung, Schema- und Größenprüfung, Request-Hash-Konflikt als 409, Datenbank-Transaktion für Task und Request-Mapping sowie persistenter Status sind abgedeckt. Der DB-Test belegt parallele gleiche Anfragen mit genau einem Task und Statusabruf nach Rekonstruktion. Migration und Tabellen-Constraints wurden gelesen.
3. Die Steam-Task- und GC-Antwortpfade wurden statisch geprüft. GC-Fehler laufen als Taskfehler zurück, werden in `runner.rs:384` protokolliert und als `FAILED` persistiert. Die HTTP-/DB-Tests rufen den Steam-Game-Coordinator nicht auf. Damit ist die Task-Warteschlange geprüft, ein realer äußerer Publish-Effekt nicht.

## Befunde

1. **Patchnotes-Fehlerdiagnostik:** `patchnotes-brain-verification-20260929/brain_feed.py:328-329` fängt unerwartete Feed-/DB-Fehler ohne Logeintrag und antwortet mit 503. Die Antwort bleibt fail-closed und verrät keine Treiberdetails, aber Betreiber sehen Ursache und Kontext nicht im Provider-Log. Wiederholte Exception-Stellen in `patchnotes_db.py:94-109` betreffen Poolstart und sind getrennt davon geprüft. Zwillingssuche: ein Laufzeit-Catch in `get_feed`; Parsing-Catches sind keine Datenbankpfade.
2. **Steam-Fehlerdiagnostik, behoben:** `steam-web/src/routes/builds.rs` protokolliert die drei Store-Fehlergrenzen mit den unterschiedlichen Operationsschlüsseln `builds.publish.submit`, `builds.publish.fetch_outcome` und `builds.publish.get_request` sowie dem Store-Fehler. Die HTTP-500-Codes und generischen Antworttexte bleiben unverändert. Der Test `store_failures_keep_generic_http_responses_and_log_distinct_operations` prüft die drei Logereignisse und verhindert, dass Fehlerdetails oder Request-IDs in der HTTP-Antwort erscheinen. Umsetzung in `0bb39a34` und `6ead4976`, Draft-PR #82.
3. **Grenze des Steam-Nachweises:** Die Tests belegen persistente API-Idempotenz und Taskzustände, nicht die Wiederholbarkeit eines unklaren Steam-GC-Timeouts. Der echte GC-Pfad wurde aus Sicherheitsgründen nicht ausgeführt; seine externe Wirkung bleibt unbewiesen.

## Testnachweis

- Patchnotes, Arbeitsverzeichnis `/home/nathanael/.worktrees/patchnotes-brain-verification-20260929`: `uv run --with aiohttp==3.14.3 python -m unittest discover -s tests -p 'test_brain_feed.py' -v`; Exit 0, 33 Tests.
- Brain, Arbeitsverzeichnis `/home/nathanael/.worktrees/brain-pre-g5-providers-20260929/rust`: `PATH=/home/nathanael/.cargo/bin:$PATH cargo +1.97.1 test -p brain-feeds --locked --offline -- --include-ignored`; Exit 0, 9 bestanden.
- Steam fmt: `PATH=/home/nathanael/.cargo/bin:$PATH cargo +1.97.1 fmt --package steam-web -- --check`; Exit 0.
- Steam clippy: `PATH=/home/nathanael/.cargo/bin:$PATH cargo +1.97.1 clippy -p steam-web --all-targets --features testing --locked --offline -- -D warnings`; Exit 0.
- Steam gezielter Fehlerpfadtest: `PATH=/home/nathanael/.cargo/bin:$PATH cargo +1.97.1 test -p steam-web --features testing --locked --offline -- routes::builds::tests::store_failures_keep_generic_http_responses_and_log_distinct_operations --include-ignored`; Exit 0, 1 bestanden, 85 gefiltert.
- Steam vollständige Suite: `PATH=/home/nathanael/.cargo/bin:$PATH cargo +1.97.1 test -p steam-web --features testing --locked --offline -- --include-ignored`; Exit 0, 86 bestanden, 0 fehlgeschlagen, 0 ignoriert. Der Lauf nutzte Wegwerf-Postgres mit Peer-Authentifizierung.
- Steam-Vertrag, gleiches Arbeitsverzeichnis: `PATH=/home/nathanael/.cargo/bin:$PATH cargo +1.97.1 test -p api-contract --locked --offline -- --include-ignored`; Exit 0, 24 bestanden.
- Ein Steam-Gesamtlauf nach vorzeitigem Zurücksetzen von `.github/ci/test-db.conf` endete mit Exit 101 und SQLSTATE `28P01` für den konfigurierten CI-DB-Nutzer `ci`. Die Test-URI wurde erneut temporär auf Unix-Socket/Peer gestellt; der oben gemeldete Gesamtlauf bestand. Die Datei wurde anschließend auf ihren Ausgangswert zurückgesetzt. Es wurden keine Passwort- oder DSN-Umgebungsvariablen verwendet.
- `cargo +1.97.1 fmt --all -- --check` endete mit Exit 1 wegen bestehender Formatabweichungen in Dateien des benachbarten Deadlock-Bots-Workspace. Der Paketcheck für `steam-web` bestand; fremde Dateien wurden nicht geändert.
- Finaler Suite-Summenstand: 152 bestanden, 0 fehlgeschlagen, 0 ignoriert. Ausgangsstand vor der Diagnoseänderung: 151 bestanden, 0 ignoriert.

## Rust-Nachfolger und historische CI-Ursachen

- Der Patchnotes-Rust-Code `brain-feeds/src/patchnotes.rs` ist Consumer-Code: Er parst den Feed und bereitet Datensätze vor. Er implementiert weder den HTTP-Provider noch dessen Datenbankzugriff. Im Patchnotes-Repository zeigt der Graph als Provider-Laufzeit `brain_feed.py` und `brain_feed_server.py`; ein Rust-Provider-Nachfolger wurde nicht gefunden. Befund 1 bleibt eine Legacy-Diagnoselücke. Daraus folgt keine Rust-Readiness-Aussage.
- Patchnotes PR #49, Head `0c9fd6fd1393849391de086cac8ffdf388ed0814`, Run [36264192220](https://github.com/EarlySalty/Deadlock--Patchnotes-Bot/actions/runs/36264192220): Python SAST + Dependency Audit, Trivy Filesystem Scan und Semgrep SAST endeten als `failure`. Die Annotationen dieser Jobs sagen: “The job was not started because recent account payments have failed or your spending limit needs to be increased.” Die Annotation unterscheidet nicht zwischen den genannten Abrechnungs- und Ausgabenlimit-Ursachen. Es gab keine Steps und keine Runner-Zuweisung. Das belegt einen Jobstartfehler, keinen Security-Fund im Code.
- Steam PR #73, Head `c12e9ab8e10b84af538316a5b482962d19780dcf`, Runs [36263890346](https://github.com/EarlySalty/Deadlock-Steam-Bot/actions/runs/36263890346) und [36263888015](https://github.com/EarlySalty/Deadlock-Steam-Bot/actions/runs/36263888015): `clippy + test` und `cargo-audit (informativ)` wurden mit derselben Jobstart-Annotation abgewiesen. Der Merge-Commit-Run [36265482009](https://github.com/EarlySalty/Deadlock-Steam-Bot/actions/runs/36265482009) zeigt dieselbe Ursache bei beiden Jobs. Workflow-Logs sind leer, die Check-Run-Annotationen enthalten den konkreten Grund. Es gibt daraus keinen Beleg für einen Rust-, Clippy- oder Audit-Codefehler.

ORCHESTRIERUNG[OR-1]: Stufe mittel | Schritt done | Artefakt: .tasks/2026-09-29-technical-closeout/E-REPORT.md
TESTNACHWEIS[TW-1]: 152 passed, 0 ignored | Baseline: 0 rot
WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: .tasks/2026-09-29-technical-closeout/E-REPORT.md
