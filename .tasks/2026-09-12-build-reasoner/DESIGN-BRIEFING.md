# Briefing: build-reasoner (Design, Paket P0)

[Orchestrator] Paket P0 Design. Auftrag:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/AUFTRAG.md`
(vollständig lesen). Du bist Architekt, nicht Bauherr: du schreibst nur die zwei
Dateien unten, keinen Code, kein Branch, kein Commit.

- Repo: `/home/nathanael/repos/Deadlock-Brain` (lesend, main)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Was du lieferst

1. `.tasks/2026-09-12-build-reasoner/ARCHITEKTUR.md`
   - Neues Crate `rust/crates/dbrain-reasoner` (Arbeitsname, du darfst einen
     besseren vorschlagen), Module und ihre Schnittstellen als Rust-Signaturen
     (Structs, Enums, Traits, pub fn), so präzise, dass drei Worker parallel
     bauen können, ohne sich zu widersprechen.
   - Datenfluss: welche DB-Tabellen und Snapshots jedes Modul liest, was es
     schreibt (neue Tabellen mit DDL-Entwurf im Schema `brain`).
   - Agentenrollen (Hero-Analyst, Item-Analyst, Patch-Analyst, Meta-Analyst,
     Composer, Kritiker): welche deterministisch in Rust sind, welche einen
     Deepseek-Aufruf über `deadlock-brain-core::ai` bekommen, mit welchem
     Ein- und Ausgabeformat (JSON-Schema), und wie sie zusammenarbeiten
     (Reihenfolge, Übergabeobjekte, Abbruchregeln). KI liefert nie Zahlen.
   - Build-Objekt: Kern, Situationsblöcke, Reihenfolge, Warum je Item,
     Konfidenz je Item, Quellenbelege; Abbildung auf den bestehenden
     Publish-Payload in `dbrain-builds/src/spec.rs`.
   - Backtest: Kennzahlen (Kern-Überdeckung, Reihenfolge-Nähe,
     Patch-Wechsel-Erkennung), Vergleichsbasis aus `tierlist.hero_build_sources`
     und `watched_build_authors` je Patch-Stand, Pflichtfall Warden.
   - CLI-Befehle unter `deadlock-brain reason ...`.
   - Paketschnitt: A, B, C (mehr nur, wenn nötig), disjunkt nach Dateien, mit
     Abhängigkeiten und Startreihenfolge. Paket A legt Crate, `lib.rs`,
     `types.rs` und Cargo-Einträge an; B und C fassen nur eigene Moduldateien
     an, die Delegator-Integration trägt die `mod`-Zeilen nach.
   - Was aus `build_optimizer.rs`, `dbrain-builds`, `dbrain-enrich`
     wiederverwendet wird und was ersetzt wird (mit Begründung).

2. `.tasks/2026-09-12-build-reasoner/MECHANIK.md`
   - Die Rechenregeln, die der Code umsetzen muss, jede als Formel mit
     Datenquelle (Feldname im Snapshot-Payload): Kaufbonus je Slot und Tier
     (`purchase_bonuses`), Tickrate gegen Proc-Cooldown, bedingte Items
     (aktiv mit Cooldown, handlungsgebunden, aufladend, zustandsgebunden) mit
     Abwertungsfaktor, Aktiv- gegen Passivwert, `disabled`/`shopable`,
     Slot-Knappheit gegen Soul-Budget, Nebenwaffe, Imbue-Zuordnung,
     Soul-Kurve und Kaufzeitpunkte (Lane, Kern, Spätspiel).
   - Helden-Modell: Damage-Plan, Skalierungsstats, Fähigkeitsrollen,
     Waffenprofil, aus welchen Payload-Feldern.
   - Patch-Delta: wie ein Patch-Event zu einer Verschiebung im Helden- oder
     Item-Modell wird (Vorzeichen, Größe, betroffene Mechanik), und wie daraus
     "Warden ist wieder Meta" ableitbar wird.
   - Meta-Signale: Gewichtung von deadlock-api Winrate/Verbreitung,
     Top-Autoren-Builds, Creator-Claims; Mindeststichproben.
   - Grenzen: was das Modell bewusst nicht rechnet, und was der Backtest
     dann zeigen muss.
   - Prüfe den Screenshot `referenz/lightbringer-warden.png` und lege dar,
     welche Regeln nötig sind, damit der Algo diesen Kern findet.

## Regeln

- Grundlage sind die echten Daten: lies Snapshots und Tabellen aus der
  zentralen Postgres (Zugang wie in den Skripten des Repos, Secrets aus
  Infisical, nie ausgeben) und die Skill-Datei
  `~/.claude/skills/deadlock-build-bauen/SKILL.md`. Keine erfundenen Feldnamen.
- Wo Daten fehlen (z. B. Kaufzeitpunkte der Autoren-Builds), sag es klar und
  schlage die Quelle vor, statt es zu überbrücken.
- Keine Code-Kommentare in den Signaturen. Deutsch mit echten Umlauten,
  keine Gedankenstriche. Englische Spielnamen bleiben englisch.
- Fertigmeldung in diesem Thread: Pfade der beiden Dateien, offene Fragen als
  nummerierte Liste mit deiner Empfehlung je Frage.
