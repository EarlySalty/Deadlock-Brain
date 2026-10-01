# Datenbankgebundene Tokenablage

Die produktive Gemini-Strecke läuft vollständig in Rust und spricht Chromium über eine private CDP-Pipe an, ohne offenen Debug-Port. dbrain-session-store ist ihre gemeinsam verwendete Speicherbibliothek und hält Cookies, LocalStorage und IndexedDB verschlüsselt in core.browser_credentials. Der Browserkontext ist nichtpersistent; temporäre leere Chromium-Startdaten enthalten keine Kontositzung. Python bleibt ausschließlich Legacy-Referenz. Konto-ID und Revision sind zwingend; ein Widerruf ist ein Fehler und kein leerer Standardzustand.

## Verbindliche Speichergrenze

Kontozugänge und wieder benötigte Geheimwerte werden nur feldverschlüsselt in der Datenbank gespeichert. Für reine Bearer-Prüfungen wird der Rohwert nicht zurückgewonnen, sondern ein irreversibler Lookup-Schlüssel verglichen. Die vorhandene tb-crypto-Implementierung bleibt die gemeinsame Feldkrypto. Konto- und Feldkontext müssen authentifiziert sein.

Keine neuen Token-JSONs, Refresh-Dateien, dotenv-Zugänge oder persistenten Browserprofile. Kein stiller Datei-, Klartext- oder Fremdkonto-Fallback. Keine Tokenwerte, verschlüsselten Blobs, Session-URIs oder Browserzustände in Logs, Berichten oder Shellargumenten.

Master-Key, OAuth-Anwendungssecrets, Datenbankverbindung und Infisical-Bootstrap bleiben außerhalb der Anwendungstabellen im bestehenden Secret-Manager. Sie sind keine zweite Ablage der rotierenden Kontotokens. Ein Verschlüsselungsschlüssel wird nicht neben seine eigenen Ciphertexte gelegt. TradingBot gehört nicht zu dieser Umstellung.

## Lokaler Arbeitsstand

Die zusammengehörigen Worktrees liegen als Geschwister unter /home/nathanael/.worktrees/token-db-local-20260930/. Die vorhandenen relativen Abhängigkeiten auf Deadlock-Bots und tb-crypto werden dort wiederverwendet. Quellkopien, ein zweites Kryptopaket und Änderungen an geteilten Checkouts sind nicht nötig. Alle Rust-Prüfungen verwenden den vorhandenen sccache und die gemeinsame Zwischenablage /home/nathanael/.cache/rust-build/{workspace-path-hash}.

Produktive Datenbanken und Konten wurden für diesen Arbeitsstand nicht migriert. Änderungen sind keine Aussage über die laufenden Dienste. Tests dürfen nur explizite Wegwerf-Datenbanken und synthetische Konten verwenden.

## Späterer koordinierter Cutover

1. Zugehörige neue Leser und Writer gemeinsam bereitstellen, vorhandene verschlüsselte Sicherung und Wiederherstellungsweg prüfen. Keine angewandte Migration ändern.
2. Alte Writer anhalten. Neue zentrale Schema-Migrationen anwenden. Gehashte Session-IDs nicht mit einem alten Consumer mischen. Bestehende Restore-/ETL-Werkzeuge vor einem Import auf das neue Tokenformat abstimmen; die Constraints lehnen Rohwerte ab.
3. Bestehende Steam-Guard-Werte ausdrücklich per privater Pipe an das Beispielprogramm import_guard im Steam-Core geben. Es nutzt dieselbe Kontokonfiguration und denselben Secret-Launcher wie der Dienst. Gleiche Freigaben dürfen erneut importiert werden, andere bestehende Werte und Widerrufe werden nicht überschrieben. Keine alten Dateien durch den Agenten öffnen.
4. VOD-Resume-Werte vor dem Start mit dem Beispiel migrate_resume_sessions im Twitch-VOD-Paket und beim separaten Archiv mit --migrate-token-storage transaktional umstellen. Danach den NOT-VALID-Constraint der Twitch-Tabelle validieren. Ein Fehler lässt den jeweiligen Migrationsbestand unverändert. Abgeschlossene Uploads behalten ihre Video-ID.
5. Kontozuordnung, Entschlüsselung und Neustart-Wiederaufnahme prüfen. Erst danach alte Credential-Dateien oder Bootstrap-Kontotokens kontrolliert außer Betrieb nehmen. Die vorhandenen Dateien werden hier weder gelöscht noch als Backup verdoppelt.

Ein Code-Rollback allein reicht nach einem irreversiblen Hash-Cutover nicht. Entweder die neuen Lookup-Verträge beibehalten oder gemeinsam auf einen zuvor geprüften Datenbankstand zurückgehen. Ein nicht durchgeführter Restore-Test ist keine bestätigte Rollback-Fähigkeit.

## Rust-Laufzeit und bisherige Pause

`config/gemini-browser.json` enthält normale Einstellungen: stabile Konto-ID, Infisical-Konfigurationspfad, Geheimnisnamen, Browserpfad, Antwortfrist und `learning_enabled`. Die bisherige Pause bleibt mit `false` erhalten. Der Timer startet `scheduled-auto-learn`; bei Pause lädt er weder Secrets noch Browser und ruft kein Modell auf. Freigaben für Modelle und Producer bleiben unverändert.

Die Rust-Strecke liest den direkten Infisical-Snapshot einmal und teilt ihn zwischen Datenbankpool, Sessionstore und optionalen Hinweisen. Sie importiert keine Token-Dateien und verwendet keine ENV-Konfiguration. Ein fehlender DB-Zustand wird bei Analyse als fehlende Anmeldung gemeldet; ein Widerruf bleibt ein Fehler. `gemini-login` prüft die Anmeldung, bevor er Zustand speichert. Die Tokenumstellung aktiviert den Lernbetrieb nicht.

Der separate Helfer erhält normale Einstellungen und die Nummer einer geerbten privaten Pipe als CLI-Argumente (`get|put CONFIG PIPE_FD`). Keine Geheimwerte stehen in Argumenten oder Ausgaben. Kontozustand wird revisionsgebunden gespeichert. Nicht darstellbare IndexedDB-Werte brechen die Speicherung ab, statt Daten still zu verwerfen. Optionale Pausenhinweise nutzen einen vorhandenen Infisical-Webhook; ihr geheimnisfreier Status begrenzt auf eine Meldung pro Tag und zwei pro sieben Tagen und zählt unterdrückte Wiederholungen.
