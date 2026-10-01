# Masterplan: ein Brain, eine Datenstrategie, Rust als Kern

> Planversion 1.0: Konsolidierung/Jev (U1), Wiki (U3), Ökosystem/Replays (U4) und alle Nutzeranforderungen (U2) gemeinsam eingeplant. Ausführung und reale Abnahme stehen noch aus.

## 1. Verbindliches Ergebnis

`Deadlock-Brain` wird das einzige aktive Repository für die eigene Brain-/AI-Infrastruktur. Ein zentraler Answer Kernel verantwortet Routing, Retrieval, Evidenzauswahl, Antwortmodell, Quellenprüfung und Antwortprofile. Twitch, MCP, CLI, Web und interne Agents bekommen keine zweite AI-Architektur.

Der eigene produktive Backend-Code wird Rust: Domainfunktionen, Ingestion, Feeder, Retrieval, Jev-/Provider-Clients, Scheduling, API, Adapter, Migrations- und Wiederaufbauwerkzeuge. Python ist nur für dokumentierte einmalige Konvertierung oder optionale lokale Hilfsanalysen zulässig. Kein produktiver Python-Prozess, kein PyO3-Kern, kein Python-Sidecar und kein versteckter Aufruf eines alten Python-Services.

Externe Datenbanken und Modell-APIs müssen nicht selbst in Rust implementiert sein. Rust steuert sie über geprüfte Schnittstellen. Auch native Nicht-Python-Bibliotheken sind kein verdeckter Python-Kern, benötigen aber einen dokumentierten Betriebs-/Lizenzentscheid. Eine Browseroberfläche oder statische Docs-Site darf JavaScript/TypeScript enthalten; serverseitige Brain-Logik nicht.

**Alle Daten berücksichtigen heißt: jede Datenquelle erfassen, klassifizieren, zuordnen und abgleichen. Es heißt nicht: alle Rohdaten, Geheimnisse, Caches und Millionen Datensätze in Git oder in einen Vektorindex kopieren.**

## 2. Zielstruktur

```text
Deadlock-Brain/
├── Cargo.toml / Cargo.lock / rust-toolchain.toml
├── apps/
│   ├── brain-api/          # Rust, HTTP und Zusammenbau des Kernels
│   ├── brain-worker/       # Rust, Ingest/Feeder/Jobs
│   ├── brain-twitch/       # Rust, nur Plattformadapter
│   ├── brain-mcp/          # Rust, nur Protokolladapter
│   ├── brain-cli/          # Rust, normale Anfragen über Brain API
│   └── docs-site/          # optional, Darstellung ohne AI-Entscheidungen
├── crates/
│   ├── brain-contracts/    # gemeinsame Typen, keine Providerabhängigkeit
│   ├── brain-policy/       # Auth-Kontext, ACLs, Provider-Egress
│   ├── brain-storage/      # Datenspeicherung, Versionen, Checkpoints
│   ├── brain-ingestion/    # Parser, Normalisierung, Chunking, Feeder
│   ├── brain-domain/       # Entitäten, Builds, Learning, Optimizer,
│   │                      # Coaching, Meta, Analytics, Lineage
│   ├── brain-retrieval/    # lexical+dense+strukturierte Fakten
│   ├── brain-providers/    # LLM-/Embedding-/HTTP-Integrationen
│   ├── brain-jev/          # typisierte Entscheidungen, kein Chatmodell
│   ├── brain-kernel/       # einziger Antwort-Orchestrator
│   └── brain-client/       # API-Vertrag für dünne Adapter
├── knowledge/             # nur Inhalte, deren Git-Sichtbarkeit erlaubt ist
│   ├── public/
│   ├── internal/          # nur bei geeignetem privaten Repo
│   └── generated/         # eigene Provenienz und Vertrauensklasse
├── tools/brain-admin/     # Rust: Migration, Verify, Rebuild, Replay
├── evals/                 # versionierte und bereinigte Testdaten
├── infra/                 # Deployment, Migrationsdateien, Monitoring
└── architecture/migration/# dieses Planpaket + Fortschritt/Übergaben
```

Das ist ein Vorschlag für einen Cargo-Workspace, keine Pflicht zu einem Prozess pro Crate. API und Worker sind zunächst die getrennten Ressourcenbereiche; zusätzliche Netzwerkdienste müssen einen gemessenen oder betrieblichen Nutzen haben. Gute vorhandene Rust-Komponenten werden nach Prüfung wiederverwendet.

## 3. Ein gemeinsamer Anfrageweg

```text
Adapter → Authentifizierung → serverseitig erlaubte Scopes/ACLs
        → Query-/Entity-Normalisierung
        → exakter Domain-/Cache-Pfad ODER Retrievalpfad
        → optionale Jev-Routingentscheidung innerhalb erlaubter Quellen
        → lexical + dense + strukturierte Fakten
        → optionale Jev-Relevanzauswahl
        → versioniertes Evidence Pack + Answerability
        → erlaubtes Antwortmodell ODER präzise Keine-Evidenz-Antwort
        → Quellen-/Supportprüfung → Antwortprofil → Adapter
```

Jev kann keinen Scope freischalten, kein Geheimnis freigeben und keinen beliebigen Provider auswählen. Es liefert Vorschläge; normale Rust-Regeln setzen diese innerhalb des zulässigen Budgets um. Sichere exakte Fakten brauchen nicht automatisch Jev und ein generatives Modell.

## 4. Chats und Liefergegenstände

| Chat | Verantwortung | Wesentlicher Nachweis |
|---|---|---|
| 00 | Architekturführung und Integration | freigegebene ADRs, STATUS, Release-Gates |
| 01 | Ist-Zustand, vollständiges Daten-/Funktionsinventar, Baseline | belegte Source-/Feature-Matrix und Messprofil |
| 02 | Rust-Workspace, Verträge, Policy und CI-Grundlage | baubares Fundament und Contract-/Auth-Tests |
| 03 | Storage, Datenmodell und Gesamtdatenmigration | wiederanlaufbare Migration mit vollständigem Abgleich |
| 04 | Rust-Ingestion, Connectors und Brain Feeder | idempotente Updates, Löschungen und Rebuild |
| 05 | vollständiger Rust-Port der Domainfunktionen | Feature-Paritätsmatrix und Differenztests |
| 06 | Hybrid Retrieval und Suchindexentscheidung | reproduzierbare Recall-/Latenz-/Speichervergleiche |
| 07 | LLM-/Embedding-Provider und Jev in Rust | HTTP-Vertragstests, Fehlerpfade und Shadow-Auswertung |
| 08 | Answer Kernel und produktive API | ein konsistenter Ende-zu-Ende-Antwortpfad |
| 09 | Rust-Adapter und Docs-Anbindung | alle Consumers ohne eigenen Modell-/RAG-Pfad |
| 10 | Qualität, Sicherheit und Performance | unabhängige Release-Gates und Lastberichte |
| 11 | Produktivumstellung und Legacy-Abbau | Cutover-/Rollback-Nachweis und Python-freier Betrieb |
| 12 | Wiki-Discovery, Parser und Wissenskarten | Gameplay-Coverage, Facts/Provenienz, erzeugte Hero-Wissenskarten |
| 13 | Externe Datenquellen, Contract-Drift und Schemaüberwachung | versionierte API-/Git-Feeds und nachvollziehbare Schemaänderungen |
| 14 | Rust-Replays und Beobachtungen | validierte Replay-Events, begrenzte Ressourcen und Population-Zulieferung |

## 5. Reihenfolge und sinnvolle Parallelität

Verbindliche Startmatrix: [10 Reihenfolge und Parallelität](10_REIHENFOLGE_UND_PARALLELITAET.md). Chatnummern sind IDs, nicht Ausführungsreihenfolge.

**A — Start:** 00 initialisiert; danach 01 Inventar und 10 Testdesign parallel. Keine produktiven Veränderungen.

**B — Fundament nach G0:** 02 Workspace/Verträge und 03 Schemaentwurf mit eindeutig verteilten Dateien. 06/07/12/13/14 dürfen abgegrenzte Recherche, kleine freigegebene Fixtures und Prüfentwürfe vorbereiten. G1 verlangt auch Source Contract v2, Facts/Rules, Wissenskarten, Replay-Observation und Knowledge-Release-Verträge.

**C — Implementierung nach integriertem G1:** 03/04/05/06/07/12/13/14 arbeiten in getrennten Modulen. 08 baut zunächst gegen feste Testports; 09 gegen Contract-Test-API. 10 prüft laufend. Echte Integration bleibt offen, solange reale Abhängigkeiten fehlen.

**G2 — echter Pilot:** Eine öffentliche und eine interne Quelle, Wiki-Hero mit Abilities/Items/Mechaniken, ein externer historischer Datenfeed, ein Replay-Beispiel, ein Adapter sowie Delete-/ACL- und Konfliktfall laufen durch den Rust-Pfad. Kern-Contracts und Such-/Embeddingkonfiguration sind gemessen und festgelegt. Keine teure Vollmigration davor.

**D — Gesamtdaten und Funktionserhalt:** 03/04/12/13/14 übernehmen die freigegebenen Korpora und laufenden Updates; 05 schließt Fachparität und den Build-/Population-Ausbau; 08/09 integrieren echte Quellen und Consumers. G3 verlangt vollständige Pflicht-Coverage, Graph-/Buildtests, Replay-Referenzsuite und reale Consumerparität im Staging. Jev bleibt zunächst Shadow.

**E — gemeinsame Abnahme:** 10 misst Qualität, Datenschutz, Datenaktualität und Performance unter gemischter Last; 11 probt Deployment/Rollback. G4 enthält ausdrücklich Wiki-, Quellen-/Schema- und Replay-/Population-Nachweise. Erst danach ist produktiver Cutover zulässig.

**F — Abschluss:** 11 führt mit ausdrücklicher Freigabe Canary/Writer-Cutover aus (G5). Nach Betriebskontrolle und freigegebenem Rückfallfenster folgen Legacy-Ende/Archivierung (G6). Die übrigen Chats beheben gezielt Abnahmefehler; kein paralleler ungeprüfter Umbau während des Cutovers.

## 6. Entscheidungen nicht endlos offenlassen

| Entscheidung | Vorschlag / Bedingung | Fällig |
|---|---|---|
| Rust-Fundament | Cargo-Workspace, Tokio, axum; Versionen nach Prüfung pinnen | G1 |
| Repo-Sichtbarkeit | interne Daten niemals in öffentlich lesbarer Git-Historie | vor Import |
| Kanonischer Store | bestehenden geeigneten Store bevorzugen; PostgreSQL als Startkandidat | G1 |
| Lexical Retrieval | echte BM25-Variante mit Tantivy gegen einfache FTS-Baseline prüfen | G2 |
| Dense Retrieval | bestehenden geeigneten Store vs. pgvector prüfen; separater Dienst nur bei belegtem Bedarf | G2 |
| Embeddings | Modellrevision, Dimension, Normalisierung, Distanz und Vorverarbeitung festschreiben | G2 |
| Jev | implementiert, standardmäßig Shadow/aus; Aktivierung pro Funktion nach Nutzenmessung | G4 |
| SLOs/RPO/RTO | konkrete Werte aus Hardware, erwarteter Last und Betriebserfordernis freigeben | G0/G1 |

PostgreSQL-FTS ist nicht automatisch BM25. Ein akzeptierter FTS-only-Endzustand wäre eine bewusste Abweichung vom BM25-Vorschlag, keine versteckte Erfüllung. Search-Store und Modellentscheidung nicht gleichzeitig mit der Sprachmigration ändern, ohne vergleichbare Testvarianten zu behalten.

## 7. Was „fertig“ bedeutet

Ein Repository allein genügt nicht. Fertig ist das Vorhaben erst, wenn alle freigegebenen Datenquellen entweder vollständig übernommen oder begründet eingeordnet sind, alle benötigten Funktionen Rust-Parität haben, jeder Consumer denselben Kern nutzt, Berechtigungen auch bei Cache/Providerfehlern halten, der Betrieb ohne Python funktioniert und die freigegebenen Performanceziele unter realistischer Last nachgewiesen sind.

## 8. Gemeinsame Wissensarchitektur und Erweiterungen

[11 Wiki/Wissen/Builds](11_WIKI_WISSEN_UND_BUILDS.md), [12 externe Quellen](12_QUELLEN_UND_SCHEMAWATCH.md) und [13 Replays](13_REPLAYS_UND_POPULATION.md) sind Pflichtbestandteile dieses Projekts, keine separat verschobenen Folgeprojekte.

Alle Eingaben laufen über unveränderte, versionierte Rohquellen mit Berechtigungen und Provenienz. Daraus entstehen getrennt: strukturierte Facts/Rules und Mechanikbeziehungen, erklärende RAG-Passagen, empirische Beobachtungen/Population sowie schwache Community-Claims. Der Answer Kernel kombiniert zulässige Evidenz aus einem konsistenten Knowledge-Release; reine Zahlenberechnung und Build-Legalität erledigt Rust, nicht ein LLM.

Die Hero-Wissenskarte ist eine kompakte maschinenlesbare Projektion mit erklärender Ansicht. Sie hängt an denselben Fact-/Rule-/Source-IDs, Patch-/Mode- und Releaseversionen wie der Planner. Keine zweite Datenbank aus handgeschriebenen Markdownkarten.

## 9. Bestehende Rust-Struktur erhalten

Die Ordner oben bilden logische Verantwortungen ab. U4 berichtet bereits über `dbrain-sources`, `dbrain-normalize`, `dbrain-enrich`, `dbrain-retrieval`, `dbrain-reasoner`, `dbrain-builds`, `dbrain-learn` und `dbrain-population`. Ob diese im eingesetzten Commit funktionieren, prüft 01. 02 ordnet die Verantwortungen in `PFAD_OWNER.csv` bestehenden Dateien zu, bevor mehrere Chats schreiben. Kein zweites `brain-domain` neben funktionierenden Fach-Crates nur für eine neue Namenskonvention.

Für neue Wiki-, externe Quellen- und Replaymodule getrennte Pfade reservieren. Keine Pflicht zu einem Service pro Modul. Aufwendiges Replayparsing, Bulkimporte und Reembedding erhalten getrennte Ressourcenbudgets vom interaktiven APIpfad.

EXT-01 ist als Eingangs-/Planungsfrage geschlossen; siehe [08](08_ERGAENZUNGEN_INTEGRIERT.md). Tatsächliche Live-Verträge, Rechte, Abhängigkeiten und Performance bleiben zu prüfen, nicht durch die Recherchen automatisch bewiesen.
