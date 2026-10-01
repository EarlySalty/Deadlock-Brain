# V1: bestehende interne Archivübernahme konkretisieren

Diese Betriebsentscheidung setzt den Nutzerauftrag vom 1. Oktober 2026 um:
„Delegiere bitte im Fertigbau von dem deadlock brain“, zusammen mit dem angehängten
Statusbericht `Deadlock-Brain-Status-2026-10-01.md`. Dieser fordert ausdrücklich,
vorhandene Datenübernahmeentscheidungen an Snapshot, Quellenrechte und
Aktiv-/Widerrufs-/Löschinventar zu binden und den vorgesehenen Core-Import auszuführen.

Vertragsbasis ist `88553d6179eef9fc32c08d36c2adbf80adf57557`:

- `architecture/migration/LEGACY_CORE_MIGRATION.md` ordnet die vier Tabellen
  `entities`, `entity_aliases`, `patch_events`, `patch_event_enrichments` dem Core zu.
- `.tasks/2026-09-30-direct-completion/REPORT.md` bestätigt die bestehende interne
  Übernahmeentscheidung und hält Egress-/Publikationsrechte geschlossen.
- `ops/brain-postgres/legacy-core-cutover.json` legt die unveränderten Grenzen fest.
- `brain-legacy-import/src/cutover.rs` bindet genau diesen Umfang an reale
  Schema-/Datenbank-/Snapshotfingerprints und vollständige Zustandsinventare.

## Unveränderte Grenzen

| Quelle | Zugriff | Providerweitergabe | Externe Veröffentlichung | Rohaufbewahrung |
| --- | --- | --- | --- | --- |
| legacy-entities | public, ausschließlich game.public | nein | nein | ja |
| legacy-patchnotes | private, ausschließlich brain.legacy.review | nein | nein | ja |

Die Einordnung umfasst vorhandene Archivdaten für interne Verarbeitung und
belegt keine zusätzliche Drittanbieter-Lizenz, aktuelle Spielgültigkeit oder
öffentliche generative Nutzung. Forum-Claims, Sheets, Transkripte, Replaydaten und
abgeleitete Modellnotizen sind nicht Bestandteil dieser Übernahme.

## Verbindliche Konkretisierung vor dem Lauf

Die tatsächliche private Autorisierungsakte referenziert dieses versionierte
Dokument und den Nutzerauftrag; sie enthält die neue `--observe-snapshot`-Ausgabe
mit exakten OIDs, Label, Epoch, Schema-/Snapshot-SHA256 und Tabellenzahlen.
Alle beobachteten IDs werden explizit als aktiv, widerrufen oder gelöscht
klassifiziert. Der SHA256 der vollständigen Akte ist `approval_ref` und wird als
`authorization_ref` in beiden SourcePolicies eingetragen. Der Policyfingerprint
bindet diese Policies und die drei disjunkten Inventare im bestehenden Format.
Erst diese vollständig gebundene Config darf importiert werden.

Die bisherige leere Zieldatenbank besitzt keine widerrufenen oder tombstonierten
Core-Heads. Das Archivschema hat keine eigenen Widerrufs-/Löschspalten; bei der
lesenden Prüfung am 1. Oktober 2026 waren in Entity-Metadaten keine solchen
Zustandsfelder vorhanden, in Patch-Metadaten fand die entsprechende Schlüsselsuche
nur `steam_author`. Das ist ein begrenzter lokaler Inventarbefund und kein Beweis
für den externen aktuellen Rechtsstatus. Etwaige vorhandene separate Widerrufs-
oder Löschakten haben Vorrang vor der Aktivklassifikation. Unbekannte Lizenzen
bleiben unbekannt und geschlossene Egress-/Publikationsrechte bleiben geschlossen.

Das Dokument selbst enthält keine erfundenen Snapshotwerte und keine
vorweggenommene Liste aktiver IDs. Die neue Beobachtung und ihre konkrete
Inventarklassifikation werden vor Import unabhängig geprüft. Wiederholungen
müssen denselben Release erzeugen; neue Zustände erfordern eine neue Bindung.
