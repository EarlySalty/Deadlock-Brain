status: aktiv
Datum: 2026-10-03
Stand: 2026-10-03T10:19:27Z

# Unabhängige B2-Eigenabnahme

Frischer nativer lesender Prüfer `a80e83655289d465f`, geerbtes GPT 6.1 Sol high. Elternsession übernimmt den strukturierten Rückbericht. Keine neuen Compiler, Prüfwrapper, Quelledits oder Gitmutationen durch den Prüfer.

Urteil: Abnahme des B-Git-Export-/Parser-Endstands **J**, Fix nötig **N**. Keine bestätigten neuen Funde. Gesamtauftrag einschließlich D-Depotdaten und C3-Integration damit nicht abgenommen.

## Stabile Prüfung

Elfdatei-Freeze einschließlich Harness-Cargo zu Beginn und am Ende selbst nachgerechnet, unverändert: `3cef2c79fd36c9462948f90ceb4fa28b6b82b6ad1102769a8b8812a66132055c`. Separates Zehndatei-Wrappermanifest identisch vor/nach und korrekt, SHA-256 `80fd9cc6e812a570934d7ecc1bae765d4f5719a30e9452c3d716febd46a2a386`. Alle Artefaktprüfsummen gültig. HEAD bei Abnahme `2734c2da4e814ff79953e8e825275b0216a6af16`.

Tatsächliches `RUN_EXIT 0`, fmt, vier Compiler und vier Clippy-Modi mit `-D warnings` bestätigt. Keine Lintunterdrückung. VPK-Bounds vor Payloadallokation, gehaltene Dateideskriptoren, verankerte Begleitarchive, Ausschluss der v2-Prüfsummenabschnitte und fallible Reservierung unter aktuellem Endhash erneut bestätigt, `rust/crates/dbrain-sources/src/game_files/vpk.rs:193-258`.

TESTNACHWEIS[TW-1]: 33 passed, 0 ignored | Baseline: unbekannt rot

33 Ausführungen umfassen 32 verschiedene Tests: 27 Extraktortests und fünf Validatortests, zusätzlich Wiederholung einer VPK-Probe unter 128 MiB. Keine neue Testausführung durch Abnahmeprüfer.

## Empirische Datenbelege

Beide JSONL-Dateien vollständig lesend gegen aktuelle Originalbytes geprüft: null Rohtext- oder Hashabweichungen. Beide Rohbestände stimmen zusätzlich mit gepinnten Git-Blob-Hashes überein.

| Bestand | Dokumente | Fakten | Numerische Quelllexeme | Größte Zeile, Bytes ohne LF |
| --- | ---: | ---: | ---: | ---: |
| GameTracking | 237 | 331.287 | 55.504 | 117.157.259 |
| deadlock-data | 423 | 345.076 | 13.367 | 11.107.243 |

Nachweisroot `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/round3-proof/run-5gzlJFjs/`.

GameTracking ist der dokumentierte ausgewählte Bestand: 237 von 456 Dateien im dokumentierten Texterweiterungssatz. Das ist kein Beleg für vollständige Spielabdeckung. Prüfer hat keine fehlenden VData/VPulse oder entsprechenden Citadel-Skript-/Resource-Dateien festgestellt. Auswahlnennner und vollständig geprüfter lokaler Inventarnenner sind getrennt zu nennen.

Die acht Nichtdokumentdateien samt Originalhashes unabhängig bestätigt. Sechs PNGs ohne Bildauswertung, LICENSE/README als Begleitbelege erhalten. Rohtexte und Parserstatus auch ohne strukturierte Fakten erhalten. Keine behauptete ausgeführte Spiellogik, Referenzauflösung oder beleglose Einheiten. Rechteprüfung bei C3 offen, `redistribution_allowed=false`.

## Separat offene D-Depotabnahme

Bestehender Validator verlangt Git-Revisionen, leere Steam-Felder und lose Originaldateien, `pruefharness-b/src/bin/validate-jsonl.rs:255-357`. Er nimmt echte Steam-/VPK-Provenienz noch nicht ab. Benötigt werden reale D-Rohdaten und unmittelbare Manifest-/Dateiinventarbindung einschließlich physischer VPK-Begleitdateien und daraus gelesener Ressourcenbytes. Historische D-Prüfstände ersetzen diesen Nachweis nicht. Keine C3-Integrations- oder Deploymentfreigabe.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 0/0 geprüft

## Unabhängig bestätigte Commitbindung

Derselbe Abnahmeprüfer hat nach dem Eigencommit ausschließlich lesend dessen Parent, genauen Siebendatei-Umfang und SHA-256 aller sieben Git-Blobs gegen das bereits abgenommene `source-before.sha256` bestätigt. Commit `48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5`, Parent `2734c2da4e814ff79953e8e825275b0216a6af16`. Sein Urteil gilt ausdrücklich für exakt diesen Parser-Eigencommit: Abnahme J, Fix nötig N. Keine neue technische Prüfrunde oder Mutation.

Eigencommit an aktiven C3 übergeben, Featurebranch remote gesichert. Nur diesen Commit übernehmen, niemals den historischen Branch mergen. Nächster Schritt: gemeinsame D/B-Abnahme und echte Depotextraktion nach belegter Rohdatenübergabe. Keine Gesamt-, Merge- oder Live-Freigabe durch diese Bereichsabnahme.
