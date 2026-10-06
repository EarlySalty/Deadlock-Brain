# Brain-Site in Rust

Die vorhandene Oberfläche und die erzeugten Steckbriefe werden weiterverwendet. Der Dienst ersetzt den Python-Dateiserver, ohne die Gestaltung oder die Kommentar-API umzubauen.

## URL und Dateien

Caddy entfernt das öffentliche Präfix `/brain`. Die öffentliche Startseite bleibt `https://deutsche-deadlock-community.de/brain/site/`, der Backendpfad ist `/site/` auf `127.0.0.1:8087`. `/site` führt zu `/site/`. Ein eigener Backendhandler für `/brain` ist nicht vorgesehen.

Der öffentliche Dateivertrag in `src/bin/site/assets.rs` umfasst `site/index.html`, `site/app.js`, `site/style.css`, `site/vendor/marked.min.js`, `builds_registry.json`, `hero_meta_compact.json`, `item_meta_compact.json`, die drei Berichte `HERO_STRENGTH.md`, `ITEM_STRENGTH.md`, `MASTER_METHODOLOGY.md` sowie die in der Registry genannten Heldendossiers unter `understanding/`. Die Registry wird beim Start und beim Dossierabruf geprüft. Aktualisierte Heldenlisten und Dossiers werden ohne Dienstneustart übernommen.

Erzeugte Steckbriefe werden unter `site/entities/{hero|ability|item}/{hex-Schlüssel}.html` gelesen. Die Publikationsentscheidung bleibt beim vorhandenen Renderer. Der Server erzeugt keine Profile und liest keine internen Originale. Verzeichnisauflistungen, unbekannte Dateien, Quellcode, Rohdaten und Symlinks sind durch die Dateiauswahl und komponentenweises Öffnen mit `O_NOFOLLOW` gesperrt. Eine Datei darf höchstens 16 MiB enthalten.

## Kommentare und PostgreSQL

GET `/api/comments` liefert `{key: [{text, ts}]}`. POST mit `action: "add"` fügt einen nicht leeren Kommentar an und liefert `{ok, comments}`. Die Standardgruppe ist `general`. Andere Aktionen verändern den Bestand nicht. Der Text wird auf 4000 Unicode-Zeichen gekürzt, die Zeitangabe auf 40. Gruppen dürfen höchstens 40 Zeichen enthalten. Zeitangaben mit HTML-Zeichen, Gruppen mit Steuerzeichen und Texte mit Nullzeichen werden mit 400 abgelehnt; das bestehende Frontend setzt Zeitangaben in HTML ein. Anfragen über 64 KiB erhalten 413.

Die neue Migration `scripts/migrations/2026-10-06-brain-site-comments-v1.sql` legt `brain.site_comments_v1` mit ihrer Identitätssequenz an. Gruppen werden beim Anhängen transaktional gesperrt; GET und POST liefern die gespeicherte Reihenfolge nach `id`. Der Dienst führt keine DDL aus und schreibt keine JSON-Dateien.

Der getrennte Ownerweg lautet `brain-migrate up-site-comments --config <vorhandene lokale Ownerkonfiguration>`, die lesende Prüfung `brain-migrate check-site-comments --config <vorhandene lokale Ownerkonfiguration>`. Migration und Grants gehören in den regulären Installationsschritt nach dem Merge, nicht in den Dienststart. Die bestehenden Coremigrationen müssen bereits vorliegen. Erneutes Ausführen der Kommentarmigration erhält vorhandene Zeilen.

`ops/brain-postgres/roles.sql` definiert `brain_site`. `grants.sql` vergibt dieser Rolle Schema-USAGE, SELECT/INSERT auf die Kommentartabelle und USAGE auf ihre Sequenz. Die Startprüfung verweigert Owner-/Ingest-/Servicekonten, verändernde Kommentarrechte und Leserechte auf andere Brain-Tabellen. Vor dem Wechsel ist der bisherige Kommentarbestand zu prüfen. Vorhandene Kommentare müssen verlustfrei übernommen werden; ein inzwischen gefüllter Altbestand darf nicht durch eine leere Tabelle ersetzt werden.

## Installation durch Paket A

Das neue Binary heißt `deadlock-brain-site` im bestehenden Paket `deadlock-brain`. Der vorhandene `ops/brain-release`-Helfer erfasst sämtliche Workspace-Binaries per Cargo-Metadaten und nimmt die Site sowie das passende `brain-migrate` ohne eine zusätzliche Registrierung ins SHA-gebundene Bundle auf. Die mitgelieferte Unit verwendet den bestehenden Installationspfad `/opt/deadlock-brain/maintenance-current/deadlock-brain-site`. Die bisherige Python-Unit wird erst nach regulärem Main-Release, Migration, Grants und Zugangsanbindung ersetzt.

Für `/etc/deadlock-brain/infisical-site.json` gilt unverändert das Format des bestehenden `deadlock_brain_core::pg::pg_pool_from_config`-Laders. `database_secret` muss einen separat bereitgestellten Zugang für `brain_site` auswählen, nicht den bisherigen Ingestzugang. Projekt, Umgebung, Secretpfad und lokaler Transport werden aus der bestehenden Betriebsanbindung übernommen. Die Unit verwendet das vorhandene systemd-Credential `infisical-token`. Zugangsdaten werden nicht in diesem Repo abgelegt.

Paket A provisioniert den eingeschränkten PostgreSQL-Zugang, den Infisical-Eintrag und die Konfiguration. In der bestehenden `ops/brain-postgres/pg_hba.conf` benötigt `brain_site` vor `local all all reject` die Regel `local brain brain_site scram-sha-256` für den Unix-Socket `/run/deadlock-brain-postgresql`, Port 5446; die TCP-Sperren bleiben bestehen. Der vorhandene Passworthelfer `set-role-passwords.sh` kennt bisher drei andere Rollen und provisioniert `brain_site` noch nicht. Diese zusätzliche Secretanbindung muss vor dem Dienstwechsel über den Betriebsweg eingerichtet werden. Ohne diese Anbindung bleibt der neue Dienst aus. Die Unit bindet ausschließlich Loopback; ein anderer Bindwert wird vom Binary abgelehnt.

## Lokaler Nachweis

`/home/nathanael/.cargo/bin/cargo test -p deadlock-brain --bin deadlock-brain-site -- --include-ignored --nocapture` startet eigene PostgreSQL-16-Cluster auf eigenen Unix-Sockets und echte HTTP-Server auf freien lokalen Ports. Die Tests prüfen Persistenz über einen Server-/Poolneustart, parallele Kommentare, Grenzen, Rechte und Dateisperren. Mit `BRAIN_SITE_TEST_CORPUS_ROOT` lässt sich der öffentliche Corpus gegen die tatsächlich gelieferten Bytes prüfen; ohne diese Variable laufen reproduzierbare lokale Testdateien. Das Test-HTML für die drei Entitätsarten ist ein Auslieferungsnachweis, kein Nachweis erzeugter Spielprofile. Das Backend auf 8087 und öffentliche Kommentare werden dabei nicht verändert.
