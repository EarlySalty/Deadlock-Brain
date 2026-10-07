status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Worker Q1: vorhandenen Antwortprovider vervollständigen

## Ziel und Vertrag

Paket Q nach `BRIEFING-Q.md` und `AUFTRAG.md`: offizieller Fireworks-DeepSeek-Flash-Provider, Modell aus zentraler Auswahl statt festem Namen, Denken bei kleinem Antwortbudget aus, konsistente konfigurierbare großzügige Frist. Kein neuer Connector oder Anbieter. Referenz: `deadlock-brain-core/src/model_resolver.rs`, `rust/vendor/fireworks-model-selection`, `brain-providers/src/lib.rs`, `brain-serve/src/service.rs`, `brain-serve/src/config.rs`, `brain-kernel/src/execution.rs`. Bestandssuche ist durchgeführt; vor weiterer Codesuche Graphify laden und den Brain-Graph unter `/home/nathanael/.graphify/projects/deadlock-brain/graphify-out/graph.json` verwenden.

Der aktuelle laufende Dienst nutzt bereits 60 Sekunden Requestfrist und 55 Sekunden Providerfrist. Diese Fristen sind Felder, keine neue feste LLM-Grenze. Repariere zuerst den zentralen Modellpfad und das Requestfeld für Denken aus. Breite Änderungen an Fristobergrenzen sind ohne konkreten Nachweis nicht nötig. Offizielle Fireworks-Aufrufe müssen den vorhandenen zentralen Auswahlreader benutzen und bei fehlender gültiger Auswahl scheitern; synthetische lokale Providerprüfungen behalten die ausdrücklich übergebene Konfiguration. Keine Modellnamen einkompilieren.

## Eigentum

Du schreibst ausschließlich `rust/crates/brain-providers/`, nötigenfalls `rust/crates/brain-serve/src/config.rs` und `rust/crates/brain-serve/src/service.rs`, sowie eine eng passende Providerprüfung in `brain-serve/tests`. Keine anderen Crates, keine Wiki-/Replay-/Feed-Dateien, keine Workspace-Manifeste oder Locks. Die eigene Crate-Manifestabhängigkeit auf den bestehenden Auswahlreader ist erlaubt. Keine neuen Code-Kommentare, keine globale Formatierung.

## Arbeitsstand

Branch `feat/brain-fertig-q-20261003`, HEAD `511a347b653beba13c2bf130f4bead7a7196cc2a`. Eigenes WIP der Hauptsession: `architecture/migration/evals/q-core-baseline-20261003.json`. Nicht anfassen. Keine Commits, Pushes oder Merges durch dich. Kein Deploy und keine live API-Aufrufe. Du bist ein nativer Worker und delegierst nicht weiter.

## Beweisziel

Compiler- und vorhandene Provider-/Serviceprüfungen nachziehen. Neue gezielte Prüfung kann den tatsächlich serialisierten offiziellen Request samt `reasoning_effort` und zentralem Modellwechsel belegen, ohne Netzwerkmodell. Rust 1.97.1, höchstens zwei Jobs. Vor jedem Compiler/Test beide Sperren blockierend halten: `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock`, danach `/tmp/deadlock-cargo-release.lock`. Unmittelbar vorher NonZombie-Compilerprobe gemäß `HOSTPROBE.md`; bei Fremdcompilern Sperren halten und nach 30 Sekunden erneut prüfen. Tests über Skill `rolle-test-waechter`, Logs vollständig speichern, Zahlen und Exitcode melden. Kein `cargo --locked`, solange eine nötige eigene Manifeständerung noch keinen passenden Lock hat: stattdessen Hauptsession über die nötige Lockaktualisierung informieren und noch nicht kompilieren. Die Hauptsession integriert das Lock.

## Routing

Auftraggeber Teil-Orchestrator Q, Session `c671588c-6192-4bf5-8206-28bb30163666`; Hauptorchestrator `43a4886c-e135-484b-838a-0512d224a634`. Status allein durch `teil-q`, Versuch 1. Nie `TODO.md` oder `REGISTER.md` schreiben. Ergebnisdatei: `bereiche/q/Q1-PROVIDER.md` im gemeinsamen Auftragsordner `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung`. Bericht enthält Änderungen, Prüfungen, Lockbedarf und offene Grenzen. Keine eigene Bugreview-Rolle; der einzige Bug-/Security-Reviewer bleibt der Merge-Gate.
