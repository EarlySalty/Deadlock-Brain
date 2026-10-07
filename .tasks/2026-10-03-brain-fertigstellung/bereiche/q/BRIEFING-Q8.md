status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Q8: konkrete Storage-Retention der gelieferten Writer

## Ziel und Vertrag

Native Workerrolle, keine weitere Delegation. Löse den bestätigten Q2-Storageblocker soweit der vorhandene Kernvertrag das sicher erlaubt. Keine erneute Writerimplementierung. Verbindliche Betreiberentscheidung `VON_HAUPT.md:32`: Sheet-/YouTube-Bestand nur intern, unbekannte Lizenz, keine Veröffentlichung oder externe Modelle, aktuelle Fassung plus zwölf Kalendermonate Historie, Entfernung bei verschwundener Quelle. Metadaten oder eine erfolgreiche Timerantwort erfüllen diese Aufbewahrung nicht. Die Writer bleiben gesperrt, bis ein echter normaler Storepfad integriert und geprüft ist.

Vor Neubau Graphify global und im bestehenden Brain-Graph nach Retention, Purge, Quellenwiderruf und Release-Pins fragen. Die Hauptsession hat keine PgStore-/Retentionknoten im aktuellen Graph gefunden; kein Graph-Neubau. Danach alle einschlägigen vorhandenen `brain-storage`-Module, Schema, tatsächliche Migrationen und Rechte sowie Snapshot-/Readerverträge lesen. Bekannte relevante Dateien: `src/lib.rs`, `pg_release.rs`, `pg_jobs.rs`, `schema.rs`, `local_pg_reader.rs`, `tests/ir_contracts.rs`. Rechtewiderruf und historische Releaseauthorization sind bereits vorhanden und müssen wiederverwendet werden. Das Fehlen eines Purgepfads wurde noch nicht als vollständiger Ausschluss aller Nebenpfade bewiesen.

Baue einen engen transaktionalen PgStore-Retentionspfad für die eigenen Quellfamilien `google-sheet/` und `youtube-core/`, keine allgemeine Löschfunktion für fremde Quellen. Er muss echte Inhalte und abhängige materialisierte Daten erfassen, aktuelle nicht verschwundene Fassungen erhalten, überholte Fassungen nach der bestätigten Frist entfernen und nach verlässlich bestätigtem Verschwinden Inhalte nicht weiter lesbar halten. Keine bestehende Record-/Corpus-Hashbindung still verändern und keine dangling Release-Pins erzeugen. Bereits laufende Reader und Caches müssen im Aktivierungsvertrag berücksichtigt sein; eine gespeicherte Löschung allein darf nicht als ausgelieferter Datenschutzbeweis gelten.

Wenn die Rechte oder immutable Releaseverträge einen sicheren direkten Pfad nicht erlauben, liefere den konkret erforderlichen abgestuften Commit-/Aktivierungs-/Purgevertrag an Q/Z mit Belegstellen. Kein als fertig behaupteter Stub, keine stillschweigende Lockerung der Aufbewahrung, keine fremden oder aktiven Releases pauschal löschen. Das ist ein technischer Implementierungsblocker, keine erneute allgemeine Produktfrage. Neue nötige DB-Rechte so eng wie möglich vorschlagen, keine allgemeinen DELETE-Rechte für brain_ingest.

## Eigentum

Eigene neue Module innerhalb `rust/crates/brain-storage/src/` und passende Storage-Tests. Solange Q6 die Consumer prüft, bestehende Storage-, Contract- oder Readerdateien nur lesen; neue Module zunächst nicht in lib.rs verdrahten. Q6-Abschluss steht in `bereiche/q/Q6-CONSUMER-PRUEFUNG.md`; nicht darauf untätig warten, sondern den vorhandenen Vertrag und konkrete Umsetzung im eigenen neuen Modul ausarbeiten. Danach darfst du `brain-storage/src/lib.rs` ausschließlich um den nötigen Modulexport ergänzen; bestehende Runtime-/Readersemantik nicht außerhalb des begründeten eigenen Retentionspfads verändern. Keine Feeddateien, dort baut Q7. Keine API-, Service-, Provider-, Shadow-, Contract-, Manifest-, Lock- oder Unitdateien. Bericht über genaue zusätzliche nötige Integration statt fremde Pfade zu ändern. Keine neuen Code-Kommentare, keine globale Formatierung.

Gemeinsame Storage-/Migrationsänderungen sind in AN_HAUPT.md angekündigt. Bestehende Migrationen niemals ändern. Keine Migrationsnummer vergeben, bevor die Hauptsession origin/main frisch geholt hat; bei Migrationsbedarf zunächst in deinem eigenen Bericht den vollständigen SQL-Vertrag ohne vergebene Nummer liefern. Keine Migration anwenden und keine Datenbank beschreiben.

## Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `5c220a8f047eb980d953d9f9f285b34739b5ed88`. Erhaltenes WIP aller Q-Worker bewahren. Parallel Q6 Consumerprüfung und Q7 YouTube-/Leermengenergänzung in getrennten Pfaden. Höchstens drei aktive native Worker. Keine Git-Mutation, kein Deploy, Neustart, Quellenabruf, Anbieteraufruf oder DB-Schreibzugriff. Z ist Integrationsverantwortlicher und führt gemeinsame Abnahme, Gate, Installation und Umschaltung durch. Q übergibt lokal geprüften SHA vor eigenem Live-Nachweis.

## Beweisziel

Lade code-suche, rolle-test-waechter, humanizer und no-em-dashes. Eigene reine Planungstests dürfen Fristen und unveränderte SourceRecords prüfen; sie ersetzen keine echte PgStore-Wirkung. Compiler ausschließlich Rust 1.97.1, höchstens zwei Jobs, beide Hostlocks blockierend in richtiger Reihenfolge, frische NonZombie-Probe nach HOSTPROBE.md. Keine parallele Compilerfreigabe und keine fremden Prozesse stoppen. Bestehende Storage-Suites erhalten, betroffene Tests mit --locked --offline und --include-ignored; fehlende erlaubte Test-DSN konkret melden. Keine produktive DB als Testziel. Nicht verdrahteten Code als kompiliert oder geprüfte Wirkung ausgeben.

Bericht nennt vorhandene Komponenten und Ausschluss der Nebenpfade, tatsächlich geänderte Dateien, spezifische Purge-/Pin-/Cachegrenzen, notwendigen minimalen Migrations-/Grantvertrag, echte Exitcodes/Testzahlen sowie die genaue noch nötige Verdrahtung mit Q7 und Z. Einziger Bug-/Securityreview bleibt das Gate. Kein eigener Reviewthread; du bist Implementierer des Retentionspfads.

## Routing

Auftraggeber Teil Q, Paket q, Versuch 1, Produzent teil-q. Hauptorchestrator Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Direkt an diese native Hauptsession und `bereiche/q/Q8-RETENTION.md` im gemeinsamen Taskordner berichten. Kein TODO.md, REGISTER.md oder Statusereignis, keine Sessionkontakte, keine neuen T3-Threads. Bei Vertragsgrenze mit konkretem Lösungsweg melden und eigenen sicheren Rest fertigstellen, keinen Liveabschluss behaupten.
