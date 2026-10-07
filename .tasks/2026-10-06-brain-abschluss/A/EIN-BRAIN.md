# Ein Brain: belegte Antwortpfade und Umbau

Stand: 06.10.2026, Inventur bis etwa 23:34 CEST. Native read-only Worker A-E1/A-E2, Workflow `wf_bcd0feae-229`, abgeschlossen. Keine Source-/DB-/Dienständerung durch diese Inventur und keine öffentlichen Testnachrichten. Fundstellen beziehen sich auf Git-Objekte des Remote-main, nicht auf fremde uncommittierte Checkouts. Gemeinsame Rohbelege: eigene Taskausgabe `wiahtffx4.output` und Workflowjournal.

## Aktuelle Priorität seit 07.10.2026, 01:45

Zuerst das Grundding live abnehmen: Discord-DM, Erwähnung, Hilfekanal und Twitch liefern zeitnah gemeinsame Brainantworten mit aktuellem Spielwissen, Patch-Story und Serverwissen. A-G1 prüft die tatsächlichen Eintrittspfade und echte Fragen lesend, A-F1b schließt die aktive Profilkette ab. Site-/Kommentarrest und andere Nebenarbeiten warten. Bereits laufender Invite-Skill bleibt als gemeinsamer Quellen-/Identitätsanschluss erhalten, ersetzt aber keinen vollständigen Grundding-Beweis.

Danach wird der vorhandene Build-Reasoner als Brain-Skill genutzt. Fehlende Nach-Patch-Matches über vorhandene Deadlock-API-Quellen ergänzen, keine Publishgrenze senken. Erst reguläres Publish mit echter hero_build_id gilt als Abschluss. D-Ernte über AN_HAUPT-D.md, bei Dateieigentumsüberschneidung hat A Vorrang. Die spätere Inventurreihenfolge unten ist diesem Vorrang untergeordnet.

## Verbindlicher Vertrag

Bots führen Mechanik aus, speichern den echten Zustand und stellen die gemeinsame Brain-Antwort zu. Kein zweiter Antwortweg oder Bot-Invite-Wording. Die gemeldete fehlerhafte verspätete Lounge-Antwort darf B sofort unabhängig von A entfernen. Gewollte funktionierende Antworten bleiben bis zum live belegten Ersatz erhalten.

Nach ausdrücklicher Nutzerfreigabe darf ausschließlich der eigene Invite-Status als Enum plus Zeitpunkt über den bestehenden Provider laufen. Keine Steam-IDs, Namen, Fremddaten, Auditrohzeilen oder unbereinigte Frage-/Kontextdaten. Identität und Rechte intern prüfen, dann die erlaubte Projektion bilden. Kein Modell-/Timeoutwechsel. Die älteren Inventurvorbehalte „erst lokales Modell“ und „B muss auf A warten“ sind durch die Klarstellungen in VON_HAUPT.md für diese genaue Minimalprojektion erledigt. Für sonstige Communitydaten gilt weiterhin die bestehende Datenschutzgrenze.

## Main und Laufzeit getrennt

| Dienst | Remote-main der Inventur | Laufzeitbeleg |
|---|---|---|
| Deadlock-Brain | `10ebbb208aaaac9ddf6e07bc89a6af4a620f86ef` | Tatsächliches Serve-Exe im Maintenance-Layout auf e56e075d; Health/Ready 200, kein aktueller Skillbeleg |
| Deadlock-Bots | von 600b832a auf `e1f11614e437d5e4e5610f5a9c5d997913f292ad` fortgeschrieben | Installierter Release noch 600b832a, Startjournal bestätigt Brainanschluss; direkte Prozess-Exe-Inspektion des Workers verweigert, daher kein neuer Deploybeweis |
| Deadlock-Twitch-Bot | `67786ba2c0b740f964a337dd0d75c19e7958ef66` | Installierter Release/Hash passend, Dienst aktiv; einzelne effektive Modi/Kanaleinstellungen nicht vollständig zugänglich |
| Deadlock--Patchnotes-Bot | `a148aa694db62dfd217d6ca09e0ff56f5b3954a5` | Laufendes Exe stammt aus genau diesem Release |

„Brain ja“ in den Tabellen meint den vorhandenen gemeinsamen `/v1/answer`-Consumer für den eigentlichen Antwortwortlaut. „Teilweise“ heißt gemeinsame Retrieval-/Generatorbausteine, aber noch eigener Wortlaut außerhalb dieses Consumers. Verdrahtung ist kein Zustellungsbeweis.

## Discord und Bots

| Pfad | Wirkung | Brain | Umbauweg |
|---|---|---|---|
| `/home/nathanael/repos/Deadlock-Bots/rust/bin/dl-bot/src/main.rs:1059`, `modglue.rs:395,670,684` | Direkte Botgespräche/DMs und proaktiver Serverguide | ja | Vorhandenen Consumer/Sender und Kanalrechte erhalten, Wissen/Skills im Brain ergänzen. Guide und Lounge verwenden denselben Kanal |
| `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-brain/src/brain_api.rs:65,85,101`, `lib.rs:106-174` | Typisierte Antwort mit echter Discord-Autorenidentität | ja, Kommandos unvollständig | `answer_for_discord` wiederverwenden; `/brain`/`!brain` führen derzeit nicht überall die Requesteridentität mit |
| `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/concierge.rs:5150,5210,5328`, `dl-bot/src/main.rs:1046,1140` | Wissensantworten über alten AnswerEngine, eigener Gesprächs-/Smalltalkwortlaut und Aktionskarten | teilweise | Gemeinsamen Consumer für Antwortwortlaut, bestehende Privatsphäre-/Paten-/Aktions-/Zustellmechanik erhalten. Einzelne aktive Nutzungen nicht belegt |
| `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/faq.rs:825,851`, `dl-bot/src/main.rs:1130,1776` | FAQ und Ticket-Schattenkandidaten über alten AnswerEngine | teilweise | FAQ-/Ticketmechanik und Evidenzfehler erhalten, Brain-Ergebnis in bestehende Darstellung geben |
| `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/passive_help.rs:48,76,119`, `dl-bot/src/main.rs:1777` | Eigener Fragendetektor, dann AnswerEngine | teilweise | Detektor/Kanalgates erhalten, gemeinsamen Antwortconsumer nutzen. Startjournal meldet automatische Hilfe inaktiv, kein fehlender Bedarf bewiesen |
| `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/invite_lounge.rs:416,460,468` auf Live-600b832; `:434,513,540` auf Main-e1f1161 | Einladung ausführen/speichern, bislang eigene öffentliche Statusantwort; Main entfernt fehlerhafte Abschlussantwort | nein | Priorität 1: echte Statusquelle als Brain-Skill lesen. Mechanik und sofortige Fehlerantwort-Abschaltung bleiben bei B. Fehlender-Code-Hinweis auf Main noch fest, nicht pauschal als Statusquelle benutzen |
| `/home/nathanael/repos/Deadlock-Bots/rust/bin/dl-knowledge/src/main.rs:809,811,962` | Legacy `/public/v1/ask` mit eigenem Generator, daneben öffentliche Retrievalschnittstelle | nein für ask | Öffentliche Retrievalquelle weiterverwenden, verbleibende ask-Consumer vor Stilllegung prüfen. Dienst auf Loopback 8896 aktiv, Concierge/FAQ verwenden aktuell Retrieval |
| `/home/nathanael/repos/Deadlock-Bots/rust/bin/dl-bot/src/turnierglue.rs:421,489`, `dl-squads/src/lagebild.rs:684,698,846,876` | Turniervorschläge und Team-/Scrim-Lageberichte mit eigenem LLM | nein | Fakten als Skills, Freigabe/Revision/Aktion erhalten; spätere Nutzenpriorität, nicht neuen Guide bauen |
| `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/coaching_requests.rs:985` | Eigene Textanalyse von Coachinganfragen | nein | Analysewortlaut getrennt von vorhandenen Website-/Reservierungs-/Rollen-/Benachrichtigungspfaden umziehen; reale Nutzung nicht pauschal unterstellen |
| `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/voice_change_hint.rs:10,105,196` | Eigener Detektor und fester erklärender Voice-Hinweis | nein | Erklärung aus Brain, Kanalausschlüsse und Cooldown bleiben Mechanik |
| `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-changelog/src/lib.rs:275,314,334` | Zugelieferten Changelog/Alarm zustellen, erzeugt Wortlaut nicht selbst | kein eigener Generator | Transport erhalten, Wortlaut am tatsächlich produzierenden Dienst umziehen |

## Twitch

| Pfad | Wirkung | Brain | Umbauweg |
|---|---|---|---|
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/bin/tb-bot/src/brain_chat_wiring.rs:375,433,467,505`, `tb-knowledge/src/brain.rs:30,67` | Direkte Fragen, typisierte Brain-Antwort, Threadzustellung/Rate-Limit/Log | ja bei Typed/aktiv | Vorhandenen Consumer nutzen. Frühere echte Answered/Sent-Belege vorhanden; einzelne aktuelle Einstellungen nicht vollständig einsehbar |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-chat/src/pipeline.rs:1104`, `tb-bot/src/chat_wiring.rs:2033` | Angenommene Brainfrage sperrt nachgelagerte Engagementantwort derselben Nachricht | ja, Routing | Eigentum an der Frage erhalten, keine Doppelantwort |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-dashboard-api/src/handlers/self_explainer.rs:442,514,857` | Öffentliche Streamer-Fragebox mit Legacy/Typed/Shadow; Legacy/Shadow formuliert selbst | teilweise | Vorhandenen Typed-Adapter, Verlauf/Quellenprojektion/Antwortvertrag vollständig machen statt neue Fragebox |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-dashboard-api/src/handlers/dashboard_assistent.rs:425,591`, `ai_chat.rs:104,119,139` | Dashboardhilfe und Analyse-Folgechat über zentralen Twitch-Provider | nein | Authentifizierung und echte Dashboardfakten als Skills, Darstellung beibehalten |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-engagement/src/pipeline.rs:268,348,367,489,506,546`, `tb-bot/src/chat_wiring.rs:1776` | Eigener Smalltalk/Engagementgenerator mit Off/Shadow/Test/Live | nein | Freigabe, Unterdrückung und Sichtbarkeit erhalten, Wortlaut im Brain. Generierter Text bedeutet nicht, dass er gesendet wurde |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-chat/src/promo_pitch.rs:863,880`, `promos.rs:893,1085,1228` | Kanalpromos und Partner-/Anlasspitches mit eigenem Generator und Ersatzwortlaut | nein | Trigger, Review, Cooldown, Links und Sender erhalten; Brain formuliert |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-chat/src/invite_question.rs:688,1099,1104,1113`, `lfg_pitch.rs:568,862,872` | LLM-Detektoren und feste Community-/LFG-Antwortpools | nein | Erkennung und Ziel-/Zustellgates erhalten, erklärenden Wortlaut umziehen. Kein Steam-Playteststatus |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-chat/src/standard_replies.rs:46,47,242,291`, `fun_responses.rs:87,122` | Feste Grüße, Veröffentlichungserklärung und Dank | nein | Sachliche Veröffentlichungs-/Einladungserklärung zuerst, Höflichkeitsmechanik gesondert behandeln |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-chat/src/commands.rs:152,602,728,752`, `rank_lookup.rs:300,324,360` | Hilfe, Verbindung/Einladung, Rang/Matchstatistik, Moderation/Raid und Aktionsbestätigung | nein | Kommandos und echte Datenadapter erhalten, erklärenden Wortlaut gemeinsam formulieren, keine Rechte-/Identitätserfindung |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-tips/src/engine.rs:20`, `tb-bot/src/chat_wiring.rs:1869,1885` | Ausgewählte statische Wissenstipps | nein | Auswahl/Verlauf/Gates erhalten, Wissen dem Brain zuführen |
| `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-chat/src/title_ai.rs:788` | Eigene Streamtitelgenerierung | nein | Späterer Skill, Titelvalidierung erhalten |

## Patchnotes und andere Modellpfade

Patchwortlaut entsteht außerhalb des Brains in `/home/nathanael/repos/Deadlock--Patchnotes-Bot/rust/patchnotes-bot/src/runtime.rs:700,736`; bestehender Übersetzungsweg `patchnotes-content/src/translation.rs:91`. `patchnotes-presentation/src/feed.rs:18,20` bietet bereits `brain.feed.patchnotes.v1`, `patchnotes-bot/src/brain_catalog.rs:23` Datenintegration. Feed, Layout, Cursor, Outbox und Multiguildzustellung wiederverwenden, keinen zweiten Patchfeed bauen. Keine eigenmächtige Änderung des bestehenden Übersetzungsmodells.

Erkennung/Entscheidung ist gesondert zu erfassen: Discord `dl-moderation/src/content_analyzer.rs:163`, `content_verifier.rs:77`, `dl-activity/src/lfg_freetext.rs:126`; Twitch `tb-engagement/src/threads.rs:285`, `tb-analytics/src/post_stream.rs:380`, `tb-social-media/src/enrich_pipeline.rs:321`. Diese Modellaufrufe sind nicht alle Antwortgeneratoren. Brain-Lab `tb-dashboard-api/src/handlers/brain_lab.rs:35,82` ist vorhandene lesende Reasonerbrücke. Kein Buttonlabel pauschal durch einen Modellaufruf ersetzen.

## Invite-Skill: belegter minimaler Anschluss

1. Vertrauenswürdige Autorenidentität aus `dl-brain/src/lib.rs:106-174` und Brain `brain-api/src/lib.rs:199-232`; nicht aus Fragetext/Freundescode. Twitch hat aktuell keine bestätigte Discord-/Steam-Personenberechtigung und bekommt keinen persönlichen Status durch öffentliche Retrievalscopes.
2. Vorhandene authentifizierte Quelle `dl-bot/src/mcp.rs:155-227`, `mcp/public.rs:91-175` erweitern. Bestehenden zentralen Pool/Zugang nutzen, kein neuer breiter Brain-DB-Account. Quelle gibt ausschließlich eigene Enum-/Zeitprojektion zurück, anonymen Member-Fallback sperren.
3. Steamquelle `steam-persistence/src/betainvite.rs:43-84`, `invite.rs:25-43,107-139`, `links.rs:187-208`, `steam-flows/src/invite.rs:724-885`: Audit, Request, Task/GC-Code und verifizierte kanonische Links. 161 Auditzeilen, zwölf Code-0-Ereignisse und eine Code-6-Ablehnung im geprüften Wochenfenster. Auditexistenz umfasst Code 5 AlreadyHasGame; Recoveryzeit ist nicht ursprünglicher Versandzeitpunkt. FAILED ohne Code keine sichere Ablehnung; fehlendes Audit kein Beleg für nie eingeladen. Mehrfachlinks/nullable Legacy-steam_id64 nicht raten.
4. Brain `brain-serve/src/discord_live.rs:275-325,436-594` und `brain-contracts/src/lib.rs:168-181,205-208,256-273,465-506` bieten requestgebundene Evidence, Freshness, Verbrauchs- und Publikationsprüfung. Neuer Skill bleibt flüchtig, keine Status-Dauerablage als SourceRecordV2. Antwort bleibt `brain.public.v1` über den bestehenden Consumer.
5. Tatsächlicher Providerpayload wird auf die freigegebene Minimalform reduziert, inklusive Frage/Kontext und Quellenlabels. Keine IDs/Namen/Rohfragen. Fehler/unbekannt nicht als Erfolg, Zeitsemantik sichtbar korrekt. Source und Brain gemeinsam abnehmen, keine Community-Testnachrichten.

## Reihenfolge und Urteil

Inventur abgeschlossen. Invite-Skill A-E3/E4 im jeweils eigenen vorbereiteten Worktree beauftragt, Briefing `BRIEFING-INVITE-SKILL.md`, Workflow `wf_b72ffda3-140`; noch kein Livebeleg. Die Steckbriefauslieferung bleibt gleichrangige Priorität 1. Vorhandenen aktuellen Main-Materialisierungsfix ausliefern, nicht doppelt bauen.

Danach Concierge/FAQ/Passive-Hilfe zuerst, vorhandene Twitch-Typed-Routen/Streamer-Fragebox vervollständigen, anschließend Dashboard-/sachliche Standardantworten/Pitches. Turnier-/Scrim-/Coachingwortlaut, Titel, Tipps und Patchübersetzung nach Nutzen. Bestehende Personenrechte, Freigaben, Fehler-/Evidenzunterscheidung, Cooldowns, Outboxes und Zielkanäle erhalten. Nicht lokale Verarbeitung sonstiger Communitydaten bleibt gesperrt, bis die passende bestehende Freigabe/Providergrenze tatsächlich belegt ist.

Historischer gemeinsamer Vertrag liegt unter `/home/nathanael/Documents/.tasks/2026-09-19-unified-community-game-ai/CONTRACT.md:3-23` und `CD-NACHWEISE.md:3-17`, nicht im aktuellen Brain-Repo. Der dortige botseitige AnswerEngine ist keine aktuelle einheitliche Brain-Endantwort. Wiederverwenden bedeutet dessen Evidenz-/Fehlerverträge erhalten, die gemeinsame Endformulierung heute aber durch den belegten Brain-Consumer führen.

BESTAND[BS-1]: teilweise | Fundort: /home/nathanael/repos/Deadlock-Brain/rust/crates/brain-serve/src/discord_live.rs:275 | Anknüpfung: gemeinsamer Consumer, authentifizierte MCP-Lesegrenze und requestgebundene Evidence
