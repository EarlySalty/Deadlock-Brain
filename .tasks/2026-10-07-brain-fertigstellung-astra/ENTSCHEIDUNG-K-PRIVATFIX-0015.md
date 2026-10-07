# Entscheidung K-Privatfix (08.10.2026, ca. 00:15 CEST, Haupt-Orchestrator im Nutzerauftrag)

Antwort auf `PRIVATFIX-BLOCKER-K.md`. Kleinster sicherer Schnitt, kein neuer Projektions- oder Threadrechte-Bau jetzt:

1. Beide Öffentlichkeitssperren (`answer_discord_event` frühes `BRAIN_PRIVATE_HELP`, Antwortsperre bei modglue.rs ca. :819) entfernen. Antwort geht an `event.channel_id` bzw. in die DM, wenn `can_reply` stimmt. Wer in einem Kanal, Thread oder einer DM schreibt, sieht diesen Ort; dafür ist kein Threadrechte-Resolver nötig.
2. In nicht öffentlichen Kanälen, Threads und DMs liest das Brain für diese Anfrage **keine Discord-Nachrichten** anderer Personen (Discordlive-Lesewerkzeug bzw. `read_messages`-Evidence für diese Anfrage aus). Es antwortet aus Spielwissen, Server-/Doku-Wissen und der eigenen Frage. Damit gehen keine fremden Nachrichtentexte an Luna, und der Threadrechtevertrag wird nicht gebraucht. Das ist keine Game-only-Sperre: Server-, Doku- und Invite-Status-Fragen bleiben erlaubt.
3. Öffentliche Kanäle bleiben wie bisher.
4. Sichere Discordlive-Projektion und Threadsicht für das Mitlesen privater Kanäle sind ein späteres eigenes Paket, nicht Voraussetzung dieses Fixes.
5. Regressionsfälle: Spielfrage im Staff-Kanal, in einem Thread und in einer DM bekommt eine Brain-Antwort ohne Discordlive-Lesezugriff. Dann Gate (Test-Gate kennt seit a593c5d `cargo-slot`), Merge, regulärer Bots-Deploy wie bei 0fb873c6, Nutzerprobe.
