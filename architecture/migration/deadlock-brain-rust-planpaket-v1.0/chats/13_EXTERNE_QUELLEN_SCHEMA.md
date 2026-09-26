# Chat 13 · Externe Quellen, APIverträge und Schemaüberwachung


**Startbedingung:** Nach G0 Quellen-/Contractprüfung und erlaubte Fixtures; Implementierung nach G1. API-/Git-/Driftpilot G2, Pflichtfeeds G3, Ausfall-/Semantiktests G4.

**Zusätzlicher Kontext:** U4, 12_QUELLEN_UND_SCHEMAWATCH.md; tatsächliche bestehenden Sources-Adapter, Source Contract aus 02, Store 03, Worker 04, Fachnormalisierung 05.

**Ziel:** Gegenwärtige und historische Spieldaten, offizielle Änderungen und Match-/Metadaten versioniert zuführen und Quellen-/Schemaänderungen vor falscher Publikation erkennen.

**Eigentümerschaft:** externe API-/Assets-/deadlock-data-/GameTracking-/Schema-Adapter und ihre gepinnten Quellfixtures/Driftreports. Konkrete Teilpfade in vorhandenem dbrain-sources etc. reservieren. Wiki-Seiten 12, Replaydecoder 14, fachliche Auswertung 05, Root-CI/Contracts 02, SQL 03, allgemeine Jobengine 04.

1. Jede Quelle/Kandidat aus der Matrix 12 mit geplanter Rolle, tatsächlicher Verfügbarkeit, vorhandener Implementierung, Quellrevision, Nutzungsfreigabe und Herkunft einordnen. Vorhandene brauchbare Adapter härten statt duplizieren.
2. Source Contract v2 liefern: Rawhash, Version, Zeiten, Parser, Upstream, gemeinsame Ursprungsartefakte/Parserfamilien, Rechte und Prüfstatus. Keine automatisch erfundene Quellenunabhängigkeit.
3. Deadlock API/Assets auf dem realen Vertrag pinnen. Deterministische Fixture-/verbrauchte-Felder-Tests plus separaten Upstreamwatcher umsetzen. Erreichbarkeitsfehler != Breaking Change. Keine automatische Dependency-Aktualisierung im PR-Gate.
4. Git-Historie von deadlock-data commitbezogen importieren. Identitäten/Locale/Mode/Version/Generator beibehalten. Commitdiff und Reparse bei Parseränderung getrennt testen; Gitzeit nicht als Patchzeit ausgeben.
5. GameTracking/SchemaExplorer/Protoschemas als gepinnte Change-Signale konsumieren. Feldabhängigkeiten zu Parsern/Facts/Observations/Karten/Rules erfassen. Inkompatibilität oder semantische Änderung quarantiniert betroffene Daten, nicht blind das ganze System.
6. Match-/Metadaten mit Fenster/Kohorte/Quellstatus für 05 normalisieren. Offizielle Patch-/Devquellen vs. Reddit/Statlocker/Sheets und sonstige Claims getrennt behandeln. Widersprüche zur Reconciliation, nicht destruktiv überschreiben.
7. U4s sekundäre Parser/Assetprojekte nur als begründet freigegebene Referenz einsetzen. Kein eigener notwendiger Python-/Fremd-Runtimeextraktor. Kein pauschales Kopieren ungeklärter Game-Assets oder Fremdcodes.

**Liefergegenstände:** Quellenregister, gehärtete Rust-Adapter, gepinnte OpenAPI-/Git-/Schemasnapshots, Contract-/Semantic-Diffberichte, Watcher und Abhängigkeits-/Quarantänetests.

**Abnahme:** Pflichtfeeds/versionierte Historien laufen reproduzierbar, keine stillen Quellen-/Parserverwechslungen. Ausfälle/Drift sperren betroffene Veröffentlichung; Quellenrechte und korrelierte Ableitungen bleiben erhalten.

**Erster Schritt:** Bestehende Adapter und tatsächliche API-/Gitverträge gegen einen minimalen gepinnten Snapshot prüfen; Source-v2-Anforderungen an 02/03 liefern.



**Verbindlich:** Eigene reguläre Backend-, Worker-, Parser-, Learning- und Rebuildpfade sind Rust. Kein notwendiger Python-/JVM-/.NET-Sidecar, kein Legacy-HTTP-Kern. Externe fremdverwaltete Datenfeeds dürfen konsumiert werden; die eigene Implementierung/Steuerung bleibt Rust. Optionale Offline-Referenzwerkzeuge und kleine Hilfsskripte nur dokumentiert. Rechte/Egress setzt Code durch.

**Arbeitsregeln:** Lies 00_START_HIER.md, 01_MASTERPLAN.md, 02_GEMEINSAME_REGELN.md, 06_VERTRAEGE_UND_GRENZEN.md, 08_ERGAENZUNGEN_INTEGRIERT.md und 10_REIHENFOLGE_UND_PARALLELITAET.md sowie STATUS/ADRs/Übergaben unter architecture/migration/. Vor Änderungen Basis-Commit, Contract-/Schemaversion, reale freigegebene Pfade und Arbeitsmodus nennen. Vor G1 nur vorbereiten. Kein zweiter Store, Scheduler, Vertragsentwurf als heimlicher Runtimevertrag oder eigener LLMpfad. Änderungen an fremden Modulen per CHANGE_REQUEST.

**Nachweis/Übergabe:** Eigene Tests und relevante Integrationstests tatsächlich ausführen; Command, Commit, Ergebnis und nicht ausgeführte Prüfungen trennen. Golden-/Mocknachweis ersetzt keine echte Daten-/Providerintegration. Ausgefüllte vorlagen/UEBERGABE.md mit Commit/PR, Grenzen und next-owner liefern. Ohne Codezugang keine Änderungen oder Tests behaupten. Erst integrierter Commit und bestätigte Checks machen den Auftrag fertig.
