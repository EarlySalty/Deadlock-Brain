---
title: "Witchmail"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_absorbing_armor"
canonical_name: "Witchmail"
snapshot_id: 40257
source_document_id: 7073
payload_hash: "c1455ac8a8cf068c44f6d5f601801d46049c96e2d37136d72b3e7b90efd1ff6e"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.278958+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Witchmail

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_absorbing_armor`
- Snapshot ID: `40257`
- Source-Dokument: `7073`
- Kurzinfo: Witchmail aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Taking heavy hits of {g:citadel_inline_attribute:'SpiritDamage'} from an enemy reduces a <span class=\"highlight\">random ability cooldown</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 22
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 14
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
        "Key": "DamageThreshold",
        "Type": "tech_damage",
        "Value": 75
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1,
    "DescKey": "#upgrade_absorbing_armor_desc",
    "Main": [
      {
        "Key": "CooldownReductionPerHit",
        "Type": "cooldown",
        "Value": 4
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_absorbing_armor",
  "Name": "Witchmail",
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
    "CooldownReduction": {
      "Key": "CooldownReduction",
      "Value": 7
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "CooldownReductionPerHit": 2,
    "TechPower": 26,
    "TechResist": 5
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
      "lookup": "witchmail",
      "name": "Witchmail",
      "type": "item"
    }
  ]
}
````
