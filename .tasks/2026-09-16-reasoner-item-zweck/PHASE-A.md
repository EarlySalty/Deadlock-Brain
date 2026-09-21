# PHASE-A: kanonische Spirit->Feuerraten-Konversion vereinheitlicht

Stand 2026-09-16. Branch `feat/reasoner-purpose-a`. Basis-Commit 706b129.
Phase-A-Produktcommits: f68dcc3 (Vereinheitlichung), 6ca86c3 (Astra-Review-Fixes
R1+R2). Messbinary-Revision Nachher: `6ca86c3`. Vorher-Baseline: `806d4d58`
(Produktivstand vor Phase A). Beide auf identischen eingefrorenen Eingaben.

## Was Phase A aendert (Produktcode)

- Eine Definition der Konversion: `mechanics::spirit_weapon_rate_provenance` /
  `spirit_weapon_rate_per_spirit`. `combat.rs` (Sim/Marginal), `item.rs`
  (statischer Score) und `mechanics::damage_plan` leiten die Spirit->Feuerrate
  nicht mehr getrennt ab. `ERoundsPerSecond` direkt, `EFireRate` nur Prozent-
  Fallback, nie beide addiert.
- `damage_plan` und der Item-Score nutzen dieselbe zustandsbezogene Waffen-
  projektion `mechanics::weapon_with_spirit`; der innewohnende `base_spirit_power`
  fliesst genau einmal ein (Feuerrate physikalisch bei 0 gedeckelt).
- Endliche negative Konversion bleibt SIGNIERT (Downside). Nicht endliche Werte
  sind unbekannt: valider Alias darf sie als Recovery ersetzen, sonst bleibt es
  unbekannt (rate 0) und senkt im Item-Score die Confidence samt Evidence-Nachweis.

## Vorher/Nachher-Backtest (identisch eingefroren)

Befehl (Vorher wie Nachher):
`FROZEN_POPULATIONS=nachweise/PHASE0-POP.json build_evaluation evaluate FROZEN-V2.json <out> "Warden,Infernus,Abrams,Lady Geist,Vindicta"`

Ergebnis: die Auswertungsdateien sind byte-identisch (Vergleich ohne die Zeile
`algorithm_revision`: 0 Diff-Zeilen). Alle Metriken unveraendert:

| Held | Autor-Build | Referenzwaffen | jaccard@12 | kendall | Staple-Gate | Delta |
|---|---|---|---|---|---|---|
| Warden | 779996 | 6/9 | 0.500 | 0.4706 | 9/10 FALSE | 0 |
| Infernus | 256053 | 5/7 | 0.846 | 0.6557 | 9/9 true | 0 |
| Abrams | 749234 | 3/7 | 0.263 | 0.7152 | 4/4 true | 0 |
| Lady Geist | 253366 | 2/7 | 0.263 | 0.6419 | 4/4 true | 0 |
| Vindicta | 805943 | 2/3 | 0.600 | 0.3799 | 9/9 true | 0 |

Artefakte: nachweise/PHASE0-EVAL.json (Vorher), nachweise/PHASEA-EVAL.json
(Nachher). Das rote Warden-Staple-Gate (fehlt Enduring Speed 2447176615) bleibt
rot; es wird nicht weichdefiniert.

## Warum 0 Delta ehrlich und erwartbar ist

Auf FROZEN-V2 ist `base_spirit_power` fuer alle Helden `null` -> 0.0 (Spirit wird
ueber Items/Level erworben, nicht als Startwert). Damit ist der am innewohnenden
Spirit angesetzte Konversionsbeitrag in `damage_plan` und die R1-Basisverankerung
im Item-Score exakt 0; sie aendern den Basiszustand nicht. Zusaetzlich zeigt die
Roster-Modell-Abdeckung ueber alle 38 Helden 0 Vorzeichen-Divergenz und
0 nicht endliche Konversionen. Daraus folgt roster-weit:

- `combat.rs`: alter und neuer Wert unterscheiden sich nur bei nicht endlichen
  Daten (0 Faelle) -> identisch.
- `item.rs`: alte Filterung negativer Werte gegen neue signierte Behandlung
  unterscheidet sich nur bei negativen Skalen (0 Faelle); die R1-Verankerung ist
  bei `base_spirit_power=0` ein Nulloffset -> identisch.
- `damage_plan`: Spirit-Beitrag `= base_spirit_power * rate = 0` -> identisch.

Phase A ist damit auf diesem gemischten historischen Stand regressionsfrei und
zugleich semantisch korrigiert. Die Wirkung der Korrekturen ist an den Faellen
belegt, die hier nicht vorkommen (siehe Tests), nicht durch veraenderte Eingaben.

## Belegte Wirkung der Korrekturen (Unit-Tests, Produktivpfad)

- `mechanics::damage_plan_reflects_spirit_only_for_a_converter`: bei
  `base_spirit_power=50` und ERoundsPerSecond 0.2 hebt der Konverter seine
  weapon_dps ueber den Basiswaffenwert, der Nullkonverter nicht.
- `item.rs::r1_item_marginal_anchors_base_spirit_exactly_once`: Gegenprobe aus dem
  Astra-Review am oeffentlichen Consumer `spirit_fire_rate_value`
  (bullet 10, clip 16, reload 2, Basisrate 4, k 0.01, Item +100 Spirit):
  base_spirit 50 -> passive_dps 512/135 = 3.7925926; Kontrollfall base_spirit 0 ->
  160/39 = 4.1025641.
- `item.rs::r2_nonfinite_conversion_is_visible_as_unknown_and_lowers_confidence`:
  nicht endlicher Primaerwert ohne validen Ersatz -> Confidence Low + sichtbarer
  Unbekannt-Nachweis; valider Konverter -> High, kein Nachweis.
- `mechanics::spirit_rate_provenance_reports_visible_status`,
  `spirit_weapon_rate_keeps_finite_negative_signed_as_downside`,
  `spirit_weapon_rate_drops_non_finite_as_unknown`,
  `spirit_weapon_rate_is_name_independent` (Umbenennungs-Neutralitaet),
  `spirit_weapon_rate_uses_rounds_directly_and_never_adds_fire_rate`,
  `combat::spirit_does_not_change_weapon_rate_for_a_null_converter` (Gegenprobe zu
  `spirit_changes_whole_weapon_rate_and_magazine_value`).

## Tests / Formatter / Clippy

TESTNACHWEIS[TW-1]: 184 passed, 16 ignored | Baseline: 0 rot (vorher 173 passed,
16 ignored; +11 neue Phase-A-Tests, davon 3 im Review-Nachgang).
Befehl: `cargo test -p dbrain-reasoner --lib` (cargo 1.97.1), Exit 0.
Clippy: `cargo clippy -p dbrain-reasoner --lib --examples -- -D warnings`, Exit 0.
rustfmt auf den geaenderten Dateien (`--edition 2021`); die vorbestehende
population_prior-Formatabweichung im crate-weiten `cargo fmt --check` stammt nicht
aus Phase A und wurde nicht angefasst.

## Offene Punkte / BLOCK-Uebergabe

- Frischer Gesamtfreeze weiter als Performance-BLOCK (PHASE-0.md): die
  damage_plan-Wirkung bei `base_spirit_power>0` ist im Produktivpfad erst nach
  einem frischen Freeze auf Live-Assets messbar; auf FROZEN-V2 ist sie inert.
- Diese Messung ist der gemischte FROZEN-V2 (a57382a) + frische Population
  (2026-09-16), keine frische Gesamtbaseline.
- Kein Selbst-Review als Freigabe. Folge-Review durch ChatGPT gemaess
  ARBEITSTEILUNG.md; kein main-Merge/Deploy/Publish durch diesen Worker.
