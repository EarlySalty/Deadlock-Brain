# Review-Briefing: build-reasoner (Paket A, Runde 1)

[Orchestrator] Review Runde 1 für Paket A. Lesend, kein Code, kein Branch.
Du bist der einzige Thread für dieses Review. Keine Unter-Threads oder
Unter-Agenten spawnen.

- Repo: `/home/nathanael/repos/Deadlock-Brain` (main, e7f86c2) und Worktree
  `/home/nathanael/.worktrees/deadlock-brain-a` (Branch
  `feat/build-reasoner-a`, Commit 3bef602, Basis 29bb842)
- Diff: `git -C /home/nathanael/.worktrees/deadlock-brain-a diff 29bb842..HEAD`
- Auftrag: `.tasks/2026-09-12-build-reasoner/AUFTRAG.md`, Spec
  `ARCHITEKTUR.md` (Abschnitte 1, 2, 5, 6, 9), `MECHANIK.md`, Briefing des
  Workers `BRIEFING-A.md`, Vorcheck `VORCHECK-ERGEBNIS.md`, Entscheidungen
  in `PAKETE.md`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`

## Was du prüfst

1. Spec-Treue: entsprechen `types.rs` (Namen, Felder, Enums, Signaturen)
   und die `pub fn` in `data.rs` und `ai_roles.rs` exakt ARCHITEKTUR.md
   Abschnitt 5, 6 und 9? Jede Abweichung mit Zeile und Folge für B und C,
   die gegen diese Typen bauen. Abweichungen, die der Worker in seiner
   Fertigmeldung begründet hat, bewertest du inhaltlich, nicht formal.
2. Ladeschicht gegen echte Daten: liest `data.rs` die Felder, die der
   Vorcheck belegt hat (Helden-Payload `purchase_bonuses`, `cost_bonuses`,
   `scaling_stats`; Item aus `item_or_ability` plus `item_card`, Kosten aus
   Snapshots, `item_catalog.item_id` als GC-ID; `patch_events` per
   entity_name plus Alias, `hero_item_stats` leer statt Abbruch)? Prüfe die
   SQL-Texte gegen das echte Schema (DB-Zugang wie in `scripts/`, Secrets
   aus Infisical, nie ausgeben) und einen echten Warden-Ladelauf, wenn ein
   Test oder Beispiel dafür existiert.
3. KI-Rollen: JSON-Schemas je Rolle wie Abschnitt 9, Parser verwerfen
   Items, die nicht im Build sind, KI liefert nie Zahlen, `--no-ai` als
   Typ-Schalter, `response_format` und `reasoning_effort` gesetzt, kein
   Modellname im Code, bestehende Aufrufer unverändert (Felder None).
4. Sicherheit und Hygiene: keine Secrets, keine Code-Kommentare, keine
   neuen externen Crates, sqlx-Offline-Cache falls compile-geprüfte Queries
   dazukamen, `cargo fmt`, `cargo clippy -p dbrain-reasoner`, `cargo test
   -p dbrain-reasoner` selbst laufen lassen und Zahlen nennen (Worker
   meldet 7 bestanden, 1 ignoriert).
5. Testtiefe: reichen die Tests für ein Fundament, auf dem zwei Pakete
   parallel bauen? Fehlende Fixtures oder Roundtrips benennen.

## Ergebnis

Datei `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-A.md`:
vollständige Mängelliste, je Mangel Nummer, Datei:Zeile, Befund, Schwere
(blockierend, wichtig, nit), Vorschlag. Am Ende ein Urteil: FREIGABE oder
NACHBESSERN, plus die Punkte, die B und C sofort wissen müssen. Fertigmeldung
in diesem Thread mit dem Pfad und dem Urteil. Deutsch, echte Umlaute, keine
Gedankenstriche.
