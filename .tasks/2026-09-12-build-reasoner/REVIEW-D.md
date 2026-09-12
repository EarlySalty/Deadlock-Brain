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
