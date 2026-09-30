status: aktiv, konkrete Startvoraussetzungen statisch geprüft; keine DB-Verbindung
Datum: 2026-09-30

# Ergänzung: tatsächlicher PostgreSQL-Vertrag für Serve

Produktbasis `9a29b81d230c01e5c03423cc34ba34c1074eab69`. Ergänzt SERVE-VORBEREITUNG.md, kein Schema-/Rechte-/Dienständerungsauftrag.

## Socket und Identität

Die lesende Unix-Socketaufnahme zeigt `/run/deadlock-brain-postgresql/.s.PGSQL.5446`. Separat existiert der normale Cluster unter `/var/run/postgresql/.s.PGSQL.5432`. Dies bestätigt den dedizierten Socket als konkreten Kandidaten, nicht Datenbankname, Rechte oder CorpusRelease. Kein Verbindungsversuch durchgeführt. Die Serve-Beispielconfig mit Port 5432 darf nicht ungeprüft auf Produktion übertragen werden.

Die vorhandene Brain-PG-Systemunit läuft als `deadlock-brain-pg`. Der spätere Serve-Dienst braucht eine eigene nachgewiesene Datenbankrolle. Beim bestehenden Peer-Modus muss die tatsächliche OS-Identität des Serveprozesses zur DB-Rolle beziehungsweise vorhandenen pg_ident-Zuordnung passen. Die Userunit-Vorlage mit Beispielrolle brain_serve belegt diese Zuordnung nicht. Alternativ unterstützt der vorhandene Code Passwortauthentisierung über eine bestehende Infisical-Secretreferenz; das ist noch kein bestätigter produktiver Passwortstart. Kein neues Authverfahren bauen und keine Secrets zur Diagnose auslesen.

## Schema und konkrete Rechte

`rust/crates/brain-storage/src/local_pg_reader.rs:381-403` prüft genau eine Versionszeile und die Tabellenform. Erwartete Konstanten:

- `CORE_SCHEMA_VERSION = 2`, `brain-storage/src/schema.rs:7`.
- `STORE_VERSION = "brain.store.v2"`, `brain-contracts/src/store.rs:9`.

Die Shape-Prüfung `brain-storage/src/schema.rs:63-75` benötigt die bestehenden Record-/Head-/Release-/Job-/Checkpoint-/Ownershiptabellen. Ein leerer neu gestarteter PostgreSQL-Dienst erfüllt diesen Vertrag nicht.

Wichtig: Serve ist **nicht vollständig schreibgeschützt**. `local_pg_reader.rs:406-422` verlangt SELECT und INSERT auf `brain.conversation_owners_v1`. `:515-532` beansprucht die Conversation-ID mit `INSERT ... ON CONFLICT DO NOTHING` und prüft danach den bestehenden Actor. Eine reine SELECT-Rolle reicht nicht für Startup/Antwortpfad. Diese begrenzte Ownership-Schreibberechtigung ersetzt keine Ingest-/Migrationsrechte.

Zusätzlich benötigt der Reader die bestehenden Leserechte für Schema, Release, Quellenrevisionen und aktuelle Heads. `read_snapshot` startet eine Repeatable-Read-Transaktion mit read_only=true (`:462-469`), während die Ownership-Transaktion separat schreibt. Vor Freigabe konkrete Least-Privilege-Grants nachweisen; kein pauschales CREATE/Superuser und keine produktive Probe hier.

`PgCoreStore::migrate_core` ist explizite privilegierte Wartung (`schema.rs:93-145`), nicht Teil des Serve-Startpfads. Die Funktion selbst verlangt externes Writer-Fencing; ihre Transaktionssperren sichern keinen späteren Cutover. Weder Starten der Unit noch grüne Readiness migriert automatisch das Schema oder sperrt alte Writer.

## Unit-Sandbox und Infisical-Socket

Hostversion read-only bestätigt: systemd 255.4. Die Userunit-Vorlage setzt PrivateTmp=yes, PrivateDevices=yes und ProtectSystem=strict (`service/systemd/brain-serve.service.example:25-28`). Der bestehende Infisical-Leser verlangt Socketbesitzer UID 0 (`deadlock-brain-core/src/pg_secrets.rs:89`); der mitgelieferte Transport prüft Dateibesitzer und Verzeichnisse (`rust/vendor/uplink-infisical-transport/src/lib.rs:14-56`).

Die tatsächliche spätere Unit muss diesen geschützten Socket aus ihrer eigenen Namespace-/Mountsicht erreichen und UID 0 korrekt sehen. Der bekannte Hostfall bei systemd-Userunits mit Mount-Sandbox ist deshalb vor einer Übernahme der Beispielhärtung zu prüfen: tatsächliche uid_map und sichtbarer Socketbesitzer im zugeteilten Dienstkontext, nicht nur erfolgreiche Prüfung in der Agentenshell. Keine Sandboxoption jetzt pauschal abschalten; eine erfolgreiche Prüfung dieses noch nicht installierten Dienstes wird nicht behauptet. Dies ist ein konkreter Startnachweis, kein neuer Secretweg.

## Wirkung für den Rückweg und die Tests

Ein späterer echter Answer-Smoke kann eine Ownershipzeile schreiben und ist daher keine pauschal reine Leseprobe. Wegwerfidentität/-Conversation und zulässiges Ziel müssen zum gesondert zugeteilten Livebeweis gehören. Keine echten Nutzerkonten umkonfigurieren. Aktuelle Ownership-/ACL-/Tombstonezustände beim Rollback nicht mit einem alten Datenbackup überschreiben.

DB-/Auth-/Ownershipfälle bleiben getrennte echte Nachweise. Ein erfolgreicher Standard-Unitlauf oder Clippy-Exit ersetzt weder diesen Rollenvertrag noch eine Migration, einen Restore oder Writer-Fencing.

## Konkrete Grenze der vorhandenen DB-Harnesses

Am 30.09. statisch gelesen, nicht gestartet:

- `scripts/test_brain_serve.sh:14-53` erzeugt einen privaten Wegwerfcluster, bindet nur Unix-Socket, Port 55439, max_connections=12 und shared_buffers=16MB. Peer-Zuordnung ist auf das ausführende unprivilegierte Konto und Testrollen begrenzt; SCRAM ist ein separater Fixturepfad. Cleanup stoppt genau den erzeugten Cluster, bei Fehler bleibt das Verzeichnis mit Log erhalten. Es startet weder den produktiven Cluster noch systemd.
- `scripts/test_brain_storage_upgrade.sh:21-58` erzeugt einen anderen privaten Cluster, Unix-Socket und Port 55441, max_connections=24 und shared_buffers=16MB. Er prüft Upgrade, Restore und Least-Privilege. Cleanup entfernt das temporäre Verzeichnis auch bei Testfehler; sein vollständiger äußerer Log muss vor einem späteren Lauf erhalten werden.
- Beide Skripte starten danach echtes Cargo, setzen intern `CARGO_TARGET_DIR=$ROOT/rust/target`, `CARGO_BUILD_JOBS=2`, `SQLX_OFFLINE=true` und Test-DB-Koordinaten. Serve-Harness `:55-84`, Upgrade-Harness `:59-64`. Ein von außen gesetzter vorhandener Targetcache würde dadurch überschrieben. Sie sind deshalb **nicht unverändert für den aktuellen Ein-Job-/Cache-Reuse-Vertrag ausführbar**. Die aktuelle Workspace-Zuteilung deckt keinen ihrer direkten Aufrufe ab.
- Die Testnamen sind gezielt gefiltert, kein Workspace-`--ignored`. Die vier Pilotphasen und der Atomicity-/Fencing-Vertrag werden durch diese beiden Skripte trotzdem nicht vollständig abgedeckt; dafür bleiben eigene konkrete Abläufe erforderlich.

Nächste Vorbereitung: den vorhandenen Testweg an den bestehenden Cache und einen Job binden, ohne neue Produktkonfiguration, neue Modelle oder höhere DB-Grenzen. Erst nach belegter Quellvorbereitung einen exakten separaten DB-/Prozesslauf anmelden. Bis dahin keine DB-Harness-Ausführung, keine neue Infrastruktur und keine Übertragung des Standardtest-Ergebnisses auf diese Fälle.
