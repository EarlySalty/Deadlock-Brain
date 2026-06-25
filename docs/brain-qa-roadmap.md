# Roadmap: Vom Daten-Sammler zum Fragen-Beantworter

Stand 2026-06-25. Grundlage: `docs/_work/brain-capability-inventory.md` (vollständige Code-/Daten-Inventur).

## Ist-Zustand — wo das Brain stark ist

Das Brain ist heute eine exzellente **Wissens-Aufbereitungs-Maschine**, aber noch keine **Frage-Antwort-Maschine**. Stark:

- **Ingestion + Normalisierung:** trusted `deadlock-data` (Scaling, Cards, Changelogs), Patchnotes, Sheet, Statlocker → saubere Entities, Aliasse, Patch-Events, Hero-Stats.
- **Entity-Auflösung:** 1.088 Entities, 6.381 Aliasse, Lineage/Legacy — robustes Matching von Namen/Anzeigenamen auf kanonische Entities.
- **Kontext-Assembler:** `context`, `timeline`, `review`, `build` bauen bereits dichten, quellenbelegten Kontext (genau das, was ein RAG-System einem LLM vorlegt).
- **Spezial-Analysen:** `analyze-build`, `analyze-match`, `enrich patch-impact` rufen ein Modell und persistieren Ergebnisse.

## Die Lücke — warum es echte Fragen noch nicht beantwortet

Die Inventur ist eindeutig: **es fehlt der letzte Meter zwischen „Kontext bauen" und „Frage beantworten".** Konkret:

1. **Kein allgemeiner `ask "<Frage>"`-Befehl**, der Frage → Intent → Retrieval → Trust-Gewichtung → LLM-Antwort → Quellen automatisch verbindet. Nur `analysis run-minimax` macht aus Kontext eine Antwort — und das ausschließlich für den Review-Kontext, nicht für freie Fragen.
2. **Intent-Erkennung ist ein grober String-Matcher** (`analyze_query`): findet bekannte Hero-/Item-Namen, fällt sonst auf einen Default zurück; nicht mit den Retrieval-Pipelines verdrahtet.
3. **Creator-Claims hängen in der Luft.** 1.087 Claims (mit Status + Verifier-Verdikten in `verifier_json`) sind NICHT in den Review-/Context-Pfad eingebunden. Kein Join, keine Gewichtung nach Trust/Status/Datum.
4. **Kein systematisches Trust-Modell.** `deadlock-data` ist als trusted markiert, aber das Retrieval gewichtet trusted Ground-Truth nicht konsequent gegen Sheet/Statlocker/Creator-Claims. Keine automatische Konfliktlösung (aktueller Snapshot vs. Patch-Historie vs. Claim).
5. **Keine semantische Suche.** Vektor-/Embedding-Pfad ist bewusst ausgeklammert (`todo!`), `mechanic_notes` ist leer — freie Mechanikfragen finden nichts.

## Die vier Bausteine (in empfohlener Reihenfolge)

### SP-A — Der `ask`-Befehl (das Rückgrat, größter Hebel)
Ein einziger Befehl `ask "<Frage>"`, der die schon vorhandenen Teile verkettet:
Frage → verbesserte Intent-Erkennung → passenden Assembler wählen (`context`/`timeline`/`build`/Item) → Kontext + Quellen → Modell (GPT/Claude für Genauigkeit, MiniMax erst nachgelagert) → belegte Antwort mit Quellenangabe.
**Reuse:** `build_entity_context`, `build_entity_timeline`, `build_review_context`, `prompt_de`, `source_references`, der Modell-Client. **Neu:** nur die Intent→Assembler-Verdrahtung + ein Antwort-Generator mit „belegt vs. wahrscheinlich". Das allein schaltet die Mehrheit der typischen Fragen frei (siehe Q&A-Lückenliste der Inventur).

### SP-B — Trust-gewichtetes Retrieval + Creator-Claims einbinden
Claims (mit L2-Verdikt) in den Kontext einspeisen und **nach Trust-Ebene gewichten**: `deadlock-data` (Ground-Truth) > Sheet/Statlocker > verifizierte Claims > unverifizierte Claims. Bei Widerspruch zwischen aktuellem Snapshot, Patch-Historie und Claim eine klare Auflösungsregel (Ground-Truth schlägt Claim; Claim nur als „Creator sagt …, unbestätigt").
**Voraussetzung:** die transcript-basierte Lernpipeline (läuft gerade) liefert die *korrekten* Claims; die L2-Verdikte werden als echte Spalten/Index nutzbar gemacht statt nur in `verifier_json` zu liegen.

### SP-C — Semantische Suche (die ausgeklammerte Vektor-Welle)
Embeddings über Claims, Mechanik-Notizen und Patch-Events, damit freie Mechanik-/„Wie funktioniert X?"-Fragen Treffer finden, die kein exakter Name-Match liefert. Füllt `mechanic_notes`, aktiviert `search_mechanic_notes`.

### SP-D — Die Lern-Pipeline als Nachschub (im Bau)
Transcript ziehen → GPT/Claude extrahiert Claims → gegen Trusted-Daten verifizieren → in die Wissens-DB. Das ist der **Nachschub-Motor**, der SP-A/B kontinuierlich mit frischem, geprüftem Creator-Wissen füttert. Videotyp-Klassifikation trennt verbale Strategie (Transcript reicht) von visueller Tech (braucht das Bild).

## Empfohlene Sequenz

1. **SP-D fertigstellen** (Datenbasis korrekt — läuft bereits): Transcripts + Klassifikation, dann GPT/Claude-Extraktion + Verifikation.
2. **SP-A bauen** (sofort sichtbarer Wert): `ask`-Befehl auf den bestehenden Assemblern — ab hier beantwortet das Brain echte Fragen.
3. **SP-B** (Qualitätssprung): Trust-Gewichtung + Claims im Kontext — Antworten werden belegt und konfliktbereinigt.
4. **SP-C** (Reichweite): semantische Suche für freie Mechanikfragen.

**Kernaussage:** Das Brain hat das schwere Stück (Daten, Normalisierung, Kontext) bereits. Der Sprung zum echten Q&A ist primär **Verdrahtung** (SP-A) plus **Trust-Disziplin** (SP-B) — kein Neubau, sondern das Zusammenstecken vorhandener Bausteine, gefüttert von der gerade entstehenden Lern-Pipeline.
