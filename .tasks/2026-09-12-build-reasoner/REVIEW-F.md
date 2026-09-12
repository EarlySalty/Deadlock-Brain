# REVIEW-F (Runde 1): build-reasoner Paket F

**Urteil: NACHBESSERN**

Kern ist als Einkaufskurve nach Kostenband sauber gebaut, Tests gruen, Clippy
sauber, die E-Regeln (nur positive Scores im Kern, Situationsbloecke) bleiben
erhalten. Blockierend ist nichts. Der zentrale Zweck von F, die Zielgroesse des
Kerns, ist aber falsch kalibriert: die unabhaengige Rundung je Band summiert zu
einem systematisch zu kleinen Kern (Warden live 8 Items gegen Gesamtmedian 10,5
und Referenz 19). Das gehoert vor dem Merge korrigiert.

## Nachweise

- Reasoner-Tests: `cargo test -p dbrain-reasoner` -> 90 passed, 0 failed, 16 ignored.
- Workspace: `cargo test --workspace` -> 278 passed, 0 failed, 58 ignored.
- Clippy: `cargo clippy -p dbrain-reasoner --all-targets -- -D warnings` -> exit 0, 0 Warnungen.
- DB read-only (Central-Pool, DEADLOCK_CENTRAL_DSN): Build 779996 = Warden v45,
  5 modCategories. Genau eine mit "CORE" im Namen ("CORE ITEMS /// CAN BUY ANY
  3K ITEM INSTEAD OF VEIL", 19 mods). TRYHARD 1, CAN BUY 1 OR 2 6, GREED 2,
  namenlos 11.
- Live: `deadlock-brain reason build Warden --no-ai --no-persist --json`
  -> `core` = 8 Items, Tier-Verteilung Tier2:1, Tier3:2, Tier4:5, Tier1/5:0.

TESTNACHWEIS[TW-1]: 278 passed, 58 ignored | Baseline: 273 passed, 58 ignored (E), keine Tests verschwunden, +5 neu

## Maengelliste

### Wichtig

1. **Kern systematisch zu klein (`meta.rs:180` `layout_stats`, Feld `target`).**
   Jedes Bandziel ist `median.round()` einzeln je Tier. Die Summe der gerundeten
   Bandmediane trifft nicht den Median der Kern-Gesamtgroesse: Warden
   0+1+2+5 = 8, waehrend `total_median` 10,5 ist und der Referenzkern (779996)
   19 Items hat. Live reproduziert: Warden-Kern = 8. Unabhaengige Rundung
   nach unten verliert reproduzierbar Items, das ist kein einmaliges Artefakt,
   sondern der Regelfall bei mehreren halbzahligen Bandmedianen.
   *Fix:* Bandziele per Groesste-Reste-Verfahren so verteilen, dass ihre Summe
   `total_median.round()` trifft (Nachkommaanteile absteigend aufrunden, Rest
   abrunden). Siehe Kalibrierungsfrage KF1.

2. **Flex-Budget haengt am unterschaetzten Ziel (`meta.rs:154`
   `flex_slots = total_target.saturating_sub(12)`, genutzt in
   `composer.rs` `select_core_items`).** Weil `total_target` durch Mangel 1 zu
   klein ist, ist `flex_slots` fast immer 0 (Warden: 8 - 12 -> 0). Der Flex ist
   damit an den falschen Zielwert gekoppelt statt an die Slot-Kapazitaet und
   verschaerft die Unterfuellung. Mit Fix 1 gemeinsam loesen: Flex aus der
   Slot-Verteilung der korrigierten Ziele ableiten.

### Nit

3. **rustfmt-Churn bloeht den Diff** (Import-Reihenfolge `data.rs:4`
   `{Row, postgres::PgPool}`, Re-Export-Umsortierung `lib.rs` ai_roles, Kollaps
   von `per_soul_value` und mehreren `assert!`-Bloecken). Stammt aus einer
   anderen Toolchain-Version, nicht funktional. Kann bleiben, macht das Review
   nur lauter.

4. **Bandziel kann bei einseitiger Slot-Verteilung unterfuellt bleiben
   (`composer.rs` `select_core_items`).** Sind viele Kandidaten eines Bands im
   selben Slot und der Flex erschoepft, wird der Rest per `continue`
   uebersprungen und landet spaeter in Optional; das Band bleibt unter Ziel.
   Edge-Case, bei realer Slot-Streuung unkritisch.

## Geprueft und in Ordnung

- **Layout-Erkennung (Punkt 1).** `core_categories` matcht Kategorienamen gegen
  core/kern/standard/main/primary/basis/default, sonst groesste Kategorie. Auf
  779996 trifft genau die CORE-Kategorie mit 19 Items, deckt sich mit dem Seed.
  Die in FERTIG-F genannten 28 Kern-Items reproduzieren aus der aktuellen
  Version (v45) nicht, dort sind es 19; die 28 sind vermutlich eine veraltete
  oder ueber mehrere Builds addierte Zahl, kein Mangel.
- **Mid-Phase ohne Scoring-Nebenwirkung (Punkt 3).** `buy_phase` wird in
  `item.rs:106` nur auf dem ScoredItem gespeichert, `per_soul_value` haengt in
  `item.rs:52` an `combat_value` und `cost`, nicht an der Phase. Die neue
  Mid-Phase steuert weder Zustandsfaktor noch per_soul-Gewichtung. In
  `select_core_items` entscheidet per_soul_value vs total nach Tier (<=2 bzw.
  ab 3), nicht nach Phase. `mechanics.rs` durfte laut Briefing mit Begruendung
  angefasst werden; die Aenderung ist auf die Phasenreihenfolge begrenzt und
  konsistent (Mid erbt Sell-Priority None wie Core/Late).
- **E-Regeln erhalten.** `core_candidates` filtert Situationsitems und
  `score.total <= 0` raus, der Kern bleibt rein positiv. Slot-Caps 4/4/4 plus
  Flex und Reihenfolge Lane, Mid, Core, Late sind umgesetzt.

## Kalibrierungsfragen (getrennt, mit Empfehlung)

- **KF1 Zielgroesse des Kerns.** Groesste-Reste auf `total_median` (Empfehlung)
  gegen Q3 als Bandziel. Groesste-Reste trifft die Absicht "Kern als Kurve ueber
  das Spiel" am fairsten und haelt die Bandproportionen, ohne Gewichte. Q3
  macht Builds groesser und aggressiver, geht aber ueber den Median hinaus und
  waere eine bewusste Stilentscheidung, keine neutrale Zielgroesse. Empfehlung:
  Groesste-Reste auf `total_median.round()`.
- **KF2 Vergleich E gegen F im Backtest.** Kern-Ueberdeckung ist auf den
  Reasoner-Kern normiert, ein kleinerer Kern hebt sie mechanisch (F 8 Items
  0,375, E 19 Items 0,2105), das misst nicht Qualitaet. Fair vergleichbar sind
  nur groessenrobuste Kennzahlen: Treffer je Referenz-Kern-Item (Recall gegen
  die 19er-Referenz) und zusaetzlich Jaccard(Reasoner-Kern, Referenz-Kern).
  Empfehlung: beide ausweisen, Kern-Ueberdeckung als alleinigen Vergleich fallen
  lassen.

## Separates Paket (kein F-Mangel)

- **Loader-Fehler bei anderen Helden.** `reason build "Lady Geist"` und
  `reason build Infernus` (je `--no-ai --no-persist --json`) brechen mit exit 1
  und "Datenfehler: Ability 0 gehoert nicht zum geladenen Helden" ab. Ursache:
  `hero.rs:20` `integer(ability.get("id"))` liefert 0, wenn eine
  Ability-Snapshot-Zeile kein oder ein leeres `id`-Feld hat; `position()` an
  `hero.rs:24` findet keine Ability mit id 0 und wirft den Data-Fehler. Also
  unvollstaendige Snapshot-Daten oder falscher id-Schluessel fuer diese Helden,
  Warden hat vollstaendige ids und laeuft. `hero.rs` ist im F-Diff unberuehrt,
  Fehler ist vorbestehend. Eigenes Paket.
