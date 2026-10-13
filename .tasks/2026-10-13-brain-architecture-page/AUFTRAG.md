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

Browser: only /home/nathanael/.local/bin/moli with layout, image and font enabled. Candidate HTML is injected into the specified private page's isolated Moli tab. The candidate check redraws the original connector code after Moli's first layout because Page.setDocumentContent does not execute script tags or update geometry synchronously. Live verification must navigate the served file normally, without injection or redraw.

Images and geometric checks: /home/nathanael/.claude/sichtpruefung/2026-10-13-brain-architektur/. The diagram retains its fixed width with horizontal scrolling; surrounding text reflows at 390 px. Native Moli DOM checks cover 33 boxes/29 edges in Heute and 25 boxes/23 edges in Ziel.
