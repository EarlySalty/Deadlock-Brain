status: aktiv
Datum: 2026-10-03

# Paket K: geschützter Bots-Bereich

Stand 14:14 UTC: Bots PR #459 ist weiterhin offen und Draft, Remote-Kopf e805fbed60a9176984538f2edaa3960209c589ab. Der fremde Thread c0b1d111-e402-4ed3-bb0b-68958a1ba699 läuft und integriert einen gekoppelten Consumer-/Gruppenstand. Deshalb keine Schreibarbeit oder Deploy im Bots-Consumerbereich. Thread a99dc9e9-3ce1-41bc-ba6b-391cc190f518 steht auf ready; seine letzte Meldung nennt noch fehlende gemeinsame Abnahme.

2nd-Brain, Docs und Twitch laufen unabhängig weiter. Drei native Bestandsworker sind gestartet. Zusätzlich zum alten PR #2 liegt bereits Sol-Arbeit auf sol/c9-second/7bf0e0375ee34a00, Kopf afc30f059d3ce4eab9fa402d3c08e505e5dff923; diese wird vor jedem Neubau geprüft.

## Schnittstelle zu Q und Z: serverseitiger Anfragebeleg fehlt

Der laufende brain-serve-Prozess ist PID 3506677 aus maintenance-releases/511a347b653beba13c2bf130f4bead7a7196cc2a. In den gelesenen Q-Quellen erzeugen rust/crates/brain-api/src/http.rs und lib.rs keine Anfrageereignisse; brain-serve/src/service.rs protokolliert Start, Ende und Poolstatistik. Das Journal enthält in der letzten Stunde keine Anfragezeilen. Das Beweisziel aus BRIEFING-K.md verlangt ausdrücklich eine serverseitige Antwortzeile mit Consumer-Kennung.

Benötigte Q-/Z-Schnittstelle: ein redigiertes serverseitiges Anfrageereignis nach authentifizierter Anfrage, mit Consumer-Kennung aus dem Credential-Grant, Request-ID, Route und Ergebnisstatus. Keine Anfrage-/Antworttexte, Header, Credentials oder personenbezogenen Kontodaten protokollieren. K ändert den fremden Kernbereich nicht. Bitte diese Beweismöglichkeit im Kern beziehungsweise im bestehenden serverseitigen Auditweg bereitstellen und den Pfad nennen. Consumer-Bau kann währenddessen weiterlaufen.

Zusätzlich ist der vorhandene aktualisierte 2nd-Brain-Adapter an Brain-Revision 32f603219ad896182693dd85639ac19793272042 gebunden. Diese liegt lokal vor, ist aber nicht in origin/main beziehungsweise im Q-Kopf 511a347b653beba13c2bf130f4bead7a7196cc2a. Sie enthält bereits den privaten Operatortransport. Der aktualisierte Adapter verlangt diesen Unixsocket; die aktuelle produktive Kernrevision bietet ihn nicht. K kann diese fremde Kernarbeit nicht selbst übernehmen. Der private Operatorvertrag muss vor dem gemeinsamen Live-Beweis in die Kernintegration von Q/Z einfließen; den alten internen TCP-Adapter wiederzuverwenden würde die vorhandenen Sicherheitskorrekturen verlieren.

## Consumer-Voraussetzungen im Live-Kern

Der wirksame Kernstand hat laut Bestandsprobe nur twitch-bot mit bot.public freigegeben. Für die übernommenen sicheren Adapter fehlen produktiv:

- Second-Brain: dediziertes vorhandenes Infisical-Secret BRAIN_SERVE_SECOND_BRAIN_TOKEN, serverseitiger Principal und second_brain.internal am privaten Operatorweg.
- Docs: BRAIN_SERVE_DOCS_PUBLIC_TOKEN, serverseitig docs-client im Kanal docs mit docs.public sowie ein freigegebener öffentlicher Docs-Wissensrelease mit auflösbaren Belegen. Der Docs-Client darf den Operatorweg nicht nutzen.

Bitte diese Freigaben beziehungsweise den vorhandenen zentralen Einrichtungsweg mit der Kernintegration in Q/Z binden. K legt keine Secrets außerhalb Infisical an und ersetzt eine fehlende Freigabe nicht durch Twitch-Credentials. Den Status der Secretwerte prüft der jeweilige sichere Starter, ohne sie auszugeben.

## Fremde Git-Stände außerhalb von K

Der Stop-Hook nennt zusätzlich Deadlock-Docs main mit zwölf ungepushten Commits und Brain feat/brain-rust-cutover-20260919. Das ist keine eigene Arbeit dieses Pakets. Docs main steht auf 67bb24706fa37fbdccb6b99845f06c45d1448d0e und weicht mit 64 fehlenden Remote-Commits und zwölf lokalen Commits von origin/main 4b072aee3127564def674f4b53d9be5d1d42cfdf ab. K verändert diesen fremden Checkout und Branch nicht. Eigene Docs-Abnahme und Integration laufen ausdrücklich gegen frisch gefetchtes origin/main im eigenen Worktree, kein Push aus dem fremden main-Checkout. Den fremden Brain-Legacybranch übernimmt K ebenfalls nicht.

## Stand 15:30 UTC: Twitch-Verwaltung und wiederaufgenommene Prüfungen

Die zwei Bauworkflows wurden nach der Harness-Unterbrechung mit erhaltenen Ständen wiederaufgenommen: Second-Brain wf_0f6257af-761 mit Worker a537d9d3af7afa41c, Docs wf_b4676efe-964 mit afa5afbcc90e1207c. Noch kein Abschlussrecord, kein Prüferfolg behauptet. Bots-Thread c0b1d111-e402-4ed3-bb0b-68958a1ba699 bleibt running. Er meldet grüne Nachsuiten, aber noch kein neues Gateurteil; K übernimmt ihn nicht.

Twitch ist bereits auf dem Consumer-Release 9a6356a679e05d0868f1178cf64a79c99b3e5105 installiert. Die tatsächlichen Brain-Modi in /var/lib/deadlock-twitch/config/bot.toml und /proc der beiden Dienste sind für diese Sitzung nicht lesbar. Bestehende Verwaltungswege wurden geprüft: MCP hat keine Brain-Konfigurationswerkzeuge; der Dashboard-Betriebseditor erlaubt nur drei Poolfelder und lehnt fremde Felder ab; tb-config-check prüft die TOML, zeigt die Brain-Modi aber nicht. Der vorhandene Release-Wrapper bietet keine passende Konfigurationsumschaltung. Der In-App-Browser hat keinen verfügbaren Automationshost.

Benötigt wird ein zulässiger enger Verwaltungsweg für bot.brain_client und dashboard.options.brain_client zum Lesen und Aktivieren gegen 127.0.0.1:8788. Kein generisches sudo, keine neue Diagnose-API als Umweg, keine Änderung fremder Konten. Außerdem fehlt ein verifiziertes Twitch-Testkonto für den echten Chatbeweis. Ein Dashboard-Testaufruf erzeugt Datenbank- und Discord-Logging-Nebenwirkungen; er wird nicht als wirkungsfrei dargestellt. Details in SCOUT-twitch.md.

Für Docs und Second-Brain sind die sicheren CLI-Verträge vorhanden. Ein dauerhafter installierter CLI-Ausführungsweg mit normaler TOML und vertrauenswürdigem Infisical-Bootstrap fehlt noch. Der vorhandene Docs-Korpusdeploy installiert keinen Query-Adapter. Empfehlung: die bestehenden Release-/Credential-Wege um die on-demand Rust-CLIs ergänzen, ohne neue Antwortdaemons; den konkreten zulässigen Pfad bitte im Betriebsvertrag von Q/Z festhalten.

## Übernahme bestätigt und enger Twitch-Entwurf

UEBERNAHME-CODEX.md und VON_HAUPT.md bis 15:46 UTC sind gelesen. Gekoppelte Integration und Produktivwechsel bleiben bei Z. Kein selbstständiger Einzelmerge oder globaler Chat-Cutover in K. Docs ist auf 3e570a8aa0bf867bf1baf35b064165804b77fcb4 mit 22 bestandenen Tests, Format und Clippy geprüft; frische Intent-Abnahme aaa499d9cdfb642d3 läuft. Second-Brain steht nach Dokumentationsabschluss auf 54979646adde835335fa24ddd2545e10f996df52; eigener Compilerprüflauf b57u6j5pw bleibt mechanisch unter beiden Hostlocks eingeordnet.

Der konkrete Twitch-Entwurf und das enge Dateieigentum stehen in ENTWURF-TWITCH-CONFIG.md: vorhandenen Rust-tb-config-check und editor erweitern, ausschließlich die fünf freigegebenen Modus-/Endpoint-/Enabled-Felder der beiden Brain-Consumer. Inspektion redigiert, Änderung mit altem Config-Hash unter bestehender Dateisperre atomar, Rechte und Eigentümer erhalten. Schreibziel ausschließlich /var/lib/deadlock-twitch/config/bot.toml, keine neue API oder erhöhte Rechte in der CLI. Eigentum: tb-config-check.rs, editor.rs, betroffene tb-config-Tests und kurze Betriebsdokumentation. Keine Wrapperänderung in diesem Bauschritt. Installation und Ausführung brauchen nach gemeinsamer Security-Abnahme den vorhandenen zulässigen privilegierten Verwaltungsweg; konkreten Launcher übernimmt die Integration, nicht ein generisches sudo aus K.

Die noch vor Lesen der neuen Entscheidung durchgeführte authentifizierte GET-Probe auf http://127.0.0.1:8769/twitch/api/admin/config/operating war erfolgreich. Beide Dienste melden denselben aktiven und gespeicherten Fingerprint a5ca906012a5c8cb5637cb560ae0011b1c47320ca6a496f679e9f319db13f55f. Der bestehende Editor liefert nur Poolfelder und keinen Brainzustand. Keine Configmutation, kein Testchat, kein Discord-Aufruf. Benanntes TWITCH_INTERNAL_API_TOKEN einmalig über den geprüften lokalen Infisical-Transport im Speicher verwendet; kein Wert ausgegeben oder gespeichert. Die abgeschlossene Bestandssuche wird nicht fortgesetzt.

Nächster Schritt: enge Rust-Konfigurationsergänzung bauen lassen, Docs-Intent-Ergebnis und Second-Brain-Prüfergebnis verarbeiten, dann Gatebelege für die gemeinsame Integration liefern.

## Stand 16:55 UTC: Twitch-Bau und Botsübernahme

Twitch hat einen sauberen eigenen Baukopf ffa037c885ca45c0b7a43759ed6714305ed92f99. Genau die angekündigten Dateien wurden verändert: tb-config-check.rs, editor.rs, tests/editor.rs und rust/docs/brain-config-cli.md. 82 Tests bestanden, keine ignoriert; Format und ausgewähltes Clippy Exit 0. Vollständiges Clippy meldet Exit 101 wegen field_reassign_with_default im unveränderten bestehenden Testaufbau dashboard_options.rs:363. Das wird nicht als grüne Gesamtprüfung ausgegeben. Originalbelege: /tmp/tb-config-final.8QvC37.

Vor dem Nachzug zusätzlich angekündigter Schreibpfad: rust/crates/tb-config/src/dashboard_options.rs, ausschließlich die vorhandene Testinitialisierung zur Behebung dieses Clippyfehlers. Keine produktiven Configregeln oder weiteren Felder ändern, keine Lint-Unterdrückung. Derselbe vorhandene native Bauworker zieht diesen begrenzten Prüffehler nach und führt danach vollständiges Clippy sowie betroffene Tests unter beiden Hostlocks aus. Keine zusätzliche Reviewrolle; gemeinsame Bug- und Security-Abnahme bleibt bei Z und dem Merge-Gate.

Der vorhandene Releaseinstaller verteilt tb-config-check noch nicht. Z benötigt dessen SHA-gebundenen Bau und den bestehenden zulässigen privilegierten Verwaltungsweg zur Installation und zum eng begrenzten Configaufruf. K beschafft keine Rechte und aktiviert keine globalen Chatmodi. Inspektion gibt nur Hash und erlaubte Brainfelder aus; Schreiben ist auf /var/lib/deadlock-twitch/config/bot.toml festgelegt und erfordert erwarteten alten Hash.

Bots-Schutzstatus vor Übernahme erneut ausschließlich lesend geprüft: a99dc9e9-3ce1-41bc-ba6b-391cc190f518 stopped, c0b1d111-e402-4ed3-bb0b-68958a1ba699 ready. SCOUT-bots.md hält den sicheren elfteiligen Consumerbereich bis e3e649ccc13193c9dee16cca7651cabb26f25ab3 fest. Eigener Worker übernimmt diesen in den vorgesehenen eigenen Worktree von frischem origin/main; aktuelle Launcherkorrekturen bleiben erhalten. Keine Privacy-/Community-Migrationen, keine fremden Resolverpfade, keine Draftumschaltung oder eigene gekoppelte Installation.

Beide CLI-Intent-Abnahmen und tatsächlich ausgeführten Prüfungen sind abgeschlossen. Die Teilübergabe mit den vollen Docs-/Second-Brain-SHAs liegt in UEBERGABE.md. Der lesende Betriebsvertragsworkflow ist ebenfalls beendet; konkrete Installation und Anfrage erst nach Zs gemeinsam geprüfter Freigabe. Kein Testchat oder Discord-Aufruf wurde ausgeführt.

## Stand 17:36 UTC: konkreter Twitch-Senderweg und Beweisgrenze

BETRIEBSVERTRAG-CLI.md und BETRIEBSVERTRAG-TWITCH.md liefern die konkreten vorhandenen Binary-/FD5-/TOML-/Hashverträge. Twitch-Inspektion liefert den tatsächlichen Datei-SHA256, nicht den früheren semantischen Dashboardfingerprint. Noch keine Installation oder Configmutation. Twitch-Nachprüfworker und Bots-Bauworkflow laufen weiter; keine neue vollständige Grünmeldung erhalten.

Der rein lesende Sendervertragsworkflow ist beendet, Ergebnis in SENDERWEG-TWITCH.md. 18 Auth-/Helix-/Chatdateien gegen Blobs am beobachteten 84ce376c9ea7aab69116fc7399183032308d712e abgeglichen; dieser Quellenstand ist kein abschließender eigener Bauprüfbeleg. Vorhandene BotTokenManager-/Helixwege können IDs und echte Scopes prüfen. Gespeicherte Streamercredentials werden durch den bestehenden TokenProvider unterstützt; user:write:chat steckt im Uplink-Profil, nicht in normalen Basis-/Dashboard-/Titelprofilen. Keine Secrets oder Laufzeitrequests in dieser Untersuchung.

Konkreter verbleibender Senderrest: ein vorhandener erlaubter Nicht-Bot-Sender mit tatsächlicher validierter User-ID und user:write:chat sowie eine geeignete vorhandene Senderschnittstelle. ChatApi und Owner-Adminchat senden selbst als Bot und eignen sich nicht als Testfragensteller. Kanalzulassung und Chatabonnement müssen ebenfalls nach Installation belegt sein. Keine fehlenden Credentials aus der earlysalty-Freigabe herleiten, kein Senderneubau oder interaktiver credentialhaltiger Plattformtoken-Ersatzweg aus K.

Konkrete Grenze des geforderten Quellen-/Replybelegs: ursprüngliche EventSub-Message-ID wird als Brain-ID und Auditschlüssel übernommen; Frage und Antwort werden vor Versand gespeichert, der Zustellstatus danach. SendOutcome::Sent hält die zurückgegebene Twitch-Reply-ID nicht; der vorhandene OBS-Ereignisweg kann Parent-/Replyreferenzen speichern, seine tatsächliche Laufzeitverdrahtung ist noch zu bestätigen. Die Chatbrücke verwirft außerdem Citationlabels und entfernt Links. public.tb_chat_brain_answers allein belegt daher weder die tatsächliche Reply-ID noch eine auflösbare unterstützende Quelle. Gemeinsame fachliche Abnahme braucht diese zusätzlichen echten Belege an der Antwort. Das ist eine Quellen-/Betriebsvertragsgrenze, kein neues eigenes Bug-/Securityreview. K patcht weder fremden Kern noch Loggingdienst und behauptet keinen bisher fehlenden Beweis.

Die Nutzerfreigabe bleibt eng: erst nach gemeinsam geprüfter Installation eine sachliche echte Erwähnung in earlysalty über vorhandenen bestätigten Sender, anschließend Originaleingang, korrelierte Kernzeile, fachliche belegte Antwort und beobachteten Reply. Kein Chat, keine Moderation und keine Discord-Nebenwirkung bisher.

## Stand 18:50 UTC: tatsächliche Abschlüsse und verbleibender Botsblocker

Twitch-Nachzug erfolgreich abgeschlossen auf 84ce376c9ea7aab69116fc7399183032308d712e: 82 Tests, 0 ignoriert, Format und vollständiges Clippy --all-targets -D warnings Exit 0. Genau erlaubter Testinitialisierer nachgezogen, keine produktive Validierungsänderung. BAU-TWITCH.md und /tmp/tb-config-nachzug.VZVfSv/ binden die echten Nachprüfungen an den neuen SHA. Frische unabhängige Intent-Abnahme wf_4941a30e-893 / w6sk0xc67 gestartet, rein lesend, kein Bug-/Securityreview. Z behält gemeinsame Integration, Gate, Installation und Aktivierung.

Bots-Workflow wf_b5147463-f82 ist tatsächlich blockiert beendet, kein laufender Prüfer. Elf sichere Commits konfliktfrei übernommen auf a96de1d5c00d1771fafc351e2046922d00ce045d, neuere Launcherfixes erhalten. Alle zehn Checks Exit 127 wegen fehlendem Rustup im bereinigten PATH; Wrapper Exit 1. Keine Compiler oder Tests gelaufen. Drei eigene ungeprüfte Korrekturen bleiben uncommittiert erhalten: docs/BRAIN_API_ADAPTER.md, rust/crates/dl-brain/src/brain_api.rs und rust/scripts/check-brain-consumer.sh. PATHkorrektur vorhanden, nicht nachgeprüft. Abschließender Schutzrecheck meldet c0b1d111-e402-4ed3-bb0b-68958a1ba699 running; Sourcewrites und Commits pausiert. Keine fremden Threads oder Worktrees verändert. Vollständige gelesene Belege und Betriebsvertrag in BAU-BOTS.md, local-status.json und reuse-provenance.json. Neuer Schutzstatus wird nur lesend geprüft.

Bots verwendet im vorhandenen Vertrag ebenfalls twitch-bot/twitch und TWITCH_INTERNAL_API_TOKEN über FD3/Infisical. Keine separate Discordkennung behaupten. Der tatsächliche eigene Clientaufruf muss zusammen mit dem vollständigen Request-ID-Hash gegen Qs Kernereignis korreliert werden; der gemeinsam verwendete Principal allein unterscheidet diese zwei Consumer nicht. Clienthashergänzung noch ungeprüft und uncommittiert.

Die frühe Senderprüfung wurde beendet, ohne tatsächlichen Authrequest: platform-token und uplink/me können automatisch refreshen und DB schreiben, daher für den rein lesenden Auftrag nicht aufgerufen. earlysalty-ID und user:write:chat weiterhin unbekannt, nicht als fehlend bestätigt. Vorhandene Adminroute GET /twitch/api/admin/system/oauth-scopes gefunden und Handler gelesen; Loader und zusätzliche stabile IDbindung noch zu bestätigen, Route noch nicht genutzt. Keine Credentialausgabe, OAuthänderung oder Chatnachricht.

Tatsächliche Unterauftragswache: finale Workerabschlüsse ersetzen die alten Supervisor-/Fortschrittsbeobachtungen. Ein kombinierter rein lesender Node-Prozess-/FD-Metadatenaufruf wurde zuvor vor Ausführung mit „Git-Push blockiert: indirekte Shell-Ausführung mit möglichem Git-Push ist nicht prüfbar.“ abgewiesen. Ursache der Hookheuristik nicht verifiziert. Abgelehnten kombinierten Scanner nicht über einen anderen Werkzeugweg wiederholt, keine Schutzregeln geändert. Erlaubte eigene Logmetadaten und konventionelles ps wurden getrennt ausgewertet; vollständige FD-/Lockprozesszuordnung gelang nicht. Keine fremden Prozesse gestoppt.

Status 14 mit echter Zeit 18:50:29 UTC veröffentlicht. Der Abstand zu Status 13 war länger als 20 Minuten; keine rückwirkende Zeitangabe erfunden. Docs und Second-Brain bleiben fertig teilübergeben; Gesamtes Paket noch aktiv, keine eigene gekoppelte Installation oder Livebehauptung.

## Haltepunkt auf Nutzerwunsch umgesetzt

HALT.md und aktualisierte VON_HAUPT.md gelesen. Keine neuen Läufe gestartet. Eigenen Twitch-Intentworkflow über TaskStop beendet; übrige eigene Worker bereits beendet, CronList ohne geplante Aufgaben. Drei Botskorrekturen als wip: committiert, neuer HEAD 31fdab311c014873ae3080d5f4f8f5e36b991665. Alle vier eigenen Featurebranches nach origin gepusht, Twitch-Remote-SHA zusätzlich bestätigt; versionierte Arbeitsbäume sauber. STAND.md mit genau 15 Zeilen angelegt. Bestehende Akten und lokale Prüfberichte erhalten. Kein Merge, Deploy, Configwechsel oder Settlen; Turn endet, Hauptsession übernimmt.
