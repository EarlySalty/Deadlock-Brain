status: vorbereitet, Ressourcenbedarf zur Zuteilung; noch kein geprüfter gemeinsamer Fixhead
Datum: 2026-09-30

# Nächster begrenzter G5-Lauf nach B2-Fixabnahme

Adressat: Integrator/Root über die bestehende zentrale BRAIN-G5-BUILD-REQUEST.txt, keine Session-zu-Session-Nachricht. Twitch ist laut Nutzer13:44:20UTC mit7/7 Quellen/Engine gesund, alle vier ELFs a82, Opsa685/main und Migration155. Reihenfolge des Integrators bleibt STT, Chat/Titel, Clip-Social/Context; keine eigenmächtige Konkurrenz durch Brain.

## Quellbindung und tatsächlicher Stand

Letzter gepushter vollständiger Quellstand4ee56de1ef465b4b29633a543c3e2930b3003303: B2c5d2b1f und B1-R1a427be3 enthalten. Kein neuer gemeinsamer B2-Fixhead. Aktuelle uncommittierte Fixarbeit in brain-storage/src/lib.rs, memory_repository.rs, pg_jobs.rs und pg_release.rs. Deshalb diese Befehle jetzt ausdrücklich nicht ausführen. Vor Zuteilung tatsächlichen Fixhead und unabhängige Nachabnahme des Deltas binden; zusätzliche geänderte Produktcrates würden eine Anpassung erfordern.

B2-BLOCK854825b: Race zwischen Prüfung und publish, zu späte Releasefeldvalidierung, Scratchfixturekollision. Gemeinsame Korrektur im vorhandenen Sol1166046 aktiv. B1-Nachreview11cd23e: PflichtargumentfixGO, verbleibender historischer Cargo-Pfadrest an drei CI-Aufrufen. Dieser Rest betrifft die untenstehenden lokalen absoluten Cargoaufrufe nicht; kein GitHub-Actions-Merge-Gate.

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

Nur nach unabhängiger Abnahme von R3, der vollständigen Fixtureisolierung und den tatsächlichen R1-/R2-Gegenproben zuteilen. Enthält echte private PostgreSQL-Schreibvorgänge, mögliche getrennte Testrollen, lokale Importer-/Serve-/Providerfixtures und1800 Lastanfragen bei8/16/32 Workern. Bisher Port55439, max_connections12 und shared_buffers16MB; Fix darf Budgets nicht ungefragt erhöhen. Neue Rollen, Testselektoren, Zielnamen, Cleanup und Reihenfolge müssen aus fertiger Abgabe ergänzt werden, bevor dies als ausführbarer Gesamtvertrag gilt. Kein Zugriff auf produktive DB/Dienste, kein freistehender Zusatzaufruf gegen eine zufällig vorhandene Fixture.

## Sachliche Datenbelege, getrennt von Laufzuteilung

ARCHIV-METADATEN-1347.md belegt vier erfolgreiche Read-only-Prüfungen mit Counts/Hashes und ROLLBACK: Zielbrain leer, OIDs und Schemahash,905 Entities/3778 Aliase/32821 Events und Enrichments/348 Patch-IDs, bestehende Leserrechte; alle348 aktuellen Pilotpatchheads privat brain.legacy.review. Keine Rohinhalte ausgegeben. Fehlend bleiben echter Importer-Snapshotfingerprint samt Label/Epoch und gültige vollständige Rechte-/Widerrufs-/Tombstoneinventare. Keine Platzhalterbefüllung oder Importfreigabe aus Auditwerten. Read-only-Fortsetzung ist bereits autorisiert und benötigt keine pauschale Einzelrückfrage.
