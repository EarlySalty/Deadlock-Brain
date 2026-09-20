# Eigener Rust-Rohdatencheck statt weiterer Cloud-Worker

Stand: 16.09.2026. Umsetzung und Quellenpruefung direkt durch ChatGPT ueber codex-mcp. Keine neuen T3-Threads, Claude-/Opus-Subagenten oder Ersatzanbieter gestartet. Der bestehende Thread 803d3e94 bleibt bei seinem laufenden Paket 0+A. Eigener Worktree: `/home/nathanael/repos/wt/brain-raw-mechanic-audit`, Branch `feat/reasoner-raw-mechanic-audit`, Basis `32a70d1`.

## Geliefert

Das vorhandene Rust-Example `ability_coverage` besitzt einen zusaetzlichen DB-/KI-/Netzwerk-freien Modus `raw`. Es liest unveraenderte lokale Assets, inventarisiert die originalen Property-Werte, Bedingungsfelder, Usage-Flags und Skalierungsfunktionen mit JSON-Pfad und prueft die Waffen-/Faeigkeitsreferenzen jedes Helden. Die vorhandene Frozen-Auswertung bleibt erhalten. Kein zweiter Reasoner, keine produktive Score-, Planner- oder Publish-Aenderung.

Quellbytes bekommen SHA256. Fehlende und mehrdeutige Referenzen werden als offen ausgegeben, nie durch die erste passende Definition ersetzt. Negative endliche Koeffizienten bleiben signiert. Nichtnumerische Literale, unbekannte Skalierungsformen und Einheiten werden nicht zu Null oder einer geratenen Mechanik gemacht. Namen/IDs sind Berichtsdaten, keine Zweck- oder Bewertungsregeln. Existierende Eingabe-/Ausgabedateien werden nicht ueberschrieben. Der kleine Ergebnisbericht ist `RAW-MECHANICS-SUMMARY.json`; der vollstaendige Rohbericht bleibt im ignorierten Cargo-target-Verzeichnis.

## Tatsachlich gemessen

| Rohdatenpruefung | Ergebnis |
|---|---:|
| Helden im explizit gewaehlten Cache | 38 |
| Item-/Waffen-/Ability-Definitionen | 726 |
| Properties in diesen Definitionen | 14.003 |
| Nichtleere explizite `conditional`-Textfelder | 18 |
| Nicht als einheitenloses endliches Zahlenliteral interpretierte Werte | 1.023 |
| Rohe Skalierungseintraege/-funktionen inklusive Null-/Hilfsfeldern | 5.584 |
| Negative Property-Skalierungskoeffizienten | 5 |
| Fehlende/mehrdeutige Referenzen fuer 4 Signaturfaehigkeiten und Primaerwaffe je Held | 0 |

Diese Zaehler sind KEIN Prozentsatz simulierter oder verstandener Mechaniken. Insbesondere sind 5.584 rohe Eintraege nicht 5.584 wirksame Spirit-Konversionen, und 1.023 nichtnumerische Literale sind nicht automatisch Parserfehler. `conditional` ist nicht die einzige Bedingungsquelle: `ConditionallyApplied`-Flags, Modifier und Triggertexte muessen gesondert ausgewertet werden.

## Wesentlicher eigener Befund: mehr als Feuerrate

Der kompakte Bericht enthaelt **14 nicht-null Hero-Stat-Eintraege mit `scaling_stat=ETechPower` bei 11 Helden**. Alle Werte unten sind Rohkoeffizienten dieses Cache-Paars, keine aktuelle In-Game-Messung und keine ungeprueft festgelegten Einheiten.

| Held | Raw-Zielstat und Koeffizient |
|---|---|
| Vindicta | `EBulletDamage=0.022` |
| Wraith | `ESprintSpeed=0.05` |
| Haze | `EClipSize=0.5` |
| Grey Talon | `EBulletDamage=0.08`, `EMaxMoveSpeed=0.0084` |
| Warden | `EFireRate=0.25`, `ERoundsPerSecond=0.01` |
| Yamato | `EClipSize=0.15` |
| Vyper | `EMaxMoveSpeed=0.0138` |
| Sinclair | `EMaxMoveSpeed=0.0138` |
| Venator | `EBulletArmorDamageReduction=0.12178`, `ETechArmorDamageReduction=0.12178` |
| Victor | `EBaseHealthRegen=0.08` |
| Paige | `EHeavyMeleeDamage=0.3` |

Folgerung fuer die Abnahme: Ein Zaehler mit nur einem Feuerraten-Konverter belegt nicht die Vollstaendigkeit des Helden-Konversionsgraphen. Waffenschaden und Magazin brauchen ebenfalls nachgewiesene Consumers in Waffenzyklus, damage_plan und marginaler Bewertung. Bewegung, typisierte Defensive, Regen und Nahkampf sind weitere konkret belegte Consumers fuer die Folgephasen. Die beiden Warden-Darstellungen duerfen nicht einfach addiert werden. Welche Einheit und Kombinationsregel gilt, muss je Zielstat am Datenvertrag/Consumer belegt werden, nicht aus dem Namen des Helden geraten werden.

Auch negative Werte sind nicht pauschal schlecht: die gefundenen Koeffizienten betreffen unter anderem Parry-Cooldown, gegnerische Feuerrate, Ressourcengenerierung und Armor-Reduction. Vorzeichen erhalten; Zielobjekt, Einheit und Trigger bestimmen den Nutzen. Die fuenf originalen Funktionen samt Pfad stehen unverkuerzt im kompakten Bericht.

## Provenienz und Grenzen

Die zwei Dateien wurden explizit als vorhandene lokale Quellen ausgewaehlt, nicht als neueste Version behauptet. Ihre Client-/Patchversion und gemeinsame Patchkonsistenz sind hier NICHT belegt. Diese Rohdatenpruefung ersetzt weder FROZEN-V2+Population noch die Baseline von Worker A. Sie aendert keine laufenden Messinputs.

- Helden: `data/raw/deadlock_assets_api/heroes.05b319ab2aad485b.json`, 1.013.475 Bytes, SHA256 `05b319ab2aad485b9d1126bc57a80e1858b723816b4447c5a459e86745aa7a21`.
- Definitionen: `data/raw/deadlock_assets_api/items.168e4f665ca9ba9a.json`, 6.043.679 Bytes, SHA256 `168e4f665ca9ba9adbc8b20953451d0cd506cacb305a788f8a8ee0ef25a244a2`.

Die oeffentliche, am 16.09.2026 gelesene OpenAPI-Spezifikation unter `https://api.deadlock-api.com/openapi.json` dokumentiert `/v1/assets/*` und `/v1/assets/client-versions` mit explizitem `client_version`. Der alte Assets-Repo-README verweist noch auf den separaten Assets-Host. Im lokalen Produktcode verwenden `dbrain-sources/src/assets_api.rs` und `dbrain-builds/src/api.rs` ebenfalls noch den alten Host. Das ist ein zu pruefender Migrationspunkt, KEIN von dieser Arbeit bewiesener Produktionsausfall. Frische Entity-Payloads konnten ueber die hier verfuegbaren Web-/Netzwerkwege nicht erfolgreich geladen werden. Die gemessenen Werte stammen ausschliesslich aus den genannten lokalen Dateien.

Graphify und direkte curl/jq/ps-Aufrufe waren in dieser MCP-Sitzung nicht freigegeben. Keine Allowlist oder Sicherheitskontrolle geaendert/umgangen; Quellpruefung ueber erlaubte Dateitools, bestehende T3-Leseaufrufe und Cargo. Keine zentrale DB-Verbindung, Migration, Dienststeuerung, Veroeffentlichung oder neue Produktions-Pythonlogik.

## Reproduktion und Tests

Cargo: `/home/nathanael/.cargo/bin/cargo` (1.97.1); im Worktree-Unterordner `rust`.

```text
cargo test --offline -p dbrain-reasoner --example ability_coverage
cargo test --offline -p dbrain-reasoner --lib
cargo clippy --offline -p dbrain-reasoner --example ability_coverage -- -D warnings
cargo run --offline -p dbrain-reasoner --example ability_coverage -- raw HEROES.json ITEMS.json NEW-FULL.json NEW-SUMMARY.json
```

- Example: 10 Tests bestanden, 0 fehlgeschlagen, 0 ignoriert. Darunter signierte/ungueltige Werte, verschachtelte Provenienz, JSON-Pointer-Escaping, mehrdeutige/fehlende Referenzen, Umbenennungsneutralitaet, unveraenderte kompakte Ausgabe und Dateischutz.
- Bestehende Reasoner-Library: 173 bestanden, 16 ignorierte DB-Tests, 0 fehlgeschlagen. Keine ignorierten DB-Tests als ausgefuehrt ausgegeben.
- Fokussiertes Clippy mit `-D warnings`: bestanden.
- Formatierung ueber Cargo mit temporaerem, ignoriertem Manifest `target/audit-format/Cargo.toml`, das nur das geaenderte Example samt seinen Support-Modulen als Ziel nennt. Keine repo-weite Formatierung.
- Zwei echte Rohdatenlaeufe erfolgreich, gleiche oben aufgefuehrte Strukturzaehler. Kein behaupteter dreifacher KI-Backtest; die drei identischen Serialisierungen sind ein Unit-Test des kompakten Audits.

## Status

Eigenstaendiger gepruefter Tool-/Recherchepatch, noch keine unabhaengige Mergefreigabe. Kein main-Merge/Deploy und keine neue hero_build_id. Der Produkt-Reasoner ist damit nicht fertig. Der eigene Patch wird auf dem Feature-Branch gesichert; eigene Tests ersetzen nicht die unabhaengige Pruefung vor Integration. Fuer diesen Offline-Pruefer ist kein Dienstneustart sinnvoll.
