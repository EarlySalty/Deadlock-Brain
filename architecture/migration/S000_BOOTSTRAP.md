# S000: Koordinationsbasis

Planversion: 1.0  
Basis Commit: `55776c7532bb461fab93bbf7c582e2a40a81d2c9`  
Planpaket SHA256: `945be6983ff0235eb0ec36b451f9613ea567e0962ca5658364e8665980177716`  
Quelle im Basis Commit: `Deadlock-Brain_Rust-Daten_Planpaket_v1.0.zip`

## Umfang

S000 richtet die Koordinationsbasis für den Umbau ein. Es verändert keinen produktiven Rust Pfad, keine Datenbank und keinen laufenden Dienst.

Der vorhandene Rust Workspace bleibt die Ausgangsbasis. `rust/Cargo.toml` enthält elf Workspace Mitglieder, darunter `deadlock-brain-core`, `dbrain-sources`, `dbrain-normalize`, `dbrain-enrich`, `dbrain-retrieval`, `dbrain-reasoner`, `dbrain-builds`, `dbrain-learn` und `dbrain-population`. Diese vorhandenen Fachmodule werden für die Folgearbeit reserviert, statt parallel neue Ersatzmodule anzulegen.

## Verifizierter Startstand

* Das Planpaket ist im Basis Commit vorhanden und laut Paketmanifest Planversion 1.0.
* Das Paket beschreibt 15 Chat Aufträge und 60 Anforderungen.
* `deadlock-brain-core` enthält gemeinsame Config, HTTP, Fireworks, Postgres und Datenmodelle.
* `dbrain-sources` enthält Quellenadapter für Assets, Deadlock API, Deadlock Data, Forum, Google Sheet, Patchnotes, Reddit, Statlocker und Wiki.
* `dbrain-retrieval`, `dbrain-reasoner`, `dbrain-builds`, `dbrain-learn` und `dbrain-population` sind bereits als eigene Workspace Crates vorhanden.

Diese Aussagen beziehen sich auf den Code im Basis Commit. Live Runtime, aktive Timer, reale Datenmengen, Rechte, Last und Produktionszustand sind damit nicht geprüft.

## Freigabe nach S000

Chat 01 darf das Inventar und die Baseline im Modus `prepare_only` aufnehmen. Chat 10 darf parallel das Testdesign im Modus `prepare_only` erstellen. G0 bleibt offen, bis Code, Daten, Runtime, Hardware, Last und Rechte mit Belegen erfasst sind.

Contract Version und DB Schema Version bleiben in S000 ungesetzt. Produktive Migration, Cutover, Vollimport und Legacy Abbau gehören nicht zu S000.
