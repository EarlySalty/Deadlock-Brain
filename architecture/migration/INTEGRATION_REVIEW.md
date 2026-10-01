# Integrationsreview Rust-Migration

Stand: 26.09.2026. Rolle: Reviewer, Integrator und lokaler Verifizierer (Claude, lokal auf dem Arbeitsserver).
Branch: `migration/rust-integration`, Worktree `~/.worktrees/brain-rust-integration-20260925`, Basis `origin/main` `25c6ed6951370b092f60c67a35bdbe37440ece5d`.
Getesteter Code-Commit: `15653cd` (alle folgenden Commits ändern nur Dokumentation).
Nichts wurde gemergt, deployt oder neu gestartet. Keine produktive Datenbank, kein Dienst und keine Community-Nachricht wurde verändert.

**PRODUCTION_CUTOVER_READY: NEIN**

Aktueller Stand: Dieser Bericht ist ein datiertes Protokoll. Maßgeblich für den heutigen Stand ist [PRE_G5_TECHNICAL_REVIEW.md](PRE_G5_TECHNICAL_REVIEW.md); Aussagen in den Abschnitten 1 bis 20 zu fehlendem Pooling, fehlendem C9, fehlendem Legacy-Konverter und fehlenden Feeds sind dort und in den Abschnitten 21 und 22 ersetzt.

Begründung (Stand 25.09.2026) in einem Satz: Der neue Kern läuft lokal mit echten Daten, aber es gibt keinen lauffähigen Dienst, echte Dokumente scheitern am Default-Budget, Wiki-, Replay- und Build-Pfade sind nicht an den Antwortpfad angeschlossen, und es gibt weder Staging noch einen freigegebenen Provider.

## 1. Eingänge und Entscheidung

| Eingang | Commit | Entscheidung | Begründung |
|---|---|---|---|
| `codex/core-completion-20260925` (PR #38) | `64cbf3b` | übernommen | konfliktfrei; Formatierungscommit `fa54a2a` geprüft (42 von 46 Dateien bitgleich zur rustfmt-Ausgabe, 4 wegen Modulauflösung nicht einzeln prüfbar, durch Workspace-fmt abgedeckt); keine Python-Bridge, kein Legacy-Fallback, keine Secrets. Der angekündigte Handoff `CODEX_CORE_COMPLETION.md` fehlt, PR-Text sagt noch "Verification in progress". |
| `codex/wiki-completion-20260925` (PR #35) | `f7fafa5` | übernommen | Konflikt `brain-contracts`: Core-Fassung behalten (Obermenge mit strengerer Validierung), doppelter Workspace-Eintrag entfernt, Lockfile neu aufgelöst |
| `codex/external-sources-completion-20260925` (PR #36) | `b44f2fe` | übernommen | Konflikte in `assets_api.rs`, `deadlock_data.rs` waren reine Formatierungskonflikte gegen `fa54a2a`; fachliche Fassung aus External Sources |
| `codex/replay-decoder-completion-20260925` (PR #37) | `f0b5bf1` | übernommen | CI-Konflikt: beide Schritte behalten |
| `codex/consumer-ci-completion-20260925` (PR #39) | `0bb949a` | übernommen | konfliktfrei; Referenz-Suites von Einzel-Pins auf den kombinierten Checkout umgestellt |
| PR #25 `review/brain-s01-s09-20260924` | `2c47086` | Code über Core, Doku ergänzt | Code kam über drei Cherry-Picks in Core; 89 fehlende Dateien (Planpaket v1.0, S03 bis S08 Vorbereitung) als historische Nachweise übernommen |
| PR #26 Cutover | `e2377f7` | enthalten | bitgleich in Consumer/CI (`infra/cutover/`) |
| PR #27 Quality/Performance | `5676167` | enthalten | bitgleich in Consumer/CI (`architecture/migration/evals/`, `benchmarks/s10/`) |
| PR #28 externe Quellen | `5e64ca4` | ersetzt | vollständig in External Sources Completion enthalten |
| PR #29 Replay | `afdffba` | ersetzt | in Replay Completion enthalten, S14-Dateien in neuerer Fassung |
| PR #30 Wiki | `fd00f24` | ersetzt | in Wiki Completion enthalten, S12-Dateien in neuerer Fassung |
| PRs #16 bis #22 | diverse | enthalten | jede Datei ohne Abweichung im Integrationsstand |
| Hero-/Wiki-Code (PR #13) und S01 (PR #34) | auf `main` | Basis | unverändert erhalten |

Verworfen wurde keine Arbeit vollständig. Nicht übernommen wurden nur überholte ältere Fassungen derselben Dateien.

## 2. Integrationskorrekturen (lokal, klein)

| Commit | Inhalt |
|---|---|
| `a8be08b` | Merge Wiki, Contracts- und Lockfile-Konflikt semantisch gelöst |
| `75ef5f9` | Merge External Sources, Formatierungskonflikte gelöst |
| `3889b2d` | Merge Replay, CI-Schritte beider Seiten behalten |
| `ef01700` | Consumer-CI prüft Wiki, Sources und Replay auf dem kombinierten Checkout statt auf drei Einzel-Pins |
| `963fd03` | Planpaket v1.0 und S03 bis S08 Vorbereitung aus PR #25 übernommen |
| `1e6803a` | sechs External-Sources-Dateien rustfmt, damit der Workspace-Formatcheck grün ist |
| `8c28619`, `15653cd` | lokaler Pilot-Test `rust/crates/brain-api/tests/local_pilot.rs` und `scripts/run_local_pilot.sh` (opt-in, nur Scratch-Cluster) |

## 3. Tests auf dem Integrationsstand

Toolchain Rust 1.97.1, `SQLX_OFFLINE=true`, keine DSN in der Umgebung. Pflichtprüfungen nach jedem Merge:

| Schritt | Commit | fmt | clippy `-D warnings` | test | release | Tests |
|---|---|---|---|---|---|---|
| main + Core | `39b239f` | 0 | 0 | 0 | 0 | 602 bestanden, 0 fehlgeschlagen, 62 ignoriert |
| + Wiki | `a8be08b` | 0 | 0 | 0 | 0 | 604 / 0 / 62 |
| + Sources | `75ef5f9` | **1** | 0 | 0 | 0 | 640 / 0 / 63 |
| + Replay | `3889b2d` | **1** | 0 | 0 | 0 | 697 / 0 / 64 |
| + Consumer/CI | `ef01700` | **1** | 0 | 0 | 0 | 709 / 0 / 64 |
| Endstand | `15653cd` | 0 | 0 | 0 | 0 | 709 / 0 / 67 (3 neue opt-in Pilot-Tests ignoriert) |

Der rote Formatcheck ab dem Sources-Merge stammt aus sechs bewusst unformatierten External-Sources-Dateien und ist mit `1e6803a` behoben.

Spezial-Suites auf `8c28619` (Code identisch zu `15653cd` außer dem Pilot-Test):

| Suite | Ergebnis |
|---|---|
| Wiki S12 `check-completion.py` | Exit 0, 214 bestanden, 0 fehlgeschlagen, 6 ignoriert |
| Replay `check-decoder.sh` | Exit 0, 170 bestanden (57 Debug, 57 Release, 56 Audit), 2 ignoriert; Audit meldet korrekt `status=blocked`, `real_matches=0` |
| Consumer client / quality / cutover | Exit 0, 14 / 85 / 54 bestanden, 0 ignoriert |
| Postgres-Vertragstest `test_brain_core_postgres.sh` | Exit 0, 1 bestanden (isolierter Unix-Socket-Cluster) |
| Live-Contract externe Quellen | Exit 0, 1 bestanden, 2 GETs |

TESTNACHWEIS[TW-1]: 709 passed, 67 ignored | Baseline: 0 rot (main 25c6ed6 vor Core nicht separat gemessen; alle Zwischenstände 0 fehlgeschlagen)

GitHub Actions auf PR #40 (https://github.com/EarlySalty/Deadlock-Brain/pull/40):

| Head | Run | Ergebnis |
|---|---|---|
| `5cfb0a5` | [36198651529](https://github.com/EarlySalty/Deadlock-Brain/actions/runs/36198651529), Versuch 1 | rot: "Integrated suite (wiki)" fand kein `protoc`, weil sie jetzt den ganzen Workspace samt Replay-Decoder baut; dadurch auch "Consumer Offline Gate" rot |
| `73477e8` | [36199382899](https://github.com/EarlySalty/Deadlock-Brain/actions/runs/36199382899) und [36199382850](https://github.com/EarlySalty/Deadlock-Brain/actions/runs/36199382850), Versuch 1 | grün: alle Core-, Audit- und integrierten Suites, Consumer Offline Gate, Rust compile, Migration composition, GitGuardian; "Semantic review" übersprungen |

Ein Check namens "Required PR Gate" existiert in diesem Repo nicht; das Consumer Offline Gate ist der vorhandene deterministische Aggregator. Jeder spätere Push macht diese Abnahme ungültig.

## 4. Runtime (lokal, nur lesend, 25.09.2026 23:45 und 26.09.2026 00:40 CEST)

| Unit | Zustand | Befund |
|---|---|---|
| `deadlock-brain-build-data` | failed, täglich 03:30 | läuft aus dem schmutzigen Hauptcheckout (Branch `feat/brain-rust-cutover-20260919`, uncommittete Skripte) mit Binary vom 23.09.; der Assets-Fix aus PR #33 ist darin nicht enthalten |
| `deadlock-brain-youtube-learning` | failed, alle 6 Stunden | Worktree `brain-live-main` detached auf `dfefc8f`, Preflight verlangt `main`, `deadlock-brain-yt` Binary fehlt |
| `deadlock-brain-wiki-refresh` | success 18:35, `state: ready` | Binary `/opt/deadlock-brain/releases/12814d9`; kein Commit-Pin konfiguriert |
| `deadlock-brain-patchnotes-sync` | success, alle 5 Minuten | Skript außerhalb des Repos, Binary im geteilten Hauptcheckout |
| `deadlock-brain-sheet-sync` | success 20:17 | Hauptcheckout |
| `deadlock-brain-site` | running seit 20.09. | `python3 server.py` außerhalb des Repos, Executable inzwischen gelöscht (Python-Update) |
| `dl-knowledge` | running seit 20.09. | Deadlock-Bots Release `46c4f07c`, zweiter Retrieval-Pfad |
| `dl-brain-feeder` | success 20.09. | Deadlock-Bots |
| `wiki-freshness` | failed 25.09. | Deadlock-Docs, "1 Seite(n) nicht pruefbar" |

Gegenüber S01 unverändert. Das S11-Runtime-Audit bestätigt die Liste und meldet `python_candidates=0`, obwohl `deadlock-brain-site` Python ausführt: die gelöschte Executable verdeckt den Namen (Falsch-Negativ des Werkzeugs).

## 5. Postgres und Storage (isolierter Scratch-Cluster, Unix-Socket, kein TCP)

| Fall | Ergebnis |
|---|---|
| Migration, erneute Migration auf befülltem Stand | bestanden (`migrate_core` idempotent) |
| Upgrade v1 nach v2 mit Altdaten | nicht separat geprüft |
| Rollback der Migration | nicht vorgesehen (keine Down-Migration) |
| Import, Resume über Checkpoint, Duplicate (Replay-Receipt), Fencing gegen abgelaufene Lease | bestanden (Vertragstest und Pilot) |
| Tombstone bei gelöschter Datei | bestanden, gelöschtes Dokument nicht mehr im Antwortpfad |
| ACL-Widerruf auf veröffentlichtem Release | bestanden, sofort wirksam über die Heads |
| Release-Konsistenz | Snapshot-Digest `0ee04956...` vor und nach Absturz identisch |
| Crash/Restart | `pg_ctl -m immediate`, automatische Recovery im Serverlog, Daten und Checkpoint intakt |
| Leerer Rebuild in frische Datenbank | identischer Digest `0ee04956...`, 7 Dokumente |

## 6. Wiki

Lokaler Zugang funktioniert: `siteinfo` mit HTTP 200 (Codex bekam remote 403). Lizenz laut `rightsinfo`: CC BY-NC-SA 4.0. MediaWiki 1.46.0, 26 Namespaces, 85 932 Seiten, davon 881 Artikel und 81 604 Dateien.
Ein echter Capture wurde nicht gestartet: Discovery inventarisiert alle nicht-virtuellen Namespaces mit `aplimit=50` (etwa 1 719 Requests bei mindestens 5 Sekunden Abstand) und bricht per Design bei `max_pages <= 10 000` ab. Ein Lauf hätte nur Last ohne Ergebnis erzeugt. Revisionen, Parser, Delta, HeroKnowledgeCard, Alias und Rebuild sind damit nur offline mit synthetischen Fixtures belegt. Der produktive Bestandspfad (Wiki-Refresh über deadlock-data, PR #13) läuft weiter.

## 7. Externe Quellen

Live-Contract lokal: OpenAPI 3.1.0 / API 0.1.0, 380 364 Bytes, Raw-SHA-256 `341bc2b2...`; `/v1/assets/colors` 10 334 Bytes, `d441a848...`, Vertrag gültig, keine Extra-Felder. Identisch mit der Codex-Messung vom 25.09., also kein Drift.
Git-Pins: Der produktive Source-Cache `deadlock-data` steht auf `66d0832d` (18.09.2026). Nach Integration verlangt `pull deadlock-data` und der Wiki-Refresh einen expliziten Commit-Pin; die laufende Unit setzt keinen und würde nach einem Deploy fail-closed abbrechen. Schemaänderung, Quarantäne, Raw-Hash und Parserrevision sind offline getestet, nicht mit echter Drift.

## 8. Replays

Auf dem Server liegt keine `.dem`-Datei (Suche in `/home`, `/opt`, `/var/lib`). Ohne ausdrücklich freigegebenen Match gibt es keinen echten Durchstich; es wurde nichts heruntergeladen. Decoder, Sandbox, Budgets und Dublettenlogik sind nur synthetisch belegt (170 Tests).

## 9. Provider und Jev

Kein freigegebenes Modell und kein Credential für den neuen Kernel; deshalb kein echter Providerlauf. Geprüft sind Timeout, Retry, Budget, fehlerhafte Antworten und Egress über lokale Fault-Server (Core-Tests) und im Pilot ein Providerfehler mit genau drei begrenzten Versuchen. Token wird in `Debug` als `<redacted>` ausgegeben. Jev steht standardmäßig auf `Disabled`, sonst nur `Shadow` ohne Autorität.

## 10. Echter Ende-zu-Ende-Pilot

Daten (außerhalb von Git unter `~/.local/share/deadlock-brain/pilot-20260925/`): sechs echte Heldendossiers aus dem veröffentlichten Game-Wiki-Snapshot `20260925T163559` (Manifest mit SHA-256) als öffentliche Quelle und eine echte interne Seite aus Deadlock-Docs (`internal/deadlock-brain/match-demo-learning.html`, Commit `92ed237`) als private Quelle.
Pfad: Dateiquelle, fencierter Commit in Postgres, Release, Absturz und Neustart der Datenbank, `ReleaseRetriever`, `Kernel`, `brain-api` über axum auf Loopback, `BrainClient`. Provider ist ein lokaler Stub, der Egress per Inhalts-Hash protokolliert. Nicht-Loopback-HTTP war per Blackhole-Proxy gesperrt; es gab keine Legacy-Aufrufe (der neue Pfad hat keinen Fallback).

| Fall | Default (Limit 6, 12 000 Tokens) | Diagnose (Limit 1, 200 000 Tokens) |
|---|---|---|
| öffentliche Frage | budget_exceeded | answered, Beleg `abrams.md` |
| interne Frage mit Scope | answered | answered |
| interne Scope-Anfrage mit öffentlichem Token | abgewiesen | abgewiesen |
| interne Frage mit öffentlichem Scope, kein Leak | kein interner Egress | kein interner Egress |
| exakte Zahl | budget_exceeded | answered, Beleg enthält Wert unverändert |
| Alias EN "Lady Geist" | budget_exceeded | answered |
| Alias "geist" | budget_exceeded | answered (nur lexikalischer Treffer, keine echte DE/EN-Aliasauflösung) |
| unbekannte Entität | insufficient_evidence | insufficient_evidence |
| fehlende Evidenz | insufficient_evidence | insufficient_evidence |
| falscher Patch | insufficient_evidence | insufficient_evidence |
| falscher Mode | insufficient_evidence | insufficient_evidence |
| ungültiger Token | abgewiesen | abgewiesen |
| Providerfehler | budget_exceeded | provider_error nach 3 Versuchen |
| ACL-Widerruf | kein Egress | kein Egress |
| Löschung | kein Egress | kein Egress |
| legaler / nicht legaler Build | nicht prüfbar | nicht prüfbar |

Ergebnis: Default 10 von 15, Diagnose 15 von 15. Der Pilot ist **nicht bestanden**, weil die Default-Konfiguration echte Dokumente nicht beantworten kann und Wiki-IR, Hero-Karte, Build-Legalität und Replay nicht im Antwortpfad hängen.
Ursache Budget: Der Retriever liefert ganze Dokumente ohne Chunking, der Provider reserviert ein Token pro Payload-Byte (`brain-providers/src/transport.rs`). Ein Dossier mit 28 KB ergibt JSON-escaped rund 60 KB.

## 11. Consumer

| Repo | PR | Checks | Review |
|---|---|---|---|
| Deadlock-Twitch-Bot | #983 Draft | "Typed Brain fixtures (knowledge)" rot durch bekannten Seed-Altfehler, Rest grün | neuer Port wird von keiner Route installiert, Historie bleibt auf dem alten Pfad |
| Deadlock-Bots | #457 Draft | grün | Adapter nicht verdrahtet. **Risiko:** auf `main` läuft noch der Workflow "Auto merge after gates"; der PR stellt ihn erst auf report-only um. #457 darf bis dahin nicht auf ready gesetzt werden |
| Deadlock-Docs | #3 Draft | grün | eigenständiges CLI-Werkzeug, nichts aktiviert |
| Deadlock-2nd-Brain | #1 Draft | rot, Runner kontoseitig nicht gestartet | kein Testfehler, Hosted-CI fehlt |

Keine Nachricht gesendet, kein Consumer umgestellt. MCP und CLI des Brain nutzen weiterhin den Bestandspfad; für sie gibt es keinen Adapter.

## 12. Qualität, Sicherheit, Performance

Hardware: AMD EPYC 9334, 16 vCPU (1blu-Container, ploop), 48 GB RAM, Postgres 16 Scratch-Cluster auf demselben Host. Host-Load während der Messung etwa 9.
Messung im Pilot (Diagnosevariante, 7 Dokumente, Stub-Provider, 8 parallele Clients, 600 Anfragen): p50 225 ms, p95 382 ms, p99 469 ms, Maximum 544 ms, 31,6 Anfragen je Sekunde, RSS 28 MB, CPU 47 Sekunden User über den Lauf (etwa 2,5 Kerne), IO 0 Bytes. Statusverteilung: 498 answered, 100 insufficient_evidence (widerrufener Held, korrekt), **2 unauthorized_evidence** ohne erkennbaren Grund.
Update-Verzug: Ingest und Release von 7 Dokumenten 152 ms; leerer Rebuild ebenso schnell.
Diese Werte sind keine Produktionswerte: kein echter Provider, winziger Korpus, alles auf Loopback.
Befunde: Der Retriever liest pro Frage zwei bis drei Mal den kompletten Release-Snapshot (skaliert linear mit dem Korpus). Die zwei `unauthorized_evidence` unter Last sind vermutlich Lese-Timeouts in der Evidenzprüfung, die der Kernel als Berechtigungsfehler meldet (fail-closed, aber falsch etikettiert).

## 13. Restore und Cutover-Probe

Geprüft: Backup-Äquivalent über leeren Rebuild aus den Rohquellen (identischer Digest), Absturz und Recovery, Writer-Fencing über Lease und Fence-Token, letzter Checkpoint nach Neustart erhalten, Delta (eine geänderte Datei ergibt genau einen neuen Record), aktuelle ACL- und Löschsperren wirken nach dem Neustart weiter.
Nicht geprüft: `pg_dump`/`pg_restore` eines Produktionsstands, Umschaltung eines echten Writers, Writes nach Umschaltung und Rollback auf den Altpfad. Dafür fehlt eine Staging-Umgebung. Produktion wurde nicht angefasst.

## 14. Gate-Stand

| Gate | Stand |
|---|---|
| G0 | teilweise: Inventar und Runtime belegt, SLO und Lastprofil nicht freigegeben |
| G1 | offen: gemeinsame Verträge für Wiki-IR, Replay-Observation und externes SourceIr nicht vereinheitlicht |
| G2 | nicht bestanden (siehe Pilot) |
| G3 bis G6 | offen |

## 15. Produktionsrelevante Risiken

1. Nach einem Deploy des integrierten Stands brechen Wiki-Refresh und `pull deadlock-data` ohne konfigurierten Commit-Pin ab.
2. Die neuen Tabellen werden vom Dienst selbst per `migrate_core` angelegt; in Produktion migrieren wir sonst von Hand als `postgres`, der Dienstrolle fehlt dort voraussichtlich das Recht.
3. `dbrain-sources` hängt als Produktionscrate an `architecture/migration/s12` (Probe-Crate im Doku-Baum).
4. Deadlock-Bots `main` kann #457 automatisch mergen, sobald der Draft-Status fällt.
5. Build-Data und YouTube-Learning laufen weiter rot aus nicht versionierten Checkouts; der Fix liegt nur auf `main`.
6. Der Wiki-Check `check-completion.py` ist Python und überschreibt versionierte Berichte bei jedem Lauf.

## 16. Bekannte Regressionen

Keine im integrierten Code gefunden: alle bestehenden Tests grün. Verhaltensänderung mit Betriebswirkung ist nur Risiko 1 (fail-closed ohne Pin).

## 17. Codex-Folgeaufträge

| Nr | Bereich | Auftrag | Abnahme |
|---|---|---|---|
| C1 | Kernel/API | Composition Root und Dienst-Binary `brain-serve` mit Config-Datei, Tokens aus Infisical, PgStore plus LocalPgReader; systemd-Vorlage ohne Aktivierung | Pilot läuft gegen das Binary statt gegen den Test |
| C2 | Retrieval | Chunking der Records mit Token-Schätzung passend zur Provider-Reservierung; indexierte Suche (BM25/Tantivy laut Plan) statt Vollsnapshot pro Frage | `scripts/run_local_pilot.sh` Default-Variante 15 von 15 |
| C3 | Kernel | Lese-/Timeoutfehler in `validate_evidence` als `unavailable` statt `unauthorized_evidence` melden | Lasttest ohne falsche Berechtigungsfehler |
| C4 | Contracts | Wiki-IR (Bedingungen, Varianten, Unknown, Locator), Replay-Typen (Unknown-Zeit, Tick -1) und externes SourceIr in `brain-contracts` vereinheitlichen; S12-Probe-Crate nach `rust/crates/` verschieben | G1-Vertragstests |
| C5 | Wiki | Namespace-Allowlist für Discovery, echter begrenzter Capture, `stage_sources_with_pool` gegen Scratch-DB, Fact-/Release-Port | lokaler Pilot mit zwei echten Revisionen |
| C6 | Domain | Hero-Karte und Build-Legalität (`dbrain-builds`, Reasoner) als Fact/Rule-Evidenz in den Kernel; DE/EN-Aliase aus ADR S05 | Pilotfälle legaler und nicht legaler Build |
| C7 | Quellen | Commit-Pin als Config-Feld für Wiki-Refresh und Patchnotes-Sync vor jedem Deploy | Unit-Start mit Pin grün, ohne Pin sichtbarer Fehler |
| C8 | Tooling | `check-completion.py` in Rust oder Shell, Berichte nicht mehr in versionierte Dateien schreiben; Runtime-Audit erkennt Python auch bei gelöschter Executable | CI grün, `git status` sauber nach Lauf |
| C9 | Consumer | Vertragslücken aus `CODEX_CONSUMER_CI_COMPLETION.md` Abschnitt 3, Twitch-Seed-Altfehler | Consumer-Tests grün |
| C10 | Replay | nach Freigabe eines echten Matches durch den Betreiber: Durchstich laut `CODEX_REPLAY_DECODER_COMPLETION.md` | echter Match als `supported`/`unverified` je Feld |
| C11 | Storage | Upgrade v1 nach v2 mit Altdaten testen, Down- oder Rückwegstrategie dokumentieren | Scratch-Test |

## 18. Entscheidungen beim Betreiber

1. Welches Modell und welcher Provider den neuen Kernel bedienen dürfen (Regel: nur mit ausdrücklicher Freigabe).
2. Freigabe eines echten Replay-Matches für den lokalen Pilot.
3. Ob das Wiki unter CC BY-NC-SA 4.0 für Brain-Antworten genutzt werden darf (Nichtkommerzialität).
4. Aufbau einer Staging-Umgebung für G3.

## 19. Nötige Schritte vor Production

C1 bis C7 und C11 umsetzen, danach Pilot in Default-Konfiguration 15 von 15, Staging mit echtem Datenstand, echter Provider im Shadow-Betrieb, Consumer-Parität (G3), Qualitäts-, Last- und Restore-Probe unter gemischter Last (G4), dann eine gesonderte ausdrückliche Freigabe des Betreibers für S17/G5. Dieser Bericht enthält keine Freigabevorlage, weil die Antwort NEIN ist.

## 20. Nachtrag 26.09.2026: Welle 1/2 integriert, eigene Brain-Instanz

C4, C11, C2/C3, C1, C7/C8, C5 und C6 (PRs #41 bis #45, #47, #48) sind auf `migration/rust-integration` integriert, getesteter Code-Commit `fe41451`. Details, Konfliktauflösungen und Befunde: [WAVE1_WAVE2_LOCAL_REVIEW.md](WAVE1_WAVE2_LOCAL_REVIEW.md). Die eigene PostgreSQL-Instanz für Brain (Port 5446, eigener OS-User, eigenes PGDATA, eigene Rollen, eigene Backups) ist eingerichtet: [BRAIN_POSTGRES_ISOLATION.md](BRAIN_POSTGRES_ISOLATION.md), Datenkopie aus DL-Main: [BRAIN_DB_MIGRATION_REPORT.md](BRAIN_DB_MIGRATION_REPORT.md).

Stand der Folgeaufträge aus Abschnitt 17: C1, C2, C3, C4, C6, C7, C8 und C11 sind integriert und lokal grün. C5 ist offline grün, der echte Capture wartet auf die Rechteentscheidung. C9 und der Pooling-Fix existieren noch nicht. C10 bleibt ohne freigegebene `.dem` offen.

Risiken aus Abschnitt 15 nach der Integration: Risiko 2 ist behoben (`brain-serve` prüft nur noch mit `check_core_schema()`, Migration nur per `brain-migrate`). Risiko 6 ist behoben (Python-Prüfer entfernt). Neu: Der Anfragepfad öffnet über `LocalPgReader` rund 10 PostgreSQL-Verbindungen je Anfrage; nur der Rollendeckel begrenzt sie.

TESTNACHWEIS[TW-1]: 940 passed, 71 ignored | Baseline: 0 rot

Marker dieses Abschnitts gelten für den Stand vor PR #49 und sind durch Abschnitt 21 ersetzt.

BRAIN_DB_ISOLATED: JA
BRAIN_DATA_MIGRATION_VERIFIED: JA
600_REQUEST_TEST_PASSED: NEIN (vor PR #49, ersetzt)
DEFAULT_E2E_PASSED: JA
WIKI_REAL_PILOT_PASSED: NEIN
CONSUMER_STAGING_PASSED: NEIN
PRODUCTION_DB_CUTOVER_READY: NEIN
PRODUCTION_CUTOVER_READY: NEIN

## 21. Nachtrag 26.09.2026 (Abend): DB-Pooling #49 und C9 #50 integriert

PR #49 (`codex/fix-brain-db-pooling`, Head `d020891`) und PR #50 (`codex/fix-c9-consumer-wiring`, Head `0424790`) sind semantisch reviewet und auf `migration/rust-integration` integriert; integrierter Code-Commit `ed06e13` (Merge `a24c80d` für #49, `ed06e13` für #50 auf #49-Basis). Damit sind die Aussagen „Pooling fehlt" und „C9 fehlt" aus Abschnitt 20 und den Nachweisstufen ersetzt.

Vollständiger Bericht mit allen Messwerten, Consumer-PR-Shas, Staging- und Restore-Ergebnissen: [FINAL_LOCAL_INTEGRATION_REVIEW.md](FINAL_LOCAL_INTEGRATION_REVIEW.md). Kurzfassung:

- Workspace auf `ed06e13`: fmt/clippy `-D warnings` grün, 943 Tests passed / 0 failed, Release-Build grün; `test_brain_serve.sh`, `test_brain_storage_upgrade.sh`, `test_brain_core_postgres.sh`, `test_wiki_runtime.sh`, `s12/check-completion.sh` grün.
- 600-Request-Test gegen die echte getrennte Brain-Instanz (Pool hart 4): je Stufe 600 answered, Peak-Pool 4, 8 Neuverbindungen, 0 „too many clients", 0 falsche `unauthorized_evidence`, Recovery nach erzwungenem Instanz-Neustart grün; `max_connections=40` unverändert.
- Default-E2E nach Pooling: 18/18 (inkl. `build_rejected`, ACL-Revoke, Delete, Providerfehler, Scope-Leak-Schutz, Neustart-Recovery).
- Consumer-Staging lokal: CLI, MCP, Docs-Adapter, 2nd-Brain-Adapter live gegen brain-serve-Staging; Twitch und Bots über Fixture-/Testpfade. Twitch-Befund „Shadow-Probe verzögert sichtbare Legacy-Antwort" im PR #984-Diff gefunden und mit Commit `d877a9d` behoben (Probe abgekoppelt, Regressionstest).
- Isolation und Backup/Restore erneut geprüft: alle Isolationsproben PASS; Restore-Probe `brain` fingerprint-gleich, `brain_pilot` erwartbar neu befüllt (Pilot-Wegwerfdatenbank, Sicherung von 03:19).
- Der rote „Semantic review"-Check an Bots #459 ist quota-bedingt („monthly quota exceeded") und wurde nicht umgangen; das GitHub-Runner-Problem an 2nd-Brain #2 (`runner_id=0`) ist infrastrukturbedingt.

TESTNACHWEIS[TW-2]: 943 passed, 0 failed | Baseline: 0 rot

BRAIN_DB_ISOLATED: JA
BRAIN_DATA_MIGRATION_VERIFIED: JA
DB_POOLING_VERIFIED: JA
600_REQUEST_TEST_PASSED: JA
DEFAULT_E2E_PASSED: JA
CONSUMER_STAGING_PASSED: JA
WIKI_REAL_PILOT_PASSED: NEIN
PROVIDER_SHADOW_PASSED: NEIN
REAL_REPLAY_PASSED: NEIN
G1_READY: JA
G2_READY: NEIN
G3_READY: NEIN
G4_READY: JA
PRODUCTION_CUTOVER_READY: NEIN

## 22. Nachtrag 26.09.2026: Pre-G5-Review

Legacy-Kernimport (`brain-legacy-import`), Brain-Seite der Feeds (`brain-feeds`: Patchnotes-Feed, Build-Publish-Port, Assets-API nach Kern) und die Wiki-Anbindung an den normalen `DocumentStorePort` sind gebaut und lokal verifiziert. Consumer-PR Deadlock-Bots #459 wurde auf Draft gesetzt, weil die Auto-Merge-Automation auf dessen `main` sonst nach Rückkehr des Semantic-Review-Kontingents gemergt hätte. Alle Zahlen, Gates und die verbleibenden G5-Blocker: [PRE_G5_TECHNICAL_REVIEW.md](PRE_G5_TECHNICAL_REVIEW.md).
