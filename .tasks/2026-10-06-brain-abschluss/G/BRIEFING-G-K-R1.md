# G-K-R1: Typisierte Fehlerabrechnung für Werkzeugturns

status: tatsächlich gestartet als Task `w7kbtvxfg`, Run `wf_18a32653-098`, Agent `a479471c9d2ac757f`, nach tatsächlichem JSON-Abschluss `wvlm6fn18` / `wf_89eb7717-56e`. Kein paralleler Brain-Vertrags- oder Providerschreiber. Vorgänger hat 71 Vertrags- und 32 Providerfälle geprüft, danach aber noch finale JSON-Duplikatablehnung ohne Belege ergänzt. Diese letzte Ergänzung ist vor Produktabnahme mit Compiler und Tests zu prüfen; 103 ist kein aktueller Gesamtbeweis. Source-/Verbraucherläufe trafen während G-M-WIP auf noch fehlende Combathelfer; fremde Dateien nicht bearbeiten oder zurücksetzen.

## 1. Ziel und Vertrag

Begrenzten G-K-Anschlussbedarf aus `G/G-K-NACHWEISE.md` lösen. `PortError` trägt bislang nur Fehlerart, aber keine gemessene Usage fehlgeschlagener `AnswerProviderPort::answer_turn`- und `ToolExecutionPort::execute`-Aufrufe. Der Kernel kann deren Verbrauch nicht vollständig ausweisen. Der ursprüngliche G-K-Worker ist beendet, sein bestehender Werkzeugloop/Cache bleibt Grundlage. Lies tatsächliche JSON-Worker-Rückgabe, `G/PLAN.md` C3 und Abschnitt 8, `G/G0-0645-VERTRAG.md`, `G/G-K-NACHWEISE.md` und `G/G-P-NACHWEISE.md`.

Ergänze den kleinsten kompatiblen typisierten Fehlerabrechnungsvertrag im bestehenden Brain-Vertragsbereich. Nach Graphify-Nachlesen: `brain-providers/src/transport.rs` besitzt bereits `Charge` und `charged_usage`; erfolgreiche Turns belasten dort fehlgeschlagene Retries konservativ. `ProviderError` und `PortError` verlieren beim Fehlerübergang dagegen die Abrechnung. Diese bestehenden Bausteine erweitern, keinen zweiten Zähler oder Ledger bauen. Tatsächlich beobachtete Provider-/Tool-Usage muss zusammen mit Fehlerart durch den bestehenden Provider und Kernel getragen werden. Bestehende Text-/Domainaufrufer und Fehlerzuordnungen erhalten; keine Nutzung einer globalen Nebenvariable, neuer Telemetriepipeline oder Modellargumente. Serverseitige ursprüngliche Deadline, Kontext und Anfragebudgets bleiben unverändert.

Fehlerpfade nach externem Aufruf dürfen keine verbrauchten Runden oder belegte Usage verlieren. Retries, Parser-/Validierungsfehler nach kostenpflichtiger Antwort, Budgetüberschreitung in Folgeturn und Fehler eines späteren Toolcalls nach vorherigem Erfolg gehören in dieselbe kumulierte Abrechnung. Vor dem Fremdaufruf verweigerte lokale Validierung von tatsächlich erfolgtem Fremdaufruf unterscheiden. Fehlende oder unzuverlässige Modell-Usage nicht als gemessene Null ausgeben; konservative Reservierung und wirklich beobachtete Werte bleiben unterscheidbar. Fehlende Abrechnung fail closed behandeln, kein erneuter Aufruf mit frischem Budget und keine erfundene genaue Schätzung. Prüfbare Netzrunden und Byte-/Inputobergrenzen sind keine behaupteten echten Modell-Tokenzahlen.

Keine Änderung an Modellen, Provideridentität, Preiseinstellungen, Timeouts, ENV-Konfiguration oder öffentlichen API-Anfragefeldern. Acht Werkzeuge und die gerade korrigierte duplikatsichere JSON-Strecke erhalten. Kein echter G-V-Dispatcher, E-Importer oder neues Spielwerte-Lesemodul. Grafiken/Webseiten baut der Nutzer separat; G liefert strukturierte Zahlenreihen ohne Roadmapeintrag.

## 2. Eigentum

Nach bestätigtem JSON-Abschluss exklusiv `rust/crates/brain-contracts/src/{lib.rs,tools.rs,provider_input.rs}` und direkte Vertragsprüfungen; `brain-providers/src/{lib.rs,transport.rs,hardening.rs}` und unmittelbar betroffene Tests; `brain-kernel/src/{lib.rs,execution.rs,flight.rs,cache.rs,outcome.rs}` und unmittelbar betroffene Tests. Nur notwendige Fehlerabrechnung; keinen weiteren Refactor. `dbrain-sources`, Manifeste, Policy/API, Serve, Retrieval, Reasoner und Konfiguration nicht ändern. G-M arbeitet parallel ausschließlich Reasoner.

Graphify vor Bestandssuche, danach gefundene Stellen nachlesen. Kein eigener Reviewer und keine zweite Parser-/Cache-/Providerstrecke. Rust only, keine neuen Code-Kommentare, Modelle, festen Timeouts, ENV-Schalter, Dienste oder Crates. G-P-R1-Rohbelege nicht überschreiben; eigene neue Belege unter `G/pruefungen/g-k-r1/`.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, vorbereiteter Dokument-HEAD `ce21a457`. Tatsächlichen HEAD und Vorgängerabschluss vor Start nachsehen. Provider-/Kernel-WIP ist uncommittiert, vorhandenen Stand fortsetzen und nichts zurücksetzen. Kein Git, weitere Agenten/Workflows/T3-Threads oder Sessionnachrichten. Keine Produktions-DB, Liveprovider, Releasebuilds, Deploy, Neustart, Tick, Cleanup oder Settle. Release-Hold durch Hauptsteuerung 09:00 aufgehoben. Bereichsführung schließt nach eigener Abnahme/Gate auf dem aktuellen origin/main nach E/F ab; Worker bleibt ohne Git-/Runtimewirkung. Bestehender Buildslot, eigener Debugtarget, höchstens drei Cargo-Jobs.

## 4. Beweisziel

Reproduzierbarer Fehler nach gemessener Provider- und Tool-Usage muss vollständig am Kernelresultat erscheinen. Erfolg plus späterer Fehler kumulieren, Wiederholungen unter gleicher Deadline/Restbudget, validierungsbedingte Ablehnung vor Netzstart ohne erfundenen Verbrauch, nicht zitierte Toolabhängigkeiten weiter freigabeprüfen. Budget- und Quellenfehler bleiben getrennte Kategorien. Kein Cacheeintrag aus fehlgeschlagener oder unbelegt abgerechneter Anfrage. Mehrere Call-IDs und beide Wireformen bleiben gebunden; doppelte JSON-Felder weiter abweisen. Isolierte Fixtures sind für Kernelzustände erlaubt, nicht als echte Luna-Probe ausgeben. Keine Wortlaut- oder echte Wall-Clock-Tests.

Passende Compiler-, Format-, strikte Clippy- und betroffene bestehende Suiten prüfen. Vorheriger Kernelstand: 49 passed, dieselben 18 failed, 0 ignored gegenüber Baseline 34 passed/18 failed; nicht als grüne Gesamtsuite ausgeben. Provider/Vertragszahlen stammen vom tatsächlich abgeschlossenen JSON-Worker, nicht vom alten 29-/67-Stand übernehmen. Vollständige Befehle/Logs, unverdeckte Exits, tatsächliche Zahlen und Quellenfingerprints. Bestehende Tests nicht löschen, ignorieren oder abschwächen. Bereichsführung erstellt verifizierten Teilcommit und fährt `gate_hook.py --review`; bei BLOCK frischer Fixer. Keine technische Gateausnahme.

## 5. Routing und Rückgabe

Auftraggeber native G-Bereichsführung, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Nach belegtem Start genau ein Worker als Statusproduzent. Register, AN_HAUPT und TODO bleiben bei Führung. Wache nach 20 Minuten. Rückgabe exakter kompatibler Vertrag, konsumierende Provider-/Kernelpfade, echte Fehlerabrechnungsbelege, Befehle und Testzahlen, übrige G-V-/E-/Luna-Grenzen. Keine Nutzerfragen. Deutsche Produkttexte mit echten Umlauten, ohne Gedankenstriche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
