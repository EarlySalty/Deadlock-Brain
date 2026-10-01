# C11 – Storage-Upgrade und belastbarer Rückweg

Stand: 26.09.2026. Branch: `codex/fix-c11-storage-upgrade`.
Basis: `origin/migration/rust-integration`, `087c522`.
Scope: ausschließlich C11. Kein Merge, Deployment, Dienstneustart oder Zugriff auf eine produktive Datenbank. Sämtliche DB-/Rollenänderungen der Tests passieren in einem neu erzeugten, privaten Scratch-Cluster.

## 1. Befund und Versionsgrenzen

`PgStore::migrate_core` führte bisher die v1- und v2-SQL-Dateien getrennt aus; beide enthielten eigene `BEGIN`/`COMMIT`. Ein Fehler in Schritt zwei konnte damit Schritt eins bereits dauerhaft hinterlassen. Die Aufrufstellen im Basisstand sind ausschließlich `brain-storage/tests/core_store.rs` und `brain-api/tests/local_pilot.rs`; Konstruktoren und Reader migrieren nicht. Das Integrationsreview beschreibt also zutreffend ein **Rechterisiko bei einer künftigen Startup-Verdrahtung**, nicht einen bereits vorhandenen produktiven Rust-Core-Service.

Das historische SQL-v1 besitzt genau drei Tabellen: `source_record_revisions`, `source_record_heads`, `corpus_releases_v1`. Quellenidentität, ACL, Provenienz, Inhalt und Tombstone liegen in den Records; eine eigene Quellen-, Fact-, Lease- oder Checkpoint-Tabelle gab es nicht.

Wichtig: SQL-Schemaversion, `CONTRACT_VERSION` und Rust-Typname sind nicht dasselbe. Der Typ `SourceRecordV2` existierte bereits mit `CONTRACT_VERSION = brain.v1`.

Es gibt zwei historische v1-Release-Formate:

| Historischer Stand | Bedeutung | Upgradeverhalten |
|---|---|---|
| `1db885d3b2a4a200c80385d3602ce74c17f8562b` | Nur maximale Revision je Quelle | **Atomarer Abbruch.** Daraus lassen sich keine eindeutigen historischen Dokumentpins rekonstruieren. |
| `f18502f46fcb403fa5998c550a3191efc29e9544` | Korrigierter v1-Vertrag mit konkreter Revision je Dokument, unverändertes SQL-v1 | Reproduzierbarer, datenbewahrender Upgradepfad auf das aktuelle v2-Schema. |
| Integrationsstand vor C11 | v2-Jobs/Checkpoints vorhanden, noch ohne Versionsmarker | Separater Migrator prüft den Bestand und registriert das kompatible Schema. |

Für früheste Source-Maximum-Releases darf niemand heutige Heads, Zeitstempel oder pro Dokument irgendeine höchste Revision zu einem vermeintlich historischen Release erklären. Nötig ist ein unabhängig belegtes Originalmanifest mit konkreten Dokumentpins und eine separat geprüfte Reparatur. C11 enthält bewusst keinen geratenen Backfill und behauptet nicht, diese verlorene Information rekonstruieren zu können.

## 2. Implementierung

`brain-storage/src/schema.rs` enthält die einzige Implementierung von `migrate_core` und den neuen Service-Preflight `check_core_schema`.

Der Migrator verwendet eine gemeinsame SQLx-Transaktion für beide unveränderten historischen SQL-Dateien, Datenvalidierung und Versionsmarker. Die äußeren Transaktionswrapper werden streng geprüft und nicht innerhalb der SQLx-Transaktion ausgeführt. Ein transaktionales Advisory Lock serialisiert parallele Migratoren; Tabellenlocks verhindern konkurrierende Änderungen während der Bestandsprüfung. Lock-Timeout: 5 Sekunden; Statement-Timeout: 60 Sekunden. Das sind Wartungsgrenzen pro Datenbankstatement, kein Versprechen über die Gesamtlaufzeit einer großen Migration.

Die Bestandsprüfung liest Records und Releases in begrenzten Keyset-Seiten. Sie prüft die aktuellen Decoder/Validatoren, Übereinstimmung von ID/Revision/Hash/Tombstone zwischen Spalten und JSON, Head/History-Konsistenz und Existenz aller Release-Pins samt aktuellen ACL-Heads. Inkompatible Daten führen zum Abbruch statt zu stiller Umdeutung. Beliebige historische Hashalgorithmen werden nicht ungefragt neu berechnet; die Test-Fixture hat explizite SHA-256-Inhaltshashes und prüft diese unabhängig.

**Keine bestehenden Records, Heads, Releases, ACLs, Checkpoints oder Leases werden umgeschrieben.** Das neue, separat versionierte DDL `scripts/migrations/2026-09-26-brain-core-compatibility-v2.sql` legt ausschließlich die Metadatentabelle an; der Migrator registriert die Version erst nach erfolgreicher Bestandsprüfung. Der zusätzliche Singleton-Marker `brain.core_schema_version` speichert SQL-Version `2`, Store-Vertrag `brain.store.v2` und Installationszeitpunkt. Ein Wiederholungslauf verändert auch diesen Zeitpunkt nicht. Eine unbekannte/neue Version wird nicht überschrieben.

## 3. Migration strikt getrennt vom Service

Neu: eigenständiges Binary `brain-migrate` in der vorhandenen Crate `brain-storage`, keine zweite Storage-/Contract-Crate.

```sh
cargo build --manifest-path rust/Cargo.toml --locked -p brain-storage --bin brain-migrate
brain-migrate check --config /path/to/local-postgres.json
brain-migrate up --config /path/to/local-postgres.json
```

Beispiel einer **nicht geheimen** lokalen Konfiguration:

```json
{
  "socket": "/var/run/postgresql",
  "port": 5432,
  "database": "REPLACE_WITH_EXPLICIT_DATABASE",
  "user": "REPLACE_WITH_MIGRATION_ROLE"
}
```

Alle Verbindungskoordinaten sind Pflichtfelder. Nur absolute Unix-Sockets; keine DSN, kein Passwortfeld, kein TCP-Fallback. Die administrative Ausführung setzt lokale Peer-Authentifizierung voraus; sie liest keine `.pgpass`. Fehlerausgaben enthalten keine DB-Inhalte oder Verbindungsgeheimnisse. Dies ist kein neuer Secret-Verteilungsweg.

`up` benötigt einen separat ausgeführten Schema-Owner/Migrationsaccount mit den nötigen DDL-Rechten. **Die Service-Rolle braucht weder Schema-Ownership noch CREATE/ALTER.** Sie benötigt `USAGE` auf `brain`, SELECT auf dem Versionsmarker und den benötigten Tabellen sowie fachlich notwendige DML-Rechte. Keine Mitgliedschaft in der Owner-Rolle, keine Schreibrechte auf den Versionsmarker, keine pauschalen Rechte auf zukünftige Tabellen. Rollen-/Grant-Verwaltung bleibt ein expliziter administrativer Deployment-Schritt; C11 ändert keine produktiven Rollen.

Integration in C1/Composition Root:

```rust
let store = brain_storage::PgStore::new(pool);
store.check_core_schema().await?; // SELECT-only, vor Listener/Jobs
// Niemals store.migrate_core() im Service-Startup oder in einem Reader aufrufen.
```

`check_core_schema` läuft in einer READ-ONLY-Transaktion, validiert den Marker und plant einen schemaqualifizierten Spaltenprobe-SELECT ohne Corpus-Scan. Fehlendes, unlesbares, beschädigtes oder unbekanntes Schema bedeutet Startup-Abbruch mit Hinweis auf den separaten Migrator. Der Check ist kein vollständiger Integritätsaudit aller Zeilen und kein Ersatz für die Upgrade-Prüfung. Bestehende unmarkierte v2-Installationen brauchen den einmaligen administrativen Lauf. Die beiden vorhandenen Testaufrufe von `migrate_core` bleiben absichtlich privilegierte Scratch-Setup-Schritte. Der parallele C1-Branch wird hier nicht verändert.

## 4. Reproduzierbare Scratch-Probe

```sh
bash scripts/test_brain_storage_upgrade.sh /path/to/cargo
```

Voraussetzungen: nicht-root Testnutzer, Workspace-Rust-Toolchain, gelockte Cargo-Abhängigkeiten lokal verfügbar, PostgreSQL-Binaries über `pg_config --bindir`.

Der Runner akzeptiert **keine DB-Adresse/DSN**. Er erzeugt `/tmp/brain-c11.<Zufall>/data` und einen eigenen Unix-Socket, deaktiviert TCP und verwirft ambient PostgreSQL-Verbindungsvariablen. Der Rust-Test prüft absoluten/kanonischen Scratch-Pfad, Marker, Datenverzeichnis, Rollenidentität und fehlende Netzwerklistener **vor** Datenbank- oder Rollenänderungen. Nur neu angelegte `brain_c11_*`-Datenbanken werden verwendet. Aufräumen betrifft ausschließlich diesen Aufruf; keine vorhandenen Cluster werden übernommen oder gestoppt.

Die eingefrorene Fixture unter `rust/crates/brain-storage/tests/fixtures/storage_v1/` enthält die bytegleiche historische SQL-Datei samt Commitbeleg und Prüfsummen. Das Seeding verwendet die statische alte JSON-Repräsentation und direkte parametrisierte INSERTs, nicht heutige Store-Schreibmethoden oder eine Serialisierung aktueller Rust-Structs.

Geprüft werden:

- Drei Quellen, elf historische Revisionen, sieben Heads und zwei unterschiedlich gepinnte Releases; IDs, sämtliche ursprünglichen Zeilenwerte inklusive Zeitstempel, UTF-8-Inhaltshashes, Provenienz, Patch/Mode/Gültigkeit und Metadaten unverändert.
- Öffentliche, interne und private ACLs, mehrere gleichzeitig nötige Scopes, nach Veröffentlichung verschärfte ACLs, private Daten ohne Scope-Freigabe und ein explizites Egress-Verbot. Tombstones bleiben wirksam auch auf älteren Releases.
- Tatsächlicher Aufruf des Migrationsbinaries, read-only Preflight vor/nach Upgrade, native `LocalPgReader`-Snapshots und Conversation-Ownership.
- Neue Checkpoint-/Job-Tabellen zunächst leer, weil echtes v1 sie nicht hatte; anschließend echte v2-Checkpoint-Commits, Resume, Replay-Receipt und Monotonie der Writer-Fences. Wiederholte und parallele Upgrades dürfen weder Checkpoints noch Leases oder Zeitstempel zurücksetzen.
- Frische Scratch-Dienstrolle ohne DDL-/Owner-/CREATE-/TEMP-Rechte: Schema-Check, Lesen und normale Schreiboperationen funktionieren; CREATE, ALTER, Schreiben des Versionsmarkers und Migration als Service-Rolle werden abgewiesen.
- Echtes Custom-Format-`pg_dump` des v1-Stands und `pg_restore --single-transaction --exit-on-error` in eine neue Datenbank; vollständiger Vergleich mit dem B0-Bestand statt eines Rebuilds aus Quellen.
- Atomare Fehlerpfade für frühe Source-Maximum-Releases, fehlende gepinnte Revisionen, widersprüchliche Record-Spalten/JSON, teilweise vorhandenes defektes v2-DDL (auch vor erstmaligem v1-DDL), blockierende Writer und eine unbekannte künftige Version.

Die vorhandene Storage-Matrix in `rust-core-verification.yml` führt diese Probe zusätzlich zum bisherigen Core-Postgres-Vertragstest aus. Das vorhandene Storage-Artefakt enthält `c11-storage-upgrade.log`; kein neuer unabhängiger Gate-/Deploypfad.

## 5. Rückweg: forward-only, kein blindes Down

Es gibt **keine Down-Migration**. Tabellen für Checkpoints, Replay-Receipts, Writer-Fences und Conversation-Ownership einfach zu entfernen würde genau die neuen Sicherheits-/Resume-Zusagen verlieren. Das früheste Source-Maximum-Releaseformat lässt sich aus präzisen Pins außerdem nicht verlustfrei als gleichbedeutender alter Vertrag herstellen.

### Compatibility Window

| Binary/DB-Kombination | Erlaubt? |
|---|---|
| Historisches v1-Binary + ursprüngliches/restauriertes passendes v1 | Nur im separat freigegebenen Rückweg, mit passendem Config-/Raw-Stand und gefencten v2-Writern. |
| Aktuelles v2-Service-Binary + v1 oder unmarkiertes v2 | **Nein.** Zuerst separat administrativ migrieren, dann read-only Preflight. |
| Aktuelles v2-Binary + Marker `2 / brain.store.v2` | Ja, innerhalb des geprüften Vertrags und mit passenden DML-Rechten. |
| Altes v1-Binary + neues v2-Schema | **Betrieblich verboten**, auch wenn einzelne Tabellenlesezugriffe technisch noch funktionieren. Kein getesteter Mixed-Version-Servicebetrieb; alte Writer kennen keine v2-Fences. |
| Aktuelles Binary + unbekannte höhere Schema-/Store-Version | **Nein**, Check und Migration müssen abbrechen. |

Das Fenster ist bewusst **ein kompatibler Store-Vertrag**, keine pauschale Zusage „N−1-Binaries funktionieren“. Das neue Markerfeld kann alte Binaries nicht nachträglich zum Beachten einer Grenze zwingen. Dafür ist externes Writer-Fencing zwingend.

### Backup-/Restore-Grenze

**B0 ist der notwendige v1-Restorepunkt:** nach Stoppen und Fencing sämtlicher Writer, vor erstem Upgrade-DDL und vor erstem v2-Write. Dazu gehören ein überprüft restaurierbarer DB-Dump, exakter alter Binary-/Commitstand, nicht geheime Config, Quell-/Parserpins und benötigte unveränderliche Raw-Artefakte. Externe v1-Checkpointdateien, sofern eine konkrete Installation welche besitzt, gehören ebenfalls zum gemeinsam eingefrorenen Stand. Das hier historische SQL-v1 hatte keine DB-Checkpoints.

Ein `pg_dump` sichert eine Datenbank, nicht clusterweite Rollen/Tablespaces. Deren autorisierte Wiederherstellung, Extensions, Eigentümer/ACLs, Betriebskonfiguration und Secret-Zugriffswege müssen im Deployment-Runbook gesondert vorhanden sein. Die Scratch-Probe benutzt eine gemeinsame reine Test-Owner-Rolle; sie ist kein Produktionsrollen-Restoretest.

Vor Freigabe von v2-Writes kann B0 den alten fachlichen Stand vollständig zurückbringen. **Nach angenommenen v2-Writes ist Restore auf B0 kein verlustfreies Rollback.** Neu angenommene Writes/Checkpoints fehlen. Vor allem spätere ACL-Widerrufe und Tombstones dürfen nicht durch Freigabe eines älteren Backups wieder öffentlich sichtbar werden. Erst diese Deltas aus einem überprüften dauerhaften Journal/Raw-Stand korrekt nachführen und prüfen, oder den Rückweg weiter gesperrt lassen und vorwärts reparieren. Kein blindes Vermischen neuer Tabellen mit alten Releases.

### Operativer Ablauf für einen später separat freigegebenen Cutover/Rollback

1. Alle Feeder, Timer, Replay-/Wiki-/Source-Writer, Admin-Werkzeuge und Service-Schreibpfade inventarisieren, stoppen und extern fencen. Alte v1-Service-/Reader-Prozesse ebenfalls vor dem Upgrade aus dem Traffic nehmen und beenden. Neue Sessions verhindern und vorhandene Sessions/Transaktionen drainen oder gezielt beenden. `NOLOGIN` allein beendet bestehende Sessions nicht. Keine zwei Writer-Generationen gleichzeitig zulassen.
2. B0 mitsamt Binary/Config-/Quellstand sichern und einen Restore in eine neue isolierte Datenbank mit IDs/Hashes/ACLs/Tombstones/Release-Pins prüfen. Erst dann den Owner-Migrator ausführen. DDL-Fehler: Transaktion bricht ab, Writer bleiben bis zur Prüfung gefenct.
3. Neues Binary mit reinen Service-Rechten und `check_core_schema` prüfen; zuerst keine Writer/Traffic freigeben. Für Freigabe bleibt der übergeordnete Integrations-/Staging-Prozess zuständig.
4. Bei Rückweg zuerst v2 vollständig fencen und drainen. Den aktuellen v2-Stand für Diagnose und mögliche Delta-Reconciliation zusätzlich sichern. Nicht im bestehenden v2-Schema Tabellen entfernen.
5. B0 in eine **neue** Datenbank restaurieren, passenden alten Binary-/Config-/Raw-Stand zuordnen. Neuere sicherheitsrelevante Widerrufe/Löschungen und andere angenommene Writes vollständig berücksichtigen oder gesperrt bleiben. Wiederhergestellte Daten und Rechte erneut prüfen.
6. Erst nach separater fachlicher/Betriebsfreigabe den gewählten alten Writer zulassen. Kein automatisches Entfencen beider Versionen. Bei unvollständigen Deltas ist Forward-Fix der sichere Standard.

Die Scratch-Probe drainiert ihre Dienstverbindungen, sperrt neue Logins/Schreibrechte ausschließlich der Testrolle und prüft den Restore bei weiterhin gesperrter Dienstrolle. Es wird anschließend kein Dienst gestartet.

## 6. Testnachweis

Lokal am 26.09.2026 ausgeführt, mit Cargo/Rust-Toolchain 1.97.1, PostgreSQL-Server 16.14 und `pg_dump`/`pg_restore` 16.15. Der vollständige Workspace-Test lief inklusive Doc-Tests. Anschließend wurden am finalen Code nochmals Workspace-Format/Compile/Clippy/Build, die Storage-Tests und beide PostgreSQL-Proben mit `env -i`, isoliertem Test-HOME, explizitem Cargo-/Rustup-Cache und ohne übernommene DB-/Secret-Umgebung ausgeführt.

| Prüfung | Ergebnis |
|---|---|
| `cargo fmt --manifest-path rust/Cargo.toml --all -- --check` | PASS |
| `cargo check --manifest-path rust/Cargo.toml --workspace --all-targets --locked --offline --jobs 2` | PASS |
| `cargo clippy --manifest-path rust/Cargo.toml --workspace --all-targets --locked --offline --jobs 2 -- -D warnings` | PASS |
| `cargo test --manifest-path rust/Cargo.toml --workspace --locked --offline --jobs 2` | **712 bestanden, 0 fehlgeschlagen, 68 ignoriert** |
| `cargo test --manifest-path rust/Cargo.toml -p brain-storage --all-targets --locked --offline --jobs 2` | **13 bestanden, 0 fehlgeschlagen, 2 PostgreSQL-Opt-in-Tests ignoriert**; diese anschließend separat aktiviert |
| `cargo build --manifest-path rust/Cargo.toml --workspace --locked --offline --jobs 2` | PASS, Debug-Build im eigenen Worktree, kein Deployment |
| `bash scripts/test_brain_storage_upgrade.sh` | **PASS**, echter v1→v2-Lauf, Wiederholung/Parallelität, unmarkiertes v2 mit Checkpoints, Backup/Restore, Rechteprüfung und negative Proben; finaler Testlauf 10,15 Sekunden ohne Buildzeit |
| `bash scripts/test_brain_core_postgres.sh` | **PASS**, vorhandener Atomicity-/Fence-/Release-/Restart-Vertrag bleibt grün |
| `shellcheck scripts/test_brain_storage_upgrade.sh`, `bash -n`, `git diff --check` | PASS |

Die 68 standardmäßig ignorierten Workspace-Tests sind **nicht** als bestanden gewertet; darunter liegen die beiden hier anschließend ausdrücklich ausgeführten PostgreSQL-Tests. Lokale Rohlogs liegen im ignorierten `.core-test-logs/`: `c11-workspace-test.log`, `c11-final-status.tsv`, `c11-final-{fmt,check,clippy,storage-tests,build,upgrade,postgres-regression,shellcheck,diff}.log`. Beide Scratch-Server wurden ordnungsgemäß beendet; der C11-Runner hat seine temporären Cluster entfernt. Die eingefrorene SQL-Fixture wurde zusätzlich direkt mit `git show f18502f:…` verglichen: identischer SHA-256 `e4d0f631c9b2fbd1b0883c178d93acbf7c29efeabf4da819eeb0a6375d8dc79b`.

Ein separater Release-Build, Hosted-CI und Produktiv-/Staging-Prüfungen sind nicht Teil dieser lokalen Nachweise.

## 7. Verbleibende Grenzen

Diese Probe ist reproduzierbarer PostgreSQL-Storage-/Upgrade-Nachweis mit historisch korrekter synthetischer Fixture, **kein Export/Upgrade einer Produktionsdatenbank**. Ein echter Staging-Restore mit repräsentativem Datenvolumen, Umgebung, Rollen und dem tatsächlich ausgewählten Altbinary bleibt vor dem Cutover erforderlich. Früheste Source-Maximum-Releases brauchen explizite Rekonstruktion aus belastbarer Evidenz statt eines automatischen Datenraten-Schritts. C1 muss den neuen read-only Preflight konsumieren; C11 fasst seinen parallelen Worktree nicht an.

PostgreSQL-Referenzen: [LOCK, Version 16](https://www.postgresql.org/docs/16/sql-lock.html) (Transaktionsdauer/Writer-Interaktion) und [pg_dump, Version 16](https://www.postgresql.org/docs/16/app-pgdump.html) (konsistentes Einzel-DB-Backup, Grenzen bei clusterweiten Objekten).
