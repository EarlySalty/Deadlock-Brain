# S01 Baseline Bericht

## Messbarer Ist Stand

Der Deadlock Brain Code ist bereits überwiegend als Rust Workspace organisiert. Die wesentlichen Domain Bereiche Quellen, Normalisierung, Learning, Builds, Reasoner, Population und Retrieval besitzen Rust Crates.

Der lokale Datenordner hatte am 24.09.2026 eine gemessene Größe von 1.813.651.236 Bytes. Die Messung sagt nichts über logische Datensatzanzahl, Vollständigkeit oder Migrationsfähigkeit aus.

Korrigiert am 25.09.2026: Fünf Brain Timer und ein Brain Dienst laufen als User Units (Details in CODE_RUNTIME_INVENTAR.md). Grün sind Patchnotes Sync, Sheet Sync, Wiki Refresh und der Site Dienst. Rot sind Build Data (22. bis 24.09.2026 fehlendes secret-exec Binary, seit 25.09.2026 NXDOMAIN von assets.deadlock-api.com) und YouTube Learning (Preflight im detached Worktree brain-live-main, Funktion selbst absichtlich pausiert). Die periodische Datenversorgung ist damit keine vollständig grüne Runtime Baseline.

## Antwort und Retrieval Baseline

S01 hat keine belastbare End to End Latenzmessung durchgeführt. Der Plan verlangt die gemeinsame Messung mit Chat 10 auf benannter Zielhardware und festem Lastprofil. Ein Chat 10 Ergebnis liegt im S01 Arbeitsstand nicht vor.

Deadlock Bots dokumentiert für dl-knowledge eine Golden Evaluation mit 224 Fällen in sechs Paketen. Diese Angabe wurde im aktuellen Docs Bestand gefunden. S01 wertet sie als vorhandenes Testinventar, nicht als neu ausgeführten Qualitätstest.

## Ingest Baseline

Real laufende Ingest Pfade sind Patchnotes Sync (5 Minuten), Sheet Sync (4 Stunden), Wiki Refresh (6 Stunden) und Build Data (täglich, rot). YouTube Learning ist als Timer aktiv, aber im Wrapper pausiert. `scripts/run_build_learning.sh` und `scripts/run_player_match_learning.sh` haben keine Unit und keinen Journal Eintrag seit 20.09.2026.

## Ressourcen

Festplatte: Brain data 1.813.651.236 Bytes.
CPU, RAM, Queue Lag, DB Pool Wartezeit, p50, p95 und p99 wurden in S01 nicht erhoben. Diese Werte benötigen einen funktionierenden, fest gepinnten Runtime Pfad und den Testentwurf aus Chat 10.

## Baseline Entscheidung

Für Code und Datenbestand ist eine belastbare Inventarbasis vorhanden. Für Performance und erfolgreiche periodische Versorgung bestehen offene Punkte. Die beiden roten Timer und die fehlende Chat 10 Messung müssen vor einer Performance Freigabe geschlossen werden.
