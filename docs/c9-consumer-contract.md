# C9-Vertrag für Producer, Auth und Consumer

Die öffentliche API verwendet weiterhin `answer_for_publication`. `publication_allowed=false` wird durch interne Leserechte oder Provider-Egress niemals zu einer Veröffentlichungsfreigabe. Bestehende Veröffentlichungstests bleiben maßgeblich.

## Getrennte Identitäten und Releases

| Principal | Scope | Wissensstand | Transport |
| --- | --- | --- | --- |
| `twitch-bot/twitch` | `bot.public` | Bisheriger fester Standardrelease | Öffentliche lokale HTTP-API |
| `docs-client/docs` | `docs.public` | Eigenes Release der 38 geprüften öffentlichen Docs-Seiten | Öffentliche lokale HTTP-API |
| `second-brain/internal` | `second_brain.internal` | Eigenes internes Release der freigegebenen Betriebsseiten | Privater Unixsocket |

Die normale Serverkonfiguration bindet authentifizierte Principals an feste Release-IDs und Knowledge-Versionen. Die Query enthält weder Principal noch Releasewahl. Start und Readiness prüfen alle gebundenen Releases einschließlich Policy, aktueller Köpfe und Quellenidentität. Fehlende oder unpassende Releases verhindern den Start beziehungsweise die Readiness.

`BRAIN_SERVE_DOCS_PUBLIC_TOKEN` und `BRAIN_SERVE_SECOND_BRAIN_TOKEN` bezeichnen getrennte Infisical-Credentials. Vorhandene Twitch-Identität, Scopes und Standardrelease bleiben erhalten. Metadaten, Credentialnamen und Konfiguration enthalten keine Tokenwerte.

## Interner Operatorweg

`/v1/operator/query` wird ausschließlich an einem absoluten privaten Unixsocket registriert. Das Verzeichnis muss dem freigegebenen lokalen Benutzer gehören und Modus `0700` haben; der Socket hat `0600`. Der Listener prüft die tatsächliche Peer-UID über `SO_PEERCRED`. Vorhandene Socketpfade blockieren den Start. Der Client prüft Besitzer und Rechte erneut vor jeder Anfrage.

Die öffentliche CredentialRegistry enthält die interne Credential nicht. Die öffentlichen Answer- und Retrievalwege verweigern die C9-interne Identität zusätzlich. Der öffentliche Router besitzt keine Operatorroute; eine Weiterleitung auf seinen TCP-Port kann den internen Weg deshalb nicht erreichen.

`brain.internal.operator.v1` ist ein eigener typisierter Vertrag mit lokaler Audience, Release und Originalausschnitten samt Quellenkennung, Dokumentkennung und Revision. Er enthält keine öffentliche Textantwort. Der Weg ruft weder Kernel noch Modellanbieter auf. Aktuelle ACLs werden vor Rückgabe der Belege erneut geprüft. Anfrage, Antwort, Aufnahme und Laufzeit sind begrenzt; Abbruch und Timeout stornieren das gemeinsame RequestDeadline.

Der 2nd-Brain-Adapter verwendet ausschließlich diesen internen Typ und `second_brain.internal`. Seine Ausgabe ist für die lokale Operatoranzeige bestimmt. Öffentliche Bot-, Discord-, Twitch- und Exportpfade dürfen ihn nicht übernehmen. Docs bleibt auf `PublicAnswerResponse`.

## Daten und Import

`brain-maintain import-c9-release` übernimmt hashgebundene CoreDocuments über den vorhandenen DocumentSet-/PgStore-Vertrag. Es schreibt ein eigenes unveränderliches Release atomar, ohne Serverkonfiguration oder aktiven Standardrelease zu ändern. Quellenidentität, Rohhash, Gitrevision und exakte Policy werden vor jeder Wirkung geprüft. Wiederholung prüft dasselbe veröffentlichte Paket; ein fremder bestehender Quellcheckpoint wird nicht überschrieben.

Docs veröffentlicht ausschließlich `docs.public` mit Public-Visibility und expliziter Veröffentlichung/Egress-Freigabe. Das interne Paket umfasst genau `systeme/deadlock-bots.md` und `projekte/brain-feeder.md` aus dem belegten 2nd-Brain-Gitstand, ohne Textänderungen. Publication, Provider-Egress und Raw-Retention sind dort false. Datierte Betriebsseiten belegen gespeichertes Wissen und keinen heutigen Livezustand.

## Gemeinsame Abnahme

Die Abnahme umfasst Scope-, Principal- und Release-Isolation, fehlende Releases, Neustart/Readiness, aktuelle ACLs, Socketrechte und Peer-UID. Dieselbe Docs-Credential muss die echte öffentliche Voice-Anleitung liefern und auf internem Scope sowie Operatorweg verweigert werden. Die interne Credential muss auf den öffentlichen Answer-/Retrievalrouten verweigert werden. Die Operatorroute darf am öffentlichen TCP-Port nicht vorhanden sein. Interne Inhalte und Secretwerte gehören nicht in Prüflogs.

Die drei gemeinsam geänderten Serve-Dateien werden mit dem getrennten Brain-Provider-Eigenanteil kombiniert und vollständig geprüft. Der Peer verantwortet seine Provider-/Modellverdrahtung; C9 führt keinen eigenen Modellwechsel durch. Die lokale Codex-CLI hat keinen belegten harten Ausgabetokendeckel. Vorabbudget und nachträgliche Verbrauchsprüfung sind keine harte Deckelgarantie. Nullbudget, deaktiviertes Toolschema, Timeout, Prozessgruppen und Bytegrenzen benötigen Laufzeitbelege am finalen Peerstand. Der interne Operatorweg bleibt ohne LLM/Egress.

Der konkrete Gruppenstatus, Pakethashes, Bootstrapentwurf und finalen Köpfe stehen im gemeinsamen Taskordner unter `C9-GRUPPENVERTRAG.json`, `DOCS-C9-VERTRAG.json` und den beiden Release-Manifesten. Ein Komponentengate ersetzt die gemeinsame Abnahme nicht. Vor Gruppen-ALLOW erfolgen kein produktiver Import, Grant, Serverstart oder Einzelmerge.

## Gemeinsamer Lernjobbetrieb

Das gemeinsame Release enthält die drei kompilierten Binaries `brain-serve`, `brain-maintain` und `deadlock-brain`. Das Legacybinary wird zusätzlich als Hardlink unter `legacy/sheet-sync/deadlock-brain` und `legacy/build-data/deadlock-brain` installiert. Jeder dieser beiden Jobstämme besitzt normale `config/ai.json`, `config/infisical.json` und `config/settings.json` ohne Secretwerte. Sheet-Sync behält `settings.data_dir=/home/nathanael/repos/Deadlock-Brain/data`, Build-Data behält `/home/nathanael/.worktrees/brain-live-main/data`. Es erfolgt keine Datenkopie oder Migration.

Die direkten revisionsgebundenen Rust-ExecStarts behalten das bestehende `LoadCredential=infisical-token`. Die normalen Infisicalconfigs verwenden `credential_name=infisical-token`. Der vorhandene systemd Runtime-Credentialdirectory hat Vorrang: fehlende, leere oder ungültige Credentials führen zum Abbruch. Der FD-Ladeweg ist nur außerhalb dieses Runtimewegs zulässig. Die Lernjob-Units öffnen keinen FD5. Der alte `INFISICAL_TOKEN_FILE`-ENV-Fallback entfällt im eng begrenzten Peeranteil.

Vor Freigabe müssen beide tatsächlichen Hardlink-Startpfade den richtigen Exepfad, Konfigstamm und unveränderten Datenpfad auflösen. Gleicher Inode und Binaryhash sind nachzuweisen. Fehlende normale Konfiguration oder Credentials dürfen keinen Rückfall auf Kanoncheckout, Arbeitsverzeichnis, FD oder ENV auslösen. Die fachlichen Jobargumente bleiben erhalten. Erst nach gemeinsamem Gate und abgestimmtem Releaseslot erfolgen Aktivierung und echte Lernjobprüfung ohne überlappende Timerläufe.


## Gemeinsame normale Bot-TOML

Serve und Maintenance lesen dieselbe explizite absolute `<SHA-Release>/config/bot.toml`. Serve übernimmt seine bisherigen Felder aus `[brain.serve]`, öffentliche Grants aus `[[brain.serve.credentials]]` und die sicheren Infisical-Metadaten aus `[brain.infisical]`. Historische Feldnamen wie `api_key_env`, `password_env` und `token_env` bezeichnen ausschließlich Infisical-Schlüssel. Es findet kein ENV-Lesen dieser Werte statt.

Die private Second-Registry steht ausschließlich unter `[brain.operator.credential]`, ihr Socket und Release unter `[brain.operator]`. Beide Registries werden bei Start und Readiness gegen ihre festen Releases geprüft. Die öffentliche Registry nimmt keine Second-Identität oder Second-Scope auf. Eine Maintenance-Aktivierung verändert ausschließlich `[brain.serve.release]` und erhält sämtliche übrigen Tabellen einschließlich der Operatorbindung.

Die bisherigen Maintenancefelder stehen direkt unter `[brain.maintenance]`, der bestehende Runnervertrag unter `[brain.maintenance.runtime]`. Dessen `maintenance_config`, `serve_config` und `infisical_config` müssen exakt dieselbe absolute Bot-TOML benennen. Alte JSON-Konfigurationen werden nicht geladen. Normale Configdateien werden nichtblockierend und ohne Symlinkfolge geöffnet, vor dem Lesen als reguläre Dateien geprüft und begrenzt gelesen. C9 akzeptiert genau eine sichere Credentialquelle, prüft private reguläre Credentialdateien beziehungsweise Descriptoren und erhält den positionsunabhängigen bestehenden Loader. Ein ungültiger expliziter FD führt zum Fehler und wechselt nicht auf einen Dateipfad.

Die normalen Adaptertabellen sind `[brain.docs]` und `[brain.second_brain]` mit ihren jeweiligen `.infisical`-Tabellen. Principal, Kanal, Scope und Release sind dort keine Berechtigungswahl. Die beiden Legacy-Hardlinkstämme verwenden jeweils ihre eigene `config/bot.toml` mit `[ai]`, `[infisical]` und `[settings]`. Ihre bisherigen externen Datenpfade bleiben erhalten. Die vorhandene Community-FD3-Strecke und deren Bootstrapmetadaten bleiben unverändert; ihr gemeinsamer Credential-Dateiopen erhält nur die abgestimmte FIFO-Korrektur.

Diese Quellumstellung benötigt neue Gates und die zugesagte Nachsuite. Vorlagen mit `@RELEASE_ROOT@` sind noch nicht aktiviert. Vor dem Gruppengate werden die tatsächlich geprüften Source-SHAs und Deploypfade eingesetzt. Komponenten-ALLOW, Formatierung und syntaktisch gültige TOML ersetzen keine gemeinsame Abnahme oder produktive Auth-/Release-/Socket-/Lernjobprüfung.
