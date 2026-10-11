status: aktiv, ausdrücklich erlaubte isolierte Quellarbeit; keine Compiler- oder Laufzuteilung
Datum: 2026-09-30

# Enger Read-only-Snapshotmodus im bestehenden Rust-Importer

Intent562a877b-0939-440a-964d-1145d9e9431a, bestehender einziger Autor66adf9ee-bc03-4ff3-91da-73cd8efc5e72. Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder Unter-Agenten spawnen. Kein Modellwechsel. Keine Code-Kommentare.

## Ausdrücklicher Auftrag und getrennte Arbeitsorte

Der Nutzer erlaubt nach abgeschlossenen U2-/PG1-Läufen den engsten Read-only-Zusatz am BESTEHENDEN Rust-Importer, falls der ausführbare kanonische Fingerprintmodus fehlt. Derselbe Leser, Config und Secretweg; garantiert keine Zielmutationen, kein zweiter Importer/Loader. Ausgabe nur tatsächliche Snapshotmetadaten/Hashes/Counts/IDs. Kein Secret oder Rohinhalt. Neu erstelltes Snapshotlabel ausdrücklich als neue Beobachtung kennzeichnen, niemals als historischen Wert. Interne technische Übernahme ist beauftragt, aber keine neue Veröffentlichung oder Provider-Egress. Keine Policy ändern, keine approval_ref erfinden oder leere Widerrufslisten aus Unwissen erzeugen.

NEUER isolierter Arbeitsort nur für diesen Zusatz: /home/nathanael/.worktrees/brain-g5-readonly-snapshot-20260930, Branch fix/g5-readonly-snapshot-20260930, frisch und sauber auf c5951b610aa2545d2c0b43b33b5fe1906198b292. Worktree gegen reguläres Cleanup gesperrt. KEIN neuer Cache. Der bisherige Worktree /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930 bleibt aufc5951b6 eingefrorener PG2-Prüfkopf: dort nichts editieren, umschalten, resetten oder mergen. Aus dem bestehenden Thread ausdrücklich am neuen isolierten Pfad arbeiten. Keine weiteren Worktrees anlegen.

## Geprüfter Bestand und eigentliche Lücke

Graphify zuerst abgefragt, keine präzise Zuordnung; konkrete vorhandene Pfade anschließend gelesen:

- Importerbinärdatei `rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs:463-495` akzeptiert genau `[_, flag, path] if flag == "--config"`, ruft `run(config)` auf und schreibt erst dessen Ergebnis. Kein beobachtender CLI-Modus.
- `:29-39` bestehende normale Config mit legacy/target, snapshot_label/epoch, owner/release/sources, production_binding und report. Keine neue Konfigwelt oder ENV-Schalter.
- `:156-170` bestehendes `options(&Endpoint)` mit new_without_pgpass und auth_env-Secretreferenz. Internen bestehenden Infisical-/Secret-Exec-Weg erhalten, keine Secretwerte selbst holen oder ausgeben.
- `:288-320` vorhandenes Quelllesen und Konvertieren; ab:321 Targetpool, danach PgStore, Claim/Commit/Publish. Beobachtungsmodus muss VOR diesem Pfad separat enden, nicht durch angeblich harmlosen Probeimport gehen.
- `pg.rs:19-25`: `BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY`, derselbe `read_inside`, abschließend `ROLLBACK`. Vier feste Archivtabellen, geordnete Entity-/Patch-/Aliasabfragen. Diesen kanonischen Leser wiederverwenden, keinen zweiten SQL-Loader bauen.
- `cutover.rs:39-53`: SHA256 über serde_json-Tupel `(label, epoch, schema_sha256, table_counts, entities, patch_lines, sources)`. Exakt diese Funktion wiederverwenden. Eigenes Nachbauen per SQLhash, Dateihash oder anderer Serialisierung liefert einen anderen Vertrag.
- `cutover.rs:57-64`: Policyhash bindet approval_ref, Policies und Zustandslisten. Der Read-only-Modus soll keinen gültigen Policyhash oder eine Freigabe erfinden.

## Umfang und Invarianten

Nur bestehende Crate brain-legacy-import, deren vorhandene Tests und eigener Bericht; nötige CLI-Dokumentation im bestehenden ops/brain-postgres-Bereich eng ergänzen. Keine andere Produktcrate, Manifest-/Lock-/Dependencyänderung, umfassende Formatierung, Modellroute oder neue Binärdatei. Keine produktive JSON-Vorlage ausfüllen. Bestehenden Importpfad und alle Cutoverguards unverändert wirksam erhalten.

1. Expliziter beobachtender Modus am selben Binary, mit bestehender Config. Fehlende production_binding darf die reine Beobachtung nicht zum Schreibpfad umleiten. Import ohne Binding bleibt weiterhin gesperrt. Config-/Pfadverwechslungen vor Verbindungen zurückweisen.
2. Ausschließlich tatsächliche Archiv-Leserrolle brain_readonly auf genau dem vorhandenen Unix-Endpunkt /run/deadlock-brain-postgresql:5446/brain mit BRAIN_LEGACY_READ_AUTH für diesen Produktionsbeobachtungsweg. Bestehende Identitätsabfrage wiederverwenden, tatsächliche Rolle/DB/OIDs/Unix prüfen. Keine Targetverbindung, kein Targetsecret erforderlich, kein PgStore, Migration, Claim/Lease/Checkpoint/Commit/Publish oder Conversationclaim. Read-only sowohl über Rollenbegrenzung als auch Transaktion nachweisen.
3. `read_legacy`, `entity_documents`, `patch_documents` und `cutover::snapshot_sha256` wiederverwenden. Nur eine zusammengehörige Archivsicht. Zeitpunkt/Label/Epoch und OIDs nachvollziehbar ausweisen; einen neuen beobachteten Snapshot nicht mit dem historischen Importlabel verwechseln. Keine erfundenen Herkunfts-/Importzeiten.
4. Ausgabe als explizite Whitelist: Snapshotlabel/-epoch und Beobachtungsart/-zeit, tatsächliche Identitäts-/Schema-OIDs, Schema-/Snapshotfingerprint, Tabellen-/Dokumentcounts und tatsächlich beobachtete logische IDs je Quelle. Keine Entitypayloads, Patchzeilen, URLs, Inhaltstitel, Metadatenobjekte, DB-DSNs oder Secretwerte. Auch Fehler nicht mit solchen Inhalten durchreichen. IDs als beobachtet, NICHT automatisch als freigegeben aktiv/revoked/tombstone klassifizieren.
5. Ausgabe an stdout genügt als sicherer existierender CLI-Weg; keine ungeschützte Dateiausgabe oder Überschreibung des Importreports. Falls vorhandener Reportpfad bewusst wiederverwendet wird, eigenständigen beobachtenden Berichtstyp und sichere Datei-/Fehlersemantik beweisen, niemals /DO_NOT_RUN umgehen. Kein Anspruch einer atomaren Policyfreigabe durch diesen Report.
6. Statische gezielte Gegenbeweise vorbereiten: ohne Targetzugang/Targetsecret lesend nutzbar; falsche Quelle/Rolle/DB abgelehnt; Import bleibt ohne Bindung gesperrt; Fehler leaken keinen Sentinelrohtext; Hash entspricht derselben kanonischen Funktion; beobachtete IDs werden nicht zu Freigabelisten; keine Zielmutation bei Erfolg oder Fehler. Vorhandene Testmodule nutzen. Tatsächlicher Lauf später separat, kein Test darf jetzt ausgeführt werden.

## Prüfung und Abgabe

Jetzt ausschließlich Quell-/Diff-/Formatprüfung, keine Cargo-Compiler/Tests, SQLverbindungen, Secretabfragen, Modelle, Medien, PG-Fixtures, Import/Serve, Dienste oder Release. Keine neuen Caches. Das alte grüne Clippy/U1/U2/PG1 aufc5951b6 gilt nicht für den Zusatz. Fehlende Laufbeweise ausdrücklich nennen, kein eigener Modell-Gate-Aufruf ohne zugeteilten Slot.

Vollständigen engen Diff selbst statisch prüfen, Quellfix committen und nur Branch fix/g5-readonly-snapshot-20260930 pushen. Separater Bericht READONLY-SNAPSHOT-ERGEBNIS.md mit tatsächlichem Quellkopf, gewählter CLI-Syntax, Config-/Secretbindung, reiner Read-only-Grenze, Ausgabe-/Fehlerwhitelist, neuen versus historischen Labels, exakten später nötigen Compile-/Fixture-/Produktionsleseproben. Keine Rohwerte oder Platzhalterrechte. Danach unabhängiger bestehender Reviewer52c34332 auf genau diesem Delta, nicht wieder gesamterG5.

Kein Main-Merge, kein Rebase des eingefrorenen Prüfziels, kein Reset oder Branch-/Worktreecleanup auf Stop-Hook-Zuruf. Bump-up nur bei echtem nicht auflösbarem Blocker: `[Bump-up] Paket Read-only-Snapshot: Grund: ... Erledigt: ... Worktree: ... Offen: ...` an Intent562a877b-0939-440a-964d-1145d9e9431a, danach stoppen.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 0 min, Nutzer untersagt zusätzliche Wache | Worktree: /home/nathanael/.worktrees/brain-g5-readonly-snapshot-20260930
