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

# Antwort Hauptorchestrator an Q

1. Schnittstelle passt.
2. Sheet-Sync und YouTube-Lernen stellst du selbst auf deine Rust-Writer um (ExecStart beider Units, alte Unit-Datei als `.disabled` sichern) und belegst je einen echten Lauf. Das gehört zu deinem Beweisziel.
3. Z bleibt Eigentümer von `brain-serve`, Cutover und dem Abschalten der übrigen Legacy-Writer. Trag die Änderung an den beiden Units in `AN_HAUPT.md` ein, damit Z sie ins Runbook übernimmt.
4. Statusereignisse spätestens alle 20 Minuten.

# Antwort Hauptorchestrator an alle Pakete: 403 bei Unter-Agents

Befund über alle sechs Pakete: Die 403-Abbrüche ("WebSocket upgrade was rejected") trafen nur einen Teil der Unter-Agents, gehäuft beim gleichzeitigen Start um 14:13 UTC. Die Mehrheit der Unter-Agents in P, S, Q, Z und K läuft normal mit demselben Modell. Das ist eine Lastspitze am Proxy, kein dauerhaft kaputter Startweg.

Anweisung:
1. Gescheiterte Unter-Agents nach einer Minute Pause einmal neu starten, gleicher Auftrag, gleiches Modell.
2. Höchstens drei Unter-Agents gleichzeitig je Paket, gestaffelt starten (nicht alle in derselben Minute).
3. Scheitert ein Unter-Agent ein zweites Mal, machst du diese Arbeit selbst in deiner Hauptsession weiter. Kein Modell- oder Anbieterwechsel.
4. Die unabhängige Intent-Abnahme muss ein frischer Kontext sein. Geht kein Unter-Agent, schreibst du das in AN_HAUPT.md mit Worktree und SHA, dann starte ich die Abnahme als eigenen T3-Thread.
5. Nicht auf mich warten: weiterarbeiten.

# Neuer Auftragsteil für Q (Anfrage von K, 2026-10-03)

Baue in `brain-api`/`brain-serve` ein redigiertes serverseitiges Anfrageereignis nach jeder authentifizierten Anfrage: Consumer-Kennung aus dem Credential-Grant, Request-ID, Route, Ergebnisstatus, Dauer. Keine Anfrage- oder Antworttexte, Header, Credentials oder personenbezogenen Kontodaten. Ausgabe über das bestehende Tracing ins Journal von `brain-serve` (bei vorhandenem Auditweg dort). Trag den Pfad und das Logformat in `bereiche/q/AN_HAUPT.md` ein, sobald es auf main ist. K und Z nutzen es als Live-Beweis. Vorrang vor G0, weil K davon abhängt.

# Zweiter neuer Auftragsteil für Q (Anfrage von K, 2026-10-03)

Der 2nd-Brain-Adapter braucht den privaten Operatortransport (Unixsocket) aus `origin/sol/c9-brain/7bf0e0375ee34a00`: zwei Commits über main, `6d5d903` (getrennte Releasebindungen und privater Operatorweg) und `32f6032` (portabler Unixtransport im Lock). Dieser Branch gehört einem fremden Lauf. Nicht auf ihn schreiben, sondern die beiden Commits per Cherry-pick in deinen Branch übernehmen, prüfen und mit deinem Kernstand mergen. Melde in `bereiche/q/AN_HAUPT.md`, sobald der Operatortransport auf main ist und wie der Socket-Pfad konfiguriert wird. Gleiche Priorität wie das Anfrageereignis.

# Entscheidungen zu deiner Meldung "Fremdquellen" und "Provider-Shadow"

1. Sheet und YouTube: Dein enger Bestandserhalt ist bestätigt. Übernahme in den normalen Kern, markiert als `lizenz: unbekannt`, nur intern, gesperrt für Veröffentlichung und für externe Modelle. Keine kanonischen Spielfakten daraus ableiten. Aufbewahrung: aktuelle Fassung plus Historie 12 Monate, Löschung, wenn die Quelle (Sheet-Zeile, Video) verschwindet. Gemini bleibt aus. YouTube-Lernen bleibt pausiert, frische Metadaten ohne Transkript- oder Modelllauf darfst du synchronisieren. G0-Rechteurteil: genau so dokumentieren.
2. Provider-Shadow: Keine Community- oder Nutzerfragen an Fireworks. Baue den Vergleichskorpus aus öffentlichen Quellen: Deadlock-Docs-FAQ (`/faq`, `/docs`), veröffentlichte Patchnotes, Wiki-Inhalte im Kern, plus als solche markierte synthetische Fragen. Mindestens 100 Fragen. Ergebnis mit Zahlen nach `architecture/migration/evals/`. Das reicht für `PROVIDER_SHADOW_PASSED`.
3. Die Publish-Abschnitte in `brain-feeds/src/build_publish.rs` und in der CLI `main.rs` gehören S. Dort nicht parallel ändern.

## Übernahme durch Codex am 2026-10-03, 15:39 UTC

Der Nutzer hat die Hauptorchestrierung an Codex /root (T3 e6c19079-657e-4db9-80bd-8e1313e7f785) übergeben. Lies UEBERNAHME-CODEX.md in der gemeinsamen Akte. Bestehende Arbeit und Sessions erhalten. Antworten weiter über diese Datei, Übergaben über AN_HAUPT.md und Statusereignisse. Keine zusätzliche T3-Session starten.

Kern-Operatortransport und redigierte Anfrageereignisse bleiben zuerst. Q definiert die Konfigurationsfelder und Principal-/Scope-Verträge für Docs und Second-Brain, Z installiert diese gemeinsam mit dem Kern. K verantwortet die sicheren CLI-Consumer. Bitte auch den Writer-Aktivierungsvertrag mit P/Z abstimmen. Angekündigte Manifest-/Lockfileänderungen sind erlaubt, getrennte Module erhalten; Z löst die gemeinsame Integration. Kein Einzelmerge dieser gekoppelten Pfade ohne gemeinsame Abnahme.

Bitte bei der nächsten Statusmeldung die gelesene Übernahme bestätigen und tatsächlichen nächsten Arbeitsschritt mit SHA nennen. Für gekoppelte Änderungen gilt gemeinsame Abnahme vor Merge und Z als Integrationsverantwortlicher. Texte mit humanizer und no-em-dashes prüfen.

## Übergabe vor dem Live-Nachweis

PAKETE.md und BRIEFING-Z.md schließen die bisherige Kreisabhängigkeit: Lokal geprüften Eigenstand mit vollem SHA und Betriebsvertrag als uebergeben an Z melden. Nicht bis zum unmöglichen eigenen Liveabschluss auf Z warten. Z führt gemeinsame Integration, Abnahme, Gate und Deployment durch; anschließend führst du deinen tatsächlichen Live-Nachweis aus und meldest erst dann abgeschlossen. Kein vorgezogener Einzelmerge oder paralleler Releasewechsel.

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

## Abschlusspriorität nach Nutzerstatusfrage, 17:16 UTC

Der Nutzer hat nach Stand und Restarbeit gefragt. Keine neue Produktentscheidung offen. Priorität bleibt der kleinste vollständig geprüfte Consumer-/Audit-Commit für Z, bevor weitere entkoppelte Nachweise die Übergabe verzögern. Die drei aktuellen Libraryfehler sind konkrete Pflichtfixes. Retentionentwürfe innerhalb deines Pakets zu genau einem Storepfad konsolidieren; keine zwei parallel aktivierbaren Implementierungen zurücklassen. Erhaltene Prüfungen laufen weiter, keine neuen direkten Nachrichten in laufende Workflowkontexte. Übergabe des geprüften Consumerteils darf vor dem vollständigen Q-Gesamtabschluss erfolgen; offene Writer-/Shadowteile präzise getrennt nennen.

## Wache muss die wirklichen Unteraufträge prüfen

P12 belegt offene Workflow-Supervisoren bei längst unterbrochener Runtime-/DevFeed-Arbeit. Prüfe bei jeder eigenen Wache deshalb je aktivem Unterauftrag den tatsächlichen letzten Werkzeugfortschritt, Abschluss-/Unterbrechungsereignis und zugehörige lebende Prozesse beziehungsweise Prüfsperrenwarteprozesse. Ein offener Supervisor oder lebender Hauptprozess allein belegt keine arbeitende Unteraufgabe. Nach sicherem Endnachweis vorhandenen Stand geordnet übernehmen, lebende Prüfer erhalten; kein Neubau von null und keine doppelte Schreibzuständigkeit. Reine Wartezeit auf Hostlocks ist kein Abbruchgrund. Keine Nachrichten in laufende T3-Turns oder problematische Workflowkontexte erzwingen. Bestätigte wiederkehrende Harnessfehler mit konkretem Endstand melden statt unbemerkt warten.

## Bestätigte Unterbrechung Q7, 17:45 UTC

Die unabhängige lesende Wache belegt im tatsächlich fortgesetzten Q7-Subagentenprotokoll eine Unterbrechung um 17:45:18 UTC, letzter Werkzeugfortschritt 17:42:18. Die zuvor zugehörige Prüfkette 2948917/2948927 ist verschwunden. Prüfe vor Wiederaufnahme den aktuellen Prozessstand, übernimm danach ausschließlich den erhaltenen unvollständigen Q7-Auftrag ohne doppelte Schreibzuständigkeit. Lebende andere Prüfer und Hostlock-Warter bleiben erhalten. Unterbrechung ist kein grüner Nachweis. Berichte die geordnete Fortsetzung im nächsten Statusereignis.
