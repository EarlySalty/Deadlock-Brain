status: aktiv
Datum: 2026-10-03
Rolle: nativer Bau-Worker, Teilbereich A, Versuch 1
Vertrag: wiki-spielwissen-v1

# Rust-Wikiinventar: Bauübergabe

Worktree: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`
Branch: `feat/brain-wiki-spielwissen-a`
Ausgangs-SHA: `2734c2da4e814ff79953e8e825275b0216a6af16`

Der neue Modulstand ist geschrieben und für die unabhängige Prüfung eingefroren. Er ist noch nicht kompiliert oder durch ausgeführte Tests bestätigt. Kein Commit, Push, Merge oder Deploy durch den Bau-Worker.

## Geänderte eigene Dateien

- `rust/crates/dbrain-sources/src/wiki_inventory.rs`: öffentliche Konfiguration, Namespace-Durchlauf, Fortsetzung, Offline-Einstieg und Zugriffssperre.
- `rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs`: API-v1/v2- und XML-Normalisierung, Provenienz, Seitentypen, historische Inhalte und vorsichtige JSON-Wertextraktion.
- `rust/crates/dbrain-sources/src/wiki_inventory/storage.rs`: unveränderliche Revisionsdateien, Konflikte, Checkpoint und atomare JSONL-Veröffentlichung.
- `rust/crates/dbrain-sources/src/wiki_inventory/tests.rs`: elf gezielte Unit-Tests ohne reale Netzabrufe.

Keine vorhandene Rust-Datei, kein Cargo-Manifest, kein Lockfile und kein Schema wurden verändert. Die Registrierung `pub mod wiki_inventory;` gehört weiterhin C. Neue Abhängigkeiten sind nicht erforderlich.

## Öffentliche Schnittstelle

Alle folgenden Funktionen sind synchron und liefern `crate::Result<T>`. Asynchrone Aufrufer müssen die bestehenden Grenzen für blockierende Arbeit verwenden.

```rust
pub fn collect_wiki_inventory(
    work_dir: &Path,
    http: &deadlock_brain_core::http::HttpClient,
    options: &WikiInventoryOptions,
) -> Result<WikiInventoryReport>;

pub fn collect_wiki_inventory_with_transport(
    work_dir: &Path,
    options: &WikiInventoryOptions,
    get: impl FnMut(&[(String, String)]) -> Result<serde_json::Value>,
) -> Result<WikiInventoryReport>;

pub fn write_wiki_api_capture(
    work_dir: &Path,
    payload: &serde_json::Value,
    context: &WikiSourceContext,
    options: &WikiInventoryOptions,
) -> Result<WikiInventoryReport>;

pub fn write_wiki_export_capture(
    work_dir: &Path,
    xml: &str,
    context: &WikiSourceContext,
    options: &WikiInventoryOptions,
) -> Result<WikiInventoryReport>;

pub fn read_wiki_inventory_report(
    work_dir: &Path,
    max_total_bytes: usize,
) -> Result<WikiInventoryReport>;
```

`read_wiki_inventory_report` liest den erhaltenen Checkpoint und erstellt die abgeleiteten Ausgabedateien erneut. Der Aufruf ist deshalb kein rein lesender Dateizugriff.

Zusätzlich öffentlich:

```rust
WikiSourceContext::from_siteinfo(&serde_json::Value) -> Result<WikiSourceContext>
normalize_api_response(&Value, &WikiSourceContext, &str) -> Result<WikiNormalization>
normalize_mediawiki_export(&str, &WikiSourceContext, &str) -> Result<WikiNormalization>
```

`WikiNormalization` enthält `documents: Vec<Value>`, `pages: Vec<WikiInventoryPage>` und `gaps: Vec<WikiGap>`.

`WikiInventoryOptions` hat die Felder `enabled`, `access_policy_reviewed`, `observed_at`, `refresh_inventory`, `clear_access_block`, `min_delay_seconds`, `timeout_seconds`, `max_pages`, `max_response_bytes`, `max_total_bytes` und `batch_size`. Standardmäßig sind Netzwerk und Zugriffsfreigabe aus. `observed_at` muss ausdrücklich als UTC-Zeit nach RFC 3339 angegeben werden. Es gibt keine Konfiguration über ENV-Dateien.

Ein regulärer späterer Netzlauf verwendet ausschließlich `https://deadlock.wiki/api.php`, den vorhandenen `HttpClient::get_no_redirect`, einen Abrufversuch, mindestens fünf Sekunden Abstand und `maxlag=5`. Er fordert die echten nichtnegativen Namespaces aus `siteinfo` an und erhält Redirects mit `gapfilterredir=all`. API-Fortsetzungen werden einschließlich `continue`, `gapcontinue` und gegebenenfalls `rvcontinue` gespeichert. Virtuelle Namespaces werden nicht als Seitenlisten behandelt.

Ein vorhandener abgeschlossener Checkpoint verursacht bei unveränderten Optionen keinen neuen Abruf. `refresh_inventory=true` beginnt einen neuen Inventardurchlauf und erhält die gespeicherten Dokumentrevisionen. Für Wiederaufnahme danach `refresh_inventory=false` verwenden. HTTP 401/403, explizite API-Zugriffsfehler und HTML statt JSON halten den Zugang an; keine automatische Umgehung oder Wiederholung. `clear_access_block` ist ausschließlich für einen später erneut geprüften und ausdrücklich freigegebenen Zugang bestimmt.

## Aufruf für die zwölf echten Offline-Seiten

Eingaben liegen außerhalb Git:

- Rohseiten: `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/legacy-raw/`
- Tatsächliche `siteinfo`-Antwort: `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/source-evidence/wiki-siteinfo.json`
- Die originalen Abrufzeiten stehen in den im Recherchebericht benannten Cache-Sidecars.

Der Datenausführer liest die echte `siteinfo`-Antwort und erstellt einmal den Grundkontext mit `WikiSourceContext::from_siteinfo`. Für jede Rohseite wird dieser Kontext geklont:

```rust
context.license_observed_at = Some("2026-10-03T03:18:42Z".into());
context.source_fetched_at = Some(actual_cache_fetch_time_utc);
context.source_capture_sha256 = Some(original_raw_file_sha256);
context.historical_capture = true;

let options = WikiInventoryOptions {
    observed_at: actual_offline_observation_time_utc,
    ..Default::default()
};
let report = write_wiki_api_capture(output_dir, &raw_payload, &context, &options)?;
```

Der Offline-Einstieg braucht keine Netzfreigabe. `source_fetched_at` kommt aus dem echten Sidecar und darf nicht durch das heutige Datum ersetzt werden. Revisionszeit, früherer Abruf, heutige lokale Beobachtung und heutiger allgemeiner Lizenznachweis bleiben getrennt.

Das tatsächliche Altformat `query.pages` als Objekt und `revisions[].slots.main["*"]` wird unterstützt. `content` ist exakt der übergebene Revisions-Wikitext. Der Vertrags-Hash wird über dessen UTF-8-Bytes berechnet. Der zusätzliche historische Klartextauszug bleibt mit eigenem Hash unter `metadata.rendered_extract` erhalten; daraus entstehen keine unbedingten aktuellen Zahlenfakten. Die drei `query.redirects`-Zuordnungen bleiben in `metadata.query_redirect_mappings`; fehlende eigene Redirect-Seiten- und Revisions-IDs werden als Lücke erfasst.

Ausgabe im angegebenen Verzeichnis: `documents.jsonl`, `inventory.json`, `checkpoint.json`, unveränderliche Einzelrevisionen unter `documents/` und Konfliktaufzeichnungen unter `conflicts/`. Diese Dateien bleiben außerhalb Git. Wiederholung derselben ID, Revision und desselben Inhalts erhält die zuerst gespeicherte Beobachtung. Eine weitere Revision bleibt daneben erhalten; gleicher Versionsschlüssel mit anderem Inhalt überschreibt nichts.

## Inhalt und Grenzen

Namespace-Namen stammen aus der echten Quelle. Data=3002 und Update=3000 werden nicht aus einer synthetischen Fixture vertauscht. Bucket wird als eigener JSON-Datenbereich erfasst. Template-, Module-, Category-, Redirect- und Dateibeschreibungsseiten bleiben als solche erhalten. Medien-Binärdateien werden nicht gesammelt.

Strukturierte Fakten entstehen ausschließlich aus direkt vorhandenen skalaren JSON-Werten in Data-/Bucket-Seiten mit belegtem JSON-Inhaltsmodell. Der Fakt speichert den JSON-Pfad, den unveränderten Wert und die Quellrevision. Einheiten werden nicht geraten. Prosa, Lua und Wikitext werden nicht ausgeführt oder zu unbewiesenen Spielwerten verrechnet. Bedingungen und uninterpretierte Strukturen bleiben im Originalinhalt erhalten. Historische Quellen sind als historisch markiert; eine Artikelrevision ist keine bestätigte aktuelle Spielversion.

Template-/Modulabhängigkeiten werden syntaktisch erfasst, aber weder vollständig semantisch aufgelöst noch an eigene Revisionen gepinnt. Das bleibt in den Dokumentmetadaten sichtbar. XML-Exporte erhalten alle enthaltenen Revisionen, beweisen aber allein kein vollständiges Wiki-Inventar.

Die allgemeine Lizenz ist aus `siteinfo` belegt. `redistribution_allowed=false` bleibt als vorsichtige Grenze erhalten. Seiten-/Asset-Ausnahmen und Rechte der historischen Revisionen sind dadurch nicht bestätigt. Der Hash `source_siteinfo_sha256` bezieht sich auf die kompakte `serde_json`-Darstellung des eingelesenen JSON-Werts; `metadata.source_siteinfo_hash_representation` benennt das ausdrücklich. Der SHA-256 der Original-Rohdatei bleibt separat in `source_capture_sha256`.

Die aktuelle Wiki-Sperre wurde vom Recherche-Worker belegt. Der Bau-Worker hat keine Wiki-Netzabrufe durchgeführt. Es gibt keinen belegten vollständigen oder aktuellen Live-Durchlauf. Ein Offline-Inventar erhält deshalb `inventory_complete=false` und `content_complete=false`; Wiki-Statistikzahlen werden nicht als tatsächliches Namespace-Seiteninventar ausgegeben.

## Prüfstand und konkrete Hinweise für den frischen Prüfer

`rustfmt` lief zunächst erfolgreich auf genau den vier eigenen Dateien. Danach wurden noch HTML-Zugriffssperre, Lizenz-Beobachtungszeit und die Prüfung neuer IDs pro vollständigem Batch ergänzt. Ein abschließender Formatcheck für diesen eingefrorenen Stand steht aus. Cargo, Clippy und Unit-Tests wurden wegen der fehlenden C-Modulregistrierung ausdrücklich nicht gestartet. Die echte Offline-Ausführung und Wiederholungsprobe gehören zum getrennten Datenausführer.

Die elf geschriebenen Tests behandeln API-v1-Provenienz, Namespace-Fortsetzung und Wiederaufnahme, idempotente Wiederholung, erhaltene neue Revisionen, Inhaltskonflikte, Data/Update/Bucket, unaufgelöste Abhängigkeiten, unterdrückte oder unbekannte Revisionen, XML-Historie, abgeschalteten Zugang und fehlerhafte Eingaben sowie zyklische Fortsetzung und Zugriffssperre.

Zwei beim lokalen Lesen erkannte Punkte sind für die unabhängige Prüfung offen und wurden dem Teil-Orchestrator gemeldet:

1. Bei unterdrücktem oder fehlendem Revisionstext bleibt eine bereits bekannte `revid` derzeit nicht im Inventareintrag erhalten. Der Inhalt wird sicher nicht ausgegeben; der Versionsvermerk des Inventars braucht eine Prüfung beziehungsweise Korrektur.
2. Eine später separat importierte ältere Offline-Aufzeichnung kann den aktuellen Inventareintrag auf eine ältere Revision setzen. Die unveränderlichen Dokumentrevisionen bleiben erhalten, doch die Auswahl der neuesten Inventarrevision braucht eine Prüfung beziehungsweise Korrektur.

Der Code wurde nach der Freeze-Meldung nicht weiterentwickelt. Eine unmittelbar zuvor begonnene Umordnung wurde auf den vollständigen bisherigen Stand zurückgestellt, damit keine undefinierte Variable stehen bleibt. Die unabhängige Prüfung entscheidet über erforderliche Fixes. Gebaut als Quellstand: ja. Kompiliert, ausgeführte Tests, unabhängig geprüft, gemergt und live: nein.
