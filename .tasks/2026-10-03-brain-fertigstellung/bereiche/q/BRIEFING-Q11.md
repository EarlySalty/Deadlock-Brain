status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Q11: Retentionentwürfe im erhaltenen Stand konsolidieren

## Ziel und Vertrag

Native Workerrolle, keine weitere Delegation. Lies `Q8-RETENTION.md`, `AN_HAUPT.md`, `VON_HAUPT.md` und Zs `BETRIEBSVERTRAG.md`. Kein Neubau von null. Der Hauptorchestrator verlangt genau einen Storage-Retentionspfad statt zweier paralleler Entwürfe. Bestätigte Rechte: interne Sheet-/YouTube-Quellen mit unbekannter Lizenz, aktuelle Fassung plus zwölf Kalendermonate Historie, verschwundene Inhalte entfernen; keine Veröffentlichung oder externen Modelle.

Konsolidiere die erhaltenen Entwürfe `source_retention.rs`/`pg_source_retention.rs`/`pg_source_retention.sql` und `pg_q_retention*`. Bevorzugte Basis ist source_retention, weil dessen eigene Ablösezeitpunkttabelle die historische Frist unabhängig von später gelöschten Zwischenrevisionen erhalten soll. Prüfe diese tatsächliche Eigenschaft und übernimm nur gerechtfertigte Bestandsteile des zweiten Entwurfs. Keine alten Releases, Pins, Replayhashes oder Batchrohkopien still beschädigen. Kein allgemeines DELETE-Recht für brain_ingest. Ein SQL-Entwurf oder Aufbewahrungsfeld ist noch keine Wirkung.

## Eigentum und aktuelle Schreibgrenze

Nur die bereits vorhandenen neuen Q8-Retentiondateien und ihre eigenen Integrationstests unter `rust/crates/brain-storage/`. Ein endgültiger Pfad darf bleiben; den unbenutzten zweiten Entwurf vor Entfernung als nicht kompilierbare Referenz in einem eigenen Unterordner dieser Taskakte sichern, keine wertvollen Dateien verlieren. Vorher jede betroffene Datei lesen. Keine bestehenden Bibliotheks-, Schema-, Release-, Job-, Feed-, Ingestion-, Provider-, API-, Serve-, Maintenance-, Manifest-/Lock- oder nummerierten Migrationsdateien ändern.

Wichtig: Q10 prüft gerade den priorisierten Consumerstand. Seine tatsächlichen kompilierten Quellen müssen eingefroren bleiben. Deine bisherigen neuen Module sind nicht aus brain-storage/src/lib.rs exportiert. Du darfst deshalb hier ausschließlich diesen noch unexportierten Pfad und seinen vorhandenen gezielten Testharness konsolidieren. Kein lib.rs-Export in diesem Auftrag. Den echten PgStore-/Migrations-/Writeranschluss samt PostgreSQL-Wirkung danach als konkrete unmittelbar umsetzbare Restarbeit liefern, nicht als offen zu entscheidende Architekturfrage. Keine zweite Aktivierungsorchestrierung: Z hält Aktivierung, Readerwechsel und öffentliche Bindungen.

## Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `5c220a8f047eb980d953d9f9f285b34739b5ed88`. Beide Entwürfe uncommittiert, niemals produktiv angewandt. Q10 besitzt Consumer-/Serve-Konfiguration und hat Übergabepriorität. Neue Statusdateien, gemeinsame Berichte, TODO.md und REGISTER.md gehören nicht dir. Keine Git-Mutation oder neue T3-Threads.

## Beweisziel

Graphify zuerst, dessen neue Kernpfad-Abdeckung ist lückenhaft. Dann gezielt erhaltene Storage-/Schema-/Testverträge lesen. rolle-test-waechter, humanizer, no-em-dashes anwenden. Keine neuen Code-Kommentare, keine globale Formatierung, Rust ausschließlich. Beide Hostlocks blockierend, frische vollständige NonZombie-Probe, höchstens zwei Jobs, Rust 1.97.1, --locked --offline und Tests mit --include-ignored. Reine Sperrwartezeit ohne Timeout; vorhandenen eigenen PID nicht duplizieren. Keine fremden Prozesse stoppen.

Gezielten vorhandenen Retentionharness nach Konsolidierung kompilieren, ausführen, strikt linten und eigene Dateien formatprüfen. Keine PostgreSQL-Produktion mutieren, keine schreibenden psql-Aufrufe, keine angewandte Migration ändern. Für den Folgeanschluss den vorhandenen isolierten PostgreSQL-Testweg, nötige Grant-/Ownerfelder, Pin-/Release-/Checkpointkopien-/Readerverträge konkret ermitteln. Fehlt eine echte Testverbindung, nicht still überspringen und nicht als Livewirkung melden. Keine Secrets ausgeben oder in Dateien schreiben. `require_history_retention` bleibt unangetastet.

Gib den einen verbleibenden Pfad, aufbewahrte Referenz, vollständige Prüfbefehle mit Exitcodes und Testzahlen sowie exakte Anschlussdateien und Sicherheitsgrenzen zurück. Keine zusätzlichen Bug-/Securityreview-Agenten, einziger Review ist Zs gemeinsamer Gate.

## Routing

Auftraggeber Teil Q, Paket q, Versuch 1, Produzent teil-q. Hauptorchestrator Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Rückgabe nur an diese native Hauptsession. Kein Kontakt oder Resume laufender Workflow-/T3-Kontexte, keine weitere Delegation, kein Modellwechsel oder Sessionneustart. Lokal konsolidiert, exportiert, PostgreSQL-geprüft und live strikt getrennt melden.
