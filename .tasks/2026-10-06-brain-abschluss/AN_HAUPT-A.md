# Paket A an Hauptorchestrator

## 07.10.2026, Steuerung 05:25: A/G-Schnittstellen für Brain v2

VON_HAUPT.md und BRIEFING-G.md gelesen, Paket G a867ef50 übernommen. A schließt ausschließlich die laufende Steckbrief-Freischaltung samt G1-Abnahme als Übergang ab. Danach keine neue Spielwissen-Dokumentstrecke, kein API-Spiegeladapter und keine Rechenschicht durch A. Strukturierte Entitäten, Rechnen, Ansichten und Antwortwerkzeuge gehören G auf Es Datenbasis; Build-/Publishregel F. Discord, Twitch, Invitequelle und Zustellung bleiben A. Main-/Betriebshold und exklusiver live_strecke unverändert.

Schnittstellenbedarf an G vor gemeinsamen Sourceänderungen:

1. **Antwortvertrag:** /v1/answer samt vorhandenen Discord-/Twitch-Consumern und authentifizierter Requesterbindung erhalten. bot.public/public, Antwortstatus, Request-ID und vorhandene Ausgabegrenzen bleiben. G liefert strukturierte Werte, Version und Belege im gemeinsamen Antwortweg; A führt Request-ID bis zum Sender und belegt echte Zustellung. Kein eigener Bot-Tooldispatcher, LLM-Connector oder Build-Befehl. Erweiterungen des öffentlichen Vertrags vor Änderungen mit A abgrenzen.
2. **Eigener Invite-Status:** vorhandenen Discord-MCP-Leser als requestgebundene Quelle nutzen. Requesteridentität serverseitig binden, Werkzeug liefert nur eigenen Enum plus Zeitpunkt, keine IDs/Namen/Drittdaten an das Modell und keine dauerhafte Quellenablage. Gewöhnliche Fragen bleiben gewöhnliche Fragen. Der offene E4f-BLOCK betrifft eine Cooldown-Ausnahme vor dem vollständigen Answerer; die begrenzte Statusoperation und ihr Budget vor dem frischen Fixer mit G abstimmen, keine neue Sonderengine.
3. **Schreibgrenzen:** begonnener A-Inviteanschluss im eigenen Brainfeature berührt brain-contracts/src/invite.rs, brain-api/src/lib.rs sowie brain-serve/src/discord_live/invite_tests.rs und brain-serve/tests/process_e2e.rs. Gs Werkzeugumbau am Antwortdienst darf diese Änderungen nicht parallel überschreiben. A behält dl-brain/Discord-MCP und Bot-Sender; G meldet benötigte gemeinsame Dateien/Vertragsfelder über AN_HAUPT-G.md, dann geordnete Übergabe statt parallelem Schreiben.
4. **Übergang und Abbau:** zunächst dieselben fünf G1-Fragen und echte Consumerzustellung, später Gs v2-Fragen über denselben Eingang. Freies Server-/Betreiberwissen behält den bestehenden Textweg. Spielwissen-Dokumentpakete, ihr Veröffentlichungsweg und Sheet-Sync erst nach Gs belegtem Gleichstand abbauen. A baut dort nichts auf Vorrat; derzeit nichts gelöscht.

Dies ist die A-seitige Schnittstellenmeldung, noch keine behauptete Zustimmung oder fertige technische Schnittstelle von G. Die bisherige 05:10-Ankündigung eines eigenen A-Anschlusses an Es Spielwertetabellen ist durch die neue Zuständigkeit G ersetzt.

E3f jetzt vollständig zurück: sauberer Featurestand 19f6d49f, SHA-gebundener gpt-6.1-sol BLOCK bestätigt, kein Push/Deploy. Allgemeine Invite-Zeit-/Verfügbarkeitsfragen werden weiterhin als eigener Status umgeschrieben. A las Gate/Reviewzustand/Commit und tatsächliche Testzahlen: 214 passed/0 failed/0 ignored, aber eine Serviceprüfung gefiltert; keine ungefilterte Vollsuite behauptet. Neuer frischer E3g-Fixer noch nicht gestartet, G1 hat Vorrang. Details A/E3F-RUECKGABE.md.

A/G-Dateiabgrenzung ergänzt: Der bestätigte Invitefeaturestand berührt zusätzlich brain-api/src/http.rs, brain-contracts/Cargo.toml und src/lib.rs, src/provider_input.rs, brain-providers/src/hardening.rs, brain-serve/src/discord_live.rs sowie Cargo.lock. Gs aktuelle Akte lässt Invites/Botanschlüsse unangetastet und grenzt gemeinsame Dateien vor Bau im G-Plan ab. Keine fremden Änderungen oder neuer Dokumentweg durch A.

## 07.10.2026, Steuerung 05:10: gemeinsamer API-Spiegel, Schnittstelle vor Umbau

Übernommen: A liest aktuelle Spielwerte ausschließlich aus Es lokalem, versionsgebundenem API-Spiegel. Steckbriefe/Patch-Story bleiben am bestehenden Veröffentlichungsweg; Discord und Twitch ausschließlich /v1/answer. Keine eigenen Werteabfragen pro Frage, Kopien, Parser oder Zwischenformate. Bestehende Quellen-, Ableitungs- und Veröffentlichungsprüfungen bleiben. Die Anweisungen 05:05 ersetzen den alten Matchnachhol-/100er-Auftrag: kein Einzelmatchimport durch A, neue Reasoner-/Publishumsetzung gehört F.

Schnittstellenbedarf vor jeder Änderung gemeldet: A benötigt Es verbindlichen Lesevertrag für Entitäts-ID, client_version, deutsche Lokalisierung, Quellenrevision/Hash und Zuordnung zum aktiven Patch. Ausgangspunkte laut E/DEADLOCK-API.md:142-147 sind die bestehenden source_documents/entity_snapshots/source_record_revisions und patch_events/patch_changes. Das Tabellenmapping ist noch kein bestätigter neuer Lesevertrag. E legt den Schreibvertrag fest; A ändert keine E-/F-Tabellen oder Dateien und baut keinen zweiten Adapter daneben. Nach Vertrag dieselbe Revision für öffentliche Steckbriefe und Reasoner, fehlende historische Zuordnung ausdrücklich offen halten.

Abbau: bisherige parallele aktuelle Spielwertegewinnung aus GameTracking/Spieldateien, deadlock-data und Wiki nach belegtem API-Gleichstand ablösen. Offizielle Patchvolltexte, historische Belege und ergänzende Mechanikerklärungen nicht pauschal entfernen. Noch nichts gelöscht; genaue ersetzte Pfade und Gleichstandsbelege gehören in die jeweilige Umsetzungsrückgabe. Betriebs-/Main-Hold und live_strecke-Eigentum unverändert, G1 bleibt die Abnahme.

## 07.10.2026: finaler Releasezielstand bfda408c, Hold bleibt

Direkte Schlussübergabe übernommen: origin/main bfda408cb988722ddceadb56bca5b72e12d12731, Dokumentkontext plus D-Resolverfix SHA-identisch integriert. Laut Orchestrator Kontext 23 Prüfungen und D 97 Prüfungen sowie jeweilige Format-/Compiler-/Clippy-/Gate- und frische finale SHA-Abnahme grün. Keine neue A-Suite oder eigene Integration behauptet. live_strecke allein baut, installiert und veröffentlicht die Profile deterministisch. A macht keinen weiteren Main-Push oder Betriebseingriff, Luna 18769/Timerfence unverändert. Nach belegter Aktivierung exakt fünf G1-Fragen erneut; ENV-Feature und fremde Branches getrennt erhalten.

Invite-E4f nur auf Feature abgeschlossen, weiterhin nicht freigegeben: 2be2df16, sauber, kein Push/Deploy. Beide alten Funde bearbeitet, neuer regulärer gpt-6.1-sol BLOCK wegen Cooldown-Ausnahme für beliebige Fragen mit angehängtem „invite status“. Gate und beide Eintrittspfade direkt gelesen. Frischer E4g-Fixer erst nach vorrangiger G1-Abnahme, kein erneuter Auftrag an den alten Implementierer. Der eingefrorene bfda408c-Release ist davon unverändert. Details A/E4F-RUECKGABE.md.

## 07.10.2026, Steuerung 04:10 übernommen: ausschließlich G1-Lücke 1 zuerst

Nach Recovery aktuelle Steckbriefe/Patch-Story über vorhandene Veröffentlichung aktiv in den Korpus bringen, Aktivierung ausschließlich live_strecke, danach dieselben fünf Abnahmefragen. Twitch-Boolean und Discord-Zustellidentität direkt danach, kein Vorziehen von Builds/Site/Betreiberernte. A hält Main-/Runtime-Hold, Luna 18769 und Writerfence unverändert. Belegprosa kurz, Ergebnis answered mit Beleg zählt.

Secret-Befund ohne Wert: Der von G1 gemeldete TWITCH_ANALYTICS_DSN-Teilausschnitt steht im fehlgeschlagenen psql-Toolresult am 07.10.2026 00:32:07.616 UTC, Fundort `/home/nathanael/.claude/projects/-home-nathanael-repos-Deadlock-Brain/2c7de4c9-bac4-43ad-b91a-f8ac889f09b4/subagents/workflows/wf_d1ea7d62-e2c/agent-a305609c5ce0dce12.jsonl:144`, tool_use_id call_eb79071ae307482982a9a625d0a624f5. Nur den Fundort ermittelt, keinen Inhalt erneut ausgegeben. Geheimnisanteil/Rotation entscheidet Haupt über bestehenden Secretprozess. Keine Behebung oder Rotation durch A behauptet.

Nebenher fertig: ENV-Fix R2 als sauberer Featurecommit 8a3a921938b9318e9a76ea8d6e45172693292a83, 151 reale Tests/Format/Compiler/striktes Clippy grün, regulärer gpt-6.1-sol ALLOW gegen 75db93ef, eigene PG gestoppt. A las SHA, Status, Gate und Testresultatzeilen; regulärer Featurebackup Exit 0, kein Main/Deploy. Details A/R2-RUECKGABE.md. Blockiert die Profilabnahme nicht.

Die fünf ursprünglichen G1-Fragentexte direkt aus den Requestdateien übernommen, Wiederholungsabnahme vorbereitet in A/G1-ABNAHME.md. Neue Aufrufe erst nach belegter Aktivierung durch live_strecke, keine eigene Veröffentlichung oder öffentliche Testnachricht.

## 07.10.2026: live_strecke exklusiv, zwei D-Maincommits belegt

Neuer direkter Orchestratorauftrag zum Hold bestätigt: ausschließlich live_strecke baut, installiert, startet neu und tickt. Keine eigenen A-Main-Pushes oder Runtimeaktionen. Frisch lesend geholtes Remote-main 75db93ef91010ccfe6c3d501eb2e7107e79f3c12. Gemeldeter dokumentgebundener Ursachenfix samt atomarem Writer-/Identitätsschutz bleibt beim fremden Live-Agent, A baut ihn nicht doppelt. Luna 18769 und Timerfence bleiben unangetastet.

Herkunft der zwischenzeitlichen D-Pushes überprüft: baca936e parent 8d61a949, ausschließlich drei Testdateien; 0ade1a3d parent baca936e, zwei Reasonerdateien. Beide Commitgrenzen direkt in Git gelesen. Reguläre vorhandene gpt-6.1-sol-ALLOWs in D/REVIEW.md:3 und :19. Bestehende Prüfzahlen: K8/K9 34 bestanden/0 ignoriert, striktes Clippy an unverändertem release_port-Befund rot; K10 Vollsuite 284/6 gegen unveränderte 279/6, fünf eigene Prüfungen und striktes Reasoner-Clippy grün. Kein pauschaler grüner Gesamtlauf oder neue A-Suite behauptet. Voller Belegbericht A/D-HERKUNFT-20261007.md.

D dokumentiert die Abweichung selbst: beide Main-Pushes vor Entdeckung des in A-Akten veröffentlichten Holds, eigener Releasebau um 03:32 gestoppt. Aktueller D-Hold und live_strecke-Eigentum am 03:52 bestätigt. Neuer enger playstyle-Resolverfix wird von D auf Root-Auftrag getrennt auf Feature von 75db93ef vorbereitet, kein paralleler A-Umbau. A-R2 hält den eigenen ENV-Fix ausschließlich auf Feature. E4-Invitequelle jetzt mit zwei bestätigten Gatebefunden an frischen Fixer A-E4f, kein Main/Deploy.

## 07.10.2026: Recovery freigegeben, Twitch-Bestandsgrenze exakt geklärt

Gemeldete Hauptfreigabe übernommen: einzig der fremde Live-Agent führt offiziellen brain-release rollback auf fde910f6a0199c00f44083e73fc8f4c5e4f80b86 aus. Zwei identische gültige Format-2-Rootmanifeste und 32 unabhängige Artefakthashes wurden von der Gegenseite gemeldet, nicht von A neu gemessen. 8d-Enrich-/CLI-/ENV-Änderungen bleiben unaktiviert. Keine eigenen Main-Pushes oder Runtimeaktionen, Luna/Writerfence unverändert. ENV-Fixer A-R2 läuft ausschließlich auf eigenem Feature, Start 8d61a949.

G2 read-only: Bestehendes twitch-bot/Kanal twitch mit genau bot.public/public benötigt ausschließlich entity_profile_model_context=true. Kein neuer Reader oder breiterer Datenzugang. Gemessener Korpus hat noch null veröffentlichte abgeleitete Profilquellen. Normale bestehende ConfigWriter-/CAS-Mechanik identifiziert, nur Live-Agent darf sie verwenden. Standardaktivierung verschiebt gekoppelte Pins, boolesches Update allein erhält sie. Details A/G2-RUECKGABE.md. Keine Konfigmutation durch A.

Gewünschte direkte Zustellung an 3fcd8f71 scheiterte: read meldete session None, t3-harness send Kein aktiver lokaler Thread mit dieser ID vorhanden. Kein weiterer Routingversuch oder neuer Thread, keine behauptete Direktzustellung. Gemeinsamer Hold steht in dieser Hauptakte und A/RELEASEFENSTER.md; vom Haupt für Paket D und weitere Brain-Main-Merges absichern.

## Dringend, 07.10.2026: gemeinsames Releasefenster für alle Brain-Main-Merges absichern

Neu ausdrücklich vereinbart: Der Live-Agent des parallelen Discord-Brain-Fix ist einziger Deployer und Tickverantwortlicher. A macht keine weiteren Main-Pushes, Installationen, Neustarts oder Ticks. Bitte Paket D und weitere Brain-Main-Merges ebenfalls auf diesen Hold binden; Main erst nach gemeinsam beschlossenem endgültigem Zielstand fortschreiben. Produktive Luna-Konfiguration und Writerfence bleiben unangetastet. Kein weiterer Standardbuild, solange der ENV-Testfund offen ist. Eigenen engsten Fix bereitet A nur als geprüftes Feature vor.

Eigene Auslieferungscontroller R1 und D1 angehalten, Quellen und warme Targets erhalten. Brain-Bundle `/home/nathanael/.local/state/brain-a-release-20261007-8d61a949` hat noch kein Manifest, nur target. PID 3332465 existiert nicht mehr; null eigene Releaseprozesse. Kein Install/Restart/Tick durch A aus diesem Versuch. Stand 8d61a949 enthält fde910f6 vollständig. Die drei fertigen fde-Builds der Gegenseite wurden korrekt nicht stale installiert, kein Revert/Gatebypass.

Timeout-/Querybefund laut parallelem Orchestrator: statement_timeout Backend 2557102 am 06.10.2026 22:43:06 GMT. fde910f6 projiziert zehn Identitätsprüfungen über GeneratedHeaders, komplette JSON-Gleichheit bleibt. Zuletzt gemeldet: beide produktiven Zeiger 10ebbb20, Luna 127.0.0.1:18769 gesund, ConfigWriter-Luna/dl-bot-Profilflag true, Timer/Writer gefencet. A hat diese fremde Messung nicht als eigene wiederholt. Echte Mirage-/Warden-/Unknown-Antworten und öffentliche Profilquittungen bleiben die Abschlussbedingungen. Vollständige Übergabe: `A/RELEASEFENSTER.md`.

## 07.10.2026: geprüfter Brain-Main veröffentlicht, normale Auslieferung gestartet

Kombinierter Stand vollständig grün: 126 reale Tests, Format, Compiler und striktes Clippy, 1325 Dateien mit Git identisch. Nur die Merge-Attribution ergänzt, Gitbaum unverändert. Regulärer Main-Push `8d61a949` Exit 0. Saubere eigene detached Releasequelle angelegt, vorhandener brain-release plan Exit 0, A-R1 führt normale Build-/Verify-/Install-/Livekette samt priorisiertem Profilpfad aus. Kein ungeprüfter Branch aus dem Stop-Hook übernommen; schmutziger Kanon und laufender Invite-Fixer bleiben geschützt.

A-G1 bestätigt die konkrete Grundding-Lücke: 62 Dokumentquellen, keine aktive Spielprofil-/Patchnotesquelle. Steam-Wartung beantwortet der Backendweg in 4,961 Sekunden mit Beleg, Spiel-/Patchfragen insufficient_evidence. Das ist noch keine frische Discord-/Twitch-Zustellungsabnahme. Zusätzlich ist die bestehende Twitch-Freigabe entity_profile_model_context derzeit false; der vorhandene Registrierungsweg wird geprüft, ohne Rohdatenfreigabe oder Modellwechsel.

Sicherheitshinweis aus A-G1: Ein fehlgeschlagener lokaler DB-Verbindungsaufruf gab einen Anteil von TWITCH_ANALYTICS_DSN im Tool-Output aus. Kein Wert wird hier wiederholt. Betroffenen Zugang über den bestehenden Secret-Prozess prüfen und bei tatsächlichem Geheimnisanteil rotieren; Scope und alle betroffenen Consumer vorher feststellen. Das ist ein potenzieller Zugangsleckbefund, kein belegter Kontozugriff durch Dritte.

## 07.10.2026: konkreter Grundding-Blocker und frischer Invite-Fixer

Profil-Liveabschluss auf noch laufendem 10ebbb20: normaler Tick Exit 0, aber ENTITY_PROFILE_REFRESH_FAILED. Null Ableitungsquittungen und null Git-Steckbriefdokumente, drei echte interne Abnahmefragen insufficient_evidence. Main fde910f6 enthält bereits den zusätzlichen Veröffentlichungs-/Fehlerklassifikationsfix; A führt dessen gemeinsame Auslieferung mit dem eigenen integrierten Kandidaten weiter. Keine Datenbankzustände von Hand korrigiert oder Stage 2 vorgezogen.

Invite-Brainfeature d8a0e727 ist durch gpt-6.1-sol blockiert, nicht ausgeliefert: Erkennung zieht gewöhnliche FPS-/Verfahrensfragen in den Status-Skill und übersieht „Bin ich eingeladen?“. Frischer Fixer A-E3f beauftragt, rote bestehende Prüfungen eingeschlossen. Allgemeine Brainfragen behalten Vorrang. Site cac87635 nur sauber auf Feature gesichert und pausiert, kein Gate/Main/Deploy. A-G1 prüft die vorhandenen echten Discord-/Twitchwege; A-D1 baut den bereits verifizierten Main-Consumer im eigenen Worktree über den belegten vorhandenen Releaseweg.

## 07.10.2026: Grundding hat Vorrang, danach Build-Skill

Neue Reihenfolge übernommen: zeitnahe gemeinsame Brainantworten in Discord-DM, Erwähnung, Hilfekanal und Twitch zuerst, mit aktueller Patchbindung, Steckbriefen/Patch-Story und Serverwissen. Livebeweis an echten Fragen bleibt die Abnahme. Site-/Kommentarrest und andere Nebenteile warten. Danach Build-Skill über vorhandenen Reasoner, zusätzliche vorhandene Matchdaten prüfen, Publish-Grenzen erhalten und echte Build-ID melden. D-Ernte nur über die Akte koordinieren, A behält Vorrang bei Überschneidung.

Discord-URLfix `e18f5222` soeben regulär auf Main gepusht, noch kein eigener Botdeploy oder Zustellungsbeweis. Brainintegration nach neuem Main fde910f6 jetzt `1e925d5f`, regulärer Gate ALLOW; warme kombinierte Nachprüfung läuft. Der Steam-Wartungsfakt ist im gemeinsamen Standardantwortweg bereits live belegt, die vollständige Grundding-Abnahme noch offen.

## 07.10.2026: Steam-Wartungswissen live im gemeinsamen Brain

Punkt 1 abgeschlossen: vorhandene Steam-Integrationsseite auf Docs-Main `6fa4ca37` ergänzt, normal als geprüfte Dokumentrevision 2 aufgenommen und über den bestehenden ConfigWriter aktiviert. Standardantwortweg `Explain` antwortet mit Beleg: nachts Dienstag auf Mittwoch, meist 10 bis 20 Minuten. Alle 61 anderen Quellenbindungen unverändert. Explizites `Fact` meldet noch insufficient_evidence, daher keine Abnahme für jedes Antwortprofil. Kein C9-Slot, Produktcode, Modell oder Datenumfang geändert, keine öffentliche Testnachricht. Eigener Docs-Branch/Worktree nach Ancestry-Beweis entfernt. Punkt 2 bleibt nach Invite-Skill und Steckbriefen. Belege: `A/K1B-RUECKGABE.md`, `/tmp/brain-a-steam-wartung-final-proof-20261007/`.

Eigene Brainintegration `dcff5d9c` jetzt vollständig geprüft: Compiler und striktes Clippy grün, 126 Tests wirklich bestanden, private PG gestoppt. Discordconsumer `e18f5222`: 38 echte Bibliothekstests und vollständiger dl-bot-Compiler grün; striktes Clippy scheitert aktuell an dl-central-db/platform_connections.rs:30, normaler vollständiger Clippy-Lauf wird getrennt ausgewertet. Main-/Binaryabschluss der beiden Kandidaten weiter offen.

## 07.10.2026: Steam-Fakt braucht noch die echte Dokumentbindung

Der Fakt ist bisher nur gesichert, nicht im Brain. Der vorhandene Shared-Importer für Wiki/Gamefile und der enge !commands-Import sind dafür ungeeignet. Der allgemeine überprüfte Dokumentweg ist vorhanden und wird verwendet: A-K1b ergänzt einen normalen Git-Wissensstand mit echter Quellenpolitik und normaler Aktivierung. Kein zweiter Ingest, keine falsche Wiki-Herkunft. Die Ernteautomatik bleibt nach Invite-Skill und Steckbriefen. Beleg und Auftrag in `A/BETREIBERWISSEN.md`, `A/BRIEFING-K1B.md`.

## 07.10.2026: aktueller Main live, Wartung noch ohne Abschluss

Lesend belegt: Serve und beide Releasezeiger jetzt Main 10ebbb20, Manifest-/Binaryhash passend, Kern-/Profilschema kompatibel. A hat diesen Deploy nicht ausgeführt. Eigener regulärer Wartungsstart 00:26:39 wurde 00:28:58 durch TERM abgebrochen; kein abgeschlossener Tick. Ursache noch unbelegt, kein Neustartloop. Profilstand 378 Entitäten/64342 Fakten, null Ableitungsquittungen. Neueste Docs-Reviews melden JEV_RESPONSE_INVALID. Der Gitfix ist live, die Steckbriefabnahme weiterhin offen. Discordintegration e18f5222 erhält B-Invitefix, Kombinationsgate ALLOW; vier eigene aktivierte DB-Testfehler ohne Test-DSN werden durch A-V2 im privaten Harness nachgeprüft. Details `A/BETRIEB.md` und `A/BRIEFING-V2.md`.

## 07.10.2026: Betreiberwissen übernommen

Steam-Wartung wird sofort über den vorhandenen kuratierten Wissensweg aufgenommen, mit Betreiberquelle und ohne Behauptung eines aktuellen Ausfalls. Die dauerhafte Ernte von Betreiberantworten wird nach Invite-Skill und Steckbriefen eingeordnet. Zuerst vorhandene D2-/Community-Brücke prüfen, keinen zweiten Ingest bauen. Nur bestätigte Betreiberantworten als Kandidaten, fremde Nachrichten nicht als Fakten und Rohtexte intern. Aufnahme und Live-Abnahme werden gesondert belegt.

## 07.10.2026: Ein-Brain-Inventur fertig, zwei Kandidaten mit ALLOW

`A/EIN-BRAIN.md` enthält die belegte Antwortpfadkarte. Minimaler Invite-Skill wird durch A-E3/E4 am gemeinsamen Consumer und vorhandenen MCP-Lesepfad gebaut; Site-Port A-F3b läuft. Eigene Kandidaten remote gesichert: Brain `dcff5d9c` mit vorhandenem Main-Materialisierungsfix und Sheet-Exit-Fix, Discord `b29a55a4` mit behobener URL-Rekombination. Beide regulären Gates nach je einem technischen Wiederanlauf ALLOW durch gpt-6.1-sol. Keine Gateumgehung. A-V1 prüft den kombinierten Brain-Stand vor Main-/Releaseabschluss; noch kein neuer Deploy oder Livebeweis. Details und bekannte rote Zusatzläufe in `A/F2C-RUECKGABE.md`, `A/F4-RUECKGABE.md`, `A/REGISTER.md`.

## 06.10.2026: Git-Ursache geklärt, vorhandenen Mainfix ausliefern

A-F1 belegt einen kalten GameTracking-Teilklon mit sechs fehlenden Blobs. Main enthält den Materialisierungsfix `37cfc6c6` bereits, frisch geprüftes Main `10ebbb20`; live weiterhin e56e075d. Kein zweiter Fix gebaut. Originalimport mit 112 Dokumenten und 311.557 Fakten bestanden, regulärer Tick/Steckbriefe/Antwortabnahme noch offen. A führt die aktuelle Main-Auslieferung und eigene Branchabschlüsse fort. Der Stop-Hook nennt auch den ausdrücklich geschützten fremden Kanon samt aktiven Worker-WIPs; daraus folgt kein Zurücksetzen, Löschen oder ungeprüfter Merge dieser Stände.

## 06.10.2026: Discord-Gatebefund bearbeitet, kein Deploy

Discord-Feature `c4508fb3` ist remote gesichert. Regulärer Gate gpt-6.1-sol blockiert URL-Rekombination beim Linkentfernen; frischer Fixer A-F2c arbeitet nur diesen Befund ab. 36 Bibliothekstests bestanden, strenge Clippy-Baseline auf beiden Ständen mit vier Warnungen rot. Kein Main-Merge oder Deploy. Eigener Aktencheckpoint `0703e9f0` auf `feat/brain-a-abschluss-20261006` gepusht. Neue Invite-Freigabe steht in `A/INVITE-VERTRAG.md`.

## 06.10.2026: Invite-Kopplung und Datenschutz geklärt

Nachtrag aus `VON_HAUPT.md` und direkte Rückgabe übernommen: B entfernt die fehlerhafte verspätete Lounge-Antwort sofort und liefert unabhängig von A aus. Der frühere Hinweis 23:18 ist damit für diese Fehlerantwort erledigt. Invite-Skill: ausschließlich eigener Status als Enum plus Zeitpunkt, ohne Steam-IDs, Namen oder Daten Dritter, über den bestehenden Antwortweg und Provider ausdrücklich zulässig. Funktionierende gewollte Antworten bleiben bis zum live belegten Brain-Ersatz erhalten. A setzt EIN-BRAIN.md und Steckbriefe fort.

## 06.10.2026, 23:18 CEST: Kopplung mit Invite-Mechanik

Der gemeinsame Invite-Antwortersatz ist noch nicht live. Bitte B-Mechanikfix und Antwortabschaltung getrennt behandeln: die bisherige sichtbare Antwort erst nach A-Livebeleg ausliefern lassen oder abschalten, wie in `VON_HAUPT.md` gefordert. A-E1/E2 ermitteln aktuell Antwortpfade, Statusquelle und Rechte-/Providervertrag; die Freigabe für Spielwerte gilt nicht automatisch für personenbezogene Invite-Daten. Kein neuer Bottext, Connector oder eigenmächtiger Modellwechsel.

## 06.10.2026, 23:10 CEST: Entscheidung „ein Brain“ angenommen

Punkt 1 und 3 sind verbindlich übernommen: `A/EIN-BRAIN.md` bekommt neben den Steckbriefen Priorität 1. Der Invite-Status wird als lesender Brain-Skill an den bestehenden Discord-Antwortweg angeschlossen. Kein zweiter Antwortweg und keine eigenen Invite-Texte im Bot. Die Mechanik und ihre Fixes bleiben bei B; A übernimmt ausschließlich Wissensquelle und gemeinsame Antwort. Bestehendes sichtbares Verhalten erst nach live belegtem Brain-Ersatz entfernen. Weitere Antwortpfade werden anhand des Nutzens umgezogen.

## 06.10.2026: Branchentscheidungen für C eingetragen

`C/OFFEN.md`: alle 62 alten SHA-Zeilen entschieden, 44 erhalten und 18 verwerfen mit Main-/Blob-/Eigenanteilsbelegen. Die 21 neu gesicherten WIPs bleiben ausdrücklich erhalten, ohne ungeprüfte Produktübernahme. G5-/Replay-/Q/Z-Sperren, heutige Arbeit, lokale Daten und Abo-Archive bleiben geschützt. A hat nichts gelöscht. Site-Fortsetzung läuft mit dem konkret nötigen PostgreSQL-Migrationseigentum; tatsächliche öffentliche `/brain/site/` war bereits erreichbar, fehlende Steckbriefseiten sind die gesonderte Lücke.

## 06.10.2026, 22:25 CEST: erste Bestandsaufnahme abgeschlossen

`A/STAND.md` enthält den belegten Soll/Ist-Abgleich für alle geforderten Bereiche. Main `d6131cc`, live `e56e075`. Spielwissen teilweise gespeichert, aber Git-Dienstfehler, keine Quittungen/HTML und nicht im aktiven Antwortkorpus. Vorhandener Discord-Fix `b08d9366` ist noch nicht gemergt. Twitch-Zustellung und enger Serverzugang funktionieren, aktuelle Wissens-/Guide-Abnahme offen. Reasonermechanik überwiegend vorhanden; reguläres Publish derzeit durch Low und 47 statt 100 Nach-Patch-Matches gesperrt. Kein Gesamtfertigbeleg.

Fertigbau beginnt mit getrennten eigenen Worktrees für den konkreten Profil-/Git-Laufzeitpfad und die Übernahme des Discord-Fixes. Game Invites bleiben bei B. `C/OFFEN.md` wird auf bestehende Arbeit geprüft. Rohbelege: `A/INVENTUR-*.md`, `A/BASELINE.md`; Compiler, Clippy und Formatbaseline des aktuellen Brain-Kerns grün, keine neuen Tests oder Produktivänderungen in der Inventur.
