# A-V1b: kombinierte Debugprüfung abgeschlossen

Geprüfter Commit 1e925d5fd6852e855a09cfdc47051f979491c6ff, darin Main fde910f6. Nach reiner Attributions-/Commitnachrichtenänderung Main 8d61a949c9856a69543747b0e59dbfb5bbcbe440, identischer Gitbaum 500040ac54a6e5d2e1a60cbe7ea6f52bf1ad9ff6. Quelle 1325 Dateien, null anfängliche/finale Abweichungen, genau drei neue Main-Dateien übernommen. Toolchain 1.97.1.

## Tatsächliche Ergebnisse

Format Exit 0. Compiler Exit 0, 5 Minuten 59 Sekunden. Striktes Clippy mit -D warnings Exit 0, 14 Minuten. Tests Exit 0: 126 passed, 0 failed, 0 ignored, 0 filtered. Pakete deadlock-brain und dbrain-enrich, Flags --locked --offline -j 1 -- --include-ignored --test-threads=1. Wörtliche Befehle und damals verwendete private Harnesskonfiguration in /tmp/brain-a-integration-proof-20261007/main-fde-commands.json.

Aufteilung: dbrain-enrich 7, brain-mcp 21, deadlock-brain 91, answer_config 3, runtime_tooling 4. Nulltest-Binary und Doctests getrennt im Ergebnismanifest; keine übersprungenen DB-Prüfungen als reale Ausführung gezählt.

Private frische PG, TCP aus: 39 Tabellen/null Zeilen vor Suite, reale 26 Inserts/10 Updates/26 Deletes danach. Eigener Stop Exit 0, pg_ctl status danach Exit 3/no server running, kein PIDfile. Kein Prod-/Provider-/Infisicalzugriff oder Sourcefix durch den Prüfer. Anfängliche konstante SQL-Identitätsprobe korrigiert und erfolgreich wiederholt vor DB-Reset, kein Suitefehler.

Belege /tmp/brain-a-integration-proof-20261007/main-fde-results.json, main-fde-source-final.json, main-fde-check.log, main-fde-clippy.log, main-fde-tests.log, main-fde-db-statistics-after.log und main-fde-postgres-status-final.log.

## Grenzen nach neuer Orchestratorrückgabe

Die reale Testausführung ist belegt, der neue ENV-Vertragsbefund an runtime_tooling bleibt separat. Ein grüner damaliger Lauf legitimiert keine verbotene Konfigaufnahme. A-R2 korrigiert diesen Befund nur auf Feature. 8d-Main wurde regulär gepusht, jedoch nicht durch A fertig als Release gebaut, installiert oder live abgenommen. Gemeinsames Releasefenster und freigegebene fde910f6-Recovery haben Vorrang, siehe RELEASEFENSTER.md.

TESTNACHWEIS[TW-1]: 126 passed, 0 ignored | Baseline: keine Baselinebehauptung rot
