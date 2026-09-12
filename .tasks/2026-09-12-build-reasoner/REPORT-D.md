# REPORT-D

Stand: 2026-09-12

Branch: `feat/build-reasoner-d`

## Ergebnis

Paket D integriert den Reasoner in `deadlock-brain`, stellt die drei Befehle
`reason build`, `reason patch-impact` und `reason backtest` bereit, bindet die
Seed-Datei und die Autorenquellen an und führt Publish nur bei ausdrücklichem
`--publish` aus. Der Build-Pfad lädt zuerst alle Daten, wendet die
Patch-Deltas vor dem Scoring an und lässt die KI nur Text und Klassifikationen
ergänzen. Eine Critic-Antwort darf genau eine Recompose-Runde auslösen.

Der Fireworks-Client führt seine synchronen HTTP-Aufrufe in einem Worker aus.
Damit bleibt die bestehende synchrone API erhalten und die asynchrone CLI läuft
ohne Tokio-Blocking-Drop-Panic. Bei einem 404 wird einmalig die Modellfamilie
`deepseek-v4-flash` über `/models` aufgelöst und pro Prozess gemerkt.

## Echte Warden-Läufe

Alle Läufe verwendeten den Central-Pool read-only. Credentials und DSN wurden
nicht ausgegeben und nicht in Dateien geschrieben.

| Lauf | Ergebnis |
| --- | --- |
| `reason build Warden --no-ai --json` | gültiges JSON, `hero_id=25`, 19 Core-Items, 4 Situationsblöcke, Konfidenz `Low` |
| `reason build Warden --json` | gültiges JSON, 19 Core-Items, 4 Situationsblöcke, Critic-Runde mit maximal einer Recompose-Runde |
| `reason patch-impact Warden --json` | 314 Patch-Deltas, 20 verschobene Items, gültiges JSON |
| `reason backtest --hero Warden --json` | 4 Vergleichs-Builds, gültiges JSON |

Der aktuelle Patch-Tag wurde live aus `brain.patch_events` aufgelöst:
`https://steamcommunity.com/games/1422450/announcements/detail/676255623445218602`.

## Backtest-Zahlen

Der Backtest umfasst die drei Autoren-Builds aus
`tierlist.hero_build_sources` sowie `referenz/lightbringer-warden.json`.

| Quelle | Kern-Überdeckung | Reihenfolge-Nähe | Patch-Wechsel |
| --- | ---: | ---: | --- |
| Lightbringer x Situation | 0,157895 | 0,435171 | nicht messbar |
| unbekannt, Quelle 1 | 0,157895 | 0,445693 | nicht messbar |
| unbekannt, Quelle 2 | 0,000000 | 0,410198 | nicht messbar |
| unbekannt, Quelle 3 | 0,105263 | 0,391093 | nicht messbar |
| Gesamt | 0,105263 | 0,420539 | nicht messbar |

Die Seed-Kernmenge enthält 19 Items. Der Vergleich mit dem deterministischen
No-AI-Build hat 3 gemeinsame eindeutige Kern-Items bei 35 Items in der Union:

`Jaccard = 3 / 35 = 0,085714`.

Patch-Wechsel ist `null`, weil im ausgewerteten Bestand kein zweiter
vergleichbarer Patch-Stand je Autoren-Build vorliegt.

## Deterministisches Scoring

Der folgende Checkpoint stammt aus dem echten Warden-Scoring-Test ohne
Meta-Zeilen und vor der Anwendung des Patch-Deltas im Fassade-Lauf. Er macht
die Score-Basis und die Konfidenz reproduzierbar. Alle zehn Werte haben
Konfidenz `Low`.

| Rang | Item | Score | Konfidenz |
| ---: | --- | ---: | --- |
| 1 | Spirit Burn | 104,139994 | Low |
| 2 | Juggernaut | 73,451349 | Low |
| 3 | Express Shot | 72,665889 | Low |
| 4 | Frenzy | 65,451358 | Low |
| 5 | Mystic Conduit | 61,962848 | Low |
| 6 | Frostbite Charm | 58,870817 | Low |
| 7 | Mystic Reverb | 55,038547 | Low |
| 8 | Rebuttal | 54,557760 | Low |
| 9 | Lucky Shot | 52,672100 | Low |
| 10 | Mercurial Magnum | 50,246984 | Low |

Die vier Referenz-Sanity-Checks blieben über ihrer jeweiligen Schwelle:

| Item | Score | Schwelle |
| --- | ---: | ---: |
| Veil Walker | 13,853285 | 13,433479 |
| Mercurial Magnum | 50,246984 | 18,928813 |
| Siphon Bullets | 28,149679 | 18,928813 |
| Quicksilver Reload | 23,825812 | 12,280908 |

## Ursache der niedrigen Seed-Überdeckung

Die niedrige Überdeckung ist ein sichtbares Daten- und Regelresultat, kein
Grund für eine stille Gewichtsänderung. Für den geprüften Patch-Tag gab es
keine passenden `hero_item_stats`-Zeilen, daher war `meta_support` für die
Referenz-Items 0,0. Der Fassade-Lauf arbeitet folglich mit Mechanik-Score,
Patch-Delta und den festen Composer-Regeln. Der Composer begrenzt Core auf 19
Items und klassifiziert Lane-, Counter-, Shield-, Tryhard- und optionale Items
vor dem Core-Cutoff.

| Referenz-Core-Item, nicht im Reasoner-Core | beobachteter Score | drückende Regel oder Ursache |
| --- | ---: | --- |
| Juggernaut | 73,451349 | Patch-Ranking und Core-Cutoff, keine Meta-Stütze |
| Mercurial Magnum | 50,246984 | Zustandsfaktor 0,604962 plus Patch-Ranking |
| Titanic Magazine | 47,155524 | Kaufphase `Lane`, dadurch kein früher Core-Platz |
| Spiritual Overflow | 31,994404 | Patch-Ranking und Core-Cutoff |
| Siphon Bullets | 28,149679 | Zustandsfaktor 0,604962 plus Patch-Ranking |
| Quicksilver Reload | 23,825812 | Kaufphase `Lane` und Zustandsfaktor 0,604962 |
| Monster Rounds | 21,872117 | Patch-Ranking und Core-Cutoff |
| Enduring Speed | 19,602674 | Patch-Ranking und Core-Cutoff |
| Opening Rounds | 15,011489 | Zustandsfaktor 0,350000 plus Patch-Ranking |
| Veil Walker | 13,853285 | Patch-Ranking und Core-Cutoff |
| Fleetfoot | 12,537955 | Zustandsfaktor 0,312500 plus Patch-Ranking |
| Swift Striker | 10,822579 | Kaufphase `Lane` |
| Extra Regen | 5,408750 | Patch-Ranking und Core-Cutoff |
| High-Velocity Rounds | 4,638248 | Kaufphase `Lane` |
| Unstoppable | 3,764953 | Zustandsfaktor 0,091667 plus Patch-Ranking |
| Blood Tribute | 2,085882 | Patch-Ranking und Core-Cutoff |

Die sechs niedrigsten Reasoner-Core-Werte enthalten außerdem `Trophy
Collector=-2,187810` und `Golden Goose Egg=-3,480256`. Das bestätigt, dass die
aktuelle Patch-Anwendung und die vorhandene Datenbasis vor einer fachlichen
Gewichtsänderung geprüft werden müssen.

## Verifikation

| Prüfung | Ergebnis |
| --- | --- |
| `cargo test --workspace` ohne DSN | bestanden |
| fokussierte Tests | dbrain-builds 7 bestanden, dbrain-reasoner 68 bestanden, deadlock-brain 38 bestanden, deadlock-brain-core 14 bestanden |
| `cargo clippy -p dbrain-reasoner -p dbrain-builds -p deadlock-brain-core -p deadlock-brain --all-targets -- -D warnings` | bestanden |
| Central-Test `loads_warden_and_reference_items_from_real_snapshot` | 1 bestanden |
| Central-Test `scores_warden_reference_items_from_real_snapshot` | 1 bestanden |
| `cargo test -p dbrain-reasoner -- --include-ignored` mit nur Central-DSN | 70 bestanden, 5 vorhandene Scratch-Tests wegen fehlendem `REASONER_SCRATCH_DSN` fehlgeschlagen; die beiden Central-Tests wurden separat erfolgreich ausgeführt |
| CLI-Hilfe | alle drei Reasoner-Unterbefehle sichtbar |

Publish wurde nicht gegen den Steam-Bot ausgelöst, weil das eine externe
Schreiboperation wäre. Ohne `--publish` schreibt die Build-CLI nicht. Die
Migration und der tägliche Infisical-Timer sind in der Abschlussnotiz als
Deploy-Voraussetzungen aufgeführt.

## Offene Punkte

1. Für eine belastbare Meta-Komponente müssen `hero_item_stats` für den
   aktuellen echten Patch-Tag synchronisiert werden.
2. Für Patch-Wechsel und Jaccard über Zeit werden mindestens zwei
   vergleichbare Autorenstände je Held benötigt.
3. Der vorhandene Scratch-Testbestand braucht ein separates
   `REASONER_SCRATCH_DSN`, damit der vollständige Include-Ignored-Lauf grün ist.

## Fixrunde 1

Die fünf Review-Mängel sind mit `e0e1cffde98cd26713a79e80059af914e7da58a8`
behoben. Vollständige Änderungen je Datei und Zeile, Rot-Gegenproben und
Testzahlen stehen in [FIXRUNDE-1-D.md](FIXRUNDE-1-D.md) sowie im Anhang
„Fixrunde 1“ von REVIEW-D.md im Hauptordner.

Die Fassade persistiert jetzt auch ohne `--publish` in alle vier
`reasoner_*`-Tabellen. Die obigen Aussagen über eine ausschließlich lesende
Fassade beschreiben den vorherigen Stand. Ohne DSN: 256 Tests bestanden,
53 ignoriert. Reasoner mit lesendem Central-Zugang und lokaler Scratch-DB:
83 Tests bestanden, keine Fehler. Die zuvor fehlende Scratch-Testumgebung
wurde für diese Fixrunde isoliert bereitgestellt.

## Live-Nachtrag nach Paket E (2026-09-13 00:40, Release 80417b9)

Release-Binary 00:39 (Anker "Historie, bereits im Snapshot enthalten" im
Binary), Läufe mit Persistenz gegen den Central-Pool, alle Exit 0:

| Lauf | Ergebnis |
| --- | --- |
| `reason build Warden --no-ai --json` | Kern 19 Items, alle positiv; Can buy 1 = 6, Tryhard 1, Shields 3, Optional 12 |
| `reason patch-impact Warden --json` | 3260 deduplizierte Belege, 0 angewendet, 0 Items verschoben |
| `reason backtest --hero Warden --json` | Seed Lightbringer x Situation: Kern-Überdeckung 0,2105, Reihenfolge-Nähe 0,1503; Aggregat 43 Vergleiche 0,1763 / 0,2684 |
| `brain.reasoner_item_scores` (hero_id 25) | 173 Zeilen überschrieben, Spirit Burn 105,30 vorn, Minimum minus 3 (vorher minus 4378) |

Vergleich zum Lauf vor E (Abschnitt Backtest-Zahlen): Kern-Überdeckung 0,158
auf 0,2105, Aggregat 0,10 auf 0,1763. Kern je Tier jetzt 1/1/2/10/5 (Tier 1
bis 5) gegen 3/6/2/8 im Referenzbuild; das ist Befund 14 und Paket F.

LIVEBEWEIS[DV-1]: PID kein Dienst (Timer-Binary) | exe rust/target/release/deadlock-brain 00:39 | journal -p err leer (kein Dienst gelaufen) | Anker "Historie, bereits im Snapshot enthalten" in Binary | Funktion: reasoner_item_scores Warden ohne Werte unter minus 3, patch-impact 0 angewendet | Ort: `deadlock-brain reason build Warden --no-ai --json`, Tabelle brain.reasoner_item_scores
