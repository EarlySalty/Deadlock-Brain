# C10: echte Replayvalidierung

Stand: 26. September 2026. Repository: `EarlySalty/Deadlock-Brain`.
Branch: `codex/fix-c10-real-replay-validation`.
Basis: frisch geholtes `origin/migration/rust-integration`, `087c522deda58ecf4bd6843167f51c54f681e944`.

## Entscheidung

**C10_REAL_REPLAY_ACCEPTED: NEIN. REAL_MATCHES_ANALYZED: 0.**

Es wurde keine echte Replaydatei bereitgestellt und kein konkreter Match ausdrücklich zur Analyse freigegeben. Im Gespräch gibt es keinen Dateianhang und keine Matchreferenz. Das Integrationsreview dokumentiert ebenfalls das Fehlen einer berechtigten Replaygrundlage. Die gezielte Bestandsprüfung im lokalen Brain-Datenverzeichnis fand keine bereitgestellte Datei bzw. zugehörige Freigabe. Es wurde nichts heruntergeladen, kein Match aus einem Dateinamen geraten und kein synthetischer Container als echter Match registriert.

Dieser PR bereitet einen reproduzierbaren lokalen C10-Prüflauf vor. Er schließt die fehlende Realmatch-Abnahme ausdrücklich **nicht**. Kein Deployment, kein Merge, keine produktiven Datenbankzugriffe, keine Dienste neu gestartet. `CORPUS.tsv`, Capability-Freigaben und übergreifende Migrationsgates bleiben unverändert.

## Bestandserhaltende Änderung

Das vorhandene Binary `dbrain-replay-worker` erhält den Unterbefehl:

```text
dbrain-replay-worker validate <local-raw-file> <authorized-request.json>
```

Der Prüfmodus verwendet ausschließlich den vorhandenen `WorkerDecoder`, dessen begrenzte lokale Momentaufnahme, dieselben Rust-Decoder-/Protobuf-/Snappy-Komponenten, den bestehenden `ReplayReport` und die vorhandene `classify_replay_duplicate`-Funktion. Jeder Decoderaufruf startet einen neuen isolierten Workerprozess. Es gibt keine zweite Replayarchitektur, keinen Downloader, keine zusätzliche Datenbank, keine neue Queue, keine Domainengine und keinen Provideraufruf.

Die neue Datei `src/validation.rs` ist ein Prüfbericht und eine feste Testsequenz, keine neue Observation-Zwischenrepräsentation. Der Decoder und seine Beobachtungssemantik bleiben unverändert. Parser-/Schemarevision stammen weiterhin aus dem existierenden Manifest; der Validator hat zusätzlich einen eigenen Quellfingerprint.

### Was der Prüfmodus tatsächlich belegt

1. Zwei frische vollständige Decodes derselben SHA-256-gepinnten Eingabe. Verglichen werden der vollständige typisierte Report und seine Serialisierung sowie separat SHA-256 über die geordneten Observations. Dadurch zählen IDs, Reihenfolge, Werte, Unknownzustände, Zeitbasis und Locators zum Vergleich. Eine leere oder fehlgeschlagene zweite Ausführung ist kein Gleichheitsnachweis.
2. JSON-Roundtrip des existierenden `deadlock_brain_core::replay::ReplayReport`. Das ist **nicht** gleichbedeutend mit einer erfolgreichen Überführung in den integrierten `brain-contracts`-Vertrag.
3. Locatorgrenzen gegen Dateilänge und Befehlszahl. Das ist **kein** unabhängiger Byte-/Paket-/Zustandspräfix-Referenzvergleich; dieser bleibt separat `unverified`.
4. Vorhandene Dublettenklassifikation für identische Generationen und ein echter neuer Decode mit geänderter Tick-Selektion. Der zweite Fall muss eine andere Generation mit `Reparsed` erzeugen. Ein geänderter Parserpin und dauerhaftes Store-Verhalten sind damit nicht geprüft.
5. Eine private beschädigte Ableitung: acht Byte Headerpräfix der berechtigten Eingabe. Sie trägt einen eigenen erwarteten Hash und muss als `DamagedReplay` scheitern, nicht bloß an `HashMismatch`. Das Original wird nicht überschrieben. Dies deckt nur diese Kürzung ab, nicht jede denkbare Snappy-/Protobuf-/Entity-Beschädigung eines realen Matches.
6. Je eine zusätzliche Ausführung mit CPU-Limit von einer Sekunde bzw. Adressraumlimit von 32 MiB, ohne automatische Lockerung und ohne Retries. Ergebnis und tatsächlich konfigurierte Budgets werden protokolliert. Erfolg bedeutet lediglich, dass die Ausführung unter diesem Profil abgeschlossen wurde. `WorkerFailure` wird nicht als bewiesener OOM, `BudgetExceeded` nicht ohne weiteren Nachweis als CPU-Verbrauch ausgegeben. CPUzeit, Peak-RSS und tatsächliches Erreichen eines Ressourcenlimits werden nicht erfunden.

Die Sequenz umfasst maximal sechs Decoderaufrufe für **eine** Ausgangsdatei einschließlich ihrer beschädigten Ableitung. Jeder Aufruf behält die expliziten Workerbudgets. Ein Fehler beim ersten Decode erzeugt einen gesäuberten Fehlerbericht; nicht ausgeführte Prüfungen bleiben `unverified`.

### Datenschutz und Freigabe

Der Prüfmodus verlangt die vorhandenen lokalen Verarbeitungs-/Raw-Aufbewahrungsrechte, verbietet externen Egress und verlangt zusätzlich einen expliziten Raw-SHA-256-Pin. Rawdatei und privates Berechtigungsmanifest müssen reguläre Dateien außerhalb eines Git-Checkouts sein. Auch ein temporäres Verzeichnis innerhalb von Git wird vor der ersten Raw-Momentaufnahme abgewiesen, insbesondere bei entsprechend gesetztem `TMPDIR`. Die Prüfung berücksichtigt `.git`-Dateien von Worktrees, normale `.git`-Verzeichnisse sowie kanonische und lexikalische Vorfahren. Das ist eine technische Schranke, kein Ersatz für tatsächliche Betreiberrechte, eine Aufbewahrungsregel oder einen sorgfältigen Commit-Review.

Stdout enthält ausschließlich typisierte Statuswerte, Revisionen, Hashes, Budgets und aggregierte Zähler. Keine Spieler-/Servernamen, Match-ID, Berechtigungsreferenz, Scope, Quellobjektreferenz, privaten Dateipfade, Netzwerkfeldwerte oder rohen Locators. Unknown-Gründe bleiben getrennt erhalten. Fehlermeldungen verwenden ausschließlich die vorhandene `ReplayFailure`-Klassifikation, niemals ungefilterte Parserfehlermeldungen. Berichte und private Referenzbelege trotzdem zunächst außerhalb von Git aufbewahren; nur ausdrücklich geprüfte Metadaten dürfen in einen späteren PR.

`status=blocked`, `real_match_verified=false`, `integration_verified=false` und `coaching_eligible=false` bleiben auch bei `technical_validation_passed=true` bestehen. Der Prüfmodus endet dann weiterhin mit **Exit 2**. Es gibt absichtlich keinen Schalter, der Codec-Erfolg zur vollständigen C10-Freigabe erklärt. Bei ungültiger Anfrage gibt es ebenfalls Exit 2, aber keinen Erfolgsbericht. Die bisherigen Unterbefehle `decode`, `manifest` und der Workerbetrieb bleiben erhalten.

## Feldweise Abnahmematrix

Kein echter Match wurde untersucht. Deshalb ist die Spalte **Echter Match** in jeder Zeile `unverified`; Implementierungsfähigkeit darf nicht mit Realmatch-Nachweis verwechselt werden. `supported`, `unsupported`, `unknown` und `parser-dependent` beschreiben den bestehenden technischen Pfad. Ein vorhandener Rohwert belegt nicht dessen fachliche Bedeutung.

| Feld / Bereich | Implementierung | Echter Match | Grenze / notwendiger Nachweis |
|---|---|---|---|
| Containerheader, Magic, Offsets, Stop / EOF | supported | unverified | tatsächlichen Container vollständig dekodieren; kein Dateinamenbeweis |
| Snappy / Dekompression | supported | unverified | reale komprimierte Befehle und verschachtelte Stringtable-Ebenen nachweisen; Bericht zählt nur komprimierte Observation-Locators |
| Header: Patchversion, Buildnummer, Gameverzeichnis | parser-dependent | unverified | pro Feld bekannte / fehlende Werte getrennt; Buildnummer nicht automatisch als bestätigten Spielpatch interpretieren |
| FileInfo: Playbackticks / Playbacksekunden | parser-dependent | unverified | vorhandene Felder sind keine automatisch verifizierte Matchdauer |
| Befehls-Tickbasis einschließlich Initialisierungstick -1 | supported | unverified | Tickdefinition gegen Referenz prüfen; -1 nicht zu Spielzeit 0 machen |
| Übertragenes Server-Tickintervall | parser-dependent | unverified | keine feste 60-Hz-Annahme; beobachtete Änderungen getrennt prüfen |
| Spielzeit, Pregame, Pause, Zeitursprung | unknown | unverified | kein belegter Gameclock-Anker; keine Millisekunden erfinden |
| Entityindex, Klasse, CREATE-/UPDATE-/LEAVE-/DELETE-Lifecycle | parser-dependent | unverified | Schema, Stichprobenselektion und Indexwiederverwendung gegen echte Referenz prüfen |
| CREATE-Ordinal | parser-dependent | unverified | lokales Ordinal ist weder Netzwerkserial noch Spieleridentität |
| Netzwerkserial | unknown | unverified | Backend gibt keine gesicherte Serial weiter |
| `m_iTeamNum`, `m_iHealth`, `m_iMaxHealth` | parser-dependent | unverified | Rohwerte und echte Null von fehlendem Feld unterscheiden; Units nicht fachlich freigeben |
| `m_hPawn` | parser-dependent | unverified | roher Handle, keine gesicherte Spielerzuordnung ohne Serial |
| Spielerzuordnung / Accountidentität | unsupported | unverified | keine Namen oder IDs ergänzen; Slot-/Controller-/Pawn-Verbindung separat belegen |
| Spawned-/Loading-Hero-ID-Netzwerkpfade | parser-dependent | unverified | beide Pfade separat zählen, nicht still als Hero-Namen behandeln |
| Fachliche Herozuordnung / Hero-Namen | unsupported | unverified | Mappingrevision und unabhängige Referenz fehlen |
| Zell-/Vektorkomponenten | parser-dependent | unverified | rohe Komponenten, keine bestätigten Weltkoordinaten oder Einheiten |
| Kill / Death / Assist, Itemkauf, Ability-Upgrade, Objectives | unsupported | unverified | kein technisches Paket zu einem solchen Gameplayereignis umdeuten |
| Opaque Paketmarker | parser-dependent | unverified | identifiziert technische Nachricht, nicht fachlich bestätigte Aktion |
| Raw-Locator / Befehlsbereich / Paketordinal / Zustandspräfix | supported | unverified | Grenzen und deterministische Wiedergabe allein ersetzen keine Byte-/Präfix-Referenz |
| Determinismus / gleiche Auswahl und Revisionen | supported | unverified | dieselbe echte Datei mindestens zweimal in frischen Prozessen vergleichen |
| Dubletten-/Reparse-Klassifikation | supported | unverified | reine Klassifikation; echte transaktionale Store-Idempotenz separat offen |
| Beschädigte Kopie | supported | unverified | bisher synthetische Faultnachweise und vorbereitete Headerkürzung |
| CPU-Limit | supported | unverified | synthetischer nicht kooperierender Worker; echte CPUzeit / Limitfall offen |
| Memory-Limit | supported | unverified | RLIMIT_AS ist Adressraum, nicht Peak-RSS; echte Ressourcenmessung offen |

Der maschinenlesbare Bericht enthält zusätzlich jede der zwölf aktuell selektierten Netzwerkproperties einzeln mit `implementation`, `verification`, `known`, `unknown` und aggregierten `unknown_reasons`. Bei ausbleibenden Entity-Observations bleiben Zähler null; das wird nicht als Nachweis feldweiter Abdeckung interpretiert.

## Tatsächlich fehlende Glieder der Kette

Prüfung der Basis `087c522`:

| Übergang | Vorhandener Baustein | Befund |
|---|---|---|
| Raw → Decoder → normalisierte Observation | `dbrain-replay::WorkerDecoder` | ausführbar, bislang nur synthetisch geprüft |
| Observation → bisheriger gemeinsamer Replayvertrag | `deadlock-brain-core/src/replay.rs` | typisiert, inklusive Unknown-Zeit und Provenienz |
| Replayvertrag → vereinheitlichtes `brain-contracts` | `brain-contracts/src/lib.rs` | C4 noch nicht auf dieser Basis integriert; dortiger älterer `ReplayObservation` verlangt `occurred_at_ms: u64` |
| Vertrag → Store | `ObservationStore`, `decode_into_store` | Trait / Orchestrierungsnaht vorhanden, Implementierung auf dieser Basis nur als Testdouble in `tests/ports.rs`; kein belegter produktiver Postgresadapter |
| Store → Domain-/Learning-Port | vorhandene Domain-/Learning-Bausteine | kein nachgewiesener Replay-Durchstich; keine Trainings-/Coachingfreigabe |

C10 baut keinen Ersatzadapter, keine zweite Datenbank und keinen weiteren Contractzweig, um diese Lücken zu verdecken. Nach Integration des zuständigen gemeinsamen Vertrags und Storeadapters ist der vorhandene Pfad in einer isolierten Scratch-Datenbank zu prüfen: atomarer Commit, Abfrage, Duplicate, neue Reparsegeneration, erhaltene Provenienzen/Scopes und Quarantäne ohne Teilpublikation. Die reine Port-Testdouble-Suite bleibt klar als solche bezeichnet.

## Lokaler Betreiberlauf nach Bereitstellung

Genau eine echte, ausdrücklich freigegebene `.dem` und eine private Kopie von `architecture/migration/replays/s14/AUTHORIZED_REQUEST.example.json` außerhalb sämtlicher Checkouts bereitstellen. Die Beispielvorlage ist absichtlich nicht autorisiert. Freigabe, Herkunft, Quellrevision, Raw-Aufbewahrung, Löschregel und berechtigte Matchreferenz privat dokumentieren. Den SHA-256 der tatsächlichen Datei in `source.expected_sha256` setzen. Pflichtfelder nicht mit geratenen Werten füllen. Ohne diesen Nachweis kein echter Lauf.

Aus dem geprüften Stand bauen, ohne Runtime-Konfiguration oder Dienste zu ändern:

```bash
SQLX_OFFLINE=true cargo build --manifest-path rust/Cargo.toml \
  -p dbrain-replay --bin dbrain-replay-worker --release --locked --offline
```

Mit absoluten, tatsächlich bereitgestellten privaten Pfaden ausführen. `PRIVATE_REPORT` ebenfalls außerhalb von Git wählen, mit restriktiven Dateirechten und ohne existierende Datei zu überschreiben:

```bash
umask 077
set -o noclobber
# BIN, PRIVATE_DEM, PRIVATE_REQUEST und PRIVATE_REPORT sind lokale, geprüfte Pfade.
"$BIN" validate "$PRIVATE_DEM" "$PRIVATE_REQUEST" > "$PRIVATE_REPORT"
# Exit 2 ist die weiterhin offene C10-Gesamtabnahme, kein automatischer Decode-Erfolg.
```

`technical_validation_passed`, die einzelnen Fehlerklassen und nicht ausgeführten Prüfungen auswerten. Keine Limits eigenmächtig erhöhen. Danach unabhängige feldweise Referenz mit festgehaltener Parser-/Schema-/Mappingrevision prüfen und die echte gemeinsame Store-/Domainkette separat testen. Gemeinsam genutzte Parser-/Proto-Vorfahren offenlegen. Debug-/Releasevergleich, anderer echter Parserpin und reale CPU-/RSS-Messwerte sind zusätzliche offene Nachweise, keine Ergebnisse dieses vorbereitenden PRs.

Die C10-Anforderung umfasst **einen** berechtigten Match. Der größere S14-Corpus-/Holdoutplan bleibt davon getrennt und wird nicht als neue Voraussetzung für diesen Pilot ausgegeben.

## Tests und Nachweisstand

Die neue CLI-Suite verwendet ausschließlich selbst erzeugte Wire-Daten. Sie prüft wiederholten Decode, Hash-Pin-/Rechteverweigerung, blockierte Gesamtfreigabe, unbekannte Zeit, fehlende Entityabdeckung, explizit offene Store-/Learningchecks, Quarantäneklassifikation und den Ausschluss privater Canarywerte aus Stdout. Separate Fälle sperren Rawdatei und Berechtigungsmanifest in Git-Checkouts.

Abgeschlossener lokaler Lauf: `architecture/migration/replays/s14/check-decoder.sh`, Rust/Cargo 1.97.1, `SQLX_OFFLINE=true`, `--locked --offline`, separater Cargo-Target außerhalb von Git. Exit **0**.

| Prüfung | Ergebnis |
|---|---|
| Formatter für Decoder und bisherigen gemeinsamen Replayvertrag | bestanden |
| Clippy für alle Decoder-Targets, `-D warnings` | bestanden |
| Decoder Debug | 65 bestanden, 0 fehlgeschlagen, 1 Helper ignoriert |
| Decoder Release | 65 bestanden, 0 fehlgeschlagen, 1 Helper ignoriert |
| Bestehender unabhängiger S14-Audit | 56 bestanden, 0 fehlgeschlagen |
| Debug-/Release-Capability-Manifest | mit `cmp` bytegleich, Exit 0 |
| Reales Corpus-Gate | erwarteter Exit 2, `status=blocked`, `real_matches=0`, `integration_verified=false` |
| Prüfung auf getrackte Replaydateien im bestehenden Gesamtskript | bestanden |

Insgesamt **186 erfolgreiche Testausführungen**; Debug und Release führen dieselben 65 Fälle aus. Die beiden Ignore-Einträge sind der bereits vorhandene Sandbox-Unterprozesshelper, der von den aktiven Tests ausdrücklich gestartet wird, keine übersprungene Realmatch-Abnahme. Der gesamte Rust-Workspace und eine echte Postgres-Replayintegration wurden dadurch nicht pauschal als geprüft erklärt.

Rot-Gegenproben vor der jeweiligen funktionalen Korrektur:

- Sieben neue CLI-Tests am unveränderten Binary: **0 bestanden, 7 fehlgeschlagen**, Exit 101. Nach Einbindung des Prüfmodus: **7 bestanden, 0 fehlgeschlagen**.
- Anschließend neuer TMPDIR-Schutztest ohne die Sperre: **7 bestanden, 1 fehlgeschlagen**, Exit 101. Nach der Sperre: **8 bestanden, 0 fehlgeschlagen**, im abschließenden Debug- und Release-Gesamtlauf bestätigt.

Der erste Buildversuch traf auf das System-Cargo 1.75 und scheiterte bereits am Edition-2024-Manifest. Es wurde ausschließlich die vorhandene Rustup-Toolchain 1.97.1 verwendet, ohne Edition, Dependency-Pins oder Lockfile zu lockern. Das ist ein behobenes Werkzeugpfadproblem, keine als funktionaler Rot-Test gezählte Compilerstörung.

Aktueller vollständiger Commit-SHA, PR-Referenz und tatsächlicher Remote-CI-Stand gehören zum PR-Abnahmeprotokoll. Lokale grüne Tests sind weder ein behaupteter grüner GitHub-Run noch die weiterhin fehlende C10-Realabnahme.
