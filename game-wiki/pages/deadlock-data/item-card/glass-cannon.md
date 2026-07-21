---
title: "Glass Cannon"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_glass_cannon"
canonical_name: "Glass Cannon"
snapshot_id: 40351
source_document_id: 7073
payload_hash: "17e66f93970e754dfaac86b419bd5731901d54b709a5790d45ac1a5643a53db8"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.444090+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Glass Cannon

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_glass_cannon`
- Snapshot ID: `40351`
- Source-Dokument: `7073`
- Kurzinfo: Glass Cannon aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Each hero kill grants <span class=\"highlight\">permanent Fire Rate</span> (up to a max of 8 times). Death results in the loss of 1 stack.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 80
      },
      {
        "Key": "MaxHealthLossPercent",
        "Type": "health",
        "Value": -13
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
        "Key": "FireRatePerKill",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 7
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_glass_cannon",
  "Name": "Glass Cannon",
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
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 2
    },
    "BuildUpPerShot": {
      "Key": "BuildUpPerShot",
      "Value": 1.2
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MaxStacks": {
      "Key": "MaxStacks",
      "Value": 8
    },
    "SlowDuration": {
      "Key": "SlowDuration",
      "Type": "duration",
      "Value": 3
    },
    "SlowPercent": {
      "Key": "SlowPercent",
      "LocTokenOverride": "GlassCannon_SlowPercent",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": 30
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BaseAttackDamagePercent": 60,
    "FireRatePerKill": 8
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
      "lookup": "glass cannon",
      "name": "Glass Cannon",
      "type": "item"
    }
  ]
}
````
