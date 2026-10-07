[Orchestrator] Brain-Fertigstellung: Paket Q, echte Evaluation und Live-Abnahme

status: aktiv, 2026-10-07

## 1. Ziel und Vertrag

Du bist ausführender Worker für genau Q. Baue nicht das Brain neu und starte keine zusätzlichen T3-Threads. Direkter Auftraggeber: Delegator 481426fe-b477-42b3-91c6-901811fcba1d. Haupt-Orchestrator: d3a1741e-82bc-4a48-865b-2845c663dca7.

Lies AUFTRAG.md und PAKETE.md in /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/. Dein Auftrag ist P0 und der wiederholbare reale Nachweis P1 bis P10, keine eigene Produktkorrektur oder Code-Review-Rolle. Finde zunächst bestehende Evalwerkzeuge und echte Fragen über code-suche/Graphify. Wiederverwenden statt Parallelwerkzeug.

P0: mindestens 30 echte Fragen aus #bot-logs (1374364800817303632), Discord-DMs und Twitch-Chat, Kategorien Spiel, Patch, Server, Coaching, Bot selbst und Unsinn. Erwartete Antwortart vor dem ersten Lauf festhalten und das Set anschließend unverändert verwenden. Keine synthetischen Ersatzfragen als echte Herkunft ausgeben. Pocket-Konter und Haze-Spirit-Skalierung gehören zu den belegten Kernfragen. Quellenumfang und fehlende Zugänge ehrlich ausweisen. Erwartete Fakten aus aktuellen Originalquellen, nicht aus Modellgedächtnis.

## 2. Eigentum und Datenschutz

Nur eigenes Q-Verzeichnis, vorhandene Eval-/Prüfwerkzeuge soweit eindeutig deinem Auftrag zugeordnet, neue Prüfwerkzeuge ausschließlich Rust. Keine Produktdateien von I/G/K, keine DB-Korrekturen, keine Modell-/Timeoutkonfiguration, keine zweite Provideranbindung. Deadlock-Docs nur lesen. Kanonischer Checkout und fremde Worktrees bleiben unverändert.

Nutzer- und Community-Daten MUST NOT an externe Anbieter, externe Codiermodelle, öffentliche Repos oder einen remote weiterleitenden Loopbackproxy gehen. Rohe Chatlogs und DMs deshalb nicht in deine Modell-Konversation ausgeben. Inhalte ausschließlich lokal mit bestehenden Werkzeugen und gegebenenfalls bestehendem lokalem Provider verarbeiten. Ins Modell nur nichtpersonenbezogene aggregierte Ergebnisse und technische Metadaten. Secrets NEVER ausgeben. Quellenreferenzen, Personenbezug und Originaltexte lokal zugriffsbeschränkt halten. Das echte Set lokal versionieren; keine privaten Inhalte per Git-Push veröffentlichen. Prüfe bei bestehenden Evalpfaden deren tatsächlichen Datenfluss. Ein nicht gefahrlos mögliches Remote-Repo-Artefakt ist eine präzise Datenschutzgrenze, kein Anlass, Daten zu versenden oder Herkunft zu erfinden.

## 3. Arbeitsstand

Eigener Worktree /home/nathanael/.worktrees/brain-q-eval-20261007, Branch feat/brain-q-eval-20261007. Der Delegator hat ihn nach Fetch frisch von origin/main auf f6f5cef65f1f946113f0b8216c6475f6d38ec928 angelegt, ohne Produktänderung. Bestehenden eigenen Worktree verwenden, nicht erneut anlegen. Herkunftsstand beim Start selbst dokumentieren. Dieser Startstand ist kein festzuhaltender Deploy-SHA.

Bestehende aktiven Integratoren: I 8827da25-c1f8-44f2-bef8-f3a7b7dd3137, G a867ef50-88e6-41ac-a852-724f5184c6e6, K 79c97ab5-f014-4e17-9d00-20c7adaf83ff. Kein Kontakt zu deren Sessions, kein Edit ihrer Dateien. Q liefert eine lokal ausführbare Befehlszeile samt Setversion im eigenen Bericht, die alle Deploy-Verantwortlichen benutzen können. Code-/sichere Metadatencommits erlaubt, keine Veröffentlichung von Quelldaten. Integrationsverantwortlicher für deine Prüfwerkzeuge bist du, Produktkorrekturen gehen als begründeter Befund zum Delegator.

## 4. Beweisziel und Abschluss

Erst reproduzierbarer Baseline-Lauf auf tatsächlichem Livezustand, danach je beobachtetem Deploy ein Lauf mit demselben Set und gemessenem Ende-zu-Ende-Zeitwert. HTTP 200 oder reine Kernelprobe sind keine echte Discord-/Twitch-Antwort. Belege P1 unter 20 Sekunden, P2 fünf echte Patchstichproben, P3 Coaching-Kanal 1494373349944459355 bzw. https://deutsche-deadlock-community.de/coaching und Pate in Ich-Form, P4 drei Selbstbildfragen ohne Interna, P5 ehrliche Grenzen, P6 nur eigener Invite-Status ohne Sendewirkung, P7 echte reguläre hero_build_id aus I, P8 Ausfallprobe ohne Produktionsdienst zu stoppen, P9 Prozess-/Health-/Journalbelege der Integratoren, P10 Pfadtabelle inklusive Ersatz. Kein willkürliches Nachrichtenfeuern in fremde Kanäle. Für aktiven Test nur beauftragter #bot-logs und nachweislich zum Nutzer gehöriger Testkanal. Invite nicht an andere senden, keine Veröffentlichung fremder Builds, keinen laufenden Stream trennen.

Für Browserprüfungen zuerst /home/nathanael/Documents/claude-config/wissen/agent-browser.md lesen. Nur Moli; MUST NOT Brave oder persönlichen Browser benutzen. Fehlende Funktionen melden, kein Browserwechsel. Existierende Build-Slots mit --jobs 3 und Release-flock beachten. Prüfsuiten vor und nach eigenen Änderungen korrekt auswerten; Tests nicht wegen Baseline als grün deklarieren.

Bei eigenem Code: lokaler Gate gate_hook.py --review, bei BLOCK je Runde frischer nativer Fixer aus aktueller Pyramide, gleiche Urteilmodellfolge bis ALLOW. Du fährst die Schleife selbst, keine Zwischenmeldung je Runde und kein neuer Review-Thread. Nach fünf erfolglosen Runden qualifiziert melden. Keine Schutzumgehung. Erst ALLOW, dann eigener geprüfter Merge, Push HEAD:main in Git-Einzelschritten. Kein Dienstrestart allein für Dokument-/Evaländerungen. Produktdeploys bleiben bei I/G/K. Abschließen erst nach vollständiger Evaluation oder konkret dokumentiertem externen Blocker; bei offenem Gesamtauftrag nicht verfrüht settlen.

## 5. Routing

Schreibe fachlich nach .tasks/2026-10-07-brain-fertigstellung-astra/Q/BERICHT.md im eigenen Worktree. Kurzes maschinenlesbares STATUS.json mit Paket Q, Versuch 1, Produzent q-eval, Zeit UTC, geprüftem SHA, gebaut/reviewt/gemergt/live und offenen P-Kriterien. TODO.md und zentrales REGISTER.md nicht anfassen. Der Delegator prüft alle 25 Minuten. Bei echter Frage endet deine Rückgabe mit FRAGE AN ORCHESTRATOR samt Empfehlung, keine Frage direkt an den Nutzer. Kein ListAgents/SendMessage und keine Sessionchats. Fertig mit Bereinigung ausschließlich eigener gemergter Artefakte und settle --selbst als letztem Schritt.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-q-eval-20261007
