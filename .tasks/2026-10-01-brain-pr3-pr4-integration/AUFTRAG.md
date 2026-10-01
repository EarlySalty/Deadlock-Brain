status: aktiv
Datum: 2026-10-01

# Auftrag: Brain PR3 und PR4 integrieren

## Ziel

PR3 und PR4 als zusammenhängenden Stand für eigenständige Patchanalyse, Caption-Evidenz mit Zeitsegmenten und Trust sowie gemeinsame Änderungshistorie integrieren. Die Integration wird auf den gemeinsamen Brain61-Freeze `2e9ade05015c71252e8800d61525cc8d69131c2e` abgestimmt. `origin/main` `084cdfc80d48f6f1659fc764955d7f941485e6bf` bleibt Herkunft der PR3/PR4-Aufarbeitung, aber nicht die alleinige Abnahmebasis.

## Umfang

- PR3-Eigencommits: `6b93f58af60ec612ff56631b8251a5601e257aea`, `089be38c5e4d755abb59952ba74d4e1cdfc86378`.
- PR4-Eigenstand: `9efeb1e44ead5cdf5d01e05f242291fee79e803e` und seine PR4-Vorfahren seit gemeinsamem Basiscommit `15bc1d3ac3158791ab5260aa83d415a38fb7beb1`.
- Aufarbeitung aus `origin/main`: `084cdfc80d48f6f1659fc764955d7f941485e6bf`.
- Gemeinsamer Brain61-Zielstand: `2e9ade05015c71252e8800d61525cc8d69131c2e`; zwei uncommittete Tests in dessen Worktree bleiben unangetastet.
- Keine Produktionsmigration oder Aktivierung TokenDB-exklusiver Änderungen. Keine Secrets, ENV-Dateien oder Umgebungskonfiguration lesen.

## Abnahme

1. Scope und Commit-Historie gegen PR3, PR4, `origin/main` sowie Brain61 abgleichen.
2. Konflikte im kombinierten Schreib-/Lesepfad und in den Migrationen ursächlich integrieren.
3. Vorhandene angemessene Checks ausführen und den Merge-Gate-Review gegen exakt den kombinierten Stand starten.
4. Nach ALLOW gemeinsamer Merge/Push, Deploy/Restart, Live-Funktionsbeweis, SHA-Backup und Cleanup.
5. Abschlussstatus und konkrete Readiness in `BRAIN-3-STATUS.md` im Fortsetzungsordner festhalten.
