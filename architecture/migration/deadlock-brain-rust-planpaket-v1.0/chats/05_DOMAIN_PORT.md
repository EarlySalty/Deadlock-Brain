# Chat 05 · Domainlogik erhalten, nach Rust portieren und ausbauen

**Startbedingung:** Nach G1 parallel; strukturierter Pilot für G2, vollständige Parität vor G3.

## Kontext zum Mitgeben

Feature-/Dateninventar, gesicherte Altfixtures, Fachinvarianten, Contracts, Domainstore-/Artefaktports aus 03.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **05 — Vollständiger Rust-Port der Domainlogik** im Umbau von Deadlock Brain zuständig.

**Ziel:** Erhalte das bestehende Deadlock-Know-how in Rust. Der Umbau darf nicht aus einer breiten Domainengine wieder nur einen einfachen RAG-Bot machen.

**Deine Eigentümerschaft:** `crates/brain-domain/`, fachliche Tests/Benchmarks und Domain-Paritätsmatrix; benötigte Schemaänderungen mit Chat 03 abstimmen.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Prüfe jede inventarisierte Funktion und ihre echte Implementierung. Plane Entitäten/Normalisierung, Builds, Optimierung/Learning, Coaching, Meta, Analytics, Lineage und weitere entdeckte Features getrennt.
2. Erfasse Einheiten, Patch-/Zeitgültigkeit, ID-Abbildungen, numerische Genauigkeit, Seeds, Sortierung und Seiteneffekte. Definiere vor Portierung die fachlich zulässigen Toleranzen.
3. Baue Differential-/Golden-/Propertytests auf bereinigten fixierten Daten. Altes Verhalten ist Vergleichsbasis, nicht automatisch fachlich korrekt; bekannte Altbugs nicht ungeprüft konservieren.
4. Portiere zuerst deterministische Grundlagen, danach datenintensive Berechnungen. Lege CPU-/Speicher-/Abbruchbudgets fest und liefere typisierte berechnete Evidenz mit Algorithmus- und Datasetversion.
5. Portiere regelmäßig benötigte Lern-/Updateverfahren und Artefaktladung. Ein exportiertes Modell genügt nicht, wenn der nächste Lernzyklus Python braucht. Native Nicht-Python-Runtimes nur nach dokumentierter Zulassung.
6. Jede nicht portierte erforderliche Funktion ist Blocker oder ausdrücklicher Produktentscheid; kein stiller Ersatz durch ein allgemeines LLM-Prompt.
7. Vergleiche identische Arbeit vor/nach Portierung; verhindere versteckte Netz-/Legacyaufrufe und dokumentiere Daten-/Rebuildbedarf.

**Liefergegenstände:** Rust-Domainmodule, vollständige Feature-Paritätsmatrix, Fixture-/Differentialtests, Artefakt-/Algorithmusversionen, Profiling-/Benchmarkberichte.

**Abnahme:** Alle erforderlichen Funktionen bestehen fachliche Abnahme inklusive regelmäßiger Updates. Keine FFI-/HTTP-Rückdelegation an Python. Ergebnisunterschiede sind erklärt und genehmigt.

## Konkretisierung aus den drei Recherchen · v1.0

Lies 11 und 13. Vorhandenes funktionierendes Rust erhalten und nur belegte Lücken portieren. Baue fehlende gemeinsame Fact-/Effect-/Mechanik-/Ruleverarbeitung, sicheren Ausdrucks-Evaluator, Graph-/Synergieableitung und deterministischen Build Planner. Unknown ist nicht 0; Einheiten, Mode, Variante, Patch, Budget, Inventar- und Upgradegrenzen versionieren.

Hero-Wissenskarten erhalten von dir Fakten-/Synergie-/Ruleinput, aber keine eigene alternative Engine. 12 erzeugt die Kartenansicht. 14/13 liefern empirische Beobachtungen/Meta; integriere sie getrennt von Mechanik-Wahrheit in Population/Learning/Reasoner. Abgeleitete Empfehlungen kenntlich machen. Legalität, kleine exakte Optimierungsreferenzen, Varianten, Zeit-Cutoffs und unbekannte Regeln testen; gleiche Modellantwort nicht als einzige Qualitätsreferenz verwenden.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Wähle eine inventarisierte deterministische Funktion und belege deren Parität auf fixierten Inputs; erst danach die nächste Funktionsgruppe portieren.
