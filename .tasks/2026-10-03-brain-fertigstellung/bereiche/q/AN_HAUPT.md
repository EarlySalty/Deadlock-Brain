status: aktiv
Datum: 2026-10-03

# Q: Übergabevertrag für Z, K und P

## Tatsächlicher Stand

Die Übernahme durch Codex /root und die gemeinsame Integrationsgrenze sind gelesen. Z integriert die lokal geprüften Stände, lässt denselben Integrationsstand unabhängig abnehmen und durch das Gate prüfen und installiert ihn gemeinsam. Q führt keinen Einzelmerge oder parallelen Releasewechsel aus. Der Live-Nachweis folgt nach dieser Installation.

Worktree: `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `5c220a8f047eb980d953d9f9f285b34739b5ed88`. Das ist der übernommene C9-Stand, noch kein Commit der zusätzlichen Provider- und Auditänderungen. Eigene Änderungen und Ergebnisse bleiben erhalten. Keine Worker neu gebaut, kein Modellwechsel.

Providerprüfung vom 03.10.2026, 16:04 UTC: 20 Tests bestanden, 0 fehlgeschlagen, 0 ignoriert; striktes Clippy mit Exit 0. Logs: `.q-provider-tests-fixed.log`, `.q-provider-clippy.log`. Die Baseline hatte 152 bestandene Tests und 15 ignorierte Tests.

Consumer-Nachprüfung beendet: sieben HTTP-Tests und 27 Librarytests bestanden, elf `brain-serve`-Librarytests fehlgeschlagen, null ignoriert. Die drei Testcode-Compilerfehler sind überwunden; Binarybuild und beide strikten Clippy-Schritte haben Exit 0. Elf Testfehler betreffen `docs_client_grant` beziehungsweise davon abhängige gültige Konfigurationen. Rustfmt scheitert ausschließlich am Importlayout in `brain-api/src/internal.rs`. Die tatsächliche Ursache wird eng korrigiert, ohne Grants oder Tests abzuschwächen. Bei der frischen Prozessprobe nach 18:30 UTC waren die bekannten Prüfer beendet und keine zum Q-Worktree gehörenden Compiler-/Sperrwarteprozesse vorhanden. Kein vollständig geprüfter neuer Consumercommit-SHA für Z. Q10 (`wf_fe5200ce-529`) führt ausschließlich diese konkrete Consumerkorrektur und ihre vollständige Prüfung fort. Q11 (`wf_97ccfc28-31c`) konsolidiert parallel nur die bisher unexportierten Retentiondateien und deren eigenen Harness. Keine Änderung von Storage-lib.rs, Migrationen oder Writerfreigabe während der Consumerprüfung. Die tatsächliche Wache belegt Werkzeugfortschritt in beiden Agenttranskripten um 18:44 UTC, ohne Unterbrechungsereignis.

Der frische Fetch bestätigt `origin/main=358ed4ee07d315d4d71dd0438f9446f5b6a22d49`, ohne den eigenen HEAD oder das WIP zu ändern. Direkte Nachrichten an laufende eigene Workflow-Agenten erzeugten zusätzliche lokale Agentkontexte. Diese zusätzlichen Kontexte wurden gezielt gestoppt; ursprüngliche Workflows und alle Quellen bleiben erhalten. Nachrichten in laufende Workflows werden nicht wiederholt.

Q7 und Q9 haben inzwischen ihre erhaltene Arbeit geliefert. Q7: Check und striktes Clippy für beide Crates samt Targets grün, 22 gezielte Tests bestanden, null ignoriert; frische YouTube-Metadatenprüfung und ausdrücklich bestätigter Leermengenvertrag vorhanden. `require_history_retention` bleibt aktiv, echte Quellen- und Datenbankwirkung ist nicht belegt. Q9: Runner gebaut, drei Tests bestanden, Offline-prepare erzeugt 103 eindeutige erlaubte Fragen. Der echte Vergleich bleibt gesperrt: Herkunft des vorhandenen Imports stimmt nicht mit den gepinnten Docs-Seiten überein; außerdem verlangt der Runner `bot.public`, während C9 `docs.public` liefert. Keine Herkunft passend geschrieben, kein Anbieteraufruf. G0 enthält Messdefinition und Rechteinventar, echte Consumerkennzahlen fehlen. Die 60 Anforderungszeilen bleiben 56 teilweise und 4 offen. Die Statusfrist wurde überschritten; die sitzungslokale Cronwache kann während eines laufenden Turns nicht feuern.

## Consumervertrag für Z und K

Der C9-Vertrag liegt in `docs/c9-consumer-contract.md`. Beide beauftragten Commits wurden übernommen: `e48c189` aus `6d5d903`, `5c220a8` aus `32f6032`. Vorhandene TCP-Endpunkte bleiben `/v1/answer` und `/v1/retrieve`. Der interne Endpunkt `/v1/operator/query` wird ausschließlich über den privaten Unixsocket angeboten und ruft keinen Modellanbieter auf.

Z setzt in der nicht geheimen `brain-serve`-Konfiguration:

- Docs-Grant: `actor_id="docs-client"`, `channel="docs"`, `scopes=["docs.public"]`, `provider_egress=["public"]`, ausdrücklich gepinntes `release={"id":...,"knowledge_version":...}`. Infisical-Credentialname laut Z: `BRAIN_SERVE_DOCS_PUBLIC_TOKEN`.
- Second-Brain-Grant: `actor_id="second-brain"`, `channel="internal"`, `scopes=["second_brain.internal"]`, `provider_egress=[]`, eigenes festes Release. Infisical-Credentialname laut Z: `BRAIN_SERVE_SECOND_BRAIN_TOKEN`. Genau ein interner Grant; sein Release muss mit `internal_operator.release` übereinstimmen.
- `internal_operator={"socket":"/home/nathanael/.local/state/deadlock-brain/operator/brain.sock","release":{"id":...,"knowledge_version":...}}`. Z hat diesen absoluten Pfad in `bereiche/z/BETRIEBSVERTRAG.md` festgelegt, noch nicht installiert. Privates Verzeichnis 0700 mit UID 1000, Socket 0600; Peer-UID-Prüfung. K kann den Client an diesen Pfad binden. Releasekennungen folgen aus den hashgeprüften Importmanifesten, nicht aus erfundenen Namen.

Die öffentliche Registry enthält den internen Grant nicht. Docs darf nicht auf das interne Release fallen. K bindet seine Clients an diese festen Grants und Releases; Z installiert Konfiguration, Socket und Infrastrukturzugang. Der Importpfad für die getrennten C9-Releases ist im übernommenen `brain-maintenance` enthalten und muss gemeinsam geprüft werden.

Für Zs betroffene Readinessprüfung liefert der interne Antwortvertrag `knowledge_release` als tatsächliche gebundene Release-ID, auch bei `insufficient_evidence`. Eine harmlose Testfrage mit `requested_scopes=["second_brain.internal"]`, Profil `explain` und ohne Domain kann den Kandidaten nach Restart prüfen. Im Prüfbericht ausschließlich Release-ID, Status, Zahl der Ausschnitte und Request-ID-Hash erfassen, keine Ausschnitttexte. Query und Client wählen weder Principal noch Release. Der öffentliche `/readyz`-Standardrelease allein belegt den internen Bindungswechsel nicht.

## Redigiertes Anfrageereignis für Z und K

Implementierung: `rust/crates/brain-api/src/audit.rs`, HTTP-Dispatchgrenzen in `http.rs` und `internal.rs`, enger Subscriber in `rust/crates/brain-serve/src/audit.rs`. Target `brain_api::redacted_request`, Ereignis `authenticated_request`.

Formatbeispiel, ausdrücklich noch kein Live-Log:

```json
{"event":"authenticated_request","consumer_id":"docs-client","request_id":"client-sha256:<64 lowercase hex>","route":"/v1/answer","status":200,"outcome":"insufficient_evidence","duration_ms":12}
```

Feste Routen: `/v1/answer`, `/v1/retrieve`, `/v1/operator/query`. Client-Request-IDs werden vollständig gehasht, frühe Fehler bekommen `server-sha256:<hash>`. Bekannte Grant-Paare heißen `twitch-bot`, `docs-client` und `second-brain`; andere Grantidentitäten erscheinen als `grant-sha256:<hash>`. Keine Texte, Header, Credentials, Gesprächskennungen oder URL-Parameter. Unauthentifizierte Anfragen erzeugen kein Consumerereignis. Authentifizierte Fehler und Abbrüche werden erfasst; Abbruchstatus 499. Der Subscriber lässt nur dieses eigene Target durch. K kann nach Installation seine Testanfragen über denselben Request-ID-Hash zuordnen, Z über das Journal. Das Ereignis allein beweist keinen Anbieteraufruf.

## Writer-Aktivierungsvertrag für P und Z

Q schreibt ausschließlich neue Sheet-/YouTube-Module in `brain-feeds` und die gemeinsame Writer-CLI `brain-source-sync --config <absoluter Konfigpfad>`. Patchnotes-, Build-Publish- und deren CLI-Pfade bleiben bei P/S. Gemeinsame Lockänderungen sind angekündigt und bereits lokal integriert; das Workspace-Manifest bleibt unverändert. Z löst die gemeinsame Integration.

Die CLI-Konfiguration enthält `infisical_config`, `base_release_id`, `base_release_sha256`, `owner`, `http_timeout_seconds`, `sheets` und `youtube`. Der Basishash bindet das serialisierte `CorpusRelease`. Infrastrukturzugang ausschließlich über den vorhandenen Infisical-Weg mit Credential-FD 5. Ziel ist die eigene Brain-Instanz am Unixsocket `/run/deadlock-brain-postgresql`, Port 5446, Datenbank `brain`, Rolle `brain_ingest`; historischer YouTube-Bestand wird getrennt als `brain_readonly` in einer lesenden Transaktion abgerufen. Kein Consumer-Postgres, keine Credentialdatei.

Jede Quelle verlangt ausdrücklich `ingestion_allowed=true`, eine Freigabereferenz, `license=null`, `history_retention_months=12`, `visibility=internal`, nichtleere enge Scopes, `publication_allowed=false`, `provider_egress_allowed=false`, `raw_retention_allowed=true`. Keine kanonischen Spielfakten, Gemini und YouTube-Lernen bleiben aus.

Ein erfolgreicher Writer erzeugt mit `PgStore::commit_batches_and_publish` einen Kandidaten und erhält fremde Release-Pins. Seine Ausgabe enthält `base_release_id`, `candidate_release_id`, Quellenrevisionen und `activation_performed=false`. Z prüft und aktiviert das gemeinsame Release; Q schaltet keinen Reader um. Vor den echten Läufen müssen Q/P/Z denselben Basishash, Quellenscope, Deployment-SHA und Aktivierungsweg verwenden.

Zs Betriebsvertrag vom 16:20 UTC ist gelesen. Der Runtimepfad ist `/etc/deadlock-brain/maintenance-runtime.json`, tatsächlicher `serve_config`-Pfad `/home/nathanael/.config/deadlock-brain/brain-serve.json`. Wiederverwendet werden die gemeinsame Publikationssperre und der vorhandene `ConfigWriter` sowie `ActivationPlan`; kein eigener Configwriter oder zweiter Aktivierungsorchestrator. Z baut den fehlenden gemeinsamen Kandidatenadapter.

Für diesen Adapter muss das Aktivierungsziel unterschieden werden: P veröffentlicht öffentliche Patchnotes im passenden Standardrelease; Qs Sheet-/YouTube-Kandidaten gehören wegen der bestätigten internen Rechte auf die Second-Brain-Bindung mit Scope `second_brain.internal`. Q liest deshalb deren gepinntes Basisrelease und Basishash. Bei Q-Aktivierung sind `credentials[second-brain/internal].release` und `internal_operator.release` atomar auf denselben Kandidaten zu setzen; Standardrelease, Docs-Grant und Twitch-Bindung bleiben erhalten. Der bestehende Standardrelease-only-ActivationPlan reicht dafür unverändert nicht. Z ergänzt diese enge Zielbindung im gemeinsamen Adapter statt private Q-Daten auf den öffentlichen Standardpfad zu schalten. Konkrete Kandidaten-/Knowledge-Kennungen werden vom Writer geliefert.

Die Retentionsintegration muss an dieselbe erfolgreiche Aktivierung und den Reader-/Cachewechsel gebunden werden. Unveränderliche Pins oder geladene alte Ausschnitte dürfen durch ein bloßes Aufbewahrungsfeld nicht weiter nutzbar bleiben. Q8 ermittelt und baut den konkreten engen Storevertrag; Q7 ergänzt vollständige YouTube-Metadaten und den sicheren Leermengenpfad in getrennten Dateien.

Q stellt nach gemeinsamer Installation und geprüfter Writerfreigabe seine beiden Units um: `deadlock-brain-sheet-sync.service` und `deadlock-brain-youtube-learning.service`. Altdateien werden als `.disabled` gesichert, Timer, OnFailure- und Infrastruktur-Drop-ins erhalten. Der konkrete ExecStart muss auf das SHA-geprüfte installierte Binary zeigen. Keine Unit wurde bisher geändert.

## Noch zu bauende Writerteile

Der Writer sperrt derzeit vor Secretabruf und Schreiben über `require_history_retention`. Die bestätigte Historienfrist ist im Kernspeicher noch nicht durchgesetzt. Nötig sind Quellen-Retention mit aktueller Fassung und zwölf Kalendermonaten Historie, Entfernung verschwundener Inhalte einschließlich relevanter materialisierter Daten sowie ein sicherer Umgang mit Release-Pins. Dieser Bedarf betrifft die gemeinsamen Kernpfade `brain-storage` und gegebenenfalls neue Migrationen; keine angewandte Migration ändern, Nummer erst nach frischem Fetch vergeben.

Außerdem fehlt der sichere Vertrag für eine ausdrücklich vollständig gelesene leere Quellenmenge. Die bestehende allgemeine Schutzregel gegen leere Abrufe bleibt erhalten. Historischer YouTube-Bestand ist kein aktueller Verfügbarkeitsbeweis; frische vollständige Metadatenprüfung und Löschsignale müssen vor Writerfreigabe ergänzt werden. Q arbeitet diese begrenzten Restteile weiter, ohne Retentionmetadaten oder Timer-Exit 0 als Wirkung auszugeben.

Provider-Shadow bleibt getrennt von G0: 103 genehmigte öffentliche FAQ-/synthetische Fälle, keine privaten Nutzerfragen. Noch kein echter Lauf und kein `PROVIDER_SHADOW_PASSED`. Die Aussagegrenze der bisherigen G0-Beobachtung bleibt fehlende Messbarkeit, nicht bewiesener Nullverkehr.
