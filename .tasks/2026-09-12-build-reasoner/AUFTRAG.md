# Auftrag: build-reasoner

status: aktiv (2026-09-12)

## Ziel

Deadlock-Brain baut eigene Meta-Builds je Held, die aus der Spiel-Grundlogik
kommen und nicht aus einem Winrate-Ranking: Wie funktioniert der Held (Fähigkeiten,
Skalierung, Waffe, Tickraten), was macht ein Item konkret auf diesem Helden
(Wert gegen Bedingung, Proc-Cooldown gegen Tickrate, Kaufbonus je Slot und Tier,
Shop-Boni), wann kauft man was (Lane, Kern, Spätspiel, Situationsblöcke), was hat
der letzte Patch am Helden und an den Items geändert und wie verschiebt das die
Meta. Am Ende steht je Held ein begründetes Build (Kern, Situationsblöcke wie
"Can buy 1", Tryhard, Shields, Optional, Reihenfolge, Warum-Text je Item), das
über den bestehenden Steam-Bot-Publish ins Spiel geht und über den Ask-Pfad
erklärt werden kann.

Referenzfall: Lightbringer x Situation Warden Build (Screenshot in
`referenz/lightbringer-warden.png`). Vor dem Nightshift-Patch galt Warden als tot,
mit diesem Build ist er wieder Meta. Der Algo muss aus Patch-Delta plus
Mechanik erklären können, warum dieses Build funktioniert, und im Backtest
mindestens den Kern treffen.

Bauweise: spezialisierte Agenten, die zusammenarbeiten. Der Kern ist
deterministisch in Rust (Helden-Modell, Item-Modell, Mechanik-Rechnung,
Patch-Delta, Meta-Signale, Composer, Backtest). Die KI-Rollen (Hero-Analyst,
Item-Analyst, Patch-Analyst, Meta-Analyst, Kritiker) laufen ausschließlich über
den bestehenden Fireworks-Client des Brains mit Deepseek V4 Flash und liefern
Begründungen, Klassifikationen und Kritik, nie die Zahlen.

## Arbeitsschritte

1. Design (Opus 4.8, lesend): `ARCHITEKTUR.md` und `MECHANIK.md` in diesem
   Ordner: Datenmodell, Schnittstellen der Module, Rechenregeln der Mechanik,
   Agentenrollen und ihr Zusammenspiel, Backtest-Kennzahlen, Paketschnitt
   disjunkt nach Dateien.
2. Vorcheck (GLM, lesend): `VORCHECK-ERGEBNIS.md` mit Fundstellen pfad:zeile
   für alles, was wiederverwendet wird.
3. Pakete nach `PAKETE.md` (Luna, je Paket ein Worktree und ein Thread), Reihenfolge
   und Abhängigkeiten stehen dort.
4. Review-Runden nach Ablauf, Merge nach main, Backtest-Zahlen im Report.

## Fundstellen (aus Bestandsaufnahme, Vorcheck ergänzt)

- `rust/crates/dbrain-builds/src/engine.rs:332`: `composite_score` heutiges
  Item-Ranking aus Winrate und Verbreitung, drei starre Pfade spirit/weapon/tank.
- `rust/crates/dbrain-builds/src/api.rs`: deadlock-api Assets v2 und Analytics
  (`build-item-stats`, `item-stats`), Publish-Payload in `spec.rs`/`sync.rs`.
- `rust/crates/dbrain-learn/src/build_optimizer.rs:82-155`: deterministischer
  Helden-Kontext (Damage-Plan, Scaling-Stats, Archetypen, Item-Semantik).
- `rust/crates/dbrain-learn/src/build_learning.rs`: Import von Steam-Builds
  (`learned_builds`), Fireworks-Analysen (`build_learning_notes`).
- `rust/crates/dbrain-enrich/src/lib.rs:789-859`: `run_meta_trend_analysis`
  mit Mock-Shifts, `meta_trend_notes` leer.
- `rust/crates/deadlock-brain-core/src/ai.rs`: Fireworks-Client (Deepseek V4 Flash).
- `rust/crates/dbrain-retrieval/src/lib.rs`: `ask`-Kontext, Timeline, Claims.
- DB `brain`: `patch_events` 32k, `patch_event_enrichments` 32k,
  `entity_snapshots` 30k, `hero_stat_values` 9,5k, `hero_item_stats` 500,
  `hero_item_synergies` 7,4k, `item_catalog` 251, `hero_catalog` 38,
  `learned_builds`, Claims aus YouTube, Forum, Reddit.
- DB `tierlist` (Steam-Bot): `hero_build_sources`, `watched_build_authors`,
  `meta_builds`, `meta_items`, `meta_heroes`: Builds der Top-Autoren.
- Skill `~/.claude/skills/deadlock-build-bauen/SKILL.md`: Rechenregeln
  (Kaufbonus je Slot, Tickrate gegen Proc-Cooldown, bedingte Items,
  Metrikfalle, Slots knapper als Souls, Nebenwaffe), bisher nur Anleitung.
- Steam-Bot Publish: `rust/crates/steam-core/src/task/handlers/builds/publish_original.rs:78`.

## Was nicht angefasst wird

- Steam-Bot-Code, Twitch-Bot, Discord-Bot: nur lesend, der Publish-Weg wird
  über die bestehende Schnittstelle genutzt.
- Ingest-Pfade (Forum, Reddit, YouTube, Sheet, Patchnotes) bleiben wie sie sind.
- Kein neues LLM, kein eigener Connector, keine Modellnamen im Code außerhalb
  der bestehenden Config. Kein Python.
- Keine Code-Kommentare. Echte Umlaute in allem, was der Nutzer sieht.

## Fertig-Kriterium

- `deadlock-brain reason build <held>` liefert ein begründetes Build mit Kern,
  Situationsblöcken, Reihenfolge und Warum je Item, reproduzierbar ohne KI-Aufruf
  (KI nur für Begründungstext und Kritik, per Schalter abschaltbar).
- `deadlock-brain reason patch-impact <held>` erklärt, was der letzte Patch am
  Helden und an seinen Kern-Items verschiebt.
- `deadlock-brain reason backtest` misst gegen die Builds der beobachteten
  Top-Autoren je Patch-Stand: Kern-Überdeckung, Reihenfolge-Nähe, und ob der
  Algo einen patchbedingten Wechsel findet. Zahlen im Report, Warden als
  Pflichtfall.
- Publish über den bestehenden Steam-Bot-Weg funktioniert mit dem neuen
  Build-Objekt.
- cargo fmt, clippy und test grün auf den eigenen Änderungen.

## Deploy-Weg

Merge nach main, Release-Build des Binaries `deadlock-brain`, Timer laufen
weiter; ein neuer Timer für Backtest und Build-Refresh je Patch kommt erst,
wenn der Backtest bestanden ist.

## Rahmen

- Fach-Repo: `/home/nathanael/repos/Deadlock-Brain`, Rust unter `rust/`,
  Toolchain über `~/.cargo/bin` (1.97), `/usr/bin/cargo` ist zu alt.
- Worktrees unter `~/.worktrees/deadlock-brain-<paket>`, Branch
  `feat/build-reasoner-<paket>`.
- Intent-Thread (Fable, Orchestrator): `33a32f58-476b-4a67-99cc-8f6c1e8f7001`.
- Modelle: Design und Reviews Opus 4.8, Bau Luna (gpt-5.6-luna), Vorcheck GLM.
- Secrets aus Infisical, DB-Zugriff über den bestehenden Weg des Repos.
