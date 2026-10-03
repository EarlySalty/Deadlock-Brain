status: aktiv
Datum: 2026-10-03

# C2 Betriebs-Vorcheck

Übernommener Fachbericht des nativen Workers a97e867cd5ffb6a06, geerbtes Sol high, beendet ohne lebende Kinder. Der Worker lieferte seinen Bericht in der Abschlussantwort statt an den vorgegebenen Dateipfad; C2 sichert ihn hier. Reiner Betriebs-Vorcheck, kein Review oder ALLOW, keine produktive Wirkung.

## Gemessene Baseline

- brain-serve.service: laufendes Binary /opt/deadlock-brain/maintenance-releases/511a347b653beba13c2bf130f4bead7a7196cc2a/brain-serve, HTTP 127.0.0.1:8788, /healthz und /readyz jeweils 200. Aktiver Release maintenance-6edd9236639f9b06067a3b6b67fa410e5e668aa264db1b2baf28b7e1733940de, Knowledge-Version docs-6edd9236639f9b06067a3b6b67fa410e5e668aa264db1b2baf28b7e1733940de.
- steam-core.service und steam-core-2.service: laufende Binaries unter /opt/deadlock/steam/releases/4c5621763d5f01c96d7912400517c08aa1c40df1/, HTTP 127.0.0.1:8782 bzw. 8784, /health jeweils 200, Steam und GC verbunden. steam-bot.service aus demselben Release, Port 8783, /health 200. Lokale origin/main-Referenzen passen, Worker hat nicht gefetcht.
- Für laufenden Brain-Serve ist maintenance-current maßgeblich. /opt/deadlock-brain/current zeigt dagegen auf be2aa6bd5a6504e99693f7dd1edaa16e76be91b4; nicht als laufenden Serve-SHA ausgeben.

## Bestehende Wege

Lesende Health-Probe ohne Zugangsdaten: curl --noproxy '*' --max-time 8 --max-filesize 4096 --silent --show-error http://127.0.0.1:8788/readyz. /v1/retrieve ist der vorhandene belegliefernde Brain-Pfad ohne Anbieteraufruf; Bearer/Scope erforderlich. Keine typisierte domain und kein build-Profil. Retrieval und Steam-Taskrouten wurden nicht ausgeführt, weil kein secretsfreier Clientweg belegt war.

Steam-Binaries haben --check-config mit normaler bot.toml erfolgreich geprüft, ohne Listener, Secrets oder Login. Bestehender Startpfad rust/deploy/private-start/steam-core.service.conf verwendet dl-infisical-env, --token-pipe und /etc/deadlock-token-launchers/steam.json. /usr/local/bin/bot-restart erlaubt steam-core, steam-core-2 und steam-bot, keinen brain-serve. Kein Neustart ausgeführt. Releaseinstaller für Brain/Steam noch nicht verifiziert, Twitch-Wrapper nicht umwidmen.

Brain-Import-CLI liegt im C-Integrationtree unter rust/crates/dbrain-sources/src/bin/brain-knowledge-import.rs. partition/validate/import/publish sind getrennt; validate schreibt einen Bericht, Import/Publish aktivieren keinen Dienst. Vorhandener brain-maintain-Unterbefehl write-serve-config verwendet gemeinsame Sperre, erwarteten bisherigen SHA und atomaren Austausch. Nicht ausgeführt.

## Datenbank und offene Zielbindung

Strukturgeprüfte normale Konfigurationen: ~/.config/deadlock-brain/brain-serve.json, /etc/deadlock-brain/maintenance-runtime.json, /etc/deadlock-brain/infisical.json sowie ~/.config/deadlock-steam-bot/infisical.json. Brain Serve nutzt brain_service, Maintenance brain_ingest, Datenbank brain, Unixsocket /run/deadlock-brain-postgresql, Port 5446. Infisical-Socket /run/uplink-infisical/api.sock; vorhandene FD-Verträge erhalten.

Wichtiger offener Integrationsbefund: Der neue Knowledge-Importer verwendet pg_pool_from_config des bisherigen Quellenpfads. Dessen Infisical-Referenz DEADLOCK_CENTRAL_DSN belegt noch keine Zielbindung an diese dedizierte Brain-Datenbank. C2 prüft den Aufrufer und vorhandenen dedizierten Rust-Verbindungsweg vor jedem Import. Keine DB-Korrektur von Hand und kein blindes Ausführen.

READONLY-SNAPSHOT.md dokumentiert den vorhandenen brain-legacy-import --observe-snapshot-Weg über korrekt eingebundenen Secret-Exec und brain_readonly mit abschließendem ROLLBACK. Keine SQL-Abfrage durch den Vorcheck. Vorhandener scripts/test_brain_serve.sh-Weg startet Scratch-Postgres über eigenen Unixsocket/Port 55439 und verlangt den Targetcache; benötigt beide Cargo-Locks. Nicht ausgeführt. run_isolated_serve_checks.sh ist ausdrücklich deaktiviert.

Kein Compiler-, Import-, Lizenz-, DB- oder Live-Funktionsnachweis für neue Daten. Nächster Schritt: Import-Zieldatenbank und regulären Releaseinstaller am bestehenden Rust-Code/Betriebsweg klären.
