# Wiki-Korpus und Heldenkarten

## Einordnung

Der vorhandene Rust-Pfad wird erweitert, nicht ersetzt:

`MediaWiki API -> SourceStore -> vollständiges Importmanifest -> Game-Wiki-Snapshot -> Heldenkarte -> Antwort-/Build-Kontext`.

Das ist aktualisierbares Quellenwissen, kein Modelltraining. Die numerische Buildplanung bleibt im bestehenden Reasoner. Wiki-Texte dürfen weder Spielwerte überschreiben noch ungeprüfte Käufe oder neue Mechaniken in einen berechneten Build einschleusen.

## Umfang des Imports

`dbrain_sources::wiki_corpus::pull_wiki_corpus_with_pool` erfasst alle erreichbaren, nicht weiterleitenden Artikel des Hauptnamensraums. Enthalten sind gerenderte Vorlagen, Tabellen, Überschriften, Text und beschreibende Bild-Alternativtexte. Medien-Dateien, Diskussions-/Benutzerseiten und rohe Vorlagenprogramme werden nicht kopiert. Weiterleitungen werden nicht als doppelte Wissensseiten importiert.

Jede Seite enthält ihre Seiten-ID, Revision, Revisionsdatum, `touched`-Datum, Original- und Revisionsadresse, Lizenzangaben und Urheberhinweis. Historische Seiten und Abschnitte sowie Lore bleiben im Gesamtkorpus verfügbar. Die Lizenzangaben stammen aus `siteinfo/rightsinfo`; fehlen sie, bricht der Import ab. Eine Prüfung für etwaige externe Weiterveröffentlichung wird dadurch nicht ersetzt.

`oldid` fixiert die Artikelrevision. Weil gerenderte Vorlagen sich unabhängig davon ändern können, fließt `touched` über den unterstützten API-Parameter `requestid` in den HTTP-Cache-Schlüssel ein. Vollständig fixierte Vorlagenrevisionen werden ausdrücklich nicht behauptet (`rendered_templates_pinned=false`). Eine Wiki-Revision ist außerdem kein bestätigter Spiel-Patch (`patch_verified=false`).

Paginierung, leere gefilterte Batches, doppelte IDs, wiederholte Cursor, Größenlimits, API-Fehler, fehlende Inhalte und Revisionswechsel werden geprüft. Der Import führt keine Seitenskripte aus. Zwischen echten Requests liegen mindestens fünf Sekunden; Cache-Treffer verursachen diese Wartezeit nicht. Automatische schnelle HTTP-Retries sind für diesen Import abgeschaltet. HTTP 403/429 und API-Fehler führen zum sichtbaren Abbruch, nicht zu Umgehungsversuchen.

## Vollständigkeit und Veröffentlichung

Die bestehenden Tabellen `brain.source_documents`, `brain.entity_snapshots` und `brain.source_runs` reichen aus. Eine Migration ist nicht erforderlich.

Erst ein erfolgreicher `deadlock_wiki_corpus`-Run mit `complete=true` veröffentlicht die exakte Liste seiner Snapshot-IDs. Der Game-Wiki-Rebuild verwendet ausschließlich dieses Manifest. Dadurch bleiben Teilimporte unsichtbar; gelöschte Seiten verschwinden, Umbenennungen ersetzen alte Titel, unveränderte deduplizierte Snapshots bleiben verwendbar. Vor dem ersten erfolgreichen Gesamtabgleich bleiben bestehende Legacy-Wiki-Daten lesbar, aber neue unvollständige Korpusdaten sind ausgeschlossen.

Der bestehende atomare Refresh veröffentlicht den neuen Dateisnapshot erst nach seiner Validierung. Ein Fehler beim aktivierten Wiki-Import lässt den bisherigen veröffentlichten Stand bestehen. Ein erfolgreich importierter Datenbankstand und ein veröffentlichter Dateisnapshot sind bewusst unterschiedliche Zustände.

## Heldenkarte (`hero_dossier`, Schema 1)

Beim Rebuild wird pro kanonischem `deadlock_data/hero` eine Karte abgeleitet. Sie enthält Identität und vollständige strukturierte Spieldaten, die explizit über `BoundAbilities.Key` gebundenen Fähigkeitskarten, Quellen-/Snapshot-Hashes und passende Wiki-Abschnitte. Vorhandene deutsche Lokalisierungen werden über den bisherigen Enrichment-Pfad übernommen.

Verknüpft werden exakte Helden-Seitentitel und Unterseiten, außerdem Fähigkeits-Seitentitel nur bei eindeutiger Heldenzuordnung. Eine beiläufige Erwähnung eines anderen Helden erzeugt keine Verknüpfung. Fehlende oder mehrdeutige Fähigkeiten werden als Lücken gemeldet. Abweichende Wiki-Namen ohne eindeutige Zuordnung bleiben als fehlende Abdeckung sichtbar.

Die Karte enthält bis zu 20.000 Zeichen Gameplay-Notizen. Kürzungen und ausgelassene Abschnitte sind ausgewiesen; der vollständige Text bleibt im Seitenkorpus. Historie und Lore kommen nicht als aktuelle Build-Belege in diese Notizen. `coverage.complete=false` verspricht bewusst kein vollständiges Wissen über das Spiel; die konkreten Fähigkeiten-, Quellen- und Kürzungszähler geben die tatsächliche Abdeckung an.

Ausgabe im vorhandenen Format: `pages/deadlock-data/hero-dossier.md`. Programmatischer Zugriff: `dbrain_retrieval::load_hero_dossier(root, hero)` mit exaktem Namen oder Spielschlüssel. Heldenfragen erhalten die Karte automatisch in `game_knowledge.hero_dossier`; Build-Antworten erhalten sie in `hero_knowledge` und im vorhandenen Build-Prompt. Der berechnete Build selbst, seine Item-Reihenfolge und seine Werte werden nicht verändert.

Alle Wiki-Texte bleiben nicht vertrauenswürdige Quelldaten. Die Prompts grenzen sie von strukturierten Spielwerten ab. JSON-Payloads maskieren `<`, damit ein Quellentext keine zusätzlichen Game-Wiki-Eintragsgrenzen vortäuschen kann. Das ist eine strukturelle Absicherung, keine Garantie gegen sämtliche denkbaren LLM-Fehler.

## Aktivierung nach Review

Bestehende Refresh-Konfigurationen funktionieren unverändert. Netzwerkzugriff ist standardmäßig deaktiviert. Nach geklärtem Wiki-Zugang kann folgendes optionale Mitglied zur bestehenden JSON-Konfiguration hinzugefügt werden; die vier vorhandenen absoluten Pfade bleiben erhalten:

```json
"wiki": {
  "enabled": true,
  "min_delay_seconds": 5.0,
  "cache_ttl_seconds": 604800,
  "max_pages": 20000,
  "max_response_bytes": 4194304,
  "max_total_bytes": 268435456
}
```

Der vorhandene Befehl bleibt:

```sh
deadlock-brain wiki refresh --config /absoluter/pfad/wiki-refresh.json
```

`--skip-source-update` überspringt sowohl die Deadlock-Data-Aktualisierung als auch den Wiki-Netzwerkimport. Der Rebuild kann bereits importierte Daten und daraus abgeleitete Karten weiterhin verarbeiten. `cache_ttl_seconds: 0` erzwingt frische Abrufe. Bei großen Erstimporten müssen die Laufzeitgrenzen des aufrufenden Dienstes zum Seitenumfang passen; dieser PR ändert keine produktiven Units oder Timer.

## Prüfung und Grenzen der Abnahme

Netzwerkfreie Tests prüfen API-Verträge, Paginierung, Revisionen, Vorlagen-Cache-Schlüssel, Tabellen/Prozentwerte/Unicode, sichere Eintragsgrenzen, Heldenbindung, Fehlstellen, Budgetierung und Prompt-Einbindung. Der versionierte Korpus liefert in der Abnahme vom 24.09.2026 **39 Heldenkarten mit 156 Fähigkeitsbindungen ohne Zuordnungslücken**. Diese Zahl beschreibt den Repository-Snapshot, nicht eine Behauptung über die aktuelle öffentliche Heldenliste.

```sh
cd rust
SQLX_OFFLINE=true cargo test --locked \
  -p deadlock-brain-core -p dbrain-sources -p dbrain-retrieval -p deadlock-brain \
  --lib --bins -- --test-threads=1
```

Der separate Test `wiki_manifest_tests` braucht `WIKI_SCRATCH_DSN` zu einer leeren lokalen Datenbank mit Namen `brain_wiki_test_*` auf einem anderen Port als 5432/5433/5434. Er prüft zunächst die Datenbankidentität und legt das Testschema ohne `IF NOT EXISTS` an; vorhandene Daten werden nicht überschrieben. Er prüft die tatsächliche Snapshot-Auswahl inklusive Fehlern, laufenden Imports, Umbenennungen, Löschungen und wiederverwendeten IDs.

```sh
SQLX_OFFLINE=true WIKI_SCRATCH_DSN='postgresql://testuser@127.0.0.1:55471/brain_wiki_test_fresh' \
  cargo test --locked -p dbrain-retrieval wiki_manifest_tests \
  --lib -- --ignored --test-threads=1
```

Am 24.09.2026 beantwortete der echte Wiki-API-Endpunkt den Abruf vom Arbeitsserver mit **HTTP 403**. Deshalb ist der vollständige Live-Import nicht abgenommen und es wurde kein produktiver Wiki-Inhalt importiert. Im geltenden PR-Testbetrieb wurden auch kein Deploy, kein Merge und kein Bot-Neustart ausgeführt.

API-Verträge: https://www.mediawiki.org/wiki/API:Allpages, https://www.mediawiki.org/wiki/API:Info, https://www.mediawiki.org/wiki/API:Parsing_wikitext und https://www.mediawiki.org/w/api.php?action=help&modules=main (requestid).
