status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/Deadlock-2nd-Brain-brain-consumer-fertig

# Paket K: Bestand pro Consumer feststellen

## Ziel und Vertrag

Lies GEMEINSAM.md, AUFTRAG.md und BRIEFING-K.md in ../../. Alle vier Consumer sollen produktiv über den typisierten Rust-BrainClient lesen. Vorhandene Arbeit übernehmen. Kein neuer Provider, kein Python-Produktivcode, keine neuen Code-Kommentare. Rust-Kern und Cutover gehören Q beziehungsweise Z. Dies ist eine Bestandsaufnahme, kein eigener Code-Review. Graphify vor Code-Suchen, keine vollständige Graph-Neuerstellung.

## Eigentum

Die Workflow-Zuweisung nennt genau einen Consumer. Nur lesen: dessen PR-Diff, Branchstand, Repo-Regeln und relevante Runtime-Konfiguration. Keine Dienste neu starten, keine echten Konten verändern, keine Code- oder Git-Mutationen. Bericht nur in bereiche/k/SCOUT-<consumer>.md. Keine anderen Schreibpfade, kein TODO.md und kein zentrales REGISTER.md.

## Arbeitsstand

Auftraggeber: Paketführung teil-k in dieser nativen Claude-Code-Session. Hauptorchestrator: 43a4886c-e135-484b-838a-0512d224a634. Ausgangsworktree ist sauber auf feat/brain-consumer-fertig-20261003, HEAD 44229978e1515712e589fa1df05c6a02ad8c6394. Weitere Consumer-Worktrees werden erst nach Bestandsaufnahme von der Paketführung angelegt. Keine fremden Worktrees ändern, keine neuen T3-Threads, keine Subagenten starten. GPT-6.1 Sol vom nativen Harness erben.

## Beweisziel

Berichte belegte Dateien, vorhandene BrainClient-Anbindung und Consumer-Kennung, offene Schritte für main und Deploy, bestehenden Deploy-Weg, betroffene Units und einen sicheren Funktionsbeweis im Live-Betrieb. Die Zielanfrage muss im brain-serve-Log mit Consumer-Kennung erscheinen. Authentifizierung nur über bestehende Infisical-Wege, keine Secrets ausgeben oder speichern. Unsichere oder nicht ausführbare Beweise als Grenze melden.

## Routing

Rohbericht an teil-k als Workflow-Ergebnis, komprimiert mit Pfaden und SHAs. Statusereignisse schreibt allein teil-k. Routinefragen selbst klären. Fremde Sol-Threads niemals anschreiben, stoppen oder settlen. Deutsche Artefakte mit echten Umlauten, ohne Gedankenstriche; Skills humanizer und no-em-dashes anwenden. Nur eigene vollständige Beobachtungen als Fakten kennzeichnen.
