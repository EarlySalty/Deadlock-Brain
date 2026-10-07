# HALTEPUNKT, gilt sofort und vor allem anderen (03.10.2026, 21:00 Uhr, Hauptsession 92efdb66 im Auftrag des Nutzers)

Der Auftrag wird neu geschnitten. Du bringst deine Arbeit jetzt an einen sauberen Punkt und beendest dann deinen Turn:

1. Keine neuen Prüfläufe, keine neuen Subagenten, keine neuen Bauaufgaben. Wer gerade auf `host-checks.lock` wartet: Warten abbrechen, die Sperrregel ist ersetzt (`/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md`).
2. Alles, was in deinem Worktree an eigener Arbeit liegt, auf deinem Branch committen. WIP ist ausdrücklich erlaubt, Commit-Betreff dann mit `wip:` anfangen. Nichts verwerfen.
3. Deinen Branch nach origin pushen, nicht nach main.
4. In deinen Bereichsordner `STAND.md` schreiben, höchstens 15 Zeilen: Branch, HEAD-SHA, was davon geprüft grün ist (Befehl und Testzahl), was kaputt oder offen ist, nächster konkreter Schritt. Keine PIDs, keine Sperr- oder Beweisgrenzen-Prosa.
5. Eigene Subagenten und eigene Hintergrundprozesse geordnet beenden, dann Turn beenden. Nicht settlen, die Hauptsession übernimmt.

Kein Merge, kein Deploy, keine Configänderung mehr in diesem Thread.

---

status: aktiv
Datum: 2026-10-03

# Antwort Hauptorchestrator an alle Pakete: 403 bei Unter-Agents

Befund über alle sechs Pakete: Die 403-Abbrüche ("WebSocket upgrade was rejected") trafen nur einen Teil der Unter-Agents, gehäuft beim gleichzeitigen Start um 14:13 UTC. Die Mehrheit der Unter-Agents in P, S, Q, Z und K läuft normal mit demselben Modell. Das ist eine Lastspitze am Proxy, kein dauerhaft kaputter Startweg.

Anweisung:
1. Gescheiterte Unter-Agents nach einer Minute Pause einmal neu starten, gleicher Auftrag, gleiches Modell.
2. Höchstens drei Unter-Agents gleichzeitig je Paket, gestaffelt starten (nicht alle in derselben Minute).
3. Scheitert ein Unter-Agent ein zweites Mal, machst du diese Arbeit selbst in deiner Hauptsession weiter. Kein Modell- oder Anbieterwechsel.
4. Die unabhängige Intent-Abnahme muss ein frischer Kontext sein. Geht kein Unter-Agent, schreibst du das in AN_HAUPT.md mit Worktree und SHA, dann starte ich die Abnahme als eigenen T3-Thread.
5. Nicht auf mich warten: weiterarbeiten.

## Übernahme durch Codex am 2026-10-03, 15:39 UTC

Der Nutzer hat die Hauptorchestrierung an Codex /root (T3 e6c19079-657e-4db9-80bd-8e1313e7f785) übergeben. Lies UEBERNAHME-CODEX.md in der gemeinsamen Akte. Bestehende Arbeit und Sessions erhalten. Antworten weiter über diese Datei, Übergaben über AN_HAUPT.md und Statusereignisse. Keine zusätzliche T3-Session starten.

Berechtigte echte Demos und vorhandenen Decoderpfad fertigstellen. R bleibt gemäß AUFTRAG.md keine harte Voraussetzung für G5, eine fehlende Demo darf trotzdem nicht als bestandener Replay-Nachweis erscheinen. Vorhandene Arbeit erhalten.

Bitte bei der nächsten Statusmeldung die gelesene Übernahme bestätigen und tatsächlichen nächsten Arbeitsschritt mit SHA nennen. Für gekoppelte Änderungen gilt gemeinsame Abnahme vor Merge und Z als Integrationsverantwortlicher. Texte mit humanizer und no-em-dashes prüfen.

## Laufzeitbefund zum Nachrichtenweg, 16:37 UTC

Direkte Nachrichten in laufende T3-Turns werden vom Hauptorchestrator nicht mehr verwendet. Trotz des gelesenen Queue-Vertrags belegt die Hostprobe zeitgleiche Elternneustarts und [killed]-Prüflogs bei Q nach 16:08:37 und R um 16:27. Das ist kein regulärer Prüfabschluss. Antworten und Entscheidungen kommen deshalb nur über diese Datei; bei eigenen Statusmeldungen lesen. Bereits selbst korrekt wiederaufgenommene Prüfungen erhalten; nicht wegen dieses Hinweises neu starten. Nur nachweislich beendete unvollständige Läufe am vorhandenen Stand fortführen. Keine Zeitlimits für das reine Warten auf Hostlocks, keine fremden Prozesse stoppen. Der Hauptorchestrator verfolgt die bestehenden Sitzungen lesend weiter.

## Wache muss die wirklichen Unteraufträge prüfen

P12 belegt offene Workflow-Supervisoren bei längst unterbrochener Runtime-/DevFeed-Arbeit. Prüfe bei jeder eigenen Wache deshalb je aktivem Unterauftrag den tatsächlichen letzten Werkzeugfortschritt, Abschluss-/Unterbrechungsereignis und zugehörige lebende Prozesse beziehungsweise Prüfsperrenwarteprozesse. Ein offener Supervisor oder lebender Hauptprozess allein belegt keine arbeitende Unteraufgabe. Nach sicherem Endnachweis vorhandenen Stand geordnet übernehmen, lebende Prüfer erhalten; kein Neubau von null und keine doppelte Schreibzuständigkeit. Reine Wartezeit auf Hostlocks ist kein Abbruchgrund. Keine Nachrichten in laufende T3-Turns oder problematische Workflowkontexte erzwingen. Bestätigte wiederkehrende Harnessfehler mit konkretem Endstand melden statt unbemerkt warten.
