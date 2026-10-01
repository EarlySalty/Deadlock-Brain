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

Quelle: committed Owner-Handoff `7deebcb779b6fa5572ebe1c1e554a3dbd9c2e569`, Parent `e752d2514249ece9b3702c5fd93a75680495db4c`.

Kandidatenpfade:

- `rust/crates/dbrain-normalize/src/lib.rs`
- `rust/crates/dbrain-normalize/src/patch.rs`
- `rust/crates/deadlock-brain/src/bin/deadlock-brain-patch-review.rs`
- `rust/crates/deadlock-brain/src/pg_patchnotes.rs`
- `docs/AUTONOMOUS_PATCH_REVIEW.md`

Vor Übernahme jeden Pfad gegen Basis prüfen. Nur tatsächlich fehlende Patch-ID-Kanonisierung, Import-Race-/Driftbehandlung und wirksame Snapshot-Grenze übernehmen. Kommentare aus Code nicht neu einführen. Handoff-Statusdokument bleibt als Quellbeleg erhalten und wird nicht automatisch nach Root kopiert.

## Pfadgrenzen

Pakete A und B ändern unterschiedliche Produktdateien. Kein Paket übernimmt Quellhistorie als Ganzes. Konflikte oder Abhängigkeiten, die die direkte Anwendung auf die Basis nicht sicher belegen, gehen vor jeder Änderung als konkrete Rückfrage an den jeweiligen Owner.
