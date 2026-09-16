# Review Paket M, Runde 2 (Rolle review_2_4)

**Urteil: FREIGABE**

Gegenstand: Worktree `~/.worktrees/deadlock-brain-m`, Branch
`feat/build-reasoner-population`, Spitze `be9a982`. Geprueft gegen die
Maengelliste aus REVIEW-M.md (4 wichtig, 3 Nits) und REVIEW-P.md (5 Nits) sowie
die Fix-Commits `23e0935`, `73e26b9`, `09198ff`, `a2ed800`, `4385392`,
`be9a982` (Diff `3ff5db4..be9a982`). Kein neues Voll-Audit. Ich war nicht der
Autor.

Bau selbst gefahren (PATH auf `~/.cargo/bin`, `rust/`, Debug, kein `--release`,
`set -o pipefail`):
- `cargo test -p dbrain-reasoner --lib`: 170 bestanden, 16 ignoriert, 0 rot.
- `cargo test -p dbrain-population`: 24 bestanden, 0 rot.
- `cargo clippy -p dbrain-reasoner -p dbrain-population -p deadlock-brain
  --all-targets -- -D warnings`: sauber, Exit 0.
- `cargo test --workspace` (ohne DSN): 386 bestanden, 58 ignoriert, 0 rot.
Alle vier Zahlen decken sich mit der Fertigmeldung.

## Status je Mangel aus Runde 1

**M1, POPULATION_DB_DSN zweiter Env-Name (wichtig): behoben.** Commit `09198ff`.
`load_reasoning_inputs` und der Backtest-Pfad laden ueber
`load_population_prior(&ctx.pool, hero_id)` aus demselben Pool
(`lib.rs:494` neu, aufgerufen `lib.rs:343`, `lib.rs:565`). `build_evaluation.rs`
`load_populations` nutzt jetzt `deadlock_brain_core::pg::pg_pool()`, der zweite
Env-Name ist raus (`examples/build_evaluation.rs`). Kein `POPULATION_DB_DSN` mehr
im Diff.

**M2, Staple-Hebel ueberspielt die Mechanik (wichtig): behoben.** Commit
`4385392`. `planner.rs:125` `is_priority_staple_step` bindet die kategorische
Prioritaet zusaetzlich an `step.marginal_value >= 0.0`; die drei Sortier- und
Spar-Stellen (`planner.rs:276`, `461`, `475`) nutzen sie. Die Umgehung der
Marginal-Untergrenze ist auf `staple_rescue = forced && marginal_value >= 0.0`
verengt (`planner.rs:254`). Ein solo-positiver, im Kontext anti-synergetischer
Staple wird nicht mehr erzwungen. Empirisch bestaetigt an Warden (Enduring Speed
faellt raus, siehe P5).

**M3, ueberzeichnete nutzersichtbare Begruendung (wichtig): behoben.** Commit
`4385392`. `composer.rs:578` nennt die echte Bedingung ("hebt nur Staples mit
positivem Solo-Mechanikwert, die im aktuellen Build keinen negativen
Marginalwert haben") statt "tragfaehiger Mechanikwert". Deckt sich mit der
Code-Bedingung.

**M4, falscher Nachweis-Zeiger fuer die 6/9 (wichtig): behoben.** M-MESSUNG.md
verweist fuer die 6/9 auf `nachweise/M-SECHS-pop.json`; die irrefuehrenden
Einzellaeufe `M-WARDEN-pop*.json` sind entfernt. Der Ordner
`.tasks/2026-09-16-population-baseline/nachweise/` ist committet
(`M-SECHS-pop.json`, `M-SECHS-nopop.json`, `M-DIAG-fc71b5d.json`,
`M-WARDEN-fc71b5d.json`, alle unter 5 MB).

**M5, Spielerzahl (nit): behoben.** M-MESSUNG und FERTIG-M nennen 119248
Spieler-Matches ueber 10059 Ranked-Matches (38 Helden).

**M6, build.rs-Sentinel (nit): bewusst gelassen.** Von Runde 1 selbst als
akzeptabel eingestuft; uebliches Cargo-Idiom, kein Aenderungsbedarf.

**M7, Umlaute in Commit-Betreffzeilen (nit): behoben.** Die neuen Commit-Betreffs
(`be9a982`, `4385392`, `a2ed800`, `09198ff`, `73e26b9`, `23e0935`) tragen echte
Umlaute ("nicht-negativen", "Nachweise", "Fertigmeldung"), kein ae/oe/ue.

**P-Nits (REVIEW-P, in M1-Briefing gezogen): behoben.** Commit `73e26b9`:
`--rebuild` aus `StatsArgs` entfernt (`cli.rs`), `average_badge`-Filter wie
Python (Test in `clean.rs`), `metadata_params`-Extraktion mit Test in `api.rs`.
`assert_writable` und `WIN_WEIGHT`-Angleichung laut Fertigmeldung, durch die 24
gruenen Population-Tests gedeckt.

## Neue Befunde aus den Fix-Commits

**1. Produktivpfad und Fehlertrennung (Prueffokus 1): sauber.**
`load_population_prior` (`lib.rs:494`) laeuft ueber `ctx.pool`. Der
`to_regclass('brain.population_item_stats')`-Guard trennt korrekt: fehlende
Tabelle liefert `Some(false)` und damit einen leeren Prior ("keine Daten"),
waehrend ein echter Verbindungsfehler ueber `map_err(ReasonerError::Db)?`
propagiert und den Build abbricht, also nicht als "keine Daten" verschluckt wird.
Auch `PopulationIndex::load`-Fehler propagieren (`ReasonerError::Data`). Der
Composer haengt bei leerer Population sichtbar "Population: keine Daten." an
(`composer.rs:370`). Confidence wird nicht kuenstlich erhoeht: der leere Prior
setzt `is_staple` nie und `support` 0, die Sortierung faellt auf den alten
Vergleich zurueck (belegt durch die Vorher-Gleichheit 3/9 und die 386/58-Suite).

Hinweis (nit): Der Guard prueft nur `population_item_stats`, `PopulationIndex::load`
liest aber zusaetzlich `population_ability_order` und `population_imbue_stats`
(`index.rs`). Bei einem Teil-Schema (item_stats vorhanden, die anderen fehlen)
briche der Build ab statt zu degradieren. Die Migration legt die Tabellen
gemeinsam an, der Zustand ist praktisch all-or-nothing; kein Handlungsbedarf,
nur notiert.

**2. Backtest-Ausweisung (Prueffokus 2): sauber.** `HeroBacktest.population`
ist mit `#[serde(default)]` ergaenzt (`types.rs:472`), `PopulationBacktest`
haelt `Option`-Felder (`backtest.rs:7-15`), das `Display` schreibt je Held eine
Zeile "Staple-Gate | Kendall tau | Jaccard@12" mit "null"-Fallback
(`backtest.rs:118-136`). Null ohne Population ist getestet
(`backtest.rs:511-520`: passing/failing/untracked). Persistiert wird ueber
`persistence_json(hero)` in die bestehende jsonb-Spalte `reasoner_backtests.detail`
(`lib.rs:381` UPSERT, `lib.rs:486`), keine neue Migration und idempotent per
`ON CONFLICT ... DO UPDATE`. Passt zum Briefing.

**3. Timer-Skript (Prueffokus 3): sauber.** `run_build_data_with_infisical.sh`
haengt `population sync --matches 2000` und `population stats` nach dem
bestehenden `pull build-data --hero all` an. Trotz `set -euo pipefail` sind die
Schritte nicht-fatal: `cmd || echo "..." >&2` liefert Exit 0 und laesst `set -e`
nicht ausloesen, der Fehltext geht aber ins Journal (stderr). Der bestehende
Sync wird nicht rueckwirkend als gescheitert markiert. Kein EnvironmentFile, kein
Secret-Echo, DSN kommt wie bisher. `sync` ist inkrementell/idempotent
(`since`/`max_match_id`-Cursor in `cli.rs`/`api.rs`), Timer-Overlap ist bei einem
systemd-Oneshot ausgeschlossen.

**4. Staple-Hebel-Bedingung (Prueffokus 4): behoben, Tests teilweise.** Die
Bedingung ist exakt `marginal_value >= 0.0` (`planner.rs:126`, `254`), der
Begruendungstext traegt nur, was die Zahl hergibt (siehe M3). Aber: Commit
`4385392` fuegt nur die Logik ein, keine gezielte Testfunktion, die belegt, dass
ein solo-positiver Staple mit negativem Kontext-Marginalwert nicht erzwungen
wird. Der Gate ist nur indirekt (bestehende Planner-Tests, Reasoner-Lib bleibt bei
170) und empirisch (Warden-Messung) abgedeckt. Nit, kein Blocker.

**5. ABWEICHUNG Warden-Staple-Gate (Prueffokus 5): nachvollziehbar und ehrlich.**
Das committete `M-SECHS-pop.json` deckt sich Zahl fuer Zahl mit M-MESSUNG Runde 2:
Warden `staple_gate_passed=false`, `kendall_tau=0.4642`, `jaccard_at_12=0.5`,
`missing_staples=[2447176615]` (Enduring Speed), Waffen 6 (nopop 3). Die sechs
weiteren Helden sind neu gemessen und ohne Regression gegen die eigene
Vorher-Basis: Infernus 3/7 zu 5/7, Lady Geist 1/7 zu 2/7, Abrams 0/7 zu 3/7,
Vindicta 1/3 zu 2/3, Bebop 0/4 zu 0/4, Ivy 2/3 zu 2/3; ihre Gates bleiben
bestanden bzw. null (Ivy). Die Abweichung ist offen als Modellgrenze
dokumentiert (Mobilitaet/Ausdauer von Enduring Speed nicht modelliert, gleiche
Klasse wie Monster Rounds), nicht versteckt. Warden bleibt mit 6/9 ueber der
Latte 5/9.

Freigabe-Hinweis fuer den Orchestrator: Das AUFTRAG-Fertig-Kriterium "Warden ...
Staple-Gate bestanden" ist damit fuer Warden nicht mehr erfuellt. Das ist der
berechtigte Kern von M2 (kein Erzwingen anti-synergetischer Items) und schlaegt
das Gate-Kriterium; die Abweichung ist begruendet festgehalten. Sie ist ein
Abnahme-Punkt fuer den Nutzer, kein Code-Mangel.

**6. Bau (Prueffokus 6): bestaetigt.** Alle Zahlen selbst gefahren und gruen
(170/16, 24, Clippy sauber, 386/58), kein `--release`.

**7. Stil (Prueffokus 7): sauber.** Keine Em-Dashes (U+2014) in neuen Zeilen,
keine Code-Kommentare in neuen Rust-Zeilen, keine Modellnamen, keine Secrets,
echte Umlaute in Texten und Commit-Betreffs.

## Urteil

Alle vier wichtigen Maengel behoben, alle Nits behoben oder bewusst akzeptiert,
Bau gruen, Nachweise stimmig, Abweichung ehrlich deklariert.

Offene wichtige Maengel: 0
Offene Nits: 2 (gezielter Unit-Test fuer den Marginal-Gate fehlt; Guard prueft nur eine von drei Populationstabellen)
Abnahme-Punkt fuer den Nutzer: 1 (Warden-Staple-Gate nicht mehr bestanden, als begruendete Spec-Abweichung)
