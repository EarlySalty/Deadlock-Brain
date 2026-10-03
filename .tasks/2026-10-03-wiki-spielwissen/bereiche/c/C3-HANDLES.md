status: aktiv
Datum: 2026-10-03

# C3: Handle-Binder, Quellenstand

Nativer eigener Worker a6bcbc2e101688d36 regulär beendet, kein lebender Quellwriter. Genau fünf neue eigene Dateien: steam_game_input.rs, steam_game_input/inventory.rs, filesystem.rs, output_tests.rs und tests/steam_game_input.rs. Keine B-/D-/Cargo-/lib-/CLI-/Readeränderung. 17 vorbereitete Tests, keine Test-/Compiler-/Cargo-/Produktivläufe oder Commits.

Geschriebene API: SteamGameInputOptions und limits mit serde Serialize/Deserialize/deny_unknown_fields, Inventarpins als striktes64-Zeichen-Hex und intern[32]. from_file(path,max_bytes) liest begrenzt ohne Symlink/Hardlink und mit Identitätsbindung. prepare_steam_game_input(&options,GameFileOptions) erzeugt PreparedSteamGameInput, dessen konsumierendes extract_to(artifact_directory) nach Extraktorerfolg genau ein privates NOREPLACE-Artefaktverzeichnis mit documents.jsonl und evidence.json veröffentlicht. Erfolgsfelder directory/jsonl_path/evidence_path und extractor_inventory. Bei Err kein gültiger Artefaktpfad oder Teilimport. Alle Optionen/Limits und dünne CLI-Anbindung sind an denselben CLI-Worker vermittelt.

Vollständige gepinnte Gesamt-/Depotbelege mit Root/App/Build/Depot/Manifest werden typisiert gebunden. Hashstrom128KiB prüft tatsächliche Größe, SHA1/SHA256; vollständiger verankerter Bestand vor/nach Kopie, Extras/Typ-/Inode-/Hardlink-/Rootwechsel fail-closed. Echte Disk- und RLIMIT_NOFILE-Prüfung vor Sicherung, Speicherprüfung je Datei. Keine Depot-RAMvollkopie. D-Manifesttransporthash bleibt D-belegt, kein unabhängiger Abruf.

## Tatsächliche Snapshot-Übergabe an B

Private Sicherung schreiben/syncen, chmod0400, eigenständigen O_RDONLY/NOFOLLOW-Deskriptor öffnen und Inode prüfen, Schreibdeskriptor schließen, privaten Namen entlinken. B erhält konsumierend ausschließlich das gehaltene reguläre read-only File mit nlink0. Keine Originalnamensöffnung, /proc-Reopen oder parallel benutzten Aliasdeskriptoren. Der B-Zusatz muss reguläre unverlinkte Dateien akzeptieren; dies ist kein neuer Parser oder APIwechsel. B schreibt ausschließlich seine beiden zugewiesenen Pfade, C keinen davon. Physische Referenzen und Ressourcenhash bleiben getrennt.

## Gemeldete Verfahrensabweichung

ABWEICHUNG: Direkter rustfmt auf genau fünf eigene Dateien lief ohne beide Hostlocks und frische NonZombie-Probe. Toolaufruf erfolgreich, numerischer Exit nicht separat ausgegeben, kein FD-/Lockbeleg. Parent hat dies ausdrücklich geklärt, nicht als gültigen Rustprüfbeweis übernommen. Kein neuer Lauf nur wegen der Meldung. Die ohnehin nötige gemeinsame Format-/Compiler-/Clippy-/Testprüfung muss regulär unter beiden Locks am gesamten eingefrorenen Endstand laufen.

Tatsächlicher Befehl: /home/nathanael/.cargo/bin/rustfmt --edition 2021 mit den fünf oben benannten absoluten eigenen Dateipfaden. Keine Fremdquelle formatiert. Diese Abweichung wird Root genannt und nicht verschwiegen; sie ist kein Cargo-/Testgrün und keine Umgehung zur Freigabe.

## Offen

B-Zusatzcommit, gemeinsamer eingefrorener Prüflauf, gekoppelte Numeric-/Inventar-/VPK-Abnahme und reale Depotdaten nach regulärem Steam-Deploy. Keine echte Steamabnahme aus den vorbereiteten Datei-I/O-/Hash-/Limits-/Pfad-/Race-/Readonly-/VPK-/Teiloutputtests ableiten.
