status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-r

# Worker: Replay-Report in den normalen Store

## 1. Ziel und Vertrag

Paket R aus AUFTRAG.md und BRIEFING-R.md im übergeordneten Ordner fertigstellen: echte, berechtigt öffentlich abrufbare Demos lokal dekodieren; Ergebnisse über SourceRecordV2 im vorhandenen Store speichern; interne Beispielabfrage mit Quelle und Version; wiederholter Import idempotent. Kein öffentlicher Zugriff auf private Replaydaten und keine LLM-Aufrufe.

Die native Recherche belegt den lokalen Worker und gemeinsame Replayverträge, aber keine produktive ObservationStore-Verbindung. Vorhandene Bausteine verwenden: brain-contracts/src/replay.rs mit Provenance und Dedup, brain-ingestion/src/document_set.rs mit CoreDocument zu SourceRecordV2, brain-storage/src/pg_release.rs mit atomarer Freigabe. Gemeinsame Dateien nicht ändern. document_set ersetzt origin.raw_sha256 durch den Inhaltshash: Original-Demo-SHA separat in geeigneten vorhandenen Provenance-Metadaten erhalten. Quelle, Match, Selektion, Parser-/Schema-/Extraktionsversion und RawLocator dürfen nicht verloren gehen. Unknown bleibt Unknown; aus Headern keine Gameplay-/Coachingfakten erfinden.

Öffentlicher Beschaffungsweg ist belegt: API-Salts ausschließlich mit disable_steam=true, Valve-CDN. Beispiel Match 110001910, URL http://replay271.valve.net/1422450/110001910_1897304728.dem.bz2, HEAD 200, komprimiert 45.014.647 Bytes. Noch kein Decode-Erfolgsnachweis. Rawdateien bleiben außerhalb Git. Eine separate lesende Beschaffungsarbeit läuft ohne Änderungen an deinem Code.

## 2. Eigentum

Ausschließlich rust/crates/dbrain-replay/ einschließlich eigenem Cargo.toml/Cargo.lock und architecture/migration/replays/ (neue knappe Bediennotiz). Keine neuen Kommentare. Keine Änderungen an Workspace-Cargo.toml/Cargo.lock, brain-feeds, brain-contracts, brain-ingestion, brain-storage, API oder Migrationen. Wenn ein gemeinsamer Pfad zwingend nötig ist: präzisen Vertragsbedarf melden, nicht schreiben.

Minimaler Weg bevorzugt: vorhandenen Worker beibehalten; separaten Import-CLI-Einstieg innerhalb des vorhandenen Replay-Crates ergänzen, der bestehenden Dokument-/Lease-/Batch-/Release-Mechanismus nutzt. Keine zweite DB, kein neuer Scheduler, keine Python-Implementierung. Dauerhafte Idempotenz aus normalem Store beweisen, keine ausschließlich speicherinterne Dedup-Behauptung. Reparse muss komplette Generationen konsistent ersetzen, ohne andere Matches zu verdrängen. Unterschiedliche Rohfassungen desselben Matches müssen ihre Belege behalten.

## 3. Arbeitsstand

Branch feat/brain-fertig-r-20261003, HEAD 511a347b653beba13c2bf130f4bead7a7196cc2a, bisher sauber. PR #46 (919c790f39fcb6e5265f789086aa100e6e11da5b) enthält wiederverwendbare Validierung, noch nicht auf main. Übernimm passende Code-Hunks und Tests, keine alten Manifeste oder falschen Abnahmeberichte. Keine Commits/Merges/Pushes durch diesen Worker; Abschluss macht teil-r. Keine neuen T3-Threads, keine Delegation.

## 4. Beweisziel

CLI arbeitet über den vorhandenen Postgres-Store und verweigert unbelegte Rechte oder inkonsistente Reports. Bestehende Decoder-/Porttests bleiben erhalten. cargo fmt, clippy -D warnings und betroffene Tests auf deinem aktuellen Stand ausführen und Counts nennen. Vor jedem Compilerstart die beiden Host-Locks aus /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md erwerben und bis Ende halten, NonZombie-Probe frisch unmittelbar davor, maximal -j 2. Keine globalen Target-Änderungen oder Löschungen. Kein fremder Compiler wird beendet. Bestehende Infisical-Wege verwenden; keine ENV-Dateien, keine Secrets ausgeben. Keine Prod-DB-Schreibzugriffe oder Dienste durch diesen Worker. Der echte Import-/Abfragebeweis kommt anschließend durch teil-r.

## 5. Routing

Auftraggeber teil-r, Paket r, Versuch 1. Hauptorchestrator 43a4886c-e135-484b-838a-0512d224a634, Integration Paket Z. Status allein teil-r in status/r/1; keine TODO.md oder REGISTER.md schreiben. Fachbericht hier mit geänderten Pfaden, Testbefehl/Counts, offenen echten Grenzen und sofort ausführbarem Import-/Queryweg. Nach spätestens 20 Minuten Stand melden. Bei Scope-/Vertragsblocker mit vorhandener Arbeit und konkreter Lösungsrichtung melden, kein eigenmächtiger Scope-Zuwachs.

Vor jeder Codefrage Graphify zuerst, Brain-Graph /home/nathanael/.graphify/projects/deadlock-brain/graphify-out/graph.json oder global /home/nathanael/.graphify/global-graph.json. Normale Nutzertexte auf Deutsch, echte Umlaute, keine Gedankenstriche. Rolle: nativer Implementierungsworker mit dem geerbten freigegebenen Modell.

## Wiederaufnahme um 15:13 UTC

Die beiden letzten Worker sind durch Sessionende gestoppt, kein laufender Compiler. Erhaltene eigene Änderungen: Cargo.toml mit brain-ingestion, brain-storage, sqlx, tokio und Import-Binary-Eintrag; main.rs mit validate; src/validation.rs und tests/validation.rs aus PR 46. import_main.rs fehlt. Ungeprüften Stand übernehmen, nichts erneut von Null bauen. Ein vorher gemeldeter Schreibkonflikt stammte aus zwei eigenen parallelen Läufen nach einer Live-Nachricht, keine fremde Sitzung. Jetzt exklusives Schreibrecht für genau diesen Worker; der Hauptagent schickt keine Nachrichten während der Laufzeit.

Der Wegwerf-Testcluster steht schon bereit: postgresql:///brain_replay_test?host=/tmp/brain-replay-r-core-20261003/pg&port=55439&user=nathanael. Ausschließlich Peer-Auth und Unix-Socket, listen_addresses leer. Auf genau diesem Cluster vorhandene Rust-Migrationen und Store-Proben erlaubt; keine Prod-DB. Bei gestopptem Cluster melden, nicht einen fremden starten.

Quelle aus PR-46 darf nicht unbesehen alte Verträge übernehmen. Der Original-Kommentar zur exakten Schema-Pin im Cargo.toml ist beim Kopieren verschwunden: unverändert wiederherstellen, keine neuen Kommentare schreiben. Obergrenzen bleiben aus DecodeBudget/Store-Konfiguration, keine neuen festen LLM-Zeitlimits. Nächste Schritte: echten Adapter und Import-/Query-CLI fertigstellen, formatieren und mit Host-Locks prüfen. Bei Test-Counts zwischen aktiven Suites und Helper-Ignore unterscheiden. Bericht erst bei geprüfter Abgabe oder echtem Vertragsblocker.
