# K: Twitch-Titelvertrag und Anschlussgrenze

Native read-only-Rückgabe aus wf_d31d25f0-fca, Agent ac0c88636c79f7fd3. Twitch-K-HEAD 0452e03cb7eab42d9e08ee5d39bde380514f1cd3. Keine Produktänderung, keine Secrets-/Modellwerte gelesen, kein Prüflauf oder Livebeweis.

## Bestehender Fall

`rust/crates/tb-dashboard-api/src/handlers/title.rs:28,379,462`: POST /twitch/api/v2/title/suggest. Request enthält optionale keywords und streamer sowie include_live. Keywords maximal 300 Zeichen, Zielrechte lokal geprüft.

Der aktuelle Modellkontext enthält gespeicherte Stilpräferenzen, verbotene Phrasen, Feedback, Titelhistorie samt Performance, Community-Benchmarks und gegebenenfalls Rang-/Live-/Co-Streamerinformationen. Dieser bestehende personalisierte Fall darf nicht unverändert remote über Brain laufen. Öffentliche Provenienz beliebiger keywords ist nicht belegt. Das Weglassen aller privaten Teile wäre ein eigener öffentlicher Entwurf, keine gleichwertige Migration des personalisierten Features.

Modellantwort: primary_title, alternatives, title_analysis. Lokale Parser und Bereinigung bilden die Dashboardantwort mit primary, alternatives, title_analysis und weiteren lokalen Zustandsfeldern. Helix, OAuth, Zustimmung, Feedback, Persistenz und bestehende Effekte bleiben unverändert im Bot.

## Vorhandene Freigaben und Limits

`tb-chat/src/title_ai.rs:769,781,820,912`: UseCase title_ai, Ledgerzweck title, Temperatur 0.8, Ausgabelimit 900, reasoning aus, Thinkingblöcke entfernt. Zwei HTTP-429-Retries pro Completion. Zweite Completion nur bei erfolgreicher erster Antwort ohne brauchbaren Titel.

Rate-Limit: fünf Aufträge je Streamer/Quelle in 600 Sekunden; Dashboard zehn. Retries verbrauchen keinen weiteren Limiterplatz. `tb-llm/src/hub.rs:26,383,492`: 240 Sekunden Gesamtdeadline je Completion einschließlich Ledger und 429-Wartezeiten, Retrywait maximal fünf Sekunden. Zweite Completion kann eine weitere Deadline nutzen. Keine belegte titelspezifische Geldobergrenze; Ledger ist keine Ausgabesperre, unbekannte Usage bleibt unbekannt.

`tb-llm/src/selection.rs:27`, `model_resolver.rs:15`, `fireworks-model-selection/src/lib.rs:12,146`: geschützte gemeinsame Fireworks-Auswahl, bei jedem echten Request erneut gelesen. Fehlender/ungültiger Auswahlstand scheitert geschlossen. Kein aktives Modell aus einem Fallbackstring ableiten. Keine pauschale Lunaumstellung.

## Konkreter Anschluss

Completion innerhalb generate_title_personalized_with, nach Zielrechten und Limiter, vor vorhandenen Parsern/Bereinigung. Nicht set_channel_title, nicht Feedbackspeicherung, nicht Helixhandler.

`tb-knowledge/src/brain.rs:30,48,67,121` bietet derzeit Query/PublicAnswer mit Wissenszitaten. Keine typisierte Titelantwort, aufgabenbezogene Modellauswahl, Temperatur oder Ausgabebudget. Historie/persönliche Karten werden durch require_stateless abgewiesen. Brainclient-Pin d2028d8b0c96c2097dc9b92882bb96b73bf19fd0, `tb-knowledge/Cargo.toml:7`. Der Selfexplainer mit acht Sekunden Default und 65-Sekunden-Obergrenze ist kein Ersatz für den Titelvertrag.

## Entscheidung und Restumfang

K wählt für die erste öffentliche Welle einen gesonderten nichtpersonalisierten Deadlock-Titelentwurf mit ausschließlich freigegebenen öffentlichen Spielfakten und festen serverseitigen Anweisungen. Keine Nutzerkennungen, Co-Streamerbeziehungen, Stilpräferenzen, History, Feedback, Performance oder privaten Live-/Sessiontexte an Remoteprovider. Lokale verbotene-Phrasen-Filter dürfen nachgelagert bleiben. Der vorhandene personalisierte Titelpfad bleibt unverändert und wird nicht als migriert gemeldet.

Benötigt vor Verdrahtung: typisierte Brain-Titelfähigkeit, serverseitige Bindung an denselben validierten Fireworks-Auswahlstand pro Request, bestehende 240-Sekunden-Completionfristen und Ausgabelimits, gemessene Usage/Retryparität. Keine Modell-/Credentialoverrides vom Wireclient, kein zweiter Connector, kein Prompt-Steuerprotokoll in Query.text. Gs aktiven Provider-/Kernelbereich nicht parallel ändern. Ohne diese Abhängigkeit keine scheinbar funktionierende Fassade einschalten.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/tb-chat/src/title_ai.rs:769 | Anknüpfung: vorhandener Titel-Completionpfad und tb-llm-Auswahl/Accounting
