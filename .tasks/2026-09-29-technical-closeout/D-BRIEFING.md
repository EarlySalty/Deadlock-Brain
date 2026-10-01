status: aktiv
Datum: 2026-09-29

# Paket D: drei kleine Consumer-Regressionsabnahmen

Gemeinsamer Vertrag: /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/AUFTRAG.md. Luna, einziger Thread für D, keine Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a. Nur dieses Paket, kein Refactoring, kein globales fmt. Keine Code-Kommentare.

Eigener Bericht-Worktree: /home/nathanael/.worktrees/brain-pre-g5-consumers-20260929
Branch review/pre-g5-consumers-20260929, Basis 305df2d36ec7b5d0513d6c0769051b41538d6a1b, clean.

Referenzen wörtlich aus Nutzerauftrag:
„Wenn weiterhin grün: keine unnötige Arbeit.“ (Docs)
„Wenn GitHub Job wegen Billing nicht startet: das ist externer CI-Blocker, kein Codefehler.“ (2nd-Brain)
„PR #984 wurde zuletzt als gemergt beobachtet. Nur Regression prüfen.“ (Twitch)

1. Docs PR #4 ist aktuell CLEAN/grün, Head ff20af7a8e3fcacc349cb2d97eda34dfa0897957, Branch codex/fix-c9-consumer-wiring, bestehender Worktree /home/nathanael/.worktrees/docs-c9-consumer-wiring. Prüfe docs.public, typed BrainClient, sicherer Secretpfad, kein lokales RAG/Modell/CLI-Token. Bei unverändert grün keine Codeänderung.
2. 2nd-Brain PR #2 Head ab83b691befd761a16d971af5c249a604c5d4e0d, gleicher Branchname, Worktree /home/nathanael/.worktrees/second-c9-consumer-wiring. Typed Brain adapter fixtures aktuell FAILURE. Workflow run/annotations prüfen, nicht Billing raten. Trusted internal scopes, public/*.public/wildcard reject, kein Corpus-Export, kein RAG/Modell-Fallback, keine Tokens in Args/Logs. Lokal fmt; cargo test --locked --offline; cargo clippy --locked --offline. Kleine tatsächliche Fixes sind erlaubt, größere mit genauen Fundstellen an Sol eskalieren. Vor Schreiben status prüfen; unerwarteten fremden Dirty-Stand nicht überschreiben. Dieser Worker ist exklusiv für diese beiden Consumer-Branches beauftragt. Commit/Push nur dort, bestehende PR aktualisieren, niemals nach main mergen oder Auto-Merge einschalten.
3. Twitch PR #984 gemergt als 13321934f421b9a1cd81d0be8668c2e5a8fd925b, Head 33ce4ac45edf93b5b455b16672faf935ad135dd5. Eigener neuer read-only Testworktree von aktuellem origin/main zulässig unter /home/nathanael/.worktrees/twitch-brain-regression-20260929, Branch review/brain-regression-20260929. Kein geteiltes Checkout ändern. Nur Regression: Shadow Probe detached, keine 115s-Verzögerung, typed ohne Legacy-Fallback, invalid mode fail closed, typed fixtures, Rust SQLx required. Kein Neubau ohne tatsächlichen Befund. Fremde Twitch-Patch-Arbeit nicht anfassen/anschreiben.

Graphify zuerst. Test-Wächter lesen. Keine Produktivservices/DBs, keine Nachrichten, keine Secrets/Passwort-ENV. Build-Last begrenzen, keine globalen Configänderungen. 2nd-Brain existiert momentan nicht als T3-Projekt; kein neues Projekt nötig, dieses Paket bleibt ein einzelner Cross-Repo-Abnahmethread im Brain-Projekt.

Bericht .tasks/2026-09-29-technical-closeout/D-REPORT.md im eigenen Brain-Worktree. Gegen konkrete SHAs testen, Befehle/Exitcodes/ignorierte Tests und Billing-Belege nennen. Brain-Berichtsbranch commit/push und PR nach migration/rust-integration erlaubt, nicht mergen. Consumer-PRs verlinken. Nicht bloß TODO liefern, kleine klar belegte Defekte beheben und neu prüfen. Eigene Codefixes vor Abgabe gate_hook.py --review; unabhängige Abnahme macht Orchestrator.
