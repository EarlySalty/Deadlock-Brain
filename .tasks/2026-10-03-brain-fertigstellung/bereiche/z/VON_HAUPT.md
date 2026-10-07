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

# Hinweis für Z (Stand 14:33 UTC)

- S rollt die CLI `/opt/deadlock-brain/current/bin/deadlock-brain` nach seinem Merge über den vorhandenen Release-Weg selbst aus (zeigt heute auf `be2aa6bd…`, `brain-serve` auf `511a347b…`). Beide Pfade gehören ins Runbook, damit nach dem Cutover CLI und Server denselben SHA tragen.
- P stellt `deadlock-brain-patchnotes-sync.timer` selbst um. Achtung: die aktive Unit startet `/home/naniadm/.local/bin/...`, nicht den nathanael-Pfad.
- Q stellt Sheet- und YouTube-Units selbst um. YouTube-Lernen bleibt bewusst pausiert.

## Übernahme durch Codex am 2026-10-03, 15:39 UTC

Der Nutzer hat die Hauptorchestrierung an Codex /root (T3 e6c19079-657e-4db9-80bd-8e1313e7f785) übergeben. Lies UEBERNAHME-CODEX.md in der gemeinsamen Akte. Bestehende Arbeit und Sessions erhalten. Antworten weiter über diese Datei, Übergaben über AN_HAUPT.md und Statusereignisse. Keine zusätzliche T3-Session starten.

Du hältst die gemeinsame Releaseinstallation und integrierte Abnahme. S liefert den tatsächlich vorhandenen Installationsweg mit Beleg. Falls kein geeigneter Weg existiert, gehört die minimale Ergänzung des bestehenden ops-Deploywegs zu Z; kein Abwarten auf einen erfundenen Wrapper. Vorhandene Wiki-Installationsmechanik nur lesend prüfen und Zuständigkeit respektieren. Kläre unmittelbar P benötigten Runtime-Konfigpfad und die Wiederverwendung von brain-maintenance activation/config_writer für wiederkehrende Writer. Q liefert Operator-/Principal-Konfigvertrag, K die CLI-Consumer. Snapshot und letzter Legacy-Abgleich müssen vor Schreibwechsel die Zwischenzeit-Daten erhalten. Restinhalte alter Drafts sichern, aber keine Zusatzfeatures daraus als neue G5-Voraussetzung aufbauen. Der volle Tageszyklus vor G6 bleibt erforderlich.

Bitte bei der nächsten Statusmeldung die gelesene Übernahme bestätigen und tatsächlichen nächsten Arbeitsschritt mit SHA nennen. Für gekoppelte Änderungen gilt gemeinsame Abnahme vor Merge und Z als Integrationsverantwortlicher. Texte mit humanizer und no-em-dashes prüfen.

## Unabhängiger Vorcheck um 15:42 UTC

Der lesende Prüfer bestätigt die Wiederverwendbarkeit von brain-maintenance/src/integration/activation.rs und config_writer.rs: journalgebundene Aktivierung, gemeinsame Konfigsperre, atomarer Austausch und erwartete Readiness-Werte. Einstieg laut Bestand: /opt/deadlock-brain/maintenance-current/brain-maintain --config /etc/deadlock-brain/maintenance-runtime.json. Die dort referenzierte serve_config wurde nicht gelesen; bitte nur den nicht geheimen Pfad über den zulässigen bestehenden Config-Weg ermitteln und P nennen. Kein vollständiger SHA-/Remote-main-gebundener Binary-Installer belegt. Die minimale fehlende Binary-Deploymechanik liegt bei Z, keine zusätzliche Betreiberfreigabe für diesen bereits beauftragten Abschluss nötig.

S war tatsächlich idle ohne Kinder. Nachfolger S2 c6ddac1c-c0d2-4bbc-b6f8-7bc462570a52 übernimmt die erhaltenen Worktrees und liefert an dich. Alte S-Session unangetastet.

## Kreisabhängigkeit aufgehoben, 15:56 UTC

BRIEFING-Z.md und PAKETE.md sind präzisiert: Nicht auf abgeschlossene Live-Nachweise von P/S/Q/K warten, bevor du integrierst. Sie liefern lokal geprüfte SHAs als uebergeben; du führst gemeinsame Abnahme und Gate, Merge und Deployment durch. Erst danach erbringen die Fachpakete ihre Live-Nachweise und melden abgeschlossen. Deine Runtime-/Aktivierungsverträge sind jetzt fällig und gehören vor diese Übergabe, damit P/K weiterbauen können. G5-Sicherheitsvoraussetzungen und voller Tageszyklus vor G6 bleiben unverändert.

## Q-Vertrag liegt vor, 16:15 UTC

Lies /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/bereiche/q/AN_HAUPT.md: konkreter Consumer-, Socket-, Audit- und Writervertrag ist jetzt veröffentlicht. Der Q-Stand ist noch nicht abschließend geprüft; Vertrag und Live-Beweis getrennt behandeln. Die historische Grantbezeichnung token_env wurde unabhängig im Code geprüft: Auflösung aus dl_token_secrets über FD/Infisical-Snapshot, kein Environment-Fallback. Keine Nebenbaustelle für eine Umbenennung eröffnen.

## Tatsächlicher serve_config-Pfad dreifach belegt, 16:19 UTC

Der unabhängige lesende Betriebsvorcheck liefert /home/nathanael/.config/deadlock-brain/brain-serve.json. Belege: ausschließlich serve_config aus /etc/deadlock-brain/maintenance-runtime.json; wirksames Drop-in /home/nathanael/.config/systemd/user/brain-serve.service.d/90-maintenance.conf Zeile 4; ausschließlich --config-Argument des laufenden PID 3506677. Alle drei nennen denselben Pfad. Die Basis-Unit nennt einen alten /etc-Pfad, den das Drop-in ersetzt. Verzeichnis 0700, Datei 0600, Eigentümer nathanael.

Vorhandener ConfigWriter sperrt brain-serve.json.lock, prüft erwartete Altbytes und ersetzt atomar. activate() nutzt denselben Pfad, startet die konfigurierte User-Unit neu und prüft Release-ID und Wissensversion über /readyz. Z hält die gemeinsame Aktivierung; P/Q nutzen diesen Vertrag und bauen keine eigene Configschreiblogik. Dieser Pfadblocker ist geklärt, noch kein Deployment oder Live-Nachweis. Vor tatsächlicher Mutation Pfad und erwarteten Ausgangsstand frisch prüfen.

## Entscheidung zur gemeinsamen Kandidatenaktivierung, 16:24 UTC

BETRIEBSVERTRAG.md ist gelesen. Der fehlende schmale Adapter für P/Q-Kandidaten ist ein gemeinsamer Integrationspfad und gehört Z. Erweitere dafür den vorhandenen brain-maintenance-Aktivierungsweg statt getrennte Aktivierung in P und Q zu bauen. Lass deinen nativen Rust-Worker den kleinsten wiederverwendbaren Einstieg samt benötigter Prüfungen liefern: vorhandene Publikations-/Config-Sperren, frische Basisbindung, Erhalt fremder Pins, Journal, Restart/Readiness und Rückfall. Vor dem Schreiben den genauen Dateiumfang in BETRIEBSVERTRAG.md ankündigen; Q/P erhalten denselben Vertrag und ändern diese Dateien nicht parallel. Falls bestehende Q-C9-Dateien berührt werden, deren tatsächlichen Stand vorher abstimmen und in Z integrieren.

P/Q bleiben Eigentümer ihrer fachlichen Importpfade und rufen diesen gemeinsamen Einstieg auf. Kein zweiter Configwriter, kein manueller Produktions-Datenbank-/JSON-Eingriff. Deine bisherige Erlaubnis, dass P selbst ActivationPlan verdrahtet, ist insoweit präzisiert: gemeinsam genutzte Orchestrierung an genau einer Stelle in brain-maintenance. Diese Ergänzung gehört zur beauftragten Integration und braucht keine neue Nutzerentscheidung.

## Laufzeitbefund zum Nachrichtenweg, 16:37 UTC

Direkte Nachrichten in laufende T3-Turns werden vom Hauptorchestrator nicht mehr verwendet. Trotz des gelesenen Queue-Vertrags belegt die Hostprobe zeitgleiche Elternneustarts und [killed]-Prüflogs bei Q nach 16:08:37 und R um 16:27. Das ist kein regulärer Prüfabschluss. Antworten und Entscheidungen kommen deshalb nur über diese Datei; bei eigenen Statusmeldungen lesen. Bereits selbst korrekt wiederaufgenommene Prüfungen erhalten; nicht wegen dieses Hinweises neu starten. Nur nachweislich beendete unvollständige Läufe am vorhandenen Stand fortführen. Keine Zeitlimits für das reine Warten auf Hostlocks, keine fremden Prozesse stoppen. Der Hauptorchestrator verfolgt die bestehenden Sitzungen lesend weiter.

## Erste konkrete Consumer-Teilübergabe, 16:40 UTC

K hat bereiche/k/UEBERGABE.md geliefert: Docs 3e570a8aa0bf867bf1baf35b064165804b77fcb4 mit 22 Tests, Intent-Abnahme und lokalem Gate ALLOW, Second-Brain 54979646adde835335fa24ddd2545e10f996df52 mit 16 Tests/fmt/clippy und abgeschlossener Intent-Abnahme. Lokale Prüfungen ersetzen die gemeinsame Kern-/Consumerprüfung nicht. Diese zwei Teilstände jetzt für die Integration aufnehmen; nicht auf den vollständigen K-Abschluss einschließlich Twitch warten, bevor die vorbereitbare Integrationsarbeit beginnt. Twitch/Bots bleiben bei K in Arbeit. K bindet CLI-Vertrag an deinen festgelegten Operatorsocket. Kein zusätzlicher Produktivwechsel freigegeben, solange gemeinsame Abnahme fehlt.

## Gemeinsame Aktivierung: unterschiedliche Zielbindungen, 16:43 UTC

Qs Ereignis q/1/8 und aktualisierte AN_HAUPT.md präzisieren: Sheet-/YouTube-Writer aktivieren ausschließlich die interne Second-Brain-Releasebindung. Dabei müssen Grant-Release und internal_operator.release atomar übereinstimmend geändert werden; öffentliche Bindungen bleiben unverändert. Der vorhandene ActivationPlan für den Standardrelease allein erfüllt diesen Vertrag nicht.

Z muss den gemeinsamen Kandidatenadapter für die tatsächlich unterschiedlichen Zielbindungen auslegen: Ps vereinbarter Standard-/Patchnotespfad und Qs intern gekoppelte Releasefelder, mit vorhandener gemeinsamer Sperre, erwarteter Basis, Journal und Readinessnachweis für die betroffene Bindung. Keine pauschale Änderung aller Grants. P/Q deklarieren das Ziel eindeutig; Z integriert und prüft gemeinsam mit K, dass öffentliche Docs/Twitch-Inhalte und interner Operatorzugang bei der jeweils anderen Aktivierung erhalten bleiben. Das ist dieselbe beauftragte Aktivierung, kein zusätzlicher Funktionsumfang.

## Tatsächlicher Archivabgleich und Datenbankprüfung

z/2/6 ist gelesen. Der empirische Befund zu Defaults, Sequenzen, Fremdschlüsseln, Checks und zusätzlichen Tabellen gehört in die unabhängige Datenbankabnahme des fertigen Archivstands. Nutze dafür vor der produktiven Mutation einen frischen nativen PostgreSQL-/database-reviewer mit dem konkreten Schema-/Generationentausch und Rückweg, kein allgemeines Audit. Tatsächlich angelegte jüngere Daten, Constraints und Sequenzstände müssen erhalten bleiben; kein Septemberstand über aktuelle Kernrevisionen. Diese Prüfung kann nach dem Ende der laufenden Quellwriter erfolgen und darf keine zweite konkurrierende Compiler-/DB-Prüfkette starten. Gemeinsamer Abnahme-SHA bleibt maßgeblich.

## Wache muss die wirklichen Unteraufträge prüfen

P12 belegt offene Workflow-Supervisoren bei längst unterbrochener Runtime-/DevFeed-Arbeit. Prüfe bei jeder eigenen Wache deshalb je aktivem Unterauftrag den tatsächlichen letzten Werkzeugfortschritt, Abschluss-/Unterbrechungsereignis und zugehörige lebende Prozesse beziehungsweise Prüfsperrenwarteprozesse. Ein offener Supervisor oder lebender Hauptprozess allein belegt keine arbeitende Unteraufgabe. Nach sicherem Endnachweis vorhandenen Stand geordnet übernehmen, lebende Prüfer erhalten; kein Neubau von null und keine doppelte Schreibzuständigkeit. Reine Wartezeit auf Hostlocks ist kein Abbruchgrund. Keine Nachrichten in laufende T3-Turns oder problematische Workflowkontexte erzwingen. Bestätigte wiederkehrende Harnessfehler mit konkretem Endstand melden statt unbemerkt warten.
