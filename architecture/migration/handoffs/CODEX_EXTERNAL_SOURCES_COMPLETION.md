# CODEX External Sources Completion · 2026-09-25

## Stand und Herkunft

- Repository: `EarlySalty/Deadlock-Brain`.
- Branch: `codex/external-sources-completion-20260925`.
- Basis: `origin/main` bei Arbeitsbeginn, `25c6ed6951370b092f60c67a35bdbe37440ece5d`.
- Bestand aus PR #28: Commit `5e64ca47e36df4d5de9fd071a4da04e350ba3ff0`, bestandserhaltend als `07e4254` übernommen. Die sieben ursprünglichen S13-Dokumente bleiben historische Vorbereitung; ihre damaligen Statusangaben sind kein Abschlussnachweis für diesen Branch.
- Getesteter Implementierungscommit: `cb4affeae07e1df0f607d0ea3ebdde1c24a1191d`.
- PR gegen `main`: `https://github.com/EarlySalty/Deadlock-Brain/pull/36`.
- Arbeitsmodus: explizit beauftragte Remote-Implementierung mit Fixtures und kleinen öffentlichen Contracttests. Kein Merge, Deploy, Dienstneustart oder Aktivieren produktiver Source-Jobs.
- Das Planpaket v1.0 und die vorhandenen S13-Handoffs wurden berücksichtigt. Dieser Branch erklärt keine projektweiten G1–G4-Gates, Pflichtfeeds, Rechtefreigaben oder Produktionsabnahme für bestanden.

## Architekturentscheidung

Die vorhandenen Rust-Adapter und `SourceStore` bleiben erhalten. Es gibt keine neue SQL-Migration, Tabelle, kanonische Faktenbank, Jobengine, Fremdruntime oder einen zweiten HTTP-Kern.

`external::SourceIr` ist ein gemeinsamer, lokaler Adapter-/Staging-Umschlag in `dbrain-sources`, Version 1. Er ist **nicht** ein heimlich ersetzter globaler `brain-contracts`-/Source-v2-Wirevertrag. Seine Informationen werden über vorhandene `source_documents.metadata` und bestehende Entity-Snapshots weitergegeben. Die Überführung in den parallel bearbeiteten Core-/Source-v2-Vertrag bleibt explizite Integrationsarbeit; keine SQL-/Contract-Version wird für andere Chats reserviert oder umdefiniert.

Der gemeinsame `HttpClient` erhält eine begrenzte Source-Lesemethode und eine interne Transport-Policy ohne Redirects/Decoding. Die bisherigen Client-Methoden behalten ihr Verhalten, einschließlich Gzip-Decoding. Ein Regressionstest prüft sowohl bytegenaue komprimierte Source-Antworten als auch das unveränderte Legacy-Decoding.

## Tatsächlich implementiert

| Bereich | Implementierung und Verhalten |
|---|---|
| Assets API | Vorhandener Fix `https://api.deadlock-api.com/v1/assets/...` bleibt bestehen. Standard bleibt Items/Heroes. Unbekannte/retired Kinds werden abgelehnt, wiederholte Auswahl wird dedupliziert. `colors` ist ein benanntes Objekt, Rank-Identität kommt aus `tier`, ID `0` bleibt gültig. Keine Positions-ID als Fallback. |
| OpenAPI-Snapshot | Unveränderte Response-Bytes als gepinnte Fixture; exakter Raw-SHA-256 und davon getrennte normalisierte Schema-Prüfsumme. OpenAPI-Dialekt und `info.version` werden getrennt erfasst. Kein automatisches Überschreiben der Baseline. |
| Schema Drift | Vergleich explizit konsumierter Responses sowie relevanter Request-/Security-Verträge. Lokale `$ref`-Auflösung mit Grenzen; keine externen Referenzabrufe. Version-/unbekannte Constraint-/Einheiten-/Kompositionsänderungen werden nicht als sicher kompatibel angenommen. |
| Klassifikation | Response-Feldentfernung, Type-/Formatwechsel, Verlust einer Required-Garantie und Erweiterung/Entfernung einer Response-Enum-Beschränkung blockieren den betroffenen Verbraucher. Additive Response-Felder und reine Annotationen können deterministisch non-breaking sein. Die Bewertung gilt für den dokumentierten Consumer-Scope, nicht als universelle OpenAPI-Kompatibilitätsaussage. |
| Quarantäne | Malformed JSON/UTF-8, doppelte JSON-Objektschlüssel, falsche Container, fehlende/mehrdeutige/duplizierte Identitäten, unerwartete HTTP-Status/Content-Types/Encoding und unbekannte Signale werden sichtbar quarantiniert. `payload()` verweigert dann Projektion. `SourceStore::persist_ir` schreibt den vollständigen empfangenen Rohbeleg und Metadaten, bevor es einen Quarantänefehler statt einer projizierbaren Dokument-ID liefert. |
| Raw-Artefakte | Tatsächliche HTTP-Entity-/Git-Blob-Bytes statt erneuter JSON-Serialisierung. Volle SHA-256-Dateinamen, atomisches No-Clobber-Schreiben und Integritätsprüfung bestehender Dateien. Keine stillschweigende Reparatur/Überschreibung korrupter Raw-Dateien. |
| Provenienz | Source-/Parserrevision getrennt; HTTP-Bodyhash, ETag/Last-Modified, sichere Header-Allowlist und Beobachtungszeit; Git-Commit, Pfad, Rawhash, Generator-/Zeitgrenzen. Parser-only-Reparse bekommt eine getrennte Dokumentidentität. Raw- und normalisierter Hash werden nicht verwechselt. |
| Provenienz-Deduplizierung | Wiederholte Ableitungen werden mengenbasiert dedupliziert; neu bekannte Ursprungsartefakte werden vereinigt. Gemeinsame Raw-Artefakte, explizite Ursprünge und bekannte Ableitungsfamilien bilden transitive korrelierte Gruppen. Unbekannte Herkunft bleibt ausdrücklich unbekannt. Es wird **keine** erfundene Zahl unabhängiger Stimmen ausgegeben. |
| Git-Pinning | Nur vollständige lowercase Commit-IDs mit 40/64 Hexzeichen; tatsächlicher Commit-Objekttyp wird geprüft. Keine impliziten Branches, Tags, Abkürzungen, `HEAD`, Clone-/Pull-/Fetch- oder Lazy-Fetch-Semantik. Der konkrete deadlock-data-Adapter prüft zusätzlich die konfigurierte Origin gegen seine Quellidentität. Das ist keine Signatur-/Urheberschaftsprüfung. |
| Git-Datenimport | Bestehende Fachnormalisierung bleibt erhalten, liest aber ein privates, begrenztes Staging aus gepinnten Blobs statt des veränderlichen Arbeitsbaums. JSON-Container und Entity-Identitäten werden vor Entity-/Domain-Schreiboperationen vorgeprüft. Worktrees mit `.git`-Datei funktionieren; Symlinks/Submodule werden abgelehnt. |
| History/Diff | 1–32 ausdrücklich angegebene Pins; Eltern einschließlich Merge-Eltern, Shallow-Status und fehlende Eltern bleiben sichtbar. Explizite Diff-Endpunkte, Änderungen/Löschungen und begrenzte exakte Rename-Erkennung. Fehlende Objekte werden nicht automatisch nachgeladen. Gitzeit ist keine Patch-/Spielgültigkeitszeit. |
| Schema-/Proto-Signale | `PinnedSignal::read` konsumiert lokale, commitgepinnt gelesene JSON-Schema-/Proto-Blobs über denselben IR-/Raw-/Provenienzpfad. Feldabhängigkeiten und gezielte Quarantäne sind testbar. Signale behaupten keine Gameplay-Änderung. Der Proto-Subset erkennt unter anderem Feldentfernung, Retagging, Typ-/Kardinalitätsänderung und additive optionale Felder. |
| Match-/Metadaten | Vorhandene Match-Metadata-/Player-History-Adapter verwenden jetzt begrenzte Reads, echte Response-Bytes und gemeinsamen IR-/Quarantänepfad. Konsumierte Match-/Hero-Identitäten werden geprüft; kein erfolgreicher Leerimport bei falschem JSON-Container. Bestehende Demo-/Replaylogik wird nicht durch einen neuen Decoder ersetzt. |
| CI | Der vorhandene PR-Workflow führt zusätzlich die deterministische Source-/HTTP-Regressionssuite aus. Ignorierte Live-/DB-Tests werden nicht automatisch aktiviert. Release-Gate und Merge-/Deploy-Regeln werden nicht abgeschwächt. |

### Grenzen der Consumer-Verträge

Assets/Match-Payloadprüfungen validieren Container und ausdrücklich konsumierte Felder/Identitäten, nicht sämtliche Spielmechaniken, Attribute oder Effekte. Zusätzliche Felder bleiben als Rohdaten und gemeldete Extras erhalten; daraus werden nicht automatisch neue semantische Fakten abgeleitet. Insbesondere wird für Match-Metadata kein vollständiges JSON-Response-Schema erfunden, wenn die aktuelle OpenAPI-Beschreibung es nicht liefert.

Der Proto-Parser ist absichtlich kein vollständiger Compiler: Imports, Optionen, Services, Maps, `oneof`, Extensions, unbekannte Editionen, unaufgelöste Typen und weitere nicht unterstützte Konstrukte führen zu Quarantäne. Es wird weder `protoc` noch Upstream-Code ausgeführt. JSON-/Proto-Signale und Abhängigkeitsgates sind als Bibliothek remote testbar; es wurden keine GameTracking-/SchemaExplorer-Produktionsjobs eingerichtet oder komplette Fremdrepositories geladen.

`group_provenance` liefert korrelierte Gruppen und ungeklärte Ursprünge, keine globale neue Confidence-/Reconciliation-Engine. Bestehende Domain-/Snapshot-Deduplizierung bleibt bestehen. Rechte-/Egress-Freigaben werden nicht aus öffentlicher Erreichbarkeit oder einer Code-Lizenz abgeleitet; die neuen Provenienzfelder starten nicht freigegeben. Eine globale Freigabepolicy oder Knowledge-Release-Transaktion wird hier nicht als bereits integriert ausgegeben.

## Budgets und Betriebsänderungen

- Source-HTTP: standardmäßig 8 MiB je Body, drei Versuche, 15 Sekunden pro Versuch, 45 Sekunden Gesamtbudget; Connect-Timeout höchstens fünf Sekunden. Größenprüfung sowohl auf Content-Length als auch im Stream, einschließlich Chunked-Antworten. Keine Redirects oder transparenten Decoder im Raw-Pfad.
- Retry: nur begrenzt für Verbindungs-/Timeoutfehler bzw. 429/5xx. `Retry-After` als Sekunden oder HTTP-Datum wird nicht nach unten gekürzt. Zu große oder nicht interpretierbare Warteanweisungen liefern einen sichtbaren Abbruch/Defer statt vorzeitig erneut anzufragen. Keine unbeschränkten Retry-Loops.
- Unvollständig/zu groß empfangene Bodies werden als Transportfehler abgewiesen; es wird kein vollständiger Raw-Beleg erfunden. Innerhalb des Budgets vollständig empfangene malformed Bodies bleiben erhalten.
- OpenAPI: 1 MiB; lokale Ref-Auflösung bis 64 Ebenen/100.000 Knoten; höchstens 128 explizite Abhängigkeiten und begrenzter Änderungsreport.
- Git: 20 Sekunden pro Objektbefehl; höchstens 2.048 Dateien, 8 MiB je Blob und 32 MiB pro materialisiertem Datenbaum. Bounded Diff-Output, keine externen Diff-/Textconv-/Hook-Ausführungen, keine Upstream-Downloads. History endet an den angegebenen Pins/fehlenden lokalen Objekten.
- Proto-Subset: 1 MiB, höchstens 65.536 Tokens und 16 Message-Verschachtelungen.
- Assets-/Match-Source-Reads umgehen den bisherigen URL-only-Cache, damit Request-Kontext, originale Bytes und Fetch-Provenienz nicht vermischt werden. Bestehende TTL-Optionen bleiben sichtbar als angeforderte Werte, gelten auf diesen neuen Reads aber nicht als wirksamer Cachevertrag. Andere bestehende HTTP-Aufrufer behalten ihren Cachepfad.
- `pull deadlock-data` benötigt jetzt `--commit FULL_SHA` oder `DBRAIN_DEADLOCK_DATA_COMMIT`. Wiki-Refresh mit Quellenupdate benötigt denselben expliziten Umgebungs-Pin. `--no-git-update` bleibt als Kompatibilitätsflag erhalten; automatische Updates sind grundsätzlich aus. `PullDeadlockDataOptions.update_repo=true` wird abgelehnt.
- Vor einer späteren Aktivierung müssen benötigte Git-Objekte separat und ausdrücklich bereitgestellt werden. Vorhandene Refresh-Jobs ohne Pin würden fail-closed abbrechen. Dieser Branch ändert keine produktive Konfiguration und setzt keinen Pin in Produktion.

## Reproduzierbare Nachweise

Toolchain: Rust/Cargo 1.97.1, passend zum bestehenden CI-Workflow. Alle Cargo-Aufrufe verwenden `SQLX_OFFLINE=true`; keine produktive DB, Provider oder privaten Daten wurden für die Prüfungen geöffnet. Der erste Versuch mit dem System-Cargo scheiterte an dessen alter Lockfile-Unterstützung; danach wurde explizit die passende installierte Toolchain verwendet.

Getesteter Codestand: `cb4affeae07e1df0f607d0ea3ebdde1c24a1191d`. Nachfolgende Handoff-Änderungen ändern keinen Rust-Code.

```sh
cd rust
export SQLX_OFFLINE=true

cargo +1.97.1 check --workspace --all-targets --locked -j2
cargo +1.97.1 test -p dbrain-sources -p deadlock-brain-core --locked -j2
cargo +1.97.1 clippy -p dbrain-sources -p deadlock-brain-core \
  --all-targets --no-deps --locked -j2 -- -D warnings

DBRAIN_EXTERNAL_LIVE_CONTRACT=1 cargo +1.97.1 test \
  -p dbrain-sources --test external_sources \
  live_small_current_assets_contract --locked -- --ignored --exact --nocapture
```

| Prüfung | Ergebnis |
|---|---|
| Workspace, alle Targets | Erfolgreich. CLI-/Wiki-Aufrufer mit explizitem Commitpin kompilieren. |
| Source-Unit-/Regressionstests | 88 bestanden, 0 fehlgeschlagen; 5 bestehende DB-Tests ignoriert. |
| Source-Contract-Integration ohne DB | 6 bestanden; der Live-Test wird im Standardlauf ignoriert. |
| Gemeinsamer HTTP/Core | 24 bestanden, einschließlich Raw-/Legacy-Gzip-Regression. |
| Summe deterministisch | **118 bestanden**, keine fehlgeschlagenen Tests. |
| Clippy | Erfolgreich mit `-D warnings`. |
| Formatierung | `rustfmt --check` auf neuen Modulen, dem überarbeiteten Assets-Adapter und der neuen Contract-Testdatei erfolgreich. Kein pauschaler Formatierungsumbau alter großer Dateien. |
| Separater Live-Contract | **1 bestanden**, zwei kleine GETs, keine DB-/Source-Jobs. |
| Whitespace | `git diff --check` erfolgreich. |

Abgedeckt sind aktuelle Routen/Container, ID 0, missing/extra/duplicate fields, fehlerhaftes JSON/UTF-8, doppelte JSON-Schlüssel, Versionswechsel, Response-Enums, lokale/zyklische/externe Schema-Refs, gezielte Quarantäne, echte Raw-/normalisierte Hashes, Parser-only-Dokumentidentität, neue Ursprungsinformationen bei Deduplizierung, Dirty Worktrees, `.git`-Dateien, falsche Pins/Origins/Objekttypen, Rename/Löschung/Merge/Shallow-History, Schema-/Protoänderungen, Transportgrößen, Timeouts, Redirects und Rate-Limit-Verhalten.

Ein bestehender Demo-Timeout-Test hatte unter Parallelbetrieb einen Testserver-Fehler, wenn der Client nach TCP-Connect, aber vor dem Senden von Headern korrekt abbrach. Der Testserver akzeptiert jetzt diese zulässige Abbruchvariante; Timeoutgrenze und fachliche Assertion wurden nicht gelockert. Die abschließende Suite läuft wieder parallel erfolgreich.

### Öffentlicher Live-Beleg

- OpenAPI-Dialekt: `3.1.0`; API `info.version`: `0.1.0`.
- OpenAPI: **380.364 Bytes**, Raw-SHA-256 `341bc2b2681d0bc353df721974bdbf46290c1f0645eba594d18381c3a2cbbc70`.
- `/v1/assets/colors`: **10.334 Bytes**, Raw-SHA-256 `d441a848795a58b9a4407dc5cf5ed782fd0c7e076bf5ed8cb890a9f7eddc5a84`; geprüfter Vertrag gültig, keine Extra-Felder.
- Die OpenAPI-Fixture und ihr Manifest liegen unter `rust/crates/dbrain-sources/tests/fixtures/external/`. Der Colors-Body ist nicht als Assetkorpus im Repository abgelegt.
- Der Live-Test erlaubt je Aufruf höchstens zwei GETs ohne Retry: Schema 1 MiB und Colors 64 KiB. Er ist standardmäßig ignoriert und nicht Teil des obligatorischen PR-Gates. Kein Account, Match, Replay, Bulkexport oder kostenpflichtiger Vorgang wurde abgefragt.

### Nicht ausgeführt / nicht behauptet

Die fünf bereits vorhandenen Postgres-Integrationstests sind nicht ausgeführt. Neu ist ein echter Dateisystemtest für `SourceStore::write_raw` mit einem Lazy-Pool ohne SQL-/Verbindungsaufruf; das ersetzt keinen DB-Roundtrip für `persist_ir`.

Es gab keinen Import gegen eine produktive oder private DB, keinen vollständigen Feedlauf, keinen realen Upstream-Git-Historydownload, keinen Provider-/Reconciliation-/Publication-End-to-End-Test und keinen Deploy. Der kleine öffentliche Colors-Test ist kein Nachweis für jede Assets-Variante oder sämtliche Match-/Gameplay-Felder. Lokale erfolgreiche Prüfungen sind außerdem nicht automatisch ein bestandener GitHub-Semantic-Review auf dem späteren PR-Head.

## Übergabe und Integrationshinweise

1. **Core / S02–S04:** Source-v2-/Rechte-/Quarantäneinformationen aus dem Adapter-IR bewusst in den gemeinsamen Vertrag übernehmen. Bestehende Store-Schlüssel und Metadaten nicht stillschweigend umdeuten. DB-Roundtrip, Outbox-/Release-Atomizität und Produktionsrechte separat prüfen.
2. **Wiki / S12:** Den kleinen Commitpin-Callerwechsel in `wiki_refresh.rs` mit parallelen Wiki-Arbeiten zusammenführen. Keine ältere Wiki-Implementierung über neueres `main` kopieren.
3. **Fachnormalisierung / S05 und Replay / S14:** Zusätzliche konsumierte Feldabhängigkeiten explizit registrieren. Unbekannte Proto-/Schemasemantik nicht als Gameplay-Änderung verkaufen; Parser-only-Reparse und Quelländerung getrennt halten.
4. **Betrieb:** Erst nach eigenständiger Review-/Integrationsentscheidung konkrete Commitpins, genehmigte Sources, Rechte und Budgets konfigurieren. Keine produktiven Jobs durch diesen Handoff freigeben.

Der PR ist eine überprüfbare Implementierung des remote testbaren Source-/Git-/Schema-Pfads, keine Merge- oder Produktionsfreigabe.
