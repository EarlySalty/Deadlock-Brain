# Deadlock Brain über das ChatGPT-Abo

## Prüfstand nach der ersten Suite

Die erste eingefrorene TOML-Suite ist beendet: `check`, `test` und `clippy` jeweils Exit 0 mit der im Repository festgelegten Toolchain 1.97.1, `--locked --offline -j 2` und den neun betroffenen Paketen. 657 Tests bestanden, 48 wurden als bestehende ignorierte Tests übersprungen. `SOURCE-MANIFEST.sha256` kennzeichnet die geprüften Dateiinhalte; `SOURCE-DIFF.patch` enthält den damaligen Diff der bereits getrackten Dateien. Dieses erste Artefakt ist kein vollständiges Inhaltsarchiv der damals neuen Dateien. Beide Hostlocks sind freigegeben; die abschließende Prozessprobe zeigte keinen aktiven Compiler.

Die erste Fixrunde korrigierte die Budgetmeldung, installierte Settingsvollständigkeit und ungenutzte ENV-Helfer und ergänzte vier CLI-Schalter. Ihre ursprünglichen Artefakte sind als historische Version erhalten. Die abschließende Prüfung von Fixrunde 2 ist inzwischen erfolgreich abgeschlossen.

Auch mit den ergänzten Schaltern ist die tatsächliche Werkzeugfreiheit weiterhin offen. Dafür muss das generierte `tools`-Array der passenden CLI-Ausführung belegt sein. Eine Promptanzeige oder die nachträgliche Ablehnung von Werkzeugereignissen genügt nicht. Es gibt weiterhin keinen belegten harten Tokendeckel für einen bereits gestarteten Abo-Aufruf. Kein Commit, gemeinsames Release oder produktiver Neustart wurde als fertig gemeldet.

Konkrete offene Abnahmegrenze: Für CLI 0.160.0 ist keine sichere native Telemetrie belegt, die ausschließlich `inference.tools[]` beim unveränderten OpenAI-/ChatGPT-Abo ausgibt. Rohrequest- oder Traceaufzeichnung ist wegen möglicher privater Promptfelder kein zulässiger Ersatz. Ein isolierter authfreier Loopback-Mock könnte nur seinen lokalen Lauf belegen; mögliche Unterschiede bei Provider, Modellmetadaten und Authpfad verhindern daraus eine Produktionsfreigabe. Der gebündelte gpt-6-luna-Katalog nennt unter anderem ein Freeform-Patchwerkzeug, Code Mode und experimentelle Clock-/Nachrichtenwerkzeuge. Die abschließende Gesamtfreigabe bleibt deshalb offen. Das Gate erhält diesen Befund ausdrücklich; ein grünes Quellgate allein ersetzt ihn nicht.

Maßgeblich ist Fixrunde 2. Ihre eigene Format-, Compiler-, Test- und Clippyprüfung ist grün: 657 Tests bestanden, 48 bestehende Tests ignoriert, Clippy mit `-D warnings`. Maintenance benutzt in `author.rs::codex_args` dieselbe `subscription_config_args()`-Funktion wie Legacy. ChatGPT-Login, OpenAI-Provider und Werkzeugschalter sind gemeinsam festgelegt. Modell, Reasoning, Ausgabeschema, Ergebnisdatei und Zeitlimit bleiben erhalten. C9 besitzt weiterhin TOML-, Registry-, Revalidierungs- und Starterhunks. Das anschließende Gate und die gemeinsame Laufzeitabnahme stehen noch aus.

Der bestehende Rust-Reviewer hat auch diesen engen Author-/Helperhunk statisch geprüft. Er meldet keine neuen P1- oder P2-Befunde und bestätigt die erhaltenen Authorvorgaben sowie konsistente einzelne `-c`-Argumentpaare. Dieser begrenzte Beleg ersetzt keine Compiler-, Gate-, Werkzeug- oder Abo-Laufzeitprüfung.

Der vollständige geprüfte Quellinhalt steht in `FIXRUNDE-2-SNAPSHOT.tar`, SHA256 `68ee87b69b854351b59944fa52c78e888cc85021861a0af3e70a51f945ef7558`. `FIXRUNDE-2-MANIFEST.sha256` hat Hash `80b32ae707fd9476bf42913f33ef754f71c6e5c2108342ddd87e0d83010795e1`. Alle manifestgebundenen Quellen waren vor und nach den Prüfungen identisch. Die aktuellen Logs heißen `final-fmt.log`, `final-check.log`, `final-test.log`, `final-clippy.log`, `final-consumer-test.log` und `final-consumer-clippy.log`. Erst das nachfolgende Gate gilt dem vollständigen Eigencommit; die gemeinsame Installation ist davon getrennt.

Basis: `511a347b653beba13c2bf130f4bead7a7196cc2a` (`origin/main` und zuvor aktiver Serve-Stand).
Arbeitsbranch: `codex/brain-luna-abo-20261003`.

Der Nutzer hat `gpt-6-luna` ausdrücklich für Deadlock Brain freigegeben. Der lokale Codex-Login meldet ChatGPT-Anmeldung. Ein echter Probeaufruf mit diesem Modell und erzwungener ChatGPT-Anmeldung antwortete `LUNA_ABO_OK`. Dieser Einzelbeleg ersetzt keine Abnahme der integrierten Dienste. Fireworks und automatische Modellwechsel entfallen im zentralen Legacy-Textpfad. Der Serve-Provider nutzt denselben Abo-Connector.

Für die Integration gilt der [gemeinsame TOML-Vertrag](/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/TOML-VERTRAG.md). Beide eigenen Quellsuiten sind grün. C9s gemeinsame Integration benötigt ihre eigenen Prüfungen; Installation und Produktionsfreigabe sind noch nicht belegt. Neue normale Laufzeitkonfiguration wird ausschließlich als Bot-TOML bereitgestellt. Alte JSON-Pfade dokumentieren den vorherigen Betrieb und sind kein Deployziel oder Fallback.

## Besitz und gemeinsame Integration

C9 besitzt TOML-Parser, Bootstrap, Registry, Units und deren tatsächliche Aufruferbindung. Das gilt auch für Brain-Maintenance einschließlich der bisherigen normalen Maintenancefelder. Der Abo-Peer liefert seine belegten Provideränderungen, den konsolidierten Prozessrunner und die dazugehörigen Cargo-/Reexportänderungen. Der bestehende Maintenance-Runner wird in `brain-providers` konsolidiert; Maintenance reexportiert ihn.

Die betroffenen Providerstellen sind `brain-serve/src/config.rs`, `secrets.rs` und `service.rs`: `ProviderKind::CodexCli`, optionaler CLI-Pfad, kein Infisical-API-Schlüssel für diesen Provider und Auswahl über `TextProvider`. C9 übernimmt diesen Eigenanteil im isolierten Integrationworktree und prüft ihn gemeinsam mit seinem TOML-/Registryvertrag. Datenbankrechte, Principal-Releasebindungen und interner Router werden nicht durch den Provideranteil geändert.

Der enge Legacy-Datenbankanteil ersetzt den früheren DSN-ENV-Zugang durch den bestehenden direkten Infisicalweg. Die installierten Legacy-Aufrufer laden dessen normale Zuordnung aus `[infisical]` ihrer Bot-TOML. C9 und Peer prüfen diese feldgenaue Bootstrap gemeinsam. Der fremde uncommittete `pg.rs`-/`pg_secrets.rs`-Stand im Kanon wird weder übernommen noch überschrieben. Dieser Auftrag wird nicht einzeln gemergt oder veröffentlicht.

## Release und Konfiguration

Serve und Maintenance verwenden dieselbe revisionsgebundene Datei `<gemeinsames SHA-Release>/config/bot.toml`. Serve liest `[brain.serve]`, `[brain.infisical]`, `[[brain.serve.credentials]]` und `[brain.operator]`. Maintenance liest `[brain.maintenance]` einschließlich der verschachtelten normalen Felder, etwa `[brain.maintenance.codex]`. `codex.model` wird im beauftragten Brain-Umfang auf `gpt-6-luna` gesetzt; vorhandene CLI-Datei, `timeout_ms` und bisherige Reasoning-Vorgaben werden erhalten.

Die Binaryliste des Eigenanteils umfasst `brain-serve`, `brain-maintain` und `deadlock-brain`. Weitere C9-Komponenten bestimmt dessen Vertrag. Das einmal gebaute `deadlock-brain` liegt zusätzlich per Hardlink in zwei eigenen Launcher-Stämmen. Ein Symlink genügt für die unterschiedliche physische Rootzuordnung nicht.

| Aufrufer | Revisionsgebundene Konfiguration | Tabellen und Datenpfad |
| --- | --- | --- |
| Brain-Serve und Brain-Maintenance | `<gemeinsames SHA-Release>/config/bot.toml` | `[brain.serve]` und `[brain.maintenance]` mit den zugehörigen Server- und Credentialtabellen |
| Sheet-Lernjob | `<gemeinsames SHA-Release>/legacy/sheet-sync/config/bot.toml` | `[ai]`, `[infisical]`, `[settings]`; `settings.data_dir=/home/nathanael/repos/Deadlock-Brain/data` |
| Builddata-Lernjob | `<gemeinsames SHA-Release>/legacy/build-data/config/bot.toml` | `[ai]`, `[infisical]`, `[settings]`; `settings.data_dir=/home/nathanael/.worktrees/brain-live-main/data` |

Normale Vorlagen liegen unter `ops/luna-abo/*.bot.toml`. Die Feldnamen und Typen folgen dem gemeinsamen Vertrag und den bestehenden Funktionsverträgen. Jeder Legacy-Stamm erhält genau eine `config/bot.toml`. Neue `ai.json`, `infisical.json`, `settings.json` oder `maintenance.json` werden nicht als Laufzeitkonfiguration ausgerollt. Die beiden bisherigen Datenpfade bleiben exakt erhalten; Daten werden nicht verschoben und ein gemeinsamer Settingspfad ersetzt die beiden Jobstämme nicht. Ein data-Symlink im Release ist für diese externen Datenpfade nicht nötig.

Die Legacy-TOML enthält die bestehenden Kompatibilitätswerte `ai_max_completion_tokens=16000`, `ai_temperature=0.2`, `ai_top_p=0.9` und `ai_use_token_plan=false`. Die CLI unterstützt Temperatur und Top-p nicht; diese Werte dürfen nicht als wirksame Modellparameter dargestellt werden. Für `ai_reasoning_effort` gibt es bislang keinen neuen Default. Eine vorhandene optionale Vorgabe wird an `model_reasoning_effort` weitergegeben. Das maximale Tokenbudget bleibt eine Antwortvorgabe mit nachträglicher Verbrauchsprüfung, keine belegte harte CLI-Tokenbegrenzung.

C9 ersetzt `@RELEASE_ROOT@` in den Unitvorlagen vor dem Gruppengate durch den tatsächlichen vollständigen Source-SHA und die belegte Releasewurzel. Units verwenden direkte revisionsgebundene Executables und die explizite Configbindung beziehungsweise die real geprüfte Rootauflösung. Ein `current`-Pfad ist kein Ersatz für diesen Beleg. Erst der gemeinsam geprüfte gerenderte Diff darf aktiviert werden. Die Vorlagen aktivieren nichts und überschreiben keine kanonischen Skripte.

## Strikte Legacy-Auflösung

Im installierten Betrieb bestimmt Legacy seine Jobwurzel aus dem physischen Pfad der ausführbaren Datei, einschließlich des jeweiligen Hardlinkstamms. Die Rootwahl darf nicht vom Vorhandensein einer Configdatei oder eines data-Unterordners abhängen. Jeder Job liest `config/bot.toml` aus dieser Jobwurzel und verwendet den dort ausdrücklich angegebenen externen Datenpfad.

Eine fehlende, unlesbare oder ungültige installierte Bot-TOML führt zum Fehler. Dasselbe gilt für ein fehlendes oder relatives `settings.data_dir`. Es gibt keinen JSON-, ENV-, CWD-, Kanon- oder Bauworktreefallback. Entwicklungsstandards sind nur nach belegter Quellwurzel mit `.git`, `rust/Cargo.toml` und `src/deadlock_brain` zulässig. Das vorhandene Verhalten ist an beiden tatsächlichen Launcherpfaden nachzuweisen; die Layoutbeschreibung allein ist kein Laufzeitbeleg.

## Credentials und Berechtigungsgrenzen

Secrets bleiben in Infisical. Bot-TOML enthält ausschließlich normale Einstellungen und sichere Credentialquellenparameter. Secretwerte werden weder gelesen, ausgegeben noch in TOML oder ENV kopiert.

Die vorhandenen Legacy-Units verwenden `LoadCredential=infisical-token:%h/.config/infisical-tokens/infisical-token-bots`: bei Builddata in der Serviceunit, beim Sheet-Lernjob im Drop-in `20-creds.conf`. Beide sind `Type=oneshot`. Die direkten Rust-Aufrufe erhalten `CREDENTIALS_DIRECTORY` vom Service-Manager als Laufzeitmetadatum und öffnen darin das festgelegte `credential_name=infisical-token`. Fehlt dieses Runtime-Credential, wird abgebrochen. Diese Units öffnen keinen FD 5. Der vorhandene sichere FD-Weg bleibt für tatsächliche FD-Aufrufer außerhalb dieses Runtimewegs verfügbar. Ein alter Secret-Executor mit ENV-Weitergabe gehört nicht in den neuen Unitaufruf.

Die Serverregistry bleibt bei C9. `[[brain.serve.credentials]]` bindet den sicheren Secretbezug serverseitig an Principal, Kanal, Scope und freigegebenes Release. Twitch bleibt `twitch-bot` / `twitch` / `bot.public`; Docs ist `docs-client` / `docs` / `docs.public`. Second bleibt ausschließlich in der privaten Operatorregistry unter `[brain.operator]`, mit `second-brain` / `internal` / `second_brain.internal`. Adapter wählen ihre Berechtigungen nicht selbst. Der tatsächliche Docs-FD5-Vertrag und der private Second-Unixsocket bleiben erhalten. Diese Grenzen werden gemeinsam mit dem Providerpfad geprüft.

## Tatsächliche Aufrufer und bisheriger Betrieb

Die folgenden Angaben beschreiben den erhobenen Altstand, keine neuen Deployziele:

| Aufrufer | Bisheriger Startpfad | Notwendige Änderung |
| --- | --- | --- |
| `brain-serve.service` | `/opt/deadlock-brain/maintenance-current/brain-serve` | Gemeinsames SHA-Release mit Abo-Provider und `[brain.serve]` |
| `brain-maintenance.service` | `/opt/deadlock-brain/maintenance-current/brain-maintain` | Gemeinsames SHA-Release mit `[brain.maintenance]`; bisheriges Modell `gpt-6.1-sol` wird `gpt-6-luna` |
| `deadlock-brain-sheet-sync.service` | kanonisches `scripts/run_sheet_sync_with_infisical.sh`; Defaultbinary im Kanon vom 23. September | Direkter Aufruf im eigenen Sheet-Hardlinkstamm |
| `deadlock-brain-build-data.service` | `brain-live-main/scripts/run_build_data_with_infisical.sh`; Defaultbinary im Worktree vom 30. September | Direkter Aufruf im eigenen Builddata-Hardlinkstamm |
| `deadlock-brain-wiki-refresh.service` | `~/.local/share/deadlock-brain/wiki-refresh/deadlock-brain` | Separater Wiki-Importer ohne zentralen Textaufruf in diesem Job |
| `deadlock-brain-youtube-learning.service` | `/opt/deadlock-brain/current/bin/deadlock-brain-yt` | Bestehender Gemini-Browserpfad bleibt erhalten |

Serve und Maintenance liefen zuvor auf Basis `511a347`; das Legacy-Bundle unter `current` war `be2aa6bd5a6504e99693f7dd1edaa16e76be91b4`. `~/.config/deadlock-brain/brain-serve.json` und `/etc/deadlock-brain/maintenance.json` sind historische Configpfade. Der neue gemeinsame Release führt diesen normalen JSON-Betrieb nicht fort. C9 muss alle tatsächlichen Maintenance-Aufrufer an die neue gemeinsame TOML binden.

Die Sheet-Reihenfolge bleibt `refresh-sheet`, `learn analyze-next --limit 20`, `enrich patch-impact --limit 50`, `enrich meta-trends`. Die Unitvorlagen bewahren diese Aufrufe. Nur `/opt/deadlock-brain/current` umzuhängen erreicht den bisherigen Sheet-Lernjob nicht.

Für die geprüften Timerketten werden Sheetdaten aus `raw_dir` und `cache_dir`, Postgres-Kontexte und öffentliche API-Daten verwendet. Die Populationsmigration ist per `include_str!` im Binary enthalten. Für diese Jobaufrufe ist kein Laufzeitzugriff auf `game-wiki/` oder `.tasks/` erforderlich. Manuelle Reasoner-Kommandos können weiterhin `.tasks/2026-09-12-build-reasoner/referenz` benötigen und gehören nicht zu diesen Timerketten. `deadlock-brain-yt` braucht für den Provideranteil keinen Neubau; seine generative Analyse benutzt den bestehenden Gemini-Browserpfad. Dessen tatsächliche Verwendung bleibt vor Abschluss zu dokumentieren.

## Offene Abnahme und Prüfstand

Die historischen Check- und Core-Testläufe endeten mit Exit 101 beim damaligen Manifestumbau. Danach wurden sowohl die erste TOML-Suite als auch Fixrunde 2 erfolgreich geprüft. Die zweite Prüfung umfasste alle neun betroffenen Pakete: Connector, Core und Maintenance mit eigener Testsuite sowie sechs abhängige Consumerpakete mit aktueller Integrationssuite. Compilercheck und Formatprüfung endeten mit Exit 0, beide Clippyrunden mit `-D warnings` ebenfalls. Die Gesamtzahl des finalen Stands beträgt 657 bestandene und 48 ignorierte Tests. Der eigene Slot bleibt bis Sourcecommit, Gate und vollständigem Abbau gebunden.

Der vollständige aktuelle Sourcefreeze besteht aus `FIXRUNDE-2-MANIFEST.sha256`, `FIXRUNDE-2-FILES.txt` und `FIXRUNDE-2-SNAPSHOT.tar`. Zur Reproduktion wird Basis 511a347 bereitgestellt und das Archiv an deren Wurzel entpackt; anschließend prüft `sha256sum -c FIXRUNDE-2-MANIFEST.sha256` sämtliche Dateiinhalte einschließlich neuer Quelldateien, TOML und Ops. Die vorherige Version `FIXRUNDE-SNAPSHOT.tar` mit Hash `9ff84dda80975981da1ac23b58c5f6244aab8573bc91151e472756aefc47ad2f` ist ausschließlich ein historisches Artefakt.

Vor gemeinsamer Freigabe fehlen weiterhin:

- Ausführung beider tatsächlicher Hardlink-Startpfade mit dokumentiertem physischem Exepfad, Root, geladener Bot-TOML, Binaryhash und Inode. Beide Startpfade müssen dasselbe Binary und ihre jeweils eigene Configwurzel belegen.
- Positive Datenpfadprüfung: Sheet verwendet exakt `/home/nathanael/repos/Deadlock-Brain/data`, Builddata exakt `/home/nathanael/.worktrees/brain-live-main/data`.
- Negative Prüfungen je installierter Root: fehlende und unlesbare Bot-TOML, ungültiges TOML sowie fehlendes oder relatives `settings.data_dir` müssen ohne Fallback scheitern. Fehlende Runtime-Credentials müssen ebenfalls abbrechen.
- Gemeinsame feldgenaue TOML-/Bootstrapprüfung mit Auth- und Releaseisolation sowie Prüfung aller tatsächlichen Maintenance-Aufrufer.
- Runnerbelege für Timeout, gesamte Prozessgruppe, Bytegrenzen, Abo-Authentifizierung und tatsächlich ausgeschlossene Werkzeuge.
- Aktuelle Rust-Prüfungen, unabhängige Intent-Abnahme, Rust-/Security-Review und `gate_hook.py --review` gegen die integrierte Arbeit.

Serve hatte ein Eingabebudget von 12.000 Tokens und ein Antwortbudget von 768 Tokens. Ein erster CLI-Probeaufruf meldete 11.880 Eingabetokens. Der tatsächliche CLI-Overhead muss mit ausgeschlossenen Werkzeugen erneut gemessen werden. Nutzkontext, CLI-Systemprompt und Serialisierung tragen gemeinsam zum Verbrauch bei. Die nachträgliche Prüfung ersetzt keine vorab garantierte Tokendeckelung. Nullbudgets und zu große Nutzkontexte werden vor dem Aufruf abgewiesen; der gemeldete Verbrauch wird danach geprüft.

Der statische Security-Review verlangte zusätzlich einen Beleg am tatsächlichen Tool-Schema. Deaktivierte Features und eine nachträgliche Ablehnung von Werkzeugereignissen reichen dafür allein nicht. Die bisherige statische Nachprüfung meldete nach den Korrekturen keinen weiteren Befund; sie ersetzt die noch offenen Laufzeitprüfungen nicht. Die offizielle [Konfigreferenz](https://learn.chatgpt.com/docs/config-file/config-reference) dokumentiert `forced_login_method` und `model_reasoning_effort`, jedoch keine `max_output_tokens`-Option.

Noch kein Commit, Merge, Push, Releasebuild oder Deploy dieses Eigenanteils. Nach den erfolgreichen abgestimmten Prüfungen geht der belegte Eigencommit an C9 zur gemeinsamen Integration. Erst nach gemeinsamer Abnahme werden revisionsgebundene Units und Bot-TOML aktiviert, Dienste neu gestartet und echte Antworten geprüft.
