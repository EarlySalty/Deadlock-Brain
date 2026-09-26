# Übergabe · C2/C3 — Chunked Retrieval und korrekte Readerfehler

Status: lokale Abnahme bestanden; bereit für unabhängiges PR-Review. Nicht integriert.

- Basis-Commit: `087c522deda58ecf4bd6843167f51c54f681e944` (`origin/migration/rust-integration` beim Anlegen des Worktrees).
- Runtime-Implementierung: `600ddd712f607221d9be5cab42b25113f6481a96`.
- Abschließend getesteter Code-/Test-Commit: `84d7ab262e60e8695d525736258e9948dd254bd8`; enthält zusätzlich die Korrektur der Lasttest-Assertion, keine weitere Runtimeänderung.
- Branch: `codex/fix-c2-c3-retrieval`; PR-Ziel: `migration/rust-integration`.
- Umfang: ausschließlich C2 und C3 aus `architecture/migration/INTEGRATION_REVIEW.md`. Keine Freigabe anderer Reviewpunkte oder globaler R01–R60-/Migrationsgates.
- Contract-Versionen: `brain.v1`, `brain.public.v1`, `brain.store.v2`; additive Evidenzprovenienz, `unavailable` und gezielter Head-Read-Port, siehe Kompatibilität unten. Keine SQL-Schemamigration.
- Chunker: `utf8-window-v1-1024-overlap192`; BM25: `k1=1.2`, `b=0.75`.
- Pilot: Release `pilot-r1`, Knowledge-Version `pilot-knowledge-v1`, Patch `pilot-20260925`; sieben bereits angebundene reale Dokumente. Antwortprovider ist der lokale `pilot-loopback-model`-Stub, kein Produktionsmodell.

## Ergebnis und konkrete Änderungen

### C2: Dokumente, Index und Budget

`rust/crates/dbrain-retrieval/src/chunk_index.rs` implementiert einen Rust-internen invertierten BM25-Index mit Term-Postings. Es handelt sich ausdrücklich nicht um Tantivy oder PostgreSQL-FTS. Die vorhandenen Rust-Abhängigkeiten reichen aus; `Cargo.lock` und Abhängigkeiten bleiben unverändert. Anfragearbeit traversiert passende Postings statt sämtliche Dokumenttexte erneut zu tokenisieren.

Prosa wird deterministisch in unveränderte UTF-8-Ausschnitte mit Zielgröße 1.024 Bytes und bis zu 192 Bytes Überlappung zerlegt. Grenzen liegen an Zeichen-/Tokenübergängen. Die Vereinigung der Ausschnitte deckt sämtliche Originalbytes einschließlich Whitespace ab. Sehr lange unteilbare Tokens und typisierte Fakten/Regeln werden nicht abgeschnitten; passen sie nicht ins Budget, entsteht ein expliziter Budgetfehler. Eine begrenzte Retrieval-Auswahl bedeutet nicht, dass nicht ausgewählte Textteile aus Dokument oder Index gelöscht werden.

`brain-contracts/src/retrieval.rs` ergänzt `ChunkProvenance` und eine körperlose `DocumentHead`-Projektion. Jeder Chunk behält die ursprüngliche Dokumentidentität, Revision, Content-Hash, Quellenlocator, einen halboffenen Bytebereich, Ordinal, Gültigkeit, Release-/Knowledge-Version und sämtliche Originalmetadaten. Dazu gehören Patch, Mode, Aliase und vorhandene Upstream-Provenienz. Evidenz-IDs sind SHA-256-basiert und an Release, Revision, Hash, Chunker und Bytebereich gebunden. Verschiedene Chunks behalten denselben logischen Dokumentnamen, aber unterschiedliche Evidenz-IDs.

`release_port.rs` hält bis zu vier unveränderliche Release-Indizes gemeinsam für Retriever-Klone. Ein kalter Indexaufbau liest und validiert einmal den vollständigen Snapshot; parallele Kaltstarts desselben Retrievers bauen ihn nicht mehrfach. Danach lesen Retrieval und jede Evidenzvalidierung ausschließlich aktuelle Heads ausgewählter Kandidaten. `SnapshotReadPort::read_heads` ist ein gemeinsamer Port, kein privater Nebenvertrag. `LocalPgReader` implementiert eine parametrisierte, nach Dokumentschlüsseln begrenzte SQL-Abfrage ohne Dokumentkörper oder komplette Release-Pins. Der Memory-Reader implementiert entsprechende Schlüsselzugriffe. Der Port akzeptiert maximal 256 Schlüssel, der Retriever arbeitet in Batches bis 128. Explizite Bulk-Diagnosemethoden bleiben vorhanden, werden aber nicht vom neuen Frage-/Hybridpfad benutzt.

Historische **und** aktuelle Berechtigungen müssen gelten. Scopes werden restriktiv vereinigt, die restriktivere Sichtbarkeit bleibt erhalten. Aktuelle Heads werden nicht gecacht. Revoke, Tombstone und Egress-Sperren werden bei der nächsten frischen Übergabeprüfung wirksam, einschließlich nach Provider-Aufrufen und bei Cache-/Single-Flight-Antworten. Gefälschte Inhalte, Bytebereiche, Quellenlocator, Metadaten, Revisionen oder ACLs werden zurückgewiesen.

Die Tokenisierung erhält numerische Literale: beispielsweise werden `6.5`, `65`, `-6.5` und `6,5` nicht gleichgesetzt. Anfragen mit expliziten Zahlen müssen diese im Chunktext treffen. Dokumentnamen, explizite Quellenaliase und eine kleine versionierte DE/EN-Vokabularzuordnung werden indexiert; Unicode, Groß-/Kleinschreibung und deutsche Umlauttransliteration sind abgedeckt. Dies ist keine allgemeine Übersetzungs- oder Entity-Resolution-Engine.

Der vorhandene Dense-/Embedding-Port bleibt erhalten. Dokumentvektoren werden nach Release-/Hash-/Patch-/Mode-/ACL-Prüfung auf dieselben kanonischen Chunks abgebildet. Faktenfragen, numerische Literale, Stat-/Code-Identifier und Treffer mit kanonischen Fakten/Regeln umgehen Dense. Gewichtete RRF ist deterministisch einschließlich der Reihenfolge der Fließkommaaddition. Doppelte IDs erhalten keinen Zusatzbonus; widersprüchliche Evidenz oder ACLs werden niemals zusammengeführt. Keine neue Embedding-Modellentscheidung und kein Reembedding.

`brain-contracts/src/provider_input.rs` definiert die gemeinsame modellseitige Promptdarstellung für Retriever-Packing und `brain-providers`. Gezählt wird der tatsächlich modellseitige Text nach Dekodieren der äußeren HTTP-JSON-Hülle, einschließlich der inneren Evidenz-JSON im User-Text, Rollen und Framingreserve. Das zusätzliche HTTP-Escaping und reine Transportfelder werden nicht als Modellinput reserviert. Die Schätzung bleibt eine konservative UTF-8-Byte-/Token-Obergrenze, ausdrücklich **keine** pauschale Bytes-durch-vier-Schätzung und kein behaupteter exakter Modelltokenizer. Providerusage, Retry-Reservierung und Kostenlimits bleiben geprüft. Das Defaultbudget wird nicht angehoben.

### C3: Fehlervertrag und Fail-closed

`brain-contracts` ergänzt `PortError::PermissionDenied` und `AnswerStatus::Unavailable` (Wirewert `unavailable`). Nicht implementierte kanonische Validierung und nicht implementierte gezielte Head-Reads melden `Unavailable`, niemals einen stillen Full-Snapshot-Fallback.

Der Kernel klassifiziert Fehler der kanonischen Evidenzvalidierung einheitlich:

| Ursache | Antwortstatus |
|---|---|
| Explizite Berechtigungsverweigerung, Revoke, Delete, gefälschte Evidenz | `unauthorized_evidence` |
| DB-, Pool-, Timeout- oder temporärer Readerfehler | `unavailable` |
| Ungültige/defekte Readerantwort bei der kanonischen Validierung | `unavailable` |
| Verbrauchtes Request-/Portbudget | `budget_exceeded` |

Dies gilt für die Prüfung vor der Antwort, unmittelbar vor Provider-Egress, nach dem Provider-Aufruf, für Cache-Treffer und für Single-Flight-Follower. Technische Fehler veröffentlichen weder alte Antworttexte noch Evidenz; die bereits verbrauchte Providerusage bleibt bei einem Fehler nach dem Netzwerkaufruf erhalten. Eine fehlgeschlagene Cachevalidierung entfernt den Eintrag und löst nicht verdeckt einen neuen Provideraufruf aus. Bestehende Authentifizierung und die Klassifikation externer Providerfehler wurden nicht durch einen pauschalen neuen Fehlerstatus ersetzt.

## Nachweise

Arbeitsverzeichnis für Cargo: `rust/`. Toolchain: Cargo/Rust `1.97.1`. `SQLX_OFFLINE=true`; ausschließlich lokale Scratch-Datenbanken beziehungsweise Fixtures. Testlogs liegen im ignorierten `.core-test-logs/` des isolierten Worktrees.

| Prüfung | Befehl / Testumgebung | Tatsächliches Resultat | Artefakt |
|---|---|---|---|
| Format | `cargo fmt --all -- --check` | Exit 0 | `.core-test-logs/acceptance-84d7ab2/fmt.log` |
| Clippy | `cargo clippy --workspace --all-targets --locked -- -D warnings` | Exit 0 | `.core-test-logs/acceptance-84d7ab2/clippy.log` |
| Workspace | `cargo test --workspace --locked` | Exit 0; 729 bestanden, 68 regulär ignoriert | `.core-test-logs/acceptance-84d7ab2/test.log` |
| Release | `cargo build --workspace --release --locked` | Exit 0; `CARGO_BUILD_JOBS=2` | `.core-test-logs/acceptance-84d7ab2/release.log` |
| Default-Pilot | `scripts/run_local_pilot.sh`, reale Dokumentkopie, isolierter Postgres, Loopback-Provider; keine Budget-, Limit- oder Load-Overrides | Exit 0; 15/15 bei 12.000 Inputbudget und Retrieval-Limit 6; alle vier Phasen bestanden | `.core-test-logs/acceptance-84d7ab2/pilot/` |
| Lokale HTTP-Last | Zwei Läufe mit `BRAIN_PILOT_LOAD_REQUESTS=600 BRAIN_PILOT_LOAD_WORKERS=8`, unveränderte Budgets/Limits, Runtime-Commit `600ddd7` | Lauf 1: 600 `answered`; Lauf 2: 596 `answered`, 4 `unavailable`; in beiden Läufen 0 `unauthorized_evidence` | `.core-test-logs/{pilot-default,pilot-commit-600ddd7}/after_restart_default.json` |
| Echter Reader-Timeout | `pilot_phase_reader_failures`; exklusive Sperre auf Scratch-Head-Tabelle | PostgreSQL-Lock-Timeout wird `Unavailable`; bestanden | `reader_failures.json` im Pilotbericht |
| DB nicht verfügbar | Lokaler Reader mit eindeutig nicht vorhandenem Unix-Socket | `PortError::Unavailable`; bestanden | `brain-kernel/tests/evidence_errors.rs` |
| Größerer Korpus | 4.097 Memory-Dokumente, 8 Threads × 10 Anfragen, je 2 zusätzliche Validierungen | Genau 1 Snapshotaufbau; 240 angefragte Head-Schlüssel insgesamt, größter Batch 1 | `chunked_retrieval.rs`, Workspace-Testlog |
| Technische Fehler unter Last | 600 parallele Kernel-Anfragen, temporärer Readerfehler bei jeder 37. Head-Abfrage | Nur `answered` oder `unavailable`, keine falsche Permission-Denial-Antwort | `evidence_errors.rs`, Workspace-Testlog |
| Sicherheit und Determinismus | Chunkbytes/Provenienz, Zahlen, Aliase, Patch/Mode, Revoke/Delete, Egress-Sperre, Cache und Single-Flight, gewichtete Fusion | Bestanden | Neue Retrieval-/Kernel-Regressionstests |

Der Pilotprüfer wurde **verschärft**, nicht durch ein höheres Budget oder schwächere Erwartungen grün gemacht: Er prüft die tatsächlich an den Provider gesendeten Chunktexte gegen die ursprünglichen Dateien. Im Zahlenfall müssen sowohl Schlüssel als auch exakter Wert im tatsächlich zitierten Ausschnitt vorkommen; ein Vorkommen irgendwo im Eltern-Dossier genügt nicht mehr. Nicht auf eine Quelle abbildbare Chunks lassen den Fall fehlschlagen. Mehrdeutige Matches berücksichtigen alle passenden Quelldateien, statt eine private Quelle hinter einer öffentlichen Zuordnung zu verstecken.

Die vier expliziten Pilotphasen sind Import, Neustart nach erzwungenem Scratch-Postgres-Abbruch mit Defaultfällen und optionaler Last, Reader-Timeout und leerer Rebuild. Diese gezielt ausgeführten, sonst ignorierten Tests sind von den 68 im normalen Workspace-Lauf ignorierten Tests getrennt zu betrachten.

### Abweichungen und Wiederholung

Im zweiten Lastlauf dokumentiert das Scratch-Postgres-Protokoll vier `sorry, too many clients already`-Fehler; die API meldete vier `unavailable`, nicht `unauthorized_evidence`. Eine zunächst zusätzlich eingeführte Assertion auf 600 erfolgreiche Antworten ließ diesen Lauf trotzdem scheitern. Diese nicht beauftragte Verfügbarkeitsvorgabe wurde im Test-Commit `84d7ab2` entfernt: Verboten bleiben falsche Berechtigungsfehler, sämtliche Requests müssen in der Statusverteilung enthalten sein, und alle 15 funktionalen Pilotfälle müssen unverändert bestehen. Die Statusverteilung wird weiterhin vollständig ausgewiesen. Im selben Zwischenlauf scheiterte außerdem der Rebuild-Verbindungsaufbau an `PoolTimedOut`; dessen Ursache wird hier nicht als abschließend geklärt behauptet. Die fehlgeschlagenen Zwischenartefakte bleiben erhalten.

Der abschließende Lauf auf `84d7ab2` wurde ohne optionale Last und ohne Budget-/Limit-Overrides vollständig wiederholt: alle vier Pflichtbefehle und alle vier Pilotphasen erfolgreich. Import, Crash-Recovery und leerer Rebuild besitzen denselben Snapshot-Digest `0ee049564e973330b9423caa0a4a3c3bf8d8e92555af244d32b9ae26c0a251d3`. Zusammengefasster, aus den Logs überprüfter Nachweis: `.core-test-logs/acceptance-84d7ab2/verified-summary.json`. Keine Behauptung, dass technische Fehler unter beliebiger Last verschwinden.

## Folgen und Kompatibilität

Es gibt keine Änderung an kanonischen Dokumentdaten, SQL-Migrationsnummern, Produktionskonfiguration, Secrets oder laufenden Bots. Der Index ist flüchtig und wird nach Neustart beziehungsweise nach Cacheverdrängung einmal aus dem unveränderlichen Release aufgebaut. Historische Daten werden durch Delete nicht physisch gelöscht; die aktuelle Sperre verhindert ihre erneute Evidenzweitergabe. Keine Behauptung einer physischen Lösch-/Erasure-Funktion.

Bestehende Evidenz ohne Provenienz bleibt deserialisierbar; `None` wird nicht zusätzlich serialisiert. Öffentliche Antworten enthalten weiterhin keine internen Quellenpfade, ACLs oder Provenienzobjekte. Strikt typisierte externe Clients müssen den zusätzlichen Status `unavailable` kennen; andere Implementierungen von `SnapshotReadPort` benötigen `read_heads`, wenn sie mit dem neuen Release-Retriever verwendet werden. Ohne Implementierung bleiben sie fail-closed. C1 muss den aktualisierten nativen Reader verwenden; C4/Consumer-Owner müssen den additiven Status bei ihrer Vertragsintegration berücksichtigen. Diese Übergabe implementiert keine C1-Composition-Root- oder C4-Consumerarbeit.

Ressourcengrenzen des neuen Indexes sind 256 MiB Rohdokumenttext und 500.000 Chunks je Index; zusätzlich gilt das bestehende Release-Pin-Limit. Diese Grenzen sind Schutzgrenzen, keine SLOs oder Kapazitätszusagen. Es bleibt ein synchroner lokaler Reader mit bestehenden Verbindungs-/Statement-/Locktimeouts. Diese Arbeit führt keinen neuen Produktionspool ein. Die Runtime-Implementierung ist Rust; Bash/Python im vorhandenen Pilotgerüst sind ausschließlich Testorchestrierung.

## Grenzen und Blocker

Keine Produktionsfreigabe und keine Aussage über Produktionslatenz, Durchsatz oder Antwortqualität eines echten Sprachmodells. Der reale Pilot verwendet sieben vorhandene Dokumente, echtes lokales Postgres und HTTP, aber einen kontrollierten Antwortstub. Der 4.097-Dokument-Test ist ein Memory-/Port-Nachweis für Snapshot- und Head-Aufrufzahlen, kein großer PostgreSQL-Performancebenchmark. Die Tests zur Embedding-Fusion verwenden Fixtures, kein neues Remote-Embeddingmodell.

Die konservative Byte-/Framingreservierung muss vor einer Produktionsmodellfreigabe gegen dessen Tokenizer-/Transportvertrag überprüft werden. Ein unteilbares übergroßes Faktenobjekt kann weiterhin korrekt `budget_exceeded` ergeben; es wird nicht beschädigt, um ein Testbudget zu erfüllen. Die Antwort-/Chunk-Auswahl ist begrenzt; nicht jede Frage erhält automatisch sämtliche Nachbarchunks. Keine SLOs, keine neue Modellrevision, kein vollständiger Vektorstore-/ANN-Vergleich und keine eigenmächtige Freigabe nachfolgender Migrationswellen.

## Übergabe an nächsten Besitzer

Chat 00 / Integrationsreview: PR gegen `migration/rust-integration` unabhängig prüfen; besonders den additiven Fehlervertrag, gemeinsame Budgetberechnung, frische ACL-Abfragen und den verschärften Pilot kontrollieren. C1 übernimmt die bereits vorhandene `LocalPgReader`-Implementierung mit `read_heads`; andere Reader dürfen nicht auf Vollsnapshots zurückfallen. C4/Consumer-Owner berücksichtigen `unavailable`, ohne technische Readerfehler als Berechtigungsfehler oder unbelegte Antworten umzudeuten.

Originale Pilotdokumente, rohe Providerinputs und Secrets sind nicht Bestandteil des PR. Die freigegebenen Pilotquellen wurden in ein eigenes lokales Verzeichnis außerhalb von Git kopiert; Revoke-/Delete-/Crash-Proben betreffen nur den isolierten Scratch-Stack. Änderungen fremder Worktrees bleiben unberührt.

## Integration durch Chat 00

Merge-Commit: keiner. Gate-/STATUS-Änderung: keine. Freigabe neuer Arbeitswellen: keine. Weder Merge noch Deployment, Produktionsneustart oder Produktionsmigration durchgeführt.
