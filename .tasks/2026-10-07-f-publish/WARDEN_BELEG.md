# Warden: realer F-Lauf und Veröffentlichungsblocker

## Regulärer eigener Lauf

Eigenes finales Debug-Binary, kein Release oder Dienstneustart:

```bash
/opt/deadlock-brain/current/bin/deadlock-brain-secret-exec --config /etc/deadlock-brain/infisical.json -- /home/nathanael/.worktrees/brain-f-publish/rust/target/debug/deadlock-brain reason build Warden --no-ai --json --publish --infisical-config /etc/deadlock-brain/infisical.json
```

Der vorhandene Launcher erhielt die bereits vorhandene Service-Berechtigung über FD 5. Keine Secretwerte ausgegeben oder gespeichert. Log lokal: `warden-publish-final.log`.

Exit 1, wörtlicher Fehler:

```text
Datenfehler: API-Spielwerte sind nicht für den aktiven Patch belegt.
```

Der reguläre Guard hat vor dem HTTP-Senden abgebrochen. Keine bestätigte Veröffentlichung und keine `hero_build_id`. Kein Review-Ausweichpfad, keine künstliche Queue-ID als Erfolg.

## Lesend belegte Ursache in der gemeinsamen Datenbasis

Postgres-Datenbank `deadlock`, ausschließlich SELECT-Abfragen:

- Aktiver Patch laut demselben `latest_patch_tag`-SQL wie der Publisher: `https://store.steampowered.com/news/app/1422450/view/703281025618282632`, Zeitpunkt `2026-10-05 23:05:32+00`.
- Neuester Warden-API-Snapshot: Hero-ID 25, `2026-10-02 21:02:58.842491+00`, Dokument-ID 46269, `metadata.adapter.client_version` fehlt.
- Neuester erfolgreicher Assets-Run: `2026-10-06 21:05:54.768984+00`, 39 Helden und 738 Item-/Ability-Snapshots. Seine Summary hat keine `client_version`, kein `checked_at` und kein `mirror_complete`.

Das belegt die bereits gemeldete E-Schnittstelle: ein neuer Run beweist bei deduplizierten unveränderten Payloads nicht automatisch aktuelle Snapshotbindung. Der Publish-Guard wird nicht abgeschwächt, die produktive DB nicht per Hand korrigiert. E muss die aktuelle Spiegelmitgliedschaft selbst belegen. Danach muss derselbe reguläre F-Lauf wiederholt werden; weitere Mechanik- oder Strukturfehler sind dadurch noch nicht ausgeschlossen.

## Entwurf und Vergleich ohne Rohmatches

Der vorhandene Reasoner hat einen Warden-Entwurf in `brain.reasoner_builds` für den aktiven Patch erzeugt: `used_ai=false`, keine Familie, globale Confidence Low. Der Entwurf ist kein freigegebener oder veröffentlichter Build.

Vergleich ausschließlich mit `brain.population_item_stats`, Hero 25, Bucket `all`, Aggregate vom `2026-10-07 01:45:04.461389+00`. Keine Einzelmatches gelesen, importiert oder gespeichert. Die Prozentwerte sind die vorhandene gewichtete Kaufhäufigkeit, keine Siegquote und kein Nachweis, dass der Entwurf stärker spielt. Es wurden keine vollständigen häufig gespielten Einzelbuilds rekonstruiert.

| Kaufposition im Entwurf | Item | Gewichtete Kaufhäufigkeit |
| --- | --- | --- |
| 1 | Extended Magazine | 76,1 % |
| 2 | Titanic Magazine | 90,2 % |
| 3 | Extra Spirit | 26,0 % |
| 4 | Improved Spirit | 24,6 % |
| 5 | Bullet Resist Shredder | 19,5 % |
| 6 | Enchanter's Emblem | 5,6 % |
| 7 | Quicksilver Reload | 96,0 % |
| 8 | Active Reload | gerundet 0,0 % |
| 9 | Kinetic Dash | 7,1 % |
| 10 | Weakening Headshot | 4,5 % |
| 11 | Stalker | 0,2 % |
| 12 | Swift Striker | 80,3 % |
| 13 | Glass Cannon | 0,8 % |
| 14 | Burst Fire | 1,1 % |
| 15 | Hollow Point | 1,2 % |
| 16 | Mercurial Magnum | 94,2 % |
| 17 | Unstable Concoction | kein Aggregat |
| 18 | Weighted Shots | 4,1 % |
| 19 | Disarming Hex | 0,7 % |
| 20 | Healing Tempo | 0,7 % |
| 21 | Infinite Rounds | kein Aggregat |

Abweichung vom häufigen Kaufmuster: Opening Rounds (94,1 %) und High-Velocity Rounds (92,4 %) fehlen im Kern. Fleetfoot (81,5 %) steht nur als Option, nicht als Pflichtkauf. Spiritual Overflow (80,4 %) und Enduring Speed (77,7 %) fehlen im Kern. Der Entwurf enthält stattdessen mehrere seltene Kandidaten und zwei Items ohne Populationseintrag. Die gemeinsame Zeitschrittrechnung entscheidet die Reihenfolge; Popularität ist kein Zulassungsfilter mehr.

Grenze des Belegs: reale Planung und sichere Ablehnung sind nachgewiesen. Aktuelle API-Provenienz, vollständige reguläre Publish-Abnahme und Steam-Build-ID bleiben offen.
