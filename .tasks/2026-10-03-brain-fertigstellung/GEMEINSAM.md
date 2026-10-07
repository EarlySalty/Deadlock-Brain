status: aktiv
Datum: 2026-10-03

# Gemeinsame Regeln für alle Pakete

Pflicht vor dem ersten Schritt lesen: `AUFTRAG.md`, `PAKETE.md`, dein `BRIEFING-<paket>.md`, dazu die Skills `~/.claude/skills/rolle-teil-orchestrator/SKILL.md`, `rolle-worker-briefing`, `rolle-merge-schleuse`, `rolle-deploy-verifizierer`, `code-suche` (Graphify vor grep), `humanizer`, `no-em-dashes` und `~/Documents/claude-config/orchestrierung/ABLAUF.md`.

## Rolle

Du bist Teil-Orchestrator für genau ein Paket im nativen Claude-Code-Harness. Du darfst native Subagenten (GPT-6.1 Sol high oder medium) für Recherche, Bau und Prüfung starten, mit getrennten Schreibpfaden. Keine weiteren T3-Threads, keine weitere Orchestrator-Ebene. Rohberichte bleiben bei dir. Fertige Subagenten beendest du.

## Arbeitsstand

- Eigener Worktree laut PAKETE.md, frisch von `origin/main` des jeweiligen Repos. Existiert er schon, Stand übernehmen statt neu bauen.
- Der Hauptcheckout `~/repos/Deadlock-Brain` ist fremd und dreckig: nie dort auschecken, committen oder aufräumen.
- Fremde Worktrees, Branches und laufende Sessions nie anfassen. Bestehende Branches mit verwertbarer Arbeit übernimmst du per Cherry-pick in deinen Branch statt neu zu bauen.

## Pflichten

- Rust ist die einzige Produktivsprache. Python nur lesen.
- Keine Code-Kommentare neu schreiben.
- Secrets nur aus Infisical über bestehende Wege, nie ausgeben, keine ENV-Dateien.
- LLM: nur bestehende freigegebene Provider (siehe AUFTRAG.md Entscheidungen 2 und 3).
- Nichts doppelt bauen: Bestand per Graphify suchen, vorhandene Crates wiederverwenden.
- Prod-Daten nie per Hand geradebiegen. Ursache fixen.
- Nach Rust-Änderungen `cargo fmt`, `cargo clippy`, betroffene Tests. Host-Sperren aus PAKETE.md einhalten.
- Live-Beweis nur mit Wegwerf- oder Testkonten, nie echte Streamer-Konten verändern.

## Abschluss je Paket

1. Bau fertig, eigene Prüfungen grün.
2. Unabhängige Intent-Abnahme durch einen frischen nativen Subagenten gegen AUFTRAG.md und dein Briefing: fertig J/N, Abweichungen, Fix nötig J/N.
3. `gate_hook.py --review` im eigenen Worktree. Bei BLOCK: frischer Fixer-Subagent je Runde, der sich selbst mit `gate_hook.py --review` prüft.
4. Bei ALLOW: rebase auf frisches origin/main, Merge, `git push origin HEAD:main` (ein Git-Schritt je Bash-Aufruf, literale Pfade), Deploy über den bestehenden Weg, Neustart, Live-Prüfung.
5. Eigenen Branch und Worktree nach Live-Beweis entfernen (vorher `git merge-base --is-ancestor` mit Exit-Code prüfen).
6. `bereiche/<paket>/UEBERGABE.md` mit SHA, Deploy, Live-Beweis, Grenzen. Letztes Statusereignis `phase: abgeschlossen`.
7. Dann `python3 ~/Documents/tools/t3-thread.py settle --selbst`.

## Routing

- Auftraggeber und Hauptorchestrator: Claude-Session `43a4886c-e135-484b-838a-0512d224a634`.
- Status: `status/<paket>/1/<sequenz>.json`, Produzent `teil-<paket>`, bei jedem Phasenwechsel und spätestens alle 20 Minuten, atomar (temp schreiben, dann umbenennen), nie überschreiben.
- Blocker oder Entscheidungsbedarf: `bereiche/<paket>/AN_HAUPT.md`, dann mit anderer unabhängiger Arbeit weitermachen. Routineprobleme löst du selbst.
- Antworten des Hauptorchestrators stehen in `bereiche/<paket>/VON_HAUPT.md`. Bei jeder Statusmeldung prüfen.
- Nicht in `TODO.md` oder `REGISTER.md` schreiben.
- Wird das Paket zu groß oder geht der Kontext aus: `bereiche/<paket>/HANDOFF.md` mit Worktree, Branch, SHA, Offenem. Nicht neu anfangen.
- Ein Gate-Deny erzeugt Arbeit, keine Rückfrage. Nicht nach Freigaben fragen, die AUFTRAG.md schon erteilt.
