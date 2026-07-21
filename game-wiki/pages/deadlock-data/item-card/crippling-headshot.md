---
title: "Crippling Headshot"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_banshee_slugs"
canonical_name: "Crippling Headshot"
snapshot_id: 40279
source_document_id: 7073
payload_hash: "f5671628d76c047b433aac8d38d4e426e44f6fa17f6db0ab8f343bf31217b6cf"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.315853+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Crippling Headshot

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_banshee_slugs`
- Snapshot ID: `40279`
- Source-Dokument: `7073`
- Kurzinfo: Crippling Headshot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_headshot_booster2"
  ],
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
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
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_banshee_slugs_headshots_desc",
    "Main": [
      {
        "Key": "BulletResistReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -16
      },
      {
        "Key": "MagicResistReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -16
      },
      {
        "Key": "HealAmpRegenPenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -35
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_banshee_slugs",
  "Name": "Crippling Headshot",
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
    "DiminishingMultiplier": {
      "Key": "DiminishingMultiplier",
      "Value": 0.5
    },
    "HealAmpReceivePenaltyPercent": {
      "Key": "HealAmpReceivePenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -35
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusHealth": 150,
    "BulletResistReduction": -12,
    "HealAmpReceivePenaltyPercent": -25,
    "HealAmpRegenPenaltyPercent": -25,
    "MagicResistReduction": -12
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
      "lookup": "crippling headshot",
      "name": "Crippling Headshot",
      "type": "item"
    }
  ]
}
````
