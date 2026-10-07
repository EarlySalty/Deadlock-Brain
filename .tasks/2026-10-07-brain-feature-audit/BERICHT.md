# Brain: geprüfter Bestand und empfohlene nächste Funktionen

status: Planungsbericht abgeschlossen mit offener YT-Bestandslücke, 07.10.2026

Die KI-, Grafik-, Serverguide- und YT-Inventuren sind ausgewertet. Der Bericht trennt vorhandenen Code, erhaltene Arbeitsstände und tatsächlich geprüfte Funktion. **Die früheren YT-Klassifikationen konnten nicht wiedergefunden und deshalb nicht fachlich abgenommen werden.** Diese Grenze bleibt ausdrücklich offen. Kein Produktcode, keine Konfiguration und kein Dienst wurden verändert.

## Kurzurteil

Die gewünschte Richtung passt zum aktuellen Umbau: **Entitätsdaten und Berechnungen bilden den Kern. Antworten, Grafiken und Webseiten sind verschiedene Ausgaben desselben Wissens.** Der sogenannte Steckbrief ist eine erzeugte Ansicht, kein zusätzlich gepflegtes Dokument.

Ich empfehle drei klar begrenzte Vorhaben, davon zunächst höchstens zwei gleichzeitig:

1. **Grafiken und kleine Webseiten auf den Ergebnissen des Brains.** Vorhandene Darstellungsteile wiederverwenden, fehlenden sicheren Ausgabeweg ergänzen.
2. **Brain als gemeinsame KI-Schnittstelle.** Bestehende Bot-Fachmodule schrittweise anschließen, nicht ihre gesamte Mechanik ins Brain verschieben. Der Serverguide gehört als konkreter Anwendungsfall hier hinein.
3. **Geprüftes vorhandenes Spielwissen an Entitäten anbinden.** Zunächst den früher klassifizierten YT-Bestand lokalisieren, dann Mechaniken und Spielweisen fachlich prüfen. Keine neue YT-Pipeline, keine alten Zahlen und kein zweiter Profiltext. Die Übernahme ist noch nicht baureif, weil der konkrete Klassifikationsbestand fehlt.

Forum, Replay-/Matchablage und weitere Sheet-Importer sind keine empfohlenen Baupakete. Der laufende API-/Rechenkernumbau E/F/G bleibt deren Eigentümern überlassen.

## 1. Grafiken und kleine Webseiten

### Was bereits existiert

- Rust-HTML-Darstellung für Helden, Items und Fähigkeiten, einschließlich Tabellen und Herkunftsprüfung.
- Bestehende öffentliche Build-Korpus- und Patchseiten; beide waren beim Audit erreichbar. Erreichbarkeit beweist nicht die Aktualität ihrer Spielwerte.
- G baut die strukturierte Vergleichsausgabe, Boonverläufe, Szenarien und Versionsbindung. Das sind die Zahlenquellen für neue Ansichten, kein Anlass für einen zweiten Rechner.
- Ein Rust-Port der derzeit noch mit Python betriebenen Brain-Site liegt bereits vor. Diesen vorhandenen Port nicht erneut bauen und den Pythonpfad nicht erweitern.

### Was fehlt

**Die Verbindung zwischen einem geprüften Brainergebnis, seiner Darstellung und einer tatsächlich zustellbaren Ausgabe.** Die öffentlichen Antwortverträge enthalten derzeit Text und Zitate, aber keinen geprüften Grafik-/Seitenverweis. Discord und Twitch entfernen Links aus normalen Brainantworten. Ein Link im Modelltext löst das Problem daher nicht.

Benötigt werden feste Darstellungen aus typisierten Ergebnissen, ein dienstseitig erzeugter Link beziehungsweise Anhang und die passende Bot-Ausgabe. Versionsstand, Szenario, Quellenrechte und Widerruf müssen auch für die Darstellung gelten. Kein frei ausführbares Modell-HTML und keine aus Modelltext übernommene Zieladresse.

### Empfohlener erster Umfang

Ein **Heldenvergleich als Grafik und kleine Detailseite**: zwei Helden, eine tatsächlich von G belegte Kennzahl und deren Verlauf über Boons. Exakte Werte, Bedingungen und Datenstand stehen dabei. Danach kann derselbe Ausgabeweg weitere Entityansichten und Builddarstellungen aufnehmen.

Das ist ein Startumfang, keine Beschränkung des Gesamtziels auf genau eine Diagrammart. Die Darstellung berechnet keine Spielwerte selbst. Aus dem alten Renderer werden Formatierung und sichere Ausgabe wiederverwendet, nicht der durch G abgelöste Profil-Veröffentlichungsprozess.

**Vor dem Bau zu entscheiden:** Wo liegt die Seite und wann darf sie öffentlich werden? Empfehlung: vorhandene eigene Website/Brain-Route nutzen, zunächst Vorschau; eine bewusst angeforderte öffentliche Ausgabe braucht genau eine Bestätigung. Private Nutzerprofile gehören nicht auf diese öffentliche Strecke.

Quellen: [B-FEATURE-BESTAND.md](B-FEATURE-BESTAND.md), Abschnitte 1 und 5; [B-EMPFEHLUNG.md](B-EMPFEHLUNG.md), Abschnitt 1. Der fehlende Artefaktvertrag und Discord-Linkfilter wurden zusätzlich vom Hauptorchestrator am gebundenen Code geprüft.

## 2. Eine gemeinsame KI-Schnittstelle für die Bots

### Heute ist noch nicht alles Brain

| Bereich | Tatsächlicher Stand | Passender Anschluss |
| --- | --- | --- |
| Öffentliche Discord-/Twitch-Wissensfragen | Brainclients existieren bereits. | Bestehende Clients und G-Werkzeuge verwenden. |
| Discord-Concierge, FAQ, Persona, Gesprächshilfe | Teilweise Brain nur für Wissensabruf; finale Antwort wird noch im Bot erzeugt. | Antworterzeugung ins Brain, Discordkontext über einen begrenzten Konnektor. |
| Paten, Kontaktfristen, Rollen, Versand | Überwiegend gespeicherte Zustände und feste Regeln, nicht zusätzliche KI-Aufrufe. | Bei den Bots lassen; bei Bedarf Braintext oder strukturierten Vorschlag anfordern. |
| Twitch-Chat, Pitches, Titel, Assistent, Analysen | Viele verschiedene Fachmodule nutzen bereits zentral `tb-llm`, aber nicht zentral das Brain. | `tb-llm` als kompatible Fassade an Brain anbinden; Fachverträge und Modellfreigaben erhalten. |
| Moderation, Scam, LFG und weitere Richter | Strukturierte Ergebnisse statt normaler Wissensantworten; teilweise private Inhalte. | Eigene klar begrenzte Brain-Aufgabenarten, keine erzwungene Freitextantwort mit Quellenzitaten. |
| Stream-/Audioauswertung | Eigene Datenschutz- und Lokalitätsgrenzen. | Lokal verarbeiten; Brainanbindung darf diese Grenze nicht umgehen. |
| Patchnotes | Übersetzung über freigegebenes Perplexity sonar-pro mit ausgeschalteter Suche. | Brain darf routen, muss die bestehende Ausnahme erhalten. Keine ungefragte Umstellung auf Luna. |
| Steam, Turniere, Uplink | Viele Vorgänge sind rein deterministisch. Steam veröffentlicht Builds über einen vorhandenen Aktionsvertrag. | Kein KI-Umbau ohne Modellaufruf; Plattformaktionen und deren Wiederholungsschutz erhalten. |
| Brain intern | Neben dem neuen Antwortprovider existieren ältere AI-Clients sowie eigene Dokumentautor-/Reviewwege. | Auch innerhalb des Brains konsolidieren; ein Repo allein bedeutet noch keinen gemeinsamen Adapter. |

### Warum „nur die URL austauschen“ nicht reicht

`/v1/answer` ist für wissensgebundene Antworten ausgelegt. Nicht alle Aufgaben passen hinein: ein Spamurteil, ein Titelvorschlag, eine private Dashboardanalyse oder ein Hintergrundjob haben andere Eingaben, Ergebnisse und Fehlerfälle.

Das Brain braucht deshalb **fachlich benannte, typisierte Fähigkeiten hinter einer gemeinsamen Schnittstelle**. Wissensabruf ist optional: Eine Titelfrage kann Spielwissen benötigen, ein LFG-Klassifizierer nicht. Die Bots liefern nur den notwendigen Kontext mit geprüfter Kanal-/Nutzeridentität. Das Modell entscheidet weder über Rechte noch darüber, welche fremden Daten es abrufen darf.

Ein Brain bedeutet weder ein einziges Modell noch eine einzige Datenablage. Die freigegebenen Modelle, lokale Verarbeitung privater Daten, Kosten-/Zeitgrenzen und Anbieterbesonderheiten bleiben erhalten. Ein lokaler Proxy macht ein extern rechnendes Modell nicht lokal.

### Was bei den Bots bleibt

Nachrichtenempfang und Versand, Twitch-/Discordaktionen, Rollen und Rechte, Einwilligung, gespeicherte Ablehnungen, Erinnerungsfristen, Wiederholungsschutz und deterministische Schutzregeln. Das Brain liefert Antwort, Urteil oder Vorschlag; der zuständige Dienst führt eine erlaubte Handlung aus.

Bei Brain-Ausfall bleiben lokale Schutzregeln aktiv. Es darf keinen versteckten direkten Bot-Modellfallback geben, der die gewünschte Zentralisierung wieder umgeht. Ein fehlendes Urteil darf auch nicht automatisch eine Sanktion oder Freigabe erzeugen.

### Sinnvolle erste Migrationswelle

Zunächst einen gemeinsamen Aufgaben-/Rechtevertrag festlegen und **einen Discord-Antwortfall sowie einen nichtkritischen Twitch-Fall** vollständig anschließen. Bestehende Clientfassaden weiterverwenden. Danach weitere Generatoren; private Richter und Streamanalyse erst mit belegter Datentrennung, Lokalitäts- und Ausfallregel.

Die Zentralisierung ist eines der großen Vorhaben, nicht ein paralleler Komplettumbau aller Bots in einem Durchgang.

Quellen: [A-KI-INVENTAR.md](A-KI-INVENTAR.md) mit konkreten Aufrufern, Verträgen und Ständen; [A-ARCHITEKTUR.md](A-ARCHITEKTUR.md). Der zentrale Twitchtransport und der heutige öffentliche Brainvertrag wurden zusätzlich am gebundenen Code geprüft.

## 3. Serverguide, Persona, Kontakte und Paten

**Machbar, aber größtenteils Integration vorhandener Arbeit statt Neubau.**

| Teil | Befund | Empfehlung |
| --- | --- | --- |
| DMs, Erwähnungen, begrenzte proaktive Hilfe | Enger MVP auf main vorhanden; aktuelle vollständige Zustellung nicht neu getestet. | Bestehende Abschlussarbeit fertigstellen, nicht erneut beauftragen. |
| Vollständiger Guide | Brain- und Botimplementierung in erhaltenen Arbeitsständen; nicht vollständig integriert. | Eigenanteile gegen den neuen Brainvertrag übernehmen, keine alten Komplettbranches blind mergen. |
| Persona | Stil und Kontrollen vorhanden, aber kein fertig abgenommener Vollguide. | Im Brain-Antwortauftrag halten, keinen weiteren Persona-Dienst bauen. |
| Persönliche Erinnerung | Vorläufige Datenfelder und Selbstbedienungswege vorhanden. | Vor Aktivierung Felder, Aufbewahrung, Herkunft und Löschung festlegen. Nutzerprofile strikt von öffentlichen Spielentitäten trennen. |
| Paten | Anfrage, Übernahme, Erinnerungen und Leitfaden weitgehend gebaut; Betriebsdatei schaltet Concierge aus. | Vorhandene Mechanik anschließen, nicht bloß pauschal aktivieren. |
| Kontaktserien und Begrüßung bestehender Mitglieder | Alte Abläufe vorhanden, im Vollguideumfang nicht allgemein freigegeben. | Zielgruppe, Anlass, Umfang und Abbruchregeln bewusst entscheiden. Nicht nebenbei einschalten. |
| Wiederfinden-/Pin-Hilfe mit Bildern | Konkreter älterer Wunsch, passende Bilder nicht nachgewiesen. | Kleiner späterer Gestaltungsauftrag; nicht mit Gameplaydiagrammen vermischen. |

### Konkrete Lücken vor einer Patenaktivierung

- Der geprüfte Nein-Knopf antwortet nur mit Text. Er speichert keinen eigenen dauerhaften Nein-Entscheid und verbraucht die ursprüngliche Karte nicht.
- Der alte Übernahmepfad liest eine Profilzusammenfassung. Das widerspricht dem Guideziel, private Profile nicht an Paten oder Moderatoren weiterzugeben.
- Der vorhandene neue Guide sperrt alte Patenknöpfe; eine fertig verbundene neue Übergabe fehlt.

Die ersten beiden Befunde wurden zusätzlich direkt am Bots-Snapshot `e18f5222` geprüft (`concierge.rs:8413` und `7010-7017`). Daraus folgt kein behaupteter heutiger Versand privater Daten, da dieser Liveablauf nicht ausgelöst wurde.

**Einordnung:** Guide und Paten werden als konkrete Anschlussfälle innerhalb des Vorhabens „gemeinsame KI-Schnittstelle“ geplant. Die nötige Zustands-/Datenschutzmechanik bleibt eine Botaufgabe. Dadurch entsteht keine vierte große Parallelbaustelle.

## 4. Vorhandenes YT-Wissen als Entitätswissen

### Gemessener Bestand

Der Hauptorchestrator hat beide vorhandenen PostgreSQL-Ablagen ausschließlich lesend geprüft. Der zunächst im Teilbericht vermutete Zugangsblocker besteht nicht:

| Ablage | YouTube-Videos | Gespeicherte Lernclaims |
| --- | --- | --- |
| Zentrale DB `deadlock`, Schema `brain` | 195 | 0 |
| Dedizierte Brain-DB auf Port 5446, Schema `brain_legacy` | 195 | 0 |

In der zentralen DB sind außerdem 549 Video-Snapshots und 51 Insight-Datensätze vorhanden. Die geprüften Quellenfelder aller 51 Insights weisen auf Patchwissen, nicht auf YouTube. Zwei gefundene Klassifikationsdateien enthalten ebenfalls Patchwissen. Transkripte, Videos und Patchklassifikationen sind keine gespeicherten YT-Claims.

**Die frühere Klassifikation ist dokumentiert:** `docs/TODO.md` nennt für den 28.06.2026 insgesamt 11.941 Creator-Claims; `docs/transcript-claims-pipeline.md` nennt für den 27.06.2026 ungefähr 2.090 Transkriptclaims über etwa 133 Videos. Das sind historische Dokumentationsangaben, keine heutigen Zählungen und keine zu addierenden Bestände.

Der dort genannte Kampagnenordner `~/.cache/deadlock_brain_campaign/` existiert heute nicht. Die gezielte Prüfung dokumentierter Exportpfade, der vorhandenen Quellordner und des Gitindex fand keinen greifbaren YT-Klassifikationsexport. Die Suchgrenze ist in [b/YT-EMPIRISCHER-BESTAND.md](b/YT-EMPIRISCHER-BESTAND.md) festgehalten. Backups, beliebige weitere Laufwerke und private Ablagen wurden nicht geöffnet; hashbenannte Cachedateien nicht vollständig inhaltlich untersucht.

Daraus folgt **kein belegter Datenverlust und keine Behauptung, es habe nie eine Klassifikation gegeben**. Ihr heutiger Ablageort und ihr fachlicher Prüfstand sind ungeklärt.

### Fachliche Prüfung

Fünf echte öffentliche Transkriptpassagen sind belegt: Lash- und Yamato-Combos nach einer Parade, Unstoppable mit Indomitable, eine Doorway-Wechselwirkung und Lash-Spielweise. Bei keiner dieser Proben ist bisher die Verbindung zu einer gespeicherten Klassifikation oder die heutige Gültigkeit belegt. Die Proben zeigen dennoch reale Anforderungen: mehrere beteiligte Entitäten, ihre Rollen, die Reihenfolge, Bedingungen und saubere Abschnittsgrenzen.

Der vorhandene Code kennt Klassifikationen und Prüfkennzeichen, aber `accepted` beziehungsweise das daraus erzeugte `verified` beweist keine aktuelle fachliche Prüfung. Der Transkriptimport kann ein früheres positives Urteil ohne zwingenden aktuellen Datenbeleg übernehmen. Das ist ein belegter Vertragsmangel, kein nachgewiesener aktueller Produktionsfehler.

### Passender Anschluss

Qualifizierte Aussagen werden strukturiert an kanonische Entity-IDs gebunden. Der Entitätsabruf liefert sie mit Quelle, Videozeit, Bedingungen, früherem Urteil und gesondertem aktuellem Prüfstand. Unklare, widersprochene oder überholte Aussagen werden nicht zu gültigen Empfehlungen. Der Steckbrief bleibt eine aus diesen Daten erzeugte Ansicht.

Historische Zahlen dürfen als Quellenbeleg erhalten bleiben, aber nicht aktuelle Werte oder Berechnungen speisen. Diese kommen ausschließlich aus E/G. Auch ein zahlenfreier Spieltipp kann veraltet sein; Zahlen zu entfernen genügt nicht.

**Offen für die weitere Planung:** den früheren Klassifikationsbestand lokalisieren und erst dann konkrete gespeicherte Aussagen abnehmen. Der Anschlussvertrag lässt sich planen, eine geprüfte Übernahme lässt sich heute nicht zusagen. Keine neue Extraktion oder YouTube-Pipeline als Ersatz beginnen. Belege: [HAUPT-YT-DB-NACHWEIS.md](HAUPT-YT-DB-NACHWEIS.md), [B-YT-SPIELWISSEN.md](B-YT-SPIELWISSEN.md) und [b/YT-EMPIRISCHER-BESTAND.md](b/YT-EMPIRISCHER-BESTAND.md).

## 5. Reihenfolge und Schutz des laufenden Umbaus

1. Auf Grundlage dieses Berichts die nächsten Baupakete entscheiden. Eine Implementierung ist mit dem Audit nicht freigegeben.
2. E/F/G bleiben Eigentümer von Spiegel, Builds, Berechnungen, Werkzeugverträgen und aktuellen Entityansichten. Neue Arbeit konsumiert diese Verträge, sie ersetzt sie nicht.
3. Höchstens zwei neue Bereiche zugleich: Grafik-/Webausgabe und Vorbereitung des zentralen KI-Anschlusses. Entitätswissen danach oder als klar getrennte Datenprüfung, nicht als weiterer konkurrierender Kernumbau.
4. Gemeinsam genutzte Antwort- und Entityverträge erhalten je Zeitpunkt einen Schreiber. Bot-/Frontendanschlüsse können nach festem Vertrag parallel entstehen.
5. Bestehender Release-Halt bleibt bestehen. Ein neuer Featureplan ist keine Freigabe für Main-Push, Migration, Neustart oder produktiven Testversand.

## Umfang und Grenzen der Prüfung

Zwei Sol-6.1-Hauptsessions mit high und eigenen Rechercheagenten. A hat einen echten UltraCode-Workflow mit drei Sol-Workern nachgewiesen. B nutzte drei native high-Agenten; sein agentenfreier Workflowtest belegt nicht dieselbe vollständige UltraCode-Aktivierung. Keine Modell- oder Settingänderung für diese Grenze.

Codeinventur über Brain, Discord-/Twitch-Bots, Steam, Patchnotes, Docs, Second-Brain und angrenzende eigene Dienste; außerdem konkrete alte Serverguide- und aktuelle G-Arbeitsstände. Sichere Betriebsmetadaten und öffentliche GETs wurden stellenweise gelesen. **Keine flächendeckende Liveabnahme, keine privaten Nachrichten, keine produktiven KI-Proben und keine vollständige Auditierung aller Twitch-Worktrees.** Einzelne Trigger-/Tenant-/Konfigurationsketten sind im Fachinventar ausdrücklich offen. „Vorhanden“, „auf main“, „erreichbar“ und „funktioniert heute vollständig“ werden nicht gleichgesetzt.

Maßgebliche Quellstände, Zeilen, Restlücken und Agentennachweise stehen in den verlinkten Fachberichten und [REGISTER.md](REGISTER.md). Kein Code gebaut, keine Tests als Funktionsbeweis ausgegeben, nichts deployt.
