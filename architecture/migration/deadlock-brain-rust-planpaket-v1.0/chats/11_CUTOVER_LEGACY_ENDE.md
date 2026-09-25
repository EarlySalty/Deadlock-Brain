# Chat 11 · Produktivumstellung, Betrieb und Legacy-Abbau

**Startbedingung:** Betriebsentwurf früh; Probe vor G4; Ausführung G5/G6 nur mit ausdrücklicher Freigabe.

## Kontext zum Mitgeben

Freigegebene Artefakte/Gates, Daten-/Release-/Checkpointmanifeste, Consumerinventar, Backup-/Restore-/Rollbacknachweise und konkrete Betriebsfreigaben.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **11 — Produktivumstellung, Betrieb und Legacy-Abbau** im Umbau von Deadlock Brain zuständig.

**Ziel:** Schalte kontrolliert auf den vollständigen Rustbetrieb um und beende echte Legacyabhängigkeiten, ohne neue Writes, Löschungen oder Rechte zu verlieren.

**Deine Eigentümerschaft:** `infra/` Deployment-/Monitoring-/Runbook-Dateien, Cutover-/Betriebsprotokolle, Abschaltinventar; keine eigenmächtige fachliche Schemaänderung.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Erstelle Release-Images mit Rustbinaries, Health/Readiness, Secret-Verweisen, Ressourcenbudgets und Start-/Shutdownverhalten. Prüfe Native-/Python-/Legacyabhängigkeiten einschließlich regelmäßiger Jobs.
2. Schreibe ein Cutover-Runbook mit freigegebenen Abbruchkriterien, Zuständigkeiten, vorheriger kompatibler Version und Rollbackstrategie für Writes nach dem Umschaltpunkt.
3. Probe leeren Restore/Rebuild, partielle Ausfälle und Writer-Fencing. Repliken/Indexgenerationen und aktuelle Tombstone-/ACLsperren mitprüfen.
4. Bereite internen Canary, dann Twitch und weitere Consumers vor. Shadowaufrufe nicht als Nutzerschreibvorgang duplizieren; zusätzliche Providerkosten begrenzen.
5. Erst nach expliziter Freigabe alten Writer fencen, letzten Offset festhalten, Delta abgleichen und neuen Writer aktivieren. Prüfe genau einen wirksamen Writer und entferne alte Schedules.
6. Nach bestandenen Betriebskontrollen und freigegebenem Rollbackfenster alte Services/Keys/Deployments entfernen. Repoarchivierung und irreversible Datenlöschungen separat freigeben lassen.
7. Führe abschließend Runtime-/Netzwerk-/CI-/Dependencyprüfung durch: kein aktiver Pythonkern, kein Legacy-RAG, keine direkten Consumer-Modellpfade. Nicht-Python-Betrieb muss auch nächsten Ingest-/Learningzyklus bestehen.
8. Aktualisiere Runbooks, Zuständigkeiten und Verweise. Datenschutz-/Sicherheitskorrekturen nicht durch Rückkehr zum Altstand zurücknehmen.

**Liefergegenstände:** Release-Manifest, Deploymentkonfiguration, Cutover-/Rollback-/Restoreprotokoll, Legacy-Abschaltmatrix, Nachweis Python-freier Runtime und G5/G6-Abnahme.

**Abnahme:** Tatsächlicher Alleinbetrieb des neuen Systems ist belegt; Daten/Writes/Rechte konsistent; Altpfade und Keys kontrolliert stillgelegt. Keine behauptete Archivierung ohne ausgeführte freigegebene Aktion.

## Konkretisierung aus den drei Recherchen · v1.0

Lies 11–13. Trotz höher nummerierter neuer Chats bleibst du der abschließende Cutoverowner. Staging-/Restoreproben früh vorbereiten; produktiver Wechsel erst nach G4 und ausdrücklicher Freigabe.

Release-Manifest pinnt auch Rule-/Karten-/Mechanik-/Source-/Schema-/Replay-/Populationversionen, Drift-Status und Freigaben. Fencing/Watermarks für Wiki-, externe Feeds und Replayjobs berücksichtigen. Prüfe Pythonfreiheit nach einem vollständigen regulären Update-, Reparse- und Learningzyklus, nicht nur bei einem bereits vorgefüllten APIstart. Docs-/Twitch-/Discord-/Publishjobs kontrolliert umstellen; keine Archivierung aus einer reinen Planfreigabe.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Erstelle zuerst das auf realem Inventar basierende Betriebs-/Rollback-Runbook. Produktive Umschaltungen, Secretänderungen und Archivierungen nicht allein aufgrund dieses Chatprompts ausführen.
