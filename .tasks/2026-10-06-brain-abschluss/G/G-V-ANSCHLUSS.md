# G-V: Tatsächliche Anschlüsse und Restlieferungen

status: lesender Vorcheck abgeschlossen, 07.10.2026; kein Produktbau oder Review

Task `wvxlh36sq`, Run `wf_b7f373c0-174`, Agent `ae89f0517cd654593`. Rückgabe tatsächlich übernommen. E ausschließlich am Commit `8107417036228c707582c76e9ffccdeb1b45cb90`, F am Commit `46fd86743589910d7b92a7223bdd6ab0dcf2b7c8`, gesicherter G-Produktbestand `ce21a457`. Aktencheckpoint `5a442391` enthält nur Dokumentänderungen. Gespeicherter origin/main beim Vorcheck `f6f5cef6`; E/F waren dort keine Vorfahren, beide Ancestor-Prüfungen Exit 1. Kein Fetch dieses Workers und keine Aussage über späteren Remotezustand. Graphify zuerst, anschließend gefundene Quellen und Nebenpfade geprüft. Keine fremde uncommittierte Arbeit übernommen.

## 1. E: ein Leser, aber noch kein gebundener Receipt

`brain-storage/src/asset_mirror.rs:5,21` bietet:

```rust
pub async fn latest_mirrored_client_version(pool: &PgPool) -> anyhow::Result<i64>;
pub async fn load_mirrored_assets(
    pool: &PgPool,
    client_version: i64,
    kind: &str,
    language: &str,
) -> anyhow::Result<serde_json::Value>;
```

Der bestehende Leser prüft vollständigen erfolgreichen Assets-Run, Version, Endpoint-Dokument, Art, Sprache und Validierung, liefert aber nur den Payload. Jeder Aufruf wählt erneut den jüngsten passenden Run derselben Version.

Produzentenbestand: `dbrain-sources/src/assets_api.rs:175-187` und `store.rs:131-198` erhalten Endpoint-URL, Originalbytes, Rohhash, Dokument-/Manifest-ID, Parserrevision sowie Spiegel-/Prüfzeiten. Fs private `MirrorProvenance` in `data.rs:714-800` stammt aus einer getrennten neuesten Runabfrage und liefert nur Version/Zeiten. Builds-Sync, interne SourceStore-Produzenten und LocalPgReader-Gitreceipts sind kein kompatibler Ersatz für diesen Assets-Zugriff.

**Restlieferung E:** im selben `asset_mirror`-Modul einen kompatiblen Zugang liefern, der tatsächlich gelesenen Payload und dessen vollständigen Run mit Manifest-/Endpoint-Dokument gemeinsam bindet. C1-Felder einschließlich Run-ID, Dokument-ID, URL, Originalhash und Zeiten. Kein separat abgefragter jüngster Receipt. Originalhash ist der Hash der Rohbytes, nicht eines nachträglich serialisierten Value. Bestehende Value-Aufrufer kompatibel erhalten; G baut keinen zweiten SQL-/Rohdatenleser.

## 2. E: globale Mechanikdaten

`items`, `heroes`, `heroes_all` sind in beiden Sprachen versioniert und gemeinsam lesbar. Der vorhandene `npc_units`-Import in `assets_api.rs:17-28,191-199` ist dagegen außerhalb DEFAULT_KINDS, ohne Versions-/Sprachparameter und ohne gemeinsamen Spiegelzugang. `generic-data`, `misc-entities`, `modifiers` fehlen im gemeinsamen Adapter-/Leservertrag. Weitere ungepinnte Arten sind keine G-Wertequelle.

Der E-Originalpin `openapi-20261007.json` belegt Version und Sprache für `generic-data`, `npc-units`, `misc-entities`, ausschließlich Version für `modifiers`. Restlieferung im bestehenden Adapter, Manifest und Leser; Modifiers ausdrücklich sprachunabhängig binden. Abdeckung pro Rechnung prüfen, bisherige vollständige Sechs-Endpoint-Spiegel nicht rückwirkend verwerfen. Schemata allein belegen keine vollständige Kill-/Comeback-/Urnformel.

## 3. F: reiner BuildObject-Eingang bereits vorhanden

`dbrain-reasoner/src/lib.rs:99,110,160-192` besitzt `plan_build` und `plan_build_with_playstyle`. Beide nehmen geladene Hero-/Itemmodelle, MetaIndexWithSources, Ereignisse, PatchSnapshots und ReasonerConfig entgegen; letzterer zusätzlich `requested_playstyle: Option<&str>`. Rückgabe `PlannedBuild` mit öffentlichem `build`, `scored`, `variant_scores`, `hero`, `deltas`.

Spielstil wird tatsächlich angewandt. `playstyle.rs:16-63` erlaubt `weapon`, `spirit`, `tank` und weist unbekannte Werte ab. Der synchrone Eingang ruft keinen Modellloader, AI, Persistenz, Analytics-Netzwerk oder Publish auf. Öffentliche Composer-Eingänge bestehen in `composer.rs:350,483`; `compose_build_with_author_evidence` ist privat. Kein neuer Planer oder Composer nötig.

**Begrenzte Restlieferung F:** vorhandenen Eingang um tatsächlich angewandtes Toolbudget und angefragte Imbues ergänzen, ursprünglichen Requestabbruch im Planer beachten und vorhandene PurchasePlan-/InventoryEvaluation-Belege für strukturierte Abdeckung behalten. Keine erneute Planung zum Wiedergewinnen verworfener Auswertungen. Gs Wachstum im selben Kern anbinden. Der gelesene reine Pfad erzeugt derzeit keine Varianten; keine vollständige Variantenlieferung behaupten. Alte Retrievalstrecke `dbrain-retrieval/src/lib.rs:1318-1358` meldet `playstyle_applied=false` und ist kein Ersatz.

## 4. G-M und G-K: erst abgeschlossene Verträge konsumieren

Aktiver G-M-WIP besitzt bereits reine Payloadkonverter sowie `calculate_hero_with_deadline`, `project_hero`, `hero_growth` und `compare_hero_curves`. ModelSource bindet Version, Dokument, URL, Art, Sprache und JSON-Pointer, ersetzt aber weder E-Receipt noch Freigabe. Noch kein abgeschlossener Rechenvertrag daraus abgeleitet.

G-K-WIP besitzt GameContextResolver mit resolve/validate sowie Kernel::with_tools. Gesicherter ToolExecutionPort hat definitions/execute/validate_dependencies. Aktuelle Fehlerabrechnungsfortsetzung bleibt exklusiv bei G-K-R1. Kein neuer Toolport nötig.

## 5. Herkunft und Freigabe

Bestehender Anschluss: `brain-storage/src/entity_profile.rs:27,467,587`, `local_pg_reader.rs:717,1153,1208`, `brain-contracts/src/store.rs:26,260,361`. Historische und aktuelle Quellenrechte, Veröffentlichung und Providerweitergabe sind getrennte Prüfungen unter ursprünglicher Deadline.

Die heutige Profilprojektion akzeptiert Wiki/GameFile; Es Assets-Metadaten entsprechen nicht ihrem Originaldokumentformat. Nötig sind echte Assets-Herkunft im Profilvertrag und ein geprüfter Anschluss der Originaldokumente an dieselben Freigabeprüfungen. Receipt/Pointer/Mechanikrevision erteilen keine Freigabe. Keine Assets als Wiki oder Spieldatei etikettieren, keine synthetisch behauptete Dokumentfreigabe. Identitätspfad liefert genau eine freigegebene Textentität, noch keine typisierte Kandidatenliste für entity_find.

## 6. Analytics: vorhandenen Runtimeclient erweitern

`analytics_runtime.rs:53,144,194,363` besitzt DeadlockAnalyticsClient, requestgebundene lookup_with_request_deadline und prepare_analytics_response. Meta/Population verwenden beide hero-stats; Population ergänzt nur Itemfilter. Request hat keinen Rangfilter, Parser behält höchstens einen angefragten Helden. Herkunft trägt stundenangepasstes Fenster, Roh-/Schemahash und Versuchszahl; Patchmitgliedschaft Unverified, kein DB-Schreibpfad.

Nebenpfad `dbrain-builds/src/api.rs:40-74` hat interne item_stats/hero_stats mit min_average_badge, aber nicht den Runtimevertrag. Kein zweiter G-Connector. Originalpin belegt Badgegrenzen 0 bis 116; Hero-/Itemantworten liefern keine fertige Pickrate. Gemeinsamen Nenner ausdrücklich belegen.

`brain-serve/src/analytics.rs:269-274,472-492` verlangt heute analytics.internal und verweigert Modellweitergabe. Begrenzte Runtimeerweiterung für Itemaggregate, Rangfilter und gemeinsame Population sowie ausdrücklich freigegebener öffentlicher Toolbeleg nötig. Keine stille Rechteerweiterung. Zusätzliche Dateigrenze analytics.rs vor Änderung melden.

## 7. Verdrahtung

`brain-serve/src/service.rs:418-445` verdrahtet ReleaseRetriever, AnalyticsRetriever, DiscordRetriever, Kernel, CachedKernel, ApiService. `discord_live.rs:468,551,574` bietet retrieve_with_usage, validate_evidence und validate_publication für bestehende öffentliche Serverfakten; bereits geladene Fakten wiederverwenden.

Serving-AnswerProvider delegiert bisher nur answer. Toolturns und neue Fehlerabrechnung müssen zum selben konkreten Provider weiterreichen. Serve besitzt synchronen LocalPgReader, keinen produktiv verdrahteten sqlx::PgPool. Es async Leser braucht den begrenzten Lesepoolanschluss innerhalb vorhandener Runtime und PostgreSQL-Konfiguration. Erforderliche Reasoner-/Sources-/Retrieval-/SQLx-Crateabhängigkeiten sind bereits vorhanden.

Kein Compiler, DB, Liveprovider, Runtime oder Git-Schreibschritt durch den Vorcheck. Keine Reviewerrolle oder ALLOW-Aussage.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/dbrain-reasoner/src/lib.rs:110 | Anknüpfung: vorhandener reiner BuildObject-Eingang, gemeinsamer Assets-Leser, Runtime-Analytics und bestehende Freigabe-/Serverwissenswege
