# Eigene PostgreSQL-Instanz für Deadlock Brain

Stand: 29.09.2026. Die Angaben zur dedizierten Staging-Instanz und zu früheren Prüfständen bleiben an ihren jeweiligen Messzeitpunkt gebunden. Die finale Verifikation auf `022f8a981c2164f6d8d4302bae2194e100c4f65c` lief separat mit isolierten Wegwerf-Clustern und belegt keine Produktionsfreigabe.
Alle Hostmessungen zur dedizierten Instanz stammen vom Host `v50671`. Secretwerte stehen nirgends in diesem Dokument.

## Abschlussnachweis und Geltungsbereich

| Marker | Wert |
|---|---|
| BRAIN_DB_ISOLATED | JA, Infrastrukturprüfung der dedizierten Instanz vom 26.09.2026 |
| BRAIN_DATA_MIGRATION_VERIFIED | JA, Snapshot vom 26.09.2026: Archivkopie `brain_legacy`, 49 Tabellen, 653 476 Zeilen, Zeilenzahl und md5 je Tabelle gleich |
| DB_POOLING_VERIFIED | JA, Prozess-E2E auf `022f8a9`; beobachtetes Poolmaximum 4 |
| 600_REQUEST_TEST_PASSED | JA, `022f8a9`: 600/600 Antworten bei 8, 16 und 32 Workern, Poolmaximum 4 |
| FINAL_022F8A9_WORKSPACE_GATES | JA, Format, Clippy, Tests und Release-Build jeweils Exit 0 |
| FINAL_022F8A9_WORKSPACE_TESTS | 1.011 passed, 0 failed, 75 ignored, 0 filtered |
| FINAL_022F8A9_TARGETED_IGNORED_TESTS | 8 bestanden, separat geprüft und nicht zur Workspace-Summe addiert |
| PRODUCTION_DB_CUTOVER_READY | NEIN |
| PRODUCTION_CUTOVER_READY | NEIN |

Die folgenden Konfigurations-, Rollen-, Backup- und Restore-Details beschreiben den dokumentierten Staging-Stand. Die finale Verifikation nutzte temporäre PostgreSQL-16.15-Cluster und prüfte keine Produktionsdatenbank. Historische Werte 18/18 und ältere 600-Request-Messungen sind nicht die aktuellen Werte für `022f8a9`.

## Instanz

| Eigenschaft | Wert |
|---|---|
| PostgreSQL | 16.14 (Ubuntu-Paket `postgresql-16`, dieselben Binaries wie DL-Main, eigener Prozess) |
| OS-User | `deadlock-brain-pg` (Systemnutzer, Shell `nologin`), Clientgruppe `deadlock-brain-db` |
| PGDATA | `/var/lib/deadlock-brain/postgresql` (0700, `deadlock-brain-pg`), Data Checksums an |
| Konfiguration | `/etc/deadlock-brain/postgresql/{postgresql.conf,pg_hba.conf,pg_ident.conf}` (0640 root:deadlock-brain-pg), Quelle `ops/brain-postgres/` |
| Socket | `/run/deadlock-brain-postgresql/.s.PGSQL.5446` (Verzeichnis 0755, Socket 0770 Gruppe `deadlock-brain-db`) |
| Port | 5446 (frei geprüft; DL-Main 5432, rs-relay 5433/5434 unberührt) |
| TCP | aus, `listen_addresses = ''`; zusätzlich `host ... reject` in `pg_hba.conf` |
| systemd | `deadlock-brain-postgresql.service` (System-Unit, `ProtectSystem=strict`, `ProtectHome`, `NoNewPrivileges`, kein `Environment=`) |
| WAL | im eigenen PGDATA (`pg_wal`), `wal_level=replica`, keine Archivierung |
| Logs | `/var/log/deadlock-brain/postgresql/postgresql-<Tag>.log` (0600), Verbindungen, DDL, Lock-Waits und Statements ab 1 s |
| Backup | `/var/backups/deadlock-brain/postgresql/brain-<UTC>/` (0700), `deadlock-brain-postgresql-backup.timer` täglich 02:40, 14 Stände |
| Einrichtung | `ops/brain-postgres/provision.sh` (idempotent, root), Passwörter `set-role-passwords.sh` |

DL-Main (`postgresql@16-main`, `/var/lib/postgresql/16/main`, Port 5432) wurde nicht verändert: keine Rolle, keine Datenbank, keine Konfiguration, keine Erweiterung, kein Neustart. Gelesen wurde dort nur per `pg_dump`/`SELECT` in einem Snapshot.

Startwerte für Staging, kein Produktions-SLO: `max_connections = 40`, `superuser_reserved_connections = 3`, `shared_buffers = 512MB`, `work_mem = 8MB`, `idle_in_transaction_session_timeout = 60s`.

## Datenbanken

| Datenbank | Owner | Inhalt |
|---|---|---|
| `brain` | `brain_migrate` | Zielbestand: Kernschema v2 (`brain.*`, leer) und Archivkopie `brain_legacy.*` |
| `brain_pilot` | `brain_migrate` | Wegwerf-DB für Pilot, Last und Betriebsprüfungen; wird bei jedem Pilotlauf neu angelegt |
| `brain_pilot_legacy` | `brain_migrate` | Wegwerf-DB für den Legacy-Kernimport (`scripts/run_isolated_legacy_import.sh`), echte Altdaten als `SourceRecordV2`; wird bei jedem Lauf neu angelegt |

Erweiterungen in beiden Datenbanken: nur `plpgsql`. Kein `postgres_fdw`, kein `dblink`, kein Foreign Server, kein User Mapping. `pgcrypto`, `vector` und `timescaledb` aus DL-Main werden nicht gebraucht (siehe Migrationsbericht).

## Rollen und Grants

Alle Rollen: `NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS NOINHERIT`. Keine Rolle existiert in DL-Main, keine DL-Main-Rolle existiert hier.

| Rolle | Anmeldung | Verbindungsdeckel | Rechte |
|---|---|---|---|
| `brain_migrate` | nur Peer über `pg_ident` (`nathanael`, `deadlock-brain-pg`) | 3 | Owner von DB `brain`, Schemas `brain`, `brain_legacy`, `public` und allen Tabellen; einziger Weg für `brain-migrate up` |
| `brain_ingest` | SCRAM, Secret `BRAIN_PG_INGEST_PASSWORD` | 4 | SELECT auf alle Kerntabellen; INSERT auf `source_record_revisions`, `corpus_releases_v1`; INSERT/UPDATE auf `source_record_heads`, `source_jobs_v1`, `source_checkpoints_v1`; kein DELETE, kein TRUNCATE, kein Schreibrecht auf `core_schema_version` und `conversation_owners_v1` |
| `brain_service` | SCRAM, Secret `BRAIN_PG_SERVICE_PASSWORD` | 16 | SELECT auf die sechs Kerntabellen und den Versionsmarker; SELECT/INSERT auf `conversation_owners_v1`; sonst nichts |
| `brain_readonly` | SCRAM, Secret `BRAIN_PG_READONLY_PASSWORD` | 2 | SELECT auf `brain` und `brain_legacy`, `default_transaction_read_only = on` |
| `deadlock-brain-pg` | nur Peer als gleichnamiger OS-User | Reserve | Superuser der Instanz (Administration, Backup) |

`PUBLIC` hat auf Datenbank, `public`-Schema und `brain`-Schema keine Rechte (kein CONNECT, kein TEMP, kein CREATE). `brain_ingest` darf `conversation_owners_v1` lesen, weil `check_core_schema()` alle sechs Kerntabellen mit `LIMIT 0` prüft; schreiben darf es dort nicht. Grants: `ops/brain-postgres/grants.sql`, nach jeder Migration erneut anwenden.

Nachweis `ops/brain-postgres/verify-isolation.sh`: 54 von 54 Prüfungen bestanden. Für jede der drei Laufzeitrollen scheitern CREATE TABLE, ALTER TABLE, DROP TABLE, TRUNCATE, CREATE SCHEMA, CREATE in `public`, TEMP, CREATE EXTENSION, CREATE ROLE, Schreiben des Versionsmarkers, Verbindung zur DB `postgres`, Verbindung zu DL-Main über Socket und über `127.0.0.1:5432`. Die Fehlermeldungen sind Rechtefehler (`permission denied for schema brain`, `must be owner of table`), keine Anmeldefehler; SELECT auf Releases gelingt mit denselben Zugangsdaten. `brain_service` hat nachweislich weder DDL-, TEMP-, Owner- noch Rollenrechte und ist kein Mitglied von `brain_migrate`.

## Schema

Schema-Version `2`, Store-Contract `brain.store.v2`, aufgebaut ausschließlich mit `brain-migrate up` als `brain_migrate` (Peer). `brain-migrate check` meldet vorher `core schema missing`, danach `compatible (read-only check)`.

`brain-serve` ruft beim Start `check_core_schema()` auf und bricht bei fehlendem oder falschem Schema mit `core_schema_incompatible` ab (Integrationsfix in `rust/crates/brain-serve/src/service.rs`). `migrate_core()` wird von `brain-serve` nie aufgerufen; Nachweis: Start gegen eine leere Datenbank endet fail-closed, danach existiert dort weder Schema `brain` noch der Versionsmarker.

## Connection-Architektur und Pool

Prüfstand `ed06e13` nach PR #49: `LocalPgReader` war ein gemeinsamer, hart begrenzter Pool (`postgres.max_connections` 1 bis 64, Service-Default 4, begrenzte Acquire-Wartezeit `postgres_pool_wait_ms`, Erschöpfung als `AnswerStatus::Unavailable`). Readiness, Snapshot-, Head-, Retrieval-, Evidenz- und Ownership-Zugriffe teilten ihn. Das ist historische Evidenz und kein erneuter Abschlussnachweis auf `022f8a9`; siehe `FINAL_LOCAL_INTEGRATION_REVIEW.md`, Abschnitte 1 und 5.

Historischer Stand vor PR #49 (nur noch Herkunft, nicht mehr gültig):

- `brain-serve` baut genau einen `sqlx`-Pool (`postgres.max_connections`, Validierung 1 bis 16; im Staging 12) für Release-Snapshot, Readiness und Rechteprüfung.
- Der Anfragepfad liest aber über `LocalPgReader` (synchrones `postgres`-Crate). Jede Operation (`read_heads`, `read_snapshot`, Ownership-Claim) öffnet eine **neue** Verbindung und schließt sie wieder. Das ist die connect-per-operation-Semantik, die der Auftrag ausschließt.
- Gegen Überlast gibt es zwei Bremsen: 16 HTTP-Slots in `brain-api` (danach HTTP 429 `overloaded`) und den Verbindungsdeckel 16 der Rolle `brain_service` (danach meldet PostgreSQL `too many connections for role`, die API antwortet 503 `policy_unavailable` oder mit Status `unavailable`).
- Pool-Wartezeit ist nicht instrumentiert; es gibt keine Metrik dafür.

Damals existierte der Zweig `DB_POOLING_BACKPRESSURE` noch nicht; er kam später als PR #49 und ist integriert.

## Historische Lastmessung auf dem früheren Prüfstand (600 Requests, eigene Instanz, DB `brain_pilot`)

Ergebnis nach PR #49: bestanden. Die Zahlen in `FINAL_LOCAL_INTEGRATION_REVIEW.md`, Abschnitt 5, dokumentieren je Stufe 600 answered, Peak-Pool 4, 8 neue `brain_service`-Verbindungen und 0 „too many clients“. Diese Messung ist keine aktuelle Abnahme von `022f8a9`.

### Historische Messung vor PR #49 (nicht mehr gültig)

Aufbau: echter `brain-serve` (Release-Build), echter HTTP-Client (`BrainClient`), Loopback-Provider-Stub, Pool 12, Default-Budget, Retrieval-Limit Default. `pg_stat_activity` alle 50 ms abgetastet, CPU und Speicher aus der systemd-Cgroup der Instanz, neue Verbindungen aus dem Serverlog (`log_connections`). Skripte: `scripts/run_isolated_load.sh`, `scripts/sample_brain_pg.sh`.

| Worker | Ergebnis | Peak Client-Backends | Peak `brain_service` | davon Pool / Reader | neue `brain_service`-Verbindungen | Rollendeckel erreicht | p50 / p95 / p99 ms | PG-CPU s | PG-RAM Peak |
|---|---|---|---|---|---|---|---|---|---|
| 8 | 600 answered | 12 | 10 | 2 / 8 | 6 098 | 0 | 527 / 766 / 2 076 | 36,9 | 116 MiB |
| 16 (Lauf 1) | 586 answered, 13 HTTP-Fehler, 1 unavailable | 18 | 16 | 2 / 14 | 5 974 | 14 | 898 / 1 530 / 3 167 | 34,9 | 98 MiB |
| 16 (Lauf 2) | 600 answered | 11 | 9 | 2 / 7 | 6 098 | 0 | 688 / 1 111 / 1 366 | 33,4 | 112 MiB |
| 32 | 264 answered, 298 HTTP 429, 38 HTTP 503 | 18 | 16 | 2 / 14 | 2 776 | 38 | 62 / 1 328 / 3 648 | 19,1 | 120 MiB |

In allen Stufen: 0 × `too many clients already`, 0 × `unauthorized_evidence`, keine DB-Fehler als Rechteproblem klassifiziert. `brain-serve` selbst: rund 30 MiB RSS, 21 Threads, rund 250 CPU-Sekunden je 600 Anfragen (Retrieval ist CPU-lastig). Die 13 HTTP-Fehler im ersten 16er-Lauf wurden vor der Statuserfassung gezählt; der Wiederholungslauf mit Statuserfassung ordnet die 32er-Fehler vollständig ein (429 aus der Slot-Bremse, 503 aus dem Rollendeckel).

Bewertung damals (vor PR #49): Die Instanz bleibt stabil und `max_connections` musste nicht erhöht werden. Der Test war **nicht bestanden**, weil Verbindungen nicht wiederverwendet werden (rund 10 Neuverbindungen je Anfrage) und nur der Rollendeckel die Zahl begrenzt. Nach dem Pooling-Fix wird derselbe Lauf wiederholt; erwartet sind dann höchstens Poolgröße plus Reserve und null Neuverbindungen im eingeschwungenen Zustand.

## Backup und Restore

### Aufrufe und Snapshot-Vertrag

`backup.sh` läuft weiterhin als `deadlock-brain-pg` über `deadlock-brain-postgresql-backup.service`. Ohne Argumente bleiben Ziel `/var/backups/deadlock-brain/postgresql`, 14 Sicherungen, PostgreSQL 16, Socket `/run/deadlock-brain-postgresql`, Port 5446 und Datenbank `brain` unverändert. Die Unit und `provision.sh` benötigen keine neuen Argumente oder Umgebungsvariablen. Abweichende Werte werden gequotet als `backup.sh <Ziel> [Anzahl] [PostgreSQL-bin] [Datenbank ...]` übergeben. `BRAIN_BACKUP_*` wird nicht ausgewertet.

Pro Datenbank exportiert eine offene `REPEATABLE READ READ ONLY`-Transaktion einen Snapshot. Sowohl `pg_dump --create -Fc --snapshot=...` als auch die Fingerprint-Abfragen importieren genau diesen Snapshot. Die exportierende Verbindung bleibt bis zum Ende beider Leser offen und wird auch im Fehlerfall geschlossen und abgewartet. Spätere Schreibvorgänge verändern die Referenz nicht. Das ist ein konsistenter Snapshot **je Datenbank**, keine gemeinsame atomare Transaktion über mehrere Datenbanken oder den separat erfassten Globals-Dump. Gleichzeitige Rollen-/Schemaänderungen sind damit nicht als clusterweit atomar abgesichert.

Ein neues abgeschlossenes Backup enthält neben `<Datenbank>.dump`, `<Datenbank>.toc`, `globals.sql` und `schema_version.txt` auch `fingerprint_version.txt` mit Version `1`, die tatsächlich verwendete `fingerprint.sql` und je Dump eine nicht leere `<Datenbank>.fingerprint.txt`. Alle Dateien sind in `SHA256SUMS` enthalten. `schema_version.txt` stammt aus dem Snapshot der ersten übergebenen Datenbank, standardmäßig `brain`. Die Fingerprints prüfen Datenbank-, Schema- und Tabellen-ACLs, Eigentümer, Zeilenzahl und MD5 der Tabelleninhalte in `brain` und `brain_legacy` sowie Kernversion, Tombstones, Head-Sichtbarkeit, Releases, Checkpoints und Conversation-Owner. Sie sind kein vollständiger semantischer Vergleich aller PostgreSQL-Objekttypen; die SHA-256-Dateiprüfsummen sichern zusätzlich die gespeicherten Artefakte gegen unbeabsichtigte Veränderungen.

Alle `psql`-Aufrufe verwenden `-X` und `ON_ERROR_STOP=1`; `pg_restore` arbeitet mit `--exit-on-error`. Eine fehlende Pflichttabelle, eine fehlgeschlagene Abfrage oder ein fehlgeschlagener Dump verhindert die Veröffentlichung und jede Rotation. `KEEP`-Validierung, private Dateirechte, sichere Pfadbehandlung einschließlich `CDPATH`, atomare Veröffentlichung und Schutz unbekannter Verzeichnisse bleiben erhalten.

### Restore-Prüfung und Berichte

Der Produktionsaufruf bleibt `restore-probe.sh <Backup> [Bericht]` als root. Er validiert die vollständige Prüfsummenliste, lokale Dateinamen, eindeutige Einträge, reguläre Dateien ohne Symlinks und Fingerprint-Version, bevor SQL ausgeführt wird. Die frische Wegwerf-Instanz liegt weiterhin unter `/var/lib/deadlock-brain/restore-probe.*`, verwendet einen eigenen Unix-Socket und Port 5447 ohne TCP-Listener. **Es wird keine Verbindung zur laufenden Quelldatenbank geöffnet.**

Beim Globals-Restore wird höchstens eine exakt passende `CREATE ROLE`-Zeile für die von `initdb` gerade angelegte Bootstrap-Rolle ausgelassen. Das steht als `BOOTSTRAP_ROLE_REUSED` in `globals.out`. Alle `ALTER ROLE`-Anweisungen, Grants, anderen Rollen und jede weitere gleichnamige `CREATE ROLE`-Anweisung laufen unverändert mit Fehlerabbruch. Andere SQL-Fehler werden nicht unterdrückt.

Ohne zweites Argument entsteht ein eindeutiges Berichtsverzeichnis neben den Backups: `<Backup-Elternverzeichnis>/restore-reports/<Backupname>.XXXXXX`. Ein expliziter Berichtspfad darf weder im geprüften Backup noch in einem anderen `brain-<Zeitstempel>`-Backup liegen, auch nicht über Symlinks. Relative Pfade beziehen sich auf das Aufrufverzeichnis, unabhängig von `CDPATH`. Abgeschlossene Backups werden durch die Prüfung nicht verändert. Die Meldung `RESTORE_REPORT dir=...` nennt den tatsächlich verwendeten Berichtspfad.

Der Bericht enthält Globals- und Restore-Logs, `<Datenbank>.backup.txt`, `<Datenbank>.restored.txt`, gegebenenfalls `<Datenbank>.diff` und das Serverlog. `RESTORE_EQUAL db=...` setzt einen erfolgreichen SQL-Lauf und identische Fingerprints voraus. Inhaltsabweichungen liefern `RESTORE_DIFFERS` und einen Fehlerstatus. SQL-Fehler bleiben Fehler, auch wenn vor dem Fehler bereits gleiche Prüfzeilen ausgegeben wurden. Bei Fehlschlag bleiben die Berichte erhalten; die Wegwerf-Instanz wird gestoppt und entfernt. Kann sie nicht gestoppt werden, bleibt ihr Datenverzeichnis erhalten und das Skript meldet einen Fehler, statt Dateien einer laufenden Instanz zu löschen.

Backup und Restore sperren dasselbe geöffnete Backup-Elternverzeichnis per `flock`: Backup exklusiv, Restore gemeinsam. Ein laufender Restore kann deshalb nicht durch Retention sein Backup verlieren. Ein parallel anlaufendes Backup scheitert vor Erstellung und Rotation. Nach Ende der Prüfung fällt die Sperre weg, und die normale Retention erkennt neue Backups weiterhin. Sie akzeptiert vollständige alte Formate sowie vollständige Version-1-Metadaten mit lückenloser Prüfsummenliste; unvollständige, gemischte oder unbekannte Einträge bleiben geschützt.

### Alte Backups und Vertrauensgrenze

Alte Backups ohne gespeicherte Snapshot-Fingerprints führen ausdrücklich zu **`RESTORE_UNVERIFIED` und Exit 2**, vor Start einer Wegwerf-Instanz. Das Skript stellt daraus kein `RESTORE_EQUAL` her und ersetzt die fehlende Referenz nicht durch den heutigen Livezustand. Ein neues Backup liefert die benötigte Referenz. Alte vollständige Backups bleiben für die Retention erkennbar. Berichte, die eine frühere Skriptversion bereits in alte Backup-Verzeichnisse geschrieben hat, werden nicht automatisch gelöscht; solche Verzeichnisse bleiben vorsichtshalber geschützt, bis der Betreiber die alten Berichte außerhalb der Backups gesichert hat.

Dumps, Globals und die mitgesicherte Fingerprint-Abfrage sind ausführbares SQL. Nur vertrauenswürdige Backups verwenden. `SHA256SUMS` erkennt Beschädigungen, authentifiziert aber keinen Absender und ist keine Sandbox für fremde SQL-Dateien.

Die früher protokollierten Meldungen zu `brain-20260926T031943Z` (`brain`: 340 Prüfzeilen, `brain_pilot`: 32 Prüfzeilen) stammen noch aus dem Livevergleich ohne durchgängigen SQL-Fehlerabbruch. Die spätere Änderung von 279 auf 629 Conversation-Owner in `brain_pilot` erklärt dessen damaliges `RESTORE_DIFFERS`, belegt aber keine Abweichung vom Backup-Snapshot. Diese historischen Meldungen gelten nicht als Nachweis nach dem neuen Snapshot-Vertrag.

### Isolierte Regression und laufende CI

`bash scripts/test_brain_backup.sh` führt die bestehenden neun Testblöcke und einen zusätzlichen Block zur Metadaten-Erkennung mit PostgreSQL-Stubs ausschließlich in temporären Verzeichnissen aus. `bash scripts/test_brain_restore.sh` prüft die echte Kette in 16 Fällen mit synthetischen Daten in privaten PostgreSQL-16-Clustern. Einzelne Fälle lassen sich als Argument an denselben Runner übergeben. Der Runner verweigert root, bereinigt geerbte PostgreSQL-Verbindungsvariablen und verwendet niemals Produktionssocket, Produktionsdaten oder den alten root-/Live-Defaultpfad.

Für diesen Weg ergänzt Restore ausschließlich optionale Positionsargumente: `restore-probe.sh <Backup> [Bericht] [PostgreSQL-bin] [Scratch-Elternverzeichnis]`. Unprivilegierte Aufrufe müssen beide Testpfade ausdrücklich angeben. Das Scratch-Elternverzeichnis muss bereits existieren und einen absoluten Pfad ohne Sonderzeichen haben; Backup- und Berichtspfade dürfen weiterhin Leerzeichen und andere Sonderzeichen enthalten. Es gibt keine neuen ENV-Schalter. Der Test verwendet den bestehenden PostgreSQL-bin-Parameter des Backups für einen strikt auf den privaten Quellsocket begrenzten Client-Wrapper.

Der aktive Job `backup-regressions` / **Backup path safety** in `.github/workflows/ci.yml` läuft für Pull Requests nach `main` und bei `workflow_dispatch`. Er installiert PostgreSQL 16 und ShellCheck, prüft beide Betriebsskripte und beide Runner, führt die Retention-Regression und anschließend die echten Restore-Fälle als unprivilegierter Benutzer aus. Kein Restore-Fall ist deaktiviert, ignoriert oder an eine zusätzliche Freigabevariable gebunden.

## brain-serve gegen die neue Instanz

`scripts/run_isolated_serve_checks.sh`, 16 von 16 bestanden: `/healthz` 200, `/readyz` 200, API ohne gültiges Token 401, API mit Token 200, `/readyz` bei gestoppter DB 503, Anfrage bei gestoppter DB 503 `policy_unavailable` (nicht unauthorized), Readiness und Antwort nach DB-Neustart wieder 200, SIGTERM sauber (Exit 0, Event `stopped`), Dienst-Neustart ready, falscher Release `release_unavailable`, falsche Knowledge-Version `knowledge_version_mismatch`, Schema-Version 99 `core_schema_incompatible`, fehlendes Schema `core_schema_incompatible`, kein Selbst-Migrieren.

Default-E2E (`scripts/run_isolated_pilot.sh`, echte freigegebene Pilotdokumente aus `~/.local/share/deadlock-brain/pilot-20260925`, Ingest als `brain_ingest`, DB-Neustart zwischen Ingest und Serve): 18 von 18 Fällen, darunter öffentliche und interne Frage, Scope-Leck-Schutz, exakter Zahlenwert, Alias EN und DE, unbekannte Entität, falscher Patch, falscher Modus, ungültiges Token, legaler Build, illegaler Build (`build_rejected`), Hero-Card, Providerfehler, ACL-Revoke und Delete im laufenden Release. Die Build- und Card-Fälle nutzen den synthetischen C6-Domain-Adapter, nicht echte Spielwerte.

## Abhängigkeiten zu DL-Main

Siehe Migrationsbericht. Kurz: `patchnotes.changelog_posts` (Lesen), `steam.steam_tasks` (Schreiben und Lesen für Build-Publish) und die Deadlock-Bots-Writer in das Schema `brain` brauchen explizite Feeds oder APIs, bevor Brain nur noch die eigene Instanz nutzt. Es gibt keinen zweiten DL-Main-Zugang für Brain-Rollen.

## Bekannte Blocker und Betriebsgrenzen, Stand 29.09.2026

1. ~~`DB_POOLING_BACKPRESSURE` fehlt~~ Behoben: PR #49 integriert (`ed06e13`), Pool nachgewiesen (600×3 Stufen gegen diese Instanz, Peak 4, 0 „too many clients"); siehe `FINAL_LOCAL_INTEGRATION_REVIEW.md`.
2. ~~C9-Consumer-Wiring fehlt~~ Behoben: PR #50 integriert (`ed06e13`); Consumer-Staging lokal gegen brain-serve-Staging bestanden (Twitch-Shadow-Fix `d877a9d` in PR #984).
3. Echter Wiki-Pilot: Rechte- und Lizenzentscheidung für Capture und Raw-Aufbewahrung liegt beim Betreiber. Der Adapter in den normalen Store (`wiki_runtime::stage_into_store`, Batches mit Checkpoint und Lease, kein `wiki_scratch.sql`) ist gebaut; der Scratch-Pfad bleibt nur als Offline-Regression.
4. Kein freigegebener KI-Provider oder Modell für Provider-Shadow und keine freigegebene `.dem`; die Replay-Produktentscheidung bleibt offen.
5. ~~Kein Konverter Alttabellen nach `SourceRecordV2`~~ Gebaut (`brain-legacy-import`): Patchnotes und Entitäten aus `brain_legacy` als `SourceRecordV2`, Release `legacy-core-f07ea85c09010285` in `brain_pilot_legacy`, `brain-serve` antwortet daraus. Nicht in die produktive DB `brain` importiert (G5). Übrige Tabellen sind nach `PRE_G5_TECHNICAL_REVIEW.md`, Abschnitt 3, eingeordnet.
6. Providerstatus: Brain-Feed-Verträge und Adapter sind integriert. Die Patchnotes-Providerseite bleibt Python-Legacy, ein Rust-Provider-Nachfolger ist nicht behauptet. Der Steam-Build-Publish-Provider ist integriert und separat offline geprüft. Kein Deployment und kein echter Publish; die Legacy-Direktpfade laufen bis G5 weiter.
