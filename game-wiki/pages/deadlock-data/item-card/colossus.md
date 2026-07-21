---
title: "Colossus"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_colossus"
canonical_name: "Colossus"
snapshot_id: 40315
source_document_id: 7073
payload_hash: "6a506dca26e2b44b2c13b634ceacd296d00d3961d3a53029222f11996f24453f"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.375231+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Colossus

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_colossus`
- Snapshot ID: `40315`
- Source-Dokument: `7073`
- Kurzinfo: Colossus aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_health"
  ],
  "Cost": 6400,
  "Description": "Grow <span class=\"highlight\">larger in size</span>, gaining {g:citadel_inline_attribute:'BulletResist'}, {g:citadel_inline_attribute:'SpiritResist'}, and {g:citadel_inline_attribute:'MeleeDamage'}. <br><br>Nearby enemies suffer from {g:citadel_inline_attribute:'Slow'} and have reduced <span class=\"highlight\">dash speed</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusBaseHealth",
        "Type": "health",
        "Value": 25
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "14m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 7
      },
      {
        "Key": "ModelScaleGrowthTooltip",
        "Value": 20
      }
    ],
    "ChargeUp": null,
    "Cooldown": 37.0,
    "DescKey": "#upgrade_colossus_desc",
    "Main": [
      {
        "Key": "BuffBulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      },
      {
        "Key": "BuffTechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      },
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 30
      },
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_colossus",
  "Name": "Colossus",
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
    "GroundDashReductionPercent": {
      "Key": "GroundDashReductionPercent",
      "UsageFlags": "ConditionallyApplied",
      "Value": -25
    },
    "ModelScaleGrowth": {
      "Key": "ModelScaleGrowth",
      "Value": 1.2
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -7,
    "BonusBaseHealth": 15,
    "BuffBulletResist": 10,
    "BuffTechResist": 10,
    "ModelScaleGrowth": 0.2,
    "ModelScaleGrowthTooltip": 20
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
      "lookup": "colossus",
      "name": "Colossus",
      "type": "item"
    }
  ]
}
````
