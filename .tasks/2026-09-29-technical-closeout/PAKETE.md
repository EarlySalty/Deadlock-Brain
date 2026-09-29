status: erledigt
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
| F1 | Luna | /home/nathanael/.worktrees/brain-pre-g5-docs-20260929 | docs/pre-g5-closeout-20260929 | Alte PRs einzeln vergleichen, später finale Doku nach Belegen | keine für Audit |
| G | Luna | /home/nathanael/.worktrees/brain-pre-g5-dependencies-20260929 | fix/pre-g5-locked-dependencies-20260929 -> migration/rust-integration | Neu belegten haste_core-404 ohne Pin-/Policy-Abschwächung beheben, Herkunft und Lizenz prüfen | keine |
| F | Orchestrator/Astra Review, Luna Doku später | Brain Koordinationsworktree | integration/technical-closeout-20260929 -> migration/rust-integration | PR-Inventar, alte PRs mit Beleg schließen, unabhängige Reviews und Integration, finale Statusdateien, G5-Sperre | A/B/C/D/E/G |

Paket C bekommt nach V genaue dateidisjunkte Grenzen. Finale Tests und Release-Build erst auf tatsächlich integriertem endgültigem Code-Head. Keine zwei schreibenden Worker pro Worktree. Keine globalen Formatierungsänderungen. Dauerhafte technische Blocker nicht als Betreiberentscheidung etikettieren.

## Nach unabhängiger Reviewrunde 1 (historischer Verteilungsstand)

Aktueller Abschlussstand: A12/A34/C1 in022f8a9 integriert. FINAL ist auf genau diesem Codehead bestanden, F2-Dokumentation nach Korrektur unabhängig abgenommen. Schlussabnahme7eb844d: fertig J, Fix nötig N; lokales Abschlussgate ALLOW. B/D bleiben technisch abgenommen und ungemergt. E ist als Steam82 ohne Deployment integriert. PR57 enthält ausschließlich Nachweise und Abschlussdokumentation. Externe und Betreibergrenzen stehen in ABSCHLUSS.md.

- B/D: technisches GO in REVIEW-BD.md, Consumer-PRs bleiben ungemergt. E: Diagnosefix nach E-R1 korrigiert und unabhängig mit drei Gegenproben geprüft; Steam #82 bleibt Draft, Review in REVIEW-E.md.
- A12/Luna, eigener Match-Fixworktree auf 34a2507: R-AC-A1/A2, korrekte Spielerprojektion und strikter Match-Inhaltsvertrag. Exklusiv Matchadapter, Match-CLI und Match-Fixtures; kein Analytics-/Serve-Code.
- A34/Sol, bisheriger A-Worktree und PR #59: R-AC-A3/A4, tatsächliche fachliche Meta-/Population-Anbindung und gemeinsame Deadline. Keine A12-Dateien anfassen. Kleine Fixcommits aus A12 übernimmt der Orchestrator später in A.
- C1/Luna übernimmt den abgegebenen C-Worktree auf e671c5b und PR #60: R-AC-C1 ergänzt fehlende ursprüngliche und neue Prozess-Abnahmefälle. C/Sol ist beendet, keine parallelen Schreiber in diesem Baum.
- R-AC/Astra hat Runde 1 als fde7d5d beendet. Vier A-Befunde und eine C-Abnahmelücke sind bestätigt. Der positive Vorablastlauf auf f557064 (je 600/600, Peak 4) ist keine finale Messung und überschreibt den früheren roten C-Lauf nicht.
- G bleibt extern blockiert: exakte Git-Quellen nicht öffentlich erreichbar, dungers ohne belegte Lizenz. Keine Pins geändert und kein fremder Code ungeklärt weiterverteilt.
