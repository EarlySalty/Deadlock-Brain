# Unabhängiges Release-Review R5/R6

## R5: Bestehender Parserbestand wird nicht entfernt

Lesend auf Produktion belegt (18.09.2026): ALLE 123 Events von Patch 285 haben KEIN `metadata.importer`. GROUP BY COALESCE(metadata->>'importer','<none>'),source_kind ergab `<none>|steam|123`.

Die obsolete Abrams-Zeile ist id422423, patch_snapshot_id581597, event_hash4c573bc685b9ed5a8f86fb11d431a894057047636ad1bee9b7002262dc124b30, metadata={"line_subject":"Abrams","original_bullet":"Abrams: Base gun damage increased by 5%","source_language":"en"}. Snapshot581597 ist die vorhandene Patchnote-Quelle.

`prune_direct_patch_events` filtert dagegen ausschließlich metadata.importer=deadlock_patchnotes_db_pg. Der neue Sync würde den echten alten Bestand deshalb nicht entfernen; die offiziell entfernte Zeile bliebe für den Review sichtbar. Diese realen Zeilen sind bereits in brain_release_validation_20260918 enthalten. Regression bitte genau damit/synthetisch äquivalent prüfen: nach offiziellem Refresh+Sync ist die obsolete Zeile aus aktiven Events weg, aber als Quell-/Eventrevision erhalten. Nur nachweislich zu derselben Patchquelle gehörende Altparser-Events abgleichen (über patch_snapshot_id -> snapshot.source/entity_type/payload.id + exakte URL); keine fremden Insights/anderen Patchquellen pauschal löschen. Keine titelbasierte globale Bereinigung.

## R6: Drift muss Parserstand prüfen, nicht irgendeinen Quellsnapshot

`detect_patchnote_drift` vergleicht derzeit c.raw_content mit dem jüngsten entity_snapshots-Eintrag. Der bestehende Timer ruft `pull patchnotes` VOR `parse patchnotes`; der Quellsnapshot kann also schon neu sein, während alle Events noch aus der alten Fassung stammen. Entsprechend muss `build_context` vor einem Modellaufruf sicherstellen, dass Quelle und Eventbasis wirklich zusammengehören. Sonst entsteht ein widersprüchlicher Kontext mit neuer Originalfassung plus alter Eventzeile.

Bitte erfolgreicher Parse-/Import-Fingerprint (einschließlich URL, Veröffentlichung und Parserfassung soweit vorhanden) über den bestehenden Lauf-/Importpfad nachvollziehbar speichern oder den Event-Snapshot-Stand prüfen; alle Transaktionen gegen parallele Quelländerungen schützen. Nur `max(fetched_at)` irgendeines Quellsnapshots ist kein Abschlussnachweis. Für leere gültige Patches muss ausdrücklich ein abgeschlossener Null-Event-Lauf unterscheidbar bleiben. Regression: neue Quelle und neuer Rohsnapshot, aber alte Events -> check drift=true und review blockiert; nach vollständigem Import -> drift=false; zweiter identischer Sync schreibt keine neuen Events/Reviews. Auch reine URL-/Publikationskorrekturen beachten, nicht nur raw_content.

## Weitere Abnahmegrenzen

Der aktuelle 734519-Byte-Kontext kann ein technischer Fortschritt sein, aber nur 20 direkt genannte Helden + ihre Kernfähigkeiten reichen noch nicht für indirekt betroffene UNVERÄNDERTE Helden. Allgemeine Systemänderungen brauchen belegte Gegenüberstellung zu den nicht erwähnten Kits/Mechaniken. Ein Katalog mit Namen allein ist kein Gameplay-Nachweis. Keine festen Creator-Hero-Urteile einbauen.

Einer der ignorierten YT-Tests (`claims::tests::query_claims_returns_rows_for_known_entity_parity`) erwartet einen historischen Snapshot mit exakt11941 Claims und7 Walker-Treffern. Dieser Creator-Bestand wurde bewusst NICHT in die primäre Referenz-DB geladen. Nicht durch Abschwächen der Assertion grün machen. Einen eigenständigen deterministischen Fixture-Test für denselben Query-/Filtervertrag ergänzen; historische Korpusparität getrennt ehrlich als nicht ausgeführt nennen, wenn der exakte alte Snapshot fehlt.

R4 ist vom Orchestrator auf vollständigem realem Schema mit der dritten Migration bestätigt: beide Reviews needs_revalidation, Exit0, Probe zurückgerollt. PR5 wurde als Draft für diesen Release-Branch angelegt und triggert jetzt GitHub-CI. Keine Produktionsmigration, kein Deploy, kein Modelllauf bisher.
