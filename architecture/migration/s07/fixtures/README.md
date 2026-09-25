# Synthetische S07-Vertragsfixtures

Alle Dateien wurden für diese Vorbereitung selbst erstellt. Keine Passage,
Nutzerkennung, Replaydaten, Credentials oder API-Antwort stammt aus Produktion.
Modellnamen sind Testplatzhalter. Die Token-/Confidencewerte sind erfundenes
Testmaterial, keine Messungen. Dimension 3 ist nur eine Fixtureeigenschaft.

`MANIFEST.json` bindet jede Datei über SHA-256 an ihre erwartete Verwendung
und gegebenenfalls den zugehörigen Request. `SHA256SUMS` kann aus diesem
Verzeichnis mit `sha256sum --check SHA256SUMS` geprüft werden. Das bestätigt
Integrität, nicht Providerfunktionalität. `reject_proposed_profile`-Dateien
sind überwiegend absichtlich **gültiges JSON mit falscher Semantik**.

Die fünf Rubrikpaare sind getrennt nutzbar; das Mixed-Paar prüft die
Antwortzuordnung unabhängig von Map-Reihenfolgen. Zusätzliche Positivfälle
behandeln Noul-Grenzen, Score über 1, Choice-Gleichstand, fehlende/null Usage
und umgeordnete Embeddings. Die Rohdateien `.txt` decken abgebrochenes JSON,
NaN und doppelte Schlüssel ab. Doppelte Schlüssel sind syntaktisch nicht in
jedem JSON-Parser verboten, werden im vorgeschlagenen Sicherheitsprofil aber
vor der Map-Konstruktion abgelehnt.

Die Validierungstoleranz im Manifest gilt nur für die Offline-Fixtureprüfung,
nicht als festgelegte Produktionstoleranz. Der Contractowner entscheidet
entsprechend dokumentierter Rundung. Bei Gleichstand darf jede tatsächlich
maximale erlaubte Option zurückkommen. Confidence wird auf Bereich geprüft,
nicht mittels einer erfundenen gemeinsamen Formel nachgerechnet.

Die eigentlichen Rust-Contract-/Fake-Server-Tests folgen erst nach G1 und
Portsfreigabe. Dann zuerst die fehlerhaften Antworten als rote Gegenproben
am Bestandsclient nachweisen. `FAULTMATRIX.csv` führt zusätzlich die Fälle,
die erst durch kontrolliertes Transportverhalten geprüft werden können.
