status: aktiv
Datum: 2026-10-01

# Auftrag: PR4 in die aktuelle Brain61-Integration überführen

Quelle ist PR #4, Branch `codex/patch-understanding-evidence-20260918`, SHA `9efeb1e44ead5cdf5d01e05f242291fee79e803e`. Arbeitsgrundlage ist der am 2026-10-01 frisch gelesene PR4-Stand, `origin/main` und PR #61 mit Head `b687f613b3df2c49138d9d2837e005c33e646d9f`.

Die Änderungen von PR4 werden nicht als veraltete Gesamtgeschichte übernommen. Nur aktuell benötigte Evidenz- und Trust-Korrekturen werden in einem eigenen Branch auf dem aktuellen PR61-Head umgesetzt. Insbesondere dürfen die veralteten Änderungen an `pg_insights.rs` und die großflächigen Löschungen aus `transcripts.rs` nicht übernommen werden. Änderungen fremder Worktrees bleiben unangetastet.

Zielumfang:

1. Bestehende PR4-Funktion und Fehlerkorrekturen mit dem aktuellen Brain-Rust-Aufbau abgleichen.
2. Erforderliche aktuelle Änderungen einschließlich Caption-Textprüfung, Test-DSN-Fehlerpfad, Patchidentität und Kontextgrenzen portieren oder durch aktuellen Bestand nachweislich ersetzen.
3. Passende Migration und Tests auf dem aktuellen Schema ergänzen. Keine Produktionsdaten oder ENV-Dateien lesen oder ändern.
4. Testen, Bericht und Review-Gate für den eigenen kleinen Diff durchführen. Geprüfte Commits und Belege für die Brain61-Integration festhalten.

Freigabegrenze: PR #4 darf erst geschlossen werden, wenn sein aktuell benötigter Inhalt nachweislich in der Brain61-Integrationsspur enthalten ist. Kein isolierter Merge nach `main`, kein Deploy außerhalb der Gesamtintegration. Deploy und Live-Prüfung erst nach Merge der Brain61-Gesamtgruppe.
