status: aktiv, Quellabgabe ohne Laufzeitfreigabe
Datum: 2026-09-30

# B2: gebundener Archivimport in den bestehenden Corestore

Ausgang war der gepushte B1-Head `35673e7959290431ca60641195ee1378c09194eb`. Der getrennte B2-Quellcommit auf `fix/g5-replay-deferred-20260930` ist `c5d2b1f4f18eb8fd360641b3e66fa7453cafb4d2`. Dieser Bericht erhält einen eigenen Commit. Weder die Pilotconfig noch Lockfile, PostgreSQL-Grants, Store oder fremde Produktcrates wurden geändert. B2 ist keine Freigabe für einen Produktlauf, Merge oder Deploy.

## Pfad und Bindung

`rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs:60-101` lässt den bisherigen Pilotpfad mit Ziel `brain_pilot*`, anderer Legacy-Datenbank und ohne Produktionsbindung weiter zu. Für das Ziel `brain` sind beide Datenbanken `brain`, der Unix-Socket `/run/deadlock-brain-postgresql`, Port 5446, Quelle `brain_readonly`, Ziel `brain_ingest` und die getrennten bisherigen Secretreferenzen `BRAIN_LEGACY_READ_AUTH` und `BRAIN_TARGET_INGEST_AUTH` fest. `new_without_pgpass` bleibt erhalten; weder Passwort noch Wert einer Secretreferenz steht in dieser Config oder diesem Bericht. `Identity::verify` prüft nach Verbindungsaufbau Datenbank, Rolle, Unix-Transport und die genehmigten OIDs der Datenbank sowie der beiden unterschiedlichen Schemas `brain_legacy` und `brain` (`bin/brain-legacy-import.rs:102-170`). Quelle und Ziel dürfen im Produktionspfad dieselbe Datenbank, aber nicht dasselbe logische Schema sein. Archivqueries bleiben die festen `brain_legacy`-Queries in `src/pg.rs`; Schreibzugriffe laufen weiter durch `PgStore` in `brain`.

`ops/brain-postgres/legacy-core-cutover.json` ist die normale, ausdrücklich gesperrte Produktionsvorlage: leeres Snapshotlabel, Epoch 0, leere Releasefelder, `authorization_ref: null`, `production_binding: null` und Berichtspfad unter `/DO_NOT_RUN`. Der Produktionspfad benötigt einen absoluten Berichtspfad in einem bereits bestehenden Verzeichnis. Die bestehende Pilotconfig `legacy-core-import.json` bleibt historisch und wegen ihrer öffentlichen Patchnotes für B2 ungeeignet. Eine freigegebene private Kopie der Vorlage muss separat mit realen Werten erstellt und geprüft werden; hier wird kein Produktlauf bereitgestellt.

Die Pflichtbindung `production_binding` enthält `approval_ref` im Format `sha256:<64 hex>`, Datenbank-, Archivschema- und Coreschema-OIDs, `schema_sha256`, `snapshot_sha256`, `policy_sha256`, genaue Tabellenzeilenzahlen sowie die disjunkten Listen `active_ids`, `revoked_ids`, `tombstone_ids` je Quelle (`src/cutover.rs:9-23`). Die beiden Quellen sind exakt `legacy-entities` und `legacy-patchnotes`. Der Fingerprint bindet Snapshotlabel, Epoch, sichtbare Schemasignatur, Tabellenzahlen, gelesene Archivzeilen einschließlich Rohmetadaten und die daraus gebauten Dokumente. Der Policyfingerprint bindet Approval-Referenz, Policies und Klassifikation. Jede gelesene Archiv-ID muss klassifiziert sein; aktive IDs müssen tatsächlich gelesen worden sein. Vor einem Zielconnect prüft der Importer diese Bindung und verwirft widerrufene sowie tombstonierte IDs aus den neuen Dokumenten (`src/cutover.rs:39-162`, `bin/brain-legacy-import.rs:264-296`).

Patchnotes benötigen `private` und ausschließlich `brain.legacy.review`. Beide Quellen benötigen die gebundene Herkunftsreferenz, `provider_egress_allowed: false` und `publication_allowed: false`; Entity-Sichtbarkeit und Aufbewahrung müssen im freigegebenen Policyfingerprint stehen. Der Importer schreibt die Approval-Referenz in die vorhandene strukturierte Herkunft. Bereits vorhandene aktive Heads werden mit der vollständig neu erzeugten freigegebenen Dokumentfassung verglichen, einschließlich Inhalt, Metadaten, Herkunft und Rechten (`src/cutover.rs:164-209`). Ein aktiver Head zu widerrufener oder gelöschter ID blockiert.

## Commit, Abbruch und Rückweg

Die beiden Batches, Checkpoints, Releasepins und der Dokumentzähler werden vor dem ersten Claim vorbereitet. Vorher werden vorhandene Heads und Checkpoints auf Übereinstimmung geprüft. Nach den bestehenden Lease- und Commit-Transaktionen je Quelle werden Heads und Checkpoints vor `publish` erneut geprüft; erst danach wird der Release aus den gebundenen Checkpoints publiziert (`bin/brain-legacy-import.rs:317-409`). Ein Fehler vor `publish` erzeugt durch diesen Pfad keinen als vollständig veröffentlichten neuen Release. **Die zwei Quellcommits bilden keine gemeinsame Transaktion:** Ein Fehler nach dem ersten Commit kann einen unveröffentlichten Teilstand hinterlassen; erneutes Ausführen mit unveränderter Bindung ist der vorgesehene Wiederanlauf. Vor dem Freigabeslot sind Wiederanlauf, konkurrierende Writes sowie der Zeitraum zwischen der letzten Prüfung und `publish` gegen den echten Store zu prüfen. Kein automatischer Rollback ist eingebaut. Archiv und bestehende Releases werden nicht gelöscht; für einen Rückweg ist ein gesondert genehmigter Releasewechsel mit Prüfung der aktuellen ACLs, Widerrufe und Deletes erforderlich.

Eine genehmigte Tombstone-ID wird nicht neu importiert. Ist sie bereits im Zielcheckpoint aktiv, erzeugt `prepare_document_batch` beim Weglassen den bestehenden Tombstonepfad. Ist das Ziel für diese ID leer, verhindert das Weglassen nur die Wiederbelebung: Es materialisiert **keinen historischen Tombstonerecord** und beweist keine vollständige Archivparität. Die Herkunft und Gültigkeit historischer Deletes sowie die gewünschte Materialisierung bleiben Betreiberinput. Eine Klassifikation allein ist kein Beweis für einen externen aktuellen Widerrufsstatus.

## Noch fehlende Pflichtinputs und Gegenbeweise

1. Freigegebener Archiv-Snapshot mit echtem Label und Epoch, Schemafingerprint, Datenbank- und Schema-OIDs, Rohzeilenfingerprint und Tabellenzahlen. Die Werte müssen aus derselben freigegebenen Leseprobe stammen, nicht aus der Pilotconfig.
2. Nachgewiesene Berechtigungs- und Policyentscheidung je Quelle einschließlich der privaten Patchnotes, der Herkunftsreferenz, Scope, Aufbewahrung, Publikation, Egress und der vollständigen aktiven, widerrufenen und tombstonierten ID-Inventare. Eine externe Approval-Referenz muss tatsächlich auf diese Belege zeigen; die Config kann ihren Aussteller nicht selbst beglaubigen.
3. Privater, bereits existierender Berichtsort und geprüfte Freigabe für den Secret-Exec-Transport. Kein Passwort in Argumenten, JSON oder Logs. Produktivimport, DB-Schreiben, Rollenfixture, Service, Deploy und Last bleiben gesperrt.
4. Unabhängiges Source-Review, dann Compiler-/Unit-Slot und gesonderter Scratch-DB-Slot: falscher Socket, Port, Rolle, Datenbank, Schemas und gleiche logische Quelle/Ziel müssen vor dem ersten Schreibpfad scheitern; private Patchnotes, Widerruf, Tombstone, Abbruch ohne Teilrelease, Wiederholung und konkurrierende Writes sind zu beobachten. Ein späterer Produktlauf benötigt nochmals die echten Archiv- und Zielgegenbeweise sowie einen eigenen Cutoverentscheid.

## Genau benannte spätere Befehle, jetzt nicht ausführen

Nur nach Rückgabe des Compiler- und Scratchslots; bestehender Targetcache `/home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target`, Rust 1.97.1, offline und ein Cargo-Job. Diese Befehle sind keine Aussage über einen bereits erfolgten Test.

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 check --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import --all-targets --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import --lib --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import --bin brain-legacy-import --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Die neuen reinen Negativquellen liegen in `src/tests.rs` und in den Binärtests. Der positive gleichdatenbankige Gegenbeweis `same_database_archive_to_core_requires_bound_private_snapshot` ist **ignoriert** und benötigt die ausdrücklich isolierte, frische Datenbank `brain_schema_test` auf dem `.core-test-pg`-Socket, Port 55439, Rolle `brain_core_test`. Der bestehende Serve-Runner erzeugt diese Koordinaten in `scripts/test_brain_serve.sh:23-66`, ruft diesen neuen Test aber noch nicht auf. Der folgende exakte zusätzliche Aufruf gehört nur **innerhalb** eines getrennt zugeteilten Scratch-Harnessfensters hinter die `createdb brain_schema_test`-Zeile und vor den Clusterstop. Er darf nicht mit einem produktiven Socket oder als freistehender Befehl laufen; der gesamte Serve-Runner enthält außerdem einen gesondert freizugebenden 1800-Request-Lastfall.

```sh
env -i "PATH=$PATH" "HOME=$SCRATCH" "CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}" "RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}" "CARGO_BUILD_JOBS=1" "SQLX_OFFLINE=true" "BRAIN_CORE_TEST_PG_SOCKET=$CLUSTER" "$CARGO" +1.97.1 test --locked --offline --jobs 1 --target-dir "$TARGET_DIR" -p brain-legacy-import --bin brain-legacy-import tests::same_database_archive_to_core_requires_bound_private_snapshot -- --ignored --exact --nocapture
```

Der bestehende isolierte CLI-/Storage-Harness bleibt als weitere spätere Gegenprobe vorgesehen: `bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_serve.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target`. Er allein führt den neuen ignorierten Binärtest nicht aus und benötigt vorher eine separate Prozess- und Lastfreigabe.

B2-Quellprüfung: Rust 1.97.1 `rustfmt --check` der vier Importerdateien, JSON-Parse der gesperrten Vorlage, `git diff --check`, `git diff --cached --check` und Prüfung der fünf gestagten B2-Quelldateien, jeweils Exit 0. Keine Cargo-Compiler, Tests, Harnesses, Datenbankverbindungen, Modelle, Produktprozesse, Dienste oder Deploys ausgeführt. Die vor B2 belegten 954 bestandenen und 74 ignorierten Tests sind kein Nachweis für diese Änderung.

BESTAND[BS-1]: ja | Fundort: rust/crates/brain-legacy-import/src/lib.rs:466 | Anknüpfung: bestehender DocumentSetSource, Store, Release und Pilotpfad
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 3 belegt | Senke: B2-IMPORT-ERGEBNIS.md
ORCHESTRIERUNG[OR-1]: Stufe groß | Schritt bau | Artefakt: .tasks/2026-09-30-g5-abschluss/B2-IMPORT-ERGEBNIS.md
