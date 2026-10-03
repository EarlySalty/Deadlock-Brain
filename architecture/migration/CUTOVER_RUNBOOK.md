status: aktiv
Datum: 2026-10-03

# G5-Cutover und G6 Legacy-Ende

Stand der lesenden Aufnahme: 03.10.2026, 16:27 bis 16:46 Uhr Europe/Berlin. Quellstand im Z-Worktree und per `git ls-remote origin refs/heads/main`: `511a347b653beba13c2bf130f4bead7a7196cc2a`. Vor Ausführung Bestand und Remote-Stand erneut prüfen. Dieses Runbook schaltet keinen Dienst um und erklärt G5 oder G6 nicht für bestanden.

## Startbedingungen und Stop-Bedingungen

Maßgeblich sind `.tasks/2026-10-03-brain-fertigstellung/AUFTRAG.md`, `PAKETE.md`, `GEMEINSAM.md`, `BRIEFING-Z.md` und die aktuellen Übergaben der Pakete. Die zentralen Akten liegen unter `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/`; dort nichts auschecken oder bereinigen.

1. P, S, Q und K müssen ihren Abschluss oder eine ausdrücklich nichtblockierende Grenze mit Merge-SHA und Live-Beweis ablegen. R kann mit dokumentierter Grenze offen bleiben. Am Aufnahmestand lagen für P/S/Q/K keine `UEBERGABE.md` vor. Die Meldungen in `AN_HAUPT.md` beschreiben laufende Arbeit.
2. Der Wiki-Abschluss muss als `.tasks/2026-10-03-wiki-spielwissen/ENDE.md` oder dessen `ABSCHLUSSBERICHT.md` auf dem aktuellen main stehen. Beide Dateien fehlen im aufgenommenen main. Paketstart aus `status/z/1/1.json`: `2026-10-03T14:08:55Z`. Die 24-Stunden-Grenze endet am 04.10.2026 um 14:08:55 UTC, entsprechend 16:08:55 Uhr Europe/Berlin. Ein neuer Paketversuch setzt diese Frist nicht zurück. Erst danach darf Z die fehlende Wiki-Integration in `AN_HAUPT.md` melden und den übrigen Cutover ohne Wiki-Pilot durchführen.
3. Q muss das G0-Abnahme-Artefakt mit SLO, Lastprofil, zulässigem Fragenbestand, Provider-Shadow und gemessenem Ergebnis übergeben. Erwartete Ablage: `architecture/migration/evals/`, mit konkretem Dateinamen und SHA in `bereiche/q/UEBERGABE.md`. Bis dahin gibt es keine freigegebene Zahl für Latenz, Fehlerquote oder Parallelität. Alte Pilotwerte und synthetische Last ersetzen diese Abnahme nicht. Fehlendes Artefakt oder eine Überschreitung des dort vereinbarten Grenzwerts stoppt G5.
4. K muss jede produktive Consumer-Anfrage am Server belegen. Q baut dafür das redigierte Anfrageereignis mit Consumer-Kennung, Request-ID, Route, Status und Dauer. Dieser Nachweis fehlt im aufgenommenen main. Der private Unixtransport für 2nd-Brain ist ebenfalls ein offener Q/K-Vertrag. Ein HTTP-200 des Health-Endpunkts ersetzt keinen Consumer-Beweis.
5. Vor einem Schreibwechsel müssen der aktuelle Datenbestand, der einzige Writer je Quelle und dessen Rückweg belegt sein. Fehlende Releaseherkunft, eine ungeklärte Datenlücke zwischen DL-Main und dem isolierten Archiv, ein nicht gebundener Import oder ein unbekannter Consumer stoppt den betroffenen Wechsel.

Provider und Modelle bleiben außerhalb des Q-Auftrags unverändert. Die freigegebene Auswahl steht in `AUFTRAG.md`; die Patchnotes-Übersetzung gehört P. Keine Community- oder Nutzerfragen als Shadow-Material an externe Anbieter senden. Die aktuellen Q-Entscheidungen zu Sheet und YouTube vor Ausführung lesen; pausiertes Lernen nicht durch einen G5-Probelauf reaktivieren.

## Laufender Bestand

Die User-Units liegen unter `/home/nathanael/.config/systemd/user/`. `disabled` an einer Service-Datei bedeutet bei einem aktiven Timer keine Schreibpause. Der neue Kern besteht aus `brain-storage`, `brain-kernel`, `brain-api` und `brain-serve`; ein eigenes installiertes Binary namens `brain-core` wurde nicht gefunden. Die ältere Crate `deadlock-brain-core` gehört weiter zum historischen CLI-Pfad und ist kein Beleg für den neuen Kern.

| Unit / Rolle | Aufgenommener effektiver Weg | Zeitplan und Status | Nachfolger / Eigentümer |
| --- | --- | --- | --- |
| `brain-serve.service`, Rust-Kernleser mit Gesprächsspeicherung | `/opt/deadlock-brain/maintenance-current/brain-serve`; Drop-in `brain-serve.service.d/90-maintenance.conf`; Konfiguration `/home/nathanael/.config/deadlock-brain/brain-serve.json` | aktiv, PID `3506677` | Z integriert aktuellen Q-Kern. Der Server läuft schon; das allein ist kein G5. |
| `brain-maintenance.service`, Kernwriter und Dokumentpflege | `/opt/deadlock-brain/maintenance-current/brain-maintain --config /etc/deadlock-brain/maintenance-runtime.json --infisical-config /etc/deadlock-brain/infisical.json` | `brain-maintenance.timer` aktiv, fünfminütlich, Zufallsverzögerung bis 30 Sekunden | Vorhandenen Rust-Pfad erhalten. Er kann auch Serve-Konfiguration und Dokumente verändern; während einer Freigabeumschaltung pausieren. |
| `deadlock-brain-build-data.service`, alter Rust-Schreibpfad | `/home/nathanael/.worktrees/brain-live-main/scripts/run_build_data_with_infisical.sh`; standardmäßig `rust/target/release/deadlock-brain` dieses Baums | Timer aktiv, täglich 03:30 Uhr Europe/Berlin | Skript ruft `pull build-data`, `population sync` und `population stats`. S belegt Publish, aber diese Datensynchronisation hat noch keinen belegten vollständigen Kernnachfolger. Nicht ersatzlos abschalten. |
| `deadlock-brain-sheet-sync.service`, alter Rust-Schreibpfad | effektiver Drop-in startet `/home/naniadm/Documents/Deadlock-Brain/scripts/run_sheet_sync_with_infisical.sh` | Timer aktiv, Boot plus fünf Minuten, danach vierstündlich | Q stellt die Unit selbst um und übergibt Kommando, Ziel, Datenregeln und echten Lauf. Aktuelles Skript: `refresh-sheet`, `learn analyze-next`, `enrich patch-impact`, `enrich meta-trends`. |
| `deadlock-brain-patchnotes-sync.service`, alter Rust-Schreibpfad mit Shell-/Verwaltungsanteil | Unit startet `/home/naniadm/.local/bin/deadlock-brain-patchnotes-sync.sh`; auf diesem Host derselbe aufgelöste Helfer wie `/home/nathanael/.local/bin/deadlock-brain-patchnotes-sync.sh`; festes Binary unter `/home/nathanael/.local/share/deadlock-brain/releases/25c6ed6-patchnotes-20260930/bin/deadlock-brain` | Timer aktiv, Boot plus zwei Minuten, danach fünfminütlich | P stellt den effektiven Pfad auf Rust-Feed-Verbrauch um. Helfer schreibt über `DEADLOCK_CENTRAL_DSN`, liest `patchnotes.changelog_posts` und `brain.entity_snapshots`. Er aktualisiert zusätzlich Assets und normalisierte Entitäten. Diese Nebenwirkungen vor Ablösung einem Writer zuordnen. |
| `deadlock-brain-youtube-learning.service`, Rust-Legacy-Lernen | `/opt/deadlock-brain/current/bin/deadlock-brain-yt scheduled-auto-learn` | Timer aktiv, Boot plus zehn Minuten, danach sechs Stunden | Q stellt die Unit selbst um. Vollständiges Transcript-/Modelllernen bleibt gemäß Q-Akte pausiert; ein erfolgreicher Metadatenlauf beweist nicht vollständiges Lernen. |
| `deadlock-brain-wiki-refresh.service`, Rust-Legacy-Wikiwriter | `/home/nathanael/.local/share/deadlock-brain/wiki-refresh/deadlock-brain --config /home/nathanael/.local/share/deadlock-brain/wiki-refresh/wiki-refresh.json` | Timer aktiv, 00:35, 06:35, 12:35 und 18:35 Uhr, Zufallsverzögerung bis 120 Sekunden | Wiki-Abschluss und Kernimport übernehmen. Eine produktive neue Wiki-Unit ist noch nicht belegt. |
| `dl-brain-feeder.service`, Rust-Aggregatleser und Datei-Writer | `/home/naniadm/Documents/Deadlock-Bots/scripts/run_brain_feeder.sh`, dort `rust/target/release/dl-brain-feeder` | Timer aktiv, sonntags 19 Uhr Europe/Berlin | Liest Aggregate aus zentraler PostgreSQL und optional Twitch-Analytics; schreibt einen Wiki-Drop. Kein belegter direkter Kernwriter. K/Z müssen den weiter benötigten Datei-Consumer und Ersatz klären. |
| `deadlock-patchnotes.service`, Python-Publisher | effektiver Drop-in `70-release-main.conf`; `/home/nathanael/.worktrees/patchnotes-live-main/scripts/run_patchnotes_bot.sh --config /home/nathanael/.config/deadlock-bots/patchnotes/bot.toml` | aktiv, PID `2721773`, `/proc/2721773/exe` ist `/usr/bin/python3.12` | P portiert Publisher und Feed. Derselbe Unitname kann danach Rust tragen; diesen neuen Dienst bei G6 nicht pauschal deaktivieren. |
| `deadlock-brain-site.service`, separater Python-Leser | `/home/naniadm/Documents/deadlock-build-corpus/site/server.py` | aktiv, PID `2002603`, Python 3.12 | Eigenständiger Bestandspfad außerhalb des Brain-Repos. Nicht als Writer behandeln oder ungeprüft entfernen. Route und fortbestehende Datenabhängigkeit sind noch offen. |

`/home/naniadm/Documents/Deadlock-Brain` löst auf `/home/nathanael/repos/Deadlock-Brain` auf. Die effektiven Alias-Pfade sind beim Ändern der Units entscheidend. Eine Änderung am unbenutzten Helfer oder an der Basis-Unit ohne Berücksichtigung ihrer Drop-ins wirkt nicht.

Consumer-Aufnahme: User-Units `deadlock-bot-rust.service` (PID `314719`) und `deadlock-web-rust.service` (PID `32348`), System-Units `deadlock-twitch-bot-rust.service` (PID `1620085`) und `deadlock-twitch-dashboard-rust.service` (PID `1620199`). `steam-bot.service` läuft als User-Unit, PID `3423896`. Für mehrere fremde Prozessbenutzer war `/proc/<pid>/exe` nicht lesbar; keine vollständige Releaseherkunft dieser Consumer behaupten. Eine eigene geladene Docs- oder 2nd-Brain-Unit wurde in der Aufnahme nicht gefunden. K muss ihre tatsächlichen Aufrufwege und den produktiven Ort liefern.

## Zwei Releasepfade und der Herkunftsnachweis

| Pfad | Aufgenommenes Ziel | Aussage |
| --- | --- | --- |
| `/opt/deadlock-brain/maintenance-current` | `/opt/deadlock-brain/maintenance-releases/511a347b653beba13c2bf130f4bead7a7196cc2a` | Laufender Server und Maintenance verwenden diesen Pfad. |
| `/opt/deadlock-brain/current` | `/opt/deadlock-brain/releases/be2aa6bd5a6504e99693f7dd1edaa16e76be91b4` | Enthält `brain-legacy-import`, `brain-serve`, `deadlock-brain`, `deadlock-brain-yt` unter `bin/`. Der Basis-Serve-Pfad wird durch den Maintenance-Drop-in übersteuert. S rollt die CLI nach eigenem Merge aus. |

SHA-256 von `/proc/3506677/exe` und `/opt/deadlock-brain/maintenance-current/brain-serve` ist identisch: `e372407c267785a1679652c2dbfc4122bd69b0441db36424a6603c4dfeed7232`. Das Prozessziel enthält kein `(deleted)`. Der ASCII-Anker `brain-serve-readiness` ist vorhanden. Die beiden oben genannten Commit-SHAs sind im gelesenen Serverbinary nicht enthalten; eine eigene Brain-Buildherkunftssektion wurde nicht gefunden. `brain-serve --version` gibt laut `rust/crates/brain-serve/src/main.rs:16` die Paketversion aus.

Damit ist der verwendete Releasepfad und sein Hash belegt, die Zuordnung dieser Bytes zum Source-SHA jedoch nicht unabhängig bewiesen. Ein SHA im Verzeichnisnamen genügt dafür nicht. Für den G5-Kandidaten ein vorhandenes vertrauenswürdiges Buildmanifest mit Source-SHA, sauberem Quellbaum, Buildaufruf und Binary-Hashes verlangen. Fehlt dieses, muss Z die Herkunftsgrenze vor dem Cutover schließen. Kein Urteil nach Datei-Alter.

Bestandssuche: Graphify im Brain-Graph und global, danach Prüfung der tatsächlichen Dateien im Brain-Repo, einschließlich `ops/`, `scripts/`, `service/` und weiterer verfolgter Pfade. Im aufgenommenen Brain-Stand wurde kein Brain-Deploy-Wrapper gefunden, der Releaseinstallation, aktuellen `origin/main` und Deploy-flock gemeinsam erzwingt. Ein solcher Weg darf nicht als vorhanden angenommen werden. Z muss den von S verwendeten bestehenden Release-Weg mitsamt Herkunfts- und Sperrnachweis übernehmen, bevor hier ein ausführbarer Installationsbefehl ergänzt wird.

Der vorhandene `/usr/local/bin/deploy-twitch-release` hält `/run/lock/deploy-twitch-release.lock` und ruft `/usr/local/sbin/install-twitch-release` auf. Der Installer prüft den angegebenen Checkout-SHA und die eingebettete `.twitch_build`-Revision. Er ist Twitch-spezifisch und kein Brain-Installer. In diesen beiden aufgenommenen Dateien wurde kein Abruf oder Vergleich des aktuellen Remote-main gefunden. Für K gilt daher zusätzlich dessen repoeigener Nachweis des aktuellen `origin/main`; nicht aus dem Wrappernamen ableiten. Keinen Twitch-Wrapper auf Brain-Artefakte anwenden.

## PostgreSQL, Schema und Bestandserhalt

Laufende System-Unit: `deadlock-brain-postgresql.service`, PID `3757584`, PostgreSQL 16, Konfiguration `/etc/deadlock-brain/postgresql/postgresql.conf`. Die Runtime-Konfiguration des Servers bindet die DB `brain` über `/run/deadlock-brain-postgresql`, Port `5446`, Rolle `brain_service` ein. Maintenance nutzt dieselbe DB mit `brain_ingest`. Der Datenpfad aus `ops/brain-postgres/postgresql.conf:22` lautet `/var/lib/deadlock-brain/postgresql`; die unabhängige Live-Verifikation des Datenverzeichnisses ist noch zu ergänzen.

Der alte Writer-Helfer verwendet `DEADLOCK_CENTRAL_DSN`. Dessen Wert wurde nicht ausgegeben. Der vorhandene Archivimport `ops/brain-postgres/legacy-import.sh` liest DL-Main, DB `deadlock`, Schema `brain`, und kopiert nach `brain_legacy` in die isolierte Instanz. DL-Main und Brain-Kern sind unterschiedliche Speicherziele. Das historische Archiv vom September beweist keine Übernahme späterer Legacy-Schreibvorgänge. Keine handgeschriebenen Datenkorrekturen, kein Löschen alter Daten.

Vorhandene Bausteine:

- `brain-migrate <check|up> --config <nicht geheime lokale PostgreSQL-Konfiguration>` in `rust/crates/brain-storage/src/bin/brain-migrate.rs`. Der Migrator ist vom Dienststart getrennt und verlangt expliziten lokalen Socket und Rolle.
- `rust/crates/brain-storage/src/schema.rs:7` erwartet Kernschema v2. Es bettet die Migrationen vom 24., 25. und 26.09.2026 mit `include_str!` ein und migriert transaktional mit Advisory-Lock. Maintenance bettet seine vier Migrationen vom 02.10. in `pg_maintenance.rs` ein.
- `brain-serve/src/service.rs:153` und `health.rs:71` prüfen das Schema. Der Server ersetzt keinen administrativen Migrator.
- `brain-legacy-import --observe-snapshot --config <gebundene Konfiguration>` sowie `brain-legacy-import --config <gebundene Konfiguration>` sind vorhandene Wege für Snapshotbeobachtung und Kernimport. Produktivbindung und Schema-/Rollenidentität werden im Importer geprüft.
- `ops/brain-postgres/legacy-core-cutover.json` ist eine gesperrte Vorlage: leere Snapshot-/Releasefelder, `production_binding: null`, Bericht unter `/DO_NOT_RUN/APPROVED_PRIVATE_REPORT.json`. Nicht unverändert ausführen. `legacy-core-import.json` beschreibt einen historischen Pilot in `brain_pilot_legacy`, keinen Produktiv-Cutover.
- `deadlock-brain-postgresql-backup.service` startet `/usr/local/lib/deadlock-brain/postgresql-backup.sh`; Timer aktiv, nächster aufgenommener Lauf am 04.10. gegen 02:47 Uhr. `ops/brain-postgres/backup.sh` bietet Snapshot, Fingerprints und `SHA256SUMS`; `restore-probe.sh` vergleicht in einer privaten Scratch-Instanz. Standardablage: `/var/backups/deadlock-brain/postgresql`.

Offene Datenmechanik: Ein wiederholbarer Frischeabgleich von DL-Main zum bereits vorhandenen `brain_legacy`-Archiv ist nicht belegt. `legacy-import.sh:41` löscht eine feste Staging-DB und importiert anschließend in das vorhandene Archivschema; es ist kein belegter inkrementeller Cutover-Abgleich. Nicht blind erneut starten. Z muss den vorhandenen sicheren Übergang einschließlich neuer Snapshotbindung und Erhalt der Zwischenzeit-Daten nachweisen. Das Backup der isolierten DB ersetzt außerdem keine Sicherung des zentralen Legacy-Bestands.

Die zusätzliche Bestandsprüfung konkretisiert vier Stop-Bedingungen:

1. Der vorhandene Dump-/Hashweg verwendet einen gemeinsamen lesenden DL-Main-Snapshot. Der Zielimport enthält aber erneut die Schemaanlage; bei einem bereits vorhandenen `brain_legacy` beendet `ON_ERROR_STOP` die Zieltransaktion. Ein generationeller Archivwechsel mit eindeutigem Staging und erhaltenem Vorgänger fehlt. Z darf das bestehende Archiv nicht löschen, um einen erneuten Lauf passend zu machen.
2. `brain-legacy-import --observe-snapshot` liest das Archiv und keine neue DL-Main-Kopie. Ein neuer Beobachtungszeitpunkt beweist deshalb keine Quellfrische. `cutover.rs:166` verlangt außerdem bestehende Heads passend zur neu gebundenen Projektion; ein neuer Snapshot über bereits importierten Heads braucht einen expliziten Altbindung-zu-Neubindung-Vertrag. Der aktuelle Schutz darf nicht entfernt werden.
3. Der vorhandene V1-Binder setzt Widerrufs- und Tombstonelisten leer. Er ersetzt keine belegte Übernahme inzwischen erfolgter Löschungen oder Rechteänderungen. Q muss diesen Übergang im bestehenden Importer mit monotonen Revisionen, erhaltenen Tombstones und nachvollziehbaren Checkpoints schließen.
4. Das aktuelle Importrelease enthält die beiden Legacy-Quellen. Vor einer Serve-Umschaltung muss ein zusammengesetzter Corpus unberührte Quellenpins erhalten. Archiv-Snapshot-SHA und Kern-Snapshotdigest sind unterschiedliche Nachweise. Ein fehlgeschriebener Report beweist keinen fehlgeschlagenen DB-Commit; vor einer Wiederholung den persistierten Importzustand prüfen.

Q besitzt die betroffenen Importer-/Storage-Pfade, Z die Archivaufnahme und das Runbook. Änderungen an gemeinsamen Manifesten oder Q-Code brauchen den Bereichsvertrag. Die vorhandene hashgeprüfte Serve-Konfigurationsumschaltung und erhaltene Releasepins liefern den Ansatz für den Rückweg. Neue Kernrevisionen, Checkpoints und Publikationsstände dürfen dabei nicht durch ein Vor-G5-Backup überschrieben werden; Wiederherstellung zunächst in einer separaten Probe prüfen.

Reihenfolge für notwendige Schemaänderungen: auf main gemergte Migration, Migrator aus genau diesem Stand neu bauen, Backup-/Restore-Nachweis, administrativer Migrationslauf, lesender Schema-/Rechtecheck, danach abhängigen Code starten. Angewandte Migrationen bleiben unverändert. Wiederherstellung vorläufig in separater DB prüfen; kein pauschales Downgrade und kein Überschreiben gemeinsam genutzter Prod-Daten.

## G5 in kontrollierten Schritten

Die folgenden Schritte sind Arbeitsanweisungen für Z nach geschlossenen Startbedingungen. Die aktuelle Aufnahme hat keinen dieser Schreibschritte ausgeführt.

| Schritt | Aktion und messbarer Endzustand | Rückweg / Stop |
| --- | --- | --- |
| 1. Kandidat festlegen | Aktuellen Remote-main frisch holen, Q/P/S/K-Übergaben diesem Stand zuordnen. Nach lokalem Merge-Gate von sauberem eigenem Releasebaum bauen. CLI, Server, Maintenance und die von P/Q verwendeten Writer müssen ihre Herkunft und ihr tatsächliches Datenziel ausweisen. | Kein Umschalten bei fehlendem Manifest, offenem SLO oder uneindeutiger Quelle. Bestehende Pfade bleiben unverändert. |
| 2. Rückweg sichern | Aufgelöste Ziele beider Brain-Releaselinks, Binary-Hashes, Basis-Units und Drop-ins, Enable-/Active-Zustände, Serve-Konfiguration und die aktive Corpus-Releasebindung privat sichern. Dateien mit Zugängen nicht in Git oder Akten kopieren. Backups beider Datenbestände und lesbaren Restore-Nachweis verlangen. | Vorhandene Targets behalten. Kein Löschen von Releasebäumen, Unit-Dateien, Archiv oder Legacy-Tabellen. Ohne nutzbare Sicherung stoppen. |
| 3. Writer quieszent machen | Je umzuschaltender Quelle zuerst den Timer stoppen, danach vorhandenen Lauf geordnet beenden und den Service stoppen. Das gilt auch für Maintenance, wenn sie dieselbe Freigabe oder Konfiguration ändern kann. PID, `MainPID=0`, Cgroup-Leere und ausstehende Jobs prüfen. Erst dann neuen Writer starten. | Stop ist die vorläufige Pause, die endgültige Deaktivierung folgt G6. Bei Fehlschlag neuen Writer stoppen, alte Unit/Konfiguration zurücklegen und ursprünglichen Timerzustand herstellen. Keine parallelen Writer. |
| 4. Daten und Schema binden | Frischen Snapshot nach belegtem Übergangsweg sichern; Import an Rollen, Datenbank-/Schemaidentität, Snapshotdigest, Rechte und Release binden. Migrationsbedarf getrennt behandeln. Importreport, Revisionen, Tombstones und letzte verarbeitete Checkpoints prüfen. | Bei Abweichung neue Jobs pausiert lassen. Erhaltenes Archiv bleibt stehen. Teilweise Daten nicht per Hand löschen. Kein Rücksprung auf einen inkompatiblen Serverstand. |
| 5. Neue Writer einzeln aktivieren | P belegt Publisher plus Feed und Patchnotes-Sync; Q belegt Sheet-/YouTube-Kernpfad und seine Pausen-/Rechteregeln. Wiki und Build-Data brauchen einen konkret benannten Ersatz oder eine erklärte nichtblockierende Grenze. Alter Lauf ist bereits aus. Pro Quelle genau ein Scheduler. | Bei fehlerhaftem Import zuerst neuen Timer und neuen Service stoppen, danach alten Weg wiederherstellen. Bei Discord-/Steam-Seiteneffekten zuerst deren persistierten Zustell-/Publishstand prüfen; kein automatischer Wiederholungs-Post. |
| 6. Server ausrollen | Bestehenden verifizierten Brain-Release-Weg verwenden, beide Releasepfade berücksichtigen. Effektiven Serve-Drop-in und tatsächliche Configbindung prüfen, Server geordnet neu starten. Maintenance auf denselben aktuellen Stand ausrollen, soweit der gemeinsame Stand sie betrifft. | Vorherigen Binarypfad, Konfiguration und Corpusbindung gemeinsam wiederherstellen, dann neu starten. Vor Rücksprung Schemakompatibilität prüfen. Ohne geprüften Installationsweg hier stoppen. |
| 7. Consumer und SLO abnehmen | `/healthz` und `/readyz` mit JSON-Typ und Inhalt prüfen. K führt reale berechtigte Anfragen aus Bots, Docs, 2nd-Brain und Twitch aus; Serverereignis und Request-ID abgleichen. Q-Lastprofil vollständig auswerten. P nutzt einen echten Patch oder freigegebenes Vorschau-Replay ohne Discord-Post; S liefert einen echten Publish mit `hero_build_id`. | Fehlende Consumerzeile, falsche Releasebindung, Rechteleck oder SLO-Verletzung stoppt G5. Consumer-Konfiguration auf ihren gesicherten alten Stand zurücklegen; neue Writer vor Legacy-Reaktivierung stoppen. |
| 8. Beobachtungsbeginn | G5-SHA, Zeitpunkt, Writerliste, Corpusbindung, Q-Abnahme und K-Anfragen festhalten. Gewünschte neue Timer starten, alte Gegenstücke bleiben pausiert. Kein endgültiges Entfernen der Legacy-Pfade. | Ein Fehler startet die fehlerfreie Beobachtung für den betroffenen gemeinsamen Stand neu. Rückweg aus Schritt 3 bis 7 bleibt verfügbar. |

Die Betriebskommandos laufen pro Service im richtigen Scope. Beispiel für einen alten User-Timer und seinen Lauf, erst bei zulässigem Cutover ausführen:

```bash
systemctl --user stop deadlock-brain-build-data.timer
systemctl --user stop deadlock-brain-build-data.service
systemctl --user show deadlock-brain-build-data.service --property=MainPID,ActiveState,SubState,Result
```

`After=network-online.target` und `Wants=network-online.target` ersetzen weder DB-Bereitschaft noch eine Importfreigabe. `brain-serve.service` hat in der Aufnahme keine Abhängigkeit zur isolierten PostgreSQL-System-Unit. User- und System-Manager bilden unterschiedliche Abhängigkeitsräume. DB-Bereitschaft über den vorhandenen Schema-/Ready-Pfad prüfen. `dl-brain-feeder` nennt zusätzlich `infisical.service`; die Backup-System-Unit hat `Requires` und `After` auf `deadlock-brain-postgresql.service`.

## Host-Sperren vor Compilerläufen

In dieser Dokumentationsaufnahme wurde nicht gebaut oder getestet. Für spätere Builds gilt `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md`:

```bash
exec 8>/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock
flock -x 8
exec 9>/tmp/deadlock-cargo-release.lock
flock -x 9
```

Beide Sperren während des gesamten Prüfschritts halten. Unmittelbar vor Compilerstart Prozessprobe durchführen. Aktive `rustc`, `clippy-driver`, `rustdoc` und unbekannte Cargo-Aufrufe blockieren; Zombies zählen nicht. Die enge Ausnahme ist der exakt belegte Aufruf `cargo metadata --format-version 1 --no-deps --manifest-path /absoluter/pfad/Cargo.toml` ohne weitere Argumente. Bei Unklarheit blockieren. Keine Prozessumgebungen und keine vollständigen Argumentlisten ausgeben.

Bei Exit 75 Sperren halten, 30 Sekunden warten und frisch prüfen; fremde Prozesse nicht beenden. RAM mit `free -m`, Platte am eigenen Buildpfad mit `df -h` prüfen. Kein Releasebau im zugesagten Challenges-Debugslot. Release aus dem eigenen freigegebenen Baum bauen, höchstens zwei Jobs. Diese Hostlocks sind keine belegte Brain-Deploysperre.

```bash
exec 9>&-
exec 8>&-
```

Kein weiterlaufendes Kind darf die Locks behalten.

## Live-Beweis und Funktionsbeweis

Vor und nach dem tatsächlichen Neustart den PID erfassen. Nach dem Wrapper-`exec` prüfen, nicht während noch Bash läuft. Prozessbinary muss ohne `(deleted)` auf das freigegebene Release zeigen; dessen SHA-256 mit dem Buildmanifest und dem installierten Artefakt vergleichen. Commitbezug und ASCII-Anker getrennt belegen.

```bash
systemctl --user show brain-serve.service --property=MainPID,WorkingDirectory,FragmentPath,DropInPaths
readlink /proc/3506677/exe
sha256sum /proc/3506677/exe
journalctl --user -u brain-serve.service -p err --since "2026-10-03 16:20:00" --no-pager
```

Die PID und Zeit in diesem lesenden Beispiel sind der Aufnahmestand, kein späterer Deploynachweis. Journald-Inhalte vor Ablage redigieren. Keine Zugänge, Header, Anfrage-/Antworttexte oder personenbezogenen Daten speichern. Weitere betroffene Units jeweils im passenden User-/System-Scope prüfen. Ein leeres `journal -p err` beweist nicht, dass ein Timer tatsächlich gearbeitet hat; Lastlauf, Checkpointfortschritt und erwarteter Heartbeat kommen hinzu.

Aufnahme: `GET http://127.0.0.1:8788/healthz` lieferte `200`, `application/json`, `status=ok`; `/readyz` lieferte `200`, `application/json`, `status=ready`. Der Ready-Code prüft Schema, erwarteten Snapshot und DB-Rechte. Für die aufgenommenen Brain-Units einschließlich Patchnotes war das lesbare User-Journal mit `-p err` seit 02.10.2026, 16:30 Uhr leer. Dies ist keine fehlerfreie Beobachtung des noch ausstehenden G5-Kandidaten.

API-Routen auf main: `POST /v1/answer` und `POST /v1/retrieve` (`brain-api/src/http.rs:21`), Health-Routen in `brain-serve/src/health.rs:108`. Berechtigte Funktionsanfragen über vorhandene Consumer-Konfiguration und zentrale Zugangsverwaltung ausführen; keine Schlüssel in Befehlszeilen oder Bericht. K ergänzt je Consumer die tatsächlich aktive URL beziehungsweise den privaten Socket, UI-Ort und die dazugehörige Serverzeile. Für private 2nd-Brain-Daten den neuen privaten Transport verlangen. Keine erfundene Socketadresse eintragen.

Die finale Beweiszeile wird erst nach Ausführung mit echten Werten ausgefüllt:

`LIVEBEWEIS[DV-1]: PID <alt>-><neu> | exe ohne (deleted) | journal -p err leer | Anker "<belegter ASCII-Anker>" in Binary | Funktion: <Consumer- und Writer-Ergebnis> | Ort: <aktive URL und UI-Ort>`

## G6 nach dem fehlerfreien Timerzyklus

G6 startet frühestens 24 Stunden nach erfolgreichem G5 und erst, wenn jeder relevante Nachfolger seinen natürlichen vollständigen Zyklus ohne Fehler durchlaufen hat. Die tägliche Build-Data-Ausführung um 03:30 Uhr, der Backup-Timer und der sechsstündliche Wiki-/YouTube-Takt müssen im Belegfenster enthalten sein. Der bestehende `dl-brain-feeder.timer` ist wöchentlich. Falls er für den übernommenen Funktionsumfang relevant bleibt, reicht ein beliebiges 24-Stunden-Fenster nicht: den Sonntagslauf abwarten oder vorher den abgegrenzten Weg als nichtblockierende Grenze dokumentieren. Ein manuell ausgelöster Lauf ersetzt den Schedulingnachweis nicht.

Für jeden Timer: tatsächlicher Auslösezeitpunkt, erfolgreicher Abschluss, Checkpoint oder erwarteter No-change-Zustand und leeres Fehlerjournal seit G5. Wartende Timer, `Result=success` aus einem alten Lauf und bloßes `enabled` reichen nicht. Fehlender Lauf oder Fehler hält G6 offen. Eine ausdrücklich pausierte Lernfunktion bleibt als Grenze sichtbar.

Dann die verbliebenen, bestätigten Legacy-Writer einzeln stilllegen:

1. Alten Timer stoppen, alten Service stoppen, Writerfreiheit belegen und genau diese alten Units deaktivieren. Keine DB, keinen neuen Rust-Dienst und keinen weiter benötigten unabhängigen Reader deaktivieren.
2. Basis-Unit und wirksame Drop-ins privat mit Besitzern, Dateimodus und Hash sichern. Bei einer endgültig stillgelegten Unit die Basisdatei als `<unit>.disabled` behalten; die alte Drop-in-Struktur ebenfalls außerhalb des aktiven `<unit>.d/`-Pfads sichern. `systemctl --user daemon-reload`, danach Zustand und ausbleibende alte Läufe prüfen. Bestehende `.disabled`-Sicherungen nicht überschreiben.
3. Bei von P/Q unter demselben Unitnamen ersetzten Diensten bezeichnet `.disabled` die alte Fassung. Die neue Rust-Unit und ihr Timer bleiben aktiv. Kein pauschales `disable deadlock-patchnotes.service` nach P-Abschluss.
4. Nachgewiesen unbenutzte Python-Legacy-Codepfade aus dem Brain-Repo entfernen, mit getrenntem Merge-Gate und aktuellem main. Der Python-Site-Dienst gehört einem anderen Repo und ist kein stillgelegter Writer. Solange er oder ein unbekannter Consumer einen Legacy-Leser braucht, die Abhängigkeit als Grenze führen.

Konkreter mechanischer Rückweg für eine endgültig stillgelegte User-Unit: neuen Gegenspieler zuerst stoppen; gesicherte `<unit>.disabled` unter dem ursprünglichen `<unit>`-Pfad wiederherstellen, ursprüngliche Drop-in-Struktur und Dateimodi zurücklegen, `systemctl --user daemon-reload`, die vor dem Cutover aktivierten Timer wieder aktivieren und starten. Vorher DB-/Schema- und Checkpointkompatibilität prüfen. Bei einmaligen Publikationen die persistierten Zustellstände erhalten. Keine erneute Discord-Auslieferung als Rückweg.

Pfadbeispiel für den bisherigen Wiki-Writer, erst nach belegtem Ersatz und G6-Freigabe: `/home/nathanael/.config/systemd/user/deadlock-brain-wiki-refresh.service` und `.timer` unter den Namen `deadlock-brain-wiki-refresh.service.disabled` und `deadlock-brain-wiki-refresh.timer.disabled` sichern. Falls zwischenzeitlich Drop-ins hinzugekommen sind, deren Verzeichnisse unter den entsprechenden `.service.d.disabled` beziehungsweise `.timer.d.disabled`-Namen sichern. Bei Wiederherstellung die beiden ursprünglichen Dateinamen und Drop-in-Verzeichnisse zurücklegen, `systemctl --user daemon-reload` ausführen und den zuvor aktivierten `deadlock-brain-wiki-refresh.timer` mit `systemctl --user enable deadlock-brain-wiki-refresh.timer` und `systemctl --user start deadlock-brain-wiki-refresh.timer` wieder aufnehmen. Vorhandene Sicherungen behalten. Diesen Ablauf nicht auf eine unter demselben Namen bereits aktive neue Rust-Fassung anwenden.

Der Dateirückweg und die Backup-/Restore-Bausteine sind vorhanden. Ein vollständiger Rückweg für den aktuellen Produktiv-Datenimport und sämtliche Consumer ist noch nicht bewiesen. Die entsprechenden Startbedingungen dürfen daher nicht mit einem pauschalen „Rollback vorhanden“ abgehakt werden.

## Abschlussbelege für Z

G5 erst nach Q-SLO, K-Consumerbeweis, den einzelnen Writerbelegen und geschlossenem Daten-/Releaseübergang eintragen. G6 erst nach dem Timerfenster und belegter Stilllegung. `STATUS.md`, `GATES.csv` und Abschlussbericht müssen denselben live geprüften SHA, die Corpusbindung, verbleibende Grenzen und den Rückweg nennen. GitHub Actions sind kein Gate. Bestehende Schutz-Hooks bleiben aktiv. Branch-/Worktree-Cleanup folgt getrennt nach dessen Nachweis und gehört nicht zu dieser lesenden Aufnahme.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-storage/src/schema.rs:7 | Anknüpfung: vorhandene Migratoren, Importbindung, Backup/Restore, Serve- und Maintenance-Units; fehlende Brain-Deploy- und Frischemechanik bleibt Stop-Bedingung

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: architecture/migration/CUTOVER_RUNBOOK.md
