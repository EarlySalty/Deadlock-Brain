---
title: "Metal Skin"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_metal_skin"
canonical_name: "Metal Skin"
snapshot_id: 40406
source_document_id: 7073
payload_hash: "7c72a925f4888c8eede83c67421b4cc3892ce922a7f95afda6ebdccab1c947f0"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.537344+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Metal Skin

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_metal_skin`
- Snapshot ID: `40406`
- Source-Dokument: `7073`
- Kurzinfo: Metal Skin aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "Become <span class=\"highlight\">immune to bullets</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ActiveMoveSpeedPenalty",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "-1.5m"
      },
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -20
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5.0
      }
    ],
    "ChargeUp": null,
    "Cooldown": 24.0,
    "DescKey": "#upgrade_metal_skin_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_metal_skin",
  "Name": "Metal Skin",
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
    }
  },
  "ShopFilters": [
    "Durability",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -2,
    "ActiveMoveSpeedPenalty": "6.5m",
    "BulletResist": 5,
    "GroundDashReductionPercent": 60
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
      "lookup": "metal skin",
      "name": "Metal Skin",
      "type": "item"
    }
  ]
}
````
