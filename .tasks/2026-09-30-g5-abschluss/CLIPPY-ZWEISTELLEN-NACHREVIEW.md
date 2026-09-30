# Statisches Nachreview der zwei Auto-Deref-Korrekturen

Datum: 2026-09-30. fertig: J. Fix: N.

**GO für `d35a11ccfa790d2835e45ed8409910154ac13375`.** Beide Änderungen entsprechen den tatsächlichen Clippy-Vorschlägen und erhalten Verbindung, Transaktion, Sperrwirkung und Fehlerweitergabe. Keine konkreten Restbefunde im beauftragten Zweistellendelta. Das vorherige Synchronisations-GO `dcfee1d` bleibt bestehen; ein grüner Compilerlauf wird nicht behauptet.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft

## Bindung und Prüfung

Quelle: `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`, eingefrorenes Delta `b3523fa246c39249f42cd58343f1785445da0323..d35a11ccfa790d2835e45ed8409910154ac13375`. Zwei Dateien, zwei Einfügungen und vier Löschungen. Der vollständige Diff enthält die beiden Aufrufkorrekturen samt Zusammenziehen einer bestehenden Await-/Fehlerkette; keine Nebenänderung an Sperrhelfer, Transaktionsgrenzen, Manifest, Tests oder Lintkonfiguration.

Graphify-Abfrage zu `lock_source`, `commit_batch_tx` und `pg_release` lieferte fachfremde Treffer. Danach die vorgegebenen Stellen aus dem eingefrorenen Gitstand samt unmittelbar relevanten Typen gelesen. Beide Fundstellen gegengeprüft, keine erneute Gesamtquellrunde. `git diff --check b3523fa246c39249f42cd58343f1785445da0323 d35a11ccfa790d2835e45ed8409910154ac13375`: Exit 0.

`SLOT-D-CLIPPY-NACHWEIS.md` und der darin referenzierte bestehende Compilerlog `/home/nathanael/Documents/.tasks/2026-09-30-twitch-alles-live/brain-g5-clippy-b3523fa-slot-20260930.log` wurden gelesen. Logzeilen 47 bis 65 zeigen beide `explicit_auto_deref`-Fehler und die Vorschläge `tx` beziehungsweise `&mut tx`. Der zugehörige Lauf auf `b3523fa` endete laut Laufnachweis mit 101. Kein neuer Compilerlauf durch den Reviewer.

## Semantisches Urteil je Stelle

1. **`rust/crates/brain-storage/src/pg_jobs.rs:36`, GO.** `commit_batch_tx` erhält `tx: &mut Transaction<'_, Postgres>` (`:24-28`). `lock_source` verlangt unverändert `&mut PgConnection` (`:7-10`). Die bisherige explizite Dereferenzierung `&mut **tx` wird durch die Deref-Coercion am typisierten Funktionsargument ersetzt. `lock_source(tx, &lease.source_id)` leiht dieselbe innere Connection mutabel aus, statt eine Verbindung zu öffnen oder die Transaktion zu verschieben. Aufrufposition vor den folgenden Batchoperationen und `.await.map_err(database_error)?` bleiben erhalten. Keine Änderung an Leaseprüfung, Sperrschlüssel oder Lebensdauer der Transaktion.
2. **`rust/crates/brain-storage/src/pg_release.rs:132`, GO.** `tx` ist die unverändert bei `:130` begonnene lokale Transaktion. `&mut tx` wird am selben konkreten Parameter zu `&mut PgConnection` dereferenziert; dies erreicht dieselbe Connection wie zuvor `&mut *tx`. Die vorhandene Quelleniteration und anschließenden `commit_batch_tx(&mut tx, ...)`-Aufrufe bleiben unverändert. Keine neue Begin-/Commit-/Rollbackgrenze, kein Freigeben der Sperre zwischen Quellenprüfung und Veröffentlichung. Die einzeilige Await-/Map-Error-Kette propagiert denselben Fehler über `?`.
3. **Gemeinsamer PostgreSQL-Vertrag, GO.** Der unveränderte Helfer `pg_jobs.rs:7-15` führt weiter `pg_advisory_xact_lock(hashtext($1)::bigint)` mit `core-source:<source>` auf der übergebenen Connection aus. Die Deref-Coercion ändert weder Schlüssel noch transaktionsgebundene Haltedauer. Beide konkreten Lintvorschläge sind umgesetzt, ohne `allow`, Abschwächen von `-D warnings` oder Umgehen des Helfers. Die Wirkung wurde statisch bewertet, nicht durch eine DB-Verbindung gemessen.

## Grenze und nächster Antrag

Keine Compiler, Tests, DB-Verbindungen, Fixtures, Dienste, Imports oder sonstige Runtime gestartet. Der Slot ist laut Auftrag zurückgegeben. Für den korrigierten Head fehlt weiterhin der erneute Compilerbeleg; der abgebrochene Vorgängerlauf belegt keinen erfolgreichen All-Targets-Abschluss beider Pakete.

Bei neuer konkreter Zuteilung denselben einen Paket-Clippy auf dem gebundenen Fixstand erneut beantragen, ohne vorausgehenden Check oder automatische Folgekette:

```bash
/home/nathanael/.cargo/bin/cargo +1.97.1 clippy --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-storage -p brain-legacy-import --all-targets --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- -D warnings
```

Berichtssenke: bestehender Reviewbranch `review/pre-g5-core-abnahme-20260929` im Worktree `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`. Gemäß ausdrücklichem Auftrag bleiben Branch und Worktree nach Bericht-Commit und Push bestehen. Keine Main-Integration oder Branchlöschung, auch nicht aufgrund des Stop-Hooks.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: CLIPPY-ZWEISTELLEN-NACHREVIEW.md
