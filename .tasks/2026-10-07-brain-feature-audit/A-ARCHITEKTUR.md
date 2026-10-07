# Audit A: Brain als gemeinsame KI-Schnittstelle

Stand: 07.10.2026. Reine Planungsgrundlage, keine Bau- oder Laufzeitänderung. Quelleninventar: `A-KI-INVENTAR.md`. Die drei abgeschlossenen Fachrückgaben liegen im Workflow-/Agentenprotokoll; Nachweisorte stehen in `A-STATUS.md`. Unter `a/` wurden keine Rohberichtdateien angelegt.

## Urteil und Empfehlung

**Machbar, aber `/v1/answer` ist heute kein Universalvertrag.** Brain soll die einzige anwendungsseitige Schnittstelle für Modellaufrufe, freigegebenes Wissen, Routing und Nachweise werden. Plattformzustand, Nachrichtenversand, Moderationsdurchsetzung, Streamverarbeitung und deterministische Sicherheitsprüfungen bleiben bei ihren Fachdiensten. Eine gemeinsame Schnittstelle bedeutet weder ein einziges Modell noch das Zusammenlegen aller Datenbestände.

Die passende Erweiterung ist der vorhandene Rust-Kernel mit seinen Providerports und Konnektoren. Kein zweiter Antwortdienst, kein zweiter Spielwertebestand, kein neuer Universal-SDK neben den bestehenden Clients. Twitchs `tb-llm` wird kompatible Fachfassade zum Brain, nicht ersatzlos entfernt. Discord behält Plattformadapter und lokale Sicherheitsmechanik. Ein Providerwechsel ist kein Bestandteil dieser Migration. Die ausdrücklich freigegebene Patchnotes-Ausnahme für Perplexity `sonar-pro` bleibt erhalten.

## Nachgewiesene Grundlage und Grenzen

Brain-Snapshot `9711cb630aebacfe959ed4783595b071f479be36` aus dem lokal vorhandenen `origin/main`. Kein Fetch ausgeführt. Zeilen beziehen sich auf diesen Snapshot, soweit kein Worktree genannt ist.

| Vertrag | Nachgewiesen | Konsequenz für die Zentralisierung |
| --- | --- | --- |
| Anfrage | `rust/crates/brain-contracts/src/lib.rs:65,75`: Profile Fact/Explain/Build/Coaching; Query mit request_id, conversation_id, text, optionalem DomainRequest, requested_scopes, patch, mode. | Kein generischer typisierter Judge-Auftrag, kein Mehrnachrichten-/Anhang-/Audiovertrag und keine UseCase-Auswahl für beliebige Botaufgaben. Text als verstecktes Steuerprotokoll wäre eine Regression. |
| Öffentliche Antwort | `brain-contracts/src/public_api.rs:34,43`: Status, Text und öffentliche Zitatlabels; beantwortete/rejektierte Ergebnisse benötigen Text und Belege. Usage und Providerdaten werden ausdrücklich nicht öffentlich projiziert. | Für wissensgebundene Antworten passend. Nicht passend für Scam-Urteile, LFG-Extraktion, Pitch-Abnahmen oder strukturierte Aktionsvorschläge ohne Wissenszitate. Usage muss intern zugänglich bleiben, nicht an Communitytexte angehängt werden. |
| Transport | `brain-api/src/http.rs:21,22,101,122,150`: POST answer/retrieve, Auth vor Bodylesen, 64-KiB-Grenze, Workerbindung und Deadline. `brain-providers/src/transport.rs:146,157,158`: Abo-Transport im Main-Snapshot genau zwei Eingangsnachrichten, stream=false, keine Tools. | Keine nachgewiesene SSE-/Streamingantwort, keine langen asynchronen Jobs, kein Chatverlauf als eigener Vertrag. G verändert den Tooltransport, nicht automatisch diese anderen Fähigkeiten. |
| Identität | `brain-policy/src/lib.rs:155,165,177`: Principal aus registriertem Zugang, Scopes nur einschränkbar, ConversationOwnership an actor_id. `brain-api/src/lib.rs:210`: kurzlebiger Discord-Requestscope; User-ID nur für registrierte Discord-Consumer. | Dienstidentität ist vorhanden. Ein Bot-Principal ersetzt keine vollständige Nutzer-/Channel-/Tenantbindung. Für Twitch fehlen entsprechende nachgewiesene Platform-ID-Kontexte und konkrete Datenrechte im universellen Auftrag. |
| Provider | `brain-serve/src/config.rs:80,95`, `service.rs:208`: OpenAI-kompatibel oder Codex-Abo, ein konfigurierter Antwortprovider. `brain-providers/src/lib.rs:72`: Abo-Helfer setzt gpt-6-luna. | Noch kein nachgewiesenes serverseitiges Routing nach sämtlichen Twitch-/Discord-UseCases und Datenklassen. Lokaler Loopback-Proxy bedeutet nicht lokale Modellverarbeitung. Bestehende Modelle/Timeouts müssen explizit übernommen werden, kein pauschaler Wechsel auf Luna. |
| Budget/Sicherheit | `brain-contracts/src/lib.rs:135,295`; `brain-serve/src/config.rs:109`; `brain-providers/src/transport.rs:118`; `brain-kernel/src/lib.rs:16`: Kosten-, Eingabe-, Ausgabe- und Netzbudgets, konservative Eingabezählung, getrennte Publikationsprüfung. | Wiederverwenden. Mehrere Turns, Retries und Konnektorzugriffe müssen kumuliert zählen. Ein wiederholter Request darf das Budget oder eine Aktionsfreigabe nicht erneuern. |

Die verkürzten Cratepfade in der Tabelle liegen jeweils unter `rust/crates/` in Deadlock-Brain.

### Fachbefunde, die den Zuschnitt bestimmen

- **Discord erzeugt noch selbst:** Direkter Brainclient und botseitige AnswerEngine existieren parallel. Die Engine holt Brainbelege über retrieve und erzeugt ihre Endantwort über den Botprovider. Optionales dl-knowledge-Ask ist ein weiterer Generator, aber kein zusätzliches Wissensfeature. Beides gehört in die Konsolidierung. Der Serverguideadapter ist nur im gezielt gelesenen Worktree belegt; seine vollständige KI-Weiterleitung bleibt offen.
- **Twitch hat unterschiedliche Verträge:** Chattext, JSON-Judges, private Dashboarddatenkarten, Profile, Batchanalysen und Shadowvorschläge sind separate Aufgaben. tb-llm bündelt bereits Transport und Ledger, aber nicht durchgängig Tenant-/Kostenbudget. Öffentlicher Typed-Selfexplainer und Brainchat sind bestehende Brainconsumer; der private Dashboardassistent ist es nicht. Der Brainchatclient ist auf einen älteren Brain-SHA gepinnt.
- **Privatverarbeitung ist nicht allgemein erzwungen:** Discords Providerfactory und Twitchs Fireworksguard prüfen Anbieter/Modell, nicht umfassend die private Datenklasse. Nur für einzelne Wege sind stärkere Grenzen belegt: Stream-Audit hat eine harte Hostprüfung mit ausdrücklicher Ausnahme, der private 2nd-Brain-Operator bleibt ohne Modellaufruf lokal. Der allgemeine Twitch-STT-Envkonstruktor ist weniger streng als from_local_config und wird von Outreach verwendet. Das ist eine statische Vertragslücke, kein Nachweis tatsächlichen Datenabflusses. Öffentliches Wissen schützt personenbezogenen Fragefreitext nicht.
- **Keine zusätzlichen Generatoren erfinden:** Crew-Erkennung, Spielretrieval, Steam-Veröffentlichung und die untersuchten Turnier-/Website-/Relay-/Uplinkdienste sind deterministisch. Der 2nd-Brain-Operator liefert Originalausschnitte, auch mit Profil Explain. Clipanreicherung ist gebaut, aber abgeschaltet; Socialberichte sind separat angeschlossen. Patchnotes übersetzt mit sonar-pro und disable_search=true, nicht mit freier Webrecherche.

Private Dashboardkarten, Deepchat-Sessions und kanalübergreifende Lern-/Gedächtnisdaten benötigen eine eigene nachgewiesene Eigentümer-, Audience- und Egressbindung. Vorhandenes external_llm_consent allein ersetzt die verbindliche lokale Datenregel nicht. Die genaue Deepchat-Sessionbesitzprüfung und mehrere Hintergrundconsumer sind noch nicht abschließend belegt.

### Passung zu Paket G

G-Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, beobachteter HEAD `3d6890c0ef69c910173563f140e17504ea8612a4`, zusätzlich veränderlicher Arbeitsstand. `G/PLAN.md`, insbesondere C3, Abschnitt 6 und 8, legt strukturierte Providerturns, geschlossene Werkzeuge, Herkunftsprüfung, kumulative Budgets und gleiche Providerfortsetzung fest. `brain-contracts/src/tools.rs:11,18,56` enthält beim Lesen bereits ToolName, GameRules und ToolCall; `brain-contracts/src/lib.rs:512` enthält answer_turn; `brain-kernel/src/execution.rs:111,219` enthält Werkzeugloop und ToolCalls-Verarbeitung. Das ist **gebauter, ungemergter Stand**, kein nachgewiesener Livevertrag.

Ein älterer Registerabschnitt nennt noch sieben Tools und fehlendes game_rules. Der gelesene Produktstand enthält game_rules bereits. Daher die ältere Statuszeile nicht als aktuellen Implementierungsbefund übernehmen. Ebenso wenig aus dem vorhandenen Enum eine fertig integrierte oder live laufende Mechanik ableiten.

G bleibt Eigentümer der Spielrechnung, Spiegelversionen, Belege und Werkzeugrunden. Die KI-Zentralisierung dockt dort an, baut keine Konkurrenz. Öffentliche Spieltools sind keine Freigabe für DMs, Streamtranskripte, Memberzustand oder Schreibaktionen. Buildplanung bleibt lesend; Steam-Veröffentlichung ist ein anderer, freizugebender Effekt. Die spätere Nebenrepo-Rückgabe beobachtete G-HEAD `1f5ed30f578bbac4167ed40ab50e488f3615340c`; der fremde Worktree änderte sich während der Recherche. Die genannten Lesestände sind deshalb kein atomarer oder abschließender G-Snapshot.

## Zielzuschnitt und Datenhoheit

1. **Brain-Kern:** registrierter Consumer, serverseitige Fähigkeiten/Scopes, freigegebenes Providerprofil je UseCase und Datenklasse, typisierte Modellaufgabe, Validierung, Budget und interne Usage/Auditbelege. Wissenszugriff ist optional und explizit, nicht jeder Judge braucht RAG.
2. **Fachkonnektoren:** Discord-Serverfakten, Twitch-Channelkontext, Spielspiegel/G-Tools und öffentliche Patchnotesfeeds. Sie liefern minimale typisierte Daten unter geprüftem Principal. Kein beliebiges SQL, keine modellbestimmten URLs, keine frei wählbaren Guild-/Channel-/User-IDs als Rechteersatz. Patchnotesübersetzung bleibt ein gesonderter Providervertrag; im geprüften Aufruf ist Websuche ausdrücklich ausgeschaltet.
3. **Botmechanik:** Eventabonnement, Moderationsregeln, Consent, UI, Warte-/Wiederholungszustand, Nachrichtentextversand, API-Schreibaktionen und Deduplizierung bleiben lokal. Brain erzeugt Urteil oder Vorschlag; der Bot prüft und vollzieht. Die Berechtigung darf nie aus Modelltext stammen.

### Vier getrennte Datenklassen

| Klasse | Übergabe und Speicherung | Providerregel |
| --- | --- | --- |
| Öffentliches Spiel-/Dokuwissen | Bestehende versionierte Quellen, aktuelle Publikationsfreigabe und konkrete Belege. G-Spiegel als einziger Spielwerteleser. | Nur bereits freigegebene Provider; Patchnotes behält seine Ausnahme. |
| Öffentlich sichtbarer Plattformkontext | Nur tatsächlich benötigte Nachrichten/Fakten, Platform-ID und Channelbindung. Öffentlich sichtbar heißt nicht automatisch zur dauerhaften Wissensablage oder externen Analyse freigegeben. | Je UseCase die vorhandene Freigabe übernehmen, nicht aus dem Scope-Namen ableiten. |
| Private Channel-/Userkontexte | Requestgebunden, klein, getrennt nach Platform-ID/Principal/Scopes; keine globale Suche, kein geteilter Antwortcache ohne Rechtebindung. Keine automatische Wissensspeicherung aus Prompt, Ergebnis oder Chatverlauf. | Lokale Verarbeitung, sofern keine konkrete andere Freigabe besteht. Ein externer Abo-Provider wird nicht durch einen lokalen Proxy datenschutzkonform lokal. |
| Streams/Audio/Transkripte | STT und Nutzerauswertung bleiben lokal; keine Rohstreams in den öffentlichen Brainkorpus. Brain kann nur freigegebene lokale Auswertung ansteuern. | Exakten Host-/Adresscheck und bestehendes ausdrückliches Freigabeflag erhalten. Keine Warnung statt Sperre. |

`SourceVisibility` und `provider_egress` sind gute Grundlagen, aber keine vollständige Nutzer- und Tenantisolation. ConversationOwnership am gemeinsamen Bot-actor darf nicht als Schutz zwischen zwei Streamern verkauft werden. Plattformnutzer, Eigentümerchannel, Zielaudience und Zugriff müssen unabhängig vom Modell feststehen. Ein Consumer darf seinen Scope nur einschränken, niemals durch Text, Toolargument oder Rückgabe erweitern.

## Noch fehlende Verträge, ausdrücklich Vorschläge

Keine der folgenden Fähigkeiten wird als bereits vorhandene neue API behauptet. Die genaue Transportform ist erst nach Zusammenführen der fachlichen Verträge festzulegen. Keine erfundenen Endpunktnamen.

- **Typisierte Aufgaben/Ergebnisse:** vorhandene Botschemas für Moderation, Scam, LFG, Pitch/Judge und Assistenz erhalten. Versionierte geschlossene Schemas, Ablehnungs-/Timeoutzustände, optionaler Wissensbedarf, festes UseCase-Profil. Keine Zitate erzwingen, wenn ein Urteil keine Wissensantwort ist. Die geprüfte Crew-Erkennung ist deterministisch und benötigt keinen neuen Modellvertrag.
- **Private Requestkontexte:** Herkunft, Audience, Platform-ID, Tenantbindung, erlaubte Felder, Speicherverbot und lokal erzwungene Egressklasse. Bestehender Discord-Requestkontext ist die Referenz, keine pauschale Berechtigung für alle Bots.
- **Streaming und asynchrone Aufgaben:** nur dort ergänzen, wo der Consumer sie tatsächlich benötigt. Abbruch, Ablaufzeit, Jobstatus, Überlastreaktion und Usage müssen erhalten bleiben. Eine synchrone Antwortfrist ersetzt keinen Jobvertrag für mehrstufige Stream-/Socialarbeit. Ein tatsächlicher Streamingconsumer ist in den geprüften Rückgaben nicht belegt.
- **Aktionsvorschläge:** klare Trennung von freiem Text, strukturiertem Urteil und auszuführendem Effekt. Bestehende Approval-/Idempotenzzustände bleiben beim Fachdienst. request_id allein ist kein nachgewiesener Exactly-once-Vertrag für Ban, Nachricht, Titelwechsel oder Veröffentlichung.
- **Interne Betriebsbelege:** UseCase, Provider-/Konfigrevision, Requestkorrelation, Deadline, gesamte Netz-/Token-/Kostenusage, schema-validierter Zustand, Fehler und ausgeführter Effekt. Keine privaten Prompts oder Rohtexte als automatische Auditdaten. Öffentliche Antworten enthalten diese technischen Daten weiterhin nicht.

## Kritische Moderation und Ausfallverhalten

Brain-Ausfall darf Discord/Twitch nicht schutzlos oder dauerhaft blockiert machen. Nach dem Cutover gibt es keinen stillen direkten Bot-LLM-Fallback, sonst bleibt die geforderte einzige Schnittstelle nur ein Etikett. Ausfallüberbrückung heißt lokale deterministische Schutzmechanik und der bestehende sichere Fehler-/Reviewzustand, nicht ein ungefragt gewechselter Anbieter. Deterministische Sperren, bekannte Scamindikatoren, Permissionchecks und lokale Plattformmechanik laufen weiter. Ein fehlendes Modellurteil führt in den bestehenden sicheren Fehler-/Reviewzustand, nicht zu einer automatischen Sanktion oder zu einer vermeintlichen Freigabe. Neue Freitextantworten können kurz aussetzen; bans, Inviteverwaltung und consentabhängige Aktionen brauchen weiterhin ihren jeweiligen lokalen Vertrag.

Moderation erhält eine eigene Ressourcenklasse oder eine nachgewiesene Lastpriorisierung gegenüber Batchgenerierung. Sonst konkurrieren Streamanalyse/Socialjobs und Moderation im gleichen Workerbudget. Bestehende Zeit-/Tokenbudgets als Vertrag erhalten. Soweit sie bisher Konstanten sind, werden sie im späteren Bau als Konfigfelder übernommen, ohne Zahlen oder Modellfreigaben aus diesem Audit zu erfinden. Alerting nur beim Zustandswechsel, keine Nachricht pro fehlgeschlagenem Tick.

## Minimaler Weg in drei Baupaketen

| Welle | Inhalt und Grenze | Abnahme |
| --- | --- | --- |
| 1. Vertrag und öffentliche Antwortwege | Nach G-Vertragscheckpoint die bestehenden Brainclients vereinheitlichen; bestätigte öffentliche Wissensantworten über Brain. Discord/Twitch-Identitätskontexte festschreiben, gepinnte Clientversionen berücksichtigen. Discords AnswerEngine und optionales dl-knowledge-Ask noch als getrennte Generatoren erfassen. Steam-Veröffentlichung bleibt eigener deterministischer Vertrag, kein nachgewiesener Steam-Antwortconsumer. | Eine Anfrage pro tatsächlich angeschlossenem Consumer mit korrektem Release, Scopes und Herkunft; unbekannter Scope scheitert. Keine ungeprüften privaten Fragen, keine neue Funktion, kein YouTube/Forum. |
| 2. Generierung und bestehende Ausnahme | Einen begrenzten nichtkritischen Text-UseCase mit freigegebenen Eingaben migrieren, etwa Pitch- oder Titelentwurf. Routing übernimmt bestehende Modelle/Configs. Patchnotes bewahrt sonar-pro, Providerhostpin, disable_search=true, Chunking und Vollständigkeitsprüfung. Bot behält Versand/Consent/Idempotenz. Private Assistenz/History erfordert vorher Welle 3. Abgeschaltete Clipanreicherung bleibt abgeschaltet. | Parität der Schemas, Timeouts, Freigaben, Wissensbedarfe und Fehlerzustände. Kein pauschaler Modellwechsel; tatsächlicher Aktionspfad nur nach bestehender Freigabe. |
| 3. Private Urteile und lokale Analyse | Moderation/Scam/Judges und private Stream-/Channelaufgaben erst nach Tenant-/Egress-/Ausfallvertrag. Lokalen Providerpfad in Brain einbinden, ohne Stream-STT und Botsicherheit zu verschieben. Legacydirektaufrufe nach Paritätsbeleg stilllegen. | Fremder Tenant und externer Egress werden hart abgewiesen; Brain-Ausfall hält lokale Schutzmechanik aktiv; doppelte Aufträge erzeugen keine zweite Aktion; Usage zählt alle Turns. |

Für den ersten Bauauftrag reichen Welle 1 und ein begrenzter nichtkritischer UseCase aus Welle 2. Welle 3 ist ein eigener Datenschutz-/Betriebsvertrag, kein beiläufiger Ersatz der URL in `tb-llm`.

## Entscheidungen und Konflikte

**Bereits entschieden:** ein Brain; Rust; G als gemeinsame Rechnung/Werkzeugschicht; kein YouTube-/Forumbau; Patchnotes-Ausnahme nicht widerrufen; private Streamauswertung lokal; keine automatische Wissensaufnahme aus Prompts.

**Empfehlung:** Fähigkeiten und Providerfreigaben je UseCase übernehmen, nicht einen einzigen öffentlichen Answervertrag auf alle KI-Funktionen anwenden. Reine Klassifikation zählt zum KI-Inventar und erhält Brainrouting, aber nicht zwingend RAG oder Freitext. Deterministische Klassifikation, Rechnung und Sicherheitsgates bleiben ohne Modell.

**Vor späterem Bau ausdrücklich zu klären:** Soll Brain nur Modellurteile und Vorschläge liefern oder auch eine zentrale asynchrone Auftragsverwaltung besitzen? Die kleinere sichere Variante ist zunächst Urteile/Vorschläge, Effektzustände weiter lokal. Private Concierge-/Dashboarddaten erfordern einen eigenen erlaubten Daten- und lokalen Providervertrag; die aktuelle öffentliche Brainfreigabe deckt sie nicht.

**Nicht aus diesem Audit ableitbar:** vollständige Liveparität, aktive Modellwerte aller Dienste, tatsächliche Produktnutzung aller Featurepfade, Produktionslast und Kosten. Ein Sourcecall, ein gebauter Worktree und ein laufender Dienst sind drei verschiedene Belege. Diese Akte gibt keine Deployfreigabe.
