# Deadlock-API für das eine Brain

Stand: 07.10.2026, öffentliche Leseabfragen zwischen etwa 04:13 und 04:30 CEST. Messwerte und genaue Abfrageparameter: [MESSUNG.json](MESSUNG.json). Keine Produktänderung, kein Datenimport, kein Tick, keine Veröffentlichung.

## Urteil

**Die API kann die Hauptquelle für versionierte Helden-, Item- und Fähigkeitswerte werden. Die Datenbank und die Ingest-Wege dafür existieren bereits. Eine zweite Spieldaten-Datenbank oder ein weiterer Brain-Dienst ist nicht nötig.**

**Analytics allein erfüllt die 100-Match-Grenze nicht ehrlich.** Der Publisher verlangt 100 aktuelle Spieler-Matches **derselben Buildfamilie**, nicht 100 Spiele insgesamt oder eine Summe aus Itemstatistiken. Die API liefert zusätzlich echte Einzelmatchdaten. In einer engen Warden-Probe waren 116 unterschiedliche Spieler-Match-Paare verfügbar, davon 115 mit gewertetem Ergebnis und verwertbarer Kaufstruktur. Ob davon 100 derselben Familie nach dem tatsächlich aktiven Patch angehören, bleibt bis zum bestehenden Import und erneuten Familienlauf offen.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/dbrain-sources/src/assets_api.rs:16 | Anknüpfung: bestehender Assets-Ingest, SourceStore, versionierte SourceRecords, Steckbrief-/Patch-Story-Ableitung und vorhandener Population-Import

## 1. Beweisgrenzen

1. Der Arbeitscheckout `feat/brain-rust-cutover-20260919` ist eine alte, fremd bearbeitete Basis. Die Bestandsbewertung stützt sich auf den geprüften Main-Stand `bfda408cb988722ddceadb56bca5b72e12d12731`, nicht auf dessen alte Assets-URLs. Graphify wurde zuerst lokal und global abgefragt; die Fundstellen wurden anschließend am Quellcode geprüft.
2. Das aktive Build-Daten-Service zeigt auf `/home/nathanael/.worktrees/brain-live-main`. Dessen gelesener Git-Stand war `39710e3282c830ee9b47e90945deb71d1db44724`. Das ist ein Beleg für den Arbeitsverzeichnisstand, kein Manifestbeleg des laufenden Binaries. Ein erfolgreich beendeter Timerlauf ist kein Nachweis, dass der Antwortkorpus aktuelle Spielquellen enthält.
3. Die direkte Leseprobe auf `127.0.0.1:5446` scheiterte mit „Connection refused“. `ss -ltn` zeigte keinen Listener auf 5446. Tabellen und Schreibwege unten sind deshalb durch Code und Migrationen belegt, nicht durch frische DDL oder Zeilenzahlen dieser Instanz. Daraus folgt kein Urteil, dass das gesamte Brain ausgefallen ist. Der konfigurierte Laufzeit-Datenbankzugang muss vom bestehenden Live-Eigentümer nachgewiesen werden.
4. Die gemeldeten 47 aktuellen Familien-Matches stammen aus `A/INVENTUR-REASONER.md`. Sie wurden hier nicht durch einen neuen Reasoner-Lauf überprüft. Die externen Messfenster beginnen bei öffentlichen Update-Ankündigungen; sie ersetzen nicht den im Brain aktiven `FamilyPolicy.patch_started_at`.
5. Keine API-Schlüssel verwendet. Keine Steam-IDs von Mitgliedern, Community-Nachrichten oder Nutzerprofile übermittelt. Öffentliche Spielerkennungen aus Antworten wurden nur im Arbeitsspeicher zur Zahl unabhängiger Spieler verarbeitet. Die Akte enthält keine solchen Kennungen. Keine neuen Werkzeuge installiert.

## 2. Bestand: heute vorhandene Wege

### Spielwerte, Matchdaten und Statistik

| Bestehender Weg | Externe Quelle und Endpunkte | Bestehender Speicher | Bewertung |
| --- | --- | --- | --- |
| `dbrain-sources::assets_api` | `api.deadlock-api.com/v1/assets/{items,heroes,ranks,colors,build-tags,npc-units}`; Helden aktiv und vollständig | `brain.source_runs`, `brain.source_documents`, `brain.entity_snapshots` über `SourceStore` | Hauptanschluss für API-Rohdaten. Bounded HTTP, Parserrevision und Schemaabweichungsprüfung bestehen bereits. `assets_api.rs:16`, `:103`, `:108`, `:120`. |
| `dbrain-builds` und `deadlock-brain-build-data.timer` | Dieselben Assets plus `/v1/analytics/build-item-stats`, `item-stats`, `hero-stats`, `hero-stats` mit `include_item_ids`, `ability-order-stats`, `item-permutation-stats` | `brain.item_catalog`, `brain.hero_catalog`, `brain.hero_item_stats`, `brain.hero_ability_orders`, `brain.hero_item_synergies` | Vorhandene Reasoner-Projektion behalten, nicht noch einen Katalog daneben bauen. `api.rs:9`, `:35`; `sync.rs:70`, `:96`. |
| `dbrain-population` | `/v1/assets/items`, `/v1/assets/heroes`, `/v1/matches/metadata` mit Datum, Heldenfilter, Spielerinformationen, Käufen und Statistikverläufen | `brain.population_player_matches`, `brain.population_sync_runs`; abgeleitete Populationstabellen über `aggregate.rs` | Der benötigte Einzelmatch-Import existiert. Pagination über `max_match_id`, `ON CONFLICT(match_id,account_id) DO NOTHING`. `api.rs:46`, `:163`; `cli.rs:83`, `:110`; `db.rs:43`. |
| `dbrain-sources::deadlock_api` und `brain-feeds::deadlock_match` | `/v1/matches/metadata`, `/v1/players/{account_id}/match-history`, `/v1/matches/demo/query` samt Polling und Ergebnisdownload | Gemeinsamer SourceStore bzw. versionierter Quellenweg; Match-, Verlauf- und Demoevidenz | Für diesen Auftrag nur öffentliche Matchabfragen verwenden. Spielerbezogene History- und Demo-Aufträge nicht aus Community-Identitäten erzeugen. `deadlock_api.rs:182`, `:376`, `:434`; `brain-feeds/src/deadlock_match.rs:148`. |
| `dbrain-sources::analytics_runtime` | Typisierter Abruf von `/v1/analytics/hero-stats` für Meta-/Population-Lookups | Validierte `AnalyticsObservation`, nicht automatisch eine neue Populationsstichprobe | Bestehender HTTP-/Validierungsbaustein statt neuer Connector. `analytics_runtime.rs:15`, `:179`, `:186`. |

Das Timerprogramm enthält auf beiden gelesenen Git-Ständen bereits `pull build-data --hero all`, `population sync --matches 2000` und `population stats`. Der Populationteil ist damit nicht neu zu erfinden. Die beiden Populationkommandos haben jeweils einen `|| echo`-Fehlerpfad: Der Timer kann erfolgreich enden, obwohl die Population nicht aktualisiert wurde. Der genaue verwendete Wrapper und dessen Binary müssen zur Laufzeitquelle passen.

**Relevante heutige Schwäche:** `dbrain-builds/src/api.rs:35-85` setzt keine explizite Matchzeituntergrenze. `sync.rs:81` holt dennoch den neuesten lokalen Patchtag und schreibt die zurückgegebenen Statistikdaten darunter. Ein lokaler Patchtag macht eine externe Standardzeitspanne nicht zu Nach-Patch-Daten. Die ganze Gruppe aus Item-, Helden-, Lift-, Skillorder- und Synergieabfragen muss denselben ausdrücklich gespeicherten Zeitraum benutzen. Buildhäufigkeit ist davon getrennt: Sie filtert die Aktualisierungszeit eines Builds, nicht die Startzeit eines Matches.

Der API-Vertrag für `hero-stats` nennt aktuell keinen `hero_ids`-Parameter. Der bestehende Client sendet ihn trotzdem. Die Probe lieferte alle 40 Helden; Warden wurde lokal nach `hero_id=25` ausgewählt. In einer späteren Umsetzung nur dokumentierte Filter verwenden und die Entitätsauswahl lokal ausdrücklich prüfen.

### Patchtexte, Spieldateien und Steckbriefe

| Bestehender Weg | Quelle | Speicher und Übergang | Behalten oder zusammenführen |
| --- | --- | --- | --- |
| Patchnotes-Bot und `pg_patchnotes` | `patchnotes.changelog_posts`; Auflösung verlinkter offizieller Steam-News über `ISteamNews/GetNewsForApp/v2/` | `brain.source_documents`, `entity_snapshots`, `patch_events`, `knowledge_events`, `patch_event_enrichments` | Der Bot ist ein laufender separater Quelldienst, kein neuer Brain. Auflösung vollständiger Originaltexte ist bereits vorhanden. `pg_patchnotes.rs:19`, `:253`, `:278`. |
| `pg_steam_news` und `dbrain-normalize::patch` | Direkter Steam-News-Import bzw. Parsing vorhandener Patch-Snapshots | Dieselben Patchereignisse; `brain.patch_changes` ist eine **View** auf `patch_events` samt Anreicherungen und Patchkatalog | Teilweise parallele Parser/Importwege. API-Feed als Discovery-Eingang in den vorhandenen Patch-Sync führen, keine dritte Patch-Tabelle und keinen weiteren Parser bauen. `pg_steam_news.rs:19`, `:157`, `:831`; `patch.rs:125`, `:1008`; `store.rs:365`. |
| `dbrain-sources::deadlock_data` | Gepinntes `deadlock-wiki/deadlock-data`-Git-Repository mit Helden, Fähigkeiten, Items, Karten, Änderungsdateien | SourceStore; zusätzlich `brain.entities`, `entity_aliases`, `hero_stat_profiles`, `hero_stat_values` | Für Mechaniklücken und Quellenvergleich behalten. Nicht als konkurrierende unmarkierte „aktuellste“ Kopie zu API-Werten verwenden. `deadlock_data.rs:20`, `:212`. |
| `wiki`, `wiki_corpus`, `game_files`, `git_source` | MediaWiki unter `deadlock.wiki/api.php`, vorhandene Wiki-Erfassung sowie gepinnte KV/KV3/VPK-/GameTracking-Ressourcen | Validierte Quellen und normalisierte Fakten über bestehende Ingest-/Speicherbausteine | Wiki liefert Erklärungen; Spieldateien liefern Details und unabhängige Nachweise. Kein Ersatz durch reine Analytics. `wiki.rs:20`, `:50`; `game_files.rs:77`, `:283`; `git_source.rs:79`. |
| Steckbrief-/Patch-Story-Ableitung | `entity_binding`, `entity_derivation`, `brain-storage::entity_profile` | `brain.entity_profile_entities_v1`, `entity_profile_facts_v1`, `entity_semantic_projections_v1`, `source_record_revisions`, `source_record_heads`, `entity_derived_receipts_v1`, `corpus_releases_v1` | Das ist der eine Veröffentlichungsweg. Patch-Story liest `brain.patch_changes`. A besitzt Aktivierung und Live-Abnahme. `entity_derivation.rs:131`; `entity_profile.rs:391`, `:406`, `:603`; `pg_release.rs:51`, `:329`. |

`/v1/mcp` wurde im produktiven Rust-Code nicht als vorhandener Brain-Connector gefunden. Der Treffer im getrackten Repo ist die externe OpenAPI-Testfixture. Der öffentliche Dienst funktioniert, aber „ist erreichbar“ und „wird vom Brain genutzt“ sind verschiedene Aussagen. Die Projektkonfiguration enthielt keinen nachgewiesenen aktiven Deadlock-MCP-Anschluss. Nicht eigenmächtig registriert.

Gelesene Timer: Builddaten täglich 03:30 Europe/Berlin; Patchnotes etwa alle fünf Minuten; Wiki-Refresh um 00:35, 06:35, 12:35 und 18:35. Alle standen als wartend im Benutzer-Systemd. Keiner wurde durch E gestartet oder verändert.

## 3. Was die öffentliche API tatsächlich liefert

Host für die folgenden REST-Pfade: `https://api.deadlock-api.com`. Die alte Domain `assets.deadlock-api.com` ließ sich hier nicht auflösen. Auf dem geprüften Main ist der Hostwechsel bereits umgesetzt. Diesen Fix nicht noch einmal bauen oder den alten Host wieder aktivieren.

### Assets, Patches und Builds

| Endpunkt | Echte Probe und Inhalt | Versionierung und Aktualität | Limits und Einschränkungen |
| --- | --- | --- | --- |
| `/v1/assets/heroes`, `/heroes/{id}` | HTTP 200, 40 aktive Helden; Warden 25 und Haze 13 mit Grundwerten, Skalierung und Signature-Verweisen in `items`, nicht in einem Feld `abilities`. Deutsch und Englisch abrufbar. | `client_version` auswählbar. Warden 6597 und 6759 lieferten unterschiedliche Hashes; 6757 und 6759 identische Wardenwerte. Neue Clientversion bedeutet nicht automatisch Änderung jedes Helden. | Aktuelle OpenAPI nennt kein eigenes Rate-Limit für Assets. Keine verfügbare SLA. Historische Warden-Abfrage 5044 gab 404 trotz Eintrag in der Versionsliste. Vollständigkeit je Entität/Version prüfen. |
| `/v1/assets/items`, `/items/{id}` | HTTP 200, 746 Einträge: 410 Fähigkeiten, 86 Waffen, 250 Upgrades. Werte, Bedingungen, Skalierungsdefinitionen, Kosten und Upgrades. Mystic Burst 1998374645 und Bullet Dance 731943444 tatsächlich vorhanden. | Gleicher `client_version`- und Sprachfilter. Kein unterschiedlicher „Fähigkeiten-Dienst“ nötig. IDs/Class-Namen mit Helden-Signatures verbinden. | Gesamtliste etwa 6 MB. Werte teils Zahl, teils String mit Einheit; Beschreibung enthält HTML/SVG. Bestehenden Parser verwenden, keine freie numerische Vermutung. |
| `/v1/assets/client-versions`, `/steam-info`, `/steam-info/all` | HTTP 200; 830 Assetversionen, jüngste 6759; 831 Steam-Manifeste. Version 6759 mit Source-Revision 11094174 und Buildzeit `2026-10-06T15:31:13`. | Echte technische Versionshistorie. Buildzeit enthält keinen Offset und ist keine gemessene Bereitstellungszeit der API oder des Valve-Updates. | Fehlende Versionen und unterschiedliche Listenlängen abfangen. Beobachteten ersten Abrufzeitpunkt separat speichern. Kein Beweis „innerhalb X Minuten nach jedem Patch“. |
| `/v2/patches`, `/v1/patches/big-days` | HTTP 200; 30 RSS-Einträge, 20 Forum und 10 Steam. Minor Update vom 05.10. enthält Patchzeilen. Forumseintrag „09-29-2026“ enthält nur 148 Textbytes einer Linkvorschau; großer Steam-Post verweist auf die Valve-Sonderseite. | Feed-GUID, Quelle, Datum und Inhalt; keine vollständige Patch-/Clientversions-Zuordnung. Der Forumseintrag vom 29.09. hat `pub_date=05.10.`, deshalb Datum nicht blind als Patchbeginn übernehmen. | Patchfeeds 100 Anfragen/s je IP laut Vertrag. `big-days` manuell, laut Doku mögliche Stunden Verzögerung; gemessen jüngster Eintrag 11.03.2026, somit für heutige Patcherkennung unbrauchbar. `/v1/patches` veraltet. RSS ist kein vollständiges Archiv. |
| `/v1/builds`, `/builds/{hero_id}/{build_id}` | Suche nach Held, Name, Beschreibung, Sprache, Datum, Sortierung und Pagination. Probe Warden mit zwei aktuellen Ergebnissen: 803307 Version 387 und 855068 Version 1. Detailobjekt liegt in `hero_build`. | Eigene Build-ID, Version und Veröffentlichungs-/Änderungszeit. Das ist keine Spielpatchversion und keine Match-Evidenz. `version`-Filter im Suchvertrag vorhanden. | Suche 100 Anfragen/s je IP laut Vertrag; Standardlimit 100, `start` für Pagination. Bei direkten Detailabrufen eigene Vertragsbeschreibung beachten. Tatsächliche Builddetails nicht allein aus Titel/Favoriten ableiten. |

Beispiele aus dem gepinnten Stand 6759, keine eigenen Berechnungen: Mystic Burst kostet 800, `Damage="40"`, `MinimumDamage="80"`, `AbilityCooldown="14"`. Bullet Dance hat `AbilityCooldown=165`, `AbilityDuration="3.5"`, `Radius="16m"`, `WeaponDamageBonus="7"` und drei Upgrade-Stufen. Diese Rohfelder müssen zusammen mit Bedingungen und Skalierung in die bestehenden Mechanikmodelle eingehen. Eine einzelne Zahl ist kein vollständiger Fähigkeitssteckbrief.

### Analytics, Einzelmatches und Abfragezugänge

| Endpunkt | Inhalt und echte Probe | Patchbezug und Aktualität | Limits und Einschränkungen |
| --- | --- | --- | --- |
| `/v1/analytics/hero-stats`, `/item-stats`, `/game-stats` | Helden-Wins/Losses, Item-Wins/Losses mit Kauf-/Verkaufszeit, Spiel-/Spielerzahlen. Datums- und Rang-Buckets funktionieren. Gemessene Zahlen unten. | Zeit-/Rang-/Modusfilter, aber keine verlässliche automatische Spielpatch-ID. Tagesaggregate erwiesen sich nicht als minutengenauer Patchnachweis. Itemstatistik laut Vertrag sechs Stunden Cache je Parameterkombination; für Hero-/Game-Stats dort kein gleicher Cachewert zugesichert. | Gemeinsames Budget aller Analytics: IP 200/min, Schlüssel 400/min, global 2000/min. `min_matches` bei Itemstatistik standardmäßig 20. Helden-/Itemsummen nicht als Zahl unabhängiger Matches addieren. |
| Vorhandene Zusatzanalytics: `build-item-stats`, `ability-order-stats`, `item-permutation-stats` | Buildverbreitung, Skillorder und Itemkombinationen. Bestehender Client benutzt sie bereits. Verträge am Live-OpenAPI-Endpunkt gelesen, hier keine zusätzliche Vollabnahme dieser drei Nutzdaten. | Buildverbreitung filtert `min_last_updated_unix_timestamp`; Skillorder und Itemkombinationen `min_unix_timestamp`. Cache: Builds eine Stunde, Permutationen sechs Stunden; für Skillorder kein entsprechender Wert im gelesenen Vertrag. | Gleiches gemeinsames Analytics-Budget. Itemkombinationen überlappen; ihre Fallzahlen sind keine neue Einzelmatch-Stichprobe. |
| `/v1/matches/metadata` | HTTP 200; echte Warden-Matches mit Startzeit, Ausgang, Spieler-/Helden-ID, Kauf-, Verkaufs-, Skill- und Imbue-Ereignissen. `hero_build_id` nur falls vorhanden; erste bei Matchbeginn ausgewählte Build-ID, kein Beweis durchgängig gespielten Builds. | Zeitraumfilter und Pagination über Match-ID. Keine garantierte Vollabdeckung oder gemessene Minuten-SLA. Die Probe endet bei Matchstart 07.10. 01:14:38 UTC, nicht beim Abfragezeitpunkt 02:27 UTC. | IP 30/min, Schlüssel 30/10s, global 600/min; Standardlimit 1000, Maximum 10000. Schon 116 Matches: etwa 13 MB ohne volle Statistikverläufe, etwa 49 MB mit ihnen. Kleine Seiten und bestehende Größenlimits statt blindem Maximallimit. |
| `/v1/mcp` | HTTP 200 für `tools/list` und `execute_query` mit `sql="SELECT 1 AS read_only_probe"`; Ergebnis 1, `isError=false`. Tools außerdem Tabellen-/Spalten-, Helden- und Itemlisten. | Read-only-Abfragen auf stündliche Parquet-Snapshots; DuckDB/DuckLake, nicht dieselbe Live-ClickHouse-Abfrage wie altes REST-SQL. | Kostenlos und ohne Konto laut öffentlicher Doku. Tool begrenzt Ergebnis auf 1024 Zeilen und 50 KB. Keine veröffentlichte RPM-Zusage in der gelesenen Toolliste. HTTP-Erfolg allein zählt nicht: JSON-RPC-Fehler und `isError` prüfen. |
| `/v1/sql` und öffentlicher Data Lake | SQL-Probe HTTP 200 und Wert 1. Manifest HTTP 200, Version 500, erzeugt 07.10. 02:10:47 UTC; `match_player`-Aufnahmewatermark 01:53 UTC, 153 Dateien. | REST-SQL ist ausdrücklich veraltet und soll entfernt werden. Data Lake ist versionierter Snapshot, dessen Watermark `created_at` bezeichnet, nicht Matchbeginn oder Vollständigkeit. | Altes SQL: IP 2/min und 20/h, Schlüssel 10/min, global 30/min. Kein neuer produktiver Connector darauf. Bulk-Import über MCP ist wegen Ergebnislimit nicht der Ersatz für vorhandene Metadatenpagination. |

### Nutzungsbedingungen

Die öffentliche Seite bezeichnet die Daten als „Open Data“ und „Free to Use“. OpenAPI nennt MIT und weist darauf hin, dass die API nicht von Valve unterstützt wird. Das GitHub-MIT-Dokument lizenziert Software und begleitende Dokumentation; daraus folgt **keine pauschale Lizenz an Valve-Grafiken, allen Wiki-Texten oder Spielerprofilen**. Eine zusätzliche konkrete Datenlizenz, verbindliche Vollständigkeit oder Bereitstellungs-SLA wurde in den gelesenen Seiten nicht gefunden.

Für jeden Endpunkt gelten die Herkunft und Rechte des Inhalts zusätzlich zum technischen Zugriff. Quellenangabe und vorhandene SourcePolicy-/Rechteprüfung behalten. Keine unkontrollierte öffentliche Spiegelung von Assets oder personenbezogenen Datensätzen. Die vorgeschlagene Wissensprojektion benutzt Spielwerte und belegte, aufbereitete Spielinformationen. Interne Community-Fragen gehen niemals an den externen MCP-Server.

## 4. Messung der Matchmenge

Warden ist `hero_id=25`. Die beiden Untergrenzen sind bewusst als **Ankündigungsfenster** bezeichnet:

| Messfenster | Warden-Matchzahl aus `game-stats`, alle Ränge | `hero-stats`, Mindest-Badge 91 | `item-stats`, Mindest-Badge 91 und mindestens 100 Fälle |
| --- | --- | --- | --- |
| Seit 29.09.2026 20:25:11 UTC, `1790713511` | 11.478 | 1.250, davon 530 Siege und 720 Niederlagen | 46 Items, höchste Fallzahl 1.127 |
| Seit 05.10.2026 23:05:32 UTC, `1791241532` | 1.146 | 218, davon 87 Siege und 131 Niederlagen | 15 Items, höchste Fallzahl 210 |

Alle Abfragen benutzen normale Ranked-Matches. Die unterschiedlichen Rangfilter sind absichtlich sichtbar; diese Spalten sind nicht dieselbe Grundmenge. Ergebnisse wurden zeitlich nacheinander abgeholt und haben unterschiedliche Cache-/Aggregationswege.

Zusätzlich wurde das bestehende Builddaten-Standardminimum 80 abgefragt. Der Warden-Tagesabruf gab 241, 180 und 7 Matches in den Buckets 05., 06. und 07.10. zurück. Rang-Buckets reichten von 81 bis 114, zum Beispiel 86 mit 43 Fällen, 92 mit 39 und 112 mit 6. Alle Einzelwerte stehen in `MESSUNG.json`.

**Grenzprobe:** Zwei identische `hero-stats`-Tagesabfragen mit `max_unix_timestamp=1791331200`, aber Untergrenzen 05.10. 00:00 UTC (`1791158400`) bzw. 05.10. 23:05:32 UTC (`1791241532`), lieferten beide dieselben drei Warden-Buckets mit 241/180/7. Damit ist ein abgefragter Uhrzeitfilter hier **kein Beleg für minutengenau ausgeschnittene Nach-Patch-Fälle**. Die Probe zeigt das konkrete Verhalten dieses Pfads; daraus folgt nicht, dass jeder Analytics-Endpunkt dieselbe Granularität benutzt.

Die Metadatenprobe mit `hero_ids=25`, Mindest-Badge 91, derselben späten Untergrenze und `limit=200` lieferte:

- 116 unterschiedliche Warden-Spieler-Match-Paare aus 73 öffentlichen Spielern.
- 115 gewertete Fälle mit mindestens einem passenden Upgrade-Kauf, 115 mit Fähigkeitsevents; ein Fall `NotScored` zählt nicht.
- 110 gewertete Fälle mit mindestens einem Imbue-Ereignis.
- Startzeiten von 05.10. 23:12:41 bis 07.10. 01:14:38 UTC.
- Keine Ausführung des Rust-Cleaners, kein DB-Import und kein Familienclustering. Die Prüfung belegt die Datenstruktur und Beschaffbarkeit, nicht die endgültige Aufnahme aller Fälle.

Die API liefert `start_time` ohne Offset, beispielsweise `2026-10-07 01:14:38`. Die erste Messauswertung interpretierte solche Strings irrtümlich in der lokalen Zeitzone und wurde verworfen. Die dokumentierten Zahlen interpretieren sie ausdrücklich als UTC. Vor dem Import muss der bestehende Postgres-Zugang diese Konvention erfüllen: `dbrain-population/src/db.rs:56` castet Text nach `timestamptz`; eine andere Sessionzeitzone würde den Patchbezug verschieben. Ohne Nachweis kein minutengenauer Aktualitätsbeleg.

### Gesonderte Antwort auf die 100-Match-Frage

**Nein, nicht mit Analytics-Aggregaten allein. Ja, es gibt genügend zusätzliche echte Matchdaten für einen ehrlichen Beschaffungsversuch über den bereits vorhandenen Einzelmatchweg. Eine erfolgreiche Freischaltung ist noch nicht bewiesen.**

Der aktuelle Guard in `dbrain-reasoner/src/publish.rs:109-127` verlangt:

1. Eine explizite Buildfamilie, `eligible_for_planning=true` und mindestens `max(policy.min_matches,100)` aktuelle Spieler-Matches dieser Familie.
2. Kein leerer Core und keine Konfidenz `Low`.
3. Vorhandene Skillorder und Patch-Provenienz.

`families/mod.rs:188` dedupliziert Identitäten. `:295` zählt Nach-Patch-Mitglieder anhand `observed_at >= patch_started_at`. Die Planungsprüfung verlangt normalerweise außerdem mindestens 20 unterschiedliche Spieler, ausreichenden Anteil und Kohäsion. `families/data.rs:81` lädt ein begrenztes aktuelles Populationsfenster, nicht externe Summen.

Deshalb dürfen weder die 1.146 Gesamtspiele, die 218 Analytics-Fälle noch die 115 strukturell passenden Einzelbeobachtungen direkt als Familienfeld eingetragen werden. Auch 47 bisherige plus 115 abgefragte Fälle sind **nicht** automatisch 162: Überschneidungen, andere Familien und ein neuerer aktiver Patch können die Zahl reduzieren. Das bestehende Limit von 100 bleibt unverändert.

## 5. Gegenüberstellung des Brain-Bedarfs

| Bedarf | API-Angebot | Heutiger eigener Weg | Entscheidung und Grund |
| --- | --- | --- | --- |
| Steckbriefe des aktuellen Patches | Versionierte Helden, Items, Waffen und Fähigkeiten samt deutscher Lokalisierung | Assets-Snapshots, `deadlock-data`, GameTracking/Spieldateien, Wiki, bestehende EntityProfile-Ableitung | **Ersetzen:** API als Hauptlieferant der von ihr vollständig belegten strukturierten Werte. **Behalten:** bestehende Ableitung und Quellenprüfung. Wiki/Spieldateien ergänzen nur fehlende Mechanik, Interpretation und Gegenbelege. |
| Patch-Story und Änderungshistorie | Neueste Forum-/Steam-Feeds, technische Clienthistorie; teilweise vollständige Patchtexte, teilweise nur Linkvorschau | Patchnotes-Bot, `pg_patchnotes`, `pg_steam_news`, `patch_events`, Anreicherungen und `patch_changes`-View | **Ergänzen:** `/v2/patches` für Discovery und direkte Textfälle. **Behalten:** Auflösung vollständiger Originale, historischer Bestand und vorhandene Ereignisparser. RSS und `big-days` ersetzen das Archiv nicht. |
| Item-/Fähigkeitswerte für den Reasoner | Per-Clientversion Rohwerte, Bedingungen, Skalierung und Upgrades | Kataloge, Snapshots, eigener Mechanik-/Combat-Resolver; zusätzliche Spieldateien | **Ersetzen:** wiederholte unversionierte Werteabholung durch einen gepinnten Assets-Import. **Behalten:** Mechanikrechnung und Fail-closed-Verhalten für unbekannte Effekte. API liefert Daten, nicht die fertige Reasoner-Semantik. |
| Genügend echte Nach-Patch-Matches | Analytics zeigt Menge; Metadaten enthält deduplizierbare Einzelfälle samt Kauf-/Skillhistorie | Population-Import, Familienbildung und Publisher-Guard | **Ergänzen:** gezielte, begrenzte Metadatenpagination für den betroffenen Helden ab echtem aktivem Patchbeginn. **Behalten:** Familienprüfung und 100er-Grenze. Keine Aggregate als Pseudomatches. |
| Builds und Spielstile | Build-Suche, Versionen, Detailobjekte, Verbreitung | `tierlist.hero_build_sources`, beobachtete Autoren, Population und eigener Composer/Steam-Publisher | **Ergänzen:** aktuelle öffentliche Buildversionen im bestehenden Quellenweg. **Behalten:** exakte Referenz-Build-ID, Autorenprovenienz und tatsächliche Veröffentlichung über Steam. API-Suche ist weder Buildabnahme noch Publisher. |

## 6. Konkreter Vorschlag: eine Spieldatenbasis im vorhandenen Postgres

Kein neuer Dienst, kein externer Antwortweg und kein DuckDB-/SQLite-Zustand für das Brain. Der externe Data Lake bleibt optionales Rechercheangebot; dauerhafter Zustand bleibt im bestehenden Brain-Postgres.

### Bestehende Tabellen wiederverwenden

| Ebene | Bestehende Tabellen/Views | Konkrete Verwendung |
| --- | --- | --- |
| Rohquelle und technische Versionsgeschichte | `brain.source_runs`, `source_documents`, `entity_snapshots` | Assets pro `client_version`, Sprache, Endpunkt und Parserrevision speichern. Versionsliste und `steam.inf` als normale Quelldokumente. Raw-Hash, URL, erste Beobachtung und HTTP-Status binden. Keine neue API-Schatten-Datenbank. |
| Unveränderliche Wissensrevision | `brain.source_record_revisions`, `source_record_heads`, `source_checkpoints_v1` | Bestehenden validierten Import verwenden. API-Clientversion und Inhaltsrevision getrennt führen. Für unveränderte Werte neue Versionsbindung statt erfundenem Änderungsereignis; Sprachfassungen nicht gegenseitig überschreiben. |
| Domainprojektion für Rechnung | `brain.entities`, `entity_aliases`, `hero_stat_profiles`, `hero_stat_values`, `hero_catalog`, `item_catalog`, `hero_item_stats`, `hero_ability_orders`, `hero_item_synergies` | Projektionen aus derselben gepinnten Quellenrevision erzeugen. Reasoner-Werte und Steckbriefwerte müssen dieselbe Herkunft/Version haben. Statistikzeitraum und Rang im vorhandenen Datensatz bzw. dessen Quellrevision festhalten. |
| Patchtexte und echte Einzelmatches | `brain.patch_events`, `patch_event_enrichments`, `patch_changes` als View; `population_player_matches`, `population_sync_runs` | Patchfeed in bestehenden Patch-Sync einhängen und nach kanonischer Originalquelle deduplizieren. Populationsdaten nur über vorhandenen Cleaner/Import; Abfragefenster und bestätigte Seiten im Run dokumentieren. |
| Veröffentlichte Antwortbasis | `brain.entity_profile_entities_v1`, `entity_profile_facts_v1`, `entity_semantic_projections_v1`, `entity_derived_receipts_v1`, `corpus_releases_v1` | Bestehende Profile und Patch-Story materialisieren, Herkunftsbelege und Ableitungsquittung speichern, danach genau den vorhandenen Korpus veröffentlichen. A aktiviert. |

Eine eigene neue Tabelle ist für diesen ersten Schritt nicht nötig. Falls die Umsetzung zeigt, dass der vorhandene Vertragsdatensatz kein versionsbezogenes Abfragefenster aufnehmen kann, wird eine additive Migration vorgeschlagen und separat abgegrenzt. Keine schon angewandte Migration verändern.

### Patcherkennung und Abholtakt

Die folgenden Intervalle sind **Vorschläge für bestehende Konfigfelder**, keine heute gesetzten Werte oder neuen Timer:

1. **Technische Änderung erkennen:** Versionsliste/`steam-info` und `/v2/patches` im bestehenden Quellenlauf etwa alle fünf Minuten prüfen. Feed-GUID/Inhaltsrevision und `client_version` getrennt erkennen. `big-days`, Titel allein und ein beliebiger neuester Steam-Beitrag sind kein Patch-Cutoff. Ein kosmetischer Post oder eine unveränderte Hero-Payload darf keinen erfundenen Balancepatch erzeugen.
2. **Assets gepinnt abholen:** Bei neuer verfügbarer Clientversion genau einmal vollständige Helden-/Itemdaten für die festgelegten Sprachen laden, danach Vollständigkeit und Schema prüfen. Manifest, Assets, Patchquelle und bekannte Gültigkeit atomar binden. Ein fehlender historischer Datensatz bleibt eine Lücke; niemals den aktuellen Datensatz als alte Version ausgeben. Kontrollabgleich im bestehenden Sechsstundenlauf.
3. **Patchtexte und Geschichte ergänzen:** `/v2/patches` in den vorhandenen Patch-Sync aufnehmen. Volltext übernehmen, wenn tatsächlich vollständig; andernfalls vorhandene offizielle Linkauflösung benutzen. Forum- und Steam-Ausgabe derselben Änderung nur einmal kanonisch parsen. Reine HTML-Vorschauänderungen nicht als neue Spieländerung behandeln. Fehlende Zuordnung zur Spielversion ausdrücklich offen lassen.
4. **Statistik und Einzelfälle trennen:** Analytics im bestehenden Builddatenlauf mit einem expliziten, gespeicherten unteren und oberen Zeitlimit holen; Item-/Synergierefresh etwa alle sechs Stunden passend zum Cache. Metadaten bei einer Datenlücke gezielt für den betroffenen Helden und den echten aktiven Patch abrufen, mit kleinen Seiten, Größenbudget und bestehendem gemeinsamen Rate-Limiter. Kein globaler 2000-Matches-Schnappschuss als Garantie für 100 Fälle einer seltenen Familie. Bei Datenmangel bleibt der Publisher gesperrt.
5. **Veröffentlichung und Wirkung:** Gemeinsame Faktenbindung, Steckbrief- und Patch-Story-Ableitung über `entity_binding`/`entity_derivation`, danach Ableitungsquittung und vorhandene Korpusfreigabe. Erst wenn die veröffentlichten Quellenbindungen den gleichen Patch und dieselben Revisionen zeigen, aktiviert A. E baut oder aktiviert diesen Weg nicht parallel.

### Reihenfolge nach späterer ausdrücklicher Freigabe

**Paket E darf zunächst nur den Datenlieferanten verbessern:** Versionspinning im vorhandenen Assets-Ingest, API-Feedanschluss im vorhandenen Patch-Sync und datumsgebundene Statistikabfragen. Vorher Tabellenzugang und tatsächlich laufende Quellrevision nachweisen. Änderungen an den gemeinsamen Profil-/Veröffentlichungsdateien liegen bei A.

Der gezielte Populationlauf ist eine spätere Betriebsaktion innerhalb des freigegebenen Releasefensters, nicht Bestandteil dieser Recherche. Danach denselben Reasoner, dieselbe Familienpolitik und denselben Publisher-Guard erneut lesen lassen. Die Zahl der gültigen, deduplizierten **Nach-Patch-Fälle je Familie** ist der Abnahmebeleg. Überlebende Mechaniklücken und `Low` können weiter blockieren, auch wenn die Zahl 100 erreicht.

Abnahme für das Grundding bleibt bei A: Haze, Mystic Burst, Bullet Dance und Patch-Story müssen über den bestehenden Brain-Antwortweg mit aktuellen Quellen beantwortet werden. Mehr Einträge in einer DB oder erfolgreiche API-GETs ersetzen diese Abnahme nicht.

## 7. Wichtigste Risiken und Anschlussstellen

1. **Falscher Aktualitätsstempel:** `dbrain-builds/src/api.rs:35-85`, `sync.rs:81`; mehrere Statistikpfade ohne expliziten Patchzeitraum. Zusammen prüfen, nicht nur einen Endpoint korrigieren. Gegenprüfung: der Populationweg hat bereits `min_unix_timestamp` und verarbeitet echte Startzeiten.
2. **Feed ist kein Volltextarchiv:** API-Probe „09-29-2026“ plus vorhandene Auflösung in `pg_patchnotes.rs:278`. Den vorhandenen Resolver behalten; `pg_steam_news` ist der zweite bestehende Anschluss, kein Anlass für einen dritten Parser.
3. **Falsche Grenzzählung:** Tagesaggregate und Metadaten unterscheiden sich; `families/mod.rs:295` und `publish.rs:109` zählen Familienmitglieder. Vorhandene Qualitätsgrenze bleibt verbindlich. UTC-Interpretation im Messwerkzeug korrigiert; PG-Sessionzeitzone noch live zu prüfen.
4. **Quelle gespeichert, Antwort weiter leer:** `entity_derivation.rs:131`, `pg_release.rs:51` und A-G1-Befund. Nur ein aktiver Release mit Quellenbindungen und erfolgreichen Abnahmeantworten belegt Nutzen. E liefert keinen eigenen Korpus und keinen alternativen MCP-Antwortdienst.

WIRKUNGSPRUEFUNG[WP-1]: 4 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 7/7 geprüft

Die sieben geprüften repräsentativen Pfadgruppen sind Assets, Patchfeed, Buildsuche, Analytics, Matchmetadaten, MCP und REST-SQL. Das ist eine Quellen-/Grenzprüfung ohne Implementierungsdiff, keine vollständige Review aller API-Endpunkte oder Betriebspfad-Abnahme.

## Quellen

- [Aktueller OpenAPI-Vertrag](https://api.deadlock-api.com/openapi.json), live gelesen am 07.10.2026.
- [Assets-Versionen](https://api.deadlock-api.com/v1/assets/client-versions) und [Steam-Manifeste](https://api.deadlock-api.com/v1/assets/steam-info/all).
- [Versionierte Helden](https://api.deadlock-api.com/v1/assets/heroes?client_version=6759&only_active=true) und [versionierte Items](https://api.deadlock-api.com/v1/assets/items?client_version=6759).
- [Vereinter Patchfeed](https://api.deadlock-api.com/v2/patches) und [große Patchtage](https://api.deadlock-api.com/v1/patches/big-days).
- [Öffentliche API-Seite](https://deadlock-api.com), [MCP-/Data-Lake-Dokumentation](https://deadlock-api.com/data-dumps), [Manifest](https://data.deadlock-api.com/v1/manifest.json) und [MIT-Softwarelizenz](https://github.com/deadlock-api/deadlock-api/blob/master/LICENSE).

**Status:** Analyse abgeschlossen. Umsetzung nicht freigegeben. E bleibt stehen, bis `VON_HAUPT.md` Paket E ausdrücklich freigibt. Release-Hold und A-Eigentum an der Korpusaktivierung gelten unverändert.
