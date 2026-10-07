[Orchestrator] Baubereich H: Grafiken und kleine Webseiten aus dem Brain.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: eigener Worktree nach Start

## Auftrag, Rolle, Routing

Lies AUFTRAG.md und PAKETE.md unter /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-grafik-ki/. Nutzer hat Umsetzung freigegeben. Du bist Sol 6.1 high als nativer Claude-Code-Teil-Orchestrator. Eigene native Sol-high-Bauagenten/UltraCode-Workflows ausdrücklich erlaubt, höchstens drei gleichzeitig, eindeutige Dateieigentümer. Vor Workflow workflow-authoring laden. Wirklichen Workflow-Agentenlauf nachweisen; high nicht auf xhigh/max umstellen, kein Modell-/Anbieterwechsel, kein Sonnet. Falls Workflow technisch nicht verfügbar, konkrete Grenze melden und native high-Agenten benutzen. Keine zusätzlichen T3-Threads.

Direkter Auftraggeber: Hauptorchestrator a711a4d2-1cad-4120-97ac-8b648567172b. Berichte über eigene H/STATUS.md und AN_HAUPT-H.md, nicht per ListAgents/SendMessage. Produzent teil-h, Versuch 1. Abgabe nennt SHA, Worktree, Nachweise und Anschlussbedarf. Rückfragen an Hauptorchestrator, nicht Nutzer.

## Bestand und Ziel

Audit als Vorcheck verwenden, nicht erneut komplett recherchieren: .tasks/2026-10-07-brain-feature-audit/B-FEATURE-BESTAND.md, B-EMPFEHLUNG.md, b/GRAFIK-WEB.md. Aktueller G-Stand nur lesend: /home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/PLAN.md und G/G0-0645-VERTRAG.md. Veränderte WIP-Dateien nicht als atomaren Stand behandeln. Vor Codefragen Graphify.

Erster durchgängiger Umfang ist öffentlicher Vergleich zweier Helden mit einer vorhandenen G-Kennzahl als Boonkurve, Grafik und kleine Detailseite aus exakt derselben Ergebnisstruktur. Rust-Renderer entity_profile_render.rs/html.rs und bestehender Rust-Site-Port unter ~/.worktrees/brain-a-site-20261006 sind Wiederverwendungsquellen, kein Auftrag zum alten Profilpublisher. Keine Spielrechnung im Renderer, keine aktuellen Zahlen aus Legacydaten oder Internetabruf pro Frage. Ergebnis-, Versions-, Bedingungs- und Quellenbindung erhalten. Keine frei ausführbaren Modellinhalte, keine privaten Profile. In bestehendem Schwarz-Gold-Stil, echte deutsche Umlaute, native kurze Texte, mobile Nutzbarkeit.

## Arbeitsstand und Eigentum

Kanonischer Brainbaum ist schmutzig auf fremdem Altbranch HEAD 2734c2da; nicht ändern/checkouten. Hole origin und erstelle ausschließlich deinen neuen Worktree /home/nathanael/.worktrees/brain-h-grafik-20261007 auf feat/brain-h-grafik-20261007 vom aktuellen origin/main. Vorher existierende Pfade niemals übernehmen/überschreiben. Start-HEAD, Branch, Status und eigene native Agenten in H/REGISTER.md im Worktree dokumentieren. Kopiere die eigenen Auftragsreferenzen in deine Worktree-Akte, zentrale fremde Dateien nicht editieren.

Du besitzt nur Renderer-/Darstellungsimplementierung plus eigene Tests/Akte. Vor Änderungen H/DATEIEN.md und H/ANSCHLUSS.md: konkrete Produktpfade, Verbrauchertypen, Ein-/Ausgabevertrag und erforderliche Integrationszeilen. K besitzt Manifeste, Registrierungen, HTTP/API/PublicAnswerResponse, Speicherung/Auslieferung und alle Botdateien. H ändert diese nicht parallel und liefert kleine klar abgegrenzte Anschlussvorschläge. G-Vertrags-/Kernel-/Provider-/Rechenpfade nicht anfassen; neue unabhängige Implementierung dort wäre Doppelbau. Gemeinsame Vertragsbedarfe an K über Akte, kein fremder Sessionkontakt.

## Verifikation und Übergabe

Fachcode Rust, Postgres wenn nötig, keine neuen Code-Kommentare, Secrets niemals lesen/ausgeben, keine ENV-Konfiguration. Passende Format-/Compiler-/Clippy-/bestehende Testprüfungen, bestehende rote Baseline sauber abgrenzen. Ressourcenregeln und zentrale Buildablage lesen, keine eigenen Target-Kopien. Renderer mit echten erlaubten G-Daten prüfen, ohne G nachzubauen. Native unabhängige Sichtprüfung und Screenshot Desktop/Mobil; kein Mock als Livebeweis. Keine produktiven Nachrichten oder API-Effekte.

Verifizierte eigene Featurecommits regulär gateprüfen und auf origin sichern, ausschließlich eigene Dateien adden. Du mergst NICHT nach main und deployest NICHT selbst: K integriert Schreib-/Lesepfad gemeinsam und ist ausdrücklich einziger Abschlussverantwortlicher. Ein Stop-Hook ist keine Erlaubnis für einen vorgezogenen Teilmerge. Bei Gate-BLOCK selbst je Runde frischen nativen Fixer einsetzen; nach fünf erfolglosen Runden Blocker mit Ursache übergeben. H ist nach Featureübergabe gebaut/reviewt, nicht live. Branch/Worktree bleiben bis integrierter Main-/Liveabnahme erhalten. AN_HAUPT-H.md nennt echten Beweis und offenen K-Anschluss.
