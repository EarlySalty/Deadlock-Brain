# S08: Answer Kernel und API, Vorbereitung

Stand: 24. September 2026. Auftrag: ausschließlich S08 aus Planpaket 1.0.

**Implementierung blockiert.** Dieses Paket liefert einen Ablaufentwurf, einen E2E-Prüfplan und Schnittstellenanforderungen. Es implementiert weder einen Kernel noch eine API, private Ersatzverträge oder ausführbare Mockports. Kein `mock_verified`, kein `integration_verified`, keine G2-/G3-Abnahme.

## Geprüfte Ausgangslage

| Merkmal | Belegter Stand |
| --- | --- |
| Integrierte Arbeitsbasis | `c00fc8935048bf490c1e4790f7c6195864ad49e2`, `origin/main` nach Fetch am 24.09.2026 |
| Arbeitsbranch | `migration/s08-answer-kernel-20260924` |
| Planpaket | Version 1.0, SHA256 `945be6983ff0235eb0ec36b451f9613ea567e0962ca5658364e8665980177716` |
| S08-Auftrag | `chats/08_ANSWER_KERNEL_API.md`, SHA256 `335202a5ffd5d55d8cb466b9ec7bf93985a170c795f1c8f6940a320dd9955699` |
| Contract-/DB-Schema-Version | Im integrierten `STATUS.md` offen; keine lokale Versionsvergabe |
| Knowledge-/Corpus-/Policy-Version | Nicht für G1 festgelegt; keine produktive Version abgefragt |
| G0 / G1 | In `GATES.csv` beide `offen` |
| S08-Codepfad | In `PFAD_OWNER.csv`: `unresolved_in_S000`, `pending_inventory`, Arbeit erst `G1_mocks_then_real` |
| Entscheidung | Nur Vorbereitung gemäß Ablaufentwurfsrecht aus Plan 10; keine selbst erteilte Implementierungsfreigabe |

Maßgeblich sind [STATUS](../STATUS.md), [GATES](../GATES.csv), [Pfadbesitzer](../PFAD_OWNER.csv) und [ADRs](../ADR_REGISTER.md) an der genannten Basis. S000 gibt keine Implementierung frei. Der allgemeine Ablaufentwurf ersetzt nicht die von 00 noch zu bestätigende S08-Startfreigabe.

Gelesen wurden der S08-Auftrag, Plan 00/01/02/06/07/08/10/11/12/13 und die Übergabe von 00. Das bereitgestellte ZIP stimmt im Hash mit der in S000 genannten Quelle überein. Es wird nicht erneut entpackt in die Git-Historie geschrieben. Offene fremde PRs und uncommittete Änderungen im kanonischen Checkout sind keine integrierte Grundlage und wurden nicht übernommen.

## Dateiumfang

Nur diese fünf S08-Artefakte werden ergänzt:

- `architecture/migration/s08/README.md`
- `architecture/migration/s08/E2E_ABLAUF.md`
- `architecture/migration/s08/TESTFAELLE.csv`
- `architecture/migration/s08/CR-S08-001.md`
- `architecture/migration/handoffs/08-answer-kernel-api.md`

Gesamtstatus, Gates, ADR-Register, Ownerregister, Cargo-Manifeste, Lockfile, Workflows, SQL, Quellcode und Betriebsdateien bleiben unverändert. Die S08-Übergabe und der Änderungsantrag sind Vorschläge an 00, keine Änderung gemeinsamer Freigaben.

## Wiederverwendung statt Parallelbau

Graphify wurde zuerst global abgefragt. Treffer dienen nur als Kandidaten; anschließend wurden die folgenden Stellen im tatsächlichen Quellcode gelesen. Alle Brain-Zeilen beziehen sich auf die integrierte Basis oben, nicht auf einen Live-Nachweis.

| Bestehender Baustein | Codebeleg | Konsequenz für S08 / Besitzer |
| --- | --- | --- |
| Bestehender Workspace mit elf Mitgliedern | `rust/Cargo.toml:1-17` | Vorhandene `dbrain-*`-Namen erhalten. 02 bestimmt den tatsächlichen Kernel-/API-Ort. |
| Gemeinsame Core-Module | `rust/crates/deadlock-brain-core/src/lib.rs:3-18` | Gemeinsame Verträge bei 02, keine S08-Kopie der Typen. |
| CLI-Kontext, Timeline, Review, AskContext | `rust/crates/deadlock-brain/src/main.rs:1186-1255` | Bereits vorhandene Aufrufe dokumentieren. Umstellung mit 09, nicht den unter 04/09 reservierten Pfad eigenmächtig ändern. |
| Abfrageplanung und Kontextaufbau | `rust/crates/dbrain-retrieval/src/lib.rs:303-333`, `:997`, `:1723` | `QueryPlan`, `AskContextOptions`, `ask_context`, `analyze_query` sind Wiederverwendungskandidaten bei 06/05. |
| Fakten-/Timeline-/Itemzugriff | `rust/crates/dbrain-retrieval/src/lib.rs:479`, `:548`, `:646`, `:1569` | Passende bestehende Abfragen über freigegebene Store-/Domain-/Retrievalports integrieren, nicht neu implementieren. |
| Antwortprompt liegt derzeit im Retrieval | `rust/crates/dbrain-retrieval/src/lib.rs:51-73`; CLI gibt den Prompt in `main.rs:1249-1253` aus | Verantwortungsverschiebung mit 06/09 nötig. S08 ändert die fremden Dateien nicht. Bestehende Prompttexte sind kein Beweis für Rechte- oder Supportprüfung. |
| Wiki-Kontext und Hero-/Ability-Auswahl | `rust/crates/dbrain-retrieval/src/game_wiki.rs:114-156` | Bestehende Suche behalten; Knowledge-Release und aktuelle Rechte benötigen den gemeinsamen Port von 02/03/06/12. |
| Deterministischer Reasoner | `rust/crates/dbrain-reasoner/src/lib.rs:64-92`, `:174`; Verbrauch in `main.rs:1640-1661` | Fachberechnungen und Legalität bleiben bei 05. Kernel erklärt nur freigegebene Resultate. |
| Bestehender AI-Client und Modellauflösung | `rust/crates/deadlock-brain-core/src/ai.rs:124-201` | 07 besitzt die Anpassung. Keine eigenen Credentials, Modelle oder HTTP-Clients in S08. |
| Blockierender Modellaufruf | `rust/crates/deadlock-brain-core/src/ai.rs:153-160` verwendet Thread-Spawn und synchrones Join | Abbruchfähigkeit nicht aus einer zukünftigen Async-Signatur ableiten. 07 muss Deadline-/Cancellation-Verhalten im gemeinsamen Port belegen. |
| CLI formuliert Buildantworten direkt | `rust/crates/deadlock-brain/src/main.rs:1400-1422`, `:1449-1469` | Reale Umstellung mit 05/07/09 erforderlich, bevor alle Consumer als kernelgebunden gelten. |
| Vorhandenes Python-MCP im integrierten Stand | `mcp/server.py:13-16`, `:43-70` | Nicht importiert oder gestartet. Kein S08-Fallback daraus; Migration gehört 09/11. Das ist keine Aussage über den aktuell laufenden Dienst. |

Zusätzlicher, ausdrücklich **nicht integrierter** Bestand: Im lokalen `Deadlock-Bots` existiert `rust/crates/dl-answer/src/lib.rs` mit `AnswerEngine`, `Answer`, `Retriever` und `answer_with_context`. Graphify-Treffer und anschließende Dateilesung bestätigen den Kandidaten, nicht dessen G1-Kompatibilität oder aktuellen Betriebsstatus. 01/02/09 müssen die Übernahmegrenze und den gepinnten Quellstand festhalten. S08 kopiert diese Crate nicht und führt auch deren offenen Testmodus nicht als alternativen Antwortweg ein.

Die exakten geplanten Symbole `AuthorizedContext`, `EvidencePack`, `AnswerResponse`, `AnswerService` und `PolicyEngine` wurden im Rust-Quellbaum dieser Brain-Basis nicht gefunden. Dies ist eine Symbolsuche, kein Beweis, dass keinerlei verwandte Funktion existiert. Die verbindlichen Namen und Ports müssen durch 02 festgelegt werden.

## Vorgesehener gemeinsamer Ablauf nach G1

Die folgenden Begriffe beschreiben Semantik, **kein neues Rust-Trait, JSON-Schema oder veröffentlichtes Wireformat**.

1. Transport authentifiziert Credentials über die freigegebene Policy. Identität, Rollen, Objekt-ACLs, Egressregeln, erlaubte Profile, Budget, Deadline und Trace-ID kommen serverseitig. Angeforderte Scopes und `conversation_id` werden gesondert geprüft. Clientfelder dürfen keine Rechte oder Limits erhöhen.
2. Normalisierung über vorhandene 05-/06-Bausteine. `current` genau einmal auf den freigegebenen Knowledge-/Corpusrelease und dessen Patch-/Modeversionen auflösen. Historische Datenrechte bleiben aktuell. Bei unbekanntem Patch keinen bestätigten aktuellen Wert behaupten.
3. Vor jedem Zugriff aktuelle Rechte prüfen. Zunächst zulässiger exakter Fact-/Domain-/Cachepfad. Deterministische Werte mit Einheit, Datasetrevision, Gültigkeit und Quellenreferenz ohne LLM ausgeben. Ein Treffer aus dem falschen Release oder mit gesperrter Quelle ist kein gültiger Cachehit.
4. Andernfalls den autorisierten Retrievalport verwenden. Jev zunächst aus oder Shadow; Vorschläge dürfen Scopes, Provider, Evidence-IDs und Budget nicht erweitern. Zulässige strukturierte Fakten, Textstellen und berechnete Ergebnisse in dasselbe versionierte Evidence Pack aufnehmen.
5. Answerability vor Generierung prüfen. Fehlende notwendige Buildregeln, widersprüchliche Pflichtfakten, veraltete Daten und unerreichbare Evidenz unterscheiden. Höchstens ein zusätzlicher Retrievalpass innerhalb desselben Release, derselben Rechte und desselben Gesamtbudgets. Kein rekursives Nachfragen an Modelle.
6. Antwortmodell ausschließlich über 07 und nur für aktuell autorisierte, zum konkreten Provider exportierbare Eingaben aufrufen. Das gilt auch für Frage, Conversation, Zusammenfassungen und optionales Jev, nicht nur für Quellenpassagen. Kein Modellwechsel bei Fehlern ohne freigegebene Alternative.
7. Ausgabe strukturell und inhaltlich prüfen. Evidence-ID, Quellrevision, Chunkrevision, Position, Release und aktuelle Sichtbarkeit müssen stimmen. Jede fachliche Aussage benötigt passende Unterstützung; eine existierende ID reicht nicht. Rechenresultate und Builditems dürfen durch Prosa nicht verändert werden.
8. Autorisierung unmittelbar vor Ausgabe erneut prüfen, dann nach freigegebenem Profil formatieren. Auch Cachehits und wartende Mitnutzer durchlaufen diese Kontrolle. Falls Profilkürzung Bedingungen oder Belege entfernt, die gekürzte Fassung erneut validieren oder sicher ablehnen.

## Cache und zusammengefasste laufende Arbeit

Der exakte Schlüssel muss semantisch alle ergebnisrelevanten Unterschiede abdecken: normalisierte Anfrage samt Normalisiererversion, serverseitig überprüfte Principal-/Tenant-/ACL-Identität bzw. freigegebener Berechtigungsfingerprint, Policy- und Egressversion, aufgelöstes Knowledge-/Corpusrelease mit Patch/Mode und benötigten Index-/Fact-/Rule-/Generatorversionen, Antwortprofil samt Revision, Sprache, Conversation-ID **und** autorisierte Conversation-Revision/-Kontext. Wo Generierungs- oder Budgetkonfiguration das Ergebnis verändert, gehört auch sie in die Identität oder erzwingt getrennte Arbeit.

Identische Scopenamen oder identischer Fragetext allein reichen nicht. Fingerprints werden von der Policy gebildet, niemals ungeprüft aus einem Request übernommen. Cachewerte enthalten die zur erneuten Rechte-/Quellenkontrolle notwendigen Referenzen. Eine Rechteänderung oder ein Tombstone wirkt unabhängig von TTL und historischem Release. Kein längeres negatives Caching für vorübergehende Provider- oder Rechtefehler ohne explizite Policy.

Singleflight folgt derselben Identität. Jeder Wartende behält eigene Trace-ID, Autorisierung und Deadline. Ein Clientabbruch darf keine Ausgabe an diesen Client erzeugen; andere berechtigte Wartende dürfen weiterarbeiten. Ist kein berechtigter Wartender übrig, gemeinsame Arbeit soweit vom Provider unterstützt abbrechen. Abgelaufene Arbeit nicht für spätere Anfragen wiederverwenden. Queue, Cacheeinträge, Antwortgrößen und gleichzeitige Jobs bleiben begrenzt; Limits werden von 02/07/10 freigegeben, nicht in S08 erfunden.

## Zitate, Support und Fehler

Passagen benötigen stabile Document-/Chunkrevisionen und eindeutige Locator-Semantik. Byte-, Zeichen- oder Tokenpositionen sind nicht austauschbar; UTF-8-Grenzen gehören in den gemeinsamen Vertrag. Fact-Zitate verweisen auf den tatsächlichen typisierten Wert mit Einheit und Gültigkeit. Berechnungsevidenz verweist zusätzlich auf Inputs, Algorithmus und Ruleversion. Keine internen Pfade oder unzugänglichen Quellen in öffentliche Links umformen.

Semantische Supportprüfung ist getrennt von Referenzvalidierung: Negation, Bedingungen, Zahlen, Einheit, Patch, Mode und Unsicherheiten müssen zur Aussage passen. Ohne freigegebenen Supportnachweis keine Behauptung einer belegten Antwort. Keine kalibrierte Gesamt-Confidence aus Jev-Werten oder Modellselbstauskunft ableiten.

Fachzustände wie beantwortet, unzureichende Evidenz, Daten veraltet, Quellenkonflikt, unbekannter Hero, fehlende Regel und teilweise Replay-Capabilities müssen erkennbar bleiben. Authentifizierungs-, Autorisierungs-, Eingabe-, Ratenlimit-, Budget-, Deadline- und Providerfehler erhalten die von 02 festgelegte API-Abbildung. S08 legt vor G1 keine eigenen HTTP-Codes oder Enum-Werte fest. Öffentliche Fehler dürfen weder private Quellenexistenz noch Prompts, Providerantwortkörper oder interne URLs offenlegen.

## Profile und Consumer

Noch keine festgelegten Wire-IDs oder Maximallängen. Diese Profilanforderungen gehen an 02/09:

| Profilzweck | Darstellung | Unveränderliche Grenze |
| --- | --- | --- |
| Öffentliche kurze Antwort, etwa Twitch | Kurzer Text und zugängliche Quellen; knappe Hinweise auf fehlende Evidenz | Keine internen Details, keine verlorenen notwendigen Bedingungen |
| Öffentliche ausführliche Antwort, etwa Web/Docs | Begründung mit belegten Aussagen und Unsicherheiten | Gleicher Kernel, gleiche Zahlen und gleiche Zugriffsregeln |
| Interne Antwort | Nur zusätzlich tatsächlich freigegebene Inhalte | `internal` als Clientlabel gewährt nichts; Debugrechte separat |
| Strukturierte API/MCP/CLI | Status, versionierte Fakten/Berechnungen/Karten/Buildresultate und zugängliche Zitate | Keine zweite numerische Wahrheit oder eigene Consumerprompts |

Discord und Build-Publishing gehören zum Consumerinventar. Lesen/Erklären und Veröffentlichung sind unterschiedliche Befugnisse. Eine erfolgreiche Antwort darf keine automatische Publikation auslösen. Exports verwenden ausschließlich den freigegebenen Knowledgezeiger unter aktueller Rechteprüfung, niemals direkt ein Stagingverzeichnis. Consumeranpassungen bleiben S09, Freigabe und Cutover S11.

## Traces, Streaming und Abbruch

Pro Stufe: Trace-ID, Stufenstatus, Warte-/Laufzeit und freigegebene Versionskennungen; Token/Usage und Kosten nach Rollen und Policy. Nicht angefallene Usage im exakten Pfad wird von unbekannter Providerusage unterschieden. Cachehit und Singleflight-Wartezeit getrennt erfassen. Rohprompts, Conversationtext, Secrets und private Evidence-Inhalte werden nicht standardmäßig geloggt.

Vorgeschlagene sichere Erstvariante: Antwortinhalt bis zur Quellen-, Support-, Profil- und Rechteprüfung puffern. Vorher allenfalls freigegebene, inhaltsfreie Fortschrittsereignisse senden. Kein unvalidiertes sensibles Token vorab. Ob vollständig validierter Text danach gestreamt werden darf, einschließlich Revocation während der Ausgabe, entscheidet der gemeinsame Vertrag. Ohne ausdrückliche Entscheidung bleibt die API nichtstreamend. Kein HTTP-Streamingversprechen aus dem Verhalten eines Providerclients ableiten.

Deadline, Ratenlimits und Gesamtbudget schließen Queue, Retries, Jev, beide Retrievalpässe, Generierung und Finalvalidierung ein. Ein blockierender Providerthread ist nicht nachweislich beendet, nur weil der Requestfuture fällt. Der Abbruchvertrag von 07 muss zwischen beendetem Aufruf, best-effort Abbruch und gegebenenfalls noch anfallenden Kosten unterscheiden.

## Start- und Integrationsnachweise

[CR-S08-001](CR-S08-001.md) nennt die fehlenden Übergaben. [E2E_ABLAUF](E2E_ABLAUF.md) beschreibt den ersten Durchstich. [TESTFAELLE.csv](TESTFAELLE.csv) enthält geplante Fälle, deren Ergebnis ausdrücklich `not_run` bleibt.

Nach integriertem G1 zuerst den festgeschriebenen Vertrag importieren und den E2E-Fall mit roten Gegenproben umsetzen. Das ergibt erst nach Ausführung `mock_verified`. Für G2 müssen tatsächliche 03–07-Module und die freigegebenen Pilotquellen aus 12–14 samt Lösch-/ACL-/Konfliktfall durch denselben Pfad laufen. Für G3 folgen Consumerparität, Pflicht-Coverage und die übrigen Quellen. Kein Nachweis wird durch synthetische Fixtures oder fremde PR-Ergebnisse ersetzt.

PR-Testbetrieb: Vorbereitung als offener Draft-PR sichern. Kein Merge, Main-Push, Deploy, Neustart, Produktionszugriff oder Cleanup fremder Worktrees.
