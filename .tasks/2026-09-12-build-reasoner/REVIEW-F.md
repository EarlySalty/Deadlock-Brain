# REVIEW-F (Runde 1): build-reasoner Paket F

**Urteil: NACHBESSERN**

Kern ist als Einkaufskurve nach Kostenband sauber gebaut, Tests gruen, Clippy
sauber, die E-Regeln (nur positive Scores im Kern, Situationsbloecke) bleiben
erhalten. Blockierend ist nichts. Der zentrale Zweck von F, die Zielgroesse des
Kerns, ist aber falsch kalibriert: die unabhaengige Rundung je Band summiert zu
einem systematisch zu kleinen Kern (Warden live 8 Items gegen Gesamtmedian 10,5
und Referenz 19). Das gehoert vor dem Merge korrigiert.

## Nachweise

- Reasoner-Tests: `cargo test -p dbrain-reasoner` -> 90 passed, 0 failed, 16 ignored.
- Workspace: `cargo test --workspace` -> 278 passed, 0 failed, 58 ignored.
- Clippy: `cargo clippy -p dbrain-reasoner --all-targets -- -D warnings` -> exit 0, 0 Warnungen.
- DB read-only (Central-Pool, DEADLOCK_CENTRAL_DSN): Build 779996 = Warden v45,
  5 modCategories. Genau eine mit "CORE" im Namen ("CORE ITEMS /// CAN BUY ANY
  3K ITEM INSTEAD OF VEIL", 19 mods). TRYHARD 1, CAN BUY 1 OR 2 6, GREED 2,
  namenlos 11.
- Live: `deadlock-brain reason build Warden --no-ai --no-persist --json`
  -> `core` = 8 Items, Tier-Verteilung Tier2:1, Tier3:2, Tier4:5, Tier1/5:0.

TESTNACHWEIS[TW-1]: 278 passed, 58 ignored | Baseline: 273 passed, 58 ignored (E), keine Tests verschwunden, +5 neu

## Maengelliste

### Wichtig

1. **Kern systematisch zu klein (`meta.rs:180` `layout_stats`, Feld `target`).**
   Jedes Bandziel ist `median.round()` einzeln je Tier. Die Summe der gerundeten
   Bandmediane trifft nicht den Median der Kern-Gesamtgroesse: Warden
   0+1+2+5 = 8, waehrend `total_median` 10,5 ist und der Referenzkern (779996)
   19 Items hat. Live reproduziert: Warden-Kern = 8. Unabhaengige Rundung
   nach unten verliert reproduzierbar Items, das ist kein einmaliges Artefakt,
   sondern der Regelfall bei mehreren halbzahligen Bandmedianen.
   *Fix:* Bandziele per Groesste-Reste-Verfahren so verteilen, dass ihre Summe
   `total_median.round()` trifft (Nachkommaanteile absteigend aufrunden, Rest
   abrunden). Siehe Kalibrierungsfrage KF1.

2. **Flex-Budget haengt am unterschaetzten Ziel (`meta.rs:154`
   `flex_slots = total_target.saturating_sub(12)`, genutzt in
   `composer.rs` `select_core_items`).** Weil `total_target` durch Mangel 1 zu
   klein ist, ist `flex_slots` fast immer 0 (Warden: 8 - 12 -> 0). Der Flex ist
   damit an den falschen Zielwert gekoppelt statt an die Slot-Kapazitaet und
   verschaerft die Unterfuellung. Mit Fix 1 gemeinsam loesen: Flex aus der
   Slot-Verteilung der korrigierten Ziele ableiten.

### Nit

3. **rustfmt-Churn bloeht den Diff** (Import-Reihenfolge `data.rs:4`
   `{Row, postgres::PgPool}`, Re-Export-Umsortierung `lib.rs` ai_roles, Kollaps
   von `per_soul_value` und mehreren `assert!`-Bloecken). Stammt aus einer
   anderen Toolchain-Version, nicht funktional. Kann bleiben, macht das Review
   nur lauter.

4. **Bandziel kann bei einseitiger Slot-Verteilung unterfuellt bleiben
   (`composer.rs` `select_core_items`).** Sind viele Kandidaten eines Bands im
   selben Slot und der Flex erschoepft, wird der Rest per `continue`
   uebersprungen und landet spaeter in Optional; das Band bleibt unter Ziel.
   Edge-Case, bei realer Slot-Streuung unkritisch.

## Geprueft und in Ordnung

- **Layout-Erkennung (Punkt 1).** `core_categories` matcht Kategorienamen gegen
  core/kern/standard/main/primary/basis/default, sonst groesste Kategorie. Auf
  779996 trifft genau die CORE-Kategorie mit 19 Items, deckt sich mit dem Seed.
  Die in FERTIG-F genannten 28 Kern-Items reproduzieren aus der aktuellen
  Version (v45) nicht, dort sind es 19; die 28 sind vermutlich eine veraltete
  oder ueber mehrere Builds addierte Zahl, kein Mangel.
- **Mid-Phase ohne Scoring-Nebenwirkung (Punkt 3).** `buy_phase` wird in
  `item.rs:106` nur auf dem ScoredItem gespeichert, `per_soul_value` haengt in
  `item.rs:52` an `combat_value` und `cost`, nicht an der Phase. Die neue
  Mid-Phase steuert weder Zustandsfaktor noch per_soul-Gewichtung. In
  `select_core_items` entscheidet per_soul_value vs total nach Tier (<=2 bzw.
  ab 3), nicht nach Phase. `mechanics.rs` durfte laut Briefing mit Begruendung
  angefasst werden; die Aenderung ist auf die Phasenreihenfolge begrenzt und
  konsistent (Mid erbt Sell-Priority None wie Core/Late).
- **E-Regeln erhalten.** `core_candidates` filtert Situationsitems und
  `score.total <= 0` raus, der Kern bleibt rein positiv. Slot-Caps 4/4/4 plus
  Flex und Reihenfolge Lane, Mid, Core, Late sind umgesetzt.

## Kalibrierungsfragen (getrennt, mit Empfehlung)

- **KF1 Zielgroesse des Kerns.** Groesste-Reste auf `total_median` (Empfehlung)
  gegen Q3 als Bandziel. Groesste-Reste trifft die Absicht "Kern als Kurve ueber
  das Spiel" am fairsten und haelt die Bandproportionen, ohne Gewichte. Q3
  macht Builds groesser und aggressiver, geht aber ueber den Median hinaus und
  waere eine bewusste Stilentscheidung, keine neutrale Zielgroesse. Empfehlung:
  Groesste-Reste auf `total_median.round()`.
- **KF2 Vergleich E gegen F im Backtest.** Kern-Ueberdeckung ist auf den
  Reasoner-Kern normiert, ein kleinerer Kern hebt sie mechanisch (F 8 Items
  0,375, E 19 Items 0,2105), das misst nicht Qualitaet. Fair vergleichbar sind
  nur groessenrobuste Kennzahlen: Treffer je Referenz-Kern-Item (Recall gegen
  die 19er-Referenz) und zusaetzlich Jaccard(Reasoner-Kern, Referenz-Kern).
  Empfehlung: beide ausweisen, Kern-Ueberdeckung als alleinigen Vergleich fallen
  lassen.

## Separates Paket (kein F-Mangel)

- **Loader-Fehler bei anderen Helden.** `reason build "Lady Geist"` und
  `reason build Infernus` (je `--no-ai --no-persist --json`) brechen mit exit 1
  und "Datenfehler: Ability 0 gehoert nicht zum geladenen Helden" ab. Ursache:
  `hero.rs:20` `integer(ability.get("id"))` liefert 0, wenn eine
  Ability-Snapshot-Zeile kein oder ein leeres `id`-Feld hat; `position()` an
  `hero.rs:24` findet keine Ability mit id 0 und wirft den Data-Fehler. Also
  unvollstaendige Snapshot-Daten oder falscher id-Schluessel fuer diese Helden,
  Warden hat vollstaendige ids und laeuft. `hero.rs` ist im F-Diff unberuehrt,
  Fehler ist vorbestehend. Eigenes Paket.

## Fixrunde 1

Umsetzung abgeschlossen am 13.09.2026. Ein Thread, keine Unter-Threads. Branch `feat/build-reasoner-f`, Ausgangscommit `5676a74`. Code und maschinenlesbarer Nachweis: **`41072426c02c29bde0d755896d4855181c59ae99`**, auf den Feature-Branch gepusht. Die Review-Freigabe bleibt dem Orchestrator vorbehalten.

Die erwartete Kerngröße ist erreicht: Warden hat 11 Items. Das zusätzliche Qualitätsziel Recall mindestens auf E-Niveau wird nicht erreicht: weiterhin 3/19 statt E 4/19. Die Ursachen stehen unten je fehlendem Referenz-Item; keine Gewichte wurden verändert.

### Erledigte Mängel und Entscheidungen

Alle folgenden Codeangaben sind relativ zu `rust/crates/dbrain-reasoner/src/` und beziehen sich auf Commit `4107242`.

| Punkt | Datei:Zeile | Änderung |
| --- | --- | --- |
| Wichtig 1, KF1 | `meta.rs:172`, Test `meta.rs:670` | Größte Reste auf das gerundete Gesamtziel. Bandmediane werden zuerst proportional auf dieses Ziel normiert. Warden: 0/1/2/5/0, Gesamtmedian 10,5, Quoten 0/1,375/2,75/6,875/0, Ziele 0/1/3/7/0. |
| Wichtig 2 | `meta.rs:17`, `lib.rs:462`, `composer.rs:182` | Slot-Kapazität aus dem neuesten Assets-Helden-Snapshot. Aktuell `[6,6,6]` pro Kategorie ergibt sechs gemeinsame Flex-Plätze über 4/4/4. Konstante `FALLBACK_BASE_SLOT_CAPACITY` ersetzt die nackte 12; Fallback nutzt das korrigierte Gesamtziel. |
| Nit 3 | vorhandener Formatierungsdiff | Bestehender rustfmt-Churn bleibt wie entschieden. Die im Briefing ausdrücklich ausgeschlossenen uncommitteten Änderungen in `ai_roles.rs` und `patch_tests.rs` wurden verworfen und nicht mitcommittet. |
| Nit 4 | `composer.rs:896` | Regressionstest: sieben T2-Waffenkandidaten, ein Flex-Platz und ein freier T3-Spiritkandidat. Der Kern bleibt bei fünf T2-Items; zwei verdrängte T2-Items und der T3-Kandidat stehen in Optional. Kein Auffüllen aus einem anderen Band. |
| KF2 | `types.rs:430`, `backtest.rs:103`, `backtest.rs:167`, `backtest.rs:24` | `reference_recall` und `core_jaccard` je Vergleich und als arithmetisches Mittel im Aggregat, Ausgabe als JSON und CLI-Text. Duplikate und ID 0 zählen nicht mehrfach. Leerer Referenzkern ergibt Recall 0; die vorhandene Jaccard-Konvention für zwei leere Kerne bleibt 1. |
| KF2, Persistenz | bestehender Pfad `lib.rs:400`, Test `backtest.rs:254` | Beide Felder werden additiv im bereits persistierten `brain.reasoner_backtests.detail` gespeichert: unter `per_author` je Vergleich und unter `aggregate`. Der vorhandene Upsert serialisiert den vollständigen HeroBacktest und aktualisiert `detail`. Keine neue SQL-Spalte und keine Migration erforderlich; die Tabelle besitzt bereits `detail jsonb`. Serialisierung über den produktiven Persistenzhelfer ist getestet. Kein schreibender Central-Test. |

Die Normierung ist erforderlich, weil die Summe der Bandmediane nicht der Median der Summen ist. Bloßes Auf- oder Abrunden der ursprünglichen ganzzahligen Warden-Mediane könnte 11 nicht ergeben. Restgleichstände gehen deterministisch an das niedrigere Tier. Sind alle Bandmediane null, dienen die beobachteten Bandhäufigkeiten als Proportionen. Ein zusätzlicher Test deckt diesen Fall sowie Gesamtmediane unterhalb der Summe der Bandmediane ab.

Die Snapshot-Ableitung akzeptiert vollständige, konstante `max_purchases_for_tier`-Arrays: Dann gilt die Kapazität unabhängig von der Tier-Zuordnung. Fehlende, ungültige oder unterschiedliche Tier-Werte werden mangels belegter Zuordnung zu den Kostenbändern als nicht ableitbar behandelt; dafür greift der ausdrücklich freigegebene Fallback. Unterschiedliche Tier-Werte werden insbesondere nicht pauschal als vollständig freigeschaltete Kapazität interpretiert. Der aktuelle Warden-Snapshot ist eindeutig. Test: `meta.rs:713`; korrigierter Fallback bei Gesamtmedian 14,5: Ziel 15, Flex 3 (`meta.rs:741`).

### Warden-Kern und Referenzvergleich

Layoutbasis unverändert: 42 aktuelle Warden-Autoren-Builds, Gesamtmedian 10,5. Tatsächlicher Kern: sechs Weapon-, drei Spirit- und zwei Vitality-Items; zwei der sechs verfügbaren Flex-Plätze werden gebraucht.

| Tier | E | F vor Fix | F Fixrunde 1 | Items nach Fix |
| --- | ---: | ---: | ---: | --- |
| T1 | 1 | 0 | 0 | keine |
| T2 | 1 | 1 | 1 | Titanic Magazine |
| T3 | 2 | 2 | 3 | Express Shot, Alchemical Fire, Escalating Resilience |
| T4 | 10 | 5 | 7 | Spirit Burn, Mystic Reverb, Lucky Shot, Mercurial Magnum, Juggernaut, Frenzy, Diviner's Kevlar |
| T5 | 5 | 0 | 0 | keine |
| Gesamt | 19 | 8 | 11 | |

Die folgende Tabelle verwendet konsequent die 19 Kern-Items des Seeds beziehungsweise der expliziten CORE-Kategorie von Build 779996, Version 45. E stammt aus dem versionierten `WARDEN-E-BUILD.json`; F vor Fix wurde mit den bisherigen Bandzielen und dem bisherigen Flex-Budget auf denselben aktuellen Scores rekonstruiert. Die Referenzen haben jeweils 19 Items, unterscheiden sich aber in drei IDs: Der Seed enthält Juggernaut, Witchmail und Transcendent Cooldown, die CORE-Kategorie stattdessen Frenzy, Vampiric Burst und Inhibitor.

| Stand | Treffer / Referenz | Recall Seed | Jaccard Seed | Recall 779996 CORE | Jaccard 779996 CORE |
| --- | ---: | ---: | ---: | ---: | ---: |
| E, 19 Kern-Items | 4/19 | 0,210526 | 0,117647 | 0,210526 | 0,117647 |
| F vor Fix, 8 Kern-Items | 3/19 | 0,157895 | 0,125000 | 0,157895 | 0,125000 |
| F Fixrunde 1, 11 Kern-Items | 3/19 | 0,157895 | 0,111111 | 0,157895 | 0,111111 |

Korrektur zur früheren Fertigmeldung: Die 28 Referenz-Items stammen nicht aus einer veralteten Version. `data.rs:973` zählt für den Backtest zusätzlich andere nicht-optionale Kategorien, während die explizite CORE-Kategorie 19 Items hat. Das erklärt die bisherige Jaccard 0,093023 für E und 0,090909 für F gegen 779996. Der Loader wurde in dieser Fixrunde nicht verändert. Für dessen bestehende 28er-Referenz ergeben sich:

| Stand | Recall 779996 gemäß Loader | Jaccard 779996 gemäß Loader |
| --- | ---: | ---: |
| E | 0,142857 | 0,093023 |
| F vor Fix | 0,107143 | 0,090909 |
| F Fixrunde 1 | 0,107143 | 0,083333 |

Die drei Seed-Treffer sind Titanic Magazine, Mercurial Magnum und Juggernaut; gegen 779996 trifft Frenzy anstelle von Juggernaut. Der gegenüber E fehlende vierte Treffer ist in beiden Vergleichen Boundless Spirit: Total 36,781131, Rang 10 im gesamten T4-Band, Rang 8 unter kernfähigen T4-Kandidaten. Das siebte ausgewählte T4-Item Diviner's Kevlar erreicht 49,117971. Boundless Spirit steht deshalb in Optional. Das Flex-Budget ist nicht die Ursache.

### Ursache je fehlendem Referenz-Item

Bandrang umfasst alle bewerteten Items des Bands, für T1/T2 nach `per_soul_value`, für T3/T4 nach `total`. Alle folgenden 16 fehlenden Seed-Items haben positive Scores. Keines dieser Seed-Items scheitert hier am Flex-Budget oder an einer Situationsklassifikation. Nach der Kernauswahl bleibt der bestehende Optional-Cap von zwölf Items wirksam; deshalb erscheinen nicht alle unterlegenen Kandidaten im ausgegebenen Optional-Block.

| Fehlendes Item | Tier | Bandrang | Ursache |
| --- | ---: | ---: | --- |
| High-Velocity Rounds | 1 | 11 | Bandmedian 0, Ziel 0. |
| Extra Regen | 1 | 9 | Bandmedian 0, Ziel 0. |
| Monster Rounds | 1 | 2 | Bandmedian 0, Ziel 0. |
| Opening Rounds | 2 | 19 | Ein T2-Platz, Titanic Magazine auf Rang 1 gewinnt. |
| Quicksilver Reload | 2 | 6 | Ein T2-Platz, Titanic Magazine auf Rang 1 gewinnt. |
| Swift Striker | 2 | 26 | Ein T2-Platz, Titanic Magazine auf Rang 1 gewinnt. |
| Enduring Speed | 2 | 9 | Ein T2-Platz, Titanic Magazine auf Rang 1 gewinnt. |
| Fleetfoot | 2 | 21 | Ein T2-Platz, Titanic Magazine auf Rang 1 gewinnt. |
| Veil Walker | 3 | 19 | Drei T3-Plätze, unterhalb des Band-Cutoffs. |
| Blood Tribute | 3 | 41 | Drei T3-Plätze, unterhalb des Band-Cutoffs. |
| Spiritual Overflow | 4 | 15 | Sieben T4-Plätze, unterhalb des Band-Cutoffs. |
| Siphon Bullets | 4 | 17 | Sieben T4-Plätze, unterhalb des Band-Cutoffs. |
| Boundless Spirit | 4 | 10 | Achter kernfähiger T4-Kandidat bei sieben Plätzen; Optional. |
| Unstoppable | 4 | 41 | Sieben T4-Plätze, unterhalb des Band-Cutoffs. |
| Witchmail | 4 | 21 | Sieben T4-Plätze, unterhalb des Band-Cutoffs. |
| Transcendent Cooldown | 4 | 18 | Sieben T4-Plätze, unterhalb des Band-Cutoffs. |

Für 779996 gelten dieselben Ursachen bei den 14 gemeinsam fehlenden Items. Witchmail und Transcendent Cooldown sind nur im Seed; stattdessen fehlen folgende zwei Items der CORE-Kategorie:

| Fehlendes Item nur in 779996 | Tier | Bandrang | Ursache |
| --- | ---: | ---: | --- |
| Vampiric Burst | 4 | 23 | Bestehende Situationsregel `Can buy 1`, daher kein Kernkandidat; außerdem unterhalb des Band-Cutoffs. |
| Inhibitor | 4 | 44 | Bestehende Optional-Regel, daher kein Kernkandidat; außerdem unterhalb des Band-Cutoffs. |

### Live-Nachweis und Tests

Debug-Binary aus dem eigenen Worktree, kein Release-Build. `reason build Warden --no-ai --no-persist --json` und `reason backtest --hero <Held> --no-persist --json` für Warden, Abrams, Dynamo und Kelvin: jeweils Exit 0, gültiges JSON. Zusätzlich Warden-Backtest als CLI-Text geprüft: `Recall: 0.1225 | Jaccard: 0.0851` im Aggregat. Der Backtest verwendet wie bisher deterministisches Scoring ohne KI.

Verbindungen: `default_transaction_read_only=on` sowohl über DSN-Verbindungsoptionen als auch PGOPTIONS; der Live-Test bestätigt `SHOW default_transaction_read_only = on`. Keine Schreibzugriffe auf Central, keine Migration ausgeführt. Keine Änderungen an `item.rs`, `mechanics.rs`, `patch.rs`, `hero.rs` oder `data.rs`.

| Held | Vergleiche | Aggregat-Recall | Aggregat-Jaccard | Kernüberdeckung |
| --- | ---: | ---: | ---: | ---: |
| Warden | 43 | 0,122503 | 0,085102 | 0,232558 |
| Abrams | 42 | 0,050565 | 0,036783 | 0,121212 |
| Dynamo | 43 | 0,033711 | 0,024033 | 0,073996 |
| Kelvin | 42 | 0,087936 | 0,064968 | 0,200000 |

Diese Aggregate folgen dem unveränderten CLI-Referenzvertrag einschließlich Seedpfad und der Loader-Kerndefinition. Sie sind keine Vergleiche ausschließlich gegen die 19er-CORE-Kategorie. Gegenüber F vor Fix sinkt die bisherige Kernüberdeckung bei Warden von 0,287791 auf 0,232558, bei Dynamo von 0,093023 auf 0,073996 und bei Kelvin von 0,202381 auf 0,200000; bei Abrams steigt sie von 0,119048 auf 0,121212. Wegen des geänderten Nenners ist daraus keine Recall-Verbesserung abzuleiten. Es wurden keine Helden-Sonderregeln ergänzt.

| Prüfung | Baseline F | Endstand |
| --- | --- | --- |
| Reasoner ohne DSN | 90 bestanden, 0 Fehler, 16 ignoriert | 96 bestanden, 0 Fehler, 16 ignoriert |
| Workspace ohne DSN | 278 bestanden, 0 Fehler, 58 ignoriert | 284 bestanden, 0 Fehler, 58 ignoriert |
| Reasoner `--include-ignored`, Central read-only, ohne Scratch-DSN | 97 bestanden, 9 Scratch-Fehler laut Briefing/Baseline | 103 bestanden, dieselben 9 Scratch-Fehler, 0 ignoriert |
| Clippy `-p dbrain-reasoner --all-targets -- -D warnings` | grün | grün, 0 Warnungen |
| rustfmt nur eigene sechs Rust-Dateien, `skip_children=true` | | Check grün |
| `cargo build -p deadlock-brain` | | Debug-Build grün |
| Selbstprüfung | | Scope, Diff, JSON-Nachweise, `git diff --check` geprüft |

Rot-Gegenprobe: Nach Hinzufügen der ersten fünf Tests zum unveränderten Code 91 bestanden, 4 fehlgeschlagen, 16 ignoriert. Rot waren Warden 8 statt 11, Nullmediane 0 statt 2, korrigierter Fallback 10 statt 15 sowie fehlende Recall-/Jaccard-Felder. Der Nit-4-Test war schon grün, da er ausdrücklich bestehendes Verhalten dokumentiert. Der sechste Test zur Snapshot-Kapazität war vor Implementierung durch die fehlende Funktion rot (E0425), danach grün. Eine vorhandene Erwartung wurde an die neue Gleichstandsregel angepasst: globales T1-Ziel 2 statt 1. Keine Tests entfernt.

Die neun erwarteten Fehler betreffen fünf `data::tests` mit Scratch-Kontext sowie `fix2_build_no_persist_on_read_only_connection`, `fix2_patch_impact_no_persist_on_read_only_connection`, `fix2_backtest_no_persist_on_read_only_connection` und `fix_e_loader_keeps_exact_field_dates_and_current_hero_scaling`. `REASONER_SCRATCH_DSN` ist nicht gesetzt. Eine echte schreibende Persistenzprüfung in Scratch bleibt damit ungemessen; die bestehenden Tests und der additive JSON-Serialisierungstest sind erhalten.

Maschinenlesbare Ergebnisse einschließlich der einzelnen Autorenvergleiche, Referenz-IDs, Scores und Kernlisten: `FIX-F1-EVIDENCE.json`, versioniert im Task-Ordner des Feature-Worktrees `/home/nathanael/.worktrees/deadlock-brain-f`. Rohprotokolle dieser Session: `/tmp/fix-f-baseline.log`, `/tmp/fix-f-red.log`, `/tmp/fix-f-slot-red.log`, `/tmp/fix-f-final-reasoner.log`, `/tmp/fix-f-workspace.log`, `/tmp/fix-f-central.log`, `/tmp/fix-f-clippy.log`, `/tmp/fix-f-build.log`, `/tmp/fix-f-cli-text.log`.
