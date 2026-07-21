---
title: "Weighted Shots"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_weighted_shots"
canonical_name: "Weighted Shots"
snapshot_id: 40505
source_document_id: 7073
payload_hash: "e25eb987bc4a171a0b7dfbaa81a3fc62ade854efc24eddb9fec0359d3a8bb967"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.728534+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Weighted Shots

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weighted_shots`
- Snapshot ID: `40505`
- Source-Dokument: `7073`
- Kurzinfo: Weighted Shots aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_slowing_bullets"
  ],
  "Cost": 3200,
  "Description": "Your bullets build up a <span class=\"highlight\">Movement Slow</span> on enemies.",
  "Info1": {
    "Alt": [
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 22
      },
      {
        "Key": "StaminaCooldownReduction",
        "Value": -14
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "-0.5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 40
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -22
      },
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 3.5
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 0.7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_weighted_shots_desc",
    "Main": [
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weighted_shots",
  "Name": "Weighted Shots",
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
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BaseAttackDamagePercent": 35,
    "GroundDashReductionPercent": -10,
    "SlowPercent": 20,
    "StatusResistancePercent": 10
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
      "lookup": "weighted shots",
      "name": "Weighted Shots",
      "type": "item"
    }
  ]
}
````
