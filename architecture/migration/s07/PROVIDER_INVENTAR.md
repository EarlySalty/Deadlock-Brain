# S07 · Providerinventar und Wiederverwendung

Geprüfter Code: `c00fc8935048bf490c1e4790f7c6195864ad49e2`. Zuerst Graphify-
Bestandssuche, danach Prüfung der Fundstellen und der getrackten Rust-, Python-
und Skriptpfade. Graphkanten sind Orientierung, die folgenden Befunde beziehen
sich auf den tatsächlich gelesenen Code. Runtimezustand nicht untersucht.

## Vorhandene Bausteine

| Pfad im Repository | Stand und Folgerung |
|---|---|
| `rust/crates/deadlock-brain-core/src/ai.rs:10–19,94–106,124–202` | `AiConfig`, typisierter Chat-Request, zentraler `AiClient` vorhanden. Erweitern statt parallelen Connector bauen. |
| `rust/crates/deadlock-brain-core/src/http.rs:73–99,135–187` | Persistente Blocking-Clients und gemeinsame Retrylogik vorhanden; AI verwendet diesen Transport derzeit nicht. Gemeinsame Änderung mit 02/04 abgrenzen. |
| `rust/crates/deadlock-brain-core/src/model_resolver.rs:8–62` | Modellauflösung und In-Memory-Cache vorhanden. Erhalten, aber vor Nutzung als robuster Providerport härten. |
| `rust/crates/deadlock-brain-core/src/config.rs:11–12,100–105` | Konfigurierter Fireworks-Endpunkt und Modellfamilie; Verfügbarkeit des kompilierten Modellnamens wurde nicht live bestätigt. Kein eigenmächtiger Modellwechsel. |
| `rust/crates/dbrain-retrieval/src/lib.rs:1662` | Nutzt zentralen `AiClient`. Keine neue Providerlogik in Retrieval einführen. |
| `rust/crates/dbrain-reasoner/src/ai_roles.rs`, `dbrain-enrich/src/lib.rs`, `dbrain-learn/src/{build_learning,match_demo_learning,player_decision_learning}.rs`, `deadlock-brain/src/main.rs` | Bestehende AI-Aufrufer: Signaturen und Verhalten erhalten beziehungsweise vom zuständigen Owner migrieren lassen. |
| `src/deadlock_brain/retrieval.py:17–27` | Historischer lokaler SentenceTransformer-Embeddingpfad. Kein Nachweis einer aktuellen Rust-Embeddingimplementierung oder eines laufenden Dienstes. |
| `rust/crates/deadlock-brain-yt/src/gemini.rs:68–95,188–195` | Rust startet einen Python-Browserworker. Codebefund, kein Livebefund. S09/S11 zuordnen; nicht in S07 entfernen. |

Im untersuchten getrackten Brain-Rustbestand wurde kein Jev/System-One-Client
gefunden. Aus diesem Befund folgt keine Aussage über fremde Repositories oder
nicht getrackte Laufzeitdateien. Der vorhandene Embeddingdatenbestand darf nicht
stillschweigend auf ein anderes Modell oder eine andere Dimension umgestellt werden.

## Konkrete Lücken für die spätere Implementierung

| ID | Ort am Basis-Commit | Befund und erforderlicher Nachweis |
|---|---|---|
| P07-01 | `ai.rs:124–126,153–168` | AI hält nur Config; pro Aufruf neuer Thread und HTTP-Client, anschließend synchrones Join. Wiederverwendung und nichtblockierender Aufrufpfad fehlen. |
| P07-02 | `ai.rs:294–310`; `http.rs:211–232` | Gesamter Body wird vor Validierung gelesen. Nachher 1.000 Zeichen abzuschneiden begrenzt weder Empfang noch Speicher. Größenlimit während des Lesens inklusive dekomprimierter Bytes testen. |
| P07-03 | `ai.rs:297–302`; `http.rs:229–233` | Providerfehlertext wird unverändert abgeschnitten weitergereicht. Trunkierung ist keine Secret-/Kontextredaktion. Synthetische Echo-Geheimnisse als Gegenprobe. |
| P07-04 | `model_resolver.rs:42–60` | Kein explizites HTTP-Timeout; globales Mutex während Netzwerkzugriff gehalten. Resolver muss dieselbe Gesamtdauer und dasselbe Aufrufbudget teilen. |
| P07-05 | `model_resolver.rs:8–18,32–45` | Cache ohne Endpoint-/Konfigurationsschlüssel und ohne TTL; nach 404 kann dieselbe veraltete Cacheantwort wiederkehren. Isolation, Invalidierung und begrenzte Neuauflösung testen. |
| P07-06 | `config.rs:100–105`; `model_resolver.rs:21–39` | Reihenfolge der singulären/pluralen Modell-Env-Namen ist unterschiedlich; Resolvercache kann konfigurierte Settings überholen. Präzedenz mit 02 festlegen und testen. |
| P07-07 | `ai.rs:304–310` | Erfolgreiches JSON wird als beliebiger `Value` angenommen; leeres Objekt ist kein nachgewiesener Chatabschluss. Typ-/Pflichtfeld-/Finish-Validierung fehlt. |
| P07-08 | `http.rs:152–179` | Geteilte Retries ohne Jitter, Gesamtretrybudget oder Retry-After-Behandlung; AI nutzt sie nicht. Kein nachgewiesener Circuit Breaker. |
| P07-09 | `ai.rs:169–182`; `http.rs:85–88` | In diesen Transportpfaden kein sichtbares zentrales Rechte-/Egress-Gate. Nicht als Beweis fehlender Kontrollen in allen Aufrufern verallgemeinern. Gate vor jeder Netzwerkstufe einschließlich Resolver benötigt. |
| P07-10 | `http.rs:243–278` | Generischer GET-Cache ist URL-basiert. Nicht unverändert für tenant-/credentialabhängige Providerantworten übernehmen. |

Die Befunde sind weder hier repariert noch durch zusätzliche Produktionscode-
Regressionstests abgesichert. Die 17 bestehenden Coretests wurden ausgeführt;
ihre genaue Aussage steht im Prüfbericht. Keine globale Formatierungsreparatur,
keine Entfernung vorhandener Funktionen, keine Modell-/Timeoutänderung.
