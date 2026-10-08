# G-K-R2: Ausgabezweck geprüft, Buildschutz noch offen

status: Zweckdelta lokal geprüft und ALLOW; ein BLOCK offen, 07.10.2026

Frischer Fixer `w6qizlk2u` / `wf_4795365a-b3c` tatsächlich zurückgegeben. Start `1b5ea4527576c1b8b622e86e81067b396889f2b0`, eigener Featurecommit `3242fb36ed847dc06ef71d92c71108614b5d493f`. Genau Kernel-lib.rs und execution.rs verändert. InternalRead behält frische, Cache- und Flightrechte ohne unzulässige Publikationspflicht. Providerweitergabe einschließlich unzitierter Eingaben und Neuprüfung vor finaler Ausgabe bleibt erforderlich; ForPublication prüft zusätzlich Veröffentlichung.

Bereichsführung bestätigte tatsächliche Testmarker: neue Zweckteilmenge 4 passed/0 failed/0 ignored; vollständiger Kernel 61 passed/18 failed/0 ignored, Exit 101. Vorgänger 57/18, Ursprung 34/18. failure-baseline.json enthält dieselben 18 Namen, added/removed leer. Format, Compiler und striktes Clippy Exit 0. Befehle/Exits in `G/pruefungen/g-k-r2/commands.log`. Quellenbindung durch Bereichsführung erneut mit sha256sum --check --status bestätigt: 46/46, Exit 0. Manifest-SHA `34bd77bb9c3ae27e2d8207ee1015e790b6c24d528d8350165295a5b0ef56d2d7`.

Regulärer Gate auf Zweckdelta `1b5ea452..3242fb36`, Exit 0: `[gpt-6.1-sol] ALLOW: No blocking defect found. Answer purpose is propagated consistently, and final responses and reused answers retain provider validation.` Kein ALLOW für den offenen Buildschutz oder Gesamt-G.

## Begrenzter Restvertrag

ToolExecution::validate_for prüft Aufruf, Pin und Belegzuordnung, aber keinen deterministischen Build. validate_dependencies erhält nicht das echte Ergebnis. Bestehender Buildschutz bleibt deshalb weiterhin umgehbar. Kein erfundenes Buildresultatschema und kein pauschales Abschalten gültiger BuildPlans im Zweckfix.

Bereichsführung aktiviert für einen neuen frischen Fixer genau den kompatiblen typisierten Portanschluss in brain-contracts/src/tools.rs und dessen Kernelkonsum. Der vertrauenswürdige Port muss das tatsächliche BuildPlan-Ergebnis prüfen; Wiederverwendung muss denselben gebundenen Nachweis neu prüfen. Bestehende Ports ohne Anschluss sollen sicher ablehnen. Kein Planer/Composer, keine synthetische Quellenfreigabe, kein Publish oder Steam. G-V implementiert die echte Prüfung später über Fs vorhandenen reinen Buildvertrag.

TESTNACHWEIS[TW-1]: 61 passed, 0 ignored | Baseline: 18 rot

MERGEPROTOKOLL[MS-1]: 2 Git-Schritte einzeln | Anläufe: 1 | Gate: [gpt-6.1-sol] ALLOW für Zweckdelta; kein Main-Merge
