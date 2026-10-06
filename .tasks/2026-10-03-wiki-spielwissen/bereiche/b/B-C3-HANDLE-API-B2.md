status: aktiv
Datum: 2026-10-03

# B2 bestätigt C3s Handle-Vertrag

Punkt 44 und C3-B-BYTEVERTRAG.md vollständig gelesen. Kleinster Zusatz zum bestehenden Extraktor, alter Git-/Lose-Datei-Aufruf unverändert:

```rust
pub struct BoundGameFile {
    pub relative_path: String,
    pub file: std::fs::File,
    pub expected_bytes: u64,
    pub steam_sha1: [u8; 20],
    pub expected_sha256: [u8; 32],
}

pub struct BoundGameFiles {
    pub options: GameFileOptions,
    pub manifest_transport_sha256: [u8; 32],
    pub files: Vec<BoundGameFile>,
}

pub fn extract_game_files_from_handles(
    input: BoundGameFiles,
    output: &mut impl std::io::Write,
) -> std::io::Result<GameFileInventory>;
```

**Genau benötigte B-Pfade:** `rust/crates/dbrain-sources/src/game_files.rs` und `game_files/vpk.rs` samt dortigen bestehenden Testmodulen. `game_files/anchored.rs` kann unverändert bleiben. Vier übrige Parserdateien und Cargo/Lock/lib.rs/CLI bleiben unberührt. Bestehende Parserlogik, Budgets und Legacy-API wiederverwenden. Kein zweiter Extraktor.

## Konkrete Besitz-/Beweisgrenze

C3 erstellt vollständig an D-Inventar/Manifest gebundene private unveränderliche Lesesicherungen. Ein Eingang gehört genau zu einem Root/App/Build/Depot/Manifest; Build/Depot/Manifest vorhanden und Git-source_revision nicht als Steamrevision verwenden. C3 hält vollständigen SourceInventory-Beleg einschließlich Verzeichnissen und anderer Typen. `files` enthält sämtliche regulären Inventardateien, auch übersprungene Assets und VPK-Begleitarchive. `options.root` ist im neuen Eingang Herkunftsangabe, kein Öffnungsziel.

Aufruf konsumiert und hält Dateiobjekte bis zum Ende. B setzt Cursor explizit; try_clone teilt Dateiposition, C3 verwendet Aliasdeskriptoren währenddessen nicht parallel. B prüft eindeutige normalisierte relative Pfade, regulären Typ, tatsächliche Größe und gestreamten SHA-256 am übergebenen Objekt, ohne Vollcontainer-Allokation. Steam-SHA-1 ist die bereits durch C3 geprüfte Bindung und wird weitergeführt, keine neue Hashabhängigkeit.

Lose Texte, VPK-Verzeichniscontainer und exakt abgeleitete nummerierte Begleitarchive werden nur aus diesem Handlebestand gelesen. Keine Originalnamensöffnung, kein /proc-Reopen, keine freie Roottraversierung. Fehlende/ungeklärte Archive und Bindungsfehler führen zu Err, nicht zu bestätigter Manifestherkunft. Physische Referenzen in `extraction.physical_sources`: Pfad, Größe, SHA-1/SHA-256; VPK-Ressourcen-/Inhaltshash getrennt. C3 darf bei Err eventuell teilweise beschriebene Ausgabeartefakte nicht importieren.

Root vermittelt diese bestätigte API an C3. B setzt danach ausschließlich zwei genannte Pfade um, prüft selbst und lässt unabhängig abnehmen, zusätzlicher Eigencommit; akzeptierter48b6ce1 bleibt unverändert. Aktuell nur lesender Signaturvorcheck abgeschlossen, noch keine Implementierungs-/Deployfreigabe. Finale gemeinsame D/B/C-Prüfung vor Steam-Deploy, echte Depotextraktion danach.
