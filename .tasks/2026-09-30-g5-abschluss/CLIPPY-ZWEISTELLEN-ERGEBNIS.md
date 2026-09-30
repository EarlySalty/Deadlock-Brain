status: aktiv, Zweistellen-Quellfix ohne erneuten Compilerlauf
Datum: 2026-09-30

# Zwei Clippy-Auto-Deref-Befunde

Ausgang `b3523fa246c39249f42cd58343f1785445da0323`, gepushter Quellhead `d35a11ccfa790d2835e45ed8409910154ac13375` auf `fix/g5-replay-deferred-20260930`. Der zugeteilte Clippy-Lauf auf dem Ausgangsstand endete laut `CLIPPY-ZWEISTELLEN-FIX.md` mit Exit 101 und genau zwei `clippy::explicit_auto_deref`-Befunden. Für diesen Fix wurde Clippy nicht erneut gestartet.

`rust/crates/brain-storage/src/pg_jobs.rs:36` übergibt die vorhandene `&mut Transaction<Postgres>` jetzt direkt als `tx` an `lock_source`. `rust/crates/brain-storage/src/pg_release.rs:132` übergibt die eigene Transaktion als `&mut tx`. Die automatische Dereferenzierung liefert jeweils weiterhin `&mut PgConnection`. Quell-ID, Advisory-Key, Sperrreihenfolge und Transaktionsgrenze sind unverändert. Keine weiteren Produkt- oder Testquellen, Attribute, Kommentare oder Konfigurationen wurden geändert.

Diffumfang: zwei Dateien, zwei Einfügungen und vier Löschungen einschließlich der durch `rustfmt` zusammengefassten Aufrufzeile. Rust 1.97.1 `rustfmt --check` für beide Dateien, `git diff --check` und `git diff --cached --check` lieferten Exit 0. Kein Cargo, Compiler, Test, PostgreSQL, Import, Dienst oder Deploy wurde für diese Nacharbeit gestartet. Ein erneuter Clippy-Lauf benötigt eine gesonderte Slotzuteilung; ein grünes Ergebnis wird nicht behauptet.

BESTAND[BS-1]: ja | Fundort: rust/crates/brain-storage/src/pg_jobs.rs:7 | Anknüpfung: vorhandene Quellsperre und Transaktion
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: CLIPPY-ZWEISTELLEN-ERGEBNIS.md
ORCHESTRIERUNG[OR-1]: Stufe klein | Schritt bau | Artefakt: .tasks/2026-09-30-g5-abschluss/CLIPPY-ZWEISTELLEN-ERGEBNIS.md
