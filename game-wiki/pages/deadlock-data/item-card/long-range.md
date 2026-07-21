---
title: "Long Range"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_long_range"
canonical_name: "Long Range"
snapshot_id: 40390
source_document_id: 7073
payload_hash: "8a8c7f4af8f16e7a9e39b4f3c773b5ef932e3a11fe3edae8d07c9cffbfaa4991"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.511012+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Long Range

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_long_range`
- Snapshot ID: `40390`
- Source-Dokument: `7073`
- Kurzinfo: Long Range aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Deal additional <span class=\"highlight\">Weapon Damage</span> when <span class=\"highlight\">beyond a minimum distance</span> from your target.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusAttackRangePercent",
        "Type": "distance",
        "Value": 8
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
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
        "Key": "LongRangeBonusWeaponPowerMinRange",
        "Type": "distance",
        "Value": "15m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_long_range_desc",
    "Main": [
      {
        "Key": "LongRangeBonusWeaponPower",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_long_range",
  "Name": "Long Range",
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
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusAttackRangePercent": 8,
    "LongRangeBonusWeaponPower": 30
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
      "lookup": "long range",
      "name": "Long Range",
      "type": "item"
    }
  ]
}
````
