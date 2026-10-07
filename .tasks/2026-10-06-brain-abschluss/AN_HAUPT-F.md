# Paket F: Build-Veröffentlichung

## 07.10.2026: Schnittstellen vor Umsetzung

Eigener Worktree `/home/nathanael/.worktrees/brain-f-publish`, Branch `feat/brain-build-publish-ohne-matchgrenze`, Basis `bfda408c`.

F ändert den Reasoner: Publish-Abnahme, Mechanikplanung ohne Familien-/Populationspflicht, Confidence und bestehende Regressionen. Keine Änderungen an E-Ingest, Spiegel oder Tabellen. Der Reasoner liest weiter `brain.entity_snapshots`, `hero_catalog`, `item_catalog` und Fähigkeitsdaten über vorhandene Loader. Veröffentlichung prüft den aktiven Patch und Snapshot-Provenienz unmittelbar vor dem Queue-Schreiben. E muss die Versionsbindung in diesen vorhandenen Tabellen belegbar liefern; fehlende aktuelle Werte werden nicht durch Matchzahlen ersetzt.

Abbau: 100er-Publishgrenze, verpflichtende Familienabnahme, Staple-Zulassung als Kernfilter und Matchmangel als Confidence-Abwertung. Bestehende Rohmatchablagen werden nicht erweitert; Abbau liegt bei E. Ein Antwortweg bleibt erhalten, kein neuer Bot-Befehl.

Zusätzliche Schnittstelle vor Änderung: F ergänzt in `deadlock-brain/src/main.rs` ausschließlich `publish_build_http` um den bestehenden Pool sowie dessen zwei Aufrufer. Die API-Snapshot-/Patch-Prüfung muss auch vor dem HTTP-Veröffentlichungsweg greifen. Keine Änderung an A-Antwortlogik oder E-Ingest.

Release-Hold bleibt bestehen. Kein Brain-Release, Install, Neustart oder Tick durch F. Featurecommit wird auf origin gesichert. Warden-Publish und Gate folgen im Abschlussbericht.
