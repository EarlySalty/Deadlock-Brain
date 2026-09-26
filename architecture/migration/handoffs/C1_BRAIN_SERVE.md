# C1 — Startbarer Rust-Core `brain-serve`

Stand: 26.09.2026. Ausschließlich C1 aus `architecture/migration/INTEGRATION_REVIEW.md`.

## Herkunft und Ergebnis

- Basis: `origin/migration/rust-integration`, Commit `087c522deda58ecf4bd6843167f51c54f681e944`.
- Branch: `codex/fix-c1-brain-serve`.
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

Alle Cargo-Kommandos liefen im `rust/`-Workspace mit Rust/Cargo 1.97.1, `SQLX_OFFLINE=true`, `CARGO_BUILD_JOBS=2` und lokal gecachten Dependencies. Die Workspace-Prüfungen verwenden eine bereinigte Umgebung ohne Produktions-DSN und einen isolierten Test-HOME. PostgreSQL 16 und `protoc` sind auf dem Prüfhost vorhanden.

| Prüfung | Tatsächlich beobachtetes Ergebnis |
|---|---|
| `cargo fmt --all -- --check` | Exit 0; erneuter direkter Lauf nach der Vorlagenangleichung ebenfalls Exit 0. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Erster Gesamtlauf Exit 0. Wiederholung des finalen Stands separat protokolliert. |
| `cargo test --workspace --locked` | Erster Gesamtlauf Exit 101: 14 `WorkerTimeout`-Fehler in unverändertem `dbrain-replay/tests/decoder.rs`; kein grünes Workspace-Ergebnis daraus ableiten. |
| `cargo build --workspace --release --locked` | Gestartet; beim Verfassen dieses Zwischenstands noch kein bestätigter Exit-Status. Nicht als bestanden gewertet. |
| `cargo test -p brain-serve --locked` | 10 Config-/Secret-Tests und 6 Prozess-Tests bestanden; die opt-in-Piloten wurden dabei erwartungsgemäß nicht ausgeführt. |
| `bash scripts/test_brain_serve.sh` | Exit 0: echter Prozess-/Postgres-/SCRAM-/BrainClient-E2E bestanden. |

**Verifikationsvorbehalt:** Die Funktionsprüfung des echten Dienstprozesses ist belegt. Ein vollständig grüner Workspace-/Release-/Dokumenten-Pilot-Nachweis liegt in diesem Zwischenstand noch nicht vor. Der erste parallele Lauf zeigte Replay-Worker-Timeouts sowie einen Timeout des Scratch-DB-Ingests und des anschließenden Recoverys. Später protokollierte die Wiederholung einen unerwartet beendeten `sccache`-Server. Deshalb werden nachfolgende lokale Prüfungen mit prozesslokal leeren `RUSTC_WRAPPER` und `RUSTC_WORKSPACE_WRAPPER` durchgeführt; keine globale Cargo-/Cache-/Produktionskonfiguration und keine Test-/Runtime-Limits wurden geändert. Die Ursache der Replay-Timeouts ist damit nicht als abschließend bewiesen auszugeben.

Die zusätzliche `.github/workflows/brain-serve-c1.yml` prüft exakt die vier geforderten Cargo-Kommandos und den echten Prozess-E2E auf dem PR-Stand, mit read-only GitHub-Rechten und ohne Deployment. Ein ausgelöster CI-Lauf ist erst nach einem bestätigten Ergebnis als bestanden zu bewerten.


Zusätzliche Nachweise:

- Zehn Config-/Secret-Unit-Tests und sechs automatische Prozess-Tests: fehlende/fehlerhafte Config, fehlende/ungültige/mehrdeutige Secrets, ungültige Pins/Grenzen, redigierte Fehlermeldungen, unerreichbare DB, kein Ambient-DSN-/Legacy-Fallback, SIGTERM während Startup.
- Zwei Tests der rückwärtskompatiblen LocalPgReader-Optionen: Unix-Socket-Grenze, Timeout-Validierung und Passwort-Redaction.
- `bash scripts/test_brain_serve.sh`: echter Service-Kindprozess, eigener wegwerfbarer Unix-Socket-Cluster, typisierter BrainClient, vollständige Loopback-Antwort, Health/Readiness, falsche/fehlende Releases, belegter Port, fehlende Auth, ausschließlich 404 auf Legacy-Pfaden, DB-Verlust/Wiederkehr, Release-Austausch, persistente Ownership nach Neustart, Drain einer laufenden Antwort, SIGTERM/SIGINT, echte SCRAM-Authentifizierung mit falschem/richtigem Passwort, eingeschränkte Rolle und Entzug erforderlicher Rechte. **Bestanden.** Cluster wurde danach gestoppt und entfernt.
- `bash -n scripts/test_brain_serve.sh scripts/run_local_pilot.sh`: bestanden. `git diff --check`: bestanden.

Logs verbleiben außerhalb der versionierten Dateien unter `.core-test-logs/c1/`. Sie enthalten keine echten Secretwerte. `process-e2e.log`, `brain-serve-tests.log`, die Workspace-Logs sowie einzelne `*.exit`-/`*.result.json`-Dateien dokumentieren die tatsächlichen Läufe. Fehlende Exit-Dateien sind kein Erfolgssignal. Die CI-Matrix `kernel-api-consumer` nimmt `brain-serve` auf und führt den echten Prozess-/Postgres-Test reproduzierbar zusätzlich aus; keine neuen Merge-/Deploy-Schritte.

## Dokumenten-Pilot über das Binary

Der bestehende Pilot-Test wurde von `brain-api/tests/local_pilot.rs` nach `brain-serve/tests/local_pilot.rs` verschoben. `scripts/run_local_pilot.sh` ruft dieses Package auf. Cargo stellt den tatsächlichen Executable-Pfad über `CARGO_BIN_EXE_brain-serve` bereit; das Test-Harness startet einen eigenen Prozess mit bereinigter Umgebung und prüft Readiness vor dem ersten BrainClient-Aufruf. Nur Ingest-/Release-Setup und der lokale Provider-Fixture-Server bleiben im Test. Kernel und API werden dort **nicht mehr selbst zusammengesetzt**.

Der bestehende Pfad mit PostgreSQL-Absturz/Recovery, ACL-Widerruf, Tombstone, privater Quelle und leerem Rebuild bleibt erhalten. Prozessmetriken des optionalen Lastlaufs lesen jetzt die PID des echten `brain-serve`, nicht die des Test-Harnesses. Service-Logs werden pro Variante als `brain-serve-*.log` abgelegt.

Der erste Dokumenten-Pilot wurde mit einer temporären Kopie der sieben vorhandenen Dateien aus `/home/nathanael/.local/share/deadlock-brain/pilot-20260925` gestartet. SHA-256-Vergleiche bestätigten nach diesem Versuch, dass der Ursprungsbestand unverändert blieb; die temporäre Kopie wurde entfernt. Es wurde ausschließlich der lokale Provider-Fixture-Server vorgesehen, kein externer Modell-Request.

Dieser erste Versuch **bestand nicht**: `pilot_phase_ingest` endete mit `PoolTimedOut` (Exit 101), anschließend überschritt das Scratch-Postgres-Recovery den vorhandenen `pg_ctl`-Timeout. Das Gesamtskript lieferte Exit 1. Die 15 fachlichen Fälle wurden in diesem Versuch nicht erreicht; insbesondere werden weder 10/15 noch 15/15 als Ergebnis dieses C1-Laufs behauptet. Die früheren Ergebnisse aus dem Integrationsreview sind kein Ersatz für einen neuen Messlauf.

Der eigenständige synthetische Prozess-Pilot `scripts/test_brain_serve.sh` hat den geforderten technischen Durchstich bereits nachgewiesen: tatsächlich gestartetes `brain-serve`, echte Scratch-Datenbank und typisierter BrainClient. Der vollständige Dokumenten-Pilot bleibt zusätzlich zu wiederholen. Standard-Budgets und Retrieval-Limit wurden nicht als Ausweichlösung erhöht.


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

1. Zunächst die noch offenen Workspace-/Release-Prüfungen sowie den vollständigen Dokumenten-Pilot auf einer stabilen Prüfumgebung abschließen und die exakten PR-SHA-Ergebnisse dokumentieren. Die aktuell belegten Funktionstests ersetzen diese offenen Nachweise nicht.
2. Operator-Konfiguration/Infisical-Mapping für die tatsächlich freigegebenen Provider-/Client-Secret-Namen prüfen, ohne Werte ins Terminal, Journal oder Git zu schreiben. Hier wurde ausschließlich mit synthetischen Credentials gearbeitet.
3. Reale Peer-/SCRAM-Rolle, Socketpfad, SQL-Rechte und das bereits bereitgestellte Schema prüfen. Release-ID, Knowledge-Version, vollständige Pins und aktuelle ACL-Heads müssen zum vorgesehenen Datenstand passen. Keine automatische Migration durch diese Unit erwarten.
4. Erst nach ausdrücklicher Betreiberfreigabe Modell/Endpoint/Preisobergrenzen und reale Provider-Erreichbarkeit in einer isolierten Umgebung prüfen. `/readyz` behauptet keine Modell-Verfügbarkeit; es löst keinen kostenpflichtigen Provider-Request aus.
5. Die Beispiel-Unit-Pfade, User-Unit-Eignung, Infisical-Credential-Zugang und Dateisystem-Härtung für den lokalen Host prüfen. `TimeoutStopSec` muss über `shutdown_ms` liegen. Eine Installation/Aktivierung ist ein eigener, hier **nicht** genehmigter Schritt.
6. Nach C2/C3/C6 den fachlichen Default-Pilot und gegebenenfalls den Lastlauf erneut ausführen. C1 ist kein Ersatz für diese Aufträge und keine Production-Cutover-Freigabe.

**PRODUCTION_CUTOVER_READY: unverändert NEIN.** Der C1-Service-Durchstich ist vom fachlichen Gesamt-Pilot und den übrigen Integrations-Gates zu unterscheiden.
