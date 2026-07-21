---
title: "Shrink Ray"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_shrink_ray"
canonical_name: "Shrink Ray"
snapshot_id: 40451
source_document_id: 7073
payload_hash: "ff5851e300606dd0fef3efb6871c56737b5b442a515c97107cf7450e1d69baeb"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.614704+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Shrink Ray

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_shrink_ray`
- Snapshot ID: `40451`
- Source-Dokument: `7073`
- Kurzinfo: Shrink Ray aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 9999,
  "Description": "Reduces <span class=\"highlight\">Model Size</span> and grants <span class=\"highlight\">Move Speed</span> to the target. Allows <span class=\"highlight\">usage of tunnels</span> in this mode. Can be self-cast.",
  "Info1": {
    "Alt": [
      {
        "Key": "ShrinkDuration",
        "Type": "duration",
        "Value": 60
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "40m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 30,
    "DescKey": "#upgrade_shrink_ray_desc",
    "Main": [
      {
        "Key": "ModelScaleGrowthTooltip",
        "Value": -50
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "5.0m"
      },
      {
        "Key": "BonusFireRate",
        "Value": 20
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_shrink_ray",
  "Name": "Shrink Ray",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ModelScaleGrowth": {
      "Key": "ModelScaleGrowth",
      "Value": 0.5
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -15,
    "BonusFireRate": 20,
    "BonusMoveSpeed": "2m",
    "ModelScaleGrowth": -0.15,
    "ModelScaleGrowthTooltip": -15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
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
      "lookup": "shrink ray",
      "name": "Shrink Ray",
      "type": "item"
    }
  ]
}
````
