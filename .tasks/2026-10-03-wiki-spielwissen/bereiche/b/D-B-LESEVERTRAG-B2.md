status: aktiv
Datum: 2026-10-03
Prüfstand: 2026-10-03T06:24:36Z

# D-Schreibpfad und B-Lesepfad

Unabhängiger nativer Prüfer `a2a6220ddd7f0b64c`, GPT 6.1 Sol high. B2 hält dessen Rückbericht fest. Keine Quellenänderung, Compiler, Tests, Downloads oder Datenbankzugriffe. Statische Vertragsprüfung, keine Laufzeitabnahme.

Urteil: Die vorhandenen D- und B-Schnittstellen lassen sich über C2 verbinden. Das D-Inventar ist keine fertige B-Optionsdatei. Der aktuelle B-Prüfvalidator deckt nur die beiden Git-Exports ab und nimmt echte Steam-/VPK-Daten noch nicht korrekt ab.

## Anbindung durch C2

D verwendet `AuthDownloadDeadlockGameHandler`, Task `AUTH_DOWNLOAD_DEADLOCK_GAME` mit Eingabe `{}` und bestehender Verbindung, Laneparallelität 1. Vorgesehener Root je Depot:
`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/d/steam/<app_id>/<build_id>/<depot_id>/<manifest_id>/`.

D-Inventare enthalten App-ID, Build-/Manifest-ID als Strings, Depot-ID, Manifesttransport-SHA-256, absoluten Root, UTC-Abrufzeit, erwartete Bytes und relative Dateipfade. Reguläre Dateien erhalten Größe, SHA-1 und SHA-256; Größe und Steam-SHA-1 werden vor atomarer Veröffentlichung ohne Überschreiben geprüft. Verzeichnisse haben keine Dateihashes.

B-Aufruf: `extract_game_files(&GameFileOptions, &mut impl Write) -> io::Result<GameFileInventory>`. Vorhandener Harness lädt Optionen mit `serde_json::from_reader`; JSONL und Extraktionsinventar verwenden `create_new`.

C2 erzeugt je Depot `GameFileOptions`: Root, App-ID, Build, Manifest, Depot und UTC-Zeit aus dem belegten D-Inventar; `source_revision=None`, `provenance.source_layout="resource_paths"`. Manifesttransporthash und Inventarreferenz erhalten. `source_id`, Attribution, Lizenzangaben und `max_file_bytes` ergänzen; Sprache ohne Dateibeleg `und`. Einzelnen Depotroot verwenden, nicht den gesamten D-Bestand. Keine Git-Revision oder Clientversion als Steam-Build ausgeben.

B verwendet bevorzugt `manifest:<id>`, danach `build:<id>` als Revision. Bei VPK-Ressourcen ist `original_sha256` der Hash der extrahierten Ressourcenbytes. Er entspricht nicht dem D-Hash der physischen VPK-Datei. Beide Herkunftsebenen müssen erhalten bleiben.

## Konkrete offene Arbeit

1. B-Prüfvalidator `pruefharness-b/src/bin/validate-jsonl.rs:202-206,265-268,304`: verlangt sekundengenaue UTC-Zeit und Git-Revision, verwirft Build-/Manifest-/Depotfelder und sucht VPK-Ressourcen als lose Originale. D erzeugt Millisekundenzeiten und echte Steam-Felder. Vorschlag: vorhandenen Validator gezielt für belegte Steam- und VPK-Herkunft erweitern, nachdem die laufende eingefrorene Prüfkette abgeschlossen ist. Grüne Git-Exportprüfungen sind keine Depotabnahme.
2. D `game_download/store.rs:100-105`: `serde_json::to_vec` reserviert den vollständigen Inventarpuffer vor der 256-MiB-Prüfung. B2 hat die Stelle nachgelesen und bestätigt diese Reihenfolge. Vorschlag an D/C2: vorhandenen JSON-Schreibpfad während der Serialisierung begrenzen; B2 ändert keine D-Dateien. Kein gemessener Speicherfehler und keine Gesamt-Sicherheitsbewertung.
3. Zusätzliche Datenabnahme fehlt: D hat Appgrant 1422450 aus Task 4931460 bestätigt, aber noch keinen vollständigen Task-/Depot-/Manifest-/Rohdatenbeleg übergeben. Erst danach vollständig zusätzlich extrahieren und validieren.

## Abdeckungs- und Ressourcengrenzen

B verarbeitet JSON, KV1, textuelles KV3 und weitere Texte. Kompilierte Ressourcen, Bibliotheken und andere Binärassets bleiben Inventar. Daraus folgen weder vollständige binäre Spiellogik noch bewiesene ausgeführte Mechaniken oder vollständige Gameplay-Verknüpfungen.

Bestehende B-Optionen: 32 MiB je Datei; größere Texte als Lücke. VPK 1/2: maximal 64 MiB Baum; nummerierte Begleitarchive müssen vorhanden sein. Parsertiefe 128, rechnerisches Allokationsbudget 512 MiB je Verarbeitungsschritt, keine globale RSS-Grenze. Keine globale Depot-/Dokument-/Ausgabe-/Verzeichnistiefengrenze; Inventar und Lücken wachsen im Lauf. Validatorgrenze 256 MiB je JSONL-Zeile, vom Extraktor nicht als Ausgabegrenze erzwungen.

D sieht 96 GiB Download, 192 GiB Bestand, 200.000 Manifesteinträge und 32 Depots vor. Diese Limits beweisen keine vollständige B-Verarbeitbarkeit. Taskgesamtzeit und reale Dauer sind nicht gemessen.

C2 benötigt echte Rohdaten, Gesamt-/Depotinventare einschließlich VPK-Begleitarchiven, Hashvergleich, Abdeckungsnenner, größte JSONL-Zeile und gemessene Ressourcen. `redistribution_allowed=false`; C2 prüft interne Importzulässigkeit, Veröffentlichung bleibt gesperrt.

## Hashbindung

Paketfingerabdruck: SHA-256 über lexikografisch sortierte UTF-8-Zeilen `<Datei-SHA256>  <absoluter Pfad>\n`.

B-Hauptmodul und sechs Untermodule: `a1e8e367f49d5f767b89a06286a04ca5fff2b4efa50dbc401a6a86b2dfbec487`.
D-Hauptmodul und vier Untermodule: `dd24307246ec156b9e9c7fe2ccd3069810a72c8a5fd4a0785a02bcedf67741b6`.
B-Hauptmodul: `8ca3b514ac2fd4fc0301193114f381560db1322de851ffcb2f7c842f23e45fac`.
D-Hauptmodul: `512cefc1373b4a2ca281af91ca350c0b2c6f11dcef704b8c76b0106ef78f6c19`.
B-Prüfvalidator: `d582549e53fac75e7bb96715eab8c34bd434ad77730ee0e5081d0aa51ee17966`.

Spätere betroffene Änderungen erfordern neue Prüfung. Dieser Bericht ist keine finale Compiler-/Download-/Integrationsabnahme.

## Verbindliche Inventarbindung aus Punkt 31

AN_BEREICHE.md bis Punkt 31 am 03.10.2026 um 07:03:09 UTC gelesen. D behebt seine bestätigten Logging-, Ressourcen- und Inventarbindungsfunde im eigenen Steam-Worktree. Kein D-Writer besitzt B-Parserdateien. Die oben gelesenen D-Hashes bleiben historische Prüfbindung und keine Abnahme des neuen D-Fixstands.

Beim tatsächlichen B-Lese-/C2-Importübergang muss die exakte freigegebene Manifest-/Dateiliste mit relativen Pfaden, Typ, Größe und echten Dateihashes erneut an die verwendeten Originalbytes gebunden sein. Eine früher erfolgreiche D-Downloadprüfung oder nur die Root-/Build-/Manifestangabe in Optionen reicht nicht. Nachträglich hinzugekommene, ausgetauschte oder sonst nicht belegte Dateien erhalten keine Manifestprovenienz und dürfen nicht als bestätigte Depotdokumente importiert werden. Fremddateien nicht löschen; die Abweichung als Grenze melden.

Für VPK bleibt die zweistufige Bindung nötig: verifizierte physische Container-/Begleitdateien aus D-Inventar und daraus tatsächlich gelesene Ressourcenbytes mit eigenem Ressourcenhash. Keine Gleichsetzung der beiden Hashes. C2 und B2 nehmen diese Bindung unmittelbar an der echten Übergabestrecke gemeinsam ab. Ohne echte D-Rohdaten und stabilen D-Fixstand bleibt dieser Laufzeitnachweis offen; der grüne Git-Exportlauf erfüllt ihn nicht.

## Fortsetzung mit aktivem C3

Punkt 41 gelesen: C2 ist regulär beendet; C3 übernimmt dieselbe gesicherte Integration und beide Deploys. Die historischen C2-/D-Quellprüfstände oben bleiben historische Belege. Aktuelle gemeinsame Lese-/Importabnahme ist zwischen D, B2 und C3 durchzuführen, ohne neuen Downloadpfad im B-Parser.

D-Übergabe, Stand 09:50:49 UTC, zentral gelesen am 03.10.2026 gegen 10:10 UTC: eigener D-Endprüfer noch aktiv, unabhängige D-Fixstandabnahme und Eigencommit offen, null Spieldateien, kein Live-Task vor C3-Deploy. Deshalb noch keine belegte Rohdatenübergabe oder echte B-Depotextraktion möglich. Nicht aufgrund veralteter D-Hashes oder bestätigtem Appgrant anfangen. Nach D-/C3-Übergabe exakt verwendete Dateien gegen Manifestinventar binden und vorhandenen Extraktions-/Validierungspfad gezielt für echte Provenienz verwenden; keine zweite Parserimplementierung.

## Tatsächlich bestätigter Integrationsrest

Frischer lesender Sol-high-Vorcheck abgeschlossen: Bereitschaft N, genaue aktuelle Pfade/Hashes in `D-B-BEREITSCHAFT-B2.md`, Elternprobe10:48:38UTC. D bindet Dateien beim Publish an sein Manifestinventar. B gleicht späteren Rootbestand nicht mit D-Dateiliste ab, sondern übernimmt Optionsprovenienz. In C3 noch kein produktiver Extraktionscaller oder B-Modulregistrierung. Eng fehlend ist C3s Caller mit inventargebundener Byteübergabe, einschließlich physischer VPK-Container und wirklich verwendeter Begleitarchivbytes. Vorprüfung plus spätere separate Namensöffnung reicht nicht.

Dateieigentum: C3 `dbrain-sources/src/lib.rs`, CLI/Import `src/bin/brain-knowledge-import.rs`/`knowledge_import` und gemeinsame Schnittstellen; D Downloader/Store/Inventar; B sieben unveränderte Parserdateien und lokaler Harness. B-Eingriffe nur nach konkreter Zusatzpfadzuweisung. Bestehende D-/B-Wechsel-/Fremddatei-/VPK-Testbausteine wiederverwenden, keine neue Parserimplementierung. Gemeinsame Abnahme auf endgültigen D/B-SHAs vor Steam-Deploy; echte Depotextraktion erst danach mit belegten Rohdaten. Keine finale SHA-/Deployfreigabe aus diesem Vorcheck.

