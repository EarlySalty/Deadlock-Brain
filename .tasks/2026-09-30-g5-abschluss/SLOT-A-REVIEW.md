status: statische Prüfung einschließlich echtem Locknachtrag abgeschlossen, GO auf 9a29b81
Datum: 2026-09-30

# Slot A: unabhängige statische Abnahme mit Locknachtrag

**Fertig: J für die beauftragte statische Abnahme einschließlich Delta 0c290809..9a29b81. Fix nötig: N. Statisches Gesamturteil auf 9a29b81: GO.**

Die Root-Lockbereinigung ist inzwischen committed und unabhängig statisch bestätigt: 386 Pakete, 41 entfernt, keine Gitquellen, keine neuen Paketidentitäten oder geänderten Versionen, Quellen und Checksummen. Die entfernte chrono-serde-Kante ergibt in den geprüften V1-Verwendungen keinen konkreten Quellfehler. Der einzelne Offline-Metadatenlauf stammt von der Hauptsession, nicht vom Reviewer. Der gesperrte Metadaten-Kontrolllauf und die Compilerprüfung bleiben offen. **Compilerprüfung NICHT AUSGEFÜHRT. G5-Livebeweis NICHT ERBRACHT.** Sonstige Slots B bis E bleiben gesperrt. Keine G5-/Deployfreigabe.

Die folgenden Abschnitte bis zum historischen Schlussurteil dokumentieren die ursprüngliche Slot-A-Abnahme auf `0c290809`. Aussagen zum damals unveränderten Rootlock gelten ausschließlich für diesen Stand. Der aktuelle Nachtrag auf `9a29b81` steht am Ende; die frühere Zahl 24 bezeichnet explizite Manifestwurzeln, nicht sämtliche Cargo-Member.

## Exakte Bindung der ursprünglichen Slot-A-Abnahme

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
| Explizite Manifestwurzeln vorher / nachher | 25 / 24 |
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

## Historische Slotplanung auf 0c290809, keine Ausführungsfreigabe

Aktualisierung auf `9a29b81`: Nur der erste unten aufgeführte Metadatenbefehl wurde anschließend einzeln zugeteilt und von der Hauptsession ausgeführt. Alle weiteren Befehle bleiben unzugeteilt und wurden vom Reviewer nicht gestartet.

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

## Historisches Schlussurteil auf 0c290809

Der Replay-Manifest-/CI-Schnitt ist statisch nachvollziehbar; vorhandener Replaybestand und V1-Wurzeln bleiben erhalten. Die abschließend zugewiesene Abgabe **Produkt 86a0bd6, Gesamtstand 0c290809** erhält **statisches GO für die Slot-A-Vorbereitung**. SA-1 ist geschlossen. Die noch unveränderte Rootlockdatei und der kopierte Replaylock bleiben konkrete offene Cargo-Arbeit im späteren Slot, keine bereits verifizierte Git-freie Lockauflösung.

**Fertig J für Slot A, Fix N. Compilerprüfung NICHT AUSGEFÜHRT. G5-Livebeweis NICHT ERBRACHT. Kein Merge, Deploy oder G5-Entscheid.**

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft (statische Abnahme, keine echten Fremdaufrufe; SA-1 im Berichtsdelta geschlossen)
TESTNACHWEIS[TW-1]: 0 passed, 0 ignored | Baseline: kein Testlauf angefordert oder ausgeführt; statische Graphanalyse und Shell-Syntaxprüfung sind keine Tests
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: unabhängiger Slot-A-Bericht

## Nachtrag: tatsächlicher Root-Lockdiff auf 9a29b81

### Auftrag und Nachweisbindung

Am 30.09.2026 ausdrücklich zugewiesen: ausschließlich statisches Delta `0c290809bb259a06dfe3c5d0e8eac9c0008c6e7f..9a29b81d230c01e5c03423cc34ba34c1074eab69` im bisherigen Quellworktree. Eigener Berichtsausgangspunkt: `fde282438433ebb58c526fda1c83480aab5c4656`. Keine erneute Vollprüfung des bereits abgenommenen Manifest-/CI-Schnitts.

Der eingefrorene Diff enthält genau `rust/Cargo.lock` und den neuen Beleg `.tasks/2026-09-30-g5-abschluss/SLOT-B-METADATA.md`. Rustquellen, Manifeste, CI, Replaylock und Parserpins haben in diesem Delta keine Änderung. `git diff --check` auf den beiden vollständigen Ziel-SHAs: Exit 0.

`SLOT-B-METADATA.md:18-29` dokumentiert den einzeln zugeteilten Aufruf der Hauptsession: `/home/nathanael/.cargo/bin/cargo +1.97.1 metadata --offline --format-version 1`, cwd `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust`, Exit 0, 562 ms, kein Fehlertext und kein `rust/target` vor oder nach dem Lauf. Der Beleg nennt 386 Pakete/Resolve-Nodes und 25 Cargo-Member. Das sind berichtete Laufdaten, keine eigene Wiederholung. Die Metadata-Rohdatei liegt nicht zur unabhängigen Wiederzählung vor; ihr dokumentierter SHA256 `f143b11277a256d4d66278318ff1e7d014b1521d3b01154ffa4aec5ffdd1cf71` ist daher ebenfalls übernommene Provenienz. Unabhängig nachgeprüft ist der committed Lockgraph.

### Eigener semantischer Lockvergleich

Beide eingefrorenen Lockblobs wurden vollständig wie in Abschnitt 3 zerlegt. Jede Dependencyreferenz wurde innerhalb ihres jeweiligen alten oder neuen Graphen auf Name, Version und Quelle aufgelöst. Dadurch werden entfernte Versionsqualifizierungen nicht mit geänderten Abhängigkeiten verwechselt. Keine mehrdeutige oder unaufgelöste Referenz, kein unverständlicher Blockrest.

| Eigenschaft | Eigenes Ergebnis |
| --- | ---: |
| Paketidentitäten vorher / nachher | 427 / 386 |
| Entfernte / neue Paketidentitäten | 41 / 0 |
| Geänderte Versions-, Quellen- oder Checksummentupel erhaltener Pakete | 0 |
| Kanten im vollständigen alten / neuen Lockgraphen | 1289 / 1166 |
| Erhaltene Pakete mit semantischer Kantenänderung | 1 |
| Gitpakete im neuen Rootlock | 0 |
| Von den 24 expliziten V1-Wurzeln erreichbar | 386 |
| Lokale Pakete im neuen Rootlock | 25 |

Die neue Paketmenge entspricht exakt den bereits auf `0c290809` statisch erreichbaren 386 V1-Paketen. Die 41 entfernten Einträge sind exakt die zuvor außerhalb dieser Menge verbliebenen Pakete, einschließlich `dbrain-replay`, der sieben Gitpakete und der Replay-spezifischen Protobuf-/Testabhängigkeiten. Alle 24 expliziten V1-Wurzeln sind erhalten. Die übrigen Änderungen an Dependencystrings sind reine Entqualifizierungen; die einzige semantische Änderung an einem erhaltenen Paket ist `chrono 0.4.45 -> serde 1.0.228`, entfernt in `rust/Cargo.lock:467-478`.

SHA256 des unabhängig gelesenen neuen Rootlocks:

```text
2f3dd52e2fee3be8ee03f99e9969e2b56629384b118730956f9637b0198cb291
```

Der separate Replaylock ist bytegleich zum bisherigen Stand. Seine ausstehende eigene Bereinigung und die alten Replay-Nachweisgrenzen bleiben bestehen; sie sind kein im neuen V1-Lock verbliebener Gitbedarf.

### 24 explizite Wurzeln, 25 Cargo-Member

`rust/Cargo.toml:3-29` enthält 24 explizite Memberpfade. `rust/crates/deadlock-brain-core/Cargo.toml:25` bindet zusätzlich `../../vendor/uplink-infisical-transport` ein. Dessen Manifest `rust/vendor/uplink-infisical-transport/Cargo.toml:1-11` liegt innerhalb des Rootworkspaces, hat keinen eigenen Workspace und wird nicht ausgeschlossen. Die automatische Aufnahme dieser lokalen Pfadabhängigkeit erklärt den gemeldeten 25. Cargo-Member. Derselbe Baustein ist im alten und neuen Lock enthalten. Kein neu hinzugefügtes Produktpaket und kein fehlender expliziter V1-Eintrag. Der Paketname des expliziten Pfades `crates/dbrain-wiki` lautet weiterhin `dbrain-s12-wiki-probe`.

### chrono ohne serde: statische Verwendungsprüfung

Graphify wurde vor der Suche nach chrono-/Serde-Verwendungen und dem lokalen Transportbaustein abgefragt. Anschließend wurden die eingefrorenen Rustmanifeste und Quellen nach chrono, DateTime und NaiveDate-/NaiveTime-Typen durchsucht und die gefundenen Datenpfade einschließlich Beispiele und Tests nachgelesen. Im geprüften V1-Code ist kein Bedarf für direkte Serde-Serialisierung oder -Deserialisierung eines chrono-Typs belegt:

- `dbrain-retrieval/src/game_wiki.rs:501-516`: `SnapshotRow` mit `Option<DateTime<Utc>>` hat nur Debug-/Clone-Derives. JSON erhält Strings über `to_rfc3339()` in `:117,527-533`; Markdown und Index formatieren ebenfalls ausdrücklich (`:854-857,967-1011`). `game_wiki_localization.rs:9-16,132-135` hält den Zeitpunkt ohne Serde-Derive und wandelt ihn vor `json!` um.
- `dbrain-retrieval/src/lib.rs:7321-7337`: SQLx dekodiert `DateTime`, `NaiveDateTime` und `NaiveDate`, danach entstehen ausdrücklich `JsonValue::String`-Werte. Das in `dbrain-retrieval/Cargo.toml:20` aktivierte SQLx-Feature `chrono` ist die DB-Typanbindung, keine Anforderung an chrono/serde. Die Beispiele `ask_latency.rs:140-156` und `ask_publish_smoke.rs:173` geben RFC3339-Strings aus.
- `deadlock-brain/src/pg_patchnotes.rs:90-104,144-155`: Die chrono-haltigen Strukturen `PreparedPatch` und `EventParseContext` besitzen keinen Serialize-/Deserialize-Derive. JSON erhält `posted_at_text` nach `to_rfc3339()` (`:528-566`) beziehungsweise eine ausdrücklich umgewandelte Option (`:1069`). Der DB-Schreibpfad verwendet ebenfalls einen String (`:1373-1386`), nicht chrono-Serde.
- `dbrain-wiki/src/pages.rs:238-245` und `deadlock-brain-core/src/http/bounded.rs:263-270` parsen RFC-Daten und entnehmen Integer-Zeitstempel. `deadlock-brain/src/pg_steam_news.rs:224-247` verwendet das Datum nur für Filterzeit und Vergleiche. `pg_insights.rs:191-201`, `main.rs:1829` und `wiki_refresh.rs:100,149-152` formatieren ausdrücklich Strings.

**Kein chrono-Feature-Fix aus diesem statischen Befund erforderlich.** Die entfernte Serde-Kante ist für diese Verwendungen nicht nötig. Daraus folgt kein Compilerbeweis; insbesondere wurde keine Traitauflösung durch rustc geprüft und keine erfolgreiche Workspaceprüfung behauptet.

### Aktuelles Urteil und verbleibende Grenze

**Fertig J, Fix N, statisches GO auf `9a29b81d230c01e5c03423cc34ba34c1074eab69`.** Die bisher offene committed Root-Lockbereinigung ist statisch geschlossen. SA-1 bleibt geschlossen. Keine neue Produkt- oder Berichtskorrektur verlangt.

Der Reviewer hat weder Cargo noch Compiler, Tests, Fetch, Prozessharness, Modellserver oder Dienste gestartet. Keine neuen Caches, ENV-Konfiguration, Secretlesung, Unterthreads, Produktänderung, Merge oder Deployment. Der einzelne berichtete Offline-Metadatenlauf aus vorhandenem Cargo-Cache belegt weder frischen Quellenzugang noch einen Build ohne vorhandenen Cache, erfolgreiche Tests oder Livebetrieb. **Compilerprüfung NICHT AUSGEFÜHRT. G5-Livebeweis NICHT ERBRACHT. Sonstige Slots B bis E weiter gesperrt.**

Nächster Freigabepunkt beim Integrator ist ausschließlich der bereits vorgeschlagene Kontrolllauf `cargo +1.97.1 metadata --locked --offline --format-version 1` mit Prüfung eines unveränderten Rootlocks. Dieser Bericht teilt ihn nicht zu; der Reviewer führt ihn nicht aus.

Werkzeuggrenze beim abschließenden Textcheck: `ctx_execute_file` verweigerte die eigene Berichtsdatei unter `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, da der Server noch `/home/nathanael/repos/Deadlock-Brain` als Projektwurzel bindet: „resolves outside the project root“. Kein Wiederholungsversuch über einen anderen Dateilesepfad, keine Regeländerung. Textkontrolle manuell; der reguläre `git diff --check` im zugewiesenen Reviewworktree bestand.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft (statischer Lock-/Feature-Nachtrag, keine echten Fremdaufrufe)
TESTNACHWEIS[TW-1]: 0 passed, 0 ignored | Baseline: kein Testlauf angefordert oder ausgeführt; kein Compiler- oder Livebeweis
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: statischer Locknachtrag im bestehenden Reviewbericht
