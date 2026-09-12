# FERTIG-F

Paket F ist umgesetzt und geprüft. Stand: 13.09.2026, Europe/Berlin. Ein Thread, keine Unter-Threads.

Worktree: `/home/nathanael/.worktrees/deadlock-brain-f`
Branch: `feat/build-reasoner-f`
Ausgangscommit: `80417b9`
Persistenz: keine neue Migration. Die Layouts werden beim Reasoner-Lauf read-only aus `tierlist.hero_build_sources` und `brain.item_catalog` abgeleitet. Dadurch folgen sie der aktuellen Quellenlage, ohne eine veraltete Layout-Tabelle zu pflegen. Die Abfrage wählt je `hero_build_id` die höchste Version, bei Gleichstand den neuesten Fetch.

## Umsetzung

- `types.rs:367`: Neue Typen für Band-Median, Q1/Q3, Zielgröße, Flex-Budget und Gesamtindex pro Held beziehungsweise global.
- `meta.rs:103`, `meta.rs:148`, `meta.rs:186`: Erkennung von Core-/Kern-/Standard-/Main-/Primary-/Basis-/Default-Kategorien, Fallback auf die größte Kategorie, Quantile und Layout-Ableitung über alle aktuellen Autoren-Builds.
- `data.rs:1069`: Read-only-Loader für alle aktuellen Autoren-Builds und den vollständigen Tier-Katalog.
- `lib.rs:461`: Layout wird vor dem Scoring geladen und in `MetaIndexWithSources` an den Composer gereicht.
- `composer.rs:135`, `composer.rs:266`, `composer.rs:321`: Bandweise Auswahl, Tier 1/2 nach `per_soul_value`, Tier 3 bis 5 nach `total`, Slot-Caps 4/4/4 mit Layout-Flex, danach Sortierung Lane, Mid, Core, Late und Optional-Cap 12.
- `mechanics.rs:210`, `mechanics.rs:237`: Die Kaufphasen bilden jetzt Lane, Mid, Core und Late ab, damit die geforderte Layout-Reihenfolge ausdrückbar ist. Scoring-Gewichte bleiben unverändert.
- Tests in `meta.rs` und `composer.rs` decken Kategorie-Fallback, Quantile/Flex, bandenspezifische Auswahl, Phasenreihenfolge und Slot-Caps ab.

## Layout aus Autoren-Builds

Der Live-Lauf fand 1590 aktuelle Zeilen vor. 1587 Builds hatten mindestens ein im Tier-Katalog erkanntes Core-Item, verteilt auf 38 Helden. Drei Zeilen ohne erkanntes Item wurden nicht in die Statistik aufgenommen.

Quantildefinition: inklusive lineare Interpolation, Q1 bei 0,25, Median bei 0,50, Q3 bei 0,75. Zielwerte sind gerundete Band-Mediane. Das globale Layout ist:

| Band | Median | Q1 bis Q3 | Ziel |
| --- | ---: | ---: | ---: |
| T1 | 1 | 0 bis 3 | 1 |
| T2 | 2 | 1 bis 4 | 2 |
| T3 | 3 | 1 bis 4 | 3 |
| T4 | 3 | 1 bis 6 | 3 |
| T5 | 0 | 0 bis 0 | 0 |
| Gesamt | 10 | 8 bis 13 | 9 |

Warden hat eigene Quellen und verwendet deshalb nicht das globale Fallback:

| Band | Median | Q1 bis Q3 | Ziel |
| --- | ---: | ---: | ---: |
| T1 | 0 | 0 bis 1,75 | 0 |
| T2 | 1 | 0 bis 4,75 | 1 |
| T3 | 2 | 1 bis 3 | 2 |
| T4 | 5 | 2,25 bis 7 | 5 |
| T5 | 0 | 0 bis 0 | 0 |
| Gesamt | 10,5 | 7 bis 13 | 8 |

Warden nutzt 42 Quellen-Builds, das Layout-Ziel ist `T2:1, T3:2, T4:5`, insgesamt 8 Kern-Items. Die per-Held-Ziele aus den 38 aktuellen Autoren-Pools:

| Held | Builds | T1 | T2 | T3 | T4 | T5 | Gesamt |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Infernus | 74 | 1 | 2 | 2 | 4 | 0 | 9 |
| Seven | 43 | 0 | 2 | 2 | 4 | 0 | 8 |
| Vindicta | 41 | 1 | 3 | 3 | 2 | 0 | 9 |
| Lady Geist | 41 | 0 | 3 | 3 | 5 | 0 | 11 |
| Abrams | 41 | 2 | 3 | 4 | 3 | 0 | 12 |
| Wraith | 40 | 0 | 2 | 2 | 5 | 0 | 9 |
| McGinnis | 42 | 1 | 3 | 2 | 3 | 0 | 9 |
| Paradox | 41 | 1 | 3 | 4 | 2 | 0 | 10 |
| Dynamo | 42 | 1 | 2 | 3 | 2 | 0 | 8 |
| Kelvin | 41 | 1 | 3 | 2 | 2 | 0 | 8 |
| Haze | 40 | 1 | 2 | 2 | 4 | 0 | 9 |
| Holliday | 40 | 0 | 2 | 3 | 3 | 0 | 8 |
| Bebop | 40 | 0 | 2 | 3 | 3 | 0 | 8 |
| Calico | 42 | 0 | 2 | 2 | 4 | 0 | 8 |
| Grey Talon | 41 | 0 | 2 | 3 | 3 | 0 | 8 |
| Mo & Krill | 40 | 0 | 2 | 4 | 4 | 0 | 10 |
| Shiv | 41 | 1 | 3 | 2 | 1 | 0 | 7 |
| Ivy | 42 | 1 | 2 | 3 | 3 | 0 | 9 |
| Warden | 42 | 0 | 1 | 2 | 5 | 0 | 8 |
| Yamato | 41 | 1 | 3 | 3 | 4 | 0 | 11 |
| Lash | 40 | 1 | 2 | 3 | 3 | 0 | 9 |
| Viscous | 39 | 1 | 2 | 3 | 3 | 0 | 9 |
| Pocket | 41 | 1 | 2 | 3 | 2 | 0 | 8 |
| Mirage | 42 | 1 | 4 | 2 | 3 | 0 | 10 |
| Vyper | 39 | 0 | 1 | 3 | 4 | 0 | 8 |
| Sinclair | 40 | 0 | 2 | 3 | 3 | 0 | 8 |
| Mina | 41 | 0 | 2 | 3 | 4 | 0 | 9 |
| Drifter | 41 | 0 | 3 | 3 | 5 | 0 | 11 |
| Venator | 41 | 1 | 2 | 5 | 4 | 0 | 12 |
| Victor | 41 | 0 | 2 | 2 | 5 | 0 | 9 |
| Paige | 41 | 1 | 3 | 3 | 2 | 0 | 9 |
| The Doorman | 39 | 1 | 2 | 3 | 2 | 0 | 8 |
| Billy | 42 | 0 | 3 | 4 | 2 | 0 | 9 |
| Graves | 42 | 2 | 3 | 3 | 3 | 0 | 11 |
| Apollo | 41 | 0 | 3 | 3 | 4 | 0 | 10 |
| Rem | 41 | 1 | 2 | 3 | 2 | 0 | 8 |
| Silver | 41 | 0 | 2 | 3 | 1 | 0 | 6 |
| Celeste | 40 | 1 | 3 | 2 | 5 | 0 | 11 |

## Warden Build und Rangbeleg

Der Debug-Lauf `reason build Warden --no-ai --no-persist --json` lief gegen Central mit `default_transaction_read_only=on`. Ergebnis: acht Kern-Items, genau die Warden-Zielverteilung, plus `Can buy 1:6`, `Tryhard:1`, `Shields:3`, `Optional:12`.

| Reihenfolge | Item | Tier | Phase |
| ---: | --- | ---: | --- |
| 1 | Express Shot | 3 | Lane |
| 2 | Titanic Magazine | 2 | Lane |
| 3 | Spirit Burn | 4 | Mid |
| 4 | Mystic Reverb | 4 | Mid |
| 5 | Mercurial Magnum | 4 | Mid |
| 6 | Juggernaut | 4 | Late |
| 7 | Frenzy | 4 | Late |
| 8 | Escalating Resilience | 3 | Late |

Vergleich zum Paket-E-Warden-Lauf mit globalem Cutoff 19:

| Tier | Paket E, 19 Kern | Paket F, Layout-Kern |
| --- | ---: | ---: |
| T1 | 1 | 0 |
| T2 | 1 | 1 |
| T3 | 2 | 2 |
| T4 | 10 | 5 |
| T5 | 5 | 0 |
| Gesamt | 19 | 8 |

Rang und Band-Rang beziehen sich auf den aktuellen Warden-Score-Lauf. Der globale Rang folgt `total`. Der Band-Rang folgt für T1/T2 `per_soul_value`, ab T3 `total`:

| Seed-Kern-Item | Tier | Globalrang | Bandrang | Phase | Im F-Kern |
| --- | ---: | ---: | ---: | --- | --- |
| High-Velocity Rounds | 1 | 139 | 11 | Lane | nein |
| Opening Rounds | 2 | 65 | 19 | Late | nein |
| Extra Regen | 1 | 133 | 9 | Late | nein |
| Quicksilver Reload | 2 | 37 | 6 | Lane | nein |
| Monster Rounds | 1 | 44 | 2 | Late | nein |
| Swift Striker | 2 | 97 | 26 | Lane | nein |
| Titanic Magazine | 2 | 14 | 1 | Lane | ja |
| Veil Walker | 3 | 75 | 19 | Late | nein |
| Enduring Speed | 2 | 53 | 9 | Late | nein |
| Fleetfoot | 2 | 89 | 21 | Late | nein |
| Mercurial Magnum | 4 | 10 | 6 | Mid | ja |
| Spiritual Overflow | 4 | 24 | 15 | Late | nein |
| Siphon Bullets | 4 | 29 | 17 | Late | nein |
| Boundless Spirit | 4 | 19 | 10 | Late | nein |
| Blood Tribute | 3 | 155 | 41 | Late | nein |
| Unstoppable | 4 | 145 | 41 | Late | nein |
| Witchmail | 4 | 49 | 21 | Late | nein |
| Transcendent Cooldown | 4 | 38 | 18 | Late | nein |
| Juggernaut | 4 | 2 | 2 | Late | ja |

## Punkt 4, bestehende Mechanikentscheidungen

Der bestehende Zustandsfaktor von ungefähr 0,6 bleibt unverändert. Im aktuellen Warden-Nachweis liegt der Faktor für die bereits geprüften state-bound Items weiterhin bei `0,604962`, unter anderem Mercurial Magnum und Siphon Bullets. Paket F ändert keine Gewichte und keine Zustandsannahme. Die Layout-Auswahl ändert nur, wie viele Items je Tier in den Kern dürfen.

Lane wird nicht aus dem Kern ausgeschlossen. Lane-Items werden innerhalb des ausgewählten Kerns zuerst ausgegeben. Dadurch kann Titanic Magazine als T2-Kern-Item erscheinen, obwohl T1/T2 über `per_soul_value` gewählt werden. Die neue Mid-Phase ist erforderlich, damit die geforderte Reihenfolge Lane, Mid, Core, Late aus tatsächlichen Kaufkosten abbildbar ist. Kosten bis 1600 sind Lane, bis 3200 Mid, bis 6400 Core, darüber Late; bei Helden-Skalierung wird die Spanne zwischen Pivot und Late-Schwelle zusätzlich geteilt.

## Backtest über mehrere Helden

Alle Läufe waren `--no-persist` und read-only. Warden hat 42 Central-Builds plus den Seed, Abrams 41 Central-Builds plus den Seedpfad, Dynamo 42 Central-Builds plus den Seedpfad und Kelvin 41 Central-Builds plus den Seedpfad. Der Seedpfad wird vom bestehenden CLI-Vertrag mitgeladen; die angegebene Central-Autorenbasis bleibt der maßgebliche Pool.

| Held | Vergleiche | Kern-Überdeckung | Reihenfolge-Nähe |
| --- | ---: | ---: | ---: |
| Warden | 43 | 0,287791 | 0,248299 |
| Abrams | 42 | 0,119048 | 0,225307 |
| Dynamo | 43 | 0,093023 | 0,266373 |
| Kelvin | 42 | 0,202381 | 0,212079 |

Warden-Vergleiche gegen die geforderten Referenzen:

| Vergleich | Kern-Überdeckung | Jaccard | Reihenfolge-Nähe |
| --- | ---: | ---: | ---: |
| Seed Lightbringer x Situation | 0,375000 | 0,125000 | 0,185013 |
| Build 779996, Version 45 | 0,375000 | 0,090909 | 0,217579 |
| Aggregat über 42 Central-Autoren-Builds | 0,113673 | 0,087088 | 0,248299 |

Die CLI-Kernüberdeckung des Aggregats ist auf den Reasoner-Kern normiert und beträgt `0,287791`; die Tabelle darüber weist zusätzlich die mittlere Autoren-Kernüberdeckung `0,113673` aus. Der Aggregat-Jaccard ist der Mittelwert der 42 Einzel-Jaccards. Als ergänzende Vereinigungsmetrik beträgt der Jaccard zwischen F-Kern und der Union aller 42 Autoren-Kerne `0,037037`.

Seed und Build 779996 teilen jeweils drei von acht F-Kern-Items. Die Seed-Union hat 24 Items, die Union mit dem F-Kern 24 Items, daher `3/24 = 0,125000`. Build 779996 hat 28 Kern-Items, die Union mit dem F-Kern umfasst 33 Items, daher `3/33 = 0,090909`. Gegenüber Paket E steigt die Seed-Jaccard von `0,117647` auf `0,125000`; die E-Baseline hatte 19 Kern-Items und 0,210526 Reasoner-Kernüberdeckung.

Lady Geist, Infernus und mehrere weitere Central-Helden konnten in der vorhandenen Snapshot-Basis nicht valid backgetestet werden, weil der bestehende Loader `Ability 0 gehört nicht zum geladenen Helden` oder ein unvollständiges Waffenprofil meldet. Für die Ergebniswerte wurden daher nur die vier erfolgreichen Läufe verwendet. Es wurde keine Helden-Sonderregel ergänzt.

## Tests und Verifikation

| Prüfung | Ergebnis |
| --- | --- |
| `cargo test -p dbrain-reasoner` | 90 bestanden, 0 Fehler, 16 ignoriert |
| `cargo test --workspace` | Alle ausgeführten Tests bestanden, 0 Fehler; DSN-Tests bleiben ohne Scratch-DSN ignoriert |
| `cargo clippy -p dbrain-reasoner --all-targets -- -D warnings` | grün |
| Formatter-Check der geänderten Reasoner-Dateien | grün |
| `cargo build -p deadlock-brain` | Debug-Build grün, kein Release-Build |
| Central-Snapshot-Test `loads_warden_and_reference_items_from_real_snapshot` | 1 bestanden, 0 Fehler, 251 Items, 42 Autoren-Builds |
| Live `reason build Warden --no-ai --no-persist --json` | Exit 0, JSON gültig |
| Live Backtests Warden, Abrams, Dynamo, Kelvin | Exit 0, JSON gültig, keine Schreibzugriffe |
| Live `fix_e_live_warden_evidence` | grün, Central read-only; aktuelle Scores, Ränge und Jaccards geprüft |

Die lokalen Scratch-Tests konnten nicht ausgeführt werden, weil `REASONER_SCRATCH_DSN` nicht gesetzt ist. Central wurde mit `PGOPTIONS='-c default_transaction_read_only=on'` und zusätzlich über `--no-persist` verwendet.

Nicht zur F-Änderung gestaged sind bereits vorhandene Formatierungsänderungen in `ai_roles.rs` und `patch_tests.rs`; sie wurden im Worktree erhalten.
