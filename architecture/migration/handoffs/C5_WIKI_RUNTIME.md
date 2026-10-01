# C5 — begrenzter Wiki-Runtime-Pilot

## Basis und Merge-Reihenfolge

Arbeitsbranch: `codex/fix-c5-wiki-runtime-integration`.

Beim Start war `migration/rust-integration` auf `087c522`, C4 lag als **noch nicht gemergter PR #43** auf `ea2bd15`. Deshalb basiert dieser Branch auf genau diesem C4-Commit; C4 wurde nicht eigenmächtig gemergt. Der PR zielt auf `migration/rust-integration`, ist aber von der vorherigen C4-Integration abhängig. Vor Freigabe Basis aktualisieren und Checks auf dem integrierten Stand wiederholen.

Scope dieses Changes: C5. Keine C6-Domain-Vollständigkeit, kein C7/C8-Cutover, keine Produktionsfreigabe und keine Änderung der globalen G0/G1-Abnahme.

## Startbarer Pfad

```text
brain-wiki-pilot capture (explizites --allow-network)
  -> finite Page-ID-/Titel-/Category-/Hero-Auswahl
  -> gepinnte aktuelle + vorherige Revision, vollständige Property-Continuation
  -> capture.json (Original-Slottext, keine Template-/Lua-Ausführung)
  -> vorhandener Parser
  -> Versioned<brain_contracts::wiki::WikiIr> (brain.ir.v1)
  -> stage_sources_with_pool (bestehender SourceStore, content-addressed Raw)
  -> PgStore (unveränderliche Raw- und seitenbezogene IR-Generationen)
  -> optional exakt geprüfte StoredDomainObject::NumericFact
  -> PgStore::publish_release / CorpusRelease
  -> LocalPgReader -> DomainReader / DomainStorePort
```

Keine zweite Contract-Crate und kein paralleler Facts-Store. Die Erweiterung verwendet `brain-contracts`, `dbrain-s12-wiki-probe`, `dbrain-sources`, `brain-storage` und den bereits vorhandenen bounded HTTP-Core.

Das Binary ist im bestehenden Package `dbrain-sources` enthalten:

```bash
cd rust
cargo build --locked -p dbrain-sources --bin brain-wiki-pilot
cargo run --locked -p dbrain-sources --bin brain-wiki-pilot -- --help
```

## Begrenzung und Fehlerverhalten

`CaptureOptions.scope` ist verpflichtend. Eine Namespace-Allowlist ist nur ein Filter, **kein Crawl-Auftrag**. Ohne explizite Seiten, Kategorien oder Heroes startet kein Request. Es gibt im neuen Collector keinen `allpages`-Request, keinen Redirect-Expand und kein rekursives Verfolgen von Links, Templates oder Unterkategorien.

| Eigenschaft | Pilotbeispiel | Harte Implementierungsgrenze |
|---|---:|---:|
| Seiten | nur Abrams + Haze, maximal 2 | 64 |
| API-Requests gesamt | 24 | 512 |
| HTTP-Entity-Bytes gesamt | 2 MiB | 8 MiB |
| Mindestabstand | 5 Sekunden | live mindestens 5 Sekunden |
| Capture-Deadline | 180 Sekunden | maximal 1 Stunde |
| Revisionsfenster | aktuelle + eine vorherige | 2 |

Siteinfo, Titel-/ID-Auflösung, Category-Discovery, Revisions- und Dependency-Continuation teilen sich **ein** Request-/Byte-/Zeitbudget. HTTP-Timeouts werden auf die verbleibende Deadline begrenzt, inklusive persistiertem Rate-Limiter. Der bestehende HTTP-Core begrenzt Content-Length und Stream-Lesen; bei unbekannter Länge wird höchstens ein zusätzliches Prüfbyte gelesen. Das Budget zählt Entity-Bytes, nicht TLS-/HTTP-Header.

Keine automatischen Retries, Redirects, Ersatzhosts, Auth-Umgehungen oder Requests an aus Quelltext gelesene URLs. Der Live-Endpunkt bleibt der bestehende feste Wiki-API-Endpunkt.

Ein Budgetfehler, API-Warning, wiederholter/ungültiger Continuation-Token, fehlende Seite, Namespace-Verstoß oder Wechsel der Revision während der Dependency-Erfassung beendet den Capture **mit Fehler**. Keine stille Abschneidung und kein als vollständig ausgegebenes Teilergebnis.

Category-Discovery fragt nur direkte `cmtype=page`-Mitglieder in ausdrücklich erlaubten Namespaces ab. Auch leere Batches mit gültiger Continuation werden fortgesetzt. Alle Modul- und generischen Tokens werden erhalten; Tokens dürfen keine Query-Identität oder Namespace-Filter überschreiben.

Das neue Format `wiki-scoped-capture-v1` enthält einen ausdrücklich als Auswahl ausgewiesenen `selected_pages`-Manifest und `discovery_scope`. `discovery_complete` bedeutet hier ausschließlich **Vollständigkeit der angegebenen Auswahl**, nie Full-Wiki-Coverage. Alte `s12-capture-v1`-Fixtures bleiben offline lesbar; ihre historische Namespace-Inventory-Semantik aktiviert keinen neuen Netzwerkpfad.

## Quellen, Rechte und Wissensgrenzen

Originale MediaWiki-Revisionsnummern, Slot-UTF8 und SHA-256 bleiben erhalten. Gemeinsame Origins tragen konkrete `oldid`-Locatoren, Parser-/Schema-Version, Sprache, Rechte und Provenienz. Ältere Raw-Revisionen bekommen keine erfundenen historischen Dependencies: deren Dependencies sind ausdrücklich unknown; die Property-Erfassung gilt nur für den gepinnten Head.

Nicht ausgewählte Dependencies werden als ungelöste Referenzen erhalten, **nicht nachgeladen**. Damit können komplexe echte Hero-Seiten im kleinen Pilot korrekt unknown/quarantined bleiben. Eine erfolgreiche HTTP-Antwort ist keine Freigabe als Spiel-Fact.

Raw-Aufbewahrung braucht eine eigene explizite Zustimmung und eine eingetragene Quellenlizenz. Wiki-`rightsinfo` wird als untrusted Metadatum aufbewahrt, nicht als Berechtigungsentscheidung übernommen. Die Raw-/IR-/Fact-Records bleiben `Internal` mit `wiki.review`; Veröffentlichung und Provider-Egress werden nicht automatisch erlaubt. Der Pilot ruft keinen Provider auf.

`Unknown`, Conditions, Varianten, Units, Aliase und Dependencies bleiben in der gemeinsamen C4-IR. Für den vorhandenen numerischen Domain-Port werden nur Werte projiziert, deren Feld **und gesamte Dependency-Closure** auf konkrete Revision und Hash geprüft sind. Quarantänisierte Dependencies blockieren ihre abhängigen Facts. Bedingungen/Varianten werden nicht in unbedingte Zahlen umgedeutet. Nicht exakt als Domain-`f64` darstellbare Dezimalwerte bleiben ausschließlich in der exakten Wiki-IR; sie werden nicht still gerundet.

Der erste echte Capture erstellt absichtlich ein **leeres Mapping und keine Fact-Freigabe**. Raw -> IR -> Store -> Scratch-Release funktioniert damit bereits; die Anzahl geprüfter Facts ist zunächst null. Ein Operator muss anhand der echten Kandidaten/Locatoren ein Mapping und einen `ProjectionReview` mit `approved_fields` sowie `source_revisions` prüfen. Diese Freigaben nicht blind aus dem Capture generieren. Die mitgelieferten überprüfbaren Zahlen-Facts gehören ausschließlich zu synthetischen Testseiten.

Release-Patch/Mode sind explizite Operator-Eingaben. Die Startwerte `pilot-unverified` / `pilot-review` sind keine Aussage über den aktuellen Spielpatch. Die GameValidity der Quelle wird nicht aus Wiki-Zeitstempeln abgeleitet.

## Reproduzierbarer Offline- und Scratch-Test für Claude

Voraussetzungen: Rust 1.97.1, PostgreSQL-Werkzeuge (`pg_config`, `initdb`, `pg_ctl`, `createdb`) und die bestehenden Workspace-Buildvoraussetzungen. Als unprivilegierter Benutzer ausführen:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
bash scripts/test_wiki_runtime.sh
```

Dieser Befehl führt keine Wiki-Requests aus. Er baut die Fixture-/Mock-/Contracttests, erzeugt einen neuen temporären PostgreSQL-Cluster mit **Unix-Socket und ohne TCP-Listener**, führt den sonst ignorierten DB-Test ausdrücklich aus und startet anschließend das echte CLI mit synthetischen Capture-/Mapping-/Review-Dateien (`plan` und `stage`). Der Cluster wird per Trap gestoppt; Logs, Raw-Artefakte und Receipts bleiben im ausgegebenen `/tmp/brain-c5-pg.*`-Verzeichnis zur Prüfung erhalten.

Es wird kein Anwendungs-DSN, kein `.env`, kein Infisical-Secret und keine Produktionskonfiguration geladen. `ScratchWikiStore` prüft vor dem ersten Write den Scratch-Marker, den tatsächlichen `data_directory`, Datenbank-/Rollennamen und Unix-Socket-Transport. Es gibt keinen beliebigen DSN-Parameter und keinen Fallback auf `pg_pool()`.

Die Scratch-Initialisierung verwendet die vorhandenen Core-Migrationen und ein minimales, zum bestehenden SourceStore passendes `source_runs`/`source_documents`-Schema. `wiki_scratch.sql` ist **keine Produktionsmigration**.

## Bewusst kleiner echter Capture — ausschließlich lokal und opt-in

**Dieser Befehl wurde remote nicht ausgeführt.** Vorher die Rechteentscheidung, Lizenz und Raw-Aufbewahrung ausdrücklich prüfen und in einer lokalen Kopie setzen; das Beispiel enthält keine Secrets.

```bash
cp config/wiki-c5-pilot.example.json /tmp/wiki-c5-config.json
# /tmp/wiki-c5-config.json: eigene decision_ref und geprüfte source_license
# eintragen, offline_review_allowed und raw_retention_allowed bewusst freigeben.

bash scripts/run_wiki_pilot.sh --allow-network \
  /tmp/wiki-c5-config.json /tmp/wiki-c5-real-001
```

Nur Abrams und Haze werden als Hero-Titel aufgelöst. Der Wrapper erzeugt danach ausschließlich eine neue Scratch-DB und stoppt sie wieder. Capture, `analysis.json`, gemeinsame `ir.json`, Starter-Mapping, `release-request.json`, Raw-Dateien und Stage-Receipt liegen unter dem angegebenen neuen Ausgabeordner. Bei Capture-Fehlern wird nicht weitergestaged.

Für wenige **konkrete** Template-Seiten: zunächst deren genaue Titel aus `analysis.json` / den ungelösten Referenzen prüfen; anschließend diese Titel in `scope.pages` oder `heroes[].pages` und Namespace `10` in die Allowlist aufnehmen. `max_pages` entsprechend klein erhöhen. Keine automatische Dependency-Expansion, kein "alle Templates" und keine Wahl beliebiger vermeintlich neuester Versionen.

Nach manueller Mapping-/Review-Prüfung kann derselbe Raw-Capture ohne Netzwerk in einer neuen Scratch-DB verarbeitet werden:

```bash
bash scripts/run_wiki_pilot.sh --from-capture \
  /tmp/wiki-c5-real-001/capture/capture.json \
  /tmp/wiki-c5-real-001/capture/mapping.json \
  /tmp/wiki-c5-real-001/capture/release-request.json \
  /tmp/wiki-c5-reviewed-001
```

Ein `receipt.json` mit `scratch_release_written: true` bestätigt ausschließlich das lokale Scratch-Release. Es setzt keinen produktiven Latest-Pointer und hebt keine G0/G1-Blocker auf.

## Delta, Reparse und Wiederholung

```bash
cd rust
cargo run --locked -p dbrain-sources --bin brain-wiki-pilot -- delta \
  /tmp/before/capture.json /tmp/before/mapping.json \
  /tmp/after/capture.json /tmp/after/mapping.json /tmp/wiki-c5-delta
```

Gleicher Scope und unveränderte Revisionen ergeben keine Raw-Invalidierung. Ein geändertes Template invalidiert nur seine transitiven Dependants innerhalb der erfassten Auswahl; unabhängige Hero-IR/Facts behalten ihre Generation. Bei unvollständigen Dependency-Closures bleibt die Invalidierung konservativ, aber weiterhin auf den kleinen Capture begrenzt. Unterschiedliche Scopes sind nicht als Delete-Delta vergleichbar. Eine entfernte Category-Mitgliedschaft führt nicht automatisch zu einem Upstream-Tombstone.

Neue Mapping-/Parser-Ergebnisse werden aus vorhandenem Raw über den gleichen Extract-/Stage-Pfad erzeugt. Mapping-Neuprojektion bleibt gemäß bestehendem Contract von Raw-Reparse getrennt. Ein neuer Abrufzeitpunkt schreibt keine bereits gespeicherte Wiki-Revision um. Gleiche Raw-Revisionen werden auf Hash/Inhalt, Origin und Rechte geprüft; die erste gespeicherte Beobachtung bleibt unveränderlich. Aktuell widerrufene Rechte und veraltete Captures werden vor Raw-Restaging abgewiesen.

Release-IDs sind unveränderlich. Für einen geänderten Capture oder ein Reparse mit anderer IR einen neuen expliziten Release-Namen verwenden. Raw- und IR-Staging ist wiederholbar, aber **keine globale Transaktion über Dateisystem und beide Stores**: Bei Fehlern können ungepublishte Staging-Artefakte zurückbleiben. Das Release wird erst nach erfolgreichem Schreiben aller referenzierten Records veröffentlicht. Ein abgebrochener Capture erzeugt überhaupt kein vollständiges Capture-Artefakt.

## Testabdeckung und Abnahme

Fixtures: `rust/crates/dbrain-wiki/tests/support/c5.rs` (synthetische Hero-, Template-, Alias- und unabhängige Hero-Seite; zwei Revisionen).

- `capture_transport.rs`: Allowlist, Page-/Category-/Hero-Scopes, Request-/Page-/Byte-/Deadline-/Intervall-Budgets, leere Category-Batches, Token-Injection/Loops, Revisionsfenster, Dependency-Continuation und Race-Erkennung, Unknown/Alias/Locators, unchanged/changed Delta, transitive selektive Invalidierung, Scope-Wechsel.
- `dbrain-sources/tests/wiki_runtime.rs`: gemeinsame IR -> überprüfte Facts/Release, explizite Rechte, unvertrauenswürdige Lua-/Template-Inhalte, transitive Quarantäne, unveränderte unabhängige IR/Facts, Scratch-Guard.
- Der ausdrücklich gestartete Scratch-Test prüft echte Raw-Dateihashes, SourceStore-Metadaten, PgStore-Historie, unveränderte Wiederholung, geänderte Revision, Reparse ohne Raw-Verlust, alte Release-Pins sowie ACL-Prüfung über den echten `LocalPgReader` / `DomainStorePort` an der Blocking-Worker-Grenze.
- Bestehende Offline-/Golden-Tests bleiben Bestandteil des Laufs. `.github/workflows/wiki-runtime-pilot.yml` führt den Scratch-Pilot auf PRs gegen den Integrationsbranch aus, ohne Live-Capture-Option.

### Remote ausgeführt — 26.09.2026

Rust 1.97.1, isoliertes Test-HOME und keine Anwendungs-/Produktions-DB-Zugangsdaten:

| Prüfung | Ergebnis |
|---|---|
| `cargo fmt --all -- --check` | Exit 0 |
| `cargo clippy --locked -p brain-contracts -p brain-storage -p dbrain-s12-wiki-probe -p dbrain-sources --all-targets --no-deps -- -D warnings` | Exit 0 |
| `bash scripts/test_wiki_runtime.sh` | Exit 0; **263 reguläre Tests + 1 separat ausgeführter Scratch-DB-Test bestanden, 0 Fehler** |
| Echtes CLI `plan` mit exportierter Fixture | 4 IR-Records, 3 überprüfte Facts, keine Veröffentlichung |
| Echtes CLI `stage` mit derselben Fixture | Scratch-Release geschrieben; 8 Raw-Revisionen, 4 IR-Records, 3 Facts, 0 Provider-Aufrufe |
| `git diff --check` | Exit 0 |

Der reguläre Testlauf enthält 8 Ignore-Markierungen; eine davon ist der anschließend ausdrücklich ausgeführte C5-Scratch-Test. Die übrigen 7 opt-in-/Live-/separaten DB-Tests wurden nicht aktiviert. Es wird keine vollständige Workspace- oder Produktionsabnahme aus den vier geprüften Packages abgeleitet.

Die finale Scratch-Evidenz liegt auf dem Testhost unter `/tmp/brain-c5-pg.yvvPjm`, der vollständige Lauf unter `/tmp/brain-c5-db.log`. Der dortige PostgreSQL-Server wurde durch den Cleanup-Trap gestoppt. Der DB-Test hat zusätzlich nachgewiesen, dass ein **aktueller Template-ACL-Widerruf abhängige Hero-Facts aus alten Releases ausblendet**, während der unabhängige Hero lesbar bleibt. Der vorhandene Domain-Reader wertet dazu ausschließlich optional mitgelieferte, versionierte `DocumentRevision`-Dependencies aus; unbekannte Versionen, gefälschte Hashes und doppelte Identitäten sind fail-closed.

Die reale Wiki-Verfügbarkeit, konkrete Seitengrößen, passende reale Mapping-Profile und Source-Rights-Freigabe bleiben der ausdrücklich lokalen Abnahme durch Claude vorbehalten. Es wurde kein Live-/Full-Wiki-Crawl, keine 85k-Migration und kein Zugriff auf eine Produktionsdatenbank ausgeführt.
