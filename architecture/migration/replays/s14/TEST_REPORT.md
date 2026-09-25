# S14: tatsächliche Prüfungen und offene Nachweise

Datum: 24.09.2026. Basis: `30326512568b7370524956839100462ba71bdb92`.
Prüfumfang: ausschließlich additive S14-Vorbereitung und eigenständiges
`audit/`-Crate, nicht der produktive Brain-Workspace oder ein Replaydecoder.
Getesteter Implementierungscommit: `d4668ccec431ff128c4778d7839487848c8c4eb6`.
Die komplette Vorbereitungssuite wurde nach diesem Commit erneut erfolgreich
ausgeführt. Draft-PR #29 enthält die Änderungen; der folgende Dokumentationscommit
verändert den getesteten Code nicht. Der vollständige Abschlusshead und seine
erneuten Prüfungen stehen im PR-Protokoll.

## Ausgeführte Prüfungen

Arbeitsverzeichnis: eigener `brain-s14-replay-20260924`-Worktree.
Aktuelle Toolchain: Rust/Cargo 1.97.1, zusätzlich System-Rust/Cargo 1.75.0.
Audit-Lockfile Version 3, genau ein eigenes Paket, keine externen Dependencies.

| Prüfung | Tatsächliches Ergebnis |
|---|---|
| `cargo fmt --manifest-path .../audit/Cargo.toml -- --check` | bestanden |
| `cargo clippy --manifest-path .../audit/Cargo.toml --all-targets --locked --offline -- -D warnings` | bestanden, keine Warnungen |
| `cargo test --manifest-path .../audit/Cargo.toml --locked --offline` | 46 Bibliotheks- und 10 CLI-Tests bestanden; 0 fehlgeschlagen, 0 ignoriert |
| `cargo build --manifest-path .../audit/Cargo.toml --release --locked --offline` | bestanden |
| Rust/Cargo 1.75.0 mit separatem Targetverzeichnis | dieselben 56 Tests bestanden |
| aktuelle echte Corpus-/Capabilityregister | erwarteter Exit 2, 0 deklarierte echte Matches, keine Integrations-/Implementierungsfreigabe |
| Rotkontrolle: in einer isolierten Kopie vor Rückgabe alle Blocker löschen | erwarteter Exit 101; 26 von 46 Librarytests fehlgeschlagen, 20 bestanden |
| `git diff --check` | bestanden; wird nach Staging/Commit erneut geprüft |

Die Rotkontrolle verändert ausschließlich eine kurzlebige Kopie in einem neu
angelegten temporären Verzeichnis. Die eigentliche Implementierung blieb dabei
unverändert. Das temporäre Verzeichnis wurde entfernt. Das belegt, dass ein
pauschales Entfernen der Blocker von der Testsuite erkannt wird, nicht die
Vollständigkeit aller denkbaren Mutationen.

## Reproduzierbare Befehle

```sh
# Aus dem Repositoryroot; CARGO kann auf die gewünschte Toolchain zeigen.
CARGO=/home/naniadm/.cargo/bin/cargo \
  bash architecture/migration/replays/s14/audit/check.sh

# Zusätzlich tatsächlich geprüfte Mindesttoolchain:
RUSTC=/usr/bin/rustc /usr/bin/cargo test --locked --offline \
  --manifest-path architecture/migration/replays/s14/audit/Cargo.toml \
  --target-dir architecture/migration/replays/s14/audit/target/msrv
```

Keine Rustcompiler/Font-/Binärdateien werden eingecheckt. Die Buildausgaben sind
über die paketlokale `.gitignore` ausgeschlossen.

## Was die Tests wirklich prüfen

Synthetisch konstruierte Inventardaten: strikte Header/Enums/Hashformate und
Größenlimits, fehlende/ungültige Quellenmetadaten, Rechte-/Storageangaben,
Capabilityvollständigkeit, Referenz-/Unit-/Zeitbasis-/Ableitungsnachweise,
Matchdublettenkontrolle, Rawhash-Identitätskonflikte, versionsreine Abdeckung,
Unknown-Behandlung und zeitliche/matchgetrennte Holdout-Metadaten.

Ein zunächst unbekannter Patch darf spätere widersprüchliche Patchangaben nicht
verdecken. Unbekannte Matchidentitäten zählen nicht als angebliche Dubletten.
Halbe Ergebnisse zweier Parserrevisionen dürfen nicht gemeinsam einen vollständigen
Match vortäuschen. Fehlerausgaben enthalten keine eingelesenen Zellwerte/Pfade.
CLI prüft zusätzlich Exitcodes, Hilfe, ungültiges UTF-8, fehlende Dateien,
Verzeichnisse/Symlinks, Manifestgröße und unbekannte Argumente.

Die positiv vollständigen Unit-Testtabellen sind **synthetische Testmetadaten**.
Das Label `real` darin wird nur benutzt, um diese Prüflogik isoliert auszuführen;
kein solcher Testfall steht im echten CORPUS.tsv, und kein Test erzeugt echte
Gameplay-Observations oder bestätigt reale Referenzen.

## Tatsächliche Corpusausgabe

```json
{"audit_version":"s14.inventory.v1","status":"blocked","implementation_authorized":false,"integration_verified":false,"manifest_rows":0,"real_matches":0,"complete_matches":0,"synthetic_cases":0,"repeated_match_rows":0,"missing_capabilities":0,"blockers":["both_holdout_partitions_required","corpus_empty","golden_corpus_below_minimum","holdout_cutoff_missing"]}
```

`missing_capabilities=0` bedeutet bei einem leeren Corpus, dass keine Casezeilen
zu prüfen waren, **nicht** dass Pflichtfelder belegt wären. `corpus_empty` und
`golden_corpus_below_minimum` blockieren explizit den leeren Fall.

## Nicht ausgeführt / keine Behauptung

Kein haste-/valveprotos-Build oder funktionaler Decodervergleich; nur tatsächliche
Manifests und vollständige Gitrevisionen gelesen. Keine realen Replays beschafft,
gehasht, decodiert, quarantiniert oder veröffentlicht. Kein Zugriff auf Produktions-
DB, kein SourceStore-Write, keine Queue-/Worker-/Migrationsintegration, kein neuer
Scheduler, kein Provideraufruf und kein Dienstneustart.

Keine echten Ressourcen-/Dekompressionsbomben-/Crash-/Kill/Reap-Tests; keine
Reparse-/Restore-/Schemawechselprobe mit echten Daten; keine normalisierten Golden-
Observations; kein unabhängiger Feldvergleich; kein echtes Population-/Kernel-E2E;
keine zeitlichen empirischen Holdouts und keine gemischte Last auf Zielhardware.
Dafür gelten die separaten noch offenen Prüfungen im Referenz-/Budgetplan.

Root-Workspace-Format/Clippy/Tests/Releasebuild wurden nicht erneut ausgeführt,
da keine Root-/Runtime-Datei geändert wird. Eine vorhandene GitHub-CI kann diesen
isolierten neuen Pfad noch nicht automatisch abdecken; Anbindung per CR an S02/S10.
CI-Status im PR getrennt von der lokal belegten Suite dokumentieren. S14 bleibt
unvollständig und unintegriert, unabhängig von grünen Vorbereitungschecks.
