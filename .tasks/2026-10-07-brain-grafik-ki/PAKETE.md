# Pakete und Schreibverantwortung

status: aktiv, 07.10.2026

**Nutzerkorrektur nach Dispatch:** Pate = Deadlock Brain = Concierge. K integriert diese eine persönliche Brain-Hilfe mit Guidefähigkeit. Das menschliche Patenprogramm gehört nicht zum Auftrag; Hinweise darauf in der historischen Auditplanung sind kein Bauauftrag. Maßgeblich: NUTZERKORREKTUR-PATE.md und aktualisierte AUFTRAG.md.

| Paket | Rolle | Eigener Bereich | Übergabe |
| --- | --- | --- | --- |
| H | Sol-high-Teil-Orchestrator Grafik/Web | Eigener Brain-Worktree, Renderer und sichere Darstellung aus typisiertem Ergebnis; eigene H-Akte | geprüfter Feature-SHA plus isolierte Anschlussänderungen an K |
| K | Sol-high-Teil-Orchestrator zentrale KI, Guide/Paten; Gesamtintegrator | Eigene Brain/Bots/Twitch-Worktrees, neue KI-Fachadapter, gemeinsame API-/Client-/Botanschlüsse, Guide/Paten, Gesamtintegration | kombinierter SHA, Gate, Livebeweis, Cleanup |
| S | Schmaler Sol-high-Aufgabenstand-Agent | ausschließlich zentrale TODO.md, nur Paketdefinitionen und Statusdateien lesen | verdichteter Stand, keine fachlichen Entscheidungen |

## Keine konkurrierenden Schreiber

G aus dem bestehenden Brain-Abschluss besitzt weiterhin Werkzeugturns, Rechenkern, Provider/Kernel, öffentliche Entityverträge und seinen G-Schnitt. E/F/I besitzen laufende Spiegel-/Buildintegration. Diese Arbeit bleibt unangetastet. H und K dürfen vorbereitend disjunkte Module auf aktuellem main bauen; benötigen sie geänderte G-Dateien, liefern sie erst einen abgegrenzten Anschlussbedarf und ändern diese Dateien erst auf dessen integriertem, geprüften Stand. Keine Nachrichten an fremde Sessions.

K besitzt alle gemeinsamen Registrierungen, Workspace-Manifeste, PublicAnswerResponse-Erweiterungen, Brain-HTTP-Routen sowie Discord-/Twitch-Ausgabeanschlüsse. H liefert hierfür eine H/ANSCHLUSS.md mit Typen, Signaturen und begrenztem Integrationsdiff, schreibt diese Produktpfade aber nicht selbst. H verändert weder Botrepos noch G-Rechnung. H nutzt vorhandene Darstellungsdateien bzw. einen abgegrenzten Darstellungsbereich; vor erstem Produktedit konkrete Pfade in H/DATEIEN.md veröffentlichen. K liest diesen Vertrag vor eigenen Edits. Bei ungeklärtem Konflikt nur den strittigen Pfad stoppen, disjunkt weiterarbeiten.

H und K arbeiten ausschließlich in eigenen Worktrees unter /home/nathanael/.worktrees. Repos am Start:
- Brain: geteilter Featurebranch feat/brain-rust-cutover-20260919, HEAD 2734c2da, schmutzig; Remote-main f6f5cef65f1f946113f0b8216c6475f6d38ec928.
- Bots: geteilter main dbda52b8, schmutzig; Remote-main 56571e40fa215a5cbca081b09827d78fdff4c00d.
- Twitch: geteilter main d8284816, stark schmutzig; Remote-main 9315b3cf7e4a6feeffc32b603db26eca78b03a2c.
Diese Stände sind Beobachtungen, keine Baupins. Vor eigenem Worktree aktuelle Remotes holen. Kanonischen Branch/HEAD niemals wechseln.

## Baufolge

1. H baut den festen Rust-Darstellungskern gegen nachgelesene typisierte G-Ergebnisse, mit Versions-/Quellen-/Bedingungsanzeige und sicherer Ausgabe. Vorhandenen Renderer/Rust-Site-Port wiederverwenden.
2. K konkretisiert minimalen Aufgaben-/Identitätsvertrag und baut disjunkte Botfassaden sowie Guide-/Patenzustände. Einen Discord-Antwortfall und einen nichtkritischen Twitch-Entwurf wählen und im K/PLAN.md verbindlich benennen; übrige KI-Wege bleiben Folgearbeit.
3. K integriert G-Abhängigkeiten aus origin/main, H-Darstellung samt gespeichertem Artefakt/Linkroute und Botzustellung. Gemeinsame Dateien nur durch K. Kein Mainmerge von H allein.
4. K prüft den Gesamtzustand, vorhandene Suites und sichere Ausfälle; unabhängige Intent-Abnahme, regulärer Merge-Gate. K ist einziger neuer Deployer. Normaler Abschluss seit 09:00 wieder erlaubt; keine Fremdbuild-Absprachen.

## Artefakte und Status

H und K schreiben nur ihre Bereichsakten H/ bzw. K/ im eigenen Worktree. Zentrale REGISTER.md nur Hauptorchestrator, TODO.md nur S. Jeder Bereich schreibt STATUS.md mit absolutem Worktree, Branch, SHA, gebaut/reviewt/gemergt/live, aktiven nativen Agenten und Blockern. Zustandswechsel zusätzlich als unveränderliches JSON unter status/<paket>/1/<sequenz>.json nach ABLAUF.md. Auftrag 2026-10-07-brain-grafik-ki, Produzenten teil-h und teil-k. Eigene Bereichsberichte AN_HAUPT-H.md bzw. AN_HAUPT-K.md. Keine Rohlogs in zentrale Statusdateien.

S liest zunächst zentrale Dateien. Sobald Worktrees im Register stehen, liest S deren Bereichsstatus; keine Fachberichte/Code/Daten/Reviewlogs. S bleibt bis Gesamtabschluss ansprechbar und startet keine Unteragenten. Hauptorchestrator kontrolliert etwa alle 20 Minuten selbst die eigenen T3-Threads und trägt IDs/Starts nach.
