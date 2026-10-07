status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/steam-publish-fertig

# Minimaler Steam-Lockfile-Abgleich und gebundene Publish-Prüfung

## Ziel und Vertrag

Steam-Publish-Code steht auf 9aec0cc897b01b74d417ab9b510314cbbbd02535. Die bisherigen Tests liefen nicht: --locked verweigerte einen nötigen Lockfile-Abgleich. Der äußere Werkzeuglauf meldete trotzdem Exit 0; das ist kein Prüferfolg. Auftraggeber hat den minimalen Abgleich in der gemeinsamen Akte bereiche/s/VON_HAUPT.md:32-36 ausdrücklich erlaubt.

Lies diese Entscheidung zuerst. Graphify vor Bestandssuche, dann vorhandene Fundstellen prüfen. Der Workspace nutzt dl-central-db aus ../../Deadlock-Bots/rust/crates/dl-central-db. Beim letzten Lesen löste das auf /home/nathanael/.worktrees/open-pr-472-community-bridge-20261001 auf. Das dortige Manifest verlangt chrono-tz, der Steam-Lockeintrag enthält es noch nicht.

## Eigentum

Nur rust/Cargo.lock im bestehenden Steam-Worktree schreiben. Die beiden Publish-Dateien ausschließlich lesen. Kein allgemeines cargo update, keine ungefragten Versionssprünge. Fremde Deadlock-Bots-Dateien und gemeinsame Verknüpfung nicht ändern. Keine neue T3-Session, keine Unteragenten, keine globale Formatierung, kein main-Merge, kein Deploy oder Publish. Secrets weder klar lesen noch ausgeben oder schreiben, keine Prozessumgebungen lesen.

## Arbeitsstand und Bindung

Steam /home/nathanael/.worktrees/steam-publish-fertig, feat/steam-publish-fertig-20261003, HEAD 9aec0cc897b01b74d417ab9b510314cbbbd02535. Eigenen tatsächlichen Stand und den aufgelösten Dependency-Pfad samt vollem SHA vor dem Lauf erheben. Relevante uncommittierte Änderungen und Quellfingerabdruck erfassen. Bei veränderter fremder Basis nach dem Lauf gilt der Test nicht als SHA-gebunden; nicht als grün melden. Bei dauerhaft beweglicher Basis erlaubt der Hauptorchestrator einen eigenen unveränderten isolierten Dependency-Checkout des belegten SHA für diese Prüfung, ohne globale Links umzuhängen. Keine produktive Quelle aus dem fremden Worktree ersetzen.

GPT-6.1 Sol erben, xhigh. Brain wird von einem anderen eigenen Fixer geschrieben; nicht anfassen. Z prüft den gemeinsamen finalen Dependency-Stand später erneut. Einen minimalen Feature-Lockcommit mit Modelltrailer und Push nach feat/steam-publish-fertig-20261003 darfst du nach eigener erfolgreicher Prüfung erstellen. Keine main-Mutation.

## Prüfvertrag und Fehlerweitergabe

Alle Cargo- und Rust-Hilfsprüfungen unter beiden blockierenden Sperren: erst /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock, danach /tmp/deadlock-cargo-release.lock. Keine Zeitlimits für reines Lockwarten. Nach Erwerb frische NonZombie-Probe, exakte Metadata-Ausnahme nach HOSTPROBE.md, höchstens zwei Jobs. Bei Fremdcompilern beide Sperren halten und nach 30 Sekunden erneut prüfen. Keine fremden Prozesse stoppen. Nach Abschluss eigene Kinder und Freigabe belegen.

Verwende /home/nathanael/.cargo/bin/cargo 1.99, PATH zuerst /home/nathanael/.cargo/bin, SQLX_OFFLINE=true, CARGO_BUILD_JOBS=2, -j 2. Erforderliche Lockeinträge aus der vorhandenen Auflösung ergänzen, ohne allgemeines Neuauflösen aller Versionen. Nach dem Abgleich endgültige Testläufe wieder mit --locked.

Bestehende Suites: steam-core --lib publish_original::tests; steam-web --lib routes::builds::tests, beide mit --include-ignored. Der Web-Test postgres_concurrent_publish_survives_reconstructed_state nutzt die vorhandene automatisch migrierte Wegwerf-DB und prüft ihren Namen. Keine Produktions-DSN einsetzen und kein DB-Testskip als Erfolg melden.

Eigener Prüfwrapper muss Cargo-Fehler weitergeben. Nach jeder Ausführung den tatsächlichen Cargo-Exit übernehmen und unabhängig davon die Logresultate prüfen: kein Erfolg ohne positive passed-Anzahl oder bei error:-Zeilen. Volle Logs in bereiche/s/pruefung-v2/steam-lock-* der gemeinsamen Akte. Keine Pipe hinter Cargo, kein abschließendes Echo als Exit. Der Compiler- und Testbeweis braucht tatsächliche Zahlen, nicht allein den äußeren Werkzeugstatus.

## Rückgabe

Bericht als Rückgabewert an Teil-S2: voller eigener SHA, exakter minimaler Lockdiff, aufgelöster Dependency-Pfad, voller Dependency-SHA und Fingerabdruck vor/nach, echte Testanzahlen mit Exit-Codes, Nachweisorte und offene Grenze. Frischer unabhängiger Lockdiff-Prüfer und abschließende Gateabnahme folgen beim Teil-Orchestrator. Nicht selbst reviewen oder weitere Agenten starten.

Auftraggeber Codex /root T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Status allein teil-s2 unter status/s/2. TODO.md und REGISTER.md nicht ändern. Natürliches Deutsch, humanizer und no-em-dashes.
