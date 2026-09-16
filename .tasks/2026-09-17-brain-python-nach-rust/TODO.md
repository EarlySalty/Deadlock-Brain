# deadlock-brain: Python komplett nach Rust

Stand: 2026-09-17. Eigener Auftrag, getrennt vom RAG-Ausbau-Programm (Deadlock-Bots/.tasks/2026-09-17-rag-ausbau-visual-brain).

## Ziel

deadlock-brain wird komplett Rust. Der gesamte Python-Anteil (38 Dateien in src/deadlock_brain) wird portiert und danach entfernt. Kein Python-Rest, keine Bruecke, keine Parallelpfade. Regel: alles in Rust, Persistenz Postgres.

## Umfang

- src/deadlock_brain/*.py (38 Dateien): u. a. retrieval.py (get_embedding, build_entity_context, search_mechanic_notes), brain_pipeline.py, build_optimizer.py, entity_normalizer.py, lineage.py, legacy_entities.py, timeline.py, quality.py, sheet_*.py, storage.py, cli.py, http.py, youtube_learning.py.
- Vorhandene Rust-Seite (rust/crates/deadlock-brain, Reasoner/dbrain-population) ist die Zielbasis. Fehlende Faehigkeiten dort ergaenzen statt neu daneben.

## Beruehrungspunkt zum RAG-Programm

Die Embedding-/Retrieval-Logik aus retrieval.py wird ohnehin Rust-nativ gebraucht (Phase 2 lokales Dense-Embedding). Diese Portierung und der RAG-Ausbau teilen sich das Rust-Embedding-Fundament. Reihenfolge und Zustaendigkeit beim Start der Portierung festlegen, damit nicht zweimal gebaut wird.

## Vorgehen (offen, beim Start ausplanen)

1. Bestandssuche: welche Python-Funktionen haben schon ein Rust-Gegenstueck, welche nicht.
2. Modul fuer Modul portieren, bestehende Tests gruen halten, Rot-Gegenprobe je neuem Rust-Test.
3. Python-Aufrufer (cli.py, http.py, systemd-Units) auf die Rust-Binaries umstellen.
4. Python-Dateien und ungenutzte Abhaengigkeiten entfernen.
5. Live-Beweis, dann Merge/Deploy.
