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
