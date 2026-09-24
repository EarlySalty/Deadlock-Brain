# S01 Baseline Bericht

## Messbarer Ist Stand

Der Deadlock Brain Code ist bereits überwiegend als Rust Workspace organisiert. Die wesentlichen Domain Bereiche Quellen, Normalisierung, Learning, Builds, Reasoner, Population und Retrieval besitzen Rust Crates.

Der lokale Datenordner hatte am 24.09.2026 eine gemessene Größe von 1.813.651.236 Bytes. Die Messung sagt nichts über logische Datensatzanzahl, Vollständigkeit oder Migrationsfähigkeit aus.

Der installierte tägliche Build Timer ist aktiv. Der ausgelöste Service ist aktuell nicht erfolgreich, weil das erwartete Release Binary deadlock-brain-secret-exec fehlt. Damit ist die heutige periodische Datenversorgung keine grüne Runtime Baseline.

## Antwort und Retrieval Baseline

S01 hat keine belastbare End to End Latenzmessung durchgeführt. Der Plan verlangt die gemeinsame Messung mit Chat 10 auf benannter Zielhardware und festem Lastprofil. Ein Chat 10 Ergebnis liegt im S01 Arbeitsstand nicht vor.

Deadlock Bots dokumentiert für dl-knowledge eine Golden Evaluation mit 224 Fällen in sechs Paketen. Diese Angabe wurde im aktuellen Docs Bestand gefunden. S01 wertet sie als vorhandenes Testinventar, nicht als neu ausgeführten Qualitätstest.

## Ingest Baseline

Der Build Timer liefert einen realen Scheduler Nachweis. Weitere README Cron Beispiele und Learning Wrapper sind Code und Dokumentation, aber in S01 nicht als laufender Scheduler bestätigt.

## Ressourcen

Festplatte: Brain data 1.813.651.236 Bytes.
CPU, RAM, Queue Lag, DB Pool Wartezeit, p50, p95 und p99 wurden in S01 nicht erhoben. Diese Werte benötigen einen funktionierenden, fest gepinnten Runtime Pfad und den Testentwurf aus Chat 10.

## Baseline Entscheidung

Für Code und Datenbestand ist eine belastbare Inventarbasis vorhanden. Für Performance und erfolgreiche periodische Versorgung bestehen offene Punkte. Der Runtime Fehler und die fehlende Chat 10 Messung müssen vor einer Performance Freigabe geschlossen werden.
