---
title: "Battle Vest"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_regenerating_bullet_shield"
canonical_name: "Battle Vest"
snapshot_id: 40432
source_document_id: 7073
payload_hash: "43e2d83db4d17fec71623c037f99686ab3eea0a84843b4168a9b4ab3bce00550"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.581905+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Battle Vest

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_regenerating_bullet_shield`
- Snapshot ID: `40432`
- Source-Dokument: `7073`
- Kurzinfo: Battle Vest aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "While you are <span class=\"highlight\">above 65% health</span>, gain <span class=\"highlight\">{g:citadel_inline_attribute:'WeaponDamage'}</span> and <span class=\"highlight\">{g:citadel_inline_attribute:'BonusFireRate'}</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 18
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_regenerating_bullet_shield_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 18
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 7
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_regenerating_bullet_shield",
  "Name": "Battle Vest",
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
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusFireRate": 8,
    "BulletResist": 12,
    "OutOfCombatHealthRegen": 3
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
      "lookup": "battle vest",
      "name": "Battle Vest",
      "type": "item"
    }
  ]
}
````
