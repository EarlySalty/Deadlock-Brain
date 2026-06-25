# deadlock-data Survey und Integrationsplan

Datum: 2026-06-25  
Scope: Read-only Survey. Externe Repos wurden shallow nach `/tmp` geklont. Keine Live-DB-Writes, keine Source-Implementierung.

## Kurzfazit

`deadlock-wiki/deadlock-data` ist fuer Deadlock-Brain eine starke, Cloudflare-freie TRUSTED-Quelle fuer strukturierte Spieldaten. Gegenueber unserer aktuellen Assets-/API-Pipeline liefert sie vor allem:

- vollstaendige Wiki-Build-Daten als Git-Artefakt mit Commit- und `version.txt`-Audit,
- Hero-Basiswerte plus `LevelScaling`, `SpiritScaling`, Weapon-Substruktur und sinnvolle Stat-Auswahl,
- Ability-/Item-Cards mit UI-nahem Property-Modell inklusive `Scale`-Objekten fuer Spirit- und Power-Increase-Scaling,
- Changelogs als Raw-Text, Wiki-Wikitext und bis 2026-01-30 auch strukturierte Tag-JSONs,
- `resource-lookup.json` fuer robuste Name -> Key -> Typ-Aufloesung,
- Localizations fuer Namen, Lore, Rollen, Beschreibungen und Tooltips.

`deadlock-wiki/pages` enthaelt im shallow Clone keine eingecheckten Seiten-Wikitext-Dateien mit menschlicher Prosa. Es ist ein Bot-/Generator-Repo, das zur Laufzeit per `mwclient` Seiten vom Wiki liest und nach `./data` schreibt. Damit ist es in diesem Zustand kein Cloudflare-freier statischer Prosa-Datenweg. Es belegt aber, welche Seitenarten relevant waeren: Hauptseiten, `Update history`, Ability-`Notes`, Blueprints und Data-Pages.

## Geklone Repos

| Repo | Pfad | HEAD | Commit-Zeit | Hinweis |
|---|---|---|---|---|
| `github.com/deadlock-wiki/deadlock-data` | `/tmp/dl-data` | `67d2c9122e0841c1f240f8968977d70f8143c631` | `2026-06-22T11:31:13+00:00` | `Deadbot v1.14.0 | Client 6592 - Jun 19 2026` |
| `github.com/deadlock-wiki/pages` | `/tmp/dl-pages` | `acc31295ceb33dc6af651e469b6e661371508065` | `2025-01-01T20:53:25-05:00` | Bot-Code, keine eingecheckten `./data`-Seiten |

`/tmp/dl-data/data/version.txt`:

```text
ClientVersion=6592
ServerVersion=6592
ProductName=citadel
appID=1422450
ServerAppID=1422460
SourceRevision=10757662
VersionDate=Jun 19 2026
VersionTime=14:48:08
```

## deadlock-data Inventar

| Pfad | Struktur | Anzahl | Zweck |
|---|---:|---:|---|
| `data/json/hero-data.json` | Objekt `hero_key -> Hero` | 64 | Hero-Basiswerte, Movement, Weapon, Level-/Spirit-Scaling, Ability-Bindings |
| `data/json/ability-data.json` | Objekt `ability_key -> Ability` | 356 | Raw/engine-nahe Ability-Properties, Modifier, Upgrades |
| `data/json/ability-cards.json` | Objekt `hero_key -> {Name, 1..4}` | 64 | Wiki-/UI-nahe Ability-Cards mit Props, Scale, Cooldowns, Upgrades |
| `data/json/item-data.json` | Objekt `item_key -> Item` | 279 | Kosten, Tier, Slot, Components, aktive/passive Properties |
| `data/json/item-cards.json` | Objekt `item_key -> ItemCard` | 279 | Wiki-/UI-nahe Item-Cards mit `Info1`, `Other`, `Upgrades` |
| `data/json/npc-data.json` | Objekt `npc_key -> NPC` | 61 | Objectives/NPCs mit Health, DPS, Resistenz, WeaponInfo |
| `data/json/resource-lookup.json` | Objekt `display_name_norm -> lookup` | 439 | Alias-/Resolver-Bruecke: Name, Key, Typ, optional Hero |
| `data/json/attribute-data.json` | Objekt `Weapon/Vitality/Spirit -> attrs` | 3 | Stat-Labels/Postfixes |
| `data/json/hero-meaningful-stats.json` | Objekt `stat_key -> true` | 53 | Filter fuer relevante Hero-Stats |
| `data/json/stat-infobox-order.json` | Objekt mit `category_order` | 4 | UI-Reihenfolge fuer Stats |
| `data/json/soul-unlock-data.json` | Array/Objekt nach Level | 36 | Required Souls, AP, Ability Unlocks |
| `data/json/generic-data.json` | Objekt | 44 | Game-weite Economy/Objective/Map-/Shop-Parameter |
| `data/json/convars.json` | Objekt | 3.797 | Engine-/Convar-Werte, nur selektiv sinnvoll |
| `data/json/misc-data.json` | Objekt | 86 | Weitere Engine-/Game-Parameter |
| `data/json/midtown-metadata.json` | Objekt | 2 | Map-Metadaten |
| `data/csv/hero-data.csv` | CSV-Export | 64 Zeilen plus Header | Flacher Export von `hero-data` |
| `data/csv/item-data.csv` | CSV-Export | 279 Zeilen plus Header | Flacher Export von `item-data`, sehr breite Sparse-Spalten |
| `data/changelogs/changelog_configs.json` | Objekt `date -> config` | 127 | `forum_id`, `date`, `link`, `is_hero_lab` |
| `data/changelogs/raw/*.txt` | Patchnotes Text | 127 | Raw Patchtext |
| `data/changelogs/wiki/*.txt` | Wikitext | 116 | Wiki-markierte Patchnotes, `{{Update layout}}`, `{{HeroIcon}}`, Links |
| `data/changelogs/versions/*.json` | Array Events | 109 | Strukturierte `Description` + `Tags`, nur bis 2026-01-30 vorhanden |
| `data/changelogs/tag_tree.json` | Baum | 1 | Taxonomie fuer Changelog-Tags |
| `data/changelogs/hotfixes.json` | Array | 0 | Aktuell leer |
| `data/localizations/*.json` | Objekt localization key -> text | mehrere Sprachen | Namen, Lore, Rollen, Beschreibungen; einige Sprachen sind `{}` |
| `data/item-component-tree.txt` | Mermaid-artige Kanten | 58 Kanten | Item-Komponentenbaum |
| `data/assets/*.png` | PNG | 3 | Kartenbilder fuer Crates/Statues/Shops |

Die JSON-Dateien sind PascalCase. Das ist wichtig, weil unsere aktuelle Assets-API-Normalisierung auf `name`, `id`, `class_name` in lower/snake-case achtet. Bei `deadlock-data` sollte die Source selbst `canonical_name` und Alias-Metadaten sauber setzen.

## IDs, Keys und Matching

Das primaere technische ID-Modell ist der Top-Level-Key in den JSON-Objekten:

- Hero: `hero_atlas`, `hero_inferno`, `hero_haze`
- Ability: `ability_afterburn`, `citadel_ability_bull_heal`, `ability_incendiary_projectile`
- Item: `upgrade_magic_reach`, `upgrade_chain_lightning`
- NPC: `alt_npc_boss_tier1`

`resource-lookup.json` ist die beste Bruecke zu unserem `entities`-/Aliasmodell:

```json
{
  "abrams": {"name": "Abrams", "key": "hero_atlas", "type": "hero"},
  "afterburn": {
    "name": "Afterburn",
    "key": "ability_afterburn",
    "hero_name": "Infernus",
    "hero_key": "hero_inferno",
    "type": "ability"
  },
  "mystic expansion": {
    "name": "Mystic Expansion",
    "key": "upgrade_magic_reach",
    "type": "item"
  }
}
```

`hero-data.json` verbindet Heroes mit Abilities ueber `BoundAbilities`:

```json
{
  "Name": "Abrams",
  "Role": "hero_atlas_role",
  "MaxHealth": 800.0,
  "Stamina": 3,
  "LevelScaling": {
    "BulletDamage": 0.1,
    "MaxHealth": 52.0,
    "TechPower": 1.1,
    "DPS": 1.4286
  },
  "SpiritScaling": {},
  "BoundAbilities": {
    "1": {"Name": "Siphon Life", "Key": "citadel_ability_bull_heal"},
    "2": {"Name": "Shoulder Charge", "Key": "citadel_ability_bull_charge"},
    "3": {"Name": "Infernal Resilience", "Key": "citadel_ability_passive_beefy"},
    "4": {"Name": "Seismic Impact", "Key": "citadel_ability_bull_leap"}
  }
}
```

`LevelScaling` hat im aktuellen Snapshot diese Key-Union:

`BonusAttackRange`, `BulletDamage`, `BulletDamageAltFire`, `BulletResist`, `DPS`, `HeavyMeleeDamage`, `LightMeleeDamage`, `MaxHealth`, `PowerIncreases`, `SustainedDPS`, `TechPower`, `TechResist`.

`SpiritScaling` ist nicht durchgehend leer. 12 Heroes haben aktuelle Eintraege. Beispiele: `hero_haze` skaliert `ClipSize` und `SustainedDPS`, `hero_orion` skaliert `MaxMoveSpeed`, `BulletDamage`, `DPS`, `SustainedDPS`, `hero_warden` skaliert `RoundsPerSecond`, `FireRate`, `DPS`, `SustainedDPS`.

Ability- und Item-Cards enthalten die direkt nutzbaren Scaling-Props:

```json
{
  "Key": "ability_afterburn",
  "Name": "Afterburn",
  "Info1": {
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Value": 14.0,
          "Scale": {"Value": 0.66, "Type": "spirit"},
          "Type": "tech_damage"
        }
      ]
    }
  },
  "Upgrades": [
    {"DPS": 16},
    {"OutgoingTechDamagePercent": -35},
    {"BurnDuration": 3}
  ]
}
```

Item-Beispiel aus `item-data.json` und `item-cards.json`:

```json
{
  "Key": "upgrade_magic_reach",
  "Name": "Mystic Expansion",
  "Cost": 800,
  "Tier": 1,
  "Slot": "Tech",
  "Activation": "Passive",
  "IsImbue": true,
  "TechRangeMultiplier": 20,
  "TechRadiusMultiplier": 20,
  "PropertyUpgrades": {
    "TechRadiusMultiplier": 15,
    "TechRangeMultiplier": 15
  }
}
```

`item-component-tree.txt` ist als Kantenliste parsebar:

```text
graph
monster_rounds ---> cultist_sacrifice
tesla_bullets ---> capacitor
high_velocity_rounds ---> opening_rounds
slowing_bullets ---> weighted_shots
long_range ---> sharpshooter
```

Die Keys sind nicht immer identisch mit den Item-JSON-Keys (`monster_rounds` statt z.B. `upgrade_monster_rounds`). Mapping sollte deshalb zuerst ueber `resource-lookup`, dann ueber Normalisierung ohne Prefixe (`upgrade_`, `item_`) laufen.

## Changelog-Struktur

`changelog_configs.json` enthaelt je Patch:

```json
{
  "2024-05-03": {
    "forum_id": "427",
    "date": "2024-05-03",
    "link": "https://forums.playdeadlock.com/threads/05-03-2024-update.427/",
    "is_hero_lab": false
  }
}
```

`data/changelogs/raw/*.txt` enthaelt normalen Patchtext. `data/changelogs/wiki/*.txt` enthaelt Wikitext mit `{{Update layout}}`, `{{HeroIcon|...}}`, `{{AbilityIcon|...}}`, `[[...]]`-Links. Beispiel fuer `2026-05-22`:

```text
{{Update layout
| month = May
| day = 22
| year = 2026
| source = https://forums.playdeadlock.com/threads/05-22-2026-update.135477/
| notes =
=== General ===
* Base HP reduced by 10 for all heroes
* [[Guardian|Guardians]] scaling resistance ...
```

`data/changelogs/versions/*.json` ist bereits in einzelne Events zerlegt:

```json
{
  "Description": "* Parties of 3+ players now need to have an additional hero in their roster (from 3 to 4)",
  "Tags": ["Other"]
}
```

Wichtig: `versions/*.json` endet im untersuchten Snapshot bei `2026-01-30.json`, waehrend `raw` und `wiki` bis `2026-05-31` reichen. Die Integration sollte deshalb `raw`/`wiki` als primaere vollstaendige Patchnote-Quelle behandeln und `versions` als optionale Tag-/Event-Anreicherung verwenden.

`tag_tree.json` ist eine brauchbare Concept-Taxonomie fuer Patch-Events:

- `Hero -> Ability -> <Ability Name>`
- `Item -> Weapon/Vitality/Spirit Item`
- `NPC -> Creep -> Denizen/Trooper`
- `Objective -> Guardian/Base Guardian/Walker/Patron/Shrine`
- `Souls`, `Breakable`, `Shop`, `Map`, `Powerup`, `Zipline`, `Rejuvenator`, `Cosmic Veil`, `Street Brawl`-nahe Concepts ueber `Other`/Tags.

Das passt direkt zur vorherigen Top-Down-Luecke: viele `general` Patch-Events sind eigentlich Game-Concepts.

## Localizations

`data/localizations/english.json` und `german.json` liefern Namen, Lore, Rollen und Beschreibungen ueber dieselben Localization-Keys, die in den Daten referenziert werden:

```json
{
  "hero_atlas": "Abrams",
  "hero_atlas_role": "Charges into close combat",
  "ability_afterburn": "Afterburn",
  "ability_afterburn_desc": "Weapon hits build up a burning effect...",
  "upgrade_magic_reach": "Mystic Expansion",
  "upgrade_magic_reach_desc": "Imbue an ability to increase its range and effect radius."
}
```

Deutsch ist fuer UI-/Antwort-Kontext nuetzlich:

```json
{
  "ability_afterburn": "Nachbrennen",
  "upgrade_magic_reach": "Mystische Erweiterung"
}
```

Diese Daten koennen Alias- und Textfelder verbessern, sollten aber als Localization-Metadaten markiert bleiben. Nicht alle Sprachen sind befuellt; mehrere Dateien sind nur `{}`.

## pages Repo: Prosa-Befund

Der shallow Clone von `deadlock-wiki/pages` enthaelt keine eingecheckten Wikitext-Seiten, keine `data/`-Snapshots und keine `*.wiki`-/`*.wikitext`-Dateien. Gefundene Dateien sind nur:

- `README.md`, `requirements.txt`, `.env.example`
- `src/core/{main.py,reading.py,writing.py,validation.py,uploader.py}`
- `src/utils/{wiki.py,constants.py,file.py,...}`

Der Code belegt aber die geplante Seiten-Pipeline:

- `PageReader._get_data_pages()` liest `Data:AbilityData.json`, `Data:ItemData.json`, `Data:HeroData.json`.
- `PageReader._get_tracked_pages()` liest fuer jede Resource die Hauptseite und `Update history`; fuer Abilities zusaetzlich `Notes`.
- `_read_write_page()` ruft `self.wiki_obj.site.pages[page_name].text()` auf und schreibt erst dann nach `./data/...`.
- `PageWriter._merge_data()` erhaelt manuell editierbare Bereiche zwischen `<!--EditFreely-Section:...-->`-Markern.
- `Wiki` nutzt `mwclient.Site(SITE_URL, path='/')` und Login mit `BOT_WIKI_USER`/`BOT_WIKI_PASS`; `SITE_URL` steht im Repo auf `deadlocked.wiki`.

Bewertung:

- Ja, das Projekt ist konzeptionell fuer menschliche Prosa/Notes geeignet, weil es Hauptseiten, Ability-Notes und `EditFreely`-Sektionen bewahrt.
- Nein, das geklonte Git-Repo enthaelt diese Prosa nicht als eingecheckte Daten.
- Ohne funktionierenden MediaWiki-Zugriff und Bot-Credentials ist `pages` aus dieser Umgebung kein Cloudflare-freier Prosa-Pfad.
- Einziger aktuell statischer Wikitext im untersuchten Material sind `deadlock-data/data/changelogs/wiki/*.txt`, also Patchnotes, nicht Hero-Tipps/Strategie/Combos.

## Abgleich gegen vorhandenes Deadlock-Brain

Aktueller Source-Stand im Brain:

- `src/deadlock_brain/sources/assets_api.py` und `rust/crates/dbrain-sources/src/assets_api.rs` ziehen `https://assets.deadlock-api.com`:
  - `/v2/items`, `/v2/heroes?only_active=true`, `/v2/heroes`, `/raw/items`, `/raw/heroes`
  - optional auch `/v2/ranks`, `/v1/colors`, `/v2/build-tags`, `/v2/npc-units`
  - Inserts landen generisch in `entity_snapshots` mit `entity_type` `hero`, `item_or_ability`, `npc_unit` usw.
- `deadlock_api` zieht Match-Metadata von `api.deadlock-api.com`, nicht Game-Data.
- `patchnotes_db` liest eine lokale zentrale SQLite-Tabelle `changelog_posts`.
- `wiki` kann einzelne MediaWiki-Seiten per `deadlock.wiki/api.php` ziehen, ist aber standardmaessig disabled und in dieser Umgebung praktisch blockiert.
- Google-Sheet-Quellen normalisieren Stats/Rankings/Items in `hero_stat_profiles`, `hero_stat_values`, `sheet_*`.
- `dbrain-normalize` baut `entities`, `entity_aliases`, `patch_events`, `entity_lineage`, `legacy_entities` aus diesen Snapshots.

Zusatznutzen von `deadlock-data` bezogen auf das implementierte Brain:

1. **Git-auditierbare Source of Truth**  
   Statt HTTP-Einzelendpunkte ohne Repo-Kontext gibt es `HEAD`, `version.txt`, Dateipfade und Diffs pro Commit. Das passt besser fuer TRUSTED Ingestion und Reproduzierbarkeit.

2. **Explizite Scaling-Kurven**  
   Unsere Sheet-Stats haben viele Growth-/Ratio-Werte, aber `sheet_heroes_stats.spirit_scaling` ist laut Top-Down-Analyse leer. `deadlock-data` liefert Hero-`LevelScaling`, Hero-`SpiritScaling` und per Ability/Item-Card konkrete `Scale: {Value, Type}`-Objekte.

3. **Ability- und Item-Detailmodell**  
   Assets-API-Snapshots werden bisher hauptsaechlich als Rohsnapshots normalisiert. `deadlock-data` bietet bereits Wiki-ready Kartenstruktur: `Info1`, `Info2`, `Main`, `Alt`, `Move`, `Duration`, `Other`, `Upgrades`, Cooldowns, Charges, Cast-Range, Duration, Damage-Typen.

4. **Item-Kosten/Tiers/Komponenten**  
   `item-data.json` liefert `Cost`, `Tier`, `Slot`, `Activation`, `Components`, `IsImbue`, `PropertyUpgrades`; `item-component-tree.txt` liefert Upgrade-Kanten. Das fuellt die Item-Topologie fuer Build-Erklaerungen.

5. **Alias-Resolver**  
   `resource-lookup.json` liefert genau die Bruecke, die unsere Entity-/Claim-/Patch-Aufloesung braucht: Display-Name -> technischer Key -> Typ -> optional Hero fuer Abilities.

6. **Patchnotes ohne lokale Zentral-DB**  
   `raw` und `wiki` Changelogs koennen direkt als `patchnote`-Snapshots importiert werden. Das reduziert Abhaengigkeit von `deadlock_patchnotes_db` und ergaenzt Wiki-Markup/Tags.

7. **Changelog-Tags und Concepts**  
   `versions/*.json` und `tag_tree.json` koennen `patch_events.metadata_json` oder eine spaetere Concept-Bridge anreichern. Das adressiert die bekannten `general`-Event-Luecken fuer Objectives, Souls, Trooper, Map, Shop usw.

8. **Localizations**  
   Englisch und Deutsch liefern Namen, Lore, Rollen und Beschreibungen. Das kann Antworten in Deutsch verbessern und Alias-Varianten liefern, ohne deadlock.wiki direkt abzufragen.

9. **NPC-/Objective-Daten**  
   `npc-data.json` enthaelt detaillierte Werte fuer Guardian/Walker/Midboss/Trooper-nahe Einheiten. Unsere Assets-API kann `/v2/npc-units` ziehen, aber im Brain ist bisher keine tiefe Normalisierung fuer Objective-/NPC-Stats sichtbar.

Grenzen:

- `deadlock-data` ersetzt keine Match-Metadata, Builds, Statlocker-Daten oder YouTube-/Gameplay-Claims.
- `pages` liefert aktuell keine statische Strategy-/Tipps-Prosa.
- `versions/*.json` ist nicht bis zum neuesten Raw/Wiki-Changelog gepflegt.
- `convars.json` ist sehr breit und sollte nicht unkritisch in Entity-Modelle normalisiert werden.
- Viele interne/unreleased Heroes und Abilities sind enthalten; InDevelopment/IsDisabled/InHeroLabs muessen beibehalten werden.

## Integrationsdesign: neue Source `deadlock-data`

### Zielpfad und Update-Modell

Vorgeschlagener lokaler Cache:

`data/external/deadlock-data`

Der Pfad sollte in `.gitignore` stehen. Ablauf:

1. Wenn Pfad fehlt: `git clone --depth 1 https://github.com/deadlock-wiki/deadlock-data data/external/deadlock-data`.
2. Wenn Pfad existiert: `git -C data/external/deadlock-data fetch --depth 1 origin master` und Fast-Forward auf `origin/master`.
3. Vor jedem Import lesen:
   - `git rev-parse HEAD`
   - `git log -1 --format=%cI`
   - `data/version.txt`
4. Source-Run `source='deadlock-data'` mit Summary:
   - `commit_sha`
   - `commit_time`
   - `client_version`
   - gezaehlte Dateien/Records
   - `changed_files` gegen vorherigen importierten Commit, falls vorhanden.

Kein Scraping, keine Browser-Abhaengigkeit, keine Secrets.

### Source-Dokumente

Jede importierte Datei wird als `source_documents`-Eintrag gespeichert:

- `source`: `deadlock_data`
- `external_id`: z.B. `json/hero-data.json@67d2c...`
- `title`: `deadlock-data json/hero-data.json`
- `url`: `https://github.com/deadlock-wiki/deadlock-data/blob/<sha>/data/json/hero-data.json`
- `content_type`: `application/json`, `text/csv`, `text/plain`
- `metadata_json`: `{repo, commit_sha, version, file_path, source_trust:"trusted"}`

Damit bleiben Raw-Inputs auditierbar wie bei den anderen Sources.

### Entity-Snapshots

Empfohlene Snapshot-Strategie:

| Datei | Snapshot-Typ | `external_id` | `canonical_name` | Bemerkung |
|---|---|---|---|---|
| `hero-data.json` | `hero` oder `hero_internal` | Top-Level-Key `hero_atlas` | `Name` oder Localization/Lookup | `IsDisabled`, `InDevelopment`, `InHeroLabs` in Payload/Metadata behalten |
| `ability-data.json` | `ability` oder `ability_internal` | Ability-Key | `Name` oder Lookup/Localization | Hero-Link aus `ability-cards`/`resource-lookup` |
| `ability-cards.json` | `ability_card` | `<hero_key>:<slot>` oder Ability-Key | Card `Name` | zusaetzliche Snapshot-Schicht fuer UI-/Scaling-Props |
| `item-data.json` | `item` oder `item_special` | Item-Key | `Name` | `Cost`, `Tier`, `Slot`, `Components` |
| `item-cards.json` | `item_card` | Item-Key | Card `Name` | `Info1`, `Other`, `Upgrades`, `Scale` |
| `npc-data.json` | `npc_unit`/`objective` | NPC-Key | `Name` | Game-Concept-Unterstuetzung |
| `resource-lookup.json` | `resource_lookup` | Lookup-Key | `name` | Alias-/Resolver-Source |
| `localizations/*.json` | `localization` | `<lang>:<key>` oder Datei | Key/Name | eher Source-Document + Alias-Material, nicht alles als Entity |
| `changelogs/raw/*.txt` | `patchnote` | Date-Key | Config/source title | Payload muss `raw_content`, `title`, `url`, `posted_at` enthalten |
| `changelogs/wiki/*.txt` | `patchnote_wikitext` | Date-Key | Config/source title | als Wikitext-Beleg, nicht primaer fuer Parser |
| `changelogs/versions/*.json` | `patchnote_structured` | Date-Key | Date | optionale Tag-/Event-Anreicherung |

Wichtig fuer bestehende Normalisierung:

- Current entity normalizer kann `canonical_name` aus Snapshot-Spalte nutzen. Die Source sollte also nicht darauf vertrauen, dass PascalCase `Name` automatisch erkannt wird.
- `external_id` als technischer Key (`hero_atlas`, `ability_afterburn`, `upgrade_magic_reach`) wird bereits als Alias aufgenommen.
- Fuer bessere Aliase sollte die Source zusaetzlich `metadata_json` oder Payload-Felder mit `deadlock_data_key`, `lookup_names`, `localization_keys`, `hero_key` setzen; ein spaeterer Normalizer kann daraus Alias-Kinds `deadlock_data_key`, `localized_en`, `localized_de`, `resource_lookup` ableiten.

### Hero-Stats und Scaling in bestehende Tabellen

Ohne neue Tabellen kann `hero-data.json` in `hero_stat_profiles`/`hero_stat_values` abgebildet werden:

- Ein `hero_stat_profiles` pro Hero-Snapshot:
  - `snapshot_id`: der `hero` Snapshot aus `deadlock_data`
  - `entity_id`: per Alias aufgeloest
  - `hero_name`: canonical/display name
  - `source`: `deadlock_data`
  - `external_id`: `hero_atlas`
- `hero_stat_values` als EAV:
  - Basiswerte: `max_health`, `base_health_regen`, `stamina`, `sprint_speed`, `max_move_speed`, `light_melee_damage`, `heavy_melee_damage`, `bullet_resist`, `tech_resist`, ...
  - Weapon-Werte mit Prefix: `weapon.bullet_damage`, `weapon.rounds_per_second`, `weapon.clip_size`, `weapon.reload_time`, `weapon.dps`, `weapon.sustained_dps`, ...
  - Level-Scaling mit Prefix: `level_scaling.bullet_damage`, `level_scaling.max_health`, `level_scaling.tech_power`, ...
  - Spirit-Scaling mit Prefix: `spirit_scaling.clip_size`, `spirit_scaling.dps`, ...

`stat_label` sollte aus `attribute-data.json`/`stat-infobox-order.json` abgeleitet werden, wenn vorhanden. `raw_value` bleibt der JSON-Wert als String; `numeric_value` wird fuer numerische Scalars gesetzt.

Ability-/Item-Scaling passt nicht sauber in `hero_stat_values`, weil es Entity-uebergreifend ist. Ohne Migration:

- komplett im jeweiligen `entity_snapshots.payload_json` halten,
- zusaetzlich abgeleitete `entity_snapshots` Typen `ability_stat`/`item_stat` vermeiden, solange Retrieval noch nicht dafuer gebaut ist.

Mit spaeterer Migration waere eine generische Tabelle sinnvoll:

```sql
entity_stat_values(entity_id, source_snapshot_id, stat_key, stat_label, numeric_value, raw_value, scale_type, scale_value)
```

Fuer diese Aufgabe bleibt das nur Design, keine Umsetzung.

### Patchnotes in bestehende Pipeline

Bestehender Parser erwartet `entity_snapshots.entity_type='patchnote'` mit Payload-Feldern:

- `raw_content` oder `translated_content`
- `title`
- `url`
- `posted_at`

`deadlock-data` kann daher Changelogs ohne neue Patch-Pipeline einspeisen:

```json
{
  "id": "2026-05-22",
  "title": "05-22-2026 Update",
  "url": "https://forums.playdeadlock.com/threads/05-22-2026-update.135477/",
  "posted_at": "2026-05-22",
  "raw_content": "...",
  "_deadlock_data": {
    "commit_sha": "...",
    "wiki_wikitext_path": "data/changelogs/wiki/2026-05-22.txt",
    "is_hero_lab": false
  }
}
```

Dann kann `dbrain-normalize::parse_patchnotes` daraus `patch_events` erzeugen. `versions/*.json` kann danach anhand `patch_external_id/date` genutzt werden, um `patch_events.metadata_json` mit Tags zu ergaenzen oder neue Concept-Kanten vorzuschlagen.

### Mapping auf Top-Down-Modell

Zielmodell:

```text
Game
  -> Source/Audit: deadlock-data commit + version
  -> Heroes
      -> Base Stats
      -> LevelScaling / SpiritScaling
      -> Weapon
      -> Abilities
          -> Cooldowns, Charges, Range, Duration
          -> Damage/Effect Props
          -> Spirit Scale curves
          -> Upgrades
      -> Patch Changes
      -> Localized Lore/Role/Descriptions
  -> Items
      -> Cost, Tier, Slot, Activation
      -> Components / upgrade tree
      -> Props and Scaling
      -> Patch Changes
  -> NPC/Objectives/Concepts
      -> NPC Stats
      -> Changelog Tags
      -> Patch Changes
```

Konkrete Bruecken:

- `resource-lookup` und Localizations erweitern `entity_aliases`.
- `BoundAbilities` und `ability-cards` liefern Hero -> Ability Beziehungen. Ohne neue Tabelle kann die Beziehung in Ability-Snapshot-Metadaten liegen; mit spaeterer Tabelle passt sie in das im Top-Down-Doc vorgeschlagene `entity_relationships`.
- `item-component-tree.txt` und `item-data.Components` liefern Item -> Component/Upgrade Beziehungen.
- `tag_tree.json` liefert `game_concepts`-Seed-Material fuer Objective/Souls/Map/Shop/Zipline/Rejuvenator.
- `changelogs/wiki` liefert Wikitext-Belege fuer Patch-Events; `raw` bleibt Parser-Quelle.

### TRUSTED-Markierung

Da es keine bestehende `trust_level`-Spalte gibt, sollte die Markierung in `metadata_json` erfolgen:

```json
{
  "source_trust": "trusted",
  "source_origin": "deadlock-wiki/deadlock-data",
  "generated_by": "deadbot",
  "commit_sha": "...",
  "client_version": "6592"
}
```

Spaeter koennen Retrieval/Review diese Metadata lesen und Quellen priorisieren.

### Auto-Update und Diff

Import-Diff pro Run:

1. Letzten importierten `commit_sha` aus `source_runs.summary_json` suchen.
2. `git diff --name-status <old_sha>..<new_sha> -- data/...` falls beide Commits lokal vorhanden sind.
3. Falls shallow History den alten Commit nicht enthaelt: fallback auf Dateihash-Vergleich der letzten `source_documents`.
4. Nur geaenderte Dateien neu parsen; Snapshots bleiben hash-dedupliziert ueber bestehendes `payload_hash`.
5. `version.txt`-Aenderung separat reporten.

Bei Changelogs:

- neue `raw/*.txt` als neue `patchnote`-Snapshots importieren,
- geaenderte bestehende Patchdateien als neue Snapshot-Version mit gleichem `external_id`, neuem `payload_hash`,
- `patch_events` nur rebuilden oder inkrementell nach Snapshot-ID einfuegen; bestehende `UNIQUE(event_hash)` verhindert Duplikate.

### Aufwand

Grobe Schaetzung:

| Phase | Aufwand | Inhalt |
|---|---:|---|
| 1. Git-Source + Dokumentimport | 0.5-1 Tag | Clone/pull, Version/Commit lesen, Source-Documents, Run-Summary |
| 2. Core-Snapshots | 1-1.5 Tage | Heroes, Abilities, Items, Cards, Resource Lookup, Localizations, Changelogs |
| 3. Hero-Stats EAV | 0.5-1 Tag | `hero_stat_profiles`/`hero_stat_values` aus `hero-data` |
| 4. Patchnote-Anbindung | 0.5-1 Tag | Raw Changelogs als `patchnote`, optional Wiki-Wikitext-Dokumente |
| 5. Alias-/Relationship-Anreicherung | 1-2 Tage | `resource-lookup`, Localizations, BoundAbilities, Components; ggf. ohne Migration erstmal Metadata |
| 6. Tests/Quality | 1 Tag | Temp-DB-Tests, Parser-Snapshots, Alias-Matching, keine Live-DB |

Minimal nutzbare Version: 2-3 Tage. Vollere Top-Down-Anreicherung mit Beziehungen/Concepts: 4-6 Tage plus Review fuer Schemaentscheidungen.

## Empfohlene Reihenfolge

1. `deadlock_data` Source in `dbrain-sources` additiv implementieren, aber zunaechst nur Dokumente und Snapshots.
2. `entity_normalizer`/Rust-Normalizer nur so erweitern, dass PascalCase/Metadata-Aliase sauber verarbeitet werden.
3. `hero_stat_profiles`/`hero_stat_values` aus `hero-data` generieren.
4. `changelogs/raw` als `patchnote`-Snapshots einspeisen und bestehenden Parser wiederverwenden.
5. `resource-lookup` als Alias- und Ability-Hero-Bruecke nutzen.
6. Erst danach entscheiden, ob eine generische `entity_stat_values` und `entity_relationships` Migration noetig ist.

## Offene Risiken

- `pages` ist kein statischer Prosa-Datenbestand. Hero-Tipps/Combos bleiben ohne Wiki-Zugriff oder anderen Dump offen.
- `deadlock-data` enthaelt interne/unreleased Inhalte; Retrieval muss aktive Inhalte priorisieren.
- PascalCase-Felder erfordern explizites Mapping, sonst verschenken wir Aliase und Canonical Names.
- `versions/*.json` ist unvollstaendig fuer 2026-02 bis 2026-05; Tags duerfen nicht als vollstaendige Patch-Historie gelten.
- Item-Komponentenbaum nutzt teils gekuerzte Keys; Mapping braucht Fallback-Normalisierung.
