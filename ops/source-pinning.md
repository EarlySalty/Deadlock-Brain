# Explizite Quellenpins (C7)

Diese Dateien sind Konfigurations- und Unit-Beispiele, keine Installation oder Freigabe. Keine Credentials gehören in Quellenpins. Der bestehende Infisical-/Secret-Exec-Pfad bleibt bestehen.

## Autorität und Änderungen

`config/source-pins.json` enthält den versionierten Config-Vertrag. `wiki refresh --config` verwendet dieselbe Struktur unter `source_pins` in `config/wiki-refresh.json`, ausschließlich aus dieser Datei. Umgebungsvariablen verändern diesen Wiki-Pin nicht.

Der eingetragene Commit `66d0832dc30148ca66984f3f4c4a4d340af6f7d0` ist die vollständige Auflösung des bereits im Integrationsreview genannten `66d0832d`, **keine Auswahl von HEAD/latest**. `ClientVersion=6698` wurde aus `data/version.txt` genau dieses Commitobjekts gelesen. Die Beispiele halten somit den bereits dokumentierten Quellenstand fest. Ein Betreiber muss ihn vor einem späteren Deploy ausdrücklich prüfen; der neue Code lädt keine fehlenden Objekte aus dem Netz nach.

Pflichtfelder: `schema_version: 1` der Config, `deadlock_data.commit` (40/64 Zeichen, lowercase hex), `deadlock_data.parser_revision` (`dbrain-deadlock-data/2`). Optional: `deadlock_data.schema_version` als erwartete SourceIr-Version (derzeit 1, keine erfundene Upstream-Schemaversion) und `deadlock_data.data_version` als exakte `ClientVersion`.

Falsche/fehlende Felder, unbekannte Parser, ein nicht lokal vorhandenes Commitobjekt, ein Blob/Tag statt eines Commits, eine falsche Repository-Origin oder eine abweichende Data-Version sind Jobfehler. Es gibt weder HEAD-Auflösung noch Fetch, Checkout, Pull, Filter oder Hooks im Import. Der Worktree und dessen Branch sind nicht Datenautorität.

Ein Update erfolgt nur nach ausdrücklicher Freigabe und Änderung dieser Config. Ein gegebenenfalls notwendiger Abruf des **freigegebenen exakten** Commitobjekts ist ein separater Betreiber-Schritt. Ein Parserupdate benötigt ebenfalls eine ausdrücklich angepasste Parserrevision; der Wert wird gegen die implementierte Revision geprüft, nicht nur als beliebiges Etikett gespeichert.

## Read-only Preflight

```sh
deadlock-brain wiki refresh --config /absolute/wiki-refresh.json --check-config
deadlock-brain pull deadlock-data --source-config /absolute/source-pins.json \
  --repo-dir /absolute/deadlock-data --check-config
```

Beide prüfen lokale Gitobjekte und die Config vor Poolaufbau, HTTP-Import und Schreibzugriffen. Der neue Preflight ruft keinen Credential-Provider auf; beim Pull werden weiterhin die bisherigen lokalen Settings geladen. Erfolgsantwort: `valid:true`, `writes:false`. Ein normaler Job führt dieselbe Prüfung aus und reicht dieselben geladenen Optionen an den Import weiter. Beim Pull wird danach auch die vorhandene Patchnote-Normalisierung unverändert ausgeführt.

`wiki refresh --skip-source-update` bleibt ein ausdrücklicher **Export des vorhandenen Datenbankstands**, kein gepinnter Reimport und keine Reproduzierbarkeitszusage für den DB-Inhalt. Die Config bleibt syntaktisch verpflichtend. Der lokale Quellcache muss für diesen reinen Export nicht verfügbar sein. `--check-config` und `--skip-source-update` sind deshalb nicht kombinierbar.

## CLI-/Environment-Kompatibilität

Für bestehende Secret-Exec-Jobs, die `pull deadlock-data` aufrufen, ist eine der folgenden Varianten möglich:

```ini
# In der bestehenden Unit; nur Vorlage, nicht automatisch installieren:
Environment="DBRAIN_SOURCE_PINS_CONFIG=/absolute/source-pins.json"
```

Alternativ, **nicht gleichzeitig mit einer Pin-Config**:

```ini
Environment="DBRAIN_DEADLOCK_DATA_COMMIT=66d0832dc30148ca66984f3f4c4a4d340af6f7d0"
Environment="DBRAIN_DEADLOCK_DATA_PARSER_REVISION=dbrain-deadlock-data/2"
```

Auch `--commit FULL_SHA --parser-revision dbrain-deadlock-data/2` bleibt möglich. Bei CLI und Environment ohne Config haben die expliziten CLI-Felder Vorrang. Bei `--source-config` beziehungsweise `DBRAIN_SOURCE_PINS_CONFIG` werden zusätzliche CLI-/Environment-Pinfelder als Konfigurationskonflikt abgewiesen, einschließlich leerer gesetzter Environment-Overrides. `--no-git-update` bleibt kompatibel; implizite Gitupdates sind grundsätzlich aus.

Die Wiki-Unit-Vorlage enthält einen read-only `ExecStartPre` für ihre JSON-Config. Beim später ausdrücklich beauftragten Deploy müssen Binary und dazu passende Config gemeinsam bereitgestellt werden. Keine Unit wurde durch C7 installiert, aktiviert oder gestartet.

## Optionaler Wiki-Korpus

Der bisher optionale Korpus bleibt deaktiviert, solange `wiki.enabled` nicht gesetzt ist. Bei Aktivierung ist `wiki.source_pin` verpflichtend. Dieser separate Legacy-Korpuspfad ist nicht der S12-Capture-/Discovery-Umbau aus C5.

Ein Pin enthält `parser_revision: "dbrain-wiki-corpus/1"`, optional `schema_version: 1`, explizite `license_text`/`license_url` und eine endliche `pages`-Liste. Jede Seite benötigt `page_id`, `revision_id`, `title`, `revision_timestamp`, `page_touched` und `response_sha256`. Der letzte Wert ist `external::normalized_hash` der vollständigen freigegebenen MediaWiki-Parse-Antwort: SHA-256 der deterministischen JSON-Repräsentation, **nicht** der HTTP-Bytes und kein RFC-8785-Versprechen.

Im Job gibt es keine Latest-/Allpages-Discovery mehr: nur `oldid` aus dem Pin. Der Responsehash wird vor Persistierung geprüft. Das verhindert, dass geänderte transkludierte Vorlagen bei gleicher Artikelrevision unbemerkt akzeptiert werden. Die Abhängigkeiten werden damit nicht als eigenständig revisionsgepinnte Vorlagen ausgegeben (`rendered_templates_pinned:false` bleibt ehrlich); `rendered_content_hash_pinned:true` bezeichnet die Hashprüfung. Kann MediaWiki den freigegebenen Render nicht mehr liefern, scheitert der Job sichtbar. Es wird keine neuere Revision oder ein neuer Hash automatisch ausgewählt. Gleiche akzeptierte Antworten ergeben gleiche normalisierte Inhalte; Beobachtungs-/Laufzeitstempel sind keine Inhaltsidentität.

Lizenzangaben, Revisionen und Hashes dürfen nicht aus diesen Erläuterungen erfunden werden. Die ausgefüllte Config stammt aus einem separat geprüften Capture. Bestehende Budgets, Mindestabstand, API-Fehlerbehandlung und atomare Wiki-Veröffentlichung bleiben wirksam.
