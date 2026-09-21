# PHASE-0: eingefrorene Messbasis (vor Phase A)

Stand 2026-09-16. Worktree `/home/nathanael/repos/wt/brain-purpose-a`, Branch
`feat/reasoner-purpose-a`, Basis-Commit 706b129. Messbinary-Revision zum
Auswertungszeitpunkt: `806d4d58` (eingecheckter sauberer Stand, SOURCE_CLEAN=true).
Zentrale DB ausschliesslich lesend ueber `pg_pool_read_only()` (setzt
`default_transaction_read_only=on` an den Verbindungsoptionen); der Populations-
Freeze lief zusaetzlich in `BEGIN ... READ ONLY` nicht, sondern als reine SELECTs
ueber diese read-only-Verbindung.

## Status kurz

- Vorher-Backtest steht als eingefrorenes, DB-freies Artefakt (PHASE0-EVAL.json).
- Roster-Modell-Abdeckung steht (PHASE0-COVERAGE.json).
- Population frisch eingefroren (PHASE0-POP.json), Hash belegt.
- Frischer Vollfreeze GESCHEITERT (Performance-Blowup), als separater BLOCK gefuehrt.

## Messgrenze (verbindlich)

Der Vorher/Nachher-Vergleich nutzt `FROZEN-V2.json` (Heldenmodelle + Items,
Algorithmus-Stand `a57382a`, eingefroren 2026-09-13) zusammen mit einer frisch
eingefrorenen Population (2026-09-16). Das ist reproduzierbar und fuer beide
Codefassungen bitidentisch als Eingabe, aber ein GEMISCHTER historischer Stand
(Item-/Patchmischung aus 2026-09-13 plus Population 2026-09-16), KEINE frische
Gesamtbaseline. Verglichen wird gegen diese frisch berechnete Phase-0-Baseline,
nicht gegen die in FROZEN-V2 eingebettete alte `live_baseline`.

Historische Werte (kendall 0.577, jaccard 0.500 aus `reason backtest` live) sind
NICHT diese Messung; sie stammen aus einem anderen Werkzeug und Zeitpunkt.

## Eingefrorene Eingaben (Hashes)

- FROZEN-V2.json (Heldenmodelle/Items): sha256 `5451b0b4f3cde928f12f…` (dokumentiert
  in .tasks/2026-09-13-build-komposition/MESSVERTRAG.md, hier gegengeprueft),
  Datei unter /home/nathanael/Documents/.tasks/2026-09-13-build-reasoner-ganzbuild/nachweise/.
- nachweise/PHASE0-POP.json (Population, frisch): sha256 `bcec521e7e4db06f585b…`,
  38 Helden, 200 Staples, 5669 Praevalenz-Items. Read-only aus zentraler DB.
- nachweise/PHASE0-EVAL.json (Vorher-Backtest): sha256 `7c7ba4a9d9100614796e…`.
- nachweise/PHASE0-COVERAGE.json (Roster-Modell-Abdeckung): 38 Helden, 1 Konverter,
  37 Nullkonverter, 0 negativ/nicht endlich, 0 Vorzeichen-Divergenz.

## Vorher-Backtest (eingefroren, DB-frei)

Befehl (identisch fuer Nachher):
`FROZEN_POPULATIONS=nachweise/PHASE0-POP.json build_evaluation evaluate FROZEN-V2.json PHASE0-EVAL.json "Warden,Infernus,Abrams,Lady Geist,Vindicta"`

| Held | Autor-Build | Referenzwaffen | jaccard@12 | kendall_tau | Staple-Gate |
|---|---|---|---|---|---|
| Warden | 779996 | 6/9 | 0.500 | 0.4706 | 9/10, FALSE (fehlt Enduring Speed 2447176615) |
| Infernus | 256053 | 5/7 | 0.846 | 0.6557 | 9/9, true |
| Abrams | 749234 | 3/7 | 0.263 | 0.7152 | 4/4, true |
| Lady Geist | 253366 | 2/7 | 0.263 | 0.6419 | 4/4, true |
| Vindicta | 805943 | 2/3 | 0.600 | 0.3799 | 9/9, true |

Warden ist der einzige datenbelegte Konverter im Roster; die vier weiteren dienen
als Nullkonverter-Holdouts und Breite. `switch_detected` durchgaengig null.

### Warden-Kern (779996-Vergleich, Vorher)

Extended Magazine, Titanic Magazine, High-Velocity Rounds, Opening Rounds,
Extra Spirit, Improved Spirit, Swift Striker, Quicksilver Reload, Fleetfoot,
Extra Health, Mercurial Magnum, Rebuttal, Spiritual Overflow, Fury Trance,
Glass Cannon, Leech.

Fehlurteil-Abgleich (BEFUND bezog sich auf den frueher veroeffentlichten Build
811202): Mystic Regeneration, Rusted Barrel und Healing Tempo sind in diesem
Vorher-Kern NICHT enthalten. Glass Cannon ist enthalten (BEFUND: nur mit
Survivability-Deckung vertretbar). Getroffene Referenzwaffen (6/9): Titanic
Magazine, High-Velocity Rounds, Opening Rounds, Swift Striker, Fleetfoot,
Spiritual Overflow; verfehlt: Frenzy, Blood Tribute, Monster Rounds.

## Unquantifizierte Mechanik / Konversions-Abdeckung

Die Modell-Abdeckung liest die bereits normalisierte `HeroModel.scaling`, nicht die
rohen Assets; sie ist damit ein Beleg fuer den Loader-Stand, kein Beweis fuer die
Vollstaendigkeit des Asset-Parsings. Auf FROZEN-V2 traegt genau ein Held
(Warden) eine Spirit->Feuerraten-Konversion (ERoundsPerSecond 0.01, EFireRate 0.25
als redundanter Prozent-Fallback). Ob neuere Helden im aktuellen Assetstand weitere
Konversionen tragen, ist mit FROZEN-V2 NICHT belegt (siehe Freeze-BLOCK).

## BLOCK: frischer Vollfreeze nicht abgeschlossen

Befehl: `build_evaluation freeze nachweise/PHASE0-FROZEN.json 5< ~/.config/infisical-tokens/infisical-token-bots`
(read-only ueber pg_pool_read_only, REPEATABLE READ READ ONLY).

Laufzustand: 38 Helden erfolgreich eingefroren (letzte drei in hero_id-Reihenfolge:
Rem, Silver, Celeste). Danach blieb der Prozess (PID 3226244) ueber ~20 Minuten bei
konstant ~96% CPU und ~3 GB RSS im `reason_build_with_options`-Aufruf fuer den
naechsten Helden stehen, ohne weitere Ausgabe. Kein Fortschritt in einem 9-Minuten-
Beobachtungsfenster. Prozess anschliessend beendet; es entstand keine Teildatei
(atomarer Write am Ende).

Der haengende Held wird NICHT aus der Reihenfolge als "bewiesen langsam" benannt;
belegt ist nur: der Held nach Celeste in hero_id-Ordnung loest im Live-Baseline-
Build einen Performance-Blowup aus. Das ist ein separater Performance-BLOCK und wird
nicht still aus der Gesamtfreigabe gestrichen. Ein frischer Gesamtfreeze steht damit
aus; die Phase-A-Messung laeuft ersatzweise auf dem oben benannten gemischten
FROZEN-V2+Population-Stand.

## Testbaseline (Produktivpfad-Unittests, ohne DB)

TESTNACHWEIS[TW-1]: 173 passed, 16 ignored | Baseline: 0 rot
Befehl: `cargo test -p dbrain-reasoner --lib` (cargo 1.97.1). Exit 0.
