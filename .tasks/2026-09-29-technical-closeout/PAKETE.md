status: aktiv
Datum: 2026-09-29

# Arbeitspakete

Gemeinsamer Vertrag: AUFTRAG.md. Nur Orchestrator integriert nach unabhängigem Review.

| Paket | Modell | Repo/Worktree | Branch/Ziel | Scope und Nachweis | Abhängigkeit |
|---|---|---|---|---|---|
| V | Luna | Brain Koordinationsworktree | keine Codeänderung | Mini-Vorcheck, tatsächliches API-Repo, Fundstellen/Harness/Assets/Wiki/Provider/Replay, lokale laufende Arbeit | keine |
| A | Sol | Brain /home/nathanael/.worktrees/brain-pre-g5-finalize-20260929 | integration/pre-g5-finalize-20260929 -> migration/rust-integration | Vorhandene Match- und Analytics-Runtime-Arbeit vollständig verdrahten, validieren, exportieren, Unit-/Integrationstests. Kein Reset. Exklusiv brain-feeds, dbrain-sources und zugehörige Wiring-Änderungen, Cargo.lock. | keine |
| B | Sol | Bots /home/nathanael/.worktrees/bots-c9-consumer-wiring | codex/fix-c9-consumer-wiring -> main als Draft, NICHT mergen | Offenen Merge abschließen, C9-Vertrag und alle relevanten Tests, commit/push | keine |
| C | Sol | eigener Brain-Testworktree | fix/pre-g5-harness-20260929 -> migration/rust-integration | Secret-sicherer Test-Harness, E2E/Last-Runner, Fact-/Assets/Wiki-Regressionsnachweise; keine Source-Adapter ändern | V; finale Messung nach A |
| D | Luna | eigene Consumer-Worktrees | bestehende Docs #4 und 2nd #2, NICHT mergen | Docs ohne unnötigen Diff prüfen; 2nd offline suite, Billing unterscheiden; Twitch #984 nur Regression | keine |
| E | Luna | eigene Provider-Worktrees | nur bei Befund gezielter PR | Patchnotes #49 und Steam #73 plus Schema #461 verifizieren, Contract-Kompatibilität, Idempotenz/parallel/restart, kein echter Publish | keine |
| F | Orchestrator/Astra Review, Luna Doku später | Brain Koordinationsworktree | integration/technical-closeout-20260929 -> migration/rust-integration | PR-Inventar, alte PRs mit Beleg schließen, unabhängige Reviews und Integration, finale Statusdateien, G5-Sperre | A/B/C/D/E |

Paket C bekommt nach V genaue dateidisjunkte Grenzen. Finale Tests und Release-Build erst auf tatsächlich integriertem endgültigem Code-Head. Keine zwei schreibenden Worker pro Worktree. Keine globalen Formatierungsänderungen. Dauerhafte technische Blocker nicht als Betreiberentscheidung etikettieren.
