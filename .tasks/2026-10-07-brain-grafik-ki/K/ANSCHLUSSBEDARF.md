# K: Benötigter enger KI-Anschluss

Dies ist der Integrationsbedarf, kein bereits implementierter oder freigegebener Ersatz für Gs aktiven Vertrag. Kein allgemeines Freitextprotokoll für strukturierte Aufgaben.

## Fähigkeiten

| Fähigkeit | Eingabe | Ergebnis | Modellweg |
| --- | --- | --- | --- |
| Öffentliches Discord-Guidewissen | Bewusste Frage aus zulässigem öffentlichem Kanal, bestehende Personen-/Zustellrechte lokal | vorhandene PublicAnswerResponse samt Wissensstand/öffentlichen Belegen | vorhandener Brain-Antwortweg; keine privaten FAQ-Verläufe/DMs |
| Bestehender Twitch-Titelteilfall, falls zulässig belegt | ausschließlich vorhandener nichtpersonalisierter Kontext, kein stilles Entfernen nötiger Inputs | bestehende primary_title, alternatives, title_analysis, Wissensabruf optional | vorhandene validierte Fireworks-Auswahl für title_ai je Request, keine Lunaumstellung |

Kein neuer öffentlicher Titelgenerator, keine neue UI/Route und kein zweiter Titelpfad. Ohne bereits bestehenden geeigneten Teilfall bleibt Titel-Cutover offen. Der gemeinsame typisierte Vertrag wird vorbereitet, nicht als bereits verdrahtete Fähigkeit ausgegeben.

Private persönliche Hilfe ist keine dritte aktivierte Fähigkeit dieser Welle. Ohne tatsächlich lokalen freigegebenen Provider bleibt private Modellverarbeitung gesperrt. Serverguide bleibt Fähigkeit des einen Brain; keine zweite Persona.

## Serverseitige Bindung

Dienstidentität aus registriertem Zugang, bekannte Plattformbindung und genau erlaubte Fähigkeit. Nutzer-/Kanal-/Guildrechte prüft der Bot anhand stabiler IDs vor Anfrage und vor Versand. Kein Clientfeld oder Modelltext darf fremde Nutzeridentitäten, zusätzliche Rechte, Modelladressen, Anbieter oder Geheimnisse wählen. Bei privaten Eingängen ist public-Markierung kein Freibrief.

Vertragsversion, Request-ID und Ergebnisart vor Ausgabe prüfen. Unbekannte Fähigkeit, falscher Dienst, falsche Version, fehlende Quellenfreigabe oder fehlende Modellauswahl scheitern geschlossen. Brain-Ausfall aktiviert keinen Direktmodellfallback. Botschutz bleibt lokal bestehen. API liefert keine erfundenen Erfolgsergebnisse, wenn der Anschluss fehlt.

## Bestehende Titelparität

UseCase title_ai, Ledgerzweck title, Temperatur 0.8, Ausgabelimit 900 und Completiondeadline 240 Sekunden. Zwei 429-Retries, Wartezeit maximal fünf Sekunden innerhalb derselben Deadline. Eventueller zweiter Completionversuch bleibt eine eigene gemessene Ausführung mit ursprünglicher fachlicher Zulässigkeit. Unbekannte Usage nicht als null buchen. Ein ausgewähltes Modell wird pro Request gebunden, nicht zwischen Retry-/Folgeturns neu ausgelost.

Lokale Rate-Limits, Zielrechte, verbotene-Phrasen-Filter, Thinking-/Ausgabebereinigung, Dashboardmapping, Feedback, OAuth und Helixeffekte bleiben am bestehenden Botpfad. Gespeicherte Stile, Performance-/Historiedaten, private Livekontexte und Co-Streamerbeziehungen bleiben außerhalb eines öffentlichen Providerauftrags. Sind sie für den vorhandenen Teilfall nötig, ist dieser ohne lokalen Providervertrag nicht migrierbar. Ein parallel weiterlaufender Direktmodellweg ist kein fertiger Cutover.

## Voraussetzung

Die derzeitige AnswerProviderPort/Query/PublicAnswer-Strecke kann diese Generierung nicht durch ein anderes Promptformat garantieren. Benötigt wird ein geprüfter typisierter Generierungsport im vorhandenen Provider mit per-Aufgabe-Konfiguration und validierter gemeinsamer Auswahl. Erst nach G-Abschluss auf origin/main gemeinsame Export-/Provider-/Kerneldateien ändern. Keine parallele Providerkopie, keine Fake-Evidenzen und kein zweiter Connector.

## Grafik anschließen

H liefert feste Darstellung aus Gs typisiertem Vergleichsergebnis. K erstellt Dienst-ID und kontrollierten Link/Anhang unabhängig vom Modelltext. Gleicher Zahlen-/Versions-/Bedingungs-/Quellenstand in Grafik und Detailseite. Freigabe und Widerruf werden auch bei Abruf und Cachetreffer geprüft. Keine privaten Profile/Chats in diesem öffentlichen Artefaktweg. Allgemeine Discord-/Twitch-Linkfilter bleiben aktiv; nur separat geprüfte Dienstartefakte ausgeben.
