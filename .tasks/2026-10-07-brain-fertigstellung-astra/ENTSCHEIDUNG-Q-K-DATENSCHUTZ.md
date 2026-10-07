# Entscheidung Q und K: Datenschutz und Quellen

status: aktiv, 2026-10-07, 10:55 UTC

Grundlage: Q/BERICHT.md und Q/STATUS.json vom 10:47:59 UTC im eigenen Q-Worktree. Q hat korrekt keine echte Communityfrage an den nachweislich extern weiterleitenden Codex-Proxy gesendet. Das ist ein echter Blocker für die vollständige private Antwortabnahme, keine Erlaubnis für Modellwechsel oder Lockerung.

## Q: weiter bearbeitbare Schritte

1. P0-Quellensammlung ist von der späteren LLM-Ausführung getrennt. Funktionierende kleine `read_messages`-Abfragen mit vorhandener Pagination nutzen und die Daten nur lokal verarbeiten; den defekten Großexport nicht erneut voraussetzen. Keine private Rohdatenansicht im Modellkontext. Lokalen Quellenbestand und datensparsame Herkunftsversion sichern, auch wenn die 30 Fälle noch nicht aus allen Quellen vollständig sind. Keine synthetischen Lückenfüller.
2. `GET /users/@me/channels` ist kein vollständiges DM-Archiv. Bestehende interne Kanal-/Konversationsreferenzen und deren vorhandene Verwaltungsleser über Graphify suchen. Twitch-Chatquelle und zum Nutzer gehörigen Testkanal über bestehende Konfiguration/Verwaltungsdaten lesend verifizieren. Keine neuen Schreibrechte oder breiten Secrets, keine Fremdkanalnachrichten. Nur aggregierte Mengen und technische Fehler im externen Modellkontext. Wenn konkrete Zugriffsreferenzen tatsächlich fehlen, benenne genau die fehlende Quelle statt den gesamten Auftrag auf pauschale Quellenbereitstellung zurückzustellen.
3. Vorhandene zentrale Konfiguration und lokale Dienste rein lesend darauf prüfen, ob bereits ein wirklich lokal rechnender freigegebener Provider für diese Datenklasse existiert. Ein Hostname auf localhost genügt nicht. Kein neuer Connector, kein Anbieter-/Modell-/Timeoutwechsel, kein Download oder kostenpflichtiger Start. Das Ergebnis ist eine konkrete Entscheidungsgrundlage für K und Haupt-Orchestrator.
4. Erwartete Antwortart vor einem Modelllauf festlegen. Wo aktuelle Originalfakten noch fehlen, nicht als vollständig beschriftetes Set zählen. Öffentliche Spielquellen dürfen unabhängig von privaten Frageinhalten geprüft werden. Datensparsame Wiederholbarkeit und Hash-/SHA-Zuordnung herstellen, ohne private Inhalte zu veröffentlichen. Lokale reine Retrievalproben nur als solche kennzeichnen. P8 kann als isolierte Consumer-Ausfallprüfung ohne private Inhalte vorbereitet werden, ohne Produktionsdienst zu stoppen.

Q bleibt zuständiger Worker desselben Auftrags. Keine Nebenaufgabe, kein Ersatzthread. Bericht und STATUS aktualisieren, Produktfixes bleiben bei I/G/K. Der offene lokale Worktree ist nicht erledigt, nur weil sein Startcommit schon auf main liegt. Keine Löschung während aktiver Arbeit und kein Scheincommit zur Beruhigung des Stop-Hooks.

## K: reale Providergrenze

Den konkreten Befund zu codex_subscription über 127.0.0.1:18769 übernehmen. Private FAQ/DM/Community-Kontexte weiterhin vor dem Remoteweg sperren. Bestehende erlaubte öffentliche Antwortpfade und Fähigkeiten weiter verdrahten, kein Modellwechsel. Liefere die kleinste konkrete Restentscheidung für vollständige private Antworten: vorhandener wirklich lokaler Provider ja/nein, erforderlicher bestehender Konfigurationsanschluss und unveränderte Rechte-/Datenverträge. Keine zusätzliche Antwortengine.

Bereits ausdrücklich genehmigte Invite-Ausnahme unverändert erhalten: A/EIN-BRAIN.md erlaubt ausschließlich den eigenen Status als Enum plus Zeitpunkt über den bestehenden Provider nach interner Identitäts-/Rechteprüfung. Keine Namen, Steam-IDs, Fremddaten, Auditrohzeilen oder unbereinigte Frage-/Kontextdaten. Diese bestehende Ausnahme ist kein lokaler Provider und keine Freigabe für andere Communitydaten. PAKETE.md präzisiert die bisher zu pauschale Formulierung entsprechend.

## Eskalation

Der Delegator meldet dem Haupt-Orchestrator den belegten Zielkonflikt mit Empfehlung: private Inhalte nur über lokal rechnenden bestehenden Zentralprovider. Bis eine bestehende Freigabe belegt oder eine konkrete Nutzerentscheidung getroffen ist, keine Remote-Liveprobe mit echten privaten Inhalten. Die übrigen Implementierungs-/Quellenarbeiten laufen weiter.
