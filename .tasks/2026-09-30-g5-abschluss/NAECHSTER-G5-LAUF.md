status: vorbereitet, B2e878530 nachgeprüft; enger R1-Testrest vor vollständiger Quellabnahme, kein Lauf zugeteilt
Datum: 2026-09-30

# Nächster begrenzter G5-Lauf nach B2-Fixabnahme

Adressat: Integrator/Root über die bestehende zentrale BRAIN-G5-BUILD-REQUEST.txt, keine Session-zu-Session-Nachricht. Twitch ist laut Nutzer13:44:20UTC mit7/7 Quellen/Engine gesund, alle vier ELFs a82, Opsa685/main und Migration155. Reihenfolge des Integrators bleibt STT, Chat/Titel, Clip-Social/Context; keine eigenmächtige Konkurrenz durch Brain.

## Quellbindung und tatsächlicher Stand

Letzter gepushter vollständiger Quellstand392267b133a9a5a8a91602247a441ff7bf609c4b enthält gemeinsamen B2-Fix e878530 und getrennten CI-Fix0c56f85 sowie deren Berichte. Nachreview eceb14c2d7b9bf0dc6eae0d37da3f5c7aa94d1da seit14:42 abgeschlossen, gelesen und gepusht: CI GO, R2/R3 geschlossen, R1-Produktkorrektur statisch akzeptiert. Vollständiger R1-Abschluss noch BLOCK, weil beide Writer in den Testquellen vor Publish beendet werden. Nur diesen deterministischen Konkurrenznachweis zieht derselbe Autor mit Dispatch1172680 nach. Kein Neuaufbau akzeptierter Produktteile.

Vor Zuteilung endgültigen engen Fixhead und unabhängige Nachabnahme binden. Der Reviewer nennt als engste Alternative einen Paket-Check. Hier bleibt zur Vermeidung doppelter Compilerarbeit das vorbereitete Paket-Clippy der erste beantragte Aufruf; keinen zusätzlichen Check davor automatisch starten. Beide sind compiler-only, Tests und PG-/Lastläufe bleiben getrennt. Die drei CI-Aufrufe sind abgenommen und erzeugen keinen eigenen zusätzlichen Cargo-Bedarf. Jüngster Nutzerstand: Chat/Titel63e3 GateALLOW, neuer Reader-Probe-Build, Clip-ID-Fix in letzten Checks. Kein Brain-Slot aus dieser Meldung ableiten.

## Angefragte erste Klasse: Compiler, danach getrennte Unitläufe

Arbeitsverzeichnis für alle Aufrufe: /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust. Ausschließlich vorhandene Toolchain1.97.1, Cargo-Home /home/nathanael/.cargo und Targetcache /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target. Kein Fetch, kein Releasebuild, kein neuer Cache, kein Betreiber-ENV-Setup. Kapazität vor tatsächlicher Ausführung erneut prüfen, keine feste RAMzusage. Ein Cargojob, mögliche interne Compiler-/Linkerthreads. Kein Start/Restart von Produktdiensten.

1. Zunächst gezieltes Clippy der zwei tatsächlich angefassten Produktcrates; ersetzt einen doppelten vorausgehenden Check für diesen Slot:

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 clippy --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-storage -p brain-legacy-import --all-targets --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- -D warnings
```

2. Nach grünem Compilerbeleg getrennt nichtignorierte Librarytests:

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-storage -p brain-legacy-import --lib --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- --test-threads=1
```

3. Getrennt nichtignorierte Importer-Binärtests:

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.toml -p brain-legacy-import --bin brain-legacy-import --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- --test-threads=1
```

Keine automatische Verkettung, kein --ignored. Vollständige lokale Logs, tatsächliche Exits und SHA pro Lauf erhalten, Slot nach zugeteiltem Umfang sofort zurückgeben. Die endgültigen Testwirkungen müssen am Fixhead statisch geprüft sein; neue nichtignorierte DB-/Prozessfälle dürfen nicht still in diese Klasse geraten. Historische954/74 gelten nicht für den Fix.

## Zweite Klasse erst nach fertigem Fixtureanschluss

Vorhandener Serve-Harness, kein neuer Runner:

```sh
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_serve.sh /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

R3 ist aufe878530 statisch abgenommen: separate brain_cutover_test-Datenbank, ignorierter Binärtest tests::same_database_archive_to_core_requires_bound_private_snapshot exakt vor Serve-E2E, abschließende Prüfung der unverändert leeren brain_schema_test-Fixture, bestehendes Clustercleanup. Sieben serielle Cargoaufrufe. Der ganze Runner enthält echte private PostgreSQL-Schreibvorgänge, lokale Importer-/Serve-/Provider-/SCRAM-Fixtures und 1.800 Lastanfragen, je600 bei8/16/32 Workern. Port55439, max_connections12 und shared_buffers16MB unverändert. Der Cutoverfall verwendet eine synthetische privilegierte Testrolle mit require_auth=false, ersetzt also keine getrennten produktiven Minimalrollen-/Secret-Exec-Beweise. Vor Zuteilung zusätzlich den neuen R1-Synchronisationsfix samt tatsächlicher Testwirkung abnehmen; keine Budgeterhöhung, kein Produktzugriff und kein freistehender Zusatzaufruf gegen eine zufällig vorhandene Fixture.

## Sachliche Datenbelege, getrennt von Laufzuteilung

ARCHIV-METADATEN-1347.md belegt vier erfolgreiche Read-only-Prüfungen mit Counts/Hashes und ROLLBACK: Zielbrain leer, OIDs und Schemahash,905 Entities/3778 Aliase/32821 Events und Enrichments/348 Patch-IDs, bestehende Leserrechte; alle348 aktuellen Pilotpatchheads privat brain.legacy.review. Keine Rohinhalte ausgegeben. Fehlend bleiben echter Importer-Snapshotfingerprint samt Label/Epoch und gültige vollständige Rechte-/Widerrufs-/Tombstoneinventare. Keine Platzhalterbefüllung oder Importfreigabe aus Auditwerten. Read-only-Fortsetzung ist bereits autorisiert und benötigt keine pauschale Einzelrückfrage.
