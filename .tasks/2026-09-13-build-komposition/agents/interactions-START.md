# Startbrief Agent B: Kampf-/Interaktionsregression

Basis: `e263b5f69c10f613b1df97e4e035acdf9b71dc71`.

## Aktueller harte Befund

Die saubere Release-Auswertung gegen unverändertes `FROZEN-V2.json` (SHA256 `5451b0b4f3cde928f12f09a4ea5f4fa3128c93fd8ca227f8208a1357f6cb7b1a`) liefert für Warden Build 779996 Version 45 nur **1/9 Referenzwaffen** (`Titanic Magazine`), Recall `1/19 = 0,0526316`, Jaccard `0,0294118`.

Der dokumentierte frühere gemeinsame Zwischenstand erreichte **5/9 Waffen**. Damit ist die Integrationsfassung fachlich NICHT mergefähig. Dieser Befund ist eine Entwicklungsreferenz, kein Auftrag, Itemnamen zu hardcoden.

## Auftrag

1. Regression zwischen dem dokumentierten 5/9-Zwischenstand und `e263b5f` eingrenzen. Prüfe besonders die später integrierten echten Ability-/Item-Interaktionen, Ziel-HP-Capping, Proc-/Imbue-Cooldowns und die daraus entstehenden Inventar-Marginalwerte.
2. Reproduziere Ursachen mit generischen mechanischen Gegenproben. Eine Korrektur muss aus Spiel-/Snapshotsemantik folgen und auch für andere Helden gelten.
3. Prüfe die sechs in `MESSUNG-ZWISCHENSTAND.md` genannten Helden erneut auf plausible Ability-/Proc-Ausgaben; Null ist nur erlaubt, wenn mechanisch begründet.
4. Keine Referenzlisten, keine heldenspezifischen Gewichte und keine Planner-/Composer-Änderungen.

## Dateibesitz

Nur `combat.rs`, `ability_interactions.rs`, `item_interactions.rs`, `data.rs`, `hero.rs`, `item.rs` und passende Fixtures. `types.rs`/`lib.rs` nicht anfassen; notwendige Schnittstellenänderung im Abschlussreport anfordern.

## Gate

- relevante neue Regressionstests;
- `cargo test -p dbrain-reasoner --lib` ohne neue Fehler;
- `cargo clippy -p dbrain-reasoner --all-targets -- -D warnings`;
- Report unter `agents/interactions-ABSCHLUSS.md` mit Ursache, Commitbereich, Vorher/Nachher und offenen Unknowns.
