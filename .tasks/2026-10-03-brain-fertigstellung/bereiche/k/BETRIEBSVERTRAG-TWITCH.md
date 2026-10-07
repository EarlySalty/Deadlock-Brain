status: Bau vorhanden, Nachprüfung und gemeinsame Nutzung offen
Datum: 2026-10-03

# Twitch: enger Konfigurationsvertrag für Z

Vorfixkopf: ffa037c885ca45c0b7a43759ed6714305ed92f99. Abschließender SHA folgt nach vollständiger Clippy-/Testprüfung und frischer Intent-Abnahme. Keine produktive Ausführung in K.

## Vorhandene CLI-Ergänzung

Binary tb-config-check im Crate tb-config. Bestehende Gesamtprüfung bleibt erhalten. Neue Inspektion liefert ausschließlich revision, bot_brain_client.{mode,endpoint}, bot_brain_chat_enabled und dashboard_brain_client.{mode,endpoint}:

```bash
tb-config-check --config /var/lib/deadlock-twitch/config/bot.toml --brain-inspect
```

revision ist SHA256 der tatsächlichen gespeicherten UTF-8-TOML einschließlich Kommentaren und Formatierung. Sie ist nicht der semantische fingerprint aus dem bisherigen Dashboard-Pooleditor. Den früher beobachteten Fingerprint a5ca906012a5c8cb5637cb560ae0011b1c47320ca6a496f679e9f319db13f55f keinesfalls als expected-revision verwenden.

## Erlaubter Schreibumfang

```bash
tb-config-check --config /var/lib/deadlock-twitch/config/bot.toml \
  --brain-apply --expected-revision <frisch-inspektierter-Datei-SHA256>
```

Streng typisiertes JSON auf stdin, höchstens 16 KiB. Genau fünf optionale Felder; weggelassene Werte bleiben erhalten. Beispiel für die erst später gemeinsam freigegebene Aktivierung:

```json
{
  "bot_brain_client": {
    "mode": "typed",
    "endpoint": "http://127.0.0.1:8788"
  },
  "bot_brain_chat_enabled": true,
  "dashboard_brain_client": {
    "mode": "typed",
    "endpoint": "http://127.0.0.1:8788"
  }
}
```

Modi legacy, shadow, typed. Unbekannte Felder, doppelte Schlüssel, null, falsche Typen und leere Änderungen werden abgewiesen. Bestehende lokale URL- und Gesamtvalidierung bleiben wirksam. Scopes, Zeitlimits, Secrets, Provider und sonstige Einstellungen werden weder ergänzt noch geändert. Falls der bestehende Scopebestand keine Aktivierung zulässt, konkret melden und den engen Vertrag nicht umgehen.

Fester produktiver Schreibpfad, bestehende Editorsperre, erwarteter alter Dateihash und atomarer Austausch. Eigentümer, Gruppe und Modus erhalten, keine erhöhte Rechtebeschaffung durch das Binary. Symlinks, harte Links beim Schreiben, Sonderdateien, Checkoutablagen, unzugängliche Sperren und Revisionskonflikte führen zum Abbruch. Erfolg Exit 0, Fehler Exit 2 ohne Eingabeinhalt. Ausgabefehler nach erfolgtem Austausch setzen die Änderung nicht zurück; vor einem erneuten Versuch erneut inspizieren.

## Installation und Nutzung

Die vorhandenen Releaseinstaller verteilen tb-config-check noch nicht. Z muss das Binary aus dem gemeinsam geprüften Integrationsstand SHA- und hashgebunden über den bestehenden zulässigen privilegierten Verwaltungsweg installieren. Der passende enge Wrapper muss das feste Ziel und diese begrenzten Operationen erlauben; kein generisches sudo oder neuer HTTP-/Diagnosedienst. Kein installierter Pfad oder bereits wirksame Wrapperfreigabe wird hier behauptet.

Das ausführende Konto benötigt Configleserechte, Schreib-/Suchrechte im Verzeichnis, Zugriff auf .bot.toml.lock und Rechte zum Metadatenerhalt. Ein tatsächlicher Deny wird mit genauer Aktion und Grund dokumentiert. Eine geschützte TOML wird nicht über einen neuen Umweg vollständig ausgegeben.

Vor jeder Änderung frisch inspizieren und den alten erlaubten Felderstand für einen kontrollierten Rückweg lokal sichern. Erst nach gemeinsamer Consumer-/Security-Abnahme und Kerninstallation ändern. Der Befehl startet keine Dienste und beweist keine geladene Konfiguration. Z koordiniert den vorhandenen Neustartweg bot-restart twitch-bot twitch-dashboard und überprüft den tatsächlich geladenen SHA, keine generischen sudo/systemctl-Aufrufe.

Chataktivierung ist global für die vorhandenen berechtigten Partnerkanäle; kein einzelner Brain-Kanalschalter wurde gefunden. Diese Reichweite und die Nebenwirkungen müssen gemeinsam abgenommen sein. Die frühe Kanalfreigabe für earlysalty erlaubt keinen vorgezogenen globalen Cutover. Dashboard-Testfragen erzeugen DB- und Discord-Logging-Nebenwirkungen und sind nicht wirkungsfrei.

## Fachlicher Nachweis

Nach geprüfter Installation nur wenige sachliche echte Erwähnungen in earlysalty über einen vorhandenen bestätigten Sender. Auth-/Helix-User-ID und Botidentität prüfen; kein Bot-eigener Sender, keine fremde Identität. TESTFREIGABE-TWITCH.md gilt. Antwort zur ursprünglichen Twitch-Nachricht nachweisen, deren Message-ID ist die Brain-Request-ID. Deren SHA256 ohne Zeilenumbruch muss zum redigierten authenticated_request des Consumers twitch-bot passen. Dazu tatsächliche auditierte fachliche Antwort mit auflösbarem Beleg und Original-Reply. Ein authentifizierter Kernaufruf allein ersetzt Chateingang und Reply nicht.

Belegte Quelle: rust/docs/brain-config-cli.md im eigenen Twitch-Worktree, dazu ENTWURF-TWITCH-CONFIG.md und SCOUT-twitch.md. Kein eigenes Bug-/Securityreview; alleiniger Reviewer bleibt der Merge-Gate über Zs gemeinsame Integration.
