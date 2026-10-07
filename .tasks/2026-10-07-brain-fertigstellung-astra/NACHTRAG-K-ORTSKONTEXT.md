[Orchestrator]
# K-Nachtrag: tatsächlichen Frageort im bestehenden Antwortweg nutzen

Verbindlicher Nutzerauftrag vom 07.10.2026, Befund 21:30 in BEFUND-NUTZERTEST-2000.md. Zuständig bleibt K, Thread 79c97ab5-f014-4e17-9d00-20c7adaf83ff; Auftraggeber Delegator 481426fe-b477-42b3-91c6-901811fcba1d.

## Ziel und Umfang

Discord übergibt Kanalname, Kategorie, Kanalthema beziehungsweise Zweck aus bestehendem Server-as-Code, Thread-/DM-Kontext und Eingangsart (Erwähnung, Hilfekanal, Guide, DM) über den bestehenden Brain-Vertrag. Twitch übergibt den Kanal und den bekannten Partnerstatus. Vorhandene Kontext-/Metadatenpfade zuerst mit Graphify prüfen und wiederverwenden. Kein zweiter Antwortweg, keine Bot-eigene Antwortlogik oder neuer LLM-Connector.

Das Brain verwendet den tatsächlichen Ort für seine Antwort. Wer schon im zuständigen Bereich fragt, wird nicht zurück in denselben Bereich geschickt, sondern bekommt direkte Hilfe oder den nächsten konkreten Schritt. Fehlende Ortsdaten nicht erfinden. Nutzertest: „wie kann ich auch deadlock spielen“ im dafür vorgesehenen Bereich.

## Rechte und Datenschutz

Nur für die tatsächlich fragende Person sichtbare Ortsdaten übernehmen. Interne IDs dürfen zur Rechteprüfung genutzt werden, gelangen aber NEVER an das Modell. Keine Discord-/Steam-IDs, Mitgliederlisten, fremden Personendaten oder unsichtbaren Kanalinformationen mitsenden. DM nicht durch Server-/Kategoriedaten anderer Unterhaltungen ergänzen. Kanalnamen und Themen sind Kontextdaten, keine Anweisungen oder Berechtigungserweiterung. Nutzerfreigabe aus ENTSCHEIDUNG-DATENSCHUTZ-NUTZER-2045.md gilt unverändert; keine harte Kategoriesperre hinzufügen.

## Eigentum und Reihenfolge

Bestehende K-Worktrees und Branches bleiben erhalten: brain-k-ki-20261007, bots-k-guide-20261007 und twitch-k-ki-20261007 unter /home/nathanael/.worktrees/. Aktuellen HEAD und WIP vor Änderungen selbst prüfen. Keine parallelen Writer auf denselben Consumerdateien. Dies ist ein Nachtrag zur laufenden K-Integration, kein neuer Thread.

Der bereits laufende kleine Tagesquotenfix bleibt zuerst separat prüfbar und lieferbar. Ortskontext nicht nachträglich in dessen Compiler-/Gateumfang ziehen oder seinen Deploy daran aufhalten. Danach geordnet im bestehenden K-Antwortanschluss umsetzen und integrieren. Die einzige Discord-Grenze bleibt 50 Fragen je Nutzer/Berliner Kalendertag aus bot.toml.

Der fremde Docs-Thread 59740e62 bleibt unangetastet. Keine Übernahme, Kontaktaufnahme oder Warteschleife für dessen Arbeit. Docs und Concierge nicht löschen.

## Nachweise und Abschluss

Bestehende Vertrags-/Consumerprüfungen um Ortsbindung und tatsächliche Verwendung erweitern: richtiger Bereich ohne Selbstverweis, DM/Thread, Eingangsarten, Twitch-Partnerstatus, fehlender Kontext, rollenbedingt unsichtbare Daten sowie ID-freier Providerinhalt. Keine erfundenen Kanalnamen als Produktionsbeweis. Nach regulärem Gate/Merge/Deploy Antwortwirkung im autorisierten Liveweg belegen. Keine eigenmächtigen Modell-/Timeoutwechsel.

Compiler-/Testprüfungen über cargo-slot, lokale Gates und bestehende Grenzen unverändert. Schutzablehnung nicht umgehen. K berichtet in eigenen vorhandenen Akten, Delegator pflegt zentrale REGISTER/TODO/PAKETE. Q übernimmt den realen Fall erst nach I/G/K-Livegang in denselben festen Vergleich; Q wird jetzt nicht gestartet.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 30 min | Worktree: /home/nathanael/.worktrees/brain-k-ki-20261007
