# docs-search-fix

Branch `fix/brain-docs-search-20261023`, Basis `179c41d4`.

## Ursache

1. `search.sql` nutzte nur `websearch_to_tsquery` mit UND-Semantik. Die Modellanfragen enthalten
   Synonyme und Füllwörter (z. B. `Steam account mit Discord Server verknüpfen verbinden`), daher
   passte fast nie ein Abschnitt auf alle Begriffe.
2. Breitere Treffer liefen in den 500-ms-Guard: `serve.doc_section` prüft pro Kandidatenzeile die
   Veröffentlichung (~7,6 ms je Zeile). Schon `Steam` (131 Kandidaten) lief in den Timeout.
3. Befehle stehen als Pfad-Token (`/checkrank`) im Index; ohne Schrägstrich gesucht gab es keinen Treffer.
4. Die Werkzeugbeschreibung sagte nicht, dass die Doku deutsch ist und was sie abdeckt; das Modell
   suchte englisch oder gar nicht.

## Änderung

- Migration 33 `scripts/migrations/2026-10-23-brain-gamefacts-docs-search-v33.sql`:
  `serve.doc_section` gibt die Schlüssel aus `canon.doc_search` aus (gleiche Spalten, gleiche Prüfung),
  damit Schlüssel-Joins über den Primärschlüssel laufen; neue View `serve.doc_search_hit`
  (nur veröffentlichte Quittungen, jüngster Lauf je Seite, keine Texte) für `brain_tool`/`brain_service`.
- `search.sql`: strikte Stufe unverändert vorn; nur wenn sie leer ist und die Anfrage keine
  Operatoren (`-`, `OR`) hat, Rückfall auf Abschnitte mit mindestens zwei Suchbegriffen, inklusive
  `/befehl`-Variante. Höchstens 12 Kandidaten laufen durch die volle Prüfung von `serve.doc_section`.
- Werkzeugbeschreibung `search_docs` in `service.rs`.

## Nachweis

`/home/nathanael/.local/state/brain-wave2-proof-20261014/docs-search-fix/`
- Basis (gleicher Build ohne Fix): docs 0/10 (`e1-base-docs.txt`), integriert 1/10.
- Fix: docs beantwortet 8/10, korrekt 7/10, Zitate 8/8; unanswerable 20/22 wie Basis
  (`e1-fix-docs-refusal.txt`). Rest: docs-003 Zitat vom Kernel abgelehnt, docs-006 Modell lehnt trotz
  Treffern ab, docs-010 Antwort ohne erwarteten Begriff. Der Timeout bei unanswerable-006 kommt aus
  dem Spielwerte-Port, nicht aus der Doku-Suche.
