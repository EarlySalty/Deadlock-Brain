[Orchestrator] Reiner Audit A: alle KI-Antwortwege und Brain als gemeinsame Schnittstelle

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: hauptbaum, nur lesend

Lies zuerst `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-feature-audit/AUFTRAG.md`.

## Rolle, Auftraggeber, Arbeitsstand

Du bist Teil-Orchestrator A, GPT 6.1 Sol mit high im Claude-Code-Harness. Native Claude-Code-Subagenten und UltraCode-Workflows sind ausdrücklich erwünscht, getrennt nach Fachbereich. Höchstens drei gleichzeitig aktive Rechercheagenten in deinem Bereich. Keine weiteren T3-Threads. Erbe das freigegebene Sol-Modell, kein Sonnet, kein anderer Anbieter und kein xhigh/max. Lade `workflow-authoring` vor Workflowaufrufen; nutze native Agenten, wenn UltraCode nachweislich nicht verfügbar ist, und melde die Grenze. Kein Bau.

Hauptorchestrator `a711a4d2-1cad-4120-97ac-8b648567172b`. Canonical Brain ist veraltet und schmutzig; lies aktuelle `origin/main`-Snapshots und konkrete bestehende Worktrees, ändere weder Gitrefs noch Checkout. Kein Commit/Push/Merge. Das ist ausdrücklich ein Rechercheauftrag, nicht durch Abschluss-Hooks in einen Bauauftrag umdeuten.

## Inhalt

Nutzer: „alles was mit AI ist und irgendeine Antwort generiert oder sowas ins Brain verschieben“, Brain als einziger Adapter. Twitch-Komponenten sind gesondert zu durchdenken; bedarfsgerechte Konnektoren für Wissen aus dem Brain. Vorab prüfen, Bericht vorlegen, damit der Nutzer planen kann.

Inventar mindestens für Deadlock-Brain, Deadlock-Bots, Deadlock-Twitch-Bot einschließlich Dashboards, Deadlock-Steam-Bot, Deadlock--Patchnotes-Bot, Deadlock-Docs/Deadlock-2nd-Brain; angrenzende eigene Dienste (Turniere, Uplink, Website, Stream-Audit, Clips, Titel, Social) auf tatsächliche KI-Aufrufe prüfen. ai-coach und TradingBot sind ausgeschlossen. Gemeinsame SDKs/Provider nicht als zusätzliche Funktion doppelt zählen. Keine privaten Nachrichten sammeln.

Für jeden KI-Pfad: Repo/Datei/Symbol, Zweck, Trigger/Consumer, aktueller Provider und Konfigquelle (keine Secrets), Eingabeklassen, private Daten ja/nein, freie Antwort oder strukturiertes Urteil/Aktion, Nutzer-/Tenantbezug, Aufruf-/Antwortvertrag, Live-/Feature-/Legacyzustand und Beleg, vorhandener Brain-Anschluss, Entscheidung: schon Brain, in Brain verlagern, Konnektor, lokal deterministisch lassen, Sonderfall/offene Produktentscheidung.

Untersuche insbesondere:
- Discord Brain-Client gegenüber verbleibenden Concierge-/Paten-/Moderations-/Sonstigen LLM-Pfaden.
- Twitch zentraler `tb-llm` und alle Aufrufer: Chatantworten, Anlass-/Partner-Pitches, LFG, Dashboard-Assistent, Titel, Analyse/Judges, Scam, Crew, Social/Outreach, Stream-Audit. Netzwerkzugriff und Antwortgenerierung vs. reine Klassifikation trennen.
- Patchnotes: vorhandene ausdrückliche Perplexity-sonar-pro-Ausnahme nicht durch globalen Modellwechsel verletzen. Kann Brain routen, ohne Anbieterfunktion oder Freigabe zu verlieren?
- Brain aktuelle Provider-/Werkzeugverträge und G-Umbau. `/v1/answer` existiert, aber nicht automatisch geeigneter Universalvertrag für strukturierte Judges, Streaming, asynchrone Arbeiten, Schreibaktionen oder private Kontexte. Notwendige Vertragslücken konkret benennen, keine neue API erfinden ohne als Vorschlag zu kennzeichnen.
- Datenhoheit: öffentliche Wissensquellen, private Channel-/Userkontexte, lokale Auswertung von Streams, erlaubte Provider. Zugriff nach Platform-ID, Principal/Scopes, kein offener Querzugriff, keine automatische Wissensspeicherung aus Prompts.
- Kritische Moderation bei Brain-Ausfall, Zeit-/Tokenbudgets als bestehende Config, Idempotenz von Aktionen, Kosten/Usage, Auditierung. Lokale Botmechanik und deterministische Sicherheitsgates sollen nicht unnötig ins Brain wandern.

## Ergebnis und Eigentum

Schreibe allein:
- `A-KI-INVENTAR.md`: Belegtes Inventar mit konkreten Codepfaden/Zeilen und Abdeckungsgrenzen.
- `A-ARCHITEKTUR.md`: Urteil zur Machbarkeit, passender Zielzuschnitt zu G, minimale Migration in Wellen, Konflikte, Entscheidungen, klare Empfehlung.
- `A-STATUS.md`: eigener Thread/Session, Modell/Effort/UltraCode-Nachweis, native Agenten und Stand, Fertiggrenze.

Alles unter `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-feature-audit/`. Native Rohberichte dürfen in `a/` liegen. Keine andere Akte ändern. Paket B prüft Serverguide-Funktionen und Grafiken; du prüfst deren KI-Aufrufwege, nicht dieselben Produktdetails erneut.

Der Bericht muss nicht riesig sein, aber der Suchumfang muss stimmen. Widersprüche zwischen Doku und Code entscheiden statt abschreiben. Schon implementiert ist nicht gleich angeschlossen und nicht gleich live. Vermutungen kenntlich machen.

Bei Fertigmeldung nur Ergebnis, wichtigste Entscheidungen, exakte Berichtspfade und Lücken. Rückfragen an Hauptorchestrator als `FRAGE AN ORCHESTRATOR:`, keine Nutzerfragen. Keine eigene Zustellung an fremde Sessions. Keine Laufzeitänderung. Lass den Thread nach Bericht stehen, damit der Hauptorchestrator ihn nach Lesen settlen kann.
