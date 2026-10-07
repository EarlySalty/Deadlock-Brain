# Haupt-Orchestrator: nötige Providerentscheidung für private Antworten

status: aktiv, 2026-10-07, 10:55 UTC

## Urteil

Vollständige private Live-Abnahme ist mit dem derzeit belegten Providerweg nicht zulässig. Q bestätigt tatsächlich laufend `codex_subscription`, Modell `gpt-6-luna`, Basis `http://127.0.0.1:18769/v1`. Der Prozess ist ein Codex-Abo-Proxy mit externem Responses-Ziel. Der Transport übermittelt die geerdete Frage. Loopback bedeutet hier nicht lokale Inferenz.

Die Vorgaben »echte DM-/Communityfragen beantworten und prüfen«, »keine Nutzer-/Communitydaten extern« und »kein Modellwechsel ohne Nutzerfreigabe« lassen sich mit diesem belegten Weg nicht gemeinsam erfüllen. Es wurde durch Q keine private Frage abgesendet, kein Modell gewechselt und kein Dienst verändert.

## Empfehlung und Entscheidung

Datenschutzgrenze beibehalten. Für private Inhalte einen wirklich lokal rechnenden, zentral konfigurierten Provider verwenden, ohne zweite Brain-Engine. Q prüft jetzt, ob ein solcher bereits freigegeben vorhanden ist; es gibt noch keinen belastbaren Modell-/Endpointvorschlag. Falls keiner vorhanden ist, braucht es eine ausdrückliche Nutzerentscheidung über das lokale Modell beziehungsweise dessen Bereitstellung. Keine externe Datenfreigabe und keinen spontanen kostenpflichtigen Anbieterwechsel empfehlen.

Die bereits ausdrücklich freigegebene eigene Invite-Minimalprojektion aus A/EIN-BRAIN.md bleibt zulässig: Enum plus Zeitpunkt, ohne Namen, Steam-IDs, Fremddaten oder rohe Frage-/Kontextdaten. Sie löst die übrigen privaten Antwortpfade nicht.

## Weiterlaufende Arbeit

I/G/K arbeiten weiter; Q sammelt Fragen lokal über kleine funktionierende MCP-Seiten und bestehende Datenleser. Quellenreferenzen und Twitch-Testkanal werden empirisch geklärt, nicht dem Nutzer als Recherchefrage zurückgegeben. Öffentliche Originalspielquellen und isolierte Ausfallprüfungen bleiben bearbeitbar. Der Gate-/Merge-/Liveabschluss wird deshalb nicht vorgetäuscht.

Nachweis: /home/nathanael/.worktrees/brain-q-eval-20261007/.tasks/2026-10-07-brain-fertigstellung-astra/Q/BERICHT.md, Stand 10:47:59 UTC. Konkrete Fortsetzung: ENTSCHEIDUNG-Q-K-DATENSCHUTZ.md. Noch keine vollständige P0-Version, keine echte hero_build_id und kein Abschluss P0 bis P11.
