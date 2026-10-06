status: aktiv
Datum: 2026-10-03
Stand: 2026-10-03T06:47:09Z

# B-Daten passen nicht in das gelesene C2-Partitionslimit

Urteil: konkrete Integrationsgrenze, keine Parserkorrektur bei B. Der vollständige grüne GameTracking-Lauf liefert eine JSONL-Zeile mit 117.157.259 Bytes ohne LF, rund 111,73 MiB. C2s aktueller Partitionspfad lehnt jede Zeile über 64 MiB ab. Die vollständige GameTracking-Ausgabe ist mit diesem Pfad nicht partitionierbar.

Gemessen durch B-Prüfworker: PRUEFLAUF-B2.md und round2-proof/GameTracking-Validatorlog. Beide vollständigen Quellenläufe und Rohtext-/Hash-/Zahlenlexemprüfungen grün. Größte deadlock-data-Zeile 11.107.243 Bytes.

Lesende Gegenprüfung am 03.10.2026 um 06:47:09 UTC:
`/home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration/rust/crates/dbrain-sources/src/bin/brain-knowledge-import.rs:219,293-297`.
`PARTITION_MAX_BYTES = 64 * 1024 * 1024`; `line.len() > max_bytes` gibt einen Fehler zurück. Dateihash SHA-256 `f68b0fb1bc609106a57d2ff3373d7dd3cb4071f062202b4ffa025086b5d8f428`. Kein C-Compiler-/CLI-/Datenbanklauf durch B2, keine C-Datei verändert.

Vorschlag an C2: vorhandenen begrenzten Partitionspfad so anpassen, dass ein tatsächlich gemessenes großes Dokument verlustfrei als eigene Partition verarbeitet werden kann. Größe und Ressourcen anhand der echten B-Ausgabe prüfen. Rohtext, Fakten und exakte Zahlenlexeme erhalten; kein Abschneiden, kein zweiter Parser und keine pauschale unbeschränkte Verarbeitung. C2 besitzt diesen Pfad.

B2 ergänzt gerade Ressourcenmessung und fmt/Clippy im bestehenden eigenen Harness. Neue echte D-Rohdaten fehlen weiterhin; ihre Depotabnahme ist getrennt offen. Dieser Befund sperrt den genannten C-Partitionsweg, nicht die bereits nachgewiesene B-Extraktion.
