status: erledigt, begrenzte rein lesende Prozesszuordnung; keine Laufzeitaktion
Datum: 2026-09-30

# PID1280937 ist der bestehende Build-Datenrefresh, kein G5-Start

## Beobachtete Zuordnung

- PID1280937 deadlock-brain, Parent Bash1280909, danach Usermanager933. UID/GID1000, vier Threads, rund61.040KiB RSS in der ersten Stichprobe.
- Beide Prozesse liegen laut /proc/cgroup in deadlock-brain-build-data.service unter user@1000.service/app.slice. Arbeitsverzeichnis /home/nathanael/.worktrees/brain-live-main; tatsächliches ELF /home/nathanael/.worktrees/brain-live-main/rust/target/release/deadlock-brain.
- systemctl --user show mit ausgewählten sicheren Properties: bestehende Unit, Type=oneshot, MainPID1280909, activating/start, NRestarts0, Start30.09./15:11:31UTC, InvocationID37e689519e194c3f8f24b8c064ae5447.
- Zugehöriger vorhandener Timer deadlock-brain-build-data.timer ist aktiv. Kalender täglich03:30 Europe/Berlin, letzte Timer-Auslösung30.09./01:30:04UTC, nächste01.10./01:30UTC. Die neue Ausführung15:11UTC ist damit nicht als reguläre heutige Timer-Auslösung belegt. Wer sie ausgelöst hat, geht aus diesen sicheren Metadaten nicht hervor. Keine Behauptung einer bestimmten fremden Sitzung oder Nutzerhandlung.

## Belegte Laufwirkung und Grenzen

PID-eigene Socket-Inodes wurden ohne Argumente, ENV oder Payloads den Netzmetadaten zugeordnet: vier bestehende TCP-Verbindungen zu lokalem Port5432 und eine bestehende externe HTTPS-Verbindung auf443. In dieser Stichprobe kein TCP-Listener des Prozesses. Das ist ein laufender Netzwerk-/DB-Refresh, kein Compilerlauf. Erfolgreiche Writes oder ein abgeschlossener Refresh sind dadurch nicht bewiesen.

Der vorhandene Wrapper scripts/run_build_data_with_infisical.sh:62-67 im zugehörigen Checkout startet nacheinander pull build-data, population sync und population stats. Der statisch gelesene CLI-Pfad main.rs:2237 bindet build-data an dbrain_builds::sync_build_data. Dessen bestehender Source sync.rs enthält Schreibpfade für brain.item_catalog, brain.hero_catalog, brain.hero_item_stats, brain.hero_ability_orders und brain.hero_item_synergies, einschließlich Bereinigung. Deshalb ist die Laufklasse nicht als read-only zu behandeln. Welche dieser Phasen gerade läuft und welche Datensätze sie tatsächlich verändert hat, wurde ohne Prozessargumente/Querytexte nicht behauptet.

Der Checkout ist sauber auf main25c6ed6951370b092f60c67a35bdbe37440ece5d, Stand25.09. Dieser Gitstand allein beweist nicht, aus welchem Commit das laufende ELF gebaut wurde. Der gelesene Wrapper ist eine vorhandene Legacy-Referenz, keine neu erstellte oder übernommene ENV-/Secretkonfiguration.

## Nachtrag aus Nutzermeldung und Gitgegenprüfung

Der Nutzer hat die Zuordnung inzwischen über Intent662fd521 bestätigt: PaketB b5f29fd9 reparierte den vorhandenen Build-Daten-Timer, PaketD fasst Brain nicht an. Das ergänzt die sichere Prozesszuordnung um die vom Nutzer übermittelte Auftragsherkunft; diese Session hat keine fremde Sitzung kontaktiert. Kein G5-Abschluss daraus ableiten.

Den genannten Commit fd6e157622e283cf4ff007c3c355348681c20e99 gibt es im gemeinsamen Objektbestand. Die tatsächliche Diffstat umfasst allerdings neben den zwei Unitdateien auch140 geänderte Zeilen in rust/crates/dbrain-builds/src/api.rs für Retry/Backoff. Bei der eigenen Gegenprüfung zeigte origin/main per ls-remote weiterhin25c6ed6951370b092f60c67a35bdbe37440ece5d, ebenso brain-live-main; fd6e157 war auf fix/paket-b-timer-20260930 enthalten. Daher weder bereits erfolgten Main-Merge noch Unit-only-Diff behaupten. Bei der späteren eigenen Integration den dann tatsächlichen Mainstand erneut binden und diesen Timer-/API-Eigenanteil erhalten; nichts zurücksetzen oder fremd mergen.

## Abgrenzung zur G5-Instanz

Zusätzliche begrenzte Transaktion15:15:15UTC gegen die bestehende dedizierte Brain-Instanz /run/deadlock-brain-postgresql:5446 als vorhandene Rolle brain_migrate: REPEATABLE READ READ ONLY, statement_timeout2s, lock_timeout1s, Exit0 und ROLLBACK. Zielbrain weiterhin ohne Source-Heads, Revisionen oder CorpusRelease. Außer der eigenen Probe keine weiteren Clientdatenbanken in der pg_stat_activity-Stichprobe sichtbar; keine Querytexte gelesen. Das ist eine Zeitpunktbeobachtung, kein umfassender historischer Schreibbeweis.

Unser G5-Autor hat ausschließlich Testquellen8949198 und Berichtb3523fa geliefert. Unser eigener G5-Importer-/Serve-/Produktivlauf wurde nicht gestartet. Der vorhandene Build-Datenrefresh ist kein G5-Cutover und belegt weder Produktimport noch typed POST /v1/answer.

Kein Prozess gestoppt, kein Dienst geändert, keine fremde Session kontaktiert, kein Worktree angefasst. Keine Prozessargumente, ENV, Secretdateien, DB-Rohinhalte oder Netzwerkpayloads gelesen. Zugriff auf exe/cwd des Usermanagers933 ergab EACCES; nicht umgangen.
