[Orchestrator] Abnahmeergänzung zum laufenden Audit B, kein neuer Auftrag.

# Empirischer Nachtrag des Hauptorchestrators zum YT-Bestand

status: aktiv, 07.10.2026

Der vermeintlich fehlende lesende Zugang ist aufgelöst. Bestehende lokale Peer-Verbindungen funktionieren ohne Credentials zu lesen, ohne Konfigurationsänderung und ausschließlich in `BEGIN READ ONLY ... ROLLBACK`.

## Gemessene Daten

1. Dedizierte Brain-Instanz: `psql -X -w -h /run/deadlock-brain-postgresql -p 5446 -U brain_migrate -d brain`.
   - `current_user=brain_migrate`, `current_database=brain`, `transaction_read_only=on`.
   - `brain.youtube_learning_claims` existiert dort nicht, der Bestand liegt in `brain_legacy`.
   - `brain_legacy.youtube_learning_claims`: **0 Zeilen**.
   - `brain_legacy.youtube_videos`: **195 Zeilen**.
2. Ursprüngliche zentrale DB: `psql -X -w -d deadlock`.
   - `current_user=nathanael`, `transaction_read_only=on`.
   - `brain.youtube_learning_claims`: **0 Zeilen**.
   - `brain.youtube_videos`: **195 Zeilen**.
   - `brain.youtube_transcript_claim_attempts`: **0 Zeilen**.
   - Videostatus: 176 queued/missing, 17 queued/ready, 1 failed/ready, 1 queued/unavailable.
   - `brain.entity_snapshots` für YouTube: **549 youtube_video-Snapshots**, keine Claimtyp-Treffer in dieser Abfrage.
   - `brain.insight_records`: **51 Zeilen**, Inhalt/Herkunft noch nicht geprüft. Spalten: insight_type, entity_type, entity_name, subject, summary, reason, validity_status, currentness, trust_tier, confidence, occurred_at, observed_at, source_urls text[], source_references jsonb, payload/metadata jsonb und Speicherzeiten.

Dies beweist nur die genannten aktuellen Tabellen, nicht die Abwesenheit früherer Klassifikationen in anderen Ablagen. Keine Behauptung von Datenverlust. Keine Tabellen wurden verändert, keine Backups geöffnet, keine Secrets oder private Nachrichten gelesen.

## Offene gezielte Prüfung für Audit B

- Den Datenzugangsblocker im Bericht korrigieren und die aktuelle leere Claimsablage ausdrücklich von vorhandenen öffentlichen Transkripten unterscheiden.
- Nachsehen, ob `insight_records` ausschließlich aus Patchdaten oder auch aus öffentlichen YT-Quellen stammt. Zunächst nur Quellen-/Typaggregate, keine ungefilterten potenziell privaten Texte.
- Alte klassifizierte Artefakte anhand der dokumentierten Kampagnen-/Exportpfade lokalisieren. Kein vollständiger Festplattenscan und kein Schreiben in Archive. Ein existierender Claims-JSON-Export mit öffentlicher Herkunft wäre ein echter klassifizierter Bestand; allein fünf VTT-Passagen sind es nicht.
- Falls echte Klassifikationen gefunden werden, wenige eindeutig öffentliche Entitybeispiele mit Modellurteil und Quellenrevision prüfen. Keine aktuelle Gültigkeit erfinden. Falls nichts auftaucht, geprüfte Orte und diese Grenze präzise melden.

Keine neue Pipeline, keine produktiven Writes, keine Importierung. Dieser Nachtrag schließt die bereits beauftragte empirische Bestandsfrage, er eröffnet kein anderes Feature.
