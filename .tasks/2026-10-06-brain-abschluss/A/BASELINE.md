# Paket A: Compiler- und Betriebsbaseline

06.10.2026, 22:23 CEST. Unveränderter Quellstand `d6131cc52711a3e8b02d299704244f8d7dbdbce6` im eigenen Worktree `/home/nathanael/.worktrees/brain-a-abschluss-20261006`. Keine Produktänderung und kein Deploy.

## Prüfungen

| Prüfung | Ergebnis | Original |
|---|---|---|
| Cargo check, brain-maintenance und brain-serve, `--locked --jobs 2` | Exit 0, 3m 47s | Worktree `A/BASELINE-CHECK.log` |
| Cargo clippy, dieselben Crates, `--locked --jobs 2 --all-targets --no-deps -- -D warnings` | Exit 0, 3m 03s | Worktree `A/BASELINE-CLIPPY.log` |
| Cargo fmt, dieselben Crates, `-- --check` | Exit 0, keine Diffmarker | Transcript |

Cargo: `/home/nathanael/.cargo/bin/cargo`, Version 1.99.0. Check und Clippy nutzen ausdrücklich das eigene Target `/home/nathanael/.local/state/brain-a-baseline-20261006-target`, kein globales CARGO_TARGET_DIR. Das veraltete `verify-change.sh` enthält keinen Brain-Weg und zeigt noch auf einen alten Repo-Ort; deshalb wurden die belegten Cargoaufrufe direkt ausgeführt. Keine Tests gelaufen, daraus wird kein Testnachweis abgeleitet.

Der Guardrail verlangte vor Clippy einen direkten Read des Test-Wächter-Skills, obwohl der Skill bereits über Skill geladen war. Exakt den genannten Skill gelesen, dann Clippy regulär erneut gestartet. Keine Hookumgehung.

## Release und Health

`/usr/local/libexec/brain-release plan /home/nathanael/.worktrees/brain-a-abschluss-20261006`: Exit 0 vor den Diagnoseartefakten. Geprüfter Remote-main und Quell-SHA waren gleich. Regulärer root-eigener Helfer vorhanden. `current` und `maintenance-current` zeigen beide `e56e075d486a75f83f4954b58d8113588082d3f1`.

`http://127.0.0.1:8788/healthz` und `/readyz`: HTTP 200, `application/json`, Status `ok` beziehungsweise `ready`. Aktiver Wissensstand ist `maintenance-rebase-bfb28d8367f50c8766bf5fd027b9a2a35d30d7207046085b6d9038894144575e`, Wissen `docs-rebase-bfb28d8367f50c8766bf5fd027b9a2a35d30d7207046085b6d9038894144575e`. Das belegt Erreichbarkeit, noch keine richtige Spielantwort.

Maintenance konfiguriert beide vorhandenen Gitquellen (`deadlock-wiki-deadlock-data`, `steamtracking-gametracking-deadlock`) mit passenden `source.review:`-Scopes. Die alte fehlende Maintenance-Sichtbarkeit ist damit kein aktueller belegter Fixauftrag. Operator-Sichtbarkeit, Profilbestand und tatsächlicher Aktivierungspfad werden von der laufenden Inventur separat geprüft.

## Gate-Aufruf

Tatsächlicher Pfad: `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py`. Gültiger Hilfeaufruf ist `--review --help`, nicht `--help` allein. Letzterer lieferte JSON-stdin-Fehler und ist kein Gateurteil. Späterer Prüflauf: `--review --repo <absoluter eigener Worktree> --base <Basis-SHA> --head <Kandidat-SHA>`, ohne Modelloverride.
