status: aktiv
Datum: 2026-10-01

# Gate-Runde 1: BLOCK

Gate-Aufruf: `gate_hook.py --review`, Repository `luna-abschluss-brain-pr3-20261001`, Basis `2e9ade05015c71252e8800d61525cc8d69131c2e`, geprüfter Commit `a438dbe`.

Der breitere Lauf gegen `origin/main` erhielt kein Modellurteil, weil der Prompt 6.021.421 Zeichen umfasste. Der Scope-Lauf gegen den gemeinsamen Brain61-Freeze erhielt ein BLOCK von `gpt-6.1-sol`.

## Blockierende Befunde

1. **Nicht portable Cargo-Pfade.** `rust/crates/deadlock-brain-core/Cargo.toml:26`, `rust/crates/dbrain-session-store/Cargo.toml:13-14`, `rust/crates/deadlock-brain-yt/Cargo.toml:30` und `rust/crates/dbrain-session-store/src/database_tests.rs:33` verweisen auf sibling Worktrees oder externe Migrationen. Ein Checkout dieses Repos allein kann die Abhängigkeiten nicht auflösen.
2. **Migration fehlt im Caption-Integrationspfad.** `.github/workflows/patch-understanding.yml:131` und `docs/AUTONOMOUS_PATCH_REVIEW.md:34` lassen `2026-09-18-patch-insights-evidence.sql` aus. Der Caption-Writer schreibt `timing_status` und `parser_version`, die diese Migration hinzufügt.
3. **Caption-Evidenz kann veraltet erscheinen.** `rust/crates/deadlock-brain/src/bin/deadlock-brain-mcp.rs:313` vertraut `t.content_hash`, statt den aktuellen Transcripttext zu hashen. Textänderungen bei unverändertem Hash können alte Segmente zurückgeben.
4. **UPDATE-Trigger invalidiert nur die neue Identität.** `scripts/migrations/2026-09-18-patch-evidence-followup.sql:178` berücksichtigt laut Gate nicht die alte Patch- oder Event-Identität. Ein Wechsel von Patch A zu B kann A's gespeicherten Entwurf gültig erscheinen lassen.
5. **Patch-IDs werden nicht kanonisiert.** `rust/crates/deadlock-brain/src/bin/deadlock-brain-patch-review.rs:99,102,201,212` verwenden rohe Schreibweisen für Lock, Speicher, Kontext und Revisionsabfrage. `patch_01` und `patch_1` können auseinanderlaufen.
6. **Analyse lässt `item_or_ability` aus.** `rust/crates/deadlock-brain/src/bin/patch-insights.rs:170` filtert laut Gate diesen Typ aus dem Loader.

## Nicht blockierende Befunde

7. **SQL-Wildcard-Escaping unvollständig.** `rust/crates/deadlock-brain/src/bin/deadlock-brain-mcp.rs:145` fehlt `ESCAPE '!'`; der Statfilter bei Zeile 150 ist ein benachbarter Eintrittspfad.
8. **Caption-Dokumentation übertreibt die Sprachauswahl.** `rust/crates/deadlock-brain-yt/src/transcripts.rs:218` fordert `en.*` an und akzeptiert `en`/`en-orig`; die Dokumentation behauptet zusätzlich deutsche Untertitel und Metadatenfilterung.

Gate-Hinweis: Zwillingssuche für `).context(` wurde wegen 27 Fundstellen übersprungen.

## Status

Keine Gate-Fixes vorgenommen. Die explizite Auftragsgrenze untersagt Unterthreads. Der vorgeschriebene frische Fixer-Thread kann daher in dieser Session nicht angelegt werden. Kein Push, Merge nach `main`, Deploy oder Live-Nachweis.
