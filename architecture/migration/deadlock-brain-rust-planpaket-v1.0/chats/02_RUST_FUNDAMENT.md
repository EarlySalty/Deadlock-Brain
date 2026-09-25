# Chat 02 · Rust-Workspace, Verträge, Policy und CI

**Startbedingung:** Welle B nach G0; Vertragseigner ab G1.

## Kontext zum Mitgeben

G0-Inventar, 06_VERTRAEGE_UND_GRENZEN, Datenmodellabstimmung mit Chat 03, Auth-/Egress-Anforderungen und freigegebene Zielplattformen.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **02 — Rust-Workspace, Verträge, Policy und CI** im Umbau von Deadlock Brain zuständig.

**Ziel:** Erzeuge ein baubares Rust-Fundament und genau einen versionierten Vertrag, auf den alle anderen Arbeitschats implementieren.

**Deine Eigentümerschaft:** Root-Workspace/Toolchain/Lockfile, `crates/brain-contracts/`, `crates/brain-policy/`, zentrale CI-Grunddateien; Ownership in vorhandene Struktur einpassen.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Nutze geeignete vorhandene Rust-Komponenten. Richte Workspace, gepinnte Toolchain, gemeinsame Dependencies, Featurematrix und reproduzierbare Release-Builds ein. Tokio/axum sind Planvorschläge, Versionswahl per ADR.
2. Implementiere Wiretypen und interne Ports für Query, AuthorizedContext, Dokumentrevision, Chunk, Evidence, AnswerResponse, Usage, Fehler und CorpusRelease. Rust-/JSON-/OpenAPI-Abgleich durch Contracttests sichern.
3. Authentifizierung aus Credentials ableiten; actor/channel/requested_scopes aus dem Body gewähren keine Rechte. Conversation-Ownership, Objekt-ACL und Provider-Egress getrennt prüfen.
4. Definiere Port-Abhängigkeiten ohne Zyklen. Keine Provider-SDKs in Domain oder Adapter ziehen. Async-/Dispatch-Entscheidung mit kompilierenden Testimplementierungen belegen.
5. Lege Konfiguration, Deadline-/Budgettypen, Fehlersemantik, Tracing-Redaktion und kontrolliertes Shutdown fest. Secrets nicht als Debug-Ausgabe serialisieren.
6. Baue CI für Format, Clippy, Tests, Releasebuild, Target-/Featurematrix und Python-/Legacy-Ausschluss auf. Chat 10 liefert zusätzliche Qualitätschecks; dessen globale CI-Änderungen gemeinsam integrieren.

**Liefergegenstände:** Baubarer Workspace, Contract-Version, API-/Schemaartefakte, Policy-Tests, Ports mit Testimplementierungen, CI-Protokoll, ADRs und G1-Übergabe.

**Abnahme:** Reproduzierbarer Build; gefälschte Identität/Scopes und unerlaubter Egress werden abgewiesen; Verträge sind maschinenprüfbar. Kein versteckter Legacy-Kernel hinter einem Rust-HTTP-Wrapper.

## Konkretisierung aus den drei Recherchen · v1.0

Lies 06 und 11–13. G1 enthält SourceRecordV2, GameValidity, Fact/Effect/Rule, SynergyEvidence, HeroKnowledgeCard, ReplayArtifact/Observation, PopulationSlice, SchemaChange und KnowledgeExport. Daten-/Releasevertrag mit 03; Fachsemantik mit 05; Quellenbedarf mit 12–14. Einen vorhandenen CorpusRelease erweitern statt konkurrierender Zeiger.

Reale Pfadzuordnung, modulare Grenzen, Fixtures und kompilierende Ports integrieren, bevor parallele Implementierungen freigegeben werden. Security-/Dependency-/Branchregeln früh prüfen, offene Auditwarnungen nicht pauschal ignorieren. Fremdtools nur als optionale Referenzen, kein produktiver Fremd-Runtimekern.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Lege zunächst die Dependency-Richtung, Typen und Contracttests fest. Implementiere danach das kleinste baubare Fundament, nicht alle Fachmodule gleichzeitig.
