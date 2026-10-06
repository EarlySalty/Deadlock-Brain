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

## Discord-Spielwissen

Der Discord-Bot fragt über `/v1/answer` mit `dl-bot/discord`, `bot.public` und der bestehenden Infisical-Referenz `DISCORD_BRAIN_CLIENT_TOKEN`. Damit er die aktuellen Spielprofile für seine Antworten erhält, braucht dieses Credential ausdrücklich `entity_profile_model_context=true`. Eine Freigabe für `docs-client/docs` gilt nur für diesen Consumer. Ohne die Discord-Freigabe durchsucht der Bot ausschließlich den gebundenen Dokumentrelease und kann trotz vorhandener Spielprofile keine passende Antwort finden.

Der gebundene Release muss auch die veröffentlichten Spielprofile enthalten.
Ein reiner Dokumentrelease reicht trotz gesetzter Modellfreigabe nicht aus.
Interne Rohspielquellen bleiben intern. Der vorhandene Publisher erzeugt daraus
geprüfte kompakte Dokumente unter `git-game-facts-derived` mit eigener Herkunft
und Veröffentlichungsfreigabe. Bei einer Standard-Aktivierung folgen freigegebene
Spielconsumer dem neuen Release, wenn ihre bisherige Bindung derselben Basis
entspricht. Eigenständig gebundene Releases und andere Consumer bleiben erhalten.

[`discord-credential.example.json`](../ops/brain-maintenance/discord-credential.example.json) zeigt den Eintrag für die bestehende `credentials`-Liste. Die Releasewerte müssen dem bereits veröffentlichten Wissensstand entsprechen. Der Dienst benötigt außerdem den vorhandenen `entity_profile_maintenance_config`-Pfad. Die Konfiguration wird über `brain-maintain write-serve-config` mit dem bisherigen SHA und dem gemeinsamen ConfigWriter geändert; anschließend wird `brain-serve.service` neu gestartet. Eine lokale Probe mit `deadlock-brain answer`, `bot.public` und derselben Discord-Secretreferenz prüft den öffentlichen Antwortpfad, ohne eine Discord-Nachricht zu senden. Tokens bleiben dabei im Infisical-Client und im Prozessspeicher.

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
# GPT Luna über das Abo

Deadlock-Brain hat einen eigenen `CodexSubscriptionProvider` am bestehenden
`AnswerProviderPort`. Er verwendet GPT Luna (`gpt-6-luna`) über die vorhandene
Codex-Anmeldung mit ChatGPT-Abo. Die Anbieter-Konfiguration steht in
`config/codex-subscription-provider.example.json`. Für diesen Weg wird kein
API-Schlüssel geladen und keine bezahlte API als Ersatz angesprochen.

Der Adapter spricht das Nachrichtenformat der vorhandenen Rust-Brücke
`claude-code-proxy` 0.1.43 auf einer eigenen Loopback-Adresse. Die Dienstinstanz
muss geerbte Modell-, Anbieter- und Transport-Overrides entfernen. Die Antwort
allein bestätigt das effektive Modell nicht, da die Brücke den angefragten Namen
zurückgibt. Die feste Modellroute und die isolierte Dienstkonfiguration sichern
die Auswahl. Fremde Proxyinstanzen behalten ihre Konfiguration.

Anfragen enthalten ausschließlich Systemtext und Nutzerdaten, keine Werkzeuge.
Die Denktiefe ist `low`; Denkblöcke werden verworfen. Quellenprüfung,
Berechtigungen, Egress und Nutzungsgrenzen bleiben im Brain. Das Abo hat keinen
hinterlegten API-Tokenpreis. Die Brücke garantiert keine harte Obergrenze über
`max_tokens`; gemeldete Nutzung oberhalb des Brain-Budgets wird verworfen.
