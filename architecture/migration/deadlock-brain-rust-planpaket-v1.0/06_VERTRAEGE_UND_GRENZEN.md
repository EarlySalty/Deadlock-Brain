# Vertragsentwurf und Architekturgrenzen

Dies sind verpflichtende semantische Anforderungen an die in Chat 02 zu implementierenden Rust-/JSON-Verträge. Es ist noch keine veröffentlichte OpenAPI-Spezifikation. Unabhängige Chats dürfen daraus keine abweichenden privaten Typen ableiten; der erste integrierte Contract legt Namen und Wireformat verbindlich fest.

## Request und vertrauenswürdiger Kontext

Öffentlicher Request: `query`, `requested_scopes`, `locale`, `conversation_id?`, `response_profile`. `channel` darf als Darstellungshinweis vorkommen, ist aber kein Berechtigungsbeweis. User-/Serviceidentität stammt aus geprüften Credentials, nicht aus `actor.type = developer` im JSON. Conversation-Zugriff separat prüfen.

Der Server bildet `AuthorizedContext`: verifizierter Principal, zulässige Scopes und Objekt-ACLs, Policyversion, Provider-Egress-Regeln, Deadline, Budget, erlaubte Antwortprofile und Trace-ID. Serverseitig gesetzte Obergrenzen können vom Client nur enger, nie weiter gewählt werden.

```text
effective_sources ⊆ authenticated_sources
routed_sources    ⊆ effective_sources
model_inputs      ⊆ authorized_evidence ∩ provider_egress_allowlist
```

Eine explizite Anfrage nach unzulässigen Scopes wird konsistent abgelehnt oder nach dokumentierter API-Regel eingeschränkt. Jev darf nicht aus einer hohen Wahrscheinlichkeit für „interne Frage“ eine Berechtigung ableiten.

## Antwort

`AnswerResponse` enthält Status, Text, zugängliche Zitate, Unsicherheits-/Degradierungsinformation und Trace-ID. Beispiele für fachliche Status: beantwortet, unzureichende Evidenz, Quelle zu alt, temporär nicht verfügbar. Auth-/Validierungs-/Rate-Limit-Fehler haben eigene API-Fehlerklassen.

Detaillierte Evidence Packs, Retrievalscores, Modellkonfiguration, interne Pfade und Kosten sind nicht automatisch Bestandteil einer öffentlichen Antwort. Usage-/Debugdaten werden nach Rolle gefiltert. Ein Feld `confidence` wird entweder sauber definiert und kalibriert oder bleibt optional; Jev-Noul ist keine automatisch gültige Gesamtantwort-Confidence.

## Evidenz

`Evidence` unterscheidet Passage, strukturierte Tatsache und berechnetes Ergebnis. Alle Varianten tragen eine stabile Evidence-ID, zulässige Sichtbarkeit, Quell-/Datenrevision, Gültigkeit, Provenienz und Corpusrelease. Passagen haben Document-/Chunkrevision und genaue Position. Berechnungen haben Inputreferenzen, Algorithmusversion, Einheiten und Resultat.

Das finale Modell darf nur IDs aus dem bereitgestellten Evidence Pack zitieren. Der Kernel prüft referenzierte IDs, zulässige Versionen, Positionen und Zugriffsrechte. Diese strukturelle Prüfung beweist noch nicht, dass jede Behauptung semantisch belegt ist; Supportprüfungen und Evaluierung sind zusätzlich erforderlich. Ein öffentlicher Quellenlink darf keine interne Dateistruktur verraten.

## Interne Ports

| Port | Aufgabe | Nicht erlaubt |
|---|---|---|
| `PolicyEngine` | Auth-Kontext, Scope-/Objekt-/Egress-Entscheid | Modellbasierte Freigabe |
| `SourceConnector` | versionierte Quellrecords und Änderungen lesen | selbständig veröffentlichen |
| `DocumentStore` / `DomainStore` | kanonische Revisionen/Fakten/Checkpoints | AI-Prompting |
| `EmbeddingProvider` | dokumentierte Vektorrepräsentation | stiller Modellwechsel |
| `Retriever` | nur erlaubte Kandidaten mit Provenienz | eigene finale Antwort |
| `DecisionProvider` | Jev-ähnliche typisierte Entscheidungen | Authentifizierung oder Textantwort |
| `AnswerProvider` | aus erlaubter Evidenz formulieren | unkontrollierter Tool-/Quellenzugriff |
| `AnswerService` | gesamter Antwortablauf | Rückdelegation an Legacykernel |

Objektsicherheit und Async-Trait-Gestaltung in Chat 02 bewusst entscheiden und kompilieren; ein ungeprüfter Pseudocode-Trait ist kein kompatibler Port. Providerclients liegen hinter Ports, nicht in Domainmodulen oder Adaptern. Runtime-wirksame Konfiguration kommt aus dem serverseitigen Composition Root der API/Worker.

## Versionierung

Wirevertrag, DB-Schema, Normalisierer, Chunker, Embeddingmodell/-revision/-dimension/-pooling/-präfix/-distanz/-normalisierung, Suchgeneration, Prompt-/Jev-Rubrik und Policyversion getrennt versionieren. Gleiche Vektordimension allein bedeutet keine Modellkompatibilität. Mutable Provideraliase im Trace auflösen; reproduzierbare Releases verwenden nach Möglichkeit feste Modellrevisionen.

## Fehler- und Budgetgrenzen

Gesamtdeadline, Teilbudgets, maximale Kandidaten/Token, begrenzte Retries mit Backoff und Jitter sowie Circuit Breaker zentral definieren. Laufende Arbeit bei Abbruch soweit technisch möglich beenden; blockierende CPU-Arbeit kooperativ begrenzen, statt eine nicht vorhandene Abbruchgarantie anzunehmen.

Bei Jev-Ausfall: getestete sichere Retrieval-/Regelbaseline oder kein belastbares Ergebnis. Bei nicht erlaubtem externem Provider: nur ausdrücklich freigegebener alternativer Pfad, sonst keine Datenübermittlung. Bei veralteter/inkonsistenter Indexgeneration: Fehler bzw. freigegebene konsistente ältere Generation unter aktueller ACL, kein beliebiger Datenmix.


## Gemeinsamer Quellen- und Wissensvertrag v1.0

**Vor G1 fixieren, nicht getrennt in 12/13/14 neu definieren.** Die folgenden Namen sind semantische Vorschläge; 02/03 passen sie an bestehende geeignete Typen an.

| Vertrag | Pflichtsemantik |
|---|---|
| `SourceRecordV2` | source_id, source_kind, source_revision, url/path, content_hash, retrieved_at, observed_at; published_at/available_at soweit belegbar; parser_name/version, upstream refs, derivation_family, root_artifact refs, parser_family, trust_class, access/license/egress policy, schema fingerprint und validation status |
| `GameValidity` | game_version, patch_id oder unknown, mode/variant, valid_from/to, Zuordnungsbeleg und uncertainty; Sourcezeit ist keine Gamezeit |
| `Fact` | stabile entity-/stat-/predicate IDs, typisierter Wert/Einheit/Operation, Bedingungen, GameValidity, Source-/Derivationsreferenzen und Prüfstatus |
| `Effect` / `Rule` | begrenzte typisierte AST, Trigger/Bedingung/Folge, Hard-/Soft-Klasse, versionierte Grenzen, Evidenz und Evaluatorversion |
| `SynergyEvidence` | source-explicit/mechanical/rule-derived/empirical/model-hypothesis, Input-Facts/Observations, Regel/Algorithmus, Kontext und Unsicherheit |
| `HeroKnowledgeCard` | Hero-ID, Locale, Patch/Mode, Knowledge-/Generatorversion, Fact-/Ability-/Mechanik-/Rule-Refs, getrennte Strategiehinweise, Unknowns und Quellen |
| `ReplayArtifact` / `Observation` | Hash/Location, Match, Feld-Capabilities, Parser/Schema, Tick-/Zeitbasis, Entity/Event, Unit, Rohlocator, Ableitung, Rechte und Validierung |
| `PopulationSlice` | deduplizierte Matches, Fenster/Kohorte/Patch/Mode, n, Missingness, Quelle/Ableitung, Coverage und Unsicherheit |
| `SchemaChange` / `ReconciliationCase` | alte/neue Revision, betroffene Felder/Abhängigkeiten, Konfliktstatus, Quarantäne-/Revalidierungsentscheid |
| `KnowledgeExport` | stabile Knowledge-ID, Source-/Fact-/Rule-Set, Knowledge-/Patchversion, Freigabestatus, bereinigte Sichtbarkeit und Exporthash |

`CorpusRelease` wird zum übergreifenden Knowledge-Release erweitert oder entsprechend aliasiert; **keinen zweiten unabhängigen Releasezeiger erzeugen**. Es pinnt nun auch Fact-/Rule-/Mechanik-/Karten-/Observation-/Populationversionen. Ein Query/Build/Export sieht zusammenpassende Versionen. Aktuelle ACLs/Löschungen gelten unabhängig vom historischen Release.

`SourceRecordV2` darf unbekannte Felder explizit als unbekannt tragen, keine erfundenen Zeit-/Versionsangaben. Pflichtregeln vor Aktivierung streng prüfen. Facts mit widersprüchlicher Gültigkeit/Mode nicht ungeprüft zusammenführen.

Zusätzliche Ports: `WikiExtractor`, `ExternalSourceAdapter`, `ReplayDecoder`, `ObservationStore`, `RuleEvaluator`, `BuildPlanner`, `HeroCardProjector`, `KnowledgePublisher`. 12–14 implementieren Quellenports, 05 fachliche Evaluierung/Planung, 03 Stores, 04 orchestrierte Veröffentlichung, 08 API-Fassade; 02 besitzt die Typdefinitionen. Keine separaten Providerwege oder unkoordinierte Publisher.

Build-/Katalog-Requests führen Patch/Mode und geben die aufgelöste Version zurück. Nach Nutzerkontext zusätzliche Budget-/Inventarparameter validieren. Fach-Read-/Buildendpoints sind zulässig, solange alle AI-Antworten und Berechtigungen dieselben Kernverträge nutzen.
