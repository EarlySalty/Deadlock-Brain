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

## Folgerunde

Nach Commit des Alias-Fixes denselben Gate-Aufruf mit demselben Modell gegen dieselbe PR61-Basis erneut ausführen. Kein Merge, keine Produktionsmigration und kein Deploy vor ALLOW und Abschluss der Gesamtintegration Brain61.
