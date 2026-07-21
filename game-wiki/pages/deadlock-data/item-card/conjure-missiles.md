---
title: "Conjure Missiles"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_magic_missile"
canonical_name: "Conjure Missiles"
snapshot_id: 40395
source_document_id: 7073
payload_hash: "fadc347768b9c7f571e9468533c1c932f3db9d17a49c87152ae470d531657fed"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.518815+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Conjure Missiles

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_missile`
- Snapshot ID: `40395`
- Source-Dokument: `7073`
- Kurzinfo: Conjure Missiles aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 70
      },
      {
        "Key": "BonusFireRate",
        "Value": 10
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "1m"
      },
      {
        "Key": "MaxHealthLossPercent",
        "Type": "health",
        "Value": -15
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
    "DescKey": "#upgrade_glass_cannon_desc",
    "Main": [
      {
        "Key": "WeaponPowerPerKill",
        "UsageFlags": "ConditionallyApplied",
        "Value": 10
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_magic_missile",
  "Name": "Conjure Missiles",
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
    "BonusClipPerKill": {
      "Key": "BonusClipPerKill",
      "Value": 2
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MaxClipBonus": {
      "Key": "MaxClipBonus",
      "Value": 14
    },
    "MaxStacks": {
      "Key": "MaxStacks",
      "Value": 5
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
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
