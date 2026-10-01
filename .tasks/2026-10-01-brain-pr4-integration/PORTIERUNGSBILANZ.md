status: aktiv
Datum: 2026-10-01

# PR4-Portierungsbilanz

Frisch ermittelte PR4-Dateiliste: 21 Pfade, 2.022 Einfügungen und 831 Löschungen gegenüber `main`. Der Quellstand ist `9efeb1e44ead5cdf5d01e05f242291fee79e803e`. Die aktuelle Integration basiert auf PR61 `b687f613b3df2c49138d9d2837e005c33e646d9f`.

| PR4-Pfad | Disposition in Brain61 |
|---|---|
| `.github/workflows/patch-understanding.yml` | Nicht übernommen. GitHub Actions sind laut Projektvorgabe kein Merge-Gate und kein Auftrag. Lokale gezielte Rust-Tests sind gelaufen. |
| `.tasks/2026-09-18-brain-evidence-u0/AUFTRAG.md` | Historischer Auftrag; durch den aktuellen Integrationsauftrag ersetzt. |
| `.tasks/2026-09-18-brain-evidence-u0/INTEGRATION.md` | Historischer Integrationsstand; nicht als aktueller Brain61-Stand übernommen. |
| `.tasks/2026-09-18-brain-evidence-u0/REGISTER.md` | Historisches Register; durch `2026-10-01-brain-pr4-integration/REGISTER.md` ersetzt. |
| `.tasks/2026-09-18-brain-evidence-u0/REPORT.md` | Historischer Bericht; relevante Bestandsbefunde sind in `BRAIN-4-STATUS.md` neu geprüft. |
| `.tasks/2026-09-18-brain-evidence-u0/REVIEW.md` | Alte Review-Runde; für den neuen Stand durch das aktuelle `REVIEW.md` ersetzt. |
| `docs/AUTONOMOUS_PATCH_REVIEW.md` | Inhaltlich aktualisiert und übernommen. Beschreibt aktuelle Migration, Patch-ID-Vertrag, Importtrust, Caption-Evidenz und bekannte Prüfgrenzen. |
| `rust/crates/deadlock-brain-yt/src/testutil.rs` | Selektiv übernommen. Gesetzter, aber unerreichbarer Test-DSN führt jetzt zu einem geheimnisfreien Fehler statt stillem Skip. |
| `rust/crates/deadlock-brain-yt/src/transcripts.rs` | Selektiv in die bestehende PR61-Implementierung portiert. JSON3-Rohdaten, Zeitsegmente, Wort-Offsets, SHA-Abgleich und transaktionale Speicherung ergänzt. PR4s umfangreiche Löschungen wurden nicht übernommen. |
| `rust/crates/deadlock-brain/src/bin/deadlock-brain-patch-review.rs` | Übernommen und am Brain61-Kontext validiert. Nichtkanonische Eingaben wie `patch_01` werden vor Lock und Speicherung abgewiesen. |
| `rust/crates/deadlock-brain/src/pg_insights.rs` | Nur Trust-Fixes übernommen. Confidence kann Trust nicht anheben, Datumswerte bleiben patchbezogen, Event-IDs werden aufgelöst, URLs nicht positionsbasiert gekoppelt und nur importierte Insights materialisiert. |
| `scripts/migrations/2026-09-18-patch-evidence-followup.sql` | Nicht als Folgemigration übernommen. Notwendige Zeit-, Quellenidentitäts- und Invalidierungslogik ist in die neue, eine PR61-Migration konsolidiert. |
| `scripts/migrations/2026-09-18-patch-evidence.sql` | Nicht unverändert übernommen. Schema und Trigger wurden anhand des aktuellen PR61-Bestands neu gefasst und in `2026-10-01-patch-evidence-review-v1.sql` konsolidiert. |
| `tests/patch-understanding/caption-scratch-schema.sql` | Nicht übernommen. Scratch-DDL bildet den alten isolierten Testaufbau ab; seine Kompatibilität mit dem aktuellen Brain61-Schema ist ungeprüft. |
| `tests/patch-understanding/cli-smoke-fixture.sql` | Nicht übernommen. Fixture setzt die alte Testmigration und ein reduziertes Snapshot-Schema voraus; vor Wiederverwendung neu an aktuelle DDL anpassen. |
| `tests/patch-understanding/followup-caption-assertions.sql` | Nicht übernommen. Assertions beschreiben die alte Migrationsfolge; Caption-Verhalten braucht eine neue Scratch-DB-Prüfung gegen die konsolidierte Migration. |
| `tests/patch-understanding/followup-history-assertions.sql` | Nicht übernommen. Assertions zur Erstbeobachtung müssen gegen die aktuelle Brain61-Eventstruktur neu ausgeführt werden. |
| `tests/patch-understanding/followup-identity-assertions.sql` | Nicht übernommen. Assertions zur kanonischen Identität gehören in eine neu angepasste Scratch-DB-Prüfung. |
| `tests/patch-understanding/heresy-comparison.json` | Nicht übernommen. Gehaltene Vergleichsdaten gehören nicht in den Kontext der Produktivanalyse und waren kein geprüfter Fähigkeitsnachweis. |
| `tests/patch-understanding/schema-assertions.sql` | Nicht übernommen. Die Datei koppelt mehrere alte Schemaannahmen und ist keine gültige Assertion-Suite für PR61 ohne Anpassung. |
| `tests/patch-understanding/schema-fixture.sql` | Nicht übernommen. Isolierte Altfixture; ersetzt keine Validierung der vollständigen Brain61-Migration. |

## Abschlussgrenze

Die CLI- und Insight-Trust-Fixes sind durch lokale Rust-Tests abgedeckt. Die Caption-Codeänderung ist wegen fehlender SQLx-Offline-Metadaten nicht vollständig kompiliert; Migration, Caption-Transaktion, Rollenrechte und Re-Run-Verhalten sind nicht gegen Scratch-Postgres geprüft. Darum ist diese Bilanz kein Beleg, dass PR4 bereits vollständig in PR61 oder `main` integriert wurde. PR4 bleibt offen, bis der getestete Integrationsstand und die vereinbarte Zielbranch-Übernahme belegt sind.
