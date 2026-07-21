---
title: "Colossus"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_colossus"
canonical_name: "Colossus"
snapshot_id: 40036
source_document_id: 7072
payload_hash: "62a488d46877fac3c7664bbeea9742357c4ea43d75a6d85ee04c56668d67ac2f"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.717315+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Colossus

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_colossus`
- Snapshot ID: `40036`
- Source-Dokument: `7072`
- Kurzinfo: Colossus aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 37.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BaseAttackDamagePercent": 15,
  "BonusBaseHealth": 25,
  "BonusMeleeDamagePercent": 30,
  "BuffBulletResist": 35,
  "BuffTechResist": 35,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_health"
  ],
  "Cost": 6400,
  "Description": "Grow <span class=\"highlight\">larger in size</span>, gaining {g:citadel_inline_attribute:'BulletResist'}, {g:citadel_inline_attribute:'SpiritResist'}, and {g:citadel_inline_attribute:'MeleeDamage'}. <br><br>Nearby enemies suffer from {g:citadel_inline_attribute:'Slow'} and have reduced <span class=\"highlight\">dash speed</span>.",
  "GroundDashReductionPercent": -25,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_colossus",
  "ModelScaleGrowth": 1.2,
  "ModelScaleGrowthTooltip": 20,
  "Name": "Colossus",
  "PropertyUpgrades": {
    "AbilityCooldown": -7,
    "BonusBaseHealth": 15,
    "BuffBulletResist": 10,
    "BuffTechResist": 10,
    "ModelScaleGrowth": 0.2,
    "ModelScaleGrowthTooltip": 20
  },
  "Radius": "14m",
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "SlowPercent": 30,
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
      "lookup": "colossus",
      "name": "Colossus",
      "type": "item"
    }
  ]
}
````
