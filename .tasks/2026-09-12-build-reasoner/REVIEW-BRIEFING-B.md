# Review-Briefing: build-reasoner (Paket B, Runde 1)

[Orchestrator] Review Runde 1 für Paket B. Lesend, kein Code, kein Branch.
Du bist der einzige Thread für dieses Review. Keine Unter-Threads oder
Unter-Agenten spawnen.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-b` (Branch
  `feat/build-reasoner-b`, Commit 68dc58c, Basis 3bef602 = Paket A)
- Diff: `git -C /home/nathanael/.worktrees/deadlock-brain-b diff 3bef602..HEAD`
- Auftrag: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/AUFTRAG.md`,
  Spec `ARCHITEKTUR.md` (Abschnitte 5, 7, 9, 13) und vor allem `MECHANIK.md`
  (alle Rechenregeln), Worker-Briefing `BRIEFING-B.md`, Fertigmeldung
  `FERTIG-B.md`, Paketschnitt-Entscheidung in `REVIEW-A.md` (Mangel 1) und
  `PAKETE.md`, Skill-Regeln `~/.claude/skills/deadlock-build-bauen/SKILL.md`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`

## Was du prüfst

1. Rechenregeln gegen MECHANIK.md, Regel für Regel: Kaufbonus je Slot und
   Tier aus `purchase_bonuses`, Tickrate gegen Proc-Cooldown, bedingte Items
   mit Abwertungsfaktor je `ConditionKind`, Aktiv- gegen Passivwert,
   Slot-Knappheit gegen Soul-Budget, Soul-Kurve und Phasen, Nebenwaffe,
   Imbue-Zuordnung, Kampffenster-Metrik. Jede Abweichung von der Formel mit
   Zeile; jede Formel ohne Unit-Test mit Zahlen aus der Spec benennen.
2. Paketschnitt: `hero.rs` nimmt das geladene `HeroModel` aus `data.rs`
   entgegen und ergänzt nur Skalierungsstufe, Damage-Plan-Verfeinerung und
   Waffenprofil; keine Doppelung der Payload-Auswertung aus `data.rs`. Die
   Signaturänderung gegenüber ARCHITEKTUR.md Abschnitt 7 (geladenes Modell
   statt rohem Payload) ist entschieden, prüfe nur, ob Paket C und die
   Fassade damit arbeiten können.
3. Scoring in `item.rs`: deterministisch, endgültige Zahlen, Meta nur als
   Nebensignal über `MetaIndex`, kein starrer Pfad spirit/weapon/tank. Warden
   muss als Waffen-Kern mit Spirit-Nebenwirkung herauskommen. Fahre den
   Echtdaten-Test selbst mit `DEADLOCK_CENTRAL_DSN` (Zugang wie in
   `scripts/`, Secrets aus Infisical, nie ausgeben, Verbindung read-only) und
   nenne die Scores der vier Referenz-Items Veil Walker, Mercurial Magnum,
   Siphon Bullets, Quicksilver Reload sowie die fünf besten und fünf
   schlechtesten Items für Warden. Liegen die vier Referenz-Items nicht über
   der Kern-Schwelle, ist das ein blockierender Befund mit Ursachenanalyse
   (welche Regel drückt sie runter).
4. Hygiene: keine Secrets, keine Code-Kommentare, keine neuen Crates, keine
   KI-Aufrufe; `cargo fmt`, `cargo clippy -p dbrain-reasoner --all-targets
   -- -D warnings`, `cargo test -p dbrain-reasoner` (mit temporärer
   Moduldeklaration in `lib.rs`, nicht committen) und Zahlen nennen (Worker:
   20 bestanden, 2 ignoriert).

## Ergebnis

Datei `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-B.md`:
vollständige Mängelliste, je Mangel Nummer, Datei:Zeile, Befund, Schwere
(blockierend, wichtig, nit), Vorschlag; dazu die Warden-Zahlen aus Punkt 3.
Am Ende ein Urteil: FREIGABE oder NACHBESSERN, plus die Punkte, die Paket D
bei der Integration wissen muss. Fertigmeldung in diesem Thread mit dem Pfad
und dem Urteil. Deutsch, echte Umlaute, keine Gedankenstriche.
