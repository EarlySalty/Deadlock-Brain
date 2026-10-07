# A-C: Altbranches fachlich entscheiden

Nativer read-only Blatt-Worker, kein Reviewer oder Implementierer. Auftraggeber Paket A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`, Hauptorchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Keine zusätzlichen Agenten/T3-Threads und keine Sessionnachrichten.

## Ziel und Eigentum

Lies den heutigen `../AUFTRAG.md`, `../BRIEFING-A.md`, `STAND.md`, `INVENTUR-REASONER.md`, `INVENTUR-WISSEN.md` und `../C/OFFEN.md` samt `INVENTAR.md`/`BACKUP-SHAS.txt`. Die 62 offenen Zeilen werden ohne Auslassung auf drei read-only Worker verteilt. Nur deine im Workflow genannten Tabellenpositionen prüfen. Keine Dateien, Branches, Worktrees, PRs, Prozesse, DB oder Dienste verändern. Deine Ausgabe ist eine native strukturierte Rückgabe. Allein Paket A trägt Entscheidungen in C/OFFEN.md ein, C setzt sie um.

## Beweisziel

Je SHA entscheiden `übernehmen` oder `verwerfen` mit kurzem Grund und konkretem Beleg. „übernehmen“ bedeutet gezielt verwerten oder bewusst erhalten, nicht den kompletten alten Branch mergen. „verwerfen“ nur, wenn der relevante Inhalt belegbar vom heutigen Main ersetzt ist oder ausschließlich veraltete Halte-/Arbeitsnotizen enthält, deren bewahrte SHA-Akte genügt. Eigenständige ungemergte Produktänderungen, wertvolle Artefakte oder fremde laufende Arbeit nicht wegurteilen. Zurückgestellte Funktionen bleiben erhalten; kein impliziter Löschauftrag.

Graphify vor jeder Codebestandssuche. Diffgegenstand ist der ursprüngliche Eigenanteil ab tatsächlichem Mergebase und seine heutige Entsprechung, nicht der riesige vollständige Rückwärtsdiff gegen Main. `merge-base --is-ancestor` mit Exit und `git cherry` prüfen; Ancestry 1 allein beweist weder fehlende Funktion noch erforderliche Übernahme. Tatsächliche Main-Fundstelle/Blobgleichheit oder konkrete eigenständige Differenz belegen. Kein pauschales Urteil aus Branch-/PR-Titel. Nur lesende Git-Aufrufe mit literalen absoluten Repo-/Worktreepfaden. Uncommittierte Bestände aus C/INVENTAR gesondert nennen; Entscheidung des alten SHAs gilt nie für noch nicht gesicherten WIP.

Native codebuilder A-F1 bis A-F4 besitzen heute `brain-a-profile-20261006`, `brain-a-discord-20261006`, `brain-a-site-20261006`, `brain-a-sheet-20261006`; zusätzlich Paket-A-Integration `brain-a-abschluss-20261006`. Sie sind unabhängig vom Altbestand geschützt. Der fremde kanonische Checkout bleibt unangetastet.

## Rückgabe

Je zugeteilter Tabellenzeile: Branch/Alias, achtstelliger und nachgelesener voller SHA, Entscheidung, kurzer Grund, konkrete Belegpfade/Kommandobefunde, etwaige uncommittierte Schutzgrenze und selektiv brauchbarer Eigenanteil. Vollständige Abdeckung deiner Positionen bestätigen, Unsicherheit als Erhaltungsentscheidung mit fehlendem Beleg nennen. Keine zusätzlichen Reviews, Modellrunden oder Main-Integration. Deutsch, echte Umlaute, keine Gedankenstriche. Wache durch Paket A nach 20 Minuten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: hauptbaum
