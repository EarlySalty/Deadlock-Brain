# K: Fachübergabe an Hauptorchestrator

Status: aktiv, Versuch 1. Auftraggeber: a711a4d2-1cad-4120-97ac-8b648567172b.

## Präzisierung bestätigt

Pate, Concierge und Deadlock Brain sind eine persönliche KI-Hilfe. Serverguide ist deren Fähigkeit. Menschliche Vermittlung, Rollen, Übernahmen und entsprechende Nein-/Anfragepfade sind vollständig aus K entfernt. Keine eigenen Produktänderungen daran entstanden. Erster Rechercheworkflow mit veraltetem Teilauftrag gestoppt. Aktueller Workflow und Briefings enthalten die Korrektur. Keine zweite Persona, keine breiten Kontaktserien, keine ungeklärte Gedächtnisablage.

## Konkrete Anschlussgrenze vor Produktänderung

Brain-K startet auf f6f5cef65f1f946113f0b8216c6475f6d38ec928. Frisch geholtes origin/main enthält weder `brain-contracts/src/tools.rs` noch den erhaltenen Vollguide oder Rust-Siteport. Gs aktuelle `AN_HAUPT-G.md`, Kopf und 09:00-Abschnitt, benennt ungemergten Produkt-WIP und aktive Vertrags-/Provider-/Kernel-Fortsetzung G-K-R1. Keine fertige G-Vertragsabnahme oder Mainintegration behauptet.

Der jetzige `AnswerProviderPort::answer(Query, AuthorizedContext, Evidence)` in `brain-contracts/src/lib.rs:510` und öffentliche `/v1/answer` sind wissensgebundene Antwortverträge. `brain-serve/src/service.rs` komponiert einen einzigen konfigurierten Antwortprovider. `brain-providers/src/transport.rs:14` ist privater grounded-Transport; Abozweig akzeptiert genau System/User, keine Tools. Damit darf ein Twitch-Titelentwurf weder als verstecktes Query-Steuerprotokoll noch mit erfundenen Evidenzen noch pauschal auf Luna angebunden werden.

Benötigte Abhängigkeit: geprüfter integrierter G-Vertrag, anschließend enger typisierter Generierungsanschluss mit aufgabengebundener bestehender Modell-/Budget-/Timeoutkonfiguration. K baut keine parallele Providerimplementierung und kopiert keinen G-WIP. Eigene Botzustände und der erhaltene Rust-Siteport werden inzwischen disjunkt vorbereitet. Die konkrete Twitch-Fassade wird vom nativen high-Worker gegen den vorhandenen Titelpfad nachgelesen.

Im aktuellen `brain-contracts/src/provider_input.rs:31` enthält der bestehende Antwortprompt noch ein menschliches Patenangebot und trennt Paten von Coaching. Dieser Prompt muss beim gemeinsamen Anschluss die aktuelle Bedeutung Pate=Brain=Concierge einhalten. K verändert den aktiven G-Vertrags-/Providerbereich derzeit nicht parallel. Das ist keine Freigabe, menschliche Patenmechanik im Bot zu ändern.

Ein Loopback-Abo-Proxy ist remote Modellverarbeitung. Öffentliche Antwortfälle sind damit möglich; private Guide-/DM-/Profilinhalte benötigen einen tatsächlich lokalen freigegebenen Provider oder müssen vor dem Modellaufruf gesperrt bleiben. Keine Config-/Datenschutzlockerung.

## Native Arbeit

Nachweise und IDs: `K/REGISTER.md`. Tatsächliche Agentenstarts stehen in wf_d31d25f0-fca/journal.jsonl. Zwei Workflowworker (Guidebau, Twitch-Vertrag) plus ein nativer Siteport-Worker, insgesamt höchstens drei. Alle geerbt Sol 6.1 high. Keine Modell-/Settingsänderung. Alte kanonische Checkouts und E/F/I/G/H-Schreibbereiche unangetastet.

Gebaut: noch keine abgeschlossene Rückgabe. Reviewt: nein. Gemergt: nein. Deployt/live: nein. Kein falscher Abschluss.

Wache 20 Minuten: eigener sessiongebundener Job 7e018b31. Er endet spätestens nach sieben Tagen und wird beim Abschluss gelöscht.
