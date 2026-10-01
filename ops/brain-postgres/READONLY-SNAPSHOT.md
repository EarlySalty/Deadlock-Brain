# Lesender Archiv-Snapshot vor der Datenübernahme

Der bestehende Rust-Importer unterstützt neben dem unveränderten Importaufruf einen ausdrücklich lesenden Modus:

```text
brain-legacy-import --observe-snapshot --config /absoluter/pfad/legacy-core-cutover.json
```

Der Modus verwendet die normale JSON-Konfiguration und den bestehenden Secret-Exec-Weg. Zugangsdaten werden nicht in der JSON-Datei hinterlegt. Für die dedizierte Instanz gelten die bereits vorhandenen Referenzen `BRAIN_PG_READONLY_PASSWORD` und `BRAIN_PG_INGEST_PASSWORD`. Die früheren Beispielnamen `BRAIN_LEGACY_READ_AUTH` und `BRAIN_TARGET_INGEST_AUTH` entsprechen nicht dem vorhandenen Runtime-Bestand und werden nicht als zusätzliche Secrets angelegt.

## Wirkung

Die Beobachtung verbindet sich ausschließlich als `brain_readonly` über `/run/deadlock-brain-postgresql`, Port `5446`, mit der Datenbank `brain`. Sie prüft die tatsächliche Datenbank-, Rollen- und Schemaidentität und verwendet den vorhandenen Reader mit `REPEATABLE READ READ ONLY` und abschließendem `ROLLBACK`.

Es wird keine Zielverbindung geöffnet, kein Ingest-Passwort angefordert und weder ein Quellcheckpoint noch ein Release geschrieben. Das in der Importkonfiguration angegebene Berichtsverzeichnis wird im Beobachtungsmodus nicht beschrieben. Die Ausgabe geht ausschließlich an stdout. Sie ist als interne Metadaten zu behandeln und mit Modus `0600` außerhalb von Git aufzubewahren.

Die Ausgabe enthält einen neuen Beobachtungszeitpunkt mit Snapshotlabel, den kanonischen Snapshot- und Schema-Fingerabdruck, die tatsächlichen Tabellen- und Dokumentzahlen, die Schema-OIDs und die beobachteten logischen Dokument-IDs. Rohtexte und Zugangsdaten werden nicht ausgegeben. Historische Labels aus der Konfigurationsvorlage werden nicht als neue Beobachtung übernommen.

## Credential-Bindung

Der Secret-Exec-Prozess erhält das vorhandene Infisical-Credential über systemd `LoadCredential` oder einen ausdrücklich eingerichteten regulären Credential-Dateideskriptor. Ein zufällig geerbter Deskriptor ist kein gültiger Ersatz. Die vorhandene `config/infisical.json` wird ausdrücklich per `--config` an Secret-Exec übergeben.

Die reine Beobachtung funktioniert auch mit `production_binding=null` und ohne Ingest-Credential. Der schreibende Import bleibt dabei gesperrt. Der integrierte PostgreSQL-Test prüft beide Eigenschaften sowie unveränderte Zieltabellen.

## Keine automatische Nutzungsfreigabe

Beobachtete IDs sind keine bestätigten `active_ids`. Der Modus erzeugt deshalb weder eine `approval_ref` noch einen `policy_sha256` oder erfundene Listen widerrufener und gelöschter Dokumente. Ein erfolgreich gelesener Snapshot erlaubt keine Veröffentlichung und keinen Provider-Transfer. Diese Grenzen gelten auch dann, wenn die interne Archivübernahme bereits beauftragt ist.

Die vollständige produktive Importbindung muss weiterhin den tatsächlich beobachteten Snapshot, die belegten Quellenrechte und die vollständigen Zustandslisten zusammenführen. Ein erfolgreicher lesender Lauf ist kein Produktionsimport und kein Nachweis eines öffentlich antwortenden Consumers.
