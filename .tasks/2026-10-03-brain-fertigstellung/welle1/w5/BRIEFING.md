# W5 Discord-Live-Fakten fürs Brain

Rolle: Blatt-Worker. Auftraggeber D1, Kopf Hauptsession T3 `92efdb66-6e7e-495c-873e-32f2912c2fa2`. Regeln: `../../DELEGATOR-REGELN.md`. Bericht an D1 in `AN_D1.md` neben dieser Datei.

## Ziel

Das Brain kann Fragen zum Discord-Server mit aktuellen Live-Fakten beantworten, etwa „Welche Lanes gibt es?“, „Wo finde ich X?“, „Ist gerade wer im Voice?“. Nur lesend und nur aus öffentlichen Bereichen.

## Was erlaubt ist

- Kanalstruktur: Kategorien, Kanalnamen, Kanaltyp, Topic, Reihenfolge.
- Voice: Anzahl der Leute je öffentlichem Voice-Kanal bzw. je Lane. Keine Namen.
- Infotexte, die unser eigener Bot in öffentliche Kanäle gepostet hat (Panels, Anleitungen, angepinnte Bot-Nachrichten).

„Öffentlich“ heißt: Die Rolle @everyone (bzw. die normale Mitgliederrolle nach Verify) darf den Kanal sehen. Das wird aus den Berechtigungen berechnet, nicht aus Kanalnamen.

## Was ausgeschlossen ist

Tickets, Moderation, Admin- und Team-Kanäle, DMs, alles, was @everyone nicht sieht. Keine Nachrichten von Nutzern, keine Nutzernamen, keine Profile. Nichts davon geht an den Antwort-Provider.

## Weg

1. Bestand nutzen statt neu bauen: dl-bot hat schon lesende Discord-Zugriffe (`rust/bin/dl-bot/src/mcp.rs`: `server_overview`, `list_channels`, `read_messages`) und das Brain hat Quellen- und Retrievalpfade. Zuerst per Graphify prüfen, wo eine Live-Quelle am kleinsten andockt. Kein zweiter Discord-Client und kein neuer Token, den bestehenden internen Weg wiederverwenden.
2. Filter auf öffentliche Kanäle zentral an einer Stelle, mit Test, der einen Ticket- und einen Mod-Kanal ausschließt.
3. Kurzer Cache (etwa 60 Sekunden), damit nicht jede Frage Discord abfragt.
4. Prüfen, Gate, Merge, Deploy über die bestehenden Wege (Brain über W1 bzw. `../w1/EINGANG.md`, dl-bot über den bestehenden Bots-Weg).
5. Abnahme live: In earlysalty liefert „Welche Lanes gibt es auf dem Discord-Server?“ die echten Lanes. Testfrage über D1 an die Hauptsession.

Nur das Nötige, keine Extras.
