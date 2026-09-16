# P-WARDEN-POPULATION: Warden-Baseline aus der Population

Erzeugt mit `deadlock-brain population stats --hero Warden` und
`deadlock-brain population show --hero Warden` gegen die lokale Wegwerf-DB
`population_dev` (Unix-Socket), gefüllt vom 10k-Ingest.

## Laufdaten

- Ingest-Lauf: 10000 Matches angefragt und gesehen, 116198 neue Spieler-Matches,
  2342 schon vorhanden, Laufzeit 478,0 s (rund 8 Minuten, anonym mit 9 Anfragen
  pro Minute gedrosselt).
- Datenbank nach dem Lauf: 119248 Spieler-Matches über 10059 Matches, 38 Helden
  (inklusive zweier idempotenter 200er-Kontrollläufe).
- Warden (hero_id 25), Spieler-Matches je Bucket: all 4017, weapon 1998,
  vitality 1380. Der Bucket spirit bleibt aus, weil unter 25 Prozent der
  Warden-Spieler spirit-lastig kaufen.

## Bucket all (Show)

```
Held Warden (25), Bucket all
Staples ab 70 Prozent:
  Quicksilver Reload          96.8%  Pos  7.0
  Mercurial Magnum            95.4%  Pos 11.0
  Opening Rounds              93.4%  Pos  1.0
  Titanic Magazine            90.9%  Pos  5.0
  High-Velocity Rounds        90.7%  Pos  0.0
  Spiritual Overflow          82.8%  Pos 14.0
  Fleetfoot                   79.6%  Pos 10.0
  Swift Striker               79.2%  Pos  6.0
  Extended Magazine           76.0%  Pos  4.0
  Enduring Speed              75.8%  Pos  9.0
Imbue-Ziele:
  Quicksilver Reload         -> Alchemical Flask
  Mercurial Magnum           -> Alchemical Flask
  Surge of Power             -> Willpower
  Mystic Expansion           -> Binding Word
  Compress Cooldown          -> Alchemical Flask
  Duration Extender          -> Last Stand
  Ballistic Enchantment      -> Alchemical Flask
  Echo Shard                 -> Binding Word
Skill-Reihenfolge (Modus): Alchemical Flask > Alchemical Flask > Willpower > Binding Word > Alchemical Flask > Last Stand > Alchemical Flask > Binding Word > Binding Word > Binding Word > Willpower > Willpower > Willpower > Last Stand > Last Stand
```

Skill-Reihenfolge im all-Bucket: genau gefolgt von 1645 von 3189 Warden-Spielern
mit voller Skill-Historie.

## Bucket weapon (Show)

```
Held Warden (25), Bucket weapon
Staples ab 70 Prozent:
  Quicksilver Reload          97.2%  Pos  7.0
  Mercurial Magnum            95.5%  Pos 11.0
  Opening Rounds              94.8%  Pos  1.0
  High-Velocity Rounds        93.3%  Pos  0.0
  Titanic Magazine            93.1%  Pos  5.0
  Spiritual Overflow          90.0%  Pos 14.0
  Fleetfoot                   82.5%  Pos 10.0
  Swift Striker               81.9%  Pos  6.0
  Extended Magazine           80.1%  Pos  4.0
  Enduring Speed              75.4%  Pos  9.0
```

## Lesart

- 10 Staples ab 70 Prozent im all-Bucket. Die Kernwaffenkette (Quicksilver
  Reload, Mercurial Magnum, Opening Rounds, Titanic Magazine, High-Velocity
  Rounds) sitzt über 90 Prozent, dazu Bewegungs- und Ausdaueritems.
- Verkaufsrate trennt Komponenten von Halte-Items sauber: High-Velocity Rounds
  und Extended Magazine werden zu 99 Prozent später aufgewertet und verkauft,
  Mercurial Magnum und Spiritual Overflow bleiben bis zum Ende liegen.
- Imbue ist bei Warden eindeutig: Quicksilver Reload und Mercurial Magnum zeigen
  fast zu 100 Prozent auf Alchemical Flask. Dünne Zellen sind als solche
  markiert.
- Die drei Neigungs-Buckets weichen nur leicht voneinander ab; Warden ist
  waffen- und vitalitätslastig, spirit fällt unter die 25-Prozent-Schwelle.

Dies ist die Baseline, gegen die Paket M das erzeugte Warden-Build misst
(Staple-Gate, Kendall tau gegen die Median-Position, Jaccard@12).
