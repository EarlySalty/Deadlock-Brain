# Review durch den Orchestrator

18. September 2026. Der Reviewer ist nicht der Autor der U0-Implementierung. Prüfung von Folgemigration, CI, Test-Helfer und Betriebsbefunden; keine Behauptung einer fachlichen Gameplay-Abnahme.

## Vor Freigabe beheben

### R1: Caption-Integration kann trotz gesetztem DSN still grün werden

`rust/crates/deadlock-brain-yt/src/testutil.rs`, `test_pool`: `.connect(&dsn).await.ok()` macht jeden Verbindungsfehler zu `None`. Der explizit gestartete ignorierte Test kehrt dann erfolgreich zurück. Genau das wurde im ersten lokalen Versuch beobachtet. Der zweite erfolgreiche reale Lauf repariert diesen CI-Fehlermodus nicht.

Bei explizit konfiguriertem Test-DSN muss eine fehlgeschlagene Verbindung den Test fehlschlagen lassen, mit einer geheimnisfreien Fehlermeldung. Fehlender DSN darf für normale optionale Lokaltests weiterhin übersprungen werden; der verpflichtende CI-Integrationsaufruf muss sicherstellen, dass er gesetzt ist. Gegenprobe: bewusst nicht erreichbarer Test-Port ergibt Exit ungleich 0; erreichbarer Scratch-Cluster plus Schema ergibt den echten Erfolg. Kein Fehlertext mit DSN oder Geheimniswerten. Nötige Anpassung am Testhelfer und CI ist freigegeben.

### R2: Neue CLI passt nicht zur vorhandenen Patchidentität

Konkrete Daten und enger Folgeauftrag stehen in `INTEGRATION.md`, Abschnitt 1. 123 Ereignisse liegen unter Quellen-URL, nicht `patch_285`. Kontext, Revisionsprüfung, Sperren und Invalidierung müssen dieselbe belegte Zuordnung verwenden. Ereignisbestände nicht manuell umschreiben.

### R3: Kontextfilter lässt vorhandene Item-/Fähigkeitsdaten aus

`INTEGRATION.md`, Abschnitt 2. Tatsächlichen Snapshot-Typ `item_or_ability` berücksichtigen. Bei Überschreitung Grenzen beibehalten, verständlich und mit realen Größen abbrechen. Kein abgeschnittener Kontext und kein reduzierter Lauf als vollständige Abnahme.

## Dokumentation berichtigen

`REPORT.md` und Kopfkommentar der Folgemigration behaupten teilweise „keine Datenneuschreibung“, obwohl `UPDATE brain.knowledge_events ...` die abgeleiteten Zeitwerte und Metadaten über den Trigger neu schreibt. Präzise formulieren: keine Löschung oder Umschreibung der Quellrevisionen; vorhandene abgeleitete Knowledge-Event-Zeitwerte werden bewusst aktualisiert.

„Kein Produktionscode geändert“ ist ebenfalls zu weit: SQL-Views und Triggerfunktion sind Produktionslogik. Richtig ist „kein Rust-Laufzeitcode für die beiden ursprünglichen Korrekturen geändert; produktive SQL-Views und Triggerfunktion geändert“.

## Weiterhin offen, nicht wegprüfen

Aktueller offizieller Steam-Text weicht vom gespeicherten Original ab; Quelle vor aktuellem Primärdatenlauf über vorhandenen revisionssicheren Importweg abgleichen. Vollständiger Mechanikkontext überschreitet die bestehenden Limits deutlich. Diese Punkte dürfen nicht durch größere unkontrollierte Modellläufe oder erfundene Vollständigkeit übergangen werden. Keine Produktionsmigration, kein Merge und kein Deploy durch den Worker; bisherige Freigabegrenzen bleiben.
