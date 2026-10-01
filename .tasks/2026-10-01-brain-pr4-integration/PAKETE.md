status: aktiv
Datum: 2026-10-01

# Pakete: selektive PR4-Integration

| Paket | Umfang | Stand | Beleg |
|---|---|---|---|
| P1: Patchanalyse | eigenständige History- und Review-CLI, Quellenreferenzvalidierung, nicht veröffentlichter Entwurf | gebaut; Gate-Runde 1 fand Alias-Schlüssel-Bypass, Fix umgesetzt, Folgerunde offen | `rust/crates/deadlock-brain/src/bin/deadlock-brain-patch-review.rs` |
| P2: Evidenzschema | Revisionen, Beobachtungszeit, Segment-Evidenz, Trigger, Views, Advisory Locks, Rollenrechte | gebaut; Scratch-Postgres-Anwendung und Re-Run-Prüfung offen | `scripts/migrations/2026-10-01-patch-evidence-review-v1.sql`, `ops/brain-postgres/grants.sql` |
| P3: Caption-Persistenz | JSON3-Zeitsegmente, Roh-SHA, Text-SHA, transaktionales Speichern, harter Test-DSN-Fehler | gebaut; YT-Compile durch fehlenden SQLx-Offline-Cache blockiert, DB-Integration offen | `rust/crates/deadlock-brain-yt/src/transcripts.rs`, `testutil.rs` |
| P4: Abschluss | Gate-Runde 2, PR4-Statusnachweis, Integration in Brain61 | offen; keine isolierte Integration nach `main`, kein Deploy solange PR61 offen | `BRAIN-4-STATUS.md` |

Keine Unterthreads oder Unteragenten. Fremde Worktrees unverändert.
