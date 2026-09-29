status: erledigt
Datum: 2026-09-29

# R-AC Runde 4: technische Codeabnahme A+C

**Fertig: J. Fix nötig: N. Technische Codeabnahme A+C: GO.**

Der bekannte A4-Restbefund ist mit unveränderter unabhängiger HTTP-Gegenprobe geschlossen. Im geprüften Delta wurde kein neuer blockierender Defekt nachgewiesen. Dieses Urteil ist **keine G5-Freigabe, keine finale Workspace-Abnahme und keine Produktionsfreigabe**. Der vollständige Workspace-/Release-/PostgreSQL-/Lastlauf folgt auf dem tatsächlichen Integrationshead.

## Prüfstand

- Auftrag `R-AC-R4-BRIEFING.md`, Intent `562a877b-0939-440a-964d-1145d9e9431a`, unabhängiger Review-Thread `52c34332`.
- Abgegebener gemeinsamer A+C-Head: `6ddeb6c068d3997375f9e60a1bf2272272319e7f`, PR #59. A4-Abgabe `cd5b0aa`, Produktfix `a5504d3`, Testnachtrag `bc46bee`; C1-Nachtrag `52ef6c5`.
- Eigener vorheriger Berichtsstand `9d07222`; autorisierter lokaler Reviewmerge und tatsächlicher Prüfstand `d8889e4a7201e251c0802955e2c6cb7e0edca655`.
- Eigener Baum `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`. Keine Produktänderungen, Unterthreads oder fremden Worktree-Änderungen.
- Neue Arbeitsbelege: `/tmp/brain-rac-r4.NzS4D2/`. Der unveränderte Gegenprobentreiber stammt aus `/tmp/brain-rac-r3.OW58Dy/analytics_probe.rs`.

Der von der Hauptsession berichtete C60-Merge `4c962b83cc3e17c5525e91f90da8ed3ca718d01f` ist keine eigene Integrationsaktion dieses Reviewers. Es erfolgte kein Merge nach migration/main und kein Produktionszugriff.

## Bekannte Mängelliste

| Befund | Urteil am gemeinsamen Prüfstand |
| --- | --- |
| A1/A2 | Geschlossenes Urteil aus dem A12-Nachtrag in R3 bleibt bestehen. Match-Projektion und Driftvalidator wurden im aktuellen Delta nicht verändert. Keine erneute vollständige Match-/Store-Abnahme behauptet. |
| A3 | Geschlossen. Die unveränderte unabhängige Probe bestätigt erneut normale Meta-/Population-Antworten, effektives Zeitfenster, Rechte-/Cachegrenzen und sichere Patch-Ablehnung ohne Provider-Fallback. |
| A4 | **Jetzt geschlossen.** Snapshot-Dauer verbraucht dasselbe absolute Budget; nach bereits gesendetem HTTP 504 startet keine Analytics-Anfrage mehr. |
| C1 | Geschlossen. Nachträge statisch geprüft und der sichere Sechs-Fall-Runner auf dem gemeinsamen Reviewstand selbst ausgeführt. Die SCRAM-Nachweisgrenze bleibt ausdrücklich erhalten. |

## 1. A4: dieselbe reale HTTP-Gegenprobe ist jetzt grün

Der Rust-Treiber wurde nicht geändert. SHA256:

```text
435409afb9e3dec16bc377398f04ede81c8bd1181cd93a72c77c21da9bbcd771  analytics_probe.rs
```

Der Treiber bindet die aktuelle, unveränderte Produktdatei `brain-serve/src/analytics.rs` per `include!` ein und wird gegen die aktuellen Cargo-Artefakte gebaut. Er verwendet `brain_api::router`, `ApiService`, `CachedKernel`, `Kernel`, `ReleaseRetriever` und den echten Analytics-HTTP-Client. Request und Quellaufruf laufen über lokale TCP-Verbindungen. Der Snapshot-Port ist kontrolliert verzögert; die Probe ist kein PostgreSQL-Latenzbenchmark. Der Provider verweigert Aufrufe und zählt sie.

Gleiche Parameter und gleiche Erwartung wie in R3: Request-Budget 200 ms, Analytics-Einzelbudget 100 ms, Snapshot-Verzögerung 600 ms; nach Ablauf keine neue ausgehende HTTP-Anfrage.

```json
{"case":"expired_snapshot_must_not_start_new_http","detail":{"budget_ms":200,"client_response":"HTTP/1.1 504 Gateway Timeout","client_response_ms":202,"http_arrival_ms":[],"per_http_timeout_ms":100,"snapshot_delay_ms":600,"worker_completion_ms":602},"passed":true}
```

Vergleich: R3 hatte trotz HTTP 504 nach 202 ms einen neuen Analytics-Request nach 601 ms und Worker-Ende nach 697 ms. R4 hat nach derselben Clientantwort **keinen** Analytics-Request; der Worker endet nach Rückkehr des Snapshots nach 602 ms. Das bedeutet nicht, dass ein laufender synchroner Snapshot gewaltsam abgebrochen wird. Geschlossen ist die konkrete Budgeterneuerung mit anschließendem neuen Netzwerkaufruf.

Der gesamte unveränderte Treiber erfüllt **15 von 15 Laufzeitprüfungen, Exit 0**. Dazu gehören positive Meta-/Population-Antworten, Item-Matchzählung statt Kaufzählung, wirksame Zeitgrenzen, Scope-/Principal-/Release-Cachegrenzen, Patch-Ablehnung, fehlender Provider-Fallback und Kapazitätsgrenzen.

Weitere Deadline-Ergebnisse: zwei Quellen mit gemeinsamem 600-ms-Budget enden nach 600 ms ohne erfolgreiche verspätete Antwort. Ein synthetischer 503 bei 100-ms-Gesamtbudget löst einen Versuch aus, keinen zusätzlichen Retry mit neuer Zeitspanne. Vier belegte Slots im normalen Antwortpfad führen ohne neue Netzwerkanfrage zu `Unavailable`.

## 2. Systematische Prüfung des gemeinsamen Deadlinewegs

Graphify wurde vor der gezielten Fundstellenprüfung befragt; für die neuen Deadline-Symbole gab es keine passenden Knoten. Anschließend wurden die bekannten drei Produktpfade und ihre Aufrufer geprüft.

1. **Normaler Antwortpfad:** `brain-serve/src/analytics.rs:253` setzt die absolute Deadline vor dem Snapshot. Prüfungen nach Snapshot (`:267`), Slot-Erwerb (`:285`), Meta (`:306`) und Population (`:324`) verhindern nachfolgende Arbeit mit verbrauchtem Budget. Meta und Population erhalten denselben Zeitpunkt (`:304`, `:313`). Es gibt keine Budgeterhöhung.
2. **Beobachtungsroute:** `analytics.rs:488` setzt die Deadline vor Autorisierung und Slot-Warten. Prüfungen nach Autorisierung und Slot-Erwerb sowie vor erfolgreicher Antwort verhindern einen nach Ablauf als erfolgreich ausgegebenen Lookup. `timeout_at` und der Blocking-Worker benutzen denselben Zeitpunkt (`:508`, `:532`, `:534`). Beide bestehenden Vier-Slot-Tests wurden selbst ausgeführt und bestanden.
3. **Analytics-Client:** `dbrain-sources/src/analytics_runtime.rs:186` bis `:215` reicht die absolute Deadline über `get_bounded_until` weiter und verwirft eine nach Ablauf validierte Antwort. Der URL-/Projektionsvertrag wurde dabei nicht verändert.
4. **Gemeinsamer HTTP-Kern:** `deadlock-brain-core/src/http/bounded.rs:85` ergänzt den absoluten Einstieg; der vorhandene `get_bounded` bleibt über denselben internen Pfad ohne zusätzliche absolute Grenze erhalten. `remaining_budget` begrenzt auf das Minimum aus relativem Gesamtbudget und absoluter Restzeit. Vor Senden und jedem Versuch wird erneut geprüft; `can_wait` berücksichtigt dieselbe Restzeit für Verbindungsfehler und 429/5xx (`:140`, `:150`, `:163`, `:230`, `:239` bis `:260`).

Die neun Bounded-HTTP-Tests liefen selbst erfolgreich: bisherige Byte-/Header-/Kompressionsgrenzen, Retry/Rate-Limit, Redirect-/Clientfehlerbehandlung, ungültige Budgets und Timeout sowie die neuen Fälle „abgelaufene absolute Deadline verbindet nicht“ und „absolute Deadline verhindert Retry nach erster Antwort“. Der zweite neue Fall überschreitet die absolute Grenze, während das relative HTTP-Gesamtbudget noch nicht verbraucht ist; er isoliert damit den neuen Schutz.

## 3. C1-Nachträge und A+C-Wechselwirkung

Die tatsächlichen Nachträge gegenüber dem bisherigen Reviewstand wurden geprüft: `scripts/test_brain_serve.sh`, `brain-serve/tests/process.rs` und `process_e2e.rs`.

- Die Alias-Konfliktprüfung liegt jetzt vor dem Widerruf. Zwei positive normale Fragen belegen Warden 770 und Abrams 650 mit je einer Quelle; danach liefert Guardian bei Retrieval-Limit 1 `InsufficientEvidence`. Die späteren Widerrufsprüfungen bleiben vorhanden.
- Der neue SCRAM-Fall verwendet ein zur Laufzeit erzeugtes synthetisches Passwort im privaten Scratch-Cluster, nicht im Environment. Erfolgreiche SQLx-Anmeldung und Ablehnung des falschen Werts mit SQLSTATE `28P01` werden geprüft. Keine Produktionszugangsdaten.
- Der gemeinsame Runner aktiviert sechs gezielte Fälle: zwei Legacy-Scratchfälle, Serve-Prozess-E2E, Scratch-SCRAM und zwei Prozessprüfungen zu Konfiguration/Redaktion. Eigener Lauf: **6 passed, 0 failed, 0 ignored, 26 filtered, Exit 0**.

### Vorläufige Messung auf dem Reviewstand

| Worker | Requests | Answered | Clientfehler | Dauer | Reader-Verbindungen |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 8 | 600 | 600 | 0 | 1852 ms | 4 |
| 16 | 600 | 600 | 0 | 1272 ms | 4 |
| 32 | 600 | 600 | 0 | 855 ms | 4 |

Poolmaximum und Peak 4, fünf erzeugte Verbindungen einschließlich Recovery, 9115 wiederverwendete Checkouts. `wait_count=5084`, `wait_timeout_count=1`, `wait_max_micros=150068`. Der einzelne Timeout gehört zur absichtlichen Sättigungsprüfung, nicht zu einem fehlgeschlagenen Lastrequest. Pool-Wartebudget unverändert 150 ms. Kein echter Provider und keine reale Matchabfrage.

Diese Werte belegen den aktuellen kombinierten Reviewstand. Sie sind **nicht** die finale Workspace-/Integrationslastmessung und ersetzen diese nicht.

## 4. Eigene Befehle und Ergebnisse

### Build und Library-Suites

```sh
env -i USER=nathanael PATH=/home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin CARGO_HOME=/home/nathanael/.cargo CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target /home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo build --manifest-path /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/Cargo.toml --locked --offline -p brain-kernel -p brain-serve -p dbrain-sources -p deadlock-brain-core --tests --message-format=json > /tmp/brain-rac-r4.NzS4D2/build.jsonl 2> /tmp/brain-rac-r4.NzS4D2/build.log
```

Exit 0. Aktuelle Testprogramme und Bibliotheken wurden anhand der Cargo-Artefaktliste zugeordnet, nicht anhand ihres Dateialters. Anschließend liefen die folgenden vier Programme in Reihenfolge als `&&`-Kette, Exit 0:

```sh
env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/brain_kernel-bbbcb7f606e79142 > /tmp/brain-rac-r4.NzS4D2/kernel.log 2>&1
env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/brain_serve-4d727e3e69d92ab4 > /tmp/brain-rac-r4.NzS4D2/serve.log 2>&1
env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/dbrain_sources-f74331c8c60e1171 > /tmp/brain-rac-r4.NzS4D2/sources.log 2>&1
env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/deadlock_brain_core-613f3bbd1c5689a1 > /tmp/brain-rac-r4.NzS4D2/core.log 2>&1
```

| Library-Suite | Passed | Failed | Ignored |
| --- | ---: | ---: | ---: |
| brain-kernel | 15 | 0 | 0 |
| brain-serve | 17 | 0 | 0 |
| dbrain-sources | 103 | 0 | 5 |
| deadlock-brain-core | 26 | 0 | 0 |
| Summe | 161 | 0 | 5 |

Die fünf ignorierten Source-DB-Tests wurden nicht aktiviert. Kein vollständiger Workspace-Testlauf oder Lauf sämtlicher Integrationstests wird behauptet. Die Autorenangabe 201/12 ist nicht als eigene Laufzahl übernommen.

### Unveränderte unabhängige Gegenprobe

```sh
/home/nathanael/.cargo/bin/rustc +stable --edition=2021 /tmp/brain-rac-r3.OW58Dy/analytics_probe.rs -L dependency=rust/target/debug/deps --extern brain_serve=rust/target/debug/deps/libbrain_serve-2b2494871e10569a.rlib --extern brain_contracts=rust/target/debug/deps/libbrain_contracts-c2a3d5430dd3a1d0.rlib --extern brain_kernel=rust/target/debug/deps/libbrain_kernel-fdc474d34bc23084.rlib --extern brain_api=rust/target/debug/deps/libbrain_api-a996f628c6e11efa.rlib --extern brain_policy=rust/target/debug/deps/libbrain_policy-766ebe4671b051c6.rlib --extern dbrain_sources=rust/target/debug/deps/libdbrain_sources-cb96410a993a4f0f.rlib --extern dbrain_reasoner=rust/target/debug/deps/libdbrain_reasoner-310d38df49664fb2.rlib --extern dbrain_retrieval=rust/target/debug/deps/libdbrain_retrieval-1c081d4c1c8831c2.rlib --extern axum=rust/target/debug/deps/libaxum-ecf11048589fc2fa.rlib --extern tokio=rust/target/debug/deps/libtokio-167092b10e6f5621.rlib --extern tempfile=rust/target/debug/deps/libtempfile-6fb3e2d82ec0a5bb.rlib --extern sha2=rust/target/debug/deps/libsha2-a4a9af9010090e4c.rlib --extern serde_json=rust/target/debug/deps/libserde_json-fe8c7a93cbe9f687.rlib -o /tmp/brain-rac-r4.NzS4D2/analytics-probe > /tmp/brain-rac-r4.NzS4D2/probe-build.log 2>&1

env -i PATH=/usr/bin:/bin /tmp/brain-rac-r4.NzS4D2/analytics-probe > /tmp/brain-rac-r4.NzS4D2/probe.log 2>&1
```

Compile und Lauf jeweils Exit 0. 15 Laufzeitprüfungen erfüllt, 0 fehlgeschlagen. Kontrollierte I/O-Zeitmessungen außerhalb der eingecheckten Unit-Suite; Erwartungen und 200-/600-ms-Gegenfall sind unverändert gegenüber R3.

### Sicherer Prozessrunner

```sh
/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/scripts/test_brain_serve.sh > /tmp/brain-rac-r4.NzS4D2/process.log 2>&1
```

Exit 0, 6 passed, 0 failed, 0 ignored, 26 filtered. Der private PostgreSQL-Cluster wurde vom Runner beendet. Gesamter eigener R4-Testnachweis: **167 passed, 0 failed, 5 ignored**, dazu getrennt **15 erfüllte Laufzeitgegenproben**. Keine neue Baseline erhoben; der konkrete A4-Rot-/Grünvergleich stammt aus derselben unveränderten Gegenprobe an den festgehaltenen R3-/R4-Ständen.

SHA256 der neuen Ergebnisdateien:

```text
4f72e9915592d7fd2cca91fd7b1c13ee2fec6c1f751e962294b8b30625f5acd9  probe.log
4ad89d4c3fefcf490dde09a8a17037ee104d255fb7fca6378c7656416cd07e46  process.log
```

## Verbleibende Nachweisgrenzen und Übergabe

Keine offenen blockierenden Befunde aus der bekannten A+C-Mängelliste. Zwei bereits dokumentierte nicht blockierende Grenzen bleiben sichtbar:

- Ein Retrieval-Fehler transportiert weiterhin keinen `Usage`-Wert. Fehlgeschlagene HTTP-Versuche können daher mit null Netzwerk-Runden erscheinen. Der geprüfte Pfad reserviert vorab sein Rundenbudget und startet danach keinen Provider-Fallback; die neue Deadline wird dadurch nicht verlängert.
- SCRAM ist als SQLx-Scratch-Anmeldung geprüft, nicht als `brain-serve`-Passwortstart. Der frühere ENV-Passwortpfad bleibt außerhalb des erlaubten Nachweises. Der tatsächliche Serve-Prozess wurde im privaten Peer-Harness geprüft.

A+C kann aus Sicht dieses unabhängigen Codereviews in den vorgesehenen Integrationsschritt gehen. Die Hauptsession führt den lokalen Merge-Gate-Weg und anschließend die finale Workspace-/Release-/PostgreSQL-/Lastprüfung auf dem resultierenden Integrationshead durch. Dieser Reviewer führt weder Integrations-/Main-Merge noch Deploy aus.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft
TESTNACHWEIS[TW-1]: 167 passed, 5 ignored | Baseline: keine neue Gesamtbaseline; identische A4-Gegenprobe R3 rot, R4 grün; zusätzlich 15 erfüllte Laufzeitprüfungen
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: interner Reviewbericht im Taskordner
MERGEPROTOKOLL[MS-1]: 1 lokaler Reviewmerge einzeln | Anläufe: 1 | Gate: kein Main-/Integrationsmerge angefordert; unabhängiges A+C-Codeurteil GO
