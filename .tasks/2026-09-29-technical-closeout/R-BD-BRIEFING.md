status: aktiv
Datum: 2026-09-29

# R-BD: unabhängige Consumer-Abnahme vor G5

Astra, frischer unabhängiger Reviewer, kein Autor. Einziger Thread für dieses Paket, keine Unterthreads oder Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a. Hauptauftrag: /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/AUFTRAG.md. Keine Code-Kommentare. Graphify zuerst, bei fehlendem lokalen Graph den bestehenden /home/nathanael/.graphify/global-graph.json verwenden, keine Neuindizierung.

Eigenes Berichts-Worktree: /home/nathanael/.worktrees/brain-pre-g5-consumer-review-20260929, Branch review/pre-g5-consumer-abnahme-20260929, clean bei 1d2a440. Nur dort REVIEW-BD.md in .tasks/2026-09-29-technical-closeout/ schreiben, committen und eigenen Branch pushen. Keine Produktdateien ändern, keine fremden Worktrees verändern. Keine PRs mergen, keine Produktion, keine realen Nachrichten oder Tasks, keine Secrets lesen/ausgeben. Nicht auf Stop-Hook-Zuruf andere Branches abschließen oder löschen.

Prüfgegenstand B:
- /home/nathanael/.worktrees/bots-c9-consumer-wiring, Head 46edc1023a819ba0ba3c37b7de96c333f485f797, Code-Head 553e13491489a3cca9c74118fcc82e54cff3502b.
- Basis für eigenen Diff origin/main = 42175e5fd68a1c83bf4201b4c40cca1099f00652. Erhaltener Merge 00a4e1d7 hat Eltern 74cc1142 und 42175e5f; eingehenden Main-Anteil nicht als C9-Neubau bewerten.
- B-REPORT.md dort in .tasks/2026-09-29-brain-c9-closeout/. Erst Diffstat/Status, dann Bericht. 16 eigene Pfade, etwa 1100 Zeilen.
- Verifizieren: unset Modus erlaubt dokumentiertes legacy, explizit leer/ungültig fail-closed; typed nur BrainClient ohne Legacy/LLM/RAG-Fallback; shadow unabhängig und ohne Verzögerung des sichtbaren Legacy-Pfads; kollisionssichere IDs; unavailable/build_rejected sichtbar richtig, sichere Discord-Ausgabe; Config- und Secret-Grenzen; Release-Workflow bleibt report-only, keine Policy-Abschwächung; registrierter Startpfad tatsächlich benutzt.
- Die Suite im Bericht meldet 232 ausgeführte Tests, 3 ausdrücklich gefilterte echte Golden-/Live-Fälle. Workspace no-run ist nur Kompilierung. Workspace fmt/clippy haben unveränderte Baseline-Mängel. Nicht als Vollgrün ausgeben. Eigene gezielte sichere Gegenproben sind erlaubt, keine unkontrollierten Integrationstests auf produktiver DB.
- PR #459 muss Draft bleiben, kein Auto-Merge.

Prüfgegenstand D, nur Regression und Belegprüfung:
- D-REPORT.md liegt in deinem eigenen Baum. Kein neuer Consumer-Code.
- Docs /home/nathanael/.worktrees/docs-c9-consumer-wiring ff20af7a8e3fcacc349cb2d97eda34dfa0897957, PR #4 nicht mergen. docs.public exklusiv, typisierter Client, kein lokales RAG/Modell, Secret aus bestehendem sicheren Pfad, kein CLI-Token.
- 2nd-Brain /home/nathanael/.worktrees/second-c9-consumer-wiring ab83b691befd761a16d971af5c249a604c5d4e0d, PR #2 nicht mergen. Nur vertrauenswürdige interne Scopes, öffentliche und Wildcard-Sets ablehnen, kein Corpus-Export/RAG/Modellfallback/Tokenargument. Billing-Jobblocker ist separat vom lokalen Testlauf belegt. Ein eigener lokaler fmt/test/clippy-Lauf ist erwünscht, vorhandene Toolchain 1.97.1 verwenden, keine Produktionsdienste.
- Twitch #984 bereits gemergt als 13321934, getestet gegen cf3d77085ed350554914b14c3d9981d37b95903a. Keinen Consumer neu bauen. Der D-Testworktree wurde clean als reiner Basisbranch entfernt, keine Produktänderung behauptet. Bei weiterem Bedarf ausschließlich read-only git show oder eigenen Testworktree. Shadow darf nicht auf Probe warten, typed kein Fallback, ungültiger Modus scheitert.

Abgabe: fertig J/N, Abweichungen, Fix nötig J/N. Vollständige konkrete Mängelliste mit Priorität, Datei:Zeile, reproduzierbarem Szenario und minimalem Fixvorschlag. Vermutungen ausdrücklich trennen. Auftragsgrenzen wie fehlende G5-Freigabe sind keine Codefehler. Kein Implementieren im Review. Genau geprüfte SHAs, eigene Befehle/Exitcodes nennen. Ist Code oder Testbeleg unzureichend, BLOCK statt höflicher Freigabe. Meldung an Hauptsession im Abschluss; keine fremden Sessions kontaktieren.
