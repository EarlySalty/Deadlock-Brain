# Startbrief Agent C: Kaufplanungsregression

Basis: `e263b5f69c10f613b1df97e4e035acdf9b71dc71`.

## Aktueller harte Befund

Die saubere Release-Auswertung gegen unverändertes `FROZEN-V2.json` (SHA256 `5451b0b4f3cde928f12f09a4ea5f4fa3128c93fd8ca227f8208a1357f6cb7b1a`) liefert für Warden Build 779996 Version 45 nur **1/9 Referenzwaffen** (`Titanic Magazine`), Recall `1/19 = 0,0526316`, Jaccard `0,0294118`.

Der dokumentierte frühere gemeinsame Zwischenstand erreichte **5/9 Waffen**. Keine Itemnamen oder Autorenreferenzen in die Logik übernehmen; der Fall dient nur als Regressionserkennung.

## Auftrag

1. Prüfe unabhängig vom Kampfagenten, ob Planner/Inventar/Progression den Rückgang verursacht: Kandidatenfilter, Kostenbandrest, Sparentscheidung, Zweischritthorizont, Verkaufserlös, Upgradeverbrauch, 12 Slots, Active-Limit, `used`, feste Imbue-Bindung und Skillprogression.
2. Erzeuge generische Minimaltests für jede gefundene Planungsinvariante. Besonders prüfen: bezahlbare positive Käufe dürfen nicht durch fehlerhafte Zukunftsbewertung verschwinden; Verkauf darf nur nach Verlustbewertung erfolgen; erfüllte Layoutbänder dürfen andere Bänder nicht blockieren; erworbene Reihenfolge muss Publish entsprechen.
3. Vergleiche `plan`-Ausgabe mit `compose_build`: jeder Core-Kauf muss exakt einem Planübergang entsprechen, ohne nachträgliches Resortieren oder Phantomitem.
4. Keine Kampfsemantik, Referenzdaten oder Holdoutlogik ändern.

## Dateibesitz

Nur `planner.rs`, `inventory.rs`, `progression.rs`, `composer.rs` und eigener Report. `types.rs`/`lib.rs` nicht anfassen; notwendige Schnittstellenänderung im Abschlussreport anfordern.

## Gate

- relevante neue Regressionstests;
- `cargo test -p dbrain-reasoner --lib` ohne neue Fehler;
- `cargo clippy -p dbrain-reasoner --all-targets -- -D warnings`;
- Report unter `agents/planner-ABSCHLUSS.md` mit Ursache, Invarianten, Vorher/Nachher und Grenzen.
