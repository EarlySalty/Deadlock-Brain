# CR-13-001: Voraussetzungen für externe Quellen und Schemaüberwachung

Status: vorgeschlagen, nicht genehmigt.
Antragsteller: S13. Entscheider: S00. Fachbesitzer: S02/S03/S04, mit S05/S12/S14.
Prüfbasis: `30326512568b7370524956839100462ba71bdb92`.
Arbeitsmodus dieses PRs: `prepare_only`, ausschließlich S13-Übergabeartefakte.
Betroffene Anforderungen: R32, R38, R39, R42–R48, R52, R54, R55, R58, R60.

## Anlass und Grenze

G0 und G1 sind im integrierten Status offen. Weder gemeinsame Contract- noch Schema-Version ist festgelegt. Der bestehende SourceStore hat bereits Rawdateien, Source Documents, Entity Snapshots und Source Runs. Ihn ersetzen oder einen lokalen S13-Vertrag als Runtimevertrag einführen wäre falsch.

Die folgenden Punkte sind Anforderungen an die zuständigen Besitzer, keine freigegebenen Rust-Typen, SQL-Spalten, Wire-Feldnamen oder neue Betriebsdienste. Die endgültige Form entsteht in S02/S03 und wird integriert, bevor S13 Produktionsmodule ändert.

## S02: Gemeinsamer Source-v2-Vertrag und HTTP-Port

Der Vertrag muss stabile Source-/Recordidentität und die getrennten Revisionen der Quelle, Rohdaten, Parser, Generatoren und Normalisierung ausdrücken. Er muss einen unveränderten Rawhash samt Hashalgorithmus und auffindbarem Rawartefakt von einem optionalen normalisierten Inhaltshash unterscheiden. Nicht verfügbare Originalbytes dürfen nicht durch neu serialisiertes JSON als angeblich identische Rohquelle ersetzt werden.

Abrufzeit, beobachtete Verfügbarkeit, Upstream-Publikation, Git-Commitzeit und belegte Spiel-/Patchgültigkeit sind getrennt. Locale, Mode, Client-/Gameversion und gültiges Datenfenster bleiben unbekannt, wenn die Quelle sie nicht belegt. Keine Ableitung der Patchgültigkeit allein aus Gitzeit oder API-Abrufzeit.

Für Korrelationen werden Upstreamreferenzen, gemeinsame Ursprungsartefakte und Parser-/Generatorfamilien benötigt. Unbekannte Herkunft ergibt nicht automatisch Unabhängigkeit. Die Identität des Generators muss eine konkret auflösbare Revision unterstützen; eine Abkürzung aus einer Commitnachricht ist nur ein Hinweis.

Datenklasse, Zugriffsrechte, Speicherung, Fixtureweitergabe, Veröffentlichung und Provider-Egress brauchen getrennte prüfbare Entscheidungen mit Herkunft. Ein Repository-Code-Lizenzlabel ist kein Ersatz. Rechteentscheidungen müssen beim Abruf beziehungsweise vor der jeweiligen Verwendung geprüft werden, nicht nur als Freitext im Report stehen.

Prüfstatus muss mindestens erfolgreiche Validierung, unvollständige Daten, Quarantäne, unbekannte Semantik und Abruffehler unterscheidbar machen. Erreichbarkeitsfehler sind kein Schemanachweis. Ein HTTP-200 allein ist kein gültiger Record. Fehlende Pflichtfelder und IDs dürfen nicht in erfundene Positions-IDs oder bestätigte Nullwerte umgewandelt werden.

Im vorhandenen `deadlock-brain-core`-HTTP-Port klären: begrenzte unveränderte Responsebytes, Status, relevante Header, Timeout-/Retry-/Backoff-Kontrolle, Cacheherkunft und abrufbare Schemafingerprints. Bereits vorhandene Fähigkeiten zuerst nachweisen und verwenden. Keine zweite reqwest- oder Providerimplementierung in S13. Die Limits stammen aus dem gemeinsamen Budgetprofil, nicht aus einer unbelegten S13-Leistungszusage.

## S03: Provenienz und Revisionshistorie im vorhandenen Store

`SourceDocumentInput`, `SourceStore`, `brain.source_documents`, `brain.entity_snapshots` und `brain.source_runs` bleiben der Ausgangspunkt. Die Integration muss alte Beobachtungen erhalten, parserabhängige Revisionsstände unterscheidbar speichern und Wiederholungen derselben Arbeit idempotent behandeln. Idempotenzschlüssel und Migrationen legt S03 fest; hier wird keine bereits angewandte Migration verändert.

Benötigt werden ein explizites Staging-/Freigabeverhalten und eine nachvollziehbare Quarantäne für betroffene Daten. Ein halber Import oder abgebrochener Lauf darf nicht als vollständiges neues Knowledge-Release sichtbar werden. Reparse bei unveränderten Rawdaten ist eine neue Ableitung, kein behaupteter Spielpatch. Last-good-Daten bleiben nur dort verwendbar, wo Zeitgültigkeit und aktuelle Rechte das erlauben.

Schemaabhängigkeiten müssen vom tatsächlich konsumierten Upstreamfeld über Parser/Fact/Observation/Karte/Regel bis zur Publikation verfolgbar sein. Lokaler Fehler darf nicht blind sämtliche Daten invalidieren. Umgekehrt dürfen betroffene Ableitungen nicht unverändert als aktuell publiziert werden. Widersprüche erhalten eine Reconciliation-Referenz, statt bestehende Belege destruktiv zu überschreiben.

Rawartefakte liegen außerhalb Git mit einem geprüften Speicher-/Retention-/Restorepfad. Eine flüchtige `/tmp`-Datei und ein Hashmanifest ersetzen keinen reproduzierbaren Fixturebestand. Für Datenlöschung und Rechtewiderruf gilt die gemeinsame Datenpolitik.

## S04: Gemeinsame Jobs statt zweitem Scheduler

S13 benötigt Jobs für begrenzten Quellenabruf, commitbezogenen Gitimport, Reparse und getrennte Upstreambeobachtung. Checkpoint, Retry, Backpressure, Ressourcenlimit, Abbruch und Status gehören in die vorhandene Jobengine. Ein Quellenausfall darf keine erfolgreiche leere Version erzeugen. Keine automatischen Dependency-Updates oder fremden Downloads im deterministischen PR-Gate.

Gitjobs müssen eine konkrete Revision lesen, statt eine laufende Arbeitsbaumänderung als dieselbe Revision auszugeben. Shallow-Grenzen, fehlende Archive, Rename/Delete und Merge-Eltern sind Teil des Importnachweises. Ein Zielcommit und die importierte Historienabdeckung müssen nach einem Neustart wiederherstellbar sein.

## S05 und S14: Fachliche Übergabe

S13 liefert belegtes Datenfenster, Filter/Kohorte, Patch/Mode soweit bekannt, Quellstatus, Coverage/Missingness sowie stabile Match-/Playerreferenzen. S05 entscheidet über Population, Ranking und fachliche Interpretation. Item-Winrate wird nicht zu einer mechanischen Regel oder Kausalitätsbehauptung.

S14 besitzt Replaydecoder und seine Capability-Matrix. SchemaExplorer/GameTracking/Protosignale von S13 sind Inputs mit Herkunft, keine vollständige Parserfreigabe. Ein Schemawechsel muss den tatsächlich konsumierten Feldern zugeordnet werden. Referenzparser mit demselben Upstream zählen nicht als unabhängige Bestätigung.

Offizielle Patchangaben, Entwicklerkontext, Spielzustand, Beobachtungen, Population und Community-Claims bleiben getrennte Evidenzklassen. Ein Importer-Flag `trusted` ist keine allgemeine Fakten- oder Rechtefreigabe.

## Vorgeschlagene Pfadentscheidung durch S00

| Pfad | Vorschlag, noch keine Schreibfreigabe |
|---|---|
| `rust/crates/dbrain-sources/src/assets_api.rs` | S13 nach G1; gemeinsame HTTP-/Storeänderungen separat über S02/S03 |
| `rust/crates/dbrain-sources/src/deadlock_data.rs` | S13 nach G1; Fachprojektionen mit S05 koordinieren |
| `rust/crates/dbrain-sources/src/deadlock_api.rs` | Ein einziger Integrationsbesitzer muss benannt werden. S13 spezifiziert Metadatenfeeds, S14 Demo-/Replaypfad; keine parallelen Schreibsessions in derselben Datei |
| `rust/crates/dbrain-sources/src/store.rs` | S03-Vertrags-/Persistenzänderungen mit bisherigem Besitzer S04 abstimmen |
| `rust/crates/dbrain-sources/src/lib.rs` und Cargo-Dateien | Export-/Workspaceänderungen durch zuständigen Integrationsbesitzer; kein eigenmächtiger S13-Workspaceumbau |
| Neue Schema-/Diff-Teilmodule innerhalb `dbrain-sources` | S00/S04 reservieren nach Bestandssuche; hier kein zweiter Runtimepfad angelegt |
| `architecture/migration/handoffs/s13/**` und S13-Übergabe | Dokumentation dieses Vorschlags; keine Änderung zentraler Gates, Owner oder STATUS |

## Abnahme dieses Change Requests

S00 bestätigt die Pfadverantwortung, G0 und anschließend G1 anhand integrierter Commits. S02 liefert die versionierten gemeinsamen Ports samt Tests, S03 den getesteten Schema-/Storestand, S04 den nutzbaren Jobport. Die jeweiligen PRs müssen Basis, Test-SHA, erlaubte Daten und Grenzen benennen.

Erst danach rebasiert S13 auf diese integrierte Basis, ersetzt die noch offenen Contractannahmen durch die echten Typen und führt `AKZEPTANZFAELLE.json` gegen gehärtete Adapter aus. Für die neun bestehenden Assets-Routen ist eine vollständige Zuordnung nötig; die ungeklärten Rawrouten bleiben Pflichtlücken. Die Vorbereitung selbst genehmigt weder G1 noch eine Quellen-/Fixture-/Publikationsfreigabe.
