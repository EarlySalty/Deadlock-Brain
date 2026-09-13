# Gemeinsame Build-Bewertung

Stand: 13.09.2026, verbindlicher begrenzter Ausbau auf main a57382a.
Die fachliche Abnahme ist offen. Technisch bestandene Tests ersetzen sie nicht.

## Anspruch und Grenze

Der Reasoner bewertet das tatsächlich gehaltene Inventar in mehreren expliziten
Kampfszenarien. Derselbe Wert dient der Kaufplanung und ihren Begründungen.
Er ist ein deterministisches, begrenztes Kampfmodell, keine vollständige
Nachbildung der Spielengine und keine bewiesene historische Meta-Prognose.
Unbekannte Wirkungen und gesetzte Gegner-/Kampfannahmen bleiben in der Ausgabe.
Es gibt keine Helden-Sondergewichte, Referenz-Itemlisten im Algorithmus,
neuen KI-Zugänge oder Modellwechsel.

## Daten- und Bewertungspfad

Aktuelle zusammengehörige Hero-/Item-Snapshots bleiben die Mechanikquelle.
Loader erhalten zusätzliche Wirkungs- und Upgradeinformationen, soweit sie
aus den Snapshots tatsächlich ableitbar sind. Nach dem Snapshot bereits
enthaltene Patchänderungen werden weiterhin nicht nochmals aufaddiert.

`combat::evaluate_inventory(hero, items, config)` betrachtet alle gehaltenen
Items gemeinsam: Shopboni, Spirit-Skalierung, Waffenschaden und Feuerrate,
Magazin und Nachladen, Fähigkeiten und verfügbare Kampfzeit, Auslöser,
Cooldowns, Buffdauer und Zustandsbedingungen. Schaden, Heilung/Schutz,
Utility, Auslösezahlen und Ablauf bleiben getrennt nachprüfbar. Verkauftes
oder beim Upgrade verbrauchtes Inventar liefert keinen späteren Effekt.
Nicht verlässlich modellierbare Wirkungen werden ausdrücklich unquantifiziert.

## Kaufpfad

`inventory` prüft Käufe, Komponentenverbrauch, Verkäufe, Nettokosten und
Slotgrenzen atomar. `planner` verwendet diesen Übergang und bewertet das
gesamte Inventar danach. Auswahl nach marginalem gemeinsamen Nutzen ersetzt
das Einzelitem-Ranking; eine begrenzte Suche hält mehrere Kandidaten offen
und berücksichtigt einen weiteren Kauf. Rechenaufwand und Suchgrenzen sind
fest und heldenunabhängig. Die Kurve erklärt Erwerb, Kosten, Besitzwechsel
und mechanischen Mehrwert; Upgradeverbrauch ist kein Verkauf.

Neueste Builds aktiver beobachteter Autoren liefern weiterhin Layout,
Kern-/Situationsrollen und Skillfolge. Paarstatistik bleibt ein getrennt
benanntes Nebensignal. Sie ersetzt die gemeinsame Mechanik nicht.

## Dateibesitz

- Kampfmodell: `combat.rs`, `inventory.rs`, `types.rs`, `data.rs`, `hero.rs`,
  `item.rs`, `mechanics.rs`; in `lib.rs` nur Kampf-/Inventar-Modulexporte.
- Planner: `planner.rs`, `composer.rs`, nötige `meta.rs`/`backtest.rs`-Integration;
  `lib.rs` Fassade und Planner-Export; diese Akte.
- Messbasis: lesende Freeze-/Vergleichs-Examples und Messbelege.
- Hauptsession: Integration, unabhängige Abnahme und reguläres Gate, Release,
  Betriebsprüfung und Bereinigung.

## Fachliche Abnahme

Vorher und nachher laufen gegen dieselben eingefrorenen Mechanik-, Meta- und
Referenzdaten. Warden muss mindestens fünf der neun Referenzwaffen aus Build
779996 Version 45 treffen. Das allein genügt nicht: mehrere andere Helden,
physikalisch plausible Abläufe und Inventar-/Kostenübergänge werden geprüft.

Autorenübereinstimmung aus verwendeten Lehrdaten wird als In-Sample-Ergebnis
gekennzeichnet. Zurückgehaltene Autoren dürfen weder Layout noch Kernrolle,
Verkaufsbeleg, Skillfolge oder andere Lehrsignale liefern. Deren Ergebnis steht
separat als Autoren-Holdout, nicht als historischer Patchbacktest. Fehlende
Vergleichsquellen und Vorher-/Nachherstände werden nicht ersetzt oder erfunden.

Erst unabhängige fachliche Abnahme, Bug-/Security-Gate, Merge/Push, Release,
Live-Prüfung sowie vollständige Branch-/Worktree-Bereinigung schließen ab.
