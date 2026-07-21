---
title: "Express Shot"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_express_shot"
canonical_name: "Express Shot"
snapshot_id: 40341
source_document_id: 7073
payload_hash: "49bd8d17eee4d0baae6f785fde1e6c81426af3f6c90fc1d1850ca9e7aeddecff"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.426355+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Express Shot

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_express_shot`
- Snapshot ID: `40341`
- Source-Dokument: `7073`
- Kurzinfo: Express Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 3200,
  "Description": "Your next attack will <span class=\"highlight\">fire twice</span> in quick succession with <span class=\"highlight\">increased damage</span> and velocity. This attack consumes extra ammo.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Type": "bullet_damage",
        "Value": 60
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ProcBulletVelocity",
        "LocTokenOverride": "BonusBulletSpeedPercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 100
      },
      {
        "Key": "ProcAmmoConsumed",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": 8,
    "DescKey": "#upgrade_express_shot_desc",
    "Main": [
      {
        "Key": "ProcBaseAttackDamagePercent",
        "LocTokenOverride": "BaseAttackDamagePercent",
        "Scale": {
          "Type": "power_increase",
          "Value": 2.0
        },
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 125.0
      },
      {
        "Key": "ProcBaseAttackDamagePercentAltFire",
        "LocTokenOverride": "BaseAttackDamagePercentAltFire",
        "Scale": {
          "Type": "power_increase",
          "Value": 1.3
        },
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_express_shot",
  "Name": "Express Shot",
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
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusBulletSpeedPercent": 45,
    "ProcBaseAttackDamagePercent": 75,
    "ProcBaseAttackDamagePercentAltFire": 25
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
      "lookup": "express shot",
      "name": "Express Shot",
      "type": "item"
    }
  ]
}
````
