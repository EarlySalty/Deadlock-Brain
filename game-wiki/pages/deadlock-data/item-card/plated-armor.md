---
title: "Plated Armor"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_deflecting_armor"
canonical_name: "Plated Armor"
snapshot_id: 40325
source_document_id: 7073
payload_hash: "8718941bc6ea9595dc9f207aa4af24b6f8d64b7d59b4487333854fc78b585be5"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.396188+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Plated Armor

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_deflecting_armor`
- Snapshot ID: `40325`
- Source-Dokument: `7073`
- Kurzinfo: Plated Armor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Gain a chance to either deflect incoming bullets, preventing all {g:citadel_inline_attribute:'WeaponDamage'} or prevent all <span class=\"highlight\">on-hit effects</span> from bullets.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 130
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
    "DescKey": "#upgrade_deflecting_armor_desc",
    "Main": [
      {
        "Key": "DeflectionPercent",
        "Type": "bullet_armor_up",
        "Value": 30
      },
      {
        "Key": "BulletProcDeflectionPercent",
        "Type": "bullet_armor_up",
        "Value": 50
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_deflecting_armor",
  "Name": "Plated Armor",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 1
    },
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
    "DeflectionRandomness": {
      "Key": "DeflectionRandomness",
      "Value": 1
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
    "BulletProcDeflectionPercent": 15,
    "DeflectionPercent": 15
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
      "lookup": "plated armor",
      "name": "Plated Armor",
      "type": "item"
    }
  ]
}
````
