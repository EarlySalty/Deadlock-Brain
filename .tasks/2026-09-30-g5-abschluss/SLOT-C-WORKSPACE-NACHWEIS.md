status: erledigt, zugeteilter Standardlauf bestanden; G5 bleibt offen
Datum: 2026-09-30

# Workspace-Test auf dem V1-Stand ohne Replay

## Bindung und tatsächlicher Lauf

- Quellworktree: `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`, Branch `fix/g5-replay-deferred-20260930`.
- Tatsächlicher HEAD: `ca4a8f236a75ba89ce60d2140c735acd5d1f5bee`, davor und danach sauber. Gegen Produkt-/Lockbasis `9a29b81d230c01e5c03423cc34ba34c1074eab69` ausschließlich die Berichtsdatei WORKSPACE-TESTVORBEREITUNG.md geändert.
- Integrator teilte genau den statisch geprüften lokalen Prozess-/Mocklauf zu. Start `2026-09-30T11:08:51Z`, PID `3794497`, Hintergrundauftrag `bi8uvp8u8`.
- Tatsächlicher Harnessabschluss: `completed`, Exitcode **0**. Cargo per `exec`, keine Pipeline zur Maskierung des Exitcodes.
- Compiler-Slot mit Eingang der Abschlussmeldung sofort zentral zurückgegeben. Nutzer bestätigte die Übernahme und Übergabe an den Integrator für dessen vier Twitch-Releasebinaries. Kein weiterer Cargo-Aufruf.

Arbeitsverzeichnis: Quellworktree `/rust`.

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 test --workspace --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- --test-threads=1
```

Vorhandener Targetcache, vor Start 193 GiB frei. Kein `--ignored`, keine neue ENV-Konfiguration, kein automatischer Folgelauf. Die angemeldeten lokalen Testbinaries, Kindprozesse, Loopback-/Unix-Socket-Mocks und temporären Dateien gehören zum Lauf. Der Completion-Test verwendet Cargo-/Rust-/Probe-Stubs; dessen zusätzliche `rust/target/wiki-completion.*`-Dateien sind Berichte, kein zweiter Compiler-Cache.

## Ergebnis aus dem vollständigen Log

**954 passed, 0 failed, 74 ignored, 0 measured, 0 filtered out.** 91 Ergebnisblöcke einschließlich leerer Ziele. 74 einzeln protokollierte Ignorezeilen stimmen mit den Summen überein. Keine Compilerfehler oder FAILED-Testzeilen gefunden. Cargo meldet `Finished test profile ... in 5m 31s`; das ist nicht die gesamte gemessene Prozessdauer.

| Paket | Tatsächlich ignoriert |
| --- | ---: |
| brain-feeds | 1 |
| brain-legacy-import | 2 |
| brain-serve | 6 |
| brain-storage | 2 |
| dbrain-builds | 5 |
| dbrain-enrich | 3 |
| dbrain-learn | 5 |
| dbrain-normalize | 7 |
| dbrain-reasoner | 16 |
| dbrain-retrieval | 15 |
| dbrain-sources | 7 |
| deadlock-brain-yt | 5 |
| **Summe** | **74** |

Die konkreten ignorierten Namen und Voraussetzungen stehen in WORKSPACE-TESTVORBEREITUNG.md auf Quellcommit ca4a8f2. DB-/SCRAM-/Restore-/Pilot-/Fencing-, echte lokale Wiki-/Snapshot- und öffentliche Livefälle sind **nicht bestanden**, sondern nicht ausgeführt. Keine pauschale spätere `--ignored`-Freigabe. Replay ist ausdrücklich außerhalb dieses V1-Workspace und nicht Teil dieser Zählung.

Vollständiger lokaler Log, restriktive Dateimaske, nicht auf Git hochgeladen:
`/home/nathanael/Documents/.tasks/2026-09-30-twitch-alles-live/brain-g5-workspace-test-ca4a8f2.log`

116489 Bytes. SHA256: `87f607c2796023977bc5d29e59f028a33ef66ca5ca36ffdb16fe1ec0334f2f6e`.

## Nachweisgrenze

Dies belegt den tatsächlichen Standardtest mit vorhandenem Cache. Kein frischer Quellenfetch, kein Releaseartefakt, kein produktiver DB-/Provider-/Servebeweis, keine Migration und kein Cutover. Die statische Nebenwirkungsprüfung wird nicht zu einer lückenlosen Laufzeit-Netzwerkmessung erklärt. Keine Discord-/Twitch-Nachricht und kein Steam-Build veröffentlicht. G5 wird anhand der bereits vorhandenen echten Nachweise weiter vorbereitet, nicht aufgrund dieses Standardlaufs als abgeschlossen markiert.

TESTNACHWEIS[TW-1]: 954 passed, 74 ignored | Baseline: zugeteilter serieller Workspace-Standard auf Produktbasis 9a29b81, Exit 0; ignorierte Fälle getrennt offen
