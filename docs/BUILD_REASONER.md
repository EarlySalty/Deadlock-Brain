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
deadlock-brain reason build Warden --no-ai --no-persist --json
deadlock-brain reason patch-impact Warden --no-persist --json
deadlock-brain reason backtest --hero Warden --no-persist --json
```

Der Seed-Pfad ist über `--seed-path` oder `DEADLOCK_REASONER_SEED_PATH`
konfigurierbar. Standard ist `.tasks/2026-09-12-build-reasoner/referenz/`.
Standardmäßig schreibt der Reasoner Build-, Score-, Delta- und Backtest-Zeilen
in `brain.reasoner_*`; nur der Steam-Publish hängt an `--publish`.

## Was der Reasoner schreibt

| Befehl | Tabellen |
| --- | --- |
| `reason build` | `brain.reasoner_builds`, `brain.reasoner_item_scores` |
| `reason patch-impact` | `brain.reasoner_patch_deltas` |
| `reason backtest` | `brain.reasoner_backtests` sowie `brain.reasoner_builds` und `brain.reasoner_item_scores` für die verglichenen Builds |

Ohne `--no-persist` speichert die Fassade immer, auch bei `--no-ai` oder
`--json`. Eine Read-only-Verbindung liefert dabei einen DB-Fehler statt eines
Builds oder Reports. Alle drei Befehle unterstützen `--no-persist` für
Trockenläufe und Read-only-Verbindungen. Der Schalter unterbindet auch die
Persistenz der intern erzeugten Backtest-Builds. Daten werden weiterhin gelesen;
KI-Aufrufe bleiben von der jeweiligen KI-Einstellung abhängig.

Nur `reason build --publish` erreicht den Steam-Bot über dessen Task-Queue.
`--no-persist` und `--publish` schließen sich aus.

## Deploy-Hinweis

Vor dem regulären Einsatz muss die Migration
`scripts/migrations/2026-09-12-reasoner.sql` angewendet sein. Der Central-Zugang
benötigt für die vier Reasoner-Tabellen Lese- und Schreibrechte sowie die nötigen
Sequenzrechte. Für Zugänge ohne Schreibrechte ist `--no-persist` erforderlich.

Es gibt keinen periodischen Build-Data-Timer mehr; `deadlock-brain-build-data` ist mit P8 (2026-10-28) entfernt. Reasoner-Läufe werden manuell über die `deadlock-brain reason`-Befehle gestartet.

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
