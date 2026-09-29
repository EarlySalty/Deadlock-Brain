status: erledigt
Datum: 2026-09-29

# R-AC Runde 5: Gesamtgate-Nachträge

**Fertig: J. Fix nötig: N. Technische Codeabnahme A+C: GO.**

Die drei beauftragten Nachträge sind unabhängig geprüft. Kein blockierender Defekt im geprüften Delta nachgewiesen. Das Urteil ist **keine G5-Freigabe, keine finale Workspace-Abnahme und keine Produktionsfreigabe**. Der zusätzliche zweite Snapshotbefund wird hier eigenständig geschlossen; das R4-Urteil zum ersten Snapshot wird nicht als Beweis dafür verwendet.

## Prüfstand und Grenzen

- Auftrag `R-AC-R5-BRIEFING.md`, Intent `562a877b-0939-440a-964d-1145d9e9431a`, Review-Thread `52c34332`.
- Fester kombinierter Head: `72db816056fb0ed53810ab77ea4417dc0812e7ca`, PR #59.
- A34-Abgabe `628fc76c8392e89f805591aad6b7875f0f1e31bf`, Produktfix `7217246beccb045b04d75e5b8785b91bad36681c`; A12-Abgabe `fd89bd0b7d2891a87aa8ee1e82f9e392cbac16fe`, Produktfix `9a33f29498c84f7407e12b4adfbbfa76ec994c67`.
- Autorisierter lokaler Reviewmerge und tatsächlicher Prüfstand: `01a2bb3d21118e4f01e7de5acd052e1532aae37c`. `git diff --name-only 72db816056fb0ed53810ab77ea4417dc0812e7ca HEAD -- rust config scripts` ist leer.
- Eigener Branch `review/pre-g5-core-abnahme-20260929` im bestehenden Reviewworktree. Keine Produktänderungen, Unterthreads, fremden Worktree-Änderungen, Integrations-/Main-Merges oder Produktionszugriffe.

Neue Arbeitsbelege liegen unter `/tmp/brain-rac-r5.rIxFvm/`. Es wurden ausschließlich synthetische Daten und lokale HTTP-Verbindungen verwendet. Der NDJSON-Quelllocator ist nur ein Fixturewert; kein Replaydownload oder realer API-Aufruf. Die Konfigurationsproben laden Dateien, starten aber keinen Dienst und lesen keine Zugangsdaten.

## 1. Zweiter Snapshot: direkte innere Gegenprobe

Der neue unabhängige Rust-Treiber `final_guard_probe.rs` bindet die unveränderte aktuelle `brain-serve/src/analytics.rs` ein und verwendet die frisch gebauten Produktbibliotheken. Er setzt **keine injizierte Kerneluhr** ein. `Kernel::new` läuft mit seiner normalen monotonen Uhr. Der erste Snapshot ist schnell; ausschließlich der zweite `SnapshotReadPort::read_snapshot` schläft kontrolliert 600 ms. Die Quellantwort kommt von einem lokalen TCP-Server über den tatsächlichen Analytics-Client. Requestbudget 200 ms, HTTP-Einzelbudget 100 ms.

| Eigene Gegenprobe | Ergebnis |
| --- | --- |
| Direkter `Kernel<AnalyticsRetriever, NoProvider>` | Nach 687 ms `BudgetExceeded`, null Citations, genau zwei Snapshotlesungen, ein HTTP-Aufruf, null Provideraufrufe |
| Normaler `ApiService<CachedKernel<Kernel>>` | Nach 683 ms HTTP-Status 200 mit öffentlichem `budget_exceeded`, null Citations, zwei Snapshotlesungen, ein HTTP-Aufruf, null Provideraufrufe |
| Direkter schneller Folgeaufruf | `Answered` mit einer Citation und erneutem Quellaufruf |
| Service-Folgeaufruf nach Ablauf | `Answered`, erneuter Quellaufruf; das abgelaufene Ergebnis wurde nicht als Erfolg zwischengespeichert |
| Nächster Service-Aufruf | `Answered` aus dem gültigen Cache ohne weiteren Quellaufruf und ohne Provider |

Die normale Service-Gegenprobe ruft den tatsächlichen synchronen `handle_answer`-Pfad auf, nicht den äußeren HTTP-Timeout. Damit kann HTTP 504 den inneren Status nicht verdecken. Der gesonderte bestehende 15-Fall-Treiber prüft weiterhin den realen HTTP-Router.

Der synchron laufende Snapshot wird nicht zwangsweise nach 200 ms abgebrochen. Geprüft ist die Verwerfung nach Rückkehr: keine verspätete erfolgreiche innere Antwort und kein Erfolgscache aus diesem Ergebnis. Die kontrollierte I/O-Gegenprobe ist kein PostgreSQL-Latenzbenchmark und kein neuer eingecheckter Wall-Clock-Unit-Test.

### Gemeinsame Erfolgszweige, Cache und Flight

Graphify wurde zuerst befragt. Im Reviewworktree fehlt der lokale Graph; die globale Abfrage lieferte für diese neuen Symbole keine belastbare Zielstelle. Anschließend wurden der eingefrorene Diff und die bekannten betroffenen Dateien gezielt gelesen.

- `rust/crates/brain-kernel/src/execution.rs:62`: gemeinsamer `publish`-Guard prüft die ursprüngliche Deadline unmittelbar vor der Antwortkonstruktion. Analytics (`:188`), kanonische Domainantwort (`:231`), gewöhnlicher Fact (`:256`) und Providerantwort (`:363`) laufen darüber. Keine direkte erfolgreiche Rückgabe an diesen vier Stellen übrig.
- `execution.rs:143` und `:150`: erneute Prüfung nach der ersten Evidenzvalidierung schließt den zweiten Snapshotbefund. Vor Provideraufruf wird die Restzeit nach der Egressvalidierung neu berechnet (`:294`); nach dem Provider erfolgt weitere Validierung und Deadlineprüfung (`:349`, `:356`). Der Provider bekommt kein erneuertes Vollbudget.
- `rust/crates/brain-kernel/src/cache.rs:58`: Cachetreffer werden erneut validiert und nur innerhalb der Requestdeadline ausgegeben. Nur `Answered` mit Citations wird eingetragen (`:97`). Die neue reale Gegenprobe belegt Fehlerverwerfung, erneutes Laden und anschließenden positiven Cachetreffer.
- `rust/crates/brain-kernel/src/flight.rs:147`: Follower validieren geteilte Citations erneut; der abschließende Deadlineguard gilt für Leader und Follower (`:168`). Fehler werden geteilt, nicht dauerhaft behalten. Bestehende Tests für parallele Fehler, getrennte Domain-Schlüssel und Follower-Revalidierung liefen erfolgreich.

Der Autoren-Test `successful_fact_domain_and_provider_paths_reject_late_validation` wurde in der eigenen Kernel-Suite ausgeführt. Er ergänzt die reale Analytics-Gegenprobe für die übrigen drei Zweige. Der vorhandene injizierte-Uhr-Test für direkten Kernel, Service und HTTP lief ebenfalls; er wird nicht als eigene reale Verzögerungsprobe ausgegeben.

## 2. Wirksames Stundenfenster beim Configladen

`rust/crates/brain-serve/src/config.rs:308` erzeugt den tatsächlichen `AnalyticsLookupRequest`; `:318` verwendet dessen `validate`. Die Schema-, Einzelzeit- und Gesamtzeitgrenzen bleiben zusätzlich erhalten.

Der eigene Treiber schreibt fünf private temporäre JSON-Fixtures auf Basis der Beispielkonfiguration und ruft **`Config::load`** auf. Keine Serviceinitialisierung oder Datenbankverbindung. Erwartete Ablehnungen prüfen konkret `Error::ConfigInvalid("analytics")`:

| Angefragtes Fenster | Wirksames Fenster | Erwartung und Ergebnis |
| --- | --- | --- |
| [3601, 7199] | [7200, 3600] | abgelehnt |
| [3601, 7200] | [7200, 7200] | abgelehnt |
| [0, 3599] | [0, 0] | abgelehnt |
| [3600, 7200] | [3600, 7200] | angenommen |
| [3601, 10800] | [7200, 10800] | angenommen |

Damit sind nicht nur die Autorenfixture, sondern auch Gleichheit der wirksamen Grenzen und zwei positive Grenzfälle nachgewiesen. Der bestehende Config-Regressionstest lief zusätzlich in der Serve-Suite.

## 3. Physische NDJSON-Zeilen

`rust/crates/brain-feeds/src/deadlock_match.rs:657` zählt physische Zeilen vor dem Entfernen leerer Zeilen. `:684` hält den separaten logischen Datensatzordinal; `:702` verwendet ausschließlich die physische Zeilennummer für den Locator. Byte-/Zeilenlimits und Identitätsprüfung wurden nicht gelockert.

Eigene Fixture mit CRLF, führenden Leerzeilen, Tabs, Zwischen- und Schlussleerzeilen, synthetischem Match 456 und Account 123:

```text
Physische Datensätze: Zeilen 3 und 6
Locators: .../jobs/review/result.ndjson#L3 und #L6
Logische IDs: match/456/demo/player_state/000001 und /000002
Evidence-IDs: player_state:456:000001 und player_state:456:000002
```

Die Probe prüft alle vier Eigenschaften. Der bestehende Autorenfall mit LF und Zeilen 2/4 lief zusätzlich in der Feed-Suite. Es fand keine erneute Match-/Store-Gesamtabnahme statt; A1/A2 bleiben gemäß R3 geschlossen.

## Eigene Läufe und genaue Ergebnisse

Arbeitsverzeichnis für die folgenden Befehle: `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`.

### Build und Library-Suites

```sh
env -i USER=nathanael PATH=/home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin CARGO_HOME=/home/nathanael/.cargo CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target /home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo build --manifest-path /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/Cargo.toml --locked --offline -p brain-kernel -p brain-serve -p dbrain-sources -p deadlock-brain-core -p brain-feeds --tests --message-format=json > /tmp/brain-rac-r5.rIxFvm/build.jsonl 2> /tmp/brain-rac-r5.rIxFvm/build.log
```

Build Exit 0. Artefakte aus dieser Cargo-JSON-Liste identifiziert, nicht anhand ihres Alters. Die fünf folgenden Aufrufe liefen in dieser Reihenfolge als `&&`-Kette, Gesamtexit 0:

```sh
env -i PATH=/usr/bin:/bin rust/target/debug/deps/brain_kernel-bbbcb7f606e79142 > /tmp/brain-rac-r5.rIxFvm/kernel.log 2>&1
env -i PATH=/usr/bin:/bin rust/target/debug/deps/brain_serve-4d727e3e69d92ab4 > /tmp/brain-rac-r5.rIxFvm/serve.log 2>&1
env -i PATH=/usr/bin:/bin rust/target/debug/deps/dbrain_sources-f74331c8c60e1171 > /tmp/brain-rac-r5.rIxFvm/sources.log 2>&1
env -i PATH=/usr/bin:/bin rust/target/debug/deps/deadlock_brain_core-613f3bbd1c5689a1 > /tmp/brain-rac-r5.rIxFvm/core.log 2>&1
env -i PATH=/usr/bin:/bin rust/target/debug/deps/brain_feeds-d19d7c456d9d9326 > /tmp/brain-rac-r5.rIxFvm/feeds.log 2>&1
```

| Suite | Passed | Failed | Ignored | Filtered |
| --- | ---: | ---: | ---: | ---: |
| brain-kernel | 16 | 0 | 0 | 0 |
| brain-serve | 18 | 0 | 0 | 0 |
| dbrain-sources | 103 | 0 | 5 | 0 |
| deadlock-brain-core | 26 | 0 | 0 | 0 |
| brain-feeds | 19 | 0 | 0 | 0 |
| Summe | **182** | **0** | **5** | **0** |

Die fünf vorhandenen ignorierten DB-Tests wurden nicht aktiviert. Keine neue Workspace-, Release-, PostgreSQL-, Prozess- oder Lastmessung in R5 behauptet. C1 bleibt mit dem eigenen sechsfachen Prozessnachweis aus R4 geschlossen; dessen SCRAM-Nachweisgrenze bleibt erhalten. Die Autorenangaben 229/13 und 22 sind nicht in diese eigenen Zahlen eingerechnet.

### Unabhängige Treiber

Neuer Treiber, letzter maßgeblicher Compile und Lauf jeweils Exit 0:

```sh
/home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --edition=2021 /tmp/brain-rac-r5.rIxFvm/final_guard_probe.rs -L dependency=rust/target/debug/deps --extern brain_serve=rust/target/debug/deps/libbrain_serve-2b2494871e10569a.rlib --extern brain_contracts=rust/target/debug/deps/libbrain_contracts-c2a3d5430dd3a1d0.rlib --extern brain_kernel=rust/target/debug/deps/libbrain_kernel-fdc474d34bc23084.rlib --extern brain_api=rust/target/debug/deps/libbrain_api-a996f628c6e11efa.rlib --extern brain_policy=rust/target/debug/deps/libbrain_policy-766ebe4671b051c6.rlib --extern brain_feeds=rust/target/debug/deps/libbrain_feeds-0cfe46b69d37c205.rlib --extern dbrain_sources=rust/target/debug/deps/libdbrain_sources-cb96410a993a4f0f.rlib --extern dbrain_reasoner=rust/target/debug/deps/libdbrain_reasoner-310d38df49664fb2.rlib --extern dbrain_retrieval=rust/target/debug/deps/libdbrain_retrieval-1c081d4c1c8831c2.rlib --extern axum=rust/target/debug/deps/libaxum-ecf11048589fc2fa.rlib --extern tokio=rust/target/debug/deps/libtokio-167092b10e6f5621.rlib --extern tempfile=rust/target/debug/deps/libtempfile-6fb3e2d82ec0a5bb.rlib --extern sha2=rust/target/debug/deps/libsha2-a4a9af9010090e4c.rlib --extern serde_json=rust/target/debug/deps/libserde_json-fe8c7a93cbe9f687.rlib -o /tmp/brain-rac-r5.rIxFvm/final-guard-probe > /tmp/brain-rac-r5.rIxFvm/final-probe-build.log 2>&1
env -i PATH=/usr/bin:/bin /tmp/brain-rac-r5.rIxFvm/final-guard-probe > /tmp/brain-rac-r5.rIxFvm/final-probe.log 2>&1
```

**11 Prüfungen erfüllt, 0 fehlgeschlagen.** Ein vorheriger grüner Entwicklungslauf verwendete noch `Config::parse` und prüfte keine Evidence-IDs. Für das Urteil zählt ausschließlich die oben dokumentierte Endfassung mit `Config::load` und Evidence-IDs. Wiederholungen werden nicht addiert.

Der unveränderte 15-Fall-Treiber `/tmp/brain-rac-r3.OW58Dy/analytics_probe.rs` wurde separat mit derselben expliziten Bibliotheksliste ohne `brain_feeds` kompiliert. Eingabepfad, Ausgabepfad und Buildlog waren entsprechend `analytics_probe.rs`, `/tmp/brain-rac-r5.rIxFvm/analytics-probe` und `analytics-probe-build.log`. Anschließender tatsächlicher Lauf:

```sh
env -i PATH=/usr/bin:/bin /tmp/brain-rac-r5.rIxFvm/analytics-probe > /tmp/brain-rac-r5.rIxFvm/analytics-probe.log 2>&1
```

Compile und Lauf Exit 0, **15 Prüfungen erfüllt, 0 fehlgeschlagen, 0 ignoriert**. Meta-/Population-Semantik, wirksame Parameter, Principal-/Scope-/Release-Cachegrenzen, sichere Patch-Ablehnung, vier Slots, Rundenbudget, gemeinsame Quellendeadline und ausbleibender Provider-Fallback bleiben nachgewiesen. Die alte erste Snapshotprobe liefert weiterhin HTTP 504 nach 202 ms, keine Quellankunft und Worker-Ende nach 602 ms. Das ist getrennt von der neuen zweiten Snapshotprobe.

Ein erster indirekter Compileraufruf über Node wurde vor Ausführung vom Git-Schutz abgewiesen. Danach wurde der hier dokumentierte explizite `rustc`-Aufruf verwendet. Kein Hook wurde geändert oder umgangen; der abgewiesene Aufruf zählt nicht als Testlauf.

SHA256 der maßgeblichen Belege:

```text
435409afb9e3dec16bc377398f04ede81c8bd1181cd93a72c77c21da9bbcd771  /tmp/brain-rac-r3.OW58Dy/analytics_probe.rs
8fb8a9316745e726495aa8adfcd6fbaf030907656161e60198eb332ab171e72f  /tmp/brain-rac-r5.rIxFvm/final_guard_probe.rs
2271aef58203a25fe2bec6af65b608206e3d4d8c91671f864393597be6013286  /tmp/brain-rac-r5.rIxFvm/final-probe.log
4f72e9915592d7fd2cca91fd7b1c13ee2fec6c1f751e962294b8b30625f5acd9  /tmp/brain-rac-r5.rIxFvm/analytics-probe.log
```

## Übergabe

Keine offenen blockierenden Befunde aus diesem R5-Delta. Die bekannten Nachweisgrenzen bleiben sichtbar: fehlende Usage-Rückgabe bei Retrievalfehlern, vorübergehend belegte Claim-Leases gemäß Gesamtgate und SQLx-SCRAM statt Service-Passwortstart. Kein neuer Port-, Budget- oder Credentialumbau wird als erfolgt ausgegeben.

`MERGE-GATE-A.md` der Hauptsession berichtet auf `72db816056fb0ed53810ab77ea4417dc0812e7ca` ein erneutes Gesamtgate mit Exit 0 und ALLOW. Das ist fremder Gate-Nachweis, kein eigener Integrationsversuch. Die Hauptsession verantwortet den vorgesehenen Integrationsschritt nach `migration/rust-integration` und die finale Workspace-Prüfung. Dieser Reviewer veröffentlicht ausschließlich seinen Bericht auf dem eigenen Reviewbranch.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft
TESTNACHWEIS[TW-1]: 182 passed, 5 ignored | Baseline: keine neue Gesamtbaseline; zusätzlich 15 unveränderte und 11 neue unabhängige Laufzeitprüfungen erfüllt
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: interner Reviewbericht im Taskordner
MERGEPROTOKOLL[MS-1]: 1 lokaler Reviewmerge einzeln | Anläufe: 1 | Gate: kein eigener Main-/Integrationsmerge; Gesamtgate-ALLOW der Hauptsession getrennt benannt; unabhängiges A+C-Codeurteil GO
