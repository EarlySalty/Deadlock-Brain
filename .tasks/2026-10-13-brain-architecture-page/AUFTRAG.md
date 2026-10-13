# Package SEITE

Goal: refresh the private architecture page and route its connectors outside the boxes while preserving its layout, colors, legend and hover details.

Scope: ARCHITEKTUR.html, its dated backup, and installation of that HTML file only. No Caddy configuration, database, service or application changes. Branch handover to the orchestrator; no main merge.

Acceptance: both tabs show verified status dated 2026-10-13; all planned steps have explicit status and date; maintenance and dl-knowledge are shown stopped; enforced memory caps are distinguished from the later memory-use target; connectors have no box intersections; both tabs are inspected in Moli including width 1280; the served response equals the source byte for byte.

Evidence checked on 2026-10-13:
- origin/main edf503b2 contains P0 c7123fe7 and P0b ef0418d5. /opt/deadlock-brain/current and the three running executable paths select ef0418d5.
- The real memory-controller limits for brain-serve, deadlock-brain-site and brain-browser-consent are 536870912, 268435456 and 67108864 bytes. Their system services are active. The requested deadlock-brain-serve name does not exist; the actual unit is brain-serve.service.
- The user maintenance timer and dl-knowledge service are inactive and disabled. Their journal confirms stops at 2026-10-12 00:21:13 through 00:21:16 CEST. Maintenance service is inactive/static; the timer is the disabled scheduler.
- The six legacy user timers are inactive, still enabled, and have no next run. TODO.md decision 2a keeps them stopped.
- P1r and P2/P3/P4/P5/P9 v2 branches exist and are absent from `git branch --merged origin/main`. TODO.md records five P1 BLOCKs, its stop and the redesign restart. Decisions at 17:47 and 18:01 require P10 hourly hero/item stats retained forever after wave 2.

Browser: only /home/nathanael/.local/bin/moli with layout, image and font enabled. Candidate HTML is injected into the specified private page's isolated Moli tab; its unchanged script is executed because Page.setDocumentContent leaves script tags inert. No test-only connector redraw is used. Moli returns zero offsetHeight for a newly revealed tab before its next layout; the final connectors use stable side ports instead of relying on that measurement. Live verification navigates the served file normally, without injection.

Images and geometric checks: /home/nathanael/.claude/sichtpruefung/2026-10-13-brain-architektur/. The diagram retains its fixed width with horizontal scrolling; surrounding text reflows at 390 px. Native Moli DOM checks cover 33 boxes/29 edges in Heute and 25 boxes/23 edges in Ziel.

Final live proof: HTTP 200 with text/html; charset=utf-8. curl response, assigned repository source, worktree source and /srv/private-html/brain-architektur/index.html are byte-identical. Served file owner/mode root:caddy 0644. The original file is preserved beside the assigned source as ARCHITEKTUR.bak-20261013.html. Caddy configuration and services were not modified or restarted. Both tabs were inspected at 1920, 1280 and 390 px, plus right-scrolled 1280 views. All six geometric checks find zero connector/box intersections, no body overflow and hover details on every box. JavaScript syntax and the no-dash check pass. No Rust build or application test suite was run because no application code changed. Proof applies to Moli, not to other browsers or the user's phone.

Review blocker: gate_hook.py --review exited 2 twice on a08701c1, before any reviewer could start. Both configured backends failed namespace creation with Cannot allocate memory (bwrap and unshare). No ALLOW or BLOCK was returned; there is no merge approval. The final stable-port followup is verified live but has no independent gate verdict. No main merge was performed. Logs: /tmp/claude-1000/-home-nathanael-repos-Deadlock-Brain/e8bf9ddb-d257-4e08-a26f-42545819b402/tasks/b1hop55cj.output and bpmu345bz.output. No namespace protection was disabled, and no unrelated process was stopped.

SICHTPRUEFUNG[SP-1]: 8 Aufnahmen | 0 mit Befund | Bilder unter /home/nathanael/.claude/sichtpruefung/2026-10-13-brain-architektur | angesehen: live-ist-1920.png, live-ziel-1920.png, live-ist-1280.png, live-ziel-1280.png, live-ist-390.png, live-ziel-390.png, live-ist-1280-right.png, live-ziel-1280-right.png

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 1 belegt | Senke: http://100.117.29.112/brain-architektur/

MERGEPROTOKOLL[MS-1]: 0 Git-Schritte einzeln | Anläufe: 0 | Gate: kein Urteil, Werkzeugausfall bei Namespace-Erzeugung; kein Main-Merge

Next integrator action when namespace creation works: python3 -I /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-seite-20261013 --base origin/main --head HEAD. The private file installation was explicitly requested and is already live; this handover is not permission to merge application code or task records to main.
