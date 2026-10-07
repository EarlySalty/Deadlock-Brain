status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/<repo>-brain-consumer-fertig

[Orchestrator] Paket K: Brain-Consumer produktiv

Zuerst `GEMEINSAM.md` in diesem Ordner lesen, sie gilt vollständig.

## Ziel

Alle Brain-Consumer lesen produktiv über den typisierten `BrainClient` aus dem Rust-Kern:

- **Deadlock-2nd-Brain PR #2** (`codex/fix-c9-consumer-wiring`, mergeable): abschließen, mergen, deployen.
- **Deadlock-Bots PR #459** (Draft, Konflikt) und **Deadlock-Docs PR #4**: Dort laufen gerade fremde Sol-Threads (`a99dc9e9…`, `c0b1d111…` in Deadlock-Bots, `14966995…` in Deadlock-Docs) auf demselben Branch. Solange deren Session-Status `running` ist (`python3 ~/Documents/tools/t3-thread.py read --thread <id>`), schreibst du dort nicht. Prüfe bei jeder Wache den Zustand. Sind sie gestoppt oder fertig und ist der PR noch offen, übernimmst du den Stand auf deinem eigenen Branch und schließt ab. Nicht in diese Threads schreiben, sie nicht stoppen oder settlen.
- Deadlock-Bots hat eine Auto-Merge-Automation für Nicht-Drafts: PRs nicht auf "ready" setzen. Merge läuft lokal über den Merge-Gate, nicht über GitHub.
- **Twitch #984** ist gemergt: prüfen, dass der Twitch-Bot live gegen den Rust-Kern liest; wenn nicht, aktivieren.

## Eigentum

Nur die Brain-Client-Anbindung in den vier Repos. Deploy je Repo über den bestehenden Weg (Twitch: Release-Wrapper `deploy-twitch-release`; dl-bot: Memory `dl-bot-build-und-deploy`). Twitch-Releases nur im eigenen Worktree bauen.

## Arbeitsstand

Je Repo Worktree `~/.worktrees/<repo>-brain-consumer-fertig`, Branch `feat/brain-consumer-fertig-20261003` von `origin/main`.

## Beweisziel

Je Consumer eine echte Anfrage im Live-Betrieb, die nachweislich über `brain-serve` beantwortet wurde (Log von `brain-serve` mit Consumer-Kennung), Dienste neu gestartet.

## Routing

Paket K, Versuch 1, Produzent `teil-k`. Rest siehe GEMEINSAM.md.
