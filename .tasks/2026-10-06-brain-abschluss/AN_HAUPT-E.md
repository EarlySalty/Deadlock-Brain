# Paket E: API-Datenbasis für Brain v2, Release-Hold gilt

## Schnittstelle zu G, Entscheidung 05:25

G (`a867ef50`) besitzt Rechenschicht, Entitätsansichten und Werkzeuge im Antwortdienst. E liefert ausschließlich den versionsgebundenen Import und den gemeinsamen lokalen Leser. Keine Dokumentpakete, Steckbriefveröffentlichung, Rechenschicht oder zusätzlichen Antwortwege durch E.

**Einstieg:** `brain_storage::asset_mirror::{latest_mirrored_client_version, load_mirrored_assets}`. Beide Funktionen verwenden `sqlx::PgPool`, liefern `anyhow::Result` und führen keine HTTP-Abfrage aus. `load_mirrored_assets(pool, client_version, kind, language)` liefert unverändertes Original-JSON. Arten: `items`, `heroes`, `heroes_all`. Sprachen: `english`, `german`. `heroes` enthält die aktiven Helden; Fähigkeiten stehen im Items-/Fähigkeitskatalog, ihre Zuordnung im Heldenpayload. Keine eigene Fähigkeitentabelle. G bindet Entitäten über die API-IDs.

| Vorhandene Tabelle | Vertrag für G |
| --- | --- |
| `brain.source_runs` | `source='assets'`, `status='ok'`, `summary.mirror_complete=true`; `summary.client_version`, `parser_revision`, `mirrored_at`, `checked_at`, `manifest_document_id` und `endpoints["<kind>/<language>"].source_document_id` binden den vollständigen Spiegel. Nur vollständige erfolgreiche Runs lesen. |
| `brain.source_documents` | `source='deadlock_assets_api'`; Dokumentkennung enthält Version, Art und Sprache. `metadata.adapter.{client_version,kind,language}`, `metadata.validation.state='validated'`; Originalpayload unter `metadata.contract.data.payload.value`. URL, Rohhash, Schema und Herkunft bleiben im bestehenden Vertrag. |
| `brain.entity_snapshots` | Bestehende Projektion, nicht die versionsvollständige Historie. Identische Payloads mehrerer Versionen können dedupliziert sein. G liest die Originaldokumente über den gemeinsamen Leser. |
| `brain.patch_events`, `brain.patch_changes` | Bestehende Patchereignisse mit Datum, Entitybindung und Parserergebnis. API-Feed ergänzt nur Discovery. Technische Clientversion ist kein automatisch bestätigter Balancepatch. G darf Patchgeschichte nicht ohne belegte Zuordnung als Clientversionsgeschichte ausgeben. |

Keine Migration, zusätzliche Datenkopie oder neue Korpusaktivierung. Fehlende explizit angeforderte Version, Sprache, Art oder unvollständiger Lauf ergeben einen Fehler, niemals einen Live-Fallback. `latest_mirrored_client_version` bedeutet ausschließlich die jüngste vollständig erfolgreiche lokale Version, nicht eine unabhängig bestätigte aktive Balancepatchversion. Nach fehlgeschlagenem neueren Import bleibt die ältere Historie lesbar; G darf sie nicht ohne Versionsprüfung als aktuell ausgeben. Das lokal erhaltene Manifest liegt ebenfalls in `brain.source_documents`, `metadata.adapter.role='client_manifest'`, `metadata.adapter.client_version`, `external_id='steam_info/<version>@<derivation_hash>'`; `not_patch_mapping=true` kennzeichnet die fehlende bestätigte Patchzuordnung. `hero_catalog` und `item_catalog` sind nur bestehende Builds-Projektionen, keine versionsgebundene Rohdatenquelle für G. Zusätzliche bisherige Assets-Arten wie Farben und Ränge bleiben ungepinnt und erhalten ausdrücklich keine behauptete Clientversions-/Sprachbindung; der gemeinsame Leser bietet sie nicht an.

Der Leser wurde zunächst in Core geplant. Core hängt aber bereits von Builds ab; Builds kann deshalb nicht neu von Core abhängen. Der endgültige Ort ist Storage, unterhalb beider Verbraucher. Der erste Testlauf hat diesen eigenen Abhängigkeitszyklus aufgedeckt; er ist korrigiert.

## Eigentum und Betriebsgrenze

Eigener Worktree `/home/nathanael/.worktrees/brain-e-deadlock-api`, Branch `feat/brain-deadlock-api-daten`, Basis `bfda408c`. Die Freigaben 04:40, 04:55 und die Korrekturen 05:05, 05:10, 05:25 gelten. E speichert keine Einzelmatches und baut keinen Nachholweg. Publish-Regel und Reasoner gehören F. A besitzt die Übergangsfreischaltung, G deren strukturierte Nachfolge.

**Aktenort:** Der Worktree-Schutz verweigert das Schreiben in die gemeinsame Akte im ursprünglichen Checkout. Dieser eigene Bericht wird mit dem Featurecommit gesichert. Die ursprünglichen Analyseartefakte `E/DEADLOCK-API.md` und `E/MESSUNG.json` bleiben erhalten; ihre damalige Matchempfehlung ist durch 05:05 ersetzt.

**Vor Bearbeitung gemeldete Dateigrenzen:** Assets-Adapter, Builds-Client/Sync/Patchfenster, Storage-Lesemodul samt vorhandener Scratch-Leseprobe, Patchimport und Builddaten-Wrapper. In `deadlock-brain/src/main.rs` ausschließlich Assets-Hilfetext, Patch-Ingest-Argumente/Dispatch und Ingesttests. Diese Datei kann A überschneiden; keine Antwort-, Profil-, Veröffentlichungs- oder Aktivierungslogik durch E. Keine Produktänderung an Population oder Reasoner.

**Bestandskorrektur:** Graphify meldete ältere `sync_patchnotes`-/`refresh_official`-Funktionen. Auf der freigegebenen Basis existierte tatsächlich nur `pg import-patchnote`. Die neue Feed-Discovery hängt am aktuellen Einzelimport, dessen Originalauflösung und dessen Parser, nicht an einer zweiten Patchpipeline.

## Umgesetzt

1. Bestehender Assets-Ingest fragt das Versionsmanifest ab und speichert Items, aktive sowie alle Helden auf Englisch und Deutsch unverändert je `client_version`. Originaldokumente erhalten versionierte Kennungen. Nur erfolgreiche vollständige Runs sind für gemeinsame Leser verfügbar. Innerhalb von 24 Stunden darf derselbe vollständige Spiegel wiederverwendet werden; `checked_at` verlängert `mirrored_at` nicht. Bei neuer Version oder Ablauf erfolgt Vollabgleich.
2. Builds liest Items und Helden aus genau diesem lokalen Spiegel. Alle sechs Analytics-Pfade haben dieselbe belegte Patchuntergrenze. Buildprävalenz nutzt `min_last_updated_unix_timestamp`, ausdrücklich kein Matchzeitbeleg; Matchaggregate nutzen `min_unix_timestamp`. Nicht unterstützte `hero_ids` am Hero-Stats-Endpoint entfernt; Auswahl erfolgt lokal. Erfolgsjournal trägt Version und Zeitfenster.
3. `pg sync-patchnotes` liest `/v2/patches` als Discovery, verwendet vorhandene Originalauflösung, Parser, Patchtabellen und Deduplizierung. Neue Originalseiten-URLs erlauben nur exakte öffentliche Originalhosts per HTTPS ohne Zugangsdaten oder abweichenden Port. Kosmetische Ankündigungen und unaufgelöste Vorschauen werden nicht als Gameplaypatch importiert. Große Updatevorschauen ohne Volltext werden sichtbar als offen gemeldet, nicht als vollständiger Archivabgleich behauptet.
4. Bestehender Wrapper ruft Assets, Patch-Discovery und Builddaten auf. Population-Sync/-Stats und die fehlerverdeckenden Erfolgsechos sind entfernt. `set -euo pipefail` bleibt; ein Fehler beendet den Lauf.

**Betriebsverdrahtung read-only geprüft:** `deadlock-brain-build-data.timer` läuft täglich 03:30 Europe/Berlin. Sein Dienst ruft den Wrapper im `brain-live-main`-Worktree auf. Dieser Featurestand ist dort noch nicht installiert. Versionswechsel wird bei jedem Assets-Aufruf geprüft; ein zusätzlicher sofortiger Versionsereignis-Trigger wurde nicht gebaut. Installation und Live-Ingest muss `live_strecke` nach Freigabe übernehmen. Keine Betriebsaktion durch E.

## Abbauliste

| Eigener Weg | Ersatz und Stand |
| --- | --- |
| `dbrain-builds/src/api.rs`: ungepinnte Live-Katalogabrufe | Entfernt; gemeinsamer lokaler Assets-Leser verdrahtet. Bestehende Katalogprojektionen bleiben für aktuelle Builds-Verbraucher. |
| Builddaten-Wrapper: Population-Sync/-Stats und `|| echo` | Entfernt; keine Matchimporte über diesen Timerweg. |
| `dbrain-sources/src/game_files.rs`: strukturierte Helden-/Item-/Fähigkeitswerte | Gepinnter API-Spiegel; Abbau nach belegtem Gleichstand. Nicht angebotene Mechanik-/Kartendaten behalten. |
| `dbrain-sources/src/deadlock_data.rs`, `wiki.rs`, `wiki_corpus.rs`: dieselben Spielwerte | Derselbe Spiegel; Werte, IDs, Fähigkeiten und deutsche Texte pro Version vergleichen. Prosa, Herkunft, Rechte und historische Lücken gesondert behalten. |
| `dbrain-population/src/catalog.rs`, Populationtabellen/-Aggregate/-Sync/-Stats; `statlocker.rs`-Spielerimporte; `deadlock_api.rs`-Match-/History-/Demoablagen samt Rohartefakten | Nur als Abbau erfasst. Bestehende Daten nicht gelöscht und verwendete Verbraucher nicht pauschal entfernt. F/G entscheiden über benötigte kleine zeitgebundene Nebensignale. |

Der offizielle Patchnotes-Parser bleibt. Noch kein Gleichstand für die übrigen Parser behauptet. Kein Code auf Vorrat für G.

## Aktueller Fremdvertrag, vor Bearbeitung gemeldet

Die bestehende kleine öffentliche Contractprobe war rot: heutiges OpenAPI hat sich gegenüber dem Pin vom 25.09. geändert, daher griff die Schemaquarantäne. E hat die konkreten Unterschiede geprüft und den heutigen Original-Pin als neue, unveränderte Fixture aufgenommen (454617 Bytes, SHA256 `d76b44f82d1da13eed5831e772498a56993ebd56f4976c118f8ecaca79747359`). Bei Hero, Ability, Weapon, Upgrade, ItemProperty und StartingStats keine entfernten Top-Level-Felder oder Typänderungen; zusätzliche Felder bleiben Originaldaten. Items-/Heroes-/Colors-Antwortcontainer, positive Integer-IDs, `client_version` und beide Sprachen bleiben gleich. Requestschema ordnet `oneOf` anders; globale Security, Server und SecuritySchemes bleiben gleich. Der September-Pin bleibt unverändert erhalten.

Zusätzliche vor Bearbeitung gemeldete Dateigrenze: `dbrain-sources/src/schema_watch.rs` und OpenAPI-/Manifestfixtures. Das globale Pin-Update fordert außerdem vier neue Permanent-Buff-Aggregatfelder in Hero-Stats. Drei bestehende Analytics-Tests wurden dadurch rot; E ergänzt ausschließlich ihre gemeinsame `#[cfg(test)]`-Fixture `hero_row` in `analytics_runtime.rs`, keine Produkt-/Populationlogik. Keine automatische Baseline-Aktualisierung und keine Abschwächung des Drift-Gates. Eine explizite DB-freie Liveprobe im bestehenden Assets-Testmodul prüft den tatsächlichen Versionsmanifest-/Schema-/Adapterweg für alle drei Spielwertearten und beide Sprachen. Ihr erster Lauf fand einen echten Vertragsfehler: Items enthalten zulässige leere Namen (`/7/name:string_bounds`). Das aktuelle API-Schema erlaubt bei Hero, Ability, Weapon und Upgrade ausdrücklich jeden String ohne Mindestlänge. Der Adapter erhält leere Namen jetzt unverändert; stabile IDs bleiben Pflicht, falsche Typen und doppelte IDs werden weiterhin abgewiesen. Eine deterministische Gegenprobe deckt beide Seiten ab. Kein Ersatzname und keine ID aus Position oder Anzeigenamen. G nutzt weiterhin ausschließlich denselben Storage-Leser.

## Stop-Hook-Nachtrag

Der Stop-Hook fordert Main-Merge und Cleanup. Das aktuelle gemeinsame `A/RELEASEFENSTER.md` wurde erneut gelesen: weiterer Main-Push bleibt ausdrücklich gesperrt, ausschließlich `live_strecke` besitzt die Betriebsstrecke. Daher kein Main-Merge, keine Branch-/Worktree-Löschung und kein Settle. Das Gate hat weiterhin kein Urteil. Die ältere Zwischenlog-Ablage wird zusätzlich auf derselben Featurebranch gesichert; keine Produktänderung.

Origin-Sicherung bereits bestätigt: `f3c84fb4ee442196964387347773a75d704ebafd`, per `git ls-remote` gemessen. Alle geprüften Quellen stimmen unverändert mit HEAD überein. Acht schreibende Git-Einzelschritte bis zu dieser ersten Sicherung; der Lognachtrag folgt separat.

## Verifikation und Übergabe

**Umgesetzt und lokal verifiziert, nicht merge- oder livefertig.** Schlusslauf `E-tests-final-3.log`: 513 passed, 0 failed, 24 ignored. Die echte isolierte Postgres-Leseprobe und die Leername-Gegenprobe liefen. Beide getrennten öffentlichen Contractproben bestanden zusätzlich mit je 1 passed: 6759, Items 746, aktive Helden 40, alle Helden 65, jeweils Englisch und Deutsch. Keine produktive DB, keine Einzelmatches und keine Mitgliederkennungen. Literalbefehle, Rohhashes, vorangegangene Fehler und ignorierte Grenzen: `E/NACHWEISE.md`.

Format, Shellsyntax und Clippy aller fünf betroffenen Pakete (`--all-targets --no-deps -- -D warnings`) bestehen. Der breite Dependency-Clippy bleibt an vier `map_or_identity`-Diagnosen in `dbrain-enrich/src/lib.rs:519,651,1291,1442` rot. Keine Altfehlerbehauptung ohne Baseline, keine fremde Produktänderung und keine Lint-Ausnahme.

**Commits:** `5e70da3a6f40c0f1eedc78565641d8dfce582f56` (geprüfter Originalschema-Pin samt erforderlicher Analytics-Testfixture), `e65efae2c7c53dd4d17d75f51974753d6fd84d08` (API-Spiegel, gemeinsamer Leser, Zeitfilter, vorhandener Patchimport, Wrapper). Feature-Sicherungsziel: `origin/feat/brain-deadlock-api-daten`. Die Berichtssicherung folgt auf dieselbe Branch; Schlussantwort nennt den tatsächlich bestätigten Pushstand.

**Gate blockiert technisch, kein Modellurteil:** Pin zweimal gegen `bfda408c`, API-Diff einmal getrennt gegen `5e70da3a`; alle drei Exit 2. `bwrap`/`unshare`: `Cannot allocate memory`. Erster Lauf zusätzlich Grok HTTP 402. Kein ALLOW und keine inhaltlichen BLOCK-Funde. Auch der kleine API-Diff scheitert am selben Werkzeugpfad. Kein verlässlich ableitbarer Reset-Zeitpunkt. Kein Modellwechsel, keine Schutzumgehung und keine weiteren erfolglosen Pollversuche. Origin-WIP ist ausdrücklich keine Mergefreigabe.

**Offen:** Gate-Werkzeugpfad reparieren und beide Commits regulär nachprüfen, breiten Dependency-Lint im zuständigen Bereich nachziehen, Main erst nach ALLOW und aufgehobenem Hold integrieren. `live_strecke` übernimmt Installation, vorhandene Timerverdrahtung für zeitnahe Versionswechsel sowie täglichen Vollabgleich, Ingest und Liveantwortbeweis. G nutzt den oben beschriebenen Datenvertrag. Reihenfolge und Orte: `E/TODO.md`.

Main-Merge/-Push, Releasebuild, Installation, Neustart und Tick bleiben gesperrt. Worktree und Branch bleiben erhalten; kein Settle bei offener Übergabe. Vor Berichtscommit und Featurepush wurden fünf schreibende Git-Schritte einzeln ausgeführt; keine Mergeanläufe.

TESTNACHWEIS[TW-1]: 513 passed, 24 ignored | Baseline: unbekannt rot

WIRKUNGSPRUEFUNG[WP-1]: 6 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 7/7 geprüft

Geprüfte Pfadgruppen: Manifest, OpenAPI, Assets in beiden Sprachrunden, sechs Analytics-Aufrufpfade, Patchfeed, neue Original-HTML-Abrufe und bestehende Steam-Ledger-Auflösung. Ergebnis: lokale Versionsbindung, sichtbare Fehler/Skips, keine Erfolgsverdecker und kein Matchimport. Technische Prüfblocker bleiben ausdrücklich offen.

MERGEPROTOKOLL[MS-1]: 5 Git-Schritte einzeln | Anläufe: 0 | Gate: kein Urteil, Exit 2; kein Main-Merge

BESTAND[BS-1]: ja | Fundort: rust/crates/dbrain-sources/src/assets_api.rs:35 | Anknüpfung: SourceStore, versionsgebundene Originaldokumente, vorhandener Patchimport und bestehender Builddaten-Sync
