status: aktiv
Datum: 2026-10-03

# Gemeinsame Readerabnahme und Coverageplanung

Punkt 43 übernommen: C3 baut den engen Caller-/Bytebindungsrest im bestehenden Integrationspfad. D behält Writer/Store. B liefert fachliche Readerabnahme und spätere tatsächliche Depotextraktion. Abgenommener Parser-Eigencommit `48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5` bleibt unverändert. Noch kein Auftrag für einen B-API-Eingriff und kein paralleler Writer.

## Vor der gemeinsamen Abnahme festzuhalten

Finale D-, B- und C3-SHAs samt tatsächlichen Dateihashes und Quellfreeze. C3 benennt seinen realen Extraktionscaller und den Byteübergabepfad; D liefert endgültigen Manifest-/Inventarvertrag. Vorläufige Arbeitsstände aus D-B-BEREITSCHAFT-B2.md sind keine finale Freigabe.

Falls C3 einen kleinen B-API-Eingriff braucht, zuerst schriftlich festhalten: genaue API-Signatur und Eingabe-/Ausgabevertrag, konkrete B-Dateipfade, Besitz-/Lebensdauer der tatsächlich geprüften Bytes beziehungsweise Deskriptoren, Fehler- und Provenienzverhalten sowie passende Prüffälle. Danach ausschließlich B in diesen Pfaden, eigener zusätzlicher Commit mit Selbstprüfung und unabhängiger Abnahme. C3 schreibt diese B-Pfade nicht parallel. Kein Amend des abgenommenen Eigencommits und kein zweiter Parser.

## Abnahmematrix vor Steam-Deploy

| Prüfgruppe | Beobachtbarer Sollzustand |
| --- | --- |
| Identität und Inventar | Root/App/Build/Depot/Manifest und Manifesttransport-/Inventarbeleg stimmen mit der endgültigen D-Übergabe. Optionen allein sind kein Herkunftsnachweis. Dateiliste mit relativen Pfaden, Typ, Größe, SHA-1/SHA-256 ist der genaue Nenner. |
| Dateiwechsel und Fremddateien | Zusätzliche unterstützte Texte und Binärdateien, fehlende Einträge, Typ-/Linkwechsel und Austausch zwischen Prüfung und Nutzung ergeben keine bestätigte Manifestherkunft. Tatsächlich verwendete Bytes nachweisen, nicht separat erneut geöffnete Namen. Fremddateien niemals löschen. |
| VPK | Verzeichniscontainer und tatsächlich genutzte nummerierte Begleitarchive an D-Inventar binden. Physische Containerhashes getrennt von Ressourcenhashes, Bounds/CRC sind kein Ersatz für D-SHA-Bindung. Austausch und Trunkierung sowie Änderung zwischen Vorprüfung und Ressourcenlesen müssen im tatsächlichen Readerpfad erkennbar sein. |
| Ausgabe und Import | Vollständiger Rohtext, Original-/Inhaltshash, Herkunftsreferenz, Faktwerte und Zahlenlexeme bleiben erhalten. Keine Kürzung großer Zeilen oder unbemerkte Herkunftsübernahme. Steam-/VPK-Prüfung am bestehenden Pfad ergänzen; Git-/Lose-Datei-Validator allein reicht nicht. |
| Grenzen und Bericht | Erkennbar unterschiedliche physische Dateien, virtuelle VPK-Ressourcen, Textdokumente, strukturierte Werte, nur erhaltene Texte und Inventar-/Lückenfälle zählen. Prozess-RSS und monotone Dauer je tatsächlichem Lauf messen, keine globale Host-RAM-Garantie aus VmHWM ableiten. |

Vorhandene Bausteine: D `game_download/tests.rs:225-277` und B `game_files/vpk.rs:410-432,481-489`, siehe Bereitschaftsbericht. Noch keine Testausführung auf künftigem Callerstand. Synthetische Wechsel-/Fehlerfälle prüfen den Readervertrag und sind ausdrücklich keine echte Depotabnahme. Compilerprüfungen unter beiden Hostlocks, frische NonZombie-Probe vor jedem Compiler, höchstens zwei Jobs.

Gemeinsame endgültige D/B/C3-Prüfung vor notwendigem Steam-Deploy. Erst danach regulärer D-Download über bestehenden Zugang und tatsächliche Extraktion/Validierung belegter Rohdaten. Kein neuer Login oder Lizenzrequest.

## Tatsächlich vorhandene strukturierte Formatauswertung

Belegter Dispatcher im unveränderten Parser: `rust/crates/dbrain-sources/src/game_files.rs:481-540`, Klassifikation `:565-638`, Textdekodierung `:655-683`. Dateiendung ist kein Beweis erfolgreicher Strukturierung; der tatsächliche Parserstatus und Faktumfang entscheiden.

| Klasse | Gegenwärtige Auswertung |
| --- | --- |
| JSON `.json` | JSON-Werte und Blattpfade, exakte Zahlenlexeme. Bei Parserfehler Rohtext/Status erhalten, keine erfundenen strukturierten Fakten. |
| Textuelles KV3 | `.kv3` oder KV3-Textheader in einem zugelassenen anderen Textformat. Generische KV3-Blattwerte, keine ausgeführte Ressourcen-/Gameplay-Semantik. `.json` bleibt im JSON-Zweig. Binäres KV3 wird nicht dekodiert. |
| KV1 | `.txt`, `.kv`, `.res`, `.vdf`, `.vdata`, `.vdata_inc`, `.gi`, `.kv1`, sofern nicht vorher KV3-Text erkannt. Schlüsselwerte, Wiederholungen und Bedingungen erhalten; Bedingungen und Referenzen werden nicht ausgeführt/aufgelöst. Lokalisierungstexte können darunterfallen. |
| Weitere zugelassene Texte | `.vpulse`, `.inf`, `.cfg`, `.ini`, `.vmap`, `.xml`, `.csv`, `.vjs`, `.lua`, `.nut` sind ohne erkannten KV3-Textheader nur Rohtext mit `text_preserved_without_semantic_parser`, keine strukturierten Fakten. Insbesondere keine CSV-/XML-Auswertung, Script-Ausführung oder VPulse-Graphauflösung. Auch Fehler der strukturierten Parser bleiben Rohtext mit Status, nicht erfolgreich strukturierter Inhalt. |
| Zunächst nur Inventar | `.vdata_c`, `.vmap_c`, `.vmdl_c`, `.vmat_c`, `.vtex_c`, `.vsnd_c`, `.vpcf_c`, `.dll`, `.so`, `.exe`, Bilder und nicht zugelassene Endungen. Keine Binär-/Bild-/Koordinaten-/Shader-/Spiellogikauswertung. Der Fallbackname binary_asset ist eine technische Kategorie, kein Beweis, dass jede solche Datei wirklich binär ist. |

VPK ist eine Containerstufe, kein zusätzlicher semantischer Parser: `*_dir.vpk` mit VPK-Version1/2 wird geöffnet; eingebettete und nummerierte Ressourcen werden nach denselben erlaubten Endungen/Textparsern behandelt. Nummerierte Begleitarchive sind physische Dateien; virtuelle Ressourcen separat zählen. Andere `.vpk`-Dateien bleiben zunächst Inventar. Unterstützte Textdekodierung: UTF-8 sowie UTF-16LE/BE mit BOM; ungültige Kodierung/NUL ergeben Lücken statt erfundener Texte.

Die Klasse items/heroes/abilities aus Pfadnamen beweist keine aufgelöste Spielbedeutung. Extrahierte Werte haben unbelegte Einheiten und Gameplaybindung weiterhin als unbekannt/uninterpretiert. Textparser beweisen weder vollständige Spiellogik noch ausgeführte Mechaniken, vollständige Referenzgraphen oder sämtliche Ressourcenformate.

## Tatsächlicher Datenbericht nach Rohdatenübergabe

Je endgültigem Depot getrennt berichten: D-Manifest-/physischer Dateinenner, verifizierte physische Dateien, VPK-Container/Begleitarchive, virtuelle Ressourcen, dokumentierte Texte, erfolgreiche strukturierte Parser und Faktanzahl, Rohtexte ohne Strukturparser sowie Inventar-/Ausschluss-/Lückenfälle mit konkreten Gründen. Erfolgreiche Parser mit null Werten getrennt von Parserfehlern und reinem Rohtext behandeln. Ausschlüsse und nicht traversierte Bereiche gegen den D-Nenner sichtbar halten, kein stiller Vollständigkeitsnenner aus nur ausgegebenen Dokumenten.

Die bekannten Git-Exports sind eigene Prüfbestände, keine Steam-Coverage: GameTracking ist eine dokumentierte Auswahl von 237 aus 456 Dateien im Texterweiterungssatz mit 237 lokalen Dokumenten; deadlock-data umfasst 431 Inventardateien, 423 Dokumente und acht erhaltene Nichtdokumentdateien. Keine tatsächlichen Steam-Formatmengen oder Depot-/Manifesthashes erfinden. Noch keine realen D-Depots übergeben.

Nächster fachlicher Schritt: C3s konkreten finalen Caller-/Bytevertrag gegen diese Matrix abnehmen; falls B-API nötig, zuerst genaue Vertrags-/Pfadzuweisung erhalten.
