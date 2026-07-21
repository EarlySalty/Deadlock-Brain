---
title: "Trophy Collector"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_trophy_collector"
canonical_name: "Trophy Collector"
snapshot_id: 40211
source_document_id: 7072
payload_hash: "9b412b522ae6c61493dbab47d2457114d5cbafcd35ff1e4e2b16a1d5045f16e3"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.181126+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Trophy Collector

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_trophy_collector`
- Snapshot ID: `40211`
- Source-Dokument: `7072`
- Kurzinfo: Trophy Collector aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusSprintSpeed": "2.0m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_sprint_booster"
  ],
  "Cost": 1600,
  "Description": "Whenever you score an <span class=\"highlight\">assist or kill</span>, gain extra <span class=\"highlight\">sprint</span>, <span class=\"highlight\">ability range</span> and <span class=\"highlight\">passive soul generation</span>. This effect stacks and persists through death.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_trophy_collector",
  "MaxStacks": 16,
  "Name": "Trophy Collector",
  "NonPlayerBonusWeaponPower": -15,
  "OutOfCombatHealthRegen": 2,
  "PropertyUpgrades": {
    "BonusSprintSpeed": "12m",
    "MaxStacks": 83,
    "OutOfCombatHealthRegen": 6,
    "StackingTechRadiusMultiplier": 3,
    "StackingTechRangeMultiplier": 3
  },
  "ShopFilters": [
    "WeaponDamage",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StackingBonusSprintSpeed": "0.15m",
  "StackingGoldPerMinute": 18,
  "StackingTechRadiusMultiplier": 0.75,
  "StackingTechRangeMultiplier": 0.75,
  "StreetBrawl": false,
  "TargetTypes": null,
  "ThinkRate": 3,
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
      "lookup": "trophy collector",
      "name": "Trophy Collector",
      "type": "item"
    }
  ]
}
````
