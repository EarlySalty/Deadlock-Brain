status: aktiv
Datum: 2026-10-01

# Deadlock-Brain Paket B Timer

- Branch: `fix/paket-b-timer-20260930`
- Start-Head: `fd6e157622e283cf4ff007c3c355348681c20e99`
- Ziel: Transiente Deadlock-API-Fehler begrenzt und mit Backoff wiederholen.
- Umfang: Build-Data-API und dazugehörige Timer-/Service-Semantik. Keine Zusatzfeatures.
- Produktions-Hold: keine Main-Merges, Finalbuilds, Deploys, Restarts oder Änderungen an Config/current/DDL bis zur Token-DB-Livefreigabe.
- Ausgangsprüfung: Build-Data-Timer läuft täglich um 03:30 Europe/Berlin mit `Persistent=true`. Der Ausgangscommit ergänzt zusätzlich `Restart=on-failure` und ein Startlimit von 6 Starts pro 1800 Sekunden. Diese zweite Retry-Schleife ist im Branch entfernt, damit die Gesamtzahl der HTTP-Versuche pro API-Aufruf bei 5 bleibt. Die installierte Unit wurde nicht verändert. API-Client-Timeout: 15 Sekunden.
- Twitch-Abgleich: Thread `fae171ea-4ee4-4d0f-baf0-0fc29ea1eb49` bestätigt einen getrennten Auftrag für den Collector-Ausfallmelder. Der Twitch-Branch-Head `5b26b488` und sein `worker.rs`-WIP betreffen Collector-/YouTube-Upload-Meldungen, nicht Build-Data-Retries. Es gibt keinen gemeinsamen Retry-/Timer-Vertrag und keine Grundlage für Cross-Repo-Integration.
- Aktueller Stand: Brain-HEAD bleibt `fd6e157622e283cf4ff007c3c355348681c20e99`, Branch `ahead 1 / behind 8`, keine `MERGE_HEAD`. Eigene uncommitted WIP sind nur `api.rs`, die Build-Data-Service-Unit-Bereinigung, die Entfernung des unverbundenen YouTube-Service und die Paketstatusartefakte.
- Status: Quellen- und Vertragsreview abgeschlossen. Schwere gemeinsame Prüfungen/Gate seriell beim Integrator ausstehend. Kein Einzelmerge, keine TokenDB-Source-/Runtimeänderung. Root-Callback gesendet, T3-Sequenz 1306101.
