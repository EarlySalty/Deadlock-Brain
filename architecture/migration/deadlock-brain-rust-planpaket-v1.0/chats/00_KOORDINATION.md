# Chat 00 · Koordination, Architektur und Integration

**Startbedingung:** Sofort; begleitet alle Wellen bis G6.

## Kontext zum Mitgeben

Alle Grundpläne 00–08, Recherche und Nutzeranforderungen; danach STATUS, ADRs und integrierte Übergaben.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **00 — Koordination, Architektur und Integration** im Umbau von Deadlock Brain zuständig.

**Ziel:** Führe genau ein gemeinsames Rust-, Daten- und Konsolidierungsprojekt. Die Arbeitschats liefern kompatible Bausteine statt unabhängiger Teilarchitekturen.

**Deine Eigentümerschaft:** `architecture/migration/STATUS.md`, Anforderungsregister, ADR-Freigaben, Integrationsentscheidungen; keine alleinige Änderung fremder Implementierungsmodule.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Prüfe alle drei bereitgestellten Originalrecherchen U1/U3/U4, die Nutzeranforderungen U2 und 08_ERGAENZUNGEN_INTEGRIERT.md. EXT-01 ist auf Planungsebene geschlossen; identische Ökosystemuploads nicht doppelt bearbeiten. Offene Live-/Rechte-/Umsetzungsfragen getrennt erfassen.
2. Lege STATUS, Requirement-IDs, Modulbesitzer und Architekturentscheidungen an. Starte zunächst Chat 01 und das Testdesign aus Chat 10; gib Implementierungswellen anhand G0–G6 frei.
3. Vereinbare mit 02/03 die Contract-/Schema-Zuständigkeit. Änderungen an gemeinsamen Typen, Root-Manifests, Lockfile und Datenmigrationen serialisiert integrieren.
4. Für jedes Merge Basis-Commit, getesteten Commit, Contract-/Schema-Version, Testnachweis und offene Grenzen prüfen. Nicht gemergte Chatantworten sind kein Projektfortschritt im Code.
5. Bewerte Zielkonflikte: Qualität vor Sparversprechen; Rechte vor Cachetreffern; Funktionserhalt vor kosmetisch abgeschlossener Portierung. Decisions und Abweichungen dokumentieren.
6. Koordiniere abschließend die Daten-/Funktions-/Consumer-Abnahme und die ausdrücklich freizugebenden Produktiv- und Archivierungsaktionen.

**Liefergegenstände:** STATUS, ausgefüllte Anforderungsmatrix, ADR-Register, geordnete Merge-/Wellenliste, Gate-Protokolle und nächste freigegebene Chat-Aufträge.

**Abnahme:** Jede Pflichtanforderung hat einen Besitzer und einen überprüfbaren Nachweis. Kein Chat überschreibt stillschweigend eine gemeinsame Entscheidung. G6 ist erst nach realer Abschlussprüfung freigegeben.

## Konkretisierung aus den drei Recherchen · v1.0

Lies verbindlich den Parallelplan 10 und die Spezialpläne 11–13. Halte 15 Aufträge mit festen Ownern, aber starte nicht alle gleichzeitig zur Implementierung. Nach Initialisierung nur 01 und 10 Testdesign; nach G0 02/03; nach integriertem G1 die getrennten Module. Mockfreigabe und Echtintegration getrennt im STATUS ausweisen.

Vor Schreibarbeit `PFAD_OWNER.csv` am echten Repository anlegen; geeignete `dbrain-*`-Module nicht durch neue `brain-*`-Duplikate ersetzen. Neu 12 Wiki, 13 externe Quellen/Schema, 14 Replays; 05 bleibt Fach-/Buildowner. R26 ff., neue Pilot-/Coveragegates und Nutzungsfreigaben prüfen. Bereits laufende v0.9-Arbeit nicht zurücksetzen; betroffene Verträge/Gates kontrolliert erweitern.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Beginne mit dem belegten Quellenstand und einer Tabelle aus freigegebenen Zielen, offenen Fakten, Abhängigkeiten und nächstem Arbeitschat. Führe keine produktiven Änderungen aus.
