status: aktiv
Datum: 2026-10-03

# Gemeinsamer Download-Schreib- und Extraktionslesevertrag

Punkt41: C3 übernimmt ab 03.10.2026,09:57UTC die bisher C2 zugewiesene Integration und beide Deploys. Sämtliche operativen C2/B2-Abnahmeanforderungen unten gelten jetzt unverändert für C3/B2; historische B2-Berichte/Snapshots bleiben erhalten. D/Versuch1 und Datei-/Bytevertrag unverändert.

Diese Notiz hält den vor Bauabschluss geprüften B-Lesevertrag fest. Sie ist noch kein Nachweis eines kompatiblen fertigen Downloaders, kein grüner Test und kein tatsächlicher Spieldownload. C muss den fertigen D-SHA zusammen mit dem endgültigen B-SHA vor dem ersten Steam-Deploy abnehmen.

## B-Commitbindung aus Root-Wache 10:30 UTC

Root meldet geprüften B-Eigencommit 48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5. Caller bestätigt um 10:31:31Z das lokale Commitobjekt und bytegleiche aktuelle Kopien von game_files.rs, game_files/vpk.rs und game_files/anchored.rs. Neue SHA256: Hauptmodul b26a5035019b05c9920c8af14c30857a817d503882025874f73d24b4aee4e3da; VPK 142cefcbeafdb89c78b59d46dd6b0a2f3ee647f6401889c16d22c519834bf046; anchored e6bd4e5221d8806e85f043dc90a116db87848f38a45bad57dd37047858e09474. Die früheren WIP-Hashes bleiben historische Prüfbindung. Kein Nachweis aller B-Dateien allein aus diesen drei Vergleichen.

B bereitet den bestehenden D/B-Vertrag und die gemeinsame Abnahme vor, C3 ist informiert. Frische D-Folgerunde prüft die Schnittstelle gegen diesen belegten B-Commit, ohne B-Dateien zu ändern. Tatsächliche Inventar-/Originalbytebindung, neue Steam-/VPK-Extraktion und Importabnahme bleiben ohne D-Rohbestand offen.

## B-Leser

Pfad /home/nathanael/.worktrees/brain-wiki-spielwissen-b/rust/crates/dbrain-sources/src/game_files.rs. Zeitpunkt der gezielten Prüfung 2026-10-03T04:57Z. Datei-SHA-256 8ca3b514ac2fd4fc0301193114f381560db1322de851ffcb2f7c842f23e45fac. B-Datei zu diesem Zeitpunkt uncommittiert; SHA kann sich vor der Integration ändern. Keine B-Datei geändert.

Öffentlicher Einstieg: extract_game_files(&GameFileOptions, &mut impl Write) -> io::Result<GameFileInventory>. GameFileOptions erwartet root, app_id, source_id, observed_at, build_id, manifest_id, source_revision, depot_id, language, attribution, license_name, license_url, provenance und max_file_bytes. root ist ein bestehender normaler Rohdatenpfad; build_id/manifest_id/source_revision sind optionale Strings, depot_id optional u32. provenance muss ein JSON-Objekt sein. observed_at muss UTC mit T und abschließendem Z sein; C validiert zusätzlich vollständiges RFC 3339.

Als normales Originallayout ist provenance.source_layout=resource_paths erlaubt. Echte Steam-Originaldateien dürfen nicht als GameTracking-Ableitung bezeichnet werden. Für unbekannte Werte bleibt Option None; keine Clientversion als Steam-Build-ID oder Depotmanifest-ID verwenden.

## Anforderungen an den neuen D-Schreiber

1. Eindeutige Rohdatenroots pro tatsächlich regulär gewähltem Depot und Manifest sowie vollständige Originalrelativpfade beibehalten. Alternativ eine eindeutig belegte Dateizuordnung für jeden Manifeststand liefern. Keine pauschale manifest_id/depot_id über mehrere unterschiedliche Depots.
2. VPK-Directorydateien und ihre zugehörigen nummerierten Archive unverändert relativ beieinander belassen. B kann über die Directorydatei die vorhandenen Originalarchive lesen; ein extrahiertes Textverzeichnis ersetzt keinen vollständigen Steam-Spielbestand.
3. Pro Rohroot App-ID, tatsächliche Steam-Build-/Depot-/Manifestwerte, beobachtete UTC-Zeit, Plattform/Branch/Sprachen, relative Originaldateipfade, Größen und SHA-256 bereitstellen. Reguläre Quellenkennung, keine Zugangsdaten, CDN-Geheimwerte oder absoluten lokalen Benutzerpfade als öffentliche source_locator ausgeben.
4. Vollständigkeit ausschließlich nach validiertem gesamten Manifest-Dateibestand melden und an die tatsächlich gewählte Plattform/Branch/Sprachen binden. Teilbestände, negative Berechtigungsantworten, Timeouts und Fehler bleiben sichtbar. Keine Vollständigkeit aller Plattformen behaupten.
5. Download-/Taskinventar, D-Schreiber und B-Leser gemeinsam durch C prüfen. Unabhängige lokale D-Abnahme ersetzt nicht die gekoppelte Intent-/Gate-Abnahme. Die tatsächlichen Originaldateien werden nach C-Deploy mit dem vorhandenen B-Leser verarbeitet und danach durch C in den bestehenden Brain-Datenpfad importiert.

Die konkreten B-Felder und Grenzen wurden dem aktiven nativen Rust-Implementierer als Klärung seines bestehenden Auftrags übergeben. Keine zweite Implementierung oder Änderung an gemeinsamem Cargo-/Schema-/Corepfad.

## B2-Vertragsnachtrag, Punkte 30/31

D-B-LESEVERTRAG-B2.md im B-Bereich am 03.10.2026 um 07:15Z gelesen. Die dort genannten D-Hashes sind historische Snapshots, keine Abnahme des noch folgenden Fixstands. Der grüne Git-Exportlauf aus Punkt 30 ist keine Steam-/VPK-Depotabnahme.

C2 erzeugt pro tatsächlichem Depot GameFileOptions aus dem belegten Inventar. source_revision bleibt None, source_layout ist resource_paths, Sprache ohne Dateibeleg und. Manifesttransporthash und Inventarreferenz erhalten. Inventar ist keine fertige B-Optionsdatei. Millisekundenzeiten und echte Build-/Manifest-/Depotwerte müssen vom B-Validator angenommen werden; dessen Erweiterung bleibt bei B2/C2.

Unmittelbar am tatsächlichen B-Lese-/Importübergang prüfen C2 und B2 die exakte Dateiliste erneut gegen relative Pfade, Typ, Größe und echte Hashes der verwendeten Originalbytes. Nachträglich hinzugefügte oder ausgetauschte Dateien dürfen keine Manifestprovenienz erhalten. Fremddateien bleiben erhalten; Abweichungen führen zur Ablehnung des Roots oder einer ausdrücklich ausgewiesenen Grenze. VPK braucht getrennte Herkunftsebenen: verifizierte physische Directory-/Begleitcontainer und daraus gelesene Ressourcenbytes mit eigenem Ressourcenhash. Ressourcenhash und Containerhash niemals gleichsetzen.

B2 bestätigt zusätzlich eine Ressourcenlücke in D-store.rs: serde_json::to_vec reserviert vor der 256-MiB-Prüfung. Der frische D-Fixer begrenzt den vorhandenen JSON-Schreibpfad während der Serialisierung. Dieser Zusatz gehört zur bestehenden Ressourcenfundfamilie und ist kein gemessener Speicherfehler.

B-Grenzen bleiben separat: 32 MiB je Originaltextdatei, 64 MiB VPK-Baum, Tiefe 128, rechnerisch 512 MiB je Verarbeitungsschritt und 256 MiB Validatorzeile. D-Limits beweisen keine vollständige B-Verarbeitbarkeit. C2 verarbeitet die in Punkt 30 gemessenen maximalen JSONL-Zeilen von 117.157.259 beziehungsweise 11.107.243 Bytes verlustfrei im eigenen Importpfad. Keine D-Änderung an Parser oder Importgrenzen; Veröffentlichung echter Steam-Originale bleibt gesperrt.

Gemeinsame finale Abnahme erfordert stabilen D-/B-Commit und echten D-Rohbestand. Vollständiger zusätzlicher Extraktions-/Validatorlauf, Hashbindung, VPK-Nachbarschaft, Abdeckungsnenner, größte JSONL-Zeile und gemessene Ressourcen bleiben bis zum echten Download offen.

Speicherprüfung 2026-10-03T04:58:39Z: D-Rohdatenpfad liegt auf dem bestehenden Dateisystem mit 598 GiB verfügbar laut df. Das ist eine Momentaufnahme, kein Reservierungs- oder Vollständigkeitsnachweis.
