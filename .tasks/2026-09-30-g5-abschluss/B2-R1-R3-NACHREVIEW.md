# B2-R1 bis R3: unabhängiges Nachreview

Datum: 2026-09-30. fertig: J. Fix: J (gezielter R1-Testquellenrest).

**BLOCK für den vollständigen R1-Abschluss.** Die drei Produkt-/Anschlusskorrekturen sind statisch nachvollziehbar: gemeinsame Schreibgrenze, fertige Releasevalidierung vor Claim und separate angeschlossene Cutover-Fixture. Der ausdrücklich beauftragte synchronisierte Konkurrenzgegenbeweis fehlt jedoch weiterhin. Die zwei hinzugefügten Abläufe sind seriell und prüfen die neue Sperrwirkung nicht. Kein neuer Produktdefekt wird daraus abgeleitet.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft

Der Fremddienstpfad ist der statisch gelesene PostgreSQL-Storevertrag. Kein Compiler, Test, DB-/Rollenfixture, Import, Produktprozess, Modell, Dienst oder Deploy wurde gestartet.

## Eingefrorene Bindung

- Quelle `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`: `4ee56de1ef465b4b29633a543c3e2930b3003303..e878530a476431ff844d22f70cd86780d3cc3de4`. Genau sechs Dateien, 456 Einfügungen und 73 Löschungen. Gegenstand sind fünf Rustdateien und der bestehende Serve-Runner, kein beweglicher Arbeitsbaum.
- Autorbericht und Korrektur des ursprünglichen Importberichts aus `8c31b09deea51ae00130a56a42efee7034b08d92` gelesen. Dieser Folgecommit verändert zwei Berichtsdateien. Der spätere CI-Dreistellenfix erhält das getrennte Urteil in `B1-R1-N1-NACHREVIEW.md`.
- Bestehende Berichtssenke `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`, sauberer Ausgangsstand nach `11cd23e`. Keine Produktdatei geändert.
- Graphify zuerst, anschließend eingefrorenen Diff, betroffene Funktionskörper und Testabläufe gelesen. `git diff --check 4ee56de e878530`: Exit 0. `bash -n` auf dem eingefrorenen Serve-Runner über stdin: Exit 0. Die vom Autor gemeldete Rustfmt-Prüfung wurde nicht unabhängig wiederholt. Keine Typ-/Compiler- oder Laufzeitbestätigung daraus abgeleitet.

Codezeilen unten beziehen sich auf `e878530`. `Importer` bezeichnet `rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs`.

## R1-Nachbefund, P2: Die Konkurrenztests starten die Veröffentlichung erst nach dem Writerabschluss

**Anker:** `Importer:921-940` und `:973-988`, insbesondere die Join-Awaits bei `:928` und `:976`.

**Tatsächlicher Ablauf:** Im Tombstonefall wird der Writer mit `tokio::spawn` gestartet und mit `competing_delete.await.unwrap()` vollständig beendet. Erst danach werden Cutover-Lease und `commit_batches_and_publish_checked` angefordert. Im Scopefall passiert dasselbe mit `competing_scope.await.unwrap()`. Es gibt keine überlappenden Writer-/Publikationstransaktionen, keine synchronisierte Unterbrechung nach Sperrerwerb oder Baselinevergleich und keinen nachgewiesenen wartenden direkten Apply-Aufruf.

**Konkrete Nachweislücke:** Beide Testabläufe würden auch ohne die neuen `lock_source`-Aufrufe dieselben Ablehnungen prüfen: Der abgeschlossene Tombstonebatch macht den alten Checkpoint-CAS ungültig; der abgeschlossene direkte Apply verändert den Head, den der spätere vollständige Vergleich abweist. Das ist eine statische Aussage über die Reihenfolge, kein ausgeführter Mutationstest. Ein Defekt, der Änderungen zwischen erfolgreichem Headvergleich und Publish wieder zulässt, würde durch diese Testquellen nicht entdeckt. Auch die Sperre bei einem unveränderten Batch ist damit nicht erprobt, obwohl beide vorbereiteten `records`-Listen leer sind.

**Zwillingssuche:** Beide neuen `tokio::spawn`-Stellen, deren Join-Awaits und die drei Aufrufstellen der geprüften Publishmethode im Importer sind gelesen. Die zwei negativen Storeaufrufe verwenden jeweils einen Batch; der normale Importpositivfall verwendet zwei Quellen, aber ohne konkurrierenden Writer. Im Fixdelta gibt es keinen ergänzenden synchronisierten Sperrtest. Die Autorformulierung „konkurrierend“ beschreibt hier einen Writer nach einer alten Baseline, nicht einen zeitlich konkurrierenden Vorgang während der geschützten Veröffentlichung.

**Enge Nacharbeit:** Die bestehenden zwei Gegenbeweise um eine deterministische Synchronisation ergänzen. Die Publikation muss nachweislich ihre Quellsperren halten beziehungsweise den geschützten Vergleichspunkt erreicht haben, während ein zweiter Vorgang einen Batch-Tombstone oder direkten Scope-Apply auf dieselbe Quelle versucht. Der Test muss die Blockade bis Commit/Rollback und den danach erhaltenen Widerrufs-/Tombstonezustand beobachten. Leerer Batch und Zwei-Quellenpfad gehören zu der bereits beauftragten R1-Grenze. Eine bloße Zeitverzögerung oder ein vollständig abgewarteter Writer ersetzt diese Synchronisation nicht. Die Gegenprobe muss auf fehlende Schreibsperren reagieren. Keine neue allgemeine Testsuite und keine weitere Produktarchitektur beauftragen.

Dies ist der konkrete Grund für den verbleibenden BLOCK, nicht das gegenwärtige Verbot einer Testausführung. Auch nach einem erfolgreichen Lauf der jetzigen Testquelle wäre diese spezifische Aussage nicht belegt.

## Statisch geschlossene Produkt- und Anschlussbefunde

1. **R1-Schreibgrenze:** `brain-storage/src/pg_jobs.rs:7-16` verwendet `pg_advisory_xact_lock` auf `core-source:<source_id>`. `commit_batch_tx` nimmt diese Sperre vor Job-/Checkpointarbeit, auch vor Replay und unabhängig von `records.len()` (`:24-63`). Direkter `PgStore::apply` läuft über `apply_connection` und nimmt dieselbe Quellsperre vor seiner Recordsperre (`brain-storage/src/lib.rs:164-181`). Der gebundene Mehrquellenpfad sammelt IDs im `BTreeSet` und nimmt die Quellsperren sortiert vor dem ersten Batch (`pg_release.rs:90-138`). Die verschachtelte Sperrnahme erfolgt in derselben Transaktion. Im geprüften Code keine verbliebene Umgehung durch diese beiden Store-Writerpfade gefunden.
2. **R1-Atomarität und Vergleich:** `expected_cutover_heads` ergänzt die vorgeprüfte Baseline um die vorbereiteten Records und prüft die erwarteten Checkpoints (`Importer:234-255`). `commit_batches_and_publish_checked` validiert Headmenge und Releasepins, schreibt beide Batches und vergleicht gespeicherte Checkpoints, komplette Heads und deren SQL-/JSON-Identitäten innerhalb derselben Transaktion (`pg_release.rs:99-186`). Publish, Release-Readback und Pinanzahlprüfung liegen vor `tx.commit()` (`:187-211`). Ein Fehler davor rollt die dortigen Batch-/Checkpoint-/Releaseänderungen zurück. Der nachträgliche `snapshot` im Importer ist nicht mehr die einzige Integritätsprüfung. Statisches GO für diese R1-Produktkorrektur; der Restbefund betrifft ihren gezielt geforderten Konkurrenznachweis.
3. **R2 vor erster Mutation:** `Importer:374-376` validiert den fertig abgeleiteten Release mit dem unveränderten Bestandsvalidator vor der ersten Claim-Schleife (`:396-403`). `validate_release` wurde öffentlich exportiert, nicht inhaltlich gelockert. Reine Testquelle `:606-644` deckt Präfix 495/496 ASCII-Bytes, 495/496 UTF-8-Bytes mit `é` sowie Version/Patch 512/513 ab. Der Scratchfall verlangt bei den negativen Werten unveränderte Zähler für Jobs, Heads, Revisionen, Checkpoints und Releases (`:823-840`), ausgehend vom leeren Ziel, und enthält danach einen gültigen Grenzimport. R2 statisch GO; Testquellen nicht ausgeführt.
4. **R3-Fixture und Anschluss:** `scripts/test_brain_serve.sh:60-63` erzeugt `brain_schema_test` und separat `brain_cutover_test`; `:75-79` ruft den bestehenden ignorierten Binärtest exakt vor dem Serve-E2E auf. Der Test schreibt in `brain_cutover_test` (`Importer:679-715`) und prüft abschließend lesend, dass die Schema99-Datenbank weiter ohne `brain`/`brain_legacy` ist (`:998-1014`). `brain-serve/tests/process_e2e.rs` ist unverändert. Das vorhandene Cluster-Cleanup umfasst beide Datenbanken. Filter, sieben serielle Cargoaufrufe, Toolchain/Cache/Ein-Job-/Offlineflags und bestehende Fehlerweitergabe bleiben nachvollziehbar. R3 statisch GO.
5. **Geltungsgrenzen:** Claims erfolgen weiterhin vor der atomaren Batch-/Publishtransaktion. Ein späterer Fehler kann Claim-/Leasezustand bis zum Ablauf der 60 Sekunden hinterlassen, auch wenn Records und Release zurückgerollt werden. Nach erfolgreichem Commit dürfen spätere Writer neue Heads setzen; die gebundene Veröffentlichung linearisiert am Commit und ist keine dauerhafte Writersperre. Rohe SQL-Schreiber außerhalb der beiden Storepfade sind nicht durch deren Advisory-Lockvertrag erfasst. Allgemeines `publish_release` und der Pilotpfad behalten ihre getrennten Verträge. Kein Schema-, Grant-, Rollen-, Dependency-, Lockfile- oder produktives Configdelta in dieser Fixrunde.

## Sachlich korrigierte Grenzen bleiben gültig

Der aktualisierte Autorbericht begrenzt den Snapshotfingerprint jetzt auf projizierte `LegacyRead`-Daten und daraus erzeugte Dokumente. Aktive widerrufene Zielheads blockieren vor Batchvorbereitung; im leeren Ziel materialisiert Weglassen keinen historischen Tombstone. Diese Korrekturen entsprechen der Quelle. Reale Approval-/OID-/Snapshot-/Policywerte und vollständige ID-Inventare werden hier weder erzeugt noch als vorhanden oder fehlend behauptet. Die bestehende Cutoverbeauftragung wird nicht erneut zur allgemeinen Genehmigungsfrage gemacht.

## Engster nächster Cargo-Bedarf

Nach konkreter Zuteilung durch den Integrator und Bindung des sauberen geprüften Quellstands genügt zunächst **ein Check der zwei betroffenen Pakete einschließlich Testtargets**. Kein Workspace-Neubau, kein Fetch, kein Test- oder DB-Start:

```bash
/home/nathanael/.cargo/bin/cargo +1.97.1 check --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import -p brain-storage --all-targets --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Vorhandenes Cargo-Home `/home/nathanael/.cargo` und vorhandene Toolchain/Targetcache vorausgesetzt. Der Bericht-/CI-Folgehead `392267b133a9a5a8a91602247a441ff7bf609c4b` enthält gegenüber `e878530` Berichte und den getrennt geprüften CI-Fix; er ändert diese Rust-/Runnerbindung nicht. Vor einem späteren Lauf tatsächlichen SHA und Sauberkeit binden, nicht einen beweglichen Pfad mit der geprüften Revision verwechseln. Der zentrale Slot bleibt beim Twitch-Integrator; der Befehl wurde nicht ausgeführt.

Erst weitere zugeteilte Schritte wären Clippy und reguläre Tests derselben beiden Pakete. Sie führen den ignorierten Scratchfall nicht automatisch aus. Der vorhandene vollständige Scratch-/Prozessaufruf lautet:

```bash
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_serve.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Tatsächliche Klasse: sieben serielle Cargoaufrufe; privater Unix-Socket-PG auf Port 55439, `max_connections=12`, `shared_buffers=16MB`; zusätzliche Cutover-Datenbank mit Migration, Archivfixtures, Batch-/Apply-/Releasewrites, dazu bisherige Importer-/Serveprozesse, lokale Provider-/SCRAM-Fixtures und **600 Lastanfragen je 8/16/32 Worker**. Weiterhin 1.800 Lastanfragen zusätzlich zu funktionalen Requests, kein leichter Smoke. Der neue Positivfall verwendet weiterhin dieselbe synthetische privilegierte Testrolle für Lesen/Schreiben und `require_auth=false`; produktive Rollen-/Secret-Exec-Beweise werden dadurch nicht ersetzt. Nichts davon ist mit diesem Bericht ausgeführt oder zugeteilt.

## Abschluss

R2 und R3 statisch geschlossen. R1-Produktkorrektur statisch nachvollziehbar; R1-Abschluss bleibt wegen der konkret seriellen statt synchronisierten Testabläufe offen. Nächste Aktion: dem bestehenden Autor diesen eng begrenzten Testquellenrest an `Importer:928,976` zurückgeben. Keine neue Vollinventur oder Quellrunde außerhalb dieses belegten Befunds.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: B2-R1-R3-NACHREVIEW.md
