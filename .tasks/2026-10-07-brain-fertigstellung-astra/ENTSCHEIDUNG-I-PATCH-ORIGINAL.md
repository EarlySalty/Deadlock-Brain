# I: bestätigte Patchzuordnungsfehler beheben

Stand: 07.10.2026, 16:09 UTC. Derselbe offene I-Auftrag.

Die einmalige Evidenzprüfung ist durchgeführt. Der frühere Reader-Befund wird nicht erneut behauptet; zwei andere tatsächliche Produktfehler sind in Runde 15 reproduziert. Dafür braucht es weder eine Produktentscheidung noch eine Schutzlockerung. Die begrenzte Fortsetzung ist beauftragt.

## Vertrag und Eigentum

I behält E/F sowie eigenen Integrationskandidaten. Startstand laut Rückgabe 860793d7f89d2af9f7510d663e56d592fedf18b2 gegen ca4d877f, vor Beginn tatsächlich prüfen. Gegenprobe 90c17800, alle bisherigen Fixes und fremder WIP bleiben erhalten. Kein neuer T3-Thread; ein frischer nativer Fixer für diese beiden zusammengehörigen Originalquellenfehler. Nur bestehende Patch-/HTML-/URL-Strecke und unmittelbar zugehörige Nachweise, keine neue Pipeline oder fremde G/K-Datei.

## Fixziel

1. Bei einer konkreten Steam-Ereigniskennung muss der gelesene Originalbody zu genau dieser Kennung gehören. Kein Fallback auf irgendein anderes Ereignis nach Titel oder Zeit. Bei fehlendem Original keine fremden Inhalte zuordnen oder vorhandene gültige Ereignisse überschreiben. Bestehenden Parservertrag prüfen und wiederverwenden.
2. Anerkannte Steam-/Forum-Links mit Fragment einheitlich im bestehenden URL-Weg behandeln. HTTP-Anfrage benötigt kein Fragment; für Prüfung und Abruf dieselbe kanonische Anfrage-URL verwenden, ursprüngliche Herkunft nachvollziehbar halten. Origin-/Schema-/Redirect-/Größen-/Timeoutgrenzen nicht lockern. Kein pauschales Verschlucken anderer Abruffehler und kein Tageslaufabbruch nur wegen eines harmlosen Fragments.

Beide diagnostischen Fehlverhaltenszeugen in Regressionen mit richtigem Sollverhalten überführen. Positiver Originalfall mit passender GID und beide Fragmentquellen müssen nachweislich funktionieren. Kontrollierte HTML-Probe ist keine echte Liveantwort. Bildlink-NIT getrennt halten, keine unbelegte Erledigung.

## Prüf- und Freigabegrenze

Bestand zuerst code-suche/Graphify. Passende Compiler-/Format-/Clippy-/bestehende Testprüfungen über den tatsächlich zugelassenen bisherigen I-Prüfweg. Schutzablehnung nie durch anderen Worker, Toolweg, Wrapper oder Hookänderung umgehen. Gemeinsamer Produkt-Gate weiterhin mit demselben urteilsgebenden Claude Opus 5.5. BLOCK erzeugt frischen Fixerkontext, spätestens nach fünf erfolglosen Runden qualifizierte Rückgabe. Kein Modellwechsel und kein Merge bei BLOCK.

Nach echtem ALLOW regulären E-Abschluss und dokumentierte analytics_runtime-Eigentumsübergabe gemäß PAKETE.md liefern. F/G und echter Publish bleiben eigene offene Lieferziele, nicht aus E-Gate ableiten. Neuer HEAD entwertet alte Livebelege.

## Routing

I führt eigene Nachweise und AN_HAUPT-I.md; Delegator 481426fe-b477-42b3-91c6-901811fcba1d führt zentrale Akte. Keine Rundennachrichten. G/K-Prüfsperren werden separat gemeldet und nicht durch I umgangen.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-e-deadlock-api
