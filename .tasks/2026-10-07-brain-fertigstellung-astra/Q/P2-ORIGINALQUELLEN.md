# Q: fünf vorbereitete Originalpatchstichproben

Quellensnapshot: `q-patches-v1-20261007`, erfasst am 2026-10-07 um 11:59:00 UTC. Keine private Frage im Request, kein Modellaufruf und keine Chat-Zustellung. **Originalbelege vorbereitet, P2 noch nicht live abgenommen.**

## Quelle und Integrität

Der vorhandene Importer `rust/crates/deadlock-brain/src/pg_patchnotes.rs:19-21` benennt Steam-App `1422450`, `ISteamNews/GetNewsForApp/v2/` und 500 Beiträge als Suchumfang. Q verwendet diesen öffentlichen Originalzugang, keinen neuen Produktimporter:

`https://api.steampowered.com/ISteamNews/GetNewsForApp/v2/?appid=1422450&count=500&maxlength=0`

Der Q-Snapshot enthält 41 Beiträge aus `steam_community_announcements`. Fremde Nachrichtenfeeds wurden ausgeschlossen. Die Feedzugehörigkeit allein ist keine Gameplayklassifikation: Ankündigungen und Zahlenpatches bleiben getrennt.

Lokale Datei: `Q/private/q-patches-v1-20261007/public-patches.json`, SHA256 `490208eeb5cf0952f2dfb20c1bd8f78882c0090c996bb999bc37622abfaa19e0`. Dieser Hash bindet den normalisierten Snapshotcontainer einschließlich Abrufzeit, nicht unveränderte HTTP-Bytes. Die folgenden `contents`-Hashes binden jeweils die UTF-8-Zeichenkette aus dem ursprünglichen Steam-JSON vor Textbereinigung.

## Stichproben mit festgehaltenen Erwartungen

| Fall | Offizieller Beitrag, Veröffentlichungszeit UTC | Direkt belegte Änderung | Erwartete Antwortart |
|---|---|---|---|
| P2-01 | [Minor Update vom 05.10.2026](https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1845383656394833), `2026-10-05T23:05:32Z` | Rat Kings Scrap Grenade: Schaden 65 auf 70; Rat Swarm: Grundabklingzeit 28 auf 24 Sekunden. | Als Gameplay-/Balanceänderung erkennen, die richtigen Vorher-/Nachherwerte mit Quelle nennen. |
| P2-02 | [Minor Update vom 16.09.2026](https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1844115010490072), `2026-09-16T20:16:43Z` | Guardian-Belohnung um 10 Prozent erhöht, Walker-Belohnung um 5 Prozent erhöht. | Relative Erhöhung erklären, nicht als absolute Seelenmenge oder Prozentpunkte ausgeben. |
| P2-03 | [Minor Update vom 22.08.2026](https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1841579228672283), `2026-08-22T21:40:46Z` | Radiant Regeneration: Heilung beim Wirken 70 auf 65. | Konkrete Verringerung als Gameplayänderung erkennen, keine kosmetische Meldung daraus machen. |
| P2-04 | [Minor Update vom 12.08.2026](https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1840944183775204), `2026-08-12T22:57:44Z` | Apollos Riposte: Verringerung der Nahkampfresistenz von −22 Prozent auf −25 Prozent. | Stärkere Resistenzverringerung erklären; nicht wegen des negativeren Werts als Abschwächung beschreiben. |
| P2-05 | [Minor Update vom 28.07.2026](https://steamstore-a.akamaihd.net/news/externalpost/steam_community_announcements/1839041357039193), `2026-07-28T20:24:35Z` | Haze Sleep Dagger: Spirit-Schadenskoeffizient 2,8 auf 2,2. Pocket Affliction: DPS ungefähr 6 Prozent reduziert, einschließlich Spirit-Skalierung und Ausbauten. | Historische Änderung mit Datum nennen. Aktuelle Gesamtwerte nicht allein aus diesem alten Patch ableiten; dafür den aktuellen Assets-Snapshot verwenden. |

Die Stichproben wurden aus tatsächlichen öffentlichen Zahlenänderungen gewählt, nicht aus privaten Evalfragen. Sie zählen deshalb **nicht** als fünf zusätzliche echte P0-Fragen. Erwartungen stehen vor jedem späteren Antwortlauf fest.

## Inhaltshashes

| Fall | Steam-GID | SHA256 der originalen `contents`-Zeichenkette |
|---|---|---|
| P2-01 | `1845383656394833` | `be200138d80e244d57f2c22b81e839e9cd34989faa6895ab1ace75de523a3d08` |
| P2-02 | `1844115010490072` | `6603a1efd95ca16d802b19adb65e54ccac9fccc4f22e75d4c34bc8f89116a1ae` |
| P2-03 | `1841579228672283` | `08352d7971bdebbf20445c6d829db58629fb6423fbbaf1271373f78305b43ba2` |
| P2-04 | `1840944183775204` | `cda56596f7745b6401a7cf06dc7788acee6c6dd767dc39ef59e3865bae4951e3` |
| P2-05 | `1839041357039193` | `f5153835efa33fd4b8db69d044b9d63ad30fd5d7bfdfab78210c4057e1ed4ec8` |

## Aktualitätsgrenze

Im abgefragten Steam-Suchumfang ist der Minor Update vom 05.10.2026 der jüngste gefundene numerische Balancepatch. Der neuere offizielle Beitrag `Mind the Birds!` vom 06.10.2026 ist eine Heldenankündigung, kein aus demselben Titel abgeleiteter Zahlenpatch. Er wird nicht pauschal als kosmetisch abgetan. Das gesamte Forum und mögliche weitere Patchkanäle wurden für diesen Nachweis nicht vollständig abgeglichen. Q behauptet deshalb keinen weltweit jüngsten bestätigten Gameplaypatch.

Noch offen: Brainantworten auf die fünf Stichproben, tatsächliche Erkennung kosmetischer Forumbeiträge, aktuelle gemeinsame Quellenbindung von I und zugestellte Antworten. Kein Produktionsdienst, Forumimporter oder Datenbankzustand wurde dafür verändert.
