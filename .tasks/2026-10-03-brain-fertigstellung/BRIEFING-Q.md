status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

[Orchestrator] Paket Q: Brain-Kern Restpunkte vor G5

Zuerst `GEMEINSAM.md` in diesem Ordner lesen, sie gilt vollständig.

## Ziel

Alle Kernpunkte schließen, die laut `architecture/migration/STATUS.md`, `GATES.csv`, `PRE_G5_TECHNICAL_REVIEW.md` und `ANFORDERUNGSSTATUS.csv` (60 Anforderungen ohne Status) G2 und G3 auf "teilweise" halten, soweit sie nicht P, S, R, K oder dem Wiki-Auftrag gehören:

1. **Provider-Shadow:** Antwortpfad über den freigegebenen Provider (neuestes stabiles DeepSeek Flash bei Fireworks, Modellname aus Config, Timeout als Konfigfeld mit großzügigem Default, Denken aus bei kleinem Budget). Shadow-Lauf gegen echte Fragen, Vergleich mit Bestandspfad, Ergebnis nach `architecture/migration/evals/`. `PROVIDER_SHADOW_PASSED` belegt setzen.
2. **Sheet- und YouTube-Kernanbindung:** `deadlock-brain-sheet-sync` und `deadlock-brain-youtube-learning` schreiben über den Rust-Kern (`SourceRecordV2`, normaler Store) statt Legacy.
3. **Relevanzschwelle im Fakt-Profil** fertig und belegt.
4. **G0:** SLO und Lastprofil festlegen und messen (aus echtem Verkehr der Consumer), Rechteinventar vervollständigen.
5. `ANFORDERUNGSSTATUS.csv`: jede Anforderung mit Status und Nachweis füllen. Was zu P/S/R/K/Wiki gehört, mit Verweis aufs Paket.

## Eigentum

Deadlock-Brain `rust/` und `architecture/migration/evals/`, `ANFORDERUNGSSTATUS.csv`. Nicht: Patchnotes-Feed (P), Build-Publish (S), Replay-Decoder (R), `STATUS.md`/`GATES.csv` (Z). Gemeinsame Dateien (`Cargo.toml` Workspace, `Cargo.lock`, Migrationen) nur nach Ankündigung in `AN_HAUPT.md`.

## Arbeitsstand

Worktree `~/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003` von `origin/main`. Brain-Postgres: eigene Instanz Port 5446, Socket `/run/deadlock-brain-postgresql`, Gruppe `deadlock-brain-db` (Memory `brain-rust-integration-20260926`). Cargo mit Toolchain 1.97.1.

## Beweisziel

Shadow-Ergebnis mit Zahlen, Sheet- und YouTube-Lauf schreiben echte Datensätze in den neuen Store (Abfrage belegt), Relevanzschwelle mit Testfällen, G0-Messwerte. Nach Merge: betroffene Units (`brain-serve`, Sheet-, YouTube-Timer) neu gestartet und ein Lauf sauber.

## Routing

Paket Q, Versuch 1, Produzent `teil-q`. Rest siehe GEMEINSAM.md.
