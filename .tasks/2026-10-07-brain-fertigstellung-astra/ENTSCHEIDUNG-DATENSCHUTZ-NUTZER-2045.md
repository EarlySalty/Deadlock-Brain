# Nutzerentscheidung Datenschutz für das Brain (07.10.2026, ca. 20:45 CEST)

Ersetzt für die Testphase die Sperre aus `ENTSCHEIDUNG-Q-K-DATENSCHUTZ.md`, Abschnitt K.

1. **Testfreigabe:** Der Nutzer gibt frei, dass Fragen aus Discord und Twitch samt dem nötigen Antwortkontext über den bestehenden Provider (`gpt-6-luna` über den Codex-Abo-Proxy) laufen. Grund: erst testen, ohne API-Kosten. Ein späterer Wechsel auf die OpenAI-API oder einen anderen Anbieter ist reine Konfiguration (`provider.kind`, `base_url`, `model`, Schlüssel aus Infisical), kein Umbau.
2. **Weiterhin nie mitgeschickt:** Discord- und Steam-IDs, Mitgliederlisten, fremde Personendaten. Datensparsam bleiben, nur was die Antwort braucht.
3. **Kanalsicht bleibt rollenbasiert (Korrektur 20:55):** Die Brücke (dl-bot `/mcp/public`) zeigt dem Brain nur, was die fragende Person im Discord selbst sehen darf. Das bleibt so; keine zusätzliche harte Kategoriesperre, der Fix-Thread `3e93aeea` ist zurückgezogen.
4. K und Q dürfen damit private Antworten und echte Discord-Fragen in die Abnahme nehmen, unter Beachtung von Punkt 2 und 3.
