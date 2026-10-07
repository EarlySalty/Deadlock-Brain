# Auftrag: Deadlock Brain fertigstellen und live bringen (Delegator Astra)

Auftraggeber: Haupt-Orchestrator, Claude-Hauptsession im T3-Thread d3a1741e-82bc-4a48-865b-2845c663dca7, im Auftrag des Nutzers.
Rolle: **Delegator und Überwacher.** Du baust nicht selbst (harte Regel für Astra). Du übernimmst die laufenden Brain-Pakete, verteilst neue Arbeit an Worker-Threads aus der Pyramide (`t3-thread.py pyramide worker_gross|worker_mittel|fixer`, Start über Skill `t3-threads-managen`), überwachst sie alle 20 bis 30 Minuten und führst alles bis zum Live-Abschluss. Register: `REGISTER.md` in diesem Ordner, Aufgabenstand: `TODO.md` hier.
Repos: Deadlock-Brain (Kern), Deadlock-Bots (dl-bot, dl-brain-Consumer, Concierge), Deadlock-Twitch-Bot (Twitch-Chat-Anschluss), Deadlock-Docs (Wissenskorpus, nur lesen/übernehmen).

## Ziel

Das Deadlock Brain ist die **eine zentrale Stelle für jede Antwort an Nutzer**: Discord (Erwähnung, DM, Hilfekanal, Concierge, FAQ, Tickets) und Twitch-Chat. Es hat aktuelles Spielwissen (Helden, Items, Mechaniken, Patches über den lokalen Deadlock-API-Spiegel), Server- und Community-Wissen (Kanäle, Coaching, Paten, Regeln) und Skills (eigener Invite-Status, Build-Reasoner mit Veröffentlichung). Die Bots führen nur Mechanik aus (zustellen, speichern, Invite senden). Concierge und Docs-Repo gehen auf Sicht im Brain auf.

## Was nicht

- Kein zweiter Antwortweg, keine neuen Bot-eigenen Antworttexte, kein neuer LLM-Connector. Modell- oder Timeoutwechsel nur nach Nutzerfreigabe; LLM nur über die zentral hinterlegten Provider.
- Keine Matchdaten lokal speichern, Deadlock-API als Quelle spiegeln; keine Publish-Grenze für Builds senken.
- Nutzer- und Community-Daten nie an externe Anbieter; Invite-Status nur als eigene Minimalprojektion (Vertrag in `.tasks/2026-10-06-brain-abschluss/A/EIN-BRAIN.md`).
- Das Brain weiß, was es ist und wofür es da ist, aber nicht, wie es intern funktioniert (keine Antworten über eigene Modelle, Pipelines, Code). Datenschutzbefehle wie "stopp" bleiben erklärbar.
- Concierge-Code und Docs-Repo nicht löschen, bevor der Brain-Ersatz live belegt ist; Löschen nur mit Nutzerfreigabe.
- Dauerhaft laufende Dienste nur Rust. Twitch-Bot-Seiten nur unter `/twitch/<name>`.
- Keine Session-zu-Session-Chats; fremde Threads, die du nicht übernimmst, nicht anfassen.

## Stand bei Übergabe (07.10.2026, gegen 12:30 CEST)

Nichts aus den laufenden Brain-Paketen ist auf main oder live. Bisheriger Brain-Hauptorchestrator: `a711a4d2-...` (schreibt `.tasks/2026-10-07-brain-grafik-ki/TODO.md`); du übernimmst seine Überwachungsrolle für die unten genannten Pakete. Laufende T3-Threads:

| Paket | Thread | Stand |
|---|---|---|
| I: Integration E (Deadlock-API als Datenquelle) + F (Build-Publish ohne Match-Grenze) | 8827da25 | Gate BLOCK im Patchimport, mehrere Runden; zuletzt: kosmetische Forumtexte gehen als Gameplay-Patch durch. Fixer lief. Akte `.tasks/2026-10-06-brain-abschluss/` (AN_HAUPT-I.md) |
| G: Brain v2 Rechenschicht und Werkzeuge | a867ef50 | Gate fand Retry-Fehler (429/503 brechen Wiederholungen ab), Fixer lief |
| K: zentrale KI und Guide-Integration | 79c97ab5 | seit 08:30 UTC nach API-Fehler mitten in der Antwort ohne Fortschritt prüfen; Stand in `.tasks/2026-10-07-brain-grafik-ki/TODO.md` (Remote feat/brain-k-ki-20261007, Guide-Fix 8745a0eb, Botvertrag noch unverdrahtet, H-Renderer übernommen) |
| Concierge-Fix (Deadlock-Bots) | 858c44f3 | vom Haupt-Orchestrator, gemergt, schließt gerade ab; nicht übernehmen |

Bestandsinventur der Antwortpfade: `.tasks/2026-10-06-brain-abschluss/A/EIN-BRAIN.md`. Live-Symptom heute: Spielfragen ("wie countert man Pocket", "scalled haze auch mit Magic dmg?") bekamen im Discord "Frag bitte Nani", weil das Spielwissen nicht über das Brain antwortet.

## Vorgehen

1. Bestand sichten (Register, AN_HAUPT-Dateien, TODOs, Remote-Branches gegen main), hängende Threads prüfen (K zuerst). Tote Threads geordnet beenden, ihre Worktrees und Branches erhalten und an frische Worker übergeben, nie Arbeit doppelt bauen.
2. Integrationsreihenfolge festlegen (Abhängigkeit G vor K-Grafik, I unabhängig), zusammengehörige Schreib- und Lesepfade gemeinsam integrieren.
3. Worker-Regeln in jedes Briefing: Gate-Fixschleife fährt der Worker selbst mit frischen Fixer-Subagenten bis ALLOW (claude-config `orchestrierung/ABLAUF.md` Schritt 7), Abschluss mit Merge, Deploy, Neustart, Live-Beweis, Aufräumen, `settle --selbst`. Für Twitch-Releases den Prozessnachweis per `deploy-twitch-release --pruefen`, sobald vorhanden.
4. Ein festes Evaluationsset aus echten Fragen anlegen (siehe P0) und nach jedem Deploy laufen lassen.

## Prüfkriterien: fertig ist, wenn alle erfüllt und belegt sind

| Nr | Kriterium | Beleg |
|---|---|---|
| P0 | Evaluationsset mit mindestens 30 echten Fragen aus #bot-logs, Discord-DMs und Twitch-Chat (Spiel, Patch, Server, Coaching, Bot selbst, Unsinn), mit erwarteter Antwortart, versioniert im Repo | Datei + Lauf-Protokoll je Deploy |
| P1 | Spielfragen (u. a. Pocket-Konter, Haze-Skalierung) bekommen in Discord (Erwähnung, DM) und Twitch-Chat eine sachlich richtige Antwort zum aktuellen Patchstand, innerhalb von 20 Sekunden, nie "Frag Nani" | Live-Antworten im Testkanal #bot-logs (1374364800817303632) und einem Twitch-Testkanal, Screens/Logs |
| P2 | Patchfragen ("was wurde bei X im letzten Patch geändert") stimmen mit dem echten Patch; kosmetische Forumbeiträge zählen nicht als Gameplay-Patch | 5 Stichproben gegen Patchquelle |
| P3 | Server-Fragen: Coaching-Wunsch verlinkt in Discord `<#1494373349944459355>`, auf Twitch Discord bzw. https://deutsche-deadlock-community.de/coaching; Paten-Angebot in Ich-Form; nie "schreib dem Concierge" | Live-Antworten |
| P4 | Selbstbild: erklärt, was es ist und kann, verrät keine Interna | 3 Fragen live |
| P5 | Nicht beantwortbares: ehrlicher Spiel-Hinweis statt Erfindung; "Frag Nani" nur bei Fragen zum Twitch-Bot | Eval-Fälle |
| P6 | Invite-Status als Skill: Nutzer erfährt nur seinen eigenen Status, keine Fremddaten | Live-Probe mit eigenem Konto |
| P7 | Build-Reasoner als Skill: Build-Wunsch führt zu regulärem Publish mit echter `hero_build_id` | ID im Bericht |
| P8 | Ausfallverhalten: Brain nicht erreichbar oder langsam, dann antworten die Bots kurz und ehrlich, kein Hängen, kein "Frag Nani" | gezielte Probe |
| P9 | Alle Pakete auf main, deployt, laufende Prozesse aus dem main-SHA (exe-Abgleich), Health/Ready 200, Fehlerjournal seit Deploy leer | Prozess- und Journalbeleg je Dienst |
| P10 | Alte Antwortwege (Concierge-AnswerEngine, dl-knowledge `/public/v1/ask`, FAQ, passive Hilfe) laufen über den Brain-Consumer oder stehen als Abschaltliste mit Ersatzbeleg im Bericht; gelöscht wird nur nach Nutzerfreigabe | Tabelle |
| P11 | Jede Zusammenführung hatte Gate ALLOW; Branches und Worktrees der Pakete aufgeräumt, Threads gesettelt | Register |

Erreichter Zwischenstand je Kriterium gehört in `TODO.md`, nicht in Chatmeldungen.

## Meldungen an den Haupt-Orchestrator (d3a1741e)

Nur: (a) Übernahme bestätigt mit Bestandsliste und Plan, (b) echte Entscheidungen des Nutzers (Modell, Löschen, Geld, Datenschutz, Produktfragen) mit Empfehlung, (c) Blocker nach fünf erfolglosen Runden, (d) Abschlussbericht als Tabelle P0 bis P11 mit Belegen. Keine Zwischenstände je Runde.
