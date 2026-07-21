---
title: "Spellbreaker"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_spellbreaker"
canonical_name: "Spellbreaker"
snapshot_id: 40459
source_document_id: 7073
payload_hash: "58329bb5856513e6097f734a4e387449149db2357207d10f7162cdf7926b5788"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.629004+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Spellbreaker

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spellbreaker`
- Snapshot ID: `40459`
- Source-Dokument: `7073`
- Kurzinfo: Spellbreaker aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_debuff_reducer"
  ],
  "Cost": 6400,
  "Description": "The next instance of high {g:citadel_inline_attribute:'SpiritDamage'} you take is significantly reduced.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 18
      },
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 25
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamageThreshold",
        "Type": "tech_damage",
        "Value": 175
      }
    ],
    "ChargeUp": null,
    "Cooldown": 9,
    "DescKey": "#upgrade_spellbreaker_desc",
    "Main": [
      {
        "Key": "SpiritDamageReductionProc",
        "Type": "tech_armor_up",
        "Value": 65
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spellbreaker",
  "Name": "Spellbreaker",
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
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -3,
    "StatusResistancePercent": 15,
    "TechResist": 15
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
      "lookup": "spellbreaker",
      "name": "Spellbreaker",
      "type": "item"
    }
  ]
}
````
