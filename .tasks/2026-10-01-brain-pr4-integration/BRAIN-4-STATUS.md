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
2. `cargo test -p deadlock-brain --bin deadlock-brain-patch-review`: 9 bestanden, 0 fehlgeschlagen, 0 ignoriert.
3. `cargo fmt --check` und `git diff --check` erfolgreich.
4. `deadlock-brain-yt --all-targets` ist noch nicht kompilierbar mit `SQLX_OFFLINE=true`, weil für die bestehende `query!`-Abfrage in `transcripts.rs` kein SQLx-Cacheeintrag vorliegt. Der Build mit Datenbankzugriff wurde wegen „Keine ENV/Secrets/Umgebungskonfig lesen“ nicht versucht.
5. Die Migration und Rollenrechte sind lokal geändert, aber nicht gegen Scratch-Postgres ausgeführt. Gate, Push, PR4-Schließung, Merge, Migration, Deploy und Live-Prüfung sind daher offen.

## Nächster Meilenstein

Eine zulässige Scratch-Postgres-Prüfung ermöglicht YT-Compile, Evidenzintegrationstest und Migrationstest. Danach die vollständige Integrationsspur mit dem Merge-Gate prüfen. Kein Merge nach `main`, keine Produktionsmigration und kein Deploy, solange PR61 als Gesamtintegration offen ist.
