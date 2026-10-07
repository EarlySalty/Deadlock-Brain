[Orchestrator] Paket I. Worktree /home/nathanael/.worktrees/insights-cdp-20261005, Branch fix/insights-cdp-20261005. Freigabe erst nach fremdem Prüfer. Kein Push, kein Deploy.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/insights-cdp-20261005

# Paket I: Insights-Handshake bleibt fehlgeschlagen

Rolle: Blatt-Worker. Du startest keine weiteren Threads.

Auftraggeber: Kopf, T3-Thread 31575951-23e7-4dd9-b1af-8700f7ff45fe.
Bericht: /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-05-nebenfehler/AN_HAUPT-I.md

## Ziel

`dl-insights-sync.service` ist seit dem 05.10.2026 07:15:23 CEST `failed`, Status 1. Vorher schon um 06:15. Der nächste Timer ist wöchentlich, die Unit holt das von selbst nicht nach.

Das Binary ist gestartet. `ensure_brave` hat einen vorhandenen Browser-Endpunkt gesehen. Danach steht im Journal:

`Allow-Overlay nicht weg, öffne Inspect-Tab`

und anschließend zweimal:

`CDP-Handshake Timeout unter ws://127.0.0.1:33369/devtools/browser/f2c5b220-4710-4066-956b-f933adb4acdd. Allow-Klick hat nicht gegriffen oder ein anderer DevTools-Client haelt den Socket.`

Fundstelle auf `origin/main`: `rust/bin/dl-insights-sync/src/brave_cdp.rs`, `connect_browser` und `handshake_with_keys`. Das Zeitlimit dort ist 10 Sekunden. `rust/bin/dl-insights-sync/src/allow.rs` schickt über xdotool zuerst Tab, dann Return, weil Cancel den Fokus hat.

Soll: Ein Endpunkt, der die Verbindung annimmt, wird ohne menschlichen Klick und ohne xdotool verbunden. Ein gehaltener Socket oder ein ausbleibendes Allow bleibt ein Fehlerstatus mit der vorhandenen Fehlerklasse. Die Unit wird dadurch nicht grün. Der Wochentimer bleibt. Das Zeitlimit wird nicht als einziger Unterschied angehoben.

## Eigentum

Worktree: `/home/nathanael/.worktrees/insights-cdp-20261005`
Branch: `fix/insights-cdp-20261005`
HEAD beim Start: `5e77e7ef744876c82cf5b2c530778ec1549a4e62`

Schreiben darfst du nur unter `rust/bin/dl-insights-sync/` und in Tests dieses Bins.

Unangetastet: das laufende Brave, Display `:10`, `/home/naniadm/Documents/Deadlock-Bots`, die Knowledge-Unit, das Skript `scripts/run_dl_knowledge_service.sh`, systemd, der Wochentimer.

## Beweis

Ein lokaler Fixture-Test zeigt beide Seiten. Ein Websocket, der die Verbindung annimmt, liefert Erfolg, ohne `confirm_allow` aufzurufen. Ein Websocket, der nicht annimmt oder den Handshake nicht abschließt, liefert den Fehlerstatus. Kein Klick im echten Brave, kein Neustart von Brave, kein Live-Lauf der Unit.

Wenn der Code ohne einen Menschen am Display nicht verbinden kann, hörst du auf und schreibst genau das in den Bericht. Dann bleibt der Timeout unverändert.

## Abschluss

Kein Push, kein Merge, kein Deploy, kein `systemctl`. Bericht mit Branch, SHA, Testbefehl und Ergebnis nach `AN_HAUPT-I.md`. Der Kopf schickt den Stand danach an einen fremden Prüfer.
