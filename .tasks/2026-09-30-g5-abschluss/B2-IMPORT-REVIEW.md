# B2: unabhängige statische Importerabnahme

Datum: 2026-09-30. fertig: J (Review abgeschlossen). Fix: J.

**BLOCK: drei konkrete Befunde.** Die neue Produktionsverzweigung und ihre Vorprüfungen sind vorhanden. Die Veröffentlichung ist aber nicht gegen Änderungen nach der letzten Prüfung gesichert; ungültige Releasefelder werden teilweise erst nach Zielwrites abgewiesen. Der vorgeschlagene Anschluss des neuen Scratchtests kollidiert außerdem mit einer vorhandenen Fixture. Kein Compiler-, Test-, DB-, Rollen-, Produktprozess-, Modell-, Dienst- oder Deploylauf wurde ausgeführt.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft

Die zwei Fremddienstpfade sind die statisch geprüften PostgreSQL-Verbindungen für Archivlesen und Zielstore, keine Liveverbindungen. Fehler werden über `Result` bis zum CLI-Exit 1 weitergegeben; Wiederanlauf und Erfolgsaussage haben die unten beschriebenen Grenzen.

## Geprüfte Bindung

- Quelle als eingefrorene Git-Blobs: `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`, Basis `35673e7959290431ca60641195ee1378c09194eb`, B2-Head `c5d2b1f4f18eb8fd360641b3e66fa7453cafb4d2`. Genau fünf Quelldateien, 1088 Einfügungen und 46 Löschungen. Kein Delta an Cargo-Lock/Manifest, Store oder Grants. B1-R1-WIP wurde nicht gelesen oder mitbewertet.
- Autorbericht aus `e2cb15486f9816ac541b2025d53833039926bd6c:.tasks/2026-09-30-g5-abschluss/B2-IMPORT-ERGEBNIS.md` gelesen und gegen Code geprüft. Dieser Commit ergänzt gegenüber dem Quellhead eine Berichtsdatei, keinen weiteren Produktcode.
- Eigene Berichtssenke: `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`, sauberer Ausgangshead `eb6ac35a3ee089bc655f10248a95e918fe9d96e6`.
- Graphify vor den Codefragen verwendet; die globalen Treffer waren für die neuen Importerstellen nicht hinreichend. Anschließend konkrete eingefrorene Importer-, Store-, Checkpoint- und direkt betroffene Teststellen gelesen. `git diff --check 35673e7 c5d2b1f` ohne Befund; JSON-Parse der eingefrorenen neuen Vorlage erfolgreich. Rust weder kompiliert noch ausgeführt. Fünf neue Testfunktionen statisch gefunden, keine bestehende Testfunktion entfernt.

Alle folgenden Codezeilen beziehen sich auf `c5d2b1f`. `Importer` bezeichnet `rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs`.

## Überprüfte Befunde

### B2-R1, P2: Prüfung und Veröffentlichung haben keine gemeinsame Schreibsperre

**Anker:** `Importer:381-407`, insbesondere `:398-407`. Fehlerklasse: veralteter Prüfbefund wird als aktueller Erfolg ausgegeben.

**Konkrete Folge:** Beide Importbatches sind committed. `target_heads` und die Checkpointabfragen bestätigen den gebundenen Zustand. Danach schreibt ein weiterer gültiger Writer für eine der als aktiv genehmigten IDs eine neue Tombstone-Revision mit passendem Checkpoint. Der Importer publiziert trotzdem den zuvor berechneten Release und meldet Erfolg samt ursprünglicher Dokumentzahl. Die alte Revision existiert weiterhin; die nachfolgende Kontrolle zählt `snapshot.revisions`, nicht genehmigte aktuelle Heads. Der veröffentlichte Stand erfüllt damit die unmittelbar zuvor geprüfte aktive Inventar-/Checkpointbindung nicht mehr. Dasselbe Zeitfenster besteht zwischen der Headabfrage und den einzelnen Checkpointabfragen, soweit eine Änderung erst nach der jeweiligen Prüfung erfolgt.

**Nachweiskette:** `brain-storage/src/pg_jobs.rs:74-78` setzt die Quellenlease beim Commit auf idle. `Importer:381-397` liest ohne gemeinsame Transaktion oder gehaltenes Fencing. `brain-storage/src/pg_release.rs:15-52` sperrt den Release-Schlüssel und prüft das Vorhandensein historischer Revisionen, nicht die Importer-Baseline oder aktuelle Quellcheckpoints. `:135-146` liefert historische Revisionen und aktuelle Heads getrennt zurück. Der Importer prüft danach bei `:406` lediglich die Zahl historischer Revisionen. Ein aktueller Tombstone reduziert diese Zahl nicht.

**Abgrenzung:** Das beweist keine Wiederbelebung gelöschter Heads und keinen ACL-Bypass des Readers. Die bestehenden aktuellen Head-/ACL-Prüfungen bleiben erhalten und können die betroffene Antwort sperren. Der Defekt ist die trotzdem erfolgreiche Veröffentlichung als vollständig gebundener Import. Ein erneuter Lauf kann anschließend schon an `verify_head` scheitern, obwohl der vorherige Lauf Erfolg gemeldet hat.

**Enge Korrektur:** Baselineprüfung und Veröffentlichung unter denselben wirksamen Transaktions-/Writergrenzen durchführen; ein weiteres ungesperrtes Nachlesen schließt das Fenster nicht. Vorhandenen Baustein `PgStore::commit_batches_and_publish` in `brain-storage/src/pg_release.rs:62-109` berücksichtigen, statt einen zweiten Importstore zu bauen. Seine Verwendung allein belegt noch keine Sperre gegen direkte `PgStore::apply`-Writer bei unveränderten Records; die konkrete geschützte Baseline muss mitgeprüft werden. Kein Writer-Fencing als bereits gegeben behaupten.

**Zwillingssuche/Gegenbeweis:** Beide Quellen durchlaufen dieselbe Schleife. Geprüft wurden Einzelcommit, `publish_release`, atomare Bestandsmethode und direkter Apply-Pfad (`brain-storage/src/lib.rs:163-176`). Benötigt wird ein deterministischer Scratch-Gegenbeweis mit synchronisiertem konkurrierendem Commit nach der letzten zulässigen Prüfung, für Tombstone und Scopewiderruf. Erwartung: kein falscher Erfolgsbericht und kein neuer als gebunden bestätigter Release aus dem überholten Prüfergebnis. Kein solcher Test ist im B2-Delta vorhanden.

### B2-R2, P2: Ungültige Releasefelder werden nach beiden Quellencommits erkannt

**Anker:** `Importer:81-84,350-366,398-401`; `rust/crates/brain-legacy-import/src/lib.rs:502-531`. Fehlerklasse: unvollständige Konfigurationsprüfung vor Schreibwirkung.

**Konkreter Input:** Eine ansonsten gültige Produktionsbindung mit `release.id_prefix` aus 496 ASCII-Zeichen. `route` akzeptiert den nichtleeren Wert. `release_from_checkpoints` hängt Bindestrich und 16 Digestzeichen an und erzeugt damit eine 513 Byte lange Release-ID. Die beiden `claim`-/`commit`-Folgen schreiben Jobs, Records und Checkpoints. Erst `publish` ruft den bestehenden Releasevalidator auf und verwirft die ID wegen der Grenze von 512 Byte. Der vorhersehbar ungültige Auftrag hinterlässt Zielmutationen ohne Release.

**Nachweis/Zwillinge:** `brain-storage/src/memory_repository.rs:241-262` setzt dieselbe 512-Byte-Grenze für Release-ID, `knowledge_version` und `patch`; `pg_release.rs:19` ruft diesen Validator auf. Die beiden letztgenannten Felder haben im neuen `route` ebenfalls lediglich eine Leerprüfung. `release_from_checkpoints` prüft Dokumentzahl und doppelte Quellen, aber nicht diese Releasefelder. Der Pilotzweig verwendet denselben Helper; das Fehlermuster ist dort bereits möglich und wird mit der Produktionsfreigabe nicht abgesichert. Der Befund ist die neue Produktionsanwendbarkeit, keine Behauptung einer neu eingeführten Pilotregression.

**Enge Korrektur/Gegenbeweis:** Den tatsächlich erzeugten Releasevertrag vor dem ersten Claim validieren, einschließlich der fertigen ID-Länge. Gleichwertige Bestandsregeln wiederverwenden und keine höhere Grenze einführen. Grenzfälle 495/496 Byte für den Präfix sowie 512/513 Byte für Version/Patch nachweisen. Für den negativen Scratchfall müssen Jobs, Records, Checkpoints und Releases unverändert bleiben. Ein Fehlertest, der lediglich den abschließenden Exit prüft, reicht dafür nicht.

### B2-R3, P2: Der neue Positivtest belegt eine bereits reservierte Scratchdatenbank

**Anker:** `Importer:575-585,612-623,745-755`; vorgeschlagener Anschluss im Autorbericht, Abschnitt „Genau benannte spätere Befehle“.

**Konkrete Folge:** Der Autor schlägt vor, `same_database_archive_to_core_requires_bound_private_snapshot` hinter `createdb brain_schema_test` im bestehenden Serve-Harness auszuführen. Der Test migriert darin `brain`, erstellt `brain_legacy` und hinterlässt beide Schemas. Der später ausgeführte Bestandstest `rust/crates/brain-serve/tests/process_e2e.rs:433-445` erwartet dieselbe frische Datenbank und führt `CREATE SCHEMA brain` aus. Das scheitert dann an dem bereits vorhandenen Schema, statt den beabsichtigten inkompatiblen Schema99-Startup nachzuweisen. Umgekehrte Reihenfolge löst es nicht: Dann trifft der neue Test auf den absichtlich inkompatiblen Altzustand.

**Status und Zwillingssuche:** Bei `c5d2b1f` ist der neue Test tatsächlich `#[ignore]` und an keinen Runner angeschlossen. Das bestehende Serve-Harness ist deswegen durch B2 derzeit nicht verändert; betroffen ist der behauptete ausführbare zusätzliche Testvertrag. `scripts/test_brain_serve.sh:60` erzeugt genau diese DB, und die beiden genannten Rusttests sind ihre geprüften Verbraucher. `pool.close()` am Testende entfernt kein Schema. Der Name ist daher keine freie neue Fixture.

**Enge Korrektur/Gegenbeweis:** Den neuen Fall an den vorhandenen Harness anschließen, mit eigenem ausdrücklich isoliertem Fixtureziel und dessen Cleanup oder nachgewiesenem Wiederherstellen des leeren Testzustands. Keinen parallelen Runner bauen und den vorhandenen Schema99-Test nicht abschwächen. Danach beide Fälle im zugeteilten Ablauf nachweisen. Ein freistehender Aufruf mit einem beliebigen Socket ist kein Ersatz für diesen Anschluss.

## Unauffällige Schutzpfade und Grenzen der Autorbehauptungen

1. **Eintrittspfade und Endpunkte:** `main -> run -> run_with_destination` ist der produktive CLI-Pfad (`Importer:234-252,422-455`). Der Pilotzweig behält `brain_pilot*`, andere Quelldatenbank und Ausschluss einer Produktionsbindung (`:64-70`). Der neue Zweig bindet beide Datenbanken an `brain`, Socket `/run/deadlock-brain-postgresql`, Port 5446, Rollen `brain_readonly`/`brain_ingest` und getrennte bestehende Secretreferenzen (`:53-95`). Schemanamen bleiben feste SQL-Bezeichner, kein frei konfigurierbares Ziel-SQL. `Identity` prüft DB, Rolle, Unix-Transport sowie DB-/Schema-OIDs (`:98-153`); Port/Socket werden über explizite Verbindungsoptionen gebunden, nicht noch einmal serverseitig abgefragt. OIDs sind lokale Identitäten, kein externer Freigabeaussteller. Bibliotheksfunktionen bereiten Daten vor; sie führen selbst keinen Claim oder Storewrite aus. Die bestehenden `PgStore`-Writer sind nicht durch Importer-Konfiguration global gesperrt.
2. **Bindung vor Zielzugriff:** Fehlende Produktionsbindung/abweichende Endpunkte scheitern in `route`. `verify_sources` wird nach der Archivlesung und vor dem Zielconnect ausgeführt (`Importer:264-302`). Es prüft Approvalformat, positive und unterschiedliche Schema-OIDs, Snapshot-/Schema-/Policyfingerprints, Tabellenzahlen, genaue Quellmenge und disjunkte ID-Klassifikation (`cutover.rs:67-162`). Jede beobachtete Dokument-ID muss klassifiziert sein; aktive und widerrufene IDs müssen beobachtet sein, Tombstone-IDs dürfen als historische Löschungen fehlen. Beide aktiven Quellmengen müssen nichtleer bleiben. Das entspricht einem begrenzten Import, nicht dem Beweis eines vollständig gelöschten Archivs.
3. **Private Rechte und Fingerprintumfang:** Patchnotes bleiben `private` mit exakt `brain.legacy.review`; Egress und Publikation sind für beide Quellen false, die Herkunft trägt dieselbe Approvalreferenz (`cutover.rs:95-118`, `lib.rs:444-456`). Die Fingerprints prüfen Konsistenz der gelieferten Bindung, nicht deren Aussteller oder aktuelle externe Rechte. `snapshot_sha256` bindet den tatsächlich gelesenen `LegacyRead` und die daraus erzeugten Dokumente. Das ist kein Hash jedes Rohfelds der vier Tabellen: `pg.rs:51-90` projiziert bei Patchnotes unter anderem nur `metadata->>'source_language'`; das vollständige Patchmetadata-JSON ist nicht enthalten. Die Autorformulierung „einschließlich Rohmetadaten“ gilt für Entity-Metadaten (`pg.rs:108-126`), nicht pauschal für sämtliche Archivspalten. Keine vollständige Roharchiv- oder Policyparität daraus ableiten.
4. **Tombstones und Wiederanlauf:** Bestehende aktive Heads müssen dem freigegebenen vollständigen Record entsprechen; genehmigte gelöschte/widerrufene IDs dürfen als vorhandene Heads bereits tombstoniert sein (`cutover.rs:166-207`). Ein noch aktiver Head einer jetzt gelöschten/widerrufenen ID wird hingegen vor Batchvorbereitung abgewiesen (`Importer:322-344`). Die Autorbehauptung, ein aktiver Zielcheckpoint werde durch Weglassen automatisch tombstoniert, beschreibt den unteren `prepare_document_batch`-Baustein, ist aber im neuen Produktionspfad durch diese Vorprüfung nicht erreichbar. Das ist konservatives Blockieren, kein ausgeführter Widerruf. Im leeren Ziel materialisiert das Weglassen keinen historischen Tombstone. Gleiche Bindung und unveränderte Records sind logisch wiederholbar; Checkpointgenerationen steigen auch bei leeren Änderungsbatches. Nach Fehler zwischen den zwei Quellencommits kann ein Teilstand bestehen bleiben. Kein automatischer Restore; neue ACLs, Ownership oder Deletes werden hier nicht zurückgedreht.
5. **Scope und Vorlagen:** Geändert sind Importer, dessen Testquellen und eine normale gesperrte JSON-Vorlage. Die alte Pilotconfig bleibt unverändert. `legacy-core-cutover.json` enthält `production_binding: null`, leeres Snapshotlabel/Releasefelder, Epoch 0 und `/DO_NOT_RUN/...`; `route` lässt diese Vorlage nicht produktiv laufen. Die reale Existenz und Freigabe ausgefüllter Operatorwerte wurde nicht untersucht. Approval/OIDs/Fingerprints und Klassifikation sind eingeführte Pflichtinputs, keine neue von diesem Review verlangte Genehmigungsschleife und keine Behauptung, dass diese Werte im Betrieb fehlen. Die Befunde R1 bis R3 bestehen unabhängig von der Bereitstellung gültiger Werte.

## Testquellen: tatsächliche Abdeckung statt Laufbehauptung

| Neue Testquelle | Statisch erkennbare Aussage | Verbleibende Grenze |
| --- | --- | --- |
| `src/tests.rs:603`, `cutover_requires_complete_private_policy_and_exact_snapshot` | Leerer Policyhash, abweichender Snapshothash, geänderte Entity-Metadaten und öffentliche Patchnotes werden verworfen; Herkunft und private Sichtbarkeit werden geprüft. | Keine DBwirkung und keine vollständige Mutationsmatrix für OIDs/Inventare. |
| `src/tests.rs:681`, `cutover_excludes_approved_tombstones_and_rejects_resurrection` | Eine klassifizierte Tombstone-ID wird ausgelassen; aktiver Gegenhead, geänderte Sichtbarkeit, Herkunftsmetadaten und Inhalt werden abgewiesen. | Kein vorhandener DB-Delete, kein `revoked_ids`-Lauf, kein Produktions-Scopewechsel. |
| `Importer:503`, `production_route_requires_explicit_exact_endpoint_and_distinct_roles` | Konfigurationsguard für Bindung, Socket, Ziel-DB, Rollen, Zielport, Quell-Secretreferenz und Pilot-Abgrenzung. | Pure Guardfunktion; keine verbundenen Rollen oder Schreibzähler. |
| `Importer:536`, `connected_identity_must_match_approved_database_and_schemas` | Identitätsprüfung mit falschem Archiv-OID, TCP-Adresse und Benutzer. | Synthetische Struktur, keine tatsächliche Clusteridentität oder Grants. |
| `Importer:576`, `same_database_archive_to_core_requires_bound_private_snapshot` | `brain_schema_test` hat kein Pilotpräfix; mit `run_with_destination` wird wirklich der neue Produktionszweig betreten. Geprüft werden fehlende Bindung, falscher Archiv-OID, Erstimport, private Patchnote, Wiederholung und später geändertes Archiv. | Ignoriert, nicht angeschlossen und R3. Verwendet dieselbe privilegierte Rolle `brain_core_test` für Quelle/Ziel und `require_auth=false`. Kein Positivnachweis getrennter produktiver Rollen/Secretreferenzen, kein echter CLI-/Secret-Exec-Aufruf. |

**Noch gezielt nachzuweisen:** R1-Race und Abbruch nach erster Quelle/vor Publish mit Wiederanlauf; R2 ohne Zielmutationen; produktionsnaher Positivpfad mit getrennten lokalen Testrollen und passenden Minimalrechten; falsche DB-/Core-OID-/logische Schemagrenze sowie fehlende/überlappende ID-Klassifikation vor Writes. Der neue Negativ-Scratchtest zählt bisher Heads, nicht Jobs/Checkpoints/Revisionen zusammen. Private Patchnotes, Widerruf und Tombstone müssen unter dem tatsächlichen Produktionszweig nachgewiesen werden, nicht durch Übernahme alter Pilot-/Memorytests. Diese Testquellen sind nicht ausgeführt. Frühere 954/74 gelten nicht als B2-Nachweis.

## Exakte spätere Befehle und Laufklassen

**Jetzt nicht ausführen.** Vor einer späteren Zuteilung muss ein sauberer Stand mit den geprüften/fixierten Blobs gebunden werden. Der genannte Quellworktree enthält bewegliche B1-R1-Arbeit; seine Pfadangabe allein bindet keinen Commit. Keine vorhandene Arbeitskopie durch den Reviewer umschalten. Toolchain und Caches müssen bereits vorhanden sein.

### Compiler und nichtignorierte Importertests

```bash
/home/nathanael/.cargo/bin/cargo +1.97.1 check --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import --all-targets --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import --lib --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import --bin brain-legacy-import --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Drei getrennte serielle Vorgänge, kein automatisch verketteter Sammellauf. Vorhandenes Cargo-Home `/home/nathanael/.cargo`; keine neuen Caches oder Downloads. Check kompiliert, die zwei Testbefehle starten Rust-Testprozesse mit reinen/In-Memory-Fällen; ignorierte PG-Fälle bleiben ausgeschlossen. Ein Cargojob ist keine Zusage für einen einzigen Test-/Linkerthread oder eine feste RAMgrenze. Vollständigen Log und tatsächlichen Exit erhalten, kein Logfilter als Erfolgsbeleg.

### Neuer Scratchfall: derzeit kein freistehender ausführbarer Gesamtvertrag

Der genaue Testselektor lautet:

```bash
env -i "PATH=$PATH" "HOME=$SCRATCH" "CARGO_HOME=/home/nathanael/.cargo" "RUSTUP_HOME=/home/nathanael/.rustup" "CARGO_BUILD_JOBS=1" "SQLX_OFFLINE=true" "BRAIN_CORE_TEST_PG_SOCKET=$CLUSTER" /home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -p brain-legacy-import --bin brain-legacy-import tests::same_database_archive_to_core_requires_bound_private_snapshot -- --ignored --exact --nocapture
```

`SCRATCH` und `CLUSTER` sind hier ausdrücklich **interne vom vorhandenen Harness erzeugte Testkoordinaten**, keine produktive ENV-Konfiguration und keine erfundenen existierenden Pfade. Der Befehl ist vor R3-Fix nicht als ausführbarer Harness freigegeben. Die nötige Änderung muss Zieldatenbank, Reihenfolge, Isolation und Cleanup mechanisch herstellen; der Autor muss danach den vollständigen vorhandenen Runneraufruf mit diesem Filter liefern.

Tatsächliche Klasse: Compiler plus Rust-Testprozess, echter privater Unix-Socket-PG auf Port 55439, Migration des Core-Schemas, Erstellen des Archivschemas und der vier Fixturetabellen, synthetische Inserts, Claims/Checkpoints/Releasewrites und Archivänderung. Keine produktiven DB-Verbindungen und keine externe Quelle. Die aktuelle Testquelle erzeugt keine getrennten Rollen; deren Gegenbeweis benötigt eine ausdrücklich zugeteilte lokale Rollenfixture. Keine Rollenerstellung wurde hier ausgeführt.

### Bestehender Serve-Harness

```bash
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_serve.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Dieser vorhandene Direktaufruf ist für seinen eingefrorenen B1-Vertrag argumentseitig korrekt, beweist aber bei B2-Head den neuen ignorierten Test nicht. Er umfasst private PG-Schreibfixtures, Importer-/Serveprozesse, lokale Providerfixtures, synthetischen SCRAM-Nachweis und **600 Lastanfragen bei jeweils 8, 16 und 32 Workern**, also 1.800 Lastanfragen zusätzlich zu funktionalen Requests. Unix-Socket Port 55439, `max_connections=12`, `shared_buffers=16MB`; Clusterstop und Fehlerlog-Aufbewahrung durch bestehenden Runner. Keine Umdeutung zum leichten Smoke, keine Lastzuteilung durch dieses Review. Nach R3-Anschluss wird sein erweitertes Ressourcenprofil erneut konkret gebunden, nicht automatisch gestartet.

## Abschluss

Statisch belegt sind die engen Endpunkt-/Rollen-/Schema- und Inhalts-/Policyprüfungen sowie die gesperrte Produktionsvorlage. Kein GO für das vollständige B2-Delta wegen R1 bis R3. Nächster Schritt: die drei Befunde an den bestehenden Autor geben, Fixdelta getrennt sichern und im bestehenden Reviewerthread nachprüfen. B1-R1, Twitch-/Ops-Livezustand und reale Operatorfreigaben bleiben außerhalb dieses Berichts. Keine zusätzliche pauschale Nutzerfreigabe verlangt.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 2 belegt | Senke: B2-IMPORT-REVIEW.md
