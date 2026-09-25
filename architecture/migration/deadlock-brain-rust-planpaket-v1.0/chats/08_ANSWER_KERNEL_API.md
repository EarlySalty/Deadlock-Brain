# Chat 08 · Einziger Answer Kernel und produktive API

**Startbedingung:** Nach G1 zunächst gegen fixierte Testports; echter Pilot an G2; vollständig vor G3/G4.

## Kontext zum Mitgeben

Contracts/Policy aus 02, Storage/Domain/Retrieval/Provider aus 03–07, Antwortprofile der Consumer, Qualitätsfälle von 10.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **08 — Einziger Answer Kernel und produktive API** im Umbau von Deadlock Brain zuständig.

**Ziel:** Jede AI-Antwort und jede eindeutige Domainantwort läuft durch denselben autorisierten Rust-Kern.

**Deine Eigentümerschaft:** `crates/brain-kernel/`, `apps/brain-api/`, Antwortformatierung und Kern-E2E-Tests. Keine neue Schattenimplementierung fremder Module.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Verdrahte Authkontext, Normalisierung, sichere exakte Fakten-/Cachepfade, Retrieval, optionale Jev-Schritte, Evidence Pack, Answerability, erlaubtes Antwortmodell und Quellenprüfung.
2. Halte den direkten Faktenpfad nachvollziehbar: aktuelle Datasetversion, Einheit und Quelle. Keine LLMrunde für deterministische Antworten nur aus Architekturkonvention erzwingen.
3. Bilde fehlende Evidenz, veraltete Daten, Providerfehler und Budgetüberschreitung explizit ab. Ein zweiter Retrievalpass ist begrenzt und bleibt innerhalb aktueller Berechtigungen.
4. Validiere Citation-IDs/Revisionen/Positionen und ergänze Supportprüfung. Erfundene Quellen nicht als korrekte Antwort ausgeben. Ein strukturvalides Zitat beweist keine semantische Unterstützung.
5. Implementiere versionierte exakte Caches und Zusammenfassung identischer laufender Arbeit mit ACL-/Policy-/Corpus-/Profil-/Conversation-Schlüssel. Jede Ausgabe prüft aktuelle Sperren; sensitive Debugfelder rollenbasiert filtern.
6. Messe jede Stufe mit Trace-ID, Wartezeiten, Token/Cost und Fehlerstatus, ohne standardmäßig vollständige private Prompts zu loggen. Cancellation, Streaming und Finalvalidierung konsistent entscheiden; kein unvalidierter sensibler Token vorab ausgeben.
7. Liefere API für alle Consumer und teste bei gesperrten Legacyadressen. Implementiere keine dauerhafte Pythonfallbackroute.

**Liefergegenstände:** Rust-Kernel/API, E2E-Tracefixtures, Antwortprofile, Cache-/Citation-/Fehler-/Auth-Tests, API-Vertragsnachweis und G2/G3-Übergabe.

**Abnahme:** Öffentliche/interne/strukturierte Anfragen und Fehlerfälle laufen durch echte Rustmodule. Jeder Quellenzugriff und Modellaufruf ist autorisiert. Keine eigenständige AI-Logik im Consumer nötig.

## Konkretisierung aus den drei Recherchen · v1.0

Lies 11–13. Public/internal, Facts+Prosa, Hero-Wissenskarte, legale Buildplanung und freigegebene empirische Evidenz durch denselben Kern führen. `current` einmal auflösen; Patch/Mode/Knowledge-Version zurückgeben. Harte Rechnungen/Regeln in 05; LLM darf nur geprüfte Ergebnisse erklären.

Nach G1 gegen feste Mocks implementieren erlaubt; G2/G3 erfordern reale Stores/Worker/Domain/Retrieval/Provider und die jeweils benötigten 12–14-Quellen. Quellenkonflikt, unzugängliche Evidenz, fehlende Build-Regel, unbekannter Hero und teilweise Replay-Capabilities explizit melden statt erfinden. Wissen/Exports nur über freigegebenen Releasezeiger.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Baue einen nachvollziehbaren E2E-Test mit festem AuthorizedContext, Evidence Pack und Antwortstatus; ersetze Testports schrittweise durch echte Integrationen.
