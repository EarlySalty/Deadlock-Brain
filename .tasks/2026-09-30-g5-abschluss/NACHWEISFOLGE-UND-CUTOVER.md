status: aktiv, Clippy/U1/U2/PG1 tatsächlich grün; PG2 und produktive Läufe nicht zugeteilt
Datum: 2026-09-30

Aktueller Stand: SLOT-F-CLIPPY-NACHWEIS.md Exit0, SLOT-G-U1-NACHWEIS.md drei bestanden/einPGignoriert, SLOT-H-U2-NACHWEIS.md20 bestanden/einPGignoriert, SLOT-I-PG1-NACHWEIS.md ein isolierter Upgrade-/Restore-/Minimalrollenfall bestanden plus Cleanup. Jeweils eigene ausdrückliche Zuteilung und sofortige Slotrückgabe. Eingefrorener Quellheadc5951b6 unverändert. PG2 nicht gestartet, historische Startformulierungen im Verlauf unten nicht als aktuelle Läufe lesen.

Nutzerpräzisierung: bestehender Auftrag autorisiert interne technische Übernahme und Bereitstellung unter Erhalt privater Grenzen, keine neue Veröffentlichung und kein Provider-Egress. Leere allgemeine Freigabevorlagen erzwingen keine erneute pauschale Bestätigung. Vorhandene echte Aufgaben-/Quellenreferenzen sind zu prüfen und zu verwenden; Sachlücken bleiben Snapshot-/Status-/Widerrufsmetadaten. Falls deren kanonischer Fingerprint nur mangels ausführbarem Read-only-Modus nicht erhebbar ist, Vorbereitung eines engen Zusatzes am bestehenden Rust-Importer erlaubt, derselbe Leser/Config/Secretpfad, keine Zielmutationen und Ausgabe nur Hash/Counts/ID-Inventar. Der eingefrorene Prüfkopf bleibt dabei unverändert; keine Import-/Serve-/Policyänderung beauftragt.

Aktualisierung16:19UTC: Clippy aufc5951b6 beendet mit tatsächlichem Exit0, BelegSLOT-F-CLIPPY-NACHWEIS.md. U1 nach eigener ausdrücklicher Zuteilung alsb6sdg0hdh/PID1716785 seit16:19:54UTC gestartet. Kein Start der restlichen unten vorbereiteten Pakete.

## Bestehende Aufträge als Evidenz, keine neuen Formalfreigaben

AUFTRAG.md:12-16 dokumentiert den Gesamtabschluss einschließlich produktiver Aktivierung nach erfüllten Nachweisen sowie Replay außerhalbV1. B2-IMPORT-BRIEFING.md:8-14 enthält den ausdrücklich bereits autorisierten Archiv-zu-Core-Cutover; :29-37 bindet vorhandene Rollen, private Patchnotes und das Verbot einer stillen Veröffentlichung. Das sind vorhandene Aufgaben-/Scopebelege und keine erneut offenen Nutzerentscheidungen. Diese Vorgaben werden beim Auflösen der Produktionsbindung berücksichtigt, nicht wegen einer leeren Configvorlage verworfen.

Die weiterhin konkrete Lücke ist die nachprüfbare Zuordnung dieser geltenden Vorgaben und der tatsächlichen Quellen-/Rechtebelege zu exakt dem importierten Snapshot und seinen vollständigen Zustandslisten. Ein approval_ref muss auf einen solchen vorhandenen, inhaltlich passenden Beleg verweisen; eine beliebige Hashzeichenfolge oder eine vom Agenten neu behauptete Rechtefreigabe ist kein Ersatz. Publikations-/Egressrechte werden weder aus dem Auftrag noch aus historischen public-Heads zusätzlich abgeleitet. Nach den unabhängigen Laufproben zuerst die vorhandenen Quellenverträge mit der tatsächlichen Datenbindung zusammenführen; nur eine dann belegte echte Produktentscheidung an den Nutzer zurückgeben.

# Kleinste nächste Nachweise und tatsächliche Cutover-Lücken

Geprüfter Quellhead: c5951b610aa2545d2c0b43b33b5fe1906198b292, Quellfix efb56023deda07ae2c273883d617f2f26b263fc8. Bestehende Produktabnahmen gelten weiter. Diese Vorbereitung liest vorhandene Testselektoren, Runner und Bindungsverträge; sie ist keine neue Gesamtquellen-Abnahme. Twitch hat laut Nutzer gemeinsamen All-Targets-Check und Fresh-Schema-Test bestanden, Originalgate und Release folgen. Brain hat weiterhin kein Compilerfenster. Keine zusätzliche Wache.

## Reihenfolge und gemeinsamer Ressourcenvertrag

Nach tatsächlich grünem, separat zugeteiltem Clippy: U1, U2, danach PG1 und PG2 jeweils als ausdrücklich zugeteiltes Paket. Kein automatisches Weiterlaufen und keine Workspace-Wiederholung zum Zählen. Pro Paket tatsächlichen Head, Start, PID/Session, vollständigen geschützten Log, Exit, Loghash und Cleanup sichern; Slot beim Ende sofort zurückgeben, nicht erst nach Bericht oder Fehleranalyse. Rot erzeugt einen konkreten engen Befund, keine blinde Wiederholung. Keine neue Modellwahl oder neue Reviewrunde ohne Produktdelta.

Alle Cargo-Aufrufe: vorhandene Toolchain1.97.1, locked/offline/jobs1, vorhandenes Cargo-Home und derselbe Targetcache. Kein Fetch, Releasebuild oder neuer Cache. DB-/Prozesspakete sind ausdrücklich mehr als Compilerarbeit. Historische954 bestandene Workspacefälle aufca4a8f2 sind kein Beleg für den aktuellen B2-Stand. Zeitbedarf und tatsächlicher RAM-/Verbindungspeak sind nicht neu gemessen und werden nicht zugesagt.

## U1: kleinster direkter B2-Unitbeleg

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import --bin brain-legacy-import --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- --test-threads=1
```

Am Quellhead drei nichtignorierte reine Tests und ein ignorierter PG-Fall. Erwarteter Umfang ist statisch ermittelt, noch kein Laufresultat:

- `tests::production_route_requires_explicit_exact_endpoint_and_distinct_roles`: Produktionsziel, Port, Rollen und bestehende Secretreferenzen; fehlende Bindung fällt geschlossen aus.
- `tests::connected_identity_must_match_approved_database_and_schemas`: Identitäts-/OID-Wertevergleich ohne Verbindung.
- `tests::complete_release_is_validated_by_byte_length_before_claim`: fertige Release-ID512/513Bytes, UTF-8, Knowledgeversion und Patchgrenzen.

Quellen: Importerbinärdatei:542-644, ignorierter Test:934-936. Keine DB, kein Kindprozess, keine Serveinstanz durch diese drei Testkörper. Der ignorierte Fall darf nicht als bestanden gezählt werden. Insbesondere beweist der reine Längentest allein noch keine unveränderte echte Ziel-DB bei Ablehnung.

## U2: kleine betroffene Librarys statt gesamtem Workspace

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-storage -p brain-legacy-import --lib --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- --test-threads=1
```

Statisch zehn normale Storagefälle und zehn normale Importerfälle; ein zusätzlicher Importer-PG-Fall bleibt ignoriert. Storage verwendet MemoryStore/MemoryRepository und eingebetteten SQL-Text; Importer prüft Konvertierung, Idempotenz, Tombstones, Policyrevision, vollständige private Bindung, Snapshotdrift und Wiederauferstehung im Speicher. Quellen: brain-storage/src/lib.rs:308-356, schema.rs:265-275, domain_reader.rs:324-485; brain-legacy-import/src/tests.rs:65-360,602-738. Tatsächliche Ergebniszahlen erst aus dem Lauf übernehmen. Kein Beweis für PostgreSQL-Sperren, echte Rollen oder Produktionsfreigabe.

## PG1: kleinster vorhandener eigenständiger PG-Runner

```sh
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_storage_upgrade.sh /home/nathanael/.cargo/bin/cargo /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Ein Cargo-Aufruf, genau `brain-storage --test storage_upgrade v1_upgrade_restore_and_least_privilege -- --ignored --exact --nocapture`. Vorhandener Wegwerfcluster unter `/tmp/brain-c11.*`, ausschließlich Unix-Socket, Port55441, max_connections24 und shared_buffers16MB als unveränderte Runnerwerte. Kein Produktionscluster und keine systemd-Aktion. Tatsächliche Migration/Schreibvorgänge, pg_dump/pg_restore und synthetische Minimalrolle. Der Test ruft die historische Fixturehashprüfung selbst auf, kein doppelter separater Hash-Test nötig.

Beweisziel: bestehende v1-Fixture in aktuelle Struktur überführen, Dump/Restore auf leerem Testziel und eingeschränkte Rolle tatsächlich prüfen. Vollständigen äußeren Log erhalten: Cleanup entfernt den Scratchpfad auch nach Testfehler, außer wenn Stoppen scheitert. Kein Beweis für einen aktuellen Produktionsdump, reale Infisical-Zugangsdaten oder die B2-Writersperre. Quelle: Runner:25-78, storage_upgrade.rs:228-254 und bestehender Testkörper.

## PG2: B2-Parallelität und echter Serveprozess im vorhandenen gemeinsamen Runner

```sh
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_serve.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Dies ist der kleinste vorhandene vollständige Fixture-Einstieg für den gebundenen Cutoverfall, **kein kleiner Smoke**. Der Runner besitzt keinen Phasenschalter. Ein einzelner ignorierter Test gegen irgendeinen vorhandenen Socket wäre kein gleichwertiger sicherer Ersatz. Keine neue Runnerarchitektur und keine herausgeschnittenen Shellfragmente.

Wirkung: ein neuer privater PostgreSQL-Cluster, nur Unix-Socket Port55439, max_connections12/shared_buffers16MB, sieben getrennte Fixture-Datenbanken und sieben serielle Cargo-Aufrufe:

| Schritt | Existierender Test | Konkreter Beweis |
| --- | --- | --- |
| 1 | cli_scratch::cli_reads_archive_and_replays_tombstones_and_revokes | Echter Importerprozess mit synthetischen Archivdaten, Wiederholung, Tombstones und Widerruf |
| 2 | tests::scratch_import_release_tombstone_and_revoke | PgStore/LocalPgReader, Idempotenz und spätere Sperren auch bei älterem gepinntem Release |
| 3 | tests::same_database_archive_to_core_requires_bound_private_snapshot | Fehlende/geänderte Bindung abweisen; Releasegrenzen ohne Zielmutationen; zwei Quellen und leere Batches; wirklich wartende Batch-Tombstone- und direkte Scopewriter; Commit- und Rollbackfolge; bestehende Schema99-Fixture nicht verschmutzen |
| 4 | binary_loopback_health_readiness_shutdown_and_no_fallback | Echter Serveprozess plus lokaler Providerstub, Readiness/Health, typed Answer mit Release/Zitat, Ablehnungen, kein Legacyfallback, Drain und Shutdown |
| 5 | private_scratch_scram_accepts_runtime_password_and_rejects_wrong_password | Tatsächliches SCRAM mit synthetischem Passwort, falsches Passwort scheitert |
| 6 | incomplete_configuration_and_inline_secrets_never_reach_startup | Fehlerhafte Konfiguration scheitert vor Dienststart; keine Sentinelwerte im Log |
| 7 | missing_config_and_bad_arguments_exit_without_echoing_input | Fehlerhafte CLI scheitert ohne Eingabeecho |

Schritt4 enthält1.800 Lastanfragen, je600 mit8/16/32 Workern, absichtliche DB-Sättigung und Erholung. Bestehender Servepooldeckel4 muss bestehen; `db_pool_load` und `db_pool_metrics` liefern Poolmesswerte einschließlich erwartetem peak_connections4. Dies ist nicht der gemessene Peak des Cutoverfalls. Dort sind fünf parallele Verbindungen geplant; realer Peak bleibt ohne gesonderte Messung offen. Kein Budget oder Verbindungsdeckel wird erhöht.

Der Runner prüft am Ende das PG-Log auf Verbindungsüberläufe. Cleanup stoppt genau seinen Cluster; bei Fehler bleibt das private Verzeichnis zur Diagnose, bei Erfolg wird es entfernt. Äußeren Log mit Exit und Hash unabhängig davon erhalten. Testseitig behauptete Geheimnisunterdrückung ist kein Anlass, ungesichtete Rohlogs mit Zugangsdaten zu veröffentlichen.

Quellen: test_brain_serve.sh:23-103; Importerbinärdatei:679-932 und934-1280; process_e2e.rs:691-725,744-789,952-969. Vorhandene Testumgebung wird vom Runner lokal isoliert; keine neuen Betreiber-ENV-Dateien oder Konfigwege. Der privilegierte Cutover-Fixturebenutzer und SCRAM-Sentinel beweisen nicht brain_readonly/brain_ingest oder Secret-Exec in Produktion.

### Kleiner Serve-Vorbeleg nur als Alternative bei entsprechend kleinem zugeteiltem Fenster

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-serve --test process incomplete_configuration_and_inline_secrets_never_reach_startup --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- --exact --nocapture --test-threads=1
```

Ein vorhandener Prozess-Negativtest, keine PG-Fixture und kein erfolgreicher Servestart. Nicht als zusätzliche Pflicht vorPG2 einplanen, da er dort bereits enthalten ist. Nicht pauschal alle nichtignorierten `process`-Tests als reine Units anfordern: derselbe Testcontainer enthält auch TCP-/Unix-Listener und simulierte DB-Start-/Signalpfade (`process.rs:86-120,192ff`).

## Tatsächliche Cutover-Voraussetzungen, unabhängig von grünen Tests

| Nachweis | Tatsächlich vorhanden | Konkrete fehlende Bindung oder Aktion vor Cutover |
| --- | --- | --- |
| Datenbankidentität | Frühere Read-only-Messung: Socket /run/deadlock-brain-postgresql, Port5446, DBbrain, OIDs16388/25331/16389, Schema2/brain.store.v2 | Im tatsächlichen Importer- und Dienstkontext erneut prüfen; frühere brain_migrate-Abfrage ist kein Minimalrollen-Login |
| Quellenbestand |905 Entities,3778 Aliase,32821 Patchzeilen/Enrichments,348 Patch-IDs; Herkunftslinks vorhanden | Kanonische Importersicht als zusammengehöriger Snapshot; Counts sind kein Inhaltsfingerprint |
| Snapshot | Schemahash5fe2c40427d3dd57f3d07b714936a633e372ae0a22b6cee4a237a58366aa3a4f früher gemessen | Echtes Label und Epoch plus exakt `snapshot_sha256` über Label, Epoch, Schemahash, Counts, EntityRow-/PatchLineRow-Vektoren und beide aufbereiteten Quellen; kein Ersetzen durch SQL- oder Dateihash |
| Policy | Patchnotes bleiben private/brain.legacy.review; publication_allowed=false und provider_egress_allowed=false für beide Quellen | Nachvollziehbarer tatsächlicher Entscheidungsbeleg mit approval_ref, identischer authorization_ref in beiden Policies und daraus gebundener policy_sha256 |
| Zustandslisten | Historische Pilotheads, bisher keine belegten vollständigen aktuellen Widerrufslisten | Je legacy-entities und legacy-patchnotes disjunkte active_ids/revoked_ids/tombstone_ids; tatsächliche Archivdokumente vollständig abdecken. Fehlende Metadaten nicht als leere Listen auslegen |
| Produktionsconfig | ops/brain-postgres/legacy-core-cutover.json ist absichtlich gesperrte Vorlage | production_binding=null, Label/Releasefelder leer, Epoch0, authorization_ref=null und /DO_NOT_RUN bleiben unverändert, bis echte Belege vorliegen. Kein Probeimport zum Erzeugen fehlender Freigaben |
| Rollen und Secret-Exec | Bestehende Referenzen BRAIN_LEGACY_READ_AUTH/BRAIN_TARGET_INGEST_AUTH, Leserrechte früher geprüft | Echte brain_readonly-/brain_ingest-Verbindung über vorhandenen Secretweg ohne Secret-Ausgabe; Servicekontext und tatsächliche UID-/Socket-Sicht prüfen |
| Writer/Fencing | Shared Store-Writer sperren pro Quelle; B2-R1 statisch abgenommen | Laufende Jobs und direkte SQL-Writer inventarisieren, Drain und nachweisbare Schreibsperre am Cutover. Advisory-Locks schützen keine Writer, die diesen Vertrag umgehen. Früherer Timerzustand ist kein aktueller Sperrbeleg |
| Backup/Rückweg | Historischer Upgrade-/Restorerunner vorhanden | Aktueller Backuphash, Restore auf leerem Ziel, finale Offsets/Änderungen und kompatibles vorheriges Release. Widerrufe/Tombstones beim Rückweg nicht zurückdrehen |
| Veröffentlichung | Zielcore um15:15:15UTC noch ohne Heads/Revisionen/Releases | Tatsächliche gebundene Veröffentlichung mit vollständigem Report und verifizierter Release-ID/Knowledgeversion; historische Leerheit nicht als Gegenwartsbeleg ausgeben |
| Serve | Rust-Binary/Config-/Startvertrag vorhanden | Gemergter SHA, unveränderliches Artefakt, konkreter Loopbackport, Rolle mit Leserechten plus SELECT/INSERT auf conversation_owners_v1, gültiges Release und echte Infisical-Sicht der Unit |
| Consumer | Typed POST/v1/answer und vorhandener Twitchconsumer | Integrator bindet denselben Endpoint/Release/öffentliche Scopes, vorhandene Dienstidentität und freigegebenen Provider; echter Antwortbeweis über Consumer mit Wegwerfconversation, keine Chatnachricht. Health200 reicht nicht |

### Warum Policy und Snapshot sachlich noch offen sind

Von fünf referenzierten Entity-Quelldokumenten hatte keines einen policy-Key; von428 Patch-Quelldokumenten nur136. Vorhandene Freitextwerte waren lediglich gehasht. Das belegt weder eine gültige Nutzungsentscheidung noch vollständige Widerrufsinformationen. Die1.637 historischen removed-Events sind Patchereignisse, keine Liste gelöschter Dokumente. Historische Pilotexperimente, unbekannte Herkunftsrechte und der allgemeine Auftrag „Ja, nach allen Nachweisen“ ersetzen diese Datenbindung nicht. Keine neue pauschale Freigabefrage, aber auch keine synthetische approval_ref.

Der Fingerprintvertrag steht in cutover.rs:39-64, Prüfung in:67-164. read_legacy verwendet eine zusammengehörige Read-only-Repeatable-Read-Transaktion (pg.rs:19ff). Die geprüfte Importer-CLI bietet genau `--config` und führt `run` aus (Binärdatei:463-495), keinen zugesicherten read-only Fingerprintmodus. Deshalb keine nicht vorhandene `--dry-run`-Option versprechen und nicht den schreibenden Importer zur Bestandsaufnahme starten. Benötigt wird der tatsächliche gebundene Snapshotbeleg aus dem bestehenden Snapshot-/Importkontext; fehlt dafür ein ausführbarer Weg, ist das ein eng abzugrenzender Werkzeugrest, kein Anlass zu einem zweiten Importer oder globalen Architekturumbau.

Ein grünerPG2-Lauf schließt synthetische Laufzeitbeweise, nicht diese produktiven Lücken. Ein isolierterAPI-Anschluss ist ferner kein abgeschlossener Writerwechsel. Replay bleibt außerhalbV1 und wird nicht als bestanden verbucht.

ORCHESTRIERUNG[OR-1]: Stufe groß | Schritt review | Artefakt: .tasks/2026-09-30-g5-abschluss/NACHWEISFOLGE-UND-CUTOVER.md
