# ADR Register für Migration Plan 1.0

## ADR S000 001: Bestehende Rust Crates erhalten

Status: angenommen für S000  
Basis Commit: `55776c7532bb461fab93bbf7c582e2a40a81d2c9`

Entscheidung: Die vorhandenen `dbrain` Crates und `deadlock-brain-core` bilden die Ausgangsbasis. Der Zielplan beschreibt Verantwortungen, aber erzwingt in S000 keine Umbenennung und keine parallelen Ersatzmodule.

Beleg: `rust/Cargo.toml` im Basis Commit enthält elf Workspace Mitglieder. `deadlock-brain-core/src/lib.rs` beschreibt gemeinsame Verträge, Config, HTTP, Fireworks, Postgres und Datenmodelle. Die Fachcrates für Quellen, Normalisierung, Enrichment, Retrieval, Reasoning, Builds, Learning und Population sind im Workspace vorhanden.

Folge: Chat 01 prüft Funktionsstand und aktive Nutzung. Chat 02 ordnet spätere Contract Arbeit in diese reale Struktur ein.

## ADR S000 002: S000 verändert keine Produktion

Status: angenommen für S000

Entscheidung: S000 enthält Koordinationsartefakte. Rust Quellcode, Datenbank, Datenbestände, Timer, Services und Deployment werden in diesem Arbeitspaket nicht verändert.

Folge: Für S000 ist kein Runtime Restart erforderlich. Live Zustand und aktive Services bleiben Bestandteil von G0.

## ADR S000 003: Erste Folgewelle bleibt Vorbereitung

Status: angenommen für S000

Entscheidung: Chat 01 und Chat 10 erhalten nach Integration von S000 den Modus `prepare_only`. G0 bleibt offen. Chat 02 und Chat 03 erhalten in S000 keine Implementierungsfreigabe.

Folge: Die nächste belastbare Entscheidung entsteht aus Inventar, Baseline und Testdesign.
