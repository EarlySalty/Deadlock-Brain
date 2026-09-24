# S01 Code und Runtime Inventar

Stand: 24.09.2026
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

Die Repo Unit service/systemd/deadlock-brain-build-data.timer ist im Nutzer System installiert.

Runtime Nachweis am 24.09.2026:

* deadlock-brain-build-data.timer: aktiv und wartend, nächster Lauf 25.09.2026 03:30 CEST
* deadlock-brain-build-data.service: letzter Lauf fehlgeschlagen
* Fehlerursache laut systemd Status: Release Binary /home/nathanael/repos/Deadlock-Brain/rust/target/release/deadlock-brain-secret-exec fehlt
* deadlock-brain.service: nicht gefunden
* deadlock-brain-youtube.service: nicht gefunden

Damit ist ein geplanter periodischer Build Pfad real vorhanden, aber zum Inventarzeitpunkt nicht erfolgreich lauffähig.

## Laufende Pfade gegenüber Dokumentation

Der periodische Build Timer ist Runtime belegt. README und docs nennen zusätzlich Cron Beispiele und weitere Learning Wrapper. Für diese Beispiele liegt in S01 kein Runtime Nachweis vor. Sie werden deshalb als dokumentiert, nicht als laufend eingeordnet.

## Datenbestand

/home/naniadm/Documents/Deadlock-Brain/data belegt zum Messzeitpunkt 1.813.651.236 Bytes. Sichtbare Klassen auf Verzeichnisebene sind backups, cache, external, gemini_profile, insights, raw, youtube_audio und youtube_transcripts. game-wiki enthält pages sowie index.md und log.md.

S01 hat keine privaten Inhalte, Secrets oder Rohdaten aus diesen Verzeichnissen in Berichte kopiert.
