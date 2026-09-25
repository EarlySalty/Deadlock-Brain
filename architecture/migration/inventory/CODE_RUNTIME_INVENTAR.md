# S01 Code und Runtime Inventar

Stand: 24.09.2026, Runtime korrigiert am 25.09.2026
Basis Commit Deadlock Brain: 2734c2da4e814ff79953e8e825275b0216a6af16
Arbeitsbranch: feat/brain-s01-inventory-20260924

## Repositories

| Bereich | Pfad | Branch | Commit | Arbeitsbaum | Befund |
|---|---|---|---|---|---|
| Deadlock Brain | /home/naniadm/Documents/Deadlock-Brain | feat/brain-rust-cutover-20260919 | 2734c2da4e81 | verändert | Produktcode und Datenbestand vorhanden. S01 arbeitet deshalb in eigenem Worktree. |
| Deadlock Docs | /home/naniadm/Documents/Deadlock-Docs | main | 98bf75aa6902 | verändert | Kuratierte interne Brain und Bot Dokumentation vorhanden. |
| Deadlock 2nd Brain | /home/naniadm/Documents/Deadlock-2nd-Brain | main | 0171da453592 | verändert | Interner Wissensbestand vorhanden. Inhalt wurde in S01 nicht exportiert. |
| Deadlock Twitch Bot | /home/naniadm/Documents/Deadlock-Twitch-Bot | main | 1442640c3ea4 | verändert | Konsument und Rust Abhängigkeiten auf Deadlock Brain vorhanden. |
| Deadlock Bots | /home/naniadm/Documents/Deadlock-Bots | main | ff635f7b354c | verändert | Discord Brain Pfad, dl-knowledge Retrieval und zentrale Brain Tabellen vorhanden. |

## Rust Module im Deadlock Brain

Der Workspace enthält die Crates dbrain-builds, dbrain-enrich, dbrain-learn, dbrain-normalize, dbrain-population, dbrain-reasoner, dbrain-retrieval, dbrain-sources, deadlock-brain-core, deadlock-brain-yt und deadlock-brain.

Wichtige belegte Pfade:

* Quellen und Ingest: rust/crates/dbrain-sources/
* Normalisierung und Lineage: rust/crates/dbrain-normalize/
* Learning und Optimizer: rust/crates/dbrain-learn/
* Builds und Publishing: rust/crates/dbrain-builds/ und rust/crates/dbrain-reasoner/
* Population und Analytics: rust/crates/dbrain-population/
* Retrieval: rust/crates/dbrain-retrieval/
* Kern, HTTP, Provider und Postgres: rust/crates/deadlock-brain-core/
* YouTube Pipeline: rust/crates/deadlock-brain-yt/
* CLI und Postgres Adapter: rust/crates/deadlock-brain/

## Quellenadapter im Code

Belegt sind Adapter für Deadlock API, Deadlock Assets API, deadlock-wiki/deadlock-data, deadlock.wiki, Entwicklerforum, Reddit, Google Sheet, Statlocker und Patchnotes DB. HTTP und Wiki Caches liegen nach README unter data/cache/.

## Consumer

### Deadlock Twitch Bot

rust/Cargo.lock pinnt Deadlock Brain Crates auf den Git Commit d8c34270868e129098e12243f53f5b52ee507b8b. docs/BRAIN_BUILD_LAB.md beschreibt die Nutzung des Brain Build Lab. docs/TODO-OFFEN.md nennt eine HTTP Anbindung an Deadlock Brain als Alternative zum lokalen Spielwissen.

### Deadlock Bots

docs/specs/2026-06-29-brain-command-design.md dokumentiert den Discord Brain Pfad über die CLI deadlock-brain ask-context. rust/crates/dl-answer/src/game.rs verarbeitet Game Knowledge. rust/bin/dl-knowledge und scripts/run_dl_knowledge_service.sh bilden einen weiteren Retrieval Pfad. Zentrale Brain und Knowledge Tabellen liegen in rust/crates/dl-central-db/migrations/.

## Runtime

Korrigiert am 25.09.2026 (Review PR 15). Die erste Fassung vom 24.09.2026 kannte nur den Build Timer und ist in diesem Abschnitt überholt.

Erhebung: `systemctl --user list-unit-files 'deadlock-brain*'`, `systemctl --user list-timers 'deadlock-brain*' --all`, `systemctl --user show <unit>` (FragmentPath, DropInPaths, ExecStart, WorkingDirectory, Result, ExecMainStatus, Zeitstempel) und `journalctl --user` am 25.09.2026 zwischen 04:25 und 04:45 CEST. Environment Werte, Credentials und DSNs wurden nicht ausgegeben. `systemctl list-unit-files '*brain*'` auf Systemebene liefert keine Unit. `/home/naniadm` ist ein Symlink auf `/home/nathanael`, `~/Documents/Deadlock-Brain` ein Symlink auf `~/repos/Deadlock-Brain`.

### Deadlock Brain User Units

| Unit | Takt | Letztes Ergebnis 25.09.2026 | Laufzeitpfad | Zuständigkeit |
|---|---|---|---|---|
| deadlock-brain-patchnotes-sync.timer und .service | alle 5 Minuten | success 04:26, "no new patchnotes (changelog=285 brain=285)" | `~/.local/bin/deadlock-brain-patchnotes-sync.sh` (Skript außerhalb des Repos, Stand 06.07.2026) mit `DEADLOCK_BRAIN_ROOT=~/Documents/Deadlock-Brain` aus der Unit, also Binary `rust/target/release/deadlock-brain` im geteilten Hauptcheckout; bei neuem Patch `pull deadlock-data`, `pull assets --kind items --kind heroes`, `normalize entities --rebuild` | Ingest Patchnotes und Assets, S04 |
| deadlock-brain-sheet-sync.timer und .service | alle 4 Stunden | success 04:16 bis 04:17 | `~/repos/Deadlock-Brain/scripts/run_sheet_sync_with_infisical.sh` über Drop-in `10-infisical-wrapper.conf`, Binary `rust/target/release/deadlock-brain` mit `deadlock-brain-secret-exec` im geteilten Hauptcheckout | Ingest Google Sheet, S04 |
| deadlock-brain-wiki-refresh.timer und .service | 00:35, 06:35, 12:35, 18:35 | success 04:11:46 bis 04:11:56, `state: ready` | `~/.local/share/deadlock-brain/wiki-refresh/deadlock-brain` als Symlink auf `/opt/deadlock-brain/releases/12814d9/deadlock-brain` (Merge von PR 13, sha256 laut `release.json`), Config `wiki-refresh.json` ohne `wiki` Block, Wiki Korpus Import daher aus | Game Wiki und Heldenkarten, S12 und S06 |
| deadlock-brain-build-data.timer und .service | täglich 03:30 | failed, exit 1, an jedem Tag vom 22. bis 25.09.2026 | `~/repos/Deadlock-Brain/scripts/run_build_data_with_infisical.sh` und `rust/target/release/deadlock-brain` im geteilten Hauptcheckout (Branch feat/brain-rust-cutover-20260919 mit uncommitteten Skriptänderungen) | Build Daten und Population, S04 und S05 |
| deadlock-brain-youtube-learning.timer und .service | alle 6 Stunden | failed, exit 1, seit 23.09.2026 05:10 bei jedem Lauf | `~/.worktrees/brain-live-main/scripts/run_youtube_learning_with_infisical.sh` | YouTube Learning, absichtlich pausiert, S04 |
| deadlock-brain-site.service | Dauerdienst | active (running) seit 20.09.2026 02:13 | `python3 ~/Documents/deadlock-build-corpus/site/server.py`, lauscht auf 127.0.0.1:8087, Code außerhalb des Brain Repos | Build Corpus Seite, S11 |

Nicht vorhanden: `deadlock-brain.service` und `deadlock-brain-youtube.service`. Der YouTube Pfad heißt `deadlock-brain-youtube-learning`.

Im Repo versioniert sind nur `service/systemd/deadlock-brain-build-data.*` und `ops/deadlock-brain-wiki-refresh.*`; beide installierten Service Dateien stimmen ohne Kommentare mit dem Repo Stand überein. Die Units für Patchnotes Sync, Sheet Sync, YouTube Learning und Site liegen nur unter `~/.config/systemd/user/` und sind nicht versioniert.

### Angrenzende Units anderer Repos

| Unit | Repo | Stand 25.09.2026 | Bezug |
|---|---|---|---|
| dl-knowledge.service | Deadlock-Bots Release 46c4f07c | active seit 20.09.2026 | zweiter Retrieval Pfad, S09 |
| dl-brain-feeder.timer | Deadlock-Bots | success 20.09.2026, sonntags 19:00 | Feeder in Brain Tabellen, S04 und S09 |
| wiki-freshness.timer | Deadlock-Docs | failed 24.09.2026 | Doku Frische, nicht Brain Runtime |

### Fehlerursachen, neu erhoben am 25.09.2026

Build Data, 22. bis 24.09.2026: `deadlock-brain-secret-exec Release Binary fehlt`. Das Binary liegt seit 24.09.2026 08:15 CEST in `rust/target/release/`. Die Aussage der ersten Fassung war damit schon beim S01 Commit (24.09.2026 11:33) überholt.

Build Data, 25.09.2026 03:30: `GET https://assets.deadlock-api.com/v2/items?language=english` scheitert mit `dns error`. `assets.deadlock-api.com` ist NXDOMAIN, die Assets liegen jetzt unter `https://api.deadlock-api.com/v1/assets/`. Code Fix in PR 33 (`dbrain-builds/src/api.rs` und `dbrain-sources/src/assets_api.rs`). Live wirkt er erst, wenn der Timer ein Binary aus main nutzt, siehe S01-R11.

YouTube Learning: `deploy-preflight [deadlock-brain-yt]: FEHLER: Branch '' != erwartet 'main' in ~/.worktrees/brain-live-main`. Der Worktree steht detached auf dfefc8f und wurde von anderen Sessions umgeschaltet (21.09.2026 feat/brain-rust-cutover-20260919, 23.09.2026 fix/brain-tempo-multisignal-20260923). Hinter dem Preflight bricht der Wrapper ohne `DEADLOCK_BRAIN_ENABLE_YOUTUBE_SYNC=1` bewusst mit "disabled" ab; die Unit setzt die Variable nicht, und `rust/target/release/deadlock-brain-yt` fehlt im Worktree. Es fehlt also keine aktive Funktion, der Fehler erzeugt aber alle 6 Stunden eine Ausfallmeldung.

Damit sind fünf Brain Timer und ein Brain Dienst real in Betrieb, davon drei grün und zwei rot. Keine dieser Units wurde durch S01 geändert oder neu gestartet.

## Laufende Pfade gegenüber Dokumentation

Runtime belegt sind die sechs Units oben. README und docs nennen zusätzlich Cron Beispiele und Learning Wrapper (`scripts/run_build_learning.sh`, `scripts/run_player_match_learning.sh`). Für diese beiden gibt es keine Unit, und das User Journal seit 20.09.2026 enthält keinen Aufruf; sie bleiben als dokumentiert, nicht als laufend eingeordnet.

## Datenbestand

/home/naniadm/Documents/Deadlock-Brain/data belegt zum Messzeitpunkt 1.813.651.236 Bytes. Sichtbare Klassen auf Verzeichnisebene sind backups, cache, external, gemini_profile, insights, raw, youtube_audio und youtube_transcripts. game-wiki enthält pages sowie index.md und log.md.

S01 hat keine privaten Inhalte, Secrets oder Rohdaten aus diesen Verzeichnissen in Berichte kopiert.
