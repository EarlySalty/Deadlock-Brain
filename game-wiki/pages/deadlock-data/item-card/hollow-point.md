---
title: "Hollow Point"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_hollow_point_rounds"
canonical_name: "Hollow Point"
snapshot_id: 40375
source_document_id: 7073
payload_hash: "c2dce8557ce3704e77c0ff71a8dfc5c6e263a1c0b195b4e9ddf42453efc6cc84"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.485272+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Hollow Point

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_hollow_point_rounds`
- Snapshot ID: `40375`
- Source-Dokument: `7073`
- Kurzinfo: Hollow Point aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "When you are <span class=\"highlight\">above 65% health</span>, deal additional <span class=\"highlight\">Weapon Damage</span> and your bullets reduce enemy <span class=\"highlight\">Bullet Resist</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 4.5
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
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
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_hollow_point_rounds_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      },
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -9
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_hollow_point_rounds",
  "Name": "Hollow Point",
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
    "LifeThreshold": {
      "Key": "LifeThreshold",
      "Value": 65
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
  "Tier": 3,
  "Upgrades": {
    "BaseAttackDamagePercent": 25,
    "BonusHealth": 150,
    "BulletArmorReduction": -12
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
      "lookup": "hollow point",
      "name": "Hollow Point",
      "type": "item"
    }
  ]
}
````
