# S12 — Wiki-/Knowledge-Pfad

> C4-Pfadwechsel: Aktiver Parser und Tests liegen jetzt unter `rust/crates/dbrain-wiki` im Hauptworkspace. Die historische Darstellung unten und die gespeicherten Fixtures/Berichte bleiben erhalten. `Cargo.toml.historical` und `Cargo.lock` sind Nachweise, keine aktive Crate. Siehe [aktuellen Parser](../../../rust/crates/dbrain-wiki/README.md).


## Aktueller Stand: 25.09.2026

Die ursprüngliche Offline-Suite aus PR #30 bleibt erhalten und wurde erweitert. `capture` bietet begrenzte Discovery/Continuation mit injiziertem Transport, `literal` liest ausschließlich sichere Datenliterale, und `knowledge` bindet die gemeinsame IR an die **vorhandene** `brain-contracts`-Crate. `dbrain-sources` verwendet dieselbe Bibliothek und seinen bestehenden SourceStore; keine zweite Facts-Datenbank.

Neue Offline-Kommandos: `extract`, `project` und `ir-delta`. Release- und Review-Dateien sind explizite Eingaben; aus Parsererfolg wird keine automatische Freigabe. Der optionale Netzwerkadapter bleibt standardmäßig deaktiviert. Der einzige Live-Metadatencheck ergab HTTP 403, ohne Umgehung oder weitere Abrufe.

Aktuelle Implementierung, Grenzen, Befehle, tatsächlich ausgeführte Tests und lokale Claude-Prüfung: [CODEX_WIKI_COMPLETION.md](../handoffs/CODEX_WIKI_COMPLETION.md). Reproduktion: `bash architecture/migration/s12/check-completion.sh` vom Repository aus, mit Rust 1.97.1 auf PATH. Die versionierten `reports/completion-*` sind historische Nachweise. Aktuelle Läufe schreiben ausschließlich nach `rust/target/wiki-completion.*/`; die Änderungen an Konfiguration und Tooling stehen in [C7_C8_RUNTIME_TOOLING.md](../handoffs/C7_C8_RUNTIME_TOOLING.md).

Die folgenden Abschnitte und die ursprünglichen Reports dokumentieren den **historischen prepare-only-Stand aus PR #30**. Aussagen über damals fehlende Contracts, unveränderte Workspaces und fehlende Card-Projektion sind keine Beschreibung der neuen Adapter. Liveabdeckung, Gatefreigaben und kanonische Produktion bleiben dennoch unbewiesen.

---

## Übernommene Vorbereitung aus PR #30


**Arbeitsmodus: `prepare_only`. Kein integrierter Wiki-Importer, keine kanonische HeroKnowledgeCard, kein G2/G3/G4-Nachweis.**

Basis: `3032651` (`origin/main`, am 24.09.2026 geprüft). `architecture/migration/STATUS.md` führt G0/G1, Contract-Version und DB-Schema weiterhin als offen. S12 verändert deshalb ausschließlich diesen paketlokalen Offline-Bereich und die eigene Übergabe. Root-Workspace, gemeinsame Verträge, Stores, Worker, Reasoner, produktive Wiki-Pfade, Workflows und Gatefreigaben bleiben unverändert.

## Was ausführbar vorliegt

Das eigenständige Rust-Werkzeug liest **bereits bereitgestellte, für die Offline-Prüfung freigegebene JSON-Captures**, nicht das Netzwerk. Es prüft dynamische Namespace-Inventare und die vollständige `allpages`-Continuation, führt Page-ID-/Revisions-/Hash-/Zeit-/Sprach-/Policy-Metadaten zusammen, zeigt unbekannte Syntax und erzeugt ein statusgetrenntes Manifest. `parsed` bedeutet nur syntaktisch gelesen; `normalized`, `validated` und `published` bleiben ausdrücklich null.

Die Karten-Vorschau ist ausschließlich eine reproduzierbare Sicht aus **Quellenreferenzen und Quellpositionen**. Sie kopiert keine Stats und behauptet keine geprüften Fact-/Rule-IDs. Der vorhandene `SourceStore -> game_wiki`-Pfad und PR #13 werden nicht dupliziert oder in diesem Vorbereitungspaket aktiviert. Erst S02/S03/S05 geben die echten gemeinsamen Typen und Ports vor.

Der reine Delta-Vergleich prüft unveränderliche Revisionen auch zwischen Captures, lehnt veraltete Stände ab und berechnet transitive Abhängigkeiten. Ein unveränderter Inhalt bei neuer Abrufzeit löst keine Reparse-Anforderung aus. Änderungen an Template-/Data-Abhängigkeiten betreffen nur ihre abhängigen Seiten/Karten, sofern das Abhängigkeitsinventar vollständig ist. Bei fehlenden Abhängigkeiten wird ein vollständiger Abgleich verlangt. Es werden **keine** Embeddingjobs, Releases oder Datenbankänderungen ausgeführt.

## Start und Prüfungen

Aus diesem Verzeichnis, mit Rust/Cargo auf `PATH`:

```sh
# Einmalig auf einem Rechner ohne Dependency-Cache:
cargo fetch --locked

# Danach vollständig offline:
bash check.sh
cargo run --locked --offline -- analyze fixtures/pilot.capture.json
cargo run --locked --offline -- delta fixtures/pilot.capture.json fixtures/pilot.capture.json
cargo run --locked --offline -- analyze fixtures/pilot.capture.json --require-production-ready
```

Der letzte Befehl liefert absichtlich **Exitcode 3**: Auch ein vollständig lesbarer synthetischer Capture ist keine Produktionsfreigabe. Exitcode 0 bedeutet nur, dass eine Analyse abgeschlossen wurde; Exitcode 2 bedeutet ungültige Eingabe oder Werkzeugfehler. `--allow-wiki-network`, `publish` und vergleichbare Aktivierungsoptionen existieren nicht.

Getestete Toolchain und exakte Ergebnisse stehen im separaten Prüfbericht. Deklarierte Rust-Untergrenze: 1.85; die Mindestversion ist nicht eigens getestet. Der produktive Rust-Workspace ist weder Dependency noch Bestandteil dieses kleinen Workspaces. Ein grüner bestehender GitHub-Job `Rust compile` beweist deshalb **nicht**, dass diese S12-Tests gelaufen sind; CI-Erweiterung ist in CR-S12-001 an S02/S10 angefordert.

## Capture-Format und Grenzen

`model.rs` definiert ausschließlich `s12-capture-v1` und lokale Diagnoseberichte, **keine** Ersatzdefinition für `SourceRecordV2`, `Fact`, `Rule`, `HeroKnowledgeCard` oder `CorpusRelease`. `fixtures/pilot.capture.json` ist das ausführbare Beispiel.

- `policy.offline_review_allowed` und eine nichtleere `decision_ref` müssen ausdrücklich gesetzt sein. `rightsinfo` des Wikis wird niemals selbst zu einer Berechtigungsentscheidung. Die Capture-Freigabe wird von diesem Offline-Werkzeug nicht authentifiziert; der spätere Runtime-Policy-Port darf diese Felder nicht als Credentials übernehmen.
- `siteinfo.query.namespaces` ist dynamisch. Alle nichtnegativen entdeckten Namespaces benötigen vollständige Discovery-Batches, auch wenn eine spätere Klassenentscheidung Inhalte ausschließt. Virtuelle Namespaces werden nur katalogisiert. Kein fest codierter Hero-/Itemzähler, kein implizites Auslassen von Redirects, Lore, Guides oder Medienmetadaten.
- Jeder Discovery-Batch speichert die **gesamte** gesendete Continuation. Lücken, Wiederholungen, doppelte Page-IDs oder inkompatible Responses verhindern einen vollständigen Befund. Ein API-Fehler ist keine leere erfolgreiche Seite.
- Jeder Page-Capture enthält genau eine angefragte Seite. `pageid`, `ns`, `title`, `lastrevid`, `pagelanguage`, `fullurl`, `revisions`, `templates` und `links` stammen aus dem gelieferten Capture. Metadaten werden nicht aus dem Kartentitel oder der gewünschten Ausgabesprache erfunden. URL-Felder sind upstream gelieferte Metadaten, keine vom Werkzeug besuchten oder authentifizierten URLs.
- V1-Page-Maps und V2-Page-Arrays sowie `slots.main.content`/`slots.main.*` werden gelesen. Die maximale zurückgegebene Revision ersetzt nicht `lastrevid`. Mehrere gleichlautende Captures sind idempotent; Widersprüche werden quarantiniert. Suppression, fehlendes Head-Content oder Zukunftszeiten erlauben keinen Rückfall auf eine ältere vermeintlich aktuelle Zahl.
- `dependencies_complete` ist eine Aussage des Capture-Erstellers, zusätzlich auf vorhandene Dependency-Arrays, ausstehende API-Continuation und auflösbare Seiten geprüft. Es ist **kein** Beweis eines echten vollständigen Wiki-Exports. Bei Unvollständigkeit werden Vorschauen eingeschränkt und Delta-Abkürzungen gesperrt.
- JSON wird nur in syntaktische Kandidaten mit JSON-Pointern zerlegt. Strings, `null`, Prozent/Prozentpunkt-Bezeichnungen und Bedingungen bleiben als rohe Daten erhalten. Es findet keine Dezimalarithmetik, Einheitenkonversion, Effect-Auswertung oder Normalisierung statt; JSON-Zahlen in Diagnoseausgaben sind keine kanonischen exakten Dezimal-Facts. Der Hash bezieht sich auf die ursprünglichen UTF-8-Bytes der Content-Zeichenfolge.
- Wikitext-Prosa und einfache Redirects können als solche katalogisiert werden. Nicht unterstützte Templates/Tabellen/HTML werden mit UTF-8-Positionen diagnostiziert. Lua/Scribunto wird niemals ausgeführt. Ein permissiver generischer Wiki-/Lua-Interpreter oder stiller HTMLfallback wäre vor den gemeinsamen Verträgen gerade nicht zulässig.
- Kartenbindungen sind explizite **Test-/Capture-Konfiguration**, keine persistente kanonische Entity-Mappingtabelle. DE/EN-Vorschauen teilen dieselben Quellen. Unbekannte Zuordnungen, Build-Legalität, Patch/Mode, Knowledge-Release, Strategie und Übersetzungsprovenienz bleiben offen.
- Lokale aktuelle Zugriffsverweigerung schlägt historische Erlaubnis: keine Rohkandidaten/Hashes dieser Seite im Bericht, keine referenzierenden abhängigen Kartenevidenzen. Kein Provider-/Medien-Egress ist implementiert.

Budgetgrenzen: 8 MiB Eingabedatei, 512 KiB Content pro Revision, 10.000 Page-Captures/Discovery-Batches/Klassifikationen, 128 Revisionen pro Response, 2.000 strukturierte Kandidaten pro Revision, 32 JSON-Projektionsstufen, 32 Vorschau-Bindungen, 256 Bindungsreferenzen pro Karte und 50.000 Kandidatenreferenzen über alle Vorschauen. Überschreitungen werden abgelehnt oder explizit quarantiniert, nie still als vollständige Teilmenge ausgegeben. Die CLI folgt keinen Symlinks und liest keine Geräte/Pipes.

## Quellen und vorhandene Vorarbeit

Die unterstützte MediaWiki-Response-Syntax wurde am 24.09.2026 gegen die Primärdokumentation geprüft:

- https://www.mediawiki.org/wiki/API:Siteinfo
- https://www.mediawiki.org/wiki/API:Allpages
- https://www.mediawiki.org/wiki/API:Revisions

Das ist kein Live-Vertragstest gegen Deadlock Wiki. Der einzelne Metadatenabruf vom Arbeitsserver wurde am selben Tag mit **HTTP 403** abgewiesen. Keine weiteren Abrufversuche, kein Massencrawl, keine Umgehung, keine Live-Wiki-Inhalte als Testfixture übernommen. Details in `SOURCE_CHECK.md`.

PR #13 (`3aae8e0`) enthält bereits einen Wiki-Korpus-/Hero-Dossier-Ansatz. Die Ist-Prüfung fand auf `main` die existierenden `dbrain-sources/src/wiki.rs`, `dbrain-retrieval/src/game_wiki.rs` und `deadlock-brain/src/wiki_refresh.rs`. Diese bleiben die vorgesehenen Integrationspfade. Dieses Paket enthält keine automatische Übernahme oder Freigabe des fremden PRs.

## Abnahme

`reports/WIKI_COVERAGE.csv`, `reports/pilot-report.json`, Golden-Fixture und Testprotokoll sind **nur synthetische Offline-Nachweise**. Die tatsächlichen offenen S12-Leistungen und deren Besitzer stehen in `CR-S12-001.md` und `../handoffs/12-wiki-wissenskarten.md`. Der Gesamtauftrag S12 bleibt bis zur echten Integration und Abnahme offen.

Aktuelle Check-Artefakte werden pro Lauf in `rust/target/wiki-completion.*/` geschrieben (TSV-Befehls-/Exitprotokoll, Logs, Quellhashes, Abschlussstatus). Die versionierten `reports/` bleiben historische Nachweise und werden nicht aktualisiert. Der Checker benötigt Bash, Rust und die üblichen GNU-Coreutils, kein Python.
