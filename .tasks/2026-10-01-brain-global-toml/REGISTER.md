status: aktiv
Stand: 2026-10-01

# Register: Brain globale TOML-Konfiguration

ORCHESTRIERUNG[OR-1]: Stufe mittel | Schritt bau | Artefakt: .tasks/2026-10-01-brain-global-toml

## Auftrag und Basis

- Intent-/Ansprechpartner-Thread: `cf1d8ad4-dd63-403d-a2bf-6cc8b1b9fa93`
- Gestoppter Vorgänger: `68348009`, nicht wieder aufnehmen
- Owner: aktueller Luna-Thread, alleiniger Thread für dieses Paket
- Worker-Threads und Unteragenten: keine, ausdrücklich ausgeschlossen
- Arbeits-Worktree: `/home/nathanael/.worktrees/luna-brain-global-toml-20261001`
- Arbeitsbranch: `luna/finish-brain-global-toml-20261001`
- Start-SHA: `e4a9fe6cdd36c95b5657d5d94055003fc1e1d650`
- Source-Branch/Ref: `feat/brain-global-toml-20260920` und `origin/feat/brain-global-toml-20260920`, beide frisch auf `e4a9fe6cdd36c95b5657d5d94055003fc1e1d650`
- Aktuelles `origin/main`: `084cdfc80d48f6f1659fc764955d7f941485e6bf`
- Cutover-Kandidat: `integration/brain-v1-recovered-20261001` auf `88553d6179eef9fc32c08d36c2adbf80adf57557`

## Kopplung

- PR #61 `codex/brain-functional-closeout-20260930` auf `b687f613b3df2c49138d9d2837e005c33e646d9f`, offen gegen `main`; überlappt in `deadlock-brain-core/src/config.rs`, `ai.rs`, `model_resolver.rs` und `deadlock-brain/src/main.rs`.
- Cutover-Kandidat enthält PR #61 und 58 spätere Commits. Er wird nur als Runtime-/Reviewreferenz genutzt, nicht vollständig übernommen.
- PR #40 `migration/rust-integration` auf `1c362bca6d35e7fec10125b2b159e7513a299243`, offen gegen `main`.
- Source-WIP-Worktree `/home/nathanael/.worktrees/brain-global-toml-20260920` bleibt unverändert. `BRAIN-SOURCE-BACKUP.json` ist die Provenienzliste.
- Das Source-WIP erweitert die Auswahl auf automatisch numerisch aufsteigende Flash-Modelle. Diese Modellwahländerung wird nicht übernommen.

## Thread-Register (T3)

| Paket | Thread-ID | Modell | Status | Worktree | Letzte Meldung |
|---|---|---|---|---|---|
| Brain globale TOML-Konfiguration | aktueller Luna-Thread, ID nicht im Auftrag ausgewiesen | Luna | aktiv | `/home/nathanael/.worktrees/luna-brain-global-toml-20261001` | TOML-Loader und Runtime-Anbindung in Arbeit; Readonly-Pool-Dispatch aus `84d68f4` und `21cfcd8` geordnet integriert |

## Prüffortschritt

- Tests: `dbrain-retrieval --lib` 45 passed, 0 failed, 15 ignored; `deadlock-brain-core` config 16 passed, 0 failed, 0 ignored; `deadlock-brain` wiki refresh 5 passed, 0 failed, 0 ignored; `dbrain-population` API 3 passed, 0 failed, 0 ignored.
- Compile-Check: `deadlock-brain` mit Rust 1.98 und `--locked` erfolgreich; die aktuellen `wiki_refresh` Tests kompilieren ebenfalls den aktualisierten Binary-Pfad. Für die Cargo-Läufe wurden vier externe Pfade vorübergehend auf gepinnte Abhängigkeiten im isolierten Layout gesetzt und danach zurückgenommen.
- Formatprüfung: Workspace-weites `cargo fmt --all -- --check` sowie gezieltes `rustfmt --check` melden vorhandene Formatabweichungen in weiteren beziehungsweise angefassten Dateien. Neue Zeilen sind formatiert; keine unbeteiligten Bereiche wurden geändert.
- Eigenes Merge-Gate: Runde 1 BLOCK, vier Befunde behoben. Runde 2 BLOCK, Clap-ID und Wiki-Resolver behoben. Runde 3 ALLOW auf `7c4c11f771fd9375cc45ec426e063179c4ebb5d3`. Runde 4 ALLOW mit drei NITs, behoben in `a46ad05`; Runde 5 ausstehend.
- Gemeinsamer Freeze und unabhängige Intent-Abnahme: ausstehend.
- Reguläres Merge-/Security-Gate: ausstehend.
- Merge und Push: nach Intent-Abnahme und regulärem Gate. Dedicated main worktree `/home/nathanael/.worktrees/brain-live-main` vor Änderungen erneut prüfen.
- Deploy, Dienstrestart, Livebeleg und Cleanup: ausstehend.
- Blocker: unabhängige Intent-Abnahme, abschließendes Gate und Deployment noch offen; Source-Resolver-Subsystem bleibt ausgeschlossen.
