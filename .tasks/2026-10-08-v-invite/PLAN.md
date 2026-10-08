# V: Plan für den eigenen Einladungsstatus

Stand: 8. Oktober 2026, Bestandsprüfung bis 02:40 Uhr MESZ. Planung abgeschlossen, Produktcode nicht begonnen.

## 1. Tatsächlicher Bestand und Freigabegrenze

Eigener Start ist cf02c9a06d56aeab72cc1d685cc3256f00ed9b1a. Frischer Fetch und anschließender direkter Remoteabgleich ergeben main 400381e681a2e08283db46094d1bbbde037e2813. Ks Source baf981f9146e69c9a2d270c915f45a444a2ed490 ist dessen Vorfahr, geprüft mit merge-base --is-ancestor, Exit 0. Damit ist die sachliche Mainbedingung für den Brain-Readgate inzwischen erfüllt. Die älteren K-Akten auf diesem Stand berichten noch zwei abgewiesene Mainpushes. Dieser historische Bericht überstimmt den frisch gelesenen tatsächlichen Remotestand nicht. Ein neuer Deploy oder Livebeweis wurde durch V nicht geprüft.

Die zweite Bedingung bleibt offen: kein übergebener Dateischnitt beziehungsweise explizit getrenntes Delta für V in den gemeinsamen K-/G-Dateien. Deshalb weiterhin Planung, keine Produktänderung, kein Fixer und kein zusätzlicher Botsworktree. Ohne Kontaktaufnahme zu fremden Sessions geht die Abgrenzung über diese Akte zurück an den Delegator.

Graphify wurde vor der Codesuche lokal und global gefragt. Der Worktree hat keinen lokalen Graphen; der vorhandene zentrale deadlock-brain-Graph kennt den Invite-Vertrag, aber nicht die neuen Invite-/Readgate-Symbole. Globale und Bots-Abfragen liefern ebenfalls keine aktuellen Statussymbole. Deshalb anschließend konkrete historische und aktuelle Gitquellen nachgelesen, keinen Graphrefresh oder Neubau ausgelöst.

Verbindliche historische Akten vollständig gelesen: .tasks/2026-10-06-brain-abschluss/A/REVIEW.md und E3F-RUECKGABE.md aus dem erhaltenen Quellbaum /home/nathanael/.worktrees/brain-cutover-lokalsicherung-20261008. Im eigenen Startbaum fehlen diese beiden Dateien. Dieser Quellbaum wurde nicht verändert. E3f-Code wurde über die vorhandenen Commitobjekte gelesen, nicht ausgecheckt oder verwaltet.

## 2. Genaue Wiederverwendung und benötigte Quellstände

| Stand | Verwendung | Grenze |
| --- | --- | --- |
| fde910f6a0199c00f44083e73fc8f4c5e4f80b86 | Historische E3-Basis zum isolierten Lesen des Feature-Deltas | Nicht als aktuelle Integrationsbasis verwenden |
| d8a0e727687dda38675833af38dbb698f2ef949c | Bestehender Statusvertrag, requestgebundene Evidence, API-Projektion, Providergrenze und lesender MCP-Aufruf | Enthält ursprünglichen BLOCK und unnötige Kommentaränderungen; keine blinde Übernahme |
| 0b47d78b3fb3d596c40d567384704cd1c03de1f4 | Bereits enger gemachte Verfahrens-/Botfragen und bereinigte Prüffälle | Noch keine abschließende fachliche Freigabe |
| 19f6d49f196a8ea4b9d9d90c188f3a3867914779 | Letzter zusammenhängender A/E3f-Quellstand für die frische Fixrunde | Tatsächlicher gpt-6.1-sol-BLOCK bleibt zu beheben |
| baf981f9146e69c9a2d270c915f45a444a2ed490 | Aktueller K-Readgate in main, inklusive interner Context- und Flightbindung | K-Policy nicht durch historischen E3-Code ersetzen |

Keiner der drei E3-Featurecommits ist ungeprüft zum Cherry-Pick freigegeben. Benötigt ist deren enges, fachlich erhaltenes Invite-Delta auf frischem main, mit anschließendem frischem Fix für den belegten BLOCK. Bereits aktuelle G-/K-Änderungen haben Vorrang vor alten Vollversionen derselben Dateien.

Aktueller main besitzt keinen invite.rs-Vertrag und keine SelfInviteStatus-/invite::Verdrahtung. Der vorhandene öffentliche Discordpfad ist dagegen aktiv im Source verdrahtet: brain-serve/src/service.rs:468 und :470 verwenden denselben DiscordRetriever. DiscordLive nutzt den konfigurierten HTTP-/MCP-Anschluss. Die generische Kernel-Toolport-Lücke aus K ist kein Anlass für einen zweiten Connector: E3s enger direkter lesender Statusaufruf im bestehenden DiscordLive ist der Anknüpfungspunkt, kein neuer allgemeiner Toolagent.

Historischer Vertrag: brain-contracts/src/invite.rs:12 enthält Sent, Pending, FriendshipMissing, AlreadyHasGame, Error, Unknown und Unavailable. SelfInviteStatus enthält status und optional at, mit deny_unknown_fields und RFC3339-Prüfung. status_evidence und projection bei :172/:191 begrenzen Quelle, Beleg-ID, Sichtbarkeit, Scope und Datenform. Das wird erhalten und an aktuelle requestgebundene Freigaben angeschlossen.

## 3. Enger Dateischnitt zur Übergabe an K/G

### Brain: zusammenhängende Invite-Gategruppe

| Datei unter rust/ | Geplantes V-Delta | Abgrenzung |
| --- | --- | --- |
| crates/brain-contracts/src/invite.rs | Historischen Vertrag wiederverwenden, requested korrigieren, Regressionen | Invite-eigener neuer Pfad, keine zweite Engine |
| crates/brain-contracts/src/lib.rs | Modul exportieren | K-Feld allow_discord_reads und übrige Verträge unverändert erhalten |
| crates/brain-contracts/Cargo.toml und Cargo.lock | Bestehende Chrono-Verwendung des Statuszeitpunkts übernehmen, soweit noch erforderlich | Keine Versionserfindung oder fremden Lockfileänderungen |
| crates/brain-contracts/src/provider_input.rs | Vorhandene Enum-/Zeitprojektion aus E3 in aktuellen zentralen Payloadpfad integrieren | Gs aktuelle Projektions-/Budgetverträge erhalten; eigener Status enthält keine Rohfrage |
| crates/brain-providers/src/hardening.rs | Historische Status-Evidenceprüfung vor beiden bestehenden Transporten | Keine Provider-, Modell- oder Timeoutänderung |
| crates/brain-api/src/lib.rs | Nach Authentifizierung und interner Personen-/Scopebindung eigene Frage vor Kernel normalisieren | K-Readpolicy und aktuelle G-APIsemantik erhalten |
| crates/brain-serve/src/discord_live.rs | retrieve_invite, read_self_invite und beide Freigabegrenzen wiederverwenden | K-Nachrichtenreadguard bleibt geschlossen; eigene Statusprüfung getrennt von Nachrichtenrechten |
| crates/brain-serve/src/discord_live/invite_tests.rs | Bestehende E3-Regressionen auf aktuellen Stand übertragen und BLOCK-Fälle ergänzen | Keine echten Community-Daten |

brain-api/src/http.rs ist ein historischer Überschneidungspfad, aber aktuell bereits von K für Personenbindung und Readpolicy geändert. Kein vorgeplanter V-Produktdelta dort. Erst wenn die konkrete Integrationsprüfung eine fehlende Bindung belegt, enges Delta zusätzlich ordnen. Gleiches gilt für brain-client/src/async_client.rs, brain-kernel/src/flight.rs und service.rs: aktuelle K-/G-Verdrahtung wiederverwenden, nicht auf historische Versionen zurücksetzen. process_e2e.rs nur für tatsächlich notwendige vorhandene Harness-/Fixtureanpassung, nicht pauschal aus E3f übernehmen.

### Bots: erst beim freigegebenen Consumeranschluss

Gelesener lokaler Bots-Remote-Main: 0fb873c6887c6ec8df6ce50d15c8ded9781fbadf. Dort fehlt mcp/self_invite.rs und der MCP-Eintrag self_invite_status. mcp.rs:155-185 besitzt bereits den authentifizierten öffentlichen MCP-Pfad mit request- und user-Headerbindung sowie public::access. Den verwenden, nicht ein zweites Endpoint-/Identitätsverfahren bauen.

Historische Quelle 2be2df16d7a9390823a05691bef1ae69f1b8c8c1 gegen e18f522226f8e2dec5a1c03fe97c2aba3200c8d1 enthält die lesende Postgresprojektion mcp/self_invite.rs und ihre MCP-Anbindung. V übernimmt später ausschließlich den benötigten Statusleseschnitt in einen eigenen neuen Botsworktree. Nicht den gesamten Consumercommit, keine alte Cooldownausnahme und keine alte brain_api.rs-Vollversion.

Minimal erwarteter Bots-Schnitt: rust/bin/dl-bot/src/mcp/self_invite.rs und der enge self_invite_status-Zweig in mcp.rs. modglue.rs, dl-brain/src/brain_api.rs und dl-brain/src/lib.rs bleiben zunächst K-eigen beziehungsweise aktuelle Bestandswege. Nach Ks Consumerlieferung prüfen, ob die vorhandene Personen-/Readpolicyübermittlung den eigenen Status bereits trägt. Nur nach belegter Lücke zusätzlich exakt abgegrenztes Delta mit dem Delegator vereinbaren. Vor diesem Anschluss keine Produktaktivierung und kein Versprechen eines verfügbaren echten Status.

## 4. Sollverhalten und Ursache des BLOCK

Historischer Matcher invite.rs:148 akzeptiert invite && status, ohne im allgemeinen Zweig eigene Person zu verlangen. „Wann sind Einladungen wieder verfügbar?“ und „When does an invite expire?“ gelangen dadurch in project_query, retrieve_invite und die zentrale Statusprojektion statt in normales Retrieval. Das ist eine semantische Umleitung, kein Textproblem.

Ein zentraler deterministischer Matcher muss einen ausdrücklich eigenen Status-/Zugangsbezug und einen lesenden Statuszweck erkennen. Eigene Person über Wortgrenzen, nicht Teilstrings. Allgemeine Zeit-/Ablauffragen, fremde Personen, Botbeschreibungen und Versand-/Änderungsaufträge passen nicht. Bei fachlich uneindeutiger Frage normaler Frageweg, keine persönliche Abfrage. Normalisierung darf die gewöhnliche Gesamtfrage nicht unbemerkt durch Abschneiden an erstem Fragezeichen oder Zeilenumbruch umdeuten. Eigene erlaubte Statusfragen werden dagegen vor dem Provider auf den vorhandenen neutralen Statustext reduziert, einschließlich privater Zusätze.

Echte Beispiele: „Bin ich eingeladen?“, „Ist meine Einladung verschickt?“, „Wie ist mein eigener Deadlock-Einladungsstatus?“, „How long has my invite been pending?“ und die belegten eigenen Deadlock-Zugangsfragen. Allgemeine Beispiele: die beiden aktuellen BLOCK-Fälle, „Wie kann ich eine Einladung verschicken?“, „Welche FPS bekomme ich in Deadlock?“ sowie beliebige gewöhnliche Fragen mit dem Wortlaut „invite status“.

Zwillingsstellen gemeinsam prüfen: invite.rs requested/project_query/projection, brain-api lib.rs, provider_input.rs:15, hardening.rs:45 und DiscordRetriever bei Retrieval sowie beiden Evidencefreigaben. Ein einzelner engerer Matcher ohne Erhalt des normalen Fragewegs genügt nicht.

K-Readgate trennt Nachrichtenreads über allow_discord_reads: discord_live.rs:275-286 verweigert vor I/O, allowed bei :341-348 sperrt öffentliche Livebelege, Beobachtungsschlüssel bei :359-379 enthält Readpolicy und Personenbindung. Der historische retrieve_invite nutzt denselben allowed-Guard. Seine unveränderte Übernahme würde private eigene Statusfragen ebenfalls sperren. Deshalb Statusberechtigung explizit als eigener begrenzter Leseschnitt prüfen: vertrauenswürdiger Consumer, erlaubte Person, bot.public-/request-Scope, request_id, Providerfreigabe und Budget. Das Nachrichtenrecht wird dabei nicht eingeschaltet; weder public_server_facts/read_messages noch andere Livebelege werden aus der privaten Statusfrage freigegeben. Flight-/Cachegrenzen behalten Ks Policy- und Personenbindung.

## 5. Heutige Quote und historische Consumerbefunde

Bots dl-brain/src/lib.rs:33 setzt 50, reserve_discord bei :59 und DiscordRateState::reserve bei :90 verwenden Europe/Berlin. handle_discord_query reserviert vor answer_discord_query. Aktuelle Kommandos und Nachrichten rufen diesen Pfad aus modglue.rs:535/:789. Nach ausgeschöpfter Quote einmal DailyLimit, danach Suppressed. Der historische personengebundene Abschlussmarker und die Invite-Cooldownausnahme sind in diesem Discordpfad nicht vorhanden. Sie dürfen nicht mit E4f wiedereingeführt werden. Legacy-handle_brain_query ist noch vorhanden, aber kein Ersatzpfad für V.

Historisches self_invite.rs:135-222 aus 2be2df16 liest aktuelle Requests und ordnet Request- und GC-/Taskbelege nach Zeitpunkt. Dies ist der Wiederverwendungskandidat für den alten Befund „historische Beobachtung verdeckt neueren Pending-/Errorrequest“. Bei Übernahme anhand der heutigen Postgresverträge erneut prüfen, einschließlich Zuordnung zum ausgewählten verifizierten eigenen Konto und Tasktyp. Keine schreibende Zustandskorrektur und kein neuer Versand. Da der aktuelle main diesen Statusleser nicht enthält, ist die historische Korrektur kein Beweis einer bereits aktiven Funktion.

Sollte später eine laufende Reservation benötigt werden, muss ihr Abschluss requestgebunden sein. Kein Abschluss allein anhand der Person, der einen neueren Request freigibt. Der heutige tägliche Zähler erfordert keinen historischen finish_question-Mechanismus.

## 6. Regressionen und Beweisgrenzen

1. Matcher und Projektion: beide aktuellen BLOCK-Fälle sowie allgemeine Verfahren/FPS/Bot-/Fremdstatusfragen bleiben unverändert; eigene Statusfragen werden begrenzt erkannt. Mehrsatzfragen, Wortgrenzen, Großschreibung, private Zusätze und Aktionswörter prüfen.
2. Zugriff: zwei getrennte Personen/Requests, fehlende und unzulässige Identität, abweichender Scope, manipulierte Evidence, falscher Request, verbotene Readpolicy und alter Beobachtungs-/Flightstand. Private eigene Frage darf den Status lesen, darf keine Nachrichtenreads auslösen. Generische private Frage nutzt weiter gespeichertes erlaubtes Wissen.
3. Egress: echte ausgehende Payloads beider bestehenden Providertransporte lokal isoliert prüfen. Bei eigenem Status nur kanonische Frage und Enum/at, keine IDs, Mitglieder, Rohfrage, private Zusätze, fremde Belege oder Steamoperationen. Keine neuen externen Modellaufrufe für die Prüfung.
4. Consumer: aktueller Kommandopfad und Nachrichtenpfad, 50 akzeptierte Fragen pro Berliner Tag, 51. einmal gemeldet, weitere unterdrückt, Kanalwechsel ohne Extrabudget und Berliner Tageswechsel einschließlich Sommer-/Winterzeit. „invite status“ schafft keine Ausnahme. Neuere Pending-/Errorrequests schlagen alte Beobachtungen; ggf. alter Abschluss kann neuere Reservation nicht lösen.
5. Fehlerzustände: unknown/unavailable/error und fehlender Zeitpunkt bleiben unterscheidbar. Keine Behauptung von Versand/Ablehnung ohne Beleg. Statusleser ruft keinen mutierenden Steamweg auf. Bestehende Scratch-/Prozessprüfungen gegen tatsächliche sichere Harnessverträge; historische grüne Zahlen sind keine neue Baseline.

Nach Übergabe frischen nativen Fixer mit diesem Quellenstand und exakt diesen BLOCK-Funden starten. Keine Wiederaufnahme des alten E3f-Kontexts, kein zusätzlicher Reviewer. Falls der reguläre Gate blockiert, frischer Fixer je Runde mit demselben bisherigen Urteilmodell; Rückmeldung nur bei Abschluss oder echtem Feststecken.

Passende Pakete: brain-contracts, brain-api, brain-providers, brain-serve und bei tatsächlicher Änderung die jeweiligen Client-/Kernelziele. cargo-slot +1.97.1, SQLX_OFFLINE=true, --locked --offline --jobs 3 gemäß K-Prüfweg. Vor Tests rolle-test-waechter für aktuelle Pflichtflags laden, vorhandene Suites erhalten, Clippy auf betroffene Ziele mit -D warnings, Formatprüfung mit passenden rustfmt-Flags. Kein Cargo-Lauf in dieser reinen Planungsphase.

## 7. Integration, Betrieb und nächste sachliche Aktion

Nach dokumentiertem Dateischnitt aktuellen main regulär in eigenen Baum integrieren und Deltas gegen diesen Stand neu prüfen. Kleine vollständige Gategruppe für die gesamte Statusgrenze statt isoliert freigegebener Matcherfragmenten. Nach ALLOW sofort liefern, keine künstliche Wartung auf andere Pakete. Produktives Statuslesen benötigt den tatsächlich gelieferten Bots-MCP-Leseschnitt; ein allein ausgeliefertes Brain-Enum ist kein Funktionsabschluss.

Release im eigenen Worktree, Branch-/SHA-Nachweis des laufenden Binary vor Deploy, Deployment über vorhandenen Wrapper vom aktuellen origin/main. Danach Neustart, private lesende Funktionsprobe ohne öffentliche Community-Testnachricht und Nachweis der tatsächlichen zentralen Antwort. Keine Rohantwort oder Personenkennzeichen in Git, Reviewkontext oder Bericht. Cleanup nur eigener Branch/Worktree nach Ancestor-Exitbeweis. Kein PR, keine alten A-Branches löschen.

Nächste sachliche Aktion für den Delegator: den in Abschnitt 3 benannten V-Dateischnitt mit K/G als übergeben festhalten. Das ist die offene Eigentumsbedingung, keine zusätzliche Planfreigabe.

BESTAND[BS-1]: teilweise | Fundort: 19f6d49f:rust/crates/brain-contracts/src/invite.rs:38 | Anknüpfung: bestehender A/E3f-Statusvertrag, zentraler Providerpayload und DiscordLive-MCP-Lesepfad
WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 live geprüft

Die drei Planungsbefunde sind der breite historische Matcher, der Konflikt zwischen historischem Statusguard und aktueller K-Readpolicy sowie der fehlende aktuelle Bots-Statusanschluss. Quellpfade wurden gelesen; die Wirkungszeile bescheinigt ausdrücklich keinen Laufzeitbeweis.
