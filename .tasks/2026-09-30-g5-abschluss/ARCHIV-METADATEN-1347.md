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

## Sachlich offen

Tatsächlich gebundener Archivsnapshot mit Label/Epoch und Importerfingerprint, belegte Herkunfts-/Policyentscheidung, vollständige aktive/widerrufene/tombstonierte ID-Klassifikation und deren aktueller Geltungsnachweis. Die konservative private Patchnotessperre ist nun am gesamten aktuellen Pilotbestand belegt. Keine Freigabe aus Platzhaltern, OIDs oder bloßer Hashkonsistenz ableiten. Die bereits erteilte Cutoverbeauftragung bleibt gültig; fehlende Belege werden ermittelt, nicht durch eine neue pauschale Erlaubnisfrage ersetzt.
