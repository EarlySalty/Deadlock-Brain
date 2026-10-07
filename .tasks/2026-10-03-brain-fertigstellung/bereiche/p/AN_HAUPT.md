status: aktiv
Datum: 2026-10-03

# Paket P: Schnittstelle und Betriebsbefund

Worktree `/home/nathanael/.worktrees/patchnotes-rust-fertig`, Branch `feat/patchnotes-rust-fertig-20261003`, HEAD `ba9d104`. 32 vorhandene Rust-Port-Commits übernommen; noch kein Merge oder Deploy.

P richtet den vorhandenen Vertrag `brain.feed.patchnotes.v1` als Rust-HTTP-Feed ein: `/v1/brain/patchnotes/v1` und `/healthz`, vorgesehen `127.0.0.1:8791`, vorhandene Schlüssel `BRAIN_FEED_API_KEY` und `PATCHNOTES_FEED_READONLY_DSN`. Keine Änderung am Feed-Vertrag geplant. Der Leser muss weiter strikt read-only bleiben.

Betriebsaufnahme: Der aktive `deadlock-brain-patchnotes-sync.timer` startet effektiv `/home/naniadm/.local/bin/deadlock-brain-patchnotes-sync.sh`, nicht den im Briefing genannten nathanael-Pfad. Der nathanael-Helfer nutzt noch Python-Secret-Export und direkte PostgreSQL-Abfragen. P prüft den effektiven Pfad und stellt die Unit auf den vorhandenen Rust-Feed-Verbraucher um. Noch keine Dateien in Deadlock-Brain geändert. Falls dafür Codeänderungen im Brain nötig werden, folgt vor dem Schreiben eine genaue Pfadankündigung.

Der Publisher läuft noch als Python-PID `2721773`, DevFeed auf `8790`, Feed-Port `8791` ohne Listener. Cutover erst nach Rust-Live-Beweis, ohne Test-Post in echte Kanäle. Replays für Patches 288, 287 und 285 sind über den streng lesenden Feed-DSN identifiziert.

## Wiederaufnahme und notwendiger Brain-Consumer

15:13 UTC: Sitzungsunterbrechung, drei Bau-Workflows gestoppt, Teilstand erhalten und geordnet wieder aufgenommen. Kein Merge, Deploy oder Produktionswrite. Der effektive naniadm-Sync-Pfad ist laut abschließendem Scout dieselbe Datei wie nathanael, Device und Inode stimmen überein.

Bestand auf Brain origin/main: `brain-feeds::patchnotes::{parse_feed,prepare_batch}` und der Feed-Vertrag sind vorhanden, aber kein Produktionsaufrufer. `deadlock-brain pull patchnotes` liest weiterhin direkt die zentrale PostgreSQL statt des HTTP-Feeds. Der vorhandene `brain-match-ingest` zeigt den nativen Store-/Lease-/Release-Weg.

P benötigt deshalb die eng begrenzte neue Datei `rust/crates/brain-feeds/src/bin/brain-patchnotes-ingest.rs` in einem eigenen Brain-Worktree `~/.worktrees/brain-patchnotes-rust-sync`, Branch `feat/brain-patchnotes-rust-sync-20261003`. Wiederverwendung der vorhandenen Parser-/Batch-/PgStore-APIs, Auth und bounded loopback HTTP, keine neue Feed-Struktur. Keine Änderung an Workspace-Cargo.toml, Cargo.lock oder dem Build-Publish-Teil geplant. Falls doch ein geteilter Pfad nötig wird, folgt vorher eine gesonderte Meldung. Die Sync-Unit erhält danach diesen Rust-Einstieg statt Shell/Python.

Release-Vertrag für Z: Der Consumer muss auf dem jeweils aktiven Core-Release aufsetzen, fremde Sources erhalten und den aktualisierten Patchnotes-Stand atomar veröffentlichen. Keine hardcodierte alte Release-ID und kein separater Legacy-Writer. Auf dem geprüften main ist `brain_serve::Config.release` weiterhin fest an `id` und `knowledge_version` gebunden; ein bloßes `PgStore::commit_batches_and_publish` aktiviert neue Daten daher nicht im laufenden Serve. Bestehende Aktivierung liegt in `brain-maintenance/src/integration/{activation,config_writer}.rs`. P übernimmt diesen vorhandenen Vertrag, keine zweite Aktivierungslogik.

Eigener Brain-Worktree ist inzwischen auf `511a347` erstellt, noch sauber. Z soll den für den Cutover vorgesehenen Runtime-Konfigpfad und den bestehenden Aktivierungsweg für wiederkehrende Patchnotes-Importe nennen. Das ist eine Integrationsvoraussetzung, keine neue Produktentscheidung. Kein Wartefenster, die unabhängige Patchnotes-Arbeit läuft weiter.

## Übernahme bestätigt, Status 6

`UEBERNAHME-CODEX.md` und die neue Antwort in `VON_HAUPT.md` gelesen. Hauptorchestrierung jetzt Codex /root, Thread `e6c19079-657e-4db9-80bd-8e1313e7f785`. Bestehende Arbeit bleibt erhalten. Gekoppelte Writer-/Consumerpfade gehen vor Merge durch gemeinsame unabhängige Abnahme; Z verantwortet den gemeinsamen Produktivwechsel. Keine parallele Releasezeiger-Umschaltung durch P.

Patchnotes-HEAD `ba9d10425f9c98f765cf79429c2b9eda12ab2541`, Brain-Consumer-HEAD `511a347`. Quellenbau ist beendet, aber nicht Cargo-geprüft: `host-checks.lock` verhinderte den Compilerstart. Vier Promptvarianten bytegenau belegt; gezieltes rustfmt und diff-check grün. Python-freier Dependency-Tree, Compiler, Parität und echter Provider-Aufruf bleiben offen. Runtime und DevFeed bauen weiter. Der freie dritte native Workerplatz geht an den angekündigten Feed-Consumer, ohne Live-Writeränderung.

Zs `AN_HAUPT.md` vom 15:12 UTC gelesen. Benötigter Vertrag für Z: tatsächlicher `serve_config`-Pfad aus normaler Runtime-Konfiguration; bestehender journalgebundener Aktivierungsaufruf für wiederkehrende Imports; welcher bestehende Lease-/Aktivierungslock konkurrierende Source-Releases serialisiert; Erhalt aller fremden Source-Pins bei Aktivierung. P schreibt weder `activation.rs` noch `config_writer.rs` um. Ein bloßer Store-Commit gilt nicht als Aktivierung im Serve.

Nächster Arbeitsschritt: geschützte Quellenprüfungen übernehmen und den begrenzten HTTP-Feed-Consumer auf dem eigenen Brain-Worktree bauen. Noch kein Merge oder Deploy. Status 6 benennt die um vier Minuten verspätete Meldung offen.

## Status 8 und gebundener Betriebsvertrag

Quellen-Cargo-Check `b2j4ickby` ist mit Exit 0 abgeschlossen: patch-sources, patchnotes-source und patchnotes-content jeweils `--all-targets -j 2`, keine Compilerwarnungen oder Fehler. Fmt, striktes Clippy, vorhandene Vertragsprüfungen und Dependency-Tree laufen als angeforderte geschützte Folgeprüfkette `bizcakhjq`; noch kein Ergebnis. Runtime, DevFeed und Brain-Consumer weiter in Arbeit. Kein Gesamtabschluss aus dem grünen Quellencheck.

Qs `AN_HAUPT.md` und Zs `BETRIEBSVERTRAG.md` vom 16:20 UTC gelesen. serve_config ist jetzt auf `/home/nathanael/.config/deadlock-brain/brain-serve.json` gebunden; Runtime-Einstieg `/opt/deadlock-brain/maintenance-current/brain-maintain --config /etc/deadlock-brain/maintenance-runtime.json`. Der Pfadblocker ist geklärt. P hält den fachlichen HTTP-Feed-Import in `brain-feeds`; Z baut und integriert den gemeinsamen Kandidatenadapter mit dem vorhandenen Publikationslock, ConfigWriter-Lock, Aktivierungsjournal, Restart und Readiness. Keine getrennte P-Aktivierung und kein direkter Config-Writer.

Für die abschließende Verdrahtung benötigt P den konkreten Adapter-Aufruf und dessen erwartetes Kandidatenmanifest. P liefert dabei Basishash, aktuellen Basisrelease, Kandidatenrelease und unveränderte fremde Source-Pins. `activation_performed=false` bleibt wahr, bis Zs vorhandener Aktivierungsweg den erwarteten aktiven Serve-Stand tatsächlich bestätigt. Die Q-Socketbindung `/run/deadlock-brain-postgresql:5446`, Datenbank brain, Writerrolle brain_ingest und Infrastrukturzugang über bestehenden Credential-FD-/Infisical-Weg werden nicht durch freie Writer-DSNs ersetzt.

Die Übergabe erfolgt mit lokal geprüften vollständigen SHAs als `uebergeben`, vor dem späteren eigenen Live-Nachweis. Z übernimmt gemeinsame Integration, Abnahme, Gate und Installation. Keine Units oder produktiven Zeiger geändert.

## Status 9: eindeutiges Ziel und Source-Kandidat

Aktuelle Antwort bis 16:43 UTC gelesen. Ps Aktivierungsziel ist ausschließlich das Standard-/Patchnotesrelease. Qs interne Second-Brain-Felder und unabhängige C9-/Docs-/Twitch-Grants bleiben unverändert. Kein direkter Nachrichteneingriff in laufende Turns; vorhandene Quellenprüfkette bleibt erhalten, künftiges reines Hostlock-Warten ohne Zeitlimit.

Brain-Erstworker ist beendet und hat im alleinigen neuen Einstieg schreibfreies `preview` und `validate` mit begrenztem authentifiziertem Loopback-HTTP geliefert. Noch kein Compilerbeweis. Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`, Datei-Erststand SHA256 `ac3d2bee5ac9b8df1f9550a694f6c9cb4ccc15e9b66fe5bf39875acd3030616f`. Acht vorbereitete Tests nicht ausgeführt. Diese Vorschau allein erfüllt den Timer-/Writerauftrag nicht.

Nach seinem Ende ergänzt ein neuer nativer Worker im selben exklusiven Dateipfad den fachlichen Kandidatenmodus: Source-Lease, vorheriger Checkpoint, basishashgebundener CorpusRelease, fremde Pins erhalten, atomarer PgStore-Commit und `activation_performed=false`. Keine neuen Manifest-/Maintenancepfade und keine eigene Aktivierung. Konkretes CLI-/Manifestformat folgt mit dem geprüften Stand. Z bindet diesen Kandidaten an seine gemeinsame Standardziel-Aktivierung.

## Status 10: Quellen tatsächlich geprüft

Geschützte Quellenprüfkette `bizcakhjq` Exit 0. Source/Config/Content: Fmt und striktes Clippy ohne Warnungen oder Fehler, 25/21/38 tatsächlich bestandene Tests. Insgesamt 84 passed, 0 failed, 0 ignored, 0 filtered; jeweils `--include-ignored`. Keine Altfehlerbaseline behauptet. Source dependency-tree ohne pyo3-/python-sys-/libpython-Einträge. Voller Bericht und Befehle in `.tasks/2026-10-03-paket-p-rust/BAU-QUELLEN.md` im Patchnotes-Worktree.

Das vollständige Bot-Binary, DevFeed und Brain-Kandidat sind damit noch nicht abgenommen. Echtdatenparität und realer Provider-Aufruf bleiben offen. Keine neue Veröffentlichung oder installierte Unitänderung. Übergabe erst mit lokal geprüftem vollständigem SHA, Live-Nachweis nach Zs gemeinsamer Installation.

## Geprüfter Brain-Baustein für Z

Brain-Kandidatenworker beendet. Check und striktes Clippy Exit 0, 16 Binarytests und 3 bestehende Patchnotes-Tests bestanden; 0 failed/ignored, 16 andere Libraryfälle gezielt gefiltert. Eigenes Brain-Worktree ist sauber auf Commit `1a5b2b33ec9f8c79a073fe67c083c14232289a2b`. Einzige Produktdatei `rust/crates/brain-feeds/src/bin/brain-patchnotes-ingest.rs`, verifizierter Datei-SHA256 `d443fe3c76eabac50174cbd4084ab215c2c01ce586ddec149f33b8dd5d87202f`.

Konkreter Aufruf und JSON-Vertrag in `bereiche/p/BRAIN-KANDIDAT.md`: `stage-candidate --config <absoluter-pfad>`, Basisrelease und serialisierter Basishash, Infisical-FD-5-Zugang, explizite öffentliche Source-Policy. Ergebnis deklariert `allowed_changed_sources=["patchnotes-feed"]`, `activation_target="standard"`, `activation_performed=false`. Bereits committed, noch nicht aktivierte Source-Pins gehen bei unverändertem Feed nicht verloren. No-change erzeugt keinen Release.

Z kann diesen geprüften Baustein in den gemeinsamen Adapterstand übernehmen. Es gibt keinen PostgreSQL-Integrationstest, Produktionsimport oder aktiven Readernachweis. Kein Einzelmerge/Push/Deploy. P insgesamt bleibt aktiv, bis Runtime/DevFeed und die Echtdaten-/Provider-Nachweise lokal geprüft vorliegen; die vollständige Paketübergabe folgt danach.

## Frischer Cutover-Ausgangsstand für Z

18:41 bis 18:43 UTC nur lesend aufgenommen: `deadlock-patchnotes.service` weiter enabled/active, PID 2721773 `/usr/bin/python3.12`, unveränderter Live-Worktree und TOML. Voller Betriebsvertrag im P-Worktree unter `.tasks/2026-10-03-paket-p-rust/BETRIEBSVERTRAG.md`, inklusive Unit-/Drop-in-Ausgangshashes und Rückweg als `.disabled`.

Wichtig für die Installation: vorhandene `60-global-toml.conf` und `70-release-main.conf` setzen ExecStart. Die native Vorlage `20-native.conf` muss nach ihnen wirksam installiert werden, beispielsweise als `90-native.conf`, oder die ersetzten Overrides gesichert aus der aktiven Reihenfolge nehmen. Bestehendes `20-creds.conf` mit LoadCredential erhalten. Die Live-TOML hat weiter `brain_feed.enabled=false`; der gemeinsame Cutover muss die vereinbarte Feed-Aktivierung tatsächlich binden. Keine Units oder Konfigurationen durch P verändert, kein Rust-Live-Beweis.

## Status 14: tatsächliche Prüfergebnisse, noch kein Abschluss

18:32 UTC: aktuelle eigene Runtime-/DevFeed-Transkripte ohne Unterbrechungsereignis geprüft. Runtime untersucht einen konkreten Bot-Testfehler: `ingestion_is_atomic_and_historical_previews_remain_suppressed`, 20 passed, 1 failed. Bot- und Storage-Clippy melden Fehler, Presentation-Clippy bereits Exit 0. Storage-Vertrag bisher sechs Unit- und ein PostgreSQL-Test bestanden; Gesamtprüfkette offen. Kein grüner Gesamtstand behauptet.

DevFeed-Prüfer wartet weiter auf bestehende Host-Sperre. Eigene Wrapper-/Warteprozesse PID 3249768/3249774 tatsächlich lebend bestätigt. Keine doppelte Prüfkette, kein Abbruch wegen Lock-Wartezeit, keine Nachricht in laufende Turns. Status 14 liegt zehn Minuten nach der vorgesehenen Frist; Verspätung ausdrücklich dokumentiert. Quellen- und Brain-Belege bleiben erhalten. Kein Merge, Deploy oder Produktivwechsel.

## Status 12: unterbrochene Worker festgestellt

Die offenen Runtime-/DevFeed-Supervisoren hatten keinen Abschluss geliefert. Lesende Prüfung ausschließlich der eigenen Workflow-Transkripte zeigt: Runtime letztes Toolergebnis 16:29 UTC, danach Harness-Unterbrechung 16:32 UTC; DevFeed letztes Toolergebnis 16:36 UTC, Unterbrechung 16:39 UTC. Die Meldung ist kein regulärer Prüferfolg und keine neue menschliche Freigabe. Keine fremden Sessions gelesen oder angesprochen.

Erst nach diesem Endnachweis wurden nur die zwei eigenen offenen Workflow-Supervisoren gestoppt. Teilstände, externe Wartetasks und vorhandene Beweise bleiben erhalten. Gleiche Workflows werden gestaffelt mit gleichem Modell auf erhaltenem Stand fortgeführt. Briefings verlangen vor neuen Compilerläufen die Zuordnung noch laufender eigener Checks und kein Zeitlimit fürs reine Hostlock-Warten. Kein Nachrichtenresume, keine neue Port-Baseline, kein Modellwechsel.

