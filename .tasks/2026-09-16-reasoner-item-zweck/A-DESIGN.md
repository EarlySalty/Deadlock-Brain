# Phase A – Konstruktionsnotiz (fuer den A-Reviewer)

Stand: Basis-Commit 706b129, Arbeitsstand Feature-Branch `feat/reasoner-purpose-a`.
Diese Notiz beschreibt, was Phase A aendert und warum. Sie ist kein Freigabe-Ersatz;
der vollstaendige A-Review folgt nach dem fertigen A-Patch.

## Ausgangsbefund: der Konversionsgraph existiert bereits, aber doppelt

Die Spirit->Waffen-Feuerraten-Konversion ist im Produktivcode heute an ZWEI Stellen
getrennt abgeleitet, aus denselben Rohdaten (`HeroModel.scaling`, Stat
`ERoundsPerSecond` direkt bzw. `EFireRate` als Prozent-Fallback):

- `combat.rs` (~L639, Sim-Pfad, treibt den Planner-Marginalwert):
  `spirit_rate` ungefiltert, Vorzeichen bleibt erhalten.
- `item.rs::spirit_fire_rate_value` (~L150, statischer Item-Score, ordnet den
  Kompositions-Katalog): eigene Ableitung, verwirft negative/nicht endliche Skalen.

`mechanics::damage_plan` zieht die Konversion gar nicht ein (weapon_dps ist reine
Basiswaffe). Der Loader (`data.rs`) ist die gemeinsame Datenquelle.

Das ist die vom Nutzer benannte Lage: "parallel dieselbe Konversion", mit einer
echten Divergenz bei negativen/nicht endlichen Skalen.

## Doppelzaehlung: geprueft, heute kein additiver Doppelzaehler

`plan_build` -> `compose_build_with_sources`: die statischen Scores (mit
`spirit_fire_rate_value`) bestimmen nur die Katalogreihenfolge (`item_order`), der
Planner-Marginalwert kommt ausschliesslich aus dem Combat-Sim
(`evaluate_inventory`, mit `spirit_rate`). `supported_value = marginal + population`,
kein `score.total` addiert. Beide Pfade zaehlen die Konversion also je einmal fuer
ihre eigene Aufgabe; sie werden nicht summiert. Der Nutzerhinweis
"Doppelzaehlung verhindern" bezieht sich deshalb auf: keine DRITTE Ableitung
ergaenzen und die zwei bestehenden auf eine gemeinsame Definition zusammenfuehren.

## Phase-A-Aenderung (minimal, vereinheitlichend)

1. Eine kanonische Funktion (in `mechanics.rs`), einzige Definition der
   Spirit->Feuerrate-Konversion (Schuss/s je 1 Spirit): `ERoundsPerSecond` direkt,
   sonst `EFireRate * shots_per_second / 100` als Fallback, nie beide addiert.
2. `combat.rs` und `item.rs::spirit_fire_rate_value` konsumieren diese Funktion,
   ihre inline-Ableitungen entfallen. Damit ist der Loader->Mechanics->Combat->
   Item->Planner-Verbrauch vereinheitlicht.
3. Vorzeichen-/Endlichkeitsregel wird EXPLIZIT und getestet festgelegt statt wie
   heute zwischen den Pfaden zu divergieren. Wahl und Begruendung siehe unten.
4. Keine Aenderung an `damage_plan`-Magnituden (vermeidet Verschiebung von
   `purchase_bonus_value` und Klassifikation, also Regressionsrisiko). Die
   Konversion wirkt weiter dynamisch im Sim und im Item-Score.

## Vorzeichenregel

Modell-Abdeckung ueber das Roster (Werkzeug `hero_conversion_coverage`) zeigt auf
FROZEN-V2: 0 Helden mit negativer/nicht endlicher Skala, 0 Vorzeichen-Divergenz.
Die Vereinheitlichung ist damit auf dem realen Roster verhaltensneutral. Die
kanonische Funktion legt die Regel dennoch explizit fest (negative/nicht endliche
Skala -> als fehlend behandelt, entspricht dem heutigen item.rs-Pfad), weil eine
negative Spirit->Feuerrate physikalisch kein Waffennutzen ist; die combat.rs-Seite
verliert dadurch nur den ungetesteten Sonderfall, der auf dem Roster nicht vorkommt.
Der fresh Phase-0-Freeze wird gegengeprueft; falls dort ein Held negativ skaliert,
wird das als offene Differenz im Bericht ausgewiesen statt stillschweigend geglaettet.

## Nachweis

- Determinismus/Regression: `combat_parity` (DB-frei) vorher/nachher ueber das ganze
  Roster; erwartet identisch (0 Divergenz).
- Referenzwaffen/Staples/Kendall/Jaccard: `build_evaluation evaluate` auf identisch
  eingefrorenem Snapshot inkl. eingefrorener Population (FROZEN_POPULATIONS).
- Generische Tests: kanonische Funktion (positiv/fehlend/negativ/Praezedenz/
  Prozent-Fallback), Umbenennungs-Neutralitaet, +Spirit hebt Waffen-DPS bei
  Konverter und nicht bei Nullkonverter.
