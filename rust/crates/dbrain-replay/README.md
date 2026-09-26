# Lokaler Rust-Replaydecoder

Dieser Adapter decodiert Source-2-`.dem`-Container mit `haste_core` im eigenen begrenzten Prozess. Er ersetzt weder den bestehenden Deadlock-API-Adapter noch den gemeinsamen Store, Scheduler oder Provider. Aufgezeichnete Replays gehören nicht in dieses Repository.

## Nutzung

Voraussetzungen für den geprüften Build: Linux x86_64, Rust 1.97.1, `protoc` 3.21.12. AArch64 ist im Sandboxcode vorgesehen, aber hier nicht ausgeführt. Andere Plattformen werden nicht unterstützt.

```sh
cargo +1.97.1 build --manifest-path rust/Cargo.toml -p dbrain-replay --release --locked
rust/target/release/dbrain-replay-worker manifest
umask 077
rust/target/release/dbrain-replay-worker decode \
  "$AUTHORIZED_DEM" "$PRIVATE_REQUEST_JSON" > "$PRIVATE_RESULT_JSON"
```

Die drei Variablen müssen auf eigene, zugriffsbegrenzte Dateien außerhalb des Git-Checkouts zeigen. Die Anfragevorlage liegt unter `architecture/migration/replays/s14/AUTHORIZED_REQUEST.example.json`. Sie ist absichtlich **nicht freigegeben**. Die aufrufende Policy muss die tatsächliche Berechtigung prüfen und einen vorhandenen lokalen Nachweis referenzieren. Ein vom Aufrufer gesetztes Boolean ist keine rechtliche Prüfung.

`raw_object_ref` bezeichnet die bereits archivierte, berechtigte Originaldatei im gemeinsamen Raw-Store. Der Decoder lädt diesen Verweis niemals aus dem Netz. Er prüft die lokale Datei, erzeugt eine private temporäre Momentaufnahme, hasht genau diese Bytes und lässt das Original unverändert. `source_revision`, Inhalts-Hash, Schema-Pin, Parserrevision und Extraktionsrevision bleiben getrennt. Dauerhafte Aufbewahrung, Löschung und ACLs bleiben Aufgabe des bestehenden Raw-Stores. Nach einem harten Absturz des Elternprozesses können private temporäre Dateien zurückbleiben; sie gehören in dessen vorhandene Bereinigung, nicht in einen neuen Replay-Scheduler.

## C10-Prüflauf für genau eine freigegebene Datei

`dbrain-replay-worker validate "$AUTHORIZED_DEM" "$PRIVATE_REQUEST_JSON"` verwendet denselben Decoder in frischen Workerprozessen für Determinismus, Auswahl-Reparse, beschädigte Headerkopie und engere Ressourcenprofile. Der Modus verlangt einen SHA-256-Pin. Eingaben, Berechtigungsmanifest und temporärer Speicher müssen außerhalb von Git liegen. Stdout enthält ausschließlich gesäuberte Statuswerte, Hashes und aggregierte Feldzähler, keine rohen Observations oder personenbezogenen Werte.

**Exit 2 bleibt auch nach erfolgreicher technischer Prüfung bestehen:** unabhängige Realmatch-Referenz, gemeinsamer Contract-Bridge-, Postgres- und Domain-/Learning-Nachweis werden nicht durch Codec-Erfolg ersetzt. `technical_validation_passed` ist separat von `status=blocked` und den stets falschen Realmatch-/Integrations-/Coachingfreigaben auszuwerten. Der vollständige Ablauf und die offene feldweise Abnahme stehen in `architecture/migration/handoffs/C10_REAL_REPLAY.md`.

## Belegte Ausgaben und Grenzen

Decodiert werden Containerbefehle, Snappy-komprimierte Befehle, SendTables, Klassen, Netzwerkpakete, Stringtable-Aktualisierungen, vorbereitete FullPackets und ausgewählte Entity-Zustände. Entity-Mapping verwendet beobachtete Klassen und Netzwerkindizes. Eine lokale CREATE-Ordnung verhindert das Verwechseln wiederverwendeter Indizes; sie ist **keine Netzwerkserial und keine Spieleridentität**. Der gepinnte Parser stellt die echte Serial nicht bereit.

Die ausgewählten numerischen Netzwerkfelder enthalten unter anderem Health, Team-ID, rohe Pawn-Handles, getrennte Loading-/Spawned-Hero-ID-Pfade sowie rohe Zell-/Vektorkomponenten. Fehlende Werte bleiben `Unknown`, beobachtete Null bleibt Null. Einheiten, Weltkoordinaten, Matchidentität aus dem Demo, KDA, Käufe, Objectives und Coaching werden nicht aus Feldnamen erfunden. Andere Pakettypen erscheinen als begrenzte opaque Marker, nicht als erratene Spielereignisse.

Ticks bleiben vorzeichenbehaftete Quellticks. `-1` ist Initialisierung. Ein tatsächlich empfangenes `tick_interval` ist dokumentiert; es gibt keinen angenommenen 60-Hz-Wert. Spielzeit bleibt ohne belegten Ursprung `Unknown`, insbesondere bei Vorlauf, Pause oder Tickwechseln. `LEAVE` bedeutet Sichtbarkeitswechsel, nicht Tod.

Raw-Locators enthalten den Dateibereich des auslösenden Befehls, dessen Kompressionsstatus und den Hash des decodierten Befehls beziehungsweise Netzwerkpakets. Paket- und Entity-Ordinale sind keine vorgetäuschten Bytepositionen in komprimierten Daten. Entity-Zustände tragen `requires_state_prefix=true`: Für ihren Nachweis muss der gehashte Raw-Präfix ab Dateianfang einschließlich Baselines und früherer Deltas erneut abgespielt werden.

Nicht unterstützte zustandsändernde Strukturen werden quarantänisiert. Dazu gehören standalone `DemStringTables`, unbekannte FullPacket-Tabellen, wiederholte Schemas, nicht positionsgetreue Klassen-IDs sowie die vom gepinnten synchronen Backend nicht umgesetzten Netzwerk-Schema-/Clear-/Split-Nachrichten. Archivhüllen wie `.dem.bz2` werden nicht automatisch erraten oder entpackt.

## Budgets und Ausfallverhalten

Standard: 512 MiB Raw-Datei, 2 MiB pro decodiertem Befehl, 2 GiB kumulativ decodierte Bytes, 2 Millionen Befehle, 10 Millionen Pakete, 16.384 Entity-Indizes, 100.000 Observations, 32 MiB Ausgabe, 768 MiB Worker-Adressraum, 120 CPU-Sekunden, 180 Sekunden Wallclock. Die Anfrage kann innerhalb harter Obergrenzen engere oder größere Budgets wählen. Eine Überschreitung liefert keinen als vollständig ausgegebenen Teilbericht.

Snappy-Ausgabelängen werden vor untrusted Allokationen auf allen drei Ebenen geprüft: Befehle, komprimierte Stringtables und komprimierte UserData-Einträge. Die Eintragsprüfung gilt auch für UpdateStringTable, zählt die tatsächlich angekündigten Ausgabebytes zum Gesamtbudget und respektiert den festen 128-KiB-Scratchbereich des gepinnten Backends. Stringtable-Eintragszahl und Index teilen konservativ das validierte Entity-Limit. Linux-Rlimits begrenzen CPU, Adressraum und Ausgabe; Core-Dumps sind gesperrt. Die Seccomp-Allowlist sperrt neue Dateien, Netzwerkverbindungen, Kindprozesse und Prozesssteuerung. Geerbte Deskriptoren außer umgeleitetem Standard-I/O werden vorher geschlossen. Der Hauptprozess kopiert Raw-Bytes mit festem Puffer und liest nur größenbegrenzte Ausgabe. Das Adressraum-Rlimit gilt dem Worker, nicht dem ganzen bestehenden Anwendungsprozess. Der Watchdog beendet und reapet hängen gebliebene Worker. Prozessabstürze werden ohne spekulative Ursache als `WorkerFailure` behandelt.

Ein `WorkerDecoder` wird vom bestehenden Scheduler wiederverwendet; dessen Klone teilen eine Single-Flight-Sperre. Mehrere unabhängig konstruierte Instanzen sind keine globale Ressourcenplanung. Die globale Parallelität und Last konkurrierender Dienste bleiben beim gemeinsamen Scheduler.

## Gemeinsamer Port statt zweiter Infrastruktur

`deadlock_brain_core::replay` enthält `ReplayDecoder`, `ObservationStore`, `decode_into_store` und die deterministische Dublettenklassifikation. Gleiche Generationen sind idempotent. Derselbe extern belegte Match unter derselben Decoderkonfiguration wird nicht als zweites Sample gezählt. Andere Parser-/Schema-/Auswahlgenerationen werden als Reparse behandelt. Andere Sichtbarkeitsscopes werden niemals durch Deduplication freigegeben.

Der produktive Store muss die Entscheidung und das Umschalten vollständiger Generationen in seiner bestehenden Transaktion ausführen und beide Raw-Provenienzen erhalten. Der Port ist mit dem echten Worker gegen einen **Test-Store** ausgeführt. Ein neuer Postgres-Store oder ein bereits erfolgter Produktionsanschluss wird damit nicht behauptet.

## Reproduzierbarkeit und Tests

Die drei Gitrevisionen sind vollständig gepinnt; Registry-Auflösung und Checksummen stehen in `rust/Cargo.lock`. `parser_revision()` hasht die kompilierten Adapterquellen, den gemeinsamen Replayvertrag, Cargo-Manifeste und Lockfile. Weder README noch Git-HEAD bestimmen diese Revision. Externe Quellrevisionen ändern nicht still die Parseridentität.

```sh
cargo +1.97.1 fetch --manifest-path rust/Cargo.toml --locked
RUSTUP_TOOLCHAIN=1.97.1 bash architecture/migration/replays/s14/check-decoder.sh
```

Die Fixtures werden ausschließlich als eigene synthetische Byte-/Bitstreams im Test erzeugt und nach dem Test gelöscht. Sie durchlaufen den echten Parser und Worker, einschließlich Entity-Lebenszyklen. Der mit `ignored` markierte Sandbox-Helper ist nur ein gezielt von vier aktiven Tests aufgerufener Unterprozess. Er ist nicht im Produktionsbinary enthalten.

**Kein echter Replaytest ist dadurch bestanden.** Alle Laufzeit-Capabilities behalten `real_replay_verified=false`; `coaching_eligible=false`. Die verbindliche lokale Abnahmeliste steht in `architecture/migration/handoffs/CODEX_REPLAY_DECODER_COMPLETION.md`.

Die BSD-3-Lizenz und Attribution des verwendeten Haste-Codes stehen in `THIRD_PARTY_HASTE_BSD3.txt`. Auch die kleine synthetische Huffman-Encodierung bezieht sich auf dessen dokumentierten Wire-Algorithmus. Abhängigkeiten und deren Lizenzhinweise müssen bei einer späteren Binary-Distribution erhalten und geprüft werden.
