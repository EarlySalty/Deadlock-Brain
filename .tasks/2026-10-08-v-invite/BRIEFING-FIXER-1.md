# V/F1: frischer nativer Fixer für den disjunkten Invite-Teilbau

## Ziel und Vertrag

Auftraggeber 481426fe-b477-42b3-91c6-901811fcba1d, V-Primary ist dein direkter Auftraggeber. Ein Paket, zwei disjunkte eigene Arbeitsbäume. Freigabe: ENTSCHEIDUNG-WACHE-037.md Abschnitt 3, gelesen von V. Das vollständige PLAN.md und FREIGABE-1.md in der eigenen Brain-Akte sind deine Referenz, kein Anlass zur vollen dort beschriebenen Shared-Integration.

Belegter historischer BLOCK: [gpt-6.1-sol] BLOCK: General invite questions are hijacked by personal-status routing. A/E3f-Quellstand 19f6d49f196a8ea4b9d9d90c188f3a3867914779 gegen fde910f6a0199c00f44083e73fc8f4c5e4f80b86, vorherige d8a0e727 und 0b47d78b. Quelle als Gitobjekt read-only lesen, kein alter Thread, kein Cherry-Pick alter Consumer. Beide allgemeinen Beispiele „Wann sind Einladungen wieder verfügbar?“ und „When does an invite expire?“ müssen false ergeben. „Bin ich eingeladen?“ und eindeutige eigene Status-/Zugangsfragen true. Allgemeine Verfahren/FPS/Bot-/Fremdpersonenfragen bleiben gewöhnlich; mehrdeutige Mischfragen nicht hijacken. Vorhandener Enum-/Zeitvertrag, requestgebundene Evidence und Projektion wiederverwenden. Keine zweite Engine.

Bots-Quelle 2be2df16d7a9390823a05691bef1ae69f1b8c8c1 gegen e18f522226f8e2dec5a1c03fe97c2aba3200c8d1: nur vorhandenen lesenden self_invite.rs-Statusschnitt und engen MCP-Zweig verwenden. Heutige Tabellen, Rechte, Konto-/Taskbindungen, Privacy und K-Consumer nachlesen, historische Source ist kein heutiger Vertrag. Neuere Pending-/Errorrequests dürfen nicht hinter älteren GC-/Taskbelegen verschwinden. Fehlende/mehrdeutige eigene Zuordnung bleibt unknown. Kein Statusversand, keine neue Steamaktion, kein DB-Schreiben und kein manueller Produktionszustandsfix. Request_id und Identität am vorhandenen authentifizierten MCP-Eingang binden, fremde Daten bleiben intern.

## Eigentum und Schreibgrenze

Du bist alleiniger Produktwriter für:

1. /home/nathanael/.worktrees/brain-v-invite-20261008/rust/crates/brain-contracts/src/invite.rs (noch nicht vorhanden), einschließlich direkter Modultests darin.
2. /home/nathanael/.worktrees/brain-v-invite-consumer-20261008/rust/bin/dl-bot/src/mcp/self_invite.rs (noch nicht vorhanden), einschließlich direkter Tests darin.
3. /home/nathanael/.worktrees/brain-v-invite-consumer-20261008/rust/bin/dl-bot/src/mcp.rs ausschließlich Modulimport, self_invite_status im bestehenden authentifizierten öffentlichen MCP-Pfad, enger Toollisten-/Dispatchanschluss und nötige direkte Tests dieses Statuszweigs. Kein Nebenrefactoring.

Zusätzlich eigene technische Rückgabe FIXER-1.md und REVIEW-FIXER-1.md in .tasks/2026-10-08-v-invite im Brainbaum sowie eigene Logs unter /tmp/brain-v-invite-f1-20261008/. Primary schreibt AUFTRAG/REGISTER/STATUS/FREIGABE/Briefing, du schreibst diese nicht.

NICHT schreiben: Brain contracts/lib.rs, provider_input.rs, API, discord_live.rs, hardening.rs, Cargo.toml/Cargo.lock oder andere Shared-Pfade. Kein Export, auch nicht als Ersatz-Harness im Produktbaum. Bots modglue.rs, dl-brain/lib.rs und brain_api.rs gehören nicht dir. Aktuelle K-Consumer und einzige 50/Tag-Quote erhalten, keine alte Cooldownausnahme oder personengebundene finish_question-/Reservationmechanik übernehmen. Keine globale Formatierung und keine Shared-Cargoänderung.

## Arbeitsstand und Git-Erlaubnis

Brain: Worktree /home/nathanael/.worktrees/brain-v-invite-20261008, Branch feat/brain-v-invite-20261008, HEAD 22561cb2fe542c98fc383b12ca576c581d31056d, Startbasis cf02c9a0, aktueller K-Main ist integriert. Plancommit d49983d0 bleibt erhalten. Primary hat ausschließlich eigene neue FREIGABE-1.md und dieses Briefing als Dokument-WIP.

Bots: Worktree /home/nathanael/.worktrees/brain-v-invite-consumer-20261008, Branch feat/brain-v-invite-consumer-20261008, HEAD 8e1b8f03cf0d84f4754eedc8fe848ef0fceffce8, frisch von origin/main. Primary hat eine eigene Aufgabenakte angelegt, nicht anfassen.

Du darfst deine drei Produktdateien gezielt je Repo committen und nach tatsächlichem regulären Selbstgate auf die eigenen Featurebranches pushen, keine Primary-Dateien adden. Gitbefehle einzeln, literale absolute Pfade, kein add -A, kein Stash oder Reset fremder Arbeit. Commit-Trailer Co-authored-by: GPT 6.1 Sol <gpt-6.1-sol@local>. Kein Mainmerge oder Mainpush, kein Deploy/Restart/Cleanup/Settle: dieser Teilstand bleibt ausdrücklich offen bis zur Restübergabe. Schutz-Hooks nicht umgehen und keine Overrideangebote.

## Beweisziel und Stop-Bedingung

Vor Codefragen Skill code-suche und Graphify, dann konkrete Quellstellen. Moli ist der erlaubte Browser; Browserarbeit ist nicht erforderlich. Brave MUST NOT verwenden. Persönliche Dienste bleiben unangetastet. Rust und Postgres, keine neuen Connectoren/Modelle/Timeouts/Bot-Antworttexte. Secrets NEVER ausgeben, Rohdaten MUST NOT in Git oder Codiermodellkontext, Discord-/Steam-IDs, Mitgliederlisten und fremde Personendaten NEVER an das Modell. Sichere lokale Prüfdaten, keine fremden Communitydaten oder unfreigegebenen externen Modell-/APIproben.

Vor Tests rolle-test-waechter laden. Bestehende Suites erhalten, cargo-slot mit passender vorhandener Toolchain/SQLX_OFFLINE/locked/offline/jobs-Konfiguration. Tests mit `--no-fail-fast -- --include-ignored` und zusätzlichen tatsächlich vorhandenen Repo-Features, keine echten Prod-DBtests, keine Pipe die Exitcodes maskiert. Passende Formatprüfung und Clippy -D warnings auf geänderten kompilierten Botszielen. Eigenes Brainmodul rustfmt prüfen; weil lib.rs-Export/Cargo noch nicht freigegeben sind, Brain-Modultests sind noch nicht ausführbar und dürfen nicht als geprüft gelten. Keine Fake-Verdrahtung, keinen unangetasteten Basis-Testlauf als Test deiner neuen Datei verkaufen.

Nach eigenem Commit normaler Selbstgate: vorhandener `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review` mit absolutem `--repo` des jeweiligen eigenen Worktrees, `--base origin/main` und `--head HEAD`. Bisheriges tatsächliches Urteilmodell beibehalten, keine eigene Reviewerrolle oder Modellneuwürfe. Quellen/Dokumente dürfen zum regulären Gate, keine realen Personendaten. Eventuelle BLOCK-Funde am eigenen Scope prüfen; bei Fixbedarf Rückgabe mit konkretem Fund an V, damit V den vorgeschriebenen frischen Fixerkontext starten kann. Keine weiteren Subagenten oder T3-Threads durch dich.

Melde in FIXER-1.md getrennt: geändert, kompiliert, getestete Anzahl/ignored/filtered/Exitcodes, Gateurteil mit Source-SHA und Exit, committed/gepusht oder nicht; nicht gemergt und nicht live. Nenne den genauen restlichen Shared-Integrationsbedarf und fachliche Blocker. Teilbau ist keine funktionierende Liveantwort. Mehr als fünf erfolglose Gatefixrunden ist Eskalation, kein Override.

## Routing und Wache

Paket V/F1, Versuch 1, Ereignisproduzent nativer Fixer. Rückgabe an V-Primary über native Agent-Abschlussmeldung, keine Fremdsessionkontakte, keine Nutzerfragen, keine neuen T3-Threads. Zentrale TODO/REGISTER gehören dem Delegator. Nach etwa 20 Minuten deinen Stand prüfen, spätestens 30 Minuten mit gesichertem WIP oder echtem Blocker zurückgeben. Kein Rundenspam. Auftrag nicht breiter machen, offene Shared-Fragen als genaue Deltas an V.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-v-invite-20261008 und /home/nathanael/.worktrees/brain-v-invite-consumer-20261008
