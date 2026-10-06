# Lokaler Prüfharness und Offline-Datenlauf A

Stand: 03.10.2026, 10:47:28 UTC. **Der beauftragte lokale Compiler-/Test-/Offline-Daten-/Originalhash-/vollständige Wiederholungsauftrag ist tatsächlich abgeschlossen.** Alle fünf Archive und zwölf historischen Captures ergeben sechs getrennte Ausgaben mit **38.273 Dokumentversionen und 1.109.153 Fakten**. Tatsächlicher Datenexit0 am 10:35:46 UTC, Originalhashprüfung0 für alle 47 Inputdateien, unveränderte vier Modulhashs/Harness/Binary, geschlossene eigene FDs und keine eigenen Kinder. Endgültige unabhängige Datenabnahme und C3-Integration sind ausdrücklich noch nicht behauptet.

## Aktuelles Ergebnis

| Quelle beziehungsweise Capturegruppe | Seiten | Dokumentversionen | Fakten | Vollständige Wiederholung |
| --- | ---: | ---: | ---: | --- |
| `deadlock-wiki`, Archiv 15.04.2025 | 4.417 | 22.742 | 1.109.153 | byteidentisch |
| `deadlocked-wiki`, Archiv 07.11.2024 | 2.076 | 13.301 | 0 | byteidentisch |
| `deadlockwiki-org`, Archiv 30.01.2026 | 561 | 2.172 | 0 | byteidentisch |
| `deadlock-miraheze-org`, Archiv 16.06.2024 | 27 | 45 | 0 | byteidentisch |
| `deadlockwiki-miraheze-org`, Archiv 03.12.2023 | 1 | 1 | 0 | byteidentisch |
| `deadlock-wiki`, zwölf Captures vom 02.05.2026 | 12 | 12 | 0 | byteidentisch |

Alle **38.261 historischen Originalrevisions-Inhaltshashs** exakt passend, 0 fehlende Versionen, 13 identische doppelte Hauptarchiv-Revisionsblöcke ohne zusätzliche Dokumente. Zusätzlich alle zwölf Original-API-Revisionsinhalte exakt gleich zur JSONL; deren Inhalts-SHA256 wurde im tatsächlichen Rust-Harness geprüft. Vollständige ergänzende Prüfung sämtlicher Dokument-/Faktpflichtfelder und Typen: Exit0, **0 doppelte ID-/Revisionsschlüssel über die sechs Ausgaben**, überall `redistribution_allowed=false`.

TESTNACHWEIS[TW-1]: 43 passed, 0 ignored | Baseline: nicht gemessen rot

41 tatsächliche unveränderte Modulregressionen plus zwei vorhandene Util-Tests, 0 failed und 0 filtered. Clippy aller Targets und Debugbau jeweils Exit0. Selektive Formatprüfung eigener main.rs plus vier Originalmodule Exit0. Gesamt-`cargo fmt --check` bleibt **Exit1 ausschließlich in unverändertem util.rs:52** außerhalb des Eigentums; Clippy0 mit Warnungen ist kein warnungsfreier `-D warnings`-Lauf.

Datenwurzel: `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/`. Alle sechs Ausgaben haben JSONL, Inventar und eigenen `run-evidence.json`. Gesamtbeleg `harness-run-summary.json`, SHA256 `d76bd70c40844d6ecf5b9e63b19f643a8610c03978d160f93ba53cb9421a75ff`.

Alle JSONL zusammen **851.125.808 Bytes**. Größte tatsächlich vollständig gemessene UTF-8-Zeile: **4.763.003 Bytes ohne abschließendes LF**, 4.763.004 einschließlich LF. Ein Leser mit kleinerem Limit darf die Zeile nicht kürzen. Kein Verbraucherlimit wurde durch diesen Worker geändert.

Kein aktueller vollständiger Wikibestand behauptet: `inventory_complete=false`, `content_complete=false` bei allen Quellen. Die Einseitenquelle bleibt ausdrücklich ohne nachgewiesenen Spielwissenskorpus. Lizenznachweise erlauben keine Veröffentlichung. Keine neuen Live-Challenge-Belege zugemischt, kein Netzwerk/DB/LLM/Git/Deploy, keine Modul-/Core-/Util-/lib.rs-/Produktivmanifeständerung durch diesen Worker.

## Frühere chronologische Zwischenstände

Die nachfolgenden datierten Protokolle erhalten frühere Nichtläufe, genaue notwendige Fixpausen und beide echten Compilerfehler. Ihre damaligen Nullzahlen oder noch fehlenden Dateien sind keine Aussage über den oben belegten Endstand.

## Eigene Dateien

Harness: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/`

Dort wurden ausschließlich `Cargo.toml` und `src/main.rs` neu angelegt. Das Manifest hat einen eigenen leeren Workspace. Der Harness bindet die tatsächlichen Dateien `error.rs`, `util.rs` und `wiki_inventory.rs` per `#[path]` ein. Er verwendet den vorhandenen Core-Crate und die bestehenden Workspace-Abhängigkeiten mit deren Versionen. Produktive Manifeste, Lockfile, Modulregistrierung und eingefrorene Wiki-Module wurden durch diesen Worker nicht geändert. Ein eigener Cargo-Lockfile wurde noch nicht erzeugt.

`zstd` ist als CLI nicht verfügbar. Die vorhandene Hostbibliothek `/lib/x86_64-linux-gnu/libzstd.so.1` wurde per `ldconfig` festgestellt. Der eigene Rust-Harness verwendet diese Bibliothek über FFI; eine zusätzliche Cargo-Abhängigkeit wurde nicht eingeführt. Die Entpackgrenze beträgt 512 MiB. Ein ungültiger Frame oder zu großer Inhalt führt ausdrücklich zu einem Fehler. Der Entpackweg ist noch nicht kompiliert oder ausgeführt.

`chrono = "0.4"` stammt bereits aus dem Workspace. Die echten Cache-Sidecars enthalten numerische Unix-Abrufzeiten; der Harness wandelt diese in UTC nach RFC 3339 um. Revisionszeit, ursprünglicher Abruf, heutige Offline-Beobachtung und Lizenzbeobachtung bleiben getrennt. Für die Archive wird keine unbelegte sekundengenaue Abrufzeit erfunden.

## Compiler- und Locknachweis

Vor dem Versuch wurde `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md` gelesen. Der Wrapper fordert zuerst FD 8 auf `host-checks.lock`, danach FD 9 auf `/tmp/deadlock-cargo-release.lock` blockierend an. Erst anschließend sind frische NonZombie-Probe, Ressourcenprobe und der folgende Compilerbefehl vorgesehen:

```text
/home/nathanael/.cargo/bin/cargo test --offline --manifest-path /home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/Cargo.toml -j 1 -- --include-ignored
```

Keine gesetzten ENV-Variablen. Ausgabeziel wäre `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/test-1.log` gewesen. Der eigene Wrapper `b2qypeu85` wartete länger als zehn Minuten auf die Sperren und erreichte den Cargo-Aufruf nicht. Sein Ausgabefile blieb leer; `test-1.log` existiert nicht. Es gibt daher keinen Cargo-Exitcode, keinen Compilerfehler und keine ausgeführte Testzahl zu melden.

Auf ausdrücklichen Auftrag des Teil-Orchestrators wurde ausschließlich dieser eigene Wartetask per `TaskStop` beendet. Die anschließende Prüfung der NUL-getrennten Bash-Argumente im Prozessspeicher fand **0 eigene Wrapper** mit der eindeutigen `pruefharness-a/test-1.log`-Signatur. Keine Argumentlisten oder Prozessumgebungen wurden ausgegeben. Die zunächst verdächtigten PIDs 2515450 und 2515457 hatten diese Signatur nicht, waren fremd und blieben unberührt. Der eigene Task hat keine verbliebenen Lock-Deskriptoren oder Compilerkinder. Das ist der dem Teil-Orchestrator gemeldete sichere Änderungspunkt für seinen frischen Fixer.

Ressourcenprobe vor dem Warteversuch um 04:02 UTC: 49.152 MiB RAM gesamt, 34.724 MiB verwendet, 14.427 MiB verfügbar, kein Swap. Eigener Datenträger: 608 GiB frei. Der Compiler startete nicht; eine Ressourcenprobe nach einem Compilerende steht entsprechend aus.

TESTNACHWEIS[TW-1]: 0 passed, 0 ignored | Baseline: nicht gemessen rot

Tests nicht gestartet; failed und filtered sind nicht als Laufwerte gemessen. Eine vorbestehende rote Baseline wird nicht behauptet.

## Unkompilierte Quellhashs

Basis-HEAD: `2734c2da4e814ff79953e8e825275b0216a6af16`, Branch `feat/brain-wiki-spielwissen-a`.

Die folgenden SHA-256-Werte wurden um 04:20:31 UTC gemessen. Sie belegen den gelesenenen Stand, keinen ausgeführten Compilerlauf. Vor einem späteren Lauf müssen sie frisch erfasst werden.

| Datei | SHA-256 |
| --- | --- |
| `rust/crates/dbrain-sources/src/wiki_inventory.rs` | `b93b2c1a76c9b3e35d12cfb02c2c6ae9ff340f9b866b053dd6471015695cadcf` |
| `rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs` | `515765390baedfed94ddad57868853ef2493d918f51c239f02cade9cbd76d631` |
| `rust/crates/dbrain-sources/src/wiki_inventory/storage.rs` | `8c5f233224165cc80a45b3e07fdda127890799a949e46f38d04c48d0ddd647f1` |
| `rust/crates/dbrain-sources/src/wiki_inventory/tests.rs` | `2aecb0e4e8374eeef508eb834deb8316636692e26bbc72e4cf03c302e7d32f12` |
| `pruefharness-a/Cargo.toml` | `9ad50a0515c37ecd19a87406dafb2bf3c6c6164b3c905d60cffb0649f7e05620` |
| `pruefharness-a/src/main.rs` | `81159108f2ecd1ba4dcb291a710413b1c9c7bb85e1d348a2776514fbb67a74a2` |

## Vorbereiteter Datenlauf und Grenzen

Der Harness liest vollständige Archive, prüft die bekannten Hauptarchivhashs und die entpackten 177.037.503 Bytes, ermittelt getrennt Seiten-/Revisionsblöcke, unterschiedliche IDs, historische Namespace-Nenner und fehlenden oder unterdrückten Text. Die Optionen erlauben 2 GiB vollständige XML-/Spoolausgabe und 64 MiB pro API-Capture. Im Hauptarchiv wird keine Reduktion auf die letzte Revision vorgenommen. Main-Page-Duplikate sollen durch die echte Modulspool idempotent behandelt werden. Der Harness prüft nach einem zweiten identischen Import Dokumentversionsschlüssel, Inhalts-Hashes, Faktenzahl und bytegleichen JSONL-Hash. Die vorhandenen Modultests umfassen außerdem einen Konflikt ohne Originalüberschreiben.

Der ursprüngliche Modulstand bindet `source_id`, Domains und Spoolvertrag fest an deadlock.wiki. Vier fremde Archivdomains werden ausdrücklich zurückgewiesen. Der Harness schreibt sie weder auf deadlock.wiki um noch vermischt er ihre numerischen IDs. Eine getrennte Quellenkonfiguration wird durch einen anderen, frisch gestarteten Fixer bearbeitet. Der Harness muss danach auf dessen tatsächliche Schnittstelle angepasst werden. Die historischen Lizenzen der anderen Quellen müssen aus ihren eigenen Originalbelegen stammen; der Hauptquellenkontext darf ihnen nicht pauschal zugeschrieben werden.

Die zwei bekannten Revisionsinventarfehler und die spät greifende HTTP-Bytegrenze sind bereits im unabhängigen `REVIEW-LOCAL-1.md` erfasst. Dieser Worker hat sie nicht geändert und keine grüne Prüfung behauptet.

Vorgesehene Ausgabeorte, **noch nicht erzeugt**:

- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/deadlock.wiki-20250415-history/documents.jsonl`
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/deadlock.wiki-20250415-history/inventory.json`
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/legacy-2026-captures/documents.jsonl`
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/legacy-2026-captures/inventory.json`

Bisher durch diesen Worker erzeugt: **0 Dokumente und 0 Fakten je Quelle**, kein Inventar und keine Wiederholungsprobe. Die Archivzahlen 4.417/22.742, 2.076/13.301, 561/2.172, 27/45 und 1/1 stammen bislang ausschließlich aus `DUMPQUELLEN.md`; sie werden hier nicht als eigene normalisierte Laufzahlen ausgegeben. Die Quelle mit einer einzelnen Hauptseite bleibt als fehlender nachgewiesener Spielkorpus klassifiziert. XML allein wird nicht zu `inventory_complete=true` erklärt.

Originalarchive, Raw-Seiten und HTTP-Sidecars blieben unverändert. Kein Wiki-Netzwerkzugriff, keine DB-Verbindung, keine LLM-Aufrufe, Secrets oder ENV-Dateien. Kein Commit, Push, Merge, Deployment oder Dienstneustart.

## Wiederaufnahme und erneute Pause am 03.10.2026 um 05:08 UTC

Der Teil-Orchestrator gab nach `FIX-1.md` denselben Datenauftrag erneut frei. `FIX-1.md` wurde vollständig gelesen; die vier dort genannten Freeze-Hashs wurden am 03.10.2026 um 04:51:18 UTC frisch gemessen und stimmten überein:

- `wiki_inventory.rs`: `eb957daef1cb2d0f9c97cec259703e1f20621d4944622622c93441674f536312`
- `normalize.rs`: `54c45e1f5a6baf63f671bed584380f115ce9ba493e3ba47a2149a4552bc06b09`
- `storage.rs`: `46b43077716094e8dbf85481c99ffb377e601a5af8e7927427d94d8fe95ce4a2`
- `tests.rs`: `3cbe2bf83cf26782bb9a0d9e110c6a1ac43f60632915f2679fe81ad68b2b6a7b`

Der eigene Harness ist inzwischen angepasst. Die vier fremden Archive erhalten jeweils ihren Kontext aus ihrem vollständigen Original-XML über `WikiSourceContext::from_mediawiki_export`. Quellenkennung, Domain und Artikelpfad werden unverändert erhalten. Lizenzname und gegebenenfalls Lizenzadresse werden ausschließlich aus dem jeweiligen eigenen Archive.org-Metadatensatz gelesen; bei fehlendem Lizenznachweis bleibt der XML-Kontext ungeprüft. `redistribution_allowed=false` bleibt verbindlich. Der Hauptdump verwendet sein historisches siteinfo. Zusätzlich vergleicht der Harness sämtliche verfügbaren Original-XML-Revisionshashs mit den erzeugten JSONL-Versionen und prüft getrennte Quellenidentitäten, vollständige Versions-/Seitenzahlen und echte historische Namespace-Nenner.

Eigene Harness-Hashs am 03.10.2026 um 04:56:17 UTC:

- `Cargo.toml`: `9ad50a0515c37ecd19a87406dafb2bf3c6c6164b3c905d60cffb0649f7e05620`
- `src/main.rs`: `538acf9278d62a548498ccae617602c7ff0a4bf6e3c2dae6e3e04a052c44c596`

Der zweite eigene Compilerwrapper `bcdj1b90g` forderte wieder zuerst FD 8 auf dem Hostlock und danach FD 9 auf dem Cargo-Lock blockierend an. HOSTPROBE.md und der Test-Wächter wurden vor dem Versuch erneut gelesen. Befehl und Einzeljobgrenze blieben wie oben; als neues Ausgabeziel war `pruefharness-a/test-2.log` vorgesehen. Bis zur tatsächlichen Probe um 05:07:48 UTC wurde dieses Log nicht angelegt. Auch dieser Versuch erreichte keinen Cargo-Aufruf und hat keinen Cargo-Exitcode oder Testbefund.

Nach der ausdrücklichen Pause des Teil-Orchestrators für Fixrunde 2 wurde ausschließlich `bcdj1b90g` per `TaskStop` beendet. Die anschließende Signaturprüfung, mit Ausschluss des laufenden eigenen Probeprozesses, fand **0 verbliebene eigene Wrapper**. `test-2.log` blieb abwesend. Keine eigenen Lock-FDs, Compiler oder Compilerkinder verbleiben; fremde Prozesse wurden nicht verändert. Dieser sichere Zustand wurde dem Teil-Orchestrator gemeldet.

Die vom Teil-Orchestrator neu gemeldeten Befunde betreffen Quellenbindung vor Dokumentschreiben und zusätzliche belegte Autorenherkunft bei identischem Inhalt. Ein neuer Fixer bearbeitet die Module getrennt. Dieser Datenworker verändert sie nicht und startet bis zur nächsten ausdrücklichen Freeze-/Prüffreigabe keine Compiler- oder Datenläufe.

Aktueller tatsächlicher Nachweis bleibt: 0 ausgeführte Tests, 0 durch diesen Worker erzeugte Dokumente und Fakten je Quelle, keine JSONL-/Inventarausgaben und keine ausgeführte Wiederholungsprobe. Die vorherige Beschreibung des festen alten Quellenkontexts ist durch den angepassten Harness überholt; ein Laufzeitnachweis dafür liegt noch nicht vor.

## Dritter Warteversuch und Pause für Fixrunde 3, 05:50 UTC

`FIX-2.md` wurde vollständig gelesen. Die vier Freeze-Werte wurden am 03.10.2026 um 05:36:44 UTC frisch bestätigt:

- `wiki_inventory.rs`: `2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80`
- `normalize.rs`: `27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc`
- `storage.rs`: `88ff521c77032215ba2aef8b4dc4abee20fd43bedbcf5ab218620acbb43e7498`
- `tests.rs`: `82356a793f8981f021ef6ab9fcf420bfc529fa445243dcff04df2ed2749e7d91`

Die öffentlichen Aufrufe blieben zum eigenen Harness kompatibel. Er wurde nicht verändert. HOSTPROBE.md und der Test-Wächter wurden vor dem dritten Versuch erneut gelesen. Der eigene Wrapper `ba5ocz35n` wartete mit demselben Cargo-Testbefehl und einem Job blockierend auf die beiden vorgeschriebenen Sperren. Das geplante Ausgabeziel `pruefharness-a/test-3.log` wurde bis zur Probe um 05:49:44 UTC nicht angelegt. Kein Cargo-Aufruf, Compilerstart, Test- oder Datenlauf fand statt.

Auf die ausdrücklich angeordnete neue Pause hin wurde ausschließlich `ba5ocz35n` per `TaskStop` beendet. Um 05:50:10 UTC wurden 0 eigene verbliebene Wrapper anhand der eindeutigen Logsignatur festgestellt; der laufende Probeprozess wurde ausgeschlossen. `test-3.log` war abwesend. Die zusätzliche Prozessprobe fand keine verwaisten `flock`-Prozesse mit PPID 1. Keine eigenen verbliebenen Lock-FDs oder Compilerkinder, keine Veränderung fremder Prozesse oder Locks. Die sichere Grenze wurde dem Teil-Orchestrator bestätigt.

Die neu übermittelten unabhängigen Befunde betreffen Konfliktdateien außerhalb der Gesamtgrößenzählung und die Elternverzeichnis-Synchronisierung beim erstmaligen Anlegen von `provenance/` und `conflicts/`. Der nächste Fixer bearbeitet die Module getrennt. Bis zur neuen bestätigten Freeze-Freigabe startet dieser Datenworker keine weiteren Compiler- oder Datenläufe.

Die vorhandene `get_bounded`-Readeranbindung auf Cs frischer Mainbasis ist eine separate C2-Prüfung. Sie blockiert den älteren lokalen Offline-Harness nicht und wird hier weder neu implementiert noch durch einen zweiten Netzpfad ersetzt.

Alle drei bisher autorisierten Warteversuche wurden vor dem tatsächlichen Cargo-Aufruf beendet. Es gibt weiterhin keinen Compiler-/Testexit, keine ausgeführten Tests, keine erzeugten JSONL-/Inventardateien und je Quelle 0 erzeugte Dokumente/Fakten. Originaldaten und eigener Harness bleiben erhalten; der Datenauftrag bleibt offen.

## Erster echter Compilerlauf und eigener Pfadfix, Stand 06:58:43 UTC

`FIX-3.md` wurde vollständig gelesen. Die vier endgültigen Modulhashs wurden um 06:13:01 UTC, unmittelbar vor dem tatsächlichen Compilerstart unter beiden gehaltenen Locks und nach dem vollständigen Compilerende gemessen. Alle drei Messungen stimmen überein:

- `wiki_inventory.rs`: `2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80`
- `normalize.rs`: `27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc`
- `storage.rs`: `a16a36d1880a5eb3c07856c39e4c7f6b18b9a168aa4cac0de93961ae34f6827b`
- `tests.rs`: `5e71ade9d106a9adffc5935a9d141741b58805a3d4cf15eb01d324901eb9bef4`

Der vierte eigene Wrapper `b1c2kt30c`, PID 3200162, startete Cargo tatsächlich am **03.10.2026 um 06:47:12 UTC**. Beide Locks waren erworben; unmittelbar vor Start bestand die frische NonZombie-Probe. Zuvor waren der eigene FD8 und der Lockpfad anhand der identischen Inode 16006309 geprüft worden; der ausdrücklich zulässige nichtblockierende Gegenversuch auf genau diesen Pfad hatte Exit 75 geliefert. Kein Fremdprozess wurde verändert.

Tatsächlicher Befehl:

```text
/home/nathanael/.cargo/bin/cargo test --offline --manifest-path /home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/Cargo.toml -j 1 -- --include-ignored
```

Log: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/test-4.log`. **Cargo-Exit 101**, drei E0583, keine Tests ausgeführt. Die Fundstellen `wiki_inventory.rs:15`, `:16` und `:19` betreffen ausschließlich die eigene Harness-`#[path]`-Anbindung: Rust suchte `normalize.rs`, `storage.rs` und `tests.rs` neben der eingebundenen Datei statt unter deren vorgesehenem Unterverzeichnis. Dies ist ein tatsächlich gemessener Harnessfehler, kein bestätigter Compilerfehler der eingefrorenen Module. Keine Testerwartung wurde geändert.

Vor Start: 15.319 MiB RAM verfügbar, 582 GiB Platte frei. Nach Ende: 15.295 MiB RAM verfügbar, 581 GiB Platte frei, jeweils kein Swap. Der Wrapper meldete `LOCKS_CLOSED` und endete vollständig mit Exit 101. Der Compilerstart-Wächter war ebenfalls vollständig beendet.

Korrigiert wurde ausschließlich der eigene Harness: normaler `mod wiki_inventory`-Aufruf und vier neu angelegte Symlinks unter `pruefharness-a/src/` spiegeln die tatsächlichen unveränderten Modulpfade. Vor Anlage waren alle eigenen Ziele auf Nichtvorhandensein geprüft worden. Ihre dereferenzierten SHA-256-Werte stimmen exakt mit dem Freeze überein. `error.rs`, `util.rs` und der Original-Core werden weiterhin unverändert verwendet. Das ursprüngliche Hauptmodul und alle drei Kindmodule wurden nicht bearbeitet oder kopiert. Die unnötige eigene `Command`-Importwarnung wurde entfernt. Ein eigener Cargo-Lockfile ist jetzt vorhanden; das produktive Lockfile blieb unverändert.

Eigene Hashs um 06:58:43 UTC:

- `src/main.rs`: `59e2239aae92a860bf3162b47b0b813473b0f06eb3bf98491e90ba51e9d50743`
- `Cargo.toml`: `9ad50a0515c37ecd19a87406dafb2bf3c6c6164b3c905d60cffb0649f7e05620`
- `Cargo.lock`: `ab2a866b93478823752eaafed59164faf06bcd214335257c597ecc4f8e7d2c6a`

Der neue eigene stabile Wrapper `bjnkctnph`, PID 3444003, wartet auf die vorgeschriebenen Locks für denselben Befehl mit Ausgabeziel `test-5.log`. Es wurde kein erneuter Compilerstart behauptet. AN_BEREICHE.md einschließlich Punkt 28 wurde vollständig gelesen: Dieser stabile Wrapper wird nicht allein wegen Wartezeit oder Wachtimer beendet. Keine neuen Agenten, Modulfixes, Corepfade, Commits oder Releasebauten.

Aktuell weiterhin 0 ausgeführte Tests und je Quelle 0 erzeugte Dokumente/Fakten. Die tatsächlichen JSONL-/Inventar-, vollständigen Revisionshash-, Wiederholungs- und Konfliktnachweise bleiben offen.

## Notwendige Quellfix-Pause nach test-5-Warteversuch, 07:08:16 UTC

Der Teil-Orchestrator meldete den unabhängig bestätigten verbliebenen P2: Nach erfolgreichem Rename einer Provenienzdatei und fehlgeschlagenem abschließenden Elternsync wird beim Retry dieselbe vorhandene Aussage beziehungsweise Konfliktdatei ohne nachgeholten Verzeichnissync bestätigt. Dies ist sein statischer Kontrollflussbefund, keine durch diesen Datenworker ausgeführte Crash-Reproduktion.

Die darauf ausdrücklich verlangte Pause betrifft einen konkreten notwendigen Fix und keinen Wachtimer. Ausschließlich der eigene Wrapper `bjnkctnph`, PID3444003, wurde per `TaskStop` beendet. Unmittelbar davor war er noch vor Cargo auf dem ersten Hostlock wartend: FD8 am korrekten Lockpfad, FD9 nicht geöffnet, `test-5.log` nicht angelegt. Die frische Prüfung um 07:08:16 UTC belegt verschwundene eigene PID sowie geschlossene eigene FD8/FD9, weiterhin abwesendes `test-5.log` und keine verwaisten `flock`-Prozesse mit PPID1. Kein eigener Compilerstart oder Compilerkind. Der eigene Startwächter `bdhpyp91b` endete regulär mit Exit0 und `OWN_WRAPPER_ENDED_WITHOUT_CARGO`.

Der sichere Punkt wurde dem Teil-Orchestrator gemeldet. Fremde Prozesse und Locks wurden nicht verändert. Der korrigierte eigene Harness, dessen Lockfile, der tatsächliche frühere `test-4.log` mit Cargo-Exit101 und sämtliche Originale bleiben erhalten. Kein Sourcefix durch diesen Datenworker. Keine weiteren Compiler-/Datenläufe bis zum nächsten schriftlich bestätigten Freeze; derselbe Datenauftrag bleibt offen.

## Freeze4, stabiler Warteversuch und notwendige Familienfix-Pause, 07:49:43 UTC

`FIX-4.md` wurde vollständig gelesen. Vor dem sechsten Versuch wurden am 03.10.2026 um 07:25:37 UTC alle vier Endhashs unabhängig bestätigt:

- `wiki_inventory.rs`: `2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80`
- `normalize.rs`: `27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc`
- `storage.rs`: `f48f4819534870b1aa8b2c1f0042f82d1d1ca1b1056242e55dc6632271d4daea`
- `tests.rs`: `a0d7cb81ce90c7a729c4ac6e92fbe02d3b670ff5e473a307935589adaf26c6d1`

HOSTPROBE.md und der Test-Wächter wurden erneut vor dem geplanten vollständigen Lauf gelesen. Derselbe korrigierte Harness und Original-Core blieben unverändert. Der eigene Wrapper `b66lshqbc`, PID3660876, wartete stabil auf die beiden vorgeschriebenen Locks für denselben Cargo-Testbefehl mit `-j1` und `--include-ignored`. Kein Timerabbruch. Die tatsächliche Probe um 07:44:24 UTC belegte: erster Lock noch nicht erworben, eigener FD8 am korrekten Lockpfad, FD9 nicht geöffnet, `test-6.log` abwesend.

Der Teil-Orchestrator verlangte anschließend eine konkrete notwendige Familienfix-Pause. Sein nachgelesener Befund betrifft zusätzliche Autoren-/Contributor-/Capture-/Lizenzherkunft beim öffentlichen Retry desselben Konflikttexts sowie veraltetes `stored_bytes` beim Same-Spool-Retry nach sichtbarer Installation und abschließendem Syncfehler. Diese Befunde sind hier als übermittelte statische Befunde gekennzeichnet, nicht als vom Datenworker ausgeführte Reproduktion.

Um 07:49:11 UTC wurde ausschließlich der eigene Wrapperzustand frisch geprüft: weiter vor Cargo auf dem ersten Hostlock, FD9 und `test-6.log` fehlen. Ausschließlich `b66lshqbc` wurde danach per `TaskStop` beendet. Die Prüfung um 07:49:43 UTC belegt verschwundene eigene PID, geschlossene eigene FD8/FD9, weiterhin abwesendes `test-6.log` und keine verwaisten `flock`-Prozesse mit PPID1. Der eigene Startwächter `b8reh2cuz` endete regulär mit Exit0 und `OWN_WRAPPER_ENDED_WITHOUT_CARGO`. Kein test-6-Cargo-Aufruf, Compiler- oder Testexit; keine eigenen Compilerkinder. Diese sichere Grenze wurde dem Teil-Orchestrator bestätigt.

Der erste tatsächliche `test-4`-Cargo-Exit101 bleibt unverändert ein eigener Harness-Pfadfehler und keine rote Test-Baseline. Weiterhin 0 ausgeführte Tests, 0 erzeugte Vertragsdokumente/Fakten je Quelle, keine JSONL-/Inventarausgaben und keine ausgeführte Wiederholungsprobe. Originale, korrigierter Harness, eigene Lockfile und alle bisherigen Nachweise bleiben erhalten. Kein Sourcefix und keine fremden Eingriffe; derselbe Auftrag wartet auf den nächsten bestätigten Freeze.

## Freeze5 und unveränderte Fortsetzung, 08:33:14 UTC

`FIX-5.md` vollständig gelesen. Die vier Endhashs wurden erneut unabhängig gemessen und stimmen exakt überein:

- `wiki_inventory.rs`: `2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80`
- `normalize.rs`: `27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc`
- `storage.rs`: `edd384e621d9a0df11d1a36ad6cac44678285e342f68d4c363db42f1646bd7ea`
- `tests.rs`: `e26f97b9f3a9d666889cefff3445d17a8e07511c7fd527591c12e172356e3f18`

Harness, Core und eigener Lockfile sind erhalten; die drei eigenen Hashs entsprechen weiterhin der Messung um 06:58:43 UTC. HOSTPROBE.md und Test-Wächter vor dem nächsten Versuch gelesen. Der eigene Testwrapper `bq7qmhz1w`, PID4010218, wartet stabil blockierend auf dieselben vorgeschriebenen Locks. Probe um 08:33:14 UTC: FD8 am Hostlock, FD9 noch nicht geöffnet, `test-7.log` nicht vorhanden, kein Cargo-Start. Kein Timerabbruch und keine zweite Implementierung.

Vorgesehener wörtlicher Befehl ohne ENV-Konfiguration:

```text
/home/nathanael/.cargo/bin/cargo test --offline --locked --manifest-path /home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/Cargo.toml -j 1 -- --include-ignored
```

Der Wrapper hält beide Deskriptoren bis zum vollständigen Ende, prüft unmittelbar vor Cargo alle aktiven NonZombie-Compiler und ausschließlich die belegte enge Metadata-Ausnahme, erfasst Sourcehashs und Ressourcen vor/nach und bewahrt den echten Exit. Ein eigener Startwächter `bufttpckj` beobachtet ausschließlich PID4010218 und die tatsächliche Loganlage.

Die übermittelte unabhängige statische Nachprüfung6 wurde unter `REVIEW-LOCAL-6.md` vollständig gelesen. Sie bestätigt keine Restdefekte und keine begründete Fixrunde6, aber ausdrücklich keinen Compiler-/Test-/Crash-/Datenbeweis. Die 41 Modulregressionen sind weiterhin nur geschrieben. Keine eigenen Modulfixes.

Die erste ausschließlich lesende Formatprüfung des eigenen `src/main.rs` mit `rustfmt --check --config skip_children=true` endete tatsächlich mit Exit1 (`fmt-initial.log`). Die kompakte eigene Datei muss nach dem vollständigen Testwrapperende formatiert werden. Während des wartenden oder laufenden Wrappers wird sie nicht geändert; die eingefrorenen Symlink-Zielmodule werden nie formatiert oder bearbeitet.

## Zweiter echter Compilerexit und enge Harnesskorrektur, 08:52:44 UTC

Test7 startete tatsächlich am 03.10.2026 um **08:46:45 UTC**, `bq7qmhz1w`, PID4010218, beide Locks gehalten, frische NonZombie-Probe frei. `test-7.log` belegt Cargo-Exit101 vor allen Tests: Rekursionsgrenze bei `$crate::json_internal!`, tatsächliche Fundstelle `src/wiki_inventory/normalize.rs:599`. Der Compiler empfiehlt ausdrücklich `#![recursion_limit = "256"]` im Harness-Crateroot. Kein Testergebnis und keine vorbestehende rote Test-Baseline. Sourcehashs vor/nach exakt Freeze5, `LOCKS_CLOSED`; eigene PID und FD8/FD9 bei Nachprüfung verschwunden. Startwächter `bufttpckj` regulär Exit0.

Vor Test7: 14.372 MiB RAM verfügbar und 562 GiB Platte frei. Nach vollständigem Ende: 14.385 MiB RAM verfügbar, weiterhin 562 GiB frei, kein Swap. Der tatsächliche frühere test4-Exit101 und dessen anderer eigener Pfadfehler bleiben erhalten.

Ausschließlich im eigenen `src/main.rs` wurde die genaue Compilerempfehlung als Crate-Attribut ergänzt. Danach nur diese Datei mit `rustfmt --edition 2021 --config skip_children=true` formatiert. Keine Modul-/Core-/lib.rs-/Manifeständerung und keine Erwartungsänderung. Der neue eigene Harnesshash um 08:52:44 UTC lautet `d358ec15a692aeb57cbbd7f05e62be4eaa85217c3b5eaf59db4a63fc23f201fd`; alle vier Modulhashs erneut unverändert bestätigt. C2 muss die tatsächliche Crateroot-Anforderung bei Registrierung beachten; über C2s frischen Crateroot wird hier nichts behauptet.

Zusätzliche rein lesende Formatprüfung der vier Originalmodule mit `rustfmt --check --config skip_children=true`: Exit0, `fmt-freeze5.log`. Originalmaterial vor Datenverarbeitung gesichert: 47 SHA256-Zeilen für fünf Archive, zwei siteinfo-Belege, vier Archivmetadaten, zwölf Raw-Captures und 24 originale Cache-/Sidecar-Dateien. `originals-before.sha256` hat SHA256 `34f9a852da2c48d14ee8ecb405559c742d6fd8c260ddb856db0f14e70513e1b9`; nach dem tatsächlichen Gesamtlauf folgt deren Unverändertheitsprüfung.

Der eigene nächste Wrapper `bobg0akq7`, PID4138464, ist derselbe fortgesetzte Prüfauftrag. Um 08:52:44 UTC lebt er stabil und wartet vor Öffnung des zweiten Lock-FDs. Geplant ist zunächst der unveränderte Testbefehl mit `--offline --locked`, `-j1`, `--include-ignored`, Ausgabe `test-8.log`. Nur nach dessen tatsächlichem Exit0 folgen im selben gehaltenen Lockwrapper Clippy aller Targets, Debugbau und ausschließlich `cargo fmt --check`. Vor jedem Cargo-Aufruf wird die vollständige enge NonZombie-Probe erneut ausgeführt; jeder echte Exit wird separat ausgegeben und bei Fehler die Folgeprüfung nicht gestartet. Kein Release oder Datenlauf wird aus einem Wrapperstart abgeleitet.

## Tatsächliche erfolgreiche Prüfungen und abgegrenzte Formatabweichung, 09:09:16 UTC

`bobg0akq7`, PID4138464, erhielt beide Sperren. Vor jedem folgenden Cargo-Aufruf bestand die frische NonZombie-Probe. Vier Modulhashs und eigener Harnesshash vor/nach identisch zu Freeze5 beziehungsweise `d358ec15a692aeb57cbbd7f05e62be4eaa85217c3b5eaf59db4a63fc23f201fd`.

| Wirklich gestarteter Schritt | UTC-Start | Exit | Vollständiges eigenes Log |
| --- | --- | --- | --- |
| Tests ungefiltert mit `--include-ignored` | 09:02:26 | 0 | `test-8.log` |
| Clippy aller Targets | 09:02:43 | 0 | `clippy-1.log` |
| Tatsächlicher Debugbau | 09:08:16 | 0 | `build-1.log` |
| `cargo fmt --check` | 09:09:16 | 1 | `fmt-1.log` |

Wörtliche Befehle ohne gesetzte ENV-Konfiguration:

```text
/home/nathanael/.cargo/bin/cargo test --offline --locked --manifest-path /home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/Cargo.toml -j 1 -- --include-ignored
/home/nathanael/.cargo/bin/cargo clippy --offline --locked --all-targets --manifest-path /home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/Cargo.toml -j 1
/home/nathanael/.cargo/bin/cargo build --offline --locked --manifest-path /home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/Cargo.toml -j 1
/home/nathanael/.cargo/bin/cargo fmt --manifest-path /home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/Cargo.toml -- --check
```

Tatsächliches Testresultat: **43 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out**, 1,37 Sekunden Testausführung. Alle **41 Modulregressionen** und die zwei Original-Util-Tests liefen wirklich, kein Filter oder Test-DSN-Default. Keine der Regressionen wurde abgeschwächt oder übersprungen.

TESTNACHWEIS[TW-1]: 43 passed, 0 ignored | Baseline: nicht gemessen rot

Die injizierten Syncfehler-Regressionen sind reale lokale Fehlerpfadtests, kein Stromausfallbeweis. Die zuvor gescheiterten Compilerläufe test4 und test7 bleiben Compilerfehler vor Tests und keine rote Altbaseline.

Clippy hat Exit0 mit Warnungen, nicht einen warnungsfreien `-D warnings`-Lauf: fünf Warnungen im Testprofil, 13 im Binärprofil einschließlich vier Duplikaten. Unbenutzte Util-/Netzpfade sind im Offline-Harness nicht aufgerufen; zusätzlich genau eine `collapsible_if`-Anregung in `wiki_inventory.rs:455`. Keine Warnung wurde unterdrückt und kein eingefrorenes Modul verändert.

Gesamtformat-Exit1 betrifft ausschließlich unverändertes `rust/crates/dbrain-sources/src/util.rs:52`, außerhalb des eigenen Eigentums. `fmt-1.log` enthält dessen reinen Formatdiff. Keine Sourcekorrektur an Util/Core/lib.rs/Manifest. Selektive Originalmodulprüfung `fmt-freeze5.log` hat Exit0; die eigene main.rs wurde getrennt eng formatiert. Eine weitere gemeinsame selektive Checkprüfung der eigenen main.rs und vier Originalmodule ist im Datenwrapper vorgesehen. Gesamtformat1 wird dadurch nicht in Gesamtformat0 umgedeutet.

Vollständiges Wrapperende 09:09:16 UTC, `LOCKS_CLOSED`, eigene PID danach verschwunden. Vor Start 14.678 MiB RAM verfügbar und 557 GiB Platte frei, nach Ende 14.631 MiB und 556 GiB, kein Swap. Keine eigenen Compilerkinder laufen weiter.

Nächster eigener stabiler Datenwrapper `bt0vqxqzj`, PID86988, wartet auf beide Locks für den tatsächlich gebauten Debug-Harness. Noch keine Datenstart- oder Datenlaufzahl behauptet. Er verarbeitet sämtliche fünf Originalarchive und zwölf Captures, vergleicht sämtliche Originalrevisionshashs und wiederholt vollständig jede Quelle. Originale werden vor/nach gegen alle 47 gesicherten Hashs geprüft. Derselbe Auftrag, keine zweite Implementierung und kein Netzwerk/DB/LLM.

## Wirklicher vollständiger Offline-Datenstart, 09:46:17 UTC

Derselbe Datenwrapper `bt0vqxqzj`, PID86988, erhielt beide blockierenden Locks und bestand die frische NonZombie-Probe. Gemeinsame ausschließlich lesende Formatprüfung der eigenen main.rs und vier Originalmodule mit `skip_children=true`: Exit0, `fmt-isolated.log`. Gesamtformat1 im unveränderten Util bleibt ausdrücklich getrennt.

Tatsächlich gebautes und ausgeführtes Binary: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/target/debug/wiki-pruefharness-a`, SHA256 `83349db303f5661ba4c6f0ac156b4ec14ab93e36bf85d36b90aa67b163d48578`. Quellhash und vier Modulhashs beim Start exakt wie oben. Laufargument und belegte Offline-Beobachtung: `2026-10-03T09:46:17Z`, unmittelbar im Wrapper über `date -u` erfasst, nicht aus ENV. Vollständige Ausgabe in eigenem `data-1.log`, echter Exit erst nach dem vollständigen Ende.

Vor wirklichem Start: 15.984 MiB RAM verfügbar, 549 GiB Platte frei, kein Swap. Um 09:53:17 UTC lief ausschließlich das eigene Harnesskind PID253357 unter PPID86988 weiter, 1.818.888 KiB RSS; `data-1.log` noch leer und Haupt-JSONL noch nicht veröffentlicht. Dies ist ein laufender Vollversuch, kein Null-Datenabschluss. Kein Abbruch wegen Warte-/Wachtimer.

Die zusätzliche heutige öffentliche Recherche bleibt strikt getrennt unter `a-live-coverage/`. Deren übermittelte 403-/Managed-Challenge-Belege sind keine neuen erfolgreichen Artikel, Inventarnenner oder Importinputs. Weder diese Dateien noch neue Captures werden diesem Offline-Lauf zugemischt. Der eingefrorene ursprüngliche Bestand von fünf Archiven, zwölf Captures und 47 gesicherten Inputdateien bleibt maßgeblich.

## Erste vollständig gemessene Quelle, 10:23:32 UTC

Der unveränderte Gesamtprozess läuft für die übrigen Quellen weiter. Hauptarchiv `deadlock.wiki-20250415-history.xml.zst` jetzt tatsächlich vollständig verarbeitet, unabhängig gegen jedes verfügbare Originalrevisions-Inhaltsbyte gehasht und vollständig ein zweites Mal verarbeitet:

- 4.417 unterschiedliche Seiten, 22.742 unterschiedliche Revisionen und Vertragsdokumente.
- **1.109.153 strukturierte Fakten** aus diesem historischen Bestand, keine Aussage über aktuelle Spielwerte.
- 4.418 Seitenblöcke und 22.755 Revisionsblöcke; 13 identische doppelte Revisionsblöcke erhalten keinen zweiten Datensatz.
- Alle **22.742 Originalrevisions-Inhaltshashs** exakt passend; 0 fehlende Originalversionen, Quellenidentität aller Dokumente geprüft, 0 fehlende oder unterdrückte Textblöcke.
- Vollständige zweite Verarbeitung: Dokument-/Seiten-/Faktenzahlen und gesamter JSONL-Hash identisch.
- JSONL-SHA256: `7e6c7b2e0dc50049bd4a02a93aff5a94ca2ba0351056c1559b5df1b131cbc63c`.
- Komprimierter Archivhash `bea69e14f1ec35b8bbfe157b3cf9cf362f888e2321e9aa5a46a03c1670120825`, XML-Hash `4a64288c2776b46a3722054aa4fe7f61cae167b1f5e3d4125c94080c37ab240e` und 177.037.503 entpackte Bytes tatsächlich bestätigt, kein stilles Kürzen.

Tatsächliche Namespace-ID-Nenner vollständig mit Modulbericht abgeglichen: `0=771, 1=31, 2=220, 3=10, 4=7, 6=2751, 8=45, 10=277, 11=2, 12=5, 14=113, 828=78, 829=1, 3000=72, 3002=34`. Originalschema ordnet **3000=Update**, **3002=Data** zu, keine pauschale Namespace-Deutung einer anderen Wikiquelle.

Vollständig gelesener Laufbeleg: `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/deadlock.wiki-20250415-history/run-evidence.json`. JSONL und Inventar liegen im selben Verzeichnis. Quellenkennung `deadlock-wiki`, Originalherkunft `https://deadlock.wiki`, Artikelbasis `https://deadlock.wiki/`. `inventory_complete=false`, `content_complete=false`, `scope=offline_partial_source_documents`; historische Archivvollübernahme wird ausdrücklich nicht zu heutiger Wiki-Vollständigkeit erklärt.

Die anderen vier Archive und zwölf Captures sind noch kein abgeschlossener Gesamtbeweis. Originaldateien-/Modulhashprüfung nach vollständigem Wrapperende steht noch aus. C3 übernimmt die finale Integration statt C2; dieser Worker meldet weiterhin ausschließlich an seinen Teil-Orchestrator A und berührt keine fremden Sessions.

## Endbeweis des vollständigen erhaltenen Auftrags

Datenwrapper `bt0vqxqzj`, PID86988: **tatsächlicher Datenexit0**, Start 03.10.2026 09:46:17 UTC, vollständiges Ende **10:35:46 UTC**. `ORIGINALS_UNCHANGED_CHECK_EXIT=0`, abschließende Source-/Harness-/Binaryhashs vor/nach exakt gleich, `LOCKS_CLOSED`, endgültiger Werkzeugexit0. Nicht bloß fehlende PIDs begründen den Erfolg.

Zusätzliche eigene Probe um **10:47:28 UTC**: Datenwrapper86988, eigenes Harnesskind253357 sowie Testwrapper4010218/4138464 verschwunden, eigene FD8/FD9 nicht vorhanden, keine Kinder von86988. Kein eigener Wächter oder Compiler hält Sperren. Vor Datenstart 15.984 MiB RAM verfügbar/549 GiB Platte frei, nach vollständigem Datenende 16.822 MiB/548 GiB, kein Swap. Der stabile Vollversuch wurde nie timerbedingt beendet.

Alle sechs vollständigen JSONL-Ergebnisse in `data-1.log` wurden rein lesend vollständig gegen `harness-run-summary.json` abgeglichen: exakte Gleichheit, Exit0. Kein `run_error` oder `normalization_error`. Fünf vollständige Archivdurchläufe und zwölf vollständige historische API-Captures, sämtliche Quellen vollständig ein zweites Mal verarbeitet, Zahlen und gesamter JSONL-Hash jeweils identisch. **38.261 historische Originalrevisionshashmatches**, 0 fehlende Versionen; insgesamt mit den zwölf Captures **38.273 Dokumentversionen und 1.109.153 Fakten**.

### Fertige Daten- und Inventarpfade

Jedes unten genannte Verzeichnis enthält tatsächlich `documents.jsonl`, `inventory.json`, `checkpoint.json` und `run-evidence.json`:

- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/deadlock.wiki-20250415-history/`
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/deadlocked.wiki-20241107-history/`
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/deadlockwiki.org_mw-20260130-history/`
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/deadlock.miraheze.org_w-20240616-history/`
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/deadlockwiki.miraheze.org_w-20231203-history/`
- `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/legacy-2026-captures/`

Gesamtbeleg: `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized/harness-run-summary.json`, SHA256 `d76bd70c40844d6ecf5b9e63b19f643a8610c03978d160f93ba53cb9421a75ff`.

### Tatsächlich nachgemessene JSONL-Größen und Hashs

Maximalzeilen sind UTF-8-Bytes ohne abschließendes LF, keine Zeichen-/Displaybreite. Mit LF jeweils ein Byte mehr. Sämtliche Zeilen vollständig gelesen, kein Sampling.

| Ausgabe | Dokumente | Dateibytes | Maximalzeile | SHA256 der vollständig wiederholten JSONL |
| --- | ---: | ---: | ---: | --- |
| Hauptarchiv | 22.742 | 722.950.493 | 4.763.003 | `7e6c7b2e0dc50049bd4a02a93aff5a94ca2ba0351056c1559b5df1b131cbc63c` |
| deadlocked.wiki | 13.301 | 116.951.094 | 1.195.741 | `41ff8c70ecd405df805e16403a20eb0c741cf73e9b7a6c409a7866663abd8cf2` |
| deadlockwiki.org | 2.172 | 10.881.968 | 32.798 | `a9952f6f43aa021642d8571aea33729dc7482013b707ed9137ce4c63a3eca556` |
| deadlock.miraheze.org | 45 | 115.457 | 4.277 | `6abe02d138ea4dde961f092906d1074d3cb456ab5db5551f2df30600b0c9da76` |
| deadlockwiki.miraheze.org | 1 | 3.861 | 3.860 | `5399fd275a8c9aeb37377d8398c0eb2ff27ba64aae0647c42546d2b0dfdcc90f` |
| Legacy-Captures | 12 | 222.935 | 41.801 | `62f5ac30cf4116a2fdaaba66884d062ef1f29f67d7bf0ec0a9251f40841aa91f` |

**851.125.808 Dateibytes insgesamt**, 38.273 physische JSONL-Zeilen. Maximal 4.763.003 Bytes ohne LF beziehungsweise 4.763.004 mit LF. Verbraucher müssen diese tatsächlich vorkommende Größe berücksichtigen; keine Konfiguration oder Zeile wurde dafür geändert.

### Historische Namespace-Nenner und getrennte Quellen

Alle tatsächlichen Namespace-ID-Seitenmengen der fünf vollständigen XML-Dateien wurden unabhängig ermittelt und vollständig gegen den jeweiligen Modulbericht verglichen:

- `deadlock-wiki`: `0=771, 1=31, 2=220, 3=10, 4=7, 6=2751, 8=45, 10=277, 11=2, 12=5, 14=113, 828=78, 829=1, 3000=72, 3002=34`.
- `deadlocked-wiki`: `0=609, 1=27, 2=190, 3=8, 4=7, 6=809, 8=23, 10=238, 11=2, 12=4, 14=45, 828=66, 829=1, 3000=47`.
- `deadlockwiki-org`: `0=207, 2=16, 4=6, 6=227, 8=16, 10=64, 14=25`.
- `deadlock-miraheze-org`: `0=12, 6=15`.
- `deadlockwiki-miraheze-org`: `0=1`.

Originalherkünfte/Artikelbasen: `https://deadlock.wiki/`, `https://deadlocked.wiki/`, `https://deadlockwiki.org/w/`, `https://deadlock.miraheze.org/wiki/`, `https://deadlockwiki.miraheze.org/wiki/`. Beide Miraheze-Quellen und deadlocked.wiki bleiben eigenständige Quellen, keine belegten Betreiber-Aliasse der Hauptquelle. Jeder Originalrevisionsvergleich prüfte Quellenkennung, stabile Seiten-ID, Revision, Inhaltshash und passende Originalherkunft. Gleiche numerische IDs verschiedener Wikis wurden nicht zusammengeführt.

### Ergänzende rein lesende Endprüfungen

Zusätzliche administrative Abgleiche mit vorhandenem `jq` erzeugten nur Prüfdaten im eigenen Harness. Keine neue Normalisierung, neue Implementierung, Hilfsskriptdatei oder Sourceänderung.

- `legacy-original-byte-proof.json`, Exit0: alle zwölf Original-API-Revisionsinhalte exakt gleich den zwölf erzeugten JSONL-Inhalten und ihren stabilen IDs/Revisionen; 0 fehlende Originalversionen. Inhalts-SHA256 jeder JSONL war zusätzlich im tatsächlichen Rust-Harness berechnet/geprüft. SHA256 des Belegs: `5073bdb378959d70a47d6035800dc6d2b607e81dbd63b8411f42420cafb73dc7`.
- `full-contract-proof.json`, Exit0: sämtliche 38.273 Dokumente und 1.109.153 Fakten vollständig auf alle Vertragspflichtfelder/Typen, Belegzustände, Quellen-/Seiten-/Revisionsidentität, UTC-Batchbeobachtung und Lizenztypen geprüft. 0 doppelte ID-/Revisionsschlüssel über alle sechs Eingaben; durchgehend `redistribution_allowed=false`. SHA256 des Belegs: `c0b3beb1eeb8cac6878da7eee112199ae0e053ee548bffeb61f0f9c924a9f31d`.
- `jsonl-size-proof.jsonl`: vollständige Zeilen-/Byte-/Maximalzeilenmessung aller sechs finalen JSONL-Dateien, Werte wie oben.
- `originals-after-check.log`: 47 erfolgreiche Prüfzeilen, echter SHA256-Prüfexit0. Alle fünf Originalarchive, beide siteinfo-Belege, vier Archivmetadaten, zwölf Raw-Captures und 24 HTTP-Payload-/Sidecar-Dateien unverändert gegen `originals-before.sha256`.

Diese Belege liegen ausschließlich unter `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/`. Zusammen mit echten `test-4.log`, `test-7.log`, `test-8.log`, `clippy-1.log`, `build-1.log`, `fmt-1.log`, `fmt-freeze5.log`, `fmt-isolated.log` und `data-1.log` erhalten.

### Konflikte, Grenzen und Freigabestatus

Die tatsächlichen fünf Archive/zwölf Captures erzeugten keine Konfliktdateien; ihre Originalrevisionsvergleichsprüfungen fanden keine abweichenden Inhalte zum selben Schlüssel. Im ausgeführten Originaltestprogramm bestanden unter anderem `same_revision_different_content_is_retained_as_conflict_without_overwrite`, `api_conflict_then_xml_keeps_separate_authors_captures_and_first_observations` sowie alle Fix4-/Fix5-Sync-/Budgetregressionen. Diese belegen reale lokale Dateisystem-Fehlerpfade und Original-/Konflikterhaltung, keinen echten Stromausfall und keinen DB-Import.

Alle Quellen bleiben historische/partielle Offline-Quellen mit `inventory_complete=false`, `content_complete=false`, 0 unbekannten Revisionen. Zwölf Captures belegen Abrufe vom 02.05.2026 und ihre April-/Mai-Revisionen, keine heutige Abdeckung. Archiv-errors.log-Lücken und fehlende aktuelle vollständige Wiki-Inventarisierung werden durch vollständige Verarbeitung der vorhandenen XML-Dateien nicht beseitigt. Die heutige getrennte 403-Recherche ist keine zusätzliche Wissensquelle für diesen Lauf. Die einzelne Miraheze-Hauptseite bleibt ohne nachgewiesenen Spielwissenskorpus.

Lizenzen der vier fremden Archive stammen ausschließlich aus ihren eigenen Originalmetadaten beziehungsweise bleiben ohne Nachweis ungeprüft. Archive.org-Metadaten sind kein Rechtebeweis für jede Revision oder Valve-/Medienassets. Hauptarchiv verwendet eigenes historisches siteinfo, Legacy-Captures ihren separaten siteinfo-/Abrufbezug. Raw-Lua/JS und Dateibeschreibungen nur als Daten verarbeitet, keine Medienassets oder Quellenausführung. Kein Rechteupgrade oder Veröffentlichungsrecht.

Endhashs aller vier Originalmodule exakt Freeze5; eigener `src/main.rs` weiterhin `d358ec15a692aeb57cbbd7f05e62be4eaa85217c3b5eaf59db4a63fc23f201fd`, eigenes Cargo.toml `9ad50a0515c37ecd19a87406dafb2bf3c6c6164b3c905d60cffb0649f7e05620`, eigener Lockfile `ab2a866b93478823752eaafed59164faf06bcd214335257c597ecc4f8e7d2c6a`, ausgeführtes Binary `83349db303f5661ba4c6f0ac156b4ec14ab93e36bf85d36b90aa67b163d48578`. Keine Änderungen an produktivem Core, Util, lib.rs, produktiven Manifesten, Schema oder den Modulen durch diesen Datenworker, keine fremden Prozesse/Worktrees, keine Secrets/ENV, kein Netzwerk/DB/LLM/Git/Deploy.

**Lokaler Workerauftrag abgeschlossen und ausschließlich an A gemeldet.** Unabhängige abschließende Daten-/Intent-Abnahme, Modulcommit durch A, C3-Registrierung/Verbraucherprüfung einschließlich Rekursionsgrenze und Maximalzeile, Import, Gate und Deployment sind separate Zuständigkeiten und hier nicht als erledigt behauptet.
