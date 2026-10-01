status: aktiv
Datum: 2026-09-29

# Paket G: reproduzierbare gesperrte Rust-Abhängigkeiten

Luna, einziger Thread G, keine Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a. Gemeinsamer Vertrag AUFTRAG.md daneben. Nur eigener Worktree /home/nathanael/.worktrees/brain-pre-g5-dependencies-20260929, Branch fix/pre-g5-locked-dependencies-20260929, clean auf 305df2d36ec7b5d0513d6c0769051b41538d6a1b.

Wörtlicher Nutzerauftrag: „kleine, klar abgegrenzte Fixes / CI / Docs / Konfliktauflösung → Luna“; „KEINE PR-/Required-Check-/Security-Policies umgehen.“; „cargo ... --locked --offline“.

Neuer echter Befund aus nur dokumentationsänderndem PR #57:
https://github.com/EarlySalty/Deadlock-Brain/actions/runs/36550365750
https://github.com/EarlySalty/Deadlock-Brain/actions/runs/36550365924
Alle Core-/Source-/Wiki-/Replay-Suiten scheitern bereits beim dependency fetch. Kein Billing, Tests starten real. Fehler: haste_core aus https://github.com/deadlock-api/haste, rev bfb292d4798031350861ad297aa26753267a1ea6, revision not found/failed to authenticate. `gh api repos/deadlock-api/haste` heute HTTP 404.

rust/crates/dbrain-replay/Cargo.toml pinnt haste_core genau auf diesen Git-Commit und valveprotos auf https://github.com/deadlock-api/valveprotos-rs.git rev 4f4a3cb1b0c6f19af59a722acb79ecccd01f61f6. Lokal funktionieren Builds nur dank vorhandener Caches. Das darf nicht als reproduzierbarer CI-Erfolg gelten.

Aufgabe: tatsächliche Ursache ermitteln und minimal reparieren. Vorhandene exakte Git-Objekte/Cargo-Caches und Herkunft/Lizenzen verifizieren. Keine beliebige Ersatzversion/Fork, kein ungeprüfter Parser-/Schema-Pin-Wechsel. Wenn ein verifizierter kanonischer Umzug exakt denselben Commit ausliefert, nutzen. Sonst exakt gepinnte benötigte lizenzkonforme Quellen kontrolliert vendoren (keine gesamte Cargo-Registry), mit Herkunft, Commit, Hash und unveränderten Lizenzhinweisen. Lizenzkommentare beibehalten, keine neuen eigenen Code-Kommentare. Keinen neuen Zugang/Secret als Abkürzung anlegen. Bei nicht belegbarer Herkunft/Lizenz stoppen und präzise eskalieren statt Code zu erfinden.

Scope nur Cargo-Manifeste/Lockfile für betroffene Git-Abhängigkeiten, notwendige vendored Sources/Provenienz und minimaler Build-/CI-Bezug. Keine Source-Adapter/Runtime-/Fact-Implementierung. A ändert ebenfalls wenige Cargo.lock-Einträge, nur eigenes Delta committen, Integrator führt zusammen. C bearbeitet Test-Harness, nicht anfassen. Keine neuen Python-Skripte und keine Security-Policyänderung.

Beweis: sauberer Git-Cache muss exakt gepinnte Dependencies ohne private Credentials erwerben können; danach --locked --offline check/test für dbrain-replay und relevante Workspace-Metadaten. Keine Shared Caches löschen oder verändern. Ein eigener temporärer CARGO_HOME als Testwerkzeugisolation ist möglich, keine Produktkonfiguration über ENV. Bestehende Versions-/Provenienz-Tests nachziehen, nicht abschwächen. Mehrere Git-Dependencies vollständig prüfen, damit der nächste 404 nicht erst nach Merge auffällt. Kein Release-Build notwendig vor Gesamtabnahme.

Commit/Push des eigenen Branches und PR nach migration/rust-integration erlaubt, KEIN Merge und KEIN Deploy. Selbstprüfung über gate_hook.py --review mit --base origin/migration/rust-integration. Bericht .tasks/2026-09-29-technical-closeout/G-REPORT.md mit Ursache, SHA/Hash-/Lizenzbelegen, Prüfungen/Exitcodes und PR. Größere notwendige Paketvergrößerung per Bump-up melden, vorhandenen Stand erhalten.
