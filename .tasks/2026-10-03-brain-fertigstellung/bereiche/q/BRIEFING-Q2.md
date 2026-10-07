status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Worker Q2: Sheet und YouTube an den normalen Kern anbinden

## Wiederaufnahme und bestätigte Entscheidung

Der alte native Lauf wurde durch das Sitzungsende gestoppt. Übernimm die vorhandenen neuen Dateien `sheet_core.rs`, `youtube_core.rs`, `source_sync.rs`, `brain-source-sync.rs`, `tests/source_sync.rs` und die Parserextraktion, statt neu anzufangen. Q1 ist geliefert. Zwei autorisierte C9-Commits werden beziehungsweise wurden inzwischen durch die Hauptsession cherry-gepickt; vorhandenen HEAD frisch prüfen.

Die Betreiberentscheidung in `VON_HAUPT.md:32` ist verbindlich: enger Bestandserhalt im normalen Kern, Lizenz unbekannt, ausschließlich intern, Veröffentlichung und externe Modelle gesperrt, keine kanonischen Spielfakten. Aktuelle Fassung plus Historie 12 Monate; wenn Sheet-Zeile oder Video verschwindet, Inhalt aus dem aktiven Bestand entfernen. Gemini bleibt aus, YouTube-Lernen pausiert. Frische Metadaten ohne Transkript-/Modelllauf sind erlaubt. Die Konfiguration darf diese Grenzen nicht still abschwächen. Eine 12-Monats-Historie muss tatsächlich über vorhandene Store-/Retentionmechanik durchsetzbar sein; finde und verwende den Bestand, statt bloß ein wirkungsloses Metadatenfeld zu schreiben. Falls eine gemeinsame Storage-Ergänzung erforderlich ist, konkret melden, ohne die fremde Crate selbst zu ändern.

## Ziel und Vertrag

Das bestehende Rust-Lernen schreibt noch Legacy. Baue die schmale Kernanbindung aus den vorhandenen Bausteinen. `brain-ingestion/src/document_set.rs` stellt `CoreDocument`, `DocumentSetSource`, `prepare_document_batch` und `current_pins`; `brain-storage/src/pg_release.rs` stellt atomaren Batch-Commit mit Release. Vorlagen: `brain-feeds/src/patchnotes.rs`, `deadlock_assets.rs`, `src/bin/brain-match-ingest.rs`, `brain-legacy-import/src/import_firstparty.rs`. Diese Vorlagen nicht verändern. Bestand und Referenzen stehen in `BRIEFING-Q.md`, `AUFTRAG.md` und `GEMEINSAM.md`. Weitere Codesuche zuerst Graphify mit `/home/nathanael/.graphify/projects/deadlock-brain/graphify-out/graph.json`.

Sheet: vorhandenen CSV-Abruf und Parser aus `dbrain-sources/src/google_sheet.rs` lesen und wiederverwenden oder für einen reinen Abruf eng extrahieren. Kein Legacy-Schreibzugriff im Kernmodus. Stabile Dokumentidentität, Quellhash und Revision, kein künstlicher Patchbeleg und keine kanonische Hochstufung untypisierter Tabellen.

YouTube: vorhandene berechtigte Transkripte oder Metadaten unabhängig vom pausierten Gemini-Lernen übernehmen. Die Betreiberentscheidung über Rechte steht noch aus. Deshalb explizite Quelle-/Aufbewahrungskonfiguration mit fail-closed Voreinstellung; keine erfundene Lizenz, keine automatisch eingeschaltete Veröffentlichung oder Anbieterweitergabe. Keine Gemini-Reaktivierung, kein neues STT oder LLM. Für historischen Bestand darf eine separate, ausdrücklich lesende Verbindung als `brain_readonly` verwendet werden; Ziel ist der normale `brain_ingest`-Store auf Port 5446. Keine Verbindung zum zentralen Consumer-Postgres.

Vollständige Quellenmengen sind Voraussetzung für `prepare_document_batch`. Limit oder unvollständiger Abruf darf keine vorhandenen Datensätze tombstonen. Alle fremden Corpus-Pins erhalten. Veröffentlichung des Kern-Releases und Umschalten des laufenden brain-serve sind getrennt: Q darf Quellenrevisionen und ein Kandidatenrelease erzeugen, Z besitzt die Aktivierung. Konfigurierter Release-Bezug muss daher eindeutig sein und darf sich nicht selbst auf einen fremden Runtime-Release setzen.

## Eigentum

Neue Dateien `rust/crates/brain-feeds/src/sheet_core.rs`, `youtube_core.rs`, `source_sync.rs`, `src/bin/brain-source-sync.rs`, passende neue Tests. `brain-feeds/src/lib.rs` darf nur um die eigenen Module ergänzt werden. Falls nötig eng begrenzte lesende Parserextraktion in `dbrain-sources/src/google_sheet.rs`. Keine Patchnotes-/Build-Publish-/Match-/Assets-Dateien verändern, keine Workspace-Manifeste, Lockdateien oder Migrationen. Eigene `brain-feeds/Cargo.toml` ebenfalls nicht selbst verändern: Abhängigkeiten oder Binärdeklaration im Bericht nennen, Hauptsession ist dafür einziger Schreiber. Keine neuen Code-Kommentare, keine globale Formatierung.

## Arbeitsstand

Branch `feat/brain-fertig-q-20261003`, HEAD `511a347b653beba13c2bf130f4bead7a7196cc2a`, Baseline-Prüfung abgeschlossen. WIP der Hauptsession `architecture/migration/evals/q-core-baseline-20261003.json` nicht anfassen. Keine Git-Mutationen, keine Unit-/Konfigänderungen live und keine DB-Schreibzugriffe oder externen Modellaufrufe. Du bist ein nativer Worker und delegierst nicht weiter.

## Beweisziel

Belege Codepfad bis zu `SourceRecordV2` und normalem PgStore. Idempotenz, unvollständige Menge ohne Löschung, semantische Rechteänderung, erhaltene fremde Release-Pins und fehlende Rechte fail-closed gezielt prüfen. Konfiguration benötigt keine Token-Datei und lädt benannte Infrastruktur-Secrets über vorhandenen Infisical-Weg und Credential-FD 5. Secrets nie ausgeben. Eigene Tests nach `rolle-test-waechter`, Compiler mit Rust 1.97.1 und höchstens zwei Jobs. Vor jedem Compiler/Test blockierend beide Locks halten: `host-checks.lock` unter `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/`, danach `/tmp/deadlock-cargo-release.lock`. NonZombie-Probe gemäß `HOSTPROBE.md`, bei Fremdcompilern unter beiden Locks nach 30 Sekunden wiederholen. Falls Manifest/Lock fehlen, den genauen Bedarf melden und zunächst nur Code abgeben, keine heimliche Änderung gemeinsamer Dateien.

## Routing

Auftraggeber Teil-Orchestrator Q, Session `c671588c-6192-4bf5-8206-28bb30163666`; Hauptorchestrator `43a4886c-e135-484b-838a-0512d224a634`. Status allein durch `teil-q`, Versuch 1. Nie `TODO.md` oder `REGISTER.md` schreiben. Ergebnisdatei `bereiche/q/Q2-WRITER.md` im gemeinsamen Auftragsordner `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung`. Melde konkrete Runtime-Konfig, zulässiges Unit-ExecStart, Manifestbedarf, Tests und echte Grenzen. Q ist zur Umstellung der beiden Writer-Units autorisiert, setzt dies nach Integration selbst um.
