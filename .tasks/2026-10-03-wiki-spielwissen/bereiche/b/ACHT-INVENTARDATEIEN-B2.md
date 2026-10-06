status: erledigt
Datum: 2026-10-03
Stand: 2026-10-03T07:03:09Z

# Acht deadlock-data-Dateien ohne Vertragsdokument

431 Dateien bleiben im Inventar und als Originale erhalten; 423 ergeben Vertragsdokumente. Die Differenz besteht genau aus sechs PNG-Karten, LICENSE und README.md. Keine Datei wurde gelöscht oder aus dem Inventar entfernt.

## Konkrete Gründe

Sechs PNGs: Der vorhandene Textextraktor interpretiert keine Bildinhalte oder Kartenkoordinaten. Die Bilder bleiben Originalassets mit Inventareintrag. Aussagen aus den Karten wurden nicht als bewiesenes Mechanikwissen ausgegeben. Die Dateien mit Bindestrich und Unterstrich wurden nicht als Duplikate zusammengelegt; ihre Größen und Hashes unterscheiden sich.

LICENSE: Erweiterungslose MIT-Lizenz des Repositories, Copyright 2024 deadlock-wiki. Kein Spieldatendokument im aktuellen Extraktionsumfang. Der Inhalt bleibt Originaldatei und Lizenznachweis; Name, Attribution und gepinnter Lizenzlink sind bereits in options.json/provenance.json und den erzeugten Dokumenten erhalten. Die Repositorylizenz belegt keine pauschalen Rechte an Valve-Assets.

README.md: Markdown-Begleitdokument über das Repository und die automatische Erzeugung durch deadbot. `.md` gehört nicht zur vorhandenen Spieldaten-Texterweiterungsliste. README bleibt als Original und Herkunftserklärung erhalten, aber ohne Vertragsdokument. Seine Aussage zur Deadbot-Ableitung ist im Quellen- und Lizenzbericht berücksichtigt.

Der Parser verwendet für sämtliche nicht freigegebenen Erweiterungen die Sammelkategorie `binary_asset` und den Inventargrund „Binäres Asset; kein Beleg ausgeführter Spiellogik“. Für LICENSE und README ist das eine technische Fallbackkategorie, keine Behauptung, diese beiden Textdateien seien binär. Die tatsächlichen Gründe sind oben ausdrücklich benannt. Kein neuer Parser und keine heimliche Erweiterung oder Kürzung des Umfangs.

## Erhaltene Herkunft

Quelle `https://github.com/deadlock-wiki/deadlock-data`, feste Git-Revision `0d46cdecfccf77adec16aac01af6d30173e0ebb8`. Clientversion 6731 und Engine-Revision 11070267 stammen aus dem Generator-Snapshot; keine Steam-Build-/Depot-/Manifestkennung. Snapshot `git archive exact revision data README.md LICENSE`, 431 Dateien und 34.530.366 Bytes. Frühere vollständige Git-Blob-Prüfung aller 431 Dateien: null Abweichungen, 04:57:43 UTC, dokumentiert in DATENSAMMLUNG.md/provenance.json.

Root außerhalb Git:
`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/deadlock-data-0d46cdecfccf77adec16aac01af6d30173e0ebb8/files/`.
Options- und Herkunftsdateien liegen daneben; das vollständige Extraktionsinventar liegt in round2-proof. Die acht Inventareinträge enthalten relativen Pfad, Bytezahl, Kategorie, Disposition und Grund. Ihr `original_sha256` ist dort null. Die nachfolgende lesende Nachmessung hält deshalb die exakten Originalhashes ausdrücklich fest, ohne eine bereits vorhandene Inventarhashbindung vorzutäuschen.

| Relativer Originalpfad | Bytes | SHA-256 |
| --- | ---: | --- |
| LICENSE | 1.070 | 054e99dd239d7bea866f0974e58d92409937425139a46f15deebb0a8393801f5 |
| README.md | 273 | dc3f54b9a96f00c16f6470470244c100a9d56c303e40ea832fc964b7d1d3028f |
| data/assets/crate-map.png | 1.200.165 | 6245e0f0a8c894a4ba9469a7ae078464d2511b836b5e6efde85d7eb8861a3525 |
| data/assets/crate_map.png | 1.196.132 | cb67bdebc6f8f6d3dcccfb3d3e7bbba68e2171df80479ea24b925f6fe27428f4 |
| data/assets/golden-statues-map.png | 1.187.278 | 3947599e0fcfab52fcdb4072cc9cc62170abefa6e34497b285e2557ea9998f2b |
| data/assets/golden_statues_map.png | 1.186.609 | c08abe25f912e72ad9548ea508f018204993a6ad232d0b0e80a830f07203c8da |
| data/assets/shops-map.png | 1.211.238 | f714c492868d4472e0c2c2056a665167f42bcc19573426b12fd2f6b1e836470d |
| data/assets/shops_map.png | 1.217.083 | 8f5c9c33d399236ae4985b71ee412c205109edcd61426d1fc222b4f56d5c46ea |

Die Rechte-/Veröffentlichungsgrenze bleibt `redistribution_allowed=false`. Die sechs Bildinhalte sind eine ausdrückliche Extraktionslücke; LICENSE und README sind erhaltene Begleit-/Herkunftsdokumente außerhalb des JSONL-Umfangs.
