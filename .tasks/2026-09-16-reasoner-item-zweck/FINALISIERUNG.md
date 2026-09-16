# Finalisierung durch ChatGPT

Stand 16.09.2026. Eigener Worktree `/home/nathanael/repos/wt/brain-purpose-finish`, Branch `fix/reasoner-mechanics-completion`. Keine neuen Claude-/Opus-Threads oder Ersatzanbieter. Der vorhandene A-Worker ist fertig (T3 ready, HEAD 46f8bfb). Seine Änderungen sowie der bereits geprüfte Rohdatenprüfer 35b1a0c sind im Integrationsbranch zusammengeführt, noch nicht auf main.

## Tatsächlich ergänzter Produktcode

`mechanics::WeaponSpiritScaling` projiziert die aus dem Heldenmodell gelesenen absoluten Waffenachsen EBulletDamage und EClipSize zusätzlich zur bereits vereinheitlichten Feuerrate. Keine Hero-/Itemnamen oder IDs als Bewertungsregeln. Ausgangswaffe und Gesamt-Spirit sind explizit; Item-Prozente wirken danach. Endliche negative Kanten bleiben erhalten, resultierende Waffenwerte können nicht negativ werden.

Combat, damage_plan und der statische Item-Grenzwert verwenden dieselbe Projektion. Magazin-/Schadenswechselwirkungen mit Imbue berücksichtigen den konvertierten Ausgangsschaden, ohne Weapon-Prozentboni doppelt einzurechnen. Die Scoring-Erklärung nennt Schaden/Schuss, Magazin, Feuerrate und den resultierenden Grenzwert nach Nachladen.

Der statische Spirit-Consumer erkennt dieselben fünf Spirit-Aliase wie der Kampf, liest auch ausschließlich in passive_properties enthaltene Werte und zählt Spiegelungen nicht doppelt. Ein in der Passivliste stehender Wert wird nicht länger wegen der falschen XOR-Bedingung als aktiv behandelt. Der ältere Test `fix_e_fire_rate_percentage_fallback_and_passive_spirit` erwartete genau diesen Fehler: zehn Prozent statt voller Passivwirkung. Seine Erwartung ist begründet korrigiert; der ausdrücklich bedingte Gegenfall behält weiterhin zehn Prozent Wirkung.

Der Hero-Loader lehnt explizit ungültige oder fehlende Koeffizienten bei deklarierter Spirit-Eingangsachse vor der Normalisierung ab, statt die Daten in None/Null verschwinden zu lassen. Im Modell noch nicht unterstützte andere Spirit-Achsen werden als nicht quantifiziert in Evidence und erklärter Kampf-Ausgabe sichtbar; hohe Item-Confidence wird dafür nicht behauptet. Dies ist keine vollständige Implementierung von Sprint, Nahkampf und allen Defensiv-/Fähigkeitsmechaniken.

## Tatsächlich ausgeführte Gegenproben

Die neue Quellen-Gegenprobe gegen RAW-MECHANICS-SUMMARY.json war vor dem Produktfix rot: Spirit→EBulletDamage fehlte in der Projektion. Nachher bestehen die aus fünf Rohdaten-Helden bezogenen Koeffizientenprüfungen für Schaden, Magazin und Rate. Basiswaffen in diesen isolierten Tests sind kontrollierte Testzustände, keine behaupteten vollständigen In-Game-Snapshots.

Weitere Tests: negative Waffenachsen und Endstat-Grenzen, Umbenennungsneutralität, Gleichheit von konvertierter und explizit materialisierter Waffe in der Simulation, gleicher schneller/erklärter Score, alle Spirit-Aliase und passive Spiegelungen, ungültige Rohwerte vor dem Loader-Normalisierungsschritt. Namen und IDs der fünf Quellenhelden stehen ausschließlich im Test.

Befehle im rust-Unterordner mit `/home/nathanael/.cargo/bin/cargo`:

- `test -q -p dbrain-reasoner --lib`: 189 bestanden, 0 fehlgeschlagen, 16 ignorierte DB-Tests.
- `test -q -p dbrain-reasoner --example ability_coverage`: 10 bestanden, 0 fehlgeschlagen, 0 ignoriert.
- `clippy -q -p dbrain-reasoner --lib --examples -- -D warnings`: Exit 0.
- `fmt --manifest-path target/focused-format/Cargo.toml`: Exit 0, nur mechanics/combat/item/data formatiert. Das Hilfsmanifest liegt ignoriert unter target; keine neue Produktlogik oder Abhängigkeit.

## Noch keine Abnahme

Eingefrorener Vorher/Nachher-Backtest dieses erweiterten Produktpatches steht noch aus. Bestehendes rotes Warden-Staple-Gate bleibt rot, bis eine echte neue Messung etwas anderes belegt. Keine neue Veröffentlichung, kein Deploy, kein unabhängiges Gesamt-ALLOW. Eigene Tests sind kein unabhängiges Review.

Beim Lesen des bisherigen Freeze-Werkzeugs wurde zudem ein Fehler der bisherigen Diagnose sichtbar: Nach den protokollierten Helden lädt das Programm nicht zwingend einen weiteren Helden, sondern beginnt eine zweite vollständige, bislang nicht einzeln protokollierte Replay-/Vergleichsschleife. Aus der letzten Meldung Celeste folgt deshalb weder ein hängender Folgeheld noch überhaupt ein bestimmter einzelner langsamer Aufruf. Eindeutige Phasen-/ID-/Laufzeitdiagnose ist erforderlich, bevor ein angeblicher Planner-Blowup als bewiesen gilt.

Offene fachliche Abschnitte bleiben die vollständige Ereignis-/Bedingungsquantifizierung, kanalgerechte Defensive, alle weiteren Heldenachsen und die unabhängige Gesamtprüfung einschließlich Publish-Gates. Keine dieser Arbeiten wird allein durch dieses Dokument gestartet oder als erledigt ausgegeben.
