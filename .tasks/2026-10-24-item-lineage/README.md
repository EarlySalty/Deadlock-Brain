# Item lineage 2024 to now

Brief: `.tasks/2026-10-12-brain-architektur/BRIEFING_ITEM_LINEAGE.md`.

## Delivered

- Curated lineage data `rust/crates/brain-job/data/item-lineage.json` (25 renames, 5 removals), each with a public source URL and quote.
- Extractor `item-lineage` (brain-job) writes aliases and lineage facts; patch extractor v5 detects "X: Renamed to Y" lines and subject-line history.
- Migration v45: `serve.item_lineage`, `serve.item_rename_target`, `serve.site_item_lineage`.
- `find_entity` resolves former names and attaches lineage; `get_item` card carries lineage.
- Steckbrief sections "Frühere Namen", "Ersetzte Items", "Änderungsverlauf", "Nicht mehr im Shop".
- German report: `LINEAGE.html`.

## Open

- Unresolved list in `LINEAGE.html` (Extra Stamina, German former names, Health Nova patch, 77 disabled upgrades, ability renames).
- No item doc corpus in `serve.site_doc_section`; lineage reaches the Brain via tool payloads.
