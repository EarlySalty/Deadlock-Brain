[Orchestrator] Paket K. Worktree /home/nathanael/.worktrees/knowledge-launcher-20261005, Branch fix/knowledge-launcher-20261005. Freigabe erst nach fremdem Prüfer. Kein Push, kein Deploy.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/knowledge-launcher-20261005

# Paket K: Knowledge-Start sucht das Cargo-Binary

Rolle: Blatt-Worker. Du startest keine weiteren Threads.

Auftraggeber: Kopf, T3-Thread 31575951-23e7-4dd9-b1af-8700f7ff45fe.
Bericht: /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-05-nebenfehler/AN_HAUPT-K.md

## Ziel

`dl-knowledge.service` ist seit 21:22:14 CEST aktiv, PID 2970084, nach drei Starts mit Exit 127. Das Startskript hat `$ROOT_DIR/rust/target/release/dl-knowledge` ausgeführt, und diese Datei fehlte (`No such file or directory`).

Die Unit zeigt auf das Skript im Release:

`/home/nathanael/.local/share/deadlock-bots/releases/5e77e7ef744876c82cf5b2c530778ec1549a4e62/source/scripts/run_dl_knowledge_service.sh`

`ROOT_DIR` ist dort `.../source`. Das gepackte Binary liegt eine Ebene darüber als `.../5e77e7ef744876c82cf5b2c530778ec1549a4e62/dl-knowledge`. Um 21:22 ist nachträglich ein Symlink von `source/rust/target/release/dl-knowledge` auf dieses Binary entstanden. Deshalb läuft der Dienst jetzt. Der nächste Release ohne diesen Symlink scheitert wieder an Zeile 49.

Soll: Das Skript im Repo startet das vorhandene Binary. Liegt `rust/target/release/dl-knowledge` ausführbar vor, bleibt das der Weg für einen normalen Build. Fehlt es, und liegt das gepackte Binary eine Ebene über `source`, wird dieses ausgeführt. Fehlen beide, endet das Skript mit Status 1 und einer klaren Zeile. Ein Symlink ist keine Lösung. Die installierte Release-Kopie wird nicht von Hand geändert.

## Eigentum

Worktree: `/home/nathanael/.worktrees/knowledge-launcher-20261005`
Branch: `fix/knowledge-launcher-20261005`
HEAD beim Start: `5e77e7ef744876c82cf5b2c530778ec1549a4e62`

Schreiben darfst du in `scripts/run_dl_knowledge_service.sh` und in einem kleinen Test direkt daneben, falls das Repo noch keinen Skripttest hat.

Unangetastet: der laufende Dienst, die Release-Kopie, Insights, systemd, Infisical-Tokens, Fireworks. Das Skript darf für den Test keinen echten Token und keinen Infisical-Loader brauchen. `DL_INFISICAL_READY=1` und ein Fake-Binary reichen.

## Beweis

Eine lokale Fixture zeigt drei Fälle. Mit Cargo-Binary wird dieses ausgeführt. Ohne Cargo-Binary und mit ausführbarer Datei `../dl-knowledge` wird diese ausgeführt. Ohne beide endet der Lauf mit Status 1. Der laufende Dienst bleibt unangetastet.

## Abschluss

Kein Push, kein Merge, kein Deploy, kein `systemctl`. Bericht mit Branch, SHA, Testbefehl und Ergebnis nach `AN_HAUPT-K.md`. Der Kopf schickt den Stand danach an einen fremden Prüfer.
