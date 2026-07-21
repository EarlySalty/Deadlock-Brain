---
title: "Lucky Shot"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_critshot"
canonical_name: "Lucky Shot"
snapshot_id: 40042
source_document_id: 7072
payload_hash: "d053c5d34a05bd391ed4ebcd36932387ffbfb61c737b559f08171406abacac62"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.734111+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Lucky Shot

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_critshot`
- Snapshot ID: `40042`
- Source-Dokument: `7072`
- Kurzinfo: Lucky Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusClipSizePercent": 30,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "CritDamagePercent": 100,
  "Description": "Your bullets have a chance to be empowered, causing them to deal <span class=\"highlight\">bonus weapon damage</span> on hit.<br><span class=\"diminish\">Bonus damage cannot Crit.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_critshot",
  "Name": "Lucky Shot",
  "ProcChance": 25,
  "PropertyUpgrades": {
    "BonusClipSizePercent": 40,
    "CritDamagePercent": 30,
    "ProcChance": 5
  },
  "Radius": "1m",
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-data.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "lucky shot",
      "name": "Lucky Shot",
      "type": "item"
    }
  ]
}
````
