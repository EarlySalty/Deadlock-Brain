# CODEX Replay Decoder Completion

Stand: 25. September 2026. Repository: `EarlySalty/Deadlock-Brain`.
Branch: `codex/replay-decoder-completion-20260925`.
Basis: `25c6ed6951370b092f60c67a35bdbe37440ece5d`, zu Arbeitsbeginn frisch geholtes `origin/main`.
Übernommener PR-29-Stand: `afdffba173c72ced4149b4fa9efb594f8cd06902`; ursprünglicher Audit-Commit: `d4668ccec431ff128c4778d7839487848c8c4eb6`.

**Ergebnis: ausführbarer Rust-Decoder mit begrenztem Worker und synthetischer Codec-/Faultabnahme, nicht mehr nur ein Audit. Keine reale Replayvalidierung, keine produktive Store-/Lastabnahme und keine S14-Gesamtfreigabe. Nicht gemergt, nicht deployt, keine Dienste neu gestartet.**

## Bestand und Architektur

PR #29 wurde nicht blind gemergt. Sein dependencyfreier Rust-Inventarprüfer, die echten leeren Corpusregister, Kandidaten-Pins und Referenzpläne wurden bestandserhaltend auf die aktuelle Basis übernommen. Ihre Aussagen über fehlende Runtime beschreiben den historischen PR-29-Stand. Dieser Handoff und `replays/s14/IMPLEMENTED_DECODER.json` beschreiben die neue Implementierung.

Der bestehende `dbrain-sources`-Deadlock-API-Pfad bleibt erhalten. Er ist kein lokaler `.dem`-Decoder und wird nicht dafür ausgegeben. Neue Umsetzung:

- `rust/crates/dbrain-replay`: tatsächlicher Source-2-/Snappy-/Protobuf-/Entity-Decodepfad, Worker-Supervisor, Capability-CLI, Unit-, Property-, Fault- und Porttests.
- `rust/crates/deadlock-brain-core/src/replay.rs`: additive gemeinsame Contracts und Ports im auf main bereits verwendeten Core. Keine zweite Datenbank, Queue, Fact-Engine oder Provideranbindung.
- `decode_into_store`: echter Decoderaufruf, gemeinsame Dublettenentscheidung und Observation-/Quarantäne-Übergabe. Im Test wird dafür bewusst nur ein Test-Store verwendet.
- Bestehender CI-Job `Rust compile`: zusätzlich Format-/Lint-/Debug-/Release-Decodertests und der ursprüngliche Audit-Gegencheck. Kein neues optionales Gate, das bei fehlenden Tests einfach grün bliebe. Die bestehenden Merge-/Release-Regeln wurden nicht freigeschaltet.

Der parallel bearbeitete, nicht auf der Ausgangsbasis integrierte `brain-contracts`-/S02-S09-Branch wurde nicht als implizite Dependency eingebaut. S02 muss beim Zusammenführen die gemeinsamen Replaytypen erhalten oder verlustfrei in seinen Core-Vertrag übernehmen. Insbesondere darf ein verpflichtendes `occurred_at_ms: u64` nicht aus `Unknown` oder dem Initialisierungstick `-1` einen erfundenen Zeitwert machen.

## Exakte Abhängigkeiten und Revisionen

| Bestandteil | Tatsächlich verwendeter Pin |
|---|---|
| `haste_core`, außerdem dessen `haste_vartype` | `deadlock-api/haste@bfb292d4798031350861ad297aa26753267a1ea6` |
| `valveprotos` | `deadlock-api/valveprotos-rs@4f4a3cb1b0c6f19af59a722acb79ecccd01f61f6` |
| `dungers` und Teilcrates | `deadlock-api/dungers@5e1e2aac76a027987911de3ef3d23ecfd992a7fb` |
| `prost` | `=0.14.4` |
| `snap` | `=1.1.1` |
| `nix` / `seccompiler` / `libc` | `=0.29.0` / `=0.5.0` / `=0.2.186` |
| `proptest`, nur Tests | `=1.10.0` |
| Übrige Registry-Closure | exakte Versionen und Prüfsummen in `rust/Cargo.lock`; Build mit `--locked` |
| Geprüfte Buildumgebung | Linux x86_64, Rust 1.97.1, `protoc` 3.21.12 |

Der `valveprotos`-Commit ist der **tatsächliche transitive Pin** des ausgewählten Haste-Standes. Kein README-HEAD und kein unabhängiger Upstream-HEAD wurde substituiert. Dessen Manifest verlangt bereits `prost 0.14.4`; ein zunächst versuchtes exakt gepinntes `0.14.3` war inkompatibel und wurde anhand dieses Manifests korrigiert. `haste_core` wird direkt und ohne Async-/Broadcast-Feature verwendet, nicht die netzwerkfähige Haste-Root-API. Die bestehenden Core-Abhängigkeiten enthalten weiterhin eigene HTTP-/DB-Bibliotheken; daraus wird keine falsche Behauptung einer vollständig netzwerkbibliotheksfreien Binary abgeleitet. Laufzeit-Netzwerkzugriff des Decoderworkers wird durch Seccomp gesperrt.

`parser_revision()` besteht aus dem Haste-Pin und einem Hash der kompilierten Adapterquellen, des gemeinsamen Replayvertrags, der Cargo-Manifeste und des Lockfiles. Schema-Revision, `network-observations/v1`, Quellrevision und Raw-SHA-256 bleiben separate Felder. Neue Parser-/Schema-/Selektionsgenerationen dürfen alte Observations nicht still ergänzen oder vermischen.

## Tatsächlich implementierte Fähigkeiten

| Bereich | Nachweis und Grenze |
|---|---|
| `.dem`-Container | überprüfte Magic, Offsets, kanonische begrenzte Varints, Befehle, Stop/Trailer, vollständige EOF-Prüfung |
| Dekompression | Snappy-Längenprüfung vor Allokation auf allen drei Ebenen: Befehl, CreateStringTable und einzelne komprimierte UserData-Einträge in Create/UpdateStringTable; Einzel-, feste Backend-Scratch- und Gesamtgrenzen |
| Rust-Decoding | echte `haste_core::Parser`-Ausführung mit eigenem sicheren `DemoStream`; kein Dummy, Remote-API-Aufruf oder fremder Runtimeprozess |
| Entity-Mapping | echte SendTables, Klassen, Baselines, CREATE/UPDATE/LEAVE/DELETE, Indexwiederverwendung und Tick-Sampling anhand synthetisch erzeugter Wire-Daten getestet |
| Unbekanntwerte | fehlende Felder, Units, Netzwerkserial, Spieleridentität und Gamezeit bleiben unbekannt; beobachtete Null bleibt Null |
| Zeitbasis | Quelltick und tatsächlich übermitteltes Tickintervall; keine 60-Hz-Annahme, kein erfundener Pregame-/Pause-/Gamezeit-Ursprung |
| Beobachtungen | deterministische typisierte Header-, FileInfo-, ServerInfo-, EntityState- und opaque Paketmarker |
| Provenienz | SHA-256 der wirklich decodierten privaten Raw-Momentaufnahme, getrennte Revisionen, Befehlsbereich und Paket-/Callbackordinal |
| Zustandsprovenienz | Entity-Snapshots tragen `requires_state_prefix=true`; Baselines und frühere Deltas sind für ihren Nachweis erforderlich |
| Dubletten/Reparse | identische Generation idempotent; belegter gleicher Match unter gleicher Decoderkonfiguration zählt nicht doppelt; andere Revision/Selektion erzeugt eigene Generation; keine Scope-Erweiterung |
| Fehler | beschädigte Container, unbekannte zustandsändernde Struktur, Parserfehler/Panic, Hashabweichung, Ressourcenüberschreitung und ungültige Workerausgabe werden nicht als vollständige Reports publiziert |
| Sandbox | CPU-/Adressraum-/Dateigrößen-Rlimits, Core-Dump-Sperre, Seccomp-Allowlist, geschlossene geerbte Deskriptoren, leere Workerumgebung, private temporäre Dateien, Watchdog mit Kill und Reap |
| Coaching | `coaching_eligible=false`; gemeinsame Übergabe verweigert künstlich eingeschleuste Coaching-/Realvalidierungsfreigaben |

Alle Runtime-Capabilities behalten `real_replay_verified=false`. Rohwerte wie Health, Team-ID, Hero-ID-Feldpfade und Zell-/Vektorwerte sind keine dadurch bereits fachlich validierten Spielmetriken. Kill-/Death-/Assist-Events, Itemkäufe, Ability-Upgrades, Objective-Zuordnung, Weltkoordinaten und Coaching werden nicht aus unzureichenden Daten erfunden.

Unbekannte neue FullPacket-Tabellen, standalone `DemStringTables`, wiederholte Schemas, nicht positionsgetreue Klassen-IDs sowie nicht unterstützte zustandsändernde Clear-/Split-/Netzwerkschema-Nachrichten werden quarantänisiert. Ein still weiterlaufender Parser mit veraltetem Zustand wäre kein Erfolg. `.dem.bz2` und andere Archivhüllen werden nicht automatisch entpackt.

## Budgets und offene Infrastrukturabnahme

Defaultprofil: 512 MiB Raw, 2 MiB einzelner Befehl, 2 GiB kumuliert decodiert, 2 Millionen Befehle, 10 Millionen Pakete, 16.384 Entity-Indizes, 100.000 Observations, 32 MiB Ausgabe, 768 MiB Worker-Adressraum, 120 CPU-Sekunden, 180 Sekunden Wallclock. Anfragen besitzen validierte harte Obergrenzen und können engere Limits wählen. Keine automatische Lockerung bei einem problematischen Replay.

Dies ist ein explizites Test-/Startprofil, keine gemessene Produktionsfreigabe. Gegenüber dem alten PR-29-Entwurf sind Wallclock, Output- und Observationlimit nun konkrete konfigurierbare Runtimewerte. Das konservativere alte Messprofil kann über die Anfrage eingestellt werden. Eine neue Queue mit vier Plätzen wurde ausdrücklich nicht gebaut.

Ein geteilter `WorkerDecoder` erlaubt einen gleichzeitigen Decode; seine Klone teilen diese Sperre. Unabhängig erzeugte Instanzen sind keine globale Parallelitätsbegrenzung. S04 muss seine vorhandene globale Zulassung, Cancellation-/Retrypolitik und Laststeuerung anschließen. Das Rlimit gilt dem Worker-Adressraum, nicht dem gesamten bestehenden Brain-Prozess. Raw-Kopie und Workerausgabe sind im Parent größenbegrenzt. Blockierende lokale Dateisystemoperationen haben keine separate Kernel-I/O-Deadline. Ein SIGKILL/Abort wird ohne gesicherten Grund als `WorkerFailure`, nicht fälschlich als bewiesener OOM klassifiziert.

Originaldateien und Raw-Store-Provenienz werden nicht überschrieben. Die kurzlebige Decoder-Momentaufnahme ersetzt aber **keine** dauerhafte Raw-Archivierung. Bei hartem Parent-Absturz können private temporäre Dateien verbleiben; die bestehende Job-/Raw-Bereinigung muss diesen Fall abdecken. Produktive Postgres-Transaktionen, ACLs/Tombstones, dauerhafte Quarantäne, Crash-Recovery und konkurrierende Publisher wurden nicht durch einen Test-Store als bestanden erklärt.

## Ausgeführte Tests und Gegenproben

Gesamtskript: `architecture/migration/replays/s14/check-decoder.sh` mit Rust 1.97.1, `SQLX_OFFLINE=true`, isoliertem Cargo-Target und ohne Produktionsdaten.

- Formatter und Clippy `-D warnings` für den neuen Decoder einschließlich aller Testtargets, Formatprüfung des neuen gemeinsamen Contracts. Der gesamte Rust-Workspace besteht `cargo check --workspace --all-targets --locked --offline` mit `SQLX_OFFLINE=true`. Debug- und Release-Binary geben dasselbe Capability-Manifest und denselben Parserfingerprint aus.
- **57 aktive Decoder-Tests im Debugprofil und dieselben 57 im Releaseprofil.** Aufteilung: 4 Sandbox-/Ressourcen-/Watchdogtests, 33 Codec-/Fault-/Propertytests, 4 echte synthetische Entity-Decodetests, 5 gemeinsame Port-/Deduplizierungstests, 5 Strukturregressionen und 6 Tests der dritten Snappy-Ebene einschließlich Gesamtbudget und UpdateStringTable.
- Ein als `ignored` markierter Unterprozess-Helper wird von den vier aktiven Sandboxtests gezielt gestartet. Er ist kein unbearbeiteter Validierungstest und wird nicht ins Produktionsbinary gebaut.
- **56 unverändert erhaltene Audit-Tests** aus PR #29; Audit-Releasebuild und CLI-Gegenprobe bestanden. Die leeren realen Register ergeben weiterhin `status=blocked`, `integration_verified=false`, `real_matches=0` und den erwarteten Exitcode 2.
- Propertytests verwenden je 32 Fälle. Zusätzlich wird jeder strikte Präfix des selbst erzeugten Minimalcontainers verworfen. Es wurden keine aufgezeichneten Replays als Fixture übernommen oder heruntergeladen.

Fünf gezielte Rot-Gegenproben wurden ausschließlich im eigenen Worktree ohne Commit der Mutation ausgeführt. Jeweils war derselbe einzelne Test vorher grün und mit dem folgenden Defekt funktional rot, nicht bloß wegen eines Compilerfehlers:

| Vorübergehender Defekt | Test | Rot-Ergebnis |
|---|---|---|
| EOF mit bloßer Dateiposition gleichsetzen | `partial_final_varint_is_not_a_clean_eof` | 0 bestanden, 1 fehlgeschlagen, Exit 101 |
| Gesamtbudgetprüfung entfernen | `total_decoded_budget_is_enforced` | 0 bestanden, 1 fehlgeschlagen, Exit 101 |
| Local-Processing-Rechteprüfung abschwächen | `rights_are_checked_before_opening_any_raw_path` | 0 bestanden, 1 fehlgeschlagen, Exit 101 |
| Extraktionsauswahl aus Generation entfernen | `extraction_selection_creates_a_new_generation` | 0 bestanden, 1 fehlgeschlagen, Exit 101 |
| Dekomprimierte Stringtable-Einträge mit null Bytes verbuchen | `per_entry_snappy_cannot_evade_the_total_budget` | 0 bestanden, 1 fehlgeschlagen, Exit 101 |

Die mutierten Quellen wurden bytegleich wiederhergestellt; danach wurde das gesamte Prüfskript erneut ausgeführt. Dies sind fünf gezielte Mutationstests, keine behauptete Mutation-Coverage jedes einzelnen Tests und kein nachträglich erfundenes Red-first-TDD-Protokoll.

Die geprüfte Git-Revision und aktuellen GitHub-Run-IDs werden im PR-Abnahmeprotokoll am tatsächlich gepushten Head dokumentiert. Ein älterer grüner Run gilt nicht für einen neuen Push. GitHub-Actions und ein unabhängiges Review werden nicht durch diesen eigenen Testbericht ersetzt.

Erster GitHub-Lauf auf `9fcbc867e91440b55d2766e97c3b837b18b89a29`: Semantic Review `36165944545`, Versuch 1, erfolgreich. Deterministic PR CI `36165944648`, Versuch 1, fand beim `valveprotos`-Build die fehlende Standarddatei `google/protobuf/descriptor.proto` auf dem frischen Runner. Lokal waren diese Header bereits installiert. Korrektur: CI installiert zusätzlich `libprotobuf-dev` und prüft die konkrete Headerdatei vor dem Build. Der rote Lauf wird nicht als Decoder-Testabnahme ausgegeben; die vollständige CI und das Review müssen für den Korrektur-Head erneut laufen.

## Was Claude lokal konkret noch ausführen muss

### 1. Berechtigter echter Pilot und Corpus

Zuerst **einen** echten, vollständig berechtigten Match außerhalb des öffentlichen Checkouts bereitstellen. Vor dem Decode tatsächliche Rechte für Verarbeitung, Raw-Speicherung, Referenzvergleich und Aufbewahrung prüfen. Lokal dokumentieren: Raw-Objektreferenz, SHA-256, Größe, Herkunft/Quellrevision, Abrufzeit, berechtigte Matchreferenz und deren Nachweis, Patch/Build/Mode, Sichtbarkeitsscope, Ablauf-/Löschregel. Keine öffentliche URL als Rechtebeleg und keinen Dateinamen als Match-/Patchbeweis verwenden.

Danach den vorhandenen Referenzplan mit **10 bis 20 unterschiedlichen realen freigegebenen Matches** oder ausdrücklich genehmigtem Umfang füllen. Mehrere Feeds, Hashes oder Parsergenerationen desselben Matches zählen nicht als neue Matches. Mehrere relevante Builds/Patches/Modes und Hero-Konstellationen, einen echten Schemawechsel, unvollständige Aufzeichnungen und berechtigt erzeugte Beschädigungen abdecken. Referenzdaten unter denselben Rechten halten.

Die Vorlage `replays/s14/AUTHORIZED_REQUEST.example.json` ist absichtlich nicht freigegeben. Claude muss eine private Kopie mit echten Nachweisen und Limits erzeugen. Die beiden notwendigen lokalen Rechteflags dürfen erst danach gesetzt werden. `external_egress_allowed` bleibt false. Jede echte lokale Datei mit der Release-CLI verarbeiten und Manifest/Report/Exitstatus unter privaten Referenzen sichern. Kein automatischer Downloadpfad ist Bestandteil dieses Decoders.

### 2. Echter Durchstich über den gemeinsamen Anwendungspfad

Am echten Pilot Raw → Rust-Worker → gemeinsame Observations → vorhandene S03-Transaktion → selektierte abfragbare Evidenz ausführen. Dafür dieselbe aktive Knowledge-/Source-Generation wie beim gemeinsamen Pilot benutzen. Keine zweite Store-/Queueimplementierung hinzufügen. Fehlgeschlagene oder teilweise decodierte Matches dürfen keine sichtbare Teilgeneration erzeugen. Ein korruptes Pilotderivat muss begrenzt in der bestehenden Quarantäne enden.

Diesen Durchstich mit dem parallel integrierten Core-Vertrag abstimmen. Unknown-Zeit, Unknown-Units, negative Initialisierungsticks und getrennte Parser-/Quellrevision dürfen beim Contract-Port nicht verloren gehen. Die aktuelle Port-Suite allein belegt diesen Produktionsanschluss nicht.

### 3. Feldweise Referenzprüfung, keine pauschale Freigabe

Vor dem Vergleich Rubrik, Referenzrevision, Semantik und Toleranzen einfrieren. Einen kompatiblen unabhängigen Parser oder kuratierte Roh-/Spielreferenz benutzen. Gemeinsame Proto-/Parser-Vorfahren offenlegen; zwei Ableitungen desselben Parsers sind kein unabhängiger Beweis.

Für jeden realen Match konkret prüfen:

1. Magic, End-/Trailerzustand, Hash, Dateigröße und Build-/Patch-/Modebeleg; extern referenzierte Matchidentität nicht als aus dem Demo decodiert ausgeben.
2. Klassen/Entity-Indices über CREATE, UPDATE, LEAVE, DELETE und Indexwiederverwendung hinweg vergleichen. Spieler-Slot, Team, Loading-/Spawned-Hero und Wechsel separat nachweisen. Die derzeit fehlende Netzwerkserial bleibt Unknown; Handles nicht ohne Serial als gesicherte Spielerzuordnung auflösen.
3. Commandtick, Server-Tickintervall, Initialisierung, Pregame, Pause, Tick-/Intervallwechsel und Matchende gegen dieselbe Zeitdefinition prüfen. Gamezeit ohne belegten Anker bleibt Unknown. Höchstens einen **verifizierten** Tick Toleranz nach übereinstimmender Zeitdefinition verwenden, keinen pauschalen 60-Hz-Faktor.
4. Rohwerte der ausgewählten Netzwerkfelder prüfen, einschließlich echter Null, fehlender Werte und von früheren Deltas gehaltenem Zustand. Rohe Zell-/Vektorkomponenten erst nach belegtem Koordinatensystem und Quantisierung als Weltpositionen interpretieren.
5. Itembestand versus Kauf-/Verkaufs-/Transferereignis, Ability-Zustand versus Upgrade, Scoreboard-KDA versus einzelne Events sowie Objective-Identität/Typ/Akteur/Zeit getrennt prüfen. Diese Bereiche sind aktuell nicht fachlich freigegeben. Fehlende Coverage muss `unavailable`/`unverified` bleiben; zusätzliche Extractoren benötigen eigene Tests, Revisionen und Referenzen.
6. Eine Stichprobe von Raw-Locators bis zu den Bytes nachvollziehen: gehashte Originaldatei, Befehlsbereich, Snappy-Decoding, Paketordinal, Payloadhash, Entity-Callback und vollständiger erforderlicher Zustandspräfix. Keine scheinpräzisen Offsets innerhalb komprimierter Pakete erzeugen.

Ergebnis pro Feld/Build/Parser als `supported`, `derived`, `unavailable` oder `unverified` festhalten. Für Ableitungen Algorithmus-, Input- und Unitrevision dokumentieren. Keine Coaching-, Kausalitäts- oder Trainingsfreigabe allein aus erfolgreich decodierten Daten ableiten.

### 4. Echter Reparse und dauerhafte Dublettenprüfung

Jede ausgewählte berechtigte Datei mindestens zweimal in frischen Prozessen mit identischer Revision und Selektion decodieren. Kanonische Reports, Observation-IDs/-Menge, Reihenfolge, Werte, Unknownzustände und Locators müssen identisch sein. Debug-/Releaseausgaben auf demselben Pin vergleichen. Eine geänderte Auswahl und anschließend einen ausdrücklich gewählten anderen Parser-/Schema-Pin als neue vollständige Generation prüfen.

Dieselbe Datei über zwei Quellreferenzen, denselben belegten Match über zwei unterschiedliche Raw-Hashes, zwei wirklich unterschiedliche unbekannte Matches und getrennte Berechtigungsscopes einspeisen. Im **echten Store** eine Populationzählung, erhaltene Raw-Provenienzen und keine Scope-Erweiterung nachweisen. Widersprüchliche Identitätsnachweise nicht still zusammenführen. Konkurrierende Einlieferung, Prozessabbruch vor/nach Commit und Neustart testen: keine doppelte aktive Generation und kein Mix alter/neuer Observations. Fehlgeschlagene Reparse-Generationen dürfen die letzte gültige Generation nicht überschreiben.

### 5. Echte Fehler-, Ressourcen- und Datenschutzabnahme

Berechtigte Kopien des Piloten an Container-, Varint-, Protobuf-, Snappy-, Stringtable- und Entity-Grenzen kürzen/beschädigen; Originale erhalten. Falschen erwarteten Hash, unklare/entzogene Rechte, unbekannte Strukturen, Budgets knapp unter dem tatsächlichen Bedarf und ungültige Workerausgaben prüfen. Quarantäne statt Erfolgsreport, keine Teilpublikation, begrenzte Retries und überlebender Parent sind Pflicht.

Auf der tatsächlichen Zielhardware CPUzeit/Match, Peak-RSS **und** Adressraum, Raw-/decodierte-/selektierte Bytes, Walltime, Reparsezeit, Warteschlangenzeit und gleichzeitige Worker messen. Adressraumlimit nicht als RSS-Messung ausgeben. Gemeinsame Queries und Reembedding parallel belasten; p95/p99 und Fehlerquote gegen ein vorher festgelegtes Basisprofil vergleichen. Budgetbedingte Ablehnungen gültiger großer Dateien als solche berichten, nicht automatisch Limits hochdrehen. Parent-Absturz, abgebrochene Jobs, verbliebene private Temporärdateien, Sperrfreigabe und Bereinigung prüfen.

Sicherstellen, dass kein Netzwerkzugriff, keine geerbte sensible Verbindung, kein Core-Dump und kein Klartextdump von Namen/Steam-IDs/privaten Pfaden entsteht. Widerruf, Tombstones und Löschung müssen Original-Raw, abgeleitete Reports, Quarantäne und historische Releases im bestehenden System erfassen. Replays und personenbezogene Referenzdaten niemals ins öffentliche Git, in öffentliche CI-Artefakte oder in externe Modelle hochladen.

### 6. Holdout und Abnahmeabschluss

Zeitlichen Cutoff T, Development-/Holdoutzuordnung, Regel-/Mapping-/Knowledge-/Parserrevision und Zielmetriken vor Tuning einfrieren. Nach T liegende Matches nicht über andere Feeds in die Entwicklungsmenge leaken lassen. Retrospektiv erzeugte Parserresultate entsprechend kennzeichnen. Coverage, tatsächliches n, Patch/Mode, Auswahlverzerrung und Unsicherheit berichten.

Erst die tatsächlichen lokalen Nachweise in die vorhandenen Corpus-/Capabilityregister und Gate-Abnahme übernehmen. G0/G1/G2/G3/G4 und die gemeinsamen STATUS-/Owner-Dateien wurden hier nicht grün gesetzt. Ein synthetischer Header-, Entity- oder Ressourcen-Test darf niemals als bestandener echter Matchtest eingetragen werden.
