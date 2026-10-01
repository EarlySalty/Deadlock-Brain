status: aktiv
Datum: 2026-10-01

# BRAIN-4 Status

## Stand

**Orchestrierung:** PR4 wird nicht direkt nach `main` gemergt. Der Quell-PR ist alt, während die gemeinsame Brain61-Integration offen ist. Der Arbeitsbranch wurde deshalb frisch von PR61-Head `b687f613b3df2c49138d9d2837e005c33e646d9f` abgezweigt. Fremde Worktrees blieben unverändert.

## Frische Befunde

1. PR #4 ist auf GitHub offen und steht bei SHA `9efeb1e44ead5cdf5d01e05f242291fee79e803e`.
2. PR #61 ist offen und hat Head `b687f613b3df2c49138d9d2837e005c33e646d9f`.
3. PR4s Gesamtänderung an `pg_insights.rs` und die umfangreichen Löschungen in `transcripts.rs` wurden nicht übernommen. Die aktuell benötigten Trust-Fixes des Importers und die Caption-Evidenz wurden gezielt auf die PR61-Basis portiert.
4. Die aktuelle Brain61-Basis enthält bereits eine neue Rust-Architektur für Patchnotes und Review-Kontext. Sie enthält aber weder die PR4-Migrationen noch das eigenständige `deadlock-brain-patch-review`-Binärziel.
5. Die aktuelle Brain61-Version von `deadlock-brain-yt/src/testutil.rs` schluckt weiterhin einen fehlgeschlagenen Connect trotz gesetztem Test-DSN. Das ist ein zu portierender PR4-Fix.
6. PR4s Einzeländerungen zu Caption-Hash, kanonischer Quellenidentität und `item_or_ability` müssen am aktuellen Schema erneut verifiziert und bei Bedarf gezielt portiert werden.

## Validierung und offener Blocker

1. `cargo check -p deadlock-brain --bin deadlock-brain` erfolgreich.
2. `cargo test -p deadlock-brain --bin deadlock-brain pg_insights::tests`: 6 bestanden, 0 fehlgeschlagen, 0 ignoriert, 53 gefiltert.
3. `cargo test -p deadlock-brain --bin deadlock-brain-patch-review`: 10 bestanden, 0 fehlgeschlagen, 0 ignoriert.
4. `cargo fmt --check` erfolgreich. `git diff --check` folgt nach dem aktuellen Doku- und Trust-Fix-Diff.
5. `deadlock-brain-yt --all-targets` ist mit `SQLX_OFFLINE=true` durch fehlenden Cacheeintrag für eine bestehende `query!`-Abfrage in `transcripts.rs` blockiert. Ein Build mit Datenbankzugriff wurde wegen der Vorgabe, keine ENV, Secrets oder Umgebungskonfiguration zu lesen, nicht versucht.
6. Gate-Runde 1 blockierte wegen akzeptierter Alias-ID `patch_01`; der Fix verwirft nichtkanonische IDs und der Regressionstest besteht. Gate-Runde 2 erlaubte Commit `b12a5a8`, lief aber vor der selektiven Portierung der Insight-Trust-Fixes und der aktualisierten Doku. Gate-Runde 3 steht aus.
7. Commits `95b15bb` und `b12a5a8` sind gepusht. Die aktuellen Insight-Trust-Fixes und Dokumentationsänderungen sind noch nicht committed. PR4 bleibt offen, PR61 ist weiterhin die offene Gesamtintegration. Merge, Migration, Deploy und Live-Prüfung fanden nicht statt.

## Nächster Meilenstein

Die aktuellen Trust-Fixes und PR4-Portierungsbilanz committen und pushen, dann Gate-Runde 3 gegen dieselbe PR61-Basis ausführen. Scratch-Postgres-Prüfung bleibt vor Datenbankabschluss erforderlich. Kein isolierter Merge nach `main` und kein Deploy, solange PR61 als Gesamtintegration offen ist.
