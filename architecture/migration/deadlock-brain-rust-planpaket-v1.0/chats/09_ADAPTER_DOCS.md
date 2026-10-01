# Chat 09 · Rust-Adapter, Consumer und Docs

**Startbedingung:** Vorbereitung nach G1 gegen Contract-API; echte Integration mit den jeweils bereits integrierten echten APIpfaden aus 08; Cutover ausschließlich mit 11.

## Kontext zum Mitgeben

Consumerinventar, APIvertrag/Authentifizierung, bestehende Befehle/Protokolle und freigegebene Antwortprofile; Inhalte mit 03/04 koordinieren.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **09 — Rust-Adapter, Consumer und Docs** im Umbau von Deadlock Brain zuständig.

**Ziel:** Alle Interfaces behalten ihre nützlichen Funktionen, besitzen aber keinen eigenen RAG-/Provider-/Promptpfad mehr. Eigene Backendadapter werden Rust.

**Deine Eigentümerschaft:** `crates/brain-client/`, `apps/brain-twitch/`, `apps/brain-mcp/`, `apps/brain-cli/`, optionale Docs-Darstellung und dokumentierte Web-/Agentintegration.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Erstelle einen Rust-Brainclient aus dem fixierten Vertrag mit Auth, Timeout, Fehlern und Protokolltests. Kein lokaler LLMfallback.
2. Portiere Twitch-spezifische Auth/Events, Befehle, Moderation, Limits, Ausgabeaufteilung und Reconnect. Aktuelle Plattformverträge beim Implementieren gegen Primärdokumentation prüfen.
3. Binde MCP, CLI, interne Agents und Web an den Brain-Service. Admin-Migrationsbefehle bleiben getrennt privilegiert; ein normaler APIparameter darf sie nicht freischalten.
4. Gleiche die Command-/Protokollmatrix gegen Altfixtures ab. Plattformidentität nicht ungeprüft in einen privilegierten Brain-Principal verwandeln.
5. Entferne direkte Providerkeys, eigene Modellwahl, Retrievalprompts und parallele RAG-Endpunkte aus der neuen Adapterimplementierung. Alte produktive Pfade erst nach Cutoverfreigabe abschalten.
6. Übernimm Docs-Inhalte/Links und optional Website als Darstellung. Browser-JS/TS ist zulässig, serverseitige Brainlogik nicht. Interne Inhalte nicht in öffentlichen Static-Builds veröffentlichen.
7. Teste jede Consumerklasse Ende zu Ende mit echten APIports; negative Tests für Token-/Conversationmissbrauch und sicher dargestellte Quellen.

**Liefergegenstände:** Rust-Adapter/Client, Command-/API-Paritätsmatrix, E2E-Tests pro Consumer, Docs-Linkprüfung und entfernte Nebenpfade im Code.

**Abnahme:** Alle freigegebenen Consumers verwenden einen gemeinsamen Answerpfad. Plattformfunktionen sind erhalten; öffentliche Builds/Antworten enthalten keine internen Inhalte.

## Konkretisierung aus den drei Recherchen · v1.0

Lies 12. `Deadlock-Bots`/Discord und direkte Build-Publish-/Steam-Verbraucher anhand Inventar prüfen und Brain-relevante Serverteile ins Monorepo einordnen; keine separate AI-Pipeline. Fachliche Commands/Katalog-/Build-Reads nutzen dieselben Auth-/Kernelverträge.

`KnowledgeExport` benötigt Source-/Fact-/Rule-Set, Knowledge-/Patchversion, stabile ID, Redaction, Hash und Freigabestatus. Docs-Reviews/Commitbindung sowie tatsächlich vorhandene deutschsprachige Support-Evals erhalten. Kein ungeprüftes automatisches Zurückschreiben von Modellwissen in öffentliche Docs. Veröffentlichung/externes Build-Publishing erst mit expliziter Berechtigung; Mocks lösen keine echten Seiteneffekte aus.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Implementiere den zentralen Client und einen Twitch-Befehl gegen die Contract-Test-API; erweitere anschließend anhand der Paritätsmatrix.
