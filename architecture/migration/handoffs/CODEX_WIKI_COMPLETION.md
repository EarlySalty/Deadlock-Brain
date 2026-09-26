# Wiki-/Knowledge-Abschluss: remote implementierter Pfad

Stand: 25.09.2026. Branch: `codex/wiki-completion-20260925`. Ziel: Review gegen `main`, **kein Merge, kein Deploy, kein Wiki-Mirror**.

## Basis und Bestandserhalt

Der separate Worktree wurde von frisch geholtem `origin/main` **25c6ed6951370b092f60c67a35bdbe37440ece5d** erstellt. Das bereits geänderte ursprüngliche Arbeitsverzeichnis wurde nicht benutzt oder bereinigt.

PR #13 ist auf dieser Basis enthalten. `wiki.rs`, `wiki_corpus.rs`, `hero_dossier.rs` und der vorhandene Refresh-/Retrieval-Pfad werden nicht durch ältere Fassungen ersetzt. Der zusätzliche Adapter ergänzt `dbrain-sources`; der bestehende Dienst wird nicht umgeschaltet.

PR #30 wurde commitweise analysiert: **b93002f744ecb86cb3d0decee4067a6331c984c3** und **fd00f24ea5b7956ab25172cf49e6aa9f3e5f44fe** ergänzen ausschließlich bislang fehlende S12-Dateien. Diese beiden Commits wurden bestandserhaltend als **794bebb** und **eb3b29a** übernommen. Die ursprünglichen 64 Parser-/Regressions- und neun CLI-Tests sowie der Referenz-Golden bleiben erhalten. Alte Berichte unter `s12/reports/` dokumentieren weiterhin ihren damaligen Stand, nicht die Ergebnisse dieses Branches.

Auf der geprüften Main-Basis fehlte `brain-contracts`. Deshalb wurde **nur die bereits vorhandene Contract-Crate** aus PR #25, Stand **2c470864d82a2a2e954cf8eb18552a7b33a59aa0**, übernommen und als Workspace-Mitglied ergänzt. Keine eigene zweite Definition von `Fact`, `DocumentRevision`, `SourceRecordV2`, `CorpusRelease` oder `HeroKnowledgeCard`; keine Übernahme des übrigen konfliktbehafteten Core-Branches. Abweichungen an der übernommenen Contract-Datei sind ausschließlich Rustfmt. Bei späterer Integration des parallelen Core-PRs dieselbe Crate zusammenführen, nicht verdoppeln.

## Implementierter Umfang

| Bereich | Implementierung und überprüfbare Grenze |
|---|---|
| Source Manifest | Vorhandenes `Report` als maschinenlesbares Manifest: stabile Quell-/Seiten-IDs, dynamische Namespaces, Inhaltsmodelle, Revisionen und SHA-256, Source-/Fetch-Zeit getrennt, Sprache, Rechteentscheidungen, Coverage, Quarantäne und offene Abhängigkeiten. `analyze` und `extract` erzeugen es aus einem Capture; historische Manifestdateien werden nicht als Liveinventar ausgegeben. |
| Discovery und Capture | `capture::collect` verwendet einen injizierbaren Transport. Alle nichtvirtuellen Namespaces aus `siteinfo`, einschließlich Redirects, werden inventarisiert. Bodies werden ausschließlich für explizite Pilot-IDs gelesen, höchstens 64; eine leere Auswahl bedeutet nur Metadaten. Requests, Seiten und Gesamtbytes sind begrenzt. Kein automatischer Vollimport. |
| Continuation | Vollständige erlaubte Tokens werden wiederverwendet. Discovery und `templates`/`links` werden über alle Batches zusammengeführt. Wiederholte, manipulierte, unvollständige oder fehlerhafte Antworten ergeben keinen angeblich vollständigen Capture. Tokens dürfen weder Aktion noch Seitenidentität überschreiben. |
| Revisionen | Zwei neueste Revisionen werden angefordert. Ältere History-Continuation bleibt ausdrücklich als außerhalb dieses Fensters gespeichert. Head, Identität und Revision während der Dependency-Erfassung werden geprüft. Unterdrückte, fehlende, zukünftige, widersprüchliche oder veraltete Revisionen führen nicht zum Rückgriff auf alte Zahlen. |
| Strukturierte Daten | Vorhandener Duplicate-Key-sicherer JSON-Parser mit RFC-6901-Locators bleibt die Grundlage. Statische Module dürfen eine eng begrenzte `return { ... }`-Literalgrammatik enthalten. Flache, vollständige Template-Aufrufe liefern nur benannte Literalparameter. Keine Template-Expansion, keine Lua-VM, keine Funktionen, Imports, Operatoren, Globals, berechneten Schlüssel oder sonstige Ausführung. |
| Wikitext und HTML | Reiner Wikitext und Redirects behalten exakte UTF-8-Positionen. Unsichere/gemischte/nicht unterstützte Syntax bleibt sichtbar quarantiniert. Der bestehende HTML-Textfallback aus PR #13 bleibt erhalten; Einzelheiten unten. Kein Anspruch auf eine vollständige MediaWiki-/Scribunto-Implementierung. |
| Gemeinsame IR | `knowledge::extract` ist der gemeinsame Weg für JSON-, Template- und Modul-Kandidaten. Ein separat geprüftes Mapping bindet Quellfelder an persistente Entity-IDs. Rohsyntax erteilt niemals selbst diese Bindung oder eine Fact-Freigabe. |
| Einheiten und Unknown | Exakte Dezimalstrings statt stiller Float-Rundung. Prozent, Prozentpunkte und Multiplikatoren bleiben verschieden. Fehlend, explizit `null`, unbekannte Einheit, nicht repräsentierbare Dezimalwerte und `false` werden unterschieden. Keine implizite Umrechnung und kein fehlend→0. |
| Bedingungen und Varianten | Originalausdrücke samt Quellenpositionen bleiben in der IR erhalten. Es wird keine eigene Regel-Engine gebaut und nichts ausgewertet. Fehlt eine angeforderte Bedingung oder Variante, ist das unbekannt, nicht bedingungslos. |
| Aliase | Nur belegte Redirects, persistente Zielbindung, Originalsprache und sicherer transitiver Dependency-Pfad. Namensnormalisierung ist kein Fuzzy-Hero-Matching; mehrdeutige Auflösung bleibt unbekannt. |
| Source-/Fact-/Card-Adapter | Rohrevisionen werden als echte `SourceRecordV2` erzeugt. Die bestehende `SourceStore`-Schnittstelle kann diese in `brain.source_documents` ablegen. Geprüfte, verlustfrei repräsentierbare IR-Felder werden als echte `Fact` und `HeroKnowledgeCard` projiziert; Details und Einschränkungen unten. |
| Determinismus und Invalidierung | Sortierte, versionsgebundene Projektion mit SHA-256 über Karte und Provenienzumschlag. Fetch-Zeit und Reihenfolge der Mapping-Einträge ändern den Rebuild nicht. Vorhandener transitiver Delta-Algorithmus bleibt erhalten. `compare_ir` ergänzt selektive Mapping-/Entity-Änderungen; globale Policy-Änderungen invalidieren Projektionen ohne neue Rohdaten-Embeddings. |
| Freigabebindung | Mapping-, Parser- und IR-Version beeinflussen Feldidentität. Fact-Freigaben sind an Revision **und** Raw-Hash gebunden. Auch alle Abhängigkeiten müssen mit Revision und Hash geprüft sein. Ein neuer Wikiwert oder ein anderes Mapping kann keine alte Freigabe wiederverwenden. |
| Nachvollziehbarkeit | Kartenumschlag enthält Source-Locators, transitive `DocumentRevision`s, Unknowns, Mapping-/Reviewbezug, Policy-Hash, Knowledge-Version und Generator-Version. `published` bleibt immer `false`. |

### Bestehender HTML-Fallback, ausdrücklich dokumentiert

`dbrain-sources/src/wiki_corpus.rs` nutzt die MediaWiki-Parse-Antwort einer konkreten `oldid`, prüft Titel/Seiten-ID/Revision und extrahiert sanitierte Textabschnitte, Tabelleninhalte und Bild-Alternativtexte. Skripte und Navigations-/Formularinhalte werden nicht ausgeführt. Historie und Lore bleiben von aktuellen Mechanik-/Build-Notizen getrennt. Der bereits bestehende `hero_dossier`-Pfad verwendet diese Belege weiterhin nach seinen bisherigen Regeln.

**HTML ist nur ein Textfallback.** Eine Artikel-`oldid` pinnt nicht automatisch die dabei gerenderten Vorlagenrevisionen. Die vorhandenen Metadaten `rendered_templates_pinned=false` und `patch_verified=false` bleiben erhalten. Fehlgeschlagene Literalparser lösen keinen automatischen HTML-Abruf und keine HTML-Zahl→Fact-Hochstufung aus. Ein späterer kontrollierter HTML-Capture muss gesondert attribuiert und geprüft werden.

### Datenbank und Contracts: keine vorgetäuschte Vollintegration

`wiki_capture_io::stage_sources_with_pool` benutzt ausschließlich den bestehenden `SourceStore`, Source-Run und die vorhandene `brain.source_documents`-Tabelle. Es gibt **keine neue Facts-Datenbank, keine Migration, keine zweite Source-Tabelle, keine zusätzlichen Entity-Snapshots und keinen neuen Publisher oder Scheduler**. Der Adapter wurde remote kompiliert, aber nicht gegen eine echte Datenbank ausgeführt.

Der aktuelle gemeinsame `brain.v1::Fact`-Vertrag enthält keine separaten Bedingungs-/Variantenfelder; die Karte enthält außerdem keinen vollständigen Unknown-/Policy-/Locator-Vertrag. Daher werden kontextabhängige Felder nicht durch Weglassen ihres Kontextes zu nackten Facts gemacht. Die Informationen bleiben in IR und Projektionsergebnis. Rules und Synergien werden ohne Domain-Validierung nicht erfunden. Der vorhandene numerische Source-of-Truth-/Build-Pfad wird nicht überschrieben.

Das ist ein implementierter, getesteter **DTO-/Projektionsadapter**, kein Nachweis einer aktiven kanonischen Fact-Persistenz oder eines neuen Knowledge-Releases in der Produktion. Ein entsprechender öffentlicher Fact-Write-/Release-Port ist in der geprüften Basis nicht vorhanden. Diese Vertrags-/Integrationsgrenze ist von HTTP 403 unabhängig und darf nicht als reines Zugangsproblem umetikettiert werden.

## Reproduzierbare Bedienung

Rust 1.97.1 auf `PATH`; keine Secrets oder Datenbankverbindung:

```bash
export SQLX_OFFLINE=true
python3 architecture/migration/s12/check-completion.py

cd architecture/migration/s12
cargo run --locked --offline -- analyze fixtures/pilot.capture.json
cargo run --locked --offline -- extract fixtures/pilot.capture.json fixtures/completion.mapping.json
cargo run --locked --offline -- project fixtures/pilot.capture.json fixtures/completion.mapping.json fixtures/completion-release.json fixtures/completion-review.json 101 de
cargo run --locked --offline -- ir-delta fixtures/pilot.capture.json fixtures/completion.mapping.json fixtures/pilot.capture.json fixtures/completion.mapping.json
```

Die Review-/Release-Dateien oben sind **synthetische Testfreigaben, keine Freigaben für echte Wikiseiten**. `examples/contract_pilot.rs` verarbeitet ausschließlich einkompilierte synthetische Fixtures und nimmt keine Live-Dateien an. Es ist keine automatische Freigabefunktion für neue Quellen.

`check.sh` bleibt der Einstieg in die ursprüngliche Offline-Suite. Sein fehlerhafter Arbeitsverzeichnispfad wurde korrigiert. Der Formatcheck ist auf das S12-Paket begrenzt, damit die neue lokale Contract-Dependency nicht unbeabsichtigt den ganzen historischen Workspace formatiert; die veränderten Source-Dateien und Contracts werden zusätzlich getrennt geprüft. Clippy und Tests für alle S12-Targets bleiben streng. Der neue GitHub-Workflow führt diese Suite und die Source-Integration aus; er ändert keine Merge-/Deploy-Gates und verwendet keine Secrets.

## Tatsächlich ausgeführte Prüfungen

Die vollständige, maschinenlesbare Aufzeichnung steht in `architecture/migration/s12/reports/completion-results.json`: exakte Befehle, Rückgabecodes, Toolchain und SHA-256 der geprüften Quellen/Fixtures. Zugehörige `completion-*.log` enthalten die wirklichen Rust-Ausgaben. Der finale Bericht hat `complete=true`; sämtliche neun aufgezeichneten Kommandoblöcke endeten mit Exit 0.

| Prüfung | Tatsächliches Ergebnis |
|---|---|
| Übernommene und erweiterte S12-Suite | **106 bestanden, 0 fehlgeschlagen, 0 ignoriert**: 73 ursprüngliche + 33 zusätzliche Tests; einschließlich CLI-, Transport-, Grounding-, Alias-, Context-, Delta- und Golden-Prüfungen. |
| Gemeinsame Contracts | **3 bestanden**. |
| Bestehende Source-Crate | **65 bestanden, 5 ignoriert**; die fünf benötigen PostgreSQL und wurden nicht als erfolgreiche DB-Prüfung gezählt. |
| Neue Source-/Contract-Integration | **2 bestanden**, ohne Netzwerk/Datenbank. |
| Bestehender Hero-Dossier-Pfad | **7 bestanden**; 52 andere Retrieval-Tests waren durch den ausdrücklich benannten Filter nicht Teil dieses Laufs. |
| Formatter | S12-Paket, übernommene Contract-Crate sowie alle veränderten Source-Rustdateien geprüft, **Exit 0**. Unbeteiligte historische Dateien wurden nicht umformatiert. |
| Clippy | S12 alle Targets und Contracts/Source alle Targets mit `-D warnings`, **Exit 0**. |
| Releasebuild | Standalone S12-CLI und betroffene `brain-contracts`-/`dbrain-sources`-Bibliotheken, jeweils locked/offline, **Exit 0**. |
| Workspace-Kompilierung | `cargo check --workspace --all-targets --locked --offline -j2`, **Exit 0**. |
| Negativer Readiness-Test | Erfolgreicher Fixture-Capture bleibt absichtlich nicht produktionsbereit: CLI liefert beim Readiness-Zwang erwarteten Exit 3; das Testskript prüft diesen Wert. |

Insgesamt **183 bestandene Tests**, fünf explizit ignorierte DB-Tests, kein fehlgeschlagener Test im Abschlusslauf. Rust/Cargo **1.97.1**, `SQLX_OFFLINE=true`, Datenbank-Zugangsdaten für den gesamten Abschlusslauf entfernt. Keine Produktion und kein neuer Wiki-Capture wurden hierfür verwendet.

Ein zusätzlicher Regressionstest hat vor der Korrektur tatsächlich gezeigt, dass eine globale Source-Policy-Änderung keine Karteninvalidierung auslöste. Der Test bleibt in der Suite; die Korrektur ändert keine Rohhashes und erzeugt keine Embeddingjobs. Der Golden-Vergleich ergänzt unabhängige Checks jedes Fact-Locators, ersetzt diese nicht.

## Verwendete Fixtures

Alle neuen Tests verwenden den übernommenen, frei erzeugten `pilot.capture.json`: **19 synthetische Seiten, 20 Revisionen**, zwei voneinander unabhängige Helden, zwei Revisionen des ersten Helden, eine Data→Template-Kette, zwei Mechaniken, Items, Regel, Alias, Lokalisierung, History/Lore/Guide sowie ein absichtlich unzulässiges Modul. Keine echten Spielwerte, kein Upstream-Wikivolltext, keine privaten Daten.

Zusätzlich: `completion.mapping.json`, `completion-release.json`, `completion-review.json`, `completion-card.golden.json`, `literal-data.lua` und `literal-template.wiki`. Die neue synthetische Contract-Karte enthält sechs verlustfrei darstellbare Facts; fehlende oder kontextabhängige Felder bleiben Unknown/IR. Zwei-Revisions-, Revoke-, Alias-, Unit-, Mapping- und Fehlerfälle mutieren ausschließlich diese synthetischen Daten. Transporttests benutzen generierte MediaWiki-Antworten in Rust, nicht einen echten Server.

Die Originaldatei `hero-preview.golden.json` und die 73 ursprünglichen Tests bleiben unverändert erhalten. Der zusätzlich ausgeführte bestehende Hero-Dossier-Test verwendet den bereits versionierten lokalen Game-Wiki-Korpus und liest keine Produktion.

## Was ausschließlich wegen fehlendem Livezugang offen ist

Ein einzelner normaler Metadatenabruf vom Arbeitsserver am **25.09.2026** gegen `https://deadlock.wiki/api.php?action=query&format=json&meta=siteinfo&siprop=general%7Cnamespaces%7Crightsinfo` lieferte erneut **HTTP 403**. Kein Redirect-/Proxywechsel, keine Cookie-/Auth-Erfindung, kein erneuter Versuch und kein Mirror.

Deshalb liegen kein neuer echter Siteinfo-/Namespace-Capture, keine aktuelle belegte Upstream-Lizenzantwort, keine echte Zwei-Revisions-Stichprobe und kein realer Template-/Modul-Dependency-Capture vor. Ebenso wenig gibt es eine gemessene Live-Coverage oder geprüfte echte Feldmappings. Syntaxvielfalt echter Seiten kann zusätzliche explizite Parser-/Mappingregeln erfordern; der Fixtureerfolg ist kein Beweis für Vollabdeckung des echten Wikis.

Unabhängig vom 403 unbewiesen bleiben ein Scratch-DB-Roundtrip, reale Source-/Entity-Zuordnung, fachliche Regel-/Buildparität, ein freigegebenes kanonisches Knowledge-Release, Provider-/Publikationsrechte und Produktionsbetrieb. Es wurden weder Gates G0–G6 freigegeben noch Migrationsstatus global auf „fertig“ gesetzt.

## Lokale Prüfung für Claude

1. Den PR-Code und `completion-results.json` prüfen, Rust 1.97.1 verwenden und `check-completion.py` zuerst ohne DB-/Providerzugang wiederholen. Beim Zusammenführen des Core-Branches die vorhandene `brain-contracts`-Crate beibehalten, nicht duplizieren.
2. Nur mit regulär freigegebenem Wiki-Zugang einen begrenzten Metadaten-/Pilot-Capture erstellen. Bei erneutem 403 stoppen. Keine Zugangssperre umgehen und keine synthetischen Rechteentscheidungen übernehmen. Der opt-in Netzwerkadapter verwendet denselben Rate-Limiter und Lock wie der bestehende Corpus-Pfad; sein gemeinsamer HTTP-Client puffert Antworten, das nachgeschaltete Größenlimit ist **kein Streaming-Allokationslimit**.
3. Siteinfo/Lizenz, echte Namespace-IDs, Seiten-/Entity-Zuordnung, zwei Revisionen, Suppression und vollständige Abhängigkeiten prüfen. Rohcapture lokal behalten; keine privaten Fixtures/Secrets ins Repository übernehmen. Für HTML ungebundene gerenderte Templates ausdrücklich markieren.
4. Echte Mappings separat reviewen. Units, Dezimaldarstellung, Bedingungen, Varianten und Alias-Sprache anhand der Original-Locators prüfen. Jede Freigabe an Source- und Dependency-Revision **plus Hash** binden. Wiki-Revisionsdatum niemals als Spielpatch ausgeben.
5. `stage_sources_with_pool` ausschließlich gegen eine Wegwerf-/Scratch-Datenbank mit dem vorhandenen Schema roundtrippen: identischer Capture idempotent, beide Revisionen vorhanden, vollständiger Fehler-Run sichtbar, keine Entity-Snapshot-/Fact-/Release-Schreibzugriffe. Die vorhandenen ignorierten Datenbanktests nur mit bewusst freigegebener Scratch-Konfiguration aktivieren.
6. Vor Anbindung an kanonische Fact-/Release-Ports die genannten Contract-Lücken mit dem Core-/Domain-Owner lösen. Zwei echte Revisionsstände und Dependency-only-Änderung replayen: betroffene Karten neu, unabhängiger Held unverändert, Revoke entzieht Belege, frischer Fetch allein erzeugt keine neue Karte. Erst danach separat über Runtime-Aktivierung entscheiden; dieser Auftrag enthält weder Merge noch Deploy.
