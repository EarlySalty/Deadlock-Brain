# A-E4f: frischer Fixer für zwei Bots-Gatebefunde

## Ziel und Vertrag

Gate gpt-6.1-sol blockiert 11eb66ead80c29e157dec8c6d8211bbc1f02fd4d. Nur berechtigten Kern der beiden Funde beheben, vorhandenen persönlichen Status-Skill und gemeinsamen Consumer erhalten. Kein zweiter Statusspeicher, Poller, Bot-Antworttext oder LLM-Connector. Status ausschließlich eigener belegter Enum plus Zeitpunkt, keine IDs, Namen, Rohzeilen, Fehlerfreitexte oder Drittpersondaten in Resultat/Providerpayload/Log. A/BRIEFING-INVITE-SKILL.md und INVITE-VERTRAG.md gelten unverändert.

## Eigentum und übernommener Stand

Eigener bestehender Worktree /home/nathanael/.worktrees/brain-a-invite-bots-20261006, Branch fix/brain-a-invite-bots-20261006, sauber von A nach Rückgabe geprüft. HEAD 11eb66ead80c29e157dec8c6d8211bbc1f02fd4d. Kein anderer lebender Worker an diesem Worktree. Original E4 abgeschlossen mit BLOCK; nicht wieder aufnehmen. Nutze den vorhandenen warmen Target und erhaltenen privaten Harness, keinen kalten Doppelbau. Neue Belege /tmp/brain-a-invite-bots-e4f-proof-20261007/.

Eigentum eng rust/bin/dl-bot/src/mcp/self_invite.rs, rust/crates/dl-brain/src/lib.rs und zwingend passende Consumer-/Completionstellen in den schon zu E4 gehörenden main.rs/modglue.rs sowie Tests dort. brain_api.rs ist F2c und tabu, invite_lounge.rs/B-/Steam-Mechanik ebenfalls. Keine neue Migration oder produktive DB-Zustandskorrektur. Keine globalen Formatter-/Refactoringänderungen. Nur Rust ohne neue Code-Kommentare.

## Offene Gatefunde

Exakter Gatewortlaut /tmp/a-e4-gate-20261007.log, Exit 1:
[gpt-6.1-sol] BLOCK: Historical observations still mask newer request states; completion markers still identify users rather than reservations.

1. self_invite.rs:175: historische GC-/Taskbelege verdrängen neuere Requestzustände. Neue ausstehende oder fehlgeschlagene Aufträge können alten Status liefern. Aktuelle kanonisch zugeordnete Request-/Taskzustände zeitlich zusammen mit echten historischen Beobachtungen auswerten. Kein erfundener Einladungszeitpunkt, Auditexistenz ist kein neuer Versand, GC-Code 5/Recoverysemantik aus dem vorhandenen Vertrag erhalten. Quelle/Subjekt müssen eindeutig dieselbe aktuelle verifizierte Kontoauswahl bleiben.
2. dl-brain/src/lib.rs:178, Zwillinge :163/:168: Abschlussmarker benennen Nutzer statt die konkrete Reservierung. Eine alte fertig werdende Anfrage kann eine spätere Reservierung freigeben. Completion-/Erfolg-/Fehlerzweige durch dieselbe konkrete requestgebundene Reservierungsidentität sichern, jeden Zwilling mitprüfen. Ein alter Abschluss darf weder neue Reservierung löschen noch deren Erfolg vortäuschen, notwendige Freigabe nach Fehler bleibt möglich.

Graphify zuerst, dann aktuelle Fundstellen lesen. Keine neue Reviewmeinung als Freigabe anstelle des Gates.

## Bestehende Prüfungen und Beweisziel

Original: Bot 326 passed/7 failed/0 ignored, Brain 13 passed/0 failed/0 ignored. SHA-verifizierte e1f11614-Baseline 332 passed/10 failed/0 ignored; alle sieben aktuellen roten Tests waren dort bereits rot. Keine pauschale grüne Gesamtsuite behaupten, kein Altfehler ohne erneuten belegten Vergleich. Drei neue Reader-/MCPtests und Statusfolgefrage-Test grün. Fmt und Compilercheck grün, abgegrenztes Clippy Rust 1.97.1 --no-deps -D warnings grün, Rust 1.99 unveränderte Abhängigkeiten rot. Logs /tmp/a-e4-tests-native-20261007.log, a-e4-tests-brain-20261007.log, a-e4-baseline-voll-20261007.log, a-e4-clippy-197-20261007.log, a-e4-clippy-20261007.log.

Benutze festgelegte 1.97.1, passende volle bestehende Suites mit tatsächlicher privater PG und --include-ignored --test-threads=1, keine öffentlichen Communitytests oder Prodmutation. Testzählungen, Exits, notwendige alte Baseline und ausgelaufene eigene PG konkret zurückgeben. Neuerer Pending/Error gegenüber älterem GC/Audit sowie überlappende Reservierungen müssen durch reale bestehende Pfade geprüft sein. Keine Testerwartung, die den Gatebug festschreibt. Keine neuen Config-ENV, Secrets/DSNs nicht ausgeben.

## Git, Release-Hold und Rückgabe

Einziger Build-/Install-/Restart-/Tickeigentümer live_strecke. Keine Main-Pushes, Main-Merges, Releasebuilds, Runtime-/Writer-/Lunaänderungen oder Ticks. Nur grüner Featurecommit/-backup erlaubt. Ein Git-Schritt pro Bash-Aufruf, literale absolute Pfade, nur eigene Dateien, kein add -A, Forcepush, Reset/Stash.

Nach tatsächlicher Prüfung frischer Featurecommit und regulärer Selbstgate mit der unveränderten konfigurierten Kette: python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo <eigener absoluter Worktree> --base <frischer voller Bots-Main-SHA> --head <voller Feature-SHA>. Bei erneutem BLOCK an A für nächsten frischen Fixer, kein Modellroulette oder eigener Reviewthread. Kein ungekoppeltes Deploy: gemeinsame Abnahme mit laufendem A-E3f folgt durch A nach beiden ALLOWs.

Blatt-Worker A-E4f, alleiniger Ereignisproduzent, keine zusätzlichen Agenten/Reviewer/T3-Threads oder fremden Sessionnachrichten. Auftraggeber Paket A 2c7de4c9-bac4-43ad-b91a-f8ac889f09b4, Haupt 3fcd8f71-443e-48ae-825c-527eb52fbe56. Root-Akte schreibt nur A. Rückgabe SHA/Tree, konkrete Ursache/Fix, Prüfungen/Baseline, Gatewortlaut/Modell/Exit/Belege, tatsächliche Sourcekopplung und offene Liveabnahme. Wache 20 Minuten, spätestens 30; warme Compiler nicht kurz abbrechen.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-invite-bots-20261006
