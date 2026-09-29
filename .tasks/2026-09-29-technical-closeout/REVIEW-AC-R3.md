status: blockiert
Datum: 2026-09-29

# R-AC Runde 3: A34 und abgegebener A12-Nachtrag

## Urteil und geprüfter Stand

| Befund | Urteil |
| --- | --- |
| R-AC-A3 | **Behoben im abgegebenen Umfang.** Normale typisierte Analytics-Faktanfragen erreichen Meta-/Population-Beobachtungen; Patchbindung wird abgewiesen. Upstream-Parameter, Zähler und wirksames Fenster passen zum gepinnten Vertrag. |
| R-AC-A4 | **Weiter offen, P2.** Beobachtungsroute und zwei sequenzielle Quellen teilen ihre Deadline korrekt. Im normalen Antwortpfad beginnt die Analytics-Deadline jedoch erst nach dem Snapshot-Lesen. Selbst nach HTTP 504 wird dadurch ein neuer externer Lookup gestartet. |
| R-AC-A1 | **Behoben am nachgereichten A12-Stand `8a865d3`.** Vollständige Selektionsparameter und daraus abgeleitete positive Antwort unabhängig geprüft, siehe Abschnitt 5. |
| R-AC-A2/C1 | A2 durch erneute Drift-Gegenproben bestätigt. C1 behält das R2-Urteil; dessen angekündigter Nachtrag steht noch aus. |

**Keine Gesamtfreigabe.** Kein Produktionszugriff und keine finale Lastmessung. Dieser Bericht prüft das A34-Delta und den ausdrücklich nachgereichten A12-Fix, keine unbeteiligten Altpfade.

- Intent `562a877b-0939-440a-964d-1145d9e9431a`, Review-Thread `52c34332`, Auftrag `R-AC-R3-BRIEFING.md`.
- A34-Delta `34a2507..fded2ba02dad9244a9069a7dfa3309f20d4f33b4`, Implementierung `2403274` und `38cacca`.
- Vorheriger Reviewstand `63040ad`; ausdrücklich erlaubter lokaler Reviewmerge und geprüfter HEAD `b7410bdfa14b5c2410ece9ad90cf407c461f4e33`.
- Eigener Baum `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`.
- Belege außerhalb des Worktrees: `/tmp/brain-rac-r3.OW58Dy/`. Keine unfertigen fremden Bäume gelesen oder übernommen, keine Produktdateien verändert.

## 1. Verbleibender Befund: R-AC-A4, Deadline wird nach Snapshot neu begonnen

**P2, auftragsrelevanter Restbefund.** Fundstelle: `rust/crates/brain-serve/src/analytics.rs:250` und `:266` bis `:271`.

`retrieve_analytics` liest zuerst `self.release.snapshot(query, context)`. Anschließend berechnet es `Instant::now() + min(context.deadline_ms, 2 * request_timeout_ms)`. Die Snapshot-Dauer wird nicht abgezogen. Ein bereits erschöpftes Request-Budget wird somit beim Beginn der Analytics-Arbeit erneut vergeben.

Der äußere HTTP-Adapter beendet zwar die Clientantwort rechtzeitig mit 504 (`brain-api/src/http.rs:75` bis `:92`), beendet aber den laufenden Blocking-Worker nicht. Dieser kann nach dem Snapshot weiterhin Analytics-Slots belegen und neue HTTP-Anfragen starten. Der nachträgliche Budgetcheck in `brain-kernel/src/flight.rs:168` verhindert eine erfolgreiche verspätete Antwort, nicht diese nach Ablauf begonnene Arbeit. Es wird daher **kein fehlendes Client-Timeout und keine erfolgreiche Antwort nach Ablauf behauptet**.

### Unabhängige HTTP-Gegenprobe

Die Probe verwendet die unveränderte `analytics.rs` per `include!`, die frisch gebauten `ApiService`, `CachedKernel`, `Kernel`, `ReleaseRetriever`, Analytics-Client und den echten `brain_api::router`. Eingehender `/v1/answer`-Request und ausgehender Analytics-Request laufen über echte lokale TCP-Verbindungen. Der Snapshot-Port wird gezielt verzögert; es ist kein PostgreSQL-Lauf und keine Behauptung über eine gemessene Produktionslatenz. Der Provider zählt Aufrufe und verweigert sie.

Konfiguration der Gegenprobe: Request-Budget 200 ms, Analytics-Einzelbudget 100 ms, somit `2 * 100 <= 200` wie in `brain-serve/src/config.rs:315` verlangt. Die kontrollierte Snapshot-Verzögerung beträgt 600 ms.

```json
{"case":"expired_snapshot_must_not_start_new_http","detail":{"budget_ms":200,"client_response":"HTTP/1.1 504 Gateway Timeout","client_response_ms":202,"http_arrival_ms":[601],"per_http_timeout_ms":100,"snapshot_delay_ms":600,"worker_completion_ms":697},"passed":false}
```

Der Client erhält nach 202 ms HTTP 504. Der externe Loopback-Dienst erhält die neue Analytics-Anfrage erst nach 601 ms, also rund 399 ms nach der bereits gesendeten Fehlerantwort. Der Worker endet nach 697 ms. Die Erwartung „nach verbrauchter Deadline keine neue HTTP-Anfrage“ schlägt fehl.

**Konkrekte Korrektur:** Die absolute verbleibende Request-Deadline vor dem Snapshot festlegen und über Snapshot, Kapazitätsprüfung, Meta, Population und Retry weitergeben. Nach dem Snapshot muss der Ablauf geprüft werden, bevor Slot oder HTTP-Arbeit gestartet werden. Keine neue volle relative Zeitspanne aus dem unveränderten `context.deadline_ms` ableiten. Den Fall eines bereits abgelaufenen Snapshots über den normalen HTTP-Antwortpfad nachprüfen.

### Zwillingsprüfung und unauffällige Teilpfade

Graphify wurde vor der gezielten Suche befragt; für `AnalyticsRetriever` und `lookup_with_deadline` lieferte der Graph keine passenden Knoten. Danach wurden die folgenden bekannten Delta-Pfade direkt geprüft:

- Beobachtungsroute `analytics.rs:466,476,490,492`: eine Deadline vor Autorisierung und Slot-Warten, dieselbe Deadline für Worker und HTTP. Die beiden bestehenden Tests mit pausierter Tokio-Zeit liefen unabhängig erfolgreich.
- Normaler Meta-/Population-Pfad `analytics.rs:261` bis `:300`: volle Slots führen unmittelbar zu `Unavailable`; beide Quellen verwenden denselben einmal erstellten Zeitpunkt. Der Fehler ist dessen zu später Beginn, nicht ein neuer Zeitpunkt zwischen den Quellen.
- HTTP-Client `dbrain-sources/src/analytics_runtime.rs:186` bis `:209`: Restzeit statt neuem Budget, maximal zwei Versuche, Einzel- und Gesamtzeit begrenzt. Der darunterliegende `http/bounded.rs` berücksichtigt Retry-Wartezeit innerhalb der Gesamtzeit.
- Evidenzvalidierung `analytics.rs:430`: erneutes Snapshot-Lesen; anschließend schützen Cache-/Flight-Prüfung und HTTP-Timeout die Veröffentlichung. Daraus wird kein zusätzlicher selbstständiger Befund gemacht.

## 2. R-AC-A3: tatsächlicher Upstream-Vertrag und normale Antworten

### Semantik, Defaults und Zeitfenster

Geprüfter öffentlicher Upstream-Stand: `deadlock-api/deadlock-api@290cedba6cca7d8a07015e9feefecd22de27643c`. Gelesen wurde Quellcode, keine reale Spieler- oder Matchabfrage.

| Vertragsbestandteil | Upstream-Beleg und Abgleich |
| --- | --- |
| Defaults und explizite Filter | `hero_stats.rs:74` bis `:155`: Bucket ohne Gruppierung, normaler Spielmodus, ranked/unranked und ohne explizite Zeitangabe ein bewegliches Mindestdatum. Der Adapter setzt Bucket, beide Modi und beide Zeitgrenzen ausdrücklich. Kein bewegliches Zeitfenster bleibt aktiv. |
| Kein tagesgranularer Rollup | `hero_stats.rs:219` bis `:232`: bereits ein gesetztes `min_match_id` verhindert die MV-Route. `min_match_id=0` erzwingt im Adapter die Basistabelle, ohne positive Match-IDs abzuschneiden. |
| Spieler-/Matchzählung | `hero_stats.rs:443` bis `:475`: deduplizierte Spieler-Match-Zeilen, `countIf(won)`, `countIf(not won)`, deren Summe als `matches`. Bei `no_bucket` ist `matches_per_bucket` gleich `matches`, nicht eine globale Summe über Helden. |
| Itemfilter | `common_filters.rs:107` bis `:112`: `hasAll(items.item_id, [...])`, kein Array-Join. Wiederholte Käufe desselben Items vervielfachen den Zähler nicht. Mit einem Item ist dies die Quote der passenden Held-Spieler-Match-Zeilen mit diesem Kauf, keine Build-Anzahl. |
| Wirksame Zeitgrenzen | `common_filters.rs:27` bis `:32` und `:260` bis `:267`: inclusive `>=`/`<=`, Minimum abrunden, Maximum um bis zu eine Stunde aufrunden. Der Adapter sendet `ceil(min/3600)*3600` und `floor(max/3600)*3600-3600`; seine ausgewiesenen Grenzen passen zum wirksamen geschlossenen Stundenfenster. Leere Fenster werden abgewiesen. |

Quellen: [hero_stats.rs](https://github.com/deadlock-api/deadlock-api/blob/290cedba6cca7d8a07015e9feefecd22de27643c/api/src/routes/v1/analytics/hero_stats.rs), [common_filters.rs](https://github.com/deadlock-api/deadlock-api/blob/290cedba6cca7d8a07015e9feefecd22de27643c/api/src/routes/v1/analytics/common_filters.rs).

Eigene synthetische Fixture statt Übernahme der Autor-Zahlen: vier Spieler-Match-Zeilen für Held 18, zwei Siege und zwei Niederlagen. Zwei Zeilen enthalten Item 42; eine davon enthält es zweimal. Ein weiterer Held wird ebenfalls geliefert. Der normale Antwortpfad ergibt vier Matches sowie zwei von vier Item-Matches, also 50 %, nicht drei Käufe. Die Fixture modelliert den geprüften SQL-Vertrag; ein Upstream-ClickHouse wurde nicht betrieben.

Die beobachteten Requests enthalten `bucket=no_bucket`, `game_mode=normal`, `match_mode=ranked%2Cunranked`, `min_match_id=0`, `min_unix_timestamp=1790002800` und `max_unix_timestamp=1790082000`. Der ausgewiesene tatsächliche Zeitraum ist `[1790002800, 1790085600]`.

### Normaler Pfad, Rechte, Patchbindung und Cache

`brain-serve/src/service.rs:228` verbindet `AnalyticsRetriever` mit dem vorhandenen Kernel-/ApiService-Pfad. Unterstützt sind die dokumentierten typisierten Fact-Prädikate mit numerischer Held-ID und Locale `de`, keine beliebigen freien Meta- oder Build-Fragen.

- Eigene Meta- und Population-Anfragen liefern `Answered` im normalen `ApiService -> CachedKernel -> Kernel`-Pfad. Der Provider bleibt bei null Aufrufen. Schema-/Zählerdrift liefert keine Antwort und keinen Provider-Fallback.
- Eine explizite Patchbindung liefert `InsufficientEvidence` ohne neue Netzwerkabfrage. Die Beobachtung bleibt `unverified`; aus dem Zeitfenster entsteht keine behauptete Patch-Mitgliedschaft und keine patchgebundene Build-Empfehlung.
- Fehlender angeforderter Analytics-Scope liefert `UnauthorizedEvidence`; fehlender Principal-Grant liefert HTTP 403. Ein vorheriger Cache-Hit hebt diese Grenzen nicht auf.
- Gleicher Principal und gleiche Semantik: Cache-Hit ohne Netzwerk, neue Request-ID korrekt gebunden. Anderer Principal mit eigener Konversation: zwei neue Population-Lookups, keine Wiederverwendung der fremden Evidenz. Ein geänderter Release-Patch entwertet die zuvor gecachte Evidenz ohne neue Lookups.
- Statisch sind Principal, Konversation, Release-ID, Scopes, Domain, Profil, Patch und Modus in den relevanten Identitäten enthalten (`analytics.rs:133` und `brain-kernel/src/flight.rs:102`). Aktuelle Release-Prüfung und Evidenzidentität erfolgen vor Verwendung; Provider-Nutzung wird explizit verweigert (`analytics.rs:408` bis `:458`). Die 60-Sekunden-/128-Einträge-Grenze bleibt bestehen.

## 3. Verbrauchsanzeige bei fehlgeschlagenem Retrieval

Der im Autorbericht genannte Hinweis ist im geprüften Request-Pfad ein **nicht blockierender Diagnose-/Abrechnungsrest**, keine nachgewiesene Umgehung des Request-Budgets.

`analytics.rs:254` bis `:260` reserviert vor HTTP konservativ zwei Netzwerk-Runden für Meta und vier für Population. Bei weniger als vier Runden startet die eigene Population-Gegenprobe keine HTTP-Anfrage. Fehler in Meta oder Population verlassen den Retrieval-Pfad unmittelbar; der Kernel fällt dabei nicht auf den Provider zurück. Die HTTP-Versuche und Retry-Wartezeiten bleiben begrenzt.

Die `RetrievalPort`-Fehlervariante transportiert keinen `Usage`-Wert. Deshalb zeigt der Kernel bei Fehlern null Runden, obwohl ein Lookup versucht wurde (`brain-kernel/src/execution.rs:66` bis `:88`). Im untersuchten Pfad löst dieser Nullwert keine weitere Runde aus. Eine Aussage über nicht geprüfte globale Abrechnungssysteme wird daraus nicht abgeleitet. Der A4-Deadline-Befund oben besteht unabhängig von dieser verlorenen Verbrauchsangabe.

## 4. Eigene Läufe und Grenzen

### Bestandssuites

Die Testziele der drei betroffenen Pakete wurden frisch und offline gebaut. Ausgeführt wurden anschließend die drei Library-Testprogramme, nicht sämtliche Integrationstests oder Doc-Tests:

| Suite | Passed | Failed | Ignored | Exit |
| --- | ---: | ---: | ---: | ---: |
| brain-kernel | 15 | 0 | 0 | 0 |
| brain-serve | 16 | 0 | 0 | 0 |
| dbrain-sources | 103 | 0 | 5 | 0 |
| Summe | 134 | 0 | 5 | 0 |

Die fünf ignorierten Source-Tests benötigen eine zentrale beziehungsweise Scratch-PostgreSQL-Konfiguration. Sie wurden nicht als bestanden gezählt oder mit produktiven Zugangsdaten aktiviert. Die 174/12 des Autorberichts sind keine eigenen R3-Zahlen. Keine Baseline erhoben und kein Altfehlerurteil.

```sh
env -i USER=nathanael PATH=/home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin CARGO_HOME=/home/nathanael/.cargo CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target /home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo build --manifest-path /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/Cargo.toml --locked --offline -p brain-kernel -p brain-serve -p dbrain-sources --tests --message-format=json > /tmp/brain-rac-r3.OW58Dy/build.jsonl 2> /tmp/brain-rac-r3.OW58Dy/build.log
```

Build Exit 0. Ein vorausgehender direkter `env -i ... cargo test`-Aufruf wurde vom Worktree-Kommandoguard vor Ausführung zurückgewiesen, weil er die Befehlsform nicht als git-frei einordnen konnte. Kein Testlauf fand dabei statt. Die Testprogramme wurden danach anhand der Cargo-Artefaktliste ausdrücklich im zugewiesenen Worktree aufgerufen; keine Hook- oder Policyänderung.

```sh
env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/brain_kernel-bbbcb7f606e79142 > /tmp/brain-rac-r3.OW58Dy/kernel.log 2>&1
env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/brain_serve-4d727e3e69d92ab4 > /tmp/brain-rac-r3.OW58Dy/serve.log 2>&1
env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/dbrain_sources-f74331c8c60e1171 > /tmp/brain-rac-r3.OW58Dy/sources.log 2>&1
```

Die drei Befehle liefen in dieser Reihenfolge als `&&`-Kette mit Exit 0. Bestehende Prüfungen für volle Slots und verzögerte Slotfreigabe waren enthalten und bestanden.

### Unabhängige Loopback-Gegenproben

15 Laufzeitprüfungen: **14 erfüllt, 1 fehlgeschlagen, Exit 1**. Der fehlgeschlagene Fall ist der belegte A4-Restbefund. Diese Laufzeitmessungen mit kontrollierten I/O-Verzögerungen sind getrennt von den 134 Bestandstests gezählt; sie sind keine neuen eingecheckten, von Wall-Clock abhängigen Unit-Tests.

Zusätzlich zu den oben beschriebenen semantischen und Rechteprüfungen:

- Volle vier Analytics-Slots im normalen Antwortpfad: sofort `Unavailable`, keine neue HTTP-Anfrage.
- Meta und Population mit gemeinsamem 600-ms-Budget: zwei Requests, kein `Answered`, Ende nach 601 ms. Für die zweite Quelle wird nicht erneut ein volles Budget vergeben.
- Synthetischer 503 mit `Retry-After: 0` bei 100-ms-Gesamtbudget: ein HTTP-Versuch, kein zusätzlicher Versuch mit frischer Zeitspanne, kein Provideraufruf.

```sh
/home/nathanael/.cargo/bin/rustc +stable --edition=2021 /tmp/brain-rac-r3.OW58Dy/analytics_probe.rs -L dependency=rust/target/debug/deps --extern brain_serve=rust/target/debug/deps/libbrain_serve-2b2494871e10569a.rlib --extern brain_contracts=rust/target/debug/deps/libbrain_contracts-c2a3d5430dd3a1d0.rlib --extern brain_kernel=rust/target/debug/deps/libbrain_kernel-fdc474d34bc23084.rlib --extern brain_api=rust/target/debug/deps/libbrain_api-a996f628c6e11efa.rlib --extern brain_policy=rust/target/debug/deps/libbrain_policy-766ebe4671b051c6.rlib --extern dbrain_sources=rust/target/debug/deps/libdbrain_sources-cb96410a993a4f0f.rlib --extern dbrain_reasoner=rust/target/debug/deps/libdbrain_reasoner-310d38df49664fb2.rlib --extern dbrain_retrieval=rust/target/debug/deps/libdbrain_retrieval-1c081d4c1c8831c2.rlib --extern axum=rust/target/debug/deps/libaxum-ecf11048589fc2fa.rlib --extern tokio=rust/target/debug/deps/libtokio-167092b10e6f5621.rlib --extern tempfile=rust/target/debug/deps/libtempfile-6fb3e2d82ec0a5bb.rlib --extern sha2=rust/target/debug/deps/libsha2-a4a9af9010090e4c.rlib --extern serde_json=rust/target/debug/deps/libserde_json-fe8c7a93cbe9f687.rlib -o /tmp/brain-rac-r3.OW58Dy/analytics-probe > /tmp/brain-rac-r3.OW58Dy/probe-build-final.log 2>&1

env -i PATH=/usr/bin:/bin /tmp/brain-rac-r3.OW58Dy/analytics-probe > /tmp/brain-rac-r3.OW58Dy/probe-final.log 2>&1
```

Compile Exit 0, Lauf Exit 1. Ein erster Compile-Versuch verwendete nicht zusammengehörige serde_json-/sha2-Artefakte und endete mit Exit 1; korrigiert anhand der Cargo-Fingerprints, ohne Produktänderung. Ein früher Probelauf hatte außerdem eine zu enge Erwartung `Unavailable` statt ebenfalls zulässigem `BudgetExceeded`. Die Endfassung prüft die fehlende zusätzliche Netzwerkrunde, hält `2*T <= Request-Budget` ein und belegt A4 zusätzlich über den echten HTTP-Adapter. Frühere Probeausgaben bleiben als `probe.log` erhalten, werden nicht als weitere Produktbefunde gezählt.

Beleg-Fingerprints, SHA256:

```text
435409afb9e3dec16bc377398f04ede81c8bd1181cd93a72c77c21da9bbcd771  analytics_probe.rs
9e20416391c7ee08da0b686875f9bb4d723362e0edab6782a8e1f4279acce5ed  probe-final.log
0983437779d7cf195fd65d0bde5035291c21f88054571c136d909f37b40d5381  kernel.log
2a0a5591f10e12deac47dfc3f9b0a1bbc4d047beb38f555aadc38f775cb1842c  serve.log
788b98971ed9a7025a3e3aaad8cee37ed4caedab616b26dc501943dcfc7947ce  sources.log
```

## 5. Abgegebener A12-Nachtrag: A1 geschlossen

Nach dem ersten R3-Berichtscommit `d905363` ausdrücklich nachgereicht: A12 `8a865d34cc55a402931adbb9e11799c7b87f4b08`, gemeinsam mit A34 veröffentlicht als `c424566dcac4d050fb2426f352703fb2b2a42dea`. Autorisierter lokaler Reviewmerge und Prüfstand dieses Nachtrags: `7082ac8173c042da36413a0114c6efd974652d23`.

Der Nachtrag ändert Match-Adapter, Match-Fixture und A12-Bericht. Analytics-Code und A4-Fundstellen bleiben unverändert; die A34-Messungen oben gehören weiterhin ausdrücklich zum Stand `b7410bd`.

### Effektiver Upstream-Vertrag

`brain-feeds/src/deadlock_match.rs:125` enthält jetzt eine gemeinsame Parameter-Map für Generator und Locator. Erneuter Abgleich mit [bulk_metadata.rs am selben Upstream-Pin](https://github.com/deadlock-api/deadlock-api/blob/290cedba6cca7d8a07015e9feefecd22de27643c/api/src/routes/v1/matches/bulk_metadata.rs):

- Zeilen 85 bis 117: zehn boolesche Selektionsschalter. `include_info` hat Default true; die anderen haben Default false. Der neue Request setzt neun ausdrücklich auf false und `include_player_kda` auf true.
- Zeilen 208 bis 233: zusätzliche Match-/Spielerspalten sind optionale Listen, hier nicht gesetzt. Zeilen 292 bis 311: KDA aktiviert sechs Spielerbasisfelder und `kills`, `deaths`, `assists`. Zeilen 517 bis 579: ohne Info-/Zusatzflags verbleiben `match_id` und `players` auf oberster Ebene.
- Weitere Filter-/Sortierdefaults erzeugen keine zusätzlichen Antwortfelder. Der Matchmodus behält den dokumentierten Default ranked/unranked. Es wird keine neue Unterstützung beliebiger Matchmodi behauptet. Optionale Nullfelder können upstream entfallen; der bestehende Validator lässt entsprechende optionale Spielerfelder fehlen.

Eigene Gegenprobe mit synthetischen IDs 123/456 leitet den positiven Antwortkörper aus den tatsächlich generierten Parametern ab, einschließlich des Upstream-Defaults für ein fehlendes `include_info`. Die fehlgeschlagene R2-Erwartung wird nicht blind auf den jetzt abgewählten Info-Block übertragen.

```text
include_info=false
include_mid_boss=false
include_more_info=false
include_objectives=false
include_player_death_details=false
include_player_final_stats=false
include_player_info=false
include_player_items=false
include_player_kda=true
include_player_stats=false
```

Ergebnis: `match_id` plus vollständiger Neun-Felder-Spielerblock wird als ein Record akzeptiert, `batch.validate()` erfolgreich. Der alte Locator ohne `include_info=false` wird zurückgewiesen. Zusätzliche Infofelder trotz Abwahl sowie ein Locator ohne KDA-Schalter werden ebenfalls abgewiesen. Die drei ursprünglichen A2-Gegenfälle zu ungültigem hero_id, verschachtelten fremden Spielern und ungültigem accountId bleiben abgewiesen.

### Eigene Nachtragsläufe

| Lauf | Passed | Failed | Ignored | Exit |
| --- | ---: | ---: | ---: | ---: |
| Neue unabhängige Projektionsgegenprobe | 8 | 0 | 0 | 0 |
| brain-feeds Library | 18 | 0 | 0 | 0 |
| assets_starting_stats | 2 | 0 | 0 | 0 |
| match_store ohne Aktivierung des DB-Falls | 1 | 0 | 1 | 0 |
| Nachtrag gesamt | 29 | 0 | 1 | 0 |

Der PostgreSQL-Store-/Release-/Revoke-Fall blieb hier explizit ignoriert. Die vom Autor gemeldeten Scratch-PG-Läufe werden nicht als eigene Nachtragsläufe ausgegeben. Keine reale Matchabfrage. Der Unterschied zu den Autoren-Testzahlen ergibt sich aus dem kombinierten Reviewstand und der hier nicht aktivierten DB-Fixture.

```sh
env -i USER=nathanael PATH=/home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin CARGO_HOME=/home/nathanael/.cargo CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target /home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo build --manifest-path /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/Cargo.toml --locked --offline -p brain-feeds --tests --message-format=json > /tmp/brain-rac-r3.OW58Dy/a12-build.jsonl 2> /tmp/brain-rac-r3.OW58Dy/a12-build.log

/home/nathanael/.cargo/bin/rustc +stable --edition=2021 --test /tmp/brain-rac-r3.OW58Dy/match_projection_probe.rs -L dependency=rust/target/debug/deps --extern brain_feeds=rust/target/debug/deps/libbrain_feeds-dea7f42362bb6e0f.rlib --extern brain_contracts=rust/target/debug/deps/libbrain_contracts-c2a3d5430dd3a1d0.rlib --extern deadlock_brain_core=rust/target/debug/deps/libdeadlock_brain_core-b739af2b658fe51f.rlib -o /tmp/brain-rac-r3.OW58Dy/match-projection-probe

env -i PATH=/usr/bin:/bin /tmp/brain-rac-r3.OW58Dy/match-projection-probe --test-threads=1 --nocapture > /tmp/brain-rac-r3.OW58Dy/match-probe.log 2>&1

env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/brain_feeds-487c187678e15b25 > /tmp/brain-rac-r3.OW58Dy/feeds-lib.log 2>&1
env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/assets_starting_stats-f65af7401ee8cdd3 > /tmp/brain-rac-r3.OW58Dy/feeds-assets.log 2>&1
env -i PATH=/usr/bin:/bin /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/match_store-870195952ebdcb34 > /tmp/brain-rac-r3.OW58Dy/feeds-store.log 2>&1
```

Build, Probe-Compile und Probe-Lauf jeweils Exit 0. Die drei Bestandstestprogramme liefen als `&&`-Kette ebenfalls mit Exit 0. Über beide R3-Teilstände zusammen: 163 bestandene Tests, 6 ignorierte; separat unverändert 14 erfüllte und 1 fehlgeschlagene Analytics-Laufzeitgegenprobe. Das ist keine vollständige neue Suite auf einem späteren Gesamtintegrationsstand.

## Übergabe und Grenzen

A34 braucht die konkrete A4-Nachbesserung vor Schließung des Deadline-Befunds. A1 ist am abgegebenen A12-Nachtrag geschlossen. Der C1-Nachtrag wird erst nach seiner gesonderten Abgabe geprüft. Frühere Lastwerte sind kein Nachweis für eine spätere Gesamtintegration.

Bericht auf dem eigenen Reviewbranch sichern. Kein Main-/Integrationsmerge, keine neuen Modelle oder Abhängigkeiten, keine Budgetanhebung, keine realen Provider-/Match-/Replay-Abfragen, keine Nachrichten, kein Deploy und keine Branch-/Worktree-Löschung. Die lokalen Reviewmerges waren ausdrücklich erlaubt.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 3/3 geprüft
TESTNACHWEIS[TW-1]: 163 passed, 6 ignored | Baseline: nicht erhoben; separat 14 erfüllte und 1 fehlgeschlagene Laufzeitgegenprobe
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: interner Reviewbericht im Taskordner
MERGEPROTOKOLL[MS-1]: 2 lokale Reviewmerges einzeln | Anläufe: 2 | Gate: kein Main-/Integrationsmerge angefordert; R3 BLOCK wegen verbleibendem A4
