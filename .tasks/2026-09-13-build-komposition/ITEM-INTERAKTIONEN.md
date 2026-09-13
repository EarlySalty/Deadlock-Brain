# Begrenzte Iteminteraktionen

`item_interactions.rs` enthält vorbereitete numerische Regeln für zwei durch
Propertykombinationen erkannte Wirkungen. Keine Item-ID oder Heldengewichtung
steuert die Berechnung. Die Rohgegenproben stammen unverändert aus
`RAW-SNAPSHOTS-V2.json`, Snapshots 13111 und 13178 vom 30.06.2026; enthalten
sind Herkunft, ursprünglicher Payload und dessen Hash.

Mercurials `BulletsBonusMagicDamage` ist laut Prozentpostfix und Beschriftung
ein Anteil am Basis-Kugelschaden. Für 20 Grundschaden, 100 Spirit und zwei
getroffene Kugeln ergibt `(25 + 0,49 × 100) / 100 × 20 × 2 = 29,6`
zusätzlichen Spiritschaden. Der Helper verlangt einen belegten Spirit-Typ
und einen vom Kampfzustand gesetzten aktiven Magazinbuff. Ohne Buff gibt es
keinen Zusatzschaden. Bewusste Annahme ist die Auswertung des aktuellen
Spiritwertes beim Treffer; Grundschaden umfasst die gespeicherten Levelboni,
aber keine erneute Anwendung von Waffenschadenboni.

Hunters Aura besitzt im Rohsnapshot Radius `15m`, Kugelresistenzsenkung 10,
Feuerratensenkung 15 und Alleinzielmultiplikator 2. Genau ein naher Held ergibt
20 und 30; mehrere nahe Helden 10 und 15; außerhalb der Aura oder ohne nahen
Helden null. Andere Itemstats und fremde Effekte werden nicht verdoppelt.
Nähe und Anzahl sind ausdrückliche Szenarioeingaben. Fehlt die Reichweite,
meldet das Modul eine Unknown statt einen Radius zu erfinden. Der Itemmodell-
Normalisierer ersetzt die falsche, allein vom Wort „bullet“ abgeleitete
Schussbedingung durch eine vom Aura-Verbraucher zu prüfende Nähebedingung.

Das Modul selbst entscheidet keine Cast-/Nachladezeitpunkte. Der Combat-
Verbraucher muss den Magazinbuff bei der fest gebundenen Imbue-Auslösung
aktivieren, beim tatsächlichen Nachladen beenden und beim bloßen Zielwechsel
erhalten. Er muss den typisierten Zusatzschaden genau einmal je getroffener
Kugel durch die Schaden-/Spirit-Auslöser-/Heilungskette führen. Aura-Felder
müssen aus dem bisherigen generischen Statpfad herausgenommen werden, damit
der echte Aura-Verbraucher sie nicht doppelt anwendet.

Die Modulgegenproben ersetzen diesen Anschluss nicht. Fachlich vollständig
ist das Paket erst mit dessen Ereignisgegenproben und erneuter gemeinsamer
Messung. Die bisherigen Autoren-Trefferzahlen werden nicht fortgeschrieben.
