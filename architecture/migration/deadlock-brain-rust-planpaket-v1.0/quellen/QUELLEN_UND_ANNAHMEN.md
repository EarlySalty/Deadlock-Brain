# Quellen und offene Fakten · v1.0

## Bereitgestellte Grundlagen

**U1:** [Konsolidierungs-/Jev-Recherche](RECHERCHE_ORIGINAL.md), Originaldatei `Deadlock Brain Deep Research all in one Repo umstellung.md`.

**U2:** Nutzeraufträge: einziges Brain-Repo, Kern und reguläre eigene Backendpfade in Rust; kleine optionale Skripte erlaubt; Wiki, alle Daten und weitere Ökosystemquellen in einem Projekt; Performance messen; klare Chat-Reihenfolge und Parallelität.

**U3:** [Wiki-Integrationsrecherche](RECHERCHE_WIKI_ORIGINAL.md), Originaldatei `deep-research-report Deadlock Brain und Deadlock Wikli.md`.

**U4:** [Ökosystem-/Replay-Recherche](RECHERCHE_OEKOSYSTEM_ORIGINAL.md), Originaldatei `deep-research-report (5)(2).md`; identisch mit `(5)(1).md`. SHA-256 und Deduplikation: `QUELLEN_MANIFEST.json`.

Die Originaltexte bleiben unverändert. Ihre internen alten Chat-/Webzitationsmarker sind historische Verweise; sie werden nicht als in dieser Bearbeitung erneut verifizierte Primärbelege ausgegeben. Dieser Arbeitsschritt integriert bereitgestellte Analysen und lokale Planartefakte, er führt keinen neuen Liveaudit von GitHub, Wiki oder Providerdiensten durch.

Konkrete Behauptungen der Recherchen über Stars, PRs, Tests, Endpoints, Herozahlen, Itemwerte und Lizenzen sind Recherche-Snapshots. Chat 01/jeweiliger Integrationsbesitzer prüft relevante Tatsachen am tatsächlich verwendeten Commit bzw. freigegebenen Quellenstand erneut. Keine Versionsnummer oder Lizenzfreigabe aus einem alten Text automatisch übernehmen.

## Technische Literaturhinweise aus v0.9 — hier nicht erneut live geprüft

- **E1 · Cargo Workspaces:** https://doc.rust-lang.org/cargo/reference/workspaces.html — gemeinsame Workspaceverwaltung, Lockfile und Dependencies.
- **E2 · axum:** https://docs.rs/axum/latest/axum/ — HTTP-Framework und Zusammenspiel mit Tokio/Tower.
- **E3 · Tokio spawn_blocking:** https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html — begrenzte CPU-Parallelität, Blocking-Pool und Abbruchgrenzen.
- **E4 · pgvector:** https://github.com/pgvector/pgvector — Vektorsuche, Hybridkombination und Filter-/ANN-/Indexverhalten. Keine Annahme, dass dieses Projekt pgvector heute bereits einsetzt.
- **E5 · Tantivy:** https://github.com/quickwit-oss/tantivy — Rust-Suchbibliothek mit BM25; eigener Benchmark auf dem Brain-Corpus bleibt erforderlich.
- **E6 · Jev Introduction:** https://docs.typesafe.ai/introduction — strukturierte Entscheidungen; Choice, Score und Noul; unabhängige Fragen gegen denselben State.
- **E7 · Jev Quick Start:** https://docs.typesafe.ai/introduction/quickstart — dokumentierte HTTP-API; ein Rust-Client benötigt hierfür keinen Python-SDK-Wrapper.

- **E8 · PostgreSQL Text Search Controls:** https://www.postgresql.org/docs/current/textsearch-controls.html — Standardranking `ts_rank`/`ts_rank_cd`; dieses wird nicht als BM25 bezeichnet.

Die Quellen belegen technische Eigenschaften, nicht die Performance dieses noch nicht umgebauten Systems. Bibliotheks-/Modellversionen und Providerlimits vor Implementierung erneut prüfen und im Release pinnen. Preis- und Hersteller-Geschwindigkeitsangaben werden nicht als eigenes Sparversprechen übernommen.


## Planentscheidungen und Konfliktauflösung

Verbindlich sind Rust für eigene reguläre Backendpfade; bestehende geeignete Rust-Module erhalten; ein Answer Kernel; Raw/Canonical/Serving trennen; eine gemeinsame Provenienz mit korrelierten Quellenfamilien; Hero-Wissenskarten als Projektion; deterministische Buildregeln; Replaydaten als empirische, nicht automatisch kausale Evidenz; klare Owner/Gates. Das sind Planentscheidungen, keine bereits ausgeführten Codeänderungen.

U3s beispielhafte Struktur und Datenbanktabellen ersetzen keine bestehenden gleichwertigen Verträge. U1s Python-/Node-Beispiele beweisen keine heute aktive Runtime. U4s Rust-Inventar ist zu prüfen, nicht als automatischer Migrationsnachweis zu übernehmen. Details in [08](../08_ERGAENZUNGEN_INTEGRIERT.md).

## Weiterhin offen

Ist-Commits und laufende Pfade, Datenverfügbarkeit/Volumen, Einsatzhardware, SLO/RPO/RTO, Schema-/Providerverträge, Originalnutzungsrechte und Freigaben, Abdeckung der Parser pro Patch, Qualität des Replaybestands und vorhandene Testresultate. Alle Umsetzungsgates G0–G6 bleiben offen, bis Nachweise vorliegen. Geschlossen ist nur der fehlende Eingang der zusätzlichen Recherchen.
