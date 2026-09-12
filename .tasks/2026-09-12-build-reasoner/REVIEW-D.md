# REVIEW-D (Runde 1)

Stand: 2026-09-12. Paket D (Integration, CLI, Backtest), Branch
`feat/build-reasoner-d`, Diff `cbfbc98..HEAD` (5f438d2, 7fbb128). Rein lesend.

## Urteil

NACHBESSERN. Genau ein blockierender Befund: Die Migration legt vier
`reasoner_*`-Tabellen an, die kein Code schreibt oder liest, obwohl
ARCHITEKTUR Abschnitt 4 sie ausdruecklich als Schreibziele fuehrt. Das ist ein
Spec-Verstoss. Alles andere ist sauber: Fassaden-Reihenfolge, CLI, Resolver,
Sync-Patch-Tag, Timer und Hygiene passen. Die drei Live-Befehle liefern gueltiges
JSON, der Schreibpfad ist korrekt eingezaeunt. Der Blocker ist billig zu fixen
und beruehrt den funktionierenden Live-Pfad nicht.

## Maengelliste

### 1. Vier `reasoner_*`-Tabellen ohne jeden Code-Zugriff (blockierend)

Datei: `scripts/migrations/2026-09-12-reasoner.sql:1-52`, fehlende Writes in
`rust/crates/dbrain-reasoner/src/lib.rs`.

Befund: `reasoner_builds`, `reasoner_item_scores`, `reasoner_patch_deltas` und
`reasoner_backtests` werden nur per `CREATE TABLE` angelegt. Eine repo-weite
Suche findet keinen INSERT, kein SELECT, keinen Bezug ausser der SQL-Datei. Die
Fassade druckt in `reason build|patch-impact|backtest` nur JSON und ruft bei
`--publish` `enqueue_publish_task`; sie persistiert nichts. ARCHITEKTUR
Abschnitt 4 listet diese Tabellen unter "Schreibt (neue Tabellen)" und begruendet
`reasoner_item_scores` als Nachweis des Fertig-Kriteriums "reproduzierbar ohne
KI-Aufruf": jede Zahl im Build soll als Zeile mit ihren Komponenten vorliegen.
Ohne Writes bleibt dieses Kriterium unbelegt und die Tabellen leer.

Vorschlag (guenstiges Modell): den spec-konformen Weg gehen und die Persistenz
verdrahten. `reason build` schreibt `reasoner_builds` plus `reasoner_item_scores`
(Score-Komponenten liegen in `ScoredItem` bereits vor), `reason patch-impact`
schreibt `reasoner_patch_deltas`, `reason backtest` schreibt
`reasoner_backtests`. Alternative, falls die Persistenz bewusst spaeter kommen
soll: die vier Tabellen aus der Migration nehmen und mit dem Persistenz-Auftrag
zusammen nachziehen, damit kein ungenutztes Schema in die zentrale DB geht. Die
Entscheidung zwischen beiden Wegen gehoert zum Orchestrator.

### 2. Patch-Tag zweimal unterschiedlich aufgeloest (wichtig)

Datei: `rust/crates/dbrain-builds/src/sync.rs:623-630` gegen
`rust/crates/dbrain-reasoner/src/lib.rs:215-220`.

Befund: Der Sync bildet den Tag mit `to_jsonb(pe)->>'patch_external_id'`, die
Fassade mit `pe.patch_external_id` direkt. Beide liefern im Live-Lauf denselben
Wert, die Herleitung ist aber dupliziert. Genau eine Tag-Divergenz zwischen
Schreiber und Leser hat den in REPORT-D beschriebenen `meta_support = 0`
verursacht. Weichen die zwei Ausdruecke je auseinander (Spaltentyp, Rename,
Filteranpassung), liest der Reasoner still ins Leere.

Bewertung: Der aktuelle `meta_support = 0` ist kein Code-Fehler. Sync und
Fassade loesen heute identisch auf; die bestehenden `hero_item_stats` tragen nur
noch den alten Literal-Tag `current`, weil sie vor dieser Aenderung geschrieben
wurden. Nach einem frischen Sync mit echtem Tag fuellt sich die Meta-Komponente.
Das ist die Deploy-Reihenfolge, kein Blocker.

Vorschlag: eine gemeinsame Funktion oder Konstante fuer die Tag-Aufloesung, von
Sync und Reasoner geteilt, damit die zwei Stellen nicht driften koennen.

### 3. Familien-Revision lexikografisch statt numerisch gewaehlt (nit)

Datei: `rust/crates/deadlock-brain-core/src/model_resolver.rs:66`.

Befund: `.max()` vergleicht die vollen Modell-IDs als Strings. Bei ungleicher
Ziffernlaenge des Suffix (etwa `-9` gegen `-1015`) waehlt das die falsche
Revision. Reale Revisionen sind datumsartig gleich lang, daher heute folgenlos.

Vorschlag: nach dem numerischen Suffix sortieren.

### 4. Fallback-Skill-Order verliert Punkt-Typ und Delta (nit)

Datei: `rust/crates/dbrain-reasoner/src/lib.rs:322-326`.

Befund: `load_hero_ability_orders` setzt pauschal `currency_type: 0, delta: 1`.
Der Pfad greift nur, wenn die Autoren-Quellen keine eigene Order tragen und der
Composer auf `brain.hero_ability_orders` zurueckfaellt. Ein so publiziertes Build
traegt eine Skill-Order ohne Punkt-Typ- und Delta-Nuance.

Vorschlag: `currency_type` und `delta` aus der Quelle ableiten oder die
Vereinfachung bewusst vermerken.

### 5. Migration weicht von der Spec-DDL bei `order_proximity` ab (nit)

Datei: `scripts/migrations/2026-09-12-reasoner.sql:47`.

Befund: Spec-DDL (Abschnitt 4) hat `order_proximity double precision NOT NULL`,
die Migration macht die Spalte nullable. Da `BacktestReport.order_proximity`
ein `Option` ist und "nicht messbar" tragen muss, ist nullable die richtigere
Wahl. Nur ein Konsistenzhinweis; bei Fix von Befund 1 mitziehen.

## Kalibrierung (kein Spec-Verstoss, kein Blocker)

- Kern-Ueberdeckung 0,158 und Reihenfolge-Naehe 0,435 gegen den
  Lightbringer-Seed sind schwach, aber eine Datenlage, keine falsche Regel.
  Ursache ist `meta_support = 0` fuer den geprueften Tag (siehe Befund 2): die
  `hero_item_stats` liegen noch unter dem alten Tag `current`. Nach frischem
  Sync mit echtem Patch-Tag fuellt sich die Meta-Komponente und die Ueberdeckung
  ist neu zu messen.
- "Core-Cutoff bei 19", "Zustandsfaktor 0,6 auf Imbue-Items" und "Kaufphase Lane
  schliesst vom Core aus" stammen aus Scoring und Composer (Paket B und C,
  `mechanics.rs` und `composer.rs`), nicht aus D. Der Zustandsfaktor ist eine
  MECHANIK-Regel (`condition_factor`, Abschnitt 4), die Kaufphasen sind
  MECHANIK Abschnitt 12. Das sind spec-konforme Regelentscheidungen, keine
  D-Maengel. Die Core-Groesse 19 ist eine Composer-Layout-Konstante ohne feste
  MECHANIK-Zahl, also eine Kalibrierungsfrage bei Paket C, nicht bei D.
- Patch-Wechsel nicht messbar: braucht mindestens zwei vergleichbare
  Patch-Staende je Autor. ARCHITEKTUR Abschnitt 11 erlaubt das ehrliche "nicht
  messbar". Kein Blocker.
- Der D-Backtest lief vor Aufnahme von Build 779996 in
  `tierlist.hero_build_sources` (Version 45, 5 Kategorien). Nach dem Sync den
  Warden-Backtest gegen diese Quelle wiederholen.

## Bestaetigt in Ordnung

- Fassaden-Reihenfolge (lib.rs:42-96): Laden, Patch-Delta vor dem Scoring,
  Scoring, Composer, KI nur bei `use_ai`, Kritiker mit genau einer
  Recompose-Runde. Deckt sich mit ARCHITEKTUR Abschnitt 9.
- Item-Analyst-Filter (lib.rs:358-387): fremde Item-Texte werden verworfen, die
  KI bringt kein neues Item in den Build.
- `compose_build_with_sources` verdrahtet (lib.rs:48-55, composer.rs:176-186):
  `build.ability_order` wird aus Autoren-Quellen bzw.
  `brain.hero_ability_orders` gefuellt, leer nur wenn beide fehlen. Der
  REVIEW-C-Fund aus Runde 2 ist damit erledigt.
- `--no-ai` erreicht keinen Modellaufruf (use_ai=false, ai=None, kein enrich,
  kein Kritiker). `--publish` ist der einzige externe Schreibpfad
  (`enqueue_publish_task`, nur bei `args.publish`).
- CLI (main.rs:226-271, 1544 folgende): `reason build|patch-impact|backtest` mit
  den Optionen aus Abschnitt 12, Seed-Pfad ueber `--seed-path` und
  `DEADLOCK_REASONER_SEED_PATH` konfigurierbar, JSON stabil, unbekannter Held
  wird ueber `HeroNotFound` zu einem sauberen Fehler statt Panic.
- Sync (sync.rs): echter `patch_tag` aus `brain.patch_events`, Upsert auf
  `(hero_id, item_id, bracket, patch_tag)`, keine Loeschung alter Zeilen.
- Resolver (model_resolver.rs) und 404-Retry (ai.rs:183-201): Rangfolge Env,
  Resolver, konfiguriertes bzw. Kompilat-Default; nur die Familie
  `accounts/fireworks/models/deepseek-v4-flash`; kein anderer Anbieter. Der
  Worker-Thread-Umbau laesst `chat`/`chat_value` unveraendert, bestehende Aufrufer
  (enrich, learn, player) brechen nicht. Welches Modell der Resolver aufloest,
  konnte ich ohne Live-Aufruf nicht namentlich pruefen; er greift nur bei 404,
  und der AI-Build lief laut REPORT-D ohne 404, also blieb es beim Default
  `deepseek-v4-flash`.
- Migration idempotent (`IF NOT EXISTS`), keine bestehende Tabelle veraendert.
- Timer und Skript: Muster der Brain-Timer, `pull build-data --hero all` existiert
  im CLI und loest "all" auf, Secrets nur ueber Infisical und `LoadCredential`,
  kein `EnvironmentFile`, Lauf 03:30 Europe/Berlin, `Persistent=true`.
- Hygiene: keine Secrets im Code, keine neuen externen Crates (Cargo.lock plus 2
  sind die internen Workspace-Crates), keine hartkodierten Modellnamen ausser der
  bestehenden Default-Konstante.
- Tests: `cargo test --workspace` gruen laut FERTIG-D. Die 5 Scratch-Tests
  duerfen als `#[ignore]` bleiben; sie ueberspringen sauber ohne
  `REASONER_SCRATCH_DSN` (siehe sync.rs-Test mit Early-Return). Keine Testpflicht.

## Deploy-Reihenfolge

1. Migration `scripts/migrations/2026-09-12-reasoner.sql` auf dem Central-Pool
   ausfuehren (nach Fix von Befund 1, damit die Tabellen auch befuellt werden).
2. Release-Binary bauen, systemd-Service mit dem echten Checkout-Pfad
   installieren.
3. Timer aktivieren.
4. Ersten Sync ueber `pull build-data --hero all` laufen lassen, damit
   `hero_item_stats` unter dem echten Patch-Tag liegen und `meta_support`
   greift.
5. Warden-Backtest wiederholen, jetzt gegen Build 779996, und die neuen
   Kennzahlen in REPORT-D nachtragen.

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

## Review Runde 2

Stand: 2026-09-12. Rein lesend gegen die Mängelliste 1 bis 5, Code-Commit
`e0e1cff`, Diff `7fbb128..HEAD`. Kein Branch, kein Code geändert.

### Urteil: FREIGABE

Alle fünf Mängel sind behoben. Keine neuen Befunde aus dem Fix. Persistenz,
Schema und Upserts passen zusammen; Transaktionen und Idempotenz sind im Code
und in den Regressionstests belegt. Tests grün, Clippy grün.

### Je Mangel

| Mangel | Behoben | Beleg |
| --- | --- | --- |
| 1 Persistenz | ja | `lib.rs:99/166/216` rufen `persist_build`/`persist_patch_impact`/`persist_backtest` in allen drei Fassaden auf, unabhängig von `--publish`. Jede Fassade läuft in einer Transaktion (`pool.begin()` … `tx.commit()`, `lib.rs:254/289/313`): Build und alle gescorten Items liegen gemeinsam in einer Transaktion, Schreibfehler erreichen über `?` den Aufrufer. Steam-Publish bleibt separat an `--publish` gebunden (Persistenz greift nicht in `enqueue_publish_task`). |
| 1 Schema/Upserts | ja | Spalten der vier Upserts (`lib.rs:236-242`) decken sich 1:1 mit der Migration: `reasoner_item_scores` schreibt alle neun Score-Komponenten plus `confidence`/`buy_phase`; ON-CONFLICT-Ziele treffen die PKs bzw. den Unique-Index `reasoner_backtests_hero_patch_author`. `DO UPDATE SET` ohne DELETE, daher überschreibt ein zweiter Lauf statt zu verdoppeln; `run_id`/`created_at` bleiben beim Upsert erhalten. |
| 2 Patch-Tag | ja | Genau eine Auflösung in `dbrain-builds/src/patch_tag.rs`, exportiert über `lib.rs:18`. Sync nutzt sie (`sync.rs:16/81`), die Fassade ebenfalls (`reasoner/lib.rs:224` via `effective_context`). Keine zweite Herleitung mehr; das alte inline `to_jsonb(pe)->>'patch_external_id'` im Sync ist weg. |
| 3 Revisionswahl | ja | `model_resolver.rs:70-75` sortiert nach `(nicht-leer, digits.len(), digits, id)` mit `digits = revision.trim_start_matches('-').trim_start_matches('0')`. Numerisch statt lexikografisch, ohne Integer-Überlauf. Test `fix_numeric_revisions_ignore_width_and_leading_zeroes` deckt `0731/1015`, `9/1015`, `0001015/731` ab und läuft grün. |
| 4 Fallback-Skill-Order | ja | `fallback_ability_order` (`lib.rs:445`): Objekte werden als `AbilityStep` mit Punkt-Typ und Delta übernommen; reine IDs bekommen je Vorkommen `[(2,-1),(1,-1),(1,-2),(1,-5)]`, also Freischalten Typ 2 und drei Upgrades Typ 1 mit Kosten 1/2/5. Mehr als vier Schritte, ungültige IDs und unvollständige Objekte liefern einen Datenfehler. Unit-Tests `fix_fallback_skill_order_*` grün. Der vom Briefing gewünschte Live-Abgleich gegen `tierlist.hero_build_sources` lief nicht (keine DSN im Review-Lauf); die Semantik deckt sich mit der dokumentierten Warden-Vorgabe. |
| 5 order_proximity nullable | ja | Migration `2026-09-12-reasoner.sql`: Spalte `double precision` (nullable) plus `ALTER … DROP NOT NULL` für bereits angelegte Tabellen. Spec-DDL in ARCHITEKTUR nachgezogen. Test `fix_migration_retains_all_scores_and_idempotent_backtests` prüft nullable, ON-CONFLICT-Keys und das Fehlen von NOT NULL/DELETE. |

### Migration, Upgrade-Pfade

Auf leerem Schema: `CREATE TABLE IF NOT EXISTS` mit vollem Spaltensatz, die
`ALTER`-Zeilen sind No-ops. Auf dem ersten Entwurf (ohne die drei Zusatzspalten,
`order_proximity` NOT NULL): `ADD COLUMN IF NOT EXISTS` ergänzt die drei Spalten
nullable, `DROP NOT NULL` löst die Einschränkung. Beide Wege laufen sauber. Rest:
historische Zeilen einer schon existierenden Tabelle tragen in den drei neuen
Spalten NULL bis zum nächsten Build-Lauf; die Upserts füllen sie beim ersten
Schreiben. Konsistenzhinweis, kein Blocker.

### Testnachweis

Toolchain: System-`cargo` ist 1.75.0 und scheitert an Lockfile v4; der Lauf
nutzt `~/.cargo/bin/cargo` (1.97.1).

- `cargo test --workspace` (ohne DSN): **256 passed, 0 failed, 53 ignored**.
  Die 53 ignorierten sind die Scratch-/Central-Schreibtests, die ohne
  `REASONER_SCRATCH_DSN` sauber übersprungen werden (kein 0.00s-Stiller-Skip in
  den gezählten Suites). Deckt sich mit dem Fixbericht.
- `cargo clippy --workspace --all-targets -- -D warnings`: **grün (exit 0)**.
- Der Lauf mit Scratch-DSN (Fixer: 83 passed) wurde im Review nicht wiederholt,
  da im Review-Lauf keine isolierte Scratch-DB bereitstand.

TESTNACHWEIS[TW-1]: 256 passed, 53 ignored | Baseline: 0 rot

### Deploy-Reihenfolge (endgültig)

1. Migration `scripts/migrations/2026-09-12-reasoner.sql` auf dem Central-Pool
   ausführen (legt Tabellen an, ergänzt Score-Spalten, macht `order_proximity`
   nullable) vor dem Einsatz der neuen Fassade.
2. Release-Binary im eigenen Worktree bauen, systemd-Service mit dem echten
   Checkout-Pfad installieren.
3. Timer aktivieren.
4. Ersten Sync über `pull build-data --hero all`, damit `hero_item_stats` unter
   dem echten Patch-Tag liegen und `meta_support` greift.
5. Warden-Backtest gegen Build 779996 wiederholen und die neuen Kennzahlen in
   REPORT-D nachtragen.
