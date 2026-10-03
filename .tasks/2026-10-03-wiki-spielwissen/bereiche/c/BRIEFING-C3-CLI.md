status: aktiv
Datum: 2026-10-03

# C3: Importer fertig anbinden

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration

## Ziel und Verträge

Gewöhnlicher nativer Worker, keine Weiterdelegation. GPT 6.1 Sol high vom Elternharness erben, keine Modelle/Effort ändern. C3 ist Teil-Orchestrator und alleiniger Integrator/Deployer, Root Hauptorchestrator. Elternprozess313730, Session f83cb4c1-e8c2-4f44-8e83-ada61c2b246a, gpt-6.1-sol[1m]/high tatsächlich geprüft.

Lies zentrales BRIEFING-C3.md, CONTRACT.md und AN_BEREICHE.md Punkte14/37/41 sowie im C-Koordinationsbereich C2-BETRIEB.md, REVIEW-C2-2.md und INSTALLATIONSPLAN-C2.md. B liefert zwei belegte Git-Quellen, 660Dokumente/676363Fakten, größte JSONL-Zeile117157259Bytes ohneLF. Lies B-C-IMPORTGRENZE-B2.md und D-B-LESEVERTRAG-B2.md im B-Bereich. B allein besitzt Parser und Gesamtdatenprüfung, keinen zweiten Parser/Harness bauen. A/D haben noch keinen abschließenden Daten-/Eigencommitbeweis.

Drei zusammenhängende offene CLI-Aufgaben am vorhandenen Pfad:
1. Explizite normale Runtime-Konfiguration für dediziertes Postgresziel, Geheimnisse allein über bestehendes öffentliches deadlock-brain-core::pg::infisical_environment mit zeroisiertem Snapshot. Bisheriger pg_pool_from_config-Zentral-DSN ist nicht als dedizierter Brain-Zielnachweis geeignet. Tatsächliche Runtimeparameter frisch secretsfrei prüfen. Historischer Maintenancevertrag: Socket/run/deadlock-brain-postgresql, Port5446, DBbrain, Rollebrain_ingest, vorhandener Passwortverweis. Normale aktuelle Datei /etc/deadlock-brain/maintenance-runtime.json und vorhandene Infisical-Konfiguration strukturprüfen, keine Secretwerte lesen/ausgeben/speichern. Bestehende Zieltypen/PG-Poolmechanik und Identitätsprüfung zuerst über Graphify finden, wiederverwenden. Kein neuer Connector, keine ENV-Datei oder neuer ENV-Konfigurationsweg. Tatsächliche Identität vor Import/Publish prüfen, Fail-closed statt zufälligem Cluster.
2. Bestehenden bounded Partitionspfad verlustfrei für reale große Einzeldokumente anpassen. Keine Kürzung oder zweite Implementierung; eigener begrenzter Singletonpfad oder äquivalent an vorhandener Mechanik. Grenzen und Ressourcen anhand B-Echtdaten messen, nicht nur pauschal hochsetzen. Rohtext, Fakten, Originalhashes/Zahlenlexeme, Eingabereihenfolge und Konfliktsicherheit bewahren. Aktueller partition liest Datei komplett und validiert alles vor Ausgabe, gezielt verantwortbare Ressourcen behandeln. Releasegrenzen10000Dokumente/256MiBProjektion/500000Chunks unverändert, sie sind vom JSONL-Partitionslimit unabhängig. Messungen am echten Material nötig; keine produktiven Importe vor gemeinsamer Freigabe.
3. Publish-NIT aus REVIEW-C2-2: identische unveränderliche Release-ID bekommt beim Retry frisches created_at_epoch und scheitert. Vorhandenen Release nachlesen, Identität/Version/Patch/Pins/Rechte vollständig prüfen und gültigen bestehenden Timestamp wiederverwenden; geänderte Identität weiterhin ablehnen. Rennsicherheit im bestehenden publish_imported_heads_checked erhalten. Keine neuen Mutable-Releases, keine Bestandsänderungen von Hand.

## Eigentum

Exakt rust/crates/dbrain-sources/src/bin/brain-knowledge-import.rs samt dortigen Tests, rust/crates/brain-storage/src/pg_release.rs und rust/crates/brain-storage/tests/knowledge_release.rs. Falls minimal nötig neue eigene importerbezogene Rust-Datei unter dbrain-sources/src/bin/brain-knowledge-import/, kein zweites öffentliches CLI. Keine Cargo-/lib.rs-/Kontrakt-/knowledge_import-/source_versions-/chunk_index-/knowledge_projection-Änderung ohne Rückmeldung. Andere native Worker bearbeiten gerade Revisionen und Projektion in disjunkten Pfaden. Acht C2-Formatänderungen erhalten, drei gehören zu dir. Nicht globale Formatierung, kein fremdes Zurücksetzen.

## Arbeitsstand und Git

Worktree /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration, Branch feat/brain-wiki-spielwissen-c-integration, geerbter HEAD08a6dd78471cd6e7c43073e1c29dd0f31abb61c5, Basis511a347b653beba13c2bf130f4bead7a7196cc2a. Beide eigenen Commits erhalten,21 fremde historische Kommitte ausgeschlossen. Zunächst Quellen bauen und Schreibabschluss an C3 melden, danach einfrieren. KEIN Commit/Push/Merge/Deploy und KEIN Cargo-/Rustharness-Start bis expliziter C3-Prüffreigabe. Der Revisionsfixer besitzt zuerst die gemeinsame Prüffolge; keine Quelle während laufender Prüfungen ändern.

## Beweisziel

Quellabschluss mit nachvollziehbarem Runtime-/Identitätsvertrag, Verlustfreiheit und Retryverhalten. Bestehende passende Format-, Compiler-, Clippy-/Test- und isolierte PG-Nachweise nach C3-Freigabe. Alle Rust/Cargo-/Harness-Läufe unter beiden blockierenden Hostlocks nach HOSTPROBE.md, zuerst host-checks.lock, dann /tmp/deadlock-cargo-release.lock, frische NonZombie-Probe vor Start, maximal zweiJobs, kein Releasebau, stabile Lockwarteprozesse nicht nach20Minuten abbrechen. Eigene Ressourcenmessung am vorhandenen B-Material ist Teil der Folgeprüfung, kein zweiter B-Extraktor. Rückmeldung nötiger zusätzlicher Eigentumspfade statt improvisiertem Neubau. Kein produktiver Import, keine Konfig-/Unit-/Secretänderung im Betrieb durch dich.

## Folgefreigabe 2026-10-03T11:36:43Z

Fix2 ist regulär beendet, kein eigener Prüfer oder Gate mehr lebend. Parent hat sämtliche Exits und 267 passed/0 failed/22 initial ignored, 21 unausgeführt, echte Scratch-PG-Prüfungen und 374 unveränderte Dateien nachgemessen. Commit ad3eaca761fd9e74cc86a61307a5820a7fa860f3 hat regulären Sol-high-Selbstgate ALLOW. Dieser Teilbeweis ist keine Gesamtfreigabe. B-Eigencommit48b6ce1 wurde allein als7168574 übernommen, sieben Parserdateien unverändert, keine historische Basis.

Du erhältst jetzt zusätzlich exklusives Eigentum an:
- rust/crates/dbrain-sources/src/knowledge_contract.rs und tests/knowledge_contract.rs für denselben seekfähigen Streaming-Validator; knowledge_import.rs für die enge Angleichung des noch gemeldeten Faktenreihenfolge-Konflikts. Kein zweiter Parser oder Gleichheitshash.
- rust/crates/dbrain-retrieval/src/chunk_index.rs und src/lib.rs für einen schmalen öffentlichen Preflight-/Statistikzugriff auf vorhandenen ChunkIndex::build. Keine zweite Projektion.
- rust/crates/dbrain-sources/Cargo.toml, src/lib.rs und rust/Cargo.lock für die bereits belegte zyklusfreie Retrieval-Abhängigkeit und Registrierung des unveränderten B-Moduls. rust/Cargo.toml nur für konkret über Root bestätigte erforderliche serde_json-Features, bis dahin keine pauschale Featurewahl.
- Neue rust/crates/brain-contracts/src/postgres.rs, brain-contracts/src/lib.rs und brain-serve/src/config.rs für reine bestehende Postgres/DatabaseAuth-Typauslagerung mit kompatiblem Reexport. brain-contracts hat kein sqlx; keine neue Connector-/DB-Implementierung.

Der gespeicherte Plan gilt: private bytegenaue Eingabesicherung, begrenzter Offset-/Revisionsindex, erste Zeile bei Duplikaten über denselben Validator nachlesen; vorhandene typisierte Gleichheit ohne Clone nur für observed_at teilen. Vollständige globale Prüfung vor Partitionsveröffentlichung. Normale Partition64MiB, begrenzter Singleton128MiB, Index100000 und Spool2GiB je Eingabe als anhand endgültigen echten Materials zu bestätigende Grenzen. Release10000Pins/256MiB/500000Chunks getrennt erhalten, base.revisions statt base.heads und neue ausgewählte Köpfe verwenden. Ressourcen-/Konflikt-/Retry-/Runtimebeweise noch offen.

Punkte42 bis45 sind zentral nachgelesen. Interne Verarbeitung ist beauftragt, Veröffentlichung und Providerweitergabe bleiben gesperrt. A-Abnahme fand282 numerische Abweichungen unter1109153Fakten; Beispiele20.000010800000002 und55.555555555555564. Rawtexte/Hashes stimmen, alte A-Fakten NICHT importfreigegeben. A baut Fix6 mit getrennten Ausgaben und voller Originalzahlenprüfung; konkrete produktive Featureanforderung kommt über Root. Keine Rundung/Epsilon. In eigener späterer Prüfung Zahlenlexeme/-werte über Vertrag, Import, PG und kanonischen Leser prüfen, vorhandene Serialisierungsübergänge erhalten.

B baut separat nach Punkt44 inventargebundene Dateiobjekt-API nur in eigenen game_files.rs/anchored.rs/vpk.rs. Keine Edits dieser sieben Parserpfade durch dich. Konkrete bestätigte API steht noch aus; jetzt keinen Extraktionscaller erfinden. Andere eigener Reader-Worker erhält ausschließlich brain-maintenance/src/integration/runner.rs und runner_tests.rs. Du bearbeitest diese nicht. Keine produktiven Config-/DB-/Installänderungen.

Jetzt Quellen bis Schreibabschluss weiterbauen. Weiterhin KEIN Cargo/Rustharness, Commit/Push/Deploy vor erneuter ausdrücklicher gemeinsamer Prüffreigabe. Genaues Eigentum, nötige API-/Featureantworten und echte offene Restpunkte früh melden; keine lebende Wartetask besteht, die neu gestartet werden müsste.

## Bericht und Routing

Eigene Fachakte /home/nathanael/.worktrees/brain-wiki-spielwissen-c/.tasks/2026-10-03-wiki-spielwissen/bereiche/c/C3-CLI.md. Nur diese Fachakte schreiben; Register/TODO/Status ausschließlich C3/Root/S2. Produzent teil-c, Paket c Versuch3. Rohberichte an C3 per nativer SendMessage main, keine anderen Sessions. Melde früh Quellabschluss, nötige minimale Abhängigkeitspfade, exakte echte Datenpfade, durchgeführte/fehlende Nachweise. Stop bei echten Vertragsabweichungen oder blockierter notwendiger Runtimeidentität. Deutsche Texte, echte Umlaute, keine Gedankenstriche. Gebaut/reviewt/gemergt/live getrennt, nicht nach Dispatch finalisieren.
