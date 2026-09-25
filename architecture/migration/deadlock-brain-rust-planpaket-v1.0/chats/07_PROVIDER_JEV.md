# Chat 07 · Rust-Provider und Jev

**Startbedingung:** Vertragsprüfung früh möglich; Implementierung nach G1, Integration bis G2/G3; Aktivierung je Funktion an G4.

## Kontext zum Mitgeben

Providerinventar, aktuelle Primärdokumentation, Contracts, Egress-/Budgetregeln, Query-/Feeder-/Relevanzlabels von 10.

Lies zusätzlich `00_START_HIER.md`, `01_MASTERPLAN.md`, `02_GEMEINSAME_REGELN.md`, `08_ERGAENZUNGEN_INTEGRIERT.md`, `10_REIHENFOLGE_UND_PARALLELITAET.md`, den aktuellen `STATUS.md` sowie passende ADRs und Übergaben. Pfade beziehen sich auf `architecture/migration/` im Zielrepo; dieses Paket muss dort vorher bereitgestellt werden. Ein anderer Chatverlauf ist kein automatisch verfügbarer Kontext.

## Direkt nutzbarer Arbeitsauftrag

Du bist für Arbeitspaket **07 — Rust-Provider und Jev** im Umbau von Deadlock Brain zuständig.

**Ziel:** Implementiere zentrale robuste Modellzugänge in Rust und mache Jev messbar einsetzbar, ohne Permissions oder korrekte Antworten davon abhängig zu machen.

**Deine Eigentümerschaft:** `crates/brain-providers/`, `crates/brain-jev/`, Providerfixtures und jeweilige Vertragstests. Keine Providerkeys in Consumer-Apps.

Die reale Pfad-/Ownerdatei ist maßgeblich; vorgeschlagene `brain-*`-Namen erzwingen keine Umbenennung vorhandener `dbrain-*`-Crates. Arbeite auf dem letzten integrierten Basis-Commit und nenne vor Änderungen Contract-/Schema-Version, relevante Voraussetzungen und vorgesehenen Dateiumfang. Prüfe, ob die Startbedingung erfüllt ist. Schaffe keine zweite private Schnittstelle, wenn eine gemeinsame fehlt. Benötigte Änderungen fremder Module über `vorlagen/CHANGE_REQUEST.md` an deren Besitzer geben.

1. Prüfe vorhandene LLM-/Embedding-/Jev-APIs und zulässige Modelle. Verwende direkte dokumentierte HTTP-Verträge statt eines Python-SDK-Sidecars. Versions-/Aliasauflösung und Nutzungsdaten festhalten.
2. Implementiere typisierte serialisierte Requests/Responses, begrenzte Bodygrößen, persistente HTTPclients, Deadline, Retrybudget, Backoff/Jitter, Circuit Breaker und redigierte Logs.
3. Validiere Antworten: erwartete Question-IDs, erlaubte Labels, endliche Werte, Wertebereiche, fehlende/zusätzliche Felder gemäß Vertrag. Noul, Choice und Score nicht in eine erfundene einheitliche Confidence umdeuten.
4. Implementiere Jev-Bausteine für Relevanz, Feederklassifikation, Queryrouting, Answerability und Modellrouting getrennt schaltbar. Entscheidungsergebnisse dürfen nur innerhalb der vom Rust-Code erlaubten Quellen/Provider/Budgets wirken.
5. Schreibe Mock-/Faulttests und getrennte echte Provider-Vertragstests; letztere nur mit freigegebenen Credentials und bereinigten Daten. Getesteter HTTPvertrag ist noch kein Qualitätsnachweis.
6. Führe Shadowauswertung durch und kalibriere Schwellen auf Entwicklungslabels; Testlabels separat halten. Verlorene relevante Evidenz, tatsächliche Tokens, Gesamtkosten und Netzwerkrunden messen.
7. Liefere eine sichere Rust-Regelbaseline bei Fehlern und beschränke Ausführung auf erlaubten Egress. Eine Aktivierung ohne Nutzennachweis als offen oder abgelehnt dokumentieren.

**Liefergegenstände:** Rust-Clients, Modell-/Rubrikversionen, Contract-/Faulttests, Shadowberichte, Schwellen-/Aktivierungsentscheid pro Jev-Funktion.

**Abnahme:** Keine Pythonabhängigkeit; keine unkontrollierte Datenübermittlung; Timeout/429/5xx/kaputtes JSON sind getestet. Jev ist integriert, aber nur nach gemessenem Nutzen aktiv.

## Konkretisierung aus den drei Recherchen · v1.0

Providerclients und Jev nutzen die erweiterten Evidenztypen, ohne harte Regeln, Canonical-Facts oder Rechte zu ersetzen. Für strukturierte Lookups keinen obligatorischen Modellcall einführen. Jev-Kontext kompakt halten, wichtige Quellenpassagen jedoch nicht vor der Bewertung wegkürzen.

Egress-/Nutzungsregeln für Wiki-Text, interne Daten und Replay-/Spielerbezug vor Modellaufrufen durchsetzen. Keine Vendorpreise/Benchmarks aus U1 als aktuelle eigene Messung verwenden. Aktuelle echte Verträge prüfen, Implementierung und Aktivierungsnachweis getrennt; Jev auch im vollständig erweiterten Korpus zunächst Shadow.

**Verbindlich für diesen Chat:** Der eigene produktive Backendkern einschließlich Worker, regelmäßiger Learning-/Rebuildverfahren und Adapter ist Rust. Kein PyO3-/Python-Sidecar-/Legacy-HTTP-Kern. Kleine optionale oder einmalige Hilfsskripte nur dokumentiert, nicht als Betriebsabhängigkeit. Quellenrechte und externe Datenfreigabe setzt Code durch, nicht Jev oder das Antwortmodell. Vorhandene Funktionen und Daten werden nicht stillschweigend gestrichen.

**Tests und Übergabe:** Führe die für deine tatsächlichen Änderungen relevanten Tests aus. Dokumentiere Befehl, getesteten Commit, Resultat und nicht ausgeführte Prüfungen getrennt. Ohne Repo-/Runtimezugriff keine Änderungen oder erfolgreichen Tests behaupten. Liefere am Ende `vorlagen/UEBERGABE.md` ausgefüllt: Commit/PR, Artefakte, Versionen, Daten-/Performance-/Sicherheitsfolgen, Blocker und next-owner. Ein Chat-Abschluss ersetzt keine Integration durch Chat 00.

**Erster Schritt:** Prüfe aktuelle Providerverträge und implementiere einen typisierten Rust-HTTPclient mit Fake-Server-Tests. Übernimm SDK-Beispiele nicht ungeprüft als Rustvertrag.
