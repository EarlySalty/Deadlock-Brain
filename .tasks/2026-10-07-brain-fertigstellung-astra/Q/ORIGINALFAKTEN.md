# Q: unabhängig geprüfte Originalfakten, Version 1

Stand der API-Abfrage: 2026-10-07, 11:19:14 UTC. Kein privater Fragetext wurde an die API oder einen Modellanbieter übertragen. Diese Datei enthält nur öffentliche Spielmechaniken, keine Quellenpersonen oder Chatreferenzen.

## Herkunft

Die aktuellen Endpunkte wurden aus dem bestehenden Adapter `rust/crates/dbrain-sources/src/assets_api.rs:16-20` übernommen, nicht geraten:

| Öffentliche Originalquelle | Lokaler unveränderlicher Snapshot | SHA256 des Snapshotcontainers |
|---|---|---|
| `https://api.deadlock-api.com/v1/assets/heroes?only_active=true` | Q/private/q-original-v1-20261007/public-heroes.json, 40 Helden | `8bfd7ecd82966695285ad520a6f1bd853b60c6e5520231c4aad006974f65c69c` |
| `https://api.deadlock-api.com/v1/assets/items` | Q/private/q-original-v1-20261007/public-items.json, 746 Entitäten | `6c93c2f5449f34c4b2d45600491739fdbc2871755f0aed83fe453459d3e5f65a` |

Die Hashes binden die lokal normalisierten JSON-Snapshotcontainer einschließlich Abrufzeit. Es sind keine behaupteten Hashes eines unveränderten HTTP-Byte-Receipts. Q verändert Is Spiegel- oder Receipt-Pipeline nicht.

Ein erster Versuch mit dem alten Host `assets.deadlock-api.com` scheiterte an DNS. Anschließend den vorhandenen Adapter geprüft und den aktuellen Host `api.deadlock-api.com` erfolgreich verwendet. Das war ein Fehler der Prüfannahme, kein Ausfall der aktuellen Originalquelle.

## Haze: Spirit ist nicht pauschal zusätzlicher Waffenschaden

Erwartete Antwortart zur belegten Haze-Kernfrage: **Ja, konkrete Teile skalieren mit Spirit. Einzelne Fähigkeiten und Dauer-/Proc-Skalierung unterscheiden, nicht behaupten, sämtliche Kugeln skalierten unmittelbar mit Spirit.**

| Aktuelle Entität | Direkt gelesener Originalwert |
|---|---|
| Haze | Helden-ID `13`, `class_name=hero_haze` |
| Sleep Dagger, ID `2948410412` | `properties.Damage.value=65`; `scale_function.specific_stat_scale_type=ETechPower`, `stat_scale=2.2`. Nicht den historischen Koeffizienten 2.8 verwenden. |
| Smoke Bomb, ID `2414191464` | `properties.AbilityDuration.value=8`; Spiritkoeffizient `0.1`. Die Skalierungsstatistik enthält `ETechPower` und `ETechDuration`. |
| Bullet Dance, ID `731943444` | `properties.AbilityDuration.value=3.5`; Spiritkoeffizient `0.03`. Daneben `EChannelDuration` und `ETechDuration`. Beschreibung: feuert die Waffe mit zusätzlichem Kugelschaden. |
| Fixation, ID `1080948381` | Grund-Proc `0`, Spiritkoeffizient `0`; erster Fähigkeitsausbau ergänzt `ProcDamage=40`, Spiritkoeffizient `0.8` und Proc bei `20` Stacks. Waffenschadensbonus pro Stack ist eine getrennte Eigenschaft. |

Der historische Wiki-Abdruck stammt aus Commit `e3fb36eb…` vom 2026-07-08 und enthält für Sleep Dagger `2.8`. Er wurde ausdrücklich nicht als aktuelles Goldlabel verwendet.

## Pocket: Konterhinweise an aktuelle Mechaniken binden

Erwartete Antwortart zur Pocket-Kernfrage: **Praktische, quellengebundene Gegenmaßnahmen erklären, mit Timing statt pauschalen Itemgarantien. Nicht auf Nani oder einen anderen Antwortbot verweisen.**

| Aktuelle Entität | Direkt gelesener Originalwert und Grenze |
|---|---|
| Pocket | Helden-ID `50`, `class_name=hero_synth` |
| Barrage, ID `938149308` | Spirit-Schaden pro Projektil `32`, Spiritkoeffizient `0.465`; Treffer auf Helden erhöhen Pockets Schaden. Erst Flächentreffer und das Aufbauen des Bonus vermeiden, nicht nur späteren Schaden behandeln. |
| Flying Cloak, ID `1976701714` | Die Originalbeschreibung bestätigt einen nach vorne fliegenden, schädigenden Mantel und den erneuten Tastendruck zur Teleportation. Einen verfügbaren Fluchtweg beim Konter berücksichtigen. |
| Enchanter's Satchel, ID `3747867012` | Die Originalbeschreibung bestätigt den Rückzug in den Koffer und Spirit-Flächenschaden beim Ende. Frühes Beenden durch eine Aktion möglich. Kein unbestätigtes numerisches Unverwundbarkeitslabel ergänzen. |
| Affliction, ID `2954330093` | Grund-DPS `32`, Spiritkoeffizient `0.21`, Debuffdauer `10`. Originalbeschreibung: Schaden ist nicht tödlich und löst keine Item-Procs aus. Historisches Wiki-DPS `34` und Dauer `11` nicht als aktuell verwenden. |
| Silence Wave, ID `619484391` | Originalbeschreibung: Schweigen unterbricht keine kanalisierten Fähigkeiten. Schweigen vor einem Cast und Unterbrechen eines laufenden Casts nicht gleichsetzen. |
| Knockdown, ID `1254091416` | Originalbeschreibung: Betäubung nach `2s`, länger gegen Ziele in der Luft. Kein sofortiger garantierter Treffer; Timing und Fluchtmöglichkeiten erklären. |

`Dispel Magic`, ID `3731635960`, ist im aktuellen Datensatz vorhanden. Aus dem Itemnamen allein wird hier **keine** vollständig belegte Reinigungswirkung oder garantierte Affliction-Entfernung abgeleitet. Die verfügbaren Angaben genügten dafür in dieser Prüfung nicht. Erwartete Antwortgrenzen bleiben ehrlich.

## Was dies nicht beweist

Keine Modellantwort, keine Discord-/Twitch-Zustellung und kein P1-Zeitwert. Fünf separat vorbereitete Originalpatchstichproben stehen inzwischen in `P2-ORIGINALQUELLEN.md`; ihre Antwortabnahme ist offen. Keine Zusage, welcher Patch über sämtliche Quellen hinweg der jüngste bestätigte Gameplay-Patch ist. Diese Fakten werden getrennt vom privaten Teilset gehalten; die noch offenen Herkunfts-/Goldlabelprüfungen werden nicht als vollständig ausgegeben.
