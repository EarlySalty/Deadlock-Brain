# K: Discord-Logtest isolieren

## Ziel und Vertrag

Gemeinsamer regulärer Discord-Gate `56571e40..4af3776e`, Task bguflw0y7, Exit0 ALLOW. Log `/tmp/k-discord-consumer-combined-gate-20261007.log` tatsächlich gelesen. NIT: `dl-brain/src/brain_api.rs:536` installiert im Loggingtest dauerhaft einen globalen Subscriber. Parallele Tests können den Assertionbuffer füllen; eine spätere globale Installation scheitert. Nur diesen Test scoped isolieren, ohne seine Privacy-Assertions abzuschwächen. Die frühere with_default-Version funktionierte im eigenen Zwischenlauf nicht; beim Fix den tatsächlichen asynchronen Logpfad mitprüfen, kein globaler Ersatz.

## Eigentum und Arbeitsstand

Worktree `/home/nathanael/.worktrees/bots-k-guide-20261007`, Branch `feat/bots-k-guide-20261007`, HEAD4af3776e auf origin. Kein aktiver Bots-Writer. Ausschließlich Loggingtest und notwendige Testimports in `rust/crates/dl-brain/src/brain_api.rs` ändern. Produktionsfunktion, ModGlue, Provider, Modelle, Timeouts, Identität und Linkfilter unverändert. Kein zusätzlicher Provider oder Loggerprozess. Vor Suche Graphify, dann genau den Test lesen. Kein neues produktives Feature, keine neuen Code-Kommentare. Frischer Fixer, kein eigener Reviewer.

Kein Commit, Push, Merge, Deploy, Restart oder weitere Delegation. K integriert. Parallel aktive native Writer sind in Brain-K an zentralem Provider und an Artefakten, andere Dateien/Repo. Keine Sessionkontakte oder fremden Prozesse.

## Nachweis

Betroffenen Privacy-Logfall und vollständige bestehende dl-brain-Auswahl mit realem asynchronem Testpfad prüfen, nicht nur synchronen Subscriberfake. Assertions zur fehlenden Rohfrage und fehlenden Secrets erhalten; keine echten Nutzerdaten oder Secrets lesen. Compiler und scoped Format prüfen, passende strikte Clippy-Auswahl samt bestehenden drei exakten Baselinebefunden transparent melden. Vorherige Librarybaseline wurde mit Cargo/Rust1.99.0 gemessen, nicht andere Toolchains vergleichen. Exakte Befehle, Exit-Codes, passed/failed/ignored/filtered und Logpfade zurückgeben. Keine pauschale Altfehlerbehauptung oder Unterdrückung.

Absolute moderne Cargo-CLI, bestehender Cache und wirklich freier Buildslot `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/build-slot-{1,2,3}.lock`; höchstens `--jobs 3`. Bestehende Featureauswahl `dl-central-db/testing`. Kein fremder Compilerstop oder Targetlöschung. Kein Test-Exit hinter Pipe oder Marker verstecken. K fährt regulären Gate auf committed Fix, ohne Modelloverride.

## Sicherheit und Routing

NEVER read, print or write plaintext secrets.
MUST NOT send private user/community data to remote models.

Nur neutrale synthetische Testfrage, keine echte private Probe. Kein ListAgents, SendMessage, T3-Thread oder zentrale TODO-/Registeränderung. K-Session988eeaea-28ee-424c-b362-e250610cde91, teil-k, Versuch1; Delegator481426fe-b477-42b3-91c6-901811fcba1d, Hauptorchestrator d3a1741e-82bc-4a48-865b-2845c663dca7. Rückgabe nur bei Task-Ende oder echtem Blocker direkt als native Task-Rückgabe.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/bots-k-guide-20261007
