status: erledigt, begrenzte Read-only-Bestandsnachweise; keine Importfreigabe
Datum: 2026-09-30

# Archiv-, Rechte- und Pilotmetadaten um13:47UTC

## Tatsächlicher Zugriff

Vier begrenzte psql-Lesevorgänge mit Exit0 und ROLLBACK. Bestehender legitimer Weg: sudo -n -u nathanael -g deadlock-brain-db /usr/bin/psql -X -w, Socket /run/deadlock-brain-postgresql, Port5446, vorhandene Peer-Rolle brain_migrate. Je Verbindung BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY, statement_timeout2s, lock_timeout1s. Keine schreibende SQL-Anweisung, Schema-/Rollenfixture, Secretauflösung, ENV-Konfiguration, Compiler- oder Produktprozessaktion. Nur Metadaten, Counts und Hashes ausgegeben, keine Dokumente, Rohtexte oder Passwörter.

Transaktionszeiten:13:46:56.432864UTC und13:47:45.964702UTC in brain;13:48:15.251756UTC in brain_pilot_legacy;13:48:49.110278UTC wieder brain. Diese vier Transaktionen sind ausdrücklich kein gemeinsamer atomarer Snapshot über beide Datenbanken. Erste Ausgabe enthielt eine harmlose Locale-Fallbackwarnung, psql dennoch Exit0; keine Locale-/ENV-Änderung.

## Ziel und Archiv

- Zielbrain: Datenbank-OID16388, Coreschema brain OID16389, Archivschema brain_legacy OID25331. Schema2/brain.store.v2. Weiterhin keine Heads, Revisionen oder CorpusReleases (EXISTS-Prüfung).
- Archivtabellen: entities905, entity_aliases3778, patch_event_enrichments32821, patch_events32821. Separat348 unterschiedliche patch_external_id.
- Keine mehrfachen Enrichments je Event, keine NULL-patch_external_id, keine verwaisten/NULL-Entity-Verknüpfungen der Aliase, keine verwaisten Enrichment-Verknüpfungen in den geprüften Counts.
- brain_readonly hat SELECT auf alle vier Archivtabellen, jeweils kein INSERT/UPDATE/DELETE. Das bestätigt bestehende Tabellenrechte, keinen SCRAM-/Secret-Exec-Lauf unter dieser Rolle.

Die Schemasignatur folgt der bestehenden pg.rs-Projektion: information_schema.columns für die vier Tabellen, sortiert table_name/ordinal_position, Zeile table_name.column_name:data_type:is_nullable, LF-verbunden ohne abschließendes LF, SHA256 der UTF8-Bytes.

`schema_sha256 = 5fe2c40427d3dd57f3d07b714936a633e372ae0a22b6cee4a237a58366aa3a4f`

Berechnet unter brain_migrate. Sichtbarkeit der information_schema-Projektion unter der tatsächlichen Importleserrolle bleibt beim späteren gebundenen Import zu prüfen. Dies ist kein snapshot_sha256 des Importers, kein eingefrorener Archivrelease und keine Freigabereferenz.

## Rechte-/Deletehinweise im Archiv

Alle905 Entity- und32821 Patchmetadaten sind JSON-Objekte. In den geprüften vier Tabellen existieren keine expliziten Spalten mit den Suchbegriffen deleted/revok/tombstone/visibility/scope/policy. Es bestehen historische Snapshot-ID-/Zeitstempelspalten. In den obersten Entity-/Patchmetadata-Schlüsseln wurden keine Treffer für delet/revok/tombstone/visibility/scope/policy/approv/snapshot gefunden.

Das ist ausschließlich ein enger Struktur-/Schlüsselbefund. Verschachtelte Metadaten, andere bestehende Quellen und externe Freigabeunterlagen sind damit nicht ausgeschlossen. Daraus darf insbesondere weder ein leeres Widerrufs-/Tombstoneinventar noch eine öffentliche Freigabe abgeleitet werden.

## Vollständige aggregierte Pilotkopfdaten

| Quelle | Sichtbarkeit | Scopes | Aktuelle Heads | Tombstones |
| --- | --- | --- | --- | --- |
| legacy-entities | public | game.public |905|0|
| legacy-patchnotes | private | brain.legacy.review |348|0|

Diesmal alle aktuellen Pilotheads aggregiert, nicht bloß die frühere20er-Stichprobe. Entityheads zuletzt26.09./09:05:54.473702UTC geändert, Patchheads26.09./09:06:26.780956UTC. Historische Revisionen:905 Entityrevisionen und1044 Patchrevisionen, alle ohne Tombstoneflag. Keine Rohinhalte gelesen. Frühere Patchrevisionen wurden nicht als aktuelle Freigabe bewertet.

Metadaten-Auditfingerprint je Quelle: SHA256 über LF-verbundene jsonb_build_array(logical_id,revision,content_hash,tombstone,visibility,allowed_scopes)::text, sortiert logical_id, UTF8. Keine Datensatzinhalte ausgegeben.

- legacy-entities: `8943663c15ee8f4b442ff99ac3e1b65eb6ab57f2a6e0e689cbe3ebcee3e9932a`
- legacy-patchnotes: `f78a6d815083a4559d86014ec8d26d497a20420b7949f9bea6484a0c178993bd`

Diese beiden Fingerprints sind selbst beschriebene Metadaten-Auditwerte, ausdrücklich nicht die B2-Felder snapshot_sha256 oder policy_sha256.905/348 stimmt zahlenmäßig mit aktuellem Archivbestand überein, beweist aber weder Inhaltsgleichheit noch historische Deletevollständigkeit. Kein blindes Kopieren des Piloten nach Produktion.

## Ergänzung: tatsächliche Herkunfts- und Policyabdeckung ab14:35UTC

Sechs weitere begrenzte Read-only-Transaktionen aufbrain, bestehender Zugriff und dieselben2s/1s-Grenzen, jeweilsExit0/ROLLBACK. Keine Rohdokumente, Payloads, Quell-URLs oder Secrets ausgegeben. Die vorhandenen Metadaten wurden gezielt entlang der Snapshotverknüpfung geprüft, nicht als neue Freigaben erzeugt.

- Archiv hat49 Tabellen; darunter37123 entity_snapshots,7308 source_documents,558 source_runs,810 entity_lineage und26685 knowledge_events. Diese Bestände existieren bereits, keine neue Herkunftsablage nötig.
- Alle905 Entityzeilen haben first_snapshot_id mit vorhandenem Snapshot. Alle32821 Patchzeilen haben patch_snapshot_id mit vorhandenem Snapshot. Keine fehlenden referenzierten Snapshots oder Quelldokumente in diesen Joinprüfungen.
- Entitäten hängen an5 unterschiedlichen vorhandenen Quelldokumenten, Patches an428; keine NULL-Verknüpfungen. Das sind428 Quelldokumente für348 Patch-IDs, ausdrücklich kein Eins-zu-eins-Snapshot oder Beweis aktueller Lizenzrechte.
- Keines der5 Entityquelldokumente hat den obersten Metadatenkey policy.136 der428 Patchquelldokumente haben policy,292 nicht. robots_policy kommt in diesen beiden verknüpften Teilbeständen nicht vor. Im gesamten source_documents-Bestand existiert policy146-mal und robots_policy600-mal; diese weiteren Zeilen nicht blind auf den B2-Teilbestand übertragen.
- Die136 relevanten policy-Werte sind zwei vorhandene Freitextwerte, keine strukturierte Scope-/Egress-/Publikationsfreigabe. Wortlaut nicht ausgegeben. Wert-SHA256 `1777afc11bbb316b18522027165266456dbffeb8312ca6dccad3ae4caf1abb39`:117 Dokumente,74 Zeichen. Wert-SHA256 `2b6c0beec78ca73174f72f3a18e1c48524857917d008ba06ae8b2dc4220e2fb6`:19 Dokumente,76 Zeichen. Beide erwähnen history, die geprüften Begriffe robots/disallow/public/private/auth/archive nicht. Wortvorkommen sind ausdrücklich keine Rechteentscheidung; aus diesen Tests wird kein allow/deny abgeleitet. Diese Werte sind nicht der B2-policy_sha256.
- knowledge_events enthält unter anderem1637 Ereignisse removed mit validity_status patch_history und currentness historical_patch_event. Das bezeichnet vorhandene fachliche Patchhistorie, nicht automatisch1637 gelöschte Dokumente oder Widerrufe. Keine Tombstoneliste daraus erzeugen.
- source_runs enthält548 ok,8 error und2 running mit historischen imported_at-Zeitpunkten. Ein historisches running ist kein Nachweis eines aktuell laufenden Writers. Keine Dienste oder Jobs deshalb verändert.

Die vorhandene Doku architecture/migration/BRAIN_POSTGRES_ISOLATION.md:10-20 belegt eine historische Archivkopie vom26.09. mit49 Tabellen und653476 Zeilen sowie damaligem Tabellenvergleich. Ihre Marker sind an diesen alten Snapshot gebunden und nennen Produktionsbereitschaft ausdrücklich NEIN. Das ersetzt keinen neu gebundenen Importerfingerprint oder aktuelle Rechteinventare. Die neue Prüfung schließt die Herkunftslink-Lücke für den betrachteten Teilbestand, nicht die Freigabe-/Widerrufslücke.

## Sachlich offen

Tatsächlich gebundener Archivsnapshot mit Label/Epoch und Importerfingerprint, belegte Herkunfts-/Policyentscheidung, vollständige aktive/widerrufene/tombstonierte ID-Klassifikation und deren aktueller Geltungsnachweis. Die konservative private Patchnotessperre ist nun am gesamten aktuellen Pilotbestand belegt. Keine Freigabe aus Platzhaltern, OIDs oder bloßer Hashkonsistenz ableiten. Die bereits erteilte Cutoverbeauftragung bleibt gültig; fehlende Belege werden ermittelt, nicht durch eine neue pauschale Erlaubnisfrage ersetzt.
