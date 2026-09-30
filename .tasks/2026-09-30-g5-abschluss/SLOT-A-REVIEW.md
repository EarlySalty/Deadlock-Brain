status: statische Prüfung abgeschlossen, Slot-A-Vorbereitung GO
Datum: 2026-09-30

# Slot A: unabhängige statische Abnahme

**Fertig: J für die statische Slot-A-Vorbereitung. Fix nötig: N. Statisches Gesamturteil auf 0c290809: GO mit ausdrücklich offenem Lock-/Cargo-Nachweis in Slot B.**

Der im ursprünglichen Bericht bestätigte ENV-/j2-Abgabemangel SA-1 ist im ausdrücklich nachgereichten Berichtscommit korrigiert. Im unveränderten Produktdiff ist kein weiterer konkreter Quellfehler nachgewiesen. Der Manifest-/CI-Schnitt ist statisch nachvollziehbar; die Root-Lockbereinigung und Cargo-Auflösung sind noch nicht abgeschlossen. Dieses GO akzeptiert die zulässige statische Vorbereitung und ihren ehrlichen Übergabestand, nicht eine fertige Git-freie Cargo-Auflösung. **Compilerprüfung NICHT AUSGEFÜHRT. G5-Livebeweis NICHT ERBRACHT.** Keine G5-/Deployfreigabe.

## Exakte Bindung

- Basis: `1c362bca6d35e7fec10125b2b159e7513a299243`.
- Geprüfter Produktcommit: `86a0bd6e52e5704db2d50901c918db02b4b3eb64`.
- Geprüfter Gesamt-/Berichtshead: `0c290809bb259a06dfe3c5d0e8eac9c0008c6e7f`, ausdrücklich während der Prüfung nachgereicht. Erstbericht `f0ef730b9994f33757015721aa3fff67de1b955f` und dessen Korrektur ändern ausschließlich `.tasks/2026-09-30-g5-abschluss/SLOT-A-ERGEBNIS.md`; Produktcommit bleibt `86a0bd6`.
- Quelle read-only: `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`, Branch `fix/g5-replay-deferred-20260930`. Eigener Berichtsbaum: `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, bestehender Reviewbranch, Ausgangsstand `7eb844dd604935495d3fd42165620690f059c739`.
- Die direkte Slotzuteilung erlaubt die sofortige Prüfung der lokal vorhandenen Commits; die ältere Briefing-Kopfzeile zur erst gepushten Abgabe wurde dadurch überholt. Zu Beginn waren beide Arbeitsbäume sauber und der Quellbranch laut lokalem Tracking bereits synchron. Kein eigener Fetch oder Remote-Nachweis.

Der abschließend geprüfte Quellbranch steht sauber und laut lokalem Tracking origin-synchron auf `0c290809bb259a06dfe3c5d0e8eac9c0008c6e7f`. Nach ausdrücklicher Nachbeauftragung wurde zusätzlich exakt `git diff f0ef730 0c290809` gelesen: eine Berichtsdatei, keine Produktänderung. Die bereits abgeschlossene Produktgraphanalyse wurde nicht wiederholt.

## Vorgehen und Ressourcen

Zuerst Status und vollständigen Diff-Stat geprüft, danach Produktdiff und unabhängige Gegenprüfung, zuletzt den eingefrorenen Autorenbericht. Graphify vor der Bestandssuche: globale Abfrage zu Replay, Workspace und Prüfskript; Treffer überwiegend benachbart, keine Extraktion gestartet. Maßgeblich sind die eingefrorenen Git-Blobs, nicht der historische Code im eigenen Reviewbaum.

Acht Produkt-/Schnittdateien geändert, dazu ein Berichtsartefakt. Kein Cargo, Compiler, rustfmt, Clippy, Test, Benchmark, Releasebuild, Prozessharness, Modellserver, Dienststart oder Restart ausgeführt. Keine Unterthreads, Paketinstallation, neuen Caches, Secrets oder ENV-Konfiguration. Keine fremden Dateien geschrieben. Als Syntaxprüfung lief ausschließlich `bash -n` auf dem Decoder-Prüfskript, das bytegleich zum zugewiesenen Produktcommit war: Exit 0. `git diff --check 1c362bc f0ef730`: Exit 0. Das sind keine Compiler- oder Testnachweise.

## 1. Rootworkspace und unveränderte V1-Pfade

`rust/Cargo.toml:3-29` entfernt genau `crates/dbrain-replay` aus den Mitgliedern und schließt diesen Pfad ausdrücklich aus. Die übrigen 24 Wurzeln bleiben erhalten. Es handelt sich nicht um ein Verstecken durch `default-members`, optionale Features oder einen Aufruf mit `--exclude`.

Die einzige Rust-Quelländerung liegt in `rust/crates/dbrain-replay/src/lib.rs:40`: Der Parserfingerprint liest den eigenen Replay-Lockstand statt des Rootlocks. Typed `/v1/answer`, Auth, Infisical, Modelle, Budgets, DB-Rollen und Services haben keinen Diff. Keine verbliebene Testdatei oder Testerwartung wurde verändert. Historische G2/G3-/G5-Belege wurden nicht umgeschrieben.

## 2. Eigenständiger Replaybereich

- `rust/crates/dbrain-replay/Cargo.toml:3,15-18,26`: eigener Workspace; Version `0.1.0` und die bisher geerbten Anforderungen für serde einschließlich `derive`, serde_json, sha2 und tempfile stimmen exakt mit dem bisherigen Rootmanifest überein.
- Die beiden Path-Dependencies auf `brain-contracts` und `deadlock-brain-core` bleiben erhalten. Replayquellen, Worker und Tests wurden nicht entfernt. Der Bereich bleibt Teil dieses Repositorys, keine behauptete eigenständig ausgelieferte Kopie seiner Path-Dependencies.
- Haste bleibt `bfb292d4798031350861ad297aa26753267a1ea6`, valveprotos `4f4a3cb1b0c6f19af59a722acb79ecccd01f61f6`, Dungers transitiv `5e1e2aac76a027987911de3ef3d23ecfd992a7fb`. Keine neue Version, Quelle oder Checksumme.
- `check-decoder.sh:6-14` verwendet das separate Manifest und behält Debug-, Release- und gesperrte Real-Korpus-Prüfung. Die README benennt den eigenen Targetpfad und macht die erforderliche spätere Lockauflösung sowie den fehlenden neuen Build-/Replaynachweis sichtbar. Der frühere ENV-/j2-Skriptbestand ist kein freigegebener lokaler Slot-B-Aufruf.

Der kopierte Replay-Lock ist ein Ausgangsstand, keine abgeschlossene eigenständige Cargo-Auflösung. Quellenzugang, Lizenz und echter Replaykorpus werden dadurch nicht nachgewiesen. Replay bleibt gemäß Nutzerentscheidung außerhalb von V1.

## 3. Unabhängige statische Lockgraph-Gegenprüfung

Read-only Auswertung der Git-Blobs mit Node, ohne Dateischreibvorgang oder Cargo. Die Auswertung zerlegt sämtliche Paketblöcke, liest Abhängigkeiten als vollständige Paketidentitäten und verlangt pro Kante genau einen Treffer. Versionen und gegebenenfalls Quellenqualifikation werden berücksichtigt; Namen werden nicht als alleinige Identität verwendet. Nicht verstandene Restfelder oder mehrdeutige Kanten hätten die Auswertung abgebrochen.

| Eigenschaft | Eigenes Ergebnis |
| --- | ---: |
| Paketknoten im bisherigen und aktuellen Rootlock | 427 |
| Rootmitglieder vorher / nachher | 25 / 24 |
| Entfernte Rootwurzel | ausschließlich dbrain-replay |
| Aufgelöste Paketkanten | 1289 |
| Versionsqualifizierte Kanten | 222 |
| Namen mit mehreren Paketversionen | 37 |
| Quellenqualifizierte Dependencystrings in dieser Datei | 0 |
| Von den 24 V1-Wurzeln erreichbare Pakete | 386 |
| Davon Gitquellen | 0 |
| Außerhalb dieser V1-Menge verbliebene Lockeinträge | 41 |
| Von Replay erreichbare Pakete | 325 |
| Im kopierten Replaylock darüber hinaus enthaltene Einträge | 102 |
| Unaufgelöste oder mehrdeutige Kanten | 0 |

Die Auswertung verfolgt sämtliche Lockkanten einschließlich der dort gespeicherten Test-/Build-Abhängigkeiten. Alle 24 bisherigen Nicht-Replay-Wurzeln sind erhalten. Das ist eine konservative statische Erreichbarkeitsanalyse des bestehenden Graphen, keine Cargo-Feature-/Targetauflösung.

**Wichtige Trennung:** Null Gitquellen im erreichbaren V1-Teilgraphen heißt nicht, dass die committed Rootlockdatei Git-frei ist. `rust/Cargo.lock` ist bytegleich zur Basis. Sie enthält den alten lokalen `dbrain-replay`-Paketeintrag (`:793`) sowie sieben Gitpakete aus drei Quellen, darunter Dungers (`:1022`), Haste (`:1382`) und valveprotos (`:3746`). `rust/crates/dbrain-replay/Cargo.lock` ist eine bytegleiche Kopie dieser gesamten Datei, nicht auf seine 325 erreichbaren Pakete bereinigt.

SHA256 aller drei verglichenen Lockstände, Basis-Root, Produkt-Root und neue Replaykopie:

```text
50a20d14dc5af6584a62149bffd419a9691afe3ecf426b464fab020723045a1e
```

Der Autorenbericht benennt die zwei durch Worktree-Isolation abgewiesenen Schreibversuche und die ausstehende Cargo-Auflösung ehrlich. Eigener Gegenbeleg ist die tatsächlich unveränderte Lockdatei; die damaligen Schutzereignisse wurden nicht selbst erneut ausgeführt. Keine Umgehung und kein Cargo-/Buildbeweis daraus abgeleitet.

`SLOT-A-SOL.md:38` gestattet ausdrücklich, einen unsicheren alten Lockstand zu erhalten und den konkreten Slot-B-Bedarf zu melden. Deshalb wird die transparente Verschiebung nicht als erfundener zweiter Produktfehler behandelt. Eine **abgeschlossene Git-freie committed V1-Lockauflösung** oder ein funktionierender `--locked`-Lauf kann auf diesem Stand dennoch nicht abgenommen werden. Die CI enthält weiterhin `cargo fetch --locked`; ihr erfolgreicher Lauf ist nicht bewiesen. Kein beobachteter Cargo-Fehler wird behauptet, weil Cargo nicht lief.

## 4. CI, Skripte und Artefaktverbraucher

74 versionierte Shell-, Workflow- und Manifestdateien außerhalb der Taskakte statisch nach Replayaufrufen und den betroffenen Git-/Protobuf-Abhängigkeiten geprüft. Im untersuchten ausführbaren Bestand ruft kein verbleibender V1-Workflow den Decoderprüfer oder ein Replaypaket über das Rootmanifest auf.

- `.github/workflows/ci.yml`: automatischen Decoderaufruf und dessen Protobuf-Installation entfernt; V1-Workspacecheck und Sources/Core-Regression bleiben unverändert.
- `.github/workflows/rust-core-verification.yml:155-164`: Reference-Matrix enthält Wiki und Sources statt zusätzlich Replay. Sechs Core- und zwei Tooling-Suiten bleiben erhalten.
- `rust-core-verification.yml:218-259`: Aggregator verlangt weiterhin erfolgreiche Matrizen und genau einen nicht leeren, nicht abgelaufenen Beleg pro SHA/Versuch. Erwartungsmenge konsistent zehn statt elf Artefakte, ohne `integrated-replay`. Die Zusammenfassung sagt ausdrücklich, Replay sei zurückgestellt und hier nicht geprüft.
- `architecture/migration/replays/s14/check-decoder.sh` bleibt separat ansprechbar; keine Abschwächung des blockierten realen Korpusnachweises. Ein noch vorhandener `protoc --version`-Provenienzeintrag im unveränderten `brain-serve-c1.yml` ist weder ein Decoderlauf noch ein fehlendes Replayartefakt.

Keine neuen Compiler- oder CI-Erfolgsaussagen. Die alten 1011 Workspace-Tests aus `022f8a9` werden nicht auf diesen geänderten V1-Scope übertragen. Die früheren echten Replay-/Lizenzlücken sind nicht geschlossen, sondern für V1 ausdrücklich zurückgestellt.

## 5. SA-1: im Erstbericht bestätigt, auf 0c290809 geschlossen

Der Erstbericht `f0ef730` enthielt in `.tasks/2026-09-30-g5-abschluss/SLOT-A-ERGEBNIS.md:36-39` einen Metadata-Aufruf ohne `--offline`, ENV-Präfixe für `CARGO_TARGET_DIR`/`SQLX_OFFLINE` und `-j2`. Zeile 50 enthielt zusätzlich `CARGO_TARGET_DIR`/`RUSTUP_TOOLCHAIN` vor dem späteren Decoderprüfer. Das widersprach den aktuellen Slotvorgaben. Ein verbotener tatsächlicher Compilerlauf wurde daraus nicht abgeleitet.

Der ausdrücklich nachgereichte Diff `f0ef730..0c290809` korrigiert diesen Abgabemangel:

- Erste Metadata-Auflösung offline ohne `--locked`, danach zusätzliche Prüfung mit `--locked --offline`.
- Vorgeschlagene Checks mit `--locked --offline`, explizitem vorhandenen `--target-dir` und `--jobs 1`, ohne ENV-Präfixe.
- SQLx-Konfiguration ausdrücklich ungeklärt; keine neue Konfiguration vorausgesetzt. Befehle als nicht ausgeführte Vorschläge bezeichnet, Slotzuteilung weiterhin erforderlich.
- Der spätere Decoderprüfer wird nicht mehr als unmittelbar freigegebener Aufruf vorgegeben. Seine eigene Zuteilung und Prüfung bleiben offen; der vorgeschlagene separate Replaycheck bleibt ebenfalls ungetestet.

Der Nachtrag beschreibt außerdem den Bindungsfehler konkret: tatsächliche cwd und Zielpfade im richtigen Quellworktree, dazu die Guardmeldung „this command is too complex to verify that it stays inside the worktree“. Das ist ein abgewiesener Schreibversuch, kein Beleg fehlender Commits. Eigene Gegenprüfung bestätigt, dass die benannten Commits vorhanden und die Lockdateien unverändert sind. Die Meldung bleibt Autorenbericht; sie wurde nicht durch einen erneuten verbotenen Schreibversuch reproduziert.

**SA-1 geschlossen. Keine weitere Berichtskorrektur für Slot A erforderlich.** Die Vorschläge im Autorenbericht ersetzen nicht die vom Integrator zuzuteilenden exakten Befehle für Slots B bis E. Das Verbot von Compiler-, Prozess- und Dienstläufen in Slot A bleibt unverändert.

## Noch benötigte Schritte, keine Ausführungsfreigabe

Vorhandene Caches: Cargo `/home/nathanael/.cargo`; exklusiv zuzuteilender Targetcache `/home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target`. In Slot A wurde keiner beschrieben oder neu angelegt.

**Slot B, erst nach Zuteilung**, Arbeitsverzeichnis `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust`, gemäß aktueller Buildanfrage:

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 metadata --offline --format-version 1
/home/nathanael/.cargo/bin/cargo +1.97.1 metadata --locked --offline --format-version 1
/home/nathanael/.cargo/bin/cargo +1.97.1 fetch --locked
/home/nathanael/.cargo/bin/cargo +1.97.1 fmt --all -- --check
/home/nathanael/.cargo/bin/cargo +1.97.1 clippy --workspace --all-targets --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- -D warnings
/home/nathanael/.cargo/bin/cargo +1.97.1 test --workspace --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Vor Compilerbeginn SQLx-Konfiguration und Testthreadgrenze im erlaubten vorhandenen Konfigurationsweg klären. Cargo-Metadaten und Lockdiff müssen die tatsächliche V1-Auflösung ohne Replay-/Gitpakete zeigen, ohne Versions-, Quellen- oder Checksummentausch. Fetch im vorhandenen Cache ist kein Cold-Cache-Nachweis. Fehlender Cacheinhalt ist ein konkreter Slot-B-Blocker, keine Erlaubnis zum Anlegen eines neuen Caches.

**Slot C**, separat zuzuteilen, genau ein Releasebuild aus demselben Verzeichnis:

```sh
flock -w 600 /tmp/deadlock-brain-release-build.lock /home/nathanael/.cargo/bin/cargo +1.97.1 build --workspace --release --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

**Slot D:** Noch kein exakter Harnessaufruf freigegeben. Vorhandene PG-/Serve-/Wiki-Pfade zuerst auf Cache-Reuse, ENV-Konfiguration und versteckte Compilerstarts prüfen. Kein Lauf oder Lastbenchmark in Slot A.

**Slot E:** Noch kein startbereiter Dienstbefehl. Tatsächliche Unit, Config, Endpoint, revisionsgebundenes Artefakt und Rückweg müssen belegt und vom Integrator einem Fenster zugeordnet werden. Kein neuer Modellpfad und kein eigener Twitch-/DL-Neustart.

## Schlussurteil

Der Replay-Manifest-/CI-Schnitt ist statisch nachvollziehbar; vorhandener Replaybestand und V1-Wurzeln bleiben erhalten. Die abschließend zugewiesene Abgabe **Produkt 86a0bd6, Gesamtstand 0c290809** erhält **statisches GO für die Slot-A-Vorbereitung**. SA-1 ist geschlossen. Die noch unveränderte Rootlockdatei und der kopierte Replaylock bleiben konkrete offene Cargo-Arbeit im späteren Slot, keine bereits verifizierte Git-freie Lockauflösung.

**Fertig J für Slot A, Fix N. Compilerprüfung NICHT AUSGEFÜHRT. G5-Livebeweis NICHT ERBRACHT. Kein Merge, Deploy oder G5-Entscheid.**

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft (statische Abnahme, keine echten Fremdaufrufe; SA-1 im Berichtsdelta geschlossen)
TESTNACHWEIS[TW-1]: 0 passed, 0 ignored | Baseline: kein Testlauf angefordert oder ausgeführt; statische Graphanalyse und Shell-Syntaxprüfung sind keine Tests
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: unabhängiger Slot-A-Bericht
