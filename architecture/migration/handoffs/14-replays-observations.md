# Übergabe: S14 / Rust-Replays und normalisierte Beobachtungen

Status: **Vorbereitung getestet; vollständiges S14 blockiert, nicht integriert.**
Basis-Commit: `30326512568b7370524956839100462ba71bdb92` (`origin/main`).
Ergebnis-Branch: `migration/s14-replay-preparation-20260924`.
Implementierungscommit: `d4668ccec431ff128c4778d7839487848c8c4eb6`.
PR: https://github.com/EarlySalty/Deadlock-Brain/pull/29 (Draft, offen).
Tatsächlich getesteter Commit: `d4668ccec431ff128c4778d7839487848c8c4eb6`;
Format, Clippy, 56 Tests und Releasebuild nach dem Commit erneut bestanden.
Die nachfolgende Änderung ergänzt nur die Abschlussdokumentation. Der vollständige
aktuelle PR-Head und dessen erneute Prüfungen werden im PR-Protokoll festgehalten.
Contract-/Schema-Version: auf main offen, nicht von S14 geändert.
Source-/Corpus-/Modellversion: kein freigegebener echter Replaycorpus.
Betroffene Anforderungen: R01, R03, R18, R43, R48–R54, R59, R60; nur Vorbereitung,
keine vollständige Abnahme dieser Anforderungen.

## Ergebnis und konkrete Änderungen

Additiv unter `architecture/migration/replays/s14/`:

| Pfad | Ergebnis |
|---|---|
| `audit/Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/main.rs`, `tests/cli.rs`, `check.sh` | dependencyfreier eigenständiger Rust-Inventarprüfer; kein Runtimevertrag, kein Replaydecoder |
| `CORPUS.tsv`, `CAPABILITIES.tsv` | echte Register absichtlich leer, keine erfundenen Replayfälle |
| `CAPABILITY_REVIEW.tsv` | alle neun Pflichtbereiche je unbekanntem Patch/Parserstand ausdrücklich unverified |
| `DECODER_CANDIDATES.json` | live geprüfte Kandidaten-/Transitivrevisionen und tatsächliche Features |
| `CR-S14-01.md` | nötige Contract-/Store-/Job-/Schema-/Owner-/CI-Übergaben |
| `REFERENCE_AND_BUDGET_PLAN.md` | separate echte Feld-/Referenz-/Ressourcen-/Privacy-/Reparse-/Holdoutabnahme |
| `README.md`, `TEST_REPORT.md`, `.gitignore` | belegter Ausgangsstand, Bedienung, Grenzen und tatsächlich ausgeführte Tests |

Kein Rootmanifest, Rootlockfile, DB-Schema, Scheduler, Answer Kernel, produktiver
Parser oder anderer besetzter Runtimepfad geändert. Standalone-Audit nur zur
Vorbereitung, nicht Mitglied des produktiven Rust-Workspaces. Keine neuen externen
Rust-Dependencies und kein permanentes Python-/JVM-/.NET-Hilfsprogramm.

## Nachweise

| Prüfung | Befehl/Umgebung | Tatsächliches Ergebnis |
|---|---|---|
| Format, Clippy, Tests, Releasebuild | `CARGO=/home/naniadm/.cargo/bin/cargo bash architecture/migration/replays/s14/audit/check.sh`, Rust 1.97.1 | bestanden; 46 Library- und 10 CLI-Tests |
| Mindesttoolchain | System-Rust/Cargo 1.75.0; `--locked --offline`, separates Targetverzeichnis | dieselben 56 Tests bestanden |
| Falsch-grün-Gegenprobe | isolierte Codekopie entfernt alle Blocker | erwarteter Exit 101; 26/46 Librarytests rot |
| echtes Corpusregister | manuelle Audit-CLI auf den echten leeren Registern | Exit 2; 0 Matches; implementation_authorized=false und integration_verified=false |
| Kandidatenreview | haste bfb292d…, zugehöriger valveprotos-Pin 4f4a3cb… | Manifests gelesen; kein Decoderbuild/Replaytest |
| echte Replay-/Store-/Worker-/Population-/Kernelintegration und G2–G4 | fehlende Freigaben/Ports/Referenzen | nicht durchgeführt |

Details einschließlich Grenzen und exakter Befehle: `replays/s14/TEST_REPORT.md`.
Eine grüne vorhandene Root-CI ersetzt diese lokale Prüfung nicht; die neue
Standalone-Suite benötigt die angefragte CI-Anbindung durch S02/S10.

## Relevante Funde

Auf integrierter Basis sind G0 bis G6 sowie gemeinsame Contract-/DB-Schemaversion
weiter offen. Auch der separat gesichtete `brain.v1`-Entwurf in Draft-PR #25
enthält noch keinen ausreichenden Replay-/Observationzeit-/Policy-/Provenienz-
und Decoder-/Storeport für S14. Konkrete Lücken stehen im CR.

Der vorhandene `dbrain-sources/src/deadlock_api.rs`-Pfad nutzt externe Demo-SQL-
Jobs; er ist nicht mit einem eigenen Rust-.dem-Decoder gleichzusetzen. Der
begrenzte Scan bekannter Checkouts ergab keine freigegebenen Golden-Dateien;
kein vollständiges Host-/Volume-/DB-Inventar und keine Behauptung genereller
Nichtverfügbarkeit. Rohdaten/Secrets wurden nicht geöffnet oder kopiert.

Aktuell gesichteter haste-Root und haste_core haben `async`, aber kein
`deadlock`-Feature. Auch der von haste gepinnte valveprotos-Stand nutzt andere
Features als die README suggeriert. Default-Branch-HEAD von valveprotos darf
nicht still anstelle des tatsächlichen transitiven Pins eingesetzt werden.
Die geprüften Revisionen stehen vollständig im Kandidatenmanifest.

## Folgen und Grenzen

Datenmigration/Kompatibilität/Wiederanlauf: keine produktive Änderung.
Berechtigungen/Secrets/Egress: keine Secretabfrage, kein Replaydownload,
keine Freigabe oder Sichtbarkeitserweiterung, kein Provideraufruf.
Latenz/Ressourcen: Auditmetadaten auf 256 KiB pro Datei begrenzt; keine Aussage
über reale Replay-/Querylast. Harte Workerlimits sind nur zur Abnahme vorgeschlagen.
Vorhandene Funktionen: vollständig unverändert; keine fremde uncommittete Arbeit
übernommen oder gelöscht. Keine Dienste neu gestartet und keine Jobs ausgelöst.

Das Audit überprüft die Konsistenz deklarierter Metadaten, nicht die Wahrheit
der Rechte-, Hash-, Referenz- oder Capabilitybehauptungen. Selbst positive
Metadaten vergeben niemals Implementierungs-/Integrationsfreigaben. Synthetische
Tests sind keine Golden-Replays, Referenzvalidierung oder Populationabnahme.

## Übergabe an nächste Besitzer

S00/S01: G0 anhand tatsächlichen Inventars und zugelassener Replayreferenzen
prüfen; additive Pfade und späteren Runtimeowner bestätigen. S02/S03: gemeinsam
versionierte Replay-/Observationports und G1 integrieren. S04/S13: gemeinsamen
begrenzten Rust-Jobpfad und kompatible Schemapins liefern. S14 danach: erster
echter erlaubter Raw→Decoder→Observation-Durchstich, anschließend Golden-Replays
und deterministischer Reparse. S05/S10: empirische Integration, unabhängige
Referenz-/Holdout-/Lastabnahme. Kein heimlicher zweiter Store, Scheduler,
Vertrag oder Freigabemechanismus.

## Integration durch S00

Merge-Commit: keiner.
Gate-/STATUS-/Owneränderung: keine.
Freigegeben von: keine G1-/Runtime-/Produktivfreigabe.
Arbeitsmodus: additive Vorbereitung zur Review; keine nächste Implementierungswelle
von S14 eigenmächtig freigegeben.
Planrevision: hochgeladenes Paket v1.0 (20260924-204949).
Echte Integration oder Fixture: öffentliche Kandidatenquellen live gelesen;
synthetische Audit-Tests; kein echter Replay-/Brainintegrationsnachweis.
Sensible Replay-/Publikationsrechte: erster zugelassener interner Fall offen.
