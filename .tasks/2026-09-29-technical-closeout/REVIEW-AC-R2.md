status: blockiert
Datum: 2026-09-29

# R-AC Runde 2: A1/A2/C1

## Urteil

| Befund | Ergebnis dieser Nachprüfung |
| --- | --- |
| R-AC-A1 | **Offen, blockierend.** Die Spielerprojektion wird angefordert, aber die standardmäßig ebenfalls aktivierten Match-Info-Felder werden vom neuen Projektionsvalidator abgewiesen. Eigene Gegenprobe reproduziert den Fehler. |
| R-AC-A2 | **Behoben für die bekannte Mängelliste.** Die drei zuvor akzeptierten Drift-Gegenfälle werden jetzt abgewiesen; eine gültige isolierte Spielerprojektion wird als validierter Batch akzeptiert. Die positive Gesamtvertragslücke bleibt unter A1 offen. |
| R-AC-C1 | **Behoben.** Ergänzte Prozessprüfungen am kombinierten Stand ausgeführt; sicherer Runner mit 3 bestandenen Tests abgeschlossen. |
| R-AC-A3/A4 | **Weiter offen.** Außerhalb dieser Nachprüfung; A34 wurde nicht aufgenommen. |

Keine Gesamtfreigabe. Die Lastwerte unten sind eine vorläufige Messung dieses Prüfstands, keine finale Integrationsmessung. Keine Produktänderungen durch den Reviewer, keine Produktionsaktivierung, keine Integration nach migration/main.

## Prüfstand und Umfang

- Auftrag: `R-AC-R2-BRIEFING.md`, Intent `562a877b-0939-440a-964d-1145d9e9431a`, Review-Thread `52c34332`.
- A12: `93468b044a5b3369fcaddbcb4f6bfa489ba917ac`, Produktfix `8e8ce76`, Basis `34a2507`.
- C: `ead4a7915a8edc82b9c4b27235852c0a481fee89`, C1-Fix `1882419b6178174df7b7a225561a3ec653cac430`.
- Eigener Reviewbaum: `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`.
- Autorisierte lokale Kombination: A12-Merge `a38e4fd45b99af47bbf80bf3906a0d2c7d4f0a14`, danach C-Merge und geprüfter HEAD `a3d564fc60270e71fb79998dba586e0cc35083eb`.

Geprüft wurde die bekannte Mängelliste A1/A2/C1, kein neuer Vollaudit. Autorberichte wurden mit Code, gepinntem Upstream-Vertrag und eigenen Läufen abgeglichen. Eigene temporäre Belege liegen außerhalb des Worktrees unter `/tmp/brain-rac-r2.rlFRIB/`; sie werden nicht als Produktdateien eingecheckt.

## 1. R-AC-A1: positiver Request-/Antwortvertrag weiterhin gebrochen

**Priorität P1.** Fundstellen: `rust/crates/brain-feeds/src/deadlock_match.rs:125`, `:157`, `:210` und `:325`.

Der generierte Request enthält jetzt `include_player_kda=true`, aber kein `include_info=false`:

```text
https://api.deadlock-api.com/v1/matches/metadata?match_ids=456&account_ids=123&include_player_kda=true&only_filtered_players=true&limit=1&format=json
```

Der im A12-Bericht selbst verwendete Upstream-Stand `290cedba6cca7d8a07015e9feefecd22de27643c` setzt `include_info` ausdrücklich auf `true`: [Default, Zeilen 85 bis 88](https://github.com/deadlock-api/deadlock-api/blob/290cedba6cca7d8a07015e9feefecd22de27643c/api/src/routes/v1/matches/bulk_metadata.rs#L85-L88). Die Aussage, boolesche Auswahlparameter seien standardmäßig `false`, trifft auf diesen Parameter nicht zu.

Damit enthält die Antwort zusätzlich `start_time`, `winning_team`, `duration_s`, `match_outcome`, `match_mode`, `game_mode`, `average_badge_team0`, `average_badge_team1`, `average_badge` und `not_scored`: [SELECT-Projektion, Zeilen 517 bis 530](https://github.com/deadlock-api/deadlock-api/blob/290cedba6cca7d8a07015e9feefecd22de27643c/api/src/routes/v1/matches/bulk_metadata.rs#L517-L530). Die angeforderte KDA-Projektion enthält die sechs Spielerbasisfelder und drei KDA-Felder: [Spielerprojektion, Zeilen 302 bis 311](https://github.com/deadlock-api/deadlock-api/blob/290cedba6cca7d8a07015e9feefecd22de27643c/api/src/routes/v1/matches/bulk_metadata.rs#L302-L311).

`projected_match_row` akzeptiert dagegen auf oberster Ebene ausschließlich `match_id` und `players` (`:210`). Dadurch wird die gültige Antwort auf den selbst generierten Request vor der Erstellung des Batches quarantänisiert. Der Ingest erreicht für diese Antwort weder Store noch Release.

### Eigene Gegenprobe

Synthetische IDs 123/456, keine Anfrage an die Match-API. Die echte frisch gebaute `brain-feeds`-Bibliothek erhält über `prepare_match_metadata_batch` eine `SourceHttpResponse` mit Status 200, `application/json`, `observed_at=1790000000`, einem Versuch und der tatsächlich generierten URL. Private Sichtbarkeit und Scope `account:123`; Egress, Publikation und Raw-Aufbewahrung sind deaktiviert.

Positiver Vertragsfall, der akzeptiert werden müsste:

```json
[{"match_id":456,"start_time":1790000000,"winning_team":0,"duration_s":1500,"match_outcome":1,"match_mode":1,"game_mode":1,"average_badge_team0":50,"average_badge_team1":50,"average_badge":50,"not_scored":false,"players":[{"account_id":123,"hero_id":18,"player_slot":0,"team":0,"hero_build_id":0,"pregame_hero_id":18,"kills":7,"deaths":2,"assists":5}]}]
```

Ergebnis:

```text
test upstream_default_info_with_requested_player_projection ...
REJECT quarantined source: match metadata contains fields outside the requested projection
FAILED
test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Ohne die zehn Match-Info-Felder wird derselbe vollständige Spielerblock akzeptiert: `ACCEPT records=1 validated=true`. Der Unterschied ist damit auf die unbeachtete Standardprojektion eingegrenzt, nicht auf Spieleridentität oder KDA-Typen.

**Erforderliche Korrektur:** Request und Validator auf dieselbe Projektion bringen. Bei der beabsichtigten Minimalprojektion `include_info=false` im URL-Generator und im exakten Locator-Vertrag aufnehmen. Alternativ die tatsächlich aktivierten Info-Felder vollständig typisiert behandeln. Die positive Fixture muss aus den effektiven Request-Parametern einschließlich Upstream-Defaults folgen. Kein Aufweichen der Drift-Prüfung.

**Zwillingssuche:** Graphify-Abfrage zu `match_metadata_url`, `projected_match_row`, `prepare_match_metadata_batch` und `include_info` ohne passende Knoten; danach gezielte Fundstellenprüfung. Der gemeinsame Validator betrifft Bibliothek und CLI. `brain-match-ingest.rs:144,165` verwendet den Generator und denselben Batch-Pfad. Der exakte Locator-Vertrag (`deadlock_match.rs:157`) und die feste Store-Test-URL (`tests/match_store.rs:28`) enthalten ebenfalls kein `include_info=false`. Die positive Unit-Fixture ab `deadlock_match.rs:787` bildet den aktivierten Info-Teil nicht ab. Kein zweiter unabhängiger Befund, sondern dieselbe verbleibende A1-Vertragslücke.

## 2. R-AC-A2: bekannte Drift-Gegenfälle geschlossen

`projected_match_row` prüft Top-Level- und Spielerfelder vor der Dokumenterstellung und baut die gespeicherte Projektion neu auf (`deadlock_match.rs:202` bis `:291`, Aufruf `:325`). Die ursprünglichen Gegenfälle wurden erneut gegen die echte Bibliothek ausgeführt:

| Eingabeänderung am Spieler mit account_id 123 | Ergebnis |
| --- | --- |
| `hero_id: {"drift":true}` | Abgewiesen: `match player hero_id is missing or invalid` |
| `new_private_players: [{"account_id":999}]` bei gültiger hero_id | Abgewiesen: `match player contains fields outside the requested projection` |
| `accountId: {"drift":true}` zusätzlich zur gültigen account_id | Abgewiesen: `match player contains fields outside the requested projection` |
| Gültiger vollständiger Neun-Felder-Spielerblock ohne zusätzliche Top-Level-Felder | Ein Record, `batch.validate()` erfolgreich |

Diese vier Gegenproben bestanden. Der positive Upstream-Gesamtvertrag scheitert weiterhin an A1; die Schließung von A2 ist keine Freigabe des Ingest-Pfads. Store-, Revoke- und Idempotenzprüfungen aus dem Autorbericht werden nicht als eigene neue R2-Läufe ausgegeben.

## 3. R-AC-C1: Prozessnachweise ergänzt und ausgeführt

Der sichere Runner wurde am kombinierten HEAD vollständig ausgeführt. Drei Tests bestanden, keiner fehlgeschlagen oder ignoriert, acht im gezielten Legacy-Library-Aufruf herausgefiltert. Der Prozessfall `binary_loopback_health_readiness_shutdown_and_no_fallback` lief bis zum Ende.

Start- und Nichtmigrationsfälle in `rust/crates/brain-serve/tests/process_e2e.rs:434` bis `:484`: Schema v99 führt zu Exit 1, `core_schema_incompatible`, keinem Listening und unverändertem Schema. Die leere Datenbank führt zu Exit 1, `database_unavailable`, keinem Listening und keinem neu angelegten `brain`-Schema. `scripts/test_brain_serve.sh:51` bis `:52` stellt getrennte Scratch-Datenbanken bereit.

Ergänzte fachliche Fälle im tatsächlichen Serve-Prozess:

- Englischer Alias `Guardian max health` liefert 770 (`process_e2e.rs:563`).
- `Unlisted Hero max health` liefert `InsufficientEvidence` (`:578`).
- Reverse-ACL verlangt konkret HTTP 403 und Body `forbidden` (`:640` bis `:644`).
- Nach Datenbank-Recovery beantwortet derselbe noch laufende Serve-Prozess eine neue Anfrage fachlich erfolgreich; Provider-Aufrufzahl wird geprüft (`:909` bis `:916`).
- Alias-Konflikt im normalen Retrieval-Pfad mit Limit 1 liefert `InsufficientEvidence` (`:974` bis `:990`).

Die zusätzlichen C-Änderungen seit `e540e97` zu fehlenden Assets-Werten und leerem Legacy-Archiv wurden im Delta gesichtet. Der ausgeführte Legacy-CLI-Test umfasst den Archivfall. Eine zusätzliche Ausführung der Assets-Testdatei wird hier nicht behauptet.

### Vorläufige Lastwerte dieses R2-Laufs

| Worker | Requests | Answered | Clientfehler | Dauer | Reader-Verbindungen |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 8 | 600 | 600 | 0 | 1425 ms | 4 |
| 16 | 600 | 600 | 0 | 1070 ms | 4 |
| 32 | 600 | 600 | 0 | 1154 ms | 4 |

Poolmaximum 4, Peak 4, fünf erzeugte Verbindungen einschließlich Recovery, 9115 wiederverwendete Checkouts. `wait_count=4651`, `wait_timeout_count=1`, `wait_max_micros=150068`. Der einzelne Timeout gehört zur absichtlichen Sättigungsprüfung, nicht zu einem fehlgeschlagenen Lastrequest. Das Pool-Wartebudget bleibt 150 ms; Laststufen, Poollimit und Retrieval-Budgets wurden nicht verändert.

Der Runner verwendet echtes Wegwerf-PostgreSQL per privatem Unix-Socket mit Peer-Authentifizierung und ohne TCP-Listener. Der Provider ist ein synthetischer Loopback-Dienst. Damit sind Prozess-, DB- und HTTP-Grenzen geprüft, keine echte Providerqualität. Der Runner aktiviert weder A34 noch einen echten Match-Ingest. Frühere rote Autorenmessungen werden durch diesen erfolgreichen Lauf nicht rückwirkend grün.

## Befehle und Exitcodes

Ausführung im eigenen Reviewbaum. Belegdateien: `/tmp/brain-rac-r2.rlFRIB/process.log`, `projection_probe.rs` und `projection.log`.

### Sicherer Prozessrunner

```sh
/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/scripts/test_brain_serve.sh > /tmp/brain-rac-r2.rlFRIB/process.log 2>&1
```

Exit 0. 3 passed, 0 failed, 0 ignored, 8 filtered. Enthalten: Legacy-CLI-Scratchtest, Legacy-Library-Scratchtest und Serve-Prozess-E2E.

### Frischer Bibliotheksbuild

```sh
env -i USER=nathanael PATH=/home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin CARGO_HOME=/home/nathanael/.cargo CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target /home/nathanael/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo build --manifest-path /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/Cargo.toml --locked --offline -p brain-feeds --lib
```

Exit 0. Die Gegenprobe verlinkt dieses Bibliotheksartefakt, keine fremden Build-Artefakte.

### Rust-Gegenprobe kompilieren und ausführen

```sh
/home/nathanael/.cargo/bin/rustc +stable --edition=2021 --test /tmp/brain-rac-r2.rlFRIB/projection_probe.rs -L dependency=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps --extern brain_feeds=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/libbrain_feeds.rlib --extern brain_contracts=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/libbrain_contracts-c2a3d5430dd3a1d0.rlib --extern deadlock_brain_core=/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/rust/target/debug/deps/libdeadlock_brain_core-5b3de70794cec2d9.rlib -o /tmp/brain-rac-r2.rlFRIB/projection-probe
```

Exit 0. Ein vorheriger Compile-Versuch mit dem noch aus Runde 1 benannten Core-Artefakt `libdeadlock_brain_core-4c422d1932efad79.rlib` endete mit Exit 1 wegen unterschiedlicher Crate-Instanzen. Danach wurde das zum frischen Build passende Cargo-Artefakt verlinkt. Kein Produktcode wurde dafür verändert.

```sh
env -i PATH=/usr/bin:/bin /tmp/brain-rac-r2.rlFRIB/projection-probe --test-threads=1 --nocapture > /tmp/brain-rac-r2.rlFRIB/projection.log 2>&1
```

**Exit 101. 4 passed, 1 failed, 0 ignored, 0 filtered.** Der fehlgeschlagene Test verlangt korrekte Akzeptanz des positiven Upstream-Vertrags und belegt A1. Der rote Lauf wird nicht als erfolgreicher Testlauf gezählt. Zusammen mit dem Prozessrunner: 7 passed, 1 failed, 0 ignored. Keine neue Baseline erhoben; kein Altfehlerurteil.

## Übergabe

A1 anhand des oben belegten Request-/Projektionsvertrags korrigieren lassen. Danach denselben positiven Vertragsfall und die Drift-Gegenfälle erneut prüfen. A3/A4 bleiben bis zur gesonderten A34-Abgabe offen; Gesamtfreigabe und finale Lastmessung folgen nicht aus diesem Bericht.

Die zwei lokalen Review-Merges waren ausdrücklich beauftragt. Dieser Bericht wird auf dem eigenen Reviewbranch committed und gepusht. Kein Main-/Integrationsmerge, kein Deploy, keine Branch- oder Worktree-Löschung; bestehende Hooks bleiben unverändert.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft
TESTNACHWEIS[TW-1]: 7 passed, 0 ignored | Baseline: nicht erhoben, kein Altfehlerurteil; 1 failed im positiven A1-Vertragsfall
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 1 belegt | Senke: interner Reviewbericht im Taskordner
MERGEPROTOKOLL[MS-1]: 2 lokale Merge-Schritte einzeln | Anläufe: 2 erfolgreiche lokale Review-Merges | Gate: kein Main-/Integrationsmerge angefordert; R2-Urteil BLOCK wegen A1
