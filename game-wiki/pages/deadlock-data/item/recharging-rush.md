---
title: "Recharging Rush"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_rechargingbullets"
canonical_name: "Recharging Rush"
snapshot_id: 40151
source_document_id: 7072
payload_hash: "30ee0989ec48275b32468059158cc16dcff250a849f503d9c515c600427b8ca0"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.017067+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Recharging Rush

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_rechargingbullets`
- Snapshot ID: `40151`
- Source-Dokument: `7072`
- Kurzinfo: Recharging Rush aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 25,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 10,
  "BonusClipSizePercent": 20,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "DamageThreshold": 200,
  "DamageWindow": 3.5,
  "Description": "Dealing significant {g:citadel_inline_attribute:'WeaponDamage'} replenishes a charge for <span class=\"highlight\">each of your charged abilities</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rechargingbullets",
  "Name": "Recharging Rush",
  "PropertyUpgrades": {
    "AbilityCooldown": -12,
    "BaseAttackDamagePercent": 30,
    "BonusClipSizePercent": 30
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Weapon",
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
      "lookup": "recharging rush",
      "name": "Recharging Rush",
      "type": "item"
    }
  ]
}
````
