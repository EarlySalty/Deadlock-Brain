status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Worker Q5: priorisiertes redigiertes Anfrageereignis

## Ziel und Vertrag

`VON_HAUPT.md:24` beauftragt nach jeder authentifizierten Anfrage ein serverseitiges Ereignis mit Consumer-Kennung aus dem Credential-Grant, Request-ID, fester Route, Ergebnisstatus und Dauer. Keine Anfrage-/Antworttexte, Header, Credentials oder personenbezogenen Kontodaten. Ausgabe über Tracing in das Journal von brain-serve, vorhandenen Auditweg benutzen. K und Z brauchen dies vor G0 als Live-Beweis.

Die autorisierten C9-Commits sind bereits im eigenen Branch: `e48c189` aus `6d5d903` und `5c220a8` aus `32f6032`. Erfasse deshalb sowohl öffentliche `/v1/answer` und `/v1/retrieve` als auch den privaten Operatorrouter aus `brain-api/src/internal.rs`. Nicht authentifizierte Anfragen sollen keine Consumer-Ereignisse erhalten. Auch authentifizierte Parserfehler und fachliche Ablehnungen müssen einen kontrollierten Status liefern. Die Route stammt aus der Serverroute, nicht aus einem unkontrollierten URL-String.

Bestand: `brain-api/src/http.rs` dispatcht, `brain-api/src/lib.rs` und `internal.rs` authentifizieren. `brain-serve/src/main.rs` verwendet bisher `log_event` für Lifecycle und redigierte Fehler, kein erkennbarer Subscriber. Prüfe Bestand per Graphify zuerst, dann ergänze das engste passende Tracing. Keine breiten Dependency-Logs einschalten, die SQL-/Header-/Secret-Inhalte verraten könnten. Beschränke den neuen Subscriber auf den eigenen redigierten Ereignistarget.

Client-Request-IDs sind möglicherweise nicht als rein technische und begrenzte IDs garantiert. Niemals ungeprüfte Freitexte in Logs übernehmen. Verwende bei Bedarf einen stabilen Hash mit dokumentiertem Format als redigierte Request-Kennung; das erfüllt die Korrelation ohne Rohdaten. Consumer muss aus einem vertrauenswürdigen Grant kommen, nicht aus Body/Header-Freitext oder einer nutzerbezogenen Account-ID.

## Eigentum

Du schreibst nur `rust/crates/brain-api/` für Auditmechanik und eng passende Tests, nötigenfalls `rust/crates/brain-serve/src/main.rs`, eine eigene neue `audit`-/Tracing-Initialisierungsdatei und die erforderliche Modulexportzeile in `brain-serve/src/lib.rs`. Eigene Crate-Manifeste darfst du um den vorhandenen Tracingbaustein ergänzen. Kein Workspace-Manifest oder Lock. Keine Provider-, Feed-, Maintenance- oder Consumer-Dateien. Keine neuen Code-Kommentare, keine globale Formatierung.

## Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `5c220a8f047eb980d953d9f9f285b34739b5ed88`. Q1-Provider-WIP, Q2-Writer-WIP und Q3-CSV sind vorhanden und gehören anderen Schreibbereichen. Q2/Q3 werden fortgesetzt, Q4 bleibt bis zu einem freien Platz gestoppt. Keine Git-Mutationen, keine echten Requests, kein Deploy oder Dienstneustart durch dich. Native Workerrolle, keine weitere Delegation.

## Beweisziel

Gezielte Tests prüfen strukturierte Felder, öffentliche und private authentifizierte Anfragen einschließlich Fehlern, nicht authentifizierte Anfragen ohne Consumerlog und Redigierung bösartiger Request-ID/Freitext/Credentials. Keine wortlautgebundenen Nutzertexttests. Tests/Compiler ausschließlich Rust 1.97.1, maximal zwei Jobs und mit beiden Hostlocks plus frischer NonZombie-Probe nach `HOSTPROBE.md`. Nötige Manifeständerungen machen das Lock zunächst ungültig; dann den Bedarf konkret melden und noch nicht mit Cargo kompilieren. Hauptsession integriert das Lock und prüft vollständig. Ergebnis soll spätestens mit fertigem Code und eigenem Bericht zurückkommen, nicht auf andere Worker warten.

## Routing

Auftraggeber Teil-Orchestrator Q, Session `c671588c-6192-4bf5-8206-28bb30163666`; Hauptorchestrator `43a4886c-e135-484b-838a-0512d224a634`. Status ausschließlich `teil-q`, Versuch 1. Kein `TODO.md`/`REGISTER.md`. Bericht `bereiche/q/Q5-AUDIT.md` in `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung`, mit Logformat, Modulpfad, Manifestbedarf, Prüfbefehlen und Grenzen. Einziger Bug-/Securityreview bleibt der Merge-Gate.
