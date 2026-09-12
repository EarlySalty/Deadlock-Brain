# Build Reasoner

Der Build Reasoner erzeugt pro Held einen deterministischen Build aus Helden-,
Fähigkeits-, Item-, Patch- und Meta-Daten. Die KI ergänzt ausschließlich
Begründungen, Rollen und Kritik. Zahlen, Item-Auswahl und Patch-Vorzeichen
bleiben im Rust-Kern.

## Datenfluss

`brain.entity_snapshots` und die normalisierten `brain`-Tabellen werden geladen,
zu Hero- und Item-Modellen gebaut und mit Patch-Deltas angepasst. Danach berechnet
der Reasoner Item-Scores, Meta-Nebensignale und Kaufphasen. Der Composer erzeugt
Kern und Situationsblöcke. Autoren-Details und `brain.hero_ability_orders` liefern
die Skill-Order. Bei aktivierter KI folgen die Rollen und höchstens eine
Recompose-Runde. Publish reiht nur mit ausdrücklicher Freigabe einen bestehenden
Steam-Bot-Task ein.

## Befehle

```text
deadlock-brain reason build Warden --no-ai --json
deadlock-brain reason build Warden --patch <tag> --publish
deadlock-brain reason patch-impact Warden --json
deadlock-brain reason backtest --hero Warden --json
```

Der Seed-Pfad ist über `--seed-path` oder `DEADLOCK_REASONER_SEED_PATH`
konfigurierbar. Standard ist `.tasks/2026-09-12-build-reasoner/referenz/`.
Ohne `--publish` wird keine Schreiboperation durch den Reasoner ausgelöst.

## Backtest-Lesart

Die Kern-Überdeckung ist der Anteil der Reasoner-Kernitems im Autoren-Kern.
Jaccard misst die symmetrische Schnittmenge. Reihenfolge-Nähe ist eine
normalisierte Rangdistanz gemeinsamer Items, wobei 0 identisch und 1 maximal
verschoben bedeutet. Fehlt eine ausreichende Vergleichsbasis, wird die
Reihenfolge als `null` beziehungsweise „nicht messbar“ ausgegeben.

Patch-Wechsel ist nur messbar, wenn zwei zeitlich oder per Patch-Tag zugeordnete
Stände vorliegen. Ein fehlender Autoren-Scan wird als fehlende Vergleichsbasis
gemeldet, nicht als künstlicher Wert.

## Grenzen

Reichweite, Lifesteal-Interaktionen, Kanalanteil, Abrissquote und Fensterlänge
sind teilweise Annahmen. Der Reasoner verwendet ein 40-Sekunden-Kampffenster und
55 Prozent Kanal-Uptime, sofern die Konfiguration nicht geändert wird. Item-
Kaufzeitpunkte der Autoren stammen aus Array-Reihenfolge, wenn keine Zeitstempel
je Item vorliegen. Diese Grenzen sind Teil der Interpretation und kein Ersatz für
Spieltelemetrie.
