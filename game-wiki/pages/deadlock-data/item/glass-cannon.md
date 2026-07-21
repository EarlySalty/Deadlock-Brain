---
title: "Glass Cannon"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_glass_cannon"
canonical_name: "Glass Cannon"
snapshot_id: 40072
source_document_id: 7072
payload_hash: "3e1e8b17cbd42bbc2989f5d245f9b44e565af0f8e79d5a8c2486b82a02eba812"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.803309+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Glass Cannon

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_glass_cannon`
- Snapshot ID: `40072`
- Source-Dokument: `7072`
- Kurzinfo: Glass Cannon aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 80,
  "BonusClipPerKill": 2,
  "BuildUpDuration": 2,
  "BuildUpPerShot": 1.2,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "Each hero kill grants <span class=\"highlight\">permanent Fire Rate</span> (up to a max of 8 times). Death results in the loss of 1 stack.",
  "FireRatePerKill": 7,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_glass_cannon",
  "MaxHealthLossPercent": -13,
  "MaxStacks": 8,
  "Name": "Glass Cannon",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 60,
    "FireRatePerKill": 8
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize",
    "FireRate"
  ],
  "Slot": "Weapon",
  "SlowDuration": 3,
  "SlowPercent": 30,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
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
      "lookup": "glass cannon",
      "name": "Glass Cannon",
      "type": "item"
    }
  ]
}
````
