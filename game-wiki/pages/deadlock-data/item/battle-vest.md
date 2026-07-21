---
title: "Battle Vest"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_regenerating_bullet_shield"
canonical_name: "Battle Vest"
snapshot_id: 40153
source_document_id: 7072
payload_hash: "1378260fa1488897f73a97cd17396c9e0f28a856b3062e3d4918ad7f4ace2865"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.022602+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Battle Vest

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_regenerating_bullet_shield`
- Snapshot ID: `40153`
- Source-Dokument: `7072`
- Kurzinfo: Battle Vest aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 18,
  "BonusFireRate": 7,
  "BulletResist": 18,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "While you are <span class=\"highlight\">above 65% health</span>, gain <span class=\"highlight\">{g:citadel_inline_attribute:'WeaponDamage'}</span> and <span class=\"highlight\">{g:citadel_inline_attribute:'BonusFireRate'}</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_regenerating_bullet_shield",
  "LifeThreshold": 65,
  "Name": "Battle Vest",
  "OutOfCombatHealthRegen": 3,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusFireRate": 8,
    "BulletResist": 12,
    "OutOfCombatHealthRegen": 3
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "battle vest",
      "name": "Battle Vest",
      "type": "item"
    }
  ]
}
````
