# Brain-Abschluss: Quellenvertrag und reproduzierbare Messung

Stand: 21. September 2026. Aufbauend auf Integrationscommit `22c8ebe`.

## Änderungen

Der Assets-Import verwendet den aktuellen offiziellen Vertrag unter `https://api.deadlock-api.com/v1/assets/`. Die Spezifikation wurde am 21. September über `https://api.deadlock-api.com/openapi.json` geprüft. Der frühere Assets-Hostname war vom Server nicht mehr auflösbar. Auch Katalog-Sync und der Provenienzfilter des Patch-Reviews berücksichtigen den neuen Ursprung; historische Quellen bleiben im Review erkennbar.

`pull assets --client-version <positive Zahl>` fixiert die Spielversion. Ohne diesen Parameter bestimmt der Import die neueste verfügbare Version und verwendet sie für den Abruf der gewählten Endpunkte. Standard sind `items` und aktive `heroes`. Die bisherigen Namen `raw_items` und `raw_heroes` sind dokumentierte Kompatibilitätsnamen für normalisierte Assets. Antwort und Metadaten kennzeichnen die Darstellung; doppelte Endpunkte werden zusammengeführt. Die HTTP-Antworten werden vor dem ersten Snapshot-Schreibvorgang auf Form und Identität geprüft. Das ist keine transaktionale Garantie für einen späteren Datenbankausfall.

Der Reasoner aktiviert `serde_json/float_roundtrip` als reguläres Dependency-Feature. Default im Kandidaten: aktiv, nicht auf Tests beschränkt. Der Regressionstest `frozen_float_parameters_roundtrip_without_ulp_drift` war vor der Änderung rot und besteht nach der Änderung für 4095 Gleitkommawerte. Es wurden keine Qualitätsschwellen oder Referenz-Itemlisten geändert.

Der Polling-Test prüft die bestehende Zeitbegrenzung ohne ein Rennen beim TCP-Serverstart. Der angrenzende echte HTTP-Timeout-Test bleibt erhalten; die 30-Millisekunden-Frist wurde nicht erhöht.

## Auswertung ohne zweite Reasoner-Architektur

`family_evaluation rebase-assets INPUT OUTPUT` lädt Helden, Items und Feldprovenienz über die produktiven Modell-Lader aus einer schreibgeschützten Repeatable-Read-Transaktion. Der Befehl erhält Konfiguration, Spieler-, Autoren- und Patchbelege des eingefrorenen Eingangs. Die Prüfskripte vergleichen diese Felder vor der Auswertung exakt. Die eigentliche Planung, Familienbildung, Holdout-Zuordnung und Replays verwenden die bestehenden Funktionen.

Das Beispiel `scratch_asset_catalog` verwendet die vorhandene Katalog-Klassifikation. Vor Schreibzugriffen prüft es den tatsächlichen Datenbanknamen auf den reservierten Präfix `brain_assets_eval_` und einen numerischen Suffix. Das SQL in diesem Verzeichnis ist eine isolierte Test-Fixture, keine Produktivmigration.

## Nachweise und Grenzen

Nachweiswurzel: `/home/nathanael/.local/share/deadlock-brain/releases/20260921-continuation/`.

Unter `roundtrip/` bestehen Workspace-Tests mit 580 bestandenen Tests, 0 Fehlern und 61 regulär übersprungenen Tests sowie Clippy. Die isolierten Datenbankverträge unter `database-contracts/` sind separat bestanden. Der aktuelle Import und die unveränderten Holdout-Eingaben sind nachgewiesen; die numerischen Berichte werden im abschließenden `REPORT.md` ausgewertet.

Die Live-Prüfung unter `live-read-only/` unterscheidet den installierten CLI-Stand von einem Kandidaten-Smoke des nativen MCP. Erfolgreiche Kandidatenaufrufe sind kein Deploymentnachweis. Der Installationsstand wurde nicht ersetzt.

Die reguläre Review-Schleuse liefert derzeit kein Urteil: Für ihre installierte Kette mit fünf Kritikern verbleiben 347 Sekunden pro Slot bei mindestens 420 benötigten Sekunden. Reviewer, Mindestzeiten und Sperren wurden nicht verändert. Ohne bestandenes Review gibt es keinen Main-Merge und keine produktive Umschaltung.

## Reproduktion

Die Skripte enthalten literale Pfade dieses Worktrees. `run-current-verification.sh` führt Workspace-Prüfung, isolierten Import, Einfrieren, Rebase, Planung, Holdout und drei Byte-Replays aus. Bereits vorhandene Ausgabedateien werden in den Einfrier- und Replay-Schritten nicht still überschrieben; für einen erneuten Gesamtlauf ist eine neue Nachweissenke zu wählen. Zugangsdaten werden über den autorisierten Infisical-Loader in den Prozess geladen und anschließend entfernt. Fehler von Verbindungsaufrufen werden nicht protokolliert.

`verify-live-read-only.sh` fragt das installierte Wissen ab, baut Warden mit `--no-ai --no-persist`, führt einen Backtest mit `--no-persist` aus und prüft den nativen MCP-Kandidaten gegen echte schreibgeschützte Datenbankabfragen. Es startet keine Produktivdienste neu.
