# G1: dieselben fünf Fragen nach Profilaktivierung

Ausgangsmaterial: `/tmp/brain-a-grundding-proof-20261007/*.request.json`. Texte, `bot.public` und Profil `explain` bleiben unverändert. Jeder neue Aufruf erhält eine neue Request-/Conversation-ID. Keine öffentliche Testnachricht, kein eigener Tick, Writer, Neustart oder Build. Aktivierung ausschließlich live_strecke auf finaler Source bfda408cb988722ddceadb56bca5b72e12d12731.

## Fragen

1. Discord, Steam: „Wann ist üblicherweise Steam-Wartung und wie lange dauert sie?“
2. Discord, Haze: „Gib mir einen belegten Steckbrief zu Haze in Deadlock: Fähigkeiten, Rolle und wichtige bekannte Werte. Trenne unbekannte oder widersprüchliche Angaben klar ab.“
3. Discord, Mystic Burst: „Erkläre den Deadlock-Gegenstand Mystic Burst anhand der belegten Spielwerte. Welche Wirkung und welche Grenzen sind bekannt? Behaupte keine unbekannten Werte.“
4. Discord, Bullet Dance/Patch-Story: „Was macht Hazes Fähigkeit Bullet Dance in Deadlock? Nenne belegte Werte und, falls vorhanden, ihre dokumentierten Patchänderungen. Erfinde keine Patch-Story, wenn dafür Belege fehlen.“
5. Twitch, Bullet Dance/Patch-Story: derselbe Text wie Frage 4, vorhandenes twitch-bot-Credential, keine Ausweitung der Freigabe.

## Abnahme

Nach belegter Aktivierung einmal über den bestehenden gemeinsamen Antwortweg prüfen. Je Antwort Status, passende Request-ID, tatsächliche Belege, Release-/Wissensbindung und Backenddauer festhalten. HTTP 200 allein zählt nicht. Spielantworten müssen `answered` mit Beleg liefern; Aktualität des Patches und dokumentierte Änderungen getrennt verifizieren. Einen unbelegten aktuellen Wert oder eine erfundene Patch-Story nicht als Erfolg werten.

Die fünf Backendfragen beweisen keine Nachrichtenzustellung. Discord-DM, Erwähnung, Hilfekanal und Twitch getrennt über echte vorhandene Frage-/Antwortereignisse abnehmen. Fehlende Twitch-Profilfreigabe bleibt G1-Lücke 2; Discord-Request-ID bis Sender bleibt Lücke 3. Kein Zugriff auf Secrets oder Rohkonfiguration für den Bericht, keine Werte aus Verbindungsfehlern ausgeben.
