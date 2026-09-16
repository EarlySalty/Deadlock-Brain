# Unabhängige Sichtprüfung des ersten A-Produktpatches

Geprüfter Commit: f68dcc3d54047e164d3217d972addc9852184389, Worker-Branch feat/reasoner-purpose-a, 16.09.2026. Prüfer: ChatGPT im vorhandenen Chat, kein neuer Claude-Reviewer. Vollständiger Commit-Diff sowie item.rs::spirit_fire_rate_value und mechanics.rs::weapon_dps tatsächlich gelesen. Diese Datei ist keine selbst ausgeführte Rust-Testsuite. Worker meldet 181 bestandene Tests, 16 ignorierte, Clippy sauber; diese Angaben sind bis zu eigener Reproduktion Worker-Nachweise.

Urteil: BLOCK für eine Phase-A-Abnahme. Nicht nach main integrieren oder veröffentlichen. Der vorhandene Worker bearbeitet die konkreten Punkte, keine neue Implementierung parallel beginnen.

## Bereits verbessert

Eine gemeinsame Funktion ersetzt die separaten Feuerraten-Ableitungen. damage_plan bezieht nun base_spirit_power ein. Endliche negative Konversion bleibt im Helfer signiert. Das korrigiert zwei wesentliche Abweichungen des ursprünglichen Entwurfs. Die neuen Tests prüfen diese Richtungen; daraus folgt noch keine Vollständigkeit des Assets-/Inventarpfads.

## R1: Item-Grenzwert verwendet weiterhin einen anderen Spiritzustand

Ort: item.rs::spirit_fire_rate_value, Closure dps (ungefähr Zeilen 190 bis 204).

Der Closure klont hero.weapon und addiert nur spirit * per_spirit. hero.base_spirit_power fehlt. passive_dps wird dadurch relativ zur nackten Waffe berechnet. Der neue damage_plan und combat beziehen Basis-Spirit dagegen ein. Eine gemeinsame Koeffizientenfunktion allein vereinheitlicht daher nicht die Zustandseingabe. Durch Nachladen ist der Grenzwert nicht linear; der Fehler verschwindet nicht durch einfaches Abziehen des Vorherwertes.

Algebraische Gegenprobe auf genau der gelesenen weapon_dps-Formel, keine behauptete ausgeführte Rust-Messung: bullet_damage=10, clip_size=16, reload_duration=2, Basisfeuerrate=4, Basis-Spirit=50, Konversion=0.01 Schuss/s pro Spirit, passives Testitem +100 Spirit ohne weitere Boni. Aktuell verwendet der Score DPS(5)-DPS(4)=160/39=4.102564102564102. Mit demselben realen Basiszustand wie damage_plan muss es DPS(5.5)-DPS(4.5)=512/135=3.7925925925925927 sein. Die Bruchrechnung wurde separat geprüft.

Fixanforderung: dieselbe kanonische zustandsbezogene Waffenprojektion verwenden, Basis-Spirit genau einmal einbeziehen und dann den Itemzuwachs auswerten. Nicht hero.weapon dauerhaft vormodifizieren und im Combat anschließend doppelt skalieren. Gegenprobe am öffentlichen Item-Consumer mit Basis-Spirit >0 und Kontrollfall Basis-Spirit=0; nicht nur am Helfer testen. Inventar-Compounding jenseits dieses Ausgangszustands bleibt Phase D.

## R2: 'Unknown' wird im Ergebnis nicht sichtbar

Ort: mechanics.rs::spirit_weapon_rate_per_spirit und spirit_weapon_rate_drops_non_finite_as_unknown.

.filter(is_finite) plus .unwrap_or(0.0) wirft fehlerhafte Werte weg. Der Rückgabetyp f64 unterscheidet danach nicht zwischen fehlend, explizit null und ungültig. Bei ungültigem Primärwert wird still der zweite Alias genommen. Weder der gezeigte Helfer noch seine drei neuen Aufrufstellen ergänzen dazu einen Unknown-/Fehlernachweis. Der Kommentar bzw. Committext 'unbekannt' ist deshalb kein beobachtbares Verhalten.

Fixanforderung: Fehler-/Provenienzstatus einmal an der passenden Modell-/Validierungsgrenze ermitteln und im bestehenden Unknown-/Assumptions-/Confidence-Pfad sichtbar halten, ohne teure Diagnosearbeit pro Simulations-Tick einzuführen. Eine fachlich begründete Recovery aus einem nachweislich verwendbaren Alias ist möglich, aber als solche zu kennzeichnen. Ohne validierten Ersatz darf keine bestätigte Null-Konversion mit hoher Sicherheit entstehen. Test muss nicht nur den Zahlenwert, sondern den sichtbaren Fehlerstatus prüfen. Die passende Erwartung nicht allein deshalb abschwächen, weil der aktuelle Code sie verfehlt.

## Noch benötigte Abschlussbelege

Die neue Messung vor/nach f68dcc3 auf identischen Dateien steht aus. Die 38 Zeilen in PHASE0-COVERAGE.json zeigen normalisierte Modelle; sie ersetzen weiterhin keinen Abgleich der Konversionen aus Rohdaten und keine drei datenbelegten Hero-Fixtures. Der abgebrochene Gesamtfreeze bleibt Performance-/Abdeckungsblocker, bis der betroffene Fall sauber identifiziert und gemessen ist. Ein besseres Ergebnis auf geänderten Eingaben ist kein Code-Delta.

Zusätzliche Quellenarbeit und 18 synthetische Mechanik-Gegenproben stehen in ASTRA-API-PRUEFUNG.md bzw. ASTRA-PRUEFFAELLE.json (gesichert in 5ac9442). Bedingungs-Akteur, Refresh/Stacks und typed Shields bleiben Folgephase B/C, keine verdeckte Erweiterung dieses A-Fixes.
