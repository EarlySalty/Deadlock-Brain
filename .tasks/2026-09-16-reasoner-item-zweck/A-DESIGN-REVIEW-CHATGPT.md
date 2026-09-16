# Unabhängige Entwurfsprüfung A durch ChatGPT

Stand 16.09.2026. Geprüft: Commit 806d4d58374eb3b3d59edec4f5d50d53700eaaf8, Datei A-DESIGN.md im Worker-Branch feat/reasoner-purpose-a. Zusätzliche selbst gelesene Ausgangsstellen: rust/crates/dbrain-reasoner/src/data.rs (scaling_stats, ability_model, weapon_profile, hero_model), item.rs (score_item, spirit_fire_rate_value, meta_value), combat.rs (vorher in dieser Akte belegte spirit_rate-Ableitung). Autor der Entwurfsnotiz ist der laufende Opus-Worker; diese Prüfung stammt nicht vom Implementierer.

Urteil: BLOCK für den vorgeschlagenen reduzierten Phase-A-Abschluss. Das ist keine negative Abnahme eines noch nicht vorhandenen fertigen Produktpatches. Weiterarbeit im vorhandenen Thread, kein zusätzlicher Claude-Worker.

## A-R1: damage_plan absichtlich auszulassen verfehlt den Auftrag

A-DESIGN.md, Abschnitt Phase-A-Änderung, Punkt 4: keine Änderung an damage_plan-Magnituden wegen Regressionsrisiko. Phase A verlangt aber gerade einen gemeinsamen Konversionspfad einschließlich damage_plan, damit die Hero-Identität und spätere Auslöse-/Inventarbewertung nicht mit anderen Waffenanteilen rechnen als die Simulation. Eine kosmetische Zusammenlegung von zwei Helfern bei unverändertem drittem Blindspot reicht nicht.

Minimaler korrekter Weg: eine kanonische, zustandsbezogene Waffenberechnung; Loader/Mechanics/Combat/Item-/Planner-Konsumenten daran anbinden. damage_plan muss dieselbe reale Basis-SP bzw. einen explizit übergebenen Buildzustand verwenden, nicht eine erfundene feste Spiritmenge. Bei Basis-SP=0 darf der nackte Held unverändert bleiben. Ein Null-Delta auf diesem Sonderfall ist kein Beweis für fehlende Integration.

Gegenprobe: dieselben generischen Waffendaten mit S=0 und S>0 sowie belegter Spirit-Konversion; damage_plan und Combat müssen im gleichen Zustand dieselbe Konversionsrichtung und Einheit verwenden. Ein Held ohne Konversion darf allein durch mehr Spirit keinen Waffenbonus erhalten. Nicht allein den Helfer testen, sondern die öffentlichen Konsumenten. Regressionsrisiko messen und dokumentieren, den Auftrag nicht zur Sicherung unveränderter Metriken verkürzen.

## A-R2: eine negative Kante ist nicht eine fehlende Kante

A-DESIGN.md, Abschnitt Vorzeichenregel: negative/nicht endliche Skala als fehlend behandeln, Begründung negativer Spiritwert sei kein Waffennutzen. Das verwechselt Nutzen mit Mechanik. Eine endliche negative Konversion ist eine mögliche Downside und darf nicht verschwinden. Dass sie im gegenwärtigen Modell-Roster nicht vorkommt, beweist weder korrekten Parser noch zukünftige Unmöglichkeit.

Minimaler korrekter Weg: endliche Werte signiert erhalten; resultierende physikalische Endwerte erst an der geeigneten Grenze begrenzen. Fehlend, explizit Null, negativer Wert und ungültiger Wert getrennt behandeln. Bei ungültigen Daten keinen angeblich sicheren hohen Confidence-Wert; definierter Fehler-/Unknown-Pfad statt stiller Ersatz durch einen anderen Alias. Die Alias-Priorität aus Daten belegen, niemals beide addieren.

Algebraische Gegenprobe, keine ausgeführte Produktmessung: r0=4 Schuss/s, k=-0,01 Schuss/s je Spirit, S=50 ergibt r=3,5 vor weiteren Modifikatoren. Das Entfernen der Kante ergäbe fälschlich 4. Bei direktem k=0 ist ein vorhandenes Fallback kein Anlass, doch eine zweite Konversion einzuschalten. Positive, negative, fehlende, explizit-null und ungültige Eingaben sowie widersprüchliche Aliase getrennt testen.

## A-R3: Modell-Roster ist nicht Rohdaten-Abdeckung

Das Coverage-Werkzeug liest bereits normalisierte HeroModel.scaling. Damit kann es fehlende Werte im Loader nicht nachweisen. Es ist eine gute Modell-Gegenprobe, aber kein Abschlussbeleg für alle aus Rohdaten verfügbaren Waffenstat-Konversionen.

Minimaler korrekter Weg: Roh-JSON-Pfad, Eingangsstat, Ausgangsstat, Einheit, Wert, Snapshot-/Versionsbezug und tatsächlich verwendeter Produktionspfad pro gefundener Kante im Bericht. Mindestens die im Auftrag geforderten unterschiedlichen belegten Helden; nicht vorhandene Rohdaten ehrlich offenlassen. Eine feste zusätzliche Struct-Form ist nicht Selbstzweck: vorhandene ScalingStat-Daten können wiederverwendet werden, sofern die geforderte Semantik wirklich durchgereicht wird. Keine zweite Engine.

## A-R4: neue Baseline nicht als Reparaturerfolg missdeuten

T3-Meldung vom 16.09.2026 15:26-15:27 UTC: FROZEN-V2 plus separat eingefrorene Population, neu berechnet, meldet 6/9 Referenzwaffen, 9/10 Staples, Kendall 0,471 und Jaccard@12 0,5. MR/Rusted Barrel/Healing Tempo seien bereits VOR Phase A nicht im Kern. Das ist zunächst eine Worker-Meldung, kein hier aus JSON reproduziertes Resultat.

Konsequenz: ihr späteres Fehlen darf nicht als Wirkung des A-Patches verkauft werden. Die historische Live-Veröffentlichung und dieser Replay haben andere Eingaben. Beide Codefassungen auf identischen Input-Dateien rechnen; alle Metriken nach Quelle trennen, Gesamt-Staple-Gate bleibt rot. Den abgebrochenen Vollfreeze als Performance-/Abdeckungslücke dokumentieren. Letzter geloggter Held beweist nicht die Identität des hängenden nächsten Helden.

## Status der Prüfung

Entwurf und Ausgangscode tatsächlich gelesen; keine neuen Modelle, keine zentralen DB-Abfragen oder Änderungen, kein Repo-Testlauf für diese Prüfung, kein Produktpatch, kein Merge-/Deploy-ALLOW. Die genannten Gegenproben sind konkrete Testanforderungen, keine erfundenen bestandenen Tests. Bestehenden Worker mit den Punkten A-R1 bis A-R4 weiterführen.
