---
title: "Opening Rounds"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_pristine_emblem"
canonical_name: "Opening Rounds"
snapshot_id: 40421
source_document_id: 7073
payload_hash: "733a12e9388d6027a441bbdaf67e36ab8e5dc2d5881f2c2db2b305f8826891d0"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.563111+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Opening Rounds

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_pristine_emblem`
- Snapshot ID: `40421`
- Source-Dokument: `7073`
- Kurzinfo: Opening Rounds aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 1600,
  "Description": "Your attacks have additional <span class=\"highlight\">Weapon Damage</span> against <span class=\"highlight\">enemies above 50% health</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Type": "bullet_damage",
        "Value": 60
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 8
      },
      {
        "Key": "TechPower",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_pristine_emblem_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercentBonus",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_pristine_emblem",
  "Name": "Opening Rounds",
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
    "EnemyLifeThreshold": {
      "Key": "EnemyLifeThreshold",
      "Value": 50
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BaseAttackDamagePercentBonus": 25,
    "BonusBulletSpeedPercent": 45,
    "TechPower": 18
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
      "lookup": "opening rounds",
      "name": "Opening Rounds",
      "type": "item"
    }
  ]
}
````
