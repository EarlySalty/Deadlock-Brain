status: aktiv
Stand: 2026-10-01

# Brain globale TOML-Konfiguration fertigstellen

## Auftrag

Den typisierten Vertrag aus `e4a9fe6cdd36c95b5657d5d94055003fc1e1d650` auf aktuelles `origin/main` aufsetzen und den Rust-Runtimepfad so anbinden, dass Betriebswerte aus der validierten globalen TOML-Konfiguration kommen. Die bestehende zentrale Modellwahl bleibt unverändert.

## Verbindliche Grenzen

- Keine Secrets, ENV-Werte oder Secretinhalte lesen, ausgeben oder schreiben.
- Zugangsdaten bleiben ausschließlich im bestehenden Infisical-Verfahren.
- Keine Anbieter- oder Modellwahländerung. Nur der aktuell freigegebene DeepSeek-Flash-Zielpfad bleibt bestehen.
- Keine Übernahme kompletter alter Main- oder Cutover-Historie.
- Source-Worktree `/home/nathanael/.worktrees/brain-global-toml-20260920` nur lesend untersuchen und unverändert lassen.
- Arbeits-Worktree `/home/nathanael/.worktrees/luna-brain-global-toml-20261001`, Branch `luna/finish-brain-global-toml-20261001`.
- Gemeinsame Prüfung gegen den Cutover-Kandidaten `88553d6179eef9fc32c08d36c2adbf80adf57557` und die überlappenden Brain-Runtimepfade von PR #61.
- Keine Unter-Threads und keine Unteragenten.

## Fertigkriterien

- Typisierte TOML-Konfiguration wird vor Runtime-Nebenwirkungen vollständig validiert.
- CLI-Konfigurationspfad ist absolut und alle bestehenden produktiven Settings-Leser verwenden denselben validierten Snapshot.
- Resolver- und Validierungsprüfungen decken die tatsächlich angebundenen Runtimepfade ab.
- Sinnvolle bestehende Checks, eigenes `gate_hook.py --review`, unabhängige Intent-Abnahme auf Freeze-SHA und reguläres Merge-/Security-Gate abgeschlossen.
- Bei ALLOW folgen Merge, Push, Deployment, Dienstrestart, echter Livebeleg und Cleanup gemäß Fortsetzungsablauf.
