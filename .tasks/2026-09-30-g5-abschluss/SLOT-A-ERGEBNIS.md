status: erledigt
Datum: 2026-09-30

# Slot A: statische V1-Abtrennung von Replay

Arbeitszweig: `fix/g5-replay-deferred-20260930` im Worktree `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`.
Ausgangs-HEAD: `1c362bca6d35e7fec10125b2b159e7513a299243`.
Quellvorbereitungs-Commit: `86a0bd6e52e5704db2d50901c918db02b4b3eb64`.
Dieser Bericht wird im nachfolgenden eigenen Berichts-Commit ergänzt; dessen SHA steht in der Git-Historie des Zweigs.

## Geänderte Dateien und Grenze

- `rust/Cargo.toml`: Replay aus den Workspace-Mitgliedern entfernt und explizit ausgeschlossen. Die 24 übrigen Mitglieder bleiben unverändert.
- `rust/crates/dbrain-replay/Cargo.toml`: eigenständigen Workspace angelegt; Version `0.1.0`, bisherige Dependency-Anforderungen und Serde-Feature `derive` explizit übernommen. Die Git-Revisionen von Haste und valveprotos sowie die festen Paketpins blieben unverändert.
- `rust/crates/dbrain-replay/Cargo.lock`: den ursprünglichen Root-Lockstand bytegleich als eigenständigen Ausgangsstand kopiert. `rust/Cargo.lock` blieb unverändert.
- `rust/crates/dbrain-replay/src/lib.rs`: Parser-Fingerprint liest jetzt den Replay-Lockstand. Quellen und Tests blieben bestehen.
- `architecture/migration/replays/s14/check-decoder.sh`: verwendet das eigenständige Replay-Manifest; Debug-, Release- und Real-Corpus-Grenzprüfungen bleiben im Skript.
- `.github/workflows/ci.yml` und `.github/workflows/rust-core-verification.yml`: Replaylauf und dessen nur dafür benötigte Compilerinstallation aus V1 entfernt. Die zweite Matrix erwartet jetzt zehn statt elf Artefakte und meldet Replay ausdrücklich als nicht geprüft.
- `rust/crates/dbrain-replay/README.md`: aktuelle Trennung und spätere separate Befehle dokumentiert, frühere Prüfaussagen als historischen Stand kenntlich gemacht.

Die Replay-Quellen, synthetischen Tests, der Worker, die Rechte- und Ressourcenprüfungen und der gesperrte Nachweis für einen echten Replay bleiben erhalten. Kein neuer Parser, Produktvertrag, Anbieter oder Dienst wurde eingeführt. Haste bleibt bei `bfb292d4798031350861ad297aa26753267a1ea6`, valveprotos bei `4f4a3cb1b0c6f19af59a722acb79ecccd01f61f6`, die dokumentierte transitive Dungers-Revision bei `5e1e2aac76a027987911de3ef3d23ecfd992a7fb`.

## Statischer Nachweis und offene Lock-Auflösung

Die read-only Analyse des bisherigen vollständigen Lockgraphen ergab 25 ursprüngliche lokale Wurzeln und 24 V1-Wurzeln ohne Replay. Von 427 Paketen waren 386 von V1-Wurzeln erreichbar und 41 nur wegen Replay vorhanden. Die separate Replay-Wurzel erreichte 325 Pakete. Alle 1.289 betrachteten Abhängigkeitskanten wurden eindeutig aufgelöst; keine Git-Quelle war von den 24 V1-Wurzeln erreichbar. Das ist eine statische Auswertung des vorhandenen Lockstands, kein Cargo-Metadatenergebnis und kein Buildbeweis.

**Noch offen:** `rust/Cargo.lock` enthält weiterhin alle 427 alten Einträge einschließlich der jetzt unerreichbaren Git-Quellen. Der kopierte Replay-Lockstand enthält ebenso noch die bisherigen V1-Einträge. Zwei statische Schreibversuche zur reinen Bereinigung wurden durch die Worktree-Isolation abgelehnt und haben den Root-Lockstand nicht verändert. Nach der anfänglich zurückgesetzten Shell-cwd wurde die Sitzung an `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930` gebunden; dies war die tatsächliche Laufzeit-cwd bei den abgelehnten Versuchen. Zielpfade waren `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/Cargo.lock` und beim ersten Versuch auch `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust/crates/dbrain-replay/Cargo.lock`. Kurze Guardmeldung: „this command is too complex to verify that it stays inside the worktree“. Keine Umgehung, kein Schreiben durch die abgewiesenen Befehle. Daher wurden keine Lockeinträge geraten oder Prüfungen als erfolgreich ausgegeben. Vor einem V1-Lauf mit `--locked` muss Cargo im zugeteilten Build-Slot die Root-Auflösung aktualisieren; vor einem separaten Replay-Lauf muss das Gleiche für dessen Lockstand geschehen. Die Revisionen, Quellen und Checksummen sind bis dahin unverändert erhalten.

Erlaubte tatsächliche Prüfungen: Graphify-Abfrage des globalen Graphen vor den gezielten Dateilesevorgängen; read-only Abhängigkeitsgraphanalyse mit `tomllib`; TOML-Parsing beider Manifeste und Lockdateien (24 Mitglieder, 427 Einträge pro aktuellem Lockstand, bytegleiche Kopie); YAML-Parsing beider Workflows (drei beziehungsweise vier Jobs); `bash -n architecture/migration/replays/s14/check-decoder.sh`; `git diff --check` und `git diff --cached --check`; gezielte Diff- und Statussichtung. Der Quellvorbereitungs-Commit wurde angelegt. **Nicht ausgeführt:** Cargo, rustc, rustfmt, Clippy, Tests, Benchmarks, Build, Dienststart, Live-Prüfung, Replay-Download, Git-Fetch und Merge. Bestanden sind deshalb null neu ausgeführte Tests; ausgelassene Replaytests zählen nicht als bestanden.

## Nicht ausgeführte Vorschläge für einen später zugeteilten Build-Slot

Aus dem Root dieses Worktrees, erst nach gesonderter Zuteilung und unter Beachtung des lokalen Merge-Gates. Zuerst die bisherige Root-Auflösung ohne `--locked` offline aktualisieren, danach ausschließlich mit `--locked --offline` prüfen:

```sh
cargo +1.97.1 metadata --manifest-path rust/Cargo.toml --format-version 1 --offline > /dev/null
git diff -- rust/Cargo.lock
cargo +1.97.1 metadata --manifest-path rust/Cargo.toml --format-version 1 --locked --offline > /dev/null
cargo +1.97.1 check --manifest-path rust/Cargo.toml --workspace --all-targets --locked --offline --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target --jobs 1
```

Die Root-Lockänderung und die erreichte V1-Abhängigkeitsmenge müssen anschließend durch Cargo-Metadaten und einen echten Build verifiziert werden. Ein `--locked`-Fehler vor der Aktualisierung ist kein V1-Buildnachweis. Die nötige SQLx-Konfiguration ist für diesen Vorschlag nicht geklärt; hier wird weder eine Einstellung vorausgesetzt noch ein neuer Konfigurationsweg eingeführt. Der angegebene Targetcache wurde in Slot A nicht beschrieben.

Replay bleibt zurückgestellt. Nur bei späterer eigenständiger Replay-Zuteilung sind dies ebenfalls **nicht ausgeführte Vorschläge**, keine abgeschlossene Decoderprüfung:

```sh
cargo +1.97.1 metadata --manifest-path rust/crates/dbrain-replay/Cargo.toml --format-version 1 --offline > /dev/null
git diff -- rust/crates/dbrain-replay/Cargo.lock
cargo +1.97.1 metadata --manifest-path rust/crates/dbrain-replay/Cargo.toml --format-version 1 --locked --offline > /dev/null
cargo +1.97.1 check --manifest-path rust/crates/dbrain-replay/Cargo.toml --all-targets --locked --offline --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target --jobs 1
```

Ein späterer Lauf von `architecture/migration/replays/s14/check-decoder.sh` benötigt eine eigene Zuteilung und seine eigene Prüfung. Die vorgeschlagene separate Prüfung ersetzt keinen echten autorisierten Replay-Nachweis. Die unabhängige statische Abnahme dieses Slots liegt bei der Hauptsession; es erfolgte weder Merge noch Deploy.
