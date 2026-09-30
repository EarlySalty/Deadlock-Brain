status: aktiv, direkt beauftragte Quelländerung; Ausführung separat koordiniert
Datum: 2026-09-30

# B2: vorhandenen Rust-Importer für brain_legacy nach brain absichern

Intent-Thread: 562a877b-0939-440a-964d-1145d9e9431a. Bestehender einziger Sol-Thread66adf9ee-bc03-4ff3-91da-73cd8efc5e72. **Erst laufendes B1 sauber abgeben**, dann dieses Paket auf demselben eigenen Branch fortsetzen. Keine Unterthreads oder Unteragenten. Kein neuer Importer, keine neue Architektur, Rolle oder Modellroute. Rust-only, keine Code-Kommentare.

## Direkter Nutzerauftrag

„Den bestehenden Rust-Importer für den bereits autorisierten Cutover eng und überprüfbar auf brain_legacy→brain erweitern. Keine neue Importarchitektur, kein Rollen-/Modellneubau und keine blinde Pilotkopie. Bestehende Sperren gegen identische Quelle/Ziel sowie falsche Ziele erhalten; produktiver Zielpfad muss explizit gebunden und failclosed sein.“

„Vorhandene Quellen-/Scope-/Freigaberegeln beibehalten, insbesondere die privat gesperrte Patchnotes-Stichprobe nicht still öffentlich freigeben. Unabhängige Abnahme des konkreten Deltas und gezielte Gegenbeweise gehören vor den produktiven Import.“

Keine neue pauschale Nutzerfreigabe nötig. Compiler, isolierte PG-Harnesses, Produktimport, Release und Dienste bleiben beim Integrator. Kein Produktionsschreiben oder Serve-Cutover aus einer statischen Freigabe.

## Arbeitsstand und erlaubter Scope

Worktree /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930, Branch fix/g5-replay-deferred-20260930. Ausgang B1 von ca4a8f236a75ba89ce60d2140c735acd5d1f5bee, danach deinen tatsächlichen B1-Abgabehead festhalten. Produktbasis9a29b81 hatte 954 passed/74 ignored, Clippy und Formatprüfung bestanden; diese Belege nicht als Tests deiner neuen Ruständerung ausgeben.

Scope nur bestehender Importer rust/crates/brain-legacy-import/, dessen bestehende normale JSON-Importconfig unter ops/brain-postgres/ und eigene B2-Abgabedokumentation in .tasks/2026-09-30-g5-abschluss/. Bestehende Pilotconfig nicht als produktiv umbenennen; nötige neue normale Config ausdrücklich als noch unvollständig/nicht ausführbar kennzeichnen, keine Platzhalter mit automatischem Default auf eine echte DB. Kein Lock-/Dependencywechsel ohne echten belegten Bedarf, keine anderen Produktcrate-Änderungen, kein SQL-Direktimport oder Serviceumbau. Bestandssuche Graphify vor Codefragen.

## Geprüfter Ist-Vertrag

- `rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs:56-62`: Ziel muss brain_pilot* sein, gleiche legacy-/target-Datenbank wird abgewiesen. Das ist eine absichtliche Vor-G5-Schranke, kein beliebiger Fehler zum Weglöschen.
- `:40-52`: Unix-Socket, PgConnectOptions::new_without_pgpass, explizite Endpunktfelder und bestehender auth_env-Secretreferenztransport. Kein Passwort in JSON, Log oder Bericht.
- `ops/brain-postgres/legacy-core-import.json`: bisher Quellebrain, Zielbrain_pilot_legacy. Quellen genau legacy-entities und legacy-patchnotes, historisch game.public, Egress/Publikationfalse. Diese Vorlage ist keine Produktionsfreigabe.
- `lib.rs:455-480`: vorhandenes DocumentSetSource/prepare_document_batch; `:491-520`: deterministischer Release aus echten Checkpoints, kein Dummyrelease. Bestehende Transaktions-/Lease-/Checkpointpfade wiederverwenden.
- Tatsächliche begrenzte Leseprobe der Hauptsession: Ziel-DBbrain auf Socket/run/deadlock-brain-postgresql, Port5446, Schema2/storev2, KEINE Source-Heads/Revisionen/Releases. Archivschema brain_legacy existiert.
- Rollen bestätigt: brain_readonly nur Lesen, brain_ingest bestehende Ingestrechte, brain_service SELECT auf Kern und INSERT nur Conversation-Ownership. Rollenneubau oder Grantänderung nicht nötig und nicht erlaubt.
- Pilotbrain_pilot_legacy hat historische Releases, ist aber keine fertige öffentliche Kopie. Aktuelle Kopfmetadaten: legacy-entities öffentlich/game.public; legacy-patchnotes privat/brain.legacy.review. Die begrenzte Folgeprobe bestätigt mindestens20 private Patchnotesköpfe (z.B.patch/10, patch/11), nicht nur einen zufälligen JSON-Fehlbefund. Keine Rohinhalte gelesen. **Keine stille Freigabe nach game.public.** Ob die übrigen Heads/Deletes vollständig abgedeckt sind, ist noch kein Vollnachweis; fehlende Policy-/Tombstonegrundlage muss die Produktionsausführung blockieren, nicht auf öffentlich fallen.

## Erforderliche Wirkung

1. Bestehender Pilotweg bleibt ausdrücklich begrenzt. Produktionspfad ausschließlich durch explizite normale Config-/Cutoverbindung, nie durch ENV-Schalter, Namenspräfix oder zufällige gleiche DB freischalten.
2. Quell-/Zielidentität enthält die Schemagrenze: derselbe Cluster/DB ist für brain.brain_legacy → brain.brain nur unter genau geprüfter Bindung zulässig; identische logische Quelle/Ziel oder andere Schemas/Instanzen/Zieldatenbanken bleiben verboten. Die Quellenqueries bleiben auf das Archivschema beschränkt, Writes auf den vorhandenen Corestore. Keine beliebigen dynamischen SQL-Bezeichner aus Benutzereingaben. Guards vor erstem Zielwrite und vor Claim/Checkpoint/Release erzwingen.
3. Vorhandene getrennte Rollen/Secretreferenzen und normale Config nutzen. Keine Secrets lesen oder selbst auflösen; vorhandener Rust-Secret-Exec ist der spätere interne Transport. Kein Nutzer-/Communitymaterial an Modelle senden.
4. Quelle, freigegebener Snapshot, Scope-/Visibility-/Egress-/Publikationsregeln, aktuelle Widerrufe und Tombstones müssen für Produktionslauf ausdrücklich gebunden sein. Fehlende oder widersprüchliche Bindung blockiert. Keine Pilotpayloads pauschal kopieren, keine privaten Patchnotes öffentlich umstempeln. Konservative Einschränkung nicht als volle Quellenparität behaupten. Wenn die vorhandene Schnittstelle für den Nachweis noch einen klar abgegrenzten Pflichtinput benötigt, den vorhandenen Vertrag minimal erweitern und vollständig dokumentieren, keine zweite Policyablage bauen.
5. Idempotenz, Fehleratomarität, Releasepins und Rückweg prüfen: bei Abbruch kein als vollständig veröffentlichter Teilrelease; erneuter Lauf ohne doppelte Records; bestehende Sperren/Deletes und nachlaufende Writes dürfen durch Reimport oder Rollback nicht verschwinden. Kein Löschen des Archivs oder historischer Releases.

## Gegenbeweise als Code und spätere genaue Runanforderung

Gezielte bestehende Unit-/CLI-/Scratchtests ergänzen oder anpassen, soweit sie das konkrete Delta beweisen: fehlende Produktionsbindung, falscher Socket/DB/Schema/Rolle, identische logische Quelle/Ziel, fehlende Policybaseline, private Patchnote, Tombstone/Scopewiderruf, Abbruch vor Release, Wiederholung. Positiv genau die erlaubte Archiv-zu-Core-Kombination im privaten isolierten Aufbau. Produktionsnamen allein sind kein Sicherheitsbeweis; Negativfälle müssen vor jedem Schreibpfad greifen.

**Jetzt keine Tests ausführen.** Nur Quell-/Format-/Diffprüfung, keine Cargo-Compiler, DB-Verbindung, Harness-/Prozess-/Modell-/Dienstaktion. Echte gezielte Gegenproben im Bericht exakt als noch benötigte Slots aufführen, inklusive etwaiger Migration/Fixture-/Lastwirkung. Keine Testabschwächung für grün und kein heimliches --ignored.

## Abgabe

Zuerst eigenes Delta statisch prüfen. B2-IMPORT-ERGEBNIS.md: tatsächlicher Head, vollständiger Laufzeitvertrag und Configfelder, unveränderte versus erweiterte Guards, vorhandene Rollen-/Secretreferenzen, Herkunft/Policy-/Tombstonebindung, Atomarität/Rückweg, exakte später auszuführende Compiler-/Test-/Harnessbefehle und offene Grenzen. Keine übernommene globale G5-Freigabe.

Nur eigene Dateien committen und auf eigenen Branch pushen. Keine neuen Threads, keine Wache, kein Main-Merge, keine Installationsaktion. Danach unabhängiger vorhandener Reviewer52c34332 für genau dieses Delta und dessen Schutzgrenzen. Falls die verbindliche Scope-/Rechte-/Tombstonegrundlage nicht ohne Produktentscheidung festlegbar ist: konkreten fehlenden Input samt sicheren bereits umsetzbaren Teilen melden, niemals passend erfinden.

Bump-up: [Bump-up] Paket B2-Produktimport: Grund: ... Erledigt: ... Worktree: ... Offen: ... an den Intent-Thread562a877b-0939-440a-964d-1145d9e9431a, danach stoppen.
