---
title: "Bullet Resist Shredder"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_bullet_resist_shredder"
canonical_name: "Bullet Resist Shredder"
snapshot_id: 40291
source_document_id: 7073
payload_hash: "849ac7ae40dfe0813e0883e5815bb359b6902a62ef464e1b13602e5447a8a7c1"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.335957+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Bullet Resist Shredder

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_bullet_resist_shredder`
- Snapshot ID: `40291`
- Source-Dokument: `7073`
- Kurzinfo: Bullet Resist Shredder aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Reduces <span class=\"highlight\">Bullet Resist</span> on enemies when you deal {g:citadel_inline_attribute:'SpiritDamage'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 9
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 9
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
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_bullet_resist_shredder_desc",
    "Main": [
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -10
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_bullet_resist_shredder",
  "Name": "Bullet Resist Shredder",
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
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BulletArmorReduction": -11,
    "BulletResist": 7
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
      "lookup": "bullet resist shredder",
      "name": "Bullet Resist Shredder",
      "type": "item"
    }
  ]
}
````
