# S01 Zugänge und Risiken

| ID | Thema | Nachweis | Auswirkung | Nächster Besitzer |
|---|---|---|---|---|
| S01-R1 | STATUS.md aus Chat 00 fehlt | architecture/migration enthält das Planpaket, aber keinen integrierten STATUS | G0 kann nicht durch S01 selbst als koordiniert freigegeben werden | 00 |
| S01-R2 | Build Daten Service fehlgeschlagen | Journal 22. bis 25.09.2026: erst fehlendes secret-exec Binary, seit 25.09.2026 NXDOMAIN von assets.deadlock-api.com | periodischer Build Pfad ist nicht grün | Code Fix PR 33, Auslieferung siehe S01-R11, danach 04 |
| S01-R3 | Mehrere Brain Wissenspfade | Deadlock Brain Retrieval plus Deadlock Bots dl-knowledge | Gefahr doppelter Contracts und auseinanderlaufender Wissensstände | 02, 08, 09 |
| S01-R4 | Arbeitsbäume relevanter Repos verändert | Git Status der fünf inventarisierten Repos | Snapshots für Migration dürfen nicht aus unklar vermischten Working Trees erzeugt werden | 00 und jeweilige Repo Besitzer |
| S01-R5 | Externe Quellenrechte nicht in S01 bestätigt | Adapter im Code belegt, Lizenz und Egress nicht abschließend geprüft | Import oder Publikation kann blockiert sein | 12 und 13 |
| S01-R6 | Datenmenge logisch nicht gezählt | Filesystem Größe belegt, Record Counts unbekannt | Vollständigkeitsvergleich noch nicht möglich | 03 |
| S01-R7 | Second Brain intern | eigenes Repo und interner Zweck belegt | ACL Trennung muss im Ziel erhalten bleiben | 03 und 09 |
| S01-R8 | Runtime Performance fehlt | keine Chat 10 Messung im S01 Stand | keine Aussage zu p95, p99, RAM oder Durchsatz | 10 |
| S01-R9 | YouTube und lokale Rohdaten | data/youtube_audio und data/youtube_transcripts vorhanden | Aufbewahrung, Rechte und Löschsignale müssen geprüft werden | 03 und 04 |
| S01-R10 | Legacy und Konsument Pinning | Twitch Cargo.lock pinnt älteren Brain Git Commit | Contract Cutover braucht explizite Versionsstrategie | 02 und 09 |
| S01-R11 | Timer laufen aus geteilten Arbeitsbäumen | Build Data, Sheet Sync und Patchnotes Sync nutzen `rust/target/release` (Build Data und Sheet Sync auch die Skripte) im Hauptcheckout auf feat/brain-rust-cutover-20260919 mit uncommitteten Änderungen; YouTube nutzt den umschaltbaren Worktree brain-live-main | ein Fix auf main wirkt nicht automatisch live, Branchwechsel anderer Sessions ändern die Produktion | 11, Auslieferung nur über versionierte Release Verzeichnisse wie bei Wiki Refresh |
| S01-R12 | Nicht versionierte Units | Patchnotes Sync (Skript in `~/.local/bin`), Sheet Sync Drop-ins, YouTube Learning und Site nur unter `~/.config/systemd/user` | Cutover und Rückweg sind für diese Pfade nicht reproduzierbar | 04 und 11 |
| S01-R13 | YouTube Fehlalarm | Preflight scheitert alle 6 Stunden im detached Worktree, Funktion absichtlich pausiert | Ausfallmeldungen ohne Funktionsverlust verdecken echte Fehler | 04, Entscheidung pausieren oder Unit deaktivieren |
| S01-R14 | Assets Upstream umgezogen | assets.deadlock-api.com NXDOMAIN, Ersatz api.deadlock-api.com/v1/assets; `/raw/items` und `/raw/heroes` ohne Ersatz | `pull assets` ohne `--kind` scheitert weiter an den raw Arten | 04 und 13 |
