# Rust Core Completion — Integrator-Handoff

Stand: 25.09.2026. Repository: `EarlySalty/Deadlock-Brain`.
Branch: `codex/core-completion-20260925`. PR: **#38**, Ziel `main`.

**Review-Artefakt, kein Cutover:** Es wurde nicht gemergt, nicht deployt, keine Produktionsdatenbank verändert und kein lokaler Systemd-Dienst angefasst. Es wurden ausschließlich Fixture-Credentials und eigene Loopback-Testserver verwendet. Der vorhandene schmutzige Checkout blieb unangetastet; die Arbeit liegt in einem separaten Worktree.

## 1. Basis, Ergebnis und Herkunft

- Basiscommit: `25c6ed6951370b092f60c67a35bdbe37440ece5d` — frisch geholtes `origin/main`. Ein erneuter Fetch während der Abschlussprüfung bestätigte denselben Main-Stand.
- Core-Implementierungscommit: `5f15b7c31f071f67f4a2abe54a70d2df7f9bf96c`.
- Ergebniscommit einschließlich Test-Fixture-Korrektur, DomainStore und abschließender ACL-Abgrenzung: `60dec8f58c95eecd34671b69e2e08a9d8ecd71b9`.
- Ergänzungscommit `64cbf3b0f59847d607ae97800a550ac0d7f8319f`: kanonische Fact-/Rule-Storeports und isolierter CI-Workflow.
- Test-Fixture-Korrektur: `3988df1b87a71a682f9eff33b539d3a2dfa65d6e`.
- Der nachfolgende Handoff-Commit ändert nur Dokumentation. Seine ID steht in der PR-Historie; eine eigene Commit-ID wird nicht zirkulär in den Inhalt dieses Dokuments eingebettet.
- Betrachtetes hochgeladenes Planpaket: `Deadlock-Brain_Rust-Daten_Planpaket_v1.0(20260925-160442).zip`, SHA-256 `945be6983ff0235eb0ec36b451f9613ea567e0962ca5658364e8665980177716`. Insbesondere Contracts/Grenzen sowie Domain-, Retrieval-, Provider-, Kernel/API- und Adapter-Aufträge wurden abgeglichen. Die neueren Inventare auf `main` bleiben maßgeblich; das ZIP wurde nicht über sie kopiert.

### Selektiv aus PR #25 übernommen

| Originalcommit | Neuer Commit | Übernommener Inhalt |
|---|---|---|
| `f4220c2769bbe0f85a4ead2dac444ab4d4d385db` | `a6a965d399f88a673b88e7333c7f4466d4fd8d32` | Gemeinsame Contracts, Policy/Auth und ursprüngliche Tests |
| `9bc82ca85ab496d5abc8a563af11fc7a0fe6a873` | `1db885d3b2a4a200c80385d3602ce74c17f8562b` | Revisionierter Memory-/Postgres-Storage, v1-Migration, Dateifeeder und Tests |
| `2c470864d82a2a2e954cf8eb18552a7b33a59aa0` | `f18502f46fcb403fa5998c550a3191efc29e9544` | Alias-/Normalisierungsfixes, Retrieval-Port, Provider, Jev-Verträge, Kernel, API, Client und vorhandene Tests |

Alle drei wurden mit `cherry-pick -x` übernommen, nicht durch einen Merge des konfliktbehafteten PR-Branches. Die Übernahme war auf dem neuen Main-Stand konfliktfrei. Neuere Wiki-/Build-/Inventaränderungen wurden nicht durch ältere PR-Fassungen ersetzt.

Separat erhalten: S05-Probes/Legacy-Referenz und S07-Fixtures. Alle **48** Einträge von `s07/fixtures/SHA256SUMS` wurden erfolgreich verifiziert. Historische S05-Quellhashes und `REVIEW_S01_S09_20260924.md` sind Herkunftsnachweise, keine Testbestätigung für diesen neuen Branch.

### Bewusst nicht pauschal übernommen

Die 14 älteren Plan-/Inventar-/Preflight-/Statuscommits `8282b1b`, `800a95a`, `c49bdda`, `c9a0709`, `ac528d2`, `97d2ef5`, `cfaa06a`, `d9cc0f8`, `9da8f43`, `d05ebb4`, `e960bca`, `bf0c67f`, `da25e3b`, `e90ae5d` wurden nicht als komplette Commits transplantiert. Ihre wiederverwendbaren Test-Fixtures wurden gezielt erhalten. Veraltete Aussagen über blockierte oder bereits abgeschlossene Gates überschreiben keine neueren Main-Berichte.

Nicht als neuer produktiver Ersatzpfad verwendet: Python-Bridge, Rückdelegation an Legacy-HTTP, unautorisierte statische Trefferlisten, implizite Migration beim API-Start, nicht atomarer Cursor-Fortschritt oder ein aktives Jev-Entscheidungsgate.

## 2. Tatsächlich implementierter Core

| Bereich | Umsetzung / wichtigste Grenze |
|---|---|
| Contracts | Gemeinsame Query-/Evidence-/Budget-Typen plus Store-/Checkpoint-/Lease-, Embedding-, typed Rule-/Fact- und Public-API-Verträge. Größen-/ID-/BIGINT-Grenzen; explizite Versionskennungen. |
| Auth/Policy | Credential-Registry, Scope-Einschränkung statt Erweiterung, unveränderlicher Conversation-Eigentümer, persistenter Ownership-Port, explizite Public/Internal/Private-Egress-Prüfung. Auch `public` umgeht keine explizite Objekt-ACL. |
| Storage | Memory-Referenz und nativer Postgres-Pfad; atomare Records+Checkpoint-Transaktion, CAS-Generation, Worker-Fencing, Ablaufprüfung vor Commit, identischer Replay nach verlorener Bestätigung, immutable Releases mit Revision je Dokument. |
| Snapshots | Historischer Inhalt wird gepinnt; historische **und aktuelle** ACL gelten. Aktuelle Löschung und Zugriffsentzug wirken auch auf alte Releases. Fehlende Revisionen/Heads werden abgelehnt. |
| Ingestion | UTF-8-Dateifeeder mit Größen-/Dateianzahl-/Tiefenlimits, Source-/Konfigurationsfingerprint, revisionswirksamen ACL-Änderungen und Tombstones. Fehlender Root erzeugt keine Massenlöschung. Vorbereiteter Batch ist getrennt vom atomaren Commit. |
| Domain/Rules/Facts | Bestehende neuere Domain-/Build-Logik bleibt erhalten. Ergänzt: versionierter, deterministischer Regelauswerter mit typisierter AST, Einheiten, Patch/Mode, geprüften Fakteninputs, Release-Revisionen, Provenienz und Komplexitätsgrenzen; kein Modell als Wahrheitserzeuger. `DomainStorePort`/`DomainReader` liefert typisierte Fakten und Regeln aus kanonischen Snapshots und prüft sowohl das Objekt als auch seine Originalquelle einschließlich Hash, Revision und aktueller ACL. Generisches lexical/dense Retrieval schließt solche Domain-Envelopes aus, damit die zusätzliche Herkunftsprüfung nicht umgangen werden kann. |
| Retrieval | Release-gebundener lexical Port, modell-/versionsgebundener Embedding-Port, dichter Cosinus-Index, deterministische gewichtete RRF, stabile IDs, keine Doppelboosts oder widersprüchlichen gleichnamigen Belege. Fact-Profil nutzt keine Query-Embedding-Modellrunde. |
| Provider | Nativer Rust-Chat-/Embedding-Transport, Loopback-Fault-Tests, HTTPS-Anforderung außerhalb Loopback, keine Redirects/Umgebungs-Proxys, bounded Body, Deadlines, begrenzte exponentielle Retries mit Jitter/Retry-After, Circuit Breaker, Reservierung und Prüfung von Token-/Kosten-/Rundenbudgets. |
| Jev | Deaktiviert als Standard, optional reine Shadow-Vertragsbeobachtung; kein aktiver Modus, keine Änderung von Auth, Routing oder Antwortfreigabe. |
| Kernel | Geteiltes Retrieval-/Provider-Budget, Faktenpfad ohne Modell-Fallback, strukturierte Provider-Zitat-IDs, kanonische Belegprüfung vor Egress und nach Provider-Rückkehr, keine Antwort bei zwischenzeitlichem ACL-Entzug. |
| Cache | Begrenzter TTL-Cache nach Principal/Scopes/Egress/Conversation/Release/Query/Profile/Patch/Mode/Budget, erneute ACL-/Belegprüfung bei jedem Treffer; Single-Flight teilt laufende identische Anfragen, auch Fehler, speichert Fehler aber nicht dauerhaft. |
| API/Client | Echter Axum-HTTP-Adapter `/v1/answer`, Body-Limit, 16 begrenzte Blocking-Worker, Deadline/No-Store-Header, öffentliche Antwortprojektion und nativer typisierter Client mit Versions-/Request-ID-/Größenprüfung. Kein automatischer Listener-/Dienststart. |

Der End-to-End-Test verbindet **Datei → atomaren Store → Release → Retrieval → lokalen HTTP-Provider → Kernel/Cache → echte HTTP-API → typisierten Client**. Er prüft außerdem fremde Conversation-Eigentümer, interne Metadaten und Zugriffsentzug nach Cache-Befüllung.

## 3. Tests und tatsächliche Ergebnisse

<!-- FINAL_TEST_RESULTS -->
Die gezielte Core-Suite ist bestanden: **379 passed, 0 failed, 32 ignored**. Die ignorierten Tests umfassen den separat auszuführenden neuen Postgres-Core-Test und 31 bestehende Legacy-/DB-Opt-ins. Dies ist keine Behauptung, diese Legacy-Integrationen seien getestet.

Der erste vollständige Workspace-Lauf hatte einen Fehler in `dbrain-sources::deadlock_api::tests::demo_poll_total_timeout_caps_the_next_sleep`: Ein kalter Loopback-Aufruf wurde schon nach 30 ms abgebrochen, bevor der Testserver Request-Daten sah. `3988df1` korrigiert nur die Fixture: 250 ms Request-Fenster, weiterhin deutlich unter dem angeforderten Fünf-Sekunden-Sleep. Die produktive Polling-Logik und der separate Request-Abbruchtest wurden nicht geändert.

Abschließende Pflichtprüfungen und erneute Scratch-DB-Prüfung werden anhand der abgeschlossenen Logdateien ergänzt; bis dahin gilt keine pauschale Grünmeldung.
<!-- END_FINAL_TEST_RESULTS -->

### Reproduktion

Die Skripte laufen im Repo-Root; Cargo selbst läuft unter `rust/`. Toolchain: **Rust/Cargo 1.97.1**, durch `rust/rust-toolchain.toml` gepinnt. Das System-Cargo 1.75 ist dafür nicht geeignet.

```bash
BRAIN_TEST_CARGO=/home/nathanael/.cargo/bin/cargo bash scripts/check_brain_core.sh core
BRAIN_TEST_CARGO=/home/nathanael/.cargo/bin/cargo bash scripts/check_brain_core.sh all
```

Auf einem anderen Rechner `BRAIN_TEST_CARGO` passend setzen oder Cargo 1.97.1 im PATH verwenden. `all` führt die vier geforderten Cargo-Befehle und anschließend den isolierten DB-Test aus. Exit-Codes stehen in `.core-test-logs/results-all.tsv`; Detailausgaben separat in `fmt.log`, `clippy.log`, `test.log`, `release.log`, `postgres.log`. Die Umgebung wird mittels `env -i` und separatem Test-HOME von Anwendungsschlüsseln/üblichen Datenbankkonfigurationen getrennt.

Der DB-Test startet nur `.core-test-pg`, PostgreSQL 16, User `brain_core_test`, Unix-Socket im Worktree, Port 55439, **keinen TCP-Listener**. Er verweigert die Übernahme einer laufenden Instanz, prüft Socket/User/Verbindung und beendet seine eigene Instanz per Trap. Das isolierte Schema darf ausschließlich dort zurückgesetzt werden. Es werden niemals `DATABASE_URL` oder produktive DSNs als Fallback verwendet.

## 4. Geänderte Dateien und Review-Reihenfolge

Vollständige Liste der **166 Implementierungsdateien**, erzeugt mit `git diff --name-only 25c6ed6951370b092f60c67a35bdbe37440ece5d..60dec8f58c95eecd34671b69e2e08a9d8ecd71b9`: [`CODEX_CORE_CHANGED_FILES.txt`](CODEX_CORE_CHANGED_FILES.txt). Zusätzlich enthält der Dokumentationscommit dieses Handoff und die Dateiliste.

1. Die drei `-x`-Port-Commits zur Herkunft prüfen.
2. `5f15b7c` nach Contracts → Store/Policy → Ingestion → Retrieval/Provider → Kernel/API/Client → Tests prüfen.
3. `3988df1` separat als Test-Fixture-Korrektur prüfen; `64cbf3b` als typisierten DomainStore/CI und `60dec8f` als abschließende ACL-Abgrenzung prüfen.
4. `fa54a2a061354955ddbdfcb9e0384cbd42bf72e5` ist reine Formatierung von 46 bestehenden Workspace-Dateien. Sie ist isoliert, weil der geforderte `cargo fmt --all -- --check` den gesamten Workspace mit der gepinnten Toolchain prüft. Nicht als funktionale Rückportierung älterer Main-Logik interpretieren.

## 5. Genaue Integrationshinweise

**Nicht PR #25 zusätzlich blind mergen.** Seine Core-Implementierungen sind bereits enthalten. Parallele Contract-, Migration-, Retrieval- oder Adapter-PRs müssen gegen diesen Stand abgeglichen werden; doppelte Crate-/Tabellen-/Versionsdefinitionen nicht durch Überkopieren lösen.

Für eine spätere bewusste Rust-Komposition: `PgStore` als mutierenden `DocumentStorePort`, `LocalPgReader` als `SnapshotReadPort` und `ConversationOwnershipPort`, `PolicyEngine::with_ownership_store`, `ReleaseRetriever` oder `HybridRetriever`, Provider, `Kernel`/`CachedKernel`, `ApiService` und schließlich `brain_api::router` verbinden. Die synchrone PostgreSQL-/Provider-Arbeit gehört auf begrenzte Blocking-Worker. `core_http.rs` enthält die lauffähige Fixture-Komposition.

Zuerst beide SQL-Migrationen in dieser Reihenfolge explizit auf Scratch prüfen: `2026-09-24-brain-contract-v1.sql`, danach `2026-09-25-brain-core-jobs-v2.sql`. `PgStore::migrate_core` ist ein expliziter privilegierter Test-/Migrationsaufruf, kein API-Startschritt. Später getrennte Read-/Feeder-/Migration-Rollen und Rechte prüfen; die Scratch-Superuser-Rolle ist kein Produktions-Rollenmodell.

Ingestion: Checkpoint aus dem Store lesen, `prepare_batch`, Lease erwerben bzw. gültige Lease verwenden, denselben Batch über `commit_batch` atomar abgeben. Bei unklarer Bestätigung **denselben** Batch wiederholen. Abgelaufene Worker dürfen keinen neuen Stand veröffentlichen. Nicht den alten sequenziellen `apply_scan`-Kompatibilitätspfad als dauerhaften Core-Commit verwenden. Alte nichtleere File-Checkpoints ohne Source-Identität werden absichtlich abgelehnt: kontrollierte Neuaufnahme oder explizite Migration, kein stilles Raten.

Releases explizit veröffentlichen und API-Kontexte auf eine konkrete immutable Release-ID pinnen; der Alias `current` ist dort nicht zulässig. Historische Releases lockern aktuelle ACLs nicht. Der nackte `LexicalRetriever` ist ein Kompatibilitäts-/Fixture-Baustein, kein Ersatz für kanonische Release-Validierung; der Default des neuen Validierungsports lehnt fehlende Implementierungen ab.

Typisierte Fachobjekte werden als `StoredDomainObject` mit `brain.domain.v1` und `metadata.domain_contract` abgelegt. Für numerische Fakten und Regeln `DomainReader<LocalPgReader>` als `DomainStorePort` verwenden und seine autorisierten Ergebnisse an `CoreRuleEvaluator` geben. Objekt- und Originalquellen-ACL gelten gemeinsam. Diese JSON-Envelopes sind absichtlich aus generischem lexical/dense Retrieval ausgeschlossen. Ein fachlicher API-Dispatcher, der solche numerischen Rule-Ergebnisse als Antwort komponiert, ist noch kein impliziter Teil des generischen Text-Kernels; die zusätzliche fachliche Komposition bleibt offen.

Der dedizierte Workflow `.github/workflows/rust-core-verification.yml` führt dieselben vier Cargo-Gates, den isolierten Postgres-Test, die gezielte Core-Suite und die Fixture-Hashprüfung aus. Er hat nur `contents: read`, nutzt keine Provider-Secrets und enthält keine Merge-/Deploy-Schritte. GitHub-Checks und semantisches Review sind getrennt von den hier protokollierten Ausführungen zu lesen.

Die HTTP-Antwort ist jetzt **`brain.public.v1`**, nicht der interne `brain.v1`-Evidence-/Usage-Envelope. Client und API wurden gemeinsam geändert. Bestehende externe Konsumenten müssen bei ihrer späteren Integration bewusst umgestellt werden. Interne Quellenpfade, ACLs, Provider-Metadaten und Usage werden nicht einfach durchserialisiert.

## 6. Offene Voraussetzungen und ehrliche Grenzen

### Lokal/live zu verifizieren — nicht vorgetäuscht

Es wurden keine freigegebenen Live-Provider-Credentials bereitgestellt oder benutzt. **Live-Providerzugang ist offen.** Vor Aktivierung müssen reale Modell-/Revisionsnamen, Tokenisierung, Kostenobergrenzen, Egress-/Lizenzfreigaben, Limits und Timeouts mit genehmigten Credentials bestätigt werden. Die Reservierung verwendet derzeit UTF-8-Bytes plus Framing als konservatives Eingabelimit; die Eignung dieser Obergrenze ist pro Live-Modell zu bestätigen. Unbekannter Verbrauch fehlgeschlagener Retry-Runden wird als volle Reservierung belastet, nicht als kostenlos behauptet.

Produktions-Komposition, freigegebene Secret-Provisionierung, TLS-/DB-Rollen, echte Corpus-/Index-Größen, Latenz-/Lasttests, Consumer-Umschaltung, Monitoring und Cutover/Rollback bleiben eigenständige Integrationsschritte. `LocalPgReader` ist absichtlich ein Unix-Socket-Adapter, kein fertig provisionierter Remote-TLS-Adapter. In-Memory-Conversation-Eigentümer sind keine restartfeste Produktionskonfiguration.

### Noch keine vollständige S02–S09-Produktparität

Dieser Branch ist ein ausführbarer, isoliert getesteter Rust-Core, **keine pauschale Freigabe aller Inventarzeilen oder Gates G0–G6**. Nicht allein durch externe Credentials erledigt sind insbesondere eine vollständige producerübergreifende Source-/License-/Trust-/Game-Validity-Normalisierung, ein durchgängiger fachlicher DomainStore für alle Inventarobjekte, produktive Index-Veröffentlichung und vollständige Adapter-/Runtime-Komposition. Bestehende Domain-/Build-Module wurden erhalten; der zusätzliche Rule-Port ist Grundlogik über bereits vertrauenswürdig bereitgestellte, release-gepinnte Fakten, kein automatischer Verifikationsprozess für Rohdaten.

Belegprüfung sichert Identität, Inhalt, Revision und ACL strukturell. Sie beweist nicht, dass jeder generierte Satz semantisch aus den zitierten Dokumenten folgt. Belege sind derzeit ganze Dokumente, kein vollständiger Offset-/Chunk-Provenienzvertrag. Der dichte Referenzindex liegt im Speicher und verwendet Cosinus; dauerhafte Vektorindex-Generationen, weitere Distanzverträge und vollständige Retrieval-Qualitäts-/Paritätsmessung sind nicht als erledigt zu deklarieren.

Datei-Roots müssen vom Feeder kontrolliert werden; die Pfad-/Symlink-Prüfung ist keine atomare OS-Sandbox gegen einen gleichzeitig feindlich veränderten Dateibaum. Die API puffert und validiert Antworten statt ungeprüfte Modell-Tokens zu streamen. Blocking-Ports werden durch Workergrenzen und Transport-/DB-Timeouts begrenzt, nicht durch eine behauptete harte Abbruchgarantie für beliebigen synchronen Fremdcode.

Diese Grenzen sind beim Review und beim späteren Integrator ausdrücklich zu entscheiden. Keine davon rechtfertigt, fehlende Rust-Teile durch eine neue Python-Bridge oder Legacy-HTTP-Rückdelegation zu kaschieren.
