status: aktiv
Datum: 2026-10-03

# Register

## Aktuelle Hauptorchestrierung

Seit 2026-10-03, 15:39 UTC: Codex /root, T3 `e6c19079-657e-4db9-80bd-8e1313e7f785`, ausdrückliche Nutzerübernahme. Details in `UEBERNAHME-CODEX.md`. Die historischen Ersteller bleiben unverändert; die bestehenden Fachthreads werden nicht als selbst gestartete Threads ausgegeben. T v2 meldet `stopped`, sein TODO-Stand ist 14:48 UTC und wird durch eine neue Statusrolle fortgeführt.

Eigene neue native Agenten: `/root/betriebsabgleich` prüft ausschließlich lesend Prozess-, Worktree- und Releasebelege. `/root/aufgabenstand` übernimmt T Versuch 3 als alleiniger TODO-Schreiber, schmaler Kontext ohne Fachberichte, Aktualisierung auf Anforderung. Beide wurden durch diese Hauptsession gestartet und bleiben auf ihren Auftrag begrenzt.

Eigener T3-Start S2 um 15:44 UTC: `c6ddac1c-c0d2-4bbc-b6f8-7bc462570a52`, Ersteller `e6c19079-657e-4db9-80bd-8e1313e7f785`, claudeAgent, gpt-6.1-sol / ultracode. Startkommando von T3 angenommen, erster Arbeitsschritt noch zu prüfen. Vorhandene S-Worktrees und Branches unverändert übernommen, Briefing `BRIEFING-S2.md`, Produzent `teil-s2`, Versuch 2. S1 bleibt unangetastet; sein Prozess hatte vor Übergabe in zwei Proben keine Kinder oder Prüfer.

S2-Startbeleg: Claude-Session `36b5f0c0-6349-4454-8cf4-c20d7a9e8fa0`, empfangene Assistanten-/Werkzeugereignisse um 15:45 und 15:46 UTC im eigenen Sessionjournal; T3 running. Kein bloß angenommener Start mehr. Vollständiges Startstatusereignis noch ausstehend. Betriebsabgleich mit beiden eng beauftragten Vorchecks abgeschlossen; TODO wurde durch T3 aus 29 Ereignissen aktualisiert, weitere Pflege auf Anforderung.

Hauptorchestrator: Claude-Session `43a4886c-e135-484b-838a-0512d224a634` (T3, Projekt Documents). Gestartet 2026-10-03 ca. 14:08 UTC über `t3-harness new --model sol` (GPT-6.1 Sol, Claude-Code-Harness).

## Session-Register

| Paket | Thread-ID | Ersteller | Harness | Modell/Effort | Status | Worktree | Branch | Basis-HEAD |
|---|---|---|---|---|---|---|---|---|
| P | 09ce79b1-d5ed-4ae8-b77a-1fa54fdfcb1a | 43a4886c | claudeAgent | gpt-6.1-sol / ultracode | gestartet | ~/.worktrees/patchnotes-rust-fertig | feat/patchnotes-rust-fertig-20261003 | 859852c |
| S | 6c7a66fd-e58b-4681-8960-4dd442662386 | 43a4886c | claudeAgent | gpt-6.1-sol / ultracode | gestartet | ~/.worktrees/steam-publish-fertig, ~/.worktrees/brain-fertig-s | feat/steam-publish-fertig-20261003, feat/brain-fertig-s-20261003 | 4c56217, 511a347 |
| Q | 80314ca4-8ef0-4006-8c4e-62b3bc5566a9 | 43a4886c | claudeAgent | gpt-6.1-sol / ultracode | gestartet | ~/.worktrees/brain-fertig-q | feat/brain-fertig-q-20261003 | 511a347 |
| R | e9e3df50-7001-4f55-8a64-350b0c2cbc12 | 43a4886c | claudeAgent | gpt-6.1-sol / ultracode | gestartet | ~/.worktrees/brain-fertig-r | feat/brain-fertig-r-20261003 | 511a347 |
| K | 27b6a744-a92e-4765-88c6-2c70a2b0fcd8 | 43a4886c | claudeAgent | gpt-6.1-sol / ultracode | gestartet | ~/.worktrees/Deadlock-2nd-Brain-brain-consumer-fertig (+ weitere je Repo) | feat/brain-consumer-fertig-20261003 | 4422997 |
| Z | 97ebad62-f120-4109-8f7b-f15fd0cea057 | 43a4886c | claudeAgent | gpt-6.1-sol / ultracode | tot (403 ca. 14:15 UTC), Prozess beendet, gesettelt, nicht wieder aufnehmen |
| Z v2 | b73d9271-6c39-4c58-b831-3399fc6dceb6 | 43a4886c | claudeAgent | gpt-6.1-sol / ultracode | gestartet ca. 14:22 UTC, Status unter status/z/2/ | ~/.worktrees/brain-fertig-z | feat/brain-fertig-z-20261003 | 511a347 |
| T | 24b41dbd-9f6f-4192-845a-222808a1c7f3 | 43a4886c | claudeAgent | gpt-6.1-sol / medium | tot (403 Auth beim Start), nicht wieder aufnehmen | keiner | keiner | |
| T v2 | 0504fb33-1a18-44f9-8efb-369e756204b1 | 43a4886c | claudeAgent | gpt-6.1-sol / high | gestartet ca. 14:35 UTC | keiner | keiner | |

## Fremd, nicht anfassen

Wiki-Auftrag `2026-10-03-wiki-spielwissen` (Codex /root, Sessions A/B2/C3/D/S2), Deadlock-Bots `a99dc9e9…`, `c0b1d111…`, Deadlock-Docs `14966995…`, Patchnotes `0ab3e0f8…`.
