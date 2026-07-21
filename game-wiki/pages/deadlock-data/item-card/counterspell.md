---
title: "Counterspell"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_counterspell"
canonical_name: "Counterspell"
snapshot_id: 40319
source_document_id: 7073
payload_hash: "7d1f4f6c5656db2367c36c1b1be7936bbf53b0e6094557bf8660055c8332b803"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.383622+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Counterspell

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_counterspell`
- Snapshot ID: `40319`
- Source-Dokument: `7073`
- Kurzinfo: Counterspell aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Your next parry <span class=\"highlight\">protects you from the damage and effects of enemy abilities and items</span>. On a successful spell parry {g:citadel_inline_attribute:'Heal'} and gain {g:citadel_inline_attribute:'MoveSpeed'} and {g:citadel_inline_attribute:'Spirit'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      },
      {
        "Key": "SpiritPowerInnate",
        "Type": "tech_damage",
        "Value": 5
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
      },
      {
        "Key": "SpellParryDuration",
        "Type": "duration",
        "Value": 0.8
      }
    ],
    "ChargeUp": null,
    "Cooldown": 23.0,
    "DescKey": "#upgrade_counterspell_desc",
    "Main": [
      {
        "Key": "HealOnSuccess",
        "LocTokenOverride": "HealOnCounterSpell",
        "Type": "healing",
        "UsageFlags": "ConditionallyApplied",
        "Value": 150
      },
      {
        "Key": "SpiritPower",
        "Type": "tech_power",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.75m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_counterspell",
  "Name": "Counterspell",
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
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 150,
    "BonusMoveSpeed": "2m",
    "HealOnSuccess": 250,
    "SpiritPower": 20
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
      "lookup": "counterspell",
      "name": "Counterspell",
      "type": "item"
    }
  ]
}
````
