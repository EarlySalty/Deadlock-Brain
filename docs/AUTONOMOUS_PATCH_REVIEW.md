# Eigenständige Patchanalyse und Evidenzhistorie

## Patchanalyse

`deadlock-brain-patch-review history --query <Text>` durchsucht `brain.patch_history_v1` ohne Modellaufruf. `--entity` grenzt den exakten Entity-Namen ein, `--known-at` akzeptiert RFC3339 mit Zeitzone und `--limit` liegt zwischen 1 und 500. Keine Treffer bedeuten nur, dass der Text im erfassten Korpus nicht gefunden wurde.

`deadlock-brain-patch-review review --patch patch_<changelog_posts.id>` erstellt einen Kontextentwurf aus dem gespeicherten Originaltext, den Patch-Events und den vor dem Veröffentlichungszeitpunkt gespeicherten Asset-Snapshots. Patch-IDs müssen kanonisch sein: `patch_1` ist gültig, `patch_01` wird abgewiesen. Items und Fähigkeiten werden unter `item_or_ability` berücksichtigt. Das Kontextlimit wird vor einem Modellaufruf geprüft; Überläufe werden nicht still gekürzt.

Ohne `--generate` ruft `review` kein Modell auf. Mit `--generate` verwendet es den bestehenden `AiClient`. Der Validator prüft Reportstruktur und Referenzen auf gespeicherte Event-Hashes und Snapshot-IDs. Er beweist nicht, dass eine Schlussfolgerung fachlich stimmt. Der Bericht bleibt ein Entwurf. Nur `--write` speichert ihn in `brain.patch_review_runs`; es gibt keinen Veröffentlichungspfad.

## Zeit und Quellen

Die Migration `scripts/migrations/2026-10-01-patch-evidence-review-v1.sql` protokolliert Einfügen, Änderungen und Löschungen an Patchquellen. Inhaltsgleiche Reimports erzeugen keine neue Revision. `source_published_at` bezeichnet das Quelldatum, nicht den bestätigten Spiel-Rollout. `observed_at` bezeichnet die protokollierte Beobachtung. Eine Baseline belegt keine frühere Erstbeobachtung.

Die Migration korrigiert Patch-Knowledge-Events auf die erste Beobachtung des aktuellen Inhalts und hält das Quelldatum getrennt. Änderungen an Patch-Evidenz setzen gespeicherte Review-Entwürfe auf `needs_revalidation`. Das ist keine automatische Prüfung späterer, anderer Patches auf Folgewirkungen.

## Importierte Agenten-Insights

Der Insight-Importer speichert Agenten-Ableitungen als `curated_agent_inference` mit `needs_review`, unabhängig von angeforderter Trust-Stufe oder Confidence. Datumsangaben begrenzen den Bezug auf einen Patch. Andere Zeitangaben ergeben `needs_revalidation`. Confidence muss endlich und zwischen 0 und 1 liegen. Patch-Event-IDs werden positiv, eindeutig und sortiert aufgelöst. Ihre Quellen werden getrennt von externen, ungeprüften URLs gespeichert; Arraypositionen verbinden keine Ereignis-ID mit einer URL. Der Import materialisiert nur die gerade importierten Datensätze. Höher eingestufte vorhandene Datensätze werden nicht durch Agenten-Import überschrieben.

## Untertitel-Evidenz

Der Rust-Importer bewahrt das JSON3-Rohmaterial, Segmentzeiten, Wort-Offsets und getrennte SHA-256-Werte für Rohmaterial und normalisierten Text. Text und Evidenz werden gemeinsam gespeichert. Eine reine Zeitänderung erzeugt eine neue Evidenzfassung, auch wenn der Text gleich bleibt. `brain.youtube_caption_segments_v1` zeigt Segmente nur, wenn Roh-SHA-Zeiger und Text-SHA zum gespeicherten Transkript passen. Änderungen am Transkripttext setzen `needs_claim_revalidation`.

## Datenbankrollen und Betrieb

Die Migration erteilt `brain_service` lesenden Zugriff auf die Review-Eingaben und Evidenz-Views sowie Schreibzugriff auf Review-Entwürfe. `brain_ingest` erhält Einfügezugriff auf Caption-Evidenz. `ops/brain-postgres/grants.sql` enthält dieselben Rollenrechte für spätere Rechte-Neuvergabe.

Die Migration ist für eine kontrollierte Schemaänderung vorgesehen. Sie wurde in dieser Integration noch nicht gegen Scratch-Postgres ausgeführt. Vor einem Produktionslauf müssen Migration, Wiederholung, Rechte und Caption-Transaktion gegen eine isolierte Scratch-Datenbank geprüft werden. PR61 bleibt die gemeinsame Integrationsbasis; diese Änderung wird nicht isoliert nach `main` ausgerollt.

## Prüfumfang

Die Unit-Tests des Patch-Review-Binärziels prüfen unter anderem erfundene Referenzen, doppelte IDs, fehlende Bedingungen, nichtkanonische Patch-IDs und den Entwurfsstatus. Der Caption-Integrationstest benötigt Scratch-Postgres. Ein lokaler Offline-Build dieses Ziels benötigt SQLx-Metadaten für die vorhandenen Compile-Time-Abfragen.
