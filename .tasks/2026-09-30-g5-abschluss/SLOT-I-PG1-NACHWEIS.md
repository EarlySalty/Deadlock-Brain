status: erledigt, PG1 tatsächlich grün und Cleanup belegt; PG2 nicht gestartet
Datum: 2026-09-30

# PG1: isolierter Upgrade-/Restore-/Minimalrollenbeweis

Vor Start sauberer und upstreamgleicher Headc5951b610aa2545d2c0b43b33b5fe1906198b292 geprüft. Eigene ausdrückliche Zuteilung nachU2, Nutzer meldet8,06GiB verfügbar neben einem getrennten Twitch-Release. Vorhandene Grenzen unverändert, kein neuer Cache.

```sh
bash /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/scripts/test_brain_storage_upgrade.sh /home/nathanael/.cargo/bin/cargo /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target
```

Sessionb23bbb03-c6d0-4d44-b3e3-99631ddd89e3, Harnessb77f29612, PID1761901, Start2026-09-30T16:26:57Z. Tatsächlicher Exit0 aus Completion und Harnessoutput bestätigt. Slot unmittelbar vor Loganalyse zurückgemeldet und zentrale Statusdatei aktualisiert. Ein Cargo-Aufruf mit locked/offline/jobs1 und ein existierender gezielter Test. Nur privater temporärer Unix-Cluster Port55441/max_connections24/shared_buffers16MB; keine produktive Datenbank, Dienste, Imports, Serve oder Last.

## Tatsächliches Ergebnis

```text
server started
Finished `test` profile [unoptimized + debuginfo] target(s) in 13.11s
PASS DDL-free service preflight/read/write, native reader, checkpoint resume/replay, concurrent upgrade idempotence and stale-writer fencing
PASS actual pg_dump/pg_restore to a fresh v1 database: post-cutover writes excluded, restorepoint and release/ACL semantics exact
test v1_upgrade_restore_and_least_privilege ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 7.09s
waiting for server to shut down.... done
server stopped
```

Das Testwort replay meint Checkpoint-Wiederholung, keine zurückgestellten Spiel-Replays. Der gefilterte separate Fixturehash-Test wird vom ausgeführten Testkörper selbst aufgerufen; kein zusätzlicher Lauf.

## Log und Cleanup

- Äußerer Log mit umask077 erzeugt, Modus0600 tatsächlich geprüft. Pfad /home/nathanael/Documents/.tasks/2026-09-30-twitch-alles-live/brain-g5-pg1-c5951b6-slot-20260930.log.
-1142Bytes, SHA256716de6827349f368f3ff416afe2ae021a27c10228ce5d5f4042b5ca919fa5b9b.
- Harnessoutput /tmp/claude-1000/-home-nathanael-Documents/b23bbb03-c6d0-4d44-b3e3-99631ddd89e3/tasks/b77f29612.output enthält Exit0.
- Cleanupabschluss im äußeren Log mit `server stopped` belegt. Anschließende eigene Prüfung: /proc/1761901 nicht mehr vorhanden; find /tmp auf eigene brain-c11.*-Verzeichnisse liefert keine Treffer. Kein manueller Lösch- oder Stopbefehl, kein fremder Prozess angefasst.
- Der konkrete zufällige Scratchname wurde vor Cleanup nicht separat erfasst. Der Runnerlog plus tatsächlicher Exit und leere Nachkontrolle belegen den Abschluss, nicht einen erfundenen Pfad.

Dieser Fixturebeweis ist kein aktueller Produktionsbackup-/Restorebeleg und kein realer Minimalrollen-Secret-Exec-Login. Er ersetzt auch nicht den neuen B2-Publikationsrace mit Tombstone/Scopewriter. PG2 bleibt gesondert anzufordern: bestehender test_brain_serve.sh mit demselben Targetcache, sieben serielle Cargo-Aufrufe einschließlich1.800 Lastanfragen. Kein automatischer Folgelauf.
