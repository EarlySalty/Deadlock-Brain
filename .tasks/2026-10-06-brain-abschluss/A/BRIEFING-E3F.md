# A-E3f: frischer Fixer für den blockierten Invite-Anschluss

Neue native Fixer-Blattrolle mit frischem Kontext, nicht der Implementierer. Auftraggeber Paket A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`. Keine eigenen Reviewer, Agenten oder T3-Threads. Root-Akten schreibt A. Grundding aus VON_HAUPT.md 07.10.01:45 ist oberste Priorität; normale Spiel-/Serverfragen dürfen niemals in den persönlichen Invite-Skill umgebogen werden.

## Bestehender Stand, weiterbauen

Worktree `/home/nathanael/.worktrees/brain-a-invite-brain-20261006`, Branch fix/brain-a-invite-brain-20261006, sauberer Featurecommit d8a0e727687dda38675833af38dbb698f2ef949c, frische Basis fde910f6. Nicht gepusht oder ausgeliefert. Bestehende Implementierung und warme Targets/Prüfablagen weiterverwenden, keine Neuimplementierung oder kalte Quellkopie. A-E4 besitzt den separaten Invite-Bots-Worktree; keine anderen A-/B-/D-Dateien anfassen.

Strenger Enum-/Zeitpunktvertrag, requestgebundener MCP-Abruf, vorhandene Requester-Header/Rechteprüfung/DiscordRetriever und minimale gemeinsame Providerprojektion bereits gebaut. Bestehenden Nutzervertrag in A/BRIEFING-INVITE-SKILL.md und A/INVITE-VERTRAG.md lesen: ausschließlich eigener Enum plus Zeitpunkt, keine IDs/Namen/Dritte/Rohfragen, keine neue Persistenz oder Providerstrecke.

## Verbindlicher Gate-BLOCK

Regulärer Gate gpt-6.1-sol, Exit 1, `/tmp/brain-a-invite-brain-e3-gate.log`:

> [gpt-6.1-sol] BLOCK: The invite detector hijacks ordinary questions and answers a different question.

Befund in brain-contracts/src/invite.rs:136:

> Both matching branches produce false positives: `invite && status` accepts “Wie kann ich eine Einladung verschicken?” because `verschick` overrides the procedural exclusion; the fallback at lines 137–145 accepts “Welche FPS bekomme ich in Deadlock?” and its `bekomm ich` variant without any invitation reference. | `project_query` discards these questions and substitutes the personal invitation-status question. For authorized bot requests, `retrieve_with_usage` then bypasses ordinary retrieval entirely. Narrow both branches and add regression cases for these ordinary questions.

Zusätzlicher bestätigter eigener Fehler: „Bin ich eingeladen?“ wird nicht erkannt, weil der Marker eingeladen fehlt. Ein neuer Vertragstest ist deshalb rot, die frühe Datenschutzprojektion für genau diese Form ebenfalls nicht gewährleistet. Gatebefund und Zwillinge im bestehenden Erkennungspfad korrigieren, allgemeine Spiel-/Server-/Verfahrensfragen erhalten, eindeutige eigene Invite-Statusfragen zuverlässig minimal projizieren. Keine eigene Bot-Endformulierung.

## Rote Prüfungen vollständig klären

Vorheriger Stand: brain-api 16 grün, brain-contracts 20 grün/1 rot, brain-providers 11 grün, brain-serve 54 grün/2 rot/1 ignoriert, Scratch-Serviceprüfung 1 rot. Zusammen 101 passed, 4 failed, 1 ignored. Keine Altfehler- oder Flake-Einstufung.

Zwei Analytics-Prüfungen liefern Unavailable statt Answered/BudgetExceeded. Scratch-Wiederholung liefert database_unavailable statt erwarteter Schema-Diagnose. Tatsächlichen privaten Harness und Quellen-/Pool-/Schemavertrag empirisch prüfen; Suite weder abschwächen noch still überspringen. Keine manuellen Produktions-DB-Korrekturen, Source-/Scope-/Qualitätsgrenzen bleiben erhalten. Bestehende private Scratch-Instanz war zuletzt gestoppt.

Vorhandene Logs: /tmp/brain-a-invite-brain-e3-clippy-final.log, /tmp/brain-a-invite-brain-e3-libtest-final.log, /tmp/brain-a-invite-brain-e3-libtest-rest.log, /tmp/brain-a-invite-brain-e3-scratch-retry.log. Eigentum eng nötige ursprüngliche Invite-Vertrags-/Projektions-/Retrieverpfade plus deren passende Tests. Keine Refactorings, neuen Modelle, Zeiteinstellungen oder Code-Kommentare. Rust, Graphify zuerst, höchstens ein Cargo-Job.

## Abschlussgrenze

Compiler, geändertes Format, striktes Clippy und bestehende passende Suites mit include-ignored/test-threads=1 tatsächlich zu Ende prüfen. Private Datenbank ohne Prod-/Provider-/Communityrohtextzugriff. Warme Targets erhalten, keine künstlichen Kurzlimits; Wache 20 Minuten, spätestens 30, ist kein Abbruchbudget.

Danach derselbe reguläre Gate mit konfigurierter Kette; erstes inhaltliches Urteil blieb gpt-6.1-sol. Kein gezielter Modellwechsel, kein BLOCK-Neuwurf. Bei erneutem BLOCK Fundliste an A für nächsten frischen Fixer, nicht selbst umdeuten. Bei ALLOW eigener Featurecommit/-push nach normalen Einzel-Git-Regeln, kein Main-/Binarydeploy. Tatsächliche rote/grüne Zahlen, exakte SHAs/Dateien, Gatewortlaut/Exit und offene Quellenkopplung an A zurückgeben. E3/E4 werden gemeinsam abgenommen.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-invite-brain-20261006
