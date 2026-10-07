status: aktiv
Datum: 2026-10-03

# Paket R: Prüflauf grün, echte Demo deckt Headerfehler auf

Worktree `/home/nathanael/.worktrees/brain-fertig-r`, Branch `feat/brain-fertig-r-20261003`, HEAD `511a347b653beba13c2bf130f4bead7a7196cc2a`. Keine Commits, Merges oder Deploys. Übernahme durch Codex und Antwort einschließlich Laufzeitbefund 16:37 UTC gelesen. Gemeinsame Integration und produktive Aktivierung bleiben bei Z.

## Tatsächliche Prüfung

Der am erhaltenen Stand einmal fortgesetzte Prüflauf `b3xfso49t` ist mit Exit 0 abgeschlossen. Beide vorgeschriebenen Host-Sperren, frische Compilerprobe und höchstens zwei Jobs wurden verwendet. Formatprüfung und Clippy `--all-targets -- -D warnings` bestanden. Normale Suite: 70 passed, 0 failed, 2 ignored. Der ausdrücklich ausgewählte echte Postgres-Test lief zusätzlich: 1 passed, 0 failed, 0 ignored, 3 filtered out, Laufzeit 27,11 Sekunden. Ein verbleibendes ignored ist die von vier Sandbox-/Watchdogtests direkt gestartete Subprozessfixture, keine ausgelassene Funktionsprüfung.

TESTNACHWEIS[TW-1]: 71 passed, 1 ignored | Baseline: nicht erhoben rot

Logs: `/tmp/brain-replay-r-core-checks-20261003/{fmt,clippy,tests,postgres}.log`. Das ist der geprüfte Adapter-/CLI-Stand vor der folgenden Headerkorrektur, noch kein erfolgreicher Decode der echten Demo.

## Echte Eingabe und belegter Fehler

30 unterschiedliche gespeicherte Match-Salts wurden ohne Steam-/GC-Fallback geprüft. Eine Demo verfügbar, 29 ohne Demo-URL. Vollständig entpackte Demo `110001910`: 70.861.451 Bytes, Raw-SHA `0cc5982dcfac2da1396308bf1e259d9d76fa0e2c46d5fd2a6548a26b1568056a`. Die vorhandenen Beschaffungsbelege liegen in `HTTP-BESCHAFFUNG.md` und `RAW-WERKZEUG.md`.

Der erste echte Decoderlauf endete mit Exit 2 und `InvalidContainer`. Die begrenzte lokale Headerprüfung belegte: korrektes Container-Magic, erster Befehl `DemFileHeader`, protobuf-Stempel exakt `PBDEMS2` plus ein abschließendes NUL, Länge 8. `src/decode.rs:103` akzeptierte nur die nicht terminierte Variante. Kein beschädigter Download behauptet.

Diese eng begrenzte Korrektur ist jetzt tatsächlich geprüft. Der erhaltene einzelne Sperrenwarteprozess `bdwa4urg5` erhielt die Sperren und endete mit Exit 0. Format, Clippy, reguläre Suite, echter Postgres-Test und Binarybau bestanden. Reguläre Suite 72 passed, 0 failed, 2 ignored; zusätzlich echter Postgres-Test 1 passed, 0 failed, 0 ignored, 3 filtered out, 21,54 Sekunden.

TESTNACHWEIS[TW-1]: 73 passed, 1 ignored | Baseline: nicht erhoben rot

Aktuelle Logs `/tmp/brain-replay-r-stamp-check-20261003-jGme6c/`. Der erneute echte Decode passiert den korrigierten Stempel, scheitert danach mit `UnknownStructure`, Exit 2 und leerer Reportdatei. Noch keine erfolgreiche reale Dekodierung. Der gleiche automatisch fortgesetzte native Implementierer diagnostiziert diese weitere Ablehnung lokal. Kein neuer Schreiber, keine parallele Fehlersuche und keine Lockerung der Schutzprüfungen. Der originale Schema-Pin-Kommentar bleibt erhalten; Rohdaten werden nicht an Modelle ausgegeben.

## Nächster konkreter Beweis

Nach der aktuell noch laufenden Korrektur: echter Import über den vorhandenen SourceRecordV2-/Lease-/Releasepfad in den eigenen Peer-Testcluster, Wiederholung aus getrenntem CLI-Prozess, interne Abfrage mit Quelle und Version. Publikationsrechte bleiben unverändert. Danach frische Intent-Abnahme und Merge-Gate. Noch kein erfolgreicher echter Replay-Import und kein produktiver Abschluss.

Letztes veröffentlichtes Ereignis: `status/r/1/13.json`.
