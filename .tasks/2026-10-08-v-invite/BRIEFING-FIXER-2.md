# V/F2: ausgelassene MCP-Argumente am bestehenden Statuspfad erlauben

## 1. Ziel und offener Vertrag

Frischer nativer Fixerkontext nach tatsächlichem gpt-6.1-sol-BLOCK auf Bots 08828c1aa83bdf186894419450f6a929f1244ad9. Vollständige Mängelliste in der eigenen Brain-Akte REVIEW.md, Originalgate /tmp/brain-v-invite-f1-20261008/bots-gate.log. Auftraggeber 481426fe-b477-42b3-91c6-901811fcba1d, direkte Rückgabe an V-Primary.

Bestätigt: mcp.rs:191 indiziert req[params][arguments] und produziert null bei Omission. self_invite.rs:75-76 lehnt null ab. MCP erlaubt fehlende arguments für einen parameterlosen Aufruf. Enger Fix im eigenen öffentlichen self_invite_status-Zweig: nur fehlenden Schlüssel auf {} normalisieren. Explizite ungültige Werte, nicht leere/fremde Parameter, fehlende/ungültige Personen-/Request-/Authbindung weiter sperren. Echten Endpunktfall mit omitted arguments ergänzen, sodass Zugriff auf den tatsächlich vorhandenen Loopbackrouter und realen eigenen Postgres-Testpfad läuft. Keine Lockerung des Statuslesers auf beliebige null-Eingaben.

Vorhandenen Source fortsetzen, kein Neubau. Neuere Requestzustände, Konto-/Taskbindung und minimaler Enum-/Zeitvertrag erhalten. Keine zweite Antwortengine, neuer Connector, Quotenübernahme, Steamaktion oder DB-Schreiben im Produktpfad.

## 2. Exaktes Eigentum

Alleiniger Produktwriter im eigenen Botsworktree /home/nathanael/.worktrees/brain-v-invite-consumer-20261008:

- rust/bin/dl-bot/src/mcp.rs: ausschließlich der enge eigene self_invite_status-Zweig für Argumentnormalisierung und direkte Testergänzung, falls dort erforderlich.
- rust/bin/dl-bot/src/mcp/self_invite.rs: ausschließlich nötige direkte Regressionen für das konkrete BLOCK-Delta oder durch echte Prüfung belegte eigene Compiler-/Testfehler des bereits beauftragten Statuslesers. Keine neue Fachfunktion.

Brain-Source 7c8ba75e ist bereits ALLOW und gepusht, nicht verändern. Brain lib.rs/provider_input/API/discord_live/hardening/Cargo und Bots modglue/dl-brain/Provider-/Quota-Dateien bleiben fremdes Eigentum. Keine globale Formatierung, neue Migration, fremde Testanpassung, Cargo-/Lockfileänderung, Testabschwächung oder Fake-Produktverdrahtung.

Eigene Rückgabedateien im Brainbaum .tasks/2026-10-08-v-invite/FIXER-2.md und REVIEW-FIXER-2.md, eigene sichere Logs /tmp/brain-v-invite-f2-20261008/. Primary-Dokumente und fremde Produktpfade weder ändern noch adden.

## 3. Arbeitsstand und Git

Botsbranch feat/brain-v-invite-consumer-20261008, HEAD 08828c1aa83bdf186894419450f6a929f1244ad9, Basis 8e1b8f03cf0d84f4754eedc8fe848ef0fceffce8. Produktstand sauber committed, Primary-AUFTRAG im eigenen Aufgabenordner untracked. Kein Featurepush dieses BLOCK-Stands. Brainworktree /home/nathanael/.worktrees/brain-v-invite-20261008 mit Primary-Dokument-WIP, dort keine Produktänderung durch F2.

Deinen engen Fix nach Prüfung gezielt auf dem vorhandenen Botsbranch committen, nicht neu anfangen. Git einzeln mit literalen absoluten Pfaden, kein add -A, Stash, Reset oder fremdes Cleanup. Co-authored-by: GPT 6.1 Sol <gpt-6.1-sol@local>. Regulärer Selbstgate gegen origin/main mit genau deinem Source-HEAD und bestehendem Urteilmodell gpt-6.1-sol. Vorhandener Befehl `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-v-invite-consumer-20261008 --base origin/main --head HEAD`, ohne Override oder Modellwechsel. Bei erneutem echten BLOCK konkreten Fund und Zustand zurückgeben; V startet je Runde frischen Fixer, du delegierst nicht.

Featurepush nach tatsächlichem passenden Compiler-/Testnachweis und ALLOW erlaubt. Bei weiterhin fehlendem Compilerbeweis nur lokal sichern und den echten Prüfblocker melden. Kein Mainmerge/-push, Deploy/Restart, Branch-/Worktreelöschung oder Settle: Teilstand bleibt ausdrücklich offen bis zur Shared-Restübergabe.

## 4. Beweisziel

Skills code-suche, rolle-test-waechter, rolle-merge-schleuse und passende Schreibregeln laden. Vor Codefragen Graphify, danach Source nachlesen. Rustfmt gezielt auf deinen Dateien mit passenden Flags. Passende Botsprüfung über cargo-slot +1.97.1, SQLX_OFFLINE=true, `--locked --offline --jobs 3`. Clippy für geändertes dl-bot-Ziel mit -D warnings. Bestehende sichere MCP-Tests mit `--no-fail-fast -- --include-ignored mcp::`, sowie Repo-Features nur soweit tatsächlich vorhanden.

F1 erreichte wegen besetzter Cargo-Slots weder Compiler noch Tests, Clippy ebenfalls ohne Ergebnis. Slotmechanik nicht umgehen, keine Prozesse oder Sperren anderer Sessions anfassen und keine Sessionkontakte. V hat Slotwrapper lesend bestätigt; er serialisiert drei Slots. Kein geteiltes Checkout bauen. Einen normalen geeigneten Test-/Compilerlauf starten, dessen Compilephase zugleich den neuen Code prüft, statt drei unproduktive getrennte Queuewartephasen zu wiederholen. Eigene sichere Postgres-Testbasis regulär anlegen, keine Produkt-DSN und keine realen Personendaten. Nur eigenen Wegwerfcontainer am Ende stoppen.

Prüfbefehl aus F1 als Formvorlage, neue tatsächlich eigene Scratch-DSN verwenden:

```text
SQLX_OFFLINE=true CENTRAL_TEST_DSN=<eigene-Wegwerf-DSN> /home/nathanael/.local/bin/cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-v-invite-consumer-20261008/rust/Cargo.toml -p dl-bot --bin dl-bot --locked --offline --jobs 3 --no-fail-fast -- --include-ignored mcp::
```

Volle sichere Logs in Datei, keine Pipe oder erfundenen Testmarker. Tatsächliche Anzahl passed/failed/ignored/filtered, Exitcode und Beginn oder Nichtbeginn der Compilephase melden. Ein Testlauf auf Basisstand zählt nicht als Beweis des neuen Sources. Keine künstliche Testpflicht, bestehende Suites nicht brechen oder abschwächen. Wenn reale eigene Regression rot ist, im freigegebenen Scope beheben; bei echtem fremdem Grund erst Zahl-gegen-Zahl-Baseline oder eng belegten Blocker liefern, nicht pauschal Altfehler behaupten.

Eigene weitere Compiler-/Harnessfehler dieses Statusschnitts dürfen eng und belegt mitgefixt werden. Architektur-/Shared-Abweichungen sind Stopgrund an V, keine Nutzerfrage. Gate ALLOW ist weder Compilerbeweis noch fertige Liveantwort.

## 5. Routing und Rückgabe

Paket V/F2, Versuch 2, frischer nativer Fixer. Keine weiteren nativen Subagenten oder T3-Threads, keine Fremdsessionkontakte. Nach spätestens 25 Minuten prüfbaren Teilstand und echte Blocker liefern; nicht drei volle 600-Sekunden-Queuewartungen nacheinander. Selbstgate und Rückgabedokumente gehören in deinen Lauf. V hält REGISTER/STATUS und zentrale TODO bleibt beim Delegator.

Browser unnötig. Falls erforderlich ausschließlich Moli nach Guide, Brave MUST NOT verwenden. Secrets NEVER ausgeben, Rohdaten MUST NOT in Git oder Codiermodellkontext. Keine Discord-/Steam-IDs, Mitgliederlisten oder fremden Daten ans Modell, keine echte Community-/Nutzerdatenprobe. Deutsch mit Umlauten, keine Em-Dashes.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-v-invite-consumer-20261008
