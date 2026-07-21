---
title: "Armor Piercing Rounds"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_aprounds"
canonical_name: "Armor Piercing Rounds"
snapshot_id: 39991
source_document_id: 7072
payload_hash: "0ea6359f2250329ce4e4baf27a0bb0a6db7b99125d897880d42a24e710271311"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.618746+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Armor Piercing Rounds

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_aprounds`
- Snapshot ID: `39991`
- Source-Dokument: `7072`
- Kurzinfo: Armor Piercing Rounds aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 8,
  "BonusBulletSpeedPercent": 60,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 6400,
  "Description": "Your Bullets have a chance to become unavoidable, <span class=\"highlight\">piercing through</span> enemies and <span class=\"highlight\">ignoring their Bullet Resistance</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_aprounds",
  "Name": "Armor Piercing Rounds",
  "ProcChance": 55,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 30,
    "BonusBulletSpeedPercent": 55,
    "ProcChance": 20
  },
  "ShopFilters": [
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "armor piercing rounds",
      "name": "Armor Piercing Rounds",
      "type": "item"
    }
  ]
}
````
