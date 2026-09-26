# Legacy nach Kernmodell: Inventar, Einordnung und Import

Stand: 26.09.2026, Branch `integration/pre-g5-review-20260926` auf Basis `migration/rust-integration` `d6cf9bc`.
Quelle ist ausschließlich das Archiv `brain_legacy` in der eigenen Brain-Instanz (Snapshot `legacy-import-20260926T025758Z`, 49 Tabellen, 653 476 Zeilen, siehe `BRAIN_DB_MIGRATION_REPORT.md`). DL-Main wurde nicht gelesen. `brain_legacy` bleibt Archiv und Importquelle; der Kern liest es zur Laufzeit nie.

## Kategorien

1. muss in den kanonischen Kern-Store
2. wird künftig über eine externe Quelle oder API neu bezogen
3. analytischer oder abgeleiteter Cache, wird nicht migriert
4. historisches Archiv, keine Laufzeitquelle
5. gehört einem anderen Dienst

## Alle 49 Tabellen

| Tabelle | Zeilen | Kat. | Begründung und künftiger Weg |
|---|---:|:-:|---|
| `patch_events` | 32 821 | 1 | Primäre Patchnotes-Zeilen (Valve-Steam-Ankündigungen, Forum-Spiegel). Kernquelle `legacy-patchnotes` |
| `patch_event_enrichments` | 32 821 | 1 | Strukturierte Wertänderung je Zeile (Stat, alt, neu, Einheit). Im selben Patch-Dokument |
| `entities` | 905 | 1 | Kanonische Entitäten aus der Deadlock-Assets-API. Kernquelle `legacy-entities` |
| `entity_aliases` | 3 778 | 1 | Aliase je Entität. Im selben Entitäts-Dokument |
| `hero_catalog` | 38 | 2 | Von Deadlock-Bots `api_ingest` geschrieben. Künftig `brain-feeds::deadlock_assets` direkt aus der Deadlock-API |
| `item_catalog` | 251 | 2 | wie `hero_catalog` |
| `current_entity_state` | 5 804 | 2 | Aus Snapshots abgeleiteter Aktuellstand. Künftig Assets-Adapter in den Kern |
| `sheet_items` | 4 478 | 2 | Google-Sheet-Spiegel. Adapter `google_sheet` existiert nur für den Altstore; Kernanbindung und Lizenzfrage offen |
| `sheet_tab_rows` | 2 410 | 2 | wie `sheet_items` |
| `sheet_raw_heroes` | 419 | 2 | wie `sheet_items` |
| `sheet_heroes_stats` | 396 | 2 | wie `sheet_items` |
| `sheet_boons_ap` | 90 | 2 | wie `sheet_items` |
| `sheet_hero_rankings` | 83 | 2 | wie `sheet_items` (Community-Einschätzung, keine Spielwerte) |
| `sheet_shop_bonuses` | 41 | 2 | wie `sheet_items` |
| `hero_stat_profiles` | 402 | 2 | Aus dem Sheet abgeleitete Heldenprofile, folgt dem Sheet-Adapter |
| `hero_stat_values` | 9 779 | 2 | wie `hero_stat_profiles` |
| `youtube_videos` | 195 | 2 | YouTube-Metadaten. Neu beziehen; Rechte für Transkripte offen |
| `youtube_transcripts` | 18 | 2 | Drittinhalte, Lizenz unbekannt. Nicht in den Kern ohne Rechteentscheidung |
| `youtube_feed_sources` | 13 | 2 | Konfiguration des YouTube-Jobs, gehört in dessen Config |
| `knowledge_events` | 26 685 | 3 | Abgeleitet aus Patch-Events und Forum-Claims (`source_table`), im Kern aus Patch-Dokumenten rekonstruierbar |
| `insight_records` | 51 | 3 | Modellabgeleitete Einsichten |
| `patch_impact_notes` | 883 | 3 | Modellausgaben (`model`, `prompt_text`) |
| `meta_trend_notes` | 124 | 3 | Modellausgaben |
| `legacy_entities` | 68 | 3 | Heuristische Namensaggregation aus Patch-Events |
| `entity_lineage` | 810 | 3 | Heuristische Umbenennungs-/Besitzbeziehungen aus Patch-Events |
| `hero_item_stats` | 18 982 | 3 | Aggregat aus Deadlock-API-Matchdaten; Deadlock API bleibt Quelle |
| `hero_item_synergies` | 282 132 | 3 | wie `hero_item_stats` |
| `hero_ability_orders` | 114 | 3 | wie `hero_item_stats` |
| `population_player_matches` | 167 414 | 3 | Kopie von Deadlock-API-Matchdaten. Kein Duplikat des Analysebestands |
| `population_item_stats` | 12 781 | 3 | Aggregat aus `population_player_matches` |
| `population_ability_order` | 1 305 | 3 | wie `population_item_stats` |
| `population_imbue_stats` | 748 | 3 | wie `population_item_stats` |
| `population_hero_buckets` | 87 | 3 | wie `population_item_stats` |
| `reasoner_builds` | 1 | 3 | Rechenergebnis des Build-Reasoners |
| `reasoner_item_scores` | 173 | 3 | wie `reasoner_builds` |
| `reasoner_backtests` | 4 | 3 | wie `reasoner_builds` |
| `reasoner_patch_deltas` | 540 | 3 | wie `reasoner_builds` |
| `learned_builds` | 0 | 3 | leer |
| `build_learning_notes` | 0 | 3 | leer |
| `analysis_notes` | 0 | 3 | leer |
| `mechanic_notes` | 0 | 3 | leer |
| `youtube_learning_claims` | 0 | 3 | leer |
| `entity_snapshots` | 37 123 | 4 | Rohsnapshot-Historie der Assets-API. Aktueller Stand kommt künftig direkt aus der API |
| `forum_claims` | 831 | 4 | Im Altbestand selbst als `historical_quarantine` markiert, enthält Forennamen. Kein Laufzeitwissen |
| `source_documents` | 7 308 | 4 | Index auf Rohdateien außerhalb der DB (`raw_path`), Herkunftsnachweis des Altpfads |
| `source_runs` | 558 | 4 | Laufprotokoll |
| `population_sync_runs` | 4 | 4 | Laufprotokoll |
| `youtube_transcript_claim_attempts` | 0 | 4 | leer, Laufprotokoll |
| `player_match_decision_notes` | 8 | 4 | Enthält Account-IDs einzelner Spieler; kein Kernwissen, nicht exportieren |

Summe: Kategorie 1 vier Tabellen, 2 fünfzehn, 3 dreiundzwanzig, 4 sieben, 5 keine. Die Deadlock-Bots-Tabellen `feeder_runs` und `plan_*` sowie das Schema `knowledge` wurden nie kopiert und gehören Deadlock-Bots (Kategorie 5, nicht im Archiv).

## Abbildung der Kategorie 1

| Feld | `legacy-patchnotes` | `legacy-entities` |
|---|---|---|
| `source_id` | `legacy-patchnotes` | `legacy-entities` |
| `logical_id` | `patch/<patch_external_id>` | `entity/<entity_type>/<canonical_name>` (905 eindeutig) |
| Inhalt | Kopfzeilen je unterschiedlicher Titel/URL/Zeit-Kombination, dann alle Zeilen nach `line_index`, `id`, Wertänderung aus `patch_event_enrichments` angehängt | Typ und Name, External-ID, Quelle, Aliase, alle skalaren Metadaten sortiert |
| `kind` | `prose` (Patchnotes sind Fließtext; ein 35-KB-Patch ist kein atomarer Fakt) | `fact` |
| `revision` | monoton je Dokument, 1 beim Erstimport, +1 bei Inhalts- oder Policyänderung | wie links |
| Content-Hash | SHA-256 des Inhalts, gleich `raw_sha256` der Origin | wie links |
| Provenienz | `brain.origin` (versioniert): `SourceRevision::Api{brain_legacy.archive.v1, Snapshot}`, Locator Patch-URL, Parser `brain-legacy-import.v1`, Schema-Hash der vier Archivtabellen, `retrieved_at` = Snapshotzeit, `source_time` = `posted_at` nur wenn eindeutig, Sprache nur wenn eindeutig, `origin_artifacts` = Patch-URLs | wie links, `origin_artifacts` = `deadlock_assets_api:<external_id>`, `source_time` und Sprache unknown |
| Sichtbarkeit und Scopes | aus der Import-Config, Pilot: `public`, Scope `game.public` | wie links |
| Rechte | Lizenz und Autorisierung **unknown**; `publication_allowed=false`, `provider_egress_allowed=false` im Pilot, `raw_retention_allowed=true` (Archiv liegt bereits vor) | wie links |
| Patch-Gültigkeit | `validity.patch` = Patch-ID des Dokuments, Metadatum `legacy_patch`; bewusst **kein** Retrieval-Metadatum `patch`, weil das Dokument dann nur in genau diesem Release gälte | unknown |
| Modus-Gültigkeit | unknown | unknown |
| `valid_from`/`valid_to` | unknown (Veröffentlichungszeit ist keine Spielgültigkeit) | unknown |
| Tombstones | Fehlt ein Dokument in einem späteren Import, entsteht ein Tombstone mit Revision +1; ein leerer Archivlesevorgang bricht ab statt alles zu löschen | wie links |
| Domain-Darstellung | keine; eine geprüfte Domain-Projektion (NumericFact je Wertänderung) braucht Review wie im Wiki-Pfad | Fakt-Dokument für Alias- und Katalogfragen |

## Importpfad

`brain_legacy` (Lesen als `brain_readonly`, eine `REPEATABLE READ READ ONLY`-Transaktion) → `brain-legacy-import` → `SourceRecordV2` mit versionierter Origin → `DocumentStorePort` (Checkpoint, Lease, atomarer Batch) als `brain_ingest` → `CorpusRelease` mit inhaltsadressierter ID → `brain-serve`.

- Code: `rust/crates/brain-legacy-import`, generische Revisionierung `rust/crates/brain-ingestion/src/document_set.rs`.
- Config: `ops/brain-postgres/legacy-core-import.json` (Ziel nur `brain_pilot*`, nur Unix-Socket, TCP wird abgewiesen).
- Echtlauf: `scripts/run_isolated_legacy_import.sh`, Bericht `~/.local/share/deadlock-brain/legacy-core-import-20260926/`.

## Ergebnis des Echtlaufs (26.09.2026, 34 von 34 Prüfungen)

| Prüfung | Ergebnis |
|---|---|
| Dokumente | 1 253 (905 Entitäten, 348 Patches) aus 70 325 Archivzeilen (32 821 Patch-Zeilen mit 32 821 Anreicherungen, 905 Entitäten, 3 778 Aliase) |
| Release | `legacy-core-f07ea85c09010285`, Snapshot-Digest `3051f2c4eef4874c9b769fdf540a28a31627ed59ccdb2742647ba5d4626ac135` |
| Wiederholter Import | 0 neue Records, gleiche Release-ID, gleicher Digest; auch nach dem Refactor auf `document_set` identisch |
| Dubletten | 0 je (Quelle, Dokument, Revision) |
| Hashgleichheit | 0 Abweichungen zwischen `content_hash` und SHA-256 des gespeicherten Inhalts |
| Provenienz | jeder Record trägt `brain.origin`; Lizenz, Autorisierung und Modus unknown |
| Serve, Fakt | Entitäts-Fakt und Alias (`hero_warden`) beantwortet mit Zitat |
| Serve, Unknown | unbekannter Begriff `insufficient_evidence` |
| ACL | fremder Scope sieht nichts, nicht erteilter Scope 403 |
| Patch/Modus | falscher Patch und unbekannter Modus nicht beantwortet |
| Egress | ohne Egress-Freigabe `unauthorized_evidence`, kein Provideraufruf |
| Policy-Revision | Egress-Freigabe nur für Patchnotes: 348 neue Revisionen, Entitäten 0, neuer Release `legacy-core-6c158962d92e8151`; Explain über Patch-Prosa mit Zitat über eine Loopback-Fixture (kein echter Provider) |
| ACL-Revoke | Head auf privat mit Scope `brain.legacy.review`: gepinnter Release liefert die Patchnotes nicht mehr, keine widerrufene Evidenz an die Fixture; Entitäten unberührt |
| Archiv | `brain_legacy` unverändert, produktive DB `brain` ohne Kernrecords |

Tombstones sind am Archiv nicht auslösbar, ohne es zu verändern; sie sind in `brain-legacy-import`, `brain-ingestion` und `brain-feeds` per Unit-Test mit `MemoryRepository` belegt.

## Offen vor G3

- Kategorie 2 braucht Kernanbindungen: Assets-API ist gebaut (`brain-feeds::deadlock_assets`), Google-Sheet und YouTube nicht. Heldenwerte aus dem Sheet stehen deshalb noch nicht im Kern.
- Der Import in die produktive DB `brain` ist Teil von G5 und wurde nicht ausgeführt.
- Befund Retrieval: Das Fakt-Profil beantwortet eine Frage schon bei einem einzigen gemeinsamen Term (erster Lauf: „zzqxv nonexistent entity qqzzv" traf über den Logical-ID-Term `entity` ein beliebiges Entitäts-Dokument). Eine Relevanzschwelle fehlt im Kernel; das ist ein G2-Qualitätspunkt, kein Importfehler.
