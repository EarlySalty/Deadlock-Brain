---
title: "Rebuttal"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_melee_rebuttal"
canonical_name: "Rebuttal"
snapshot_id: 40405
source_document_id: 7073
payload_hash: "6721b88d9b1e0d1eb222262962d0c12bc2afa1940860343bca4673bd9f104d28"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.535786+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Rebuttal

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_melee_rebuttal`
- Snapshot ID: `40405`
- Source-Dokument: `7073`
- Kurzinfo: Rebuttal aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "On a successful <span class=\"highlight\">Parry</span> against an enemy Hero, <span class=\"highlight\">Heal</span> yourself for the damage parried and returns that damage to the target, and temporarily gain increased <span class=\"highlight\">damage.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "ParryCooldownReduction",
        "Type": "cooldown",
        "Value": 1.75
      },
      {
        "Key": "MeleeResistPercent",
        "Value": 18
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
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
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_melee_rebuttal_desc",
    "Main": [
      {
        "Key": "BonusDamagePercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_melee_rebuttal",
  "Name": "Rebuttal",
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
    "ParrySuccessHealPercentage": {
      "Key": "ParrySuccessHealPercentage",
      "Type": "healing",
      "Value": 100
    }
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "Upgrades": {
    "BonusDamagePercent": 20,
    "BonusHealth": 150,
    "MeleeResistPercent": 22,
    "ParryCooldownReduction": 0.5
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
      "lookup": "rebuttal",
      "name": "Rebuttal",
      "type": "item"
    }
  ]
}
````
