# Übergabe · S03 / Storage und Datenmigration

Status: **blockiert vor Implementierung**. Geliefert ist ausschließlich eine belegte Startprüfung mit Schnittstellenanforderung und Abnahmematrix. Kein abgeschlossenes S03, kein Schemaentwurf mit Freigabe, kein erfolgreicher Pilot.

Datum: 24.09.2026
Basis-Commit: `c00fc8935048bf490c1e4790f7c6195864ad49e2` (`origin/main` beim Start)
Ergebnis-Branch: `migration/s03-storage-preparation-20260924`
Ergebnis-Commit / PR: nach Commit und Push im PR-Abschlussprotokoll dieses Branches dokumentiert.
Tatsächlich geprüfter Code-Commit: Basis-Commit; keine Rust-, SQL- oder Runtimeänderungen.
Contract-Version: offen. `SourceRecordV2` ist eine Plananforderung, kein integrierter Vertrag.
DB-Schema-Version: offen; die angewandte Produktionsversion wurde nicht abgefragt.
Source-/Corpus-/Modellversion: kein freigegebener Snapshot, kein geprüftes Knowledge-Release, kein Modellaufruf.
Arbeitsart: read-only Bestandsprüfung plus eigene Übergabe; **keine** Implementierungsfreigabe.
Plan: hochgeladenes Planpaket v1.0 vom 24.09.2026, insbesondere Chat 03 sowie Kapitel 02, 03, 06, 07 und 10–13.

## Ergebnis und konkrete Änderungen

Nur diese S03-Artefakte werden hinzugefügt:

- `architecture/migration/handoffs/03-storage-datenmigration.md`: Voraussetzungen, Bestandsbelege und Übergabe.
- `architecture/migration/handoffs/03-storage/CR-S03-001.md`: Antrag an 00/02 für Gate-, Pfad- und Vertragsklärung.
- `architecture/migration/handoffs/03-storage/PRUEFMATRIX.csv`: 16 geplante Abnahmefälle, alle ausdrücklich `not_run`.

Keine Änderungen an STATUS, Gateentscheidungen, Ownerregister, gemeinsamem Workspace, Lockfile, CI, anderen Arbeitspaketen oder Produktionsdaten. Keine Abhängigkeiten hinzugefügt oder entfernt. Kein Merge, Deploy oder Dienstneustart.

## Startbedingungen: tatsächlicher Stand

| Voraussetzung | Beleg | Urteil für S03 |
|---|---|---|
| G0 | `architecture/migration/STATUS.md` und `GATES.csv` im Basis-Commit: offen | Schemaentwurf nach Plan noch nicht freigegeben |
| G1 / Contracts | STATUS: Contract- und Schema-Version offen; G1 offen | Store-/Adminimplementierung gesperrt |
| Eigentümerschaft | `PFAD_OWNER.csv`: S03 `unresolved_in_S000`, `pending_inventory`; Core bei 02, Sources bei 04/13 | Keine freie Wahl neuer Crates oder Migrationsnummern |
| S01 | Übergabe auf gesondertem Stand `76205b5`, Inventar-Grundlage `2734c2d`; nicht Bestandteil dieses Basis-Commits | Hilfreicher Eingang, kein integriertes G0 und kein freigegebener Snapshot |
| Pilot | Kein gehashter und freigegebener Pilotsnapshot oder Restoreprotokoll an S03 übergeben | Keine echten Import-/Verify-Ergebnisse möglich |
| Repository-Sichtbarkeit | GitHub-Metadaten beim Start: `EarlySalty/Deadlock-Brain` ist öffentlich | Nur Codebezüge und bereinigte Prüfunterlagen veröffentlichen, keine Quellinhalte |

Das S01-G0-Protokoll nennt seinerseits fehlende koordinierte Freigabe und Baseline. Seine Runtimebeobachtungen und Ortsangaben wurden hier nicht als aktuelle Produktionsfakten übernommen. S03 integriert weder S01 noch S02 und setzt kein Gate selbst auf grün.

## Wiederverwendbarer Bestand und nachzuweisende Lücken

Alle Brain-Codebezüge beziehen sich auf den oben genannten Basis-Commit. Sie belegen Code, nicht Produktionsdaten oder erfolgreiche Ausführung.

| Bereich | Konkreter Codebeleg | Bedeutung für die spätere S03-Arbeit |
|---|---|---|
| Zentraler PostgreSQL-Zugang | `rust/crates/deadlock-brain-core/src/pg.rs:11–61`, `pg_pool_read_only`, `pg_pool_from_config`, `pg_pool` | Vorhandenen Zugang verwenden; kein zweiter Datenbank-/Secretpfad. Die Betriebs- und Testkonfiguration bleibt gesondert zu prüfen. |
| Quellenläufe | `rust/crates/dbrain-sources/src/store.rs:39–73`, `begin_run` / `finish_run` | Bestehende `brain.source_runs` berücksichtigen. Start-/Endstatus allein belegt keine Lease und keinen wiederanlaufbaren Cursor. |
| Rohobjekte | `store.rs:75–92`, `write_raw` | Dateischreiben und Existenzprüfung sind vorhanden. Ein existierender Pfad wird in diesem Pfad nicht erneut gehasht. Crashfestigkeit und unveränderte Bytes sind kein belegter Betriebszustand. |
| Dokumentrevisionen | `store.rs:94–145`, `upsert_source_document` | Vorhandener Schlüssel `(source, external_id, content_hash)` und bestehende IDs müssen abbildbar bleiben. Derselbe Hash darf Quellenberechtigungen nicht verschmelzen. |
| Entitätssnapshots | `store.rs:147–208`, `upsert_entity_snapshot_id` | Vorhandener Schlüssel `(source, entity_type, external_id, payload_hash)` und `source_document_id` sind Migrationsinput. Hashgleichheit ersetzt keine Parser-, Policy- oder Gameversion. |
| Batches | `store.rs:210–223`, `insert_many_snapshots` | Schleife ruft einzelne Pooloperationen auf. Ihr Zähler zählt durchlaufene Eingaben, nicht nachgewiesene Neuimporte. Kein Source-zu-Ziel-Abgleich daraus ableiten. |
| Gemeinsame Commitgrenze | `store.rs:39–223` | Im untersuchten Storepfad sind Rohdateischreiben, Datensätze und Runabschluss nicht als gemeinsame Crash-/Checkpointgrenze umgesetzt. Das ist ein konkret zu testender Integrationsbedarf, keine pauschale Aussage über jeden anderen Pfad. |
| Startseitige DDL | `store.rs`, `open_pool`, `ensure_patch_changes_view`, `PATCH_CHANGES_VIEW_SQL` | Poolöffnung kann eine View erneuern. Nicht ungeprüft als lesendes Inventory-/Verify-Werkzeug ausführen. |
| Vorhandene Migrationen | Separates Repo `EarlySalty/Deadlock-Bots`, `rust/crates/dl-central-db/migrations/0012_brain_knowledge_timeline.sql`; letzter Datei-Commit `f6eea340a25770d680ad7ac4fc4a50c314636211`, Arbeitsdatei gegenüber HEAD unverändert | Reale Migrationshistorie existiert bereits außerhalb des Zielrepos. Keine Kopie als zweiter Migrationseigner, kein neuer Nummernkreis ohne 00/02. Diese Datei beweist nicht den heutigen vollständigen oder angewandten DB-Stand. |

Graphify wurde zuerst zur Bestandssuche verwendet; relevante Treffer wurden anschließend am Code geprüft. Der globale Graph war gekürzt und meldete ein altes ID-Schema. Deshalb werden daraus keine Vollständigkeits- oder Abwesenheitsbehauptungen abgeleitet.

## Vor dem Schemaentwurf zu klärende Regeln

Dies sind aus dem Plan abgeleitete Abnahmeanforderungen, **keine neuen privaten DTOs, Ports, SQL-Tabellen oder beschlossenen Speicherformate**.

Logische Quellidentität, konkrete Inhaltsrevision, Parserrevision und fachliche Gültigkeit müssen getrennt sein. Die Zuordnung alter IDs darf nicht verloren gehen. Ein identischer Raw-Hash bei geändertem Parser muss neu auswertbar sein, ohne einen Spielpatch zu erfinden. Quell-, Beobachtungs-, Verfügbarkeits- und Ingestzeit bleiben unterscheidbar; unbekannte Werte bleiben unbekannt.

Rohdatei, referenzierende Datensätze, Checkpoint und ausstehende Veröffentlichung brauchen eine überprüfbare Wiederanlaufgrenze. Mit 04 sind Lease, Fencing, Eventreihenfolge und Duplicate-Behandlung festzulegen. Ein Zeitstempel allein ist kein freigegebener Eventoffset. Ein verspätetes Update darf eine neuere Löschung nicht rückgängig machen; eine explizite spätere Wiederanlage benötigt eine eigene zulässige Semantik.

Aktuelle ACL-/Delete-Sperren müssen auch historische Releases, Caches, Zitate und Providerübergaben erreichen. Generierte Daten dürfen Elternrechte nicht erweitern. Öffentliche Code-Sichtbarkeit erlaubt weder interne Inhalte noch private Replays oder Modell-Egress.

Wiki-/Git-Historien, Facts/Effects/Rules, Aliase, Ableitungsfamilien, Karten, Schemaänderungen, Replayobservations und Population sind in derselben Migrationsentscheidung und demselben Knowledge-Release zu berücksichtigen. Große Rohobjekte bleiben außerhalb öffentlicher Git-Artefakte. S03 übernimmt keine Parser-, Worker-, Such- oder Domainimplementierung der anderen Besitzer.

## Nachweise und Messgrenzen

Durchgeführt: Git-Basis- und Worktreeprüfung, lesende Codeprüfung, GitHub-Sichtbarkeitsabfrage, Vergleich der gefundenen externen Migrationsdatei mit deren HEAD. Keine Secrets abgerufen, kein Brain-Binary und kein Source-Connector gestartet, keine Datenbankverbindung hergestellt.

Die Dokumentprüfung vor dem Commit umfasst den expliziten Dateiumfang, UTF-8/CSV-Struktur, eindeutige Prüf-IDs, den Status `not_run` aller 16 Fälle sowie `git diff --cached --check`. Konkrete Ergebnisse und der danach geprüfte Head-SHA stehen im PR-Abschlussprotokoll; ein fehlender Actions-Check wird nicht als Erfolg gewertet.

Nicht ausgeführt: Rust-Compiler, rustfmt, Clippy, Unit-/Integrationstests, Migration, Verify, Restore, Replay, Rollback, Lasttest oder negativer Laufzeittest. Geänderter Produktivcode: 0 Dateien. Die CSV ist ein Testauftrag und **kein** Testergebnis. Es gibt weder `mock_verified` noch `integration_verified`.

Im Basis-Commit sind unter `.github` keine versionierten Workflowdateien enthalten. S03 ergänzt dafür keine fremde CI. Der aktuelle PR-Checkstand muss nach Erstellung gesondert geprüft und als vorhanden, fehlend oder blockiert protokolliert werden.

## Folgen

Datenmigration/Kompatibilität/Wiederanlauf: unverändert; keine Daten migriert und keine Kompatibilität bewiesen.
Berechtigungen/Secrets/Egress: keine Erweiterung, keine Inhalte oder Zugangsdaten exportiert.
Latenz/Ressourcen/Kosten: keine Laufzeitänderung oder Messung, kein Performanceversprechen.
Bestehende Funktionen: unverändert; keine fachliche Abweichung eingeführt.
Python-/Legacyfreiheit: keine neue Runtime oder Hilfsskript-Abhängigkeit; ein Python-freier Full-Rebuild ist noch nicht nachgewiesen.

Berücksichtigte Anforderungen, nicht abgenommen: R03–R09, R17, R18, R22–R24, R28, R29, R32, R34, R40–R43, R46, R49, R51, R60. R04 ist ausschließlich durch das Nichtanlegen einer neuen Betriebsabhängigkeit berührt, nicht insgesamt erfüllt.

## Übergabe an nächsten Besitzer

**00 und 02:** [CR-S03-001](03-storage/CR-S03-001.md) entscheiden. G0 mit S01/S10 integrieren; danach mit S03 den tatsächlichen Schema-/Migrationsbesitz und die gemeinsamen Verträge festlegen. Erst ein integrierter G1-Commit mit ausdrücklicher S03-Pfadfreigabe eröffnet die Implementierung.

**03 nach Freigabe:** Auf genau diesen integrierten Commit wechseln, bestandserhaltenden Schemaentwurf abstimmen und dann Migration plus Verify für einen kleinen freigegebenen Snapshot in einer isolierten Testdatenbank implementieren. [Prüfmatrix](03-storage/PRUEFMATRIX.csv) ausführen und echte Ergebnisse ergänzen. Inventory/Export/Rebuild/Replay und der Pilot brauchen die freigegebenen Ports; Vollbestand erst nach G2. Keine Produktionsumschaltung im S03-Auftrag.

## Integration durch Chat 00

Merge-Commit: offen.
Gate-/STATUS-Änderung: keine.
Freigegeben von: niemand; keine neue Arbeitswelle freigegeben.
Sensible Quellen-/Replay-/Publikationsrechte: nicht geprüft; Pflichtlücken bleiben offen.
