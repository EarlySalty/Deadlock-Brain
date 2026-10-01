status: aktiv
Datum: 2026-10-01

# BRAIN-4 Status

## Stand

**Orchestrierung:** PR4 wird nicht direkt nach `main` gemergt. Der Quell-PR ist alt, während die gemeinsame Brain61-Integration offen ist. Der Arbeitsbranch wurde deshalb frisch von PR61-Head `b687f613b3df2c49138d9d2837e005c33e646d9f` abgezweigt. Fremde Worktrees blieben unverändert.

## Frische Befunde

1. PR #4 ist auf GitHub offen und steht bei SHA `9efeb1e44ead5cdf5d01e05f242291fee79e803e`.
2. PR #61 ist offen und hat Head `b687f613b3df2c49138d9d2837e005c33e646d9f`.
3. PR4s Diff gegen den aktuellen `main` enthält 831 Löschungen. Darunter liegen veraltete Änderungen an `pg_insights.rs` und `deadlock-brain-yt/src/transcripts.rs`; diese Gesamtgeschichte darf nicht übernommen werden.
4. Die aktuelle Brain61-Basis enthält bereits eine neue Rust-Architektur für Patchnotes und Review-Kontext. Sie enthält aber weder die PR4-Migrationen noch das eigenständige `deadlock-brain-patch-review`-Binärziel.
5. Die aktuelle Brain61-Version von `deadlock-brain-yt/src/testutil.rs` schluckt weiterhin einen fehlgeschlagenen Connect trotz gesetztem Test-DSN. Das ist ein zu portierender PR4-Fix.
6. PR4s Einzeländerungen zu Caption-Hash, kanonischer Quellenidentität und `item_or_ability` müssen am aktuellen Schema erneut verifiziert und bei Bedarf gezielt portiert werden.

## Validierung und offener Blocker

1. Gezielter Build des `deadlock-brain-patch-review`-Binärziels erfolgreich.
2. `cargo test -p deadlock-brain --bin deadlock-brain-patch-review`: 10 bestanden, 0 fehlgeschlagen, 0 ignoriert.
3. `cargo fmt --check` und `git diff --check` erfolgreich.
4. `deadlock-brain-yt --all-targets` ist mit `SQLX_OFFLINE=true` durch fehlenden Cacheeintrag für eine bestehende `query!`-Abfrage in `transcripts.rs` blockiert. Ein Build mit Datenbankzugriff wurde wegen der Vorgabe, keine ENV, Secrets oder Umgebungskonfiguration zu lesen, nicht versucht.
5. Gate-Runde 1 blockierte wegen akzeptierter Alias-ID `patch_01`. Der CLI-Fix verwirft nichtkanonische IDs und hat jetzt einen Regressionstest. Gate-Runde 2 und Scratch-Postgres-Prüfung stehen aus.
6. Branch-Commit `95b15bb` ist gepusht. PR4 bleibt offen, bis die Übernahme und das Gate belegt sind. PR61 ist weiterhin die offene Gesamtintegration. Merge, Migration, Deploy und Live-Prüfung fanden nicht statt.

## Nächster Meilenstein

Commit und Push des Alias-Fixes, dann Gate-Runde 2 gegen dieselbe PR61-Basis. Eine zulässige Scratch-Postgres-Prüfung bleibt erforderlich, bevor Migration, PR4-Schließung oder Live-Abschluss möglich sind. Kein Merge nach `main` und kein Deploy, solange PR61 als Gesamtintegration offen ist.
