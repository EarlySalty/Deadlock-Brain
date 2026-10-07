status: beauftragt
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/Deadlock-Docs-brain-consumer-fertig

# Frische Intent-Abnahme Docs

## Ziel

Unabhängige fachliche Abnahme des erhaltenen Docs-Adapters auf SHA 3e570a8aa0bf867bf1baf35b064165804b77fcb4. Prüfe den Auftrag und seine Grenzen, nicht eine eigene Bug-/Security-Review. Einziger Reviewer für solche Funde bleibt das lokale Merge-Gate. Diese Abnahme darf weder fehlenden Betrieb erfinden noch einen sicheren Adapter wegen ausstehender nachgelagerter Integration als komplett produktiv beschreiben.

## Vertrag und Quellen

AUFTRAG.md, GEMEINSAM.md, BRIEFING-K.md, UEBERNAHME-CODEX.md zwei Ordner höher lesen. Hier SCOUT-docs.md und BAU-DOCS.md lesen. Der neue Hauptorchestrator ist e6c19079-657e-4db9-80bd-8e1313e7f785, gekoppelte Integration liegt bei Z. docs.public bleibt fest; keine private Operatornutzung, kein eigener Antwortdaemon, keine neuen Provider. Bestehende Sol-Arbeit wiederverwendet. Vor Codefragen Graphify, danach tatsächliche Quelle verifizieren.

## Eigentum

Nur lesende Abnahme im genannten eigenen Worktree. Keine Dateiänderung, kein Test- oder Compilerlauf, kein Gate, Git-Mutation, Deploy, Secretzugriff, Netzwerk-Live-Aufruf oder weiterer Agent. Keine fremden Worktrees ändern. Ergebnis als Rückgabe, teil-k schreibt das Artefakt.

## Beweis und Ausgabe

HEAD und sauberen Stand prüfen. Adaptervertrag mit Auftrag vergleichen: typed BrainClient, öffentlicher Scope, sichere bestehende TOML-/FD-/Infisical-Anbindung, auflösbare Docs-Frage. Vorhandene Prüfnachweise kompakt verifizieren, keine Suite neu starten. Ergebnis getrennt: Bau fachlich fertig J/N; für gemeinsame Kernintegration bereit J/N; gesamter Consumer produktiv fertig J/N; konkrete Abweichungen; Codefix nötig J/N; nachgelagerte Konfigurations-/Betriebsschritte. Noch keine normale TOML, kein ausgerollter CLI-Release, kein freigeschalteter Docs-Grant oder korrelierter Live-Beweis. SHA-Drift benennen und nicht alten Stand akzeptieren.

## Routing

teil-k, Paket K, Versuch 1. Frischer nativer Kontext, Sitzungsmodell Sol, keine Sonnet-/Fable-Subagenten. Nicht in TODO.md oder REGISTER.md schreiben. Deutsche Rückgabe mit echten Umlauten und no-em-dashes.
