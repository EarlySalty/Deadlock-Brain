status: abgeschlossen, statisches GO für Vorbereitung; Start und Cutover nicht abgenommen
Datum: 2026-09-30

# Unabhängige statische Abnahme der Servevorbereitung

**Fertig J. Fix N für die beauftragte Vorbereitung. Statisches GO. Kein Start-, Deploy- oder G5-GO.**

`SERVE-VORBEREITUNG.md` beschreibt den bestehenden Vertrag und benennt die fehlenden Betriebsbindungen, statt eine startfähige Konfiguration oder einen Livebeweis vorzutäuschen. Kein konkreter blockierender Abgabefehler bestätigt. Die nachstehenden fünf Startvoraussetzungen bleiben offen. Diese Abnahme teilt keine Ressourcenfenster zu und wiederholt keine bereits erteilte bedingte Nutzerfreigabe.

## 1. Bindung und Umfang

- Brainprodukt: `9a29b81d230c01e5c03423cc34ba34c1074eab69`, eingefrorene Git-Blobs aus `/home/nathanael/.worktrees/brain-g5-replay-deferred-20260930`.
- Twitch: `d828481624d53408e0c0a4c3ed1a8e4a6d421c40`, eingefrorene Git-Blobs aus `/home/nathanael/repos/Deadlock-Twitch-Bot`. Kein Rückschluss auf dessen späteren HEAD, installierte Version oder aktive Config.
- Berichtssenke: `/home/nathanael/.worktrees/brain-pre-g5-core-review-20260929`, Branch `review/pre-g5-core-abnahme-20260929`. Ausgangshead `035e2a99a167b98c2838e4bd25a90fda89a3d96d`, zu Beginn sauber und laut lokalem Tracking synchron. Historischer Produktcode dieses Baums wurde nicht als Prüfbasis verwendet.
- Prüfgegenstand: `/home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-30-g5-abschluss/SERVE-VORBEREITUNG.md`, 80 Inhaltszeilen, SHA256 `4767843c72eaf670b322a667169bc6281791474c18876a5d32ba21da9d46e011`.
- Auftrag: `SERVE-REVIEW-BRIEFING.md` desselben Koordinationsordners, SHA256 `0381c45bdcfd32014c9de230b63608cf451743628a5db5adf0c16aee84d09b09`.

Graphify wurde vor den Codeabfragen verwendet. Der globale Graph lieferte passende Twitchstellen, bei Brain teilweise fremde oder historische Treffer. Maßgeblich blieben gezielte Suchen und Lektüre der benannten eingefrorenen Git-Blobs; keine Graphextraktion. Kein erneuter Audit des gesamten Produktdiffs. Der parallele Test-Safety-Bericht gehört nicht zu dieser Abnahme.

## 2. Nummerierte Prüfergebnisse

### 1. Port und Consumerpfad: statisch konsistent

Brain `rust/crates/brain-serve/src/main.rs:26-32` lädt die explizite Config und bestehende Secretauflösung. `config.rs:10` verwendet eine SocketAddr ohne eigene Loopbackpflicht. `service.rs:129-169,262-264` führt die Startprüfungen vor dem Bind aus. Die Forderung nach einer ausdrücklich gesetzten Loopbackadresse ist daher eine notwendige Betriebsbindung, kein bereits vom Datentyp durchgesetzter Schutz.

Twitch `rust/crates/tb-config/src/dashboard_options.rs:12-59` bestätigt Legacy als Default, Shadow/Typed als explizite Modi, erforderliche Public-Scopes und höchstens 60 Sekunden Clienttimeout. Die lokale Begrenzung wird bei der Adapterkonstruktion wirksam: `tb-knowledge/src/brain.rs:30-46` ruft `AsyncBrainClient::new_local` auf. Der Brainclient prüft Loopback in `brain-client/src/async_client.rs:25-39`, schaltet Redirects und Proxys ab (`:43-51`) und hängt `/v1/answer` an (`:63`). Deshalb später einen passenden Basisendpoint konfigurieren, nicht bereits die vollständige Answer-URL. Für diesen Klartext-Servepfad ist ein passender lokaler HTTP-Endpoint erforderlich; eine HTTPS-URL allein fügt dem Server kein TLS hinzu.

Die tatsächliche Twitchkomposition ist vorhanden: `rust/bin/tb-dashboard/src/main.rs:427-440` holt bei Nicht-Legacy den bestehenden Dienstschlüssel und bindet den konfigurierten Adapter ein. `tb-dashboard-api/src/handlers/self_explainer.rs:101-125` belässt den gewählten Modus auch bei fehlgeschlagener Konstruktion; Typed-Fehler führen in `:855-865` zur nicht belegten Ersatzantwort, nicht zum Legacy-Modell. Shadow führt dagegen ausdrücklich zusätzlich zur Typed-Probe den Legacyweg aus (`:866-885`). Die Aussage „kein stiller Rückfall“ ist für Typed korrekt; sie bedeutet nicht, dass Shadow keine Legacy- oder zusätzlichen Provideraufrufe ausführt.

Die in der Vorbereitung genannten belegten Ports bleiben berichtete Beobachtungen der Hauptsession. Der Reviewer hat keine Sockets oder Prozesse abgefragt und keinen freien Port festgelegt.

### 2. Config, Secrets und Dienstidentität: bestehender Weg bestätigt

`brain-serve/src/config.rs:182-208` begrenzt JSON auf 64 KiB und validiert vor Laufzeit. Die Strukturen verbieten unbekannte Felder. PostgreSQL verlangt absoluten Socketpfad, gültige Rolle/Datenbank, Port und Poolgröße 1 bis 16 (`:211-230`). Release und Knowledgeversion sind explizit, `current`/`latest` werden abgewiesen (`:232-238`). Provider-URL, Modellreferenz, Preise, Budgets und Fristen werden geprüft (`:239-298`); das ersetzt keine Betreiberentscheidung für einen konkreten Anbieter oder ein Modell. Analytics bleibt optional und benötigt Schema-, Zeitfenster- und Patchbindung (`:305-323`, `service.rs:132-159`).

`deadlock-brain/src/bin/deadlock-brain-secret-exec.rs:36-55` verwendet den expliziten Infisical-Configpfad und übergibt die vorhandenen Secretwerte intern an genau das Zielprogramm per exec. `deadlock-brain-core/src/pg_secrets.rs:15-25,68-78,164-197` bestätigt Configvertrag sowie Runtime-Credential-/FD-Pfad. Die in der Beispielunit vorhandene normale ENV-Konfiguration muss dafür nicht neu eingeführt werden. `CREDENTIALS_DIRECTORY` als systemd-Mechanismus und der bestehende interne Secrettransport sind von neuer normaler ENV-Konfiguration zu unterscheiden.

Twitch `tb-dashboard-api/src/uplink_config.rs:80-82` liest die Referenz `TWITCH_INTERNAL_API_TOKEN` aus dem installierten Uplink-RAM-Bestand. Ihre Existenz im tatsächlich für Brain zugänglichen Infisical-Projekt/Pfad folgt nicht aus dem gleichen Namen. Die Vorbereitung lässt diesen Zugang zu Recht offen. `brain-serve/src/secrets.rs:41-75` verlangt vorhandene Werte, unterscheidet Provider/DB/API und verwirft doppelte Werte. Actor, Channel, Scopes und Egress stammen aus dem festen Grant, nicht aus der Frage; `brain-policy/src/lib.rs:91-100,128-145` authentifiziert und verengt angefragte Scopes. Public-Scopes müssen zu echten öffentlichen Quelldaten passen; der Name eines Scopes macht Daten nicht öffentlich.

Keine Secretwerte, produktive Config, Credentialdateien oder Umgebungen gelesen. Keine neue Secretzuordnung und kein Produktmodell ausgewählt.

### 3. Unit und Artefakt: Vorlage korrekt als Vorlage behandelt

`service/systemd/brain-serve.service.example:1-24` enthält Userunit, Secret-Exec, LoadCredential, SIGTERM, Kontrollgruppenbeendigung, Restartbegrenzung und 20 Sekunden Stopfrist. Die Vorbereitung verlangt zutreffend, dass die Stopfrist über der konkreten Shutdownfrist liegt. Absolute revisionsgebundene Pfade und wirkliche Credential-/Socketrechte müssen beim tatsächlichen Dienstbenutzer geprüft werden; die Vorlage belegt diese nicht.

Die berichteten installierten Units, PIDs, fehlerhaften/inaktiven Jobs und der weiter vorhandene Python-Siteprozess wurden nicht live nachgeprüft. Die Vorbereitung schränkt ihre Aussage auf abgefragte Namensfamilien ein und verwechselt weder `dl-knowledge` noch den Siteprozess mit typed Brain-Serve. Der vorhandene Harnesscache wird nicht als produktives Release akzeptiert. Keine Unitinstallation, Restartaktion oder fremde Consumerumschaltung vorgeschlagen oder ausgeführt.

### 4. Startup und Readiness: notwendiger Check, kein Freigabe- oder Antwortbeweis

`service.rs:129-169` prüft Schema, DB-Zugriff, konkreten Snapshot, Knowledgeversion und gegebenenfalls Analytics-Patch. `health.rs:55-83` liest den Release erneut, vergleicht den vollständigen Releasewert, validiert den Snapshot und DB-Rechte und verweigert während Drain. `CorpusSnapshot::authorized` in `brain-contracts/src/store.rs:108-158` prüft Vollständigkeit, gepinnte Revisionen und aktuelle Heads und filtert gesperrte oder unzulässige Datensätze. Die getrennten fachlichen und negativen Proben in der Vorbereitung sind deshalb erforderlich.

Zwei konkrete Grenzen für die spätere Ausführung:

- „Freigegebener Snapshot“ in der Vorbereitung bezeichnet eine zusätzlich nachzuweisende Betreiber-/Datenfreigabe. Der Code liest `corpus_releases_v1` und prüft Identität und Struktur (`brain-storage/src/local_pg_reader.rs:462-480`, `memory_repository.rs:241-264`), aber keinen externen Freigabebeschluss. Ein grünes `/readyz` beweist weder Quellenlizenz noch Modellfreigabe noch eine Antwort für die Consumer-Scopes. Die Readinessantwort enthält auch keinen vollständigen Releasebeleg; die richtige Releasebindung muss separat nachgewiesen werden.
- Serve ist fachlich kein neuer Ingestwriter, aber nicht vollständig schreibfrei: `local_pg_reader.rs:406-422` verlangt SELECT und INSERT auf `brain.conversation_owners_v1`; `:515-538` registriert Conversation/Actor per INSERT und prüft den Besitzer. `brain-policy/src/lib.rs:148-151` nutzt diesen Pfad. Ein reiner DB-Read-only-Account reicht nicht. Ein späterer Canary muss diese begrenzte persistente Wirkung berücksichtigen, ohne fachliche Writerrechte zu vergeben. Auch der tatsächliche Twitch-Handler hat nachgelagerte Protokollschreibpfade (`self_explainer.rs:889-899`); „keine Chatnachricht“ bedeutet nicht „keine DB-Wirkung“.

Das sind Präzisierungen der offenen Rechte-/Liveprüfung, keine in der Vorbereitung behaupteten erfolgreichen Nachweise. Es wird kein beobachteter Start- oder Testfehler erfunden.

### 5. Rückweg und Writer-Fencing: zutreffend vom Serveanschluss getrennt

`infra/cutover/README.md:53-67` verlangt ein kompatibles unveränderliches Rückfallrelease, Backup-/Restorebeleg und aktuelle Policy-/Tombstonezustände. `:79-95` verlangt Sperre alter Startwege, Drain und serverseitig wirksames Fencing einschließlich verzögerter alter Jobs. `:97-111` verbietet ein blindes Zurückkopieren veralteter Daten oder Rechtezustände. Genau diese Grenzen übernimmt die Vorbereitung. Ein inaktiver Job oder gestoppter Timer belegt keine gesperrte Writergeneration.

`RELEASE_MANIFEST.yaml:3-45` bleibt ein Entwurf mit fehlenden Werten und `approved: false`. Der statische Plan erfüllt diese Felder nicht. Erster Serveanschluss, späterer fachlicher Writerwechsel und vollständiger G5-Abschluss werden sinnvoll getrennt. Die bedingte Nutzerfreigabe ersetzt den historischen prepare-only-Auftragsstatus, nicht die fehlenden technischen Belege. Die spätere ausdrückliche Kennzeichnung der Replayfelder als zurückgestellt darf keine Wiki-/Provider-/Datenparitätslücken mit erledigen. Kein Abschalten alter Jobs für einen bloßen API-Test.

## 3. Konkrete fehlende Voraussetzungen vor Start beziehungsweise Cutover

1. **Nicht geheime Betriebsbindung:** abgestimmter konfliktfreier Loopbackendpoint und passende Consumer-Basis-URL, tatsächlicher Dienstname/Benutzer, absolute Config-/Releasepfade, kompatible Client-/Servefristen. Die beobachteten Ports 8787 und 8789 sind nicht verfügbar erklärt und werden nicht umgewidmet. Aktive Consumerconfig und deployter Consumerstand bleiben unbewiesen.
2. **DB, Corpus und Rechte:** dedizierte tatsächliche Brain-Instanz mit Schema-/Storeversion, Socket/Port/Rolle, Quellenleserechten und begrenztem Conversation-Ownership-INSERT; freigegebenes gepinntes CorpusRelease mit passender Knowledgeversion, fachlicher Evidenz sowie aktueller ACL-/Tombstone-/Quellenrechtsbindung. Ein vorhandener PostgreSQL-Prozess ist dafür kein Beleg.
3. **Bestehender Secret-/Providerweg:** Zugriff des Dienstbenutzers auf den vorgesehenen Infisical-Socket und Runtime-Credential, vorhandene passende Secretreferenzen einschließlich der Consumeridentität, genehmigter Provider-/Modellpfad, tatsächliche Preise und Budgets. Shadow verdoppelt potenziell Providerarbeit und benötigt dafür ebenfalls ein konkretes Budgetfenster.
4. **Revisionsgebundenes Artefakt und freigegebene Prüfung:** gemergter freigegebener SHA mit Binary-/Lock-/Toolchainnachweis, konkrete Unit und Ressourcen-/Stopgrenzen, gesonderte Zuteilung der nötigen Build-, DB-, Prozess- und Liveprüfungen. Listener/Readiness plus fachlicher typed Clientbeweis und Negativfälle müssen auf dem tatsächlich vorgesehenen Consumerpfad gemessen werden, ohne Discord-/Twitch-Nachricht. Die noch parallele Test-Safety-Abgabe ist nicht hiermit abgenommen.
5. **Kompatibler Rückweg und Gesamtbetrieb:** gefülltes revisionsgebundenes Release-/Rollbackmanifest, Restore auf leerem Ziel und kompatibler Rückfallstand ohne Zurückdrehen aktueller Rechte oder Deletes. Bei Writerwechsel zusätzlich Generation/Fencing, Drain und finale Offsets/Deltaübernahme. Fehlgeschlagene Jobs, aktive Timer, Wiki-/Providerparität und Python-/Altbetrieb bleiben gesonderte Gesamtabschlussarbeit.

Diese Punkte sind Start-/Cutoverstopps, keine neu erfundenen Produktarchitekturen und keine bereits erledigten Nachweise. Der vorbereitende Text benennt die Kategorien ehrlich; die präzisen Quellstellen oben konkretisieren die später zu belegenden Werte.

## 4. Ressourcen- und Beweisgrenzen

Kein Cargo, Compiler, Formatchecker, Clippy, Test, Fetch, DB-Zugriff, Prozessharness, Modellaufruf, Dienststart, Restart oder Deployment durch den Reviewer. Keine Unterthreads, neue Caches, ENV-Konfiguration, Secretlesung, Produktänderung oder Änderung fremder Worktrees. Geschrieben wird dieser Bericht im eigenen Reviewbaum.

Die inzwischen bestandenen Metadaten-, Format- und Clippyprüfungen werden aus dem Briefing als berichteter Zwischenstand übernommen, nicht erneut ausgeführt oder als eigene Tests gezählt. Sie belegen weder produktive Config noch DB-/Serve-Livebetrieb. Die historischen Workspace-Testzahlen werden nicht auf diese Vorbereitung übertragen.

**Abschluss: statisches GO für SERVE-VORBEREITUNG.md in der oben gehashten Fassung. Kein blockierender Abgabemangel; kein Bump-up erforderlich. Start, Consumeraktivierung und G5 bleiben mangels der aufgeführten konkreten Belege nicht abgenommen.**

Nächster Freigabepunkt: Der Intent ergänzt die vorhandene Build-/Deployanfrage zunächst um die belegten nicht geheimen Endpoint-, Unit- und Configbindungen. Kein Startbefehl aus diesem Bericht.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft (keine Liveaufrufe; vorhandene Secret-/Consumerverträge statisch gelesen)
TESTNACHWEIS[TW-1]: 0 passed, 0 ignored | Baseline: kein Testlauf beauftragt oder ausgeführt; keine Aussage über Tests auf diesem Stand
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: unabhängiger statischer Servevorbereitungsbericht
