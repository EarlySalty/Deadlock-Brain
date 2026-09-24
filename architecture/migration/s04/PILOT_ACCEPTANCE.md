# S04: Erster Pilot und Abnahmeplan

Planstand: 1.0. Basis der Vorprüfung: `c00fc8935048bf490c1e4790f7c6195864ad49e2`.

**Sämtliche Fälle unten sind Testanforderungen. Kein Fall wurde in S04 implementiert oder ausgeführt.** Es gibt noch keinen S04-Rebuildbericht, Feeder-A/B-Messwert oder Live-Lastnachweis. Erst G1, gemeinsame Ports und eine reale isolierte S03-Testdatenbank machen die Implementierung zulässig. Dieser Plan definiert kein JSON-Wireformat und keine Ersatz-Storagearchitektur.

## Erster vertikaler Durchstich nach G1

Genau ein deterministischer lokaler Datei-/Dokument-Connector nutzt den integrierten Source-, Policy-, Raw-, Store-, Checkpoint- und Publishvertrag. Er läuft im vorhandenen Rust-Workspace. Kein neues Pythonprogramm, kein externer Modellaufruf, keine Produktionsquelle und kein zweiter Scheduler.

Als neu zu erstellende synthetische Eingabe dient eine kleine Markdown-Datei mit Überschrift, deutscher Unicode-Zeile, Tabelle und Codeblock. Es werden keine privaten Docs-/Sessioninhalte übernommen. Eine zweite synthetische Quelle enthält identische Bytes bei anderen Rechten. Fixture-IDs und Ablaufzeiten sind deterministisch; echte Laufzeiten werden separat gemessen. Parser/Chunker kommen aus der freigegebenen Konfiguration von 06; es wird keine universelle Chunkgröße erfunden.

Ablauf: erstmaliger Import; identische Wiederholung; Inhaltsupdate; reine Rechteänderung; Quellenlöschung; Restart/Re-delivery; Neuaufbau aus erlaubten Raw-Revisionen plus Delta. Erst nach bestandenem echten Store-Pilot weitere Quellen anbinden. Ein In-Memory-Test ist `mock_verified`, nicht `integration_verified` und nicht G2.

## Prüffälle

| ID | Auslösung / Gegenprobe | Erwartete Invariante und Nachweis | Zuständigkeit |
|---|---|---|---|
| P01 | Erste synthetische Datei importieren. | Raw-Hash, stabile Quellidentität, Revision, Provenienz und exakte Quellpositionen sind gespeichert; aktivierte Sicht enthält Überschrift, Tabelle und Code unverfälscht. Inhalte und Policy werden über reale Ports gelesen. | 04/03/06 |
| P02 | Dieselbe gültige Revision mit gleicher Parser-, Policy-, Dependency- und Embeddingkonfiguration nochmals liefern. | Kein zweiter aktiver Record, keine zusätzliche Embeddingarbeit. Es werden wirkliche Schreib-/Job-/Providercounter verglichen, nicht bloß identische Ausgabetexte. | 04/03/06 |
| P03 | Bei gleichen Bytes nur Titel/URL oder ACL ändern. | Metadatenänderung ist sichtbar; Rechteentzug wirkt ohne Inhaltswechsel. Historischer Release und Cache umgehen aktuelle Rechte nicht. Keine unnötige Neueinbettung des unveränderten Textes. | 04/02/03/06 |
| P04 | Inhaltsrevision oder eine Template-/Parser-/Schema-/Fact-Abhängigkeit ändern. | Nur betroffene Ableitungen werden ungültig; nicht betroffene Dokumente bleiben erhalten. Gleiche Raw-Daten mit neuem Parser sind kein neuer Spielpatch. Nur erforderliche Embeddings entstehen neu. | 04/06/12/13 |
| P05 | Identische Bytes aus zwei getrennt berechtigten Quellen importieren. | Keine Vereinigung der Quellrechte. Sperre einer Quelle erzeugt keine Sichtbarkeit über ihre Dublette. Ein Principal mit ausschließlich anderen Rechten kann die gesperrte Herkunft nicht lesen. | 04/02/03 |
| P06 | Dokument löschen und danach alte Update-/Create-Ereignisse erneut liefern. | Persistente Löschmarkierung und aktuelle Policy verhindern Resurrection. Ableitungen/Cache/Index werden idempotent invalidiert. Physisches GC erfolgt nur nach freigegebener Retention. | 04/03/06 |
| P07 | Nach Delete ein ausdrücklich neueres und autorisiertes Wiederherstellungsereignis senden. | Nur der gemeinsame Versions-/Policyvertrag erlaubt eine neue Aktivierung; alte oder unautorisierte Ereignisse bleiben abgewiesen. | 04/02/03 |
| P08 | Zugriff auf Eltern eines generierten Dokuments entziehen. | Ableitung darf weder Quelle noch Zitat umgehen; aktuelle konservative Elternrechte gelten auch für historischen Release und Provider-Egress. | 04/02/03/07 |
| P09 | Pagination nach erster Seite mit Timeout/403 abbrechen. | Kein vermeintlich vollständiges Leer-/Teilmanifest, keine Massenlöschung. Fehlerzustand sichtbar; Checkpoint entspricht der definierten dauerhaften Grenze. | 04/03 |
| P10 | Prozess beim Raw-Schreiben abbrechen. | Unvollständige Datei wird nach Neustart nicht aufgrund ihres Namens als gültig angenommen. Hash/Länge stimmen vor Referenzierung; Wiederholung verliert keine bestätigten Änderungen. | 04/03 |
| P11 | Prozess unmittelbar vor und unmittelbar nach dem Commit, aber vor Eingangs-Ack beenden. | Vor Commit erneute Verarbeitung; nach Commit idempotente Wiederholung. Revision, Checkpoint und dauerhafte Folgearbeit bleiben konsistent; keine doppelte aktive Veröffentlichung. | 04/03 |
| P12 | Lease ablaufen lassen; zweiter Worker übernimmt, erster beendet verspätet. | Veralteter Worker kann weder Checkpoint zurücksetzen noch neue Aktivierung überschreiben. Gleichzeitige Claims haben die von 03 garantierte Semantik. | 04/03 |
| P13 | Validierung oder Publish nach gesichertem Ingest fehlschlagen lassen. | Quarantäne verhindert Aktivierung. Aktive konsistente Generation bleibt erkennbar. Dauerhafte Folgearbeit erlaubt Wiederanlauf; bestätigter Ingest wird nicht als bereits veröffentlicht ausgegeben. | 04/03/12 |
| P14 | Leeren isolierten Store aus freigegebenen Raw-Revisionen rekonstruieren, anschließend Deltas anwenden. | Inhalts-/Policy-/Tombstone-/Dependencymanifest mit Referenz vergleichen. Nur ausdrücklich volatile Laufmetadaten aus dem deterministischen Vergleich ausschließen. Aktuelle Rechte gelten auch beim Restore. Bericht nennt tatsächliche Mengen, Hashes, Lücken und Dauer. | 03/04/06 |
| P15 | 401/403, 429 mit Retry-After, Timeout, 5xx und ungültige Antwortform injizieren. | Fehlerklasse stimmt; Versuche, Wartezeit, Antwortgröße und Gesamtdeadline bleiben begrenzt. Keine Tokens, Authheader oder privaten Bodies im Log. | 04/02/07 |
| P16 | Gleiche URL mit verschiedenen Auth-/Policykontexten abfragen; zusätzlich Redirect auf unerlaubtes Ziel. | Kein kontextübergreifender Cachetreffer und keine Weitergabe von Credentials an unerlaubten Ursprung. Keine privaten Inhalte verlassen die erlaubte Egressgrenze. | 02/04/07 |
| P17 | Lokale Pfadausbrüche, Symlinks außerhalb der Wurzel, zu große Datei/Antwort und starke Dekompression prüfen. | Input bleibt innerhalb freigegebener Wurzel und Ressourcenbudgets. Ungültige Eingabe ist sichtbar abgewiesen oder quarantiniert, nicht still als vollständig verarbeitet. | 04/02 |
| P18 | Replay-/Bulk-/Embeddingqueues füllen; parallel reguläre Deltas, Delete/Revoke und Livefragen erzeugen. | Freigegebene Queue-/RAM-/CPUgrenzen halten; Revokes warten nicht unbegrenzt auf Bulk. Live-p95/p99 und Queuewartezeit gegen das von 10 eingefrorene Profil messen. Kein Grenzwert wird hier erfunden. | 04/10/14 |
| P19 | Writer neu starten und denselben Auftrag mehrfach zustellen; zusätzlich Fehler beim gemeinsamen Wiki-Releasepfad. | Wiederaufnahme ohne doppelte Aktivierung, verlorenen bestätigten Fortschritt oder konkurrierenden Releasezeiger. Vorhandene Wiki-Sicherungen bleiben erhalten. | 04/03/12 |
| P20 | DecisionProvider im Shadow vergleichen; generiertes Digest erneut als Eingabe anbieten; Decision/Answer ausfallen lassen. | Keine rekursive Originalereignis-/Digestkette. Decision und Schreiben bleiben getrennt; keine modellbasierte Rechtefreigabe. A/B-Bericht zeigt echte false negatives, Qualität, Latenz und Kosten statt angenommener Einsparung. | 04/07/10 |

## Nachweisstufen und Messartefakte

Bei Implementierung jeden Fall mit getestetem Git-Commit, Contract-/Schema-/Parser-/Policy-/Knowledge-Version, reproduzierbarem Befehl, isolierter Umgebung, konkretem Ergebnis und Artefakt erfassen. Nicht ausgeführte Fälle bleiben offen. Mock, echter lokaler Store, erlaubter Live-Vertrag und gemischter Lasttest sind verschiedene Nachweise.

Rebuildbericht: Referenz- und Zielmanifest, Record-/Tombstone-/Quarantänemengen, Scopeprüfung, verlorene oder doppelte Revisionen, Delta-Wasserstand, Wiederanlaufversuche, Dauer und Peak-Ressourcen. Der Bericht existiert erst nach P14; dieser Testplan ist kein Bericht.

Feederbericht: freigegebene und bereinigte Ereignisstichprobe, feste Baseline, Shadow-Entscheidungen, Fehlklassifikationen insbesondere relevante übersehene Ereignisse, Generationsqualität, konservative Elternrechte sowie tatsächliche Zeit-/Token-/Kostenwerte. Ohne integrierten 07-Port und Freigabe keine Provideraufrufe.

Versorgungsbericht: je freigegebener Quelle letzter dauerhafter Ingest, letzte validierte Aktivierung, Checkpoint, Alter, Fehler-/Quarantänestatus und bekannte Pflichtlücken. `source_runs.status=ok` allein beweist keine aktuelle Vollversorgung.

## Ausführung nach Implementierung

Nach Ownerfreigabe die im Paket vorgeschriebenen Workspaceprüfungen im `rust/`-Verzeichnis auf dem integrierten Stand ausführen:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --release --locked
```

Zusätzlich reale isolierte Store-/Crash-/Rebuildfälle und das freigegebene Lastprofil ausführen. Benötigte Datenbankparameter und native Abhängigkeiten liefert der integrierte S02/S03-Testvertrag. Kein Test darf versehentlich auf die Produktionsdatenbank zeigen. Keine fehlgeschlagenen Gates, ignorierten Tests oder reinen Fixtureergebnisse als integrierte Abnahme ausgeben.
