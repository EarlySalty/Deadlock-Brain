status: aktiv
Datum: 2026-09-29

# Paket B: Bots C9 und offenen Merge abschließen

Gemeinsamer Vertrag: /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/AUFTRAG.md. Du bist Sol und einziger Thread für B, keine Unter-Threads/Unteragenten. Intent-Thread 562a877b-0939-440a-964d-1145d9e9431a. Bump-up-Format im Vertrag.

Worktree /home/nathanael/.worktrees/bots-c9-consumer-wiring
Branch codex/fix-c9-consumer-wiring
PR https://github.com/EarlySalty/Deadlock-Bots/pull/459
Remote-HEAD 74cc114290e6490afc5dc922ef660c34025c5f36
MERGE_HEAD 42175e5fd68a1c83bf4201b4c40cca1099f00652

Wörtlicher Auftrag: „Zuletzt war ein Merge von aktuellem origin/main bereits konfliktfrei aufgelöst, aber noch nicht committed. NICHT Merge aborten. NICHT resetten.“ und „PR bleibt Draft, solange Auto-Merge/G5-Risiko besteht. NICHT nach main mergen.“

Merge ist aktuell staged und ohne U-Pfade, enthält legitime Main-Änderungen. Vollständig erhalten, prüfen und sauber committen. Nie das geteilte Main-Checkout ändern. Scope ist C9-Consumer-Wiring, die notwendige Merge-Konfliktauflösung und passende Tests/CI. Keine anderen Produktfeatures anfassen. Bericht muss Main-Merge-Delta vom eigenen C9-Diff unterscheiden. Kein Refactoring und kein pauschales fmt außerhalb eigener Pfade.

Pflichtinvarianten:
- unset BRAIN_CLIENT_MODE dokumentierter Legacy-Default; explizit leer/unbekannt fail closed.
- typed sichtbare Antwort nur BrainClient, kein Legacy/LLM/RAG-Fallback.
- shadow Legacy sichtbar; Probe vollständig detached, keine Verzögerung der sichtbaren Antwort.
- kollisionsresistente Request/Conversation-IDs.
- unavailable und build_rejected korrekt.
- keine Auto-Merge-Umgehung, Release-Gate sicher/report-only, Draft erhalten.

Tests: Typed Brain Fixtures, Workspace, config, knowledge eval und hybrid, fmt, clippy. Existing failure baseline und ignorierte Tests ehrlich angeben. Produktionsbot nicht starten, kein Discord-/Twitch-Aufruf, keine echte Nachricht. Keine produktiven DB-Credentials/Passwort-ENV. Für DB-Fälle isoliertes Wegwerf-Postgres/Peer oder sicheren vorhandenen Harness verwenden. Keine neuen Code-Kommentare.

Commit und Push dieses Branches erlaubt; PR #459 aktualisieren und in T3 verlinken. Nicht selbst mergen, nicht ready-for-review stellen, keinen Auto-Merge aktivieren. Selbstreview über vorhandenen gate_hook.py --review, unabhängigen Review übernimmt Orchestrator. Bericht .tasks/2026-09-29-brain-c9-closeout/B-REPORT.md im Bots-Worktree mit SHA, Befehlen/Exitcodes, Diffstat und Abschlusskriterien. Bis zur verifizierten Abgabe durchziehen.
