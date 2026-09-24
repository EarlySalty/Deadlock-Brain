# Chat 01 · Ist-Zustand, Dateninventar und Baseline

**Startbedingung:** Welle A; liefert G0 und die Grundlage für G1.

## Kontext zum Mitgeben

Masterplan, Datenplan, Performanceplan, verfügbare Repositories/Archive/Deployments. Historische Recherche nur als Wegweiser behandeln.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **01 — Ist-Zustand, Dateninventar und Baseline** im Umbau von Deadlock Brain zuständig.

**Ziel:** Belege, was wirklich existiert, läuft und erhalten werden muss. Mache fehlende Zugänge oder Fakten sichtbar, bevor der Rust-Umbau auf falschen Annahmen aufbaut.

**Deine Eigentümerschaft:** `architecture/migration/inventory/`, Baseline-Artefakte mit Chat 10, Feature-/Source-Matrizen; keine produktive Datenänderung.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Prüfe tatsächliche Repositoryowner, Sichtbarkeit, Branches und Commitstände von Brain, Docs, Second Brain, Twitch und jeder real gefundenen Legacy-RAG-Quelle. Vorhandene Rust-Module inventarisieren.
2. Verfolge jeden Consumer bis zu Endpoints, Provideraufrufen, Auth, Cache und Datenzugriff. Suche auch in Deployments, Cronjobs und Worker-Konfigurationen. Dokumentation und tatsächlich laufenden Pfad getrennt ausweisen.
3. Erfasse alle im Datenplan genannten Klassen, einschließlich Rohdaten, Lernartefakten, Zuständen, Anhängen, Löschsignalen und Berechtigungen. Vorhanden, nicht vorhanden, unzugänglich und unbekannt sauber unterscheiden.
4. Erstelle die Domain-Paritätsmatrix: Entities, Builds, Learning, Optimizer, Coaching, Meta, Analytics, Lineage sowie weitere wirklich gefundene Features. Erfasse Inputs, Outputs, Seiteneffekte und Testabdeckung.
5. Sichere erlaubte versionierte Testexporte und Golden-Fixtures; prüfe Backup-/Restore-Möglichkeiten. Secrets und private Daten nicht in öffentliche Berichte kopieren.
6. Miss mit Chat 10 heutige Antwortpfade, Retrieval, Domain, Ingest und Ressourcen auf benannter Hardware. Fehlende Messbarkeit bleibt eine belegte Lücke statt einer erfundenen Baseline.
7. Empfiehl Wiederverwendung, Portierung, Datenübernahme oder Abschaltung je Komponente. Noch keine Löschung oder Archivierung.

**Liefergegenstände:** Code-/Runtime-Inventar, DATENINVENTAR, DOMAIN_PARITAET, Datenflusskarte als Text, Baseline-Bericht, Zugangs-/Risikoliste und G0-Protokoll.

**Abnahme:** Jede benötigte Daten-/Funktionsklasse ist eingeordnet; kritische unbekannte Bestände blockieren G0 bzw. erhalten einen klaren Beschaffungsauftrag. Alle Ist-Behauptungen besitzen Pfad/Commit oder Runtime-Nachweis.

## Konkretisierung aus den drei Recherchen · v1.0

Zusätzlich U3/U4 auswerten und die dort genannten Rust-Crates/Adapter, Schema-/Security-PRs, Docs-Evals, Discord-Consumer `Deadlock-Bots` und Publishpfade am tatsächlichen Stand verifizieren. Historische Node-/Python-Notizen nicht als heutigen Betrieb ausgeben.

Erfasse Wiki-Namespaces/Revisionen/Abhängigkeiten, Facts/Rules/Karten, Git-Historien, OpenAPI-/Protoschemata, Replays/Observations/Population, Rechte und Referenztests. Vollständiges Quellenregister mit verfügbar/unzugänglich/unbekannt führen. Hashgleiche Uploads einmal zählen. Liefere konkrete Wiederverwendungs-/Ownershipmatrix für 02/03/12/13/14.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Prüfe den tatsächlich verfügbaren Repository- und Datenzugang und erstelle daraus die Inventarmatrix. Ohne Zugriff liefere die präzise Erhebungsliste; behaupte keine ausgeführten Scans.
