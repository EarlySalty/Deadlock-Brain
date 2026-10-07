# Paket B: Game-Invite antwortet zwei Stunden zu spät mit „schon eingeladen“

Rolle: Blatt-Worker (GPT 6.1 Sol). Du erledigst genau diesen Auftrag, startest keine weiteren Orchestratoren oder T3-Threads.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Erst `AUFTRAG.md` in diesem Ordner lesen.

## Symptom (Nutzer)

Jemand fragt nach einem Game-Invite (Deadlock-Playtest-Einladung). Das Modell reagiert erst rund 2 Stunden später und sagt dann „du wurdest schon eingeladen“. Beides ist falsch: die Antwort kommt viel zu spät, und der Inhalt ist Blödsinn bzw. passt nicht zur Lage.

## Vorgehen

1. Pfad finden (Graphify zuerst): wo die Invite-Anfrage ankommt (Discord dl-bot, Brain-Antwortweg, Twitch), wer das Modell aufruft und woher es „schon eingeladen“ ableitet. Der Invite-Versand selbst läuft im Steam-Bot (`steam-flows/src/invite.rs`, Binary `steam-bot`, Tabelle `steam.steam_tasks`, `core.steam_links.friend_bot_account_id`). Der Fehler kann repoübergreifend sein (Deadlock-Brain, Deadlock-Bots, Deadlock-Steam-Bot).
2. Echte Fälle in Logs und DB belegen: Zeitstempel der Anfrage, der Invite-Task, der Antwort. Warum 2 Stunden (Warteschlange, Retry, Brain-Tageslauf, Replay alter Nachrichten, Requeue)? Woher die Aussage „schon eingeladen“ (Status `invite_sent`, alte Task, falscher Kontext, Halluzination ohne Daten)?
3. Ursache beheben, nicht das Symptom: eine Antwort kommt entweder zeitnah mit dem echten Status oder gar nicht. Das Modell darf einen Invite-Status nur aus echten Daten nennen. Bot-Texte natürlich, kurz, echte Umlaute.
4. Prüfen, ob dieselbe Ursache auch andere verspätete Antworten erzeugt (Zwilling suchen).

## Abschluss

Eigener Worktree unter `~/.worktrees/<repo>-invite-fix`, Branch `fix/game-invite-spaete-antwort`. Gate-Selbstprüfung, Merge, Deploy, Neustart, Live-Beweis (nur Testkonto, kein Eingriff an echten Nutzerkonten), aufräumen. Bericht in `AN_HAUPT-B.md`: Ursache mit Beleg, Fix-Commit, Live-Nachweis.
