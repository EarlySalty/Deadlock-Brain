# Vorcheck: build-reasoner

[Orchestrator] Vorcheck, lesend, kein Code, kein Branch. Du bist der einzige
Thread für dieses Paket. Keine Unter-Threads oder Unter-Agenten spawnen.

## Auftrag an den Vorcheck

- Frage: Welche Bausteine im Repo und in der Datenbank kann ein neuer
  Build-Reasoner (Helden-Modell, Item-Modell, Mechanik-Rechnung, Patch-Delta,
  Meta-Signale, Composer, Backtest, Publish) wiederverwenden, und wo genau
  liegen sie? Hintergrund steht in
  `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/AUFTRAG.md`.
- Repo: `/home/nathanael/repos/Deadlock-Brain` (main). Für den Publish-Weg
  zusätzlich lesend `/home/nathanael/repos/Deadlock-Steam-Bot`
  (`rust/crates/steam-core/src/task/handlers/builds/`,
  `rust/crates/steam-persistence/src/builds.rs`).
- Werkzeug: erst `graphify query` im Repo-Root, dann die Fundstelle mit
  `sed -n` nachlesen. Für die DB: Zugang wie in `scripts/`, Secrets aus
  Infisical, nie ausgeben.

## Was ich konkret wissen will

1. Payload-Felder der Helden- und Item-Snapshots in `brain.entity_snapshots`
   (Quelle deadlock-data und Assets-API): je ein echtes Beispiel für Warden und
   für die Items Veil Walker, Mercurial Magnum, Siphon Bullets, Quicksilver
   Reload (Pfad zu Tier, Kosten, Slot, Stats, Aktiv/Passiv, Cooldown,
   Bedingungstext, Imbue). Wo stehen `purchase_bonuses`?
2. Welche Funktionen in `dbrain-learn/src/build_optimizer.rs` bauen den
   Helden-Kontext (Damage-Plan, Scaling, Archetypen), mit Signatur und Zeile;
   welche davon sind rein deterministisch.
3. `dbrain-builds`: Signaturen von `BuildSpecPayload`, `assemble_payload`,
   `sync_one_hero`, `upsert_hero_item_stats`; wie kommt ein Build bis zum
   Steam-Bot (Tabelle, Task, Handler).
4. `brain.patch_events` und `patch_event_enrichments`: Spalten, wie ein Event
   Held oder Item referenziert (entity_id, Alias), wie der Patch-Stand
   (Datum, Patch-Tag) erkennbar ist; die letzten drei Patches, die Warden
   betreffen, mit Zeilen.
5. `tierlist.hero_build_sources`, `watched_build_authors`, `meta_builds`,
   `meta_items`: Spalten, Zeilenzahl, ob Lightbringer als Autor drin ist, ob
   Item-Reihenfolge und Kategorien (Kern, Situational) gespeichert sind, ob
   ein Patch-Bezug je Build existiert.
6. `brain.hero_item_stats` und `hero_item_synergies`: Spalten, Aktualität
   (jüngster Zeitstempel), Stichproben.
7. `deadlock-brain-core/src/ai.rs`: Signatur des Fireworks-Aufrufs, wie
   Denken abgeschaltet wird, JSON-Ausgabe erzwingbar?
8. CLI-Struktur in `deadlock-brain/src/main.rs`: wie ein neues Top-Level
   Kommando `reason` mit Unterbefehlen dazukommt (Enum, Dispatch, Zeilen).
9. Tests: wie laufen die Crate-Tests (DB-Abhängigkeit, Env, sqlx offline),
   was ist heute rot (Baseline mit Zahlen).

## Antwortformat

Datei `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/VORCHECK-ERGEBNIS.md`
mit je Punkt: Fundstellen `pfad:zeile`, kurzer Befund, Risiko (Prod-DB, Auth,
fehlende Daten). Am Ende: geschätzte Zahl der betroffenen Stellen und Repos.
Fertigmeldung in diesem Thread mit dem Pfad. Deutsch, echte Umlaute, keine
Gedankenstriche.
