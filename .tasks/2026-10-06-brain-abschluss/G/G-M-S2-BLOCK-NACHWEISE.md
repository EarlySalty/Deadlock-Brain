# G-M-S2: Blocker nach fünf frischen Fixrunden

status: S2 BLOCK, keine weitere Runde gestartet, 07.10.2026

## Verifizierter Stand

Eigener Worktree /home/nathanael/.worktrees/brain-g-v2-20261007, Branch feat/brain-v2-g-20261007. Lokaler HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206. Bereichsführung bestätigte die sechs S2-Commits mit git log, den gleichen committed Prüfbaum-HEAD, leeren Primärindex und aktuelles Quellenmanifest 12/12 mit Exit 0. Eigenes ls-remote bestätigt weiterhin origin-bd83d7abdef812a30daa47aa5f6a78de4f42263a. S2 nicht gepusht.

Compiler, striktes Clippy einschließlich Abhängigkeiten und Formatcheck auf committed Endstand: Exit 0, Rohlogs durch Bereichsführung gelesen. Tests nicht ausgeführt. Kein neuer Zahlen-/Test-/Produktivbeweis und keine Änderung der historischen Suitezahlen.

Endbeleg samt Commitkette und Prüfgrenzen: G/pruefungen/g-m-checkpoints-r2/endstand.txt. Fünf frische Fixer nach erstem S2-Gate, insgesamt sechs Gruppenanläufe gegen S1. Letzter Rohgate durch Bereichsführung gelesen, Exit 1: `[gpt-6.1-sol] BLOCK: stack damage skips shred, and duplicate items replay effects.`

## Zwei bestätigte Restkerne

1. Stackschaden, committed combat.rs:1901, Arbeitsquelle :1927: Im gemeinsamen Schussereignis bekommt der Stackbonus Waffenverstärkung, aber im Default-Planerpfad keine Rüstungsverringerung. Der reguläre Bulletanteil enthält dort bereits bullet_shred (:1325); der gemeinsame Receiver erhält bei fehlendem explizitem Szenario dagegen 0 (:1931). Der Receiver (:940) ergänzt diesen fehlenden Wert nicht. Beispiel des Gates: 10 Stackbonus mit 20 Prozent Shred bleibt 10 statt 12. Default-, Fast- und Bindingpfade betroffen. Nach Graphify an den tatsächlichen Arbeitsquellen bestätigt. Lösungsrichtung: denselben vorhandenen Shred-Schritt auch auf den Stackanteil anwenden; beim expliziten Szenario keinen zweiten Shred einführen.
2. Doppelte Items, committed combat.rs:828, Arbeitsquelle :810/:820/:841: Beide öffentlichen simulate_calculation-Eingänge reichen items über items.iter().collect direkt an simulate weiter. Dort entstehen Interaktionen und Zustandsfelder je Vorkommen (:1027), wodurch Procs und Auffüllungen mehrfach laufen. evaluate_core_with_deadline normalisiert dieselben Item-IDs bereits einmal (:545), bevor es die weiteren Pfade aufruft. Nach Graphify bestätigt. Lösungsrichtung: dieselbe vorhandene ID-Normalisierung am gemeinsamen Simulationsübergang benutzen, damit Stat-, Shop- und Effektpopulation übereinstimmen. Kein neuer Inventar- oder Effektpfad.

Gate-NIT bleibt getrennt: committed combat.rs:2247 meldet Zeitauflösung 0 trotz gerasterter Zielwechsel im Druckszenario. Kein BLOCK und keine zusätzliche Korrektur als erledigt ausgeben.

## Erhaltene Fixes und Eskalation

Erhaltene frische Commits korrigieren Kontaktzeit nach früherem Kill, ProcChance, Waffenverstärkung auf Stackbonus, Teilauffüllung sowie abgeschlossenes Nachladen/Druckdauer. Kein Verwerfen oder Neubeginn. Elf Startquellen einschließlich Originalfixtures bytegleich; ausschließlich combat.rs durch die beauftragten BLOCK-Fixes verändert. S3-Deadlinewrapper bleibt unstaged, S3/S4 nicht begonnen.

Nach fünf erfolglosen Fixrunden Paketende und Eskalation über AN_HAUPT-G.md. Empfehlung: eng begrenzte Fortsetzung für diese beiden bestätigten Semantikkerne in combat.rs durch frischen nativen Fixer, anschließend derselbe volle S2-Gate gegen S1 und dasselbe Urteilmodell. Erst echtes ALLOW und committed Prüfungen gestatten origin-Sicherung sowie S3/S4. Keine zusätzliche Runde oder Befugnis aus der automatischen Abschlussmeldung ableiten.

Wache 2c0c63e6 nach tatsächlicher Rückgabe gelöscht. Keine aktiven eigenen Produktwriter mehr bekannt. Kein Main, Release, Deploy, Neustart, Livebeweis, Cleanup oder Settle. Zentrale TODO unverändert.

Workerprotokoll laut Endbeleg: 158 Git-Schritte. Dazu fünf einzelne lesende Git-Prüfungen der Bereichsführung: Commitkette, Primärstatus, origin, Prüfbaum-HEAD und leerer Index. Keine schreibende Eltern-Gitwirkung.

MERGEPROTOKOLL[MS-1]: 163 Git-Schritte einzeln | Anläufe: 6 | Gate: gpt-6.1-sol BLOCK; kein Main-Merge
