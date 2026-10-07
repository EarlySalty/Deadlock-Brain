status: aktiv
Datum: 2026-10-03

# CLI-Installationsvertrag für Z

## Bestehender Weg nicht belegt

Der Bestandsspäher `wf_de0b42a0-282` hat keinen SHA-verifizierten Brain-CLI-Installer gefunden. Untersucht wurden Brain `ops/`, `service/`, `scripts/`, Betriebsdokumentation, `/usr/local/bin`, `/usr/local/sbin`, `/usr/local/lib/deadlock-brain`, `/usr/local/libexec`, `~/.local/bin`, `Documents/Admin-Scripts` und die Brain-Releaseablage. Das Ergebnis gilt für diesen Suchumfang.

Die gefundenen Root-Wrapper betreffen Twitch: `/usr/local/bin/deploy-twitch-release:155` und `/usr/local/sbin/install-twitch-release:22`. `Documents/Admin-Scripts/remote-deploy.sh:37` ersetzt ein Dienstbinary, hat aber keinen belegten Brain-Releasevertrag mit Git-SHA und Installationssperre.

`/opt/deadlock-brain/current` zeigt auf `releases/be2aa6bd5a6504e99693f7dd1edaa16e76be91b4`. CLI-Prüfsumme: `f870cfa225e98b366c9e5b8e512a6cf88bc8d20cebc5f122df96c5bbda6c23db`. Im aktiven Release fehlen `SHA256SUMS`, `release.json`, `REVISION` und `bin/deadlock-brain-secret-exec`. Der Verzeichnisname allein beweist den Source-SHA nicht. Die installierte CLI kennt `--publish-endpoint`, `--infisical-config` und `--wait-seconds` noch nicht.

Z muss den bestehenden Ops-Pfad um die gemeinsame, SHA-gebundene Installation der CLI und des vorhandenen Rust-Secret-Exec ergänzen. S hat keinen Releasezeiger geändert. Fundstelle für Secret-Exec: `rust/crates/deadlock-brain/src/bin/deadlock-brain-secret-exec.rs:36` im Brain-Worktree.

## Sperrvertrag

Compilerstarts erwerben blockierend zuerst `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock`, danach `/tmp/deadlock-cargo-release.lock`. Nach Erwerb eine frische NonZombie-Probe nach `Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md:3`, höchstens zwei Jobs. Beide Locks bis zum vollständigen Abschluss halten. Bei fremden Compilern 30 Sekunden warten und erneut prüfen, keine fremden Compiler beenden. Die genaue Metadata-Ausnahme steht in derselben Akte.

Eine vorhandene automatische Brain-Installationssperre ist im untersuchten Bestand nicht belegt. Releaseinstallation und Umschaltung gehören Z gemäß `bereiche/s/VON_HAUPT.md:24` und `:30`.

## Geheimnisfreier CLI-Aufruf nach Deployment

Die neue CLI liest den internen HTTP-Zugang aus dem bestehenden Infisical-Vertrag. Vorhandenes Secret-Exec beschafft den DB-Zugang für den Kindprozess. Die normale Konfiguration `/etc/deadlock-brain/infisical.json` nutzt `prod`, den bestehenden Unix-Socket und Credential-FD 5. Kein Zugangswert wurde als Klartext gelesen oder gespeichert.

Nach Installation des gemeinsamen Standes und vorheriger Prüfung der unveränderten Anfrage wäre dieser Aufruf möglich. Er wurde nicht ausgeführt:

```bash
env INFISICAL_TOKEN_FILE=/home/nathanael/.config/infisical-tokens/infisical-token-bots \
  /opt/deadlock-brain/current/bin/deadlock-brain-secret-exec \
  --config /etc/deadlock-brain/infisical.json -- \
  /opt/deadlock-brain/current/bin/deadlock-brain reason build Warden \
  --no-ai --publish --wait-seconds 120 \
  --publish-endpoint http://127.0.0.1:8783 \
  --infisical-config /etc/deadlock-brain/infisical.json --json
```

Fundstellen: `rust/crates/deadlock-brain/src/main.rs:279`, `:314`, `:2224` am Feature-SHA `9a6f3d5f3ae2349d9fb076821381e45a421a0bbe`. Die HTTP-Wiederaufnahme wird im laufenden Fix weiter abgesichert; der endgültige SHA wird in UEBERGABE.md genannt.

## Status und Idempotenz

`HttpBuildPublishClient::status_for(&request)` liest hashgebunden per GET. Der endgültige `request_sha256` enthält die fertig berechnete ID und ist nicht deren Suffix. Fundstellen: `brain-feeds/src/build_publish.rs:185`, `:356` und `brain-contracts/src/feeds.rs:114` am genannten Feature-SHA. Der Steam-Speicher bindet die Anfrage transaktional: `steam-persistence/src/build_publish.rs:34` am Steam-Feature-SHA `9aec0cc897b01b74d417ab9b510314cbbbd02535`.

Health 8783 antwortete im Bestandslauf mit 200; Steam und GC auf 8782 waren verbunden. Ein unautorisierter Status-GET lieferte 401. Das ist kein Beweis, dass kein Auftrag existiert. Ein konkreter bestehender Publish-Auftrag wurde noch nicht abgefragt. Ein neuer AI-Lauf kann andere Beschreibungen und damit eine neue ID erzeugen, deshalb keinen unklaren Publish durch Neuberechnung wiederholen.

Abschluss bleibt: SHA-verifizierte Installation durch Z, unveränderte Anfrage prüfen, echter CLI-HTTP-Publish bis `DONE` mit positiver `hero_build_id`, anschließend Meldung an Z. Keine Communitynachricht.
