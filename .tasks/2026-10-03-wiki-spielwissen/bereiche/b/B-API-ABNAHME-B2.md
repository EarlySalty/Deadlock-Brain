status: abgeschlossen
Datum: 2026-10-03

# B2: unabhängige Abnahme des Handle-Zusatzes

Frischer nativer Sol-high-Prüfer `a7884bae1d7c646da` hat den stabilen Zusatz ausschließlich lesend geprüft. Urteil: **Zusatz fertig J, Abweichungen keine, Fix nötig N.** Keine Unterdelegation, Compilerläufe, Sourceänderungen oder Gitmutationen. Dateieigentum vollständig abgegeben, keine eigenen Writer, Kinder oder gehaltenen Locks.

## Bestätigter Umfang

`game_files.rs:68` bis zum Ende des gebundenen Eingangs: öffentliche Signatur und Felder entsprechen dem bestätigten Vertrag. Konsumierte Files, normalisierte eindeutige Pfade, regulärer Typ, tatsächliche Größe und gestreamter SHA-256 werden geprüft. Root ist kein Öffnungsziel. Steam-SHA-1 wird als C3-geprüfte Bindung weitergeführt.

`game_files/vpk.rs:44` bis zum Ende des gemeinsamen Lesepfads: derselbe bestehende Parser verarbeitet im neuen Eingang ausschließlich gehaltene Container und Begleitarchive. Cursorpositionen werden explizit gesetzt; Bounds, CRC und Budgets bleiben erhalten. Physische Quellen und Ressourcenhashes sind getrennt. Bestehende Handle-Tests für Fehlerfälle und Herkunftsreferenzen nachgelesen, keine bestätigten Fehlerfunde.

## Nachweise

Beweisroot: `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/round3-proof/run-CrJzHxm4`.

Vorher-/Nachhermanifest bytegleich, alle zehn Quellen- und zehn Artefaktbindungen bestätigt. Manifest-SHA-256: `9de8425ac0436c939364845686addea82a59ef07a2b6181bf99faf1708fddd7c`.

Vorhandener vollständiger Selfcheck `boqz79t3x`, Exit 0: Formatprüfung, vier Compilerläufe, vier Clippy-Läufe mit `-D warnings`, 35 Extraktortests, fünf Validatoren und eine VPK-Wiederholung unter 128 MiB. 41 Ausführungen, 40 verschiedene Tests, Pflichtflags geprüft. Beide vollständigen Legacy-JSONL-Ausgaben und Inventare bytegleich zum akzeptierten Basisstand `48b6ce1`. Validatoren bestätigen GameTracking mit 237 Dokumenten und 331.287 Fakten sowie deadlock-data mit 423 Dokumenten und 345.076 Fakten. Ressourcenlogs enthalten monotone Dauer und Prozess-VmHWM.

Beginn-/Endhashes identisch:
- `game_files.rs`: `d984148eb1f0769c581c3cc487344077b7f37f41eade83eca81c923f1334e421`.
- `game_files/vpk.rs`: `2c2396022cdd3462b625d25f7a0539016c56b5901ce8ef29ba19f8195b0e23cd`.

TESTNACHWEIS[TW-1]: 41 passed, 0 ignored | Baseline: nicht gemessen rot

## Commitbindung

Eltern hat danach ausschließlich die zwei abgenommenen produktiven Dateien als zusätzlichen Eigencommit `0aa0d9ee2660e9c8ef6fb04a4295c842d1047bb6` gesichert. Parent `48b6ce1cf277ec4de4b47ac4a0898fbc223fe6b5` unverändert. Beide tatsächlichen Git-Blobs SHA-256-genau an obige unabhängigen Abnahmehashes gebunden. Featurebranch nach origin gepusht; `ls-remote` bestätigt exakt den Zusatzcommit. Keine gemeinsame Cargo-/Lock-/CLI-Datei im Commit, keine Akte oder Daten aufgenommen.

## Grenzen

C3 bleibt verantwortlich für vollständige private unveränderliche Eingabe, keine parallele Aliasnutzung und Verwerfen von Teiloutput bei `Err`. Keine finale D/B/C-Abnahme, echte Steam-Depotabnahme oder Deployfreigabe. Der Selfcheck lief unter bisherigen serde_json-Rlibs ohne `arbitrary_precision`; der passende Produktivfeaturegraph und Punkt-47-Zahlenwertbericht bleiben separat. Dieser Hinweis ist kein bestätigter API-Fehler und löst keine neue Sourcefixrunde aus.
