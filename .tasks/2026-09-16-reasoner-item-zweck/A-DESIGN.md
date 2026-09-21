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

Korrigiert nach Nutzervorgabe vom 16.09.2026 (frueherer Punkt 4 und die
Vorzeichenregel waren BLOCK):

1. Eine kanonische Funktion (in `mechanics.rs`), einzige Definition der
   Spirit->Feuerrate-Konversion (Schuss/s je 1 Spirit): `ERoundsPerSecond` direkt,
   sonst `EFireRate * shots_per_second / 100` als Fallback, nie beide addiert.
2. `combat.rs` (Sim/Marginalpfad) und `item.rs::spirit_fire_rate_value`
   (statischer Score) konsumieren diese Funktion, ihre inline-Ableitungen entfallen.
3. Die Konversion erreicht AUCH `mechanics::damage_plan`: `weapon_dps`,
   `weapon_share` und `primary_axis` beziehen den Spirit->Waffen-Beitrag am
   innewohnenden `base_spirit_power` des Helden ein. Damit ist der Loader->
   Mechanics->Combat->Item->Planner-Verbrauch wirklich vereinheitlicht und die
   Klassifikation eines Konverters spiegelt seine Spirit-als-Waffen-Identitaet.
   Das verschiebt `purchase_bonus_value` (nutzt `damage_plan.weapon_dps`) und die
   Achsen-Klassifikation; die Wirkung wird im Backtest gemessen. Wird der Gate
   schlechter, wird das als BLOCK ausgewiesen, nicht durch Scope-Verzicht vermieden.
4. Vorzeichen-/Endlichkeitsregel (Korrektur): endliche negative Konversion ist eine
   DOWNSIDE und bleibt SIGNIERT erhalten (Spirit senkt die Feuerrate), nicht 0.
   Nicht endliche Werte (NaN/Inf) sind unbekannt/fehlerhaft und werden verworfen
   (als fehlend behandelt, nicht mit einem endlichen Minus gleichgesetzt). Damit
   wird item.rs auf die signierte combat.rs-Semantik gezogen, combat.rs verliert nur
   den Poison-Fall nicht endlicher Daten. Physikalische Gesamtstat-Grenzen (eine
   Feuerrate faellt real nicht unter 0) gehoeren in die Stat-Anwendung, nicht in die
   Konversionsdefinition.

## Roster-Realitaet (Modell-Abdeckung)

`hero_conversion_coverage` auf FROZEN-V2: 38 Helden, genau 1 Konverter (Warden,
ERoundsPerSecond 0.01), 37 Nullkonverter, 0 negativ/nicht endlich, 0 Vorzeichen-
Divergenz. Auf diesem Roster ist die Vereinheitlichung fuer combat.rs/item.rs
verhaltensneutral; die einzige messbare Aenderung kommt aus der damage_plan-
Erweiterung. Ein negativ skalierender Held kommt hier nicht vor; die signierte Regel
ist trotzdem verbindlich und getestet, damit ein spaeterer Patch sie nicht still
verliert.

## Messgrenze (verbindlich benannt)

Der Vorher/Nachher-Vergleich nutzt FROZEN-V2 (Heldenmodelle/Items, Algorithmus-
Stand a57382a, 2026-09-13) plus eine frisch eingefrorene Population (2026-09-16).
Das ist reproduzierbar und fuer beide Codefassungen identisch, aber ein GEMISCHTER
historischer Stand (Item-/Patchmischung), keine frische Gesamtbaseline. Der frische
Vollfreeze scheiterte an einem Performance-Blowup (siehe PHASE-0.md); er wird als
separater BLOCK gefuehrt, nicht aus der Gesamtfreigabe gestrichen.

## Nachweis

- Regression/Determinismus: `build_evaluation evaluate` auf FROZEN-V2 mit
  `FROZEN_POPULATIONS` fuer BEIDE Codefassungen; Vergleich gegen die frische
  Phase-0-Baseline, nicht gegen die eingebettete alte `live_baseline`.
- `combat_parity` (DB-frei) vorher/nachher als zusaetzlicher Sim-Determinismus.
- Generische Tests: kanonische Funktion (positiv/fehlend/negativ-signiert/nicht
  endlich verworfen/Praezedenz/Prozent-Fallback), Umbenennungs-Neutralitaet,
  +Spirit hebt Waffen-DPS bei Konverter und nicht bei Nullkonverter, damage_plan
  spiegelt Spirit beim Konverter.
