# Review-Briefing: build-reasoner (Paket D, Runde 2)

[Orchestrator] Review Runde 2 für Paket D, nur gegen die Mängelliste. Lesend,
kein Code, kein Branch. Du bist der einzige Thread für dieses Review. Keine
Unter-Threads oder Unter-Agenten spawnen.

- Worktree: `/home/nathanael/.worktrees/deadlock-brain-d` (Branch
  `feat/build-reasoner-d`, Code-Commit e0e1cff, Doku 54252b0, Basis 7fbb128)
- Diff der Fixrunde: `git -C /home/nathanael/.worktrees/deadlock-brain-d diff 7fbb128..HEAD`
- Mängelliste und Fixbericht: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/REVIEW-D.md`
  (Anhang "Fixrunde 1" ab Zeile 168), Fix-Briefing `FIX-BRIEFING-D.md`,
  Detailbericht im Worktree `.tasks/2026-09-12-build-reasoner/FIXRUNDE-1-D.md`
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`

## Was du prüfst

Nur die Mängel 1 bis 5 aus REVIEW-D.md: je Mangel behoben ja/nein mit
Datei:Zeile, Regressionstest vorhanden, Nebenwirkungen. Besonders:

- Mangel 1: die Upserts in `lib.rs:249` schreiben `reasoner_builds`,
  `reasoner_item_scores` (alle Score-Komponenten), `reasoner_patch_deltas`,
  `reasoner_backtests` transaktional und idempotent je Held und Patch-Tag;
  ohne `--publish` bleibt der Steam-Bot unberührt; die Migration passt zu den
  Upserts (Spalten, Unique-Keys, nullable `order_proximity`); ein zweiter
  Lauf überschreibt statt zu verdoppeln.
- Mangel 2: `dbrain-builds/src/patch_tag.rs` wird von Sync und Fassade
  wirklich beide genutzt, keine zweite Herleitung übrig.
- Mangel 3: numerische Revisionswahl im Resolver, Test 0731 gegen 1015.
- Mangel 4: Fallback-Skill-Order mit Punkt-Typ und Delta; der Fixer hat die
  Werte an echten Warden-Autorendaten abgeglichen (Freischalten Typ 2 ein
  Punkt, Upgrades Typ 1 mit 1, 2, 5), prüfe das gegen einen echten
  `abilityOrder` in `tierlist.hero_build_sources` (read-only).
- Die Migration wurde nach dem ersten Entwurf geändert; sie ist noch nirgends
  angewendet, also erlaubt, aber prüfe, dass sie auf einem leeren Schema und
  auf einem Schema mit dem ersten Entwurf sauber läuft (der Fixer meldet
  einen Upgrade-Test in der Wegwerf-DB).

Tests selbst laufen lassen und Zahlen nennen: `cargo test --workspace` ohne
DSN (Fixer: 256 bestanden, 53 ignoriert), `cargo test -p dbrain-reasoner --
--include-ignored` mit Central-DSN (Fixer: 83 mit Scratch-DSN; ohne Scratch
sind die Schreibtests ignoriert oder rot, nenne die Zahl), Clippy `-D
warnings`.

## Ergebnis

Anhang in `REVIEW-D.md` unter "Review Runde 2": je Mangel behoben ja/nein mit
Begründung, neue Befunde nur aus dem Fix. Urteil: FREIGABE oder NACHBESSERN,
plus die endgültige Deploy-Reihenfolge. Fertigmeldung in diesem Thread mit
Urteil. Deutsch, echte Umlaute, keine Gedankenstriche.
