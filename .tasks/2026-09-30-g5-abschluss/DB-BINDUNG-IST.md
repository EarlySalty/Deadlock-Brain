status: erledigt, begrenzte Leseprüfung; produktiver Import und Serve weiter offen
Datum: 2026-09-30

# Tatsächlicher Datenbankbestand und nächste G5-Änderungen

## Durchgeführte Prüfung

Die konkret angemeldete psql-Metadatenprüfung wurde vom Nutzer zugeteilt. Kleine anschließende reine Metadaten-/Datei-/Quellprüfungen sind ausdrücklich ebenfalls erlaubt. Keine schreibende DB-Aktion, kein Answer-/Modellaufruf, kein Dienststart oder Restart.

Erster psql-Aufruf: Exit2 vor der DB-Anmeldung, Socketzugriff verweigert. Ursache konkret geprüft: laufende Sessiongruppen1000/27/119, vorhandene Kontomitgliedschaft zusätzlich979 (`deadlock-brain-db`); Socketmodus0770, Gruppe deadlock-brain-db. Keine Rechte verändert. Dieselbe bereits berechtigte Identität über `sudo -n -u nathanael -g deadlock-brain-db` verwendet, weiterhin DB-Rolle brain_migrate per vorhandenem Peer-Mapping. Kein Wechsel auf root oder einen DB-Superuser.

Zweiter Aufruf: Exit0, Verbindung zu **brain**, Benutzer **brain_migrate**, Port **5446**. Explizite READ ONLY-Transaktion, statement_timeout2s, lock_timeout1s, Abschluss ROLLBACK. Anschließende kurze Leseprüfungen liefen auf demselben legitimen Weg, ebenfalls READ ONLY und ROLLBACK. Kein Secretwert, Passworthash, Prozess-ENV oder Dokumentinhalt gelesen.

## Tatsächliche Ergebnisse

- Genau eine Versionszeile: **Schema2, brain.store.v2**.
- brain_ingest, brain_migrate, brain_readonly, brain_service: LOGIN, kein SUPERUSER/CREATEDB/CREATEROLE/REPLICATION/BYPASSRLS. Verbindungsdeckel4/3/2/16 unverändert.
- brain_service: SELECT auf Versionsmarker und die sechs Kerntabellen. INSERT ausschließlich auf conversation_owners_v1. Keine UPDATE-/DELETE-Rechte auf den sieben geprüften Tabellen. Damit ist der bereits vorhandene Serve-Rollenvertrag bestätigt, keine neue DB-Rolle nötig.
- **Ziel-DB brain hat keinen CorpusRelease und weder Source-Heads noch Source-Revisionen.** Präsenz mit EXISTS geprüft, keine Vollzählung. Schemas brain und brain_legacy bestehen.
- Pilotdatenbanken brain_pilot und brain_pilot_legacy existieren. In brain_pilot_legacy liegen drei historische Releases: legacy-core-f07ea85c09010285, legacy-core-6c158962d92e8151 und legacy-core-a577c995fbe93bcd, jeweils brain-legacy-core-v1 und Patch legacy-archive-20260926T025758Z.
- Begrenzte Stichprobe eines aktuellen Heads je bekannter Quell-ID im Pilot: legacy-entities öffentlich mit game.public; legacy-patchnotes privat mit brain.legacy.review. Beide Stichproben nicht tombstoned. Das ist keine Vollprüfung des Corpus. **Den Pilotrelease deshalb nicht als fertigen öffentlichen Produktionsrelease kopieren oder einfach als Serve-Ziel auswählen.**
- Eine erste Policyprojektion auf record_json.policy war leer, weil SourceRecordV2 dieses Feld nicht hat. Nach Prüfung des tatsächlichen Typs wurde die Projektion auf visibility/allowed_scopes korrigiert. Kein fehlender Policy-Datensatz oder Produktdefekt aus dem falschen JSON-Pfad abgeleitet.

## Konkrete noch nötige Quelländerung im vorhandenen Importer

`rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs:56-62` erlaubt absichtlich nur target.database mit Präfix brain_pilot und verwirft gleiche legacy-/target-Datenbank. Der vorhandene Archivbestand liegt aber in **brain.brain_legacy**, der Zielstore in **brain.brain**. Die Beispielconfig ops/brain-postgres/legacy-core-import.json importiert ausdrücklich nach brain_pilot_legacy, nicht Produktion.

Für den autorisierten G5-Cutover ist eine **enge Erweiterung dieses vorhandenen Rust-Importpfads** nötig: die explizit gebundene Archiv-zu-Core-Kombination erlauben, getrennte vorhandene Lese-/Ingestrollen und Schemazuständigkeiten erhalten, beliebige Gleichdatenbank- oder Fremdzielzugriffe weiterhin abweisen. Kein pauschales Entfernen der Guards, kein SQL-Backfill an der Importlogik vorbei und kein neuer Importer. Normale Config bindet Quelle/Ziel/Snapshot/Release/Scopes; vorhandener Secret-Exec übergibt bestehende Infisical-Referenzen intern. Keine Betreiber-ENV-Konfiguration.

Die Quellrechte werden nicht aus einer grün gewünschten Antwort rückwärts gewählt: Die bisherige Importvorlage enthält game.public sowie provider_egress_allowed=false und publication_allowed=false. Deren beabsichtigte konservative Bedeutung und Durchsetzung im vorhandenen Quellenvertrag muss vor einer Produktionsveröffentlichung belegt werden. Keine automatische Freischaltung dieser Rechte und kein neuer Provider. Aktuelle private Heads/Tombstones und Archivdeltas gehören zur Datenscope-/Rückwegprüfung, nicht als private Inhalte in einen Prüfbericht.

## Konkrete Unit- und Configbindung

Die laufende User-Serviceverwaltung wurde rein lesend geprüft: PID933, Identität1000, Gruppen27/119/1000, **ohne** deadlock-brain-db. Ein normaler neuer Userdienst aus der Vorlage würde diese veraltete Gruppensicht erben. Kein Restart des gesamten Usermanagers, weil das fremde Dienste und Arbeitsabläufe gefährden würde.

Empfohlene Installationsbindung für den separat koordinierten Cutover: dieselbe bestehende Servevorlage als **Systemunit brain-serve.service unter dem vorhandenen Benutzer nathanael**, explizite SupplementaryGroups=deadlock-brain-db. Dadurch keine neue OS-/DB-Identität und kein Usermanager-Restart. Kein neuer Backendpfad. Absolute revisionsgebundene Binarypfade statt Beispiel /opt/deadlock-brain-c1; normale Configdateien ausdrücklich per --config; keine Environment-Zeile. Bestehende LoadCredential-/Infisical-Kette erhalten. Die tatsächliche Namespace-/UID-0-Socketsicht, Gruppen und Dateirechte bleiben vor dem Answerbeweis zu verifizieren. Noch keine Unit erstellt, installiert oder gestartet.

Serve-DBbindung ist jetzt konkret: socket_dir=/run/deadlock-brain-postgresql, port5446, username=brain_service, database=brain, auth=password mit bestehender BRAIN_PG_SERVICE_PASSWORD-Referenz. Pool/Budgets nicht erhöhen. Release-ID/Knowledgeversion erst aus einem tatsächlich erzeugten freigegebenen Produktionsrelease übernehmen. 127.0.0.1:8788 bleibt lediglich freier Kandidat; endgültiger Basisendpoint und tatsächliche public_scopes müssen in Brainconfig und der vom Twitch-Integrator verwalteten Config übereinstimmen. Kein Versuch, belegt belegte Ports8787/8789 umzuwidmen.

## Nächste ausführbare Stufen und Rückweg

1. **B1-Quellvorbereitung läuft im bestehenden Sol-Thread:** fünf vorhandene Test-/Buildrunner auf expliziten vorhandenen Targetcache, einen Job und locked/offline binden. Kein Rustprodukt-/Lockdiff, keine Runzuteilung. Danach unabhängiger vorhandener Reviewer. Berichtsmatrix ae2abe1 ist gelesen: keine erneute Vollprüfung aller954 Standardfälle nötig.
2. **Gezielter Harnessslot:** acht bekannte PG-/Store-/SCRAM-/Restorefälle auf privatem Cluster, danach vier Pilotphasen. Der Serve-E2E führt fest600 Anfragen je8/16/32 Worker aus; dafür ausdrücklich Lastslot, nicht als leichter Prozesssmoke starten. Kein Neustart5446. Exakte Befehle folgen aus der statisch abgenommenen B1-Abgabe.
3. **Enger vorhandener Importer-Cutoverpfad:** oben genannte Guards und normale Produktionsconfig vorbereiten, unabhängig prüfen. Noch keine Schreibfreigabe oder Importausführung. Quelle, Zielrolle, aktuelle ACL-/Tombstonebehandlung und Quellrechte müssen ausdrücklich gebunden sein.
4. **Release-/Schreib-/Betriebsfenster einzeln beim Integrator:** bestehender serieller Workspace-Releasebuild, revisionsgebundenes Artefakt, Backup/Restore und dann vorhandener Import in den leeren Corestore. Kein Dummyrelease. Keine konkurrierende Auslieferung zu Twitch.
5. **Serve und vorhandener Consumer:** nach schreibender Importabnahme und Dienststart interne echte Anfrage über den gepinnten Client3b86d3cb des vorbereiteten Consumers fdd7a5d2. Öffentliche freigegebene Frage, Wegwerf-Conversation; keine ChatApi-Ausgabe. Natürliche spätere Ereignisse dürfen Replyversand belegen, bis dahin ausdrücklich nur interner End-to-End-Beweis.

Erster Serve-Rückweg: Serve stoppen/deaktivieren und beim Integrator den Chat-Port deaktivieren, kein Legacyfallback. Keine Daten/Ownership/Sperren zurückdrehen. Der vollständige G5-Rückweg bleibt zusätzlich kompatibles revisionsgebundenes Rückfallartefakt plus echtes Backup/Restore und Erhalt nachfolgender ACL-/Tombstone-/Write-Deltas. Ein leerer heutiger Corestore ersetzt diese Prüfung nicht. Fachwriterwechsel verlangt zusätzlich tatsächliches Fencing; für einen bloßen Serveanschluss keine alten Jobs abschalten.
