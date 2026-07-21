---
title: "upgrade_nearby_enemy_boost"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_nearby_enemy_boost"
canonical_name: "upgrade_nearby_enemy_boost"
snapshot_id: 40411
source_document_id: 7073
payload_hash: "7084c37d1aebf6febea12e29d8e2acfd879ec365e76133d09f828a499c1cdbb7"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.546514+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_nearby_enemy_boost

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_nearby_enemy_boost`
- Snapshot ID: `40411`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_nearby_enemy_boost aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "1m"
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
        "Key": "Radius",
        "Type": "distance",
        "Value": "20m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_nearby_enemy_boost_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 48
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": null
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_nearby_enemy_boost",
  "Name": null,
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
    "MinTargetsRequired": {
      "Key": "MinTargetsRequired",
      "Value": 2
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "FireRate",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
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
  }
}
````
