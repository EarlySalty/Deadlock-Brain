status: aktiv
Datum: 2026-10-03

# Paket R: Wiederaufnahme ohne Neubau

Worktree `/home/nathanael/.worktrees/brain-fertig-r`, Branch `feat/brain-fertig-r-20261003`, HEAD `511a347b653beba13c2bf130f4bead7a7196cc2a`. Letztes Ereignis `status/r/1/13.json`, Produzent `teil-r`, Versuch 1. Hauptorchestrator Codex T3 `e6c19079-657e-4db9-80bd-8e1313e7f785`; gemeinsame Integration und Aktivierung bleiben bei Z. Nicht eigenständig nach main mergen oder einen produktiven Releasezeiger umstellen.

## Erhaltener Code und tatsächliche Prüfläufe

Eigene uncommittierte Arbeit im standalone Crate `rust/crates/dbrain-replay/`: normaler Storeadapter `src/import.rs`, Binary `src/import_main.rs`, private Validierung `src/validation.rs`, zugehörige Tests und Crate-Manifest/Lock. Keine gemeinsamen Crates, Migrationen oder Workspace-Manifeste geändert. Der ursprüngliche Schema-Pin-Kommentar im Crate-Manifest ist unverändert wiederhergestellt.

`bqptesexs` endete ohne Abschluss mit `[killed]`; seine zuvor gemeldeten PIDs waren nicht mehr vorhanden. Den erhaltenen Prüfschritt einmal in der Hauptsession fortgesetzt: `b3xfso49t` hat tatsächlich Exit 0. Format und Clippy bestanden. Normale Suite 70 passed, 0 failed, 2 ignored; zusätzlich echter ignorierter Postgres-Test 1 passed, 0 failed, 0 ignored. Eine direkt durch vier Sandboxtests verwendete Subprozessfixture bleibt ignored. Keine Altfehler-Baseline behauptet.

Logs `/tmp/brain-replay-r-core-checks-20261003/{fmt,clippy,tests,postgres}.log`. Dieser Erfolg gilt vor der anschließenden Headerkorrektur, nicht als aktueller Prüfabschluss nach weiteren Edits.

Der erste echte Decode danach scheiterte mit `InvalidContainer`. Lokale begrenzte Headerprüfung belegt den gültigen Stempel `PBDEMS2` plus genau ein NUL. `src/decode.rs:103` akzeptiert bisher nur `PBDEMS2` ohne NUL. Ein einziger nativer Sol-Coder hat die eng begrenzte Korrektur und zwei zusätzliche Grenzfalltests umgesetzt. `bdwa4urg5` ist nach tatsächlichem Sperrenerwerb mit Exit 0 beendet: Format, Clippy, 72 reguläre Tests sowie 1 echter Postgres-Test bestanden; eine direkte Sandbox-Subprozessfixture bleibt ignored. Logs `/tmp/brain-replay-r-stamp-check-20261003-jGme6c/`.

Der erneute echte Decode scheitert danach mit `UnknownStructure`. Lokal eingegrenzt auf die Game-Directory-Prüfung: Der aufgezeichnete absolute Pfad endet auf `citadel`, die bisherige Prüfung verlangt nur das nackte Literal. Zusätzlich enthält die echte Demo `DemStringTables`, den der aktuelle Adapter ausdrücklich nicht unterstützt. Kein gültiger Report oder realer Import.

Nach seinem vollständigen nativen Abschluss ohne Hintergrundkinder wurde derselbe Coder gezielt fortgesetzt. Auftrag: legitime Verzeichnisformen eng prüfen und nur den Spielbezeichner, nie den privaten Originalpfad speichern; die tatsächliche Rolle von `DemStringTables` anhand struktureller Daten und bestehender gepinnter Backend-APIs klären, vorhandenen Zustandspfad wiederverwenden, niemals einen zustandsändernden Befehl einfach überspringen. Keine neue Decoderimplementierung, keine gemeinsamen Dateien oder eigenmächtiger Pinwechsel. Native Worker-ID: `a77e9d2a20d110dc4`. Keine Live-Nachricht schicken; dies startete früher parallele eigene Läufe. Bei Wiederaufnahme tatsächlichen laufenden Zustand prüfen, keinen Doppelstart und keine Rücksetzung.

## Echte Eingabe

Komprimierte Demo `/tmp/brain-replay-r-20261003/public-demo-110001910-worker/110001910.compressed`, 45.014.647 Bytes, SHA `3fe2095e66c4d6c1b8ac2ac58898dd84662c6e970d373556c4d2cd1566d77443`, Zstd trotz `.bz2`-Endung. Entpackt mit geprüftem temporären Rust-Werkzeug, 17 Tests. Raw `/tmp/brain-replay-r-20261003/unpack-rust/raw/public.dem`, 70.861.451 Bytes, `PBDEMS2`, SHA `0cc5982dcfac2da1396308bf1e259d9d76fa0e2c46d5fd2a6548a26b1568056a`. Privater gepinnter Request daneben `request.json`, beide Dateien 0600, Verzeichnis 0700.

30 gespeicherte Match-Salts ohne GC-Fallback geprüft, eine Demo vorhanden. Quellenbelege `HTTP-BESCHAFFUNG.md`, Entpackbelege `RAW-WERKZEUG.md`. Noch kein erfolgreicher echter Decode oder Import. Keine unabhängige Gameplay-Feldreferenz. Raw und komprimierte Datei nach tatsächlichem Decode-/Wiederholungsbeweis löschen, nur Quellen-/Versions-/SHA- und aggregierte Beweise behalten.

## Eigener Testcluster

DSN `postgresql:///brain_replay_test?host=/tmp/brain-replay-r-core-20261003/pg&port=55439&user=nathanael`. Ausschließlich Unix-Socket, Peer-Auth, kein TCP, keine Passwörter. Beim letzten tatsächlichen Test lief der Cluster. Nur diesen eigenen Cluster verwenden. Am passenden Ende stoppen mit `/usr/lib/postgresql/16/bin/pg_ctl -D /tmp/brain-replay-r-core-20261003/pg -m fast -w stop`.

## Verbleibende Beweise

1. Laufenden nativen Coder erhalten, aktuelle Format-/Clippy-/Testabgabe abwarten. Compiler nur unter beiden Sperren laut HOSTPROBE.md, höchstens zwei Jobs, kein Timeout beim reinen Sperrenwarten.
2. Echte DEM mit aktuellem Binary dekodieren, nur aggregierte Daten ausgeben. Fehler nicht durch gelockerte Prüfung oder erfundene Semantik umgehen.
3. `dbrain-replay-import import <raw> <request> <worker> --peer-test <socket> 55439 brain_replay_test` zweimal in getrennten Prozessen ausführen. Danach `query <release> private:replay-r-20261003 --peer-test <socket> 55439 brain_replay_test`. Quellen-/Versionsbeleg und dauerhafte Idempotenz prüfen, Veröffentlichung nicht erlauben.
4. Frische native Intent-Abnahme gegen Auftrag/Briefing, danach lokales Gate `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review`. Verifizierten Stand mit Beweisen an Z übergeben; gekoppelte Abnahme vor gemeinsamer Integration. Git-Schritte einzeln, keine fremden Dateien stagen.
5. `UEBERGABE.md` ehrlich aktualisieren. Status spätestens alle 20 Minuten, jedes Mal `VON_HAUPT.md` prüfen. Kein `TODO.md` oder `REGISTER.md`. Selbst-settle ausschließlich bei wirklichem erlaubtem Abschluss, nicht während offener Integration.

PR #46 ist mit dem T3-Thread verknüpft. Kein neuer PR. Alte PR-/Branch-Aufräumarbeiten gehören Z.
