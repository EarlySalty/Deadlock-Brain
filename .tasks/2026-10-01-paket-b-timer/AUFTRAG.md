status: aktiv
Datum: 2026-10-01

# Auftrag: Build-Data-API-Retries begrenzen

Die Build-Data-API soll nur transiente HTTP- und Transportfehler begrenzt mit Backoff wiederholen. Dauerhafte HTTP-Fehler und fehlerhafte JSON-Antworten werden sofort zurückgegeben. Die bestehende tägliche Timerplanung bleibt unverändert. Keine Zusatzfeatures.

## Umfang

- `rust/crates/dbrain-builds/src/api.rs`: Fehlerklassifikation, begrenzte Retry-Grenze und Regressionstests.
- `service/systemd/deadlock-brain-build-data.service`: keine zusätzliche Wiederholung eines vollständigen Syncs außerhalb der begrenzten API-Aufrufe.
- Den unverbundenen YouTube-Learning-Service aus dem Ausgangscommit entfernen.
- Keine Secrets, ENV-Werte, Kommentare, Live-Config, DDL, Builds, Deploys, Restarts oder Main-Merges.

## Fertig-Kriterium

Review der Fehler- und Timersemantik am Feature-Head, `git diff --check` ohne Befund und Merge-Gate-Review, sobald dessen Prüfung ohne schwere Builds möglich ist. Cargo-Prüfungen bleiben bis zur Host-Ressourcenfreigabe ausgesetzt.
