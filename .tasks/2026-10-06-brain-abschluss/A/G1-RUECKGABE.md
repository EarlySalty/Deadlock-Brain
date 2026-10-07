# A-G1: Grundding noch nicht abgenommen

Read-only Abnahme vor neuer Recovery, abgeschlossen 07.10.2026 etwa 02:40 CEST. Keine Produktcode-, Konfig-, Dienst- oder öffentlichen Nachrichtenänderungen. Die hier dokumentierten Messungen ersetzen keine neue Abnahme nach fde910f6-Recovery durch den exklusiven Live-Agent.

## Backend und Korpus

Fünf freigegebene vorhandene Abnahmefragen, HTTP 200 und passende Request-ID. Steam-Wartung Discord: 4961 ms, answered, ein Beleg. Haze: 1610 ms, insufficient_evidence. Mystic Burst: 2060 ms, insufficient_evidence. Bullet Dance/Patch-Story Discord: 2164 ms, insufficient_evidence. Patch-Story Twitch: 2281 ms, insufficient_evidence. Das sind Backendzeiten, keine Zustellungszeiten.

Aktiver Korpus: firstparty-documentation-v1, 62 Quellenbindungen, Steamrevision korrekt. Keine aktive Spielprofil-/GameTracking-/Patchnotesquelle. Gemessene Runtime damals Brain 10ebbb20/PID 2932843, Discord e1f11614/PID 2848890, installierter Twitchrelease 67786ba2. Direkte Twitch-Exe-Inspektion EACCES, nicht umgangen.

## Eintritt und Zustellungsgrenzen

DM und Erwähnung haben gemeinsamen Brainconsumer, der Guidepfad für Kanal 1426220702054355077 ebenfalls. Zugänglicher DM-Leseweg lieferte keine Kanäle; je 200 Allgemein-/Guidenachrichten liefern keinen frischen passenden Nachweis nach Botstart. Separate automatische Discordhilfe ist inaktiv, nicht mit direktem Guidepfad verwechseln. Historische verspätete Loungeantworten zählen nicht.

Twitch besitzt typed Consumer, Threadsender, Duplikatsperre und Ledger. Betriebsconfig nicht lesbar und vorhandener DB-Zugang ohne SELECT auf Zustellledger: frischer tatsächlicher Zustellnachweis offen. Keine öffentlichen Testnachrichten erzeugt.

## Konkrete Lücken

1. Spielprofile/Patch-Story wirklich veröffentlichen und aktivieren, normale Fragen auf neuer Runtime prüfen. Recovery und Tick gehören jetzt ausschließlich dem fremden Live-Agent.
2. Vorhandene enge Twitch-Profilfreigabe fehlt; G2 konkretisiert genau ein boolesches Feld, ohne neue Sourcepolicy oder Datenfreigabe.
3. Discord verliert Brain-Request-ID vor dem Sender, gemeinsamer Eingangs-/Request-/Zustellnachweis fehlt. Fundstellen dl-brain/src/brain_api.rs:107 und dl-bot/src/modglue.rs:743. Twitch hat bereits gemeinsame Identität/Sent in tb-bot/src/brain_chat_wiring.rs:433. E4 besitzt die bestehenden betroffenen Discordanschlussdateien, kein paralleler zweiter Umbau durch A.

Belege /tmp/brain-a-grundding-proof-20261007/backend-probes.json, current-corpus-steam-binding.json, runtime-end.json, discord-guide-event-metadata.json, discord-general-event-metadata.json.

Sicherheit: G1 meldet Anteil von TWITCH_ANALYTICS_DSN in fehlgeschlagenem lokalem Verbindungs-Tooloutput. Keine Werte hier wiederholt, kein Kontozugriff behauptet. Prüfung über bestehenden Secretprozess an Haupt gemeldet. Keine unbestätigte Rotation oder Behebung behauptet.
