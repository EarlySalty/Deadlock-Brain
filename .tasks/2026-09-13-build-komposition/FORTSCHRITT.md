# Skillfolge, Fortschritt und Kaufbudget

Stand 13.09.2026. Diese Ergänzung ersetzt die frühere Maximalpreis-Summe je
Layoutplatz und die durchgehende Bewertung aller Fähigkeiten auf Basisstufe.

`progression::at_souls` erhält dieselbe Skillfolge, die im Build und im
Publish-Payload ausgegeben wird. Der Snapshot liefert `level_info.required_gold`
und `bonus_currencies`. Bei Warden stehen dort Freischaltungen bei 0, 500,
1400 und 3200 Seelen; AP beispielsweise bei 200, 900, 2000 und 2600.
Das sind Daten des eingefrorenen Snapshots, keine erfundene AP-Kurve.

Die Werte 1 für `EAbilityPoints` und 2 für `EAbilityUnlocks` entsprechen dem
[dokumentierten Spiel-Enum](https://docs.deadworks.net/api-reference/players/#ecurrencytype).
Der benötigte Betrag steht als negative Buchung in der gespeicherten Skillfolge.
Die Reihenfolge wird nicht umsortiert: fehlt eine Währung, wartet der nächste
Schritt. Fehlende IDs, Upgrade-Properties oder unbekannte Buchungen werden
sichtbar. Ohne belastbare Folge werden keine Ränge erfunden.

Ein realer Warden-Rohdatensatz enthält 47 Skillbuchungen: zunächst eine
kohärente 16er-Folge, dann eine fremde Fähigkeits-ID und wiederholte Folgen.
Der gemeinsame Verbraucher endet generisch vor der ersten unbekannten ID,
wiederholten Freischaltung oder ungültigen Buchung. Derselbe kohärente Präfix
geht in Berechnung und Publish; der verworfene Rest wird als Quellenmangel
mit Anzahl genannt und bleibt in den Rohdaten erhalten. Autoren werden dafür
nicht anders gewichtet.

Numerische Upgrades werden aus `property_upgrades` angewandt, getrennt nach
Addition/Multiplikation des Basiswerts oder der Spirit-Skalierung. Nicht-Spirit-
Skalierungen und unbekannte Upgradearten bleiben ausdrücklich unquantifiziert.
Die anschließend verwendeten Schadens-/Cooldown-/Dauerwerte entstehen durch
denselben Ableiter wie beim Snapshot-Import. Echte Standard-Boons erhöhen
Waffengrundschaden, Basisleben und Spirit, ausschließlich auf den durch
`use_standard_upgrade` markierten Stufen. Nicht gerechnete Boon-Stats werden
aufgeführt.

`EconomyPolicy` enthält globale, von Kandidaten und Referenzen unabhängige
Seelen-Checkpoints bis 80000. Die Checkpoints sind ein konfigurierbares
Vergleichsszenario, keine gemessenen Minuten oder Einkommen. Die Standard-
Policy setzt verfügbare Wirtschafts- und Fortschrittsseelen gleich; Anfangsgeld,
Zeitverluste und zusätzliche AP aus Ereignissen werden nicht angenommen.
Nettospend bleibt separat: Verkäufe und Komponentenverbrauch senken niemals
verdiente Seelen, Hero-Level oder bereits erreichte Skillränge.

Offene Layoutbänder konkurrieren um das tatsächlich vorhandene Budget.
Der Planner prüft Kauf plus Folgeentscheidung gegen Sparen bis zum nächsten
Checkpoint. Der aktuelle Mehrwert erhält halbes, der Folgezustand volles
Gewicht. Diese feste, heldenunabhängige Planungspräferenz ist sichtbar; eine
begrenzte Zweischrittsuche ist kein globales Optimum. Der Kauf kann an einem
Checkpoint ausbleiben, mehrere bereits bezahlbare Käufe können stattfinden.
Die historischen Kostenband-Phasen dienen weiter der Einordnung; maßgeblich
für den Erwerbszeitpunkt sind die ausgegebenen verdienten Seelen.

Jeder Erwerbsbeleg enthält verdiente Seelen, Nettospend, Restgeld, Level,
angewandte Skillbuchungen und Fähigkeitsränge. Feste Imbue-Bindungen sind Teil
des Suchzustands und des Caches. Eine später freigeschaltete Fähigkeit darf
kein bereits gekauftes Imbue-Item heimlich umhängen. Publish übernimmt genau
die beim Kauf ausgewiesene Bindung.

Der Cache unterscheidet erreichten Fortschritt, gehaltene Item-IDs und feste
Bindungen. Für denselben bereits ausentwickelten Zustand kann er Ergebnisse
über spätere Seelen-Checkpoints wiederverwenden, ohne neue Ränge zu übersehen.

Diese Änderung benötigt neue Vorher-/Nachher-Messung. Ein früherer Warden-
Trefferstand wird nicht als Beleg für den geänderten Algorithmus weitergeführt.
