status: aktiv
Datum: 2026-10-03
Stand: 2026-10-03T06:47:09Z

# C2 nimmt B2s gemessene Importgrenze auf

B-C-IMPORTGRENZE-B2.md und D-B-LESEVERTRAG-B2.md im lokalen B-Worktree vollständig gelesen. Punkt 30 übernommen. B2 bleibt Eigentümer seiner finalen Parser-/Datenprüfung und späteren echten Depotextraktion. Kein C2-B-Compiler, neuer Parser oder D-Downloader.

## Tatsächliche Eingabe und offener C-Fix

Haupt meldet ersten vollständigen grünen B2-Lauf: vier Compilerprüfungen, 33 Testausführungen; GameTracking 237 Dokumente/331.287 Fakten, deadlock-data 423 Dokumente/345.076 Fakten, zusammen 506.412.456 JSONL-Bytes. Das ist noch kein endgültig geprüfter Commit und kein D-Depotnachweis.

Größte GameTracking-Zeile 117.157.259 Bytes ohne LF, größte deadlock-data-Zeile 11.107.243 Bytes. B2 hat die aktuelle C-Zeilenabweisung über PARTITION_MAX_BYTES = 64 MiB konkret nachgelesen. Dadurch ist der vollständige GameTracking-Import gegenwärtig blockiert. C2 übernimmt diesen realen Integrationsfix nach dem laufenden Revisionswriter, ohne dessen Quelldateien oder eingefrorenen Prüflauf zu überschneiden.

Ziel: vorhandenen Import-/Partitionsweg verlustfrei so verarbeiten lassen, dass ein gemessenes großes Dokument eine eigene begrenzte Partition erhalten kann. Normale Partitionsgrenzen nicht pauschal aufblasen; keine Rohtext-, Fakten- oder Zahlenlexemkürzung. Vor Freigabe am echten Material maximale Zeilen-/Dokument-/Partitiongröße, Verarbeitungs-RSS, Speicher-/Textprojektion und komplette Releasegrenzen messen. Größen-, Dokument- und Chunkguard dürfen nicht nur mit größeren Konstanten ausgehebelt werden. Wiederholimport, exakte Bytes/Hashbindung und historisch vollständiger Bestand bleiben Pflicht. Noch kein entsprechender Code- oder Laufzeitnachweis.

## D/B-Vertrag, statisch bekannt und noch nicht live

C2 bindet pro tatsächlichem D-Depot ein GameFileOptions aus Root, App/Build/Manifest/Depot und UTC-Zeit des geprüften D-Inventars; source_revision=None, source_layout=resource_paths, unbelegte Sprache und. Manifesttransporthash/Inventarreferenz bewahren, Herkunft physischer VPK und extrahierter Ressource getrennt halten. Git-SHA oder Clientversion nicht als Steam-Build ausgeben. Veröffentlichung bei redistribution_allowed=false bleibt gesperrt.

B2s bisheriger Validator akzeptiert noch nicht Millisekundenzeiten/Steam-Identitäten/VPK-Ressourcen; B2 führt seine nötige Erweiterung geordnet selbst durch. D-Inventarserialisierung to_vec vor 256-MiB-Prüfung ist im B-Bericht als weiterer statischer Befund benannt, ohne gemessenen Speicherfehler. D-Prüfung läuft laut Haupt separat; C2 ändert D nicht parallel. Echte vollständige D-Inventare, Begleitarchive, Hashbindung, Abdeckung und Ressourcen bleiben vor gemeinsamer Liveabnahme offen.

Bestehende C-Releasegrenzen: 10.000 Dokumente, 256 MiB projizierter Indextext, 500.000 Chunks. Alle 676.363 gemeldeten B-Fakten sind nicht automatisch ebenso viele Chunks; das tatsächliche Projektions-/Chunkingverhalten und die Kombination mit erhaltenem Basiscorpus werden gemessen, nicht geschätzt. Kein endgültiger Gesamtimport oder Release behauptet.
