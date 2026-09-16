# Welle-2-Audit Build-Reasoner (Spitze fc71b5d)

Auditor, nur lesend. Repo `/home/nathanael/repos/Deadlock-Brain`, main `a57382a`.
Datum 2026-09-16. Kein Commit, kein Branch, kein Deploy, kein Schreibzugriff auf die zentrale DB.

## TLDR (Urteil und Vorschlag)

Die Spitze `fc71b5d` ist technisch sauber und ohne Risiko nach main integrierbar: reiner
Fast-Forward (main ist Vorfahr, `git merge-tree` meldet null Konflikte), Superset aller
Feature-Branches, Build grün (Reasoner-Lib 167 bestanden, Workspace 359 bestanden, 0 Fehler,
Clippy sauber), und sie fasst nichts an, das ein laufender Dienst nutzt (kein `main.rs`, keine
Migration, kein Timer, nur Crate-Code, Beispiele, Testdaten und Doku).

Fachlich ist sie nicht abgenommen. Die letzte belastbare Warden-Messung der Spitze steht bei
**3 von 9 Referenzwaffen** (Stand `823b0e5`; `fc71b5d` legt darüber nur ein Doku-Commit). Der
Abnahmemaßstab von mindestens 5 von 9 Waffen wird verfehlt. Die oft zitierte 5-von-9-Zahl gehört
zu einem **früheren, überholten Codestand** (Planner `02e4397`, Combat `dc8c962`), der nicht in
der Spitze liegt; danach kam die Regression `6f40f03` (1 von 9) und der Teil-Fix `823b0e5`
(zurück auf 3 von 9). Holdout, Sechs-Helden-Plausibilität und Sensitivität sind für den aktuellen
Stand nicht gemessen.

**Empfehlung:** `fc71b5d` als grüne Fortschritts- und Infrastruktur-Integration nach main mergen
(konsolidiert elf verstreute Worktrees, keine Regression gegenüber Welle 1, kein Konflikt, kein
Produktivpfad betroffen), aber ausdrücklich **nicht als fachliche Abnahme** deklarieren. Die
Waffenmehrheit bleibt offener Folgeauftrag (Welle 3: Combat-Modell, das die 5 von 9 trug,
gegen den jetzigen Stand rekonstruieren oder den Planner verbessern, danach frische
Warden- plus Holdout-Messung). Die neun Nebenbranches sind danach löschbar; keiner trägt Code,
der nachgezogen werden müsste (SHA-Liste in Abschnitt 6).

## 1. Topologie der Branches gegen die Spitze fc71b5d

Merge-Base(main, fc71b5d) = main = `a57382a`. Die Spitze ist also eine lineare Erweiterung von
main. Für jeden Branch die Commits, die nicht in `fc71b5d` liegen (`git log fc71b5d..<branch>`):

| Branch | SHA | Commits nicht im Tip | davon Code | Einordnung |
|---|---|---:|---:|---|
| feat/build-reasoner-komposition | 18bb999 | 0 | 0 | reiner Vorfahr, liegt im Tip |
| feat/build-reasoner-integration | e263b5f | 0 | 0 | reiner Vorfahr, liegt im Tip |
| feat/build-reasoner-holdout-final | 24d8a26 | 1 | 0 | Doku-Hülle (Briefing Holdout-Agent) |
| feat/build-reasoner-planner-audit | 37c0dcb | 1 | 0 | Doku-Hülle (Briefing Planner-Regression-Agent) |
| review/build-reasoner-final | 8db45af | 1 | 0 | Doku-Hülle (Briefing Kritiker-Agent) |
| feat/build-ability-interactions | 3c0c5ba | 5 | 3 | älterer Code, im Tip als Superset überholt |
| feat/build-reasoner-planner | 4009f38 | 19 | 19 | älterer Code, im Tip überholt |
| feat/build-reasoner-combat | baa3270 | 24 | 22 | älterer Code, im Tip überholt |
| feat/build-reasoner-evaluation | db6fd42 | 48 | 44 | eigene Historie, inhaltlich fast deckungsgleich mit dem Tip |

Prüfung auf verlorene Fachfunktion über Baum-Diffs `git diff fc71b5d..<branch>`: In jedem Fall
enthält der Tip deutlich mehr Code als der Branch (Tip-`combat.rs` 2783 Zeilen, `planner.rs` 858,
`progression.rs` 398, `ability_interactions.rs` 345, `item_interactions.rs` 239, `inventory.rs`
249). Die wenigen Zeilen, die ein Branch gegenüber dem Tip zusätzlich hat, sind ältere,
überarbeitete Fassungen derselben Funktionen (Beispiel Planner: eine frühere `damage_scale`,
frühere Combat-Schleife), nicht fehlende Features:

| Branch | Nur-im-Branch Rust-Zeilen (kein Testdata/Doku) | Bewertung |
|---|---:|---|
| feat/build-reasoner-evaluation | 9 | praktisch im Tip enthalten |
| feat/build-ability-interactions | 56 | überholte Vorfassung |
| feat/build-reasoner-planner | 243 | überholte Vorfassung |
| feat/build-reasoner-combat | 335 | überholte Vorfassung |

Reine Vorfahren: komposition, integration (beide sind Commits in der Tip-Historie, `18bb999`
und `e263b5f`). Reine Doku-Hüllen: holdout-final, planner-audit, review (je ein Briefing-Commit,
nicht im Tip, ohne Codewert).

## 2. Was die Spitze gegen main ändert

`git diff --stat main...fc71b5d`: 44 Dateien, 14672 Einfügungen, 366 Löschungen.

Grob je Modul (Crate `dbrain-reasoner`, sofern nicht anders genannt):

| Bereich | Datei | Zeilen (Diffstat) |
|---|---|---|
| Combat-Simulation | src/combat.rs | 2783 neu |
| Planner (Kaufkurve) | src/planner.rs | 858 neu |
| Progression (Skill/Budget) | src/progression.rs | 398 neu |
| Item-Interaktionen | src/item_interactions.rs | 239 neu |
| Fähigkeits-Interaktionen | src/ability_interactions.rs | 345 neu |
| Inventarbewertung | src/inventory.rs | 249 neu |
| Composer (Umbau) | src/composer.rs | 729 geändert |
| Datenlader/Anreicherung | src/data.rs | 807 geändert |
| Kleinere Kopplung | lib.rs 53, types.rs 38, item.rs 43, mechanics.rs 42, hero.rs 9, patch.rs 2 | gemischt |
| Beispiele (Messung) | examples/build_evaluation.rs 528, ability_coverage.rs 195, combat_parity.rs 53, support/mod.rs 33, build.rs 26 | neu |
| Testdaten | testdata/combat-snapshot 3608, ability-interactions/raw 1786, item-interactions 636, life-drain-charges 303 | neu |
| Retrieval | dbrain-retrieval/examples/ask_latency.rs 232, Cargo.toml | neu |
| Akte | .tasks/2026-09-13-build-komposition/* (14 Dateien) | Doku |

Änderungen außerhalb von `dbrain-reasoner` und `dbrain-retrieval`
(`git diff --stat main...fc71b5d -- ':(exclude)...'`): nur die Akte
`.tasks/2026-09-13-build-komposition/*` und `rust/Cargo.lock` (11 Zeilen). **Kein `main.rs`, keine
Migration, kein Timer, keine systemd-Unit, kein Produktiv-Binary.** Der Reasoner ist weiterhin
ein Crate ohne Verdrahtung in einen laufenden Dienst; ein Merge schaltet nichts scharf und
veröffentlicht keine Builds.

## 3. Bau- und Teststand der Spitze (Worktree build-reasoner-interactions-audit)

Werkzeug `cargo 1.97.1`, PATH auf `/home/nathanael/.cargo/bin`, kein `--release`, kein DSN gesetzt.

| Prüfung | Ergebnis |
|---|---|
| cargo test -p dbrain-reasoner --lib | 167 bestanden, 0 Fehler, 16 ignoriert |
| cargo clippy -p dbrain-reasoner --all-targets -- -D warnings | bestanden, keine Warnung |
| cargo test --workspace | 359 bestanden, 0 Fehler, 58 ignoriert (20 Testbinaries) |

Baseline main laut Auftrag: 273 bestanden, 58 ignoriert. Die Spitze bringt 86 zusätzliche
bestandene Tests (neuer Reasoner- und Retrieval-Code), gleiche 58 ignoriert, **keine Regression,
kein roter Test**. Die 16 ignorierten Reasoner-Tests und die restlichen ignorierten Workspace-Tests
sind die DSN- und Integrationsfälle, die ohne zentrale DB nicht laufen.

## 4. Warden-Stand

Referenz Warden `779996`, Version 45, 19 Kernitems, davon 9 Waffen. Messgrundlage
`FROZEN-V2.json` (SHA256 `5451b0b4...cb7b1a`), Beispiel `build_evaluation`, Modus `plan`.

Zeitliche Abfolge der Messungen:

| Stand | Revision | im Tip? | Waffen | Kern | Recall | Jaccard |
|---|---|---|---:|---:|---:|---:|
| Welle 1 (ABNAHME-FINAL) | Welle-1-Basis | ja | 2/9 | 3/19 | 0,157895 | 0,093750 |
| Erste gemeinsame Inventarplanung (Spitzenwert) | Planner 02e4397, Combat dc8c962 | **nein** | 5/9 | 5/19 | 0,263158 | 0,166667 |
| Dokumentierter Stand vor der Regression | bcc3f6e-Umfeld | ja | 4/9 | n/a | 0,263158 | 0,166667 |
| Integrationsbasis (Regression 6f40f03) | b29db00 | ja | 1/9 | n/a | 0,052632 | 0,029412 |
| **Nach Fix (letzter belastbarer Tip-Stand)** | **823b0e5** | **ja** | **3/9** | n/a | **0,315789** | **0,206896** |

Die letzte belastbare Messung für die Spitze ist der Fix-Stand `823b0e5` mit **3 von 9 Waffen**
(Titanic Magazine, High-Velocity Rounds, Opening Rounds). `fc71b5d` unterscheidet sich von
`823b0e5` nur um das Doku-Commit `combat-throughput-ABSCHLUSS.md` (146 Zeilen, kein Code), die
3-von-9-Zahl gilt also unverändert für die Spitze. Der Fix wurde nach `823b0e5` gemessen; die
5-von-9-Zahl aus MESSUNG-ZWISCHENSTAND ist **nicht neu bestätigt** und gehört zu einem anderen,
nicht im Tip liegenden Codestand.

Messartefakte: `FROZEN-V2.json`, `AFTER-FIRST-WARDEN-V2.json`,
`COMBAT-FIX-AFTER-823b0e5-WARDEN.json` und `COMBAT-FIX-HEAD-b29db00-WARDEN.json` liegen
**nicht eingecheckt** in `/home/nathanael/Documents/.tasks/2026-09-13-build-reasoner-ganzbuild/nachweise/`.
Im Repo eingecheckt sind nur die Welle-1-Dateien `.tasks/2026-09-12-build-reasoner/WARDEN-E*.json`.

Eigene Nachmessung nicht möglich: `DEADLOCK_CENTRAL_DSN` ist in dieser Umgebung **nicht gesetzt**,
und `examples/build_evaluation.rs` öffnet einen Postgres-Pool (`PgPoolOptions::new()` Zeile 276,
`freeze_guard` erzwingt `transaction_read_only`). Das Beispiel ist also nicht rein offline gegen
die Frozen-Datei lauffähig, sondern braucht eine DB-Verbindung. Auftragsgemäß wird das DSN nicht
beschafft; die Messung bleibt dokumentiert, nicht reproduziert.

## 5. Offene fachliche Punkte aus den Akten

- **Abnahme-Maßstab verfehlt.** ABNAHME-FINAL fordert die Mehrheit des Warden-Waffen-Kerns,
  mindestens 5 von 9. Aktueller Tip-Stand 3 von 9. Ursprünglicher Nutzer-Intent (belastbare
  eigene Meta-Builds) bleibt fachlich offen.
- **5-von-9 war transient und ist verloren.** Der Spitzenwert lag auf Planner `02e4397` und
  Combat `dc8c962` (nicht im Tip). Die Regression `6f40f03` warf ihn auf 1 von 9, der Fix
  `823b0e5` holte nur 3 von 9 zurück. Laut combat-throughput-ABSCHLUSS hat der Restabstand drei
  dokumentierte Quellen ohne neuen Fehlerbefund: feste 1,0 s Zielwechselpause deckelt den
  Durchsatz, Duell und bewegliches Ziel enden beim Zieltod und messen nur die Eröffnungsphase
  (Spätzünder wie Swift Strikers RampUp fallen weg), plus kleines nicht-monotones
  Ausrichtungsrauschen bei Munitions- und Magazinkäufen.
- **Holdout nie ausgewertet.** MESSUNG-ZWISCHENSTAND hält fest, dass ein Holdout noch nicht
  gelaufen ist; das Holdout-Briefing liegt nur als Doku-Hülle auf `24d8a26`. Ohne Holdout ist der
  bisherige Treffer Autorenübereinstimmung mit bekannter Entwicklungsreferenz, kein unabhängiger
  Algorithmusnachweis.
- **Sechs-Helden-Plausibilität und Sensitivität** sind für den Fix-Stand nicht gelaufen (laut
  ABSCHLUSS an den Orchestrator abgegeben).
- **Belegte Modelllücken** (URSACHEN-KAUFKURVE, MESSUNG-ZWISCHENSTAND): `MaxHealthLossPercent`
  bei Glass Cannon nicht quantifiziert, Lightning Scroll mit unbekanntem Damage-/Triggernutzen,
  nicht simulierte Freischaltzeitpunkte, Slot- und Erstattungsvorgaben als sichtbare Annahmen.
  Bewertungszahlen sind Modellwerte, keine gemessenen Spielschadenswerte.
- **Anzeigefeld `spirit_power`** rechnet weiter auf Kampfzeit hoch, ist aber nicht Teil von
  Scoring oder Planung (Rest aus dem Combat-Fix, unkritisch).
- **Review-Branch** `review/build-reasoner-final` enthält nur das Kritiker-Briefing, kein
  abgeschlossenes Review-Urteil im Repo. Ein unabhängiges Abnahme-Urteil zum Fix-Stand fehlt.

## 6. Merge-Empfehlung

**Konfliktlage:** `git merge-base --is-ancestor main fc71b5d` ist wahr, `git merge-tree
--write-tree --messages main fc71b5d` endet mit Exit 0 ohne Konfliktmarker. Der Merge ist ein
reiner Fast-Forward, kein Trockenmerge-Worktree nötig (deshalb keiner angelegt). Null Konflikte.

**Empfehlung:** `fc71b5d` nach main mergen als grüne, konfliktfreie Superset-Integration von
Welle 2. Sie ist strikt besser als Welle 1 (2 von 9), bricht keinen Test, betrifft keinen
Produktivpfad und beendet den Zustand von elf verstreuten Worktrees. Der Merge ist ausdrücklich
**Fortschritts- und Infrastruktur-Konsolidierung, keine fachliche Abnahme**: das Ziel von 5 von 9
Waffen bleibt offen und gehört in eine Welle 3 (Combat-Modell des 5-von-9-Standes gegen den
jetzigen Stand rekonstruieren oder Planner nachschärfen, danach frische Warden- und Holdout-Messung
mit DSN). Wer die 5 von 9 vor dem Merge sehen will, mergt nicht und iteriert auf dem Tip-Branch
weiter; angesichts des grünen, verdrahtungsfreien Codes ist die Konsolidierung nach main aber die
sauberere Wahl.

**Nach dem Merge löschbar** (SHA-Backup):

```
4009f38 feat/build-reasoner-planner
baa3270 feat/build-reasoner-combat
3c0c5ba feat/build-ability-interactions
18bb999 feat/build-reasoner-komposition
db6fd42 feat/build-reasoner-evaluation
e263b5f feat/build-reasoner-integration
24d8a26 feat/build-reasoner-holdout-final
37c0dcb feat/build-reasoner-planner-audit
8db45af review/build-reasoner-final
fc71b5d feat/build-reasoner-interactions-audit
```

Reine Vorfahren (komposition, integration) und Doku-Hüllen (holdout-final, planner-audit, review)
tragen keinen Code und sind bedenkenlos löschbar. planner, combat, ability und evaluation tragen
nur überholte Vorfassungen, deren finale Form im Tip liegt; **nichts davon muss nachgezogen
werden**. Fachlich nachzuziehen ist keine Codezeile, sondern die offene Messung (Holdout, 5 von 9,
Sechs-Helden), die in Welle 3 gehört. Die nicht eingecheckten Messartefakte im Documents-Ordner
sollten vor dem Löschen der Worktrees gesichert bleiben, da sie die einzige Belegkette der
Warden-Zahlen sind.
