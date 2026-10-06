# Fix A, Runde 1: eingefrorener Modulstand

Datum: 03.10.2026
Auftraggeber: Teil-Orchestrator A, f01cce67-209b-468e-8abb-ec2070beeaa2
Worktree: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`
Branch: `feat/brain-wiki-spielwissen-a`
HEAD unverändert: `2734c2da4e814ff79953e8e825275b0216a6af16`

Die Änderungen erfolgten erst nach der ausdrücklichen SCHREIBFREIGABE. Der neue Quellstand ist jetzt eingefroren und kann vom Datenworker kompiliert und gegen die vollständigen Archive ausgeführt werden. Keine eigenen laufenden Cargo-Aufgaben, Compiler, Wrapper oder Hostlock-FDs. Keine weiteren Moduländerungen während seines Prüflaufs.

Kompilierung bestätigt: N. Ausgeführte Regressionen bestätigt: N. Vollständige Echtdatenverarbeitung bestätigt: N. Grünes Gesamturteil: N.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft

Der verbleibende Befund ist die zentrale HTTP-Bodybegrenzung bei C. Die lokalen Revisions-/Inventarfehler und der Checkpoint-Zwilling sind im Quellstand korrigiert; ihre Laufzeitnachweise stehen noch aus. Die zwei vorhandenen Netzpfade sind statisch geprüft, ohne neue Netzrequests.

## Gelesene Grundlagen und Eigentum

Gelesen: zentraler AUFTRAG.md, PAKETE.md, CONTRACT.md und AN_BEREICHE.md bis einschließlich Punkt 16; im eigenen Bereich REVIEW-LOCAL-1.md, BAU.md und DUMPQUELLEN.md. Die vier Module wurden vollständig gelesen. Vor Codefragen wurde `code-suche` geladen und Graphify global befragt, vor der abschließenden gezielten Zwillingssuche nochmals.

Geändert wurden ausschließlich die vier unten genannten A-Moduldateien und dieser Bericht. Keine Änderungen an Core, bestehenden lib.rs-, Cargo-, Lockfile-, Schema- oder Datenbankdateien. Der bestehende Prüfharness des Datenworkers wurde ausschließlich gelesen. Keine Rohdaten, Archive, Register, Übergaben oder Statusdateien geändert. Keine Delegation, T3-Threads, fremde Sessions, Commits, Pushes, Merges oder Deploys.

## Sichere Übergabe und SHA-256

Alle vier folgenden Werte wurden nach dem letzten Formatlauf unmittelbar mit `sha256sum` ermittelt. Die früher im Arbeitsverlauf ermittelten Zwischenhashs sind kein Freeze.

| Geänderte Datei | Eingefrorener SHA-256 |
| --- | --- |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory.rs` | `eb957daef1cb2d0f9c97cec259703e1f20621d4944622622c93441674f536312` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs` | `54c45e1f5a6baf63f671bed584380f115ce9ba493e3ba47a2149a4552bc06b09` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/storage.rs` | `46b43077716094e8dbf85481c99ffb377e601a5af8e7927427d94d8fe95ce4a2` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/tests.rs` | `3cbe2bf83cf26782bb9a0d9e110c6a1ac43f60632915f2679fe81ad68b2b6a7b` |

Der Datenworker darf jetzt seinen eigenen Harness anpassen und die Prüfung dieses konkreten Snapshots beginnen. Insbesondere muss sein bisheriger absichtlich ablehnender Zweig für die vier fremden Domains durch echte getrennte Offline-Läufe ersetzt werden. Die offiziellen Quellenkontexte dürfen dafür nicht durch Domainänderung wiederverwendet werden. Vor und nach seinen Läufen die vier Hashs gegen diesen Freeze prüfen. Beide blockierenden HOSTPROBE-Sperren in vorgegebener Reihenfolge, frische NonZombie-Probe und höchstens zwei Jobs bleiben Pflicht.

## Umgesetzte Fehlerkorrekturen

### Bekannte Revision und lesbarer Inhalt

`normalize_page` liest und validiert `revid` jetzt vor jeder Inhaltsprüfung. Bekannte unterdrückte oder textlose Revisionen bleiben daher im Inventar erhalten. Die drei Zustände sind getrennt:

- `revision`: höchste bekannte numerische Quellrevision; ohne bekannte ID bleibt eine ausdrücklich unbekannte Inhaltsversion erkennbar.
- `content_available`: tatsächlicher Revisionstext für die ausgewählte `revision` ist verfügbar.
- `available_revision`: höchste bekannte lesbare Revision beziehungsweise eine ausdrücklich unbekannte lesbare Inhaltsversion.

Beispiel: lesbare Revision 101 neben unterdrückter Revision 102 ergibt `revision="102"`, `content_available=false`, `available_revision="101"`. Der Originaltext von 101 bleibt als eigenes Dokument erhalten. Unterdrückter Text und ein daneben stehender Extract werden nicht ausgegeben. Ein alleiniger unversionierter Renderextract gilt nicht als verfügbarer Originalrevisionstext.

### Beide Inventar-Einfügepfade

Der gemeinsame `merge_page`-/`merge_inventory_page`-Pfad wird sowohl bei Netzbatches als auch in `persist_offline` verwendet. Neu, dann alt setzt den bekannten jüngsten Stand nicht zurück. Für gleiche Revisionen kann bereits gesicherter lesbarer Inhalt erhalten bleiben. Die unabhängig verfügbare ältere Revision bleibt getrennt ausgewiesen.

Wiederholte XML-Seitenblöcke werden zusätzlich im Normalisierungsergebnis seitenweit zusammengeführt. Sämtliche enthaltenen verfügbaren Revisionsdokumente bleiben daneben erhalten. Identische Versionsschlüssel können weiterhin idempotent im Spool zusammenfallen; Inhaltskonflikte bleiben Fehler und überschreiben nichts.

Das neue Inventarfeld hat einen Serde-Default. Beim Zusammenführen älterer Checkpoint-Einträge wird deren vorhandenes `revision`/`content_available`-Paar in das separate Verfügbarkeitsfeld übernommen. Bereits vor dem Fix verlorene unterdrückte IDs lassen sich aus einem alten fehlerhaften Checkpoint allein nicht rekonstruieren; dafür muss die tatsächliche ursprüngliche Eingabe erneut verarbeitet werden.

### Checkpoint vor dem Einlesen begrenzt

Der Leser prüft vor der Speicherbelegung Dateityp und Größe der geöffneten Datei. Ein zusätzliches `Read::take(limit + 1)` begrenzt die tatsächlich konsumierten Bytes auch bei Wachstum nach der Dateigrößenprüfung. Der zusätzliche einzelne Prüfbyte führt bei Überschreitung zum Fehler. Checkpoint-Schreiben verweigert ebenfalls eine Ausgabe über der vorhandenen Grenze. Kein Limit wurde erhöht.

## Offline-Quellenidentität und Artikelpfad

| Tatsächliche Domain | Getrennte Quellenkennung |
| --- | --- |
| `deadlock.wiki` | `deadlock-wiki` |
| `deadlocked.wiki` | `deadlocked-wiki` |
| `deadlockwiki.org` | `deadlockwiki-org` |
| `deadlock.miraheze.org` | `deadlock-miraheze-org` |
| `deadlockwiki.miraheze.org` | `deadlockwiki-miraheze-org` |

Die Allowlist gilt ausschließlich für Offline-Kontexte. Die Live-Funktion verwendet weiterhin ausschließlich `https://deadlock.wiki/api.php`; fremde siteinfo-Antworten und fremde gespeicherte Quellenkontexte werden vor weiteren Live-Abrufen abgewiesen. Kein zusätzlicher HTTP-Stack oder neuer Netzrequest.

Dokument-IDs verwenden die tatsächliche Quellenkennung. Locator, Revisionsadresse und Attributionsadresse verwenden dieselbe tatsächliche Quelle und den belegten Artikelpfad. `/wiki/`, `/mw/index.php/` sowie der MediaWiki-Querypfad `/mw/index.php?title=` bleiben unterscheidbar. Ein im XML belegtes HTTP-Schema wird nicht auf HTTPS umgeschrieben. Identische numerische IDs und XML-wikiid werden nicht als Aliasbeleg benutzt.

Ein belegter siteinfo-Artikelpfad wird aus `query.general.articlepath` übernommen. Bei XML-Dokumenten hat der tatsächliche XML-base-Artikelpfad Vorrang vor einem gegebenenfalls späteren siteinfo-Pfad. Die bestehenden offiziellen v1-Kontexte bleiben lesbar; deren früher festgelegte offizielle Herkunft ist der Serde-Default. Neue siteinfo-Eingaben ohne Serveradresse und XML-Eingaben ohne Basisadresse werden abgewiesen. Für neue fremde siteinfo-Kontexte ist auch der Artikelpfad erforderlich.

Checkpoint, Dokument-ID und Quellenlocator werden auf konsistente Quellenkennung geprüft. Ein bereits einer anderen Quelle zugeordnetes Inventarverzeichnis wird nicht für die neue Quelle umgewidmet. Pro Archivquelle ein eigenes Ausgabeverzeichnis verwenden.

## Tatsächlich nutzbare Kontext-API

Die bisherigen Sammel-/Schreibsignaturen bleiben unverändert. Neu beziehungsweise erweitert:

```rust
WikiSourceContext::from_siteinfo(payload: &serde_json::Value) -> crate::Result<WikiSourceContext>
WikiSourceContext::from_mediawiki_export(xml: &str) -> crate::Result<WikiSourceContext>
WikiSourceContext::source_id(&self) -> crate::Result<&'static str>
```

Zusätzliche öffentliche Kontextfelder:

```rust
pub source_origin: String;
pub source_article_base: Option<String>;
pub source_siteinfo_hash_representation: String;
```

Für jedes der vier eigenständigen Archive den Kontext direkt aus dessen vollständigem Original-XML erstellen:

```rust
let mut context = WikiSourceContext::from_mediawiki_export(&xml)?;
context.source_capture_sha256 = Some(actual_original_archive_sha256);
// historical_capture ist bei diesem Konstruktor bereits true.
let report = write_wiki_export_capture(output_dir_for_this_source, &xml, &context, &options)?;
```

Der XML-Konstruktor erhält Namespace-Schema, belegte Sprache oder `und`, ursprüngliche Domain und Artikelpfad aus diesem Export. Er erfindet keine Lizenz: `name="unverified"`, `url=null`, `redistribution_allowed=false`. Für tatsächliche Quellenlizenzbelege kann der Aufrufer die schon vorhandenen `license`- und `license_observed_at`-Felder anhand genau dieser Quelle befüllen. Keine Lizenz von deadlock.wiki auf andere Domains kopieren.

Für den offiziellen Hauptdump kann weiterhin dessen tatsächlich gesichertes historisches siteinfo über `from_siteinfo` verwendet werden. Der XML-Normalisierer prüft die Herkunft gegen dessen Serveradresse und erhält für Dokumentadressen immer den im XML vorhandenen Artikelpfad. Lizenzrechte einzelner Revisionen bleiben ausdrücklich ungeprüft.

Der XML-Kontexthash wird über exakt die UTF-8-Bytes des erhaltenen siteinfo-XML-Abschnitts berechnet; seine Darstellung heißt `mediawiki_export_siteinfo_xml_utf8`. Bei API-siteinfo bleibt die Darstellung `serde_json_compact`. Der Originalarchivhash bleibt separat in `source_capture_sha256`. Beitragszuordnungen erhalten neben dem Autor auch belegte XML-Contributor-ID, IP und Löschmarkierung als Metadaten. Quellentext, Inhalts-SHA-256, Bedingungen und historische Kennzeichnung bleiben unverändert. Die Archive belegen keine aktuelle globale Vollabdeckung.

## Prüfungen und geschriebene Regressionen

Ausgeführt: gezieltes `rustfmt` auf den vier eigenen Dateien und anschließender Formatcheck, `RUSTFMT_AND_CHECK_EXIT=0`. Die abschließende gezielte Grep-Zwillingssuche belegt beide gemeinsamen Einfügepfade, Revisionsidentität vor den Hidden-Prüfungen und den begrenzten Checkpoint-Leser. Keine verbleibende bedingungslose `state.pages.insert`-Stelle.

`tests.rs` enthält jetzt 22 per Grep gezählte `#[test]`-Funktionen, zuvor elf. Die neuen beziehungsweise ergänzten Regressionen behandeln:

- Unterdrückte und textlose neue Revision neben lesbarer alter, beide Reihenfolgen; ungültige ID auch bei verborgenem Text.
- Reiner Renderextract ohne verfügbare Originalrevision.
- Capture neu, dann alt, mit lesbarer oder unterdrückter jüngster Revision; Inventar und Checkpoint.
- Netzbatches mit erneutem älterem Stand derselben Seite.
- Wiederholte XML-Blöcke in beiden Reihenfolgen, verfügbare und unterdrückte jüngste Revision, vollständige lesbare Historie und idempotente Wiederholung.
- Fünf Domains mit gleichen numerischen IDs und gleichem wikiid, getrennten Kennungen, ursprünglichen Artikelpfaden und erhaltenen Text-/Autoren-/Hashdaten.
- siteinfo-Artikelpfad und Vorrang des XML-base-Pfads, HTTP-Schema und Query-Artikelpfad.
- Fehlende oder widersprüchliche Herkunft, Hosttäuschung, vermischte Quellen und fremder Live-Kontext.
- Begrenzter Stream mit nachgewiesener maximaler Lesemenge in der geschriebenen Assertion, exakte Bytegrenze und sparse Checkpoint oberhalb der Grenze.
- Serde-Lesbarkeit älterer Checkpoint-/Kontextfelder und revisionsbewusste Zusammenführung.

Diese Rust-Regressionen wurden noch nicht ausgeführt. Kein Cargo-/Clippy-/Release-Lauf. Der Datenworker kompiliert anschließend den Freeze mit seinem Harness und verarbeitet die vollständigen fünf Archive sowie den echten Altcache. Zahlen wie 4.417 Seiten und 22.742 Revisionen bleiben bis dahin Recherchewerte, kein Nachweis dieses Modulstands. Eigene Laufzeitverifikation und echte Exit-/Testzahlen müssen nach den Läufen ergänzt werden.

## Offene HTTP-Abhängigkeit und Gate

Der lokale Hauptbefund 3 bleibt offen: `HttpClient::get_no_redirect` liest den gesamten Body vor As `max_response_bytes`-Prüfung. Diese vorhandene Nachprüfung ist ausdrücklich kein Fix der Ressourcenbegrenzung.

C muss eine schriftlich bestätigte vorhandene oder additive Core-Schnittstelle mit genauer Signatur liefern. Erforderlich ist eine harte Begrenzung während des Bodylesens, auch bei Fehlerstatus und ohne Content-Length. Redirectverbot, ein Versuch, Timeout und vorhandene Antwortmetadaten müssen erhalten bleiben. A verändert keine Core-Datei und baut keinen Ersatztransport. Eine spätere Anbindung benötigt einen neuen abgestimmten Snapshot; der jetzt übergebene Freeze bleibt während der Datenprüfung unangetastet.

`gate_hook.py --review` wurde nicht gestartet. Ein verifizierter bestehender Sol-only-Auswahlweg mit höchstens high wurde in dieser Runde nicht belegt. Die gelesene Merge-Skill-Vorgabe nennt Astra-/Opus-Defaults; diese widersprechen dem ausdrücklichen Auftrag und wurden nicht benutzt. Kein erfundener Auswahlparameter oder Modellrückfall. Cs unabhängiger Schluss-Gate auf dem integrierten SHA bleibt Pflicht.
