# Audit B: empirischer Bestand nach der Abnahmeergänzung

Stand: 07.10.2026. Diese Prüfung ersetzt die frühere Zugangslücke in `b/YT-KORPUSBEISPIELE.md` und im ersten YT-Bericht. Der Hauptorchestrator hat den bestehenden lokalen Peer-Zugang benannt; B hat die folgenden Kernbefunde selbst lesend bestätigt. Kein Import und keine Produktänderung.

## Leseweg und Transaktionsbeweis

Zwei bestehende lokale Verbindungen wurden mit `psql -X -w`, `ON_ERROR_STOP=1` und ausschließlich `BEGIN READ ONLY; ... ROLLBACK;` verwendet:

```text
psql -X -w -h /run/deadlock-brain-postgresql -p 5446 -U brain_migrate -d brain
psql -X -w -d deadlock
```

Jede Abfrage prüfte zu Beginn `current_user`, `current_database()` und `current_setting('transaction_read_only')`. Die dedizierte Instanz meldete `brain_migrate`, `brain`, `on`; die zentrale Instanz meldete `nathanael`, `deadlock`, `on`. Alle vier verwendeten SQL-Transaktionen endeten mit `ROLLBACK`.

Das belegt lesende Transaktionen, keine ausschließlich lesenden Rollenrechte. Rollenrechte wurden nicht verändert oder vorausgesetzt. Keine Credentials, DSNs, Secrets oder Umgebungswerte gelesen; keine Konfigurationsänderung. Der alte Import-/Claims-CLI wurde nicht aufgerufen.

## Aktuelle Tabellen

| Verbindung | Tabelle beziehungsweise Existenzprüfung | Ergebnis von B |
| --- | --- | --- |
| Dediziert, DB `brain` | `to_regclass('brain.youtube_learning_claims')` | null; Tabelle unter diesem Namen nicht vorhanden |
| Dediziert, DB `brain` | `brain_legacy.youtube_learning_claims` | 0 Zeilen |
| Dediziert, DB `brain` | `brain_legacy.youtube_videos` | 195 Zeilen |
| Zentral, DB `deadlock` | `brain.youtube_learning_claims` | 0 Zeilen |
| Zentral, DB `deadlock` | `brain.youtube_videos` | 195 Zeilen |
| Zentral, DB `deadlock` | `brain.youtube_transcript_claim_attempts` | 0 Zeilen |

Diese Ergebnisse gelten für die genannten Tabellen zum Prüfzeitpunkt. Die beiden Datenbanken nicht zu einem Bestand von 390 Videos addieren. Eine Datengleichheit jeder Videozeile wurde nicht untersucht.

Zusätzliche Messung des Hauptorchestrators, von B nicht erneut abgefragt: Videostatus 176 queued/missing, 17 queued/ready, 1 failed/ready, 1 queued/unavailable; 549 `youtube_video`-Snapshots in `brain.entity_snapshots`, keine Claimtyp-Treffer in dessen gezielter Abfrage. Das belegt Video-/Quellbestand, keine klassifizierten Aussagen.

## `insight_records`: strukturierte Herkunft ist Patchwissen

B las zunächst nur Schema, Quellen-/Typaggregate und strukturierte Herkunft, keine ungefilterten `summary`-, `reason`- oder `subject`-Texte.

- 51 Zeilen, alle mit `source_urls`. 38 haben nicht leere `source_patch_event_ids`; 13 haben keine Event-IDs.
- Quellenhosts in `source_urls`: `forums.playdeadlock.com` 38, `store.steampowered.com` 13 und `steamstore-a.akamaihd.net` 13. Das sind URL-Vorkommen, keine Summe zusätzlicher Insights.
- `source_references` ist bei allen 51 ein Array. Insgesamt enthält es 38 Objekte, jeweils ausschließlich mit Schlüsseln `patch_event_id` und `url`. Alle Referenz-URLs haben Host `forums.playdeadlock.com`.
- Payloadschlüssel: `patch_date`, `source_patch_event_ids`, `source_urls` jeweils 51; `patch_gid` 13; `patch_title` 1. Payload-URLhosts stimmen mit den genannten Hosts überein. Alle 51 Patchdaten sind Strings, von `2026-05-22` bis `2026-06-30`.
- Metadaten enthalten nur die Schlüssel `curator` und `import_batch`, jeweils in 13 Zeilen. `import_batch` lautet dort `steam_0630`; bei den anderen 38 fehlt es.

Typverteilung:

| Typ | Zeilen |
| --- | --- |
| `balance_delta` | 14 |
| `rework` | 12 |
| `mechanic_removed` | 8 |
| `value_invalidation` | 4 |
| Übrige Patchtypen | 13: `patch_meta` 1, `objective_rework` 2, `objective_timing` 1, `objective_troopers` 1, `obsolete_statement` 2, `meta_system` 1, `mechanic_change` 3, `item_change` 2 |

49 Zeilen tragen `curated_patch_insight` und `current_until_superseded`; 2 tragen `superseded_by_patch` und `historical_only`. 44 haben `curated_patch_history`, 7 `curated_agent_inference`. Das sind gespeicherte Kennzeichen, kein in diesem Audit erneut bestätigter fachlicher Prüfstand.

**Urteil:** Die geprüfte strukturierte Herkunft aller 51 Zeilen weist auf Patchwissen. Keine YT-Quelle ist in den untersuchten Quellenfeldern enthalten. Aus möglicherweise beiläufigen Textverweisen lässt sich hier nichts ableiten, weil keine ungefilterten Texte gelesen wurden. Diese Zeilen sind kein nachgewiesener Ersatzbestand klassifizierter YT-Claims.

### Vorhandene Patch-JSON-Dateien

Unter `data/insights/` existieren zwei strukturierte Klassifikationsdateien. B prüfte Schema, Typen und Quellenhosts, ohne Aussageprosa auszugeben:

| Datei | Umfang und Herkunft | SHA-256 |
| --- | --- | --- |
| `data/insights/2026-07-01-spark-patch-insights.json` | 38 Einträge; ausschließlich Forum-Patchquellen; Typen rework/balance_delta/mechanic_removed/value_invalidation | `79e589433638b208172b53a29bf1d25b5b2d12ba9bbf717fc1cb6b4b3c71d277` |
| `data/insights/2026-07-01-steam-0630-insights.json` | 13 Einträge; ausschließlich die genannten Steam-/Assethosts; übrige Patchtypen | `1c4ed4f69650a1b8a8cc58ff12da8bd93875c453ad6548fc0da8730f5998b4d5` |

Umfang, Typen und Herkunft passen zu den DB-Aggregaten. Eine zeilenweise Gleichheit oder Importhistorie wurde nicht nachgewiesen. Beide sind echte Patchklassifikationsartefakte, keine YT-Claims-Exporte.

## Gezielte Suche nach früheren YT-Klassifikationen

Graphify wurde erneut vor der Bestandssuche befragt. Anschließend wurden bekannte Dokumentations- und Aufgabenpfade gezielt nach Kampagnen-/Exporthinweisen geprüft. Keine vollständige Festplattensuche.

`docs/TODO.md:3-5` nennt für den 28.06.2026 historisch 11.941 Creator-Claims und Statusmengen. `docs/transcript-claims-pipeline.md:159` nennt für den 27.06.2026 ungefähr 2.090 Transkriptclaims über ungefähr 133 Videos. Diese dokumentierten früheren Zustände beweisen weder heutige Tabellenmengen noch die Existenz eines heute greifbaren Exports. Aus dem Unterschied zum heutigen Befund wird kein Datenverlust abgeleitet.

Konkrete geprüfte Orte:

| Fundstelle oder Ort | Prüfung und Ergebnis |
| --- | --- |
| `docs/TODO.md:13-16,39-40` | Dokumentierter Kampagnenordner `~/.cache/deadlock_brain_campaign/`, darin damals Treiber/Prepare-/Bundleartefakte. Der Ordner existiert heute am genannten absoluten Pfad nicht; natives `stat` meldet `No such file or directory`. Kein Treiber ausgeführt. |
| `docs/transcript-claims-pipeline.md:166` | Beschreibt `verified_<id>.json`, `verified_*.json` und Bündelung als tatsächliche alte Zwischenartefakte. Nennt darüber hinaus keinen dauerhaften konkreten Exportpfad. Beispiele im Dokument sind Vertragsschemata, keine echten Claimproben. |
| Repo `docs/`, `.tasks/`, `architecture/`, `scripts/` | Gezielt auf Kampagnen-/Exportpfade geprüft, eigene Auditberichte von der Fundauswertung ausgeschlossen. Keine weitere konkrete YT-Exportablage lokalisiert. Buildlogs und Testnamen sind keine Exporte. |
| `data/raw/youtube/`, `data/youtube_transcripts/`, `data/insights/` | Nur Dateinamen und Unterordner bis maximal vier Ebenen geprüft: 1.274, 306 und 2 Dateien. Kein tieferer Ordner ausgelassen, keine Symlinks verfolgt. Kein JSON-Dateinamenkandidat mit claim/verified/bundle/campaign/export; die zwei tatsächlichen Insightdateien separat geprüft. |
| Gitindex des bestehenden Hauptbaums | Keine getrackte JSON-Datei mit claim/verified/campaign/bundle im Namen. Keine Gitmutation und keine Suche nach gelöschten Objekten. |
| `data/cache/` | Nur direktes Dateinameninventar: 688 hashbenannte `.bin.json`-Dateien, kein expliziter Klassifikations-/Exportname. Inhalte nicht pauschal geöffnet; daraus folgt keine inhaltliche Aussage über alle Cachedateien. |

Der Suchumfang schließt historische Backups, private Profile, Browserprofile, fremde Archive und beliebige weitere Home-/Temppfade aus. Anders benannte Dateien oder nicht dokumentierte Ablagen sind durch diese Suche nicht ausgeschlossen. Es wurde kein existierender öffentlicher YT-Claims-JSON-Export lokalisiert und daher keine echte gespeicherte Entityklassifikation mit Modellurteil und Quellenrevision vorgelegt.

## Konsequenz für die Abnahme

Der Datenzugangsblocker ist aufgelöst. Die aktuelle leere Claimsablage ist empirisch belegt und von vorhandenen öffentlichen Transkripten, Video-Snapshots und Patchinsights getrennt. Die fünf bereits belegten VTT-Passagen bleiben echte Quellen, aber keine klassifizierten Claims. Kein neues Modellurteil, kein fachlicher Aktualitätsbeweis und keine aktuelle Freigabe.

Die Restgrenze lautet jetzt: Ein früherer öffentlich belegter Klassifikationsexport wurde in den dokumentierten/geprüften Orten nicht wiedergefunden. Es fehlt nicht mehr die Berechtigung zur genannten DB-Abfrage. Keine Importierung, Rekonstruktion, neue Pipeline oder Datenverlustbehauptung daraus ableiten. B bleibt nach der ergänzten Übergabe zur Abnahme stehen.
