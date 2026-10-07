[Orchestrator] Reiner Audit B: Grafiken, kleine Webseiten und vollständiger Serverguide

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: hauptbaum, nur lesend

Lies `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-feature-audit/AUFTRAG.md`.

## Rolle, Grenzen und Routing

Du bist Teil-Orchestrator B, GPT 6.1 Sol high im Claude-Code-Harness. Eigene native Claude-Code-Rechercheagenten und UltraCode-Workflows sind ausdrücklich erlaubt, höchstens drei gleichzeitig. Vor Workflow `workflow-authoring` laden. Kein neues T3-Unterprojekt, kein anderer Anbieter, kein Sonnet und kein xhigh/max. Wenn UltraCode nicht tatsächlich verfügbar ist, Grenze melden und native Agenten für die Recherche nutzen. Keine Produktänderung.

Auftraggeber ist Hauptorchestrator `a711a4d2-1cad-4120-97ac-8b648567172b`. Lies gegen aktuelle Gitstände; kanonische Checkouts können schmutzig und alt sein. Kein Checkout, Commit, Push, Merge, Deploy, Restart, keine DB-Schreiboperation, keine produktive Nachricht und keine Modellprobe. Keine privaten Mitgliederdaten in Berichte übernehmen. Fremde Threads bleiben unangetastet.

## Rechercheauftrag

Der Nutzer verwirft YouTube und Forum und will tiefer nach seinen tatsächlich gewünschten Funktionen suchen:

1. **Grafiken und kleine Webseiten aus dem Brain.** Aktueller G-Plan hat dies ausdrücklich als späteres Ziel aufgenommen; der Nutzer priorisiert es jetzt neu. Lies Wünsche, Taskakten und vorhandene Komponenten in Brain, Deadlock-2nd-Brain, Deadlock-Docs, Website, Deadlock-Bots, gegebenenfalls Twitch-Dashboards. Welche tatsächlichen Nutzerwünsche stehen dahinter? Beispiele aus belegten Aufgaben, nicht frei ausgedachte Funktionen: visuelle Helden-/Itemvergleiche, Builddarstellung, Erklärseiten, Quellen-/Patchbezug nur falls belegt. Was existiert schon als Renderer/HTML/SVG/Linkseite/Veröffentlichungsweg? Abgrenzung automatische generierte HTML/JS gegen deterministische Templates aus typisierten Daten. Was passt zu Gs versionsgepinnter Rechenschicht, was würde G doppeln? Auslieferung in Discord/Twitch, Vorschau vs. Veröffentlichung, URLs, Rechte, XSS/CSP, Lebensdauer und Caching prüfen. Kein Renderer bauen und keine Grafik erzeugen.

2. **Vollständiger Serverguide, Persona, Kontaktserien, Paten.** Den alten Auftrag tiefer als nur TODOs lesen. Startpunkte: `/home/nathanael/Documents/.tasks/2026-10-03-serverguide/HANDOFF-OFFEN-2026-10-03.md`, `/home/nathanael/.worktrees/serverguide-deploy-20261003/`, Brain `.tasks/2026-10-03-brain-fertigstellung/welle1/` insbesondere serverguide-mvp und `VON_HAUPT.md`, laufender Abschlussauftrag A/B, Deadlock-Bots aktuelle Rust-Module und Konfiguration. Prüfe DMs/Erwähnung/proaktive Hilfe, Guide-Producer, öffentliche Livefakten, Persona, dauerhafte Profile, Kontaktserien, Begrüßung bestehender Mitglieder, Opt-in/Nein/Stopp, Patenvermittlung/Übernahme/Erinnerung, V4/V5, Grafiken und Testbetrieb. Pro Teil: Nutzerwunsch, existierender Code, ein-/ausgeschaltet, aktuell erreichbar, gebaut-unintegriert, Blocker. Ein früherer Bericht „live“ ist ohne aktuellen Code-/Betriebsbeleg kein heutiger Funktionsbeweis.

3. **Weitere belegte Wünsche.** Suche alte Brain-/Serverguide-/Consumer-/Designakten nach vertagten Produktideen. Liefere eine kompakte Liste mit Quelle und aktuellem Urteil. Keine Infrastruktur-To-dos als Nutzerfeature verkaufen, keine YouTube-/Forumvorschläge und kein Replay-Neubau entgegen „keine Matchdaten speichern“.

## Ziel des Berichts

Nutzer möchte planen, nicht schon bauen. Empfehle kleinsten brauchbaren Grafik-/Webseitenumfang plus Serverguide-Teile, die sinnvoll als Erweiterung des zentralen Brain integriert werden können. Funktioniert etwas bereits, genau das sagen. Fehlt nur Verdrahtung, Wiederverwenden statt Neubau. Kennzeichne Produktentscheidungen, Datenschutzgrenzen, aktive Überschneidungen mit G/A/B und den bestehenden Release-Halt.

Paket A inventarisiert alle KI-Aufrufwege; du brauchst keine zweite globale Provider-Inventur. Konzentriere dich auf Featurewünsche, vorhandene Nutzerflows und Machbarkeit.

## Dateien und Beweisziel

Einzige Schreibzuständigkeit unter `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-feature-audit/`:
- `B-FEATURE-BESTAND.md`: tieferer Wunsch-/Bestandskatalog mit Pfaden, Zeilen/SHAs und Status.
- `B-EMPFEHLUNG.md`: Grafik-/Webseiten-MVP, Serverguide-Machbarkeit und priorisierte Restliste mit klarer Empfehlung und Begründung.
- `B-STATUS.md`: Session/Thread/Modell/Effort/UltraCode-Nachweis, native Agenten, Stand.
- Unterordner `b/` für Rohberichte deiner nativen Agenten.

Graphify zuerst. Quellenstatus main/WIP/live getrennt. Berichte in natürlichem Deutsch, echte Umlaute, keine Gedankenstriche. Keine Code-Kommentare hinzufügen. Keine Secrets lesen. Nicht bei „kein Treffer“ aufgeben, Nebenpfade und bestehende Worktrees gezielt prüfen. Zeitgrenze ist kein Grund, Fakten zu erfinden; Wissenslücken explizit benennen.

Fertigmeldung: kurze Fachübergabe plus Berichtspfade, keine gesamte Rohinventur im Chat. Rückfrage bei echter Unklarheit an Hauptorchestrator, nicht an Nutzer. Nach Bericht auf Abnahme warten, kein selbstständiger Bau oder Threadwechsel.
