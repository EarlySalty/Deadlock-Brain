# Steckbriefpflege fortsetzen

`brain-maintain resume-entity-profiles` setzt die Steckbriefpflege auf bereits
veröffentlichten Originalquellen fort. Der Befehl läuft mit dem vorhandenen
Operatorzugang und benötigt diese drei Angaben:

```text
brain-maintain --config /etc/deadlock-brain/maintenance-runtime.json resume-entity-profiles \
  --original-release-id <Originalrelease> \
  --original-release-sha256 <SHA256 der regulären CorpusRelease-Serialisierung> \
  --expected-serve-config-sha256 <SHA256 der aktuellen Serve-Konfigdatei>
```

Die Pflege prüft den gespeicherten Originalrelease und dessen aktuelle Quellpins.
Sie rekonstruiert die vollständige Faktenbindung aus den Originaldokumenten und
dem aktuellen Katalog und vergleicht Fakten, Identitäten und Semantik mit den
gespeicherten Bindungen. Fehlende oder widersprüchliche Bindungen führen zum
Abbruch. Der Befehl ergänzt dabei keine Bindungen.

Danach werden alle gebundenen Entitäten erneut mit frischen Originalrechten,
Blobbelegen, Patchhistorie und privaten Quittungen abgeleitet. Die gemeinsame
Veröffentlichung und Aktivierung laufen durch dieselben Prüfungen wie bei der
regulären Pflege. Die Aktivierung prüft erneut den erwarteten Konfigurationshash.
Bereits gespeicherte Steckbriefköpfe ersetzen keinen dieser Nachweise.

Die additive Migration
`scripts/migrations/2026-10-07-brain-patch-story-inline-v1.sql` gehört vor der
Fortsetzung in den vorhandenen Owner-Migrationsweg. Sie erhält die vollständige
Patchdatumszuordnung und Deduplizierung und vermeidet die wiederholte globale
Textaufbereitung bei jeder Entitätsabfrage. Die Zeitbudgets bleiben unverändert.

Ein fehlgeschlagener Tick gibt seinen sicheren JSON-Status aus und endet mit
einem Fehlercode. Historische fehlgeschlagene Jobs machen einen späteren
erfolgreichen Tick nicht nachträglich zum Fehler.
