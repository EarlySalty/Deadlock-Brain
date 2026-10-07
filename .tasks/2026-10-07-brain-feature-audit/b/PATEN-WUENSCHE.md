# Paten und vertagte Produktwünsche

Stand: 7. Oktober 2026. Reiner Audit, keine Produktänderung, kein Versand und keine Modellprobe.

## Ergebnis

Die Patenvermittlung ist auf aktuellem Bot-main weitgehend vorhanden. Ein Neubau wäre falsch. Der produktive Konfigurationsstand schaltet Concierge und proaktive Kontakte ausdrücklich aus. Dadurch startet nach dem vorhandenen Rust-Vertrag auch der Scheduler für Patenanfragen nicht. Das ist eine bewusste Sperre aus dem Serverguide-Auftrag, kein Anlass zur eigenmächtigen Aktivierung.

Zwei Lücken sind für die Fortsetzung entscheidend:

1. Der Nein-Knopf beantwortet den Klick, speichert aber keine eigene dauerhafte Ablehnung. Seine Antwort ersetzt auch die ursprüngliche Karte nicht. Der vorhandene `pate_offered`-Marker verhindert den regulären erneuten T2-Vorschlag, ersetzt aber keinen gespeicherten Nein-Entscheid.
2. Der untersuchte Serverguide-WIP weist alte Paten- und Steckbrief-Knöpfe ausdrücklich ab. Er enthält damit noch keinen freigegebenen Ersatz für die Patenvermittlung. Bei der Übernahme muss entschieden und belegt werden, wie der bestehende Patenweg unter den neuen Datenschutzgrenzen weiterläuft.

Dauerhafte Serverguide-Erinnerung, der begrenzte Testbetrieb und mehrere Anschlussangebote bleiben belegte offene Wünsche. Die alte Brain-Roadmap ist als pauschaler Bauauftrag teilweise überholt. Antwortkontext, Vertrauensgewichtung und hybride Suche haben inzwischen Rust-Bausteine auf main.

## Stände und Beweisgrenzen

Die folgenden main-SHAs wurden pro Repo mit `git ls-remote origin refs/heads/main` geprüft. Sie stimmen mit den lokalen `origin/main`-Referenzen überein. Quellbelege unten stammen aus `git show origin/main:<Pfad>`, sofern nicht ausdrücklich WIP angegeben.

| Repo, kanonischer Pfad | Aktuelles main | Hauptbaum |
| --- | --- | --- |
| `/home/nathanael/repos/Deadlock-Brain` | `9711cb630aebacfe959ed4783595b071f479be36` | HEAD `2734c2da4e814ff79953e8e825275b0216a6af16`, alt und schmutzig |
| `/home/nathanael/repos/Deadlock-Bots` | `e18f522226f8e2dec5a1c03fe97c2aba3200c8d1` | HEAD `dbda52b81cf8e68ea2aa7351a0e7ef2df5c81dbc`, abweichend und schmutzig |
| `/home/nathanael/repos/Deadlock-2nd-Brain` | `28b4c078c7ccde8d0708f88f5bf1cc51b01a9eff` | HEAD gleich main, sauber |
| `/home/nathanael/repos/Deadlock-Docs` | `6fa4ca3758d6f259cc0ca128e5350834fe1b4cfe` | HEAD `400231af7e8196e145111e4150c2e24c2ab79fb5`, abweichend und schmutzig |
| `/home/nathanael/repos/Deadlock-Twitch-Bot` | `e0b0dbaf662d7680c4ceaa210bf15f1443693cd8` | Für den Clip-/Consumer-Anschluss wurde main gelesen; Hauptbaum und Livefunktion nicht bewertet |

Der tatsächliche Git-Worktree liegt unter `/home/nathanael/.worktrees/serverguide-deploy-20261003/Deadlock-Bots`, nicht direkt im darüberliegenden Ordner. Er ist sauber, HEAD `7c2b0a086fb6d9426890a13955496e83e2405691`. Seine `origin/main`-Referenz ist der aktuelle Bots-main-SHA. `rust/bin/dl-bot/src/serverguide.rs` existiert auf diesem WIP-Kopf, aber nicht auf Bots-main.

Sichere Betriebsmetadaten: `deadlock-bot-rust.service` ist aktiv und läuft, MainPID `2848836`, Start am 7. Oktober 2026 um 01:23:58 CEST. Ein anfänglich geprüfter Name `deadlock-bot.service` war nicht vorhanden und ist kein Ausfallbeleg. Der Zugriff auf `/proc/2848836/exe` wurde vom Betriebssystem verweigert; kein weiterer Zugriff oder Umgehungsversuch. Der laufende Binary-SHA und der tatsächlich geladene Konfigurationsstand sind deshalb nicht unabhängig bestätigt.

Die bestehende Betriebs-TOML enthält unter `[runtime.community]`:

- `/home/nathanael/.config/deadlock-bots/bot.toml:48`: `concierge_enabled = false`.
- Dieselbe Datei, Zeile 49: Testnutzerliste vorhanden. Personenwerte wurden nicht ausgegeben oder bewertet.
- Dieselbe Datei, Zeile 52: `concierge_proactive = false`.

Das belegt den Konfigurationsstand auf Platte. Es beweist weder eine bereits erfolgte Zustellung noch die Wirkung eines bestimmten laufenden Binaries. Der historische Sperrbericht `/home/nathanael/Documents/.tasks/2026-10-03-serverguide/SOFORTSPERRE-LIVE.md:43-47` beschreibt dieselbe Scheduler-Sperre und verbietet eine Rücknahme durch Reaktivierung alter Kontaktserien.

Keine DB-Abfrage, keine privaten Nachrichten, Mitgliederlisten, Prozessumgebung oder privaten Logs wurden gelesen. Migrationen und Funktionsbeweise wurden nicht ausgeführt. Historische Testzahlen sind keine neuen Prüfergebnisse.

## 1. Paten anbieten und auf ein Ja vermitteln

**Nutzerwunsch:** Neue Spieler sollen sofort einen Menschen angeboten bekommen. Ein Ja erzeugt eine sichtbare Anfrage beim Patenteam. Ein Pate kann sie übernehmen; danach entsteht ein privater gemeinsamer Kanal.

**Quelle:** `/home/nathanael/repos/Deadlock-Bots/.tasks/2026-09-09-paten-programm-live/CONTRACT.md:15,23-25,35-38`, auf Bots-main. Die Ausgangszahlen in Zeile 19 sind historische Befunde vom 9. September und keine aktuellen Nutzungszahlen.

**Code auf Bots-main `e18f5222`:**

- `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/concierge.rs:1287-1288,1347-1358`: vorhandene Ja-/Nein-Knöpfe in den Angebotskarten.
- Dieselbe Datei, `6685`: `request_pate`; `1862-1886`: persistenter Anfragemarker und Angebot gesetzt; `8399-8411`: Ja-Klick führt unmittelbar in den Anfragenweg.
- Dieselbe Datei, `6988-6995`: Übernahme nur im vorgesehenen Server/Kanal und mit Patenrolle; `7025-7034`: gemeinsame Datenschutzprüfung beider Personen; `7037-7059`: ein noch gültiger Patenwunsch ist erforderlich.
- Dieselbe Datei, `7061-7074`: Lastprüfung, ab drei aktiven Patenschaften wird abgewiesen.

**Ein/aus, main/WIP/live:** Auf main gebaut. Die Betriebs-TOML steht auf aus. Aktuelle Vermittlung, Kanalrechte und Zustellung sind live unbekannt. Die mitgelieferte Vorlage `/home/nathanael/repos/Deadlock-Bots/config/bot.toml:32-35` steht zwar auf ein, ist aber kein produktiver Aktivierungsbeleg.

**Blocker:** Bestehende Sperre erhalten. Die neue Serverguide-Spec verbietet die Weitergabe von Profilen an Paten oder Moderatoren; `/home/nathanael/Documents/.tasks/2026-10-03-serverguide/HANDOFF-OFFEN-2026-10-03.md:112`. Der alte Claim liest dagegen einen Profil-Digest, `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/concierge.rs:7010-7017`. Dieser Datenvertrag muss vor einer Übernahme geklärt und eingehalten werden. Ein Datenschutz-Ja darf nicht stillschweigend zu einer Freigabe für private Gesprächsinhalte erweitert werden.

**Urteil: Wiederverwenden.** Vorhandenen Anfragen-, Rollen-, Last- und Kanalpfad weiterverwenden. Die neue Einbindung und die erlaubten Vermittlungsangaben müssen angepasst und belegt werden. Keine zweite Patenvermittlung bauen.

## 2. Nein und Stopp dauerhaft respektieren

**Nutzerwunsch:** Ein Nein bleibt ein Nein. Nach einem Klick verschwinden die Knöpfe; dieselbe Frage kommt nicht noch einmal. Stopp verhindert weitere ungewollte Kontakte.

**Quellen:** Verbindliche Arbeitsregel zur dauerhaften Bot-Entscheidung; Paten-Contract `/home/nathanael/repos/Deadlock-Bots/.tasks/2026-09-09-paten-programm-live/CONTRACT.md:24,37-39`; neuer Erhaltungsauftrag `/home/nathanael/Documents/.tasks/2026-10-03-serverguide/HANDOFF-OFFEN-2026-10-03.md:67,112`.

**Code auf Bots-main `e18f5222`:**

- `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/concierge.rs:8413`: `concierge:pate:no` gibt nur `text_reply(PATE_NO_TEXT)` zurück. Der Zweig enthält keine Speicherung.
- Dieselbe Datei, `1467-1481`: Antwort mit Standardwerten für `BridgeReply`; keine Aktualisierung der ursprünglichen Karte.
- `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-discord/src/interactions.rs:84-85,110-113`: `update_message` ist ein explizites Feld und durch `Default` aus. Ein knopflose Antwort ist deshalb kein Beleg dafür, dass die ursprünglichen Knöpfe entfernt wurden.
- `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/concierge.rs:461-468`: T2 wird durch `pate_offered` verhindert. Der reguläre Wiederholschutz ist vorhanden, aber unabhängig vom tatsächlichen Nein-Klick.
- Dieselbe Datei, `2764-2806`: Stopp speichert den globalen Datenschutzstatus und Concierge-Status in einer Transaktion, nimmt den Patenwunsch zurück und beginnt das Schließen vorhandener Anfragen. Dazu liegt auf main der bestehende Test `stopp_setzt_globalen_und_concierge_optout_gemeinsam`, Zeile 12657. Er wurde hier nicht ausgeführt.

**Ein/aus, main/WIP/live:** Stopp-Persistenz liegt auf main. Patenablehnung als eigener Entscheid ist im geprüften Knopfzweig nicht implementiert. Betrieb derzeit gesperrt; Funktion live unbekannt. Der Serverguide-WIP blockt die alten Knöpfe, statt diese Lücke zu beheben.

**Blocker:** Vor Reaktivierung Nein dauerhaft speichern und die ursprüngliche Angebotskarte ohne Aktionsknöpfe aktualisieren. Auch ein späterer Klick auf Ja derselben alten Karte muss gegen den gespeicherten Entscheid geprüft werden. Der jetzige Nein-Zweig gibt dafür keinen Zustand vor.

**Urteil: Bau nötig, eng am vorhandenen Pfad.** Stopp und Datenschutztransaktion wiederverwenden. Fehlend sind der dauerhafte Nein-Entscheid und das Verbrauchen der Angebotskarte. Kein neues Kontaktsystem erforderlich.

## 3. Liegen gebliebene Anfragen erinnern und schließen

**Nutzerwunsch:** Nach zwei Stunden ohne Übernahme soll einmal intern erinnert werden. Nach 24 Stunden bekommt der Neuling eine ehrliche Rückmeldung mit anderen Hilfswegen; die Anfrage wird geschlossen und kann nicht mehr übernommen werden.

**Quelle:** `/home/nathanael/repos/Deadlock-Bots/.tasks/2026-09-09-paten-programm-live/CONTRACT.md:25`; technische Abweichung und Zustellgrenzen in `/home/nathanael/repos/Deadlock-Bots/.tasks/2026-09-09-paten-programm-live/INTEGRATION_2026-09-27.md:36-57`.

**Code auf Bots-main `e18f5222`:**

- `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/concierge.rs:3010-3011`: zwei und 24 Stunden als Fristen.
- Dieselbe Datei, `5583-5598`: proaktive Profilkontakte hängen an `proactive`; Pateneskalationen laufen separat im selben Scheduler.
- Dieselbe Datei, `5604-5654`: fällige Anfrage und Datenschutzstatus werden erneut geprüft.
- Dieselbe Datei, `5657-5676`: geschlossene Karten und ausstehende Abschluss-DMs werden erneut bearbeitet.
- Dieselbe Datei, `5678-5715`: 24-Stunden-Abschluss und Zweistunden-Erinnerung mit persistent reservierten Stufen.
- Dieselbe Datei, `5757-5791`: Kartenhinweis und gesonderte verknüpfte Owner-Antwort. Laut Integrationsakte kann ein Edit allein keinen verlässlichen Discord-Ping auslösen; die Zusatzantwort ist die begründete Abweichung vom ursprünglichen Wunsch nach derselben Nachricht.
- Dieselbe Datei, `8463-8465,8526-8532`: bei `enabled=false` werden die Subscriber und der Scheduler gar nicht gestartet.

Die zugehörigen additiven Migrationen liegen auf main unter `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-central-db/migrations/`: `2026090918_concierge_pate_requests.sql`, `2026090920_concierge_pate_dm_retry.sql`, `2026100101_concierge_pate_optout_close.sql`. Die Integrationsakte fordert außerdem `2026090919_team_applications_pate_kind.sql`. Ihre Anwendung im produktiven Schema wurde hier nicht geprüft.

**Ein/aus, main/WIP/live:** Auf main vorhanden. Durch `concierge_enabled=false` derzeit laut Quellvertrag aus. Nur `concierge_proactive=false` würde die Pateneskalation nicht stoppen; die übergeordnete Aktivierungssperre ist entscheidend. Aktuelle offene Anfragen und Zustellung sind unbekannt.

**Blocker:** Einen erlaubten Scheduler-Anschluss für den vorhandenen Patenweg unter dem Serverguide-Vertrag herstellen. Migrationen und Wiederaufnahme nach Neustart müssen am endgültigen Release belegt werden. Die Integrationsakte nennt verbleibende Zustellunsicherheit bei Transport-Timeouts; eine garantierte exakt einmalige DM ist nicht belegt.

**Urteil: Wiederverwenden.** Persistente Stufen, Wiederaufnahme, Kartenabschluss und Wiederholschutz sind schon gebaut. Kein zweiter Erinnerungsdienst.

## 4. Paten gewinnen, anleiten und den Bestand sichtbar machen

**Nutzerwunsch:** Freiwillige können sich über das vorhandene Team-Panel als Pate bewerben. Bei Annahme erhalten sie die Rolle und einen Leitfaden; der Betreiber sieht den Patenstand.

**Quelle:** `/home/nathanael/repos/Deadlock-Bots/.tasks/2026-09-09-paten-programm-live/CONTRACT.md:26-30`.

**Code auf Bots-main `e18f5222`:**

- `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/team_applications.rs:31-65,277,356-385`: Pate als bestehender Bewerbungstyp, Panelknopf und Formular.
- Dieselbe Datei, `1421-1434`: Rollenvergabe bei Annahme, mit sichtbarem Fehler und Wiederholweg, wenn die Rollenvergabe scheitert.
- Dieselbe Datei, `1557-1560`: Annahme-Nachricht mit Leitfadenlink.
- `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/concierge.rs:5927-5928,5980-6024`: Leitfadenpfad mit Aktivierungssperre und gespeicherter Nachrichten-ID; vorhandene Textquelle `/home/nathanael/repos/Deadlock-Bots/assets/paten_leitfaden.toml`.
- Dieselbe Datei, `6050`: Pateninventar; `/home/nathanael/repos/Deadlock-Bots/rust/bin/dl-bot/src/aiglue.rs:155-167`: formatierte Bestandszeile mit Rolle, offenen Anfragen, aktiven Patenschaften und Leitfadenstatus.

**Ein/aus, main/WIP/live:** Bewerbungs- und Inventarbausteine auf main. Concierge-Leitfadenpflege ist an das ausgeschaltete Modul gebunden. Ob das aktuelle Team-Panel erreichbar ist, welche Freiwilligen verfügbar sind oder ob der Leitfaden aktuell gepinnt ist, wurde nicht privat oder produktiv abgefragt.

**Blocker:** Endgültigen sichtbaren Anschluss und Leitfadeninhalt unter dem neuen Datenvertrag prüfen. Historische Rollenmitgliederzahlen und alte Testberichte reichen nicht.

**Urteil: Wiederverwenden.** Bewerbung und Leitfaden sind keine fehlenden Neubauten. Aktueller Anschluss und Livewirkung bleiben unbekannt.

## 5. Serverguide-Übernahme und begrenzter Testbetrieb

**Nutzerwunsch:** Bestehende Hilfewege gehen geordnet in den Serverguide über. Der Testbetrieb hat einen klaren Teilnehmerkreis; Begrüßung, spätere Kontakte und Kanalaktivität werden getrennt freigegeben.

**Quellen:** `/home/nathanael/Documents/.tasks/2026-10-03-serverguide/HANDOFF-OFFEN-2026-10-03.md:65-67,123,126`; `/home/nathanael/repos/Deadlock-Bots/.tasks/2026-09-09-paten-programm-live/INTEGRATION_2026-09-27.md:20-26`.

**Code:**

- Bots-main `e18f5222`, `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-core/src/runtime_config.rs:146-155`: vorhandene TOML-Felder für Aktivierung, Testnutzer und Proaktivität.
- Bots-main, `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/concierge.rs:303-305,333-340`: Testnutzerliste; leer bedeutet offener Modus. Vor einem Pilot darf eine leere Liste nicht als begrenzter Test interpretiert werden.
- Guide-WIP `7c2b0a08`, `/home/nathanael/.worktrees/serverguide-deploy-20261003/Deadlock-Bots/rust/bin/dl-bot/src/serverguide.rs:1007-1013`: Paten-/Steckbrief-Knöpfe werden als alter deaktivierter Weg abgewiesen; danach folgt eine separate Pilotprüfung.

**Ein/aus, main/WIP/live:** Paten-Teststeuerung auf main vorhanden, derzeit durch übergeordnetes Aus gesperrt. Guide-Adapter nur WIP, auf Bots-main nicht vorhanden. Weder Übergabe noch aktueller Pilotbetrieb sind durch diese Akte als live belegt.

**Blocker:** Festgelegten Teilnehmerkreis und getrennte Betriebsfälle freigeben. Patenvermittlung darf beim Austausch des alten Concierge nicht still verschwinden. Andererseits verlangt das Handoff ausdrücklich, alte Kontaktserien bei der Übernahme nicht neu zu starten.

**Urteil: Wiederverwenden und Integration nötig.** Keine zweite Teststeuerung für Paten bauen. Die Freigabeentscheidung und der konkrete Paten-Anschluss sind offen. Der Abschluss-B zur Invite-Mechanik ist kein Patenauftrag und kein Erledigungsbeleg dafür.

## 6. Dauerhafte Erinnerung mit Korrektur, Ausschalten und Vergessen

**Nutzerwunsch:** Der Serverguide darf freigegebene kleine Angaben behalten; Nutzer können sie ansehen, korrigieren, entfernen, die Erinnerung ausschalten oder alles vergessen. Nach Vergessen entsteht das Profil nicht ungefragt erneut.

**Quelle:** `/home/nathanael/Documents/.tasks/2026-10-03-serverguide/HANDOFF-OFFEN-2026-10-03.md:120`. Die Akte hält endgültige Felder, Herkunft und Aufbewahrungsfristen ausdrücklich offen. Ohne freigegebene Frist darf dauerhafte Speicherung nicht aktiviert werden.

**Code, Guide-WIP `7c2b0a08`:** `/home/nathanael/.worktrees/serverguide-deploy-20261003/Deadlock-Bots/rust/bin/dl-bot/src/serverguide.rs:1196,1202-1230`: vorhandene Profilaktionen und freigegebene Feldnamen für Interessen, Ziele, Spielzeiten, Kommunikation, Antwortlänge, erklärte Abläufe und offenes Anliegen. `memory_off`, `memory_on` und `forget` werden getrennt übertragen. Das ist eine Adapterfunktion, kein Beweis für dauerhafte Speicherung oder Löschung im Brain.

**Ein/aus, main/WIP/live:** Adapter nur WIP; auf Bots-main nicht vorhanden. Der genaue produktive Gedächtniszustand und der Funktionsbeweis für Vergessen/Wiederanlageverbot sind unbekannt. Keine Profilabfrage vorgenommen.

**Blocker:** Produktentscheidung über Felder und Fristen; vollständiger Nachweis gegen endgültigen Brain-/Bots-Stand. Eine technische Frist für Berechtigungen oder Feedback ersetzt keine Profilfrist.

**Urteil: Wiederverwenden.** Vorhandene Profilkontrollen weiterführen. Neue dauerhafte Speicherung derzeit nicht freigegeben; Fehlumfang im endgültigen Backend unbekannt.

## 7. Weitere belegte Wünsche

### 7.1 Clip-Format im Onboarding gemeinsam festlegen

**Wunsch und Quelle:** Zuschnitt, Titel, Untertitel und Look gemeinsam festlegen und den Clip danach schneiden. Explizit als späteres Feature festgehalten in `/home/nathanael/repos/Deadlock-2nd-Brain/projekte/social-media-dashboard.md:114-117`, main `28b4c078`. Zeilen 118-121 halten Anbieter-/Einwilligungsfragen und die Nutzungsbedingungen offen.

**Code:** `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-dashboard-api/src/handlers/onboarding.rs:34,67`, Twitch-main `e0b0dbaf`, besitzt bereits Onboarding-Eingabe und Fortschrittsmodell. Dieser allgemeine Onboardingbaustein belegt kein Clip-Interview. Die vollständige Schnitt-/Editorstrecke wurde in diesem begrenzten Audit nicht geprüft.

**Ein/aus, main/WIP/live:** Wunsch auf main dokumentiert. Umsetzung und Aktivierung des konkreten Interviews unbekannt, kein Livebeweis. **Blocker:** ausdrückliche offene Produkt-/Datenschutzentscheidungen der Quelle, außerdem vollständige Bestandssuche im Clipbereich. **Urteil: unbekannt.** Nicht als sicher fehlenden Neubau einplanen, bevor bestehender Editor und Nebenpfade geprüft sind. Dieser Wunsch ist ein echter Produktpunkt, kein YouTube-Lernauftrag.

### 7.2 Öffentliche Supportantworten auch in Website-FAQ und Twitch-In-App

**Wunsch und Quelle:** `/home/nathanael/repos/Deadlock-Docs/PLAN.md:38-40`, Docs-main `6fa4ca37`, nennt beide als spätere Konsumenten desselben öffentlichen Antwortdienstes.

**Code:** Vorhandener Rust-Consumer `/home/nathanael/repos/Deadlock-Docs/tools/brain-adapter/src/lib.rs` auf Docs-main; zugehöriger interner, bewusst anders abgegrenzter Consumer `/home/nathanael/repos/Deadlock-2nd-Brain/tools/brain-adapter/src/lib.rs` auf Second-main. Der Datenvertrag ist genauer belegt in `/home/nathanael/repos/Deadlock-Brain/docs/c9-consumer-contract.md:35-43`, Brain-main `9711cb63`: Docs verwendet den öffentlichen Antworttyp; der Second-Brain-Operatorweg darf nicht in öffentliche Antworten übernommen werden. `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/crates/tb-config/src/dashboard_options.rs:12-35,89` auf Twitch-main `e0b0dbaf` enthält bereits einen konfigurierten Brain-Consumer.

**Ein/aus, main/WIP/live:** Consumerbausteine auf main vorhanden. Das belegt weder eine Website-FAQ-Oberfläche noch den vollständigen Twitch-In-App-Supportfluss. Aktivierung und sichtbare Funktion unbekannt. **Blocker:** genau die öffentliche Support-Oberfläche anbinden und belegen; nicht den internen Second-Brain-Weg dafür öffnen. Die Consumer-Timeout-Akte `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/welle1/w1/CONSUMER-GRENZEN-BRIEFING.md:7-19` ist eine Betriebsreparatur und kein eigenes Nutzerfeature. **Urteil: Wiederverwenden, sichtbarer Anschluss unbekannt.**

### 7.3 Freie Spiel- und Buildfragen mit Quellen beantworten

**Wunsch und Quelle:** `/home/nathanael/repos/Deadlock-Brain/docs/brain-qa-roadmap.md:18-36`, Brain-main `9711cb63`, alte Roadmap vom 25. Juni: freie Frage, automatische Kontextwahl, Quellengewichtung und semantische Suche.

**Aktueller Code auf demselben main:** `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/lib.rs:37-42,65,90,339` besitzt LexicalRetriever, HybridRetriever, Ask-Prompt, Vertrauensregeln und Ask-Kontextoptionen. `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/hybrid_port.rs:17-23,46-53,161` implementiert dichten Index und hybriden Retrieval-Port. `/home/nathanael/repos/Deadlock-Brain/rust/crates/brain-api/src/retrieval.rs:11-12,51-52` hat den typisierten Retrievalweg und verweist domain-/Buildfragen auf `/v1/answer`.

**Ein/aus, main/WIP/live:** Rust-Bausteine auf main. Konkrete Antwortqualität, freigegebene Daten und Endnutzeroberfläche live unbekannt; kein Modelltest durchgeführt. **Blocker:** Funktionsbeweis des bestehenden Pfads und aktuelle Quellenabdeckung. **Urteil: überholt als pauschaler Neubauauftrag; Wiederverwenden für die Abnahme.** Die alte Aussage, es gebe keine Frage-Antwort-Bausteine oder semantischen Pfade, beschreibt den heutigen main nicht mehr. Die hier ausgeschlossenen Video-/Forumteile der Roadmap wurden nicht untersucht.

### 7.4 Konkrete Communityangebote statt allgemeiner Wegweiser

**Wunsch und Quelle:** `/home/nathanael/Documents/.tasks/2026-10-03-serverguide/HANDOFF-OFFEN-2026-10-03.md:123`: aktuelle Mitspieler-, Coach-, Streamer- und Turnierangebote auf endgültiger Serverkonfiguration und Laufzeit prüfen.

**Code:** Der vorhandene Patenweg dieses Berichts ist ein konkreter wiederverwendbarer Teil. Allgemeine Concierge-Intents liegen auf Bots-main `e18f5222` in `/home/nathanael/repos/Deadlock-Bots/rust/crates/dl-community/src/concierge.rs:344-367`; die Guide-Adapter-Pilotgrenze liegt auf WIP `7c2b0a08` in `/home/nathanael/.worktrees/serverguide-deploy-20261003/Deadlock-Bots/rust/bin/dl-bot/src/serverguide.rs:1012-1019`. Beides belegt noch keine verfügbare Person, gemeinsame Runde oder aktuelle Veranstaltung.

**Ein/aus, main/WIP/live:** Concierge-Basis auf main und ausgeschaltet; Guide-WIP separat. Aktuelle konkrete Angebote und Vermittlung live unbekannt. **Blocker:** aktueller Angebots- und Zustimmungsvertrag sowie Funktionsbeweise. **Urteil: Wiederverwenden, Vollständigkeit unbekannt.** Allgemeine Kontaktserien und grafisches Onboarding werden von anderen Auditbereichen geprüft; dafür wurde hier kein paralleler Befund aufgebaut.

## Abgrenzung und empfohlene Reihenfolge

1. Den vorhandenen Patenpfad als Bestand festhalten. Kein Neubau und keine Aktivierung aus diesem Audit.
2. Dauerhaftes Nein und Kartenverbrauch schließen; Stopp-Persistenz erhalten.
3. Den Datenvertrag für die Serverguide-Übernahme klären: keine Profile oder DM-Inhalte an Paten weitergeben. Bestehende Anfrage und Übernahme dabei erhalten.
4. Erlaubten Anschluss für Pateneskalationen festlegen. Die aktuelle Concierge-Sperre stoppt auch den bereits gebauten Erinnerungsweg.
5. Begrenzten Testbetrieb ausdrücklich freigeben und am endgültigen Release belegen. Erinnerung nur mit freigegebenen Feldern und Fristen aktivieren.

Nicht als Nutzerfeature aufgenommen: Auto-Debug aus Docs-PLAN, reine Timeout-/Release-/Grantreparaturen, Invite-Abschluss-B, historische Prüf- und Deploymentarbeit. Nicht untersucht: YouTube-/Forumlernen, Replay-Neubau und Einzelmatchspeicherung. Historische fehlende Abhängigkeitsgraphen aus dem alten Brain-Hauptbaum wurden nicht als heutige main-Lücke übernommen.
