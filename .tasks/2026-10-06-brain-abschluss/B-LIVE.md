# Paket B: passiver Live-Nachweis

## Vor dem Neustart

Frische Vorprüfung am 06.10.2026, 23:15:24.483 UTC: 25 Lounge-Zustände, Fingerabdruck `1e65f35262eee10fd19e4562ad8d4593`, 0 `Pending`, Rückblick-Marker `1791174453`. Globale Steam-Task-Obergrenze `5150330`. Die ältere Bestandsaufnahme unten dient dem Vergleich. Nur lesender Zugriff über die vorhandene lokale Postgres-Rolle, keine Änderung produktiver Daten.

- 25 Lounge-Zustände; 0 `Pending`, 0 `Dispatching`. Alle 25 gespeicherten Objekte besitzen die acht vom Watcher erwarteten Felder. 17 Bitten sind `Attempted`, 3 `WaitingForCode`, 5 Zustände ohne Bitte.
- Fingerabdruck der Zustände: `1e65f35262eee10fd19e4562ad8d4593`.
- Vorhandener Rückblick-Marker `seven_days_v1`: `1791174453`. Er wird nicht gelöscht, um künstlich einen Rückblick auszulösen.
- Freundschaftsanfragen: 55120, letzte Task-ID `5138409`, zuletzt 18:03:02 UTC. Playtest-Einladungen: 93, letzte Task-ID `5138433`, zuletzt 18:03:49 UTC. Laufende Freundschaftsprüfungen werden nicht als neuer Einladungsversand gezählt.
- Letzte gelesene Lounge-Nachricht: `1557091286429991012`, 18:04:52 UTC. In der späteren Beobachtung werden neue Nachrichten nur gelesen, nicht beantwortet.

## Nach dem Neustart

Release-Build im eigenen Worktree: 109m 03s, Exit 0. Alle acht Einträge aus `SHA256SUMS` vor Installation geprüft. Aktivierung mit der im Briefing ausdrücklich beauftragten Release-Sequenz unter `/run/lock/deploy-deadlock-bots-release.lock`, danach Neustart über `bot-restart dl-bot web` am 06.10.2026, 23:23:58 UTC. `current` zeigt auf `releases/e1f11614e437d5e4e5610f5a9c5d997913f292ad`.

| Prozess | PID vorher | PID danach | Laufender SHA256 |
| --- | --- | --- | --- |
| dl-bot | 1405741 | 2848890 | `738b40e2f706b26244316b8f36607d90b50184e8cf4ecad1a87b3e48053ac1c4` |
| dl-web | 2968626 | 2848935 | `df3e44637f0fdfc955475b7477ff3672afd5d34290a7c50309c15cf0db75c611` |

Beide `/proc/<pid>/exe` zeigen auf die neue Releasewurzel, ohne `(deleted)`. Beide Fehlerjournale ab 23:15:24 UTC leer, `NRestarts=0`. Bot-Anker `beendet abgelaufenen Versand-Claim` im laufenden Artefakt vorhanden. MCP gebunden um 23:24:05 UTC, Discord Gateway READY um 23:24:07 UTC. Der zuvor laufende Web-Prozess stammte aus `5e77e7ef`, der Bot aus `600b832a`; beide wurden durch die neu gebauten Artefakte ersetzt.

Passiver Vergleich mit `B-PASSIV-LIVE.sql` um 23:25:21.439 UTC: 25 Lounge-Zustände, unveränderter Fingerabdruck, 0 `Pending`, 0 `Dispatching`, 0 Bitten seit Neustart, unveränderter Rückblick-Marker. Seit Task-ID `5150330`: 0 neue Freundschaftsanfragen, 0 neue Playtest-Einladungen. Discord-MCP meldet ab der frischen Vorprüfung 0 Nachrichten im Kanal. Damit ist keine neue Rückblick-Wirkung im beobachteten Start festgestellt. Wegen des vorhandenen Markers wurde kein erneuter vollständiger Rückblick erzwungen.

Die passive Beobachtung läuft bis etwa 07.10.2026, 00:24 UTC (02:24 CEST). Die einmalige Abschlussprüfung ist für diese Session eingeplant; sie setzt voraus, dass die Session geöffnet bleibt. Falls keine Bitte erscheint, wird kein positiver Live-Versandbeleg behauptet. Worktree sowie lokaler und Remote-Branch sind bereits entfernt, Belegsicherung in `B-CLEANUP.md`. Der aktive Test bleibt beim Nutzer mit seinem Zweitkonto.

## Aktiver Test für den Nutzer

In [#frag-die-community](https://discord.com/channels/1289721245281292288/1426220702054355077) mit dem Zweitkonto schreiben:

```text
Kannst du mich einladen? <Steam-Freundescode des Zweitkontos>
```

Den Platzhalter durch den eigenen Steam-Freundescode ersetzen, keinen fremden oder ausgedachten Code verwenden. Bei noch nicht eingeladenem Konto sollte eine Steam-Freundschaftsanfrage bzw. Einladung erscheinen. Eine Freundschaftsanfrage in Steam annehmen, damit die Einladung folgen kann. Bei bereits erfolgter Einladung ist kein neuer Versand zu erwarten. Die Invite-Lounge sendet keine eigene öffentliche Ergebnisantwort, auch keine späte Meldung „schon eingeladen“. Der Brain-Antwortweg wird getrennt durch Paket A ergänzt.
