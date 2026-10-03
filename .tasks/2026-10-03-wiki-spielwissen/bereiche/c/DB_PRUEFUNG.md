status: aktiv
Datum: 2026-10-03
Prüfer: nativer frischer Datenbankprüfer a25927a502fdd41e2, Sol high über belegtes lokales Alias
Belegbasis: uncommittierte C-Integration auf 511a347b653beba13c2bf130f4bead7a7196cc2a

# SQL-Fachprüfung

Ein bestätigter blockierender Defekt: URL-basierte Wiki-ID ohne Seiten-ID mit belegter numerischer Revision 456 wird im Adapter lokal-monoton als Storeversion 1 geführt. Ein späterer Import der älteren Quellrevision 123 erhält Storeversion 2 und setzt den Head auf ältere Inhalte. Betroffene Pfade: `dbrain-sources/src/knowledge_import.rs:189-224`, `brain-storage/src/source_versions.rs:229`, bestehendes `PgStore::apply_connection` in `brain-storage/src/lib.rs:253`. Zeilen beziehen sich auf den Stand vor Formatierung.

Ein frischer Fixer a10f090c7d0b4f903 erhält ausschließlich diesen Befund und die bestehenden Verträge. Quellreihenfolge muss auch ohne page_id erhalten bleiben; keine Seiten-ID erfinden. Historie erhalten, Head nicht zurücksetzen. Der wartende eigene Check bmo6cu39b wurde vor Ressourcen-/Compilerstart beendet, um keinen überholten Stand abnehmen zu lassen. Keine fremden Compiler oder Lockdateien verändert.

Übrige bestätigte statische Ergebnisse: Semantische Gleichheit umfasst Fakten und Rechte; Beobachtungszeit allein bleibt Wiederholung. Konflikte verhindern sämtliche neuen Batch-Schreibzugriffe. Locks passen zur bestehenden PgStore-Identität und sind konsistent geordnet. Numeric-Wiki mit Seiten-ID erhält ältere Historie ohne Headrollback. SQL ist parameterisiert. Ungeprüfte oder nicht weitergebbare Inhalte werden nicht öffentlich hochgestuft.

Zusätzlicher Lesepfadbedarf: ChunkIndex indexiert den Rohtext, bisher nicht die nur in kanonischen Dokumentmetadaten gespeicherten Fakten. Ein getrennt beauftragter nativer Worker a85dc0ea797104370 bindet diese Fakten an den bestehenden Chunk-/Release-Lesepfad an. Rohquelle und Hash bleiben erhalten; Projektionsbytes werden ausdrücklich von Originalbytes unterschieden. Keine Parallel-Engine und keine höheren bestehenden Limits.

Prüfgrenze: statische lokale SQL-Fachprüfung, kein Gesamt-ALLOW, kein Gate-Ersatz. Keine Datenbank-, Laufzeit-, Parallelitäts- oder 7,5-MB-Ende-zu-Ende-Prüfung ausgeführt. Die Fix- und Reader-Ergebnisse benötigen neue Prüfung des fertigen integrierten Stands.

## Nachfolgender Stand

Fixer a10f090c7d0b4f903 ist beendet. Numerische Wiki-Reihenfolge wird unabhängig von einer belegten Seiten-ID erhalten; URL-IDs verwenden echte API-Herkunft ohne erfundene page_id. Tests für 456 vor 123, Wiederholung, Konflikte, Heads/Releasepins und Revisionsgrenzen sind geschrieben, noch nicht ausgeführt. Faktenworker a85dc0ea797104370 ist ebenfalls beendet; Projektion und sieben gezielte Tests sind geschrieben. Der Parent prüft Bibliotheken im eigenen Check b6eft4jvk unter beiden Hostlocks, noch kein Erfolgsnachweis.

Zusätzlicher bestätigter CLI-Befund beim Nachlesen: `brain-knowledge-import.rs` ruft zur Veröffentlichung `commit_batches_and_publish_checked(&[], ...)` auf. Die vorhandene Implementierung in `brain-storage/src/pg_release.rs:271-272` lehnt leere Batches unmittelbar ab. Der Publish-Unterbefehl kann so keinen Release veröffentlichen. Eine bloße Umstellung auf ungesicherte Veröffentlichung genügt nicht: genaue Quellköpfe und unberührte Basispins müssen transaktional erhalten bleiben. Der aktuell getrennte CLI-Worker partitioniert ausschließlich historische Eingaben; nach dessen Eigentumsfreigabe geht dieser Publish-Befund an einen frischen Fixer, nicht an den ursprünglichen CLI-Autor. Keine falsche Veröffentlichungsbehauptung.

