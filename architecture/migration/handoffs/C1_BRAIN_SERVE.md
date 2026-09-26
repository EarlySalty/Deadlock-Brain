# C1 — Startbarer Rust-Core `brain-serve`

Stand: 26.09.2026. Ausschließlich C1 aus `architecture/migration/INTEGRATION_REVIEW.md`.

## Herkunft und Ergebnis

- Basis: `origin/migration/rust-integration`, Commit `087c522deda58ecf4bd6843167f51c54f681e944`.
- Branch: `codex/fix-c1-brain-serve`.
- PR: `EarlySalty/Deadlock-Brain#42`, Ziel `migration/rust-integration`.
- Worktree: `/home/nathanael/.worktrees/brain-fix-c1-brain-serve`.
- Ergebniscommit der Implementierung: `9e2e7863d6d4b3f92da17fbc88ceb2d40516c4ad`.
- Dieser Handoff wird anschließend in einem separaten Dokumentationscommit ergänzt; die obige SHA bezeichnet bewusst den Code, nicht eine unmögliche Selbstreferenz des Dokumentationscommits.
- PR-Ziel: `migration/rust-integration`. Kein Merge, kein Deploy, keine installierte/aktivierte systemd-Unit und keine Änderung an Produktionsdiensten oder Produktionsdatenbanken.

## Implementierter Dienst

`brain-serve` ist ein neues Workspace-Package mit eigenem Executable. Der Produktivpfad ist vollständig Rust:

```text
strikte JSON-Config + explizite Secret-Environment-Referenzen
    -> PgStore (SQLx-Pool, Startup-/Readiness-Prüfung)
    -> LocalPgReader (native PostgreSQL-Reads und persistente Conversation-Ownership)
    -> ReleaseRetriever
    -> Kernel + CachedKernel
    -> OpenAiCompatibleProvider über den vorhandenen Provider-Port
    -> PolicyEngine + brain-api::ApiService + vorhandener HTTP-Router
    -> tatsächlich gebundener TCP-Listener
```

Es wird kein In-Memory-Store als Ausfallersatz eingesetzt. Kein Python-Prozess und kein Legacy-HTTP-Client werden vom Dienst gestartet. Die bestehenden API-/Auth-Verträge, Scope-Prüfungen, Evidence-Revalidierungen, Cache-Grenzen, Provider-Retries und Kernel-Semantik bleiben erhalten. Conversation-Ownership wird im neuen Dienst ausdrücklich über den tatsächlichen PostgreSQL-Reader persistiert und über Prozessneustarts hinweg geprüft.

Die bestehende Fact-/Rule-/Prose-Evidenzverarbeitung des Kernels wird verwendet. Die in C6 geforderte zusätzliche Hero-Karten-/Build-Solver-Integration wird hier weder nachgebaut noch als fertig ausgegeben. Auch C2 (Chunking/Indexierung) und C3 (Fehlerklassifikation) bleiben unverändert.

### Startup ist fail-closed

Vor dem Öffnen des API-Listeners werden geprüft:

1. Die Config ist eine lesbare reguläre JSON-Datei von höchstens 64 KiB. Fehlende Pflichtfelder, unbekannte Felder, ungültige Werte und nicht unterstützte Provider-/Retrieval-Arten werden abgewiesen; es gibt keine stillen Default- oder Clamp-Korrekturen.
2. Alle referenzierten erforderlichen Secrets sind vorhanden und gültig. API-Tokens dürfen nicht mehrdeutig mehrfach vergeben werden. Inline-Felder wie `password`, `token` oder `api_key` sind im Config-Schema nicht zulässig.
3. PgStore erreicht die explizit konfigurierte lokale Datenbank. Der konfigurierte Release existiert vollständig und passt zur Knowledge-Version. `current` und `latest` sind keine zulässigen Release-/Knowledge-Pins.
4. Die Rolle besitzt die für Conversation-Ownership erforderlichen SELECT-/INSERT-Rechte. Der tatsächliche LocalPgReader kann denselben Release lesen und validieren.

Es gibt ein Gesamt-Startup-Limit. Fehler liefern einen erfolglosen Prozess-Exit und einen statischen, redigierten Fehlercode. Insbesondere werden weder eine andere Datenbank noch ein anderer Release noch ein anderer Provider ausgewählt.

**Keine Migration beim Dienststart:** Das Schema und der Release müssen zuvor durch einen separat autorisierten Setup-/Ingest-Schritt bereitgestellt worden sein. `brain-serve` ruft `migrate_core` nicht auf und benötigt keine DDL-Rechte. Die Migration in den Test-Harnesses betrifft ausschließlich deren Scratch-Datenbanken.

### PostgreSQL und Credentials

Die reale Integration bleibt beim vorhandenen **Unix-Socket-Reader**. Ein TCP-Postgres-Host oder eine beliebige DSN ist hier keine unterstützte alternative Konfiguration. `LocalPgReader::new` bleibt rückwärtskompatibel; `with_connection_options` ergänzt explizite Passwort-/Timeout-Übergabe, ohne die alten Defaults anderer Aufrufer zu verändern.

`postgres.auth = "peer"` bedeutet: kein Passwort wird benötigt oder aus einer anderen Quelle nachgeladen; die lokale HBA-Regel muss den Zugriff ohne Passwort erlauben. Die Scratch-Tests nutzen für die Test-Adminrolle ausdrücklich `trust`. `postgres.auth = "password"` benötigt zusätzlich `postgres.password_env` und das zugehörige nicht leere Environment-Secret. Der E2E-Test prüft diesen Pfad mit einer echten SCRAM-Regel und einer eingeschränkten Dienstrolle.

SQLx wird ohne `.pgpass`-Laden aufgebaut. Host, Port, Benutzer, Datenbank, Passwort und Transport werden ausdrücklich gesetzt; eine fremde `DATABASE_URL` oder `PGPASSWORD` aktiviert keinen Ersatzpfad. Nicht leere `PGOPTIONS` werden abgewiesen, damit nicht deklarierte Server-Startup-Optionen die Dienstkonfiguration umgehen. TCP/TLS-Provisionierung für einen anderen Reader ist kein Bestandteil von C1.

### HTTP, Readiness und Shutdown

| Route | Verhalten |
|---|---|
| `POST /v1/answer` | Bestehender typisierter API-Vertrag, Bearer-Auth, serverseitige Scopes/Budgets und persistente Conversation-Ownership. |
| `GET /healthz` | Liveness: statisches JSON, keine Datenbank-/Provider-/Secret-Details. |
| `GET /readyz` | Aktuelle, zeitlich begrenzte Prüfung von PgStore, festem Release-Manifest, Reader und nötigen DB-Rechten. HTTP 503 bei Fehlern oder Drain. |
| Andere Routen | Vorhandener 404-Fallback; insbesondere keine `/ask`, `/query` oder Legacy-Antwortstrecke. |

Readiness wird nicht durch einen echten Modellaufruf geprüft. Provider-Config, Secret, Endpoint und Preisobergrenzen werden lokal validiert; die Zulassung eines externen Providers bleibt eine Betreiberentscheidung. HTTP zum Provider ist ausschließlich auf Loopback zulässig; ansonsten gelten HTTPS und explizite Preisobergrenzen. Pro Readiness-Prüfung gilt ein Timeout; ein einzelner Semaphore-Platz begrenzt Probe-Arbeit, auch wenn ein bereits gestarteter Blocking-Read noch ausläuft.

SIGTERM und SIGINT werden schon vor den DB-Checks registriert. Im laufenden Dienst wird Drain markiert, der Listener geschlossen und laufenden Antworten Zeit zum Fertigstellen gegeben. Ein gemeinsames `shutdown_ms`-Budget begrenzt HTTP-Drain, Pool-Close und Tokio-Blocking-Worker. Das Beenden einer laufenden Provider-Antwort mit anschließendem erfolgreichen Prozess-Exit ist im echten Kindprozess geprüft. Unvollständiger Drain endet mit einem Fehlercode statt unbegrenztem Warten.

Logging verwendet nur explizite Lifecycle-Ereignisse, die numerische Listener-Adresse und statische Fehlercodes. Config-/Dependency-Fehler, Request-Bodies, Header, DSNs und Panic-Payloads werden nicht ausgegeben. Config-/Secret-Debug-Ausgaben sind redigiert. Der vorhandene native Secret-Exec-Wrapper bleibt unverändert; dessen reale Infisical-Anbindung wurde nicht mit Produktionscredentials ausgeführt.

## Config und Startkommando

Die vollständige nicht geheime Vorlage liegt unter `config/brain-serve.example.json`. Alle Hauptabschnitte sind Pflicht: `bind`, `postgres`, `release`, `provider`, `budgets`, `timeouts`, `retrieval`, `kernel`, `credentials`.

Die Vorlage ist **ein Loopback-Pilotbeispiel, keine Produktionsfreigabe**. Sie setzt einen bereits vorhandenen `pilot-r1` mit `pilot-knowledge-v1` und einen lokalen Fixture-Provider voraus. Vor einem anderen Einsatz sind reale, ausdrücklich freigegebene Pins, Modell/Endpoint, Preisobergrenzen, Scopes und Budgets einzutragen. Die Vorlage übernimmt die bestehenden Budgetwerte vollständig: vier Netzwerk-Runden, 12 000 Input-Tokens, 2 000 Output-Tokens und 50 000 Kosten-Mikroeinheiten; das Retrieval-Limit bleibt sechs.

Build aus dem Repository:

```sh
cd rust
cargo build --workspace --release --locked
```

Direkter Start in einer bereits durch den vertrauenswürdigen Secret-Launcher versorgten Umgebung:

```sh
/path/to/checkout/rust/target/release/brain-serve \
  --config /path/to/operator-config/brain-serve.json
```

Über den vorhandenen nativen Infisical-Wrapper:

```sh
/path/to/checkout/rust/target/release/deadlock-brain-secret-exec \
  --config /path/to/operator-config/infisical.json -- \
  /path/to/checkout/rust/target/release/brain-serve \
  --config /path/to/operator-config/brain-serve.json
```

Die Operator-Infisical-Konfiguration muss die in der Service-Config referenzierten Environment-Namen liefern. Secrets werden nicht als Argumentwerte übergeben. Es wurde kein Infisical-Secret gelesen oder neu angelegt.

### Runtimevariablen — ausschließlich Namen

| Name | Zweck / Pflicht |
|---|---|
| `BRAIN_SERVE_CONFIG` | Pfad zur nicht geheimen Service-Config, nur nötig, wenn `--config` nicht gesetzt ist. |
| `BRAIN_SERVE_PROVIDER_API_KEY` | Erforderliches Provider-Secret im Beispiel; tatsächlicher Name über `provider.api_key_env`. |
| `BRAIN_SERVE_API_TOKEN` | Erforderliches API-Client-Secret im Beispiel; pro Grant über `credentials[].token_env`. |
| `BRAIN_SERVE_PG_PASSWORD` | Nur bei ausdrücklich konfigurierter Passwortauthentifizierung; Name über `postgres.password_env`. |
| Weitere durch `credentials[].token_env` referenzierte Namen | Alle konfigurierten Client-Tokens sind erforderlich; kein anonymer Modus. |
| `INFISICAL_CONFIG_FILE` | Vorhandener Secret-Exec-/Infisical-Transport, entsprechend lokaler Betreiberkonfiguration. |
| `CREDENTIALS_DIRECTORY` | Wird bei Verwendung von `LoadCredential` durch systemd bereitgestellt; keine manuell eingecheckte Secret-Datei. |

Bind-Adresse, DB-Socket/Benutzer/Datenbank, Release, Knowledge-Version, Modell, Provider-URL, Budgets, Timeouts und Retrieval-/Cache-Konfiguration stehen in der JSON-Datei, nicht in konkurrierenden impliziten Environment-Defaults.

`service/systemd/brain-serve.service.example` ist ausschließlich eine **nicht installierte User-Unit-Vorlage**. Sie nutzt den vorhandenen `LoadCredential`-/Secret-Exec-Weg, SIGTERM und `TimeoutStopSec` oberhalb des Beispiel-Drain-Budgets. Die Beispielpfade sind vor jeder separat genehmigten Nutzung anzupassen. Es wurde kein `systemctl enable/start/restart` ausgeführt.

## Tatsächliche Verifikation

**C1 technisch erfüllt:** Das gebaute `brain-serve` startet als eigener Prozess; sowohl der synthetische Prozess-Pilot als auch der Dokumenten-Pilot sprechen es über den typisierten BrainClient an. Die vier geforderten Workspace-Kommandos sind im abschließenden Lauf sämtlich bestanden. Der fachliche Default-Pilot ist wegen der unveränderten C2/C3-Befunde weiterhin nicht vollständig grün.

Geprüfter Git-Stand: `1b9597f41ff2228d1f09841364db694866394f36` (Implementierung `9e2e7863d6d4b3f92da17fbc88ceb2d40516c4ad` plus ursprünglicher Handoff). Der anschließende Dokumentationscommit aktualisiert ausschließlich diesen Ergebnisbericht.

Alle Cargo-Kommandos liefen im `rust/`-Workspace mit Rust/Cargo 1.97.1, `SQLX_OFFLINE=true`, `CARGO_BUILD_JOBS=2` und lokal gecachten Dependencies. Die abschließenden Läufe verwenden eine bereinigte Umgebung ohne Produktions-DSN, einen isolierten Test-HOME und prozesslokal leere `RUSTC_WRAPPER` / `RUSTC_WORKSPACE_WRAPPER`. PostgreSQL 16.14 und `protoc` sind auf dem Prüfhost vorhanden. Cargo wurde aus `/home/nathanael/.cargo/bin` verwendet.

| Prüfung | Bestätigtes Ergebnis | Lokaler Nachweis unter `.core-test-logs/c1/` |
|---|---|---|
| `cargo fmt --all -- --check` | Exit 0 | `fmt-direct.log`, `fmt-direct.result.json` |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Exit 0 | `clippy-direct.log`, `clippy-direct.result.json` |
| `cargo test --workspace --locked` | Exit 0: 727 bestanden, 0 fehlgeschlagen, 68 regulär ignoriert | `workspace-test-direct.log`, `workspace-test-direct.result.json` |
| `cargo build --workspace --release --locked` | Exit 0, gesamter Release-Workspace gebaut | `release-direct.log`, `release-direct.result.json` |
| `cargo test -p brain-serve --locked` | 10 Config-/Secret-Tests und 6 Prozess-Tests bestanden; im Workspace-Lauf erneut geprüft | `brain-serve-tests.log`, Workspace-Log |
| `bash scripts/test_brain_serve.sh` | Exit 0, echter Prozess-/Postgres-/SCRAM-/BrainClient-E2E; abschließend wiederholt | `process-e2e-direct.log`, `process-e2e-direct.result.json` |

Die regulär ignorierten Tests werden nicht als bestanden gezählt. Der C1-Prozess-E2E und die drei Phasen des opt-in-Dokumenten-Piloten wurden zusätzlich explizit ausgeführt; deren Ergebnisse stehen separat in diesem Bericht.

Das Release-Binary `rust/target/release/brain-serve` wurde anschließend direkt geprüft: `--version` liefert `brain-serve 0.1.0` mit Exit 0; Start in leerer Umgebung ohne Config liefert Exit 1 und ausschließlich `service_failed/config_missing`, ohne Listener. SHA-256: `181cc46170815e54c6ec90b1bae71ef631d3a05c3104308991029bab81360050` (12 122 712 Bytes); Nachweis: `release-binary-proof.json`.

**Erstläufe bleiben dokumentiert:** Der erste parallele Workspace-Lauf scheiterte an 14 `WorkerTimeout`-Fehlern im unveränderten Replay-Decoder; der erste Dokumenten-Pilot scheiterte an einem Ingest-/Recovery-Timeout. Eine Wiederholung meldete einen unerwartet beendeten `sccache`-Server. Danach wurden ausschließlich die Compiler-Wrapper der Prüfprozesse deaktiviert. Weder globale Cargo-/Cache-/Produktionskonfiguration noch Test-/Runtime-Limits wurden geändert. Zwei spätere vollständige Workspace-Läufe bestanden; die genaue Ursache jedes ursprünglichen Timeouts wird daraus nicht als abschließend bewiesen ausgegeben.

Auch die GitHub-CI auf `1b9597f41ff2228d1f09841364db694866394f36` ist bestätigt erfolgreich:

- `Rust Core and Offline Audit Verification`, Run `36205212052`: `success`, einschließlich des neuen Prozess-E2E in der bestehenden Matrix.
- `Brain Serve C1 Verification`, Run `36205212264`: `success`, einschließlich exakt der vier geforderten Cargo-Kommandos und des echten Prozess-E2E.

Beide Workflows haben ausschließlich Prüfaufgaben, keine Merge-/Deploy-Schritte. Ein neuer CI-Lauf für den nachfolgenden reinen Dokumentationscommit ist von diesen bestätigten Ergebnissen zu unterscheiden.

Zusätzliche Nachweise:

- Zehn Config-/Secret-Unit-Tests und sechs automatische Prozess-Tests: fehlende/fehlerhafte Config, fehlende/ungültige/mehrdeutige Secrets, ungültige Pins/Grenzen, redigierte Fehlermeldungen, unerreichbare DB, kein Ambient-DSN-/Legacy-Fallback, SIGTERM während Startup.
- Zwei Tests der rückwärtskompatiblen LocalPgReader-Optionen: Unix-Socket-Grenze, Timeout-Validierung und Passwort-Redaction.
- `bash scripts/test_brain_serve.sh`: echter Service-Kindprozess, eigener wegwerfbarer Unix-Socket-Cluster, typisierter BrainClient, vollständige Loopback-Antwort, Health/Readiness, falsche/fehlende Releases, belegter Port, fehlende Auth, ausschließlich 404 auf Legacy-Pfaden, DB-Verlust/Wiederkehr, Release-Austausch, persistente Ownership nach Neustart, Drain einer laufenden Antwort, SIGTERM/SIGINT, echte SCRAM-Authentifizierung mit falschem/richtigem Passwort, eingeschränkte Rolle und Entzug erforderlicher Rechte. **Bestanden.** Cluster wurde danach gestoppt und entfernt.
- `bash -n scripts/test_brain_serve.sh scripts/run_local_pilot.sh`: bestanden. `git diff --check`: bestanden.

Logs verbleiben außerhalb der versionierten Dateien unter `.core-test-logs/c1/`. Sie enthalten keine echten Secretwerte. `process-e2e.log`, `brain-serve-tests.log`, die Workspace-Logs sowie einzelne `*.exit`-/`*.result.json`-Dateien dokumentieren die tatsächlichen Läufe. Fehlende Exit-Dateien sind kein Erfolgssignal. Die CI-Matrix `kernel-api-consumer` nimmt `brain-serve` auf und führt den echten Prozess-/Postgres-Test reproduzierbar zusätzlich aus; keine neuen Merge-/Deploy-Schritte.

## Dokumenten-Pilot über das Binary

Der bestehende Pilot-Test wurde von `brain-api/tests/local_pilot.rs` nach `brain-serve/tests/local_pilot.rs` verschoben. `scripts/run_local_pilot.sh` ruft dieses Package auf. Cargo stellt den tatsächlichen Executable-Pfad über `CARGO_BIN_EXE_brain-serve` bereit; das Test-Harness startet einen eigenen Prozess mit bereinigter Umgebung und prüft Readiness vor dem ersten BrainClient-Aufruf. Nur Ingest-/Release-Setup und der lokale Provider-Fixture-Server bleiben im Test. Kernel und API werden dort **nicht mehr selbst zusammengesetzt**.

Der bestehende Pfad mit PostgreSQL-Absturz/Recovery, ACL-Widerruf, Tombstone, privater Quelle und leerem Rebuild bleibt erhalten. Prozessmetriken des optionalen Lastlaufs lesen jetzt die PID des echten `brain-serve`, nicht die des Test-Harnesses. Service-Logs werden pro Variante als `brain-serve-*.log` abgelegt.

Für beide Läufe wurden jeweils temporäre Kopien der sieben vorhandenen Dateien aus `/home/nathanael/.local/share/deadlock-brain/pilot-20260925` verwendet. SHA-256-Vergleiche bestätigten nach jedem Versuch, dass der Ursprungsbestand unverändert blieb; die temporären Kopien wurden entfernt. Alle Provider-Aufrufe gingen ausschließlich an den lokalen Fixture-Server.

Der erste Versuch endete vor den fachlichen Fällen mit `PoolTimedOut` beim Ingest und anschließendem Scratch-Recovery-Timeout. Dieser fehlgeschlagene Erstlauf bleibt unter `.core-test-logs/c1/pilot/` dokumentiert und wurde nicht als fachliches Ergebnis gewertet.

Die abschließende Wiederholung unter `.core-test-logs/c1/pilot-direct/` erreichte sämtliche Phasen **über das tatsächlich gestartete Binary**:

| Phase | Ergebnis |
|---|---|
| Ingest / Release-Publikation | Exit 0 |
| Default nach erzwungenem Scratch-DB-Neustart | **10/15** fachliche Fälle bestanden; Test-Exit 101 |
| Explizite Diagnosevariante | **15/15** fachliche Fälle bestanden; Exit 0 |
| Leerer Rebuild | Exit 0 |

Das Gesamtskript endet korrekt mit **Exit 1**, weil der Default-Teil weiterhin fünf Fehler meldet. Die Fälle `public_question`, `exact_number`, `alias_en`, `alias_de_lowercase` und `provider_error` liefern im Standardlauf `budget_exceeded`, jeweils ohne Provider-Egress. Diese vorhandenen Budget-/Fehlerklassifikationsbefunde wurden nicht durch C1 verändert oder als Erfolg umgedeutet.

Die Diagnosevariante nutzt ausschließlich die schon vorhandenen expliziten Pilot-Schalter für einen Treffer und 200 000 Input-Tokens. Die Config-Vorlage und der Standardlauf behalten sechs Treffer und das bestehende Budget von 12 000 Input-Tokens bei. **Die Diagnosewerte sind weder neue Defaults noch eine Produktionsfreigabe.**

Nachweise: `summary.tsv`, `after_restart_default.json`, `after_restart_diagnostic.json`, `brain-serve-default.log`, `brain-serve-diagnostic.log`, Ingest-/Rebuild-Berichte und `pilot-direct.result.json`. Der Prozess-Pilot wurde danach ebenfalls erneut mit Exit 0 ausgeführt. Der Dokumenten-Scratch-Cluster ist abschließend nachweislich gestoppt (`pg_ctl status`: Exit 3); der separate Prozess-E2E hat seinen eigenen temporären Cluster gestoppt und entfernt. Der optionale Lastlauf wurde nicht aktiviert.

Reproduktionskommando, ausschließlich mit separat bereitgestellter Arbeitskopie der genehmigten Pilot-Dokumente außerhalb des Repositorys:

```sh
BRAIN_PILOT_ROOT=/path/to/disposable-approved-pilot-copy \
BRAIN_PILOT_REPORT=/path/to/local-pilot-report \
BRAIN_PILOT_DIAGNOSTIC_MAX_INPUT_TOKENS=200000 \
BRAIN_PILOT_DIAGNOSTIC_LIMIT=1 \
SQLX_OFFLINE=true \
bash scripts/run_local_pilot.sh
```

Die Datenkopie benötigt `public/` und `internal/` gemäß dem bestehenden Pilotvertrag. Der Test parkt eine Datei kurzzeitig für die Löschprüfung; daher ausdrücklich eine Arbeitskopie, nicht den Ursprungsbestand verwenden. `BRAIN_TEST_CARGO` und `BRAIN_TEST_PG_BIN` erlauben optional die Wahl der lokalen Toolchain-/PostgreSQL-Binaries. Der Dienst selbst besitzt keinen Test-/Fixture-Modus: auch im Pilot werden PgStore, Reader, Provider-Port und API real verwendet.

## Geänderte Dateien

```text
A	.github/workflows/brain-serve-c1.yml
M	.github/workflows/rust-core-verification.yml
A	config/brain-serve.example.json
M	rust/Cargo.lock
M	rust/Cargo.toml
A	rust/crates/brain-serve/Cargo.toml
A	rust/crates/brain-serve/src/config.rs
A	rust/crates/brain-serve/src/config/tests.rs
A	rust/crates/brain-serve/src/health.rs
A	rust/crates/brain-serve/src/lib.rs
A	rust/crates/brain-serve/src/main.rs
A	rust/crates/brain-serve/src/secrets.rs
A	rust/crates/brain-serve/src/service.rs
A	rust/crates/brain-serve/tests/common/mod.rs
R091	rust/crates/brain-api/tests/local_pilot.rs	rust/crates/brain-serve/tests/local_pilot.rs
A	rust/crates/brain-serve/tests/process.rs
A	rust/crates/brain-serve/tests/process_e2e.rs
M	rust/crates/brain-storage/src/local_pg_reader.rs
A	rust/crates/brain-storage/tests/local_reader_config.rs
M	scripts/run_local_pilot.sh
A	scripts/test_brain_serve.sh
A	service/systemd/brain-serve.service.example
A	architecture/migration/handoffs/C1_BRAIN_SERVE.md
```

## Was Claude lokal noch prüfen muss

1. Den freizugebenden PR-SHA und dessen CI-Ergebnisse prüfen. Die hier bestätigten lokalen Prüfungen und grünen CI-Läufe beziehen sich auf den oben genannten Stand; die bekannte Default-Pilot-Lücke bleibt ein separater C2/C3-Auftrag.
2. Operator-Konfiguration/Infisical-Mapping für die tatsächlich freigegebenen Provider-/Client-Secret-Namen prüfen, ohne Werte ins Terminal, Journal oder Git zu schreiben. Hier wurde ausschließlich mit synthetischen Credentials gearbeitet.
3. Reale Peer-/SCRAM-Rolle, Socketpfad, SQL-Rechte und das bereits bereitgestellte Schema prüfen. Release-ID, Knowledge-Version, vollständige Pins und aktuelle ACL-Heads müssen zum vorgesehenen Datenstand passen. Keine automatische Migration durch diese Unit erwarten.
4. Erst nach ausdrücklicher Betreiberfreigabe Modell/Endpoint/Preisobergrenzen und reale Provider-Erreichbarkeit in einer isolierten Umgebung prüfen. `/readyz` behauptet keine Modell-Verfügbarkeit; es löst keinen kostenpflichtigen Provider-Request aus.
5. Die Beispiel-Unit-Pfade, User-Unit-Eignung, Infisical-Credential-Zugang und Dateisystem-Härtung für den lokalen Host prüfen. `TimeoutStopSec` muss über `shutdown_ms` liegen. Eine Installation/Aktivierung ist ein eigener, hier **nicht** genehmigter Schritt.
6. Nach C2/C3/C6 den fachlichen Default-Pilot und gegebenenfalls den Lastlauf erneut ausführen. C1 ist kein Ersatz für diese Aufträge und keine Production-Cutover-Freigabe.

**PRODUCTION_CUTOVER_READY: unverändert NEIN.** Der C1-Service-Durchstich ist vom fachlichen Gesamt-Pilot und den übrigen Integrations-Gates zu unterscheiden.
