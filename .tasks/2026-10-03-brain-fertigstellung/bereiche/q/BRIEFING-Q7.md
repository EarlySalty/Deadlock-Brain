status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Q7: vorhandenen Writer um vollständige YouTube-Metadaten und leere Mengen ergänzen

## Ziel und Vertrag

Native Implementiererrolle, keine weitere Delegation. Baue ausschließlich zwei konkret gelieferte Q2-Restpunkte weiter, nichts erneut aufbauen. Q2 hat seine vorhandenen Module geliefert; Ergebnis steht im Workflowjournal `wf_67c014a0-7d1/journal.jsonl` unter dem eigenen Sitzungsordner und in `bereiche/q/AN_HAUPT.md`. Verbindlich sind die bestätigten Rechte in `VON_HAUPT.md:32`: intern, unbekannte Lizenz, keine Veröffentlichung oder externe Modelle, aktuelle Fassung plus zwölf Monate Historie, Löschung verschwundener Quellen. Gemini, STT, Transkriptbeschaffung und YouTube-Lernen bleiben aus. Frische Metadaten sind ausdrücklich erlaubt.

Erster Restpunkt: Historisches read_existing ist kein aktueller Verfügbarkeitsbeweis. Ergänze frische vollständige YouTube-Metadaten-/Verfügbarkeitsprüfung über vorhandene zulässige Abrufbausteine und den zentralen Infisical-Weg. Vor Neubau Graphify nach vorhandenem YouTube-Metadaten-/API-Client fragen. Kein RSS- oder teilweiser Seitenabruf als vollständige Quelle. Kein LLM-Aufruf. Keine API-Secrets erfinden oder Gemini-Secrets als Metadatenzugang verwenden. Fehlender tatsächlich verfügbarer Metadatenzugang bleibt fail-closed und wird konkret gemeldet. Vollständigkeit, Seitenende, Kanal-/Videoidentitäten und nicht gekürzte Mengen müssen beweisbar sein; Abruf-/Quota-/Schemafehler dürfen keine Löschsignale erzeugen. Bestehende freigegebene Transkripte ausschließlich aus der lokalen lesenden Altbestandsverbindung erhalten, niemals neu lernen. Neue Videos dürfen reine Metadaten tragen. Nachweisbarer Verlust der Abrufbarkeit wird als solcher benannt, nicht als bewiesene physische Löschung ausgegeben.

Zweiter Restpunkt: Sicheren zusätzlichen Ingestionsvertrag für eine ausdrücklich vollständig gelesene leere Quelle bauen. Die bisherige allgemeine Schutzregel `prepare_document_batch` gegen leere Abrufe bleibt erhalten. Nutze einen gesonderten expliziten Vertrag, mit dem ein Adapter nach bestätigter vollständiger Quellenprüfung die letzten Datensätze tombstonen kann. Kein künstliches Sentinel-Dokument, keine ungeprüfte Nullmenge, kein Schema-/Status-Workaround. Vertrag und Sicherheitsgrenze müssen im Bericht stehen.

## Eigentum

Du darfst vorhandene eigene Q2-Pfade ändern: `rust/crates/brain-feeds/src/youtube_core.rs`, `source_sync.rs`, `sheet_core.rs`, `src/bin/brain-source-sync.rs`, `tests/source_sync.rs`; eng nötige neue Unterdateien innerhalb dieser Module. Für den zusätzlichen sicheren Leermengenvertrag besitzt du `rust/crates/brain-ingestion/src/document_set.rs` und ausschließlich dessen unmittelbar passende Tests. Bestehende reguläre APIs und andere Caller nicht inkompatibel ändern. Du darfst für diesen Auftrag keine Storage-, Contract-, API-, Service-, Provider-, Shadow-, Patchnotes-, Build-Publish-, Manifest-, Lock-, Migrations- oder Unitdateien ändern. Keine neuen Code-Kommentare, keine globale Formatierung. Bestehende Retentionsperre `require_history_retention` bleibt zwingend aktiv, bis ein wirklicher Storagepfad integriert ist.

## Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `5c220a8f047eb980d953d9f9f285b34739b5ed88`. Q1/Q2/Q3/Q4/Q5 sind geliefert, vorhandenes WIP erhalten. Parallel prüft Q6 die Consumer-/Auditpfade, ohne deine Dateien zu ändern. Keine Git-Mutationen, DB-Schreibzugriffe, Deploys, Neustarts oder echten Provider-/Quellenaufrufe durch dich. API-Abhängigkeiten und fremde Arbeit erhalten; nur additive sichere gemeinsame Ingestions-API. Z integriert die gekoppelten Stände und installiert das gemeinsame Release. Kein Einzelmerge.

## Beweisziel

Engste Implementierung mit passenden bestehenden Prüfungen und ehrlicher Grenze. Zuerst code-suche/Graphify, danach gefundene Ruststellen lesen. Vor Tests rolle-test-waechter, vor Text humanizer und no-em-dashes. Compiler nur Rust 1.97.1, höchstens zwei Jobs, beide blockierenden Hostlocks in vorgeschriebener Reihenfolge, frische NonZombie-Probe nach HOSTPROBE.md. Kein zeitbegrenzter Sperrversuch als Compilerfreigabe. Cargo mit --locked --offline, betroffene Tests mit --include-ignored. Fordert die vorhandene Crate ein Manifest-/Lockupdate, konkreten Bedarf melden statt gemeinsame Dateien zu ändern.

Belege normale leere Abrufe ohne Tombstones, explizit bestätigte vollständige leere Mengen mit korrekten Tombstones und idempotentem Folgeabruf, Abbruch oder unvollständige Metadaten ohne Quellenschreibwirkung und Fremdkanal-/Duplikatabwehr. Retentionsdurchsetzung und echter Live-Lauf sind ausdrücklich noch kein Teil deines Nachweises. Ein Quellenfehler muss vor Commit aller betroffenen Batches abbrechen. Volle Logs und tatsächliche Testzahlen nennen, keine Zeitlimits oder vorbereiteten Tests als bestanden melden.

## Routing

Auftraggeber Teil Q, Paket q, Versuch 1, Produzent teil-q. Hauptorchestrator Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Berichte direkt an diese native Hauptsession und nach `bereiche/q/Q7-WRITER-RESTPUNKTE.md` im gemeinsamen Taskordner. Kein TODO.md, REGISTER.md oder Statusereignis. Keine Kontakte zu fremden Sessions, keine neuen T3-Threads. Melde den genauen verbleibenden Storage-, Grant-, Konfig- oder Quellenzugangsbedarf. Nichts als integrationsfertig melden, solange die Retentionssperre noch greift.
