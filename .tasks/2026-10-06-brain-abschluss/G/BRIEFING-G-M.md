# G-M: Gemeinsame Modelle und Rechenschicht

## Verbindliche Wiederaufnahme

Der ursprüngliche Workflow `wf_08b3462e-169` wurde beim Ende der vorigen Session gestoppt, ohne Abschlussmeldung. Der vorhandene uncommittierte Stand in Typen, Konvertern, Mechanik, Fortschritt und Simulator wird fortgesetzt, nicht erneut gebaut oder zurückgesetzt. Gepushter HEAD ist `f129c91a`; die früher im Briefing genannte Basis bleibt die Produktbasis. Vor Änderung die eigenen bisherigen Artefakte, Logs und den tatsächlichen Diff prüfen, dann fehlende Arbeit und Verifikation abschließen. Sheet und Bestand nicht neu recherchieren. Der Vertragsfix wird getrennt wiederaufgenommen; keine Brain-Vertragsdatei bearbeiten. Release-Hold erneut bestätigt: keine Main-/Runtime-/Cleanupaktion.

## 1. Ziel und Vertrag

Baue die strukturierte deterministische Zahlenansicht im vorhandenen `dbrain-reasoner`, keine zweite Parser- oder Simulationsstrecke. Verbindlich: `G/PLAN.md` C2, Abschnitte 4, 5, 6 und Zahlenabnahme; `G/SHEET-MODELL.md`, `G/BESTAND-MECHANIK.md`, `G/API-PROBEN.json`, `G/BASELINE.md`. Alle Pfade liegen unter `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/`.

Zuerst gemeinsame öffentliche Modell-/Szenariotypen und reine vorhandene Payloadkonverter kompatibel ergänzen, Compiler prüfen und Signaturen festhalten. Danach damit Mechanik, Fortschritt, gemeinsame Stataggregation, Vergleich/Ränge und den vorhandenen Simulator erweitern. Brain-Verträge/Provider/Kernel schreibt parallel G0, nicht du; deine öffentlichen Rechenparameter dürfen unabhängig in den bestehenden Reasoner-Typen leben. Kein neues Crate.

Rechenumfang: Grund-/Boonwerte und AP, HP/Regen/angeborene Resistenzen, kanonische Spiritkonversionen, Waffen-DPS/RPM/Magazinschaden/Reload, Pellets/Burst/Einzelnachladen, Shopboni, Ability-/Item-/Stack-/Proc-Schaden, Resist/Shred/Amp, Zielzustand und TTK. Derselbe Kern muss weiterhin Planer und neue öffentliche Zahlenansicht bedienen. Unbekannt bleibt unbekannt mit fehlenden Feldern/Regeln; keine Ersatznullen, frei erfundenen Caps, Konstanten aus dem Sheet oder Namens-/Heldengewichte.

Sheet-Formelparität und echte Spielmechanik getrennt. API-Rohmetriken unverändert ausweisen, berechnete Szenarien separat. Planbefund: einfacher Reloadzyklus weicht bei drei Proben um eine rechnerische Zusatzzeit 0,25 s ab. Rohfeld `recycle_time`, andere Zeitfelder und öffentliche API-Berechnungsherkunft empirisch prüfen; keine 0,25-Konstante einbauen. Falloff-/Crit-Einheit und Biasfunktion belegen, sonst nur diese Felder als unquantifiziert führen. Bestehender 0,2-s-Simulator ist kein Beleg exakter TTK; dessen Ereignisse und Waffenzeitrechnung korrigieren statt einen zweiten Simulator zu schreiben.

Ranking über vollständigen aktiven Eingabesatz derselben Version und desselben Szenarios: Rang 1 plus strikt bessere Werte; Gleichstände gleicher Rang. Perzentil 100 × (schlechtere + halbe gleiche) / gültige Population. Richtung, vollständige/fehlende Population, Einheiten und Szenario sichtbar. Kein endliches TTK-Ranking bei zensierter Killzeit.

Zusätzliche globale Hidden-Mechanics-Daten aus E fehlen. Bereits in Hero-/Abilitypayloads belegte Resource-/Rage-/Stackdaten generisch nutzbar machen; Bounty/Comeback/Urn nicht erfinden. Kein eigener Import, DB-Werteleser, API-Liveabruf je Frage oder neuer Patchparser. Reine Konverter erhalten Originalreferenzen und vorhandene 0-/Negativwerte.

## 2. Eigentum

Exklusiv eigene Worktree-Dateien unter `rust/crates/dbrain-reasoner/src/`: `types.rs`, `data.rs`, `lib.rs`, `mechanics.rs`, `progression.rs`, `combat.rs`, `defense.rs`, `ability_interactions.rs`, `item_interactions.rs`. Unmittelbar nötige eigene Reasoner-Testfixtures und ein schmales Rechenmodul sind zulässig. Öffentliche Modelllade-/SQL-/Spiegel-Auswahlabschnitte in `data.rs` bleiben F vorbehalten; du erweiterst die reinen Konverter und ihre öffentliche Zugänglichkeit, nicht die aktuelle Datenquelle. Kein F-Planer-/Composer-/Confidence-/Publishumbau.

Die G-Grenze ist vor Änderung in `AN_HAUPT-G.md` gemeldet. Andere G-Worker schreiben diese Dateien nicht. F arbeitet in einem fremden Worktree, dessen uncommittierter Stand wird nicht gelesen und kopiert. Spätere E/F-Integration ist Aufgabe der Bereichsführung. Bestehende Verbraucher müssen kompilieren; Änderungen außerhalb dieser Grenze zunächst mit konkretem kleinem Bedarf zurückgeben, nicht selbst ausweiten. Rust only, keine neuen Code-Kommentare, ENV-Konfiguration, Modelle oder festen Timeouts.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, HEAD `96e6a8da`, Produktbasis `bfda408c`. Recherche- und Aufgabenunterlagen liegen uncommittiert; erhalten. G0 baut parallel ausschließlich `brain-contracts`. Kein Commit, Push, Merge, Cleanup, Produktions-DB, Releasebuild, Dienststart oder Neustart durch dich. Kein weiterer Agent/Workflow/T3-Thread.

Graphify zuerst, vorhandenen Graph explizit über `--graph /home/nathanael/repos/Deadlock-Brain/graphify-out/graph.json`; danach gezielt Quellen lesen. Kein Graph-Neubau. Eigener Debug-Target, vorhandener Buildslot und höchstens drei Cargo-Jobs. Keine unkoordinierten Compilerläufe.

## 4. Beweisziel

Compiler, Format, striktes Clippy und tatsächlich betroffene Bestandssuites prüfen. Reasoner-Baseline 287 passed, 4 failed, 0 ignored, 3 gefiltert; vier fehlende `hero_build_id`-Fixturefälle sind nicht pauschal Produktfehler. Eigene neue Typen dürfen bestehende Suites nicht brechen. Isolierte Postgresprüfung nach bestehendem Verfahren, nie Produktion.

Echte versionierte API-/Sheetproben für Haze, Warden, Wraith sowie Abrams/Burst/Einzelnachladen verwenden. Zustände 0 und 35 Boons, 0 und insgesamt 38 Spirit, gleiches Ziel sauber trennen. Wardens Basis 17,34 ist nicht Sheet-17,3. Rechenergebnis gegen Originalfelder, nicht gegen eine passend veränderte Messgrundlage prüfen. Fehlende E-Laufzeitbindung nicht als Rust- oder Produktionsabnahme ausgeben. Alle 13 Tabs als implementiert, kaputt, subjektiv oder an konkrete Daten-/Regellücke gebunden zuordnen; keine still ausgelassenen Teilbereiche.

Gate ist der einzige Bug-/Securityreviewer. Übergib schmale prüfbare Teilstände und Befehllogs an Bereichsführung; sie fährt `gate_hook.py --review`, frischen Fixer bei BLOCK. Kein eigener Reviewagent. Bei echtem nicht baubarem Fremdvertrag den nutzbaren unabhängigen Stand und `ABWEICHUNG:` zurückgeben, kein Ersatz auf Vorrat.

## 5. Routing und Rückgabe

Auftraggeber native G-Bereichsführung, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Worker ist Statusproduzent dieses Versuchs. Rückgabe über nativen Workflow, keine Sessionnachrichten oder Nutzerfragen. Optional `G/G-M-VERTRAG.md` und `G/G-M-NACHWEISE.md`; Register, AN_HAUPT und TODO nicht ändern. Erste Wache nach 20 Minuten. Gib genaue Funktionen/Typen, Dateien, Compile-/Lint-/Testzahlen, echte Zahlenbeispiele, Mechanikherkunft und benannte unbekannte Regeln zurück. Deutsche Produkttexte mit echten Umlauten, keine Gedankenstriche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
