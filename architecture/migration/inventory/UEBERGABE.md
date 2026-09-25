# Übergabe Chat 01 Ist Zustand Dateninventar und Baseline

Status: Runtime Inventar am 25.09.2026 korrigiert, Integration durch Chat 00 ausstehend
Basis Commit: 2734c2da4e814ff79953e8e825275b0216a6af16 (Erstfassung), korrigierte Fassung auf main 535f5087b08e350dc568f2508d737d2c05326d0e
Ergebnis Commit / PR: Erstfassung 7cbe61e6 und 76205b5f in PR 15 (gestapelt auf dependency/s00-planpaket-20260924, nicht gemergt); korrigierte Fassung als Ersatz PR gegen main, siehe PR Beschreibung
Tatsächlich geprüfter Stand: Runtime Erhebung am 25.09.2026 04:25 bis 04:45 CEST
Contract und Schema Version: bestehender Ist Stand, gemeinsame v1 Contracts noch nicht durch Chat 02 fixiert
Source und Corpus Version: gemischter Ist Stand, siehe DATENINVENTAR.csv und QUELLENREGISTER.csv
Betroffene Requirement IDs: Inventar und G0 Voraussetzungen aus Plan v1.0

## Ergebnis und konkrete Änderungen

S01 ergänzt architecture/migration/inventory/ um Code und Runtime Inventar, Dateninventar, Domain Parität, Datenflusskarte, Baseline Bericht, Zugangs und Risikoliste, Quellenregister und G0 Protokoll. Produktcode und produktive Daten wurden nicht verändert.

## Nachweise

| Prüfung | Befehl / Testumgebung | Tatsächliches Resultat | Artefakt |
|---|---|---|---|
| Repo Commitstände | git rev-parse und git branch in Brain, Docs, 2nd Brain, Twitch und Bots | Commitstände aufgenommen, Working Trees teils verändert | CODE_RUNTIME_INVENTAR.md |
| Rust Dateiinventar | git ls-files für rust/crates, service, config, scripts und CI | Rust Crates, Adapter, Worker und Fixtures aufgenommen | CODE_RUNTIME_INVENTAR.md |
| Consumer Suche | rg in Twitch Bot, Deadlock Bots und Deadlock Docs | Brain Git Dependencies, Brain CLI Pfad, dl-knowledge und Evals gefunden | CODE_RUNTIME_INVENTAR.md |
| Runtime Units | systemctl --user list-unit-files und list-timers 'deadlock-brain*', systemctl --user show je Unit, systemctl list-unit-files '*brain*' | fünf Timer und ein Dienst als User Units, keine System Unit; grün: patchnotes-sync, sheet-sync, wiki-refresh, site; rot: build-data, youtube-learning | CODE_RUNTIME_INVENTAR.md |
| Fehlerursachen | journalctl --user für build-data (22. bis 25.09.2026) und youtube-learning (21. bis 24.09.2026), getent und dig für assets.deadlock-api.com | build-data: bis 24.09. fehlendes secret-exec Binary, am 25.09. NXDOMAIN; youtube: Preflight Branch '' im detached Worktree, Funktion pausiert | CODE_RUNTIME_INVENTAR.md, ZUGANG_RISIKEN.md |
| Wiki Refresh Release | release.json und sha256sum unter /opt/deadlock-brain/releases/12814d9 | Binary aus Merge 12814d9 aktiv, Lauf 25.09.2026 04:11 ready, Wiki Korpus Import aus | CODE_RUNTIME_INVENTAR.md |
| Datenordner Größe | du -sb data | 1813651236 Bytes | DATENINVENTAR.csv |
| Geheimnisprüfung | keine Secret Dateien gelesen, Environment nur als Schlüsselnamen, Journal Ausgaben mit DSN Filter | keine Secretwerte in S01 Artefakte übernommen | dieser Bericht |

## Folgen

Datenmigration und Kompatibilität: 03 bekommt eine konkrete Klassenliste und sichtbare unbekannte Record Counts.
Berechtigungen, Secrets und Egress: interne Docs und Second Brain bleiben getrennt klassifiziert. Externe Rechte sind als offen markiert.
Latenz, Ressourcen und Kosten: keine belastbare End to End Messung in S01. Chat 10 bleibt dafür zuständig.
Vorhandene Funktionen: Domain und Consumer Funktionen sind in DOMAIN_PARITAET.csv aufgenommen.
Python und Legacy: vorhandene Hilfsskripte und historische Pfade wurden inventarisiert. S01 führt keinen neuen Python Betriebsweg ein.

## Grenzen und Blocker

STATUS.md und koordinierte G0 Entscheidung aus Chat 00 fehlen im Basisstand.
Build Data und YouTube Learning sind rot. Die erste Fassung hat vier der fünf Timer und den Site Dienst nicht erfasst und die Build Ursache mit einem bereits vorhandenen Binary begründet; das ist in dieser Fassung korrigiert.
S01 meldet weder G0 noch G1 als erreicht.
Record Counts, Backup Restore Probe, externe Rechte und Performance Messungen bleiben Folgeaufträge.
Mehrere veränderte Arbeitsbäume verhindern eine Aussage, dass deren uncommitteter Zustand Teil eines stabilen Snapshots ist.

## Übergabe an nächsten Besitzer

00 integriert S01 in STATUS und entscheidet G0 gemeinsam mit Chat 10.
02 und 03 verwenden DATENINVENTAR.csv, DOMAIN_PARITAET.csv und QUELLENREGISTER.csv als Input für Contracts und Schema.
04 übernimmt alle fünf Ingest Timer aus CODE_RUNTIME_INVENTAR.md als Ist Stand: Patchnotes Sync (5 Minuten, grün, Skript außerhalb des Repos, Binary aus dem geteilten Hauptcheckout), Sheet Sync (4 Stunden, grün, geteilter Hauptcheckout), Wiki Refresh (6 Stunden, grün, Release Verzeichnis), Build Data (täglich, rot, Code Fix PR 33), YouTube Learning (6 Stunden, rot durch Preflight, Funktion pausiert). Kein Pilot darf einen dieser Pfade ersetzen oder abschalten, ohne ihn in dieser Liste zu referenzieren.
11 plant Cutover und Rückweg für alle sechs Units einschließlich deadlock-brain-site.service. Vor einem Cutover gehören Patchnotes Sync, Build Data, Sheet Sync und YouTube Learning auf versionierte Release Verzeichnisse wie Wiki Refresh (S01-R11), und die nicht versionierten Units werden ins Repo übernommen (S01-R12).
09 verfolgt die beiden Consumer bis zur aktiven Runtime.
12 und 13 verifizieren externe Quellen und Rechte; 13 übernimmt den Assets Umzug nach api.deadlock-api.com/v1/assets (S01-R14).

## Integration durch Chat 00

Merge Commit: offen
Gate und STATUS Änderung: offen
Freigegeben von: offen

## Zusätzliche Angaben v1.0

Arbeitsmodus: implement, read only gegenüber Produktdaten
Planrevision: v1.0 vom 24.09.2026
Echte Integration oder Fixture: echtes Repo und Runtime Inventar, keine Datenmigration
Betroffene Pfade: architecture/migration/inventory/
Neue nächste Arbeitswelle: nicht durch S01 freigegeben
Sensible Rechte: Second Brain, private Docs, lokale Rohdaten und externe Egress Regeln bleiben explizit zu prüfen
