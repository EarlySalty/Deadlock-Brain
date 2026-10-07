# G-M-06:45: Gemeinsames Wachstum und rekonstruierte Sheetrechnung

status: gestartet nach tatsächlicher G-M-Rückgabe, Task `wq8uvf8ah`, Run `wf_fca62072-53f`. Task `wthzcnb2d`, Run `wf_08b3462e-169`, ist abgeschlossen; vorheriger Schreibweg beendet. Kein zweiter Reasonerschreiber. Vorgänger: 17 neue Rechentests bestanden, isolierte Bestandssuite 303 passed und 5 failed gegenüber Baseline 287 passed und 4 failed. Format, Clippy und Verbrauchercompiler laut Rückgabe Exit 0; Produktabnahme weiterhin offen.

## Neue betroffene Testannahme zuerst korrigieren

`planner::tests::later_unlock_keeps_the_binding_chosen_at_purchase` erwartet zwei Käufe. Die diskrete Schussrechnung gibt dem zweiten Fixture-Item bei den aktuellen Eingaben keinen Grenznutzen; belegte Bewertung vorher/nachher jeweils 307,10185876623376. Prüfe Ursache am tatsächlichen Test und Rechenweg. Verwende eine belegbar wirksame zweite Fixtureanschaffung, ohne Assertion der bei Kauf gewählten Imbue-Bindung abzuschwächen. Kein Planer-Produktumbau. Die vier weiteren DB-Fixturefehler sind getrennt mit tatsächlicher Baseline zu melden, nicht als neue Regression oder grüner Gesamtlauf.

## 1. Ziel und Vertrag

Vorhandene G-M-Rechenschicht fortsetzen. Lies die tatsächliche G-M-Rückgabe und seinen geprüften Diff, `G/PLAN.md` C2 und Zahlenabnahme sowie `G/SHEET-MODELL.md`, Abschnitt 14. Aktenwurzel: `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/`. Acht-Tool-Vertrag ist als `3d6890c0` gesichert, Anschluss `G/G0-0645-VERTRAG.md`. Gemeinsame Steuerung 06:45 und Nachtrag 06:55 sind verbindlich. Keine komplette Sheetrecherche wiederholen.

Wachstum: Grund-DPS, Magazinschaden und HP, absolute Zunahme je Boon, beliebige gültige Boonstände und relatives Früh-zu-Spät-Wachstum aus genau derselben skalaren Projektion berechnen, die F für Builds konsumiert. Strukturierte Kurven und Überholpunkte verwenden identisches übriges Szenario. Gleichstand, mehrere Schnittpunkte, kein Überholen, Datenlücken und undefinierte relative Zunahme bei Grundwert 0 unterscheiden. Der bereits vorhandene reine Recheneingang bleibt gemeinsam; kein zweiter Rechner für F oder `hero_compare`.

Sheet-Anschluss aus Abschnitt 14:

- Fünf DNS-Blöcke referenzieren Melee und vier Fähigkeiten. Melee über die tatsächliche Heldenreferenz `items.weapon_melee` auflösen, keinen Primärwaffenersatz verwenden.
- Skalierung nach Nicht-Spirit-Stats wie `ELightMeleeDamage` und `EBaseWeaponDamageIncrease` typisiert erhalten. Gültiger Basiswert 0 und negative Werte dürfen nicht verschwinden. Single-Stat-Klasse, `scale_stat_filter`, `EAddToScale` und flache Boni anhand tatsächlicher Payloads zuordnen, keine allgemeine lineare Skalierung erfinden.
- Schadens-, Heil-, Dauer- und Zustandswerte getrennt behandeln. Shadow Transformation ist ein Buff-/Zustandsblock, kein direkter Schadensblock. Ungeklärte Heilskalierung oder Einheiten blockieren ihre abhängigen Werte, nicht unabhängige Grundwerte.
- Scratchpadvergleich als strukturierte Szenariogegenüberstellung derselben Rechnung umsetzen. Erhaltene Zwillinge belegen die Form `r*(b+q)*(1+s)`, `r0*(b*(1+w)+q)*(1+s)` und `r0*(q+b)`. Ursprüngliche gelöschte Eingabe für q und beschädigte restliche Scratchpadbezüge bleiben unbekannt. q kommt aus belegten expliziten Szenariobeiträgen; kein Nullersatz und keine Haze-Konstante. Algebraischer Vergleich mit festgehaltenem q und echte Neuauswertung von Procs bleiben unterscheidbar.

Originalpayloads sind durch unveränderte Hashes und bytegleiche E-Probe an Clientversion 6759 gebunden. Das ist keine aktive Produktionsversion und kein Balancepatchbeweis. Flying Slash, Heilwerte und Zustandswerte benötigen einen ausgeführten Rustbeleg oder eine genaue verbleibende Mechaniklücke. Globale Bounty-/Comeback-/Urn-Rohdaten aus E fehlen weiterhin; keinen Import oder Ersatzwert bauen.

Grafiken und Webseiten baut der Nutzer separat. G baut nichts dazu und trägt nichts in die Roadmap ein. Werkzeugausgaben bleiben strukturierte Zahlenreihen.

## 2. Eigentum

Exklusiv vorhandene G-M-Dateien im eigenen Worktree: `rust/crates/dbrain-reasoner/src/{calculation.rs,calculation_tests.rs,types.rs,data.rs,lib.rs,mechanics.rs,progression.rs,combat.rs,defense.rs,ability_interactions.rs,item_interactions.rs}` sowie unmittelbar zugehörige vorhandene Rechenfixtures. In `data.rs` ausschließlich reine Konverter; Fs Loader-/SQL-/Spiegelauswahl unverändert. Zusätzlich `planner.rs` ausschließlich der betroffene Test `later_unlock_keeps_the_binding_chosen_at_purchase` und seine unmittelbar nötigen Testfixtureeingaben; keine Produktionslogik oder allgemeine Testbereinigung. Diese begrenzte Testgrenze ist vor Änderung an Hauptsteuerung gemeldet. Kein Planer-, Composer-, Confidence- oder Publishumbau und keine Brain-Verträge. G-P/G-K bleiben in disjunkten Bereichen. Ein zusätzlich erforderlicher Vertrag wird mit kleinster Fundstelle zurückgegeben statt eigenmächtig außerhalb des Bereichs geändert.

Vor Bestandssuche Graphify befragen, dann genau gefundene Stellen nachlesen. Bestehende Module erweitern, keine zweite Parser-/Simulationsstrecke, keine neuen Crates. Rust only, keine neuen Code-Kommentare, ENV-Konfiguration, Modelle, festen Timeouts oder Namens-/Heldengewichte. Berechnung und Kurvenauswertung beachten den vorhandenen Abbruch-/Deadlineanschluss; keine unbeschränkten Modellbereiche durchlaufen.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, vorbereiteter Dokument-HEAD `1f5ed30f`. Tatsächlichen HEAD und Vorgängerabschluss vor Start nachsehen. Reasoner-WIP erhalten und fortsetzen, keine Rücksetzung. Kein Git, weitere Agenten/Workflows/T3-Threads, Produktions-DB, Releasebuild, Deploy, Neustart, Tick, Cleanup oder Settle. Gemeinsamer Buildslot, eigener Debugtarget, höchstens drei Cargo-Jobs.

## 4. Beweisziel

Compiler, kontrolliertes Format, striktes Clippy und betroffene bestehende Suites mit vollständigen Logs und unverdeckten Exits. Baseline und tatsächliche Vorgängerprüfung getrennt nennen; keine alte rote Suite als pauschale Ausrede. Test-Postgres nur isoliert nach bestehendem Verfahren. Keine bestehenden Tests löschen, ignorieren oder abschwächen.

Echte Zahlenabnahme aus gebundenen Originalproben: skalarer 0-/20-/35-Boonstand, identische Gesamt-Spirit-Eingabe, Kurven gegen dieselben skalaren Ergebnisse, fünf rekonstruierte DNS-Blöcke einschließlich gültigem Nullwert und Nicht-Spirit-Skalierung, drei rekonstruierte Gegenvergleiche mit bekannten und fehlenden Eingaben. Hergeleitete Sheetwerte nicht als gemessene Spielmechanik ausgeben. Alle acht Stellen einzeln dem ausgeführten Rustweg oder einer konkret benannten fehlenden Eingabe/Regel zuordnen. F erhält genaue Cratepfade, Typen und reine Funktionssignaturen zur identischen Wachstumsrechnung; kein neuer HTTP-Weg.

Gate ist einziger Bug-/Securityreviewer. Bereichsführung prüft den verifizierten Teilcommit mit `gate_hook.py --review`; bei BLOCK frischer Fixer. Kein eigener Reviewer.

## 5. Routing und Rückgabe

Auftraggeber native G-Bereichsführung, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Genau ein Worker als Statusproduzent nach bestätigtem Start. Register, AN_HAUPT und TODO bleiben bei Führung. Wache nach 20 Minuten. Rückgabe tatsächliche APIs/Dateien, Zahlenbelege, vollständige Befehle und Testzahlen, Abdeckung der acht Stellen, unbekannte Regeln und konsumierbarer F-Anschluss. Keine Nutzerfragen oder Sessionnachrichten. Deutsche Produkttexte mit echten Umlauten, ohne Gedankenstriche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
