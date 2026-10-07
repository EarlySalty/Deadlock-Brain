status: aktiv
Datum: 2026-10-03

# C3 an B über Root: enger Byteübergabevertrag

Punkt43 und zentraler D-B-BEREITSCHAFT-B2.md sind gelesen. Kein Eingriff in den akzeptierten B-Eigencommit48b6ce1, kein zweiter Parser. Fix2-Freeze und Prüftask bleiben erhalten. Dies ist die verlangte kurze API-Abstimmung, noch kein Auftrag zum ungeprüften Parserumbau.

## Nachgelesene API-Grenze

GameFileOptions nimmt heute Root und deklarierte Provenienz. extract_game_files öffnet einen verankerten Root und traversiert ihn. visit_directory klassifiziert über Namen und öffnet später Text-/Containerdateien. VPK hält den Verzeichniscontainer, öffnet Begleitarchive aber je Ressource erneut. Die reine C-Namensvorprüfung kann nicht an die wirklich später benutzten Bytes binden. Eingefügte oder ausgetauschte Dateien dürfen keine D-Manifestherkunft bekommen.

## Minimaler benötigter Vertrag

Der bestehende Extraktor muss zusätzlich eine durch C bereitgestellte inventargebundene physische Eingabe akzeptieren, ohne zweiten Parser. Bestehender Git-/Lose-Datei-Aufruf bleibt API-kompatibel. Eine optionale neue Eingabevariante beziehungsweise ein expliziter zusätzlicher Aufruf desselben Extraktors genügt.

C besitzt Normalisierung und Validierung des D-Inventars sowie Caller und Datensicherung: Root/App/Build/Depot/Manifest/Manifesttransporthash, vollständige relative Dateiliste, Typen, Größen, Steam-SHA-1 und SHA-256. C stellt aus genau diesen tatsächlichen Originalbytes private unveränderliche Lesesicherungen bereit und verifiziert Größe und Hash beim Erzeugen. Erst nach korrekter vollständiger Bindung erhalten sie D-Provenienz. Keine Änderung oder Löschung fremder Originale. Ressourcengrenzen und freier Speicher bleiben vor Sicherung geprüft.

B soll an einer schmalen bestehenden Öffnungsgrenze die gehaltenen, bereits an diese Sicherung gebundenen lesbaren Dateiobjekte samt physischer Herkunft übernehmen. Keine erneute unabhängige Namensöffnung für die inventargebundene Variante. Das gilt für gewöhnliche Textdateien, physische VPK-Verzeichniscontainer und sämtliche tatsächlich gelesenen Begleitarchive. Der Reader bekommt exakt die Inventareinträge, nicht eine neue freie Root-Traversierung. Unterstützte Auswahl und übersprungene Dateien bleiben getrennt im vollständigen Inventarnachweis sichtbar.

Die tatsächliche VPK-Extraktion bleibt ausschließlich Bs bestehender Parser: Ressourcenbytes und deren SHA-256 getrennt von den verwendeten physischen Container-/Archivhashes ausgeben beziehungsweise nachvollziehbar referenzieren. App/Build/Depot/Manifest bleiben an dieselbe Eingabe gebunden. Unbekannter Pfad, Typ-/Größen-/Hashwechsel oder fehlendes Archiv führt vor belegter Ausgabe zu Fehler statt übernommener Manifestangabe. Das späte Öffnen eines vorgeprüften Dateinamens und bloße Vorher-/Nachher-Namenschecks erfüllen diesen Vertrag nicht.

## Vorgeschlagene enge Eigentumspfade

B prüft zunächst den kleinsten passenden API-Schnitt und meldet konkrete Signatur sowie Bedarf. Voraussichtlich genügen Änderungen innerhalb der bereits eigenen rust/crates/dbrain-sources/src/game_files.rs, game_files/anchored.rs und game_files/vpk.rs samt dortigen gezielten Tests. Keine Änderung der vier übrigen Parserdateien ohne begründeten Bedarf. Kein shared Cargo-/lib.rs-/CLI-Pfad durch B. C schreibt die B-Pfade nicht.

Nach kurzer bestätigter API-Abstimmung über Root schreibt ausschließlich B einen zusätzlichen geprüften Eigencommit in diesen zugewiesenen Parserpfaden. C besitzt später Registrierung, neuen engen Caller und gemeinsame Schnittstelle sowie Produktivimport. Gemeinsame endgültige D/B/C-Abnahme gegen tatsächliche SHAs vor Steam-Deploy; echte Depotdatenabnahme nach Download. Bestehende Race-, Begleitarchiv-, Pfad-/Typ-/Hardlink-/Symlink- und Trunkierungsfälle verwenden, nicht als echte Depotabnahme ausgeben.

Offene konkrete Antwort von B: API-Signatur und genaue nötige Pfade bestätigen oder eine noch kleinere gleichwertige gebundene Übergabe am vorhandenen Dateiobjekt vorschlagen. Keine parallele Wiederaufnahme alter Writer oder neuer Extraktion.
