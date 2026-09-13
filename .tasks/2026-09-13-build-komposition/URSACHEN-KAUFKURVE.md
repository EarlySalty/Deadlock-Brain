# Diagnose der Kaufkurven nach Gesundheits- und Dauerkorrektur

Prüfbasis: `AFTER-HEALTH-DURATION-b6cd59f.json` und
`HEALTH-DURATION-METRICS-b6cd59f.json` unter
`Documents/.tasks/2026-09-13-build-reasoner-ganzbuild/nachweise/`.
Algorithmus: Combat `20ac192`, Planner `6e3159a`; keine neue Messung und keine
Holdoutdaten für diese Diagnose. Die Rohpreise und Propertynamen wurden gegen
die beiden Heldeneinträge und Referenzquellen aus `FROZEN-V2.json` geprüft.

## Tatsächlicher Zustand

Warden erreicht 4/9 Referenzwaffen und 5/19 Referenzitems. Lady Geist erreicht
1/7 Referenzwaffen und 1/18 Referenzitems. Damit bleibt die Pflichtmehrheit
unerfüllt. Beide Ergebnisse sind Autorenübereinstimmung mit Eingangsquellen,
keine unabhängige Prognose.

Die gemeinsamen Endbewertungen sind stark waffenlastig: Warden 32.380,97
Waffenschaden gegenüber 4.375,55 Fähigkeitsschaden; Geist 35.311,44 gegenüber
2.993,96. Das beweist keine optimale Ausrichtung. Es zeigt, dass der Rückgang
der Referenzübereinstimmung nicht durch ein allgemeines Übergewicht von
Spirit-Items in diesen beiden ausgewählten Endinventaren erklärt wird.

## Konkreter Fehler im ökonomischen Suchraum

`planner::Search::choices` prüfte Verkäufe nur, wenn `preview_purchase` ohne
Verkauf scheiterte. Der Inventarprüfer kennt jedoch nur Slot-/Upgrade-/Verkaufs-
regeln, nicht das verfügbare Erwerbsbudget. Ein slotlegaler, aber unbezahlbarer
Kauf wurde deshalb später verworfen, ohne einen finanzierenden Verkauf zu prüfen.

| Echter Erwerbszustand | Ausgelassene legale Alternative |
| --- | --- |
| Warden vor Glass Cannon: 11 gehaltene Items, 21.600 Nettospend; am Checkpoint 25.600 sind 4.000 verfügbar. | Mercurial Magnum kostet 6.400, Verkauf bringt 3.200. Glass Cannon kostet 6.400. Austausch wäre mit 24.800 Nettospend und 11 Slots finanzierbar. Tatsächlich erfolgt der Kauf ohne Verkauf erst bei 28.800, marginaler gemeinsamer Wert dort +114,7367. |
| Geist vor Weighted Shots: 8 gehaltene Items, 12.800 Nettospend; bei 14.400 sind 1.600 verfügbar. | Hollow Point kostet 3.200, Verkauf bringt 1.600. Weighted Shots kostet 3.200. Austausch wäre bei 14.400 Nettospend und 8 Slots finanzierbar. Tatsächlich erfolgt der Kauf ohne Verkauf erst bei 16.000, marginaler gemeinsamer Wert dort +70,9248. |

Die ausgewiesenen Marginalwerte gehören ausschließlich zu den tatsächlich
gewählten Käufen zu deren Erwerbszeitpunkt. Ein positiver Vorteil der zuvor
ausgelassenen Verkaufsalternativen ist damit **nicht** gemessen. Sie müssen
mit Verlust aller verkauften Effekte, dem korrekten Fortschritt und dem
vorhandenen Vergleich gegen Sparen berechnet werden.

Der begrenzte Fix erweitert genau diesen Fall: Ist der direkte Kauf
unbezahlbar, werden legale einzelne Verkäufe ebenfalls durch die vorhandene
atomare Inventarprüfung und dieselbe gemeinsame Bewertung geschickt. Es gibt
keinen automatischen Verkauf, keinen Referenzbonus und keine neue Gewichtung.
Ein gezielter Test bestätigt einen nützlichen Austausch bei freien Slots und
die Ablehnung eines finanzierbaren, aber schlechteren Austauschs.

## Belegte Modelllücken, keine behaupteten Ersatzrankings

Die folgenden in den Rohdaten vorhandenen Wirkungen werden im geprüften
Combatstand nicht als entsprechendes Ereignis modelliert:

- Warden-Referenz: Monster Rounds (`NonPlayerBonusWeaponPower`,
  `NonPlayerBulletResist`) betrifft Nichtspieler; alle drei Szenarien sind
  Heldenkämpfe. Farmnutzen und dadurch geändertes Einkommen fehlen.
- Warden-Referenz: Siphon Bullets besitzt `HealthStealPctHero`,
  `StealDuration`, `StealPerHit` und `ProcCooldown`. Der Item-Stats-Verbraucher
  bildet daraus keinen zeitlich begrenzten Lebensentzug samt verändertem
  Ziel-/Eigenleben. Der gleichnamige Heilwert im Fähigkeitszweig ersetzt das nicht.
- Beide Referenzen enthalten Schutz-/Bewegungsnutzen, der im pauschalen
  Kontaktszenario nicht vollständig abgebildet wird: unter anderem
  `SlowResistancePercent`, Schießen ohne Bewegungseinbuße, Teleport,
  Entzauberung und Parade. Geist hat beispielsweise Dispel Magic
  (`HealOnActivate=250`) und Counterspell (`HealOnSuccess=150`,
  `SpellParryDuration=0,8`) in der Kernreferenz. Diese konkreten Heilereignisse
  werden im geprüften Item-Verbraucher nicht ausgelöst.
- Geists tatsächlicher Endplan nennt selbst `VulnerabilityPerStack` und
  `NumBloodShards` als unbekannt. Damit fehlt eine zentrale mögliche
  Kopplung zwischen vorheriger Fähigkeitsanwendung und späterem Schaden.
  Der Bezug und die Ereignisse müssen aus Daten geklärt werden, nicht aus
  gewünschten Referenzitems rückwärts abgeleitet werden.

Es wird nicht behauptet, dass die Implementierung einer dieser Wirkungen ein
bestimmtes Item auswählen würde. Dafür fehlen in dieser lesenden Diagnose
gegenfaktische Marginalmessungen auf denselben Erwerbsinventaren.

## Zielzustand ist eine getrennte mechanische Ursache

Der geprüfte Stand nutzt kumulierten Schaden geteilt durch Helden-Basisleben
für `EnemyLifeThreshold`, setzt Treffer, Procs und Heilung aber nach Erreichen
von null Ziel-Leben fort. Die ausgegebene Annahme sagt zugleich, dass kein
automatischer Zielwechsel stattfindet. Die oben genannten fünfstelligen
Endschäden beider Helden gehören zu diesem Szenario.

Das benachteiligt an hohes Gegnerleben gebundene Wirkungen strukturell:
Opening Rounds hat im Snapshot `EnemyLifeThreshold=50` und einen bedingten
Waffenbonus von 25. Es bedeutet nicht, dass dessen Nichtauswahl allein dadurch
verursacht wurde. Ein konsistentes Zielmodell muss entweder wirklichen
Zieltod mit Kampfzeit/TTK bewerten oder Folgeziele samt Pause und Reset
zielgebundener Ereignisse ausdrücklich modellieren. Das ist ein getrenntes
Combatpaket und keine Korrektur über Itemgewichte.

## Verbleibende Suchgrenzen

Auch nach dem begrenzten Verkaufsfix werden keine Mehrfachverkäufe und keine
isolierten Verkaufsschritte gesucht. Ein direkt bezahlbarer Kauf veranlasst
keine zusätzliche freiwillige Verkaufsprüfung. Verkaufte oder verbrauchte
Items werden nicht erneut gekauft. Ein Kandidat muss bereits unmittelbar
positiven gemeinsamen Mehrwert haben; ein zunächst nachteiliger Brückenkauf
mit späterem Gewinn bleibt ausgeschlossen. Die vier Kandidaten des
Zweischritthorizonts werden nach unmittelbarem Marginalwert ausgewählt.
Autoren-Kostenbandquoten begrenzen die Anzahl der Käufe; vollständige
alternative Endinventare außerhalb dieser Struktur werden nicht durchsucht.

Diese Grenzen beschreiben den tatsächlich beanspruchten Algorithmus. Keine
davon darf durch nachträgliches Kopieren der fehlenden Referenzitems verdeckt
werden. Neue Plan-/Mechanikmessung und erneute unabhängige Abnahme bleiben nötig.
