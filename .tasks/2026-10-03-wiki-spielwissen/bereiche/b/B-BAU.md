status: aktiv
Datum: 2026-10-03

## Schnittstelle

Neue Dateien ausschließlich im B-Eigentum:

- `rust/crates/dbrain-sources/src/game_files.rs`
- `rust/crates/dbrain-sources/src/game_files/kv.rs`
- `rust/crates/dbrain-sources/src/game_files/kv3.rs`
- `rust/crates/dbrain-sources/src/game_files/vpk.rs`

`GameFileOptions` ist per serde serialisierbar. Pflichtwerte: `root`, `app_id`, `source_id`, `observed_at`, `language`, `attribution`, `license_name`, `provenance` als Objekt, `max_file_bytes`. Optionale Werte: `build_id`, `manifest_id`, `source_revision`, `depot_id`, `license_url`.

```rust
pub fn extract_game_files(
    options: &GameFileOptions,
    output: &mut impl std::io::Write,
) -> std::io::Result<GameFileInventory>
```

Der Aufrufer hält die Ausgabe außerhalb Git. Das Modul öffnet keine Datenbank, ruft keinen Anbieter auf und verändert keine Originaldatei. C registriert es in `lib.rs` und der bestehenden CLI. Keine Cargo-Änderung erforderlich.

## Quellenfassung

Für veröffentlichte GameTracking-Dateien wird `source_revision=git:<bestätigter SHA>` angegeben. `build_id`, `manifest_id` und `depot_id` bleiben ohne Beleg null. Dateiversion und SourceRevision gehören in `provenance`. Eine Repository-Revision wird weder als Depotmanifest noch als Steam-Build ausgegeben. `metadata.version_status=source_revision_only` macht diese Grenze sichtbar. Ohne irgendeine belegte Revision folgt `unknown:<content_sha256>`.

## Extraktion

KV1 erhält doppelte Schlüssel, Bedingungen und die ursprünglichen Escape-Bytes. KV3 erhält Objekte, Arrays, doppelte Schlüssel, Zahlen, Bool/Null und Typflags wie `resource_name`. Jede Faktstelle hat einen stabilen Pfad, den Quellwert und bei KV3 das ursprüngliche Lexem. Einheiten bleiben unbekannt. Ressourcenreferenzen und Vererbungen werden nicht als aufgelöste Spielmechanik ausgegeben.

Unbekannte oder unvollständig lesbare Textgrammatik bleibt als ganzes Quelldokument erhalten, mit entsprechendem `parse_status`. UTF-16 mit BOM wird ohne Ersatzzeichen nach UTF-8 überführt; Originalhash und Texthash bleiben getrennt.

VPK-Versionen 1 und 2 werden inventarisiert. Extrahierte reguläre Einträge werden auf Abschnittsgrenzen und CRC32 geprüft. Pfadwechsel aus der Quelle, absolute Pfade und Symlinks werden abgewiesen. Binärassets werden inventarisiert, nicht als ausgeführte Spiellogik ausgegeben. Symlinks und Zugangskonfiguration werden nicht gelesen. `.vpulse`, `.gi`, `.inf`, `.cfg` und `.vdata_inc` bleiben im Textbestand.

## Lokaler Prüfharness

Temporärer Driver: `/home/nathanael/.claude/jobs/010235f3/tmp/b-game-files-extract.rs`.

Aufruf nach erfolgreicher Kompilierung:

```text
/home/nathanael/.claude/jobs/010235f3/tmp/b-game-files-extract OPTIONS.json DOCUMENTS.jsonl INVENTORY.json
```

Der Driver erstellt seine Ausgabedateien ausschließlich neu; bestehende Artefakte werden nicht überschrieben. `OPTIONS.json` ist eine explizite Konfiguration ohne Zugangsdaten oder ENV-Bezug. Der Driver verwendet exakt das neue Modul und keine Kopie davon.

Prüfkommando: `bash /home/nathanael/.claude/jobs/010235f3/tmp/b-game-files-check.sh`. Beide Hostsperren werden vor dem Compiler gehalten, danach folgt eine frische NonZombie-Probe. Der Rust-Compiler 1.97.1 entspricht den vorhandenen Dependency-Artefakten. Diese werden ausschließlich gelesen. Keine Cargo-Manifeste oder gemeinsamen Modulregistrierungen geändert.

Prüfstatus: Rustfmt-Syntax- und Formatprüfung erfolgreich. Der Compiler-/Testlauf wurde wegen belegter Hostsperren nicht begonnen. Auf ausdrücklichen Auftrag des Teil-Orchestrators wurde ausschließlich der eigene wartende Check `blfbecwg6` geordnet beendet. Kein Compiler-/Testnachweis und kein Driver-Binary werden behauptet. Die unabhängige lokale Prüfung verlangt eine frische Fixrunde; die Funde gehen an einen neuen Fixer. Der Bau-Worker verändert keine produktiven Dateien mehr. Quellenstand, API, Driver-Quelle und Prüfwrapper bleiben erhalten. Kein Produktivimport, Commit, Merge oder Deploy durch den Bau-Worker.
