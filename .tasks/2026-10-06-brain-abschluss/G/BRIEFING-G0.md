# G0: Gemeinsamer Werkzeugvertrag

## 1. Ziel und Vertrag

Baue die minimale gemeinsame Vertragserweiterung für native Modellwerkzeuge im bestehenden Brain-Provider. Keine zweite Antwortstrecke. Grundlage: `BRIEFING-G.md`, Hauptsteuerung 05:40, `G/BESTAND-ANTWORT.md`, `G/SHEET-MODELL.md`. Der Planworker schreibt parallel ausschließlich `G/PLAN.md`; G0 ist dessen unabhängige Voraussetzung, kein weiterer Planauftrag.

Bestehend: `brain-contracts/src/lib.rs:510` definiert `AnswerProviderPort::answer`; `provider_input.rs` definiert Textnachrichten, gemeinsame Promptdarstellung und konservative Eingabezählung. Wiederverwenden, keine zweite Zählformel daneben. Benötigt: Tooldefinition, Aufruf-ID/Name/JSON-Argumente, typisierte Modellblöcke, zugehörige Ergebnisse, vollständiger Providerturn mit Abschlussgrund und Usage. Sieben stabile Namen: `entity_find`, `entity_profile`, `hero_compare`, `damage_calculate`, `patch_history`, `build_plan`, `server_knowledge`.

Bestehende Provider-/Mock-Implementierungen müssen weiter kompilieren. Ein ergänzender Turn-Port mit sicherem Default ist zulässig; unbekannte Tools, ungültige Aufruf-/Ergebniszuordnung und vom Modell gelieferte Actor-/Rechte-/Releasebindungen werden nicht als vertrauenswürdig behandelt. Der Kernel, nicht das Modell, besitzt AuthorizedContext und ursprüngliche RequestDeadline. Keine Erneuerung im Gesprächsvertrag.

Eingabezählung zählt wiederholten Gesprächskontext, Tooldefinitionen, Toolnamen/-IDs, Argumente und Ergebnisse einschließlich nativer Blöcke und OpenAI-kompatibler Darstellung. Checked Arithmetic, unbekannte Blöcke sichtbar ablehnen, keine ungezählten Felder. Text- und Embedding-Bestand muss unverändert funktionieren. Finale Antworten behalten `text`, `cited_evidence_ids` und Usage; Werkzeugaufrufe sind keine fertige Antwort.

## 2. Eigentum

Ausschließlich `rust/crates/brain-contracts/src/lib.rs`, `src/provider_input.rs` und ein neues schmales `src/tools.rs` sowie nötige Tests innerhalb dieses Crates. Keine anderen Produktdateien, Cargo.lock, globales Formatieren oder Refactorings. G0 besitzt diese Dateien bis zur Übergabe; weitere Bauworker starten dort noch nicht. Keine Änderungen an E/F-Dateien oder Importern. Rust only, Postgres only, keine neuen Code-Kommentare, ENV-Konfiguration, Modelle oder festen Timeouts.

Vor der Bestandssuche Skill `code-suche` und Graphify. Worktree hat keinen eigenen Graph; vorhandenen Brain-Graph explizit über `--graph /home/nathanael/repos/Deadlock-Brain/graphify-out/graph.json` lesen. Kein Extraktionslauf.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, HEAD `96e6a8da`, Basis `bfda408c`. Uncommittiert liegen ausschließlich eigene G-Recherche- und Aufgabenunterlagen. Diese erhalten. Kein Commit, Push, Merge, Cleanup, DB-Schreiben oder Runtime-Eingriff durch G0; Bereichsführung integriert und sichert. Kein weiterer Agent, Workflow oder T3-Thread durch den Worker.

## 4. Beweisziel

Das Vertragscrate sowie bestehende Verbraucher kompilieren. Passende Format-, strikte Clippy- und bestehende Vertragsprüfungen ausführen, mit vollständigen Befehlen, Exitcodes und Testzahlen zurückgeben. Eigener Target außerhalb fremder Buildpfade; bestehende CPU-/Build-Serialisierung beachten. Keine Release-Builds. Runde als begrenzt und vollständig abliefern; keine weiteren Architekturvarianten und keine Nutzerfrage.

`gate_hook.py --review` ist der einzige Bug-/Securityreviewer. G0 meldet prüfbaren Diff und Nachweise; die Bereichsführung schneidet und fährt das Gate vor Sicherung/Integration. Keine eigenen Reviewagenten. Technischen Gate-Ausfall nicht umgehen. Bestehende Baseline: `G/BASELINE.md`, 474 passed und 38 failed über vier Pakete; keine Fehler ohne Vergleich als alt bezeichnen.

## 5. Routing und Übergabe

Auftraggeber: native Bereichsführung G, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Kommunikation als native Workerrückgabe, nicht Sessionnachricht. Statusproduzent dieses G0-Versuchs: Worker. Optional eigener knapper Vertrag `G/G0-VERTRAG.md`, keine Register-/AN_HAUPT-/TODO-Änderung. Wache spätestens nach 20 Minuten. Rückgabe nennt Dateien, exakte Typen/Funktionen, sichere Defaults, Prüfungen, offene Grenzen und den kleinsten Anschluss für Provider und Kernel. Deutsche Produkttexte mit echten Umlauten, keine Gedankenstriche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
