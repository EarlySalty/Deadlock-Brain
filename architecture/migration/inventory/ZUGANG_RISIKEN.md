# S01 Zugänge und Risiken

| ID | Thema | Nachweis | Auswirkung | Nächster Besitzer |
|---|---|---|---|---|
| S01-R1 | STATUS.md aus Chat 00 fehlt | architecture/migration enthält das Planpaket, aber keinen integrierten STATUS | G0 kann nicht durch S01 selbst als koordiniert freigegeben werden | 00 |
| S01-R2 | Build Daten Service fehlgeschlagen | systemctl --user status deadlock-brain-build-data.service am 24.09.2026 | periodischer Build Pfad ist nicht grün | 04 und 11 nach G1, Fehlerbehebung getrennt vom S01 Inventar |
| S01-R3 | Mehrere Brain Wissenspfade | Deadlock Brain Retrieval plus Deadlock Bots dl-knowledge | Gefahr doppelter Contracts und auseinanderlaufender Wissensstände | 02, 08, 09 |
| S01-R4 | Arbeitsbäume relevanter Repos verändert | Git Status der fünf inventarisierten Repos | Snapshots für Migration dürfen nicht aus unklar vermischten Working Trees erzeugt werden | 00 und jeweilige Repo Besitzer |
| S01-R5 | Externe Quellenrechte nicht in S01 bestätigt | Adapter im Code belegt, Lizenz und Egress nicht abschließend geprüft | Import oder Publikation kann blockiert sein | 12 und 13 |
| S01-R6 | Datenmenge logisch nicht gezählt | Filesystem Größe belegt, Record Counts unbekannt | Vollständigkeitsvergleich noch nicht möglich | 03 |
| S01-R7 | Second Brain intern | eigenes Repo und interner Zweck belegt | ACL Trennung muss im Ziel erhalten bleiben | 03 und 09 |
| S01-R8 | Runtime Performance fehlt | keine Chat 10 Messung im S01 Stand | keine Aussage zu p95, p99, RAM oder Durchsatz | 10 |
| S01-R9 | YouTube und lokale Rohdaten | data/youtube_audio und data/youtube_transcripts vorhanden | Aufbewahrung, Rechte und Löschsignale müssen geprüft werden | 03 und 04 |
| S01-R10 | Legacy und Konsument Pinning | Twitch Cargo.lock pinnt älteren Brain Git Commit | Contract Cutover braucht explizite Versionsstrategie | 02 und 09 |
