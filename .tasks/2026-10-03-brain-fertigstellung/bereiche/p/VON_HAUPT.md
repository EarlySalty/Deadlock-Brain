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

Dein angekündigter neuer brain-patchnotes-ingest-Einstieg ist dem Paket P zugeordnet. Vorhandene Aktivierung wiederverwenden. Runtime-Konfiguration und Aktivierungsvertrag muss Z gemeinsam mit dir festlegen, bevor ein produktiver Writer umgestellt wird. Lies Zs AN_HAUPT.md und melde den benötigten Vertrag dort über deine eigene AN_HAUPT.md. Keine zweite Aktivierungslogik.

Bitte bei der nächsten Statusmeldung die gelesene Übernahme bestätigen und tatsächlichen nächsten Arbeitsschritt mit SHA nennen. Für gekoppelte Änderungen gilt gemeinsame Abnahme vor Merge und Z als Integrationsverantwortlicher. Texte mit humanizer und no-em-dashes prüfen.

Der unabhängige Vorcheck bestätigt den bestehenden journalgebundenen Aktivierungsweg in brain-maintenance/src/integration/activation.rs und config_writer.rs. Z hat den Auftrag, den tatsächlichen serve_config-Pfad aus der normalen Runtime-Konfiguration sicher zu bestimmen und dir den Vertrag zu liefern. Keine neue Aktivierung bauen. Bereits wartende Prüfläufe erhalten.

## Übergabe vor dem Live-Nachweis

PAKETE.md und BRIEFING-Z.md schließen die bisherige Kreisabhängigkeit: Lokal geprüften Eigenstand mit vollem SHA und Betriebsvertrag als uebergeben an Z melden. Nicht bis zum unmöglichen eigenen Liveabschluss auf Z warten. Z führt gemeinsame Integration, Abnahme, Gate und Deployment durch; anschließend führst du deinen tatsächlichen Live-Nachweis aus und meldest erst dann abgeschlossen. Kein vorgezogener Einzelmerge oder paralleler Releasewechsel.

## Q-Vertrag liegt vor, 16:15 UTC

Lies /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/bereiche/q/AN_HAUPT.md: konkreter Consumer-, Socket-, Audit- und Writervertrag ist jetzt veröffentlicht. Der Q-Stand ist noch nicht abschließend geprüft; Vertrag und Live-Beweis getrennt behandeln. Die historische Grantbezeichnung token_env wurde unabhängig im Code geprüft: Auflösung aus dl_token_secrets über FD/Infisical-Snapshot, kein Environment-Fallback. Keine Nebenbaustelle für eine Umbenennung eröffnen.

## Tatsächlicher serve_config-Pfad dreifach belegt, 16:19 UTC

Der unabhängige lesende Betriebsvorcheck liefert /home/nathanael/.config/deadlock-brain/brain-serve.json. Belege: ausschließlich serve_config aus /etc/deadlock-brain/maintenance-runtime.json; wirksames Drop-in /home/nathanael/.config/systemd/user/brain-serve.service.d/90-maintenance.conf Zeile 4; ausschließlich --config-Argument des laufenden PID 3506677. Alle drei nennen denselben Pfad. Die Basis-Unit nennt einen alten /etc-Pfad, den das Drop-in ersetzt. Verzeichnis 0700, Datei 0600, Eigentümer nathanael.

Vorhandener ConfigWriter sperrt brain-serve.json.lock, prüft erwartete Altbytes und ersetzt atomar. activate() nutzt denselben Pfad, startet die konfigurierte User-Unit neu und prüft Release-ID und Wissensversion über /readyz. Z hält die gemeinsame Aktivierung; P/Q nutzen diesen Vertrag und bauen keine eigene Configschreiblogik. Dieser Pfadblocker ist geklärt, noch kein Deployment oder Live-Nachweis. Vor tatsächlicher Mutation Pfad und erwarteten Ausgangsstand frisch prüfen.

## Z-Betriebsvertrag liegt vor

Lies bereiche/z/BETRIEBSVERTRAG.md in der gemeinsamen Akte. Er enthält den geprüften Runtimepfad, feste Operator-Socketadresse und Principalbindungen. Noch kein Livebeweis. Der fehlende gemeinsame Kandidatenadapter für P/Q wird von Z im bestehenden brain-maintenance-Aktivierungsweg gebaut und integriert; P/Q sollen keine zwei separaten Aktivierungsorchestrierungen bauen. Fachliche Importpfade bleiben bei P/Q, gemeinsame Sperr-/Journal-/Restart-/Readinesslogik bei Z. Z kündigt konkrete Dateigrenzen vor Schreiben an. K kann seinen Client an den festgelegten Socketpfad binden; echte Releasekennungen folgen nach Import.

## Laufzeitbefund zum Nachrichtenweg, 16:37 UTC

Direkte Nachrichten in laufende T3-Turns werden vom Hauptorchestrator nicht mehr verwendet. Trotz des gelesenen Queue-Vertrags belegt die Hostprobe zeitgleiche Elternneustarts und [killed]-Prüflogs bei Q nach 16:08:37 und R um 16:27. Das ist kein regulärer Prüfabschluss. Antworten und Entscheidungen kommen deshalb nur über diese Datei; bei eigenen Statusmeldungen lesen. Bereits selbst korrekt wiederaufgenommene Prüfungen erhalten; nicht wegen dieses Hinweises neu starten. Nur nachweislich beendete unvollständige Läufe am vorhandenen Stand fortführen. Keine Zeitlimits für das reine Warten auf Hostlocks, keine fremden Prozesse stoppen. Der Hauptorchestrator verfolgt die bestehenden Sitzungen lesend weiter.

## Gemeinsame Aktivierung: unterschiedliche Zielbindungen, 16:43 UTC

Qs Ereignis q/1/8 und aktualisierte AN_HAUPT.md präzisieren: Sheet-/YouTube-Writer aktivieren ausschließlich die interne Second-Brain-Releasebindung. Dabei müssen Grant-Release und internal_operator.release atomar übereinstimmend geändert werden; öffentliche Bindungen bleiben unverändert. Der vorhandene ActivationPlan für den Standardrelease allein erfüllt diesen Vertrag nicht.

Z muss den gemeinsamen Kandidatenadapter für die tatsächlich unterschiedlichen Zielbindungen auslegen: Ps vereinbarter Standard-/Patchnotespfad und Qs intern gekoppelte Releasefelder, mit vorhandener gemeinsamer Sperre, erwarteter Basis, Journal und Readinessnachweis für die betroffene Bindung. Keine pauschale Änderung aller Grants. P/Q deklarieren das Ziel eindeutig; Z integriert und prüft gemeinsam mit K, dass öffentliche Docs/Twitch-Inhalte und interner Operatorzugang bei der jeweils anderen Aktivierung erhalten bleiben. Das ist dieselbe beauftragte Aktivierung, kein zusätzlicher Funktionsumfang.

## Wache muss die wirklichen Unteraufträge prüfen

P12 belegt offene Workflow-Supervisoren bei längst unterbrochener Runtime-/DevFeed-Arbeit. Prüfe bei jeder eigenen Wache deshalb je aktivem Unterauftrag den tatsächlichen letzten Werkzeugfortschritt, Abschluss-/Unterbrechungsereignis und zugehörige lebende Prozesse beziehungsweise Prüfsperrenwarteprozesse. Ein offener Supervisor oder lebender Hauptprozess allein belegt keine arbeitende Unteraufgabe. Nach sicherem Endnachweis vorhandenen Stand geordnet übernehmen, lebende Prüfer erhalten; kein Neubau von null und keine doppelte Schreibzuständigkeit. Reine Wartezeit auf Hostlocks ist kein Abbruchgrund. Keine Nachrichten in laufende T3-Turns oder problematische Workflowkontexte erzwingen. Bestätigte wiederkehrende Harnessfehler mit konkretem Endstand melden statt unbemerkt warten.
