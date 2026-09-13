# Lesender Einzel-Ask-Prozessvertrag

Das neue retrieval-Example `ask_latency.rs` ergänzt den unveränderten vollständigen `ask_publish_smoke.rs`. Es startet einen frischen Kindprozess desselben Binaries, lädt über den vorhandenen Infisical-FD ausschließlich `pg_pool_read_only()`, prüft Read-only und ruft genau einmal `ask_context` auf. Optionen wie beim CLI: limit_events=80, include_unverified=false, max_claims=12, game_wiki_dir=None. Frage: „Build für HELD“. Keine Chatnachricht, KI, Persistenz oder Veröffentlichung.

Wie beim Bot endet die Prozesswartefrist nach 20 Sekunden; dann folgen Kill und Reap. Die zusätzlich gemessene Gesamtzeit umfasst Prozessstart, Infisical, Pool, Read-only-Prüfung, DB, Reasoner, JSON-Ausgabe und vollständige Antwortvalidierung. Nur gültige Antworten unter 20 Sekunden bestehen. Der vorherige Binaryhash und das spätere Schreiben der Evidence sind nicht Teil dieser Aufrufzeit. Die bisherigen doppelten Planläufe werden nicht halbiert oder als Botlatenz ausgegeben.

## Ausdrückliche Vergleichsgrenzen

Dies ist **kein Aufruf des produktiven CLI-Binaries**. Das bisherige CLI lädt weitere Settings und verwendet dort `pg_pool()` mit seinem alten DSN-Zugang. Dieses Example erzwingt stattdessen den bereits autorisierten Read-only-FD-Pfad. Kein neuer ENV-/Secretweg wird eingeführt. Root muss den tatsächlichen Releaseaufruf über den bestehenden produktiven Wrapper zusätzlich belegen.

Beide Pipes werden hier auf je 16 MiB begrenzt. Nach Prozessende erhält jeder Leser höchstens eine Sekunde; dann wird sein Task abgebrochen und abgeholt. Der Worker startet keine Nachkommen. Übergröße, unvollständige Pipes, Nichtnull-Exit, Timeout und ungültiges JSON bestehen nicht. Der alte Bot hat diese zusätzlichen Pipegrenzen nicht; diese Abweichung steht ausdrücklich in der Evidence. Stderr-Inhalte und Fehlerdetails aus dem Secret-/DB-Pfad werden nicht ausgegeben oder gespeichert.

Evidence: gestarteter Binary-SHA256, Startzeit, Frist, Gesamtzeit, Prozesswartezeit, Status, Pipelängen/-fehler, Payloadvalidität und vollständiger Ask-Wert. Bei Fehlern wird keine abgeschlossene Ask-Berechnung behauptet. Die Quellrevision wird über den Buildbeleg dieses Binaryhashs zugeordnet, nicht durch unzuverlässiges Runtime-git. Ausgabe wird exklusiv erzeugt. Fehler beim Schreiben werden bereinigt; ein regulärer Timeoutbericht bleibt als negativer Nachweis erhalten.

## Offlineprüfung und späterer Aufruf

```sh
cargo +stable test --locked --offline -p dbrain-retrieval --example ask_latency
cargo +stable clippy --locked --offline -p dbrain-retrieval --example ask_latency -- -D warnings
```

Tests verwenden nur lokale Echo-/Fehler-/Sleep-Prozesse sowie künstliche Pipes: Erfolg, Nichtnull-Exit, Timeout, Übergröße, offen gehaltene Pipe und bestehende Zieldatei. Kein FD oder Datenbankzugriff.

Erst auf gemeinsam freigegebenem Modellstand über den **vorhandenen** Infisical-FD-Launcher:

```sh
/path/to/release/examples/ask_latency /path/to/nachweise/ASK-WARDEN.json Warden
```

Der Parent liest den FD nicht; er vererbt ihn an das Kind. Der bestehende Loader schützt ihn dort mit CLOEXEC. Keine Credentialkopie oder ENV-Ausgabe. Der zusätzliche vollständige Ask-/Publish-Smoke bleibt unverändert erforderlich, ist wegen mehrfacher Planung aber kein Einzelaufruf-Latenztest.

Zum Stand dieses Commits sind weder Live-DB-Messung noch Releasebau des neuen Examples erfolgt.
