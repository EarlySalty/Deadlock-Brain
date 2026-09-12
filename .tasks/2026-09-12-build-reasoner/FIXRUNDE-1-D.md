## Fixrunde 1

Stand: 2026-09-12. Fixer im Intent-Thread
`33a32f58-476b-4a67-99cc-8f6c1e8f7001`, ohne Unter-Threads oder Unter-Agenten.

Branch: `feat/build-reasoner-d`.
Code-Commit: `e0e1cffde98cd26713a79e80059af914e7da58a8`, auf diesem Branch gepusht.
Ausgangsstand: `7fbb128`. Kein Merge oder Push nach `main`.

### Änderungen je Mangel

Alle folgenden Zeilen beziehen sich auf den Code-Commit im Worktree
`/home/nathanael/.worktrees/deadlock-brain-d`.

| Mangel | Datei:Zeile | Änderung |
| --- | --- | --- |
| 1, Persistenz | `rust/crates/dbrain-reasoner/src/lib.rs:99`, `:166`, `:216`, `:249`, `:288`, `:312` | Alle drei Fassaden schreiben über ihren Central-Pool. Build und sämtliche gescorten Items liegen gemeinsam in einer Transaktion. Patch-Deltas und Backtest-Zeilen werden ebenfalls transaktional geschrieben. Schreibfehler erreichen den Aufrufer; der Build-Test erzwingt einen Fehler und belegt den Rollback. |
| 1, vollständiges Schema | `scripts/migrations/2026-09-12-reasoner.sql:18`, `:32`, `:62` | Alle neun Komponenten des `ItemScore` werden gespeichert, einschließlich `per_soul_value`, `active_value` und `passive_value`; dazu Konfidenz und Kaufphase. Zusätzlicher eindeutiger Backtest-Schlüssel auf Held, Patch und Autor ermöglicht echte Upserts. Bestehende Tabellen werden additiv ergänzt. |
| 2, gemeinsamer Patch-Tag | `rust/crates/dbrain-builds/src/patch_tag.rs:3`, `rust/crates/dbrain-builds/src/lib.rs:18`, `rust/crates/dbrain-builds/src/sync.rs:81`, `rust/crates/dbrain-reasoner/src/lib.rs:224` | Genau eine öffentliche Auflösung für Sync und Fassade. Externe ID, UTC-Datum oder gemeinsam `unknown`; explizite Tags bleiben erhalten. Der DB-Test vergleicht Fassade und gemeinsame Sync-Funktion bei ID, Datum und fehlendem Patch. DB-Fehler werden nicht als Fallback verborgen. |
| 3, Modellrevision | `rust/crates/deadlock-brain-core/src/model_resolver.rs:64` | Vergleich nach numerischem Wert des Suffixes: führende Nullen entfernen, Ziffernlänge und Ziffern vergleichen. Kein Integer-Überlauf bei langen Revisionen. Testfälle `0731/1015`, `9/1015`, `0001015/731`; bestehender Auswahltest ruft jetzt ebenfalls den produktiven Resolver auf. |
| 4, Fallback-Skill-Order | `rust/crates/dbrain-reasoner/src/lib.rs:441`, `:445` | Explizite AbilityStep-Objekte behalten Punkt-Typ und Delta. Bei den vorhandenen ID-Listen ergibt das erste Vorkommen Typ 2, Delta -1; die drei Upgrades ergeben Typ 1 mit -1, -2 und -5. Gegen echte Warden-Autorendaten lesend bestätigt. Ungültige IDs, unvollständige Objekte und zu viele implizite Upgrades liefern einen Datenfehler. |
| 5, nullable Reihenfolge | `scripts/migrations/2026-09-12-reasoner.sql:55`, `:65`, `.tasks/2026-09-12-build-reasoner/ARCHITEKTUR.md:133` | Migration und Spec-DDL erlauben NULL. Die Migration entfernt auch bei bereits vorhandenen Tabellen ein etwaiges NOT NULL. Nicht messbare Backtest-Werte bleiben NULL. |

Die Spec-Anpassung steht auch im Hauptordner. `types.rs` wurde nicht geändert.
Die einzige Änderung an `data.rs:1105` liegt im bestehenden Testmodul:
alte und neue Scratch-Tests teilen dieselbe Mutex, damit ihre Schema-Fixtures
bei einem vollständigen Testlauf nicht kollidieren. Die neue Fixture liegt in
`rust/crates/dbrain-reasoner/src/fixtures/fix_d.sql`; die acht
Reasoner-Regressionstests stehen in `src/fix_tests.rs`.

### Persistenzvertrag und Selbstprüfung

Upserts erhalten alte Patch-Stände ohne DELETE. `created_at` und
`run_id` bleiben bei Wiederholungen erhalten. Die Backtest-Tabelle enthält
je Held, Patch und Autor eine Zeile. Bei mehreren Vergleichen eines Autors
werden Kern-Überdeckung und messbare Reihenfolge-Nähe gemittelt;
`switch_detected` wird wahr, sobald ein messbarer Vergleich einen Wechsel
zeigt. Das vollständige `HeroBacktest` einschließlich aller Einzelvergleiche
bleibt in `detail`. Ohne Autorenvergleich wird der Aggregate-Report unter
leerem `author` gespeichert.

Die vier Tabellen werden jetzt auch ohne `--publish` beschrieben.
Die bisherigen gegenteiligen Aussagen in REPORT-D, FERTIG-D und
`docs/BUILD_REASONER.md:29` beschreiben den alten Stand und sind durch diese
Fixentscheidung überholt. Der externe Steam-Publish bleibt an `--publish`
gebunden. Die allgemeine Doku liegt außerhalb der Fix-Dateiliste.

Selbstprüfung: Bindings aller neun Score-Komponenten, Konfidenz, Kaufphase,
Fassaden-Aufrufe, Fehlerweitergabe, Transaktionsgrenzen, Patch-Historie,
Wiederholbarkeit, Datum-Fallback und Migration abgeglichen.
Migration zweimal auf frischer Fixture ausgeführt. Zusätzlich das Upgrade
einer Struktur ohne die drei Zusatzspalten und mit NOT NULL bei
`order_proximity` in der Wegwerf-DB geprüft: drei Spalten wieder vorhanden,
`order_proximity` nullable. Keine neuen Crates oder Code-Kommentare.

### Testzahlen

| Lauf | Baseline 7fbb128 | Endstand e0e1cff |
| --- | --- | --- |
| `cargo test --workspace`, ohne DSN | 251 bestanden, 0 fehlgeschlagen, 49 ignoriert | 256 bestanden, 0 fehlgeschlagen, 53 ignoriert |
| Reasoner ohne DSN | 68 bestanden, 7 ignoriert | 72 bestanden, 11 ignoriert |
| Core ohne DSN | 14 bestanden | 15 bestanden |
| Builds / Binary ohne DSN | 7 / 38 bestanden | 7 / 38 bestanden |
| `cargo test -p dbrain-reasoner -- --include-ignored`, mit DSN | Nur Central: 70 bestanden, 5 wegen fehlender Scratch-DSN fehlgeschlagen | Central plus isolierte Scratch-DSN: 83 bestanden, 0 fehlgeschlagen, 0 ignoriert |
| Clippy für Reasoner, Builds, Core und Binary, alle Targets, `-D warnings` | laut Vorbericht grün | grün |
| Formatter | laut Vorbericht grün | `cargo fmt` gezielt auf Crate-Wurzeln, `rustfmt` gezielt auf eigene Moduldateien, beide Checks grün |

Rot-Gegenproben vor Implementierung: 0 bestanden, 8 fehlgeschlagene neue
Reasoner-Tests; separat 0 bestanden, 1 fehlgeschlagener neuer Resolver-Test.
Die Fehler belegten fehlende Tabellenzeilen, fehlenden Fehler/Rollback-Pfad,
`current` statt `unknown`, falsche Skill-Punkte, fehlende Score-Spalten und
die Auswahl von Revision 9 statt 1015.
Zusätzliche Rückmutation der Spec-DDL auf NOT NULL:
0 bestanden, 1 fehlgeschlagener Konsistenztest; danach wieder nullable und grün.

Central-Secrets wurden über den vorhandenen Infisical-Weg nur in den
Prozess geladen. Die beiden Central-Tests erzwingen
`default_transaction_read_only=on`. Sämtliche Test-Schreibzugriffe liefen
auf einer eigens gestarteten lokalen PostgreSQL-Instanz über Unix-Socket;
die Scratch-Tests prüfen vor DDL den Datenbanknamen `reasoner_a_fix`.
Es gab keinen Test-Schreibzugriff auf Central und keinen KI-Aufruf.
Die Testmatrix enthält keine neuen Live-Fassadenaufrufe gegen Central, weil
diese seit dem Fix absichtlich schreiben.

### Übergabe

Die fünf beauftragten Mängel sind behoben; unabhängige Freigabe bleibt beim
Orchestrator. Die aktualisierte Migration muss vor dem Einsatz der neuen
Fassade im Central-Pool angewendet werden. Deployment, Timer-Aktivierung,
Service-Neustart und frischer Sync bleiben beim Orchestrator.
Kalibrierung, Core-Cutoff, Zustandsfaktor und Lane-Phase wurden nicht geändert.
