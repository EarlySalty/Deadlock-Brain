status: aktiv
Datum: 2026-10-01

# PAKETE

## Paket A: PR61-Owner-Regressionstests

Quelle: `c4a41d7b079f44c9a9a7fc30b285b0032937cfa9` auf dem sauberen PR61-Quellbranch.

Kandidatpfade:

- `rust/crates/brain-feeds/src/build_publish.rs`: fehlende Tests für Endpoint-Parsing, Userinfo, Query/Fragment und Basispfadtransport. Produktionscode ist bereits auf der Integrationsbasis; nur fehlende Testabdeckung übernehmen.
- `rust/crates/dbrain-retrieval/tests/core_retrieval.rs`: fehlender ACL-Regressionsfall für generische Lexical-/Dense-Retrievalpfade. Nur die Testfunktion übernehmen.

Nicht übernehmen: sieben `.tasks`-/Branchstatusartefakte aus dem Quellencommit, Steam-Ledger-Änderungen und PR61-Funktionalität, die bereits auf Main liegt. Patch-IDs und direkte Treevergleiche vor Transfer dokumentieren.

## Paket B: PR9-Import-, Schema- und History-Handoff

Vorliegende Quelle: Owner-Handoff `7deebcb779b6fa5572ebe1c1e554a3dbd9c2e569`, Parent `e752d2514249ece9b3702c5fd93a75680495db4`. Der Commit patcht Import-Race, Patch-ID-Normalisierung, `posted_at`-Drift und Snapshot-Limit, aber nicht die Schema-Migrationen, auf denen diese Pfade aufbauen.

Auf `origin/main` fehlen derzeit:

- `scripts/migrations/2026-09-18-patch-evidence.sql`
- `scripts/migrations/2026-09-18-patch-evidence-followup.sql`
- `scripts/migrations/2026-09-18-patch-evidence-followup2.sql`

Diese Dateien stammen in der PR9-Historie aus `da3b55ea6619cb8007494adc40150d82375b96d7`, `370499733c74548ffe327b0a3e8a362e629cad00`, `a168371c6458cb24d6f29ea6ab8758e242fba3d2` und `dc941527fa993979535321237cbc102c82d44221`. Sie werden nicht aus der alten PR9-Vollhistorie übernommen. Der PR9-Owner muss die erforderlichen Schema-Dateien samt Import-/History-Abhängigkeiten in einem begrenzten committed Handoff gegen `be2aa6b` liefern. Die betroffenen PR9-Produktpfade bleiben bis dahin reserviert und unverändert.

Kandidatenpfade des bestehenden Handoffs, erst nach Vorliegen des Schema-Handoffs:

- `rust/crates/dbrain-normalize/src/lib.rs`
- `rust/crates/dbrain-normalize/src/patch.rs`
- `rust/crates/deadlock-brain/src/bin/deadlock-brain-patch-review.rs`
- `rust/crates/deadlock-brain/src/pg_patchnotes.rs`
- `docs/AUTONOMOUS_PATCH_REVIEW.md`

Vor Übernahme jeden Pfad gegen Basis prüfen. Nur tatsächlich fehlende Patch-ID-Kanonisierung, Import-Race-/Driftbehandlung und wirksame Snapshot-Grenze übernehmen. Kommentare aus Code nicht neu einführen. Handoff-Statusdokument bleibt als Quellbeleg erhalten und wird nicht automatisch nach Root kopiert.

## Pfadgrenzen

Pakete A und B ändern unterschiedliche Produktdateien. Kein Paket übernimmt Quellhistorie als Ganzes. Konflikte oder Abhängigkeiten, die die direkte Anwendung auf die Basis nicht sicher belegen, gehen vor jeder Änderung als konkrete Rückfrage an den jeweiligen Owner.
