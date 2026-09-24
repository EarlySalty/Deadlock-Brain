# Abnahme, Cutover und Abschluss

## Release-Gates

| Gate | Erforderlich | Primär verantwortlich |
|---|---|---|
| G0 · Ist-Zustand | drei Recherchen zugeordnet; Code-/Feature-/Runtime-/Dateninventar einschließlich Wiki/externer Quellen/Replays; Rechte-/Securitylage; Baseline und Zielprofil | 01 + 10 + 00 |
| G1 · Fundament | integrierter Workspace, Auth/Egress, reale Pfadbesitzer, kanonisches Schema und Source-v2-/Fact-/Rule-/Karten-/Observationverträge; freigegebene SLOs | 02 + 03 + 00; fachlich 05/12–14 |
| G2 · Rust-Durchstich | echter öffentlicher/interner Antwortpfad; Wiki-Hero/Build/Karte; externer Git-/API-Diff; Replayfall; Delete/ACL/Konflikt; Pilotbenchmark und fixe Such-/Embeddingversion | 03–09 + 12–14 + 10 |
| G3 · Vollständigkeit | freigegebene Gesamtdaten/Coverage abgeglichen; Domain-/Buildparität, historische Feeds, Replay-Referenzsuite, Knowledge-Exports und alle Pflicht-Consumer im echten Staging | 03–09 + 12–14 + 10 |
| G4 · Freigabe | Gesamtqualität/Sicherheit/Last; Wiki-Grounding/Buildregeln, API-/Schema-Drift, Replaycapabilities/zeitliche Holdouts; Jev-Entscheid; Restore-/Rollbackprobe und Rust-only-Backend | 10 + 11 + Modulbesitzer + 00 |
| G5 · Cutover | neue Writer eindeutig aktiv, letzte Deltas übernommen, Canary und Betriebskontrollen bestanden | 11 + 00 |
| G6 · Legacy-Ende | keine produktiven Altaufrufe, alte Jobs/Secrets/Services aus, dokumentierte Archivierung | 11 + 00 |

`offen` ist nicht `bestanden`. G3 bedeutet technische Integration im Test-/Stagingbetrieb, nicht die produktive Umschaltung aus G5. Ein als blockierend markierter Pflicht-Consumer verhindert G3, statt dessen Abnahme zu ersetzen. Testprotokolle enthalten tatsächliche Resultate und getestete Commits. Eine Freigabe gilt nicht automatisch für einen später geänderten Contract, Datenstand oder Modellalias.

## Sicherheitsabnahme

Public → internal/private, gefälschte actor/scopes/channel-Felder, fremde conversation_id, Cache zwischen Benutzern, Zitate/Download-Endpoints, Traces/Logs und Provider-Egress negativ testen. Interne Datei darf weder über Retrieval noch über Antwortcache, Citation-Metadaten, Modellprompt oder Debug-Endpoint öffentlich werden.

Dokument-Promptinjektionen verändern keine Berechtigungen und keine Providerallowlist. SSRF-/URL- und Größen-/Archivgrenzen für Connectors berücksichtigen. Auth-Ausfall führt zu einer sicheren Ablehnung, nicht zu einem weiteren Scope. Export-/Migrationstools haben getrennte administrative Berechtigungen.

## Python-freier Nachweis

Release-Images enthalten keine für den Brainbetrieb notwendige Pythonumgebung. Dependency-/Entrypoint-/Container-/Serviceinventar auf PyO3, libpython, Python-Skripte, Python-Worker und alte Serviceadressen prüfen. Netzwerkzugriffe auf Legacy-Brain-/RAG-/Pythonendpoints sperren und E2E-Tests erneut ausführen. Query, Worker, Datenimport, Rebuild und fachliches Learning müssen den freigegebenen Test ohne Pythonpfad bestehen.

Ein Offline-Test mit vorbereiteten Artefakten genügt nicht für einen Worker, der beim nächsten regulären Zyklus wieder Python benötigt. Einmalige Exporthilfen dürfen für Altformate dokumentiert bleiben, aber das neue Quellformat und dessen Wiederherstellung müssen unabhängig sein.

## Cutover-Ablauf

1. Freigegebene Binaries, Schemas, Corpusreleases, Checkpoints, Policies und Secrets als Release-Manifest festhalten. Restore und Kapazitätsreserve kontrollieren.
2. Shadowverkehr ohne doppelte Nutzerschreibvorgänge und ohne Veröffentlichung unfreigegebener Daten fahren. Kostenbudget für doppelte Modellaufrufe begrenzen.
3. Internen Canary aktivieren; danach Twitch als ersten öffentlichen Consumer, dann MCP/CLI/Web/Agents nach derselben Abnahme. Pro Stufe Rückschaltbedingungen vorab festlegen.
4. Für Writer einen gesonderten Umschaltpunkt nutzen: alten Writer fencen, letzten akzeptierten Offset erfassen, Delta abgleichen, neuen Writer aktivieren. Keine zwei unkoordinierten Feeder.
5. Nach Freigabe den neuen Pfad als alleinigen Standard setzen. Daten-/Qualitäts-/Lastkontrollen auf dem tatsächlich produktiven Stand wiederholen.
6. Während des freigegebenen Rollbackfensters alte Artefakte und nötige Backups geschützt vorhalten. Danach Legacydienste, Scheduler, Providerkeys und Infrastruktur kontrolliert abbauen.
7. Alte Repositories nach ausdrücklicher Freigabe read-only/archiviert setzen und auf das Zielrepo verweisen. Keine neuen Featurearbeiten mehr dort. Historische Daten nur nach der vereinbarten Aufbewahrung behandeln.

## Rollback-Bedingungen

Scope-/Secret-Leak, ungeklärter Datenverlust, kritische fachliche Regression oder überschrittene Betriebsgrenzen stoppen den Rollout. Performanceabweichungen werden gegen vorab gesetzte Fehlertoleranzen beurteilt, nicht nach Bauchgefühl.

Ein alter Binary-Stand darf nur mit einem kompatiblen Schema und lesbaren Datenrelease gestartet werden. Bei Writes nach dem Cutover werden diese nachweisbar replayt oder exportiert; andernfalls wird Schreiben angehalten. Ein Rollback darf aktuelle ACL-Sperren, Löschungen und Secret-Rotation nicht zurückdrehen. Read-only-Verfügbarkeit kann sicherer sein als eine inkonsistente vermeintliche Vollwiederherstellung.

## Abschließende Definition of Done

Alle verpflichtenden Anforderungen in `07_ANFORDERUNGEN.md` haben einen konkreten Nachweis. Vollständige Datenversorgung und Domainfunktionen laufen in Rust; jeder Consumer nutzt denselben Kernel; Berechtigungen gelten auch für Cache, Evidenz und Modelle; Qualitäts-/Performanceziele sind gemessen; Rebuild und Restore funktionieren; produktive Python-/Legacyabhängigkeiten sind entfernt.


## Zusätzliche Pflichtnachweise vor G3/G4

**Wiki/Facts/Builds:** Pflichtmanifest ohne ungeklärte buildkritische Lücken; stabile IDs/Zeiten/Units, Unknown-Fälle, Patch-/Modelegale Builds, belegte numerische Golden-Aussagen, rebuildbare Hero-Wissenskarten. Siehe 11.

**Externe Quellen:** API-/Git-/Schemafeeds gepinnt, Parserdiff von Gameplaydiff getrennt, korrelierte Quellen erkennbar, Konflikte/Drift vor Aktivierung abgefangen. Quellen-/Asset-/Exportfreigaben für genutzten Scope, kein pauschaler Repo-Mirror. Siehe 12.

**Replays/Population:** Pflichtcapabilities auf den vereinbarten Fällen, versionierte Referenzprüfung, Rohdatenrechte, begrenzte Decoderressourcen, deduplizierte Kohorten und verfügbarkeitskorrekte zeitliche Holdouts. Fehlende Replay-/Learningfunktionen nicht durch vorberechnete Beispieldaten verstecken. Siehe 13.

**Gemeinsame Performance:** Echtbetrieb im Staging unter parallelem Ingest/Replay/Reembedding; Antwortpfad/SLOs, Quellenfrische und Ressourcenlimits belegt. Karten/Graph/Population und Textindex auf einer kompatiblen Knowledge-Version. Mocktests allein reichen für kein Vollständigkeits-/Freigabegate.

**Monorepo/Publikation:** Brain-relevante Teile aller inventarisierten Consumers einschließlich Discord/Buildpublishing berücksichtigt. Veröffentlichung ist eine privilegierte Handlung; keine ungeprüfte Modellantwort direkt in Docs oder externe Builds zurückschreiben.

Ein Daten-/Quellenausfall rechtfertigt einen degradierten klar gekennzeichneten Betrieb, aber keine Behauptung vollständiger Wiki-/Replayintegration. Ein dauerhaft reduzierter Pflichtumfang benötigt ausdrückliche Produktfreigabe, die im Abschlussbericht sichtbar bleibt.
