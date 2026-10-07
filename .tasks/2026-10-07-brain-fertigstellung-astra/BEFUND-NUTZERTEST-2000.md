# Befund: Nutzertest im Discord, 07.10.2026 20:00 bis 20:01 CEST (#bot-logs)

Vier Fragen des Nutzers per Erwähnung an den Discord-Bot:

1. 20:00 „welchen Effekt hat Armor Piercing Rounds gegen Plated Armor“ → Brain `InsufficientEvidence` (request `dl-bot-discord-667ae3fd…-7`), Bot antwortet „Dazu habe ich gerade keine gesicherten Infos. Frag bitte Nani hier im Discord.“ Ursache: Live läuft noch der alte Stand ohne Spielwissen (Doku-Release, kein Spiegel, keine Werkzeuge). Wird mit I/G/K live behoben; diese Frage gehört in den Q-Fragensatz.
2. 20:01 eine Nachricht mit drei Erwähnungen und drei Fragen („scaled Haze auch mit Magic dmg?“, „ich bin schlecht in dem Game, was soll ich tun“, „wie countert man Pocket“) → **keine Antwort, kein Brain-Aufruf, kein Log.** Ursache im Code (Deadlock-Bots `rust/bin/dl-bot/src/modglue.rs`, `answer_discord_event`): `BrainOutcome::Cooldown { .. }` beendet stumm. Die Frage kam innerhalb der Brain-Abklingzeit nach Frage 1.

## Auftrag an K (Consumer-Verdrahtung)

- Abklingzeit nie stumm: kurze sichtbare Rückmeldung (z. B. Reaktion ⏳ oder der vorhandene Text `BRAIN_COOLDOWN` mit Restsekunden), einmal je Nachricht, kein Spam.
- Mehrere Erwähnungen mit mehreren Fragen in einer Nachricht: als eine Anfrage an das Brain geben und dort beantworten lassen, nicht verwerfen.
- Diese vier Fragen in die Abnahme (Q) aufnehmen; nach Live-Gang mit dem Testkonto wiederholen.

## Nutzerentscheidung 21:10: freie Unterhaltung statt Abklingzeit

Nutzer: Mitglieder sollen sich vorerst frei mit dem Modell unterhalten können. Die harte Begrenzung im Discord-Weg (`dl-brain/src/lib.rs`, `DiscordRateState::reserve`: eine Frage je Nutzer pro 60 s, 20 je Kanal pro Stunde, 500 pro Tag, einkompiliert) wird gelockert:
- Kein 60-s-Sperrfenster je Nutzer mehr; höchstens ein kurzer Spam-Schutz von wenigen Sekunden.
- Grenzen als Konfigfeld in `bot.toml` (`[ai]`, bestehendes `brain_cooldown_seconds` nutzen statt neuer ENV), großzügige Defaults, keine einkompilierten Zahlen.
- Folgefragen im Gespräch (Antwort auf Bot-Nachricht, DM-Verlauf, vorhandenes `BRAIN_CONVERSATION_TTL`) laufen ohne Abklingzeit.
- Greift trotzdem eine Grenze, sichtbar antworten, nie stumm.
Schnell als eigener kleiner Schritt von K umsetzen und deployen, unabhängig vom Rest.

## Präzisierung 21:15 (Nutzer, verbindlich)

- Keine 60-s-Sperre je Nutzer, keine Stunden-Grenze je Kanal.
- Einzige Grenze: höchstens 50 Brain-Fragen je Nutzer und Tag (Tageswechsel Europe/Berlin), als Konfigfeld in `bot.toml`. Die bisherige globale Tagesgrenze von 500 entfällt.
- Bei erreichter Grenze kurze sichtbare Antwort, dass es morgen weitergeht; nie stumm.

## Nutzerbefund 21:30: Brain muss wissen, wo gefragt wird

19:40 fragte ein Mitglied im Bereich, in dem Mitglieder solche Fragen stellen sollen, „wie kann ich auch deadlock spielen“. Antwort des Brains: „Frag im offenen Invite- oder Community-Bereich …“, also ein Verweis auf genau den Ort, an dem die Person schon war. Ursache: Der Discord-Consumer übergibt dem Brain keinen Ortskontext (`answer_for_discord(question, user_id)` ohne Kanal).

Auftrag an K:
- Jede Discord-Anfrage trägt öffentlichen Ortskontext: Kanalname, Kategorie, Kanalthema bzw. Zweck aus Server-as-Code, Thread oder DM, Art des Eingangs (Erwähnung, Hilfekanal, Guide, DM). Nur was die fragende Person selbst sehen darf (rollenbasierte Sicht), keine IDs an das Modell.
- Das Brain nutzt ihn in der Antwort: nie auf den Ort verweisen, an dem die Person schon ist; stattdessen direkt helfen bzw. den nächsten konkreten Schritt nennen.
- Gleiches für Twitch (Kanal, Partnerkanal ja/nein).
- Fall in die Q-Abnahme aufnehmen.
Doku-Seite zum Einladungsweg wird parallel in Deadlock-Docs korrigiert (Thread `59740e62`).

## Neuer Befund 23:30 nach Bots-Deploy 0fb873c6

Nutzer fragt in #bot-logs (nicht öffentlicher Kanal) „wie countert man Pocket“ → Bot: „Private Fragen kann ich gerade nicht sicher beantworten. Für persönliche Hilfe öffne ein Ticket …“. Um 20:00 hatte derselbe Kanal noch eine Brain-Antwort bekommen.

Ursache: `modglue.rs` `answer_discord_event` schickt in jedem Kanal, der nicht öffentlich ist (`brain_channel_is_public` false), und in DMs pauschal `BRAIN_PRIVATE_HELP`, ohne das Brain zu fragen. Das ist die alte Datenschutzsperre und widerspricht `ENTSCHEIDUNG-DATENSCHUTZ-NUTZER-2045.md` (Testfreigabe über Luna, rollenbasierte Sicht).

Auftrag an K, vorgezogen vor Ortskontext:
- Pauschale Privatsperre entfernen. In nicht öffentlichen Kanälen, Threads und DMs normal ans Brain fragen; die Sicht folgt den Rollen der fragenden Person, Antwort im selben Kanal bzw. in der DM.
- Weiterhin keine IDs, Mitgliederlisten oder Fremddaten an das Modell.
- Den irreführenden Text `BRAIN_PRIVATE_HELP` streichen, sofern er danach keinen Zweck mehr hat.
- Regressionsfall: Spielfrage in einem Staff-Kanal und in einer DM bekommt eine Brain-Antwort. Danach Gate, Merge, regulärer Bots-Deploy, Nutzerprobe.
