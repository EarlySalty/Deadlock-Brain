# Chat 06 · Hybrid Retrieval und Suchindexentscheidung

**Startbedingung:** Pilotentwurf in Welle B, Entscheidung bis G2, Umsetzung nach G1 und Qualität bis G4.

## Kontext zum Mitgeben

Query-/Quellinventar, Contracts/Policy, Storage-/Indexports, repräsentativer Corpus, unabhängige Relevanzlabels von 10 und Embeddingport aus 07.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **06 — Hybrid Retrieval und Suchindexentscheidung** im Umbau von Deadlock Brain zuständig.

**Ziel:** Finde relevante, berechtigte und zeitlich passende Evidenz schnell. Jev bekommt Kandidaten, ersetzt aber nicht den eigentlichen Retriever.

**Deine Eigentümerschaft:** `crates/brain-retrieval/`, lexical-/dense-Indexintegration und Retrievalbenchmarks; 03 besitzt kanonische Daten und Migrationsnummern.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Baue eine reproduzierbare lexical-Baseline und dense-Baseline auf identischen Quellen/Labels. Tantivy/BM25 ist ein Kandidat; einfache PostgreSQL-FTS bleibt als solche benannt. Gegen vorhandenen geeigneten Stack vergleichen.
2. Vergleiche den bestehenden Vektorstore mit einem geeigneten Startkandidaten wie pgvector. Nutze eine begrenzte Auswahl; kein produktiver Betrieb mehrerer konkurrierender Stacks ohne Nutzenentscheid.
3. Messe reale ACL-/Metadaten-/Patchfilter, restriktive Scopes, seltene Begriffe, deutsche/englische Bezeichnungen und Code. Weniger ANN-Treffer unter Filtern nicht mit guter Latenz verwechseln.
4. Implementiere deterministische Fusion, z. B. RRF als zu evaluierende Variante, Deduplikation ohne ACL-Verschmelzung, strukturierte Domainfakten und versionierte Nachbarschaftsevidenz.
5. Lege nach Pilotmessung Modellrevision, Vorverarbeitung, Dimension, Distanz, Tokenisierung, Indexkonfiguration und Kandidatenbudgets fest. Übergib den G2-Entscheid vor Voll-Reembedding an 03/04.
6. Ein Request nutzt konsistente Corpusreleases. Prüfe aktuelle Sperren vor jeder Evidenzweitergabe. Nicht genug berechtigte Treffer führt zu begrenzter Erweiterung innerhalb des erlaubten Bereichs, nie zu einem Scope-Bypass.
7. Integriere Jev-Relevanz als einzeln abschaltbaren Post-Retrieval-Schritt gemeinsam mit 07/08; messe mit 10 Recall, Latenz, Speicher, Buildzeit und Kosten.

**Liefergegenstände:** Rust-Retriever, reproduzierbarer Indexbau, ausgewertete Pilotvarianten, ADR Such-/Embeddingkonfiguration, Retrieval-/ACL-/Generationstests.

**Abnahme:** Suchentscheidung ist mit eigenem Corpus belegt; kritische Quellenfälle und freigegebene Nichtunterlegenheit erfüllt. Kein ungenehmigter Scope oder unverifizierter Indexmix erreicht Jev/LLM.

## Konkretisierung aus den drei Recherchen · v1.0

Lies 11–13. Kombiniere exakte Facts/Entities, gezielte Mechanikbeziehungen und Prosa über denselben autorisierten Retrievalport. Numerische Berechnungen direkt aus 05, nicht über Ähnlichkeitssuche in Karten. Sparse/dense/strukturierte Evidenz und Karten müssen dasselbe Knowledge-Release verwenden.

Keine Rohreplays/Ticks oder komplette Hero-Megakarten pauschal einbetten. Kleine abschnittsbezogene Karten/Erklärtexte und selektive Reprojektion nach tatsächlichem Inhaltsdiff. Patch-/Mode-/Sprach-/Aliasfälle sowie korrelierte Quellen, veraltete Karten und restriktive ACLs in Evals aufnehmen.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Fixiere einen Corpus- und Labelstand und miss lexical/dense getrennt, bevor du Fusion oder Jev optimierst.
