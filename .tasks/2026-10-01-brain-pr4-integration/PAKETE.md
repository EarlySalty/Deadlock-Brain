status: aktiv
Datum: 2026-10-01

# Pakete: selektive PR4-Integration

| Paket | Umfang | Stand | Beleg |
|---|---|---|---|
| P1: Patchanalyse und Importtrust | History-/Review-CLI, Quellenvalidierung, nicht veröffentlichter Entwurf, sichere Agenten-Insight-Importe | Gate-Runde 4 ALLOW; offene NITs betreffen Scratch-Postgres und YT-Offline-Compile | `rust/crates/deadlock-brain/src/bin/deadlock-brain-patch-review.rs`, `pg_insights.rs` |
| P2: Evidenzschema | Revisionen, Beobachtungszeit, Segment-Evidenz, Trigger, Views, Advisory Locks, Rollenrechte | Gate-Runde-3-Korrekturen committed und in Runde 4 erlaubt; Scratch-Postgres-Anwendung und Re-Run-Prüfung offen | `scripts/migrations/2026-10-01-patch-evidence-review-v1.sql`, `ops/brain-postgres/grants.sql` |
| P3: Caption-Persistenz | JSON3-Zeitsegmente, Roh-SHA, Text-SHA, transaktionales Speichern, harter Test-DSN-Fehler | gebaut; YT-Compile durch fehlenden SQLx-Offline-Cache blockiert, DB-Integration offen | `rust/crates/deadlock-brain-yt/src/transcripts.rs`, `testutil.rs` |
| P4: Dokumentation und PR4-Bilanz | aktuelle Bedienungsnotiz, Altdatei-Disposition, Scope-Nachweis | Doku revidiert; Testfixtures bleiben wegen nicht geprüfter Schema-Kompatibilität offen | `docs/AUTONOMOUS_PATCH_REVIEW.md`, `PORTIERUNGSBILANZ.md` |
| P5: Gate und Abschluss | Gate-Folgerunde, PR4-Statusnachweis, Integration in Brain61 | Gate-Runde 4 ALLOW; Aufnahme in PR61 und Scratch-Postgres-Prüfung offen; keine isolierte Integration nach `main`, kein Deploy solange PR61 offen | `BRAIN-4-STATUS.md` |


Keine Unterthreads oder Unteragenten. Fremde Worktrees unverändert.
