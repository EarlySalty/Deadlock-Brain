# Anforderungsnachverfolgung

Jede Zeile ist bis zur Abnahme mit tatsächlichen Artefakten und Commits zu verknüpfen. Die aufgeführten Nachweise sind Soll-Nachweise, noch keine erledigten Prüfungen.

| ID | Anforderung | Herkunft | Chat | Soll-Nachweis |
|---|---|---|---|---|
| R01 | Ein einziges aktives Brain-Code-Repository | Recherche: Gesamturteil | 00,11 | G6: Repository-/Serviceinventar |
| R02 | Ein Answer Kernel für alle Consumers | Recherche: Answer Kernel | 08,09 | E2E-Trace und Prüfung entfernter Nebenpfade |
| R03 | Eigener produktiver Backendkern vollständig Rust | Nutzerauftrag | 02–11 | Python-freie Images, Runtime-/Netzwerkprüfung |
| R04 | Nur optionale/temporäre Python-Hilfsskripte | Nutzerauftrag | 00,11 | Ausnahmeinventar, kein Betriebs-/Rebuildbedarf |
| R05 | Alle Datenquellen im selben Umbau berücksichtigen | Nutzerergänzung | 01,03,04 | Source Registry und vollständiger Abgleich |
| R06 | Public/Internal/Private strikt trennen | Recherche: Scopes | 02,03,10 | Negativtests für ACL, Cache, Egress und Zitate |
| R07 | Git-Sichtbarkeit und Secrets vor Import prüfen | Sicherheitsableitung | 01,03,11 | Repo-/History-/Secretprüfung |
| R08 | Provenienz, Revisionen, Gültigkeit und Quellzeiten | Recherche: Dokumentmodell | 03,04 | Revision-/Hash-/Linktests |
| R09 | Generiertes Wissen von Originalen unterscheiden | Recherche: Source of Truth | 03,04 | Elternreferenzen und konservative Rechte |
| R10 | Alle Domainfunktionen erhalten/portieren | Recherche: Domaincode | 01,05 | Featurematrix und Differentialtests |
| R11 | Hybrid Retrieval statt Jev als Suchmaschine | Recherche: Recall/Precision | 06 | BM25/dense/Filtervergleich und Querypläne |
| R12 | Jev als typisierter Router/Filter/Gate | Recherche: Jev | 07,08 | Providerverträge und Shadow-/Aktivierungsbericht |
| R13 | Jev-Feeder-Gating, generatives Modell nur zum Schreiben | Recherche: Feeder | 04,07 | Feeder-A/B-Test und Quellenprüfung |
| R14 | Zentrale LLM-/Embeddingwahl und Budgets | Recherche: Provider | 07,08 | Adapter ohne Providerkeys; zentrale Konfiguration |
| R15 | Twitch als dünner Rust-Adapter | Recherche + Rust-Auftrag | 09 | Twitch-E2E, keine lokale AI-Logik |
| R16 | MCP/CLI/Web/interne Agents über denselben Kern | Recherche: Zielbild | 08,09 | Client-/Protokollparität |
| R17 | Indizes/Embeddings rebuildbar, nicht in Git | Recherche: Runtime Storage | 03,04,06 | Leerer Restore-/Rebuildtest |
| R18 | Snapshot, Delta, Tombstones und eindeutige Writer | Nutzerergänzung + Migrationsableitung | 03,04,11 | Crash-/Replay-/Cutovertests |
| R19 | Performance und Qualität vor/nach Umbau messen | Recherche + Nutzerergänzung | 01,10 | fixierte Profile, Rohmessdaten, Qualitätsgrenzen |
| R20 | Jev zuerst Shadow, dann je Funktion freigeben | Recherche: Shadow Mode | 07,08,10 | Auswertung false negatives und Nutzen |
| R21 | Traces, Token, Kosten und Latenz nachvollziehbar | Recherche: Observability | 02,08,10 | geschützte Traces und Dashboards |
| R22 | Restore/Rollback ohne Writeverlust oder ACL-Rücknahme | Migrationsableitung | 03,10,11 | Wiederherstellungsprotokoll |
| R23 | Arbeit auf kompatible Chats mit Übergaben verteilen | Nutzerauftrag | 00,alle | Owner, Contract-Version, Commit, Testprotokoll |
| R24 | Keine ungeprüften Behauptungen über heutigen Code | Recherche: Unsicherheiten | 01,alle | Belegter Ist-Zustand und Blockerliste |
| R25 | Zusätzliche Recherchen prüfen und ohne Dubletten integrieren | U2/U3/U4 | 00 | Eingang/Planung in v1.0 geschlossen; Originalhashes, 08, R26 ff.; keine Behauptung einer Umsetzung |


## Ergänzungen aus Wiki und Ökosystem

Die Umsetzung dieser Zeilen ist offen. Eine Pflichtfähigkeit gilt nicht schon durch einen Eintrag im Register als geliefert. Nichterreichbare/gesperrte Pflichtquellen benötigen Beschaffung oder ausdrückliche Scopeentscheidung; kein stilles Verschieben in ein späteres Projekt.

| ID | Anforderung | Herkunft | Chat | Soll-Nachweis |
|---|---|---|---|---|
| R26 | Wiki-Gameplay vollständig dynamisch entdecken und Abdeckung getrennt nach Status führen | U3 Discovery/Coverage | 01,12,10 | Manifest, WIKI_COVERAGE und Pflichtlückenreport G3 |
| R27 | Strukturierte Data-/Module-/Templatequellen vor HTML; sichere Parser | U3 ETL | 12,04 | Quellfixtures, Typ-/Abhängigkeitstests, keine fremde Codeausführung |
| R28 | Heroes, Abilities, Items, Stats und sprachneutrale stabile IDs | U3 Modell/Resolution | 02,03,05,12 | Contract- und Alias-/Rename-/FK-Tests |
| R29 | Typisierte Effects, Units, Conditions, Scaling, Varianten und Stacking | U3 Effektmodell | 02,03,05,12 | AST-/Unit-/Unknown-/Rundungstests |
| R30 | Mechanikgraph und belegte Synergien statt bloßer Modellmeinung | U3 Synergien | 05,06,12 | Kanten/Inputs/Versionen und Ableitungstypen |
| R31 | Versionierte deterministische Buildregeln und legaler Planner | U3 Buildengine | 05,08,10 | alle modellierten Hard Constraints, kleine exakte Referenzfälle |
| R32 | Patch/Mode/Variante von Quellrevision und Abrufzeit trennen | U3 Versionierung | 02,03,05,12,13 | Historical-/Unknown-/Current-/Mode-Tests |
| R33 | Hero-Wissenskarte als erzeugte kompakte Sicht auf Fakten und Regeln | U2 Heldenverständnis + U3 | 12,05,08 | rebuildbare Karte mit Fact-/Rule-/Sourcebezug |
| R34 | Lokalisierte Namen/Texte ohne separate numerische Wahrheit | U3 Mehrsprachigkeit | 03,05,12 | DE/EN, Alias und Übersetzungsherkunft |
| R35 | Strukturierte/Graph-/Prosa-Evidenz gemeinsam abrufen | U3 RAG | 06,08 | Mixed-Evidence-Evals und ein Knowledge-Release |
| R36 | Unbekannte Entities/Werte/Regeln sicher behandeln | U3 Golden Tests | 05,08,10,12 | Unknown != 0, keine erfundenen Builds/Stats |
| R37 | Numerische Golden-Aussagen vollständig belegen | U3 Grounding | 05,08,10 | Fact-/Berechnungsevidenz und Citationtests |
| R38 | Template-/Data-/Fact-Änderungen gezielt invalidieren | U3 Delta | 04,06,12,13 | unverändert kein Reembedding, Änderung aktualisiert Abhängigkeiten |
| R39 | Wiki-/Text-/Game-/Medienrechte und Abruffreigabe separat | U3 Risiken | 00,01,12,13 | Quellenpolicy und sichtbarer Blocker vor betroffener Publikation |
| R40 | Staging → Validierung → atomare Knowledge-Veröffentlichung | U3 CI/CD | 03,04,08,11 | konsistenter Release und getestetes Rollback |
| R41 | Lore/Guides/History/Redirects/Medienmetadata nicht still auslassen | U3 Gesamtumfang | 01,03,12 | je Klasse Coverage, Übernahme oder ausdrücklicher Ausschluss |
| R42 | Source Contract v2 einschließlich Ableitungsfamilien | U4 nächste Schritte | 02,03,13 | Revision/Zeiten/Hash/Parser/Upstream/Policy-Contracttests |
| R43 | Korrelation von Quellen und Parserfamilien abbilden | U4 Risiken | 03,05,13,14 | Rootartefakt-/Parserbezüge; kein Mehrheitsvote aus Dubletten |
| R44 | Vorhandene API/Assetsadapter härten, nicht duplizieren | U4 Integrationsarchitektur | 01,13 | Ist-Adaptervergleich und gepinnte Contractfixtures |
| R45 | Deterministische API-Tests plus separater Drift-Watcher | U4 APIverträge | 02,13,10 | Breaking-/Unit-/Enum-/Ausfalltests ohne Live-CI-Flakiness |
| R46 | deadlock-data als versionierter Git-Historienfeed | U4 deadlock-data | 03,13 | Commit→Diff→Snapshot und rekonstruierbare History |
| R47 | Parser-/Generatoränderungen nicht als Spielpatch ausgeben | U4 Derivation | 05,12,13 | Same-Raw/New-Parser vs. New-Raw-Differenztests |
| R48 | GameTracking/SchemaExplorer/Protoüberwachung | U4 Schemawatch | 13,14 | gepinntes Schema, betroffene Mappings invalidiert |
| R49 | Rust-Replaypilot und normalisierte Beobachtungen | U4 Replay | 14,03,04 | Raw→Decoder→Observation mit Feld-/Patchcapabilities |
| R50 | Golden-Replay-Gegenprüfung ohne Fremd-Produktionsruntime | U4 Referenzparser + U2 Rust | 14,10 | 10–20 Startfälle bzw. freigegebener Umfang, Referenzreport |
| R51 | Rohreplays/Ticks nicht pauschal relationalisieren oder embedden | U4 Speicherdesign | 03,14 | zweistufige Speicherung und messbare Ressourcenlimits |
| R52 | Empirische Population getrennt von Mechanikfakten | U4 Buildreasoner | 05,13,14 | Kohorten/n/Missingness/Bias/Quellstatus |
| R53 | Zeitliche Match-Holdouts ohne Zukunftswissen | U4 Evals | 05,10,14 | Match-Split, T-Verfügbarkeit, Release-/Datensatzfreeze |
| R54 | Jedes externe Repo als Feed/Abhängigkeit/Referenz einordnen | U4 Repolandschaft | 01,13,14 | Quellenregister ohne blindes Vendoring |
| R55 | Code-, Daten-, Assetrechte und Security früh prüfen | U4 Governance | 00,01,02,13 | Policy-/Dependency-/Freigabecheck G0/G1; kein Rechtsfreibrief |
| R56 | Discord/Build-Publishing als Consumer berücksichtigen | U4 eigenes Ökosystem | 01,08,09,11 | Ist-Verträge, E2E und freigegebener Export/Publishpfad |
| R57 | Docs-Prüfprozess und echte deutsche Support-Evals erhalten | U4 Docs | 01,09,10 | versionierte Evals, Commit-/Hash-/Redaction-Freigabe |
| R58 | Funktionierendes vorhandenes Rust erhalten | U4 vs. U1 Ist-Differenz | 01,02,05 | Pfad-/Owner-/Paritätsmatrix statt kosmetischem Rewrite |
| R59 | Neue Quellen/Replays dürfen Liveantworten nicht verdrängen | U2 Performance + U3/U4 | 04,10,14 | gemischte Last, Queue-/RAM-/CPUbudgets und p95/p99 |
| R60 | Alle drei Recherchen in einheitlichen Gates und Chatwellen | U2 | 00,alle | 10 Parallelplan, freigegebene Commits, G2/G3/G4 erweitert |
