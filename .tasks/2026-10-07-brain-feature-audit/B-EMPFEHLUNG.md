# Audit B: Empfehlung für die Funktionsplanung

Stand: 07.10.2026. Planungsgrundlage, kein Bauauftrag und keine Releasefreigabe.

## Empfehlung

Drei getrennte Anschlussaufgaben planen: die gemeinsame Entitätsansicht mit qualifiziertem bestehendem Spielwissen und einem kleinen Grafik-/Webseitenrenderer, den vorhandenen Serverguide auf dem gemeinsamen Brain fertig integrieren und danach Paten/Kontaktwünsche an die vorhandenen Botabläufe anschließen. Keine zweite Spielwertequelle, kein eigener KI-Dienst und kein kompletter Neubau des Guides. Der bestätigte YT-Nachtrag gehört als Wissensanschluss zur Entitätsansicht, nicht als vierte Pipeline dazu.

Vor jeder Umsetzung bleibt der gemeinsame Release-Halt bestehen. Die heutige DM-/Erwähnungsabnahme gehört zuerst zum laufenden Abschluss-A. Ein bereits auf Bots-main vorhandener Consumerfix fehlt im referenzierten Release: Dort werden Antworten mit HTTP-Links oder Überlänge verworfen; main entfernt Links und kürzt. Diesen bekannten Anschlussfehler nicht in einem zweiten Grafik- oder Serverguideprojekt nochmals lösen. Quelle: Bots `rust/crates/dl-brain/src/brain_api.rs:121-131` @ `e1f11614`, `:138-163` @ `e18f5222`, unabhängig durch B verglichen.

## 1. Grafik-/Webseiten-MVP

Kleinster brauchbarer Umfang: eine feste Vergleichsansicht für zwei Helden mit einer ausgewählten Kennzahl und deren Verlauf über Boons. Eine kleine HTML-Seite zeigt dieselbe Grafik, die genauen Zahlen, das Szenario und die belegte Client-/Mechanikversion. Dieser Umfang ist aus der gewünschten Heldentabelle, Wachstumsrechnung und den Überholpunkten in G abgeleitet, keine neue Rechenidee. Die konkrete Auswahl einer einzelnen Kennzahl ist eine Produktentscheidung; sie darf nur aus Gs tatsächlich belegter Ausgabe stammen.

### Bestehende Grundlagen verwenden

- G liefert `hero_compare`, `ToolScenario`, Boonbereich und den serverseitigen `PinnedGameContext`. Im gemessenen Stand sind dies WIP-Verträge, kein heutiger Live-Rechner. Das Grafikpaket wartet auf die echte E/G-Integration und rechnet keine Werte nach.
- Brain besitzt bereits einen deterministischen Rust-HTML-Steckbriefrenderer und Herkunfts-/Freigabeprüfung: `brain-maintenance/src/entity_profile_render.rs:16-87,277-401` @ `9711cb63`. Diese Darstellung und `html.rs` wiederverwenden. Die bestehende öffentliche Build-Korpus-Seite und das Patch-Dashboard sind heute HTTP 200 erreichbar. Website-SVG-System und Gestaltung sind weitere Vorlagen, keine neue Wertequelle.
- Die laufende Brain-Site ist Python; der vorhandene A-Rust-Port `cac87635` ist nicht als ausgeliefert belegt. Keine neue Pythonfunktion bauen und den Site-Port nicht ein zweites Mal beauftragen.
- Patch- und Quellenbezug nur aus den mitgelieferten Belegen. Wenn Patchzuordnung ungeprüft ist, Clientversion und diese Grenze zeigen. Keine Grafik mit einem erfundenen aktuellen Patchlabel.

Gs neuester `G/PLAN.md:17` entfernt Grafik-/Webseitenbau und Roadmapeintrag ausdrücklich aus dem laufenden G-Auftrag. Dieses Paket ist separat, konsumiert strukturierte Zahlenreihen und verändert keine laufenden G-Schreibbereiche.

### Feste Templates statt Modell-HTML

Rust rendert validierte typisierte Daten in ein festes HTML-/SVG-Template. Das Modell darf Thema und gewünschte Ansicht innerhalb einer geschlossenen Auswahl vorschlagen; es liefert kein ausführbares HTML, JavaScript, CSS, SQL oder eine Veröffentlichungs-URL. Dadurch braucht der erste Umfang keine weitere Modellrunde für Layout oder Grafik und kein Bildmodell. Builddarstellung ist ein sinnvoller späterer zweiter View auf Fs Ergebnissen, nicht Teil der kleinsten Vergleichsansicht.

Erste Sicherheitsabnahme: Text/Attribute passend escapen, SVG ohne Script und `foreignObject`, keine externen Ressourcen aus Modelltext, restriktive CSP und getrennte Freigabe für öffentliche Ausgabe. Keine privaten Profile, DMs oder Kontakt-/Patenzustände in öffentliche Grafiken übernehmen.

### Vorschau, Veröffentlichung und Links

Standard zuerst Vorschau. Eine öffentliche Seite entsteht erst durch eine ausdrücklich gewählte Veröffentlichung; eine Bestätigung reicht. Öffentliche Gameplaydaten können in einem festgelegten erlaubten Pfad veröffentlicht werden. Persona-/Kontaktseiten sind nicht Bestandteil dieses MVP.

Die URL wird vom Dienst aus eigener Basisadresse und geprüfter Artefakt-ID erzeugt. Für Discord braucht sie einen passenden Bot-Linkbutton oder einen typisierten Artefaktanschluss, weil der bestehende Brain-Textconsumer URLs entfernt. Auch Twitchs `safe_chat_answer` entfernt Links und begrenzt die Antwort auf 450 Zeichen; dafür ist ebenfalls ein enger fachlicher Artefaktanschluss nötig. Die bisherigen Schutzfilter nicht für beliebige Modelllinks abschalten. Keine automatische öffentliche Website aus jedem Chatturn machen.

Versionierte Artefakte binden Clientversion, Mechanikrevision, Anfrage/Szenario, Quellen-/Rechtekontext und Rendererrevision. Vorschauablauf und Widerruf festlegen. Ein langer Cache darf eine geänderte Freigabe nicht ersetzen. Gs Cache bleibt der Rechen-/Antwortcache; ein Rendercache dient nur der Darstellung desselben geprüften Ergebnisses.

Offene Produktentscheidung: eigene Ausgabe unter der bestehenden Website oder unter der Brain-Site; öffentlich ohne Anmeldung oder nur authentisierte Vorschau. Keine neue Domain und keine offene Modell-HTML-Sandbox als Voraussetzung schaffen.

### Qualifiziertes bestehendes Spielwissen in derselben Entitätsansicht

Der YT-Nachtrag einschließlich empirischer Abnahmeergänzung steht in `B-YT-SPIELWISSEN.md`. Der lokale Peer-Lesezugang ist belegt. Beide geprüften Claimsablagen enthalten aktuell 0 Zeilen, die Videoablagen jeweils 195. Die 51 `insight_records` haben in den geprüften strukturierten Feldern Patchherkunft, keine YT-Herkunft. Ein früherer öffentlicher YT-Klassifikationsexport wurde an den dokumentierten/geprüften Orten nicht lokalisiert; keine Datenverlustbehauptung.

Fünf echte öffentliche Quellproben zeigen Parade-Combos mit Lash/Yamato, bedingte Itemwechselwirkungen, räumliche Doorway-/Paige-Bedingungen und Lash-Spielweise. Sie sind keine nachgewiesenen gespeicherten Klassifikationen. Konkrete Claim-ID, Modellurteil, Quellenrevision, kanonische Bindung und heutiger Prüfstand fehlen weiterhin. Keine Probe ist aktuell fachlich freigegeben.

Der vorhandene Code speichert Typen, Quellen und Urteile; `accepted` wird im Retrieval jedoch zu `verified`, ohne damit eine heutige Prüfung zu belegen. Auch qualitative Empfehlungen können veralten. Alte Zahlen und Originalaussagen als historischen Quellenbeleg erhalten, aktuelle Werte ausschließlich aus E/G beziehen. Zahlanteile nicht entfernen und dadurch den verbleibenden Rat als aktuell ausgeben.

G soll den qualifizierten Wissensabschnitt im bestehenden Entitätsabruf festlegen: Bezug auf bestehende Claim-ID/-Hash, kanonische Entitätsreferenzen aller Beteiligten mit Rollen, Bedingungen/Reihenfolge, Originalquelle/-spanne, getrennte Aussage-/Veröffentlichungs-/Prüfzeiten, belegte Version/Patchgültigkeit und Unsicherheits-/Widerspruchszustand. Quellenfreigaben und Abhängigkeiten vor Ausgabe und Cachetreffer prüfen. Der Steckbrief wird daraus erzeugt; keine zweite Profiltextablage und kein konkurrierendes Entitytool. Main kennt bisher nur Spieldatei/Wiki als Profilherkunft; YT nicht als Wiki umetikettieren.

Der vorgeschlagene Wissensvertrag ist aus den heutigen leeren Claimsablagen nicht aktivierbar. Vor einer Anschlussabnahme müsste ein tatsächlicher bestehender öffentlicher Klassifikationsbestand mit Quellen-/Prüfbelegen vorliegen. Die fehlenden Aussagen in diesem Audit nicht neu erzeugen oder importieren. Kein neuer Importer, keine STT/VLM-/Modellprobe, kein Q-Writerabschluss. Vertragsvorschlag und Restgrenzen stehen im Nachtrag, der neue empirische Beleg in `b/YT-EMPIRISCHER-BESTAND.md`. Die laufenden G-/F-Arbeiten bleiben bei ihren Eigentümern.

## 2. Serverguide gezielt fertig integrieren

Der MVP und der Vollguide sind getrennt:

1. MVP: DMs, direkte Erwähnung und eng begrenzte proaktive Hilfe im freigegebenen Kanal sind bereits im gemeinsamen Consumer gebaut. Aktueller Laufzeitschalter und echte heutige Antwortzustellung sind nicht unabhängig belegt. Paket A liefert den vorhandenen Mainfix aus und nimmt diese Wege nach der gültigen Releasefreigabe ab.
2. Vollguide: erhaltene Brainimplementierung `1d820bc2` und Botsadapter `7c2b0a08` selektiv gegen heutiges main integrieren. Nicht die alten Branches pauschal mergen. Öffentliche Fakten über den schon vorhandenen `discord_live`-/`public_server_facts`-Weg nutzen.
3. Persona und persönliche Erinnerung anhand der vorhandenen Guideverträge fertigstellen. Vor Speicherung endgültige Profilfelder, Herkunft und Aufbewahrungsfrist entscheiden. Selbstbedienung für Auskunft, Korrektur, Erinnerung aus und Vergessen gehört zur Freigabe.

Die Persona soll freundlich und geduldig sein, belegte Serverangebote nennen und bei Bedarf Chat statt Voice anbieten. Der Prompt und die Kontrollen sind bereits im Vollguide-WIP enthalten. Ein neuer Persona-Dienst ist unnötig. Der finale Eigenname und repräsentative Nutzertests bleiben offen.

Persönliche Profile sind keine Spielwissen-Entityprofile. Kontaktzustand und Profile bleiben lokal und zugriffsgebunden; sie werden nicht in den öffentlichen Wissensbestand oder ohne ausdrücklichen Datenschutzvertrag in einen externen Modellaufruf gegeben. Paket A inventarisiert die vorhandenen KI-Aufrufwege. Das ist die Voraussetzung für eine passende zentrale Anbindung, nicht der Anlass für eine zweite Providerinventur in B.

### Aktivierungsblocker des Vollguides

V4/V5, Versandledger und minimale Dienstrechte sind technische Freigabebedingungen, keine neuen Nutzerfeatures. Die Migrationen sind geschrieben, fehlen aber im geprüften heutigen Main-/Releasebaum; produktive Anwendung wurde nicht geprüft. Der alte Handoff meldet unvollständige Migrations-/Ledger-/Rechteabnahme. Keine schon angewandte Migration ändern. Auf dem späteren kombinierten Stand Upgrade, Widerruf, Neustart/Racefälle und Versandbestätigung prüfen.

„Guide-Producer“ bezeichnet in der alten Akte den Compiler-/Prüfablauf. Die heutige Abwesenheit seiner alten Unit beweist weder einen fertigen Guide noch einen neu entstandenen Defekt. Eine aktuelle kombinierte Abnahme fehlt.

## 3. Paten und spätere Kontakte

Vorhandene Patenabläufe wiederverwenden und den Guide als Vermittlungseinstieg anschließen. Anfragen, Übernahme, Erinnerungsstufen, Leitfaden und Inventar sind auf Bots-main vorhanden. Die Betriebsdatei hat `concierge_enabled=false`; diese Sperre erhalten. Kein neues paralleles Mentorenprogramm und keine behauptete menschliche Verfügbarkeit aus dem Modell.

Vor jeder Aktivierung zwei konkrete Lücken schließen: Der Nein-Knopf speichert bisher keinen eigenen dauerhaften Entscheid und verbraucht die ursprüngliche Karte nicht (`concierge.rs:8413` @ `e18f5222`). Der alte Claim liest einen Profil-Digest (`:7010-7017`), obwohl der Vollguide-Auftrag Profile an Paten/Moderatoren verbietet. Vermittlungsangaben auf einen eng erlaubten, ausdrücklich gewählten Bedarf beschränken. Bestehende Stopp-Transaktion und Reminderpersistenz wiederverwenden. Der Guide-WIP blockiert die alten Patenknöpfe; er besitzt noch keine fertige neue Übergabe.

Kontaktserien und Begrüßung bestehender Mitglieder bleiben bis zur ausdrücklichen Produktfreigabe aus. Die alte Concierge-Cadence besitzt T2-/T7-Kontakte und Sperrfelder. Das ist Bestand, keine heutige Freigabe oder Livebestätigung. Zielgruppe, Anlass, höchster Umfang und Abbruchregeln entscheiden, statt die ganze Legacyfunktion wieder einzuschalten. Das bereits gespeicherte Nein und Stopp dürfen bei Migration oder Wiederanlauf nicht verloren gehen.

Die fehlenden Pin-/Wiederfinden-Onboardingbilder sind ein eigener kleiner Gestaltungsauftrag. Vorhandene Rang-/Willkommensbilder erfüllen den konkreten Wunsch nicht automatisch. Nicht mit dem versionsgepinnten Gameplayrenderer vermischen.

Weitere belegte Wünsche: öffentlicher Support in Website-FAQ und Twitch-In-App hat bereits Rust-Consumer als Grundlage; sichtbare Oberflächen sind nicht nachgewiesen. Das spätere Clipformat-Interview ist dokumentiert, sein Editoranschluss aber nicht vollständig geprüft. Konkrete Communityangebote sind im Serverguideauftrag verlangt; tatsächliche Verfügbarkeit bleibt ein Livefaktenvertrag, kein Modellversprechen. Diese Punkte erst nach den drei Anschlüssen priorisieren.

## Priorität und bewusste Grenzen

Jetzt planen: den kleinen Grafikview mit klarer Veröffentlichungsgrenze und den selektiven Vollguideanschluss. Zuerst die bestehende A-Abnahme des MVP abschließen. Patenanbindung danach als kleiner fachlicher Anschluss, Kontaktserien erst nach einer ausdrücklichen Entscheidung.

Nicht neu beauftragen: YouTube-Importer, Transkript-/STT-/Videoverarbeitung, Forum, Replay-/Einzelmatchablage, neue Spielwertequelle, weiterer Provider oder zweiter Antwortdienst. Bereits klassifiziertes öffentliches YouTube-Spielwissen gehört nach dem bestätigten Nachtrag ausdrücklich zum Audit und zum möglichen Entitätsanschluss. Hidden Mechanics, Wachstumsrechnung, Meta mit Rangfiltern und Buildplaner sind laufende E/F/G-Arbeit. Sie sind Abhängigkeiten dieses Vorschlags, kein neuer B-Backlog.

## Nachweise und Wissenslücken

Die detaillierten Quellen und je Feature getrennten Zustände stehen in `B-FEATURE-BESTAND.md`, ergänzt durch `b/SERVERGUIDE.md`, `b/GRAFIK-WEB.md`, `b/PATEN-WUENSCHE.md` und `b/ARCHITEKTUR-NACHWEISE.md`. Prozessstatus und Releasepfade wurden lesend beobachtet; keine neue DM, kein Modellturn und kein Versandtest. Laufzeitschalter, produktive Profile/Grants und vollständige Artefaktherkunft sind deshalb nicht als heutige Funktionsbeweise ausgewiesen.
