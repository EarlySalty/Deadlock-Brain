# Live-Befunde für U0/U1 vom 18. September 2026

Diese Befunde stammen aus lesenden Abfragen durch den Orchestrator. Keine Produktionsmigration, kein Deploy und kein Modelllauf wurden dabei ausgeführt.

## 1. Quellenidentität und Ereigniszuordnung

Die offizielle Nutzer-URL `https://store.steampowered.com/news/app/1422450/view/698776157349216434` wurde tatsächlich abgerufen. In `data-partnereventstore` stehen gemeinsam `gid=698776157349216434` und `announcement_body.gid=698776157349216435`; es handelt sich nachweislich um dasselbe Ereignis und dessen Announcement. `announcement_body.posttime=1789589803`, `updatetime=1789590693` (2026-09-16T20:31:33Z).

Die produktive Datenbank `deadlock` enthält:

- `patchnotes.changelog_posts.id=285`, Titel `Minor Update - 09-16-2026`, `posted_at=2026-09-16T20:16:43Z`.
- `url=https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435`.
- **123** `brain.patch_events` für diesen Titel, deren `patch_external_id` genau diese Quellen-URL ist.
- **0** Ereignisse mit `patch_external_id='patch_285'`.

Die neue CLI sucht ausschließlich nach `patch_285`. Es fehlen also nicht die Ereignisse, sondern die Integration der vorhandenen Identität. Betroffen sind auch `evidence_revision`, die Review-Sperre und `capture_patch_evidence`: Quelländerungen benutzen `patch_<id>`, Ereignisänderungen die URL. Das kann sowohl den Kontext als auch die Invalidierung und den Speicherschutz falsch machen.

### Eng begrenzter Korrekturauftrag

Nach Abschluss und Push der bisherigen zwei U0-Korrekturen denselben Worktree und Thread weiterverwenden. Die vorhandenen Ereignis-IDs dürfen NICHT produktiv umgeschrieben werden. Die Analyse soll interne Quell-ID und belegte Quellen-URL nachvollziehbar auflösen, unverändert gespeicherte IDs im Beleg behalten und konsistent sperren/Revisionen prüfen/Entwürfe invalidieren. Mehrdeutige Quellzuordnungen müssen sichtbar bleiben statt nach Titel geraten zu werden. Keine neue Plattform und kein neuer Sammler. Abweichende Quellen-URL im selben Datensatz bei Update muss alte und neue Zuordnung sicher behandeln.

Prüfen: realistischer URL-basierter Fixture-Fall, Kontext lädt dessen Ereignisse; Ereignisrevision während Generierung wird erkannt; Quelländerung und Ereignisänderung markieren denselben kanonischen Review erneut prüfbedürftig. Ursprüngliche Migration unverändert lassen, nötige Korrektur als Folgemigration. Existierende `patch_<id>`-Fixtures bleiben unterstützt. Tests, Clippy, nur eigene Rust-Dateien formatieren, eigener Feature-Push und Report. Kein Produktionsschreiben oder Merge durch den Worker.

## 2. Falscher Snapshot-Typ und reale Kontextgröße

`deadlock_assets_api` speichert Items und Fähigkeiten als `entity_type='item_or_ability'`. Die CLI filtert bislang nur `hero`, `item`, `ability`; dadurch fehlen diese vorhandenen Quellen vollständig.

Lesender Nachweis mit denselben Zeit- und Quellenfiltern wie die CLI, jeweils jüngster Snapshot pro source/entity_type/external_id vor Veröffentlichung:

| Typ | Anzahl | Bytes der JSONB-Payloads |
|---|---:|---:|
| hero | 61 | 1.571.768 |
| item_or_ability | 1.457 | 9.824.654 |

Der Kontext muss den tatsächlichen Typ berücksichtigen. Ein größerer Grenzwert ist keine fachliche Lösung: schon die Helden allein überschreiten `MAX_CONTEXT_BYTES=768000`; vollständig wären es 1518 Snapshots bei maximal erlaubtem `snapshot-limit=1000`. Quellen dürfen nicht still abgeschnitten oder neue Spielmechanik-Annahmen erfunden werden. Falls ohne konzeptionelle Kontextauswahl kein Lauf möglich ist, muss die CLI einen klaren, gemessenen Blocker liefern, bevor ein Modell aufgerufen wird. Nicht ungefragt mit erhöhtem Budget oder einem anderen Modell ausweichen. Kein `patch-only`-Lauf als vollständiger U1-Nachweis.

## 3. Gespeicherte und aktuelle offizielle Revision unterscheiden sich

Gespeichertes `raw_content` von Zeile 285: 8819 Zeichen, SHA-256 `49217733829b68d4e3060adf75abe94c117a1634503755e447a7c9c52377e6c1`.

Im gespeicherten Original steht noch `Abrams: Base gun damage increased by 5%`. Diese Zeile fehlt sowohl im frisch abgerufenen offiziellen Steam-News-HTML als auch im offiziellen `ISteamNews/GetNewsForApp/v2`-Feed. Außerdem unterscheiden sich Schreibweisen (`Veilwalker`/`Veil Walker`, `Petryifying`/`Petrifying`, `projectical`/`projectile`). Das ist kein Creator-Vergleich.

Rohbelege auf dem Server, nur öffentliche Steam-Daten:

- `/tmp/brain-reference-steam-20260918.html`, SHA-256 `70c818cabfc40b4d5eaa402cf9e634edb951d39e3283fd30da3d9908884dcc46`.
- `/tmp/brain-reference-steam-news-20260918.json`, SHA-256 `42f2d9a5657f3aa4f9883b273e38f364ad6e966356777b4e30cd06ed9d36bae9`.

Der News-Feed enthält auch externe Presseartikel. Diese dürfen nicht in den Primärdaten-Kontext gelangen; die offizielle Meldung hat `feedname=steam_community_announcements` und Titel `Minor Update - 09-16-2026`.

Kein manuelles Geradebiegen der Produktionsquellen. Vor einer aktuellen Analyse müssen gespeicherte und aktuelle Revision über den vorhandenen Sammler/Importpfad revisionssicher abgeglichen werden. Der bisherige Patch-Sync-Wrapper prüft nur die maximale Quell-ID gegen bereits importierte Snapshot-IDs; damit erkennt sein Vorcheck spätere Änderungen desselben Beitrags nicht zuverlässig. Das ist ein gesonderter Betriebsbefund, keine Freigabe zum Neubau eines Sammlers.

## 4. Betrieb

PostgreSQL 16.14, Datenbank `deadlock`. Die relevanten Quelltabellen gehören Rolle `deadlock`. Neue Evidenztabellen und Views sind noch nicht installiert. Der vorhandene lokale Nutzerzugang funktioniert ohne Geheimnis mit dem Unix-Socket; Runtime-Abfrage wurde über eine transiente systemd-Unit mit bestehender EnvironmentFile und nicht geheimem Socket-DSN als Rolle `deadlock` durchgeführt. ENV-Dateien und Zugangswerte wurden nicht geöffnet oder ausgegeben.

`deadlock-brain status` lief erfolgreich. Der ältere YouTube-Lernwrapper ist deaktiviert und wurde nicht eingeschaltet. Das Referenzvideo und das Vergleichstranskript wurden nicht geöffnet. Andere Worktrees und die parallelen RAG-Pakete wurden nicht verändert.
