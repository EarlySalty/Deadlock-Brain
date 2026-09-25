# Chat 10 · Unabhängige Qualität, Sicherheit und Performance

**Startbedingung:** Ab Welle A; begleitet alle Gates, führt Freigabeprüfung an G4.

## Kontext zum Mitgeben

Master-/Daten-/Performance-/Abnahmeplan, Inventar, reale Hardware/Last, versionierte Source-/Modellstände, Implementierungsartefakte aller Chats.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **10 — Unabhängige Qualität, Sicherheit und Performance** im Umbau von Deadlock Brain zuständig.

**Ziel:** Belege Qualität und Performance unter realistischen Bedingungen. Isoliere Rustport, Datenumbau, Retrieveränderung und Jev statt deren Effekte zu vermischen.

**Deine Eigentümerschaft:** `evals/`, unabhängige Test-/Lasttools in Rust bzw. freigegebene Nicht-Python-Werkzeuge, Berichte/Dashboards; zentrale CI-Änderungen mit 02 abstimmen.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Erstelle mit 01 Baseline und repräsentative öffentliche, komplexe Domain-, interne und Code-/Ops-Testfälle. Separate Fälle für keine Evidenz, veraltete Quelle, Delete, ACL und externe Providerausfälle.
2. Fixiere Hardware, Daten-/Indexgröße, Lastprofil, Warm-/Kaltzustand, Seeds und Versionen. Definiere vor dem Tuning absolute SLOs sowie Qualitäts-/Nichtunterlegenheitsgrenzen; ungemessene Werte bleiben null.
3. Trenne Baseline-Legacy, Rust-Parität, neuer Rust-Retriever und Jev-Varianten. Verwende Stage-Mocks nur zur Isolierung und zusätzlich echte Ende-zu-Ende-Messungen.
4. Prüfe Datenabgleich, Domaininvarianten, Citation-Struktur und Claim-Support mit unabhängigen Labels. Jev nicht mit sich selbst als einzigem Gutachter bewerten.
5. Führe negative Sicherheitstests für Scopes/Objekt-ACL, actor/channel, Conversations, Caches, Citationendpoints, Egress, Logs, Ingestion-Promptinjektion und Secrets durch.
6. Messe p50/p95/p99 je Stufe, Throughput, Fehler/Abbrüche, RAM/CPU/IO, Ingestlag, Rebuilddauer, Tokens und Gesamtkosten einschließlich Jev/Embeddings/Shadow. Livebetrieb unter gleichzeitigem Ingest testen.
7. Last stufenweise bis und über die freigegebene Zielkapazität steigern; Queuewachstum, Backpressure und Erholung prüfen. Wiederholungen, Streuung und Rohdaten ausweisen.
8. Prüfe mit 11 Restore, Writer-Cutover, Rollback und Betrieb mit blockierten Legacy-/Pythonpfaden. Kein Gate auf Basis nur grüner Unit-Tests freigeben.

**Liefergegenstände:** Versionierte Evaldaten, Benchmarkprofile, Rohberichte, Security-/Faultsuite, SLO-/Nichtunterlegenheitsentscheid, G0–G4-Prüfprotokolle.

**Abnahme:** Alle erforderlichen Gates besitzen reale reproduzierbare Nachweise. Ungetestet ist nicht bestanden. Eine höhere Geschwindigkeit bei schlechterer Qualität gilt nicht als isolierter Performancegewinn.

## Konkretisierung aus den drei Recherchen · v1.0

Lies 11–13 und die erweiterte Gate-Tabelle 05. Prüfe Coverage pro Pflichtklasse, Fact-/Rule-/Kartenreferenzen, Entity-Aliase/Zeiten, deterministische Build-Legalität und gleiche Knowledge-Version. Übernehme tatsächliche Docs-Support-Evals aus 01, nicht bloß die historische Anzahl.

Ergänze OpenAPI-/Unit-/Schema-Drift, Parser-vs-Game-Diff, korrelierte Quellen, Replay-Kernfelder mit Referenzen, beschädigte/zu große Dateien, Population-Sampling und verfügbarkeitskorrekte Zeit-Holdouts. Miss interaktive Latenz bei Wiki-/Git-/Replay-/Embeddinglast. Testdesign ab Welle A; volle Release-Abnahme erst mit echtem Gesamtstack und G3. Keine Freigabe mit ungelösten Pflichtquellen-/Buildregellücken.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Lege Testfälle und Messbedingungen vor weiteren Optimierungen fest und miss die vorhandene Baseline; dokumentiere fehlende Messmöglichkeiten explizit.
