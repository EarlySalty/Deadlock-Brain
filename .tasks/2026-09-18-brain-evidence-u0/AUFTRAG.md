# Deadlock Brain: U0-Evidenzkorrekturen

Stand 18. September 2026. Grundlage: Nutzerübergabe `Uebergabeprotokoll-Deadlock-Brain-2026-09-18(1).md`, Abschnitt 5 und Startauftrag. Bestehender PR #4, Branch `codex/patch-understanding-evidence-20260918`, Basis `da3b55ea6619cb8007494adc40150d82375b96d7`.

## Auftrag und Besitz

Du bist der einzige Worker-Thread für dieses Paket. Keine Unter-Threads oder Unter-Agenten starten. Keine Modellwechsel. Intent ist diese externe ChatGPT-Orchestrierung, es existiert keine T3-Intent-Thread-ID; nicht erfinden. Der Orchestrator prüft Betrieb und U1 unabhängig. Arbeite ausschließlich im bereits angelegten eigenen Worktree `/home/nathanael/.worktrees/brain-evidence-u0-20260918`. Fremde Worktrees und den Kanon nicht verändern.

Die ursprüngliche Datei `follow-up-NOT-PUSHED-NOT-CI-TESTED.patch` ist nicht verfügbar. Die zwei beschriebenen Korrekturen müssen am vorhandenen Code umgesetzt werden. Nicht behaupten, den Originalpatch angewandt zu haben.

## Enger Umfang

1. Früheste belegte Beobachtung desselben Ereignisinhalts muss bei unverändertem Reimport, Löschen/Reimport und Rückkehr zu einem früheren Inhalt erhalten bleiben. Inhaltsänderungen haben eine eigene Erstbeobachtung. `observation_kind=baseline` und `earlier_observation_unknown` bleiben auch nach Reimport erhalten. Die Revisionszeit bleibt separat. View `brain.patch_history_v1` und Materialisierung `brain.knowledge_events` müssen eine konsistente `first_observed_at`-Semantik liefern. Keine erfundenen historischen Zeiten, Veröffentlichung ist kein bewiesener Spiel-Rollout. Prüfe auch `known-at`: ein historischer Treffer darf keine Information aus einer späteren Revision benutzen.
2. `brain.youtube_caption_segments_v1` muss zusätzlich den SHA-256 des aktuellen `brain.youtube_transcripts.transcript_text` gegen `e.text_sha256` prüfen (tatsächlichen Spaltennamen am Schema verifizieren). Gleicher Text mit neuer Zeitzuordnung bleibt eine neue Evidenzfassung; anderer Text darf keine alten Zeitsegmente liefern.
3. Nutze eine neue nachvollziehbare Folgemigration; ändere die bestehende Migration nicht. So bleibt ein möglicherweise bereits ausgerollter Stand kompatibel. Neue Migration muss wiederholbar sein, begrenzte Locks/Timeouts nutzen und keine pauschalen PUBLIC-Rechte vergeben.
4. SQL-Fixture/Assertions und CI auf beide Migrationen erweitern. Teste unverändertes Update, echte Inhaltsänderung, Löschen/Reimport, Baseline-Unsicherheit und Caption-Textänderung sowie gleiche Texte mit neuer Zeitzuordnung. Alte Migration vor neuen Assertions muss die relevanten Fehler nachweislich zeigen. Caption-Integrationstests auf vollständigem separatem Scratch-Schema nach Möglichkeit wirklich ausführen, ignoriert nicht als bestanden zählen.
5. Aktualisiere `docs/AUTONOMOUS_PATCH_REVIEW.md` mit tatsächlichem Stand und Grenzen. Keine zweite Build-/KI-/RAG-Plattform, keine Creator-Urteile und kein Videoimport. Keine Erweiterung um andere Arbeitspakete.

## Fundstellen

- `scripts/migrations/2026-09-18-patch-evidence.sql`
- `tests/patch-understanding/schema-fixture.sql` und `schema-assertions.sql`
- `.github/workflows/patch-understanding.yml`
- `rust/crates/deadlock-brain/src/bin/deadlock-brain-patch-review.rs`
- `rust/crates/deadlock-brain-yt/src/transcripts.rs`
- `docs/AUTONOMOUS_PATCH_REVIEW.md`

Graphify wurde aus dem Kanon angefragt: `graphify` nicht im Tool-PATH; ein Aufruf des vermuteten absoluten Pfades wurde vom Sicherheitsgate blockiert und nicht umgangen. Nutze den vorgesehenen erlaubten Zugang, sofern vorhanden, oder die exakten bekannten Fundstellen. Keine kompletten Reindexierungen.

## Sicherheit und Abschluss

Secrets und ENV-Dateien weder öffnen noch ausgeben. Bestehende Credential-Loader nur zum Ausführen verwenden, niemals Environment drucken. Keine Produktionsmigration und kein Deploy durch dich; das übernimmt der Orchestrator nach Prüfung. SQL-Fixture niemals in Produktion. Kein Merge und kein Push nach main. Keine Änderung anderer Dienstkonfiguration oder Modelle. Keine Veröffentlichung von Analyseposts.

Prüfungen nach Nutzerauftrag wirklich ausführen: Rust-Tests für `deadlock-brain-yt`, `deadlock-brain-patch-review`, `pg_insights::tests`, Clippy; Formatter nur für eigene Rust-Dateien. Build mit höchstens zwei Jobs und eigenem Target-Verzeichnis, keine parallelen schweren Builds. Tests in separater Datenbank/Cluster. Wenn ein Weg gesperrt ist, genaue Fehlermeldung notieren, Sicherheitsgate nicht umgehen.

Eigene geprüfte Änderungen schrittweise committen und auf den Feature-Branch pushen. Git add nur eigene Dateien; diese Auftrags-/Registerdateien dürfen dazugehören. Ergebnis als `REPORT.md`: Commit/Push, Dateien, genaue Tests mit Zahlen, ignorierte/ausgefilterte Tests, Fehler und offene Blocker. Kein Selbst-Review; Orchestrator prüft den Diff. Fertigmeldung im selben Thread, keine neuen Threads.