# Brain: vertiefte Bestandsaufnahme vor der Funktionsplanung

status: aktiv, 07.10.2026

## Nutzerentscheidung

YouTube-Pipeline und Forum werden nicht gebaut. Gewünscht ist zunächst ein Bericht als Planungsgrundlage, keine Produktänderung. Der spätere Nutzernachtrag nimmt ausdrücklich bereits klassifiziertes YouTube-Spielwissen in die Prüfung auf: Mechaniken und Spielweisen für Steckbriefe, keine alten Zahlenwerte. Maßgeblich ist `NACHTRAG-YT.md`.

1. Grafiken und kleine Webseiten aus dem Brain sind ausdrücklich interessant. Vorhandene Wünsche tiefer suchen, an den aktuellen v2-Umbau anschließen.
2. Prüfen, was beim vollständigen Serverguide, Persona, Kontaktserien und Paten schon funktioniert und was fehlt. Fehlende gewünschte Teile ins Backlog aufnehmen, nicht ungeprüft als Neubau deklarieren.
3. Als zweites großes Feature alle KI-gestützten Antwort-/Generierungswege inventarisieren. Ziel ist das Brain als gemeinsame KI-Schnittstelle. Twitch hat mehrere Komponenten; sinnvolle fachliche Konnektoren und bedarfsgerechter Wissenszugriff statt unreflektierter Zentralisierung.

## Auftrag und Grenzen

Reine Recherche. Nur eigene Auditdateien schreiben. Keine Produktdateien, Configs, Services, Datenbanken, Gitrefs oder fremde Aufgabenakten verändern. Keine Commits, Pushes, Merges, Deploys, Neustarts, Modellwechsel oder produktiven LLM-Proben. Keine Nachrichten an Nutzer, Streamer oder Community. Secrets nie lesen oder ausgeben. Öffentliches Produktverhalten und statischer Code genügen; private Chatinhalte nicht in Berichte aufnehmen.

Hauptorchestrator: `a711a4d2-1cad-4120-97ac-8b648567172b`. Die 20 Minuten in Briefings sind ausschließlich der Überwachungstakt, kein Recherchebudget und keine Abbruchfrist. Der Audit läuft bis zum belegten Bericht mit ausdrücklich benannten Restlücken. Teil-Orchestratoren: GPT 6.1 Sol, high, im Claude-Code-Harness, native Subagenten/UltraCode ausdrücklich erlaubt. Keine weiteren T3-Threads. High nicht zu xhigh hochstufen. Aktive Workflowfähigkeit belegen; falls sie fehlt, melden statt Settings zu ändern oder Modellauswahl zu umgehen.

Graphify vor Code-Suche. Bestände in main, ungemergten Branches und Livezustand sauber unterscheiden. Kanonische Checkouts können schmutzig/veraltet sein; `git show origin/main:<pfad>` und gezielte fremde Worktree-Lektüre bevorzugen. Fremde Threads nicht anschreiben oder verändern. Kein ai-coach, kein TradingBot. Keine Code-Kommentare hinzufügen.

## Vom Nutzer bestätigtes Verständnis

Die Entität mit ihren Daten ist die Grundlage, nicht ein separat gepflegter Steckbrief. Aktuelle, versionsgebundene DB-Werte und deterministische Berechnungen werden beim Abruf mit geprüftem, strukturiert zugeordnetem Spielwissen zusammengeführt. Die Steckbriefansicht, Antworten, Grafiken und kleinen Webseiten sind Ausgaben desselben Bestands. Keine zweite Profilablage oder Profilpublikation bauen.

Bereits klassifiziertes YouTube-Wissen zu Mechaniken, Spielweisen, Kombinationen und Bedingungen ist ausdrücklich zu prüfen und als strukturierte, quellengebundene Entitätsaussagen zu beurteilen. Alte Zahlen dürfen keine aktuellen Werte überschreiben; auch qualitative Aussagen brauchen Aktualitätsprüfung. Paket B übernimmt diesen Nachtrag gemäß `NACHTRAG-YT.md`. Diese neuere Vorgabe ersetzt die pauschale YouTube-Ausnahme im ursprünglichen B-Starttext.

Paket A bewertet das Brain als zentrale KI-Verarbeitung mit fachlichen Verträgen und Wissenskonnektoren. Bots liefern Kontext und führen Plattformaktionen aus; ihre deterministische Mechanik bleibt bestehen. Ein gemeinsamer Adapter bedeutet nicht ein einziges Modell oder ein einziges untypisiertes Chatprompt für alle Anwendungsfälle. Bestehende Modellfreigaben und Datenschutzgrenzen bleiben erhalten.

Der Nutzer hat die Delegation dieses Verständnisses bestätigt. Ergebnis bleibt zunächst der angeforderte Bericht als Planungsgrundlage, keine ungeprüfte Umsetzung.

## Gemeinsame Architekturgrundlage

- Brain-Auftrag `.tasks/2026-10-06-brain-abschluss/BRIEFING-G.md` und `VON_HAUPT.md`.
- Aktueller G-Plan und Register im Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/`.
- E: lokaler versionierter API-Spiegel. F: Builds und Veröffentlichung ohne 100-Match-Grenze. G: gemeinsamer Rust-Rechenkern, Steckbriefe als Ansichten, Werkzeugrunden hinter `/v1/answer`.
- Keine Einzelmatchablage, keine zweite Spielwertequelle, kein zweiter Provider oder Antwortdienst. Laufendes G nicht doppelt bearbeiten.
- Bestehender Release-Halt `A/RELEASEFENSTER.md` ist unverändert. Für diese reine Recherche gibt es keinerlei Runtimeoperationen.

## Ergebnis

Ein nachvollziehbarer deutscher Bericht mit Quellenpfaden und Zeilen oder SHAs, Inventar, bestätigtem Iststand, Bau-/Integrationslücken, Architekturpassung, Konflikten, Risiken und einer Empfehlung für höchstens zwei bis drei erste Baupakete. Explizit unterscheiden: nachgewiesen, nur gebaut, nur geplant, veraltet, unbekannt. Keine Fantasieprozente oder pauschale Aussage „alles geprüft“ ohne benannten Suchumfang.
