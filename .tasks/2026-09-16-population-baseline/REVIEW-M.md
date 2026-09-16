# Review Paket M: Populations-Signal und Warden 6/9 (Runde 1, review_1)

**Urteil: NACHBESSERN**

Unabhaengiger Review des Diffs `fc71b5d..3ff5db4` in `rust/crates/dbrain-reasoner`
(M-Commits `b21b3ea`, `c01f5a8`, `eac1875`, `ef94952`, `86804c0`, `3ff5db4`),
ohne die separat freigegebenen P-Commits. Der Autor war ich nicht.

Das Ergebnis ist echt und der Massstab ist erfuellt: Warden 779996 v45 kommt mit
Population auf 6/9 Referenzwaffen (Staple-Gate bestanden, tau 0,639, Jaccard@12
0,600), belegt durch die aufgezeichnete Datei `M-SECHS-pop.json`; ohne Population
bleibt es bei 3/9 wie auf `fc71b5d`. Kein Referenz-Itemname, kein
heldenspezifisches Gewicht, keine Seed-Kopie im Produktcode; der Prior ist
helden-unabhaengig (per Item-ID). Reasoner-Lib 170 bestanden, 16 ignoriert, 0 rot;
Clippy `-D warnings` auf `dbrain-reasoner` und `dbrain-population` sauber (selbst
gebaut, Debug, kein `--release`). Die Metriken `kendall_tau` (tau-b mit Bindungen)
und `jaccard_at` sind konsistent mit dem in P per SQL bestaetigten Bezug.

Es gibt keinen blockierenden Mangel. Die Nachbesserung betrifft vier wichtige
Punkte: ein zweiter Env-Name im Example, ein die Mechanik ueberspielender
Staple-Hebel mit dazu ueberzeichneter nutzersichtbarer Begruendung, und ein
falscher Nachweis-Zeiger fuer die 6/9 in der Messakte.

## Verifikation (belegt)

- Testlauf selbst: `cargo test -p dbrain-reasoner --lib` = 170 bestanden, 16
  ignoriert, 0 fehlgeschlagen (Fertigmeldung bestaetigt). Clippy `-D warnings`
  auf beiden Crates ohne Warnung, Exit 0. Den vollen Workspace (Anspruch 384/58)
  habe ich nicht gefahren, da viele Faelle eine DSN brauchen; der Reasoner-Teil
  stimmt.
- 6/9 gegengeprueft an den Nachweisdateien: `M-SECHS-pop.json` zeigt Warden 25
  Build 779996 v45 mit `weapon_hit_count` 6, `staple_gate_passed` true,
  `missing_staples` leer, `kendall_tau` 0,6387, `jaccard_at_12` 0,600.
  `M-SECHS-nopop.json` zeigt dieselbe Referenz mit 3. Alle sechs Helden-Zeilen
  der M-MESSUNG-Tabelle decken sich Zahl fuer Zahl mit diesen zwei Dateien
  (Infernus 3/7 zu 4/7, Lady Geist 1/7 zu 3/7, Abrams 0/7 zu 3/7, Vindicta 1/3
  zu 2/3, Bebop 0/4 zu 0/4, Ivy 2/3 zu 2/3 mit Gate null).
- Kein Warden- oder Referenz-Sonderfall im Produktcode. Grep ueber `src/` und
  `build.rs`: die einzigen Treffer auf `hero_id != 25` bzw. `"Warden"` liegen im
  Testmodul (`composer.rs:1388`) und im Beispiel `abschluss.rs`, beide
  vorbestehend und nicht Teil des M-Diffs.
- Prior wirkt nur als Meta-Stuetze, nicht als Abschreiben: `support`
  (`population_prior.rs:80-86`) rechnet `POPULATION_PRIOR_WEIGHT * prevalence *
  mechanic_slot_value.max(0.0)`, ein negativer Solo-Mechanikwert ergibt 0 (Test
  `support_scales_with_prevalence_and_positive_mechanics_only`). Blood Tribute
  (negativer Solo-Wert) bleibt laut Messung ausgeschlossen. Die 70-Prozent-Staples
  entstehen aus rund 4017 Warden-Spielern; eine einzelne Referenz kann die
  Prevalenz nicht auf 70 Prozent heben, das Signal ist also echte Population.
- Produktivpfad unveraendert: `load_reasoning_inputs` (`lib.rs`) setzt
  `population: PopulationPrior::default()`, der Composer reicht `Some(&empty)`
  durch, im Planner ist dann `is_staple` immer false und `support` 0, die
  Sortierung faellt auf den alten Vergleich zurueck. Verhalten identisch zu
  vorher, belegt durch die Vorher-Baseline 3/9 gleich `fc71b5d`.
- Gate-NIT combat: `bindings` ist sauber in `active_bindings` umbenannt
  (`combat.rs`, vier Stellen), keine Verschattung mehr.
- Keine Em-Dashes, keine Code-Kommentare in den neuen oder geaenderten Dateien
  (grep U+2014 und `//` leer); echte Umlaute in der nutzersichtbaren Begruendung.

## Maengel

1. `examples/build_evaluation.rs:455`, wichtig. Das Mess-Example liest die
   Population ueber einen zweiten Env-Namen `POPULATION_DB_DSN`
   (`std::env::var("POPULATION_DB_DSN")`). Der Review-Massstab dieses Pakets
   verbietet ausdruecklich einen zweiten Env-Namen wie `POPULATION_DB_DSN in
   Produktcode oder Example, und Paket P bezieht seine DB bewusst nur ueber
   `DEADLOCK_CENTRAL_DSN` (REVIEW-P, kein zweiter Env-Name). M weicht von diesem
   Muster ab. Erwartet: die Messung ueber denselben zentralen Pool bzw.
   `DEADLOCK_CENTRAL_DSN` fuehren (auf die Wegwerf-DB `population_dev` gezeigt),
   sodass im ganzen Baum genau ein DSN-Name existiert. Nebenpunkt am selben Ort:
   ein ungesetztes DSN liefert still einen leeren Prior; fuer die spaetere
   Produktiv-Verdrahtung muss das Fehlen der Population sichtbar sein statt
   stumm.

2. `planner.rs:119-122, 209, 250, 271-274, 455-458, 470-471`, wichtig. Die
   kategorische Staple-Prioritaet und die Umgehung der Marginal-Untergrenze sind
   nur am Solo-Wert `per_slot_value > 0` verankert (`is_priority_staple`), nicht
   am tatsaechlichen Kontext-Marginalwert. Dadurch wird ein Staple, der solo
   positiv, im aktuellen Inventar aber redundant oder leicht anti-synergetisch
   ist (`marginal_value <= 0`), trotzdem an der Untergrenze vorbei aufgenommen
   (`&& !forced`), in Auswahl und Beam strikt vor alle Nicht-Staples sortiert und
   nie zugunsten des Sparens zurueckgestellt. Der Solo-Schutz gegen negative
   Items greift also nicht im Kontext. Das ist der Hebel hinter dem Sprung 3/9 zu
   6/9; er ist als Meta-Signal zulaessig und fuer Warden unkritisch (die zehn
   Staples sind stark), bleibt aber als latenter Weg bestehen, ein im Build
   mechanisch nicht mehr positives Item allein wegen Prevalenz zu kaufen.
   Erwartet: die kategorische Prioritaet bzw. die Untergrenzen-Umgehung
   zusaetzlich an einen nicht-negativen Kontext-Marginalwert binden, oder die
   Grenze des Schutzes offen dokumentieren.

3. `composer.rs:570`, wichtig. Die nutzersichtbare Staple-Begruendung sagt "der
   Kaufanteil stuetzt nur populaere Items mit tragfaehigem Mechanikwert". Die
   tatsaechliche Schwelle ist lediglich `per_slot_value > 0`: ein Staple mit
   Solo-Wert +0,1 wird kategorisch genauso bevorzugt wie einer mit +50, und die
   additive Stuetze skaliert linear ab jedem positiven Wert. "Tragfaehig" wird
   nicht erzwungen, die Begruendung ueberzeichnet damit die Rolle der Mechanik.
   Erwartet: den Satz an die echte Bedingung angleichen (etwa nicht-negativer
   Solo-Mechanikwert) oder die Schwelle wirklich anheben, damit die Begruendung je
   Item ehrlich bleibt.

4. `M-MESSUNG.md` (Abschnitt Nachweisdateien), wichtig. Als Warden-Nachweis
   nennt die Akte `M-WARDEN-nopop.json` und `M-WARDEN-pop3.json`. Eine
   `M-WARDEN-pop3.json` existiert im Nachweisordner nicht; die vorhandenen
   Einzellaeufe `M-WARDEN-pop.json` (3/9) und `M-WARDEN-pop2.json` (4/9, dazu
   `population_backtest` untracked, also ohne geladene Population) belegen die 6/9
   gerade nicht. Die 6/9 ist ausschliesslich durch `M-SECHS-pop.json` gedeckt.
   Der Befund selbst ist echt, aber der zitierte Einzelnachweis ist falsch.
   Erwartet: den Nachweis-Zeiger auf `M-SECHS-pop.json` korrigieren und die
   irrefuehrenden Zwischenlaeufe entfernen oder als Zwischenstand kennzeichnen.

5. `M-MESSUNG.md` und `FERTIG-M.md` (Spielerzahl), nit. M nennt 80641
   Spieler-Matches aus 10000 Ranked-Matches, REVIEW-P und FERTIG-P nennen 119248.
   Die lokale `population_dev` haelt jetzt 119248 Spieler-Matches ueber 10059
   Matches und 38 Helden (selbst per SQL geprueft); alle 119248 Zeilen haben
   positives `average_badge`, gesetztes `won` und Items, ein einfacher
   Clean-Filter erklaert die 80641 also nicht. 80641 ist damit vermutlich ein
   aelterer Ingest-Zwischenstand. Auf die in P per SQL bestaetigten Aggregate und
   die 6/9 hat das keinen Einfluss. Erwartet: die beiden Zahlen in der Akte
   angleichen und die Herkunft der 80641 erklaeren.

6. `build.rs:12`, nit (Hinweis, akzeptabel geloest). Der Rerun-Sentinel bleibt
   ein nie existierender Pfad
   (`nonexistent-sentinel-forces-build-provenance-every-build`). Das ist das
   uebliche Cargo-Idiom, um die Provenienz bei jedem Build frisch einzubetten;
   der irrefuehrende Kommentar ist entfernt, der Name traegt den Zweck. Kein
   Aenderungsbedarf, nur als bewusste Entscheidung notiert.

7. Commit-Betreffzeilen (`86804c0` Prioritaet, `ef94952` Stuetze), nit. ae/ue
   statt echter Umlaute. Bereits committet, nur kosmetisch, kein Neuschreiben
   noetig.

## Zu den Schwerpunkten

- Ehrlichkeit (1): Der Fortschritt ist ein echtes, deklariertes Meta-Signal, kein
  Abschreiben der Population. Der Schutz gegen solo-negative Items existiert und
  ist getestet. Die Schwaeche liegt in der Staerke des Hebels (Maengel 2 und 3),
  nicht in einer Verkleidung des Ergebnisses.
- Metriken (2): Staple-Gate gegen `BUCKET_ALL` ist fuer einen generischen Build
  begruendet; tau-b mit Bindungen und Jaccard@12 sind korrekt, die fehlende
  Spieler-gegen-Spieler-Decke ist als ABWEICHUNG deklariert; Null-Faelle
  (weniger als zwei gemeinsame Items, leere Mengen, leerer Prior) sind sauber
  abgefangen.
- Planner (3): Determinismus bleibt (deterministische Tiebreaks nach
  `purchased_id`), keine Endlosschleife (Kaeufe bleiben an `preview_purchase`
  gebunden, betroffen ist nur die Sortier- und Untergrenzen-Logik), Slot-Deckel
  und Budget unveraendert. Der neue Pfad ist per Tests abgedeckt.
- Datenfluss (4): siehe Mangel 1 und die belegte Vorher-Gleichheit.
- Gate-NITs (5): combat sauber; build.rs akzeptabel (Mangel 6).

## Zusammenfassung

Urteil: NACHBESSERN
Blockierend: 0
Wichtig: 4
Nit: 3
