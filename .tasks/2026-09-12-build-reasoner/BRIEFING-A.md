# Briefing: build-reasoner (Paket A, Fundament)

[Orchestrator] Paket A. Auftrag:
`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/AUFTRAG.md`
(vollständig lesen), verbindliche Spec `ARCHITEKTUR.md` und `MECHANIK.md` im
selben Ordner, Paketschnitt in `PAKETE.md`, Fundstellen in
`VORCHECK-ERGEBNIS.md` (falls schon vorhanden, sonst ohne).

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-a` (existiert, Branch
  ist ausgecheckt)
- Branch: `feat/build-reasoner-a`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder
  Unter-Agenten spawnen.

## Was du baust

Das Crate `rust/crates/dbrain-reasoner` als Fundament nach ARCHITEKTUR.md
Abschnitte 1, 2, 5, 6, 9 (nur der AI-Transport-Teil):

1. `rust/Cargo.toml`: Workspace-Mitglied eintragen. `Cargo.toml` des Crates mit
   den in Abschnitt 1 genannten Abhängigkeiten, keine neuen externen Crates
   außer denen, die der Workspace schon kennt.
2. `src/types.rs`: alle geteilten Typen exakt nach Abschnitt 5 (Namen, Felder,
   Enums, Signaturen). Wo die Spec eine Lücke hat, entscheide sparsam und
   dokumentiere die Entscheidung in der Fertigmeldung, nicht im Code.
3. `src/data.rs`: Ladeschicht nach Abschnitt 6. Liest `brain.*`-Tabellen und
   `entity_snapshots`-Payloads, gibt A-Typen zurück. Verifiziere vorher die
   Spalten von `brain.hero_stat_values` und `brain.hero_item_synergies` am
   echten Schema (Entscheidung 2 in PAKETE.md). DB-Zugang wie in `scripts/`
   des Repos, Secrets aus Infisical, nie ausgeben, nie in Dateien schreiben.
4. `src/ai_roles.rs`: Wrapper um `deadlock_brain_core::ai::AiClient`,
   Request-Builder und Response-Parser für die fünf Rollen aus Abschnitt 9 mit
   den dort festgelegten JSON-Schemas. Denken abschalten wie bei den
   bestehenden Judge-Aufrufen im Repo. Kein Modellname im Code außerhalb der
   bestehenden Config. `--no-ai` als Schalter im Typ, nicht als Env.
5. `src/lib.rs`: nur `mod`-Zeilen und `pub use` für A. Module von B und C
   noch nicht eintragen.
6. Tests: Typen-Serialisierung, Parser der AI-Antworten (Fixtures ohne
   Netz), Ladeschicht gegen einen echten Snapshot für Warden und die vier
   Items Veil Walker, Mercurial Magnum, Siphon Bullets, Quicksilver Reload.

## Regeln

- Nur die Dateien aus Paket A anfassen. `mechanics.rs`, `hero.rs`, `item.rs`,
  `patch.rs`, `meta.rs`, `composer.rs`, `backtest.rs`, `publish.rs`,
  `main.rs` gehören anderen Paketen.
- Keine Code-Kommentare. Bestehende Kommentare in angefassten Dateien nicht
  erweitern.
- Toolchain: `export PATH=/home/nathanael/.cargo/bin:$PATH`, im Verzeichnis
  `rust/`. Nach jeder Änderung `cargo fmt` (nur eigene Dateien), `cargo clippy
  -p dbrain-reasoner`, `cargo test -p dbrain-reasoner`. Rote Tests, die schon
  vor dir rot waren, als Baseline mit Zahlen melden, nicht reparieren.
- Höchstens ein Release-Build gleichzeitig auf dem Host: keinen `--release`
  Build starten, Debug reicht.
- Nur den eigenen Branch committen und pushen (`git push origin
  feat/build-reasoner-a`), nie nach main. Commit-Trailer:
  `Co-authored-by: GPT 5.6 Luna <luna@local>`.
- Echte Umlaute in allem, was ein Nutzer sieht. Englische Spielnamen bleiben
  englisch. Keine Gedankenstriche.

## Bump-up

Wird das Paket größer als beschrieben oder widerspricht die Spec den echten
Daten, nicht weiterbauen. Nachricht in diesen Thread, dann stoppen:

```
[Bump-up] Paket A: Grund: ... Erledigt: ... Worktree: /home/nathanael/.worktrees/deadlock-brain-a Offen: ...
```

## Fertigmeldung

In diesem Thread: Branch, Commit-SHAs, geänderte Dateien, Testlauf mit Zahlen
(Baseline und Endstand), Entscheidungen bei Spec-Lücken als nummerierte Liste,
offene Punkte für B und C.
