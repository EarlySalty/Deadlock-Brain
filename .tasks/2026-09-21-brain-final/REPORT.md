# Integrations- und Freigabestand

Stand: 21.09.2026, Europe/Berlin. **Integration geprüft, noch kein Main-Merge und noch kein Deployment dieser Sitzung. Fachliche Abschlussmessung ausstehend.**

## Sicherer Integrationsstand

Eigener Worktree: `/home/nathanael/.worktrees/brain-final-integration-20260921`, Branch `feat/brain-final-integration-20260921`. Der geteilte Checkout enthält fremde ungesicherte Änderungen und wurde weder umgeschaltet noch gestasht, zurückgesetzt oder bereinigt.

Integrierte Ausgangspunkte: Rust-Cutover `285a35c`, aktuelles origin/main `9b76ecd`, Reasoner-Mechaniken und Familien `7966f85`, internes lesendes Labor `d8c3427`. Der Cargo-Konflikt behält sowohl die URL-Abhängigkeit als auch die Wiki-Testabhängigkeit. Der Retrieval-Konflikt behält die native Postgres-Suche nach Mechaniknotizen anstelle des alten leeren Vektor-Kompatibilitätspfads. Die neuen Wiki- und Provenienzpfade aus main bleiben vorhanden.

Das Labor benutzt den vorhandenen Planner, ist nicht öffentlich angebunden, schaltet keine KI ein und veröffentlicht keine Builds. Der Familien-Publisher behält seine bestehenden Sperren. Keine Schwellen, Referenzlisten oder populationsabhängigen Qualitätswerte wurden zur Freigabe geändert.

Zusätzliche Integrationskorrekturen: Ein vom autorisierten Infisical-Loader übergebenes Datenbanksecret ist auch im lesenden Pool verwendbar. Fehlt es, bleibt der native Credential-Transport erhalten. Ask-context und persistenzfreie Reasoner-Befehle verlangen bereits beim Verbindungsaufbau Read-only. Der späteste Startparameter setzt Read-only auch gegen einen widersprechenden DSN-Parameter. Destruktive Reasoner-Fixtures akzeptieren reservierte, eindeutig benannte Testdatenbanken; produktive Namen werden weiterhin abgewiesen.

## Neu ausgeführte technische Prüfungen

Der frische Workspace-Lauf vor der Runtime-Korrektur bestand mit **569 Tests, 0 Fehlern und 61 ignorierten Tests**. Die zuvor vorgefundenen 561 Tests waren ein älterer Lauf und wurden nicht als neue Abnahme verwendet. Nach der Runtime-Korrektur bestanden **572 Tests, 0 Fehler, 61 ignoriert**. Alle drei neuen Credential-Regressionstests liefen dabei tatsächlich. Cargo verwendete höchstens zwei Jobs und ein eigenes Buildverzeichnis. Clippy endete nach dieser Korrektur ebenfalls mit Exit 0; vorhandene Warnungen bedeuten keine Warnungsfreiheit.

SQL-Migrationsmatrix über den autorisierten Infisical-Zugang in neu angelegten Testdatenbanken: ursprüngliche Assertions und Idempotenz bestanden; vier Gegenproben scheiterten erwartungsgemäß vor ihrer jeweiligen Korrektur; anschließend bestanden fünf vollständige Assertionsätze sowie die Idempotenz beider Folgemigrationen. Zusätzlich bestanden fünf YouTube-Vertragstests, ein Caption-Persistenztest und **13 isolierte Reasoner-Vertragstests**. Das sind 19 zusätzlich ausgeführte Rust-Tests, keine Produktionswrites. Der anfängliche Fehler in der Verbindungsübergabe des lokalen Testskripts wurde korrigiert; der danach ausgeführte reguläre Testlauf war erfolgreich.

Für `61b04be` bestand GitHub-Lauf **35545490549** mit fünf erfolgreichen Jobs: Rust, Postgres, Caption Integration, YouTube Contract und Reasoner Postgres. Diese Abnahme wird nicht auf spätere Commits übertragen. Drei historisch datengebundene Warden-Tests werden nicht gegen künstliche Fixtures als aktuelle Echtmessung ausgegeben.

Artefakte und vollständige Testausgaben liegen außerhalb des Repositorys unter `/home/nathanael/.local/share/deadlock-brain/releases/20260921-final/`. Roh-Spielerdaten und Datenbankzugänge werden nicht eingecheckt. Die beiden SQL-Testskripte in dieser Task-Akte dokumentieren die genauen Aufrufe; gleichnamige vorhandene Testdatenbanken werden nicht überschrieben.

## Produktiver Ausgangszustand und Datenlücke

Der Wiki-Timer ist aktiv. Der tatsächlich aufgelöste Snapshot enthielt 1.784 Einträge, darunter 39 Helden, 295 Fähigkeiten und 175 Items. Renderdatum und Quellenrevision bleiben getrennt. Der jüngste geprüfte Deadlock-Data-Quellimport stammt vom 20.09.2026, dessen Quellenrevision vom 18.09.2026.

Der Reasoner liest seine Helden- und wesentlichen Itemmodelle dagegen aus der Assets-Quelle. Deren jüngster Datenbankzeitstempel war am 21.09.2026 noch **16.09.2026, 20:18:59 UTC**. Das neuere Wiki beweist deshalb keinen aktuellen Reasoner-Datenstand. Neue Replays und Holdouts müssen diese Lücke sichtbar machen beziehungsweise nach einem echten Quellenabzug neu gemessen werden. Alte Familienwerte werden nicht als Ergebnisse dieser Sitzung gezählt.

Beim Betriebsinventar waren Build-Data, Sheet-Sync und YouTube-Learning fehlgeschlagen. Patchnotes-Sync und Wiki hatten erfolgreiche letzte Läufe. Die separate Korpus-Webseite läuft als Python-Prozess aus deadlock-build-corpus und nutzt weder das entfernte Python-Paket noch eine Python-Brücke des Brain. Sie ist kein Nachweis des Brain-Rust-Deployments und wird nicht still als Rust ausgegeben.

## Native Runtime-Prüfung und Messung vor dem Quellenrefresh

Ein echter Start mit Systemd-Credential scheiterte zunächst, weil eine alte konfigurierte FD-Nummer inzwischen einem anderen offenen Deskriptor gehörte. Die native Auswahl bevorzugt jetzt das namentlich gebundene Systemd-Credential. Ein vorhandener alter FD bleibt gegen Vererbung beim Exec geschützt. Regressionen prüfen die Auswahl vor einem offenen Socket, den Schutz eines verdrängten FDs und den weiterhin unterstützten expliziten FD-Pfad.

Der wiederholte native Start über `deadlock-brain-secret-exec` und `deadlock-brain ask-context` endete mit Exit 0. Das Ergebnis ist gültiges JSON mit 161.508 Bytes und enthält das gebundene Heldenwissen. Dieser Vorabtest verwendet Debug-Binaries und ersetzt keinen späteren Release-Nachweis. Die produktiven Dienste wurden dafür nicht umgeschaltet.

Acht Helden wurden in einer einzigen datenbankseitig lesenden Repeatable-read-Transaktion eingefroren. Der Bestand umfasst 41.279 Spieler-Beobachtungen und 2.245 Autoren-Beobachtungen. Der jüngste Match-Zeitpunkt ist **18.09.2026, 03:10:22 Uhr, Europe/Berlin**. Die Ergebnisse sind deshalb ausdrücklich die Messung vor dem Quellenrefresh.

| Held | Training Tau | Training Jaccard@12 | Holdout Tau | Holdout Jaccard@12 | Holdout Staples | Match-Stichprobe |
|---|---:|---:|---:|---:|---:|---:|
| Warden | 0,867 | 0,833 | 0,822 | 0,833 | 10/14 | 421 |
| Infernus | 0,462 | 1,000 | 0,443 | 1,000 | 12/14 | 507 |
| Vindicta | 0,137 | 1,000 | 0,123 | 0,846 | 11/12 | 266 |
| Lady Geist | 0,104 | 0,846 | 0,132 | 0,917 | 11/13 | 353 |
| Abrams | -0,067 | 0,500 | 0,098 | 0,583 | 7/10 | 244 |
| Viscous | 0,315 | 0,833 | 0,322 | 0,833 | 10/11 | 107 |
| Shiv | 0,636 | 1,000 | 0,561 | 0,917 | 11/12 | 203 |
| Haze | 0,571 | 0,667 | 0,571 | 0,667 | 8/11 | 79 |

Jeweils der vom Planner gewählte Primärbuild, nicht die nachträglich beste Variante. Training und Holdout entdecken ihre Familien aus den jeweils zulässigen Trainingsdaten. Die Holdouts sind nach Spielern und Autoren getrennt; ihre Zuordnung benutzt eingefrorene Trainingszentren. Acht primäre Staple-Gates sind rot; Haze liegt zusätzlich unter der bestehenden Mindeststichprobe von 100 Matches. Die acht Primärbuilds melden niedrige Konfidenz. Daraus folgt keine fachliche Veröffentlichungserlaubnis.

## Sicherheitsvorfall bei der Verbindungsdiagnose

Ein falsch verwendeter libpq-Aufruf behandelte den Verbindungsstring als Datenbanknamen und gab dadurch einen geheimnishaltigen Ausschnitt in einer Fehlermeldung aus. Der Wert wird hier nicht wiederholt. Der korrigierte Aufruf verwendet den vorgesehenen Verbindungsparameter; Verbindungsfehler werden vor der Ausgabe unterdrückt und durch feste Meldungen ersetzt. Der betroffene Datenbankzugang muss über den normalen Betreiberprozess rotiert werden. Keine unkoordinierte Rotation aller Dienste wurde vorgenommen.

## Noch nicht behauptete Abschlüsse

Reguläre Merge-Freigabe, Remote-CI nach der letzten Runtime-Korrektur, Main-Merge und Push, Release-Installation, Migrationen auf Produktion, Dienstumstellung, funktionale Release-Nachweise, Quellenrefresh, abschließende Replays und Messungen sowie Bereinigung sind noch nicht abgeschlossen. Diese Felder werden nur anhand tatsächlich ausgeführter Schritte ergänzt.
