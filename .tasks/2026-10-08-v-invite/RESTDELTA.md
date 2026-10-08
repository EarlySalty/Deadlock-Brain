# V: genaue Restdeltas für die gemeinsame Brain-Verdrahtung

Stand: 8. Oktober 2026. Dies ist eine Integrationsbeschreibung, kein freigegebener Produktpatch. K/G behalten diese Dateien bis zur Restübergabe durch den Delegator. Quellenstand für die Fundstellen: Brain main 400381e681a2e08283db46094d1bbbde037e2813, bereits im eigenen Teilbaubaum 22561cb2 integriert. Nach K/Gs gesichertem Commit Positionen und Verträge erneut prüfen.

## 1. Vertragsmodul ausführbar machen

- brain-contracts/src/lib.rs: das V-eigene invite.rs exportieren. Ks bestehendes DiscordRequestContext.allow_discord_reads bleibt erhalten.
- brain-contracts/Cargo.toml und Cargo.lock: Chrono-Verwendung des historischen Statuszeitpunkts mit vorhandener Workspaceversion ermöglichen, falls nach K/Gs finalem Stand erforderlich. Keine weiteren Dependencyänderungen aus historischem Feature übernehmen.
- Danach die direkten invite.rs-Modultests erstmals im tatsächlichen Vertragscrate ausführen. Ein erfolgreicher Lauf vor diesem Export wäre kein Nachweis des neuen Moduls.

## 2. Projektion nach interner Personenprüfung

brain-api/src/lib.rs:177-235 enthält Authentifizierung und vertrauenswürdige Discord-Consumerbindung. :229-234 erzeugt den internen requestgebundenen Context einschließlich user_id und allow_discord_reads. :236 ruft den Veröffentlichungs-Kernel auf.

Benötigtes Delta: ausschließlich nach dieser Bindung und vor :236 die eindeutige eigene Statusfrage über invite::project_query normalisieren. Request-/Conversationbindung, requested_scopes und interner Context bleiben erhalten. Der Provider bekommt die kanonische eigene Frage statt Rohfrage und privater Zusätze. Gewöhnliche Fragen bleiben unverändert. Ohne freigegebene eigene Identität gibt es keinen persönlichen Fremdstatus; fehlende eigene Zuordnung muss als unbekannt beziehungsweise nicht verfügbar begrenzt bleiben.

Kein vorab benannter Delta in http.rs oder Client: aktueller K-Eingang übermittelt Personen- und Readpolicy bereits. Ein zusätzliches Interface ist erst bei konkret belegter Lücke begründbar.

## 3. Statuslesen vom Nachrichtenrecht trennen

brain-serve/src/discord_live.rs:275-286 sperrt Readpolicy=false vor dem bestehenden öffentlichen Nachrichten-/Faktenabruf. allowed bei :341-348 sperrt öffentliche Livebelege bei ausgeschaltetem Lesen. Beide Guards bleiben für ihren aktuellen Pfad geschlossen. observation_key bei :359-379 bindet Request, Person und Readpolicy; diese Trennung erhalten.

Benötigtes Delta in derselben Datei:

1. Bestehenden DiscordLive-Client und konfigurierten MCP-Anschluss für den engen self_invite_status-Leseaufruf verwenden. Personenkennung und Request-ID bleiben interne Header. Keine neuen Connectoren, Timeouts oder Toolagenten.
2. Eigene Statusberechtigung separat vom öffentlichen Nachrichtenrecht prüfen: bot.public, vertrauenswürdige Consumer-/Personenbindung, gültige request_id, eigener request-Scope, bestehende Providerfreigabe und Budget. request_id aus Query und Context muss übereinstimmen. Eine eigene Statusfrage schaltet allow_discord_reads nicht auf true.
3. Den vorhandenen A/E3f-retrieve_invite-Schnitt wiederverwenden: ausschließlich Enum/at als requestgebundene Evidence, kein weiteres Nachrichtenretrieval, keine Mitglieder oder andere personenbezogene Quellen. Statusfrage erzeugt keine Einladung und keinen schreibenden Steamaufruf.
4. Öffentliche Liveevidence und eigene Statusevidence an beiden vorhandenen Freigabewegen getrennt prüfen. Für Statusevidence Quelle, Datenform, Scope, Observation und vollständige Request-/Personenbindung erzwingen. Fremde/manipulierte Statusbelege müssen vor Provider und Veröffentlichung scheitern.
5. Private eigene Statusfragen dürfen trotz geschlossener Nachrichtenreads ihren eng erlaubten Statuspfad nutzen. Generische private Fragen nutzen weiterhin zulässiges gespeichertes Wissen, kein Live-Mitlesen als Ersatz.

Der aktuelle generische Kernel-Toolport ist kein Grund für eine zweite Antwortengine. Der direkte bereits historische Status-MCP-Leseschnitt hängt am bestehenden DiscordRetriever/DiscordLive. service.rs wird zunächst nicht verändert.

## 4. Zentrale Providergrenze

brain-contracts/src/provider_input.rs:grounded_messages enthält den gemeinsamen Payloadbau. brain-providers/src/hardening.rs:29-69 enthält authorize und die vorhandene RequestScoped-Scopeprüfung. Diese Zentralstellen behalten K/Gs aktuellen gemeinsamen Antwort- und Budgetvertrag.

Benötigtes Delta: historische eigene Enum-/Zeitprojektion in grounded_messages integrieren, nicht die frühere ganze Datei übernehmen. Eigene Statusquelle bleibt RequestScoped; Rohfrage, Discord-/Steam-IDs, Mitgliederlisten und fremde Daten gelangen nicht in den Modellpayload. Der zentrale Hardeningpfad validiert eigenen Status einschließlich query-/evidence-Zusammenhang vor beiden Transporten. Statusbelege an allgemeinen Fragen dürfen keine persönliche Antwortumleitung auslösen.

Beide echten ausgehenden Payloadvarianten später lokal isoliert prüfen. Framing/Inputbudget kommt aus demselben projizierten Payload, keine separate Schätzung oder Bot-Antworttexte. Bestehender Provider, Modell und Konfiguration bleiben unverändert.

## 5. Gemeinsame Abnahme und Liefergrenze

Nach gesichertem K/G-Commit wird die Restübergabe für lib.rs, provider_input.rs, API lib.rs, discord_live.rs, hardening.rs und erforderliche Cargo-Dateien konkret dokumentiert. V integriert dann den Commit regulär im eigenen Baum und erledigt die genannten engen Deltas. Kein gleichzeitiger Writer in denselben Dateien.

Regressionsgruppe zusammenhängend prüfen: allgemeine/eigene Fragen, private Statusrechte ohne Nachrichtenread, Scope-/Personen-/Requesttrennung, Cache-/Flighttrennung und Providerpayload. V/F1s Bots-Gate beweist nicht diese Brain-Verdrahtung; ein noch unexportiertes invite.rs beweist keine Laufzeitantwort. Nach tatsächlichem gemeinsamen ALLOW regulärer Main-/Deploy-/Restart-/Liveabschluss. Vorher erhaltene Featurebranches und Worktrees nicht aufräumen.
