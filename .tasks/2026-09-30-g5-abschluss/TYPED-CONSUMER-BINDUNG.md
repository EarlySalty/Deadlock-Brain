status: aktiv, konkrete Bindungen und nächster Prüfbedarf; kein Start oder Deploy
Datum: 2026-09-30

# Typed-Anschluss: geprüfter Consumer und vorhandener Serve

## Fest gebundene Quellstände

Brainprodukt `9a29b81d230c01e5c03423cc34ba34c1074eab69`; Quellhead `ca4a8f236a75ba89ce60d2140c735acd5d1f5bee` enthält zusätzlich nur den statischen Testbericht. Standardtest Exit 0, 954 passed, 74 ignored. Diese Zählung ist keine Abnahme der ignorierten Fälle.

Maßgeblicher Twitch-Consumer ist jetzt `/home/nathanael/.worktrees/twitch-chat-brain-live-20260930`, HEAD `fdd7a5d2de07e718dcf5509a2d12e3db6770d276`, Basis `9f6f291de56b9304f95c3d8d1566c601ecd85de9`. CHAT-BRAIN-HANDOVER.txt gelesen. Fünf Eigencommits des älteren be402985 wurden vom Integrator isoliert übernommen, dazu enge Formatkorrektur. Keine fehlende Chat-Komposition aus der alten d828481-Momentaufnahme ableiten. Consumer-Merge, Config, Migration und Deploy gehören dem Twitch-Integrator; Brain liefert den bestehenden G5-Servepfad.

## Statische gemeinsame Kompatibilität

- `tb-knowledge/Cargo.toml:7` und Cargo.lock des integrierten Consumers pinnen `brain-client` weiterhin auf `3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef`.
- Brain-Gitdiff zwischen diesem Pin und 9a29b81: kompletter brain-client und `brain-contracts/src/public_api.rs` unverändert. In contracts/lib.rs kamen nur zwei Moduldeklarationen hinzu; Query, AnswerStatus und Versionen wurden nicht geändert. Öffentliche Antwortversion `brain.public.v1`, interner Vertrag `brain.v1`.
- Die API-Differenz ist kein neues Antwortformat: Bei Policy-State-Ausfall wird jetzt der bereits bestehende typed Status Unavailable statt eines 503-Fehlerumschlags zurückgegeben. `tb-knowledge/src/brain.rs:121-132` behandelt Unavailable bereits als Backendfehler. Keine Freigabe aus einer fälschlich als beantwortet gewerteten Überlastantwort.
- Adapter, Chat-Wiring und BrainChatOptions im integrierten fdd7a5d2 sind gegenüber be402985 unverändert. `main.rs` übergibt `settings.internal_api.token`; `chat_wiring.rs` setzt diesen in den vorhandenen BrainChatBuild ein. Kein neuer Schlüssel und kein Modellpfad.
- Der Client verwendet einen expliziten lokalen Basisendpoint, hängt `/v1/answer` an, übergibt Authorization, akzeptiert keine Redirects und keine Proxys. Kein Legacyfallback. Der Chat-Port entsteht ausschließlich mit enabled=true und mode=typed; Shadow/Legacy sind keine Aktivierung dieses Ports.
- Query: vorhandene öffentliche Scopes, Explain-Profil, Request-ID der Nachricht, Conversation-ID aus Kanal-ID und Twitch-User-ID. Der Adapter ist stateless. Auf Serve-Seite kommt die Actor-/Channel-/Egressbindung aus dem bestehenden Credentialgrant, nicht aus frei übergebenen Anfragefeldern.

Das ist ein belegter **statischer** Vertrag. Der vorhandene gepinnte Client muss später tatsächlich gegen den gemergten Serve laufen. Mocktests und ein identischer Datentyp ersetzen diesen Durchstich nicht.

## Konkrete bekannte Betriebsbindungen

| Element | Bereits belegt | Nächster enger Nachweis |
| --- | --- | --- |
| PostgreSQL | Dedizierte Systemunit `deadlock-brain-postgresql.service`, OS-User/Gruppe `deadlock-brain-pg`; Socket `/run/deadlock-brain-postgresql/.s.PGSQL.5446` | Schema, Rollen und Releasebestand lesend auf dieser Instanz prüfen, kein Neustart |
| Datenbank | Versionierter Betriebsvertrag nennt Ziel-DB `brain`, Owner `brain_migrate`; `brain_pilot` und `brain_pilot_legacy` sind Prüfbestände | Keine Pilot-DB als fertigen Produktivbestand verwenden |
| Serverrolle | Bestehende Rolle **brain_service**, SCRAM, vorhandene Referenz **BRAIN_PG_SERVICE_PASSWORD**, Deckel 16. `roles.sql:6-13`, `pg_hba.conf:3`, `grants.sql:19-23` geben Quellen-/Release-SELECT und Ownership-SELECT/INSERT | Tatsächliche Grants und funktionierenden Dienstzugriff prüfen. Keine neue Rolle brain_serve aus der Beispielconfig bauen und keine Peer-Zuordnung für brain_service erfinden |
| Prüferrolle | Bestehendes `brain_migrate` hat Peer-Mapping für nathanael laut `pg_ident.conf:1`; nur für geplante Metadatenprüfung in expliziter READ ONLY-Transaktion | Keine Migration oder Rechteänderung in der Leseprüfung |
| Serveunit | Bestehende Vorlage `service/systemd/brain-serve.service.example`; noch keine installierte typed Unit in den geprüften Namensfamilien | Vorlage an tatsächlichen Release-/Configpfad und Dienstkontext binden; Infisical-Socket muss darin UID 0 behalten. Kein blindes Abschalten der Sandbox |
| Endpoint | 8787 und 8789 sind belegt. Jüngste reine Socketaufnahme zeigt außerdem 8782/83/84/86 und 8790/91/92 belegt | 127.0.0.1:8788 ist lediglich ein momentan nicht lauschender Kandidat, keine Reservierung. Erst gegen die endgültige Consumerconfig gemeinsam fest binden; kein Produktionsport behauptet |
| Gemeinsamer Dienstzugang | Bestehender interner Twitch-Schlüssel, Referenz TWITCH_INTERNAL_API_TOKEN, wird vom Consumer intern übergeben | In Serve-Credentialgrant dieselbe vorhandene Referenz nutzen. Zugriff aus dem vorgesehenen Infisical-Projekt und Kontext ohne Ausgabe des Werts belegen |
| Public scopes | `bot.brain_client.public_scopes` ist ausdrücklich erforderlich, fehlende Liste wird abgewiesen. Adapter übernimmt die Liste | Tatsächliche Liste des Integrators mit veröffentlichtem CorpusRelease und Servegrant vergleichen. `bot.public` und `docs.public` aus Tests/Vorlagen sind kein Beleg für produktive Scopes |
| Zeitgrenzen | Consumer timeout_ms maximal 60000, vorhandener Fallback 8000. Serve hat getrennte Request-/Provider-/DB-/Shutdowngrenzen | Vorhandene konkrete Budgets konsistent binden, nicht für eine grüne Probe erhöhen |
| Modell/Provider | Serve hat bereits konfigurierten Providervertrag und bestehenden Secret-Exec-Pfad | Tatsächlichen bereits freigegebenen Endpoint/Modell-/Budgetstand belegen. Keine Auswahl aus der Beispielconfig und kein neuer kostenpflichtiger Pfad |

Die Rollen-/DB-Angaben sind an den versionierten Stagingvertrag BRAIN_POSTGRES_ISOLATION.md gebunden und gegen ops/brain-postgres verifiziert, noch nicht gegen die aktuellen Tabellen. Die live sichtbare Unit/Socketadresse ist davon getrennt. Das Lesen von `/etc/deadlock-brain/postgresql` scheiterte an Dateirechten; kein Ersatz-Leseweg oder Rechteumbau. Es wurden keine Secretwerte, Credentialdateien oder Prozessumgebungen gelesen.

## Unmittelbar nächste ausführbare Klasse: lesende Metadatenprüfung

**Anforderung, noch nicht zugeteilt:** eine einzelne bestehende psql-Verbindung über Peer als brain_migrate, Ziel ausschließlich `brain` auf Socket 5446. Kein Compiler, kein Testcluster, kein Modell, kein Dienststart, kein DDL/DML. Verbindung mit `-X -w`, expliziten Koordinaten und ON_ERROR_STOP; Transaktion READ ONLY mit Rollback. Keine DSN-/Passwortumgebung.

Zu lesen: current_database/current_user, Schema-/Storeversion, nicht geheime Rollenflags und Connectionlimits aus pg_roles, SELECT/INSERT-Rechte von brain_service auf den konkreten Kerntabellen, höchstens 20 Release-IDs mit knowledge_version/patch/created_at. Keine Passworthashes, keine Rohdokumente oder Nutzertexte, keine großen Tabellen-Counts. Ein leerer Releasebestand ist ein konkreter fehlender Ingestnachweis, kein Anlass für einen Dummyrelease.

Vorgesehener Aufrufrahmen: `/usr/bin/psql -X -w -h /run/deadlock-brain-postgresql -p 5446 -U brain_migrate -d brain -v ON_ERROR_STOP=1`; der vollständige begrenzte SELECT-Aufruf wird vor Zuteilung zentral dokumentiert. Das vorhandene Peer-Mapping ist eine Quellgrundlage, kein bereits erfolgreicher Login. Bei Zugriffssperre nicht auf DL-Main, Superuser oder Secrets ausweichen.

## Danach: vorhandene Harnesses und Release

1. Standardtest ist erledigt und dessen Slot zurückgegeben. Während der Twitch-Auslieferung keine konkurrierende Deployaktion.
2. Bestehender unabhängiger Reviewer gleicht historische G5-/PG-/Restore-/Pilotnachweise gegen 9a29b81 ab. Kein historisches ALLOW ungeprüft übertragen.
3. Vorhandene DB-Harnesses sind konkret geprüft: Sie erzeugen private Cluster und nutzen gezielte Testnamen, hardcoden aber einen zweiten Targetpfad und jobs=2. Nicht unverändert starten. Die notwendigen Testläufe müssen den vorhandenen Cache und einen Job verwenden; keine Budget-/max_connections-Erhöhung. Der historische `run_isolated_serve_checks.sh` ist ausdrücklich abgeschaltet, weil er die 5446-Systeminstanz stoppen würde.
4. Separater Releasebedarf bleibt exakt der bereits dokumentierte serielle Workspace-Releasebuild mit vorhandenem Targetcache, nach konkreter Integratorzuteilung. Ein erfolgreicher Build allein ist noch kein revisionsgebunden installiertes Artefakt.
5. Erst nach Rollen-/Corpus-/Providerbindung und tatsächlichen Harnessbelegen: lokaler Listener, Readiness, fachlicher typed Clientdurchstich sowie negative Scope-/Identitäts-/Evidenzfälle. Bestehenden gepinnten Consumer verwenden, nicht einen frisch gegen neuen Brainhead gebauten Ersatzclient.

## Rückweg und Nachweis ohne Chatversand

Die erste Serveaktivierung ersetzt noch keine fachlichen Writer. Für ihren Rückweg bleiben aktuelle Daten, Ownership, ACLs und Tombstones erhalten; Brain-Serve wieder deaktivieren und beim Integrator den Chat-Port deaktivieren, **kein Legacyconsumer als Rückfall**. Das ist ein Verfügbarkeitsrückweg, kein Beweis eines kompatiblen alten Brainreleases. Für einen späteren Writerwechsel bleiben unveränderliches kompatibles Rückfallartefakt, Backuphash, Restore auf leerem Ziel, Writer-Fencing und finale Deltas Pflicht.

Keinen gesunden laufenden Twitch-/DL-Dienst für eine Brainprobe verändern. Eine interne Clientprobe darf ausschließlich die freigegebene öffentliche Testfrage beantworten und eine Wegwerf-Conversation verwenden; Ownership-Schreibwirkung ausdrücklich berücksichtigen. Kein ChatApi-Aufruf, keine künstliche Mention und keine Communitynachricht. Natürliche spätere Laufzeitereignisse können den tatsächlichen Replyversand belegen. Bis dahin lautet die Aussage nur: interne Typed-Strecke geprüft, öffentlicher Replypfad nicht live belegt.
