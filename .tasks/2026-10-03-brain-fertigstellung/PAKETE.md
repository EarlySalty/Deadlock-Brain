status: aktiv
Datum: 2026-10-03

# Pakete

Alle Pakete laufen als T3-Thread mit GPT-6.1 Sol im Claude-Code-Harness, Effort `ultracode`, Rolle Teil-Orchestrator mit nativen Subagenten (Sol high/medium). Statuskanal: `status/<paket>/<versuch>/<sequenz>.json` in diesem Ordner, Format nach `~/Documents/claude-config/orchestrierung/ABLAUF.md`. Übergabe je Paket nach `bereiche/<paket>/UEBERGABE.md`, Blocker nach `bereiche/<paket>/AN_HAUPT.md`.

| Paket | Bereich | Repo / Schreibpfad | Worktree | Branch | Abhängig von |
|---|---|---|---|---|---|
| P | Patchnotes-Bot komplett nach Rust, Deploy, Python-Unit aus | `~/repos/Deadlock--Patchnotes-Bot` (ganzes Repo); in Deadlock-Brain nur Patchnotes-Feed-Anbindung und `deadlock-brain-patchnotes-sync` | `~/.worktrees/patchnotes-rust-fertig` | `feat/patchnotes-rust-fertig-20261003` | Vertrag `brain.feed.patchnotes.v1` (liegt auf main) |
| S | Steam-Build-Publish produktiv | `~/repos/Deadlock-Steam-Bot`; in Brain nur `rust/crates/brain-feeds` Build-Publish-Teil | `~/.worktrees/steam-publish-fertig` | `feat/steam-publish-fertig-20261003` | keine |
| Q | Brain-Kern Restpunkte: Provider-Shadow (DeepSeek Flash Fireworks), Sheet- und YouTube-Kernanbindung, Relevanzschwelle Fakt-Profil, G0 SLO/Lastprofil | Deadlock-Brain `rust/` außer brain-feeds Patchnotes/Build-Publish und außer Replay-Decoder | `~/.worktrees/brain-fertig-q` | `feat/brain-fertig-q-20261003` | keine |
| R | Replay V1 mit echten Demos über berechtigte Wege | Deadlock-Brain Replay-Decoder und Reportpfad | `~/.worktrees/brain-fertig-r` | `feat/brain-fertig-r-20261003` | keine |
| K | Consumer produktiv: 2nd-Brain #2 mergen, Bots #459 und Docs #4 abschließen (sobald fremde Sol-Threads dort nicht mehr laufen), Twitch #984 aktivieren | Deadlock-2nd-Brain, Deadlock-Bots, Deadlock-Docs, Deadlock-Twitch-Bot (nur Brain-Client-Anbindung) | je Repo `~/.worktrees/<repo>-brain-consumer-fertig` | `feat/brain-consumer-fertig-20261003` | Q-Vertrag stabil |
| Z | Integration, Aufräumen, G5-Cutover, G6 Legacy-Ende, STATUS/GATES, Abschlussbericht | Deadlock-Brain `architecture/migration/`, `ops/`, Deploy, Units; Branch-/Worktree-Bereinigung | `~/.worktrees/brain-fertig-z` | `feat/brain-fertig-z-20261003` | P, S, Q, R, K, Wiki-Auftrag |
| T | Aufgabenstand (nur TODO.md aus Statusereignissen) | `TODO.md` dieses Ordners | keiner | keiner | alle |

## Produzenten je Paketversuch

| Paket | Versuch | Produzent | Thread | Gültig |
|---|---|---|---|---|
| P | 1 | teil-p | 09ce79b1-d5ed-4ae8-b77a-1fa54fdfcb1a | ja |
| S | 1 | teil-s | 6c7a66fd-e58b-4681-8960-4dd442662386 | historisch, Workflow unterbrochen, Hauptsession ready ohne Kinder |
| S | 2 | teil-s2 | c6ddac1c-c0d2-4bbc-b6f8-7bc462570a52 | ja, übernimmt erhaltene Worktrees |
| Q | 1 | teil-q | 80314ca4-8ef0-4006-8c4e-62b3bc5566a9 | ja |
| R | 1 | teil-r | e9e3df50-7001-4f55-8a64-350b0c2cbc12 | ja |
| K | 1 | teil-k | 27b6a744-a92e-4765-88c6-2c70a2b0fcd8 | ja |
| Z | 1 | teil-z | 97ebad62-f120-4109-8f7b-f15fd0cea057 | historisch, Versuch tot |
| Z | 2 | teil-z | b73d9271-6c39-4c58-b831-3399fc6dceb6 | ja |

## Schnittstellen

- Die Übergabe an Z erfolgt als `uebergeben` mit lokal geprüften SHAs. Z integriert diese Stände gemeinsam; deren erst nach Deployment mögliche Live-Nachweise sind keine Voraussetzung für den Beginn der Integration. Nach gemeinsamem G5-Deployment führen P/S/Q/K ihre Live-Prüfungen aus und melden anschließend den tatsächlichen Abschluss. Die ursprüngliche Forderung nach bereits abgeschlossenen Fachpaketen vor Zs Integration ist damit ersetzt.

- Q, R und S ändern in Deadlock-Brain disjunkte Crates. Wer doch einen gemeinsamen Pfad (Workspace-`Cargo.toml`, `Cargo.lock`, Migrationen) braucht, meldet das in `AN_HAUPT.md`; Migrationsnummern erst nach frischem `git fetch origin main` vergeben.
- Seit Übernahme um 15:39 UTC: Gekoppelte Kern-, Writer- und Consumerpfade werden gemeinsam durch Z integriert und unabhängig abgenommen, danach folgt das Bug- und Security-Gate für denselben Stand. Kein einzelner Paketmerge ersetzt diese gemeinsame Prüfung. Disjunkte Änderungen brauchen weiterhin Abnahme und Gate ALLOW. Z besitzt die gemeinsame Releaseinstallation und Umschaltung; S liefert Publish-Code und Publish-Nachweis, P/Q ihre Writer und Betriebsverträge, K die Consumer.
- Der Wiki-Auftrag `2026-10-03-wiki-spielwissen` gehört Codex /root. Z liest nur dessen `ENDE.md`/`ABSCHLUSSBERICHT.md` und origin/main.
- Host-Sperren: `host-checks.lock` dann `/tmp/deadlock-cargo-release.lock` nach `Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md`; höchstens ein Cargo-Release-Build gleichzeitig auf dem Host.
