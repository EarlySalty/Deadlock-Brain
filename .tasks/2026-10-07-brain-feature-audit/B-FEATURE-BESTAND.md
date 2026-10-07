# Audit B: Featurewünsche und heutiger Bestand

Stand: 07.10.2026. Reine Recherche im Hauptbaum und gezielt gelesenen fremden Worktrees. Kein Bau, kein Versand, keine Modellprobe und keine Betriebsänderung.

## Fachurteil

Grafiken und kleine Webseiten haben vorhandene Renderer und Veröffentlichungswege als Grundlage. Es fehlt der nachgewiesene Anschluss von Gs versionsgebundenen Zahlen an eine freigegebene Vergleichsgrafik und einen vom Bot zustellbaren Artefaktlink. Freies Modell-HTML ist dafür nicht erforderlich.

Der Serverguide-MVP nutzt bereits den gemeinsamen Brain-Consumer. Der Vollguide mit Persona, persönlicher Erinnerung und stärkerer Zustellkontrolle liegt dagegen separat vor. Patenvermittlung, Übernahme, Erinnerungsstufen und Leitfaden sind überwiegend auf Bots-main vorhanden, derzeit aber in der Betriebsdatei ausgeschaltet. Ein dauerhafter Nein-Entscheid für das Patenangebot fehlt im geprüften Knopfzweig. Ein kompletter Neubau wäre deshalb falsch, eine pauschale Reaktivierung ebenso.

## Quellen- und Statusgrenze

Abkürzungen: Brain, Bots, Docs, Second und Website meinen `/home/nathanael/repos/Deadlock-Brain`, `Deadlock-Bots`, `Deadlock-Docs`, `Deadlock-2nd-Brain` und `Website`. Twitch meint `/home/nathanael/repos/Deadlock-Twitch-Bot`. Pfadzeilen zu main stammen aus `git show origin/main:<Pfad>`, nicht aus dem abweichenden Hauptbaum.

| Quelle | Geprüftes main | Abweichender Arbeitsstand |
| --- | --- | --- |
| Brain | `9711cb630aebacfe959ed4783595b071f479be36` | Hauptbaum `2734c2da`, fremde Änderungen |
| Bots | `e18f522226f8e2dec5a1c03fe97c2aba3200c8d1` | Hauptbaum `dbda52b8`, fremde Änderungen |
| Docs | `6fa4ca3758d6f259cc0ca128e5350834fe1b4cfe` | Hauptbaum `400231af`, abweichend |
| Second / Website | `28b4c078c7ccde8d0708f88f5bf1cc51b01a9eff` / `dbd2b347014dc733d926c6998d1c03681d61ab15` | HEAD jeweils gleich main |
| Twitch | `e0b0dbaf662d7680c4ceaa210bf15f1443693cd8` | Hauptbaum `d8284816`, abweichend |

Alle sechs Mainrefs wurden durch B rein lesend mit `git ls-remote` bestätigt. Kein Fetch und keine Refänderung. G wurde bei HEAD `3d6890c0ef69c910173563f140e17504ea8612a4` gelesen; sein Worktree bleibt parallel in Arbeit. Vollständige Metadaten: `b/ARCHITEKTUR-NACHWEISE.md`.

„Auf main“ bezeichnet Codebestand. „WIP“ bezeichnet vorhandene, nicht integrierte Arbeit. „Erreichbar“ braucht einen heutigen sicheren Betriebs- oder HTTP-Beleg. Ein alter Livebericht und ein laufender Prozess sind kein heutiger Ende-zu-Ende-Funktionsbeweis. Aktuelle persönliche Profile, private DMs und Mitgliederdaten wurden nicht inventarisiert.

## 1. Grafiken und kleine Webseiten

### Tatsächliche Wünsche

| Wunsch und Beleg | Heutiges Urteil |
| --- | --- |
| Grafiken und kleine Webseiten aus dem Brain; bestehende Wünsche und v2-Anschluss prüfen. `AUFTRAG.md:7-15` dieser Akte. | Aktueller Planungswunsch, keine Baufreigabe. |
| Steckbrief je Held, Fähigkeit und Item mit aktuellen Werten und Patch-Story; Brain-Dokument und HTML auf `/brain` aus der DB. Brain `.tasks/2026-10-03-brain-fertigstellung/UEBERGABE-KOPF-GROK.md:16`. | Konkreter älterer Wunsch, Rust-Renderer bereits vorhanden. |
| Heldentabelle mit Grund-DPS, Magazinschaden, HP, Wachstum über Boons und Überholpunkten im Heldenvergleich. Brain `.tasks/2026-10-06-brain-abschluss/VON_HAUPT.md`, Entscheidung 06:45; G `PLAN.md:9-17`, `tools.rs:256-300`. | Laufende G-Arbeit. Vergleichsgrafik ist eine sinnvolle Ansicht dieser verlangten Zahlen, nicht eine neue Rechnung. |
| Grafiken und Webseiten separat bauen; G liefert strukturierte Zahlenreihen. G `PLAN.md:17`; aktuelles Brain `VON_HAUPT.md:9`. | Die ältere niedrige Roadmappriorität wurde korrigiert. G baut weder Renderer noch Grafikroute noch Roadmapeintrag. |

Ein allgemeiner frei programmierender Websitegenerator, frei erfundene Itemdiagramme oder automatisch veröffentlichte personalisierte Seiten sind durch diese Belege nicht beauftragt. Builddarstellung kann Fs Ergebnisse später anzeigen, darf dessen Planung und Veröffentlichung nicht doppeln.

### Vorhandene Komponenten

| Baustein mit Quellbeleg | Status, Erreichbarkeit und Lücke |
| --- | --- |
| Brain `rust/crates/brain-maintenance/src/entity_profile_render.rs:16-29,31-87,131-162,277-341` @ `9711cb63`: `RenderedEntityProfile`, feste HTML-Abschnitte, Tabellen, Patch-Story und Quellenlücken; Pfad `site/entities/{hero|ability|item}/{hex(entity_key)}.html`. | Rust auf main. Texte werden maskiert. Kein allgemeiner Vergleichsrenderer. Hier kein heutiger GET für eine konkrete Entitätsseite und kein vollständiger aktueller Profilkorpus bewiesen. |
| Gleiche Datei `:346-401`; `integration/entity_profiles.rs:243-301,311-379,1067-1131`: Herkunft/Freigabe und Materialisierung. | Öffentliche Werte sind an GameFile-Beleg, Gitrevision, öffentliche Sicht, leere erforderliche Scopes und `publication_allowed` gebunden. Wiki-Belege bleiben intern. Diese Grenzen für neue Ansichten erhalten. |
| Brain `brain-maintenance/src/html.rs:9-37,108-214,293-329,392`; `dbrain-retrieval/src/html_projection.rs:1-28,163-187`. | HTML-/SVG-Prüfung und Projektion vorhanden. Keine pauschale Sicherheitsfreigabe für beliebiges generiertes HTML/JS. |
| Build-Korpus unter `/home/nathanael/Documents/deadlock-build-corpus/site/`, HEAD `1c2743fa`; `server.py:8-10,23-50`, `app.js:4,25-40,55-76,98,123,153`. | Öffentliche Seite heute HTTP 200. Laufender Brain-Site-Prozess ist Python. Rust-Port im A-Worktree `/home/nathanael/.worktrees/brain-a-site-20261006`, HEAD `cac87635`, `deadlock-brain-site.rs:11-25`, `site/assets.rs:18-25,45-103`: vorhanden, nicht als ausgeliefert belegt. Keine produktive Erweiterung des Pythonpfads empfehlen. |
| Website `dl-landing/src/charts.js:18-32,58-104,155-190` und `dl-patch/src/patch.js:7-8,88-169,336-382,414-635` @ `dbd2b347`. | Gemeinsame SVG-/Farben-/Tabellen-/Tooltipbausteine und Patch-Darstellung existieren. `/patch/` heute HTTP 200. Gestaltung wiederverwenden, neue fachliche Berechnung oder Renderer gemäß Rust-Regel bauen. Dynamische Patch-API nicht abgenommen. |

Heutige öffentliche GET-Belege des Grafikworkers:

- `https://deutsche-deadlock-community.de/brain/site/index.html`: HTTP 200, Titel `Deadlock Build-Korpus`, HTML, CSP und `nosniff`. Kein `Cache-Control` und kein `ETag` im beobachteten Headerbestand. `Last-Modified` vom 06.07.2026 beweist keine aktuelle Spielversion.
- `https://deutsche-deadlock-community.de/patch/`: HTTP 200, Titel `Patch Timeline · Deutsche Deadlock Community`, CSP, `nosniff`, `ETag` und `Last-Modified`. Dies beweist die vorhandene Seite, nicht die Richtigkeit aller dynamischen Daten.

### Auslieferung und Sicherheit

Discord-main entfernt HTTP-Links und begrenzt Braintexte, Bots `dl-brain/src/brain_api.rs:138-163,174-225`. Der referenzierte Botsrelease `e1f11614` verwirft dieselben Antworten noch vollständig, `:121-131`. B hat diese Quellunterschiede unabhängig verglichen. Twitch `rust/bin/tb-bot/src/brain_chat_wiring.rs:438-467,696-733` @ `e0b0dbaf` entfernt Markdown-/linkähnliche Wörter und begrenzt Texte auf 450 Zeichen. Ein Freitextlink ist deshalb keine funktionierende neue Auslieferungsstrecke.

Erforderlich ist ein serverseitig geprüfter Artefakt-/Linkvertrag mit Botbutton oder passendem separatem Publisher. Bestehende Discord-Ausgabe- und Dashboardbausteine: Bots `modglue.rs:559-589,828-852`, `dl-dashboard/src/brain.rs:617-666`; Twitch Brain-Lab `tb-dashboard-api/src/handlers/brain_lab.rs:68-98`. Diese UI-Bausteine belegen noch keinen Grafikpublish.

Risiken und klare Grenzen:

- Legacy-Korpus `site/app.js:25-27` lädt Markdown über `marked.parse` direkt in `innerHTML`; in dieser Funktion ist keine zusätzliche Bereinigung erkennbar. Die CSP ist kein Ersatz für einen engen Inhaltsvertrag. Keine aktuelle Ausnutzung behauptet und kein Angriffstest ausgeführt.
- Öffentliches Gameplay und private Guide-/Kontaktinformationen dürfen nicht dieselbe Veröffentlichungsschiene erhalten. Sichtbarkeit und Quellenrechte vor Veröffentlichung prüfen; Löschung/Widerruf muss auch Artefakte und Cache betreffen.
- Vorschau und Veröffentlichung getrennt halten. Modelltext darf keine Zieladresse, Rechte oder ausführbares Layout bestimmen. URL aus kontrollierter Basis und geprüfter Artefakt-ID bilden.
- Rendererrevision, Clientversion, Mechanikrevision, Szenario und Quellenfreigabe an Artefakt und Cache binden. Die heutige vollständige Lebensdauer-/Widerrufs-/Cachingstrecke ist nicht nachgewiesen.

Empfehlung: ein festes Rust-HTML-/SVG-Template für eine kleine Heldenvergleichsansicht auf Gs belegten Ergebnissen. Kein freies HTML/JS, kein zweiter Rechner, kein Bildmodell. Details und vollständiges Quellenregister: `b/GRAFIK-WEB.md`.

## 2. Vollständiger Serverguide

### Auftrag, alter Stand und bereits vorhandener MVP

Der vollständige alte Auftrag wurde über das Handoff, Orchestrierung, Producer-Prüfmatrix, Abgrenzung und Abnahme gelesen. Hauptquelle: `/home/nathanael/Documents/.tasks/2026-10-03-serverguide/HANDOFF-OFFEN-2026-10-03.md:7-11,51-67,104-127`. Der engere spätere Auftrag grenzt Profile, Persona-Ausbau, Altmitgliederbegrüßung, Serien, Grafiken und Paten ausdrücklich aus: Brain `welle1/serverguide-mvp/BRIEFING.md:11-23`, `welle1/VON_HAUPT.md:123,146`.

Erhaltener Botsworktree `/home/nathanael/.worktrees/serverguide-deploy-20261003/Deadlock-Bots` bei `7c2b0a086fb6d9426890a13955496e83e2405691`. Der dort früher genannte Brain-Unterworktree fehlt; das Gitobjekt `1d820bc22a5e844144a06832caca8d18f2c55c13` ist im Brainrepo erhalten und wurde gezielt gelesen. Kein Neubau aus einem fehlenden Verzeichnis abgeleitet.

| Teil, Wunsch und Codequelle | Status, Schalter und konkreter Blocker |
| --- | --- |
| Ein gemeinsamer Guide: Brain `brain-serve/src/guide.rs:32-86` @ `1d820bc2`, Bots `serverguide.rs` @ `7c2b0a08`; MVP `main.rs:1059-1084`, `modglue.rs:645-760` @ `e18f5222`. | Vollguide mit Turn-/Snapshot-/Zustellbestätigungsrouten gebaut, unintegriert. MVP benutzt gemeinsamen Brain-Consumer; `brain_command_enabled` standardmäßig aus, aktueller Laufzeitwert nicht geprüft. Kein zweiter Assistent nötig. |
| Eigene DMs, Erwähnung und gebundene Folgefrage: Bots `modglue.rs:334-357,369-392,455-478,670-760`. | Auf main, Rechte vor Anfrage und Zustellung. Heutige DM-/Erwähnungsantwort nicht getestet. Bekannter Consumer-Auslieferungsrückstand liegt bei Abschluss-A. |
| Proaktive Hilfe: `modglue.rs:395-433,616-636,691-760`. | MVP nur in Guild `1289721245281292288`, Kanal `1426220702054355077`, bei frischer menschlicher Hilfsfrage; keine fremde Erwähnung/Replyfrage. Außerhalb keine allgemeine Kanalproaktivität. Heute aktiv und zugestellt unbekannt. |
| Öffentliche aktuelle Serverfakten: Brain `discord_live.rs:16-38,97-140,251-325,344-381,459` @ `bfda408c`, pfadgleich zu main; Bots `mcp.rs:177-190`. | Gemeinsamer `public_server_facts`-Baustein vorhanden, an Anfrage und geprüfte Sicht gebunden, höchstens 60 Sekunden gültig. Kein Neuerfinden von Angeboten nötig. Heute keine neue Fakten-/Antwortprobe. |
| Persona: Brain `brain-serve/src/guide.rs:29` @ `1d820bc2`, Handoff:122. | Warm, geduldig, erkennbarer Bot; keine erfundenen Beziehungen/Verfügbarkeiten, Chat als Voicealternative, private Erinnerung nur in eigenen DMs. Geschrieben, nicht im MVP. Eigenname und repräsentative Abnahme offen. |
| Dauerhafte persönliche Erinnerung: Brain `brain-contracts/src/guide.rs:128-154,193-221,829-860` @ `1d820bc2`. | Vorläufige Felder und Auskunft/Korrektur/Vergessen/Erinnerung aus vorhanden, unintegriert. Ohne vereinbarte `memory_retention_seconds` keine neue dauerhafte Speicherung. Finale Felder, Herkunft, Frist, Lösch-/Wiederanlageverhalten offen. Spiel-Entityprofile ersetzen diesen Wunsch nicht. |
| Bestehende Mitglieder begrüßen: Bots `concierge.rs:284-327,404-443`; Handoff:106,126. | T0-/Joinbausteine existieren. Keine Freigabe für Altmitglieder-Massenbegrüßung und kein solcher MVP-Lauf. Zielgruppe, Anlass und Pilotumfang offen. |
| Kontaktserien: `concierge.rs:446-474,3269,5586,6108-6145`. | T2/T7, Kontaktzähler und Sperren auf main. Betriebs-TOML derzeit `concierge_enabled=false`, `concierge_proactive=false`; keine laufende Wirkung aus der Datei allein abgeleitet. Serien fachlich nicht freigegeben. Vollguide verbietet eigenmächtige Serien. |
| Nein, Stopp, Vergessen: `concierge.rs:431,446-449,990-1040`; Vollguide `docs/serverguide/guide-vertrag.md:23-35`. | Sperren, Privacylock, Epochen/Löschmarker und leere Wiedereinwilligung vorhanden. Vollständiger neuer Ende-zu-Ende-Vertrag nicht abgenommen. Profilkontrolle ist kein Ersatz für einen fehlenden Paten-Nein-Entscheid. |
| Wissenspflege: Bots-WIP `serverguide.rs:65-69,107-114,419-499`; Brain-WIP `guide-vertrag.md:17,21`. | Öffentliche Quellen-/Regelallowlists, Revision/Löschung und strukturierter Kern gebaut. Unintegriert; laufende Pflege nicht bewiesen. Bestehendes Brain-maintenance und öffentliche Livefakten verwenden. |
| Pin-/Wiederfinden-Grafiken: Handoff:121; vorhandene Botsassets `assets/welcome-banners/`, `serversync/rang_guide_publish.rs`. | Verlangte zwei dunkle Goldgrafiken mit Roboter/echter Discordoberfläche, Vorschau und Textalternative nicht belegt. Allgemeine Rang-/Willkommensbilder erledigen diesen konkreten Wunsch nicht. |
| Testbetrieb: Bots-WIP `serverguide.rs:55-59,89-93,136`; MVP-Briefing:19. | Vollguide braucht ausdrückliche Aktivierung und nicht leere Testmitgliederliste. MVP nutzt enges Kanalpaar. Kein Vollrollout und kein neuer Pilotversand. |

### Guide-Producer und V4/V5

„Guide-Producer“ ist der koordinierte Compiler-/Prüfablauf des alten Auftrags, kein belegter laufender Nutzerflow. Handoff:7-11,55-92 meldet fehlende Vollprüfungen und `ready_for_takeover=false`. Seine alte Unit ist heute `not-found/inactive`. Ergebnisbelege des alten Laufverzeichnisses wurden nicht vollständig nachverfolgt; daraus folgt weder PASS noch ein neuer Fehler.

V4 `scripts/migrations/2026-10-03-serverguide-v4.sql:3-40` @ `1d820bc2` bringt kanonische Aktionsfreigaben mit Herkunft, Person/Guild, Datenschutzepoche, Ablauf und Versandreservierung. V5 `2026-10-03-serverguide-v5.sql:3-34` hält Herkunft und Widerrufe unveränderlich und verhindert Wiederherstellung abgebauter Einladungsdaten. Beide sind geschrieben, fehlen im geprüften heutigen Main-/Releasebaum. Produktive Anwendung/Grants nicht geprüft.

Offene alte Abschlussbefunde: V1-bis-V5-/Botledger-Reihenfolge, Upgrade-/Neuanlageprüfungen, minimale getrennte Dienstrechte ohne Profilzugriff sowie echte Abbruch-/Neustart-/Steamtransportbelege. Das sind Aktivierungsblocker, keine als Nutzerfeature verkauften Infrastrukturaufträge. Schon angewandte Migrationen bleiben unverändert.

## 3. Patenvermittlung und Übernahme

Die Patenakte wurde über `Bots/.tasks/2026-09-09-paten-programm-live/INTEGRATION_2026-09-27.md` und aktuellen Rust-Code geprüft. Der Bestand ist erheblich, aber die bestehende Betriebsdatei schaltet das Modul aus: `/home/nathanael/.config/deadlock-bots/bot.toml:48,52`. Testnutzerwerte wurden weder ausgegeben noch bewertet. Ein Zustand auf Platte beweist keine Wirkung des laufenden Binaries.

| Teil und Code auf Bots-main `e18f5222` | Urteil und Blocker |
| --- | --- |
| Angebotskarten, Ja-/Nein-Knöpfe, Anfragen-/Rollen-/Last-/Kanalpfad; `concierge.rs:1287-1288,1347-1358` und zugehörige Vermittlungswege. | Wiederverwenden. In Betriebsdatei aus, aktuelle Zustellung/Verfügbarkeit unbekannt. Alter Claim liest Profil-Digest `:7010-7017`; Handoff:112 verbietet Profile an Paten/Moderatoren. Datenvertrag vor Anschluss korrigieren, kein Datenschutz-Ja als Weitergabefreigabe behandeln. |
| Nein-Zweig `concierge.rs:8413` liefert nur `text_reply(PATE_NO_TEXT)`; Wiederholmarker `pate_offered` `:461-468`. | Dauerhafter Nein-Entscheid fehlt. Der Marker verhindert den regulären T2-Wiederholvorschlag, ersetzt aber keine gespeicherte Entscheidung und verbraucht die alte Karte nicht. Vor Aktivierung Nein speichern, Knöpfe der ursprünglichen Karte entfernen und alte spätere Ja-Klicks prüfen. |
| Stopp `concierge.rs:2764-2806`, bestehender Test `:12657`. | Transaktion speichert globalen Datenschutz-/Conciergezustand, nimmt Wunsch zurück und beginnt Anfragenschließung. Wiederverwenden. Test hier nicht ausgeführt, Livewirkung unbekannt. |
| Erinnerungs-/Abschlussstufen, Fristen `concierge.rs:3010-3011`; Schedulerstart `:8463-8465,8526-8532`. | Zwei-/24-Stunden-Stufen, Persistenz und Wiederaufnahme bereits vorhanden. Übergeordnetes `enabled=false` verhindert Subscriber/Scheduler; `proactive=false` allein wäre dafür nicht ausreichend. Kein zweiter Erinnerungsdienst. Schemaanwendung unbekannt. |
| Leitfaden `concierge.rs:5927-5928,5980-6024`, `assets/paten_leitfaden.toml`; Inventar `:6050`, `aiglue.rs:155-167`. | Bewerbung, Leitfaden und Bestandsanzeige wiederverwenden. Aktuelles Panel, gepinnter Leitfaden und verfügbare Freiwillige nicht abgefragt. |
| Pilotsteuerung `concierge.rs:303-305,333-340`; Vollguide-WIP `serverguide.rs:1007-1019`. | Testliste vorhanden; leer bedeutet offenen Modus, nicht begrenzten Test. Der WIP blockiert alte Paten-/Steckbriefknöpfe als deaktivierten Weg. Eine echte neue Guide-Patenübergabe fehlt, kein belegter Pilot. |

Wichtige Trennung: Patenangebot, persönliche Erinnerung, Kontaktserien und Einladungsstatus haben unterschiedliche Zustimmungszwecke. Ein Ja zu einem davon schaltet die anderen nicht automatisch ein. Profilinhalte gehören nicht in die Mentorenkarte. Details: `b/PATEN-WUENSCHE.md`.

## 4. Weitere belegte Produktwünsche

| Wunsch mit Quelle | Aktuelles Urteil |
| --- | --- |
| Clipformat im Onboarding gemeinsam festlegen: Zuschnitt, Titel, Untertitel und Look. Second `projekte/social-media-dashboard.md:114-121` @ `28b4c078`; Twitch `tb-dashboard-api/src/handlers/onboarding.rs:34,67`. | Echter vertagter Wunsch, allgemeines Onboardingmodell vorhanden. Konkretes Interview/Editoranschluss hier nicht vollständig geprüft. Anbieter-/Einwilligungsfragen offen. Keine sichere Neubauempfehlung und kein YouTube-Lernvorschlag. |
| Öffentliche Supportantworten in Website-FAQ und Twitch-In-App. Docs `PLAN.md:38-40` @ `6fa4ca37`; vorhandene Rust-Adapter in Docs/Second; Brain `docs/c9-consumer-contract.md:35-43`, Twitch `tb-config/src/dashboard_options.rs:12-35,89`. | Consumer vorhanden; sichtbare FAQ-/In-App-Oberfläche unbekannt. Öffentlichen Vertrag wiederverwenden; internen Second-Operatorweg nicht öffentlich öffnen. |
| Freie Spiel-/Buildfragen mit Quellen und semantischem Zugriff. Brain `docs/brain-qa-roadmap.md:18-36`; `dbrain-retrieval/src/lib.rs:37-42,65,90,339`, `hybrid_port.rs:17-23,46-53,161`, `brain-api/src/retrieval.rs:11-12,51-52`. | Als pauschaler Neubau überholt: lexikale/hybride Rust-Bausteine und typisierter Weg vorhanden. Quellenabdeckung und echte Antwortabnahme offen. Laufende A/E/F/G-Arbeit, kein zweiter B-Auftrag. |
| Konkrete Mitspieler-, Coach-, Streamer- und Turnierangebote statt allgemeiner Wegweiser. Serverguide-Handoff:123; Bots `concierge.rs:344-367`, WIP `serverguide.rs:1012-1019`. | Produktwunsch belegt. Patenweg und Intents wiederverwendbar. Tatsächliche Angebote, Verfügbarkeit und vollständiger Zustimmungs-/Vermittlungsweg live unbekannt. Keine erfundene Angebotsliste. |

YouTube/Forum sind verworfen. Alte Replay-/Einzelmatchideen werden nicht neu empfohlen; die neuere Entscheidung verbietet Matchdatenablage. API-Spiegel, Rechenkern, Buildveröffentlichung, Hidden Mechanics und Meta-Ränge sind laufende E/F/G-Aufgaben, keine neu entdeckten B-Features.

## 5. Aktive Überschneidungen und Release-Halt

Abschluss-A hält gemeinsame Brainantworten, Consumerfix, persönliche Invite-Statusfragen, Profile/öffentliches Wissen und den Site-Port. Abschluss-B hält verspätete Game-Invite-Versandmechanik; er ist weder dieses Audit B noch ein Paten-/Vollguideabschluss. E hält den lokalen Spiegel und Matchabbau, F den Buildplaner samt Veröffentlichung, G den gemeinsamen Rechenkern und Werkzeugrunden. Keine dieser Implementierungen in B doppeln.

`A/RELEASEFENSTER.md:7,25,49` hält die exklusive Runtimezuständigkeit und den gemeinsamen Release-Halt fest. Neue Planung oder Featurecode hebt ihn nicht auf. Dieser Audit hat keine Mainmutation, keinen Build, Install, Neustart oder Tick ausgeführt.

Sicherer Betriebsstand um 07:52:52 CEST: Brain-Serve active/running, PID `4062119`, Executable aus Maintenance-Releasepfad `bfda408c`; `/opt/deadlock-brain/current` zeigt denselben SHA. Brain-Site active/running mit Python-Executable. Bots active/running, referenzierter `/opt/deadlock/bots/current`-Pfad `e1f11614`; MainPID ist ein Launcher, tatsächlicher Binaryhash nicht geprüft. Twitch-Systemdienst active/running, kein Funktionsbeweis.

Kein neuer Artefakthash-/Manifestbeweis, da der Manifestzugriff im Kontextwerkzeug außerhalb des Workspace verweigert wurde. Kein Umgehen dieser Grenze. Keine realen DM-/Erwähnungs-/Paten-/Kontaktantworten getestet. Daher kein pauschales „alles live“.

## Nachweise und Restlücken

Vollständige Rohberichte und Quellen: `b/GRAFIK-WEB.md`, `b/SERVERGUIDE.md`, `b/PATEN-WUENSCHE.md`, `b/ARCHITEKTUR-NACHWEISE.md`. Priorisierte Bauempfehlung: `B-EMPFEHLUNG.md`; Start-/Modell-/Workflownachweis: `B-STATUS.md`.

Suchumfang: die sechs genannten Repos, alte Serverguideakten, erhaltener Guideworktree/Gitobjekt, G- und A-Site-Worktree, laufende A/B-Abschlussakten, öffentliches Korpus/Patchrouting und sichere Betriebsmetadaten. Nicht vollständig geprüft: Clipeditor-Nebenpfade, alle bestehenden öffentlichen Entitätsseiten, alle dynamischen Patchdaten, private produktive Zustände und alte Producer-Endartefakte. Diese Lücken bleiben ausdrücklich offen.
