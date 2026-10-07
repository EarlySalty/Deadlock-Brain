status: aktiv
Datum: 2026-10-03

# Frischeimport: bestehender Weg reicht noch nicht

Vergleichsstand: `511a347b653beba13c2bf130f4bead7a7196cc2a`. Lesender Bestandsworkflow `wf_8bdf86e9-a76`, abgeschlossen. Keine Datenänderungen, Import-/Restore-Läufe oder Compilerprüfungen ausgeführt.

## Urteil und kleinster Übergabevertrag

Ein gebundener Erstimport des unveränderten Archivs ist vorhanden. Die verlustfreie Aktualisierung von DL-Main über ein bereits vorhandenes Archiv in einen bereits beschriebenen Kern ist noch nicht vollständig abgedeckt. G5-Schritt 4 bleibt gesperrt.

| Eigentümer | Zu schließender Vertrag | Wiederverwendbarer Ansatz |
| --- | --- | --- |
| Z, Ops | Generationeller Archivwechsel mit eindeutigem Staging, identischem lesendem Quellsnapshot für Dump und Fingerprints, vollständigem Vergleich und erhaltenem Vorgänger. Archiv, Kern und Rohdateien nicht löschen. Nach dem Wechsel neue tatsächliche OIDs beobachten. | `ops/brain-postgres/legacy-import.sh:29-49`, vorhandene Backup-/Restore-Probe. Neue Verwaltungslogik ausschließlich in Rust. |
| Q, bestehender Importer | Geprüfter Altbindung-zu-Neubindung-Übergang über vorhandene Heads, monotone Revisionen, belegte Widerrufe/Tombstones und erhaltene Checkpoints. Die aktuellen fail-closed Guards bleiben wirksam. | `rust/crates/brain-legacy-import/src/cutover.rs:166-205`, bestehende Bindung und atomare Dokumentverarbeitung. |
| Q, Corpus und Writer | Zusammengesetztes Release mit unveränderten Pins unberührter Quellen; Zuordnung der bisherigen Verarbeitungspunkte zu P/Q/S-Nachfolgern. | Bestehender Maintenance-Rebase-Vertrag und Release-Store, kein zweiter Veröffentlichungsweg. |
| Z und Q, Rückweg | Binary-/Serve-Konfigurationsrückfall unter Erhalt neuer Kernrevisionen, Checkpoints, Rechteänderungen und Publikationsstände. Keine Wiederherstellung eines Vor-G5-Backups über neue Daten. | Hashgeprüfter Configwriter, erhaltene Releasepins, Backup-/Restore-Prüfung zunächst in separater Instanz. |

## Konkret belegte Grenzen

1. Die Archivkopie ist kein Wiederholungspfad: `legacy-import.sh:41` entfernt eine feste Staging-DB; `:46-49` importiert die Schemaanlage erneut in `brain`. Ein vorhandenes `brain_legacy` lässt die Zieltransaktion an `CREATE SCHEMA` mit `ON_ERROR_STOP` enden. Der Hauptorchestrator darf dies nicht durch ein manuelles Löschen des Archivs umgehen. Teil-Z hat diese Stellen selbst nachgelesen.
2. Eine neue Beobachtungszeit bedeutet keine frische DL-Main-Kopie. Der Importer liest Archivtabellen. Die neue Bindung wird außerdem vollständig gegen vorhandene Heads geprüft, abgesehen von deren Revisionsnummer. Snapshotlabel und Herkunft gehören zur Projektion. Teil-Z hat `cutover.rs:166-205` selbst nachgelesen.
3. Der V1-Binder deckt den engeren Erstimport ohne bekannte lokale Widerrufe ab und erzeugt leere Widerrufs-/Tombstonelisten. Ein Bestand mit späteren Löschungen oder Rechteänderungen braucht den erweiterten Übergangsvertrag.
4. Das erzeugte Importrelease pinnt die zwei Legacy-Quellen. Es ersetzt keine zusammengesetzte produktive Corpusbindung. Archiv-Snapshot-SHA und Kern-Snapshotdigest dürfen nicht verwechselt werden. Vor einem Retry ist der persistierte Zustand zu prüfen, weil ein nachgelagerter Reportfehler einen bereits erfolgten DB-Commit nicht zurücknimmt.

Die Aufnahme liefert keine neuen Nutzungsrechte. Legacy-Patchnotes bleiben privat im Scope `brain.legacy.review`, ohne Anbietertransfer oder Veröffentlichung. Bestehende Guards und zentrale Secretwege bleiben unverändert. Ein öffentlicher oder externer Pfad wird aus einer Archivkopie nicht abgeleitet.

## Vorhandene Befehle nach geschlossenen Voraussetzungen

`brain-migrate check`, `brain-legacy-import --observe-snapshot`, `brain-legacy-import --bind-v1-config`, gebundener `brain-legacy-import` und `brain-maintain write-serve-config` sind bestehende Bausteine. Ihre privaten Konfigurationen müssen tatsächliche Rollen, DB-/Schemaidentität, Snapshot-/Rechtedigest und erwarteten Serve-Konfigurationshash tragen. Die leere Cutover-Vorlage wird nicht ausgeführt.

Z meldet den engen Importer-/Corpusvertrag in `AN_HAUPT.md` zur Zuordnung an Q. Parallel bereitet Z den vorhandenen Ops-Archivweg vor; keine Änderung an Q-eigenen Crates oder gemeinsamen Manifesten ohne Bereichsvertrag.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-legacy-import/src/bin/brain-legacy-import.rs:528 | Anknüpfung: Snapshotbeobachtung, Importbindung, Checkpoints, atomare Veröffentlichung und Backup-/Restore-Probe
