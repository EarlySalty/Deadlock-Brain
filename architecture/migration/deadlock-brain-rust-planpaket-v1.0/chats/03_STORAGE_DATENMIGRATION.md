# Chat 03 · Storage, kanonisches Modell und Datenmigration

**Startbedingung:** Schemaentwurf mit Chat 02 vor G1; Pilot bis G2; Gesamtdaten nach G2; Abgleich bis G3 und Cutover mit 11.

## Kontext zum Mitgeben

Vollständiges Source-Inventar, Contracts, 03_DATENMIGRATION, ACL-/Egress-Entscheid, Snapshot-/Backupnachweise; Suchentscheidung aus 06 für abgeleitete Indizes.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **03 — Storage, kanonisches Modell und Datenmigration** im Umbau von Deadlock Brain zuständig.

**Ziel:** Übernimm sämtliche freigegebenen Daten nachvollziehbar und wiederanlaufbar. Kanonische Daten, Quellhistorie und daraus erzeugte Suchprojektionen bleiben unterscheidbar.

**Deine Eigentümerschaft:** `crates/brain-storage/`, versionierte DB-Migrationen, `tools/brain-admin/`, Migrationsmanifeste und Abgleichberichte. Gemeinsame Job-/Feeder-Infrastruktur gehört 04; fachliche Connectoren 12, 13 und 14.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Finalisiere Source Registry, logische IDs, Revisionen, Hashes, Gültigkeiten, ACLs, Provenienz, Tombstones, Jobs/Checkpoints und Artefaktregister gemeinsam mit 02/04/05.
2. Entscheide den kanonischen Store anhand Inventar und Betriebsbedarf. Halte Rohobjekte und Geheimnisse aus öffentlichen Repos; importiere Git-Historie nur nach Sichtbarkeits-/Secretprüfung.
3. Implementiere Rust-Adminwerkzeuge für Inventory, Export, Migrate, Verify, Rebuild und Replay mit Dry-Run, Batches, Leases, Transaktions-/Outbox-Grenzen und eindeutigen Exitcodes.
4. Migriere einen repräsentativen Pilotsnapshot. Bewahre Quellen-/Revisionszuordnung und Beziehungen. Quarantäne ist eine offene Abweichung, kein erfolgreicher Import.
5. Baue nach freigegebenem G2 den Vollbestand auf; vergleiche Quellrecords, Hashes, Referenzen und zugelassene Ausschlüsse. Alte Embeddings nur mit belegter Modell-/Vorverarbeitungskompatibilität übernehmen.
6. Implementiere Delta-/Delete-/ACL-Replay und konsistente Corpusreleases. Verhindere, dass verspätete Events neuere Löschungen rückgängig machen. Aktuelle Sperren gelten auch für alte Releases und Cachetreffer.
7. Probe mit 11 Writer-Fencing, letzten Watermark, Umschaltung und kompatibles Rollback; kontrolliere neue Writes nach dem Cutover.

**Liefergegenstände:** Schema/SQL-Migrationen, Rust-Adminwerkzeug, Snapshot-/Mappingmanifeste, Checkpoints, Source-zu-Ziel-Abgleich, Restore-/Replay-/Rollbacknachweise.

**Abnahme:** Jeder freigegebene Quelldatensatz hat einen nachvollziehbaren Zielentscheid. Kein ungeklärter Verlust; Crash/Wiederanlauf und Duplikatereignisse sind getestet. Full-Rebuild des Zielsystems ohne Python möglich.

## Konkretisierung aus den drei Recherchen · v1.0

Lies 11–13 und ergänze Quellableitungsgraph, root_artifact/parser_family, historische Game-Gültigkeit, Fact-/Effect-/Rulemodelle, Aliase, Graphbeziehungen, Kartenprojektionen, Schemaänderungen sowie Replay-/Populationdaten. 02 besitzt Verträge, 03 alle Migrationen; 12–14 liefern konkrete Inputanforderungen.

Seed-Register und Coveragevorlagen getrennt von tatsächlichen Importzahlen führen. Media-/Replayobjekte nicht pauschal in Git/PostgreSQL-Tickzeilen kopieren. Neue Source-/Rule-/Karten-/Observationversionen atomar im selben Knowledge-Release referenzieren. Snapshot/Delta/Restore auch für diese Datenklassen belegen.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Entwirf anhand echter Quellen die Identitäts-/Revisions- und Konsistenzregeln; implementiere anschließend Migration und Verify für einen kleinen Snapshot.
