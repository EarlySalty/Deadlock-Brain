# Pakete: build-reasoner

status: aktiv (2026-09-12)

Delegator: Fable, Thread `33a32f58-476b-4a67-99cc-8f6c1e8f7001`. Pakete sind
disjunkt nach Dateien: jede Datei liegt in genau einem Paket, kein Paket fasst
Dateien eines anderen an. Verbindliche Spec: `ARCHITEKTUR.md` und `MECHANIK.md`
in diesem Ordner. Vorcheck-Befund: `VORCHECK-ERGEBNIS.md` (sobald vorhanden).

Entscheidungen des Delegators zu den offenen Fragen aus ARCHITEKTUR.md Abschnitt 14:
1. Skalierungsstufe: Paket B liest `property_upgrades[].name` plus
   `scaling_stats` und verifiziert `scale_function` am DB-Snapshot.
2. Paket A verifiziert die Spalten von `hero_stat_values` und
   `hero_item_synergies` am echten Schema, bevor `data.rs` festgezurrt wird.
3. Reihenfolge-Nähe auf der Autoren-Seite kommt aus der Reihenfolge im
   `details`-Array; ohne Reihenfolge meldet der Backtest "nicht messbar".
4. Warden-Referenz: `referenz/lightbringer-warden.json` ist ein manueller
   Seed-Build. Paket C liest Seed-Builds aus `referenz/*.json` zusätzlich zu
   `tierlist.hero_build_sources`; nichts wird in `tierlist` geschrieben.
5. Reaktivierung des Autoren-Scans im Steam-Bot ist Paket S (eigenes Repo,
   eigener Thread), startet nach A.

Befunde aus dem Vorcheck (`VORCHECK-ERGEBNIS.md`) und Entscheidungen dazu:
6. Es gibt keinen Deadlock-Patch nach dem 22.08.2026 (Steam-News-Feed und
   `patchnotes.changelog_posts` stimmen überein, Brain-Sync ist aktuell). Ein
   Patch namens "Nightshift" existiert in keiner Quelle. Wardens letzte
   Helden-Änderungen sind vom 30.06.2026 (Willpower T3 Spirit-Skalierung hoch,
   T2 Cooldown, Bullet damage per boon runter); seither nur Spiritual Overflow
   am 28.07. leicht generft. Folge für Paket C: Patch-Wechsel-Erkennung für
   Warden vergleicht den Stand vor und nach dem 30.06.; die Meta-Verschiebung
   danach kommt aus Build-Entdeckung der Autoren, deshalb bleibt das
   Autoren-Signal hoch gewichtet und Item-Deltas zählen genauso wie
   Helden-Deltas.
7. `brain.hero_item_stats` und `hero_item_synergies` waren nur für Ivy, Graves,
   Bebop gefüllt. Der Delegator hat `pull build-data` für Warden gefahren (154
   Item-Zeilen, 16 Lift, 3000 Synergien) und zieht alle übrigen Helden im
   Hintergrund nach (`/tmp/brain-build-data-all.log`). Paket D legt dafür
   einen Timer an und gibt `patch_tag` einen echten Wert (Datum oder
   Patch-ID des jüngsten Patches beim Sync) statt "current"; `dbrain-builds/
   src/sync.rs` gehört damit zu Paket D.
8. `ChatCompletionRequest` in `deadlock-brain-core/src/ai.rs` bekommt in
   Paket A optionale Felder `response_format` und `reasoning_effort`; die
   Datei gehört zu Paket A.
9. Kosten je Item nur aus Snapshots (assets `cost`, item_card `Cost`),
   `item_catalog.item_id` ist die GC-ability_id; Bedingungstext, Imbue-Flag
   und Cooldown aus `item_card`, Zahlen aus `item_or_ability`. Paket A führt
   beide je Item zusammen.
10. Publish: `build-spec` gibt heute nur JSON aus, niemand reiht
    `BUILD_PUBLISH_ORIGINAL` ein. Der Enqueue in `steam.steam_tasks` gehört
    zu Paket C (`publish.rs`), der Steam-Bot bleibt unverändert.

| Paket | Inhalt (Dateien) | Worker-Thread | Modell | Status |
|---|---|---|---|---|
| A | `rust/Cargo.toml` (Workspace-Eintrag), `rust/crates/dbrain-reasoner/Cargo.toml`, `src/lib.rs`, `src/types.rs`, `src/data.rs`, `src/ai_roles.rs`, `deadlock-brain-core/src/ai.rs` (nur zwei optionale Request-Felder), Tests dazu | 2cbde9a0 | luna | läuft seit 2026-09-12 20:05 |
| B | `rust/crates/dbrain-reasoner/src/mechanics.rs`, `src/hero.rs`, `src/item.rs`, Tests dazu | folgt | luna | wartet auf A |
| C | `rust/crates/dbrain-reasoner/src/patch.rs`, `src/meta.rs`, `src/composer.rs`, `src/backtest.rs`, `src/publish.rs`, `dbrain-builds/src/spec.rs` (nur Erweiterung `BuildSpecMod`/`BuildSpecCategory`), Tests dazu | folgt | luna | wartet auf A |
| D | `rust/crates/deadlock-brain/src/main.rs` (Subcommand `reason`), `lib.rs`-Integration, `dbrain-builds/src/sync.rs` (echter `patch_tag`), Timer für `pull build-data`, Migration für die neuen `brain`-Tabellen, Doku `docs/BUILD_REASONER.md`, Backtest-Report | folgt | luna | wartet auf B und C |
| S | Deadlock-Steam-Bot: Autoren-Scan reaktivieren, Autoren Lightbringer und Situation ergänzen | folgt | luna | wartet auf A |

Worktrees: `~/.worktrees/deadlock-brain-<paket>`, Branch
`feat/build-reasoner-<paket>` von `main`. B und C zweigen vom gemergten A ab.

Bei Bump-up oder Kontextverlust hier den Stand nachziehen: erledigte Pakete auf
"fertig" plus Commit, laufende auf den aktuellen Stand. Jeder Bump-up bleibt
auf dem bestehenden Worktree-Stand.

Befunde nach dem S-Deploy (2026-09-13):
11. Der Autoren-Scan läuft live (508ba20): je Autor 38 Helden-Suchen mit
    Ergebniscode 1, Build 779996 (Version 45, 5 Kategorien) ist in
    `hero_build_sources`, 1590 Builds gesamt. Nebenbefund: jeder Autoren-Scan
    liefert dieselben 1522 Builds, der GC filtert nicht nach
    `author_account_id`. Die Schleife je Autor ist damit 13-fach redundant
    (rund 2,3 Minuten GC-Verkehr je Autor). Folgeauftrag S3 (klein): einmal je
    Held suchen, Autoren clientseitig zuordnen, Status je Autor aus dem
    Ergebnis ableiten.
12. Paket D (5f438d2, 7fbb128): Backtest gegen den Lightbringer-Seed
    Kern-Überdeckung 0,158, Reihenfolge-Nähe 0,435, Patch-Wechsel nicht
    messbar. Ursache laut REPORT-D: `hero_item_stats` für den neuen echten
    Patch-Tag fehlen (Meta-Stütze 0), Lane-Phase schließt vom Core aus,
    Zustandsfaktor 0,6 auf Imbue-Items, Core-Cutoff 19. Review D klärt, was
    Mangel und was Kalibrierung ist; nach dem Deploy erst Sync mit echtem
    Patch-Tag, dann Backtest wiederholen.

Befund nach dem Brain-Deploy (2026-09-12 23:32, Release 4689b80):
13. Live-Warden-Build ist falsch: Kern aus Spirit-Items (Trophy Collector,
    Golden Goose Egg), Waffen-Items nach der Patch-Anwendung bei minus 2500
    bis minus 4400 (`brain.reasoner_item_scores`), Block Optional mit 144
    Items. Ursache: alle historischen Patch-Deltas (416 Warden-Events, alle
    vor dem Snapshot-Datum) werden kumulativ auf den aktuellen Snapshot
    angewendet, dazu mehrfach vorliegende, widersprüchlich klassifizierte
    Patchzeilen. Paket E (`fix/build-reasoner-patch-delta`,
    `FIX-BRIEFING-E.md`): Deltas nur nach dem Snapshot-Datum, Dedup, Vorzeichen
    aus den Zahlen, Kern nur positive Items, Optional gedeckelt.

Befund nach Paket E (2026-09-13 00:35):
14. E behebt die Patch-Anwendung (0 Deltas auf den aktuellen Snapshot, Kern
    nur positive Scores, Spirit-Feuerrate im Scoring). Der Kern verletzt aber
    die Spiellogik: 15 von 19 Kern-Items sind Tier 4 oder 5, der Referenzbuild
    hat 3/6/2/8 über Tier 1 bis 4. Ursache ist der globale Cutoff nach
    Slot-Wert. Paket F (mittel, nach E-Merge): Kern als Einkaufskurve nach
    Kostenband, Layout aus echten Autoren-Builds abgeleitet, Tier 1 und 2 nach
    per_soul_value, Backtest über mehrere Helden. Briefing `BRIEFING-F.md`.
