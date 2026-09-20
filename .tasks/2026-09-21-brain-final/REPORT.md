# Integrations- und Freigabestand

Stand: 21.09.2026, Europe/Berlin. **Integration geprüft, noch kein Main-Merge und noch kein Deployment dieser Sitzung. Fachliche Abschlussmessung ausstehend.**

## Sicherer Integrationsstand

Eigener Worktree: `/home/nathanael/.worktrees/brain-final-integration-20260921`, Branch `feat/brain-final-integration-20260921`. Der geteilte Checkout enthält fremde ungesicherte Änderungen und wurde weder umgeschaltet noch gestasht, zurückgesetzt oder bereinigt.

Integrierte Ausgangspunkte: Rust-Cutover `285a35c`, aktuelles origin/main `9b76ecd`, Reasoner-Mechaniken und Familien `7966f85`, internes lesendes Labor `d8c3427`. Der Cargo-Konflikt behält sowohl die URL-Abhängigkeit als auch die Wiki-Testabhängigkeit. Der Retrieval-Konflikt behält die native Postgres-Suche nach Mechaniknotizen anstelle des alten leeren Vektor-Kompatibilitätspfads. Die neuen Wiki- und Provenienzpfade aus main bleiben vorhanden.

Das Labor benutzt den vorhandenen Planner, ist nicht öffentlich angebunden, schaltet keine KI ein und veröffentlicht keine Builds. Der Familien-Publisher behält seine bestehenden Sperren. Keine Schwellen, Referenzlisten oder populationsabhängigen Qualitätswerte wurden zur Freigabe geändert.

Zusätzliche Integrationskorrekturen: Ein vom autorisierten Infisical-Loader übergebenes Datenbanksecret ist auch im lesenden Pool verwendbar. Fehlt es, bleibt der native Credential-Transport erhalten. Ask-context und persistenzfreie Reasoner-Befehle verlangen bereits beim Verbindungsaufbau Read-only. Der späteste Startparameter setzt Read-only auch gegen einen widersprechenden DSN-Parameter. Destruktive Reasoner-Fixtures akzeptieren reservierte, eindeutig benannte Testdatenbanken; produktive Namen werden weiterhin abgewiesen.

## Neu ausgeführte technische Prüfungen

Die vorgefundenen Ausgaben enthalten **561 bestandene Tests, 0 Fehler und 61 ignorierte Tests** in 26 Testprogrammen. Die zuvor dokumentierte Zahl 569 war nicht durch ihre Summe gedeckt. Diese Dateien stammen von 00:50 Uhr und gelten nicht als neuer Prüflauf dieser Fortsetzung. Ein frischer Workspace-Lauf mit anschließendem Clippy läuft seit 01:41 Uhr als transienter User-Dienst; sein Ergebnis wird erst nach geprüftem Abschluss übernommen. Cargo verwendet höchstens zwei Jobs und ein eigenes Buildverzeichnis. Vorbestehende Warnungen sind kein Beleg für Warnungsfreiheit.

SQL-Migrationsmatrix über den autorisierten Infisical-Zugang in ausschließlich neu angelegten Testdatenbanken: ursprüngliche Assertions und Idempotenz bestanden; vier neue Assertions scheiterten erwartungsgemäß vor ihrer jeweiligen Korrektur; anschließend bestanden alle fünf vollständigen Assertionsätze sowie die Idempotenz beider Folgemigrationen. Zusätzlich bestanden fünf YouTube-Vertragstests und ein Caption-Persistenztest, die im normalen Workspace-Lauf ignoriert sind. Das sind sechs zusätzliche ausgeführte Rust-Tests, keine Produktionswrites.

Isolierte Reasoner-Vertragstests sind separat gestartet und werden erst nach einem tatsächlichen Exit-0-Ergebnis gezählt. Die CI prüft nun den gesamten Workspace, läuft auch auf main und enthält einen eigenen Postgres-Job für diese Reasoner-Verträge. Drei historisch datengebundene Warden-Prüfungen werden dort nicht gegen synthetische Fixtures als Echtmessung ausgegeben.

Artefakte und vollständige Testausgaben liegen außerhalb des Repositorys unter `/home/nathanael/.local/share/deadlock-brain/releases/20260921-final/`. Roh-Spielerdaten und Datenbankzugänge werden nicht eingecheckt. Die beiden SQL-Testskripte in dieser Task-Akte dokumentieren die genauen Aufrufe; gleichnamige vorhandene Testdatenbanken werden nicht überschrieben.

## Produktiver Ausgangszustand und Datenlücke

Der Wiki-Timer ist aktiv. Der tatsächlich aufgelöste Snapshot enthielt 1.784 Einträge, darunter 39 Helden, 295 Fähigkeiten und 175 Items. Renderdatum und Quellenrevision bleiben getrennt. Der jüngste geprüfte Deadlock-Data-Quellimport stammt vom 20.09.2026, dessen Quellenrevision vom 18.09.2026.

Der Reasoner liest seine Helden- und wesentlichen Itemmodelle dagegen aus der Assets-Quelle. Deren jüngster Datenbankzeitstempel war am 21.09.2026 noch **16.09.2026, 20:18:59 UTC**. Das neuere Wiki beweist deshalb keinen aktuellen Reasoner-Datenstand. Neue Replays und Holdouts müssen diese Lücke sichtbar machen beziehungsweise nach einem echten Quellenabzug neu gemessen werden. Alte Familienwerte werden nicht als Ergebnisse dieser Sitzung gezählt.

Beim Betriebsinventar waren Build-Data, Sheet-Sync und YouTube-Learning fehlgeschlagen. Patchnotes-Sync und Wiki hatten erfolgreiche letzte Läufe. Die separate Korpus-Webseite läuft als Python-Prozess aus deadlock-build-corpus und nutzt weder das entfernte Python-Paket noch eine Python-Brücke des Brain. Sie ist kein Nachweis des Brain-Rust-Deployments und wird nicht still als Rust ausgegeben.

## Sicherheitsvorfall bei der Verbindungsdiagnose

Ein falsch verwendeter libpq-Aufruf behandelte den Verbindungsstring als Datenbanknamen und gab dadurch einen geheimnishaltigen Ausschnitt in einer Fehlermeldung aus. Der Wert wird hier nicht wiederholt. Der korrigierte Aufruf verwendet den vorgesehenen Verbindungsparameter; Verbindungsfehler werden vor der Ausgabe unterdrückt und durch feste Meldungen ersetzt. Der betroffene Datenbankzugang muss über den normalen Betreiberprozess rotiert werden. Keine unkoordinierte Rotation aller Dienste wurde vorgenommen.

## Noch nicht behauptete Abschlüsse

Reguläre Merge-Freigabe, Remote-CI des neuen Gesamtstands, Main-Merge und Push, Release-Installation, Migrationen auf Produktion, Dienstumstellung, funktionale Live-Nachweise, frische Reasoner-Messungen sowie Bereinigung sind noch nicht abgeschlossen. Diese Felder werden nur anhand tatsächlich ausgeführter Schritte ergänzt.
