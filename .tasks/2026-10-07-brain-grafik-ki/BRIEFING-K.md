[Orchestrator] K: zentrale KI-Anbindung, Guide/Paten und Gesamtintegration H/K.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: eigene Worktrees nach Start

## Rolle und Vertrag

Lies AUFTRAG.md und PAKETE.md unter /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-grafik-ki/. Umsetzung ist freigegeben. Du bist Sol 6.1 high im Claude-Code-Harness, Teil-Orchestrator und einziger Integrator/Deployer dieser H/K-Arbeit. Eigene native Sol-high-Agenten/UltraCode-Workflows erlaubt, höchstens drei gleichzeitig mit disjunkten Pfaden. Vor Workflow workflow-authoring laden. Tatsächlichen Workflow-Agentenlauf belegen. High bleibt high, kein xhigh/max/Sonnet oder Modellwechsel. Bei technischer Grenze melden und native high-Agenten nutzen. Keine zusätzlichen T3-Threads.

Auftraggeber a711a4d2-1cad-4120-97ac-8b648567172b. Produzent teil-k, Versuch 1. Status und native Agenten in K/STATUS.md und K/REGISTER.md, Fachübergabe AN_HAUPT-K.md im Worktree. Keine zentralen Register/TODO-Edits, kein ListAgents/SendMessage. Rückfragen an Hauptorchestrator, nicht Nutzer. 20 Minuten ist Überwachungstakt, kein Baulimit.

## Bestand und Umsetzung

Vorcheck: .tasks/2026-10-07-brain-feature-audit/BERICHT.md, A-ARCHITEKTUR.md, A-KI-INVENTAR.md, b/SERVERGUIDE.md, b/PATEN-WUENSCHE.md. Nicht flächendeckend neu inventarisieren. Vor neuen Codefragen Graphify und Fundstellen prüfen.

Erste vollständige Welle: vorhandene Discord-FAQ/Serverguide-Endantwort ins Brain; einen nichtkritischen Twitch-Textentwurf, bevorzugt Titelentwurf ohne neuen automatischen Helix-Effekt, über kompatible tb-llm-Fassade ans Brain. Fallauswahl in K/PLAN.md festhalten. Bestehende Modellfreigaben, Budgets, Timeouts, Datenrechte und Provider-Ausnahmen erhalten. Kein pauschales Luna-Routing oder Universal-Freitextprotokoll für strukturierte Aufgaben. Typisierte Fähigkeiten im vorhandenen Brain, Wissensabruf optional. Bot behält Plattformzustand, Rechte, Versand, Zustimmung, Fristen, Sanktionen und Wiederholungsschutz. Unmigrierte Fälle als Restumfang nennen, nicht alle KI als umgezogen melden.

Nutzerkorrektur nach Dispatch: Pate = Brain = Concierge. Gemeint ist dieselbe persönliche Brain-Hilfe, nicht das menschliche Patenprogramm. Erhaltene Guide-/Conciergearbeit selektiv integrieren, keine alten Komplettbranches. Bestehende Nutzeranfrage bis Brainantwort durchgängig herstellen. Keine menschliche Patenvermittlung, Patenrollen, Übernahme- oder Patenanfragen umbauen. Ablehnungen und Datenschutz der einen Brain-Hilfe weiter respektieren. Kein pauschales Aktivieren aller proaktiven DMs, keine neuen Kontaktserien und keine persönliche Gedächtnisablage ohne festgelegte Regeln. Datensparsame Guidefunktion und bewusste Nutzeranfrage dürfen funktionieren, ohne breite Kontaktprogramme einzuschalten.

Private Nutzerdaten bleiben lokal. Loopback-Proxy zu externem Modell ist keine lokale Verarbeitung. Fehlt ein erlaubter lokaler Provider für einen gewählten Fall, erst bestehende Config prüfen, keinen Anbieter erfinden oder Datenschutz lockern. Öffentliche Antwortfälle weiter umsetzen und genaue Grenze melden. Moderation/Scam sowie vollständige private Analyse gehören nicht zu dieser ersten Migrationswelle.

## Eigentum und Ausgangsstand

Alle kanonischen Checkouts sind fremd/schmutzig. Brain HEAD 2734c2da auf altem Featurebranch, Remote-main beim Auftrag f6f5cef6. Bots Remote-main 56571e40, Twitch 9315b3cf. Nicht als feste Baupins behandeln: origin frisch holen und eigene Worktrees auf aktuellem origin/main erstellen:
- /home/nathanael/.worktrees/brain-k-ki-20261007, feat/brain-k-ki-20261007.
- /home/nathanael/.worktrees/bots-k-guide-20261007, feat/bots-k-guide-20261007.
- /home/nathanael/.worktrees/twitch-k-ki-20261007, feat/twitch-k-ki-20261007.
Bestehende Pfade nicht überschreiben/übernehmen. Eigene Anfangs-SHAs/Status dokumentieren, kanonischen HEAD niemals umstellen. Lokale Repoanweisungen vor Änderungen lesen.

K besitzt gemeinsame Manifeste, API-/Clientverträge, HTTP-Routen und Botanschlüsse. H besitzt nur isolierte Renderer/Darstellung und liefert H/DATEIEN.md, H/ANSCHLUSS.md plus geprüften Feature-SHA unter /home/nathanael/.worktrees/brain-h-grafik-20261007. H-Dateien nicht parallel schreiben. H später selektiv integrieren und Dienstlink/Anhang bis Discord/Twitch liefern. Allgemeine Linkfilter nicht abschalten, kein Link aus Modelltext übernehmen. Bestehenden Rust-Site-Port und vorhandene Route statt Neubau prüfen.

G ist weiterhin aktiver Eigentümer von tools/contracts, Provider, Kernel und Rechnung. Lies nur dessen aktuelle Akte /home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/AN_HAUPT-G.md. Dort keine parallelen Änderungen oder Kopie seines WIP. Zunächst disjunkte Fachadapter/Botzustände bauen; gemeinsamer Anschluss erst auf geprüftem integrierten G-Vertrag. Konkrete Codeabhängigkeit melden, keinen fremden Buildslot absprechen. E/F/I-Bereiche unangetastet lassen.

## Verifikation und Abschluss

Rust, Postgres, keine neuen Code-Kommentare. NEVER read, print or write plaintext secrets. MUST NOT send private user/community data to remote models. Infisical und normale Configdateien, keine ENV-Konfiguration. Compiler/Format/Clippy und vorhandene Tests, echte Baselinegrenzen. Aktuelle zentrale Build-/sccache-Regeln lesen; keine zusätzlichen Target-Kopien, Sperren nicht umgehen. Screenshots/Desktop/Mobil und echte sichere Wirkungsprobe für Grafikseite; Rückgaben/Fehler/Rechte/Version und Ausfallparität für KI-Wege. Keine fremden Konten rotieren/trennen oder echte Moderationsaktionen als Test.

Release-Halt ist aufgehoben: zentrale Abschlussakte VON_HAUPT.md Abschnitt 09:00 und A/RELEASEFENSTER.md Kopfzeile. Normaler Abschluss nach gemeinsamer Abnahme und Gate-ALLOW erlaubt, kein neuer fremder Session-Halt. Abhängigkeit G nicht durch Parallelneubau umgehen. Eigene Featurecommits verifiziert pushen. Gesamtintegration H/K einmal gegen aktuelle Mainstände abnehmen; lokaler Merge-Gate ist Reviewer, keine zusätzlichen T3-Reviewthreads. Bei BLOCK pro Runde frischer nativer Fixer, autonom bis ALLOW, nach fünf erfolglosen Runden begründet melden.

Danach einzeln mergen/pushen, vom aktuellen origin/main im eigenen Worktree regulär bauen/deployen, flock und Deploy-Wrapper verwenden, Dienste nach Repo-Regeln neu starten, Livebeweis und Cleanup aller eigenen integrierten Branches/Worktrees. Kein PR/Actions, kein Nutzerstimme-Post. Falls echter Vertragsblocker: verifizierte Featurearbeit erhalten und mit konkreter fehlender Abhängigkeit melden, kein falsches fertig. Abschluss nennt Restumfang, URLs/Bedienweg, SHAs, Gate-, Test- und Livebelege sowie gelöschte oder noch ersetzbare Altpade.
