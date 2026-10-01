status: aktiv
Datum: 2026-10-01

# Review-Runden

## Runde 1

- Gate: `gpt-6.1-sol`, Basis PR61 `b687f613b3df2c49138d9d2837e005c33e646d9f`, geprüftes HEAD `95b15bb`.
- Urteil: BLOCK wegen Finding 1; zusätzlich NIT zur ausstehenden Scratch-Postgres-Prüfung.

### Finding 1: Nichtkanonische Patch-ID-Aliase umgehen Revalidierung

- Gate-Fundstelle: `rust/crates/deadlock-brain/src/bin/deadlock-brain-patch-review.rs:253`.
- Szenario: Eingabe `patch_01` wird als Post 1 geparst, aber in Advisory Lock, Review-Schlüssel, Ausgabe, Kontext und Revisionsabfrage unverändert verwendet. Migration und Trigger verwenden `patch_1`; Quellenänderungen können daher den Recheck verfehlen und aliasierte Entwürfe unverändert lassen.
- Korrektur: `parse_patch_id()` akzeptiert nur `patch_{id}` in kanonischer Dezimaldarstellung. Tests decken `patch_1`, `patch_01` und `patch_+1` ab.
- Ergebnis: behoben im Arbeitsstand; Gate-Folgerunde erforderlich.

### Finding 2: Datenbankverhalten nicht verifiziert

- Gate-Hinweis: Migration, Rollenrechte und Caption-Integration gegen Scratch-Postgres ausführen.
- Ergebnis: offen. `SQLX_OFFLINE=true cargo check ... deadlock-brain-yt --all-targets` stoppt an fehlendem SQLx-Cache für eine bestehende `query!`-Abfrage. Ein Build mit Datenbankzugriff wurde wegen der Vorgabe, keine ENV, Secrets oder Umgebungskonfiguration zu lesen, nicht versucht. Migration und Integrationstest wurden nicht ausgeführt.

## Runde 2

- Gate: `gpt-6.1-sol`, gleiche PR61-Basis, geprüftes HEAD `b12a5a8`.
- Urteil: ALLOW. Der Gate bestätigte, dass Alias-IDs vor Kontextaufbau abgewiesen werden.
- Geltungsbereich: Diese Runde lief vor der späteren selektiven Übernahme der Insight-Trust-Fixes und der aktualisierten Portierungsdokumentation. Sie ist deshalb kein Gate-Urteil über den aktuellen Gesamt-Diff.
- Offener NIT aus Runde 1: Scratch-Postgres-Prüfung für Migration, Rollenrechte und Caption-Integration.

## Runde 3

- Gate: `gpt-6.1-sol`, gleiche PR61-Basis, geprüftes HEAD `a27667b`.
- Urteil: BLOCK.

### Finding 1: Caption-Insert-Rechte reichen für den Konfliktpfad nicht aus

- Gate-Fundstellen: `ops/brain-postgres/grants.sql:37` und `scripts/migrations/2026-10-01-patch-evidence-review-v1.sql:313`.
- Szenario: `save_transcript` schreibt `youtube_transcript_evidence` mit `ON CONFLICT(video_id,source_kind,raw_sha256) DO NOTHING`. Der Gate beurteilt die alleinigen INSERT-Rechte als unzureichend für den Konfliktzielpfad; beim laufenden `brain_ingest`-Rollensatz könnte der Speichervorgang fehlschlagen und die Transaktion zurückrollen.
- Korrektur: In Migration und Grants-Skript sind SELECT-Rechte auf `(video_id, source_kind, raw_sha256)` ergänzt. Die Schreibrechte bleiben auf INSERT begrenzt.

### Finding 2: UPDATE archiviert die alte Patch-Zuordnung nicht

- Gate-Fundstelle: `scripts/migrations/2026-10-01-patch-evidence-review-v1.sql:149`.
- Szenario: ein `patch_events`-Datensatz wechselt auf eine andere `patch_external_id`. Der Trigger invalidiert beide Entwürfe, protokolliert aber nur den NEW-Payload. Der alte Patch erhält keine neue Revision; ein danach ausgelöster Recheck kann den veralteten Kontext weiterhin als aktuell bewerten.
- Korrektur: Der Trigger sperrt die alten und neuen Patch-Schlüssel in sortierter Reihenfolge und schreibt für die alte Zuordnung eine `deleted`-Revision, bevor er die neue Zuordnung protokolliert. Die Revisionsabfrage wertet den Payload der alten Zuordnung aus.

### Finding 3: Scratch-Postgres-Prüfung offen

- Gate-Hinweis: Caption-Code bleibt unkompiliert, Datenbankverhalten ungeprüft.
- Ergebnis: offen. SQLx-Offline-Metadaten fehlen für eine bestehende Compile-Time-Abfrage. Keine ENV, Secrets oder Umgebungskonfiguration werden gelesen; daher wurde kein Scratch-Postgres aufgerufen.

### Finding 4: Importstatus-Dokumentation ungenau

- Gate-Fundstelle: `docs/AUTONOMOUS_PATCH_REVIEW.md:19`.
- Szenario: die Doku sagt, alle importierten Insights erhalten `needs_review`; explizit als `rejected` markierte Eingaben behalten laut `prepare_input()` ihren abgelehnten Status.
- Korrektur: `docs/AUTONOMOUS_PATCH_REVIEW.md` nennt `rejected` als Ausnahme zu `needs_review` und beschreibt die Konfliktschlüssel-SELECT-Rechte.

## Runde 4

Nach Commit und Push der drei Korrekturen Gate erneut mit `gpt-6.1-sol` gegen dieselbe PR61-Basis ausführen. Scratch-Postgres und SQLx-Offline-Cache bleiben als offene NITs dokumentiert. Kein Merge, keine Produktionsmigration und kein Deploy vor ALLOW und Abschluss der Gesamtintegration Brain61.
