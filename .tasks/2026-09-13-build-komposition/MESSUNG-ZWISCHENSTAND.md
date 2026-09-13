# Erste gemeinsame Messung, noch keine fachliche Freigabe

Eingabe ausschließlich `FROZEN-V2.json`, SHA im Messvertrag. Referenz Warden
779996 Version 45, 19 Kernitems einschließlich 9 Waffen. Baseline a57382a,
Nachher Combat dc8c962 und Planner 02e4397, Eval-Quellstand 547111f
(19061c6 ergänzt nur Rekonstruktionsdokumentation).

| Stand | Käufe | Gesamtkern getroffen | Waffen getroffen | Recall | Jaccard |
|---|---:|---:|---:|---:|---:|
| Baseline V2 | 16 | 3/19 | 2/9 | 0,157895 | 0,093750 |
| Erste gemeinsame Inventarplanung | 16 | 5/19 | 5/9 | 0,263158 | 0,166667 |

Die fünf Waffen sind High-Velocity Rounds, Opening Rounds, Titanic Magazine,
Swift Striker und Spiritual Overflow. Die Mehrheit wird erstmals erreicht.
Das ist Autorenübereinstimmung mit bekannter Entwicklungsreferenz und ausdrücklich
kein unabhängiger Algorithmusnachweis. Ein Holdout wurde noch nicht ausgewertet.

Der Export `AFTER-FIRST-WARDEN-V2.json` enthält die vollständige mechanische
Planung. Alle 16 Übergänge entsprechen genau den ausgegebenen Kernkäufen. Am Ende
sind 12 Items gehalten, die Nettokosten betragen 39.600 Seelen. Die tatsächlich
errechneten Shopboni lauten Spirit 52, Vitality 42 und Weapon 100.

Offene fachliche Punkte im selben Nachweis: Glass Cannon wird gewählt, während
`MaxHealthLossPercent` noch als nicht quantifiziert erscheint. Lightning Scroll
hat unbekannten Damage-/Triggernutzen, Opening Rounds eine nicht berechnete
EnemyLifeThreshold-Bedingung. Diese Lücken sind für die Abnahme relevant und
werden durch fünf Referenztreffer nicht widerlegt. Die Slot-/Erstattungsvorgaben
und die nicht simulierten Freischaltzeitpunkte bleiben im Plan als Annahmen
sichtbar. Gegenwärtige Bewertungszahlen sind Modellwerte, keine gemessenen
Schadenswerte aus Spielaufzeichnungen.

Der Debug-Lauf dauerte 87,7 Sekunden einschließlich Laden der 468-MB-Datei und
zweimaligem Planen (Build und überprüfter Planexport). Vor Mehrheldenmessungen
wird dasselbe Mess-Example optimiert gebaut und auf identische Warden-Ergebnisse
geprüft. Es wird ausschließlich das Example gebaut; kein Produktivbinary wird
getauscht und kein Dienst gestartet.

## Optimierter Lauf und sechs Helden

Der Releasebau des reinen Examples dauerte 29,43 Sekunden. Binary-SHA:
`d8bfc4f718c1316aff63eb89c8d651b11bff3c58ab8974c9a3be9c66fd81b223`.
Der Warden-Planlauf dauerte 12,48 Sekunden; seine komplette JSON-Ausgabe ist
bytegenau identisch zum Debug-Lauf (`cmp`, Exit 0). Der Release-Slot wurde danach
freigegeben. Alle folgenden sechs Helden wurden auf demselben Quellstand und
`FROZEN-V2.json` gerechnet, ohne Holdout und ohne zusätzliche Daten.

| Held | Referenzen | Recall vorher | Recall nachher | Planübergänge | Gehaltene Items |
|---|---:|---:|---:|---:|---:|
| Infernus | 3 | 0,200000 | 0,196296 | 18 | 12 |
| Lady Geist | 1 | 0,000000 | 0,055556 | 18 | 12 |
| Haze | 3 | 0,134680 | 0,329966 | 18 | 12 |
| Bebop | 2 | 0,091667 | 0,066667 | 16 | 12 |
| Ivy | 2 | 0,068182 | 0,181818 | 17 | 12 |
| Warden | 2 | 0,120614 | 0,256579 | 16 | 12 |

Warden in dieser Tabelle ist der Mittelwert beider beobachteten Referenzen,
nicht nur 779996. Vier Helden verbessern ihre Autorenübereinstimmung, zwei
verschlechtern sie. Alle Kaufkurven stimmen vollständig mit ihren Planübergängen
überein. Der gesamte Lauf einschließlich sechs Planexporten dauerte 89,48 Sekunden.

Der Nachweis zeigt jedoch weitere fachliche Lücken: Lady Geist hat im Endinventar
exakt null berechneten Fähigkeitsschaden bei rund 13.645 Waffenschaden. Infernus
hat nur rund 338 Fähigkeits- gegenüber 12.630 Waffenschaden. Alle sechs
Endinventare weisen null Proc-Schaden aus. Ob einzelne Nullen inhaltlich erlaubt
sind, muss gegen die tatsächlich gewählten Items und Heldenmechaniken geprüft
werden; sie sind kein Beweis funktionierender Proc-Ketten. Je Endinventar stehen
26 bis 38 Einträge unter unbekannten Wirkungen. Diese Befunde gingen an den
Kampfmodell-Autor und den Orchestrator, bevor weitere Messungen beauftragt wurden.

Belege im Documents-Nachweisordner: `AFTER-FIRST-SIX-HEROES.json`,
`FIRST-SIX-COMPARISON.json`, `FIRST-SIX-MECHANICS.json`.

## Selbstgate

Der reguläre Selbstgate gegen den gemeinsamen Zwischenstand wurde aufgerufen.
Der erste Versuch scheiterte am wöchentlichen Claude-Kontingent. Der vorgesehene
Fallback mit `--model gpt-6-astra` endete zwar Exit 0/ALLOW, bezeichnete die
Repositoryinspektion aber ausdrücklich als nicht durchgeführt, weil der
Toolhost deaktiviert war. Das ist keine vollständige Mergefreigabe. Der
abschließende gemeinsame Gate mit funktionierenden Lesewerkzeugen und die
unabhängige fachliche Abnahme bleiben erforderlich.
