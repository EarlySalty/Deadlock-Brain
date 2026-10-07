status: abgeschlossen
Datum: 2026-10-03

# B2: genaue Zahlenwertprüfung, Punkt 47

## Urteil

**Kein mathematischer Zahlenwertverlust: 0 von 121.556 geprüften Zahlen.** Originallexem-, Pointer-/Span-, Qualifier- und Abweichungen vom bestehenden B-Parservertrag jeweils 0. Beide erhaltenen vollständigen Git-Exporte wurden geprüft, nicht neu erzeugt oder überschrieben.

**JSON-Typenwirkung separat bestätigt:** 68.871 numerische Quellenwerte sind im Export absichtlich String, davon 13.367 aus JSON und 55.504 aus KV3. Die gespeicherten Ziffern und Exponenten sind genau wertgleich. Das beweist keine JSON-Typgleichheit: im JSON-Original steht Number, im entsprechenden Fakt String. Weitere 52.685 Zahlen sind i64-Number. Dieses Verhalten folgt dem bestehenden `game_files/json_text.rs:47` bis `:60` und bleibt auch mit `arbitrary_precision` bestehen.

Kein Präzisionsfix nötig. C3 soll den bestehenden Zahlenstring-Vertrag im Reader ausdrücklich berücksichtigen und nicht still in `f64` umwandeln. Falls der gemeinsame Vertrag zwingend Number fordert, ist `json_text::number` der kleinste vorhandene Änderungspunkt. Das wäre eine getrennte Typvertragsentscheidung, kein hier bestätigter Präzisionsdefekt. **Keine automatische Sourcefixrunde ausgeführt.**

## Tatsächlicher Umfang

660 Dokumente, 676.363 exportierte Fakten, 506.412.456 JSONL-Bytes. Alle 660 Originaldatei-/Content-/Hashbindungen geprüft. Insgesamt 582.514 Originalblätter geprüft, darunter 121.556 Zahlen und 460.958 nichtnumerische Blätter. Weitere 93.849 Fakten ohne numerische Quellrepräsentation auf unerwartete numerische Werte oder Marker geprüft.

| Quelle | Dokumente / Fakten | Geprüfte Zahlen | Gespeicherte Typen |
| --- | ---: | ---: | --- |
| GameTracking, Git `4c6431ccdb816d2911bbbaa4335169cc34140386` | 237 / 331.287 | 87.371 in 134 KV3-Dokumenten | 55.504 Dezimalstrings, 31.867 i64-Number |
| deadlock-data, Git `0d46cdecfccf77adec16aac01af6d30173e0ebb8` | 423 / 345.076 | 34.185 in 158 JSON-Dokumenten | 13.365 Dezimalstrings, 2 Exponentstrings, 20.818 i64-Number |

237 beziehungsweise 431 inventarisierte Dateien. Acht deadlock-data-Dateien sind keine exportierten Dokumente; deren frühere Erklärung bleibt in `ACHT-INVENTARDATEIEN-B2.md`. Keine Behauptung vollständiger Zahlenextraktion aus KV1-Zeichenketten, rein erhaltenem Text, diesen acht Dateien oder vollständiger Spiellogik.

Der echte Bestand enthält keine u64-only-, großen Integer- oder ganzzahligen `-0`-Fälle. Neun separate Laufzeitproben prüfen diese Grenzfamilien und Serialisierungsübergänge; sie ersetzen keine echten Daten dieser Familien.

## Methode und Beispiele

Unveränderte vorhandene Module `budget.rs`, `json_text.rs` und `kv3.rs` im lokalen Rust-Messharness eingebunden. Keine Parserkopie. JSON zusätzlich mit `RawValue` lexemgenau und `Value` mit `arbitrary_precision` am Originalpointer geprüft. Mathematische Gleichheit über Vorzeichen, Ziffern und Dezimalexponent, keine Floats, Epsilon-Toleranz oder Number-PartialEq. Bei allen tatsächlichen Zahlen erfolgreich. B-Spans sind Dateipfad mit Pointer, keine Byteoffsets.

Beispiele aus deadlock-data:
- `data/json/ability-cards.json`, Pointer `/hero_inferno/1/Info1/Main/Props/0/Value`: Original Number `40.0`, gespeicherter String `"40.0"`, mathematisch exakt gleich.
- `data/json/convars.json`, Pointer `/lb_csm_receiver_plane_depth_bias/value`: Original Number `1.5e-05`, gespeicherter String `"1.5e-05"`, mathematisch exakt gleich.

Alle Beispiele mit Original-/Inhaltshashes und Revisionen im erhaltenen Bericht. `abweichungen.jsonl` ist leer. Beide geprüften Exporte sind byteidentisch zur erhaltenen Basis `run-5gzlJFjs`.

## Tatsächlicher Feature- und Laufbeleg

Alte B-Rlib: serde_json **1.0.150**, `default/raw_value/std`, ohne `arbitrary_precision`; tatsächliche Herkunft anhand `.d`, Fingerprint und eingebetteter Registrypfade geprüft. Dieser alte Selfcheck beweist nicht den späteren Produktivfeaturegraph.

Messharness tatsächlich gelinkt: serde_json **1.0.151**, `arbitrary_precision/default/raw_value/std`. Rlib `libserde_json-1975c68cca4aef88.rlib`, SHA-256 `441e2b9d585bccb71220c3cfabb08038fe633d166808a4ade688de1a5e6fad26`. Tatsächliche `--extern`-Bindung, Fingerprint, `.d`, Registrypfade und neun Laufzeitproben belegt, keine bloße Cargo-Textannahme. Gemeinsame produktive Manifeste und der bisherige B-Harness nicht geändert. Endgültige gemeinsame C3-Feature-/Readerprüfung bleibt offen.

Erster Task `bxrpl3ppr` endete nach erfolgreichem Build mit lokalem Messharness-Clippyfehler und Exit 101. Erhalten in `lauf.log`, keine produktive Parseränderung. Reguläre Fortsetzung `bktyqd4i6` endete um **13:32:10 UTC mit Exit 0**, Gesamtdauer einschließlich Lockwartezeit 1.733 Sekunden. Build, Clippy, vier Messharness-Tests, vollständige Messung sowie Quellen-/alte Artefaktbindungen grün. Messung selbst 73,660357508 Sekunden, Peak 585.780 KiB Prozess-VmHWM.

TESTNACHWEIS[TW-1]: 4 passed, 0 ignored | Baseline: nicht gemessen rot

Beide Locks bis Ende gehalten und regulär freigegeben. Eltern bestätigt echte Endmarker und danach Abwesenheit sämtlicher benannter eigener Wrapper-/Parent-/flock-/Compiler-/Mess-PIDs. Kein Erfolg allein aus PID-Abwesenheit und keine globale Sperrfreiheit behauptet. Alle elf Quell-/Harnessbindungen aktuell unverändert, sieben Messartefaktbindungen stimmen.

## Beweisort und Grenze

`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/punkt47-zahlenwerte-20261003-solhigh/`:
`rueckbericht.json`, `ergebnis.json`, `dokumente.jsonl` mit 660 Einzelbindungen, leeres `abweichungen.jsonl`, `herkunft.json`, beide Logs und `messartefakte.sha256`.

Bestehender nativer Sol-high-Worker `a3ecb4d16e0040571` abgeschlossen. Kein produktiver Source-/Git-/Download-/DB-/Secret- oder Fremdprozesseingriff. Originale und alte Ausgaben erhalten. API-Eigencommit `0aa0d9e` und Basis `48b6ce1` unverändert. Keine finale Steam-, PostgreSQL-, Reader-, D/B/C- oder Deployabnahme aus dieser Messung ableiten.
