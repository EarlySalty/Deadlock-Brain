status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Q6: erhaltenen Consumerstand lokal prüfen

## Ziel und Vertrag

Native Workerrolle, keine weitere Delegation. Prüfe ausschließlich den gelieferten C9-/Auditstand, damit Q einen lokal geprüften Consumercommit an Z übergeben kann. Übernahme und neue Integrationsgrenze stehen in `UEBERNAHME-CODEX.md` und `bereiche/q/VON_HAUPT.md` der gemeinsamen Akte. Z übernimmt gemeinsame Integration, unabhängige Abnahme, Gate und Deployment. Keine Einzelmerges und keine Produktivänderungen durch dich.

Erhalten sind C9-Cherry-picks `e48c189` und `5c220a8`, redigiertes Anfrageereignis in brain-api/brain-serve und integriertes Cargo.lock. Provider ist lokal geprüft: 20 bestandene Tests und striktes Clippy Exit 0. Das frühere Consumerwrapperlog endet mit killed, die erwarteten drei Logs fehlten um 16:09:45 UTC. Eine eigene Prozesszuordnung um 16:14:52 UTC zeigte keinen Cargo-, Prüf- oder Lockwrapper im Q-Worktree. Vor dem Start erneut sicherstellen, dass kein erhaltener eigener Prüflauf bereits arbeitet. Keine lebende Arbeit duplizieren, keine fremden Prozesse stoppen.

## Eigentum

Prüflogs nur im Q-Worktree: `.q-consumer-api-tests.log`, `.q-consumer-lib-tests.log`, `.q-consumer-build.log`, `.q-consumer-clippy.log`, `.q-consumer-fmt.log`. Bericht ausschließlich `bereiche/q/Q6-CONSUMER-PRUEFUNG.md` im gemeinsamen Taskordner. Falls ein echter neuer Compiler- oder Linterfehler auftritt, darfst du den engsten Fix in `brain-api` oder der Auditinitialisierung `brain-serve/src/audit.rs`, `src/main.rs`, `src/lib.rs` vornehmen. Keine sonstige Codeprüfung als eigener Bug-/Securityreview, das bleibt beim Gate. Keine Provider-, Writer-, Shadow-, Manifest-, Lock-, Config-, Operatortransport- oder Maintenanceänderungen; erforderlichen fremden Fix mit genauer Fehlermeldung melden. Keine neuen Code-Kommentare und keine globale Formatierung.

## Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `5c220a8f047eb980d953d9f9f285b34739b5ed88`. Uncommittierte Provider-, API-/Audit-, Feed-, Source-, CSV- und Evaländerungen gehören Q und bleiben erhalten. Keine Git-Mutation, kein Deploy, Neustart, DB-Schreibzugriff oder echter Anbieteraufruf. Das Shadowwerkzeug ist als neuer Binärpfad geliefert, aber noch nicht kompiliert; vermeide für diese priorisierte Prüfung unnötige all-targets-Aufrufe.

## Beweisziel

Lade rolle-test-waechter, code-suche, humanizer und no-em-dashes. Codefragen zuerst Graphify, vorhandener Brain-Graph `/home/nathanael/.graphify/projects/deadlock-brain/graphify-out/graph.json`. Lies HOSTPROBE.md unter `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/`. Beide Hostlocks blockierend in genau dieser Reihenfolge halten, danach frische NonZombie-Probe, RAM und Platte prüfen. Höchstens zwei Jobs, Rust 1.97.1. Die enge Ausnahme für exakt belegtes cargo metadata --format-version 1 --no-deps --manifest-path /absoluter/Pfad/Cargo.toml darf nach dem vorhandenen Vertrag klassifiziert werden; unbekannte Argumente bleiben blockierend. Keine vollständigen Argumentlisten oder Prozessumgebungen ausgeben.

Prüfe nacheinander unter den Locks:

1. `/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml -p brain-api --test request_audit --test c9_internal_http --locked --offline --jobs 2 -- --include-ignored`
2. Derselbe Cargo-/Manifestpfad, `test -p brain-api -p brain-client -p brain-serve --lib --locked --offline --jobs 2 -- --include-ignored`.
3. Derselbe Pfad, `build -p brain-serve --bin brain-serve --locked --offline --jobs 2`.
4. Striktes Clippy für diese Librarys und gezielt brain-serve: `clippy -p brain-api -p brain-client -p brain-serve --lib --locked --offline --jobs 2 -- -D warnings`, zusätzlich `clippy -p brain-serve --bin brain-serve --locked --offline --jobs 2 -- -D warnings`.
5. Gezielt die gelieferten Auditquellen und Tests mit rustfmt --check prüfen, nicht fremde Crates umformatieren.

Volle Logs, keine Head-/Tail-/Grep-Pipe im beweisenden Lauf. Bericht nennt jeden Exitcode, Testzahlen und ausdrücklich die tatsächlich erfassten Grenzen. Keine grüne Prüfung bei Timeout, unvollständigem Log oder laufendem Kind behaupten. Bei Timeout Prozesse erhalten und ihre eigenen PIDs/Logs zurückgeben; den Auftrag nicht neu bauen. Keine eigenen ignorierten Tests still überspringen. TESTNACHWEIS[TW-1] verwenden, keine Altfehler ohne gemessene Baseline behaupten.

## Routing

Auftraggeber Teil Q, Paket q, Versuch 1, Produzent teil-q. Hauptorchestrator seit Übernahme Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Bericht direkt an diese native Hauptsession und in die eigene genannte Datei. Keine Sessionkontakte, keine zusätzlichen T3-Threads, kein TODO.md oder REGISTER.md, keine Statusereignisse. Q meldet Status und übergibt erst nach deinen echten Prüfergebnissen den vollen eigenen Commit-SHA an Z.
