---
title: "Fury Trance"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_fury_trance"
canonical_name: "Fury Trance"
snapshot_id: 40349
source_document_id: 7073
payload_hash: "00da32b27e117c24885fcb18d2bc0b5f3c15418f3bdefb5a3edecff735f1c32f"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.441092+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Fury Trance

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_fury_trance`
- Snapshot ID: `40349`
- Source-Dokument: `7073`
- Kurzinfo: Fury Trance aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_vampire"
  ],
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BulletLifestealPercent",
        "Type": "healing",
        "Value": 14
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 100
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 6
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
        "Value": 6.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 18.0,
    "DescKey": "#upgrade_fury_trance_active",
    "Main": [
      {
        "Key": "ActiveBonusFireRate",
        "Type": "fire_rate",
        "Value": 32
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 40
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_fury_trance",
  "Name": "Fury Trance",
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
    "FireRate",
    "WeaponDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 3,
  "Upgrades": {
    "ActiveBonusFireRate": 25,
    "BonusHealth": 110,
    "BulletLifestealPercent": 28,
    "TechResist": 20
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
      "lookup": "fury trance",
      "name": "Fury Trance",
      "type": "item"
    }
  ]
}
````
