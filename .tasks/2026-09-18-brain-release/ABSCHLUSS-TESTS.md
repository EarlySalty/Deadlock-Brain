# Integrationsabschluss vom 18.09.2026

Basis: 9ead46171f3d0f3c5d0e2fe739f1a9e693e37013. Branch: codex/brain-deploy-completion-20260918. PR #6 enthält den Release-Stand aus #5 und #4. Der bisherige Implementierer 57ed8534 hat die Korrekturen geschrieben; nach seinem Kontingentabbruch hat der Orchestrator die vorhandenen Änderungen geprüft, nur die beiden geänderten YT-Rust-Dateien formatiert und die Tests selbst zu Ende ausgeführt. Es wurde kein weiterer Worker gestartet.

## Änderungen

- Aktuelle Claim-Abfragen schließen Videos mit metadata.needs_claim_revalidation=true aus. Derselbe Schutz liegt in den drei Retrieval-Eingängen für exakte Entity, Entity-Text und Schlüsselwörter. Historie und gespeicherte Claims werden nicht gelöscht.
- Zwei deterministische DB-Verträge prüfen Entity/Promptversion/Modell-Filter und die Sperre revalidierungsbedürftiger Videos.
- Die neue isolierte YT-Fixture und der CI-Job führen die bisher offenen Queue- und Transcript-Claim-Verträge sowie die beiden neuen Filtertests tatsächlich aus. Der bereits bestehende Caption-Integrationstest bleibt zusätzlich aktiv.
- Der versionierte Sync bricht bei fehlgeschlagener oder formwidriger Drift-Abfrage ab. Ein Fehler wird nicht als erfolgreicher unveränderter Quellenstand behandelt. Kein neuer Modellaufruf, Provider oder Sammler.

## Wirklich ausgeführte Prüfungen

Cargo stable 1.97.1, eigenes target-rel, maximal zwei Build-Jobs, SQLX_OFFLINE=true. Keine Produktions-Fixture und keine produktiven Testzeilen.

| Prüfung | Ergebnis |
|---|---|
| deadlock-brain, Binary-Tests | 51 bestanden |
| deadlock-brain-patch-review | 11 bestanden |
| deadlock-brain-yt, normale Suite | 17 bestanden; sieben DB-Tests im normalen Aufruf ignoriert |
| Sechs YT-DB-Tests auf isoliertem aktuellem Produktionsschema mit allen drei Migrationen | 6 bestanden; umfasst Caption-Speicherung, Queue, Transcript-Claim-Prepare/Import, Prompt-/Modellfilter und Revalidierung |
| Fünf passende YT-DB-Tests zusätzlich auf exakt der neuen CI-Fixture | 5 bestanden; Wiederholung, nicht nochmals zur Gesamtzahl addiert |
| dbrain-retrieval, normale Library-Suite | 22 bestanden, 12 bestehende datenabhängige Integrationstests ignoriert |
| deadlock-brain-core, Konfiguration/Relokation | 2 bestanden; nur dieser Filter ausgeführt |
| MCP-Pytest über den tatsächlich registrierten Interpreter .venvs/gpt-workers/bin/python3 | 6 bestanden; eine Pydantic-Forward-Reference-Warnung |
| Clippy für deadlock-brain, deadlock-brain-yt und dbrain-retrieval | Exit 0; eine bereits im übernommenen Release-Code liegende dead_code-Warnung für Meta.hero_ref im Patch-Review-Binary |
| rustfmt --check für beide geänderten YT-Dateien | bestanden |
| bash -n scripts/ops/patchnotes-sync-v2.sh | bestanden |
| Drei Evidenzmigrationen auf vollständigem aktuellem Brain-/Patchnotes-Schema-Klon | bestanden |

Damit 115 unterschiedliche erfolgreiche Tests in den ausdrücklich genannten Aufrufen. Nicht als bestanden zählen: der historische 11941-Claims-Paritätstest ohne seinen ursprünglichen Datensnapshot sowie die zwölf bestehenden datenabhängigen Retrieval-Integrationstests. Der erste unittest-Discovery-Versuch fand keine Tests; erst der anschließende Pytest-Aufruf führte die sechs MCP-Tests tatsächlich aus.

Rohlogs auf dem Host: /home/nathanael/.local/share/deadlock-brain/releases/20260918-evidence/{yt-unit,yt-integration,yt-ci-fixture,brain-unit,retrieval-unit,core-config,clippy}.log. Diese Logs sind keine Grundlage für eine Behauptung fachlich geprüften autonomen Spielverständnisses.

## Unabhängiger Codecheck des Orchestrators

Die neuen SQL-Bedingungen liegen in allen drei Claim-Retrieval-Pfaden sowie der YT-Claim-CLI; Parameterbindung bleibt erhalten. Die Datensätze bleiben gespeichert. Die Drift-Validierung akzeptiert nur mode=check mit einem booleschen drift und propagiert Importfehler. In den geänderten Pfaden wurde kein neuer blockierender Fehler gefunden. Das reguläre Merge-Gate bleibt zusätzlich erforderlich.

## Stand außerhalb dieser Tests

Produktive Evidenzmigrationen, Release-Installation, erster echter Modelllauf und Live-Historienabfrage werden separat im Deploybericht dokumentiert. Der Sync verarbeitet Änderungen der vorhandenen Quelltabelle; ein automatisch fachlich geprüftes Lernsystem U2 bis U5, Gameplay-Bildauswertung U7 und eine fertige Videoausgabe U8 sind dadurch nicht geliefert. Veröffentlichung von Analyseentwürfen bleibt gesperrt.
