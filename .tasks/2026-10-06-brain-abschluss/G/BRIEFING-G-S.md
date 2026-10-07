# G-S: Acht kaputte Sheet-Stellen rekonstruieren

## 1. Ziel und Vertrag

Setze die Nutzerentscheidung aus `VON_HAUPT.md`, Abschnitt 06:45 Punkt 3, um. Die vollständige erste Sheetanalyse liegt vor; weder alle Tabs noch Bestand/Baseline erneut untersuchen. Prüfe gezielt die fünf DNS-Abfrageblöcke in `hero query` und die drei `#REF!`-Formeln `scratchpad!F62/G62/H62`. Rekonstruiere ihre beabsichtigten Eingaben, Feldzuordnungen und Rechnungen aus URL/Query, Beschriftung, Umgebung und Nachbarformeln. Dokumentiere die Herleitung in der vorhandenen `G/SHEET-MODELL.md`, mit Originalzellen, Formel, benachbarten Belegen, Einheiten, korrigierter Absicht und Anschluss an die gemeinsame Rust-Rechnung.

Verbindliche Grundlage: aktualisierter `G/PLAN.md`, bestehende `G/SHEET-MODELL.md`, `G/API-PROBEN.json` und `G/sheet/manifest.json`. Unverändertes Original und frühere Bilder liegen im ursprünglichen Bereich `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-06-brain-abschluss/G/sheet/`; ausschließlich lesbar. Workbookhash `992d0e5914036a3f4e4de159f91bae9569bf81aaae9c2222242e95da8253a8c0`. Vorhandene Analyseartefakte wiederverwenden. Den Originalexport nicht neu berechnen, überschreiben oder als korrigiertes Nutzer-Sheet ausgeben.

Jede der acht Stellen braucht einen konkreten Befund. Bei mehreren plausiblen Bedeutungen nicht still eine wählen: Nachbarvergleich und echte Rohfelder entscheiden; verbleibende Mehrdeutigkeit mit belegter engster Rekonstruktion und fehlender Regel benennen. API-Werte nicht passend verändern, keine Spielregel aus einem gelöschten Bezug erfinden. Die Nutzerentscheidung verlangt Wiederherstellung der Absicht, nicht das Kopieren des Fehlerstrings. Gib den umsetzbaren Rechenvertrag für G-M zurück. Rust-Implementierung folgt durch den einzigen Reasoner-Eigentümer, nicht durch dich.

## 2. Eigentum

Ausschließlich eigene Worktree-Datei `.tasks/2026-10-06-brain-abschluss/G/SHEET-MODELL.md`; ergänzender eigener Belegbericht `G/SHEET-REKONSTRUKTION.md` und schmale Belege unter `G/pruefungen/sheet-rekonstruktion/` sind zulässig. Vorhandenes Manifest, Roh-XLSX, Bilder und alle Produktdateien bleiben unverändert. G-M schreibt Reasoner, G0 Brain-Verträge. Keine Akte der Bereichsführung, insbesondere PLAN, REGISTER, AN_HAUPT oder TODO. Keine doppelten Schreiber.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, gepushter HEAD `b4f4b866`. Produkt-WIP anderer G-Worker erhalten. Kein Git, Produktions-DB, Releasebuild, Dienststart, Runtimeeingriff, neuer Importer, weiterer Agent/Workflow/T3-Thread oder Sessionnachricht. Rust only für neue Werkzeuge; bestehende Verwaltungs-/Analysewerkzeuge dürfen gelesen und genutzt werden. Downloadverzeichnis ist Datenquelle, kein Interpreter-Arbeitsverzeichnis. Kein neues Python-Skript oder Produktprototyp.

## 4. Beweisziel

Acht von acht Stellen mit nachvollziehbarer Rekonstruktion und abgegrenzten Unsicherheiten. Originalhash und unveränderte Originalformeln belegen. Echte benachbarte Formeln/Labels und versionierte Rohfelder verwenden, nicht frei erfundene Beispiele. Zeige, welche bestehenden Rust-Eingänge die Absicht tragen können und welche reine Erweiterung noch nötig ist; vor Codebestandsfragen Graphify, danach Fundstellen lesen. Nicht behaupten, Rust-/Spielmechanikparität sei bereits bewiesen, wenn nur die Rechnung hergeleitet wurde.

Alte pauschale Aussagen, die drei Formeln dürften bloß ausgelassen werden, im eigenen Modellbericht an die Entscheidung anpassen. Übrige vollständige Sheetanalyse erhalten. Deutsche Texte mit echten Umlauten, keine Gedankenstriche; nur belegte Aussagen. Kein eigener Reviewagent, Gate ist der einzige Reviewer.

## 5. Routing und Rückgabe

Auftraggeber G-Bereichsführung, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Worker ist Statusproduzent für G-S. Erste Wache 20 Minuten. Rückgabe: acht konkrete Stellen, Rekonstruktion/Belege, umsetzbarer Rust-Vertrag, verbleibende Unsicherheiten, genaue Dateiliste und Originalintegrität. Keine Nutzerfrage und keine weitere Delegation.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
