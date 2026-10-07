# K: Frische Budgetfixrunde nach tatsächlichem Main-Gate-BLOCK

Status: historische ausgeführte Runde aa7d5a4eac642acdc, Worker abgeschlossen. Drei Source-/Teständerungen uncommitted erhalten; noch kein Folge-ALLOW. Befehls-/Führungs-/Datenschutzvorgaben unten sind der damalige Briefingstand, keine aktuelle Startfreigabe. Neue bestätigte Regeln in HANDOFF.md: cargo-slot, gestoppter d3a1741e bleibt gestoppt, korrigierte minimale Luna-Testfreigabe. Tatsächliche neue Prüfungen in PRUEFWEG-CARGO-SLOT.md; keine alte Runnerform wiederverwenden.

## Ziel und verbindlicher Befund

Regulärer Gesamtgate ca4d877f..9fc48a08, Task blmbykgsx, tatsächliches Urteil gpt-6.1-sol BLOCK: Provider input ceilings undercount the payload enforced by transport. Log /tmp/k-brain-main-closure-gate-20261007.log. Beide Zwillinge: grounded_input_ceiling in brain-contracts/src/provider_input.rs:102 und grounded_turn_input_ceiling:187 zählen ohne die später in brain-providers/src/lib.rs:263 beziehungsweise :273 eingefügten Steuerfelder. Native tool_choice plus output_config werden mit 31 Bytes gezählt; kompatibles tool_choice mit sechs. Damit kann ein korrekt erscheinend gepackter Request vor HTTP BudgetExceeded liefern.

Ursache am gemeinsamen Payloadpfad beheben. Gezählt werden muss derselbe gesteuerte Inhalt, der gesendet wird. Bestehenden zentralen Payloadbauer und Transportzähler wiederverwenden, keine zweite Wire-/Parserimplementierung und keine kopierte magische 31-/6-Reserve. Vorhandene Wireoptionen unverändert erhalten. Legacyhelper ohne Formatparameter muss beide tatsächlich verwendeten Wireformen konservativ abdecken; Turnhelper muss die tatsächliche gewählte Form abbilden. Budgets weder erhöhen noch Prüfungen abschwächen. Kein Modell-, Timeout-, Pricing-, Config-, Rechte-, Datenschutz- oder Produktportwechsel.

## Eigentum und Arbeitsstand

Frischer nativer Fixer, nicht der vorige Site-Lintworker. Eigener Worktree /home/nathanael/.worktrees/brain-k-ki-20261007, eigener detached Mainkandidat 9fc48a08a409cd3e049bc9f26500d5daa42e4711. Keine aktive Sourcearbeit. Sourcecheckpoint auf origin/feat/brain-k-ki-20261007 gesichert. Brain-main bleibt ca4d877f, noch nicht gepusht.

Schreibbereich ausschließlich eigene übernommene brain-contracts/src/provider_input.rs und brain-providers/src/lib.rs sowie eng betroffene vorhandene synthetische Tests in brain-providers/tests/faults.rs oder brain-serve/src/answer_provider_tests.rs, falls für die Schnittstelle zwingend. Keine fremde G-Datei, kein G-Worktree, kein Kernel-with-tools, keine G-V-Port- oder Rechenimplementierung. Keine neuen Code-Kommentare. Nicht in NITs ausweichen. K allein schreibt Git, committed, prüft den tatsächlichen SHA mit dem regulären Gate und pusht.

## Beweisziel und Stopgrenzen

Erst Graphify, dann beide Helpers und echten Request-/Transportpfad lesen. Den vollständigen konkreten Kontrollfeldern und beiden Wireformen nachgehen. Vorhandene synthetische Budgetfälle erhalten oder eng verstärken, nichts abschwächen oder überspringen. Keine neuen Tests als allgemeine Pflicht. Keine Ausführung oder Variation der bisher verweigerten Testkommandoform. Compiler, kontrollierter Formatcheck und striktes scoped Clippy mit Cargo/Rust 1.97.1, absolutem eigenen Manifest, SQLX_OFFLINE=true als Prüfkonfiguration für Compiler/Clippy, `--locked --offline --jobs 3`, `--all-targets --no-deps -- -D warnings` für Clippy. Buildslot 3 halten. Ein eigener Twitch-Releaseprozess läuft auf Slot 1, nicht berühren. Bei Schutzablehnung keine Wiederholung, andere Toolroute oder Settingsänderung.

Diff und tatsächliche Exits/Logs zurückgeben. Gate kann committed SHAs prüfen, daher keine uncommittierte Änderung als ALLOW ausgeben. K committed nach eigener Diff-/Belegprüfung und fährt die verpflichtende reguläre Folgerunde mit derselben unveränderten Gatekette. Gebaut, getestet, reviewt und live getrennt.

## Routing und harte Grenzen

Nativer Fixer, geerbtes Modell, high. Produzent teil-k, Versuch 1, Session 988eeaea-28ee-424c-b362-e250610cde91. Delegator 481426fe-b477-42b3-91c6-901811fcba1d; Hauptorchestrator d3a1741e-82bc-4a48-865b-2845c663dca7. Keine zusätzliche Orchestrierung, T3-Threads, Sessionkontakte, zentralen Register-/TODO-Eingriffe oder Nutzerfragen. NEVER read, print or write plaintext secrets. MUST NOT send private user/community data to remote models. Keine private Probe oder Veröffentlichung; echte G-Pins/Verifier und Datenschutzentscheidung bleiben offen.
