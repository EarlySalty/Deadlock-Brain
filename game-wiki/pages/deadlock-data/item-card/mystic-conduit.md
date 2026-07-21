---
title: "Mystic Conduit"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_patrons_blessing"
canonical_name: "Mystic Conduit"
snapshot_id: 40416
source_document_id: 7073
payload_hash: "0e3959b249dcb8293856880180818e979d24aa2d381420754bb2d509afd1ca7b"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.554967+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Mystic Conduit

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_patrons_blessing`
- Snapshot ID: `40416`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Conduit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Provides yourself and allies with a powerful spirit focused aura. Has reduced values on allies.",
  "Info1": {
    "Alt": [
      {
        "Key": "AllyPercentage",
        "Value": 50
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "25m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_patrons_blessing_desc",
    "Main": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 40
      },
      {
        "Key": "CooldownReduction",
        "Type": "cooldown",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamageThreshold",
        "Type": "tech_armor_up",
        "Value": 300
      },
      {
        "Key": "DamageThresholdDuration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 25,
    "DescKey": "#upgrade_patrons_blessing_proc_desc",
    "Main": [
      {
        "Key": "HealAmount",
        "Type": "healing",
        "Value": 700
      },
      {
        "Key": "HealRadius",
        "Type": "distance",
        "Value": "35m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_patrons_blessing",
  "Name": "Mystic Conduit",
  "Other": {
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
    "TechPower": {
      "Key": "TechPower",
      "Value": 40
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 40
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllFriendly"
  ],
  "Tier": 5,
  "Upgrades": {
    "CooldownReduction": 10,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
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
      "lookup": "mystic conduit",
      "name": "Mystic Conduit",
      "type": "item"
    }
  ]
}
````
