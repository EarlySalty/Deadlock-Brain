# Ask-Anbindung und Publish-Format

Stand: 2026-09-13, Abschlussintegration. Dateien: `dbrain-retrieval/src/lib.rs`, dessen `Cargo.toml` und `dbrain-reasoner/src/publish.rs`.

Buildfragen über `ask-context` rufen dieselbe Reasoner-Fassade wie `reason build` auf. `use_ai=false` und `persist=false` verhindern weitere KI-Aufrufe und automatische Schreibzugriffe. Die Antwort übernimmt Kaufphasen, Reihenfolge, Items, Warum-Texte und belegte Quellen aus dem Reasoner. Bei fehlendem Warum-Text dienen vorhandene Quellen als Erklärung; fehlen auch diese, wird das offen gesagt. Eine erfragte besondere Spielweise wird nicht als berechnete Variante ausgegeben, solange der Reasoner dafür keinen Parameter anbietet.

Die alten Build-Ranking-Abhängigkeiten wurden aus Retrieval entfernt. Andere bestehende Build-Kommandos und die Legacy-Python-Pipeline bleiben eigenständige Pfade; sie werden durch diesen begrenzten Ask-Auftrag nicht auf den Reasoner umgestellt.

## JSON-Vertrag

`result_text`, `validation` und `prompt` bleiben vorhanden. Insbesondere verwendet der Rust-CLI `ask-context --prompt-only` weiter das Feld `prompt`. Die Antwort ist deterministisch, weshalb die Validierungsliste keine neu erfundenen KI-Items prüfen muss. Das Promptfeld enthält den tatsächlichen Reasoner-Build und behauptet keine Berechnung aus einem Winrate-Ranking.

**Bewusste Schemaänderung:** `build_context` enthält jetzt ein `BuildObject` mit `core`, `situations`, `ability_order`, `rationale` und den übrigen Reasoner-Feldern, statt des früheren Ranking-`BuildContext` mit `primary_path`. Die Antwort kennzeichnet das mit `build_context_schema = reasoner_build_v1` und `retrieval_meta.route = build_reasoner`. `playstyle` bleibt als Feld erhalten und ist null; ein erkannter Wunsch steht in `requested_playstyle`, `playstyle_applied` ist false. Es werden keine erfundenen Winrate-, Stichproben- oder Archetypwerte als Kompatibilitätsdaten angelegt.

Verbraucherprüfung in Rust-CLI, Retrieval und Repo-MCP: kein produktiver Leser von `build_context.primary_path` im Ask-Ergebnis gefunden; der CLI gibt JSON aus oder liest `prompt`. Externe, hier nicht vorhandene JSON-Verbraucher müssen anhand der Schemamarkierung auf `core`/`situations` umstellen. Die separate Legacy-Python-Pipeline ist kein Leser des neuen Rust-Ask-Ergebnisses.

## Publish

Die Kernkategorie heißt `Kern`, begrenzte Situationskäufe heißen `Ein Item nach Bedarf` beziehungsweise `Bis zu N Items nach Bedarf`. Die übrigen Labels stammen aus dem deutsch beschrifteten Composer. Die volle Begründung bleibt in der Buildbeschreibung; Kategoriebegründungen verwenden kurze Mechanik-/Patchbelege, werden einzeilig dargestellt und sind auf 400 Zeichen begrenzt. Layoutmaße, Itemannotation, Imbue-Ziel, Verkaufspriorität und Fähigkeitsreihenfolge bleiben erhalten.

## Prüfung

- Retrieval-Suite: 22 bestanden, 0 fehlgeschlagen, 12 unverändert ignorierte DB-Tests. Der neue Test prüft tatsächliche Reihenfolge, originale Warum-Texte, belegte Ersatztexte, Situationsitems und den ehrlichen Spielweisenhinweis.
- Publish-Roundtrip: bestanden; zusätzlich deutsche Kategoriebezeichnungen und Trennung zwischen kurzer Kategorie- und vollständiger Buildbeschreibung geprüft.
- Bestehender Katalogtest für bekannte Itemnamen bleibt als DB-Test erhalten; sein Helfer ist jetzt ausschließlich Testcode, da deterministische Ausgabe keinen zweiten KI-Namensabgleich benötigt.
- Unabhängiges Gesamturteil erfolgt durch einen frischen Reviewer, nicht durch den Autor dieser Änderung.
