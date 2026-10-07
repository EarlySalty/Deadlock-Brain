# K: Bestehende Maintenance-Prüfung

Nativer eigener Prüffixer a92067f4cff648233 abgeschlossen. Kein Produktfix, Commit, Deploy oder fremder Clusterzugriff. K hat die tatsächlichen finalen Test- und Cleanupmarker unabhängig nachgelesen.

| Lauf | passed | failed | ignored | Exit |
| --- | --- | --- | --- | --- |
| Übernommener breiter Ausgangslauf |56|4|0|nicht selbst erhoben |
| Eigener isolierter Erstlauf |59|1|0|101 |
| Eigener vollständiger Wiederholungslauf |106|0|0|0 |

Die vier Ausgangsfehler verschwinden mit bereitgestellter eigener PG-Fixture. Der eigene Erstlauf verdeckte zusätzlich die vorhandene Profil-Testfixture `/tmp/brain-a3-f1-real-profiles.json`; der Wiederholungslauf band diese ausschließlich lesend ein. Keine Fixture-Inhalte in den Bericht übernommen, keine Tests abgeschwächt, gelöscht oder übersprungen. Keine unveränderte Vorhermessung eines älteren Codestands, deshalb keine Altfehlerbehauptung.

## Tatsächlicher Aufruf

Innerhalb eigener Mount-, Netzwerk- und PID-Namespaces mit `bwrap --unshare-all`, geleerter Umgebung und read-only Produktquellen:

```text
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-k-ki-20261007/rust/Cargo.toml --locked --offline --jobs 3 -p brain-storage -p brain-maintenance --lib --test compare_artifact -- --include-ignored --test-threads=1
```

Prüfeinstellungen: `SQLX_OFFLINE=true`, bestehender `CARGO_BUILD_BUILD_DIR=/home/nathanael/.cache/rust-build/{workspace-path-hash}`, ausschließlich isoliertes `BRAIN_CORE_TEST_PG_SOCKET=/tmp/.core-test-pg`. Buildslot3 während beider Läufe gehalten. Dies sind temporäre Prüf-, keine Produktivkonfigurationen.

Der feste Maintenance-Socketpfad zeigte im Namespace nur auf den eigenen neuen Cluster. Host-Inodes unverändert. Beide selbst gestarteten Cluster sauber beendet. Nach Cleanup keine eigenen PID-Dateien oder PG-Sockets. Ausführungs- und Cleanup-Exit0.

## Nachweisorte

Temporärer eigener Prüfroot `/tmp/brain-k-maintenance-proof-20261007-HHjk2HkL/`:

- `tests.log`:60Maintenance+2Maintenance-Artefakt+40Storage+4Storage-Artefakt bestanden, jeweils0 failed/ignored/filtered.
- `cleanup.log`: `maintenance_stop_exit=0`, `core_stop_exit=0`, `final_exit=0` von K selbst nachgelesen.
- `inside.sh` und `fixture.log`: vollständige isolierte Fixture-/Start-/SQL-/Stopmechanik.
- `tests-attempt1.log`, `fixture-attempt1.log`, `cleanup-attempt1.log`: eigener59/1-Zwischenlauf, nicht als unveränderte Baseline ausgeben.

TESTNACHWEIS[TW-1]: 106 passed, 0 ignored | Baseline: nicht gemessen rot

Der Nachweis gilt für den tatsächlich kompilierten Teststand, Abschluss-HEADb310e223, nicht automatisch für spätere Provider-/Artefaktfixänderungen. Der JSONB-Fingerprintblocker ist dadurch nicht widerlegt; seine bislang fehlenden Zahlengrenzfälle gehören in die frische Fixrunde. Kein Live-, echter G-Rechen- oder privater Modellbeweis.
