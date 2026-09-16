# Reasoner: emergenter Item-Zweck – Arbeitsvertrag

Routing-Korrektur des Nutzers vom 16.09.2026: ARBEITSTEILUNG.md hat für Zuständigkeiten Vorrang vor den unten dokumentierten ursprünglichen Opus-Phasen. Keine neuen Claude-Worker oder Claude-Subagenten; ChatGPT übernimmt Quellenrecherche, Mechanikprüfung und unabhängige Sichtung selbst. Nur der bereits laufende Thread 803d3e94 darf 0+A beenden. Fachlicher Vertrag, Rust, Datenbankschutz und Abnahme bleiben unverändert.

Auftrag des Nutzers vom 16.09.2026. Fachlicher Ausgangspunkt: BEFUND.md (unverändert erhalten). Ausgangs-HEAD: 706b129. Die dokumentierten 6/9 Referenzwaffen, 9/10 Staples, Kendall tau 0,577 und Jaccard@12 0,500 sind historische Vergleichswerte, NICHT die neu gemessene Baseline.

## Ziel und Grenzen

Den vorhandenen Rust-Reasoner reparieren, nicht durch eine zweite Engine ersetzen. Alle neuen produktiven Parser-, Modell-, Simulations-, Planner- und Erklärungspfade in Rust. Bestehende Orchestrierungs-/Release-Werkzeuge dürfen benutzt werden; keine neue Python-Produktlogik. Ein Runtime-/Legacy-Audit gehört zur Abnahme, nicht blind fremde Python-Dateien löschen.

Zweck entsteht aus quantifizierter Mechanik, Auslösebedingungen, tatsächlichem Heldenschaden, Zeitverlauf und vorhandenem Inventar. Keine Hero-/Itemnamen, IDs, verborgenen Ausnahmetabellen oder handvergebenen Rollen als Bewertungsregeln. Referenzen und Population sind Messquellen, keine Einkaufsvorschrift; vorhandene Populationseinbindung nicht zur Kaschierung mechanischer Fehler verstärken. Itemnamen sind in Fixtures, Messberichten und Referenzdaten erlaubt, nicht in Produktionsentscheidungen. Referenz-Blindheit/Umbenennungstest zusätzlich zum Text-Guard.

Fachliche Präzisierung: Der im Auftrag beschriebene Screenshot 1599 -> 1391 belegt 208 HP Verlust. 13 % von 4615 sind etwa 600 HP. Kein fixer 600-HP-Abzug, kein fixes Helden-Maxlevel: echte Basis, Wachstum, Inventar und Kaufzustand verwenden. Kleiner Schaden pro Tick macht prozentualen Lifesteal mathematisch nicht automatisch schlechter; unterschiedliche Mechaniken anhand echter KV-Auslösung, Dauer, Refresh, Stacks, Ziele, Heilmodifikatoren und gegebenenfalls belegter Beschränkungen messen. Keine falsche Testprämisse hart codieren.

## Reihenfolge / Eigentum

0. Eingefrorene Messbasis vor Codeänderungen sichern (A-Worker darf dies zuerst erledigen). Bestehenden FROZEN-V2 und Autorenvergleich 779996/versionierte Kategorien wiederverwenden und ihre Grenzen erklären; aktuelle Eingaben lokal einfrieren, Hash/Versionsstand festhalten. Keine zentralen Schreibzugriffe.
A. Eigener Opus-4.8-Coder/Worktree: Konversionsgraph aus echten Helden-/Waffen-Assets durch Loader, Modell, Combat und marginale Bewertung führen. Bericht + Tests + Delta.
Review A. Frischer unabhängiger Thread, keine Selbstfreigabe. Erst bei bestandener Abnahme/Merge-Gate und nicht schlechterem eingefrorenen Backtest übernehmen.
B. Neuer Coder/Worktree auf freigegebenem A: KV-Magnituden, Bedingungssprache, Burst/Dauer/Refresh/verschiedene Ziele, Anti-Gun-Kontext, Quantifizierungsabdeckung.
Review B, dann C (echter HP-Pool, risikoadäquate Defensive), Review C, dann D (Inventar-Compounding, Ersatz/Verkauf und Mechanikketten), Review D.
E. Unabhängige Gesamtabnahme, Gates, Merge/Push, Release gemäß bestehendem DEPLOY-BRAIN.md, Migration nur durch Delegator, Service-Neustart/Live-Nachweis, Warden veröffentlichen nur bei vollständig grüner Abnahme und neue hero_build_id belegen.

Höchstens EIN aktiver Implementierungsworker; frische Reviewer nach dessen Ende. Keine vier parallelen bezahlten Threads. Höchstens ein Cargo-Releasebuild auf dem Host. Worktrees unter /home/nathanael/repos/wt/brain-purpose-*; produktiven Checkout nicht umschalten. Keine unbekannten/fremden Agenten stoppen oder Worktrees löschen.

## Datenbank und Betrieb

Zentrale Datenbank zwingend read-only (technisch erzwingen, nicht nur CLI-Konvention). CLI-Subcommands vorher auf implizite Migration/Persistenz prüfen; bevorzugt lokale Wegwerf-DB und eingefrorene Fixtures. Kein zentraler population ingest/aggregate, kein Migrate/Publish/Timer-/Service-Wechsel durch Worker. Geheimnisse niemals anzeigen/committen. Keine neuen kostenpflichtigen Modelle/Services oder Modellwechsel. Entwicklung: Debugbuilds und fokussierte Tests; nicht gleichzeitig mehrere Vollbuilds.

## Messvertrag

REPORT.md enthält je Phase: HEAD, Datenstand/Hashes, Befehle/Exitcodes, tatsächlich ausgeführte vs. ignorierte Tests, Anzahl quantifizierter/unquantifizierter Mechaniken und Veränderungen, identisch eingefrorene Vergleichsmessungen. 779996 und Population getrennt ausweisen; Autor-Kendall und Population-Kendall nicht verwechseln. Jaccard@12, Referenzwaffen, Staples und Kaufreihenfolge sichtbar machen. Bestehendes fehlgeschlagenes Staple-Gate bleibt rot, nicht weichdefinieren.

Mindestens 2–3 datenbelegte Helden für Skalierung und mehrere Holdout-Helden; Warden nicht als einziges Optimierungsziel verwenden. Ohne Screenshot/Assetbeleg kein behaupteter In-Game-Snapshot. Unbekannte Mechanik als unbekannt mit Provenienz ausgeben, nicht aus Beschreibungsnamen raten.

Abnahme-Ziel: Referenzwaffen mindestens neue Phase-0-Baseline; MR/Rusted Barrel/Healing Tempo nicht mehr als unbegründeter universeller Kern; HP-Verlust nur mit für den konkreten Gegner/Zeitverlauf relevanter Deckung. Keine pauschale HP-Erstattung für Unstoppable (CC-Immunität ist nicht Damage-Immunität), keine pauschale Shield-Freigabe gegen ungeeigneten Schaden. Kauf-/Verkaufentscheidung im tatsächlichen Inventar, keine Entwertung von Multiplikatoren durch Isolation.

Drei identische Ergebnisse nur behaupten, wenn wirklich nachgewiesen. Rust-/Replay-Determinismus von drei unabhängigen Live-KI-Antworten unterscheiden. Stabile Sortierung, identische eingefrorene Eingaben und vollständige Vergleichsartefakte; Zeitstempel-Unterschiede offen ausweisen. Ein gemischter Seed/Live-Lauf beweist keine Regressionfreiheit.

## Nachweis / Abschluss

REGISTER.md führt reale Thread-ID, Modell, Worktree, Basis, Zustand, Bericht, Reviewurteil und Commit. Fertige eigene Threads settle; Nachfolge erst nach Ergebniskontrolle. Keine Phase als fertig/deployt bezeichnen, solange nur beauftragt. Bei BLOCK wird nicht gemergt/veröffentlicht und kein Gate umgangen. Unfertige Arbeit bleibt mit exaktem nächsten Schritt auf gesichertem Branch. Artefakte explizit versionieren/pushen; keine Rohdaten mit Secrets oder große DB-Dumps ins Git.
