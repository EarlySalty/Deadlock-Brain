# S01 Datenflusskarte

## Aktueller belegter Kern

Externe Deadlock Quellen gelangen über dbrain-sources in Raw Snapshots, Source Dokumente, Caches und Postgres bezogene Strukturen. dbrain-normalize erzeugt normalisierte Entities, Claims, Patchdaten und Lineage. dbrain-population erzeugt Population und Analytics Material. dbrain-learn verarbeitet Builds, Matches und Demo Evidenz zu Lernartefakten. dbrain-reasoner nutzt Domain Daten für Mechanik, Meta, Planung und Publishing. dbrain-retrieval bildet einen Suchpfad über Game Wiki und abgeleitete Indizes.

Periodisch laufen fünf Nutzer Timer: Patchnotes Sync schreibt Patchnotes, Deadlock Data und Assets in `brain.*` und normalisiert Entities neu; Sheet Sync zieht das Google Sheet; Wiki Refresh baut den Game Wiki Snapshot samt Heldenkarten unter `~/.local/share/deadlock-brain/game-wiki/current`; Build Data holt Item Katalog, Helden und Analytics für Builds und Population; YouTube Learning ist pausiert. Der Dienst deadlock-brain-site liefert die Build Corpus Seite lokal auf 127.0.0.1:8087 aus. Stand 25.09.2026 sind Build Data und YouTube Learning rot, siehe CODE_RUNTIME_INVENTAR.md.

## Consumer

Deadlock Twitch Bot bezieht Brain Crates als Git Abhängigkeit und besitzt Build Lab Integration. Deadlock Bots besitzt einen Brain CLI Pfad und zusätzlich den Retrieval Dienst dl-knowledge mit eigenen Knowledge Tabellen und Evals. Beide Consumer müssen im späteren Contract Schnitt berücksichtigt werden.

## Dokument und internes Wissen

Deadlock Docs enthält öffentliche und interne kuratierte Inhalte sowie dokumentierte Knowledge Evals. Deadlock 2nd Brain ist ein eigener interner Wissensbestand. Eine Zusammenführung ohne ACL und Egress Trennung wäre fachlich falsch. S01 exportiert daher keine Inhalte in das Code Repo.

## Zielbeziehung für Folgepakete

01 liefert Bestand und Konflikte. 02 muss gemeinsame Contracts festlegen. 03 muss Datenmodell und Migration der vorhandenen Raw, Domain, Learning, Knowledge und Checkpoint Klassen definieren. 09 muss Discord und Twitch Consumer gegen den integrierten Kern führen. 10 muss die dokumentierten Evals und Runtime Messungen reproduzierbar übernehmen.
